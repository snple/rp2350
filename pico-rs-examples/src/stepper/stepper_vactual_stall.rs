//! # TMC2209 VACTUAL 堵转限位 — UART 直接速度控制
//!
//! 使用 TMC2209 的 VACTUAL 寄存器通过 UART 直接控制电机速度，
//! 无需 STEP/DIR 脉冲，运转绝对平稳。堵转由 DIAG 引脚检测。
//!
//! ## 编译运行
//!
//! ```bash
//! cd pico-rs-examples
//! cargo run --bin stepper_vactual_stall --target thumbv8m.main-none-eabihf
//! ```

#![no_std]
#![no_main]

use pico_rs_drivers::tmc2209::{self, Tmc2209};

use core::sync::atomic::{AtomicBool, Ordering};
use defmt::*;
use defmt_rtt as _;

use pico_rs::Steal;
use pico_rs::block::ImageDef;
use pico_rs::clocks::ClockBuilder;
use pico_rs::entry;
use pico_rs::gpio::{EdgeLevel, Gpio, Pin, Pins};
use pico_rs::interrupt;
use pico_rs::pac::{Interrupt, Peripherals};
use pico_rs::ticks::Ticks;
use pico_rs::timer::Timer0;
use pico_rs::uart::{DataBits, Parity, StopBits, Uart1};

#[unsafe(link_section = ".start_block")]
#[used]
pub static IMAGE_DEF: ImageDef = ImageDef::exe();

// ============================================================================
// 参数
// ============================================================================

/// 运行速度 (微步/秒)。VACTUAL 转换为  microsteps_per_sec / 0.715
const RUN_SPEED_HZ: u32 = 3000;

/// 微步细分数
const MICROSTEPPING: u32 = 16;

/// 运行电流 IRUN (0–31)。堵转限位应偏低，让电机容易堵转。
///
///   IRUN │  4   6   8  10  12  14  16  18  20
///   ─────┼─────────────────────────────────────
///   mA   │ 280 390 500 610 720 830 940 1.0 1.2
const RUN_CURRENT: u8 = 6;

/// 保持电流 IHOLD (0–31)
const HOLD_CURRENT: u8 = 2;

/// StallGuard 堵转阈值 (0–255)。越大越灵敏。
/// 触发条件: SG_RESULT ≤ SGTHRS × 2
/// 建议 = (堵转时 SG_RESULT) / 2
const SG_THRESHOLD: u32 = 100;

/// 堵转后停顿时间 (ms)
const STALL_PAUSE_MS: u32 = 500;

/// 堵转后退离末端的距离（以 VACTUAL 低速运行的时间 ms）
const BACKOFF_MS: u32 = 300;

/// 退离时的速度 (微步/秒)，比主速度低
const BACKOFF_SPEED_HZ: u32 = 1000;

/// UART 波特率
const TMC_UART_BAUD: u32 = 115_200;

// ============================================================================
// 引脚定义
// ============================================================================

const PIN_ENABLE: u8 = 7;

// #1 (电机 1: GP8–11, addr 0x00)
// VACTUAL 模式下无需 STEP/DIR/INDEX，方向由 set_vactual(±vel) 控制。
const TMC_SLAVE_ADDR: u8 = 0x00;
const PIN_DIAG: u8 = 11;

// ============================================================================
// 全局状态
// ============================================================================

/// 堵转标记：DIAG ISR 置 true，主循环检查后清除
static STALL_FLAG: AtomicBool = AtomicBool::new(false);

#[inline(always)]
fn gpio() -> Steal<Gpio> {
    unsafe { Gpio::steal() }
}

// ============================================================================
// 程序入口
// ============================================================================

