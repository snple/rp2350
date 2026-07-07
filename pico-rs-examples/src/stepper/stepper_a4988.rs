//! # A4988 步进电机控制 — STEP/DIR + MS1/MS2/MS3 微步配置 + 梯形加减速
//!
//! 基于 RP2350 (Raspberry Pi Pico 2) 的**步进电机控制器**，
//! 使用 **A4988** 驱动芯片，通过 **STEP/DIR** 接口驱动电机，
//! 并通过 **MS1/MS2/MS3** GPIO 引脚配置微步细分。
//!
//! ---
//!
//! # 1. 功能概述
//!
//! ## 1.1 A4988 特性
//!
//! - **简单接口**: 仅需 STEP + DIR 两根信号即可控制电机运转
//! - **微步细分**: 通过 MS1/MS2/MS3 三个引脚组合，支持全步、1/2、1/4、1/8、1/16 五种细分
//! - **可调电流**: 模块上通过电位器 (Vref) 设置峰值电流，无需软件配置
//! - **过温保护**: 内置过温关断 (TSD, ~150°C) 和过温预警 (OTW, ~120°C)
//! - **欠压锁定 (UVLO)**: 逻辑电源 (VDD) 低于阈值时自动关断，防止误动作
//! - **交叉恢复时间保护**: 内部 H 桥开关之间有死区时间，防止高/低侧直通短路
//! - **混合衰减方式**: 可选慢衰减(低纹波)或混合衰减(快速电流衰减)，改善微步精度
//! - **低成本**: 相比 TMC2209 等高级驱动，A4988 价格低廉，适合入门和低成本项目
//!
//! ## 1.2 A4988 vs TMC2209
//!
//! | 特性 | A4988 | TMC2209 |
//! |------|-------|---------|
//! | 最大电流 | 2A (峰值) | 2.8A (峰值) |
//! | 微步细分 | 最高 1/16 | 最高 1/256 |
//! | 配置接口 | 硬件引脚 (MS1/MS2/MS3) | UART 软件配置 |
//! | 静音模式 | 无 | StealthChop2 (极静音) |
//! | 堵转检测 | 无 | StallGuard4 |
//! | 电流设置 | 硬件电位器 (Vref) | UART 寄存器 (0-31 档) |
//! | 价格 | ~¥2 | ~¥10 |
//! | 适合场景 | 低成本 / 入门 / 低速轻载 | 静音 / 高精度 / 需要堵转检测 |
//!
//! ## 1.3 加减速控制
//!
//! 采用**梯形加减速**（Trapezoidal Velocity Profile），与 TMC2209 示例相同：
//! ```text
//!   速度
//!     ↑     ┌───────────────────┐
//!     │    ╱│                   │╲
//!     │   ╱ │                   │ ╲
//!     │  ╱  │      匀速区        │  ╲
//!     │ ╱   │  (MAX_SPEED_HZ)   │   ╲
//!     │╱    │                   │    ╲
//!     └─────┴───────────────────┴─────→ 步数
//!       加速      恒速            减速
//! ```
//!
//! - **加速段**: 步进频率从 `START_SPEED_HZ` 线性增加到 `MAX_SPEED_HZ`
//! - **匀速段**: 保持最大步进频率
//! - **减速段**: 步进频率线性减小到起步频率，保证精确停止
//! - 加减速由 **Timer 中断**实现，每一步动态调整间隔时间
//!
//! ---
//!
//! # 2. 硬件接线
//!
//! ## 2.1 STEP/DIR 控制接口
//!
//! | Pico GPIO | 功能 | A4988 引脚 | 说明 |
//! |-----------|------|-----------|------|
//! | GP2 | STEP | STEP | 步进脉冲输出 (上升沿有效) |
//! | GP3 | DIR | DIR | 方向控制 (高/低 = CW/CCW) |
//! | GP4 | ENABLE | ENABLE | 启用控制 (**低有效**) |
//!
//! ## 2.2 微步配置引脚
//!
//! | Pico GPIO | 功能 | A4988 引脚 | 说明 |
//! |-----------|------|-----------|------|
//! | GP5 | MS1 | MS1 | 微步配置位 1 |
//! | GP6 | MS2 | MS2 | 微步配置位 2 |
//! | GP7 | MS3 | MS3 | 微步配置位 3 |
//!
//! ## 2.3 辅助控制引脚
//!
//! | Pico GPIO | 功能 | A4988 引脚 | 说明 |
//! |-----------|------|-----------|------|
//! | GP8 | RESET | RESET | 复位输入 (**低有效**，默认拉高) |
//! | GP9 | SLEEP | SLEEP | 休眠模式 (**低有效**，默认拉高) |
//!
//! ## 2.4 完整接线图
//!
//! ```text
//!   Pico 2 (RP2350)                   A4988 模块
//!   ──────────────                    ──────────
//!   GP2  (STEP)   ─────────────────── STEP     (步进脉冲)
//!   GP3  (DIR)    ─────────────────── DIR      (方向控制)
//!   GP4  (ENABLE) ─────────────────── ENABLE   (启用, 低有效)
//!
//!   GP5  (MS1)    ─────────────────── MS1      ┐
//!   GP6  (MS2)    ─────────────────── MS2      ├ 微步配置
//!   GP7  (MS3)    ─────────────────── MS3      ┘
//!
//!   GP8  (RESET)  ─────────────────── RESET    (复位, 低有效)
//!   GP9  (SLEEP)  ─────────────────── SLEEP    (休眠, 低有效)
//!
//!   3.3V ──────────────────────────── VDD      (逻辑电源 3~5.5V)
//!   GND  ──────────────────────────── GND
//!
//!   外部 8~35V 电源 ──────────────── VMOT     (电机电源)
//!   外部 GND ─────────────────────── GND
//!
//!   ⚠ 建议在 VMOT 和 GND 之间并联 100μF 电解电容，防止电压尖峰损坏芯片
//!
//!   步进电机 (4 线双极, 如 NEMA17):
//!     线圈 A+ ──── 1A
//!     线圈 A- ──── 1B
//!     线圈 B+ ──── 2A
//!     线圈 B- ──── 2B
//! ```
//!
//! ## 2.5 A4988 微步配置表 (MS1/MS2/MS3)
//!
//! A4988 通过 MS1、MS2、MS3 三个引脚的高低电平组合来选择微步模式:
//!
//! | MS1 | MS2 | MS3 | 微步模式 | 每步角度 (1.8°电机) | 说明 |
//! |-----|-----|-----|---------|---------------------|------|
//! | LOW | LOW | LOW | 全步 (1) | 1.8° | 最大扭矩, 振动噪声最大 |
//! | HIGH| LOW | LOW | 1/2 步 (2) | 0.9° | 噪声降低, 中等平滑度 |
//! | LOW | HIGH| LOW | 1/4 步 (4) | 0.45° | 较好平滑度 |
//! | HIGH| HIGH| LOW | 1/8 步 (8) | 0.225° | 推荐的平衡点 |
//! | HIGH| HIGH| HIGH| 1/16 步 (16) | 0.1125°| 最平滑, 但扭矩和速度受限 |
//!
//! > **注意**: 模块板上通常 MS1/MS2/MS3 有内部下拉电阻，悬空 = LOW = 全步模式。
//! > 但最好不要让引脚悬空，应明确设置电平以避免噪声干扰导致误切换。
//!
//! ## 2.6 A4988 电流限制设置
//!
//! A4988 模块上有一个电位器 (小螺丝)，用于设置参考电压 Vref，从而控制最大电流:
//!
//! ```text
//!   I_max = Vref / (8 × Rsense)
//!
//!   对于常见模块 (Rsense = 0.068Ω):
//!     Vref = 0.5V → I_max ≈ 0.92A
//!     Vref = 0.7V → I_max ≈ 1.29A
//!     Vref = 1.0V → I_max ≈ 1.84A
//!     Vref = 1.2V → I_max ≈ 2.2A  (接近 A4988 极限!)
//!
//!   对于另一种常见模块 (Rsense = 0.1Ω):
//!     Vref = 0.5V → I_max ≈ 0.625A
//!     Vref = 0.8V → I_max ≈ 1.0A
//!     Vref = 1.0V → I_max ≈ 1.25A
//! ```
//!
//! > **⚠ 重要**: 在通电前先用万用表测量电位器中心焊盘对 GND 的电压 (即 Vref)，
//! > 确保电流不超过电机额定值。电流过大会导致电机过热甚至烧毁!
//!
//! ## 2.7 RESET 和 SLEEP 引脚说明
//!
//! | 引脚 | 电平 | 状态 | 说明 |
//! |------|------|------|------|
//! | RESET | **LOW** | 复位 | 内部译码器归零，忽略 STEP 输入 |
//! | RESET | HIGH | 正常 | 芯片正常工作 |
//! | SLEEP | **LOW** | 睡眠 | 最低功耗模式 (~几μA)，H 桥关闭 |
//! | SLEEP | HIGH | 唤醒 | 芯片正常工作 |
//!
//! > **技巧**: 如果不需要 RESET 和 SLEEP 功能，可以将两者直接连接到 VDD (3.3V)。
//! > 本示例通过 GPIO 控制它们，演示软件复位和休眠功能。
//! > SLEEP→HIGH 唤醒后需等待 ~1ms 让内部稳压器稳定，才能发 STEP 脉冲。
//!
//! ---
//!
//! # 3. 配置参数说明
//!
//! | 常量 | 默认值 | 说明 |
//! |------|--------|------|
//! | `START_SPEED_HZ` | 100 | 起步频率 (Hz)，梯形加减速的起始速度 |
//! | `MAX_SPEED_HZ` | 2000 | 最大步进频率 (Hz)，梯形匀速区的目标速度 |
//! | `ACCEL_STEPS` | 200 | 加速段步数，数值越大加速越平缓 |
//! | `STEP_PULSE_US` | 5 | STEP 脉冲高电平宽度 (μs)，A4988 要求 ≥1μs |
//! | `MOTOR_STEPS_PER_REV` | 200 | 电机每转全步数 (1.8° 电机 = 200 步/转) |
//! | `MICROSTEPPING` | 16 | 微步细分数 (1/2/4/8/16)，通过 MS1/MS2/MS3 设置 |
//!
//! ---
//!
//! # 4. 演示流程
//!
//! 程序启动后依次执行：
//!
//! 1. **GPIO 初始化** — 配置 STEP/DIR/ENABLE/MS1/MS2/MS3/RESET/SLEEP 引脚
//! 2. **芯片唤醒** — SLEEP 拉高解除休眠，RESET 拉高解除复位
//! 3. **微步配置** — 根据 `MICROSTEPPING` 常量设置 MS1/MS2/MS3 电平
//! 4. **Demo 1** — 梯形加减速正转 1 圈 (MOTOR_STEPS_PER_REV × MICROSTEPPING 步)
//! 5. **Demo 2** — 梯形加减速反转 1 圈
//! 6. **Demo 3** — 运行时动态切换微步细分：全步 → 1/4 步 → 1/16 步
//! 7. **Demo 4** — 休眠/唤醒功能演示
//! 8. **Demo 5** — 连续运转 3 秒
//! 9. **循环** — 无限交替正反转半圈
//!
//! ---
//!
//! # 5. 编译运行
//!
//! ```bash
//! cd pico-rs-examples
//! cargo run --bin stepper_a4988 --target thumbv8m.main-none-eabihf
//! ```

