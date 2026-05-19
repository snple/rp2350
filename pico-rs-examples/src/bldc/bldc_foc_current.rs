//! # 电流模式 FOC — 增量式编码器 + 双电流环 + SVPWM
//!
//! 基于 RP2350 (Raspberry Pi Pico 2) 的**电流模式磁场定向控制（FOC）**无刷电机驱动。
//! 使用 PIO 硬件正交解码器读取增量式编码器，双 ADC 采样两相电流，
//! 经 Clarke→Park 变换后运行 Id/Iq 双电流 PI 环 + 外层速度 PI 环，
//! 最后通过逆 Park + SVPWM 输出三相正弦调制。
//!
//! 其他驱动模式见：
//! - `bldc.rs` — 开环 6 步梯形换向
//! - `bldc_speed_ctrl.rs` — 无传感器 BEMF 闭环
//! - `bldc_hall.rs` — 霍尔 6 步梯形换向 + PI 转速控制
//! - `bldc_foc.rs` — 电压模式 FOC（霍尔 + 角度插值 + SVPWM）
//!
//! ---
//!
//! # 1. 电流模式 vs 电压模式 FOC
//!
//! | 特性 | 电压模式（bldc_foc.rs） | 电流模式（本文件） |
//! |------|----------------------|------------------|
//! | 控制变量 | 直接设 Vq（力矩方向电压） | 设 Iq_ref（力矩方向电流） |
//! | 内环 | 无 | Id/Iq 双电流 PI 闭环 |
//! | 力矩精度 | 低（电压≈力矩，受温度/转速影响） | 高（电流∝力矩，精确） |
//! | 响应速度 | 慢（电气时间常数限制） | 快（电流环带宽可达数 kHz） |
//! | 硬件需求 | 霍尔 + 逆变器 | 编码器 + 逆变器 + 2 个电流传感器 |
//! | 适用场景 | 风扇、泵、简单调速 | 伺服、机器人、高精度力矩控制 |
//!
//! ---
//!
//! # 2. 控制链路
//!
//! ```text
//! ┌──────────────────────────────────────────────────────────────────────┐
//! │                 电流模式 FOC 完整控制链路                              │
//! │                                                                      │
//! │  ┌──────────┐     ┌──────────┐     ┌───────────┐                    │
//! │  │ 目标转速  │────▶│ 速度 PI  │────▶│ Iq_ref    │                    │
//! │  │ (RPM)    │     │ 控制器   │     │ (力矩指令) │                    │
//! │  └──────────┘     └──────────┘     └─────┬─────┘                    │
//! │                                          │  Id_ref = 0              │
//! │                                          ▼                          │
//! │  ┌──────────┐     ┌──────────┐     ┌──────────┐                    │
//! │  │ Ia, Ib   │────▶│ Clarke   │────▶│  Park    │────▶ Id, Iq 实际    │
//! │  │ (ADC)    │     │ abc→αβ   │     │  αβ→dq   │                    │
//! │  └──────────┘     └──────────┘     └──────────┘                    │
//! │                                          │                          │
//! │                                          ▼                          │
//! │                                    ┌──────────┐                    │
//! │                                    │ Id PI    │──▶ Ud              │
//! │                                    │ Iq PI    │──▶ Uq              │
//! │                                    └────┬─────┘                    │
//! │                                         │                          │
//! │  ┌──────────┐                           ▼                          │
//! │  │ 编码器   │──▶ θe ───────────▶ ┌──────────────┐                  │
//! │  │ (PIO)    │                   │ 逆 Park 变换  │                  │
//! │  └──────────┘                   │  dq → αβ      │                  │
//! │                                 └───────┬──────┘                  │
//! │                                         │ Uα, Uβ                   │
//! │                                         ▼                          │
//! │                                  ┌──────────────┐                  │
//! │                                  │   SVPWM      │                  │
//! │                                  │  αβ → 3相PWM │                  │
//! │                                  └──┬──┬──┬───┘                  │
//! │                                     │  │  │                        │
//! │                                    DA DB DC                        │
//! │                                     │  │  │                        │
//! │                                  ┌──┴──┴──┴──┐                    │
//! │                                  │  三相逆变器  │                    │
//! │                                  └─────┬──────┘                    │
//! │                                        │                            │
//! │                                   ┌────┴─────┐                     │
//! │                                   │ BLDC 电机  │                     │
//! │                                   └──────────┘                     │
//! └──────────────────────────────────────────────────────────────────────┘
//! ```
//!
//! ---
//!
//! # 3. 核心架构总览
//!
//! | 模块 | 实现方式 | 说明 |
//! |------|---------|------|
//! | **角度反馈** | 增量式编码器 GP10(A) + GP11(B) | GPIO 双边沿中断 + 4 倍频状态转移表解码，精度 ~0.09°（4000 CPR） |
//! | **电流采样** | GP26(Ia) + GP27(Ib) ADC 12-bit | 支持 ACS712 / INA240 等传感器，`adc_to_current_ma()` 换算 |
//! | **Clarke 变换** | abc → αβ | Iα = Ia, Iβ = (Ia + 2·Ib) / √3 |
//! | **Park 变换** | αβ → dq（正向） | 获取 Id / Iq 实际值，用于电流闭环反馈 |
//! | **Id PI 控制器** | Q10 定点 PI | Id_ref = 0（不弱磁），输出 Ud |
//! | **Iq PI 控制器** | Q10 定点 PI | Iq_ref 由速度环提供，输出 Uq |
//! | **速度 PI** | 编码器差分求 RPM | 速度误差 → Iq_ref，100 Hz 采样率 |
//! | **逆 Park 变换** | dq → αβ | Uα = Ud·cos − Uq·sin, Uβ = Ud·sin + Uq·cos |
//! | **SVPWM** | 中性点注入法 | αβ → 三相占空比，利用率比 SPWM 高 ~15% |
//! | **PI 控制器** | 通用 `PiController` 结构体 | Q10 定点运算，带积分限幅 + 输出限幅 |
//! | **中断** | `IO_IRQ_BANK0` + `TIMER0_IRQ_0` | 编码器 4 倍频解码 @GPIO 中断 + FOC tick @10 kHz |
//!
//! ### 与电压模式 `bldc_foc.rs` 的关键区别
//!
//! | 区别点 | 电压模式 (`bldc_foc.rs`) | 电流模式（本文件） |
//! |--------|------------------------|------------------|
//! | 电流反馈 | 无（开环电压设定） | Clarke → Park 正向变换获取 Id/Iq |
//! | 内环控制 | 直接设 Vq | Id/Iq 双 PI 闭环 → Ud/Uq |
//! | 角度来源 | 霍尔 + 时间插值（60° 分辨率） | 编码器（~0.09° 分辨率） |
//! | 力矩控制 | 间接（电压 ≈ 力矩） | 精确（电流 ∝ 力矩） |
//! | 硬件需求 | 3× 霍尔 | 编码器 + 2× 电流传感器 |
//!
//! ---
//!
//! # 4. 数学基础
//!
//! ## 4.1 Clarke 变换 (abc → αβ)
//!
//! 将三相定子电流投影到二维正交坐标系：
//!
//! ```text
//! Iα = Ia
//! Iβ = (2·Ib + Ia) / √3 = (Ia + 2·Ib) × (1/√3)
//! ```
//!
//! 只需采样 Ia, Ib，由 Ic = -(Ia + Ib) 得第三相。
//!
//! ## 4.2 Park 变换 (αβ → dq)
//!
//! 将静止坐标系旋转到与转子磁场同步的 dq 坐标系：
//!
//! ```text
//! Id =  Iα × cos(θe) + Iβ × sin(θe)
//! Iq = -Iα × sin(θe) + Iβ × cos(θe)
//! ```
//!
//! Id (磁通分量)：控制弱磁/增磁，正常运行设 Id_ref = 0
//! Iq (力矩分量)：与电磁力矩成正比，由速度环输出
//!
//! ## 4.3 逆 Park 变换 (dq → αβ)
//!
//! ```text
//! Uα = Ud × cos(θe) - Uq × sin(θe)
//! Uβ = Ud × sin(θe) + Uq × cos(θe)
//! ```
//!
//! ## 4.4 电角度计算（编码器）
//!
//! 增量式编码器输出 A/B 正交信号，GPIO 中断 4 倍频解码后得到累计脉冲计数。
//!
//! ```text
//! θ_mech = (count mod CPR) × 360° / CPR
//! θ_elec = θ_mech × pole_pairs  (mod 360°)
//! ```
//!
//! CPR = Counts Per Revolution（编码器每转计数，4 倍频后）。
//!
//! ## 4.5 SVPWM
//!
//! 与 `bldc_foc.rs` 相同的中性点注入法，详见该文件文档。
//!
//! ---
//!
//! # 5. GPIO 中断 4 倍频正交解码
//!
//! 使用 A/B 通道的双边沿 GPIO 中断实现完整 4 倍频正交解码。
//! 在 `IO_IRQ_BANK0` ISR 中用经典状态转移表判断方向并累加计数。
//!
//! ```text
//! 状态转移表 (prev_AB × curr_AB → 方向):
//!
//!          curr_AB
//!          00   01   11   10
//! prev 00 [ 0 , +1 ,  0 , -1 ]
//! prev 01 [ -1,  0 , +1 ,  0 ]
//! prev 11 [ 0 , -1 ,  0 , +1 ]
//! prev 10 [ +1,  0 , -1 ,  0 ]
//!
//! 索引 = (prev_AB << 2) | curr_AB
//! ENCODER_TABLE[16] 查表 → +1 / -1 / 0
//! ```
//!
//! RP2350 在 150 MHz 主频下，GPIO 中断可轻松处理 >500 kHz 编码器频率。
//! 1000 线编码器 × 4 倍频 × 3000 RPM / 60 = 200 kHz → 完全可跟踪。
//!
//! ---
//!
//! # 6. 硬件接线
//!
//! ## 6.1 引脚分配
//!
//! | GPIO | 功能 | 外设 | 说明 |
//! |------|------|------|------|
//! | GP0 | Phase A 高侧 | PWM Slice 0 ChA | 20 kHz SVPWM |
//! | GP1 | Phase A 低侧 | GPIO SIO 输出 | 常导通 |
//! | GP2 | Phase B 高侧 | PWM Slice 1 ChA | 20 kHz SVPWM |
//! | GP3 | Phase B 低侧 | GPIO SIO 输出 | 常导通 |
//! | GP4 | Phase C 高侧 | PWM Slice 2 ChA | 20 kHz SVPWM |
//! | GP5 | Phase C 低侧 | GPIO SIO 输出 | 常导通 |
//! | GP10 | Encoder A | GPIO 输入 + 中断 | 正交信号 A 路，上拉，双边沿 |
//! | GP11 | Encoder B | GPIO 输入 + 中断 | 正交信号 B 路，上拉，双边沿 |
//! | GP26 | Ia 电流采样 | ADC Ch0 | 相 A 电流传感器（ACS712/INA240 等） |
//! | GP27 | Ib 电流采样 | ADC Ch1 | 相 B 电流传感器 |
//!
//! ## 6.2 电流传感器
//!
//! 电流传感器将相电流转换为 0~3.3V 模拟信号：
//!
//! ```text
//! 典型接线（以 ACS712-5A 为例）：
//!
//!   相电流 ──►┌──────────┐─── Vout ──► GP26 (ADC)
//!            │ ACS712   │
//!   GND ────►└──────────┘
//!
//! 输出特性：Vout = Vcc/2 + I × 感度
//!   0A → 1.65V (中点)
//!   +5A → 3.3V
//!   -5A → 0V
//!
//! ADC 读数换算：
//!   I = (adc_raw - 2048) × 3300mV / 4096 / 感度(mV/A)
//! ```
//!
//! ## 6.3 增量式编码器
//!
//! ```text
//!   编码器 ─── A ──► GP10 (带上拉/外部上拉 3.3V)
//!          ─── B ──► GP11
//!          ─── GND ──► GND
//!          ─── VCC ──► 3.3V 或 5V（根据编码器规格）
//! ```
//!
//! 开漏输出编码器需要外部上拉电阻（4.7kΩ 到 3.3V）。
//! 差分输出编码器需要 RS-422 接收器芯片转单端。
//!
//! ---
//!
//! # 7. 参数说明
//!
//! | 参数 | 默认值 | 说明 |
//! |------|--------|------|
//! | `TARGET_RPM` | 3000 | 目标机械转速 |
//! | `MOTOR_POLES` | 14 | 电机磁极数 |
//! | `ENCODER_CPR` | 4000 | 编码器每转计数（4 倍频后） |
//! | `PWM_FREQ_HZ` | 20,000 | PWM 开关频率 |
//! | `FOC_LOOP_HZ` | 10,000 | FOC 电流环频率 |
//! | `KP_ID/KI_ID` | Q10 定点 | d 轴电流 PI 增益 |
//! | `KP_IQ/KI_IQ` | Q10 定点 | q 轴电流 PI 增益 |
//! | `KP_SPEED/KI_SPEED` | Q10 定点 | 速度 PI 增益 |
//! | `CURRENT_SENSE_MV_PER_A` | 400 | 电流传感器灵敏度 (mV/A) |
//!
//! ---
//!
//! # 8. 电流环调参指南
//!
//! 电流环带宽应远大于速度环（通常 10:1 以上）：
//!
//! ```text
//! 典型带宽关系：
//!   电流环 ~2 kHz (10 kHz 采样 / ~5 倍)
//!   速度环 ~100 Hz (1 kHz 采样 / ~10 倍)
//!
//! 电流 PI 调参步骤：
//!   1. 先设 Ki = 0，只用 Kp
//!   2. Kp 从小到大，直到阶跃响应有 ~5% 超调
//!   3. 缓慢增大 Ki，消除稳态误差
//!   4. 确保电流波纹 < 10% 额定电流
//!
//! 速度 PI 调参步骤：
//!   1. 电流环调好后，锁定电流 PI 参数
//!   2. Kp_speed 从小到大，Ki_speed = 0
//!   3. 增大 Ki_speed 消除稳态速度误差
//! ```
//!
//! ---
//!
//! # 9. 常见问题
//!
//! **Q: 电机发出高频啸叫**
//! - 电流 PI Kp 过大 → 降低 Kp
//! - PWM 频率在可闻范围 → 提高到 20kHz 以上
//!
//! **Q: 电机振荡不稳定**
//! - 速度-电流环带宽比不够 → 增大电流环带宽或降低速度环带宽
//! - 编码器计数方向反了 → 交换 A/B 线或改 `ENCODER_DIR`
//!
//! **Q: 低速力矩不足**
//! - 电流传感器死区 → 校准零偏
//! - Iq_ref 被限幅 → 增大 `IQ_REF_MAX`
//!
//! **Q: 编码器计数跳变/丢步**
//! - 信号质量差 → 加屏蔽线、滤波电容
//! - PIO 时钟分频太大 → 降低 clkdiv
//!
//! ---
//!
//! # 10. 编译运行
//!
//! ```bash
//! cd pico-rs-examples
//! cargo run --bin bldc_foc_current --target thumbv8m.main-none-eabihf
//! ```
//!
//! ---
//!
//! # 11. 参考资料
//!
//! - [RP2350 Datasheet](https://datasheets.raspberrypi.com/rp2350/rp2350-datasheet.pdf)
//! - *Field Oriented Control of AC Motors* — Texas Instruments (SPRABQ8)
//! - *STM32 FOC PMSM SDK* — ST AN1292, AN4220
//! - *Microchip AN1078* — BEMF + FOC
//! - *PIO Quadrature Encoder* — Raspberry Pi Pico examples (quadrature_encoder.pio)

