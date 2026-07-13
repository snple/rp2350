//! # TMC2209 高速往返 — 精简版
//!
//! 最简驱动示例：配置 TMC2209 后高速往返旋转，不做任何诊断测试。
//!
//! ## 编译运行
//!
//! ```bash
//! cd pico-rs-examples
//! cargo run --bin stepper_simple --target thumbv8m.main-none-eabihf
//! ```

#![no_std]
#![no_main]

use pico_rs_drivers::stepper::{AxisPins, Direction};
use pico_rs_drivers::stepper::tmc2209::{AxisConfig, AxisState, TmcPins};
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
use pico_rs::uart::{DataBits, Parity, StopBits, Uart1}; // ============================================================================
// 参数
// ============================================================================

// ============================================================================
// 运行预设 — 三选一，只保留一组不注释
// ============================================================================

// -- 预设 1: 慢速运行 — 安静、低温、适合调试 ──────────────────────
// const MAX_SPEED_HZ: u32 = 2000;
// const ACCEL_STEPS: u32 = 500;
// const START_SPEED_HZ: u32 = 100;
// const TRAVEL_REVS: u32 = 1;
//   → 建议 RUN_CURRENT=8, HOLD_CURRENT=2

// -- 预设 2: 常规速度 — 日常使用 ──────────────────────────────────
// const MAX_SPEED_HZ: u32 = 6000;
// const ACCEL_STEPS: u32 = 300;
// const START_SPEED_HZ: u32 = 200;
// const TRAVEL_REVS: u32 = 2;
//   → 建议 RUN_CURRENT=12, HOLD_CURRENT=4

// -- 预设 3: 高速运行 — 性能优先 (注意散热) ──────────────────────
const MAX_SPEED_HZ: u32 = 12000;
const ACCEL_STEPS: u32 = 300;
const START_SPEED_HZ: u32 = 200;
const TRAVEL_REVS: u32 = 2;
//   → 建议 RUN_CURRENT=16, HOLD_CURRENT=4

// ============================================================================
// 通用参数
// ============================================================================

/// STEP 脉冲高电平宽度 (μs)
const STEP_PULSE_US: u32 = 3;

/// 微步细分数
const MICROSTEPPING: u32 = 4;

/// 电机每转全步数 (1.8° = 200 步/转)
const MOTOR_STEPS_PER_REV: u32 = 200;

/// 运行电流 IRUN (0–31)
///
/// 换算公式（TMC2209 datasheet §9.2）:
///   I_RMS = (IRUN + 1) / 32 × V_FS / (R_SENSE + 0.02Ω) / √2
///
/// 其中 V_FS = 0.325V (vsense=0, 默认) 或 0.180V (vsense=1, 高灵敏度).
/// 常见 TMC2209 模块使用 R_SENSE = 0.11Ω, vsense=0:
///
///   IRUN │  0   1   2   3   4   5   6   7   8   9  10  11  12  13  14  15
///   ─────┼──────────────────────────────────────────────────────────────────
///   mA   │ 60 110 170 220 280 330 390 440 500 550 610 660 720 770 830 880
///
///   IRUN │ 16  17  18  19  20  21  22  23  24  25  26  27  28  29  30  31
///   ─────┼──────────────────────────────────────────────────────────────────
///   mA   │ 940 990 1.0 1.1 1.2 1.2 1.3 1.3 1.4 1.4 1.5 1.5 1.6 1.6 1.7 1.8
const RUN_CURRENT: u8 = 12;

/// 保持电流 IHOLD (0–31). 停止后自动降到该值。换算同 IRUN。
const HOLD_CURRENT: u8 = 4;

/// UART 波特率
const TMC_UART_BAUD: u32 = 115_200;

// ============================================================================
// 引脚定义 — 修改此处切换电机
// ============================================================================

const PIN_ENABLE: u32 = 7;

// #1 (电机 1: GP8–11, addr 0x00)
const TMC_SLAVE_ADDR: u8 = 0x00;
const PIN_DIR: u32 = 8;
const PIN_STEP: u32 = 9;
const PIN_INDEX: u32 = 10;
const PIN_DIAG: u32 = 11;

// #2 (电机 2: GP12–15, addr 0x01)
// const TMC_SLAVE_ADDR: u8 = 0x01;
// const PIN_DIR: u32 = 12;
// const PIN_STEP: u32 = 13;
// const PIN_INDEX: u32 = 14;
// const PIN_DIAG: u32 = 15;