#![no_std]
#![no_main]

use pico_rs_drivers::stepper::{AxisConfig, AxisState, Direction};

use defmt::*;
use defmt_rtt as _;

use pico_rs::clocks::ClockBuilder;
use pico_rs::gpio::{self, Pins};
use pico_rs::interrupt;
use pico_rs::main;
use pico_rs::pac::{Interrupt, Peripherals};
use pico_rs::ticks::Ticks;
use pico_rs::timer::Timer0; // ============================================================================
// 配置参数
// ============================================================================

/// 起步脉冲频率 (Hz) — 梯形加减速起始速度
/// 步进电机不能从高速直接启动 (会失步)，必须从低速逐渐加速
const START_SPEED_HZ: u32 = 100;

/// 最大脉冲频率 (Hz) — 梯形加减速最高速度
/// A4988 最高步进频率取决于电源电压和负载，典型值在 1~4kHz (微步模式下)
const MAX_SPEED_HZ: u32 = 2000;

/// 加速段总步数 — 从 START_SPEED_HZ 线性加速到 MAX_SPEED_HZ 所需步数
/// 值越大加速越平缓，但到达目标速度的距离更长
const ACCEL_STEPS: u32 = 200;

/// STEP 脉冲高电平宽度 (μs)
/// A4988 数据手册要求 STEP 脉冲最小宽度 ≥1μs，此处取 5μs 留足余量
/// 过短的脉冲可能被芯片忽略; 过长会限制最大步进频率
const STEP_PULSE_US: u32 = 5;

