//! # FOC 磁场定向控制 — 霍尔传感器 + 角度插值 + SVPWM
//!
//! 基于 RP2350 (Raspberry Pi Pico 2) 的**磁场定向控制（FOC）**无刷电机驱动。
//! 使用 3 个霍尔传感器提供粗角度（60° 分辨率），配合时间插值获得连续电角度估计，
//! 通过 Clarke/Park 变换和空间矢量调制（SVPWM）产生平滑正弦波驱动。
//!
//! 其他驱动模式见：
//! - `bldc.rs` — 开环 6 步梯形换向
//! - `bldc_speed_ctrl.rs` — 无传感器 BEMF 闭环
//! - `bldc_hall.rs` — 霍尔 6 步梯形换向 + PI 转速控制
//!
//! ---
//!
//! # 1. FOC vs 梯形换向
//!
//! | 特性 | 6 步梯形 | FOC |
//! |------|---------|-----|
//! | 电流波形 | 方波/梯形 | 正弦波 |
//! | 力矩脉动 | 大（每步切换时有冲击） | 极小（连续平滑） |
//! | 效率 | ~85% | >95% |
//! | 噪声 | 高（可闻的换向噪声） | 极低（几乎静音） |
//! | 控制精度 | 60° 步进 | 连续角分辨率 |
//! | 低速性能 | 差（力矩波动明显） | 优秀 |
//! | 计算量 | 低 | 较高（三角函数、矩阵变换） |
//! | 实现复杂度 | 简单 | 复杂 |
//!
//! ---
//!
//! # 2. FOC 控制链路
//!
//! 本示例实现**电压模式 FOC**（Voltage-mode FOC），不需要电流传感器：
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────────────┐
//! │                    FOC 电压模式控制链路                          │
//! │                                                                 │
//! │  ┌──────────┐     ┌──────────┐     ┌──────────┐               │
//! │  │ 目标转速  │────▶│ 速度 PI  │────▶│ Uq 电压  │               │
//! │  │ (RPM)    │     │ 控制器   │     │ (力矩轴)  │               │
//! │  └──────────┘     └──────────┘     └────┬─────┘               │
//! │                                         │  Ud = 0              │
//! │  ┌──────────┐                           ▼                      │
//! │  │ 霍尔传感器 │──▶ 角度 θe ──▶ ┌──────────────┐                │
//! │  │ + 时间插值 │              │ 逆 Park 变换  │                │
//! │  └──────────┘              │ dq → αβ       │                │
//! │                            └───────┬──────┘                │
//! │                                    │ Uα, Uβ                 │
//! │  ┌──────────┐                      ▼                        │
//! │  │ 实际转速  │◁────────── ┌──────────────┐                    │
//! │  │ (霍尔计时) │           │   SVPWM      │                    │
//! │  └──────────┘           │ αβ → 3相占空比 │                    │
//! │                         └──┬──┬──┬───┘                    │
//! │                            │  │  │                          │
//! │                           DA DB DC (PWM)                   │
//! │                            │  │  │                          │
//! │                          ┌─┴──┴──┴─┐                       │
//! │                          │ 三相逆变器 │                       │
//! │                          │ (6 FET)  │                       │
//! │                          └────┬─────┘                       │
//! │                               │                              │
//! │                          ┌────┴─────┐                       │
//! │                          │  BLDC 电机 │                       │
//! │                          └──────────┘                       │
//! └─────────────────────────────────────────────────────────────────┘
//! ```
//!
//! **关键简化**：电压模式 FOC 直接控制 Uq（力矩方向电压），不做电流闭环。
//! 这极大降低了硬件要求（无需电流传感器），但无法精确控制力矩。
//! 对于大多数速度控制应用（风扇、泵、小型机器人）已足够。
//!
//! ---
//!
//! # 3. 核心架构总览
//!
//! | 模块 | 实现方式 | 说明 |
//! |------|---------|------|
//! | **角度反馈** | 霍尔 GP6/7/8 + 时间插值 | 60° 粗角度 + 线性插值得连续角度，EMA 平滑周期 |
//! | **逆 Park 变换** | dq → αβ | Ud=0，简化为 Uα=−Uq·sin, Uβ=Uq·cos |
//! | **SVPWM** | 中性点注入法 | αβ → 三相占空比，利用率比 SPWM 高 ~15% |
//! | **三角函数** | 256 点 Q15 正弦表 | `sin_q15()` / `cos_q15()`，Q16 角度输入 |
//! | **速度 PI** | Q10 定点, 100 Hz | RPM 误差 → Uq 调整，带积分限幅 |
//! | **PWM 输出** | Slice 0/1/2 @ 20 kHz | SVPWM 占空比，低侧常导通 |
//! | **FOC 循环** | Timer0 @10 kHz | 角度插值 + 逆 Park + SVPWM |
//! | **中断** | `IO_IRQ_BANK0` + `TIMER0_IRQ_0` | 霍尔更新 @GPIO + FOC tick @10 kHz |
//!
//! ### 与其他驱动模式的对比
//!
//! | 区别点 | 电压模式 FOC（本文件） | 6 步梯形 (`bldc_hall.rs`) | 电流模式 FOC (`bldc_foc_current.rs`) |
//! |--------|----------------------|------------------------|----------------------------|
//! | 电流波形 | 正弦波 | 方波 | 正弦波 |
//! | 控制变量 | Uq（电压） | 占空比 | Iq（电流） |
//! | 数学变换 | 逆 Park + SVPWM | 换向表 | Clarke + Park + 逆 Park + SVPWM |
//! | 力矩精度 | 低 | 最低 | 高 |
//! | 硬件需求 | 3× 霍尔 | 无/霍尔 | 编码器 + 2× 电流传感器 |
//!
//! ---
//!
//! # 4. 数学基础
//!
//! ## 4.1 电角度估计（霍尔 + 时间插值）
//!
//! 霍尔传感器每 60° 电角度产生一次跳变。两次跳变之间，通过线性时间插值
//! 估算连续角度：
//!
//! ```text
//! θe = θ_hall_base + 60° × (t - t_last_hall) / T_hall_period
//! ```
//!
//! 其中 `T_hall_period` 为上一个霍尔周期的时长（EMA 平滑）。
//!
//! ## 4.2 逆 Park 变换（dq → αβ）
//!
//! ```text
//! Uα = Ud × cos(θe) - Uq × sin(θe)
//! Uβ = Ud × sin(θe) + Uq × cos(θe)
//! ```
//!
//! 在电压模式 FOC 中，Ud = 0（不需要磁通控制），所以简化为：
//! ```text
//! Uα = -Uq × sin(θe)
//! Uβ =  Uq × cos(θe)
//! ```
//!
//! ## 4.3 SVPWM（空间矢量脉宽调制）
//!
//! SVPWM 将 αβ 平面电压矢量转换为 3 相占空比，比传统 SPWM 利用率高约 15%。
//!
//! ```text
//! 1. 判断 Uα,Uβ 所在扇区（1~6）
//! 2. 计算相邻基本矢量的作用时间 T1, T2
//! 3. 分配零矢量时间 T0 = T_pwm - T1 - T2
//! 4. 计算三相占空比：
//!    Ta = (T_pwm - T1 - T2) / 2
//!    Tb = Ta + T1
//!    Tc = Tb + T2
//!    (然后根据扇区重新排列 a,b,c)
//! ```
//!
//! ## 4.4 定点三角函数
//!
//! 在 no_std 环境下无浮点库，使用 256 点 Q15 正弦查找表：
//! ```text
//! sin_table[i] = sin(2π × i / 256) × 32767
//! 角度用 0~65535 (Q16) 表示 0°~360°
//! ```
//!
//! ---
//!
//! # 5. 硬件接线
//!
//! ## 5.1 引脚分配
//!
//! | GPIO | 功能 | 外设 | 说明 |
//! |------|------|------|------|
//! | GP0 | Phase A 高侧 | PWM Slice 0 ChA | 20 kHz SVPWM |
//! | GP1 | Phase A 低侧 | GPIO SIO 输出 | 常开（低侧互补） |
//! | GP2 | Phase B 高侧 | PWM Slice 1 ChA | 20 kHz SVPWM |
//! | GP3 | Phase B 低侧 | GPIO SIO 输出 | 常开 |
//! | GP4 | Phase C 高侧 | PWM Slice 2 ChA | 20 kHz SVPWM |
//! | GP5 | Phase C 低侧 | GPIO SIO 输出 | 常开 |
//! | GP6 | Hall A (HA) | GPIO 输入 + 中断 | 上拉，双边沿 |
//! | GP7 | Hall B (HB) | GPIO 输入 + 中断 | 上拉，双边沿 |
//! | GP8 | Hall C (HC) | GPIO 输入 + 中断 | 上拉，双边沿 |
//!
//! > **注意：** FOC 使用 SVPWM 生成正弦调制，低侧不再做 6 步开关，
//! > 而是保持常开（全部拉高），由高侧 PWM 完成调制。
//! > 如果使用集成栅极驱动（如 DRV8301 带硬件死区），可改为互补模式。
//!
//! ## 5.2 硬件要求
//!
//! 与 `bldc_hall.rs` 相同的功率级和霍尔接线。详见该文件文档。
//!
//! ---
//!
//! # 6. 参数说明
//!
//! | 参数 | 默认值 | 说明 |
//! |------|--------|------|
//! | `TARGET_RPM` | 3,000 | 目标机械转速 |
//! | `MOTOR_POLES` | 14 | 电机磁极数 |
//! | `UQ_MAX` | 全占空比的 95% | Uq 上限 |
//! | `FOC_LOOP_HZ` | 10,000 | FOC 主循环频率 (10 kHz) |
//! | `KP_SPEED` / `KI_SPEED` | Q10 定点 | 速度 PI 增益 |
//!
//! ---
//!
//! # 7. 与电流模式 FOC 的区别
//!
//! 本示例是**电压模式**（无电流传感器）：
//!
//! | | 电压模式 | 电流模式 |
//! |---|---------|---------|
//! | 输入 | 速度误差 → Uq | 速度误差 → Iq_ref → Id/Iq PI → Ud/Uq |
//! | 硬件 | 霍尔 + 逆变器 | 霍尔 + 逆变器 + 2个电流传感器 + ADC |
//! | 力矩控制 | 间接（电压≈力矩） | 精确（电流∝力矩） |
//! | 适用 | 速度控制 | 力矩控制、伺服 |
//!
//! 升级到电流模式需添加：相电流 ADC 采样 → Clarke → Park → Id/Iq PI。
//!
//! ---
//!
//! # 8. 常见问题
//!
//! **Q: 电机运行但有明显振动**
//! - 霍尔角度偏移不对 → 调整 `HALL_ANGLE_OFFSET`
//! - 插值过冲 → 检查 `step_period` 是否准确
//!
//! **Q: 电机不转或反转**
//! - 霍尔顺序不对 → 调整 `HALL_TO_ANGLE` 映射
//! - 相序不对 → 交换两相接线
//!
//! **Q: 高速时角度超前/滞后**
//! - 需要超前角补偿 → 增大 `ADVANCE_ANGLE`
//!
//! ---
//!
//! # 9. 编译运行
//!
//! ```bash
//! cd rp2350-examples
//! cargo run --bin bldc_foc --target thumbv8m.main-none-eabihf
//! ```
//!
//! ---
//!
//! # 10. 参考资料
//!
//! - [RP2350 Datasheet](https://datasheets.raspberrypi.com/rp2350/rp2350-datasheet.pdf)
//! - *Field Oriented Control of AC Motors* — Texas Instruments (SPRABQ8)
//! - *STM32 FOC PMSM SDK* — ST Application Notes AN1292, AN4220
//! - *Microchip AN1078* — BEMF + FOC 概述

