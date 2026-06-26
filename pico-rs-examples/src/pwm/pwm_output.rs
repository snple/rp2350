//! # PWM 输出示例 — 从频率配置到占空比控制
//!
//! 基于 RP2350 (Raspberry Pi Pico 2) 的 **PWM 输出**示例，
//! 演示如何使用 pico-rs 的 PWM HAL 完成以下几类常见任务：
//!
//! 1. 选择合适的 `wrap` 获得需要的占空比分辨率
//! 2. 使用 `configure_frequency_with_wrap()` 自动计算分频器
//! 3. 使用 `get_frequency()` 回读实际编程后的频率
//! 4. 使用 `CC`（compare）值控制占空比
//! 5. 在同一 slice 上独立控制 A/B 两个通道
//! 6. 切换到 phase-correct 模式获得对称波形
//!
//! ---
//!
//! # 1. 功能概述
//!
//! ## 1.1 PWM 基础
//!
//! PWM (Pulse Width Modulation) 通过改变高电平时间占比来控制平均输出电压:
//!
//! ```text
//!   占空比 = 25%                 占空比 = 75%
//!     ┌──┐      ┌──┐              ┌──────┐  ┌──────┐
//!     │  │      │  │              │      │  │      │
//!   ──┘  └──────┘  └────        ──┘      └──┘      └──
//!     ←T→                         ←──T──→
//!
//!   平均电压 = Vcc × 占空比
//!   25%: 3.3V × 0.25 = 0.825V   75%: 3.3V × 0.75 = 2.475V
//! ```
//!
//! ## 1.2 RP2350 PWM 架构
//!
//! - **8 个 PWM Slice** (RP2350A) 或 **12 个** (RP2350B)
//! - 每个 Slice 有 **A 和 B 两个输出通道**
//! - 16 位计数器 + 16 位 wrap (top) 值
//! - 可选 8.4 位分频器 (整数部分 1~255，小数部分 0~15)
//! - 支持 Phase-Correct 模式 (对称 PWM)
//! - compare 值 (`CC`) 决定占空比：当 `counter < CC` 时输出为高
//!
//! ### PWM 频率计算
//!
//! ```text
//!   普通模式:    f_PWM = f_sys / (DIV × (WRAP + 1))
//!   Phase-Correct: f_PWM = f_sys / (DIV × WRAP × 2)
//!
//!   其中:
//!     f_sys = 系统时钟 (150 MHz)
//!     DIV   = 分频值 (整数 + 小数/16)
//!     WRAP  = 计数器最大值 (0~65535)
//! ```
//!
//! ### PWM 占空比计算
//!
//! ```text
//!   duty ~= CC / (WRAP + 1)
//!
//!   例如 WRAP = 999:
//!   CC = 250  ->  25%
//!   CC = 500  ->  50%
//!   CC = 750  ->  75%
//!   CC = 1000 -> 100%
//! ```
//!
//! > 注意：如果需要“真正的”100% 占空比，compare 值需要能表示 `WRAP + 1`。
//! > 因此通常不要把 `WRAP` 设为 `0xffff`。
//!
//! ### GPIO → PWM Slice 映射 (RP2350A)
//!
//! | Slice | Channel A (偶数 GPIO) | Channel B (奇数 GPIO) |
//! |-------|-----------------------|-----------------------|
//! | 0 | GP0, GP16 | GP1, GP17 |
//! | 1 | GP2, GP18 | GP3, GP19 |
//! | 2 | GP4, GP20 | GP5, GP21 |
//! | 3 | GP6, GP22 | GP7, GP23 |
//! | 4 | GP8, GP24 | GP9, GP25 |
//! | 5 | GP10, GP26 | GP11, GP27 |
//! | 6 | GP12, GP28 | GP13, GP29 |
//! | 7 | GP14 | GP15 |
//!
//! > 规则: `slice = (gpio / 2) % 8`, `channel = gpio % 2` (0=A, 1=B)
//!
//! ## 1.3 本示例演示内容
//!
//! 1. **LED 呼吸灯**: 1 kHz，1000 级亮度分辨率
//! 2. **舵机控制**: 50 Hz，1~2ms 脉宽对应 0°~180°
//! 3. **双通道输出**: 同一 Slice 的 A/B 通道共享频率、独立 duty
//! 4. **Phase-Correct 模式**: 自动按相位修正公式配置频率
//! 5. **输出反转**: 在保持频率和 compare 值不变时翻转极性
//!
//! ---
//!
//! # 2. 硬件接线
//!
//! ## 2.1 LED 呼吸灯 (Demo 1)
//!
//! ```text
//!   GP0 ──── 330Ω ──── LED(+) ──── GND
//!                       (阳极)    (阴极)
//! ```
//!
//! 也可以直接用板载 LED (GP25, Slice 4 Channel B):
//! ```text
//!   GP25 ──── 板载 LED ──── (已内置在 Pico 板上)
//! ```
//!
//! ## 2.2 舵机控制 (Demo 2)
//!
//! ```text
//!   GP2 ─────────── 信号线 (橙/白)
//!   5V  ─────────── 电源线 (红)      ← 使用外部 5V 供电!
//!   GND ─────────── 地线   (棕/黑)
//! ```
//!
//! > **重要**: 舵机电流较大 (500mA~2A)，不要从 Pico 的 VBUS 直接供电，
//! > 应使用独立的 5V 电源，并与 Pico 共地。
//!
//! ## 2.3 双通道 (Demo 3)
//!
//! ```text
//!   GP4 (Slice 2, Ch A) ──── 330Ω ──── LED1 ──── GND
//!   GP5 (Slice 2, Ch B) ──── 330Ω ──── LED2 ──── GND
//! ```
//!
//! ---
//!
//! # 3. 常用 PWM 频率参考
//!
//! | 应用场景 | 频率 | 建议 WRAP | 说明 |
//! |---------|------|-----------|------|
//! | LED 调光 | 1 kHz | 999 | 1000 级 duty 分辨率 |
//! | 舵机控制 | 50 Hz | 65217 | 20ms 周期，便于 1~2ms 脉宽换算 |
//! | 电机 PWM | 20 kHz | 7499 | `DIV = 1` 时正好整除 |
//! | 4 线风扇 | 25 kHz | 5999 | `DIV = 1` 时正好整除 |
//! | 音频 / DAC | 44.1 kHz | 依据分辨率需求选取 | 频率优先时可减小 WRAP |
//!
//! ---
//!
//! # 4. 编译运行
//!
//! ```bash
//! cargo run --bin pwm_output --target thumbv8m.main-none-eabihf
//! ```