#![no_std]
#![no_main]

use core::cell::RefCell;
use core::sync::atomic::{AtomicBool, AtomicI32, AtomicU32, Ordering};

use defmt::*;
use defmt_rtt as _;
use panic_probe as _;

use pico_rs::adc::{Adc, AdcPin};
use pico_rs::block::ImageDef;
use pico_rs::clocks::ClockBuilder;
use pico_rs::entry;
use pico_rs::gpio::Pins;
use pico_rs::interrupt::{self, Mutex};
use pico_rs::pac::{Interrupt, Peripherals};
use pico_rs::pwm::Pwm;
use pico_rs::timer::Timer0;

#[unsafe(link_section = ".start_block")]
#[used]
pub static IMAGE_DEF: ImageDef = ImageDef::exe();

// ============================================================================
// 配置参数
// ============================================================================

/// PWM 开关频率 (20 kHz)
const PWM_FREQ_HZ: u32 = 20_000;

/// 目标机械转速 (RPM)
const TARGET_RPM: i32 = 3000;

/// 电机磁极数
const MOTOR_POLES: u32 = 14;
const POLE_PAIRS: u32 = MOTOR_POLES / 2;

/// 编码器每转计数(CPR, 4 倍频后)
/// 例如 1000 线编码器 → CPR = 4000
const ENCODER_CPR: u32 = 4000;