#![no_std]
#![no_main]

use core::cell::RefCell;
use core::sync::atomic::{AtomicBool, AtomicI32, AtomicU16, AtomicU32, Ordering};

use defmt::*;
use defmt_rtt as _;

use rp2350::clocks::ClockBuilder;
use rp2350::gpio::{EdgeLevel, Pin, Pins};
use rp2350::interrupt::{self, Mutex};
use rp2350::main;
use rp2350::pac::{Interrupt, Peripherals};
use rp2350::pwm::Pwm;
use rp2350::timer::Timer0; // ============================================================================
// 配置参数
// ============================================================================

/// PWM 开关频率 (20 kHz)
const PWM_FREQ_HZ: u32 = 20_000;

/// 目标机械转速 (RPM)
const TARGET_RPM: u32 = 3000;

/// 电机磁极数
const MOTOR_POLES: u32 = 14;

/// FOC 主循环频率 (Hz) — Timer Alarm 中断驱动
/// 10 kHz 足够 20 kHz PWM 周期的每两个周期更新一次
const FOC_LOOP_HZ: u32 = 10_000;
const FOC_LOOP_INTERVAL_US: u32 = 1_000_000 / FOC_LOOP_HZ; // 100 µs

/// 速度 PI 控制器运行分频（相对于 FOC 循环）
/// 100 → 速度环 100 Hz
const SPEED_LOOP_DIVIDER: u32 = 100;