// #3 (电机 3: GP16–19, addr 0x02)
// const TMC_SLAVE_ADDR: u8 = 0x02;
// const PIN_DIR: u32 = 16;
// const PIN_STEP: u32 = 17;
// const PIN_INDEX: u32 = 18;
// const PIN_DIAG: u32 = 19;

// #4 (电机 4: GP16–19, addr 0x03)
// const TMC_SLAVE_ADDR: u8 = 0x03;
// const PIN_DIR: u32 = 16;
// const PIN_STEP: u32 = 17;
// const PIN_INDEX: u32 = 20;
// const PIN_DIAG: u32 = 21;

// ============================================================================
// Pin trait 实现 — 零开销编译期单态化
// ============================================================================

/// TMC2209 引脚桥接，实现 STEP/DIR/DIAG/INDEX 的 GPIO 操作。
struct MotPins;

impl AxisPins for MotPins {
    #[inline]
    fn set_step(high: bool) {
        unsafe { gpio::set(PIN_STEP, high); }
    }

    #[inline]
    fn set_dir(high: bool) {
        unsafe { gpio::set(PIN_DIR, high); }
    }
}

impl TmcPins for MotPins {
    #[inline]
    fn diag() -> bool {
        unsafe { gpio::get(PIN_DIAG) }
    }

    #[inline]
    fn index() -> bool {
        unsafe { gpio::get(PIN_INDEX) }
    }
}

// ============================================================================
// 步进轴状态
// ============================================================================

static AXIS: AxisState<MotPins> = AxisState::new(AxisConfig {
    alarm: 0,
    start_speed_hz: START_SPEED_HZ,
    accel_steps: ACCEL_STEPS,
    step_pulse_us: STEP_PULSE_US,
    index_backoff: 0,
    fwd_level: false,
    blanking_steps: 200,
});

#[inline(always)]
fn timer() -> Steal<Timer0> {
    unsafe { Timer0::steal() }
}

// ============================================================================
// 程序入口
// ============================================================================

