//! # 无刷电机伺服 — 编码器 + 位置/速度/电压三环 FOC
//!
//! 基于 RP2350 (Raspberry Pi Pico 2) 的**无刷电机伺服控制器**。
//! 用普通三相无刷电机 + 增量式编码器实现类似于伺服电机（或大功率舵机）的
//! **精确位置控制**。
//!
//! 控制层级为经典三环级联：
//! **位置环（PID）→ 速度环（PI）→ 电压模式 FOC（逆 Park + SVPWM）**
//!
//! 其他驱动模式见：
//! - `bldc.rs` — 开环 6 步梯形换向
//! - `bldc_speed_ctrl.rs` — 无传感器 BEMF 闭环
//! - `bldc_hall.rs` — 霍尔 6 步 + PI 转速控制
//! - `bldc_foc.rs` — 电压模式 FOC（霍尔，速度控制）
//! - `bldc_foc_current.rs` — 电流模式 FOC（编码器，速度控制）
//!
//! ---
//!
//! # 1. 无刷伺服 vs 传统舵机
//!
//! | 特性 | 传统 PWM 舵机 | 无刷伺服（本方案） |
//! |------|-------------|------------------|
//! | 驱动方式 | 内置 H 桥 + 电位器 | FOC + 编码器 |
//! | 控制信号 | 1~2 ms PPM 脉宽 | 编码器绝对位置（可扩展 PPM） |
//! | 角度精度 | ~1°（受电位器和齿轮限制） | ~0.09°（1000 线编码器 4 倍频） |
//! | 力矩特性 | 非线性、随角度变化 | 线性（FOC 正弦驱动） |
//! | 保持力矩 | 依赖减速齿轮（有齿隙） | 电磁直接保持（零齿隙） |
//! | 速度范围 | 受限（~0.1s/60°） | 宽范围（可做高速伺服） |
//! | 功率密度 | 低 | 高（无刷效率 >90%） |
//! | 成本 | 低 | 较高（编码器 + 驱动板） |
//!
//! ---
//!
//! # 2. 控制链路
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────────────────────┐
//! │              无刷伺服三环级联控制链路                                      │
//! │                                                                         │
//! │  ┌───────────┐     ┌──────────┐     ┌──────────┐     ┌──────────┐     │
//! │  │ 目标位置   │────▶│ 位置 PID │────▶│ 速度 PI  │────▶│  Uq 电压 │     │
//! │  │ (编码器脉冲│     │ @1 kHz   │     │ @1 kHz   │     │ (力矩轴)  │     │
//! │  │  或角度°)  │     └──────────┘     └──────────┘     └────┬─────┘     │
//! │  └───────────┘                                            │ Ud = 0    │
//! │                                                           ▼            │
//! │  ┌───────────┐                                    ┌──────────────┐    │
//! │  │ 编码器     │──▶ 位置/速度 ──▶ θe ──────────▶ │ 逆 Park 变换  │    │
//! │  │ A/B GPIO   │                                  │  dq → αβ      │    │
//! │  │ 4 倍频解码 │                                  └───────┬──────┘    │
//! │  └───────────┘                                          │ Uα, Uβ     │
//! │                                                         ▼            │
//! │                                                  ┌──────────────┐    │
//! │                                                  │   SVPWM      │    │
//! │                                                  │  αβ → 3相PWM │    │
//! │                                                  └──┬──┬──┬───┘    │
//! │                                                     │  │  │        │
//! │                                                    DA DB DC        │
//! │                                                     │  │  │        │
//! │                                                  ┌──┴──┴──┴──┐    │
//! │                                                  │  三相逆变器  │    │
//! │                                                  └─────┬──────┘    │
//! │                                                        │            │
//! │                                                   ┌────┴─────┐     │
//! │                                                   │ BLDC 电机  │     │
//! │                                                   └──────────┘     │
//! └─────────────────────────────────────────────────────────────────────────┘
//! ```
//!
//! **三环带宽分配原则：**
//! ```text
//! 位置环 ~100 Hz  ← 最外层，带宽最低
//! 速度环 ~500 Hz  ← 中间层
//! FOC 环 ~10 kHz  ← 最内层，带宽最高
//! ```
//!
//! ---
//!
//! # 3. 核心架构总览
//!
//! | 模块 | 实现方式 | 说明 |
//! |------|---------|------|
//! | **角度反馈** | 增量式编码器 GP10(A) + GP11(B) | GPIO 双边沿中断 + 4 倍频状态转移表解码 |
//! | **位置 PID** | Q10 定点 PID, @1 kHz | 位置误差 → 速度指令，带 D 项抑制超调振荡 |
//! | **速度 PI** | Q10 定点 PI, @1 kHz | 编码器差分求 RPM，输出 Uq |
//! | **逆 Park** | dq → αβ | Ud=0，Uα = −Uq·sin(θe), Uβ = Uq·cos(θe) |
//! | **SVPWM** | 中性点注入法 | αβ → 三相占空比 |
//! | **三角函数** | 256 点 Q15 正弦表 | `sin_q15()` / `cos_q15()` |
//! | **位置指令** | 编码器脉冲计数 / 角度° | 主循环可切换多段位置 |
//! | **保持模式** | 位置环持续工作 | 到达目标后零速保持，抗扰恢复 |
//! | **中断** | `IO_IRQ_BANK0` + `TIMER0_IRQ_0` | 编码器 4 倍频 @GPIO + FOC tick @10 kHz |
//!
//! ### 与电流模式 FOC (`bldc_foc_current.rs`) 的关键区别
//!
//! | 区别点 | 电流模式 FOC（速度控制） | 无刷伺服（本文件） |
//! |--------|----------------------|------------------|
//! | 控制目标 | 恒速旋转 (RPM) | 精确位置 (角度/脉冲) |
//! | 外环 | 速度 PI → Iq_ref | 位置 PID → 速度指令 → Uq |
//! | 内环 | Id/Iq 双电流 PI | 电压模式（无电流传感器） |
//! | 到达行为 | 持续旋转 | 到位停止并保持 |
//! | 运动模式 | 梯形/S 曲线加减速 | 点到点/轨迹跟踪 |
//! | 硬件需求 | 编码器 + 2× 电流传感器 | 仅编码器（更简单） |
//!
//! ---
//!
//! # 4. 数学基础
//!
//! ## 4.1 位置 PID 控制器
//!
//! ```text
//! error = target_position - actual_position  (编码器脉冲)
//!
//! P = Kp × error
//! I += Ki × error          (带积分限幅防 windup)
//! D = Kd × (error - error_prev)  (微分项抑制超调)
//!
//! speed_ref = (P + I + D) >> 10  →  限幅到 ±MAX_SPEED_RPM
//! ```
//!
//! **D 项的作用**：位置控制如果只有 PI，容易在目标位置附近产生过冲振荡。
//! D 项提供阻尼效果（类似摩擦力），能显著改善定位稳定性。
//!
//! ## 4.2 速度 PI 控制器
//!
//! ```text
//! speed_error = speed_ref - actual_speed
//! Uq = Kp_speed × speed_error + Ki_speed × ∫speed_error
//! ```
//!
//! ## 4.3 电角度计算
//!
//! ```text
//! θ_mech = (count mod CPR) × 360° / CPR
//! θ_elec = θ_mech × pole_pairs  (mod 360°)
//! ```
//!
//! Q16 格式：0~65535 = 0~360°
//!
//! ## 4.4 逆 Park + SVPWM
//!
//! 与 `bldc_foc.rs` 相同，Ud=0 简化：
//! ```text
//! Uα = -Uq × sin(θe)
//! Uβ =  Uq × cos(θe)
//! ```
//! SVPWM 中性点注入法详见 `bldc_foc.rs` 文档。
//!
//! ---
//!
//! # 5. 位置控制模式
//!
//! ## 5.1 绝对定位（点到点）
//!
//! 设定目标编码器计数值，伺服自动运动到该位置并保持：
//! ```text
//! TARGET_POSITION = 2000  → 转到编码器 2000 脉冲处 (= 180°)
//! ```
//!
//! ## 5.2 多段位置切换
//!
//! 主循环中按时间或触发条件切换目标位置，实现往复运动：
//! ```text
//! 0s~2s   → 位置 0
//! 2s~4s   → 位置 2000 (180°)
//! 4s~6s   → 位置 1000 (90°)
//! 6s~8s   → 位置 0
//! …循环
//! ```
//!
//! ## 5.3 角度指令接口
//!
//! 编码器脉冲和角度的换算：
//! ```text
//! 目标脉冲 = 目标角度° × CPR / 360
//! 目标角度° = 目标脉冲 × 360 / CPR
//!
//! 示例 (1000 线, 4 倍频, CPR=4000):
//!   90°  = 1000 脉冲
//!   180° = 2000 脉冲
//!   360° = 4000 脉冲 (一整圈)
//! ```
//!
//! ---
//!
//! # 6. GPIO 中断 4 倍频正交解码
//!
//! 与 `bldc_foc_current.rs` 相同的 GPIO 中断 + 状态转移表方案。
//!
//! ```text
//! 状态转移表 (prev_AB × curr_AB → 方向):
//!          curr_AB
//!          00   01   11   10
//! prev 00 [ 0 , +1 ,  0 , -1 ]
//! prev 01 [ -1,  0 , +1 ,  0 ]
//! prev 11 [ 0 , -1 ,  0 , +1 ]
//! prev 10 [ +1,  0 , -1 ,  0 ]
//! ```
//!
//! ---
//!
//! # 7. 硬件接线
//!
//! ## 7.1 引脚分配
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
//!
//! > **注意：** 本方案为电压模式 FOC，不需要电流传感器。
//! > 如果需要更精确的力矩控制（重载应用），可在速度环内层再加电流环，
//! > 参考 `bldc_foc_current.rs` 的实现。
//!
//! ## 7.2 增量式编码器
//!
//! ```text
//!   编码器 ─── A ──► GP10 (上拉 3.3V)
//!          ─── B ──► GP11
//!          ─── GND ──► GND
//!          ─── VCC ──► 3.3V / 5V
//! ```
//!
//! 开漏输出编码器需外部上拉电阻（4.7kΩ 到 3.3V）。
//!
//! ---
//!
//! # 8. 参数说明
//!
//! | 参数 | 默认值 | 说明 |
//! |------|--------|------|
//! | `MOTOR_POLES` | 14 | 电机磁极数 |
//! | `ENCODER_CPR` | 4000 | 编码器每转计数（4 倍频后） |
//! | `FOC_LOOP_HZ` | 10,000 | FOC 内环频率 |
//! | `POSITION_LOOP_DIVIDER` | 10 | 位置/速度环分频 → 1 kHz |
//! | `KP_POS / KI_POS / KD_POS` | Q10 定点 | 位置 PID 增益 |
//! | `KP_SPEED / KI_SPEED` | Q10 定点 | 速度 PI 增益 |
//! | `MAX_SPEED_RPM` | 3000 | 位置环输出速度限幅 |
//! | `UQ_MAX` | 17000 (Q15) | 力矩电压上限 |
//! | `POSITION_DEADBAND` | 2 | 位置死区（脉冲），消除抖动 |
//!
//! ---
//!
//! # 9. 位置环调参指南
//!
//! 三环级联调参顺序：**先内后外**（FOC → 速度 → 位置）。
//!
//! ```text
//! 步骤 1: FOC 内环（通常不需调，使用默认 SVPWM）
//!
//! 步骤 2: 速度环
//!   a. Kp_speed 从小到大，Ki_speed = 0
//!   b. 手动设目标速度，观察跟随响应
//!   c. 增大 Ki_speed 消除稳态误差
//!
//! 步骤 3: 位置环
//!   a. 先设 Ki_pos = 0, Kd_pos = 0，只用 Kp_pos
//!   b. 设目标位置，观察到位情况
//!   c. 过冲 → 增大 Kd_pos（加阻尼）
//!   d. 稳态有偏差 → 增大 Ki_pos（小值即可）
//!   e. 响应太慢 → 增大 Kp_pos（但注意稳定性）
//! ```
//!
//! **典型症状：**
//!
//! | 现象 | 原因 | 调整 |
//! |------|------|------|
//! | 到位后持续振荡 | Kp_pos 太大或 Kd_pos 不够 | 减 Kp_pos，增 Kd_pos |
//! | 到位慢 / 不到位 | Kp_pos 太小 | 增大 Kp_pos |
//! | 到位有稳态偏差 | Ki_pos 太小或为 0 | 增大 Ki_pos |
//! | 高速运动后严重过冲 | 速度限幅太高 | 降低 MAX_SPEED_RPM |
//! | 保持时微颤 | 位置死区太小或 D 项噪声 | 增 POSITION_DEADBAND |
//!
//! ---
//!
//! # 10. 常见问题
//!
//! **Q: 电机到达目标位置后振荡**
//! - Kd_pos 不够 → 增大微分项增益
//! - 速度环带宽过高 → 降低 Kp_speed
//!
//! **Q: 电机锁定在错误位置**
//! - 编码器方向反了 → 交换 A/B 线或改 `ENCODER_DIR`
//! - 电角度偏移不对 → 需要做初始对齐校准
//!
//! **Q: 从外部施加力后回不到原位**
//! - Ki_pos 太小 → 增大位置积分项
//! - UQ_MAX 太小 → 提高力矩限幅
//!
//! **Q: 如何实现连续旋转模式？**
//! - 持续递增 TARGET_POSITION，伺服将以受控速度连续旋转
//! - 或直接使用 `bldc_foc.rs`/`bldc_foc_current.rs` 的速度模式
//!
//! **Q: 如何接收 RC PPM 伺服信号？**
//! - 用 PWM 输入捕获测量 1000~2000 µs 脉宽
//! - 映射到编码器位置范围：`position = (pulse_us - 1000) × CPR / 1000`
//!
//! ---
//!
//! # 11. 编译运行
//!
//! ```bash
//! cd pico-rs-examples
//! cargo run --bin bldc_servo --target thumbv8m.main-none-eabihf
//! ```
//!
//! ---
//!
//! # 12. 参考资料
//!
//! - [RP2350 Datasheet](https://datasheets.raspberrypi.com/rp2350/rp2350-datasheet.pdf)
//! - *Field Oriented Control of AC Motors* — Texas Instruments (SPRABQ8)
//! - *STM32 FOC PMSM SDK* — ST AN1292, AN4220
//! - *Motion Control of PMSM Servo Systems* — PI/PID cascade design
//! - *Microchip AN1292* — Position Control with FOC