/// Uq 最大值 (Q15, 0~32767 → 0~100% 调制深度)
/// SVPWM 线性范围最大约 32767 × 0.577 ≈ 18919，这里限制到 90%
const UQ_MAX: i32 = 17000;
const UQ_MIN: i32 = 0;

/// 速度 PI 增益 (Q10 定点, ×1024)
///   Kp_speed = 0.3  → 307
///   Ki_speed = 0.02 → 20
const KP_SPEED: i32 = 307;
const KI_SPEED: i32 = 20;
const FP_SHIFT: i32 = 10;

/// 速度积分限幅
const SPEED_INTEGRAL_MAX: i32 = 500_000;

/// 初始 Uq 值 (Q15) — 上电时的力矩电压
const STARTUP_UQ: i32 = 5000;

/// 超前角补偿（电角度，Q16 格式：0~65535 = 0~360°）
/// 高速时 BEMF 使电流滞后，可加超前角补偿。0 = 不补偿
const ADVANCE_ANGLE: u16 = 0;

/// 霍尔角度偏移（微调霍尔安装偏差，Q16 格式）
const HALL_ANGLE_OFFSET: u16 = 0;

/// 最大允许的步进周期 (µs) — 超过此值视为停转
const MAX_STEP_PERIOD_US: u32 = 200_000;