#[main]
fn main() -> ! {
    info!("=== TMC2209 Simple ===");

    // --- 外设初始化 ---
    let dp = Peripherals::take().unwrap();
    let clocks = ClockBuilder::new().build(dp.CLOCKS, dp.XOSC, dp.PLL_SYS, dp.PLL_USB);

    let ticks = Ticks::init(dp.TICKS, &clocks);
    ticks.init_timer0();

    let timer = Timer0::init(dp.TIMER0);
    let pins = Pins::init(dp.IO_BANK0, dp.PADS_BANK0);

    // --- GPIO ---
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

    // --- UART ---
    let uart = Uart1::init(
        dp.UART1,
        pins.pin::<4>(),
        pins.pin::<5>(),
        TMC_UART_BAUD,
        clocks.peri_freq(),
    );
    uart.set_format(DataBits::Eight, StopBits::One, Parity::None);
    timer.delay_ms(100);

    // --- 配置 TMC2209 ---
    let tmc = Tmc2209::new(&uart, TMC_SLAVE_ADDR);

    match tmc.read_ioin(&timer) {
        Some(v) => info!("Chip addr={:#x} ver={:#04x} ✓", TMC_SLAVE_ADDR, v.version()),
        None => {
            warn!("TMC2209 not responding — halt");
            loop {
                pico_rs_rt::asm::wfi();
            }
        }
    }

    // SGTHRS: 堵转检测阈值。80 = 中等灵敏度，正常加减速不误触发。
    // 如果仍误触发则继续提高；如果真堵转不触发则降低。
    tmc.configure(&timer, MICROSTEPPING, RUN_CURRENT, HOLD_CURRENT, 10, 80);
    // 关闭 VACTUAL 模式 — 防止上次 UART 控速残留导致电机自转
    tmc.stop_vactual(&timer);
    info!(
        "Configured: 1/{} μstep, IRUN={}, IHOLD={}",
        MICROSTEPPING, RUN_CURRENT, HOLD_CURRENT
    );

    // --- 回读确认关键配置 ---
    if let Some(c) = tmc.read_chopconf(&timer) {
        info!(
            "CHOPCONF: mres={} toff={} intpol={}",
            c.mres(),
            c.toff(),
            c.intpol()
        );
    }
    if let Some(sg) = tmc.read_sg_threshold(&timer) {
        info!("SGTHRS={} (stall if SG_RESULT ≤ {})", sg, sg * 2);
    }

    // --- 中断 ---
    unsafe {
        interrupt::enable(Interrupt::TIMER0_IRQ_0);
    }

    // --- 启用 ---
    unsafe { gpio::set_low(PIN_ENABLE) };
    timer.delay_ms(100);

    // --- 上电诊断：慢速试运行以触发开路/短路检测 ---
    // TMC2209 的开路检测 (ola/olb) 只在电机慢速运动时有效，静止时不可靠。
    info!("Diag: slow move to check open-load...");
    AXIS.start_move(&timer, 200, Direction::Forward, 500);
    AXIS.wait_done();
    timer.delay_ms(50);

    if let Some(drv) = tmc.read_drv_status(&timer) {
        let s2g = drv.s2ga() || drv.s2gb();
        let ol = drv.ola() || drv.olb();
        let ot = drv.ot() || drv.otpw() || drv.t120();
        let fault = s2g || ol || ot;
        info!(
            "DRV_STATUS: cs={} stealth={} stst={}",
            drv.cs_actual(),
            drv.stealth(),
            drv.stst(),
        );
        if s2g {
            warn!("  ⚡ 对地短路! A={} B={}", drv.s2ga(), drv.s2gb());
        }
        if ol {
            warn!("  🔌 开路/缺相! A={} B={}", drv.ola(), drv.olb());
        }
        if ot {
            warn!(
                "  🌡 过温! otpw={} ot={} t120={} t150={}",
                drv.otpw(),
                drv.ot(),
                drv.t120(),
                drv.t150()
            );
        }
        if !fault {
            info!("  ✓ 无短路/开路/过温");
        }
    } else {
        warn!("  ⚠ 无法读取 DRV_STATUS");
    }

    // ======================================================================
    // 高速往返
    // ======================================================================

    let steps_per_rev = MOTOR_STEPS_PER_REV * MICROSTEPPING;
    let travel_steps = steps_per_rev * TRAVEL_REVS;
    info!(
        "{} steps/rev × {} revs = {} steps/direction",
        steps_per_rev, TRAVEL_REVS, travel_steps
    );

    let mut forward = true;
    let mut lap: u32 = 0;
    loop {
        let dir = if forward {
            Direction::Forward
        } else {
            Direction::Reverse
        };
        lap += 1;
        info!(
            "[lap {}] {} {} steps @ {} Hz",
            lap,
            dir.as_str(),
            travel_steps,
            MAX_SPEED_HZ
        );

        AXIS.start_move(&timer, travel_steps, dir, MAX_SPEED_HZ);
        AXIS.wait_done();

        // --- INDEX 丢步检测 ---
        let idx = AXIS.check_index(travel_steps, MICROSTEPPING);
        if !idx.is_accurate(1) {
            warn!(
                "[lap {}] ⚠ STEP LOSS: expected {} INDEX pulses, got {} (≈{} microsteps lost)",
                lap,
                idx.expected,
                idx.actual,
                idx.missed_microsteps(MICROSTEPPING),
            );
        }

        // --- TMC2209 诊断 ---
        let sg = tmc.read_sg_result(&timer).unwrap_or(0);
        let stalled = AXIS.stall_occurred();
        if let Some(drv) = tmc.read_drv_status(&timer) {
            let ot = drv.ot() || drv.otpw();
            info!(
                "[lap {}] done  pos={}  sg={} cs={} stealth={} stst={} {}{}",
                lap,
                AXIS.position(),
                sg,
                drv.cs_actual(),
                drv.stealth(),
                drv.stst(),
                if stalled { "[STALL] " } else { "" },
                if ot { "[OT!]" } else { "" },
            );
            if stalled || ot {
                warn!(
                    "  DRV_STATUS: otpw={} ot={} s2g_a={} s2g_b={} ola={} olb={} t120={} t150={}",
                    drv.otpw(),
                    drv.ot(),
                    drv.s2ga(),
                    drv.s2gb(),
                    drv.ola(),
                    drv.olb(),
                    drv.t120(),
                    drv.t150(),
                );
            }
        } else {
            info!(
                "[lap {}] done  pos={}  sg={} {}",
                lap,
                AXIS.position(),
                sg,
                if stalled { "[STALL]" } else { "" },
            );
        }
        timer.delay_ms(300);
        forward = !forward;
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
