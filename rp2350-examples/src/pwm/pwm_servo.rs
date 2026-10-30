//! # 舵机控制示例 — GPIO0 PWM 输出
//!
//! 本示例演示如何用 RP2350 (Raspberry Pi Pico 2) 控制一个标准
//! 位置型舵机 (SG90 / MG996R 等)。PWM 信号从 **GPIO0** 输出。
//!
//! ---
//!
//! # 1. 舵机控制原理
//!
//! 常见舵机使用 **50 Hz** PWM (周期 20 ms)，通过**高电平脉宽**指定目标角度：
//!
//! ```text
//!   周期 20 ms
//!   ┌──────────────────────────────────────┐
//!   │                                      │
//!   │  ┌────────┐                          │
//!   │  │ 脉宽   │                          │
//!   └──┘        └──────────────────────────┘
//!      ← 0.5 ms  →  0°    (最小)
//!      ← 1.5 ms  →  90°   (中位)
//!      ← 2.5 ms  →  180°  (最大)
//! ```
//!
//! | 角度 | 脉宽 | 说明 |
//! |------|------|------|
//! | 0°   | 0.5 ms | 最小位置 |
//! | 90°  | 1.5 ms | 中位 |
//! | 180° | 2.5 ms | 最大位置 |
//!
//! > 不同型号舵机的有效范围略有差异。部分舵机只接受 1.0~2.0 ms，
//! > 若出现抖动或堵转，请把 [`SERVO_MIN_PULSE_US`] / [`SERVO_MAX_PULSE_US`]
//! > 调整到对应的脉宽范围。
//!
//! 舵机每 20 ms 才采样一次信号，因此控制值更新得再快也不会提高响应速度。
//! 示例中的角度切换间隔取 20 ms 的整数倍即可。
//!
//! ---
//!
//! # 2. 硬件映射
//!
//! GPIO0 固定映射到 **PWM Slice 0, Channel A**：
//!
//! ```text
//!   GPIO0  →  slice 0, channel A   (PWM_CH0_A)
//! ```
//!
//! 该映射由 RP2350 硬件决定：`slice = (gpio / 2) % 8`，`channel = gpio % 2`。
//!
//! ---
//!
//! # 3. 定时参数
//!
//! RP2350 的 PWM 频率为：
//!
//! ```text
//!   f_PWM = f_sys / (DIV × (TOP + 1))
//! ```
//!
//! 本示例取 `TOP = 19999`、`DIV = 150`，在 150 MHz 系统时钟下：
//!
//! ```text
//!   f_PWM = 150 MHz / (150 × 20000) = 50 Hz      (周期 20 ms)
//!   每个计数 = 1 / 50 Hz / 20000 = 1 µs
//! ```
//!
//! 因此脉宽可以直接写成微秒数：`CC = 1500` 即 1.5 ms。
//! 这是舵机控制中最方便的分辨率选择。
//!
//! ---
//!
//! # 4. 接线
//!
//! ```text
//!   RP2350 (Pico 2)                  舵机 (SG90 / MG996R)
//!   ┌──────────────┐                ┌──────────────────┐
//!   │              │                │                  │
//!   │   GP0  ──────────────────────── 信号线 (橙/白/黄) │
//!   │   GND  ──────────────────────── 地线   (棕/黑)   │
//!   │              │                │                  │
//!   │              │     外部 5V ──── 电源线 (红)       │
//!   └──────────────┘                └──────────────────┘
//!          GND ─────────────────────── 外部 5V 地 (必须共地)
//! ```
//!
//! ## 4.1 电源注意事项
//!
//! - 舵机堵转电流可达 **1~2 A**，**不要**直接从 Pico 的 5V/VBUS 取电，
//!   否则可能引起电压跌落、MCU 复位甚至损坏板卡。
//! - 使用**独立 5V 电源**给舵机供电，并将该电源的地与 Pico 的 GND 相连。
//! - 在舵机电源附近并联一个 **470~1000 µF** 电解电容，抑制启动电流冲击。
//! - 舵机信号线是 3.3V 兼容输入，RP2350 的 3.3V 输出通常可直接驱动。
//!
//! ---
//!
//! # 5. 编译运行
//!
//! ```bash
//! cargo run --bin pwm_servo --target thumbv8m.main-none-eabihf
//! ```

#![no_std]
#![no_main]

use defmt::*;
use defmt_rtt as _;

use rp2350::clocks::ClockBuilder;
use rp2350::gpio::Pins;
use rp2350::main;
use rp2350::pac::Peripherals;
use rp2350::pwm::Pwm;