/// 编码器方向：true = 默认方向，false = 反向
const ENCODER_DIR: bool = true;

/// FOC 电流环频率 (Hz)
const FOC_LOOP_HZ: u32 = 10_000;
const FOC_LOOP_INTERVAL_US: u32 = 1_000_000 / FOC_LOOP_HZ;

/// 速度环分频（相对于 FOC 循环）
/// 100 → 速度环 100 Hz
const SPEED_LOOP_DIVIDER: u32 = 100;

// ---- 电流 PI 参数 (Q10 定点) ----
/// Id PI (d 轴 — 磁通)
/// 目标 Id = 0，通常 Kp 较大、Ki 适中
const KP_ID: i32 = 512; // ~0.5
const KI_ID: i32 = 51; // ~0.05

/// Iq PI (q 轴 — 力矩)
const KP_IQ: i32 = 512;
const KI_IQ: i32 = 51;

/// 电流积分限幅 (Q10 × mA 单位)
const CURRENT_INTEGRAL_MAX: i32 = 500_000;

/// 电压输出限幅 (Q15)
/// SVPWM 线性范围最大 ~18919
const U_MAX: i32 = 17000;

// ---- 速度 PI 参数 (Q10 定点) ----
const KP_SPEED: i32 = 307; // ~0.3
const KI_SPEED: i32 = 20; // ~0.02
const SPEED_INTEGRAL_MAX: i32 = 500_000;