#![no_std]
#![no_main]

use core::cell::RefCell;
use core::sync::atomic::{AtomicBool, AtomicI32, AtomicU32, Ordering};

use defmt::*;
use defmt_rtt as _;
use panic_probe as _;

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

/// 电机磁极数
const MOTOR_POLES: u32 = 14;
const POLE_PAIRS: u32 = MOTOR_POLES / 2;

/// 编码器每转计数(CPR, 4 倍频后)
const ENCODER_CPR: u32 = 4000;

/// 编码器方向
const ENCODER_DIR: bool = true;

/// FOC 电压环频率 (Hz)
const FOC_LOOP_HZ: u32 = 10_000;
const FOC_LOOP_INTERVAL_US: u32 = 1_000_000 / FOC_LOOP_HZ;

/// 位置/速度环分频（相对于 FOC 循环）
/// 10 → 位置/速度环 1 kHz
const POSITION_LOOP_DIVIDER: u32 = 10;

// ---- 位置 PID 参数 (Q10 定点) ----
/// Kp_pos: 位置比例增益
/// 每个编码器脉冲误差产生多少 RPM 指令
/// 1.0 → 1024 (Q10)，2.0 → 2048
const KP_POS: i32 = 1024; // ~1.0
/// Ki_pos: 位置积分增益（消除稳态偏差，通常很小）
const KI_POS: i32 = 5; // ~0.005
/// Kd_pos: 位置微分增益（阻尼，抑制过冲振荡）
const KD_POS: i32 = 8192; // ~8.0

