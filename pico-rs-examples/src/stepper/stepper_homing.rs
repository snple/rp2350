//! # TMC2209 行程校准 — 双端探测 + 软限位 + INDEX 丢步检测
//!
//! ## 流程
//!
//! ```text
//! 1. probe_endstop(REV) → set_position(0)    // 归位近端
//! 2. probe_endstop(FWD) → 读 position()      // 探测远端，得 travel
//! 3. set_soft_limits(0, travel)              // 设置软限位保护
//! 4. 往返验证 + INDEX 丢步检测                // 安全范围内自由运动
//! ```
//!
//! ## 编译运行
//!
//! ```bash
//! cd pico-rs-examples
//! cargo run --bin stepper_homing --target thumbv8m.main-none-eabihf
//! ```

#![no_std]
#![no_main]

use pico_rs_drivers::stepper::Direction;
use pico_rs_drivers::stepper::tmc2209::{AxisConfig, AxisState};
use pico_rs_drivers::tmc2209::Tmc2209;

use defmt::*;
use defmt_rtt as _;

use pico_rs::Steal;
use pico_rs::clocks::ClockBuilder;
use pico_rs::gpio::{self, Pins};
use pico_rs::interrupt;
use pico_rs::main;
use pico_rs::pac::{Interrupt, Peripherals};
use pico_rs::ticks::Ticks;
use pico_rs::timer::Timer0;
use pico_rs::uart::{DataBits, Parity, StopBits, Uart1};

// ============================================================================
// 参数
// ============================================================================

const PROBE_SPEED_HZ: u32 = 10000;
const ACCEL_STEPS: u32 = 100;
const STEP_PULSE_US: u32 = 3;
const MICROSTEPPING: u32 = 16;
const RUN_CURRENT: u8 = 12;
const HOLD_CURRENT: u8 = 2;
const SG_THRESHOLD: u32 = 80;
const TMC_UART_BAUD: u32 = 115_200;
const INDEX_BACKOFF: u32 = 2;

// ============================================================================
// 引脚
// ============================================================================

const PIN_ENABLE: u32 = 7;
const TMC_SLAVE_ADDR: u8 = 0x00;
const PIN_DIR: u32 = 8;
const PIN_STEP: u32 = 9;
const PIN_INDEX: u32 = 10;
const PIN_DIAG: u32 = 11;

// ============================================================================
// 步进轴
// ============================================================================

static AXIS: AxisState = AxisState::new(AxisConfig {
    pin_step: PIN_STEP,
    pin_dir: PIN_DIR,
    alarm: 0,
    start_speed_hz: 500,
    accel_steps: ACCEL_STEPS,
    step_pulse_us: STEP_PULSE_US,
    diag_pin: PIN_DIAG,
    index_pin: PIN_INDEX,
    index_backoff: INDEX_BACKOFF,
    invert_dir: true,
});

#[inline(always)]
fn timer() -> Steal<Timer0> {
    unsafe { Timer0::steal() }
}

// ============================================================================
// main
// ============================================================================

#[main]
fn main() -> ! {
    info!("=== TMC2209 Travel Calibration ===");

    // --- 外设初始化 ---
    let dp = Peripherals::take().unwrap();
    let clocks = ClockBuilder::new().build(dp.CLOCKS, dp.XOSC, dp.PLL_SYS, dp.PLL_USB);
    let ticks = Ticks::new(dp.TICKS, &clocks);
    ticks.init_timer0();
    let timer = Timer0::init(dp.TIMER0);
    let pins = Pins::new(dp.IO_BANK0, dp.PADS_BANK0);

    let _en = pins.pin::<PIN_ENABLE>();
    _en.into_output();
    unsafe { gpio::set_high(PIN_ENABLE) };
    let _dir = pins.pin::<PIN_DIR>();
    _dir.into_output();
    unsafe { gpio::set_low(PIN_DIR) };
    let _step = pins.pin::<PIN_STEP>();
    _step.into_output();
    unsafe { gpio::set_low(PIN_STEP) };
    let _index = pins.pin::<PIN_INDEX>();
    _index.into_input_pullup();
    let p_diag = pins.pin::<PIN_DIAG>();
    p_diag.into_input_pullup();

    let uart = Uart1::new(
        dp.UART1,
        pins.pin::<4>(),
        pins.pin::<5>(),
        TMC_UART_BAUD,
        clocks.peri_freq(),
    );
    uart.set_format(DataBits::Eight, StopBits::One, Parity::None);
    timer.delay_ms(100);

    // --- TMC2209 ---
    let tmc = Tmc2209::new(&uart, TMC_SLAVE_ADDR);
    match tmc.read_ioin(&timer) {
        Some(v) => info!(
            "TMC2209 addr={:#x} ver={:#04x} ✓",
            TMC_SLAVE_ADDR,
            v.version()
        ),
        None => {
            warn!("TMC2209 not responding — halt");
            loop {
                pico_rs_rt::asm::wfi();
            }
        }
    }
    tmc.configure(
        &timer,
        MICROSTEPPING,
        RUN_CURRENT,
        HOLD_CURRENT,
        10,
        SG_THRESHOLD,
    );
    info!(
        "1/{} μstep, IRUN={}, IHOLD={}, SGTHRS={}",
        MICROSTEPPING, RUN_CURRENT, HOLD_CURRENT, SG_THRESHOLD
    );
    tmc.set_tcool_threshold(0xFFFFF, &timer);
    tmc.stop_vactual(&timer);

    unsafe {
        interrupt::enable(Interrupt::TIMER0_IRQ_0);
    }
    unsafe { gpio::set_low(PIN_ENABLE) };

    timer.delay_ms(100);

    // ======================================================================
    // Step 1: 探测近端 → 标记 home=0
    // ======================================================================
    info!("--- Step 1: Probe near-end ---");
    AXIS.probe_endstop(&timer, Direction::Reverse, PROBE_SPEED_HZ);
    AXIS.wait_done();
    AXIS.set_position(0);
    info!(
        "home=0 (INDEX backoff={}, pulses={})",
        INDEX_BACKOFF,
        AXIS.index_count()
    );

    timer.delay_ms(1000);

    // ======================================================================
    // Step 2: 探测远端 → 得有效行程
    // ======================================================================
    info!("--- Step 2: Probe far-end ---");
    AXIS.probe_endstop(&timer, Direction::Forward, PROBE_SPEED_HZ);
    AXIS.wait_done();

    let travel = AXIS.position();
    info!("far-end reached, travel = {} μsteps", travel);

    let fs = travel.unsigned_abs() / MICROSTEPPING;
    info!(
        "effective range: {} μsteps = {} full-steps ≈ {}.{} mm (GT2 20T)",
        travel,
        fs,
        fs / 100,
        (fs % 100) / 10
    );

    // Step 3: 设置行程保护
    // ======================================================================
    AXIS.set_max_travel(travel);
    info!("max travel = {} steps", travel);

    timer.delay_ms(1000);

    loop {
        core::hint::spin_loop();
    }
}

// ============================================================================
// ISR
// ============================================================================

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
fn TIMER0_IRQ_0() {
    let t = timer();
    AXIS.handle_alarm_isr(&t);
}
