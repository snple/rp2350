//! # 旋转编码器控制步进电机 — 粗调 / 细调模式
//!
//! 基于 RP2350 (Raspberry Pi Pico 2)，使用 **5 脚增量式旋转编码器开关** (EC11)
//! 控制 **步进电机** (A4988/TMC2209 驱动)，支持**粗调**和**细调**两种模式。
//!
//! ---
//!
//! # 1. 功能概述
//!
//! - **旋转编码器输入**: 四倍频 (4×) 正交解码，读取旋转方向和步数
//! - **按钮切换模式**: 按下编码器按钮在粗调/细调之间切换
//! - **粗调模式 (Coarse)**: 编码器每个脉冲驱动步进电机 **32 微步** (快速定位)
//! - **细调模式 (Fine)**: 编码器每个脉冲驱动步进电机 **1 微步** (精确调整)
//!
//! ## 编码器参数
//!
//! 本示例使用的编码器规格：**30 节点 / 15 脉冲** (每转)
//!
//! - 物理节点 (detent): 30 个/转 → 每个节点有明显的"咔嗒"手感
//! - 电气脉冲 (pulse): 15 个/转 → A/B 信号每转各产生 15 个完整周期
//! - 4× 解码后: 每转 15 × 4 = **60 个计数**
//! - 每个节点 = 60 / 30 = **2 个计数** → 以 2 为单位处理编码器增量
//!
//! ## 步进电机响应
//!
//! | 模式 | 编码器 1 节点 | 步进电机响应 | 适用场景 |
//! |------|-------------|-------------|---------|
//! | 粗调 | 2 counts | 32 微步 (约 0.576°) | 快速移动到大致位置 |
//! | 细调 | 2 counts |  1 微步 (约 0.018°) | 精确微调最终位置 |
//!
//! *(以 1.8° 电机 + 16 微步细分为例: 全步 = 1.8°, 微步 = 1.8°/16 ≈ 0.1125°)*
//!
//! ---
//!
//! # 2. 硬件接线
//!
//! ## 2.1 旋转编码器 (EC11)
//!
//! | Pico GPIO | 编码器引脚 | 说明 |
//! |-----------|-----------|------|
//! | GP10      | A         | 编码器信号 A (内部上拉) |
//! | GP11      | B         | 编码器信号 B (内部上拉) |
//! | GND       | C         | 编码器公共端 (接地) |
//! | GP12      | D         | 按钮引脚 (内部上拉) |
//! | GND       | E         | 按钮公共端 (接地) |
//!
//! ## 2.2 步进电机驱动 (A4988 / TMC2209)
//!
//! | Pico GPIO | 驱动引脚 | 说明 |
//! |-----------|---------|------|
//! | GP2       | STEP    | 步进脉冲 |
//! | GP3       | DIR     | 方向控制 |
//! | GP4       | ENN     | 使能 (低有效) |
//!
//! ```text
//!   Pico 2 (RP2350)               旋转编码器 (EC11)
//!   ──────────────                ──────────────────
//!   GP10 ◄──────────────────────── A (编码器信号 A)
//!   GP11 ◄──────────────────────── B (编码器信号 B)
//!   GND  ──────────────────────── C (编码器公共端)
//!   GP12 ◄──────────────────────── D (按钮)
//!   GND  ──────────────────────── E (按钮公共端)
//!
//!   Pico 2 (RP2350)               步进电机驱动 (A4988/TMC2209)
//!   ──────────────                ──────────────────────────────
//!   GP2 ───────────────────────── STEP
//!   GP3 ───────────────────────── DIR
//!   GP4 ───────────────────────── ENN (低有效, 低电平使能)
//! ```
//!
//! ---
//!
//! # 3. 配置参数
//!
//! | 参数 | 默认值 | 说明 |
//! |------|--------|------|
//! | `ENCODER_COUNTS_PER_DETENT` | 2 | 每个编码器节点对应的 4× 计数数 |
//! | `COARSE_STEPS_PER_DETENT` | 32 | 粗调: 每节点驱动的微步数 |
//! | `FINE_STEPS_PER_DETENT` | 1 | 细调: 每节点驱动的微步数 |
//! | `STEP_SPEED_HZ` | 2000 | 步进电机脉冲频率 (Hz) |
//!
//! ---
//!
//! # 4. 编译运行
//!
//! ```bash
//! cargo run --bin encoder_stepper --target thumbv8m.main-none-eabihf
//! ```