/// 位置积分限幅
const POS_INTEGRAL_MAX: i32 = 500_000;

/// 位置环输出速度限幅 (RPM)
const MAX_SPEED_RPM: i32 = 3000;

/// 位置死区（编码器脉冲数），在此范围内认为已到达
/// 防止目标附近微小误差导致电机持续抖动
const POSITION_DEADBAND: i32 = 2;

// ---- 速度 PI 参数 (Q10 定点) ----
const KP_SPEED: i32 = 512; // ~0.5
const KI_SPEED: i32 = 51; // ~0.05
const SPEED_INTEGRAL_MAX: i32 = 500_000;

/// Uq 最大值 (Q15)
/// SVPWM 线性范围最大 ~18919
const UQ_MAX: i32 = 17000;

/// Q10 定点移位
const FP_SHIFT: i32 = 10;

// ============================================================================
// 位置指令序列 — 演示用的多段位置
// ============================================================================

/// 目标位置序列（编码器脉冲）及每段保持时间（ms）
/// 伺服依次运动到各位置并停留
const POSITION_SEQUENCE: [(i32, u32); 4] = [
    (0, 3000),    // 位置 0°, 保持 3s
    (2000, 3000), // 位置 180°, 保持 3s
    (1000, 3000), // 位置 90°, 保持 3s
    (3000, 3000), // 位置 270°, 保持 3s
];

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
// SVPWM — 中性点注入法
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
// PI 控制器（通用）
// ============================================================================

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

    #[inline]
    fn update(&mut self, error: i32) -> i32 {
        let p = self.kp * error;
        self.integral =
            (self.integral + self.ki * error).clamp(-self.integral_max, self.integral_max);
        let output = (p + self.integral) >> FP_SHIFT;
        output.clamp(self.output_min, self.output_max)
    }

    #[allow(dead_code)]
    fn reset(&mut self) {
        self.integral = 0;
    }
}