/// 电机每转全步数 (1.8° 步距角 = 200 全步/转)
/// 常见步进电机:
///   - 1.8° (200 步/转) — 最常见 (NEMA17, NEMA23)
///   - 0.9° (400 步/转) — 高分辨率型号
const MOTOR_STEPS_PER_REV: u32 = 200;

/// 微步细分倍数 — 通过 MS1/MS2/MS3 引脚电平设置
/// A4988 支持: 1 (全步), 2 (半步), 4 (1/4步), 8 (1/8步), 16 (1/16步)
/// 微步越高: 运动越平滑、噪声越低，但可用扭矩下降、最大速度降低
const MICROSTEPPING: u32 = 16;

// ============================================================================
// GPIO 引脚分配
// ============================================================================

/// GP2 → A4988 STEP 引脚 (步进脉冲)
/// 每个上升沿驱动电机前进一个微步
const PIN_STEP: u32 = 2;

/// GP3 → A4988 DIR 引脚 (方向控制)
/// HIGH = 一个方向 (例如顺时针), LOW = 反方向
/// 实际旋转方向取决于电机接线极性
const PIN_DIR: u32 = 3;

/// GP4 → A4988 ENABLE 引脚 (启用控制, **低有效**)
/// LOW = FET 输出启用 (电机通电有保持力矩)
/// HIGH = FET 输出关闭 (电机可自由旋转, 无保持力矩)
const PIN_ENABLE: u32 = 4;

