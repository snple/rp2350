//! # 4 线调速风扇控制示例
//!
//! 本示例演示如何通过 RP2350 控制标准 PC 4 线 PWM 散热风扇。
//!
//! ---
//!
//! # 1. 4 线风扇引脚定义
//!
//! | Pin | 颜色  | 功能       | 连接说明                       |
//! |-----|-------|-----------|-------------------------------|
//! | 1   | 黑色  | GND       | 共地                          |
//! | 2   | 黄色  | +12V 供电 | 外接 12V 电源                  |
//! | 3   | 绿色  | 转速计输出 | GP1 + 4.7kΩ 上拉至 3.3V       |
//! | 4   | 蓝色  | PWM 调速  | GP0 (25 kHz PWM 输出)          |
//!
//! ## 1.1 电气注意事项
//!
//! - **转速计 (TACH)** 是开漏输出，必须上拉到 3.3V（推荐 4.7kΩ）。
//!   不可上拉到 12V，否则会烧毁 GPIO。
//! - **PWM 控制**信号电平：Intel 规范要求 5V 逻辑，但绝大多数 4 线风扇的
//!   PWM 输入兼容 3.3V 电平。如果不放心，可以用一个 N-MOSFET 做电平转换。
//! - **12V 供电**需独立电源，不要从 Pico 的 VBUS 取电。
//!
//! ## 1.2 接线图
//!
//! ```text
//!     RP2350 (Pico 2)                    4-Wire Fan
//!   ┌──────────────┐                 ┌─────────────┐
//!   │              │                 │             │
//!   │   GP0  ────────────────────────── Pin4 (蓝)  │  PWM 控制
//!   │   GND  ────────────────────────── Pin1 (黑)  │  共地
//!   │              │                 │             │
//!   │   3.3V ──┬── 4.7kΩ ─────────────── Pin3 (绿)  │  转速计
//!   │   GP1  ──┘                    │             │
//!   │              │                 │   Pin2 (黄)  │── +12V 电源
//!   └──────────────┘                 └─────────────┘
//! ```
//!
//! ---
//!
//! # 2. 工作原理
//!
//! ## 2.1 PWM 调速信号
//!
//! Intel 4 线风扇规范要求 PWM 信号频率为 **25 kHz**（可接受 21~28 kHz）。
//! 占空比决定风扇转速：
//!
//! | 占空比 | 行为 |
//! |--------|------|
//! | 0%     | 由风扇自身决定最低转速（可能停转或维持最低速） |
//! | 20%    | 低转速 |
//! | 50%    | 中等转速 |
//! | 100%   | 全速 |
//!
//! > 注：大多数风扇内部有最小转速下限，0% 占空比不代表停转。
//!
//! ## 2.2 转速计 (Tachometer)
//!
//! - 风扇每转一圈输出 **2 个脉冲**（标准 PC 风扇）
//! - 开漏输出，需要外部上拉电阻
//! - 本示例用 PWM 输入模式测量脉冲频率，换算为 RPM：
//!
//! ```text
//!   RPM = 脉冲频率 (Hz) × 60s / 2脉冲每转
//!       = 频率 × 30
//! ```
//!
//! ## 2.3 RP2350 PWM 频率配置
//!
//! ```text
//!   目标: 25 kHz 控制信号
//!   f_sys = 150 MHz, DIV = 1, WRAP = 5999
//!
//!   验证: f_PWM = 150M / (1 × 6000) = 25,000 Hz ✓
//! ```
//!
//! ---
//!
//! # 3. 编译运行
//!
//! ```bash
//! cargo run --bin fan_control --target thumbv8m.main-none-eabihf
//! ```
//!
//! 连接串口终端 (115200 baud) 观察输出。

#![no_std]
#![no_main]

use defmt::*;
use defmt_rtt as _;

use pico_rs::Peripherals;
use pico_rs::clocks::ClockBuilder;
use pico_rs::gpio::Pins;
use pico_rs::main;
use pico_rs::pwm::{InputMode, Pwm, calc_input_frequency};
use pico_rs::timer::Timer0;

// ============================================================================
// 配置参数
// ============================================================================

/// 系统时钟频率 (Hz) — PLL 配置后为 150 MHz
const SYS_CLK_HZ: u32 = 150_000_000;

/// 风扇 PWM 控制参数
/// f_PWM = SYS_CLK_HZ / (DIV × (WRAP + 1))
///       = 150_000_000 / (1 × 6000) = 25,000 Hz ✓
const FAN_PWM_FREQ_HZ: u32 = 25_000;
const FAN_PWM_WRAP: u16 = 5999;
const FAN_PWM_DIV_INT: u8 = 1;
const FAN_PWM_DIV_FRAC: u8 = 0;

/// 转速测量的门控时间 (微秒)
/// 更长的门控时间获得更稳定的读数，但更新更慢。
const TACH_GATE_TIME_US: u64 = 500_000; // 500ms

/// 风扇每转一圈输出的脉冲数 (标准 PC 风扇为 2)
const PULSES_PER_REV: u32 = 2;

/// 占空比测试序列 (百分比)
const DUTY_CYCLE_SEQUENCE: &[u32] = &[0, 25, 50, 75, 100];

/// 每个占空比稳定后的等待时间 (ms)，让风扇有时间响应
const SETTLE_TIME_MS: u32 = 3000;

// ============================================================================
// 辅助函数
// ============================================================================