/// Iq 参考最大值 (mA, Q0)
const IQ_REF_MAX: i32 = 5000;

/// Q10 定点移位
const FP_SHIFT: i32 = 10;

// ---- 电流传感器参数 ----
/// 传感器灵敏度 (mV/A)
/// ACS712-5A: 185mV/A, ACS712-20A: 100mV/A, INA240A1: 200mV/A
const CURRENT_SENSE_MV_PER_A: i32 = 400;

/// ADC 中点（0A 时的 ADC 读数，理想值 2048）
/// 实际硬件需要校准此值
const ADC_ZERO_CURRENT: i32 = 2048;

// ============================================================================
// 定点三角函数 — 256 点 Q15 正弦表
// ============================================================================

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

#[inline]
fn sin_q15(angle_q16: u16) -> i32 {
    SIN_TABLE[(angle_q16 >> 8) as usize] as i32
}

#[inline]
fn cos_q15(angle_q16: u16) -> i32 {
    sin_q15(angle_q16.wrapping_add(16384))
}

// ============================================================================
// SVPWM — 与 bldc_foc.rs 相同的中性点注入法
// ============================================================================

fn svpwm(u_alpha: i32, u_beta: i32, pwm_wrap: u16) -> (u16, u16, u16) {
    const SQRT3_Q15: i32 = 56756; // √3 × 32768

    let ua = u_alpha;
    let ub = (-u_alpha + ((SQRT3_Q15 * u_beta) >> 15)) / 2;
    let uc = (-u_alpha - ((SQRT3_Q15 * u_beta) >> 15)) / 2;

    let v_max = ua.max(ub).max(uc);
    let v_min = ua.min(ub).min(uc);
    let offset = -(v_max + v_min) / 2;

    let half_wrap = pwm_wrap as i32 / 2;

    let da = ((ua + offset) * half_wrap / 32767 + half_wrap).clamp(0, pwm_wrap as i32) as u16;
    let db = ((ub + offset) * half_wrap / 32767 + half_wrap).clamp(0, pwm_wrap as i32) as u16;
    let dc = ((uc + offset) * half_wrap / 32767 + half_wrap).clamp(0, pwm_wrap as i32) as u16;

    (da, db, dc)
}