#[entry]
fn main() -> ! {
    info!("=== TMC2209 VACTUAL Stall Homing ===");

    // --- 外设初始化 ---
    let dp = Peripherals::take().unwrap();
    let clocks = ClockBuilder::new().build(dp.CLOCKS, dp.XOSC, dp.PLL_SYS, dp.PLL_USB);

    let ticks = Ticks::new(dp.TICKS, &clocks);
    ticks.init_timer0();

    let timer = Timer0::init(dp.TIMER0);
    let pins = Pins::new(dp.IO_BANK0, dp.PADS_BANK0);

    // --- GPIO ---
    let _en = pins.pin::<PIN_ENABLE>();
    _en.into_output();
    gpio().set_high(PIN_ENABLE);

    let p_diag = pins.pin::<PIN_DIAG>();
    p_diag.into_input_pullup();
    p_diag.enable_interrupt(EdgeLevel::EdgeHigh);

    // --- UART ---
    let uart = Uart1::new(
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

    tmc.configure(
        &timer,
        MICROSTEPPING,
        RUN_CURRENT,
        HOLD_CURRENT,
        10,
        SG_THRESHOLD,
    );
    info!(
        "VACTUAL mode: 1/{} μstep, {} Hz → VACTUAL={}, SGTHRS={}",
        MICROSTEPPING,
        RUN_SPEED_HZ,
        tmc2209::microsteps_per_sec_to_vactual(RUN_SPEED_HZ as i32),
        SG_THRESHOLD,
    );

    // VACTUAL 模式下 TSTEP 不会从 STEP 引脚更新，必须设 TCOOLTHRS=max
    // 否则 DIAG 输出条件 TSTEP < TCOOLTHRS 永远不成立。
    tmc.set_tcool_threshold(0xFFFFF, &timer);
    unsafe {
        interrupt::enable(Interrupt::IO_IRQ_BANK0);
    }

    // --- 使能 ---
    gpio().set_low(PIN_ENABLE);
    timer.delay_ms(100);

    let vactual = tmc2209::microsteps_per_sec_to_vactual(RUN_SPEED_HZ as i32);
    let vactual_back = tmc2209::microsteps_per_sec_to_vactual(BACKOFF_SPEED_HZ as i32);
    info!("VACTUAL run=±{} backoff=±{}", vactual, vactual_back);

    // ======================================================================
    // 快速诊断：采样 SG_RESULT 辅助调阈值 (1s，短行程)
    // ======================================================================
    info!("Diag: quick SG_RESULT sample...");
    tmc.set_vactual(vactual, &timer);
    timer.delay_ms(100);

    let mut sg_min: u16 = 65535;
    let mut sg_max: u16 = 0;
    for _ in 0..5 {
        if let Some(sg) = tmc.read_sg_result(&timer) {
            sg_min = sg_min.min(sg);
            sg_max = sg_max.max(sg);
        }
        timer.delay_ms(200);
    }
    tmc.stop_vactual(&timer);
    timer.delay_ms(100);

    // SGTHRS 应设得比正常最小值低一些：建议 = sg_min × 0.3 ~ 0.5
    // 当前: SGTHRS={}，触发条件 SG_RESULT ≤ SGTHRS×2 = {}
    let suggested = (sg_min as u32).saturating_mul(3) / 10;
    info!(
        "SG_RESULT: min={} max={}  → 建议 SGTHRS={} (当前={})",
        sg_min, sg_max, suggested, SG_THRESHOLD,
    );
    if SG_THRESHOLD as u16 * 2 >= sg_min {
        warn!(
            "⚠ 当前 SGTHRS={} 太灵敏! 正常 SG_RESULT={} 就会触发 (阈值={})",
            SG_THRESHOLD,
            sg_min,
            SG_THRESHOLD * 2,
        );
    } else {
        info!("✓ SG_THRESHOLD 正常范围，堵转时应能触发");
    }

    // ======================================================================
    // VACTUAL 堵转限位往返
    // ======================================================================

    info!(
        "Ready. Touch end-stop to trigger stall → reverse.  VACTUAL=±{}",
        vactual,
    );

    let mut forward = true;
    let mut lap: u32 = 0;

    loop {
        lap += 1;
        let dir_str = if forward { "FWD" } else { "REV" };
        let vel = if forward { vactual } else { -vactual };

        info!("[lap {}] {} VACTUAL={} ...", lap, dir_str, vel);
        tmc.set_vactual(vel, &timer);

        // 等待堵转
        while !STALL_FLAG.swap(false, Ordering::Relaxed) {
            pico_rs_rt::asm::nop();
        }

        // 立即停止，然后反向低速退离末端
        tmc.stop_vactual(&timer);
        timer.delay_ms(50);

        info!("[lap {}] STALL! — stop + back off {} ms", lap, BACKOFF_MS);

        let back_vel = if forward { -vactual_back } else { vactual_back };
        tmc.set_vactual(back_vel, &timer);
        timer.delay_ms(BACKOFF_MS);

        tmc.stop_vactual(&timer);
        timer.delay_ms(STALL_PAUSE_MS);

        forward = !forward;
    }
}

// ============================================================================
// ISR
// ============================================================================

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
fn IO_IRQ_BANK0() {
    let pin = unsafe { Pin::<{ PIN_DIAG }>::steal() };
    if pin.check_and_clear(EdgeLevel::EdgeHigh) {
        STALL_FLAG.store(true, Ordering::Relaxed);
    }
}