// ============================================================================
// 定点三角函数 — 256 点 Q15 正弦表
// ============================================================================

/// sin(2π × i / 256) × 32767，i = 0..255
/// Q15 格式：32767 = 1.0, -32768 = -1.0
const SIN_TABLE: [i16; 256] = [
    0, 804, 1608, 2410, 3212, 4011, 4808, 5602, 6393, 7179, 7962, 8739, 9512, 10278, 11039, 11793,
    12539, 13279, 14010, 14732, 15446, 16151, 16846, 17530, 18204, 18868, 19519, 20159, 20787,
    21403, 22005, 22594, 23170, 23731, 24279, 24811, 25329, 25832, 26319, 26790, 27245, 27683,
    28105, 28510, 28898, 29268, 29621, 29956, 30273, 30571, 30852, 31113, 31356, 31580, 31785,
    31971, 32137, 32285, 32412, 32521, 32609, 32678, 32728, 32757, 32767, 32757, 32728, 32678,
    32609, 32521, 32412, 32285, 32137, 31971, 31785, 31580, 31356, 31113, 30852, 30571, 30273,
    29956, 29621, 29268, 28898, 28510, 28105, 27683, 27245, 26790, 26319, 25832, 25329, 24811,
    24279, 23731, 23170, 22594, 22005, 21403, 20787, 20159, 19519, 18868, 18204, 17530, 16846,
    16151, 15446, 14732, 14010, 13279, 12539, 11793, 11039, 10278, 9512, 8739, 7962, 7179, 6393,
    5602, 4808, 4011, 3212, 2410, 1608, 804, 0, -804, -1608, -2410, -3212, -4011, -4808, -5602,
    -6393, -7179, -7962, -8739, -9512, -10278, -11039, -11793, -12539, -13279, -14010, -14732,
    -15446, -16151, -16846, -17530, -18204, -18868, -19519, -20159, -20787, -21403, -22005, -22594,
    -23170, -23731, -24279, -24811, -25329, -25832, -26319, -26790, -27245, -27683, -28105, -28510,
    -28898, -29268, -29621, -29956, -30273, -30571, -30852, -31113, -31356, -31580, -31785, -31971,
    -32137, -32285, -32412, -32521, -32609, -32678, -32728, -32757, -32767, -32757, -32728, -32678,
    -32609, -32521, -32412, -32285, -32137, -31971, -31785, -31580, -31356, -31113, -30852, -30571,
    -30273, -29956, -29621, -29268, -28898, -28510, -28105, -27683, -27245, -26790, -26319, -25832,
    -25329, -24811, -24279, -23731, -23170, -22594, -22005, -21403, -20787, -20159, -19519, -18868,
    -18204, -17530, -16846, -16151, -15446, -14732, -14010, -13279, -12539, -11793, -11039, -10278,
    -9512, -8739, -7962, -7179, -6393, -5602, -4808, -4011, -3212, -2410, -1608, -804,
];

/// Q15 正弦函数，角度为 Q16 格式（0~65535 = 0°~360°）
/// 返回 Q15（-32767 ~ 32767）
#[inline]
fn sin_q15(angle_q16: u16) -> i32 {
    // 高 8 位作为表索引（256 个条目覆盖 360°）
    let idx = (angle_q16 >> 8) as usize;
    // 简单查表（可以用线性插值提高精度，但此处足够）
    SIN_TABLE[idx] as i32
}