// ============================================================================
// ADC → 电流换算
// ============================================================================

/// 将 ADC 原始值转换为电流 (mA, 有符号)
///
/// ```text
/// voltage_mv = adc_raw × 3300 / 4096
/// current_mA = (voltage_mv - 1650) × 1000 / sensitivity_mv_per_a
///            = (adc_raw - 2048) × 3300 / 4096 × 1000 / sensitivity
/// ```
#[inline]
fn adc_to_current_ma(adc_raw: u16) -> i32 {
    let diff = adc_raw as i32 - ADC_ZERO_CURRENT;
    // diff × 3300 × 1000 / 4096 / sensitivity
    // = diff × 3_300_000 / 4096 / sensitivity
    // 简化：diff × 806 / sensitivity  (3300000/4096 ≈ 806)
    (diff * 806) / CURRENT_SENSE_MV_PER_A
}

// ============================================================================
// PI 控制器（通用）
// ============================================================================

/// Q10 定点 PI 控制器状态
struct PiController {
    kp: i32,
    ki: i32,
    integral: i32,
    integral_max: i32,
    output_max: i32,
    output_min: i32,
}

impl PiController {
    const fn new(kp: i32, ki: i32, integral_max: i32, output_min: i32, output_max: i32) -> Self {
        Self {
            kp,
            ki,
            integral: 0,
            integral_max,
            output_max,
            output_min,
        }
    }

    /// 执行一步 PI 计算
    #[inline]
    fn update(&mut self, error: i32) -> i32 {
        let p = self.kp * error;
        self.integral =
            (self.integral + self.ki * error).clamp(-self.integral_max, self.integral_max);
        let output = (p + self.integral) >> FP_SHIFT;
        output.clamp(self.output_min, self.output_max)
    }

    /// 复位积分器
    #[allow(dead_code)]
    fn reset(&mut self) {
        self.integral = 0;
    }
}

// ============================================================================
// 全局共享状态
// ============================================================================

static TIMER: Mutex<RefCell<Option<Timer0>>> = Mutex::new(RefCell::new(None));
static ADC: Mutex<RefCell<Option<Adc>>> = Mutex::new(RefCell::new(None));

/// FOC tick 标志
static FOC_TICK: AtomicBool = AtomicBool::new(false);

/// 编码器原始计数（由 PIO 输出，主循环读取）
static ENCODER_COUNT: AtomicI32 = AtomicI32::new(0);

/// 实测 RPM (用于日志)
static ACTUAL_RPM: AtomicI32 = AtomicI32::new(0);

/// 上次采样的Ia, Ib (mA, 用于日志)
static CURRENT_IA_MA: AtomicI32 = AtomicI32::new(0);
static CURRENT_IB_MA: AtomicI32 = AtomicI32::new(0);

/// 当前输出 Ud, Uq (Q15, 用于日志)
static OUTPUT_UD: AtomicI32 = AtomicI32::new(0);
static OUTPUT_UQ: AtomicI32 = AtomicI32::new(0);

// ============================================================================
// 入口
// ============================================================================