#![no_std]
#![no_main]

#[path = "../stepper/stepper_common.rs"]
mod stepper_common;
use stepper_common::{AxisConfig, AxisState, Direction};

use core::sync::atomic::{AtomicBool, AtomicI32, AtomicU32, Ordering};

use defmt::*;
use defmt_rtt as _;
use panic_probe as _;

use pico_rs::bind;
use pico_rs::block::ImageDef;
use pico_rs::clocks::ClockBuilder;
use pico_rs::entry;
use pico_rs::gpio::{EdgeLevel, Pin, Pins};
use pico_rs::interrupt;
use pico_rs::pac::{Interrupt, Peripherals};
use pico_rs::ticks::Ticks;
use pico_rs::timer::Timer0;

#[unsafe(link_section = ".start_block")]
#[used]
pub static IMAGE_DEF: ImageDef = ImageDef::exe();

// ============================================================================
// 编码器引脚配置
// ============================================================================

/// 编码器信号 A 的 GPIO 编号
const PIN_A: u8 = 10;

/// 编码器信号 B 的 GPIO 编号
const PIN_B: u8 = 11;

/// 按钮引脚的 GPIO 编号 (编码器引脚 D)
const PIN_BTN: u8 = 12;

/// 按钮消抖时间 (微秒)
const DEBOUNCE_US: u32 = 50_000; // 50ms

// ============================================================================
// 步进电机引脚配置
// ============================================================================

/// STEP 引脚 GPIO 编号
const PIN_STEP: u8 = 2;

/// DIR 引脚 GPIO 编号
const PIN_DIR: u8 = 3;

/// ENABLE 引脚 GPIO 编号 (低有效)
const PIN_ENABLE: u8 = 4;

// ============================================================================
// 编码器参数: 30 节点 / 15 脉冲
// ============================================================================

/// 每个编码器节点 (detent) 对应的 4× 解码计数数
///
/// 公式: (4 × pulse_per_rev) / detent_per_rev = (4 × 15) / 30 = 2
const ENCODER_COUNTS_PER_DETENT: i32 = 2;

// ============================================================================
// 步进电机参数
// ============================================================================

/// 粗调模式: 编码器每个节点驱动的微步数
const COARSE_STEPS_PER_DETENT: u32 = 32;

/// 细调模式: 编码器每个节点驱动的微步数
const FINE_STEPS_PER_DETENT: u32 = 1;

/// 步进电机脉冲频率 (Hz)
const STEP_SPEED_HZ: u32 = 2000;

/// 起步频率 (Hz)
const START_SPEED_HZ: u32 = 200;

/// 加速步数
const ACCEL_STEPS: u32 = 100;

/// STEP 脉冲高电平宽度 (μs)
const STEP_PULSE_US: u32 = 5;

// ============================================================================
// 共享状态 (主循环 ↔ ISR)
// ============================================================================

/// 编码器累计位置 (4× 解码原始计数)
static ENCODER_POS: AtomicI32 = AtomicI32::new(0);

/// 上一次 A/B 状态 (bit1 = A, bit0 = B)
static LAST_AB: AtomicI32 = AtomicI32::new(0);

/// 按键按下标志 (ISR → 主循环)
static BUTTON_PRESSED: AtomicBool = AtomicBool::new(false);

/// 上次按钮中断时间戳 (低 32 位, 约 71 分钟溢出, 足够消抖)
static LAST_BTN_TIME: AtomicU32 = AtomicU32::new(0);

/// 步进电机轴状态
static AXIS: AxisState = AxisState::new();

// ============================================================================
// 正交解码查找表
// ============================================================================

/// 状态跳变表: `QUAD_TABLE[old_state << 2 | new_state]`
///
/// 返回值: 0 = 无效/无变化, +1 = CW, -1 = CCW
const QUAD_TABLE: [i32; 16] = [
    0,  // 00 → 00: 无变化
    1,  // 00 → 01: CW
    -1, // 00 → 10: CCW
    0,  // 00 → 11: 无效
    -1, // 01 → 00: CCW
    0,  // 01 → 01: 无变化
    0,  // 01 → 10: 无效
    1,  // 01 → 11: CW
    1,  // 10 → 00: CW
    0,  // 10 → 01: 无效
    0,  // 10 → 10: 无变化
    -1, // 10 → 11: CCW
    0,  // 11 → 00: 无效
    -1, // 11 → 01: CCW
    1,  // 11 → 10: CW
    0,  // 11 → 11: 无变化
];