/// Q15 余弦函数：cos(θ) = sin(θ + 90°)
#[inline]
fn cos_q15(angle_q16: u16) -> i32 {
    sin_q15(angle_q16.wrapping_add(16384)) // +90° = +65536/4 = 16384
}

// ============================================================================
// SVPWM — 空间矢量脉宽调制
// ============================================================================

/// SVPWM 输入：Uα, Uβ (Q15)，pwm_wrap 值
/// 输出：三相占空比 (da, db, dc)，范围 0~pwm_wrap
///
/// 使用中性点注入（min-mid-max 偏移法）实现等效 SVPWM，
/// 比标准扇区判别法更简洁，效果相同。
fn svpwm(u_alpha: i32, u_beta: i32, pwm_wrap: u16) -> (u16, u16, u16) {
    // 逆 Clarke 变换: αβ → abc (Q15)
    //   Ua = Uα
    //   Ub = (-Uα + √3·Uβ) / 2
    //   Uc = (-Uα - √3·Uβ) / 2
    //
    // √3 ≈ 1.732 → Q15: 56756 (1.732 × 32768)
    // 除以 2 在最后做
    const SQRT3_Q15: i32 = 56756;

    let ua = u_alpha;
    let ub = (-u_alpha + ((SQRT3_Q15 * u_beta) >> 15)) / 2;
    let uc = (-u_alpha - ((SQRT3_Q15 * u_beta) >> 15)) / 2;

    // 中性点偏移（SVPWM 等效）：
    // offset = -(max(ua,ub,uc) + min(ua,ub,uc)) / 2
    let v_max = ua.max(ub).max(uc);
    let v_min = ua.min(ub).min(uc);
    let offset = -(v_max + v_min) / 2;

    // 加偏移后归一化到 0~pwm_wrap
    // 信号范围从 [-32767, 32767] 映射到 [0, pwm_wrap]
    let half_wrap = pwm_wrap as i32 / 2;

    let da = ((ua + offset) * half_wrap / 32767 + half_wrap).clamp(0, pwm_wrap as i32) as u16;
    let db = ((ub + offset) * half_wrap / 32767 + half_wrap).clamp(0, pwm_wrap as i32) as u16;
    let dc = ((uc + offset) * half_wrap / 32767 + half_wrap).clamp(0, pwm_wrap as i32) as u16;

    (da, db, dc)
}

// ============================================================================
// 霍尔 → 电角度映射表
// ============================================================================

/// 霍尔 3-bit 编码对应的电角度基准值 (Q16: 0~65535 = 0°~360°)
///
/// 霍尔编码 (HA<<2 | HB<<1 | HC) → 该扇区起始的电角度
///
/// ```text
///  Hall │ 二进制 │ 扇区 │ 起始角度 │  Q16 值
///  ─────┼───────┼─────┼─────────┼────────
///   5   │  101  │  0  │    0°   │    0
///   4   │  100  │  1  │   60°   │  10923
///   6   │  110  │  2  │  120°   │  21845
///   2   │  010  │  3  │  180°   │  32768
///   3   │  011  │  4  │  240°   │  43691
///   1   │  001  │  5  │  300°   │  54613
/// ```
///
/// 注意：索引 0 和 7 为无效状态，映射到 0。
const HALL_TO_ANGLE: [u16; 8] = [
    0,     // 0b000 — 无效
    54613, // 0b001 — 扇区 5 (300°)
    32768, // 0b010 — 扇区 3 (180°)
    43691, // 0b011 — 扇区 4 (240°)
    10923, // 0b100 — 扇区 1 (60°)
    0,     // 0b101 — 扇区 0 (0°)
    21845, // 0b110 — 扇区 2 (120°)
    0,     // 0b111 — 无效
];

/// 霍尔编码是否有效
const HALL_VALID: [bool; 8] = [false, true, true, true, true, true, true, false];

// ============================================================================
// 全局共享状态
// ============================================================================

/// Timer 外设
static TIMER: Mutex<RefCell<Option<Timer0>>> = Mutex::new(RefCell::new(None));

/// 当前霍尔状态 (3-bit)
static HALL_STATE: AtomicU32 = AtomicU32::new(0);

/// 霍尔扇区基准角度 (Q16)
static HALL_BASE_ANGLE: AtomicU16 = AtomicU16::new(0);

/// 上次霍尔跳变时间 (µs)
static LAST_HALL_TIME: AtomicU32 = AtomicU32::new(0);