#[entry]
fn main() -> ! {
    info!("BLDC FOC Current Mode — Encoder + dual current loop + SVPWM");

    // ------------------------------------------------------------------
    // 外设初始化
    // ------------------------------------------------------------------
    let dp = Peripherals::take().unwrap();
    let clocks = ClockBuilder::new()
        .enable_adc()
        .build(dp.CLOCKS, dp.XOSC, dp.PLL_SYS, dp.PLL_USB);
    let sys_freq = clocks.sys_freq();
    info!("System clock: {} MHz", sys_freq / 1_000_000);

    let pins = Pins::new(dp.IO_BANK0, dp.PADS_BANK0);
    let timer = Timer0::init(dp.TIMER0);

    // ------------------------------------------------------------------
    // PWM — 20 kHz，3 个 slice
    // ------------------------------------------------------------------
    let pwm_wrap: u16 = (sys_freq / PWM_FREQ_HZ) as u16;
    info!("PWM wrap = {} ({} kHz)", pwm_wrap, PWM_FREQ_HZ / 1000);

    let pwm = Pwm::new(dp.PWM);

    let slice_a = pwm.slice::<0>().output_a(pins.pin::<0>());
    slice_a.set_wrap(pwm_wrap);
    slice_a.set_duty_a(0);
    slice_a.enable();

    let slice_b = pwm.slice::<1>().output_a(pins.pin::<2>());
    slice_b.set_wrap(pwm_wrap);
    slice_b.set_duty_a(0);
    slice_b.enable();

    let slice_c = pwm.slice::<2>().output_a(pins.pin::<4>());
    slice_c.set_wrap(pwm_wrap);
    slice_c.set_duty_a(0);
    slice_c.enable();

    // ------------------------------------------------------------------
    // 低侧 GPIO — 常导通
    // ------------------------------------------------------------------
    let al = pins.pin::<1>();
    al.into_output();
    al.set_high();

    let bl = pins.pin::<3>();
    bl.into_output();
    bl.set_high();

    let cl = pins.pin::<5>();
    cl.into_output();
    cl.set_high();

    // ------------------------------------------------------------------
    // ADC — 采样两相电流 (Ia on GP26, Ib on GP27)
    // ------------------------------------------------------------------
    let adc = Adc::new(dp.ADC);
    let adc_ia = pins.pin::<26>().into_adc_input();
    let adc_ib = pins.pin::<27>().into_adc_input();
    info!(
        "ADC initialized: Ia=Ch{}, Ib=Ch{}",
        adc_ia.channel(),
        adc_ib.channel()
    );

    // ------------------------------------------------------------------
    // 编码器 — GPIO 中断 + 软件 4 倍频正交解码
    // ------------------------------------------------------------------
    //
    // 使用 A/B 通道的双边沿中断实现完整 4 倍频解码。
    // 在 ISR 中用经典状态转移表判断方向并累加计数。
    //
    // RP2350 的 150MHz 主频下，GPIO 中断可轻松处理 >500kHz 的编码器频率。
    // 对于 1000 线编码器 + 3000RPM：
    //   最大编码器频率 = 1000 × 4 × 3000/60 = 200 kHz → 完全可跟踪

    let enc_a = pins.pin::<10>();
    enc_a.into_input_pullup();
    enc_a.enable_interrupt(pico_rs::gpio::EdgeLevel::EdgeHigh);
    enc_a.enable_interrupt(pico_rs::gpio::EdgeLevel::EdgeLow);

    let enc_b = pins.pin::<11>();
    enc_b.into_input_pullup();
    // B 通道也启用中断实现 4 倍频
    enc_b.enable_interrupt(pico_rs::gpio::EdgeLevel::EdgeHigh);
    enc_b.enable_interrupt(pico_rs::gpio::EdgeLevel::EdgeLow);

    // 读取初始编码器状态
    let init_a = enc_a.get();
    let init_b = enc_b.get();
    info!("Encoder init: A={}, B={}", init_a, init_b);

    // 存储上次 AB 状态用于 4 倍频解码
    // 编码在 2-bit 中：(A<<1)|B
    let init_ab = ((init_a as u32) << 1) | (init_b as u32);
    ENCODER_LAST_AB.store(init_ab, Ordering::Relaxed);

    // ------------------------------------------------------------------
    // Timer0 — FOC tick @10kHz
    // ------------------------------------------------------------------
    {
        let now = timer.get_time_us_32();
        timer.set_alarm(0, now.wrapping_add(FOC_LOOP_INTERVAL_US));
        timer.enable_alarm_interrupt(0);

        interrupt::free(|cs| {
            TIMER.borrow(cs).replace(Some(timer));
            ADC.borrow(cs).replace(Some(adc));
        });

        unsafe {
            interrupt::enable(Interrupt::TIMER0_IRQ_0);
            interrupt::enable(Interrupt::IO_IRQ_BANK0);
        }
    }

    info!(
        "FOC current mode running: {}Hz loop, target {} RPM",
        FOC_LOOP_HZ, TARGET_RPM
    );

    // ------------------------------------------------------------------
    // PI 控制器实例
    // ------------------------------------------------------------------
    let mut pi_id = PiController::new(KP_ID, KI_ID, CURRENT_INTEGRAL_MAX, -U_MAX, U_MAX);
    let mut pi_iq = PiController::new(KP_IQ, KI_IQ, CURRENT_INTEGRAL_MAX, -U_MAX, U_MAX);
    let mut pi_speed = PiController::new(
        KP_SPEED,
        KI_SPEED,
        SPEED_INTEGRAL_MAX,
        -IQ_REF_MAX,
        IQ_REF_MAX,
    );

    let mut foc_counter: u32 = 0;
    let mut log_counter: u32 = 0;
    let mut last_encoder_count: i32 = 0;

    // Timer 辅助
    let timer_now_us = || -> u32 {
        interrupt::free(|cs| {
            TIMER
                .borrow(cs)
                .borrow()
                .as_ref()
                .map_or(0, |t| t.get_time_us_32())
        })
    };

    let mut last_speed_time: u32 = timer_now_us();

    // ------------------------------------------------------------------
    // 主循环 — FOC 电流环 @10kHz + 速度环 @100Hz
    // ------------------------------------------------------------------
    loop {
        if !FOC_TICK.swap(false, Ordering::Relaxed) {
            continue;
        }

        foc_counter += 1;

        // ==============================================================
        // 1. 采样两相电流 (ADC)
        // ==============================================================
        let (ia_raw, ib_raw) = interrupt::free(|cs| {
            if let Some(ref mut adc) = *ADC.borrow(cs).borrow_mut() {
                let ia = adc.read(&adc_ia);
                let ib = adc.read(&adc_ib);
                (ia, ib)
            } else {
                (ADC_ZERO_CURRENT as u16, ADC_ZERO_CURRENT as u16)
            }
        });

        let ia_ma = adc_to_current_ma(ia_raw);
        let ib_ma = adc_to_current_ma(ib_raw);
        CURRENT_IA_MA.store(ia_ma, Ordering::Relaxed);
        CURRENT_IB_MA.store(ib_ma, Ordering::Relaxed);

        // ==============================================================
        // 2. 编码器 → 电角度 θe
        // ==============================================================
        let enc_count = ENCODER_COUNT.load(Ordering::Relaxed);

        // 编码器计数 → 电角度 (Q16: 0~65535 = 0~360°)
        // θ_mech = (count mod CPR) × 65536 / CPR
        // θ_elec = θ_mech × pole_pairs (mod 65536)
        let count_in_rev = enc_count.rem_euclid(ENCODER_CPR as i32) as u32;
        let theta_mech = ((count_in_rev as u64 * 65536u64) / ENCODER_CPR as u64) as u16;
        let theta_e = theta_mech.wrapping_mul(POLE_PAIRS as u16);

        let sin_theta = sin_q15(theta_e);
        let cos_theta = cos_q15(theta_e);

        // ==============================================================
        // 3. Clarke 变换: Ia, Ib → Iα, Iβ (整数 mA)
        // ==============================================================
        // Iα = Ia
        // Iβ = (Ia + 2·Ib) / √3
        //    = (Ia + 2·Ib) × 37837 >> 16   (1/√3 ≈ 0.5774 → Q16: 37837)
        let i_alpha = ia_ma;
        let i_beta = (((ia_ma as i64 + 2 * ib_ma as i64) * 37837i64) >> 16) as i32;

        // ==============================================================
        // 4. Park 变换: Iα, Iβ → Id, Iq (整数 mA)
        // ==============================================================
        // Id =  Iα·cos(θe) + Iβ·sin(θe)
        // Iq = -Iα·sin(θe) + Iβ·cos(θe)
        let id =
            ((i_alpha as i64 * cos_theta as i64 + i_beta as i64 * sin_theta as i64) >> 15) as i32;
        let iq =
            ((-i_alpha as i64 * sin_theta as i64 + i_beta as i64 * cos_theta as i64) >> 15) as i32;

        // ==============================================================
        // 5. 电流 PI 控制器
        // ==============================================================
        // Id_ref = 0 (不做弱磁)
        let id_error = 0 - id;
        let ud = pi_id.update(id_error);

        // Iq_ref 由速度环提供（在速度环更新时写入）
        let iq_ref = CURRENT_IQ_REF.load(Ordering::Relaxed);
        let iq_error = iq_ref - iq;
        let uq = pi_iq.update(iq_error);

        OUTPUT_UD.store(ud, Ordering::Relaxed);
        OUTPUT_UQ.store(uq, Ordering::Relaxed);

        // ==============================================================
        // 6. 逆 Park + SVPWM
        // ==============================================================
        let u_alpha = ((ud as i64 * cos_theta as i64 - uq as i64 * sin_theta as i64) >> 15) as i32;
        let u_beta = ((ud as i64 * sin_theta as i64 + uq as i64 * cos_theta as i64) >> 15) as i32;

        let (da, db, dc) = svpwm(u_alpha, u_beta, pwm_wrap);

        slice_a.set_duty_a(da);
        slice_b.set_duty_a(db);
        slice_c.set_duty_a(dc);

        // ==============================================================
        // 7. 速度 PI (每 SPEED_LOOP_DIVIDER 次执行一次)
        // ==============================================================
        if foc_counter % SPEED_LOOP_DIVIDER == 0 {
            let now = timer_now_us();
            let dt_us = now.wrapping_sub(last_speed_time);
            last_speed_time = now;

            // 编码器差分 → RPM
            let delta_count = enc_count - last_encoder_count;
            last_encoder_count = enc_count;

            // RPM = delta_count / CPR × 60 / (dt_us / 1e6)
            //     = delta_count × 60_000_000 / (CPR × dt_us)
            let actual_rpm = if dt_us > 0 {
                (delta_count as i64 * 60_000_000i64 / (ENCODER_CPR as i64 * dt_us as i64)) as i32
            } else {
                0
            };
            ACTUAL_RPM.store(actual_rpm, Ordering::Relaxed);

            // 速度 PI → Iq_ref
            let speed_error = TARGET_RPM - actual_rpm;
            let iq_ref_new = pi_speed.update(speed_error);
            CURRENT_IQ_REF.store(iq_ref_new, Ordering::Relaxed);

            // 日志 (~1 Hz)
            log_counter += 1;
            if log_counter % 100 == 0 {
                info!(
                    "RPM:{} target:{} Iq_ref:{} Id:{} Iq:{} Ud:{} Uq:{} enc:{}",
                    actual_rpm, TARGET_RPM, iq_ref_new, id, iq, ud, uq, enc_count
                );
            }
        }
    }
}