/// GP5 → A4988 MS1 引脚 (微步配置位 1)
const PIN_MS1: u32 = 5;

/// GP6 → A4988 MS2 引脚 (微步配置位 2)
const PIN_MS2: u32 = 6;

/// GP7 → A4988 MS3 引脚 (微步配置位 3)
const PIN_MS3: u32 = 7;

/// GP8 → A4988 RESET 引脚 (复位, **低有效**)
/// LOW = 复位芯片 (内部译码器归位, 忽略 STEP)
/// HIGH = 正常运行
const PIN_RESET: u32 = 8;

/// GP9 → A4988 SLEEP 引脚 (休眠, **低有效**)
/// LOW = 睡眠模式 (最低功耗, H 桥关闭)
/// HIGH = 正常运行 (唤醒后需等待 ~1ms)
const PIN_SLEEP: u32 = 9;

// ============================================================================
// 全局共享状态
// ============================================================================

/// 步进轴状态
static AXIS: AxisState = AxisState::new(AxisConfig {
    pin_step: PIN_STEP,
    pin_dir: PIN_DIR,
    alarm: 0,
    start_speed_hz: START_SPEED_HZ,
    accel_steps: ACCEL_STEPS,
    step_pulse_us: STEP_PULSE_US,
    fwd_level: false,
});

// ============================================================================
// A4988 微步配置
// ============================================================================