#[entry]
fn main() -> ! {
    info!("=== Encoder → Stepper Motor Control ===");
    info!("Encoder: 30 detents / 15 pulses per revolution");
    info!("Coarse mode: {} microsteps/detent", COARSE_STEPS_PER_DETENT);
    info!("Fine   mode: {} microsteps/detent", FINE_STEPS_PER_DETENT);

    // ------------------------------------------------------------------
    // 1. 外设初始化
    // ------------------------------------------------------------------
    let dp = Peripherals::take().unwrap();

    let clocks = ClockBuilder::new().build(dp.CLOCKS, dp.XOSC, dp.PLL_SYS, dp.PLL_USB);
    info!("Clock: sys={} Hz", clocks.sys_freq());

    let ticks = Ticks::new(dp.TICKS, &clocks);
    ticks.init_timer0();

    let timer = Timer0::init(dp.TIMER0);
    let pins = Pins::new(dp.IO_BANK0, dp.PADS_BANK0);

    // ------------------------------------------------------------------
    // 2. 编码器引脚配置
    // ------------------------------------------------------------------
    let pin_a = pins.pin::<PIN_A>();
    pin_a.into_input_pullup();

    let pin_b = pins.pin::<PIN_B>();
    pin_b.into_input_pullup();

    let pin_btn = pins.pin::<PIN_BTN>();
    pin_btn.into_input_pullup();

    // 读取初始 A/B 状态
    let init_a = pin_a.get() as i32;
    let init_b = pin_b.get() as i32;
    let init_state = (init_a << 1) | init_b;
    LAST_AB.store(init_state, Ordering::Relaxed);

    // ------------------------------------------------------------------
    // 3. 步进电机引脚配置
    // ------------------------------------------------------------------
    let p_step = pins.pin::<PIN_STEP>();
    p_step.into_output();
    p_step.set_low();

    let p_dir = pins.pin::<PIN_DIR>();
    p_dir.into_output();
    p_dir.set_low();

    let p_en = pins.pin::<PIN_ENABLE>();
    p_en.into_output();
    p_en.set_low(); // 低有效: 使能驱动器

    // ------------------------------------------------------------------
    // 4. 配置步进轴 (stepper_common)
    // ------------------------------------------------------------------
    AXIS.configure(&AxisConfig {
        pin_step: PIN_STEP,
        pin_dir: PIN_DIR,
        alarm: 0,
        start_speed_hz: START_SPEED_HZ,
        accel_steps: ACCEL_STEPS,
        step_pulse_us: STEP_PULSE_US,
    });

    // ------------------------------------------------------------------
    // 5. 使能中断
    // ------------------------------------------------------------------

    // 编码器 A/B: 双沿中断 (4× 解码)
    pin_a.enable_interrupt(EdgeLevel::EdgeHigh);
    pin_a.enable_interrupt(EdgeLevel::EdgeLow);
    pin_b.enable_interrupt(EdgeLevel::EdgeHigh);
    pin_b.enable_interrupt(EdgeLevel::EdgeLow);

    // 按钮: 下降沿 (按下触发)
    pin_btn.enable_interrupt(EdgeLevel::EdgeLow);

    unsafe {
        // GPIO 中断
        interrupt::enable(Interrupt::IO_IRQ_BANK0);
        // Timer0 alarm 0 中断 (步进脉冲)
        interrupt::enable(Interrupt::TIMER0_IRQ_0);
    }

    info!("Interrupts enabled. Rotate encoder to move stepper.");
    info!("Press button to toggle Coarse/Fine mode.");

    // ------------------------------------------------------------------
    // 6. 主循环 — 处理编码器增量, 驱动步进电机
    // ------------------------------------------------------------------
    let mut is_coarse = true; // 当前模式: true=粗调, false=细调
    let mut last_encoder_pos: i32 = 0;
    let mut stepper_position: i32 = 0;
    let mut print_count: u32 = 0;

    info!("[Mode] COARSE (press button to switch)");

    loop {
        timer.delay_ms(20); // 50Hz 轮询

        // --- 检查模式切换 ---
        if BUTTON_PRESSED.swap(false, Ordering::Relaxed) {
            is_coarse = !is_coarse;
            if is_coarse {
                info!("[Mode] COARSE ({} steps/detent)", COARSE_STEPS_PER_DETENT);
            } else {
                info!("[Mode] FINE ({} steps/detent)", FINE_STEPS_PER_DETENT);
            }
        }

        // --- 读取编码器增量 ---
        let encoder_pos = ENCODER_POS.load(Ordering::Relaxed);
        let encoder_delta = encoder_pos - last_encoder_pos;

        // 只处理完整节点 (每节点 = ENCODER_COUNTS_PER_DETENT 个计数)
        let detents = encoder_delta / ENCODER_COUNTS_PER_DETENT;
        if detents == 0 {
            continue;
        }

        // 消耗掉已处理的计数 (保留余数)
        last_encoder_pos += detents * ENCODER_COUNTS_PER_DETENT;

        // --- 计算步进电机目标步数 ---
        let steps_per_detent = if is_coarse {
            COARSE_STEPS_PER_DETENT
        } else {
            FINE_STEPS_PER_DETENT
        };
        let motor_steps = (detents.unsigned_abs()) * steps_per_detent;
        let dir = if detents > 0 {
            Direction::Forward
        } else {
            Direction::Reverse
        };

        // 更新步进电机位置跟踪
        if detents > 0 {
            stepper_position += motor_steps as i32;
        } else {
            stepper_position -= motor_steps as i32;
        }

        // --- 等待上一次运动完成后再启动新运动 ---
        if !AXIS.is_done() {
            AXIS.wait_done();
        }

        // 启动步进电机运动
        AXIS.start_move(&timer, motor_steps, dir, STEP_SPEED_HZ);

        let mode_str = if is_coarse { "C" } else { "F" };
        info!(
            "[#{}][{}] Encoder detents: {}, Motor steps: {}, Position: {}",
            print_count, mode_str, detents, motor_steps, stepper_position
        );
        print_count += 1;
    }
}