/// 平滑后的霍尔步进周期 (µs)
static HALL_PERIOD_US: AtomicU32 = AtomicU32::new(MAX_STEP_PERIOD_US);

/// 霍尔跳变计数
static HALL_COUNT: AtomicU32 = AtomicU32::new(0);

/// FOC 循环 tick 标志
static FOC_TICK: AtomicBool = AtomicBool::new(false);

/// 当前 Uq 值 (Q15, 由速度 PI 输出)
static CURRENT_UQ: AtomicI32 = AtomicI32::new(STARTUP_UQ);

/// 实测 RPM
static ACTUAL_RPM: AtomicU32 = AtomicU32::new(0);

// ============================================================================
// 入口
// ============================================================================

#[main]
fn main() -> ! {
    info!("BLDC FOC — Hall sensor + angle interpolation + SVPWM");

    // ------------------------------------------------------------------
    // 外设初始化
    // ------------------------------------------------------------------
    let dp = Peripherals::take().unwrap();
    let clocks = ClockBuilder::new().build(dp.CLOCKS, dp.XOSC, dp.PLL_SYS, dp.PLL_USB);
    let sys_freq = clocks.sys_freq();
    info!("System clock: {} MHz", sys_freq / 1_000_000);

    let pins = Pins::init(dp.IO_BANK0, dp.PADS_BANK0);
    let timer = Timer0::init(dp.TIMER0);

    // ------------------------------------------------------------------
    // PWM 初始化 — 20 kHz，3 个 slice
    // FOC 使用 SVPWM 连续调制，不做 6 步开关切换
    // ------------------------------------------------------------------
    let pwm_wrap: u16 = (sys_freq / PWM_FREQ_HZ) as u16;
    info!("PWM wrap = {} ({} kHz)", pwm_wrap, PWM_FREQ_HZ / 1000);

    let pwm = Pwm::init(dp.PWM);

    // Phase A — slice 0
    let mut slice_a = pwm.slice::<0>().output_a(pins.pin::<0>());
    slice_a.set_wrap(pwm_wrap);
    slice_a.set_duty_a(0);
    slice_a.enable();

    // Phase B — slice 1
    let mut slice_b = pwm.slice::<1>().output_a(pins.pin::<2>());
    slice_b.set_wrap(pwm_wrap);
    slice_b.set_duty_a(0);
    slice_b.enable();

    // Phase C — slice 2
    let mut slice_c = pwm.slice::<2>().output_a(pins.pin::<4>());
    slice_c.set_wrap(pwm_wrap);
    slice_c.set_duty_a(0);
    slice_c.enable();

    // ------------------------------------------------------------------
    // 低侧 GPIO — FOC 模式下全部保持常高（导通）
    // 高侧 PWM 负责全部调制
    // ------------------------------------------------------------------
    let mut al = pins.pin::<1>();
    al.into_output();
    al.set_high(); // 常导通

    let mut bl = pins.pin::<3>();
    bl.into_output();
    bl.set_high(); // 常导通

    let mut cl = pins.pin::<5>();
    cl.into_output();
    cl.set_high(); // 常导通

    // ------------------------------------------------------------------
    // 霍尔传感器
    // ------------------------------------------------------------------
    let mut hall_a = pins.pin::<6>();
    hall_a.into_input_pullup();
    hall_a.enable_interrupt(EdgeLevel::EdgeHigh);
    hall_a.enable_interrupt(EdgeLevel::EdgeLow);

    let mut hall_b = pins.pin::<7>();
    hall_b.into_input_pullup();
    hall_b.enable_interrupt(EdgeLevel::EdgeHigh);
    hall_b.enable_interrupt(EdgeLevel::EdgeLow);

    let mut hall_c = pins.pin::<8>();
    hall_c.into_input_pullup();
    hall_c.enable_interrupt(EdgeLevel::EdgeHigh);
    hall_c.enable_interrupt(EdgeLevel::EdgeLow);

    // 读取初始霍尔状态
    let read_hall = || -> u32 {
        let ha = hall_a.get() as u32;
        let hb = hall_b.get() as u32;
        let hc = hall_c.get() as u32;
        (ha << 2) | (hb << 1) | hc
    };

    let initial_hall = read_hall();
    info!("Initial hall: 0b{:03b}", initial_hall);

    if !HALL_VALID[initial_hall as usize] {
        error!("Invalid hall state 0b{:03b} — check wiring!", initial_hall);
        loop {
            timer.delay_ms(1000);
        }
    }

    let initial_angle = HALL_TO_ANGLE[initial_hall as usize];
    HALL_STATE.store(initial_hall, Ordering::Relaxed);
    HALL_BASE_ANGLE.store(initial_angle, Ordering::Relaxed);

    // 初始换向：直接用初始角度做一次 SVPWM
    {
        let uq = STARTUP_UQ;
        let theta = initial_angle
            .wrapping_add(HALL_ANGLE_OFFSET)
            .wrapping_add(ADVANCE_ANGLE);
        // 逆 Park (Ud=0): Uα = -Uq·sin(θ), Uβ = Uq·cos(θ)
        let u_alpha = -(uq * sin_q15(theta)) >> 15;
        let u_beta = (uq * cos_q15(theta)) >> 15;
        let (da, db, dc) = svpwm(u_alpha, u_beta, pwm_wrap);
        slice_a.set_duty_a(da);
        slice_b.set_duty_a(db);
        slice_c.set_duty_a(dc);
        info!("Initial SVPWM applied: da={}, db={}, dc={}", da, db, dc);
    }

    // ------------------------------------------------------------------
    // Timer0 Alarm — FOC 循环 @10kHz
    // ------------------------------------------------------------------
    {
        let now = timer.time_us_32();
        LAST_HALL_TIME.store(now, Ordering::Relaxed);
        timer.set_alarm(0, now.wrapping_add(FOC_LOOP_INTERVAL_US));
        timer.enable_alarm_interrupt(0);

        interrupt::free(|cs| {
            TIMER.borrow(cs).replace(Some(timer));
        });

        unsafe {
            interrupt::enable(Interrupt::TIMER0_IRQ_0);
        }
    }

    // 启用霍尔 GPIO 中断
    unsafe {
        interrupt::enable(Interrupt::IO_IRQ_BANK0);
    }
    info!(
        "FOC running: {}Hz loop, target {} RPM",
        FOC_LOOP_HZ, TARGET_RPM
    );

    // ------------------------------------------------------------------
    // Timer 访问包装
    // ------------------------------------------------------------------
    let timer_now_us = || -> u32 {
        interrupt::free(|cs| {
            TIMER
                .borrow(cs)
                .borrow()
                .as_ref()
                .map_or(0, |t| t.time_us_32())
        })
    };

    // ------------------------------------------------------------------
    // 主循环 — FOC @10kHz + 速度 PI @100Hz
    // ------------------------------------------------------------------
    let pole_pairs = MOTOR_POLES / 2;
    let mut speed_integral: i32 = 0;
    let mut foc_counter: u32 = 0;
    let mut log_counter: u32 = 0;

    loop {
        // 等待 FOC tick
        if !FOC_TICK.swap(false, Ordering::Relaxed) {
            // 短暂等待，避免空转烧 CPU
            continue;
        }

        foc_counter += 1;

        // ==============================================================
        // 1. 电角度估计（霍尔基准 + 时间插值）
        // ==============================================================
        let now = timer_now_us();
        let base_angle = HALL_BASE_ANGLE.load(Ordering::Relaxed);
        let last_hall = LAST_HALL_TIME.load(Ordering::Relaxed);
        let hall_period = HALL_PERIOD_US.load(Ordering::Relaxed);

        // 从上次霍尔跳变以来经过的时间
        let elapsed = now.wrapping_sub(last_hall);

        // 线性插值：60° = 10923 Q16
        // interpolated_angle = base + 10923 × elapsed / hall_period
        let interp_angle = if hall_period > 0 && elapsed < hall_period {
            let frac = (elapsed as u64 * 10923u64) / hall_period as u64;
            base_angle.wrapping_add(frac as u16)
        } else {
            // 超时或第一次 → 用基准角度
            base_angle
        };

        // 加偏移和超前角
        let theta_e = interp_angle
            .wrapping_add(HALL_ANGLE_OFFSET)
            .wrapping_add(ADVANCE_ANGLE);

        // ==============================================================
        // 2. 逆 Park 变换 (Ud=0, Uq=当前速度 PI 输出)
        // ==============================================================
        let uq = CURRENT_UQ.load(Ordering::Relaxed);

        // Uα = -Uq × sin(θe)
        // Uβ =  Uq × cos(θe)
        let sin_theta = sin_q15(theta_e);
        let cos_theta = cos_q15(theta_e);
        let u_alpha = (-(uq as i64 * sin_theta as i64) >> 15) as i32;
        let u_beta = ((uq as i64 * cos_theta as i64) >> 15) as i32;

        // ==============================================================
        // 3. SVPWM → 三相占空比
        // ==============================================================
        let (da, db, dc) = svpwm(u_alpha, u_beta, pwm_wrap);

        slice_a.set_duty_a(da);
        slice_b.set_duty_a(db);
        slice_c.set_duty_a(dc);

        // ==============================================================
        // 4. 速度 PI 控制器 (每 SPEED_LOOP_DIVIDER 次 FOC 循环执行一次)
        // ==============================================================
        if foc_counter.is_multiple_of(SPEED_LOOP_DIVIDER) {
            // 计算 RPM
            let step_period = HALL_PERIOD_US.load(Ordering::Relaxed);
            let actual_rpm = if step_period > 0 && step_period < MAX_STEP_PERIOD_US {
                let elec_period_us = step_period as u64 * 6;
                if elec_period_us > 0 {
                    (60_000_000u64 / (elec_period_us * pole_pairs as u64)) as u32
                } else {
                    0
                }
            } else {
                0
            };

            // 超时检测
            let since_last = now.wrapping_sub(LAST_HALL_TIME.load(Ordering::Relaxed));
            let actual_rpm = if since_last > MAX_STEP_PERIOD_US {
                0
            } else {
                actual_rpm
            };
            ACTUAL_RPM.store(actual_rpm, Ordering::Relaxed);

            let error = TARGET_RPM as i32 - actual_rpm as i32;

            // P
            let p_term = KP_SPEED * error;

            // I (带限幅)
            speed_integral =
                (speed_integral + KI_SPEED * error).clamp(-SPEED_INTEGRAL_MAX, SPEED_INTEGRAL_MAX);

            // PI 输出 → Uq
            let output = (p_term + speed_integral) >> FP_SHIFT;
            let new_uq = (CURRENT_UQ.load(Ordering::Relaxed) + output).clamp(UQ_MIN, UQ_MAX);
            CURRENT_UQ.store(new_uq, Ordering::Relaxed);

            // 日志 (~1 Hz)
            log_counter += 1;
            if log_counter.is_multiple_of(100) {
                info!(
                    "RPM: {} (target {}), Uq: {}, θe: {}, period: {}us",
                    actual_rpm,
                    TARGET_RPM,
                    new_uq,
                    theta_e as u32 * 360 / 65536,
                    step_period
                );
            }
        }
    }
}