/// 根据微步倍数设置 MS1/MS2/MS3 引脚电平
///
/// A4988 微步配置真值表:
/// ```text
///   MS1  MS2  MS3  │ 微步模式
///   ───────────────┼──────────
///    L    L    L   │ 全步 (1)
///    H    L    L   │ 1/2 步
///    L    H    L   │ 1/4 步
///    H    H    L   │ 1/8 步
///    H    H    H   │ 1/16 步
/// ```
///
/// # 参数
/// - `microsteps`: 微步倍数 (1, 2, 4, 8, 16)
fn set_microstepping(microsteps: u32) {
    // 根据微步倍数确定 MS1/MS2/MS3 的电平
    let (ms1, ms2, ms3) = match microsteps {
        1 => (false, false, false), // 全步:  L L L
        2 => (true, false, false),  // 1/2 步: H L L
        4 => (false, true, false),  // 1/4 步: L H L
        8 => (true, true, false),   // 1/8 步: H H L
        16 => (true, true, true),   // 1/16步: H H H
        _ => {
            warn!("A4988 不支持 1/{} 微步，使用全步模式", microsteps);
            (false, false, false) // 全步
        }
    };

    // 设置 MS1 (GP5)
    unsafe { gpio::set(PIN_MS1, ms1) };

    // 设置 MS2 (GP6)
    unsafe { gpio::set(PIN_MS2, ms2) };

    // 设置 MS3 (GP7)
    unsafe { gpio::set(PIN_MS3, ms3) };

    info!(
        "A4988 microstepping: 1/{} (MS1={} MS2={} MS3={})",
        microsteps, ms1 as u8, ms2 as u8, ms3 as u8
    );
}

/// 启用 A4988 驱动器 (ENABLE 拉低)
///
/// ENABLE 为低有效:
/// - LOW  = 输出 FET 启用，电机通电 (有保持力矩)
/// - HIGH = 输出 FET 关闭，电机可自由转动 (无保持力矩)
fn enable_driver() {
    unsafe { gpio::set_low(PIN_ENABLE) };
    info!("A4988 enabled (ENABLE=LOW)");
}

/// 禁用 A4988 驱动器 (ENABLE 拉高)
///
/// 电机失去保持力矩，可自由旋转。
/// 适用于不需要定位保持的场景 (例如手动调节)。
#[allow(dead_code)]
fn disable_driver() {
    unsafe { gpio::set_high(PIN_ENABLE) };
    info!("A4988 disabled (ENABLE=HIGH)");
}

/// A4988 复位
///
/// RESET 为低有效:
/// - 拉低 RESET 将内部译码器归位到 Home 状态
/// - 此时 STEP 输入被忽略
/// - 拉高后恢复正常工作
///
/// 使用场景: 微步位置可能错乱时，通过复位恢复已知状态
#[allow(dead_code)]
fn reset_driver() {
    // 拉低 RESET，保持一段时间
    unsafe { gpio::set_low(PIN_RESET) };

    // 等待至少 1μs (数据手册要求)
    // 使用 spin_loop 自旋等待
    for _ in 0..500 {
        core::hint::spin_loop();
    }

    // 释放 RESET
    unsafe { gpio::set_high(PIN_RESET) };
    info!("A4988 reset complete");
}

/// 进入 A4988 休眠模式
///
/// SLEEP 为低有效:
/// - 拉低 SLEEP 进入最低功耗模式 (~几μA)
/// - H 桥 FET 关闭，电机无保持力矩
/// - 内部稳压器关闭
///
/// 使用场景: 长时间不运转时降低功耗和发热
fn sleep_driver() {
    unsafe { gpio::set_low(PIN_SLEEP) };
    info!("A4988 entering sleep mode (SLEEP=LOW)");
}

/// 唤醒 A4988 (退出休眠)
///
/// 唤醒后内部稳压器需要 ~1ms 才能稳定输出电压，
/// 在此期间不应发送 STEP 脉冲。
fn wake_driver() {
    unsafe { gpio::set_high(PIN_SLEEP) };
    info!("A4988 awake (SLEEP=HIGH), waiting 1ms for regulator");
}

// ============================================================================
// 运动控制辅助函数 — 封装对 AXIS 的调用
// ============================================================================