// ============================================================================
// GPIO 中断处理 — 编码器 + 按钮
// ============================================================================

#[bind(IO_IRQ_BANK0)]
fn encoder_button_isr() {
    let a = unsafe { Pin::<PIN_A>::steal() };
    let b = unsafe { Pin::<PIN_B>::steal() };
    let btn = unsafe { Pin::<PIN_BTN>::steal() };

    // --- 编码器 A/B 中断处理 ---
    let a_fired = a.check_and_clear(EdgeLevel::EdgeHigh) | a.check_and_clear(EdgeLevel::EdgeLow);
    let b_fired = b.check_and_clear(EdgeLevel::EdgeHigh) | b.check_and_clear(EdgeLevel::EdgeLow);

    if a_fired || b_fired {
        let cur_a = a.get() as i32;
        let cur_b = b.get() as i32;
        let new_state = (cur_a << 1) | cur_b;
        let old_state = LAST_AB.swap(new_state, Ordering::Relaxed);

        let idx = ((old_state << 2) | new_state) as usize & 0x0F;
        let direction = QUAD_TABLE[idx];

        if direction != 0 {
            ENCODER_POS.fetch_add(direction, Ordering::Relaxed);
        }
    }

    // --- 按钮中断处理 (消抖) ---
    if btn.check_and_clear(EdgeLevel::EdgeLow) {
        let now = {
            let timer = unsafe { pico_rs::pac::TIMER0::steal() };
            timer.timerawl.timerawl()
        };

        let last = LAST_BTN_TIME.load(Ordering::Relaxed);
        if now.wrapping_sub(last) >= DEBOUNCE_US {
            LAST_BTN_TIME.store(now, Ordering::Relaxed);
            BUTTON_PRESSED.store(true, Ordering::Relaxed);
        }
    }
}

// ============================================================================
// Timer0 ISR — 步进电机脉冲生成
// ============================================================================

#[bind(TIMER0_IRQ_0)]
fn stepper_pulse_isr() {
    let timer = unsafe { Timer0::steal() };
    AXIS.handle_alarm_isr(&timer);
}