// ============================================================================
// 霍尔 GPIO 中断 — 更新角度基准和步进周期
// ============================================================================

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
fn IO_IRQ_BANK0() {
    let ha = unsafe { Pin::<6>::steal() };
    let hb = unsafe { Pin::<7>::steal() };
    let hc = unsafe { Pin::<8>::steal() };

    // 清除中断
    ha.clear_all_interrupts();
    hb.clear_all_interrupts();
    hc.clear_all_interrupts();

    // 读取霍尔状态
    let hall = ((ha.get() as u32) << 2) | ((hb.get() as u32) << 1) | (hc.get() as u32);
    HALL_STATE.store(hall, Ordering::Relaxed);

    // 更新角度基准
    if HALL_VALID[hall as usize] {
        HALL_BASE_ANGLE.store(HALL_TO_ANGLE[hall as usize], Ordering::Relaxed);
    }

    // 测量步进周期
    interrupt::free(|cs| {
        if let Some(ref timer) = *TIMER.borrow(cs).borrow() {
            let now = timer.time_us_32();
            let last = LAST_HALL_TIME.load(Ordering::Relaxed);
            let period = now.wrapping_sub(last);
            LAST_HALL_TIME.store(now, Ordering::Relaxed);

            // 有效周期才 EMA 平滑
            if period > 50 && period < MAX_STEP_PERIOD_US {
                let old = HALL_PERIOD_US.load(Ordering::Relaxed);
                let smoothed = (old * 3 + period) / 4;
                HALL_PERIOD_US.store(smoothed, Ordering::Relaxed);
            }
        }
    });

    HALL_COUNT.fetch_add(1, Ordering::Relaxed);
}

// ============================================================================
// Timer0 ISR — FOC 循环 tick @10kHz
// ============================================================================

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
fn TIMER0_IRQ_0() {
    interrupt::free(|cs| {
        if let Some(ref timer) = *TIMER.borrow(cs).borrow() {
            timer.clear_alarm_interrupt(0);
            FOC_TICK.store(true, Ordering::Relaxed);
            let now = timer.time_us_32();
            timer.set_alarm(0, now.wrapping_add(FOC_LOOP_INTERVAL_US));
        }
    });
}