/// 启动梯形加减速运动
fn start_trapezoidal_move(steps: u32, dir: u32) {
    let direction = if dir == 0 {
        Direction::Forward
    } else {
        Direction::Reverse
    };
    let timer = unsafe { Timer0::steal() };
    AXIS.start_move(&timer, steps, direction, MAX_SPEED_HZ);
}

/// 启动连续运转模式
fn start_continuous_run(dir: u32, speed_hz: u32) {
    let direction = if dir == 0 {
        Direction::Forward
    } else {
        Direction::Reverse
    };
    let timer = unsafe { Timer0::steal() };
    AXIS.start_continuous(&timer, direction, speed_hz);
}

/// 立即停止运动
fn stop_motion() {
    AXIS.stop();
}

/// 阻塞等待运动完成
fn wait_motion_done() {
    AXIS.wait_done();
}

// ============================================================================
// 程序入口
// ============================================================================

#[main]
fn main() -> ! {
    info!("Stepper Motor — A4988 STEP/DIR Control");

    // ------------------------------------------------------------------
    // 1. 外设初始化
    // ------------------------------------------------------------------
    let dp = Peripherals::take().unwrap();

    // 配置系统时钟: 150MHz sys_clk, 48MHz USB
    let clocks = ClockBuilder::new().build(dp.CLOCKS, dp.XOSC, dp.PLL_SYS, dp.PLL_USB);
    info!("Clock system initialized: sys={} Hz", clocks.sys_freq());

    // Initialize TIMER0 tick generator (1µs ticks from clk_ref)
    let ticks = Ticks::new(dp.TICKS, &clocks);
    ticks.init_timer0();

    // Initialize the hardware timer
    let timer = Timer0::init(dp.TIMER0);
    info!("Timer0 initialized");

    let pins = Pins::new(dp.IO_BANK0, dp.PADS_BANK0);

    // ------------------------------------------------------------------
    // 2. GPIO 配置 — STEP / DIR / ENABLE
    // ------------------------------------------------------------------

    // GP2 = STEP (步进脉冲输出)
    let p2 = pins.pin::<2>();
    p2.into_output();
    p2.set_low();

    // GP3 = DIR (方向控制)
    let p3 = pins.pin::<3>();
    p3.into_output();
    p3.set_low();

    // GP4 = ENABLE (启用控制, 低有效)
    // 初始化时先禁用驱动器 (HIGH)，待配置完成后再启用
    let p4 = pins.pin::<4>();
    p4.into_output();
    p4.set_high(); // 先禁用

    info!("GPIO: GP2(STEP) GP3(DIR) GP4(ENABLE)");

    // ------------------------------------------------------------------
    // 3. GPIO 配置 — MS1 / MS2 / MS3 (微步选择)
    // ------------------------------------------------------------------

    // GP5 = MS1
    let p5 = pins.pin::<5>();
    p5.into_output();
    p5.set_low();

    // GP6 = MS2
    let p6 = pins.pin::<6>();
    p6.into_output();
    p6.set_low();

    // GP7 = MS3
    let p7 = pins.pin::<7>();
    p7.into_output();
    p7.set_low();

    info!("GPIO: GP5(MS1) GP6(MS2) GP7(MS3)");

    // ------------------------------------------------------------------
    // 4. GPIO 配置 — RESET / SLEEP (辅助控制)
    // ------------------------------------------------------------------

    // GP8 = RESET (复位, 低有效)
    // 初始设为 LOW (复位状态)，稍后释放
    let p8 = pins.pin::<8>();
    p8.into_output();
    p8.set_low();

    // GP9 = SLEEP (休眠, 低有效)
    // 初始设为 LOW (休眠状态)，稍后唤醒
    let p9 = pins.pin::<9>();
    p9.into_output();
    p9.set_low();

    info!("GPIO: GP8(RESET) GP9(SLEEP)");

    // ------------------------------------------------------------------
    // 5. 芯片上电序列
    // ------------------------------------------------------------------
    // A4988 推荐上电顺序:
    //   1) VDD 和 VMOT 上电
    //   2) SLEEP = HIGH (唤醒)
    //   3) 等待 ~1ms (内部稳压器稳定)
    //   4) RESET = HIGH (正常工作)
    //   5) 设置 MSx 和 DIR
    //   6) ENABLE = LOW (开始工作)

    info!("=== A4988 power-up sequence ===");

    // 唤醒: SLEEP 拉高
    wake_driver();
    timer.delay_ms(2); // 等待内部稳压器稳定 (数据手册: ~1ms)

    // 解除复位: RESET 拉高
    unsafe { gpio::set_high(PIN_RESET) };
    info!("A4988 RESET released (HIGH)");
    timer.delay_ms(1);

    // ------------------------------------------------------------------
    // 6. 配置微步细分
    // ------------------------------------------------------------------
    info!("=== Configuring microstepping ===");
    set_microstepping(MICROSTEPPING);
    timer.delay_ms(1);

    // 每转总微步数 = 全步数 × 微步倍数
    let steps_per_rev = MOTOR_STEPS_PER_REV * MICROSTEPPING;
    info!(
        "Steps per revolution: {} ({} full steps x {} microsteps)",
        steps_per_rev, MOTOR_STEPS_PER_REV, MICROSTEPPING
    );

    // ------------------------------------------------------------------
    // 7. Timer0 中断配置 — 用于 STEP 脉冲生成
    // ------------------------------------------------------------------
    // 启用 Timer0 Alarm0 中断 (NVIC)
    unsafe {
        interrupt::enable(Interrupt::TIMER0_IRQ_0);
    }

    // 延时辅助闭包
    let delay_ms = |ms: u32| {
        timer.delay_ms(ms);
    };

    // ------------------------------------------------------------------
    // 8. 启用 A4988 驱动器
    // ------------------------------------------------------------------
    enable_driver();
    delay_ms(100);

    // ======================================================================
    // 演示 1: 梯形加减速正转 1 圈
    // ======================================================================
    info!(
        "=== Demo 1: Forward {} steps (1 rev @ 1/{} microstepping) ===",
        steps_per_rev, MICROSTEPPING
    );

    start_trapezoidal_move(steps_per_rev, 0);
    wait_motion_done();

    info!("Demo 1 done. Position: {} steps", AXIS.position());
    delay_ms(1000);

    // ======================================================================
    // 演示 2: 梯形加减速反转 1 圈
    // ======================================================================
    info!("=== Demo 2: Reverse {} steps ===", steps_per_rev);

    start_trapezoidal_move(steps_per_rev, 1);
    wait_motion_done();

    info!("Demo 2 done. Position: {} steps", AXIS.position());
    delay_ms(1000);

    // ======================================================================
    // 演示 3: 运行时动态切换微步细分
    //
    //   A4988 允许在不运转时更改 MSx 引脚电平来切换微步。
    //   注意: 切换微步后电机内部相位可能改变，建议在电机静止时操作。
    //
    //   流程:
    //   1) 全步模式正转 200 步 (=1 圈)
    //   2) 1/4 步模式正转 200 步 (=1/4 圈, 因为微步变细)
    //   3) 恢复 1/16 步模式反转回原位
    // ======================================================================
    info!("=== Demo 3: Dynamic microstepping change ===");

    // ---- 全步模式 ----
    info!("Switching to full-step mode (1/1)");
    set_microstepping(1);
    delay_ms(10);

    // 全步模式下 200 步 = 1 整圈
    start_trapezoidal_move(200, 0);
    wait_motion_done();
    info!("Full-step 200 steps done. Position: {}", AXIS.position());
    delay_ms(500);

    // ---- 1/4 步模式 ----
    info!("Switching to quarter-step mode (1/4)");
    set_microstepping(4);
    delay_ms(10);

    // 1/4 步模式下 200 步 = 200/(200*4) = 1/4 圈
    start_trapezoidal_move(200, 0);
    wait_motion_done();
    info!("Quarter-step 200 steps done. Position: {}", AXIS.position());
    delay_ms(500);

    // ---- 恢复 1/16 步模式 ----
    info!("Restoring 1/16 microstepping");
    set_microstepping(MICROSTEPPING);
    delay_ms(10);

    // 反转回原点: 之前走了 1 圈 (200 全步) + 1/4 圈 (200 个 1/4 步 = 50 全步)
    // 共 250 全步 = 250 × 16 = 4000 微步
    let return_steps = 250 * MICROSTEPPING;
    info!("Returning to origin: {} microsteps", return_steps);
    start_trapezoidal_move(return_steps, 1);
    wait_motion_done();
    info!("Demo 3 done. Position: {}", AXIS.position());
    delay_ms(1000);

    // ======================================================================
    // 演示 4: 休眠/唤醒功能演示
    //
    //   流程:
    //   1) 先正转半圈
    //   2) 进入休眠 (省电，电机无保持力矩)
    //   3) 等待 2 秒 (模拟空闲期)
    //   4) 唤醒 + 等待稳压器稳定
    //   5) 反转半圈回原位
    // ======================================================================
    info!("=== Demo 4: Sleep/Wake demonstration ===");

    start_trapezoidal_move(steps_per_rev / 2, 0);
    wait_motion_done();
    info!("Moved half revolution. Position: {}", AXIS.position());

    // 进入休眠
    sleep_driver();
    info!("Sleeping for 2 seconds...");
    delay_ms(2000);

    // 唤醒
    wake_driver();
    delay_ms(2); // 等待内部稳压器稳定

    // 反转半圈回原位
    start_trapezoidal_move(steps_per_rev / 2, 1);
    wait_motion_done();
    info!("Demo 4 done. Position: {}", AXIS.position());
    delay_ms(1000);

    // ======================================================================
    // 演示 5: 连续运转 3 秒
    // ======================================================================
    info!("=== Demo 5: Continuous run 3s ===");

    start_continuous_run(0, 1000); // 正转, 1000 Hz

    // 连续运转 3 秒
    delay_ms(3000);

    stop_motion();

    let steps_run = AXIS.steps_done();
    info!(
        "Demo 5 done. Ran {} steps in 3s (~{} Hz). Position: {}",
        steps_run,
        steps_run / 3,
        AXIS.position()
    );
    delay_ms(1000);

    // ======================================================================
    // 无限循环: 交替正反转半圈
    // ======================================================================
    info!("=== Loop: alternating forward/reverse ===");

    AXIS.set_position(0);

    let mut forward = true;
    loop {
        let dir = if forward { 0 } else { 1 };
        let dir_str = if forward { "FWD" } else { "REV" };
        info!("[loop] {} {} steps", dir_str, steps_per_rev / 2);

        start_trapezoidal_move(steps_per_rev / 2, dir);
        wait_motion_done();

        info!("[loop] pos={} steps", AXIS.position());

        delay_ms(500);
        forward = !forward;
    }
}

// ============================================================================
// Timer0 ISR (Interrupt Service Routine) — STEP 脉冲生成 + 梯形加减速
// ============================================================================
//
// 脉冲生成原理 (两段式中断):
//
//   时间轴: ──┬──────┬────────────────────────┬──────┬─────
//             │ ISR1 │                        │ ISR2 │
//             │  ↓   │                        │  ↓   │
//   STEP:  ___/‾‾‾‾‾‾\________________________/‾‾‾‾‾‾\____
//             │←5μs→│←── step_interval_us ──→│
//             │(脉宽)│    (频率决定)          │
//
//   ISR 第 1 次: PULSE_STATE=false → 拉高 STEP，设 alarm = now + STEP_PULSE_US
//   ISR 第 2 次: PULSE_STATE=true  → 拉低 STEP，计步，更新速度，设 alarm = now + interval
//
// A4988 在 STEP 上升沿锁存 DIR 并推进一个微步。
// 下降沿不影响电机状态，所以可以安全地用两段式中断。
//

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
fn TIMER0_IRQ_0() {
    let timer = unsafe { Timer0::steal() };
    AXIS.handle_alarm_isr(&timer);
}