/// 舵机标准 PWM 频率：50 Hz（周期 20 ms）
const SERVO_FREQ_HZ: u32 = 50;

/// 周期 20 ms = 20000 个 1 µs 计数，TOP = 计数个数 - 1
const SERVO_WRAP: u16 = 19_999;

/// 0° 对应的脉宽（µs）
const SERVO_MIN_PULSE_US: u16 = 500;

/// 180° 对应的脉宽（µs）
const SERVO_MAX_PULSE_US: u16 = 2_500;

/// 上电时的中位脉宽（90°）
const SERVO_CENTER_PULSE_US: u16 = 1_500;

/// 将角度 (0~180°) 线性映射为脉宽 (µs)。
const fn angle_to_pulse_us(angle: u32) -> u16 {
    let span = (SERVO_MAX_PULSE_US - SERVO_MIN_PULSE_US) as u32;
    (SERVO_MIN_PULSE_US as u32 + span * angle / 180) as u16
}

#[main]
fn main() -> ! {
    info!("=== Servo Control Example (GPIO0) ===");

    // ------------------------------------------------------------------
    // 1. 外设初始化
    // ------------------------------------------------------------------
    let dp = Peripherals::take().unwrap();

    let clocks = ClockBuilder::new().build(dp.CLOCKS, dp.XOSC, dp.PLL_SYS, dp.PLL_USB);
    let sys_clk = clocks.sys_freq();

    let pins = Pins::init(dp.IO_BANK0, dp.PADS_BANK0);
    let delay = rp2350_examples::common::board::init_delay(&clocks);

    info!("System clock: {} Hz", sys_clk);

    // ------------------------------------------------------------------
    // 2. GPIO0 → Slice 0 Channel A
    // ------------------------------------------------------------------
    let pwm = Pwm::init(dp.PWM);
    let mut servo = pwm.slice::<0>().output_a(pins.pin::<0>());

    // TOP = 19999, DIV = 150 → 50 Hz，每个计数正好 1 µs
    let actual_hz = servo.configure_frequency_with_wrap(sys_clk, SERVO_FREQ_HZ, SERVO_WRAP);
    let (div_int, div_frac) = servo.div();

    // 先输出中位脉宽，再打开 PWM，避免上电瞬间乱转
    servo.set_duty_a(SERVO_CENTER_PULSE_US);
    servo.enable();

    info!(
        "GPIO0: slice 0 ch A, TOP={}, DIV={}+{}/16, actual={} Hz ({} us period)",
        SERVO_WRAP,
        div_int,
        div_frac,
        actual_hz,
        1_000_000 / SERVO_FREQ_HZ
    );
    info!(
        "Pulse range: {} us (0 deg) .. {} us (180 deg)",
        SERVO_MIN_PULSE_US, SERVO_MAX_PULSE_US
    );

    // ------------------------------------------------------------------
    // 3. 定点演示：依次转到几个角度并停留
    // ------------------------------------------------------------------
    let angles: [u32; 8] = [90, 0, 45, 90, 135, 180, 90, 0];
    for &angle in angles.iter() {
        let pulse_us = angle_to_pulse_us(angle);
        servo.set_duty_a(pulse_us);
        info!("angle={} deg, pulse={} us", angle, pulse_us);
        delay.delay_ms(600); // 等待舵机转到目标位置
    }

    // ------------------------------------------------------------------
    // 4. 平滑往返扫描 3 次：0° → 180° → 0°
    //    每 20 ms 走 1°，一次单程约 3.6 秒
    // ------------------------------------------------------------------
    info!("Sweep 0 deg <-> 180 deg, 3 rounds");
    for round in 0..3u32 {
        info!("sweep round {}", round + 1);
        for angle in 0..=180u32 {
            servo.set_duty_a(angle_to_pulse_us(angle));
            delay.delay_ms(20);
        }
        for angle in (0..=180u32).rev() {
            servo.set_duty_a(angle_to_pulse_us(angle));
            delay.delay_ms(20);
        }
    }

    // ------------------------------------------------------------------
    // 5. 停在 90° 中位
    //    舵机需要持续的 PWM 信号才能保持位置和力矩，因此保持输出
    //    1.5 ms 脉宽，而不是关闭 PWM。
    // ------------------------------------------------------------------
    servo.set_duty_a(SERVO_CENTER_PULSE_US);
    info!(
        "Parked at 90 deg (pulse={} us), holding position",
        SERVO_CENTER_PULSE_US
    );

    loop {
        delay.delay_ms(1_000);
    }
}