// ============================================================================
// PID 控制器（位置环专用，带 D 项）
// ============================================================================

struct PidController {
    kp: i32,
    ki: i32,
    kd: i32,
    integral: i32,
    integral_max: i32,
    prev_error: i32,
    output_max: i32,
    output_min: i32,
}

impl PidController {
    const fn new(
        kp: i32,
        ki: i32,
        kd: i32,
        integral_max: i32,
        output_min: i32,
        output_max: i32,
    ) -> Self {
        Self {
            kp,
            ki,
            kd,
            integral: 0,
            integral_max,
            prev_error: 0,
            output_max,
            output_min,
        }
    }

    /// 执行一步 PID 计算
    #[inline]
    fn update(&mut self, error: i32) -> i32 {
        let p = self.kp * error;

        self.integral =
            (self.integral + self.ki * error).clamp(-self.integral_max, self.integral_max);

        let d = self.kd * (error - self.prev_error);
        self.prev_error = error;

        let output = (p + self.integral + d) >> FP_SHIFT;
        output.clamp(self.output_min, self.output_max)
    }

    #[allow(dead_code)]
    fn reset(&mut self) {
        self.integral = 0;
        self.prev_error = 0;
    }
}

// ============================================================================
// 全局共享状态
// ============================================================================

static TIMER: Mutex<RefCell<Option<Timer0>>> = Mutex::new(RefCell::new(None));