/// 将占空比百分比转换为 PWM compare 值
///
/// 使用 `(WRAP + 1)` 作为基数，因此 100% 对应的 compare 值是 `WRAP + 1`。
const fn percent_to_duty(wrap: u16, percent: u32) -> u16 {
    (((wrap as u32 + 1) * percent) / 100) as u16
}

/// 根据测量的脉冲频率计算 RPM
///
/// RPM = 频率(Hz) × 60秒 / 每转脉冲数
const fn frequency_to_rpm(freq_hz: u32, pulses_per_rev: u32) -> u32 {
    freq_hz * 60 / pulses_per_rev
}

// ============================================================================
// 主程序
// ============================================================================

#[main]
fn main() -> ! {
    info!("===============================================");
    info!("  4-Wire Fan Control Example");
    info!("===============================================");

    // ------------------------------------------------------------------
    // 1. 外设初始化
    // ------------------------------------------------------------------
    let dp = Peripherals::take().unwrap();

    // 配置系统时钟到 150 MHz
    let clocks = ClockBuilder::new().build(dp.CLOCKS, dp.XOSC, dp.PLL_SYS, dp.PLL_USB);
    info!("System clock: {} Hz", clocks.sys_freq());

    let pins = Pins::init(dp.IO_BANK0, dp.PADS_BANK0);
    let timer = Timer0::init(dp.TIMER0);

    // ------------------------------------------------------------------
    // 2. PWM 控制器初始化
    // ------------------------------------------------------------------
    let pwm = Pwm::init(dp.PWM);

    // ==================================================================
    // Slice 0: PWM 控制输出 (GP0, Ch A) + 转速计输入 (GP1, Ch B)
    // ==================================================================
    info!("--- Configuring PWM Control + Tachometer on Slice 0 ---");
    info!("Output: GP0 (Ch A), Input: GP1 (Ch B)");
    info!(
        "Target: {} Hz, gate time = {} ms",
        FAN_PWM_FREQ_HZ,
        TACH_GATE_TIME_US / 1000
    );

    // GP0 → Slice 0 Ch A (PWM 输出), GP1 → Slice 0 Ch B (转速计输入)
    let fan =
        pwm.slice::<0>()
            .output_a_input(pins.pin::<0>(), pins.pin::<1>(), InputMode::RisingEdge);

    // 手动设置分频器和 wrap，确保精确的 25 kHz
    fan.set_div(FAN_PWM_DIV_INT, FAN_PWM_DIV_FRAC);
    fan.set_wrap(FAN_PWM_WRAP);
    fan.set_duty_a(0); // 初始占空比 0%

    let actual_freq = fan.get_frequency(SYS_CLK_HZ);
    info!(
        "PWM control: WRAP={}, DIV={}+{}/16, actual={} Hz",
        FAN_PWM_WRAP, FAN_PWM_DIV_INT, FAN_PWM_DIV_FRAC, actual_freq
    );

    fan.enable();

    // ==================================================================
    // 3. 主循环 — 遍历不同占空比
    // ==================================================================

    info!("=== Starting duty cycle sweep ===");
    info!(
        "Each step: set duty, wait {} ms settle, then measure RPM",
        SETTLE_TIME_MS
    );

    for &percent in DUTY_CYCLE_SEQUENCE {
        let duty = percent_to_duty(FAN_PWM_WRAP, percent);

        info!("");
        info!("--- Duty cycle: {}% (cc={}) ---", percent, duty);

        // 设置 PWM 占空比
        fan.set_duty_a(duty);

        // 等待风扇转速稳定
        info!("Waiting {} ms for fan to settle...", SETTLE_TIME_MS);
        timer.delay_ms(SETTLE_TIME_MS);

        // 测量转速 — 连续 3 次取平均值
        let mut total_rpm: u32 = 0;
        let mut valid_samples: u32 = 0;

        for sample_i in 0..3u32 {
            fan.start_measurement();
            timer.delay_us(TACH_GATE_TIME_US);
            let edge_count = fan.stop_measurement();

            if fan.overflow_occurred() {
                warn!("  Sample {}: Counter overflow, skipping", sample_i);
                continue;
            }

            let freq_hz = calc_input_frequency(edge_count, TACH_GATE_TIME_US as u32);
            let rpm = frequency_to_rpm(freq_hz, PULSES_PER_REV);

            info!(
                "  Sample {}: {} edges → {} Hz → {} RPM",
                sample_i, edge_count, freq_hz, rpm
            );

            total_rpm += rpm;
            valid_samples += 1;
        }

        if let Some(avg_rpm) = total_rpm.checked_div(valid_samples) {
            info!("  => Average RPM at {}% duty: {}", percent, avg_rpm);
        } else {
            warn!(
                "  => No valid samples at {}% duty (fan may be stopped)",
                percent
            );
        }
    }

    // ==================================================================
    // 4. 最终输出 — 保持在 50% 占空比
    // ==================================================================

    info!("");
    info!("=== Sweep complete ===");
    info!("Holding at 50% duty cycle for continuous monitoring...");
    fan.set_duty_a(percent_to_duty(FAN_PWM_WRAP, 50));

    loop {
        fan.start_measurement();
        timer.delay_us(TACH_GATE_TIME_US);
        let edge_count = fan.stop_measurement();

        if !fan.overflow_occurred() {
            let freq_hz = calc_input_frequency(edge_count, TACH_GATE_TIME_US as u32);
            let rpm = frequency_to_rpm(freq_hz, PULSES_PER_REV);
            info!(
                "RPM: {} ({} Hz, {} edges)  [50% duty]",
                rpm, freq_hz, edge_count
            );
        } else {
            warn!("Tach overflow");
        }
    }
}