#![no_std]
#![no_main]

use defmt::*;
use defmt_rtt as _;

use pico_rs::block::ImageDef;
use pico_rs::clocks::{ClockBuilder};
use pico_rs::entry;
use pico_rs::gpio::Pins;
use pico_rs::pac::Peripherals;
use pico_rs::pwm::Pwm;
use pico_rs::timer::Timer0;

#[unsafe(link_section = ".start_block")]
#[used]
pub static IMAGE_DEF: ImageDef = ImageDef::exe();

// ============================================================================
// 配置参数
// ============================================================================

/// 系统时钟频率 (Hz) — PLL 配置后为 150 MHz
const SYS_CLK_HZ: u32 = 150_000_000;

/// LED 呼吸灯的 PWM wrap 值 (决定分辨率和频率)
/// f_PWM = 150 MHz / (DIV × (WRAP + 1))
/// DIV=150, WRAP=999 → f = 150M / (150 × 1000) = 1 kHz
const LED_WRAP: u16 = 999;

/// 舵机 PWM 参数
/// 舵机要求 50 Hz (20ms 周期), 脉宽 1~2ms 对应 0°~180°
/// 这里选择较大的 WRAP，以保留足够细的脉宽分辨率。
const SERVO_WRAP: u16 = 65217;

/// 舵机脉宽对应的 duty 值
///   1.0ms (0°)  → duty = 65217 × (1.0 / 20.0) = ~3261
///   1.5ms (90°) → duty = 65217 × (1.5 / 20.0) = ~4891
///   2.0ms (180°) → duty = 65217 × (2.0 / 20.0) = ~6522
const SERVO_MIN_DUTY: u16 = 3261; // 0°
const SERVO_MID_DUTY: u16 = 4891; // 90°
const SERVO_MAX_DUTY: u16 = 6522; // 180°

/// 将角度 (0~180) 转换为舵机 duty 值
const fn angle_to_duty(angle: u32) -> u16 {
    let range = (SERVO_MAX_DUTY - SERVO_MIN_DUTY) as u32;
    (SERVO_MIN_DUTY as u32 + range * angle / 180) as u16
}

/// 将百分比转换为 compare 值。
///
/// 这里使用 `(wrap + 1)` 作为基数，因此 100% 对应的 compare 值是 `wrap + 1`。
const fn duty_from_percent(wrap: u16, percent: u32) -> u16 {
    ((((wrap as u32) + 1) * percent) / 100) as u16
}

