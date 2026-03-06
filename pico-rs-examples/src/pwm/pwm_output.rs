//! # PWM 输出示例 — LED 呼吸灯 + 舵机控制
//!
//! 基于 RP2350 (Raspberry Pi Pico 2) 的 **PWM 输出**示例，
//! 演示如何使用 PWM 产生不同频率和占空比的波形。
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
//! 1. **LED 呼吸灯**: PWM 驱动 LED 渐亮渐灭 (GP0, Slice 0 Channel A)
//! 2. **舵机控制**: 50 Hz PWM, 1~2ms 脉宽对应 0°~180° (GP2, Slice 1 Channel A)
//! 3. **双通道输出**: 同一 Slice 的 A/B 通道独立设置占空比 (GP4 + GP5)
//! 4. **Phase-Correct 模式**: 对称 PWM, 适合电机驱动 H 桥
//! 5. **频率调整**: 运行时动态修改 PWM 频率和占空比
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
//! | 应用场景 | 频率 | WRAP | DIV |
//! |---------|------|------|-----|
//! | LED 调光 | 1 kHz | 999 | 150 |
//! | 舵机控制 | 50 Hz | 65535 | ~46 |
//! | 蜂鸣器 | 440 Hz~4 kHz | 根据音调 | - |
//! | 电机 PWM | 20 kHz | 7499 | 1 |
//! | 音频 DAC | 44.1 kHz | 3400 | 1 |
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
use panic_probe as _;

use pico_rs::block::ImageDef;
use pico_rs::clocks::ClockBuilder;
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
const LED_DIV: u8 = 150;

/// 舵机 PWM 参数
/// 舵机要求 50 Hz (20ms 周期), 脉宽 1~2ms 对应 0°~180°
/// f_PWM = 150 MHz / (DIV × (WRAP + 1))
/// DIV=46, WRAP=65217 → f ≈ 50.0 Hz (周期 ~20ms)
const SERVO_WRAP: u16 = 65217;
const SERVO_DIV: u8 = 46;

/// 舵机脉宽对应的 duty 值
///   1.0ms (0°)  → duty = 65217 × (1.0 / 20.0) = ~3261
///   1.5ms (90°) → duty = 65217 × (1.5 / 20.0) = ~4891
///   2.0ms (180°) → duty = 65217 × (2.0 / 20.0) = ~6522
const SERVO_MIN_DUTY: u16 = 3261;  // 0°
const SERVO_MID_DUTY: u16 = 4891;  // 90°
const SERVO_MAX_DUTY: u16 = 6522;  // 180°

/// 将角度 (0~180) 转换为舵机 duty 值
const fn angle_to_duty(angle: u32) -> u16 {
    let range = (SERVO_MAX_DUTY - SERVO_MIN_DUTY) as u32;
    (SERVO_MIN_DUTY as u32 + range * angle / 180) as u16
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
    let slice0 = pwm.slice::<0>().output_a(pins.pin::<0>(), SYS_CLK_HZ);

    // 设置分频和 wrap
    slice0.set_div_int(LED_DIV);    // 整数分频 = 150
    slice0.set_div_frac(0);         // 无小数分频
    slice0.set_wrap(LED_WRAP);      // 计数器 top = 999 → 0~999 共 1000 级
    slice0.set_duty_a(0);           // 初始占空比 = 0%

    // 使能 PWM 输出
    slice0.enable();

    let freq = SYS_CLK_HZ / (LED_DIV as u32 * (LED_WRAP as u32 + 1));
    info!(
        "Slice 0: DIV={}, WRAP={}, freq={} Hz, 1000 brightness levels",
        LED_DIV, LED_WRAP, freq
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

    let slice1 = pwm.slice::<1>().output_a(pins.pin::<2>(), SYS_CLK_HZ);

    slice1.set_div_int(SERVO_DIV);
    slice1.set_div_frac(0);
    slice1.set_wrap(SERVO_WRAP);
    slice1.set_duty_a(SERVO_MID_DUTY); // 初始 90°

    slice1.enable();

    let servo_freq = SYS_CLK_HZ / (SERVO_DIV as u32 * (SERVO_WRAP as u32 + 1));
    info!(
        "Slice 1: DIV={}, WRAP={}, freq={} Hz (servo)",
        SERVO_DIV, SERVO_WRAP, servo_freq
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

    let slice2 = pwm
        .slice::<2>()
        .output_ab(pins.pin::<4>(), pins.pin::<5>(), SYS_CLK_HZ);

    slice2.set_div_int(LED_DIV);
    slice2.set_div_frac(0);
    slice2.set_wrap(LED_WRAP);

    // Channel A = 25%, Channel B = 75% — 互补效果
    slice2.set_duty_a(LED_WRAP / 4);
    slice2.set_duty_b(LED_WRAP * 3 / 4);
    slice2.enable();

    info!(
        "Slice 2: Ch A = 25% (duty={}), Ch B = 75% (duty={})",
        LED_WRAP / 4,
        LED_WRAP * 3 / 4
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
    slice0.set_div_int(LED_DIV);
    slice0.set_wrap(LED_WRAP);
    slice0.set_duty_a(LED_WRAP / 2); // 50% 占空比
    slice0.enable();

    let pc_freq = SYS_CLK_HZ / (LED_DIV as u32 * LED_WRAP as u32 * 2);
    info!(
        "Phase-correct mode: freq={} Hz (half of normal mode)",
        pc_freq
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

    slice0.set_wrap(LED_WRAP);
    slice0.set_div_int(LED_DIV);
    slice0.set_duty_a(LED_WRAP / 4); // 25% 占空比
    slice0.enable();

    info!("Normal output: 25% duty");
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
    let slice4 = pwm.slice::<4>().output_ab(pins.pin::<24>(), pins.pin::<25>(), SYS_CLK_HZ);

    slice4.set_div_int(LED_DIV);
    slice4.set_div_frac(0);
    slice4.set_wrap(LED_WRAP);
    slice4.enable();

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