/// FOC tick 标志
static FOC_TICK: AtomicBool = AtomicBool::new(false);

/// 编码器累计计数
static ENCODER_COUNT: AtomicI32 = AtomicI32::new(0);

/// 编码器上次 AB 状态 (2-bit: A<<1 | B)
static ENCODER_LAST_AB: AtomicU32 = AtomicU32::new(0);

/// 目标位置（编码器脉冲计数）
static TARGET_POSITION: AtomicI32 = AtomicI32::new(0);

/// 实测 RPM (日志用)
static ACTUAL_RPM: AtomicI32 = AtomicI32::new(0);

// ============================================================================
// 编码器 4 倍频解码表
// ============================================================================

const ENCODER_TABLE: [i32; 16] = [
    0, 1, 0, -1, // prev=00
    -1, 0, 1, 0, // prev=01
    0, -1, 0, 1, // prev=11
    1, 0, -1, 0, // prev=10
];

// ============================================================================
// 入口
// ============================================================================

#[entry]
fn main() -> ! {
    info!("BLDC Servo — Position/Speed/Voltage triple-loop FOC");

    // ------------------------------------------------------------------
    // 外设初始化
    // ------------------------------------------------------------------
    let dp = Peripherals::take().unwrap();
    let clocks = ClockBuilder::new().build(dp.CLOCKS, dp.XOSC, dp.PLL_SYS, dp.PLL_USB);
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

    let slice_a = pwm.slice::<0>().output_a(pins.pin::<0>(), sys_freq);
    slice_a.set_wrap(pwm_wrap);
    slice_a.set_duty_a(0);
    slice_a.enable();

    let slice_b = pwm.slice::<1>().output_a(pins.pin::<2>(), sys_freq);
    slice_b.set_wrap(pwm_wrap);
    slice_b.set_duty_a(0);
    slice_b.enable();

    let slice_c = pwm.slice::<2>().output_a(pins.pin::<4>(), sys_freq);
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
    // 编码器 — GPIO 中断 4 倍频正交解码
    // ------------------------------------------------------------------
    let enc_a = pins.pin::<10>();
    enc_a.into_input_pullup();
    enc_a.enable_interrupt(pico_rs::gpio::EdgeLevel::EdgeHigh);
    enc_a.enable_interrupt(pico_rs::gpio::EdgeLevel::EdgeLow);

    let enc_b = pins.pin::<11>();
    enc_b.into_input_pullup();
    enc_b.enable_interrupt(pico_rs::gpio::EdgeLevel::EdgeHigh);
    enc_b.enable_interrupt(pico_rs::gpio::EdgeLevel::EdgeLow);

    let init_a = enc_a.read();
    let init_b = enc_b.read();
    info!("Encoder init: A={}, B={}", init_a, init_b);

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
        });

        unsafe {
            interrupt::enable(Interrupt::TIMER0_IRQ_0);
            interrupt::enable(Interrupt::IO_IRQ_BANK0);
        }
    }

    info!(
        "Servo ready: {}Hz FOC, {}Hz pos/speed loop, {} CPR encoder",
        FOC_LOOP_HZ,
        FOC_LOOP_HZ / POSITION_LOOP_DIVIDER,
        ENCODER_CPR
    );

    // ------------------------------------------------------------------
    // 控制器实例
    // ------------------------------------------------------------------
    let mut pid_pos = PidController::new(
        KP_POS,
        KI_POS,
        KD_POS,
        POS_INTEGRAL_MAX,
        -MAX_SPEED_RPM,
        MAX_SPEED_RPM,
    );

    let mut pi_speed = PiController::new(KP_SPEED, KI_SPEED, SPEED_INTEGRAL_MAX, -UQ_MAX, UQ_MAX);

    let mut foc_counter: u32 = 0;
    let mut log_counter: u32 = 0;
    let mut last_encoder_count: i32 = 0;

    // 位置指令序列状态
    let mut seq_index: usize = 0;

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
    let mut seq_start_time: u32 = last_speed_time;

    // 设置首个目标位置
    if !POSITION_SEQUENCE.is_empty() {
        TARGET_POSITION.store(POSITION_SEQUENCE[0].0, Ordering::Relaxed);
        info!(
            "Sequence [0]: target position = {} pulses",
            POSITION_SEQUENCE[0].0
        );
    }

    // 当前 Uq（由速度环输出）
    let mut current_uq: i32 = 0;
    // 当前速度参考（由位置环输出）
    let mut speed_ref: i32;

    // ------------------------------------------------------------------
    // 主循环
    // ------------------------------------------------------------------
    loop {
        if !FOC_TICK.swap(false, Ordering::Relaxed) {
            continue;
        }

        foc_counter += 1;

        // ==============================================================
        // 1. 编码器 → 电角度 θe
        // ==============================================================
        let enc_count = ENCODER_COUNT.load(Ordering::Relaxed);

        let count_in_rev = enc_count.rem_euclid(ENCODER_CPR as i32) as u32;
        let theta_mech = ((count_in_rev as u64 * 65536u64) / ENCODER_CPR as u64) as u16;
        let theta_e = theta_mech.wrapping_mul(POLE_PAIRS as u16);

        let sin_theta = sin_q15(theta_e);
        let cos_theta = cos_q15(theta_e);

        // ==============================================================
        // 2. 逆 Park 变换（电压模式：Ud=0）
        // ==============================================================
        let uq = current_uq;
        let u_alpha = (-(uq as i64 * sin_theta as i64) >> 15) as i32;
        let u_beta = ((uq as i64 * cos_theta as i64) >> 15) as i32;

        // ==============================================================
        // 3. SVPWM → PWM 输出
        // ==============================================================
        let (da, db, dc) = svpwm(u_alpha, u_beta, pwm_wrap);

        slice_a.set_duty_a(da);
        slice_b.set_duty_a(db);
        slice_c.set_duty_a(dc);

        // ==============================================================
        // 4. 位置环 + 速度环 (@1 kHz)
        // ==============================================================
        if foc_counter % POSITION_LOOP_DIVIDER == 0 {
            let now = timer_now_us();
            let dt_us = now.wrapping_sub(last_speed_time);
            last_speed_time = now;

            // ---- 位置指令序列更新 ----
            if !POSITION_SEQUENCE.is_empty() {
                let hold_ms = POSITION_SEQUENCE[seq_index].1;
                let elapsed_ms = now.wrapping_sub(seq_start_time) / 1000;
                if elapsed_ms >= hold_ms {
                    seq_index = (seq_index + 1) % POSITION_SEQUENCE.len();
                    seq_start_time = now;
                    let new_target = POSITION_SEQUENCE[seq_index].0;
                    TARGET_POSITION.store(new_target, Ordering::Relaxed);
                    info!(
                        "Sequence [{}]: target position = {} pulses (= {}°)",
                        seq_index,
                        new_target,
                        new_target * 360 / ENCODER_CPR as i32
                    );
                }
            }

            // ---- 实际速度计算 (编码器差分) ----
            let delta_count = enc_count - last_encoder_count;
            last_encoder_count = enc_count;

            let actual_rpm = if dt_us > 0 {
                (delta_count as i64 * 60_000_000i64 / (ENCODER_CPR as i64 * dt_us as i64)) as i32
            } else {
                0
            };
            ACTUAL_RPM.store(actual_rpm, Ordering::Relaxed);

            // ---- 位置 PID → 速度参考 ----
            let target_pos = TARGET_POSITION.load(Ordering::Relaxed);
            let pos_error = target_pos - enc_count;

            // 死区处理：误差在死区内时不驱动（消除抖动）
            if pos_error.abs() <= POSITION_DEADBAND {
                // 在死区内，重置积分，指令零速
                pid_pos.reset();
                speed_ref = 0;
            } else {
                speed_ref = pid_pos.update(pos_error);
            }

            // ---- 速度 PI → Uq ----
            let speed_error = speed_ref - actual_rpm;
            current_uq = pi_speed.update(speed_error);

            // ---- 日志 (~2 Hz) ----
            log_counter += 1;
            if log_counter % 500 == 0 {
                let target_deg = target_pos * 360 / ENCODER_CPR as i32;
                let actual_deg = enc_count * 360 / ENCODER_CPR as i32;
                info!(
                    "pos:{}/{} ({}°/{}°) err:{} spd_ref:{} RPM:{} Uq:{} seq:{}",
                    enc_count,
                    target_pos,
                    actual_deg,
                    target_deg,
                    pos_error,
                    speed_ref,
                    actual_rpm,
                    current_uq,
                    seq_index
                );
            }
        }
    }
}

// ============================================================================
// GPIO 中断 — 编码器 4 倍频解码
// ============================================================================

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
fn IO_IRQ_BANK0() {
    let pin_a = unsafe { pico_rs::gpio::Pin::<10>::new() };
    let pin_b = unsafe { pico_rs::gpio::Pin::<11>::new() };
    pin_a.clear_all_interrupts();
    pin_b.clear_all_interrupts();

    let a = pin_a.read() as u32;
    let b = pin_b.read() as u32;
    let curr_ab = (a << 1) | b;

    let prev_ab = ENCODER_LAST_AB.swap(curr_ab, Ordering::Relaxed);
    let idx = ((prev_ab << 2) | curr_ab) as usize;
    let dir = ENCODER_TABLE[idx & 0xF];

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