#[entry]
fn main() -> ! {
    info!("=== PWM Output Example ===");

    // ------------------------------------------------------------------
    // 1. 外设初始化
    // ------------------------------------------------------------------
    let dp = Peripherals::take().unwrap();

    let clocks = ClockBuilder::new().build(dp.CLOCKS, dp.XOSC, dp.PLL_SYS, dp.PLL_USB);
    info!("System clock: {} Hz", clocks.sys_freq());

    let pins = Pins::new(dp.IO_BANK0, dp.PADS_BANK0);
    let timer = Timer0::init(dp.TIMER0);

    // ------------------------------------------------------------------
    // 2. PWM 控制器初始化
    // ------------------------------------------------------------------
    let pwm = Pwm::new(dp.PWM);

    // ======================================================================
    // Demo 1: LED 呼吸灯
    //   GP0 → Slice 0, Channel A
    //   PWM 频率 = 1 kHz, 占空比从 0% 到 100% 渐变
    // ======================================================================
    info!("--- Demo 1: LED Breathing Effect (GP0) ---");

    // 配置 Slice 0 — 仅使用 Channel A (GP0)
    let slice0 = pwm.slice::<0>().output_a(pins.pin::<0>());

    // 使用高层 API 配置频率：
    // - 目标频率: 1 kHz
    // - WRAP=999, 所以一个周期有 1000 个计数步
    let led_actual_hz = slice0.configure_frequency_with_wrap(SYS_CLK_HZ, 1_000, LED_WRAP);
    let (led_div_int, led_div_frac) = slice0.get_div();
    slice0.set_duty_a(0); // 初始占空比 = 0%

    // 使能 PWM 输出
    slice0.enable();

    info!(
        "Slice 0: TOP={}, DIV={} + {}/16, actual={} Hz, 1000 brightness levels",
        LED_WRAP, led_div_int, led_div_frac, led_actual_hz
    );

    // 渐亮渐灭 3 个周期
    for cycle in 0..3u32 {
        info!("[Breath {}] Fade in...", cycle);
        // 渐亮: 0% → 100%
        let mut duty: u16 = 0;
        while duty <= LED_WRAP {
            slice0.set_duty_a(duty);
            timer.delay_us(1000); // 每级停留 1ms → 总 1 秒
            duty += 1;
        }

        info!("[Breath {}] Fade out...", cycle);
        // 渐灭: 100% → 0%
        let mut duty: i32 = LED_WRAP as i32;
        while duty >= 0 {
            slice0.set_duty_a(duty as u16);
            timer.delay_us(1000);
            duty -= 1;
        }
    }

    slice0.disable();
    info!("Demo 1 done");
    timer.delay_ms(500);

    // ======================================================================
    // Demo 2: 舵机控制
    //   GP2 → Slice 1, Channel A
    //   50 Hz PWM, 脉宽 1~2ms → 角度 0°~180°
    // ======================================================================
    info!("--- Demo 2: Servo Control (GP2) ---");

    let slice1 = pwm.slice::<1>().output_a(pins.pin::<2>());

    let servo_actual_hz = slice1.configure_frequency_with_wrap(SYS_CLK_HZ, 50, SERVO_WRAP);
    let (servo_div_int, servo_div_frac) = slice1.get_div();
    slice1.set_duty_a(SERVO_MID_DUTY); // 初始 90°

    slice1.enable();

    info!(
        "Slice 1: TOP={}, DIV={} + {}/16, actual={} Hz (servo)",
        SERVO_WRAP, servo_div_int, servo_div_frac, servo_actual_hz
    );
    info!(
        "Servo pulse window: 1.0ms={}, 1.5ms={}, 2.0ms={} counts",
        SERVO_MIN_DUTY, SERVO_MID_DUTY, SERVO_MAX_DUTY
    );

    // 依次转到 0° → 45° → 90° → 135° → 180° → 90°
    let angles: [u32; 6] = [0, 45, 90, 135, 180, 90];
    for &angle in angles.iter() {
        let duty = angle_to_duty(angle);
        slice1.set_duty_a(duty);
        info!("Servo: {}° (duty={})", angle, duty);
        timer.delay_ms(800); // 等待舵机到位
    }

    // 平滑扫描: 0° → 180° → 0°
    info!("Servo: smooth sweep 0° → 180° → 0°");
    for angle in (0..=180).chain((0..180).rev()) {
        let duty = angle_to_duty(angle);
        slice1.set_duty_a(duty);
        timer.delay_ms(15); // ~6 秒完成一次往返
    }

    slice1.disable();
    info!("Demo 2 done");
    timer.delay_ms(500);

    // ======================================================================
    // Demo 3: 双通道独立控制
    //   GP4 → Slice 2, Channel A
    //   GP5 → Slice 2, Channel B
    //   同一 Slice 的两个通道共享频率，但占空比独立
    // ======================================================================
    info!("--- Demo 3: Dual-Channel Output (GP4 + GP5) ---");

    let slice2 = pwm.slice::<2>().output_ab(pins.pin::<4>(), pins.pin::<5>());

    let dual_actual_hz = slice2.configure_frequency_with_wrap(SYS_CLK_HZ, 1_000, LED_WRAP);

    // Channel A = 25%, Channel B = 75% — 互补效果
    slice2.set_duty_a(duty_from_percent(LED_WRAP, 25));
    slice2.set_duty_b(duty_from_percent(LED_WRAP, 75));
    slice2.enable();

    info!(
        "Slice 2: actual={} Hz, Ch A = 25% (cc={}), Ch B = 75% (cc={})",
        dual_actual_hz,
        duty_from_percent(LED_WRAP, 25),
        duty_from_percent(LED_WRAP, 75)
    );

    // 互补渐变: A 从 0→100%, B 从 100→0%
    info!("Cross-fade: A ramps up, B ramps down");
    for i in 0..=100u32 {
        let duty_a = (LED_WRAP as u32 * i / 100) as u16;
        let duty_b = (LED_WRAP as u32 * (100 - i) / 100) as u16;
        slice2.set_duty(duty_a, duty_b);
        timer.delay_ms(30); // ~3 秒完成
    }

    timer.delay_ms(500);

    // 反向渐变
    info!("Cross-fade: A ramps down, B ramps up");
    for i in 0..=100u32 {
        let duty_a = (LED_WRAP as u32 * (100 - i) / 100) as u16;
        let duty_b = (LED_WRAP as u32 * i / 100) as u16;
        slice2.set_duty(duty_a, duty_b);
        timer.delay_ms(30);
    }

    slice2.disable();
    info!("Demo 3 done");
    timer.delay_ms(500);

    // ======================================================================
    // Demo 4: Phase-Correct 模式
    //   Phase-Correct PWM 计数器先升后降 (三角波), 产生对称波形
    //   适合 H 桥电机驱动，减少开关噪声
    //
    //   普通模式:    __/|__/|__/|  (锯齿波)
    //   Phase-Correct: __/\__/\__/\  (三角波)
    //
    //   频率换为: f = f_sys / (DIV × WRAP × 2)
    // ======================================================================
    info!("--- Demo 4: Phase-Correct Mode (GP0) ---");

    // 重新使用 slice0 (GP0)
    slice0.set_phase_correct(true);
    let pc_actual_hz = slice0.configure_frequency_with_wrap(SYS_CLK_HZ, 500, LED_WRAP);
    slice0.set_duty_a(LED_WRAP / 2); // 50% 占空比
    slice0.enable();

    info!(
        "Phase-correct mode: target=500 Hz, actual={} Hz",
        pc_actual_hz
    );

    // 保持 3 秒展示
    timer.delay_ms(3000);

    // 关闭 phase-correct, 恢复普通模式
    slice0.set_phase_correct(false);
    slice0.disable();
    info!("Demo 4 done");
    timer.delay_ms(500);

    // ======================================================================
    // Demo 5: 输出反转
    //   不改变占空比的情况下反转输出极性
    //   用途: 互补 PWM 驱动 (高侧/低侧 MOSFET)
    // ======================================================================
    info!("--- Demo 5: Output Inversion (GP0) ---");

    let inverted_actual_hz = slice0.configure_frequency_with_wrap(SYS_CLK_HZ, 1_000, LED_WRAP);
    let inverted_cc = duty_from_percent(LED_WRAP, 25);
    slice0.set_duty_a(inverted_cc); // 25% 占空比
    slice0.enable();

    info!(
        "Normal output: actual={} Hz, cc={} (25% duty)",
        inverted_actual_hz, inverted_cc
    );
    timer.delay_ms(2000);

    slice0.set_invert_a(true);
    info!("Inverted output: effective 75% duty (logical 25% inverted)");
    timer.delay_ms(2000);

    slice0.set_invert_a(false);
    slice0.disable();
    info!("Demo 5 done");

    // ======================================================================
    // 无限循环: GP25 板载 LED 呼吸灯
    // ======================================================================
    info!("=== Loop: Onboard LED breathing (GP25) ===");

    // GP25 → Slice 4, Channel B
    let slice4 = pwm
        .slice::<4>()
        .output_ab(pins.pin::<24>(), pins.pin::<25>());

    let loop_actual_hz = slice4.configure_frequency_with_wrap(SYS_CLK_HZ, 1_000, LED_WRAP);
    slice4.enable();

    info!("Slice 4: onboard LED breathing at {} Hz", loop_actual_hz);

    loop {
        // 渐亮
        let mut duty: u16 = 0;
        while duty <= LED_WRAP {
            slice4.set_duty_b(duty);
            timer.delay_us(2000);
            duty += 1;
        }
        // 渐灭
        let mut duty: i32 = LED_WRAP as i32;
        while duty >= 0 {
            slice4.set_duty_b(duty as u16);
            timer.delay_us(2000);
            duty -= 1;
        }
    }
}