// ============================================================================
// 编码器全局状态
// ============================================================================

/// 编码器上次 AB 状态 (2-bit: A<<1 | B)
static ENCODER_LAST_AB: AtomicU32 = AtomicU32::new(0);

/// Iq 参考值 (mA, 由速度环输出)
static CURRENT_IQ_REF: AtomicI32 = AtomicI32::new(0);

// ============================================================================
// 编码器 4 倍频解码表
// ============================================================================

/// 正交解码状态转移表
///
/// 索引 = (prev_AB << 2) | curr_AB
/// 值: +1 = 正转, -1 = 反转, 0 = 无变化或无效
///
/// ```text
///          curr_AB
///          00  01  11  10
/// prev 00 [ 0, +1,  0, -1]
/// prev 01 [-1,  0, +1,  0]
/// prev 11 [ 0, -1,  0, +1]
/// prev 10 [+1,  0, -1,  0]
/// ```
const ENCODER_TABLE: [i32; 16] = [
    0, 1, 0, -1, // prev=00
    -1, 0, 1, 0, // prev=01
    0, -1, 0, 1, // prev=11
    1, 0, -1, 0, // prev=10
];

// ============================================================================
// GPIO 中断 — 编码器 4 倍频解码
// ============================================================================

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
fn IO_IRQ_BANK0() {
    // 清除所有编码器引脚中断
    let pin_a = unsafe { pico_rs::gpio::Pin::<10>::steal() };
    let pin_b = unsafe { pico_rs::gpio::Pin::<11>::steal() };
    pin_a.clear_all_interrupts();
    pin_b.clear_all_interrupts();

    // 读取当前 AB 状态
    let a = pin_a.get() as u32;
    let b = pin_b.get() as u32;
    let curr_ab = (a << 1) | b;

    // 查表获取方向
    let prev_ab = ENCODER_LAST_AB.swap(curr_ab, Ordering::Relaxed);
    let idx = ((prev_ab << 2) | curr_ab) as usize;
    let dir = ENCODER_TABLE[idx & 0xF];

    // 更新位置计数
    if dir != 0 {
        let step = if ENCODER_DIR { dir } else { -dir };
        ENCODER_COUNT.fetch_add(step, Ordering::Relaxed);
    }
}

// ============================================================================
// Timer0 ISR — FOC tick @10kHz
// ============================================================================

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
fn TIMER0_IRQ_0() {
    interrupt::free(|cs| {
        if let Some(ref timer) = *TIMER.borrow(cs).borrow() {
            timer.clear_alarm_interrupt(0);
            FOC_TICK.store(true, Ordering::Relaxed);
            let now = timer.get_time_us_32();
            timer.set_alarm(0, now.wrapping_add(FOC_LOOP_INTERVAL_US));
        }
    });
}
