//! # 电流模式无刷伺服 — 编码器 + 位置/速度/电流四环 FOC
//!
//! 基于 RP2350 (Raspberry Pi Pico 2) 的**电流模式无刷伺服控制器**。
//! 在 `bldc_servo.rs`（电压模式三环）的基础上，增加 **双电流 PI 内环**，
//! 实现更精确的力矩控制和更快的动态响应。
//!
//! 控制层级为四环级联：
//! **位置环（PID）→ 速度环（PI）→ 电流环（Id/Iq 双 PI）→ 逆 Park + SVPWM**
//!
//! 其他驱动模式见：
//! - `bldc.rs` — 开环 6 步梯形换向
//! - `bldc_speed_ctrl.rs` — 无传感器 BEMF 闭环
//! - `bldc_hall.rs` — 霍尔 6 步 + PI 转速控制
//! - `bldc_foc.rs` — 电压模式 FOC（霍尔，速度控制）
//! - `bldc_foc_current.rs` — 电流模式 FOC（编码器，速度控制）
//! - `bldc_servo.rs` — 电压模式伺服（编码器，位置控制）
//!
//! ---
//!
//! # 1. 电流模式伺服 vs 电压模式伺服
//!
//! | 特性 | 电压模式伺服 (`bldc_servo.rs`) | 电流模式伺服（本方案） |
//! |------|------|------|
//! | 控制层级 | 位置 PID → 速度 PI → Uq(电压) | 位置 PID → 速度 PI → Iq/Id PI(电流) → Ud/Uq |
//! | 力矩控制 | 间接（电压 ≈ 力矩） | 精确（电流 ∝ 力矩） |
//! | 动态响应 | 受电气时间常数限制 | 快（电流环带宽可达数 kHz） |
//! | 保持刚度 | 中等 | 高（电流闭环补偿负载扰动） |
//! | 温度/转速鲁棒性 | 低（绕组电阻随温变） | 高（电流反馈自动补偿） |
//! | 硬件需求 | 编码器 | 编码器 + 2× 电流传感器 + ADC |
//! | 适用场景 | 轻载、简单定位 | 重载、高精度、工业伺服 |
//!
//! ---
//!
//! # 2. 控制链路
//!
//! ```text
//! ┌──────────────────────────────────────────────────────────────────────────┐
//! │              电流模式无刷伺服四环级联控制链路                                │
//! │                                                                          │
//! │  ┌───────────┐     ┌──────────┐     ┌──────────┐     ┌──────────┐      │
//! │  │ 目标位置   │────▶│ 位置 PID │────▶│ 速度 PI  │────▶│ Iq_ref   │      │
//! │  │ (编码器脉冲│     │ @1 kHz   │     │ @1 kHz   │     │ (力矩指令)│      │
//! │  │  或角度°)  │     └──────────┘     └──────────┘     └────┬─────┘      │
//! │  └───────────┘                                            │ Id_ref = 0  │
//! │                                                           ▼             │
//! │  ┌───────────┐     ┌──────────┐     ┌──────────┐   ┌──────────┐       │
//! │  │ Ia, Ib    │────▶│ Clarke   │────▶│  Park    │──▶│ Id PI    │→ Ud   │
//! │  │ (ADC)     │     │ abc→αβ   │     │  αβ→dq   │   │ Iq PI    │→ Uq   │
//! │  └───────────┘     └──────────┘     └──────────┘   └────┬─────┘       │
//! │                                                         │              │
//! │  ┌───────────┐                                          ▼              │
//! │  │ 编码器     │──▶ 位置/速度 ──▶ θe ──────────▶ ┌──────────────┐     │
//! │  │ A/B GPIO   │                                │ 逆 Park 变换  │     │
//! │  │ 4 倍频解码 │                                │  dq → αβ      │     │
//! │  └───────────┘                                 └───────┬──────┘     │
//! │                                                        │ Uα, Uβ      │
//! │                                                        ▼             │
//! │                                                 ┌──────────────┐     │
//! │                                                 │   SVPWM      │     │
//! │                                                 │  αβ → 3相PWM │     │
//! │                                                 └──┬──┬──┬───┘     │
//! │                                                    │  │  │         │
//! │                                                   DA DB DC         │
//! │                                                    │  │  │         │
//! │                                                 ┌──┴──┴──┴──┐     │
//! │                                                 │  三相逆变器  │     │
//! │                                                 └─────┬──────┘     │
//! │                                                       │             │
//! │                                                  ┌────┴─────┐      │
//! │                                                  │ BLDC 电机  │      │
//! │                                                  └──────────┘      │
//! └──────────────────────────────────────────────────────────────────────────┘
//! ```
//!
//! **四环带宽分配原则：**
//! ```text
//! 位置环  ~100 Hz   ← 最外层，带宽最低
//! 速度环  ~500 Hz   ← 第二层
//! 电流环  ~2 kHz    ← 第三层，快速力矩响应
//! FOC/PWM ~10 kHz   ← 最内层，SVPWM 调制频率
//! ```
//!
//! ---
//!
//! # 3. 核心架构总览
//!
//! | 模块 | 实现方式 | 说明 |
//! |------|---------|------|
//! | **角度反馈** | 增量式编码器 GP10(A) + GP11(B) | GPIO 双边沿中断 + 4 倍频状态转移表解码 |
//! | **电流采样** | GP26(Ia) + GP27(Ib) ADC 12-bit | Clarke→Park 得 Id/Iq 实际值 |
//! | **位置 PID** | Q10 定点 PID, @1 kHz | 位置误差 → 速度指令，带 D 项抑制超调振荡 |
//! | **速度 PI** | Q10 定点 PI, @1 kHz | 编码器差分求 RPM，输出 Iq_ref（力矩电流指令） |
//! | **Id PI** | Q10 定点 PI, @10 kHz | Id_ref=0（不弱磁），输出 Ud |
//! | **Iq PI** | Q10 定点 PI, @10 kHz | Iq_ref 由速度环提供，输出 Uq |
//! | **逆 Park** | dq → αβ（完整形式） | Uα=Ud·cos−Uq·sin, Uβ=Ud·sin+Uq·cos |
//! | **SVPWM** | 中性点注入法 | αβ → 三相占空比 |
//! | **三角函数** | 256 点 Q15 正弦表 | `sin_q15()` / `cos_q15()` |
//! | **位置指令** | 编码器脉冲计数 / 角度° | 多段位置序列自动切换 |
//! | **保持模式** | 位置环 + 电流环持续工作 | 到达目标后零速保持，电流闭环抗扰 |
//! | **中断** | `IO_IRQ_BANK0` + `TIMER0_IRQ_0` | 编码器 4 倍频 @GPIO + FOC tick @10 kHz |
//!
//! ### 与电压模式伺服 (`bldc_servo.rs`) 的关键区别
//!
//! | 区别点 | 电压模式伺服 | 电流模式伺服（本文件） |
//! |--------|------------|---------------------|
//! | 速度环输出 | Uq 电压 | Iq_ref 电流指令 (mA) |
//! | 内环 | 无（直接设 Uq） | Id/Iq 双电流 PI |
//! | 逆 Park | 简化 Ud=0 | 完整 Ud+Uq |
//! | 力矩精度 | 低（受温度/转速影响） | 高（电流∝力矩） |
//! | 保持刚度 | 中等 | 高（电流补偿负载变化） |
//! | ADC 需求 | 无 | 2 通道 (Ia, Ib) |
//! | 调参难度 | 低（三环） | 中等（四环，先内后外） |
//!
//! ### 与电流模式 FOC (`bldc_foc_current.rs`) 的关键区别
//!
//! | 区别点 | 电流模式 FOC（速度控制） | 电流模式伺服（本文件） |
//! |--------|----------------------|---------------------|
//! | 控制目标 | 恒速旋转 (RPM) | 精确位置 (角度/脉冲) |
//! | 最外环 | 速度 PI → Iq_ref | 位置 PID → 速度 → Iq_ref |
//! | 到达行为 | 持续旋转 | 到位停止并保持 |
//! | D 项 | 无 | 位置环有微分项（阻尼） |
//! | 位置死区 | 无 | 有（消除抖动） |
//! | 运动模式 | 连续旋转 | 点到点 / 轨迹跟踪 |
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
//! ## 4.2 速度 PI → Iq_ref
//!
//! ```text
//! speed_error = speed_ref - actual_speed
//! Iq_ref = Kp_speed × speed_error + Ki_speed × ∫speed_error
//! ```
//!
//! Iq_ref 单位为 mA，限幅到 ±IQ_REF_MAX。
//!
//! ## 4.3 电流 PI (Id/Iq)
//!
//! ```text
//! Id_error = 0 - Id_actual   (不弱磁，Id_ref = 0)
//! Ud = Kp_id × Id_error + Ki_id × ∫Id_error
//!
//! Iq_error = Iq_ref - Iq_actual
//! Uq = Kp_iq × Iq_error + Ki_iq × ∫Iq_error
//! ```
//!
//! ## 4.4 Clarke 变换 (abc → αβ)
//!
//! ```text
//! Iα = Ia
//! Iβ = (Ia + 2·Ib) / √3
//! ```
//!
//! ## 4.5 Park 变换 (αβ → dq)
//!
//! ```text
//! Id =  Iα × cos(θe) + Iβ × sin(θe)
//! Iq = -Iα × sin(θe) + Iβ × cos(θe)
//! ```
//!
//! ## 4.6 逆 Park 变换 (dq → αβ)
//!
//! ```text
//! Uα = Ud × cos(θe) - Uq × sin(θe)
//! Uβ = Ud × sin(θe) + Uq × cos(θe)
//! ```
//!
//! 注意：电压模式伺服简化为 Ud=0；本方案 Ud 由 Id PI 输出，是完整形式。
//!
//! ## 4.7 电角度计算
//!
//! ```text
//! θ_mech = (count mod CPR) × 360° / CPR
//! θ_elec = θ_mech × pole_pairs  (mod 360°)
//! ```
//!
//! Q16 格式：0~65535 = 0~360°
//!
//! ## 4.8 SVPWM
//!
//! 与 `bldc_foc.rs` 相同的中性点注入法，详见该文件文档。
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
//! 主循环中按时间切换目标位置，实现往复运动（与电压模式伺服相同）。
//!
//! ## 5.3 电流模式的优势
//!
//! 在重载下（如机械臂关节），电流环确保力矩输出精确：
//! ```text
//! 负载突变 → 位置偏差 → 速度指令 → Iq_ref 增大 → 电流环跟踪 → 力矩补偿
//!          ↑ 电压模式：V ≈ R×I + L×dI/dt + 反EMF → 力矩不精确
//!          ↑ 电流模式：电流闭环直接控制 I → 力矩 = Kt×Iq → 精确
//! ```
//!
//! ---
//!
//! # 6. GPIO 中断 4 倍频正交解码
//!
//! 与 `bldc_servo.rs` 和 `bldc_foc_current.rs` 相同。
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
//! | GP26 | Ia 电流采样 | ADC Ch0 | 相 A 电流传感器 |
//! | GP27 | Ib 电流采样 | ADC Ch1 | 相 B 电流传感器 |
//!
//! ## 7.2 电流传感器
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
//! ## 7.3 增量式编码器
//!
//! ```text
//!   编码器 ─── A ──► GP10 (上拉 3.3V)
//!          ─── B ──► GP11
//!          ─── GND ──► GND
//!          ─── VCC ──► 3.3V / 5V
//! ```
//!
//! ---
//!
//! # 8. 参数说明
//!
//! | 参数 | 默认值 | 说明 |
//! |------|--------|------|
//! | `MOTOR_POLES` | 14 | 电机磁极数 |
//! | `ENCODER_CPR` | 4000 | 编码器每转计数（4 倍频后） |
//! | `FOC_LOOP_HZ` | 10,000 | FOC / 电流环频率 |
//! | `POSITION_LOOP_DIVIDER` | 10 | 位置/速度环分频 → 1 kHz |
//! | `KP_POS / KI_POS / KD_POS` | Q10 定点 | 位置 PID 增益 |
//! | `KP_SPEED / KI_SPEED` | Q10 定点 | 速度 PI 增益 |
//! | `KP_ID / KI_ID` | Q10 定点 | d 轴电流 PI 增益 |
//! | `KP_IQ / KI_IQ` | Q10 定点 | q 轴电流 PI 增益 |
//! | `MAX_SPEED_RPM` | 3000 | 位置环输出速度限幅 |
//! | `IQ_REF_MAX` | 5000 mA | 速度环输出电流限幅 |
//! | `U_MAX` | 17000 (Q15) | 电流 PI 输出电压上限 |
//! | `POSITION_DEADBAND` | 2 | 位置死区（脉冲），消除抖动 |
//! | `CURRENT_SENSE_MV_PER_A` | 400 | 电流传感器灵敏度 |
//!
//! ---
//!
//! # 9. 四环调参指南
//!
//! 严格遵循**先内后外**的原则：电流环 → 速度环 → 位置环。
//!
//! ```text
//! 步骤 1: 电流环 (Id/Iq PI)
//!   a. 锁定电机，施加阶跃 Iq_ref
//!   b. Kp_iq 从小到大，Ki_iq = 0
//!   c. 增大到有 ~5% 过冲
//!   d. 缓慢加 Ki_iq 消除稳态误差
//!   e. Id 环参数通常与 Iq 相同
//!
//! 步骤 2: 速度环
//!   a. 固定位置环不动（Kp_pos = 0），直接写目标 RPM
//!   b. Kp_speed 从小到大，Ki_speed = 0
//!   c. 增大 Ki_speed 消除稳态误差
//!
//! 步骤 3: 位置环
//!   a. Ki_pos = 0, Kd_pos = 0，只用 Kp_pos
//!   b. 设目标位置，观察到位
//!   c. 过冲 → 增大 Kd_pos
//!   d. 稳态偏差 → 增大 Ki_pos
//!   e. 响应慢 → 增大 Kp_pos
//! ```
//!
//! **典型症状：**
//!
//! | 现象 | 原因 | 调整 |
//! |------|------|------|
//! | 高频啸叫 | 电流 PI Kp 太大 | 降低 Kp_iq/Kp_id |
//! | 到位振荡 | Kp_pos 太大或 Kd_pos 不够 | 减 Kp_pos，增 Kd_pos |
//! | 到位慢 | 速度限幅太低或 Kp_pos 太小 | 增大 MAX_SPEED_RPM 或 Kp_pos |
//! | 重载偏差 | IQ_REF_MAX 太小 | 增大电流限幅 |
//! | 保持抖动 | 位置死区太小 | 增大 POSITION_DEADBAND |
//! | 电流波纹大 | Ki_iq 太大 | 减小 Ki_iq |
//!
//! ---
//!
//! # 10. 常见问题
//!
//! **Q: 与电压模式伺服的选型建议？**
//! - 轻载 + 简单定位 → `bldc_servo.rs`（无需电流传感器，调参简单）
//! - 重载 + 高精度 + 负载突变 → 本方案（电流闭环，力矩精确）
//!
//! **Q: 电机到达目标位置后振荡**
//! - 电流环太快 + 位置环 D 项不够 → 增大 Kd_pos
//! - 速度环带宽 > 电流环带宽 → 检查带宽比
//!
//! **Q: 低速保持时电流波纹大**
//! - 电流传感器噪声 → 加硬件滤波（RC 100nF）
//! - Ki_iq 导致积分累积 → 在死区内重置积分器
//!
//! **Q: 电流传感器校准**
//! - 上电时电机不动，采集 N 次 ADC 取平均 → 更新 `ADC_ZERO_CURRENT`
//! - 温度漂移 → 定期自动零点校准
//!
//! ---
//!
//! # 11. 编译运行
//!
//! ```bash
//! cd pico-rs-examples
//! cargo run --bin bldc_servo_current --target thumbv8m.main-none-eabihf
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

use pico_rs::adc::{Adc, AdcPin};
use pico_rs::clocks::ClockBuilder;
use pico_rs::main;
use pico_rs::gpio::Pins;
use pico_rs::interrupt::{self, Mutex};
use pico_rs::pac::{Interrupt, Peripherals};
use pico_rs::pwm::Pwm;
use pico_rs::timer::Timer0;// ============================================================================
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

/// FOC / 电流环频率 (Hz)
const FOC_LOOP_HZ: u32 = 10_000;
const FOC_LOOP_INTERVAL_US: u32 = 1_000_000 / FOC_LOOP_HZ;

/// 位置/速度环分频（相对于 FOC 循环）
/// 10 → 位置/速度环 1 kHz
const POSITION_LOOP_DIVIDER: u32 = 10;

// ---- 位置 PID 参数 (Q10 定点) ----
const KP_POS: i32 = 1024; // ~1.0
const KI_POS: i32 = 5; // ~0.005
const KD_POS: i32 = 8192; // ~8.0
const POS_INTEGRAL_MAX: i32 = 500_000;
const MAX_SPEED_RPM: i32 = 3000;

/// 位置死区（编码器脉冲数）
const POSITION_DEADBAND: i32 = 2;

// ---- 速度 PI 参数 (Q10 定点) ----
/// 输出单位为 mA (Iq_ref)
const KP_SPEED: i32 = 307; // ~0.3
const KI_SPEED: i32 = 20; // ~0.02
const SPEED_INTEGRAL_MAX: i32 = 500_000;

/// Iq 参考最大值 (mA)
const IQ_REF_MAX: i32 = 5000;

// ---- 电流 PI 参数 (Q10 定点) ----
/// Id PI (d 轴 — 磁通)
const KP_ID: i32 = 512; // ~0.5
const KI_ID: i32 = 51; // ~0.05

/// Iq PI (q 轴 — 力矩)
const KP_IQ: i32 = 512; // ~0.5
const KI_IQ: i32 = 51; // ~0.05

/// 电流积分限幅
const CURRENT_INTEGRAL_MAX: i32 = 500_000;

/// 电压输出限幅 (Q15)
const U_MAX: i32 = 17000;

/// Q10 定点移位
const FP_SHIFT: i32 = 10;

// ---- 电流传感器参数 ----
/// 传感器灵敏度 (mV/A)
const CURRENT_SENSE_MV_PER_A: i32 = 400;

/// ADC 中点（0A 时的 ADC 读数，理想值 2048）
const ADC_ZERO_CURRENT: i32 = 2048;

// ============================================================================
// 位置指令序列 — 演示用的多段位置
// ============================================================================

const POSITION_SEQUENCE: [(i32, u32); 4] = [
    (0, 3000),    // 0°, 保持 3s
    (2000, 3000), // 180°, 保持 3s
    (1000, 3000), // 90°, 保持 3s
    (3000, 3000), // 270°, 保持 3s
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
// ADC → 电流换算
// ============================================================================

/// 将 ADC 原始值转换为电流 (mA, 有符号)
///
/// ```text
/// voltage_mv = adc_raw × 3300 / 4096
/// current_mA = (voltage_mv - 1650) × 1000 / sensitivity_mv_per_a
///            = (adc_raw - 2048) × 806 / sensitivity
/// ```
#[inline]
fn adc_to_current_ma(adc_raw: u16) -> i32 {
    let diff = adc_raw as i32 - ADC_ZERO_CURRENT;
    (diff * 806) / CURRENT_SENSE_MV_PER_A
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
static ADC: Mutex<RefCell<Option<Adc>>> = Mutex::new(RefCell::new(None));

/// FOC tick 标志
static FOC_TICK: AtomicBool = AtomicBool::new(false);

/// 编码器累计计数
static ENCODER_COUNT: AtomicI32 = AtomicI32::new(0);

/// 编码器上次 AB 状态 (2-bit: A<<1 | B)
static ENCODER_LAST_AB: AtomicU32 = AtomicU32::new(0);

/// 目标位置（编码器脉冲计数）
static TARGET_POSITION: AtomicI32 = AtomicI32::new(0);

/// Iq 参考值 (mA, 由速度环输出，电流环消费)
static CURRENT_IQ_REF: AtomicI32 = AtomicI32::new(0);

/// 实测 RPM (日志用)
static ACTUAL_RPM: AtomicI32 = AtomicI32::new(0);

/// 电流采样值 (日志用)
static CURRENT_IA_MA: AtomicI32 = AtomicI32::new(0);
static CURRENT_IB_MA: AtomicI32 = AtomicI32::new(0);

/// 电流 PI 输出电压 (日志用)
static OUTPUT_UD: AtomicI32 = AtomicI32::new(0);
static OUTPUT_UQ: AtomicI32 = AtomicI32::new(0);

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

#[main]
fn main() -> ! {
    info!("BLDC Servo Current Mode — Position/Speed/Current/Voltage 4-loop FOC");

    // ------------------------------------------------------------------
    // 外设初始化
    // ------------------------------------------------------------------
    let dp = Peripherals::take().unwrap();
    let clocks = ClockBuilder::new()
        .enable_adc()
        .build(dp.CLOCKS, dp.XOSC, dp.PLL_SYS, dp.PLL_USB);
    let sys_freq = clocks.sys_freq();
    info!("System clock: {} MHz", sys_freq / 1_000_000);

    let pins = Pins::init(dp.IO_BANK0, dp.PADS_BANK0);
    let timer = Timer0::init(dp.TIMER0);

    // ------------------------------------------------------------------
    // PWM — 20 kHz，3 个 slice
    // ------------------------------------------------------------------
    let pwm_wrap: u16 = (sys_freq / PWM_FREQ_HZ) as u16;
    info!("PWM wrap = {} ({} kHz)", pwm_wrap, PWM_FREQ_HZ / 1000);

    let pwm = Pwm::init(dp.PWM);

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
    // ADC — 两相电流采样 (Ia on GP26, Ib on GP27)
    // ------------------------------------------------------------------
    let adc = Adc::init(dp.ADC);
    let adc_ia = pins.pin::<26>().into_adc_input();
    let adc_ib = pins.pin::<27>().into_adc_input();
    info!(
        "ADC initialized: Ia=Ch{}, Ib=Ch{}",
        adc_ia.channel(),
        adc_ib.channel()
    );

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

    let init_a = enc_a.get();
    let init_b = enc_b.get();
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
            ADC.borrow(cs).replace(Some(adc));
        });

        unsafe {
            interrupt::enable(Interrupt::TIMER0_IRQ_0);
            interrupt::enable(Interrupt::IO_IRQ_BANK0);
        }
    }

    info!(
        "Servo ready: {}Hz FOC/current, {}Hz pos/speed, {} CPR encoder",
        FOC_LOOP_HZ,
        FOC_LOOP_HZ / POSITION_LOOP_DIVIDER,
        ENCODER_CPR
    );

    // ------------------------------------------------------------------
    // 控制器实例
    // ------------------------------------------------------------------

    // 位置环 (PID)
    let mut pid_pos = PidController::new(
        KP_POS,
        KI_POS,
        KD_POS,
        POS_INTEGRAL_MAX,
        -MAX_SPEED_RPM,
        MAX_SPEED_RPM,
    );

    // 速度环 (PI) → Iq_ref (mA)
    let mut pi_speed = PiController::new(
        KP_SPEED,
        KI_SPEED,
        SPEED_INTEGRAL_MAX,
        -IQ_REF_MAX,
        IQ_REF_MAX,
    );

    // 电流环 (Id/Iq PI) → Ud/Uq (Q15)
    let mut pi_id = PiController::new(KP_ID, KI_ID, CURRENT_INTEGRAL_MAX, -U_MAX, U_MAX);
    let mut pi_iq = PiController::new(KP_IQ, KI_IQ, CURRENT_INTEGRAL_MAX, -U_MAX, U_MAX);

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

    // ------------------------------------------------------------------
    // 主循环 — 电流环 @10kHz + 位置/速度环 @1kHz
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

        let count_in_rev = enc_count.rem_euclid(ENCODER_CPR as i32) as u32;
        let theta_mech = ((count_in_rev as u64 * 65536u64) / ENCODER_CPR as u64) as u16;
        let theta_e = theta_mech.wrapping_mul(POLE_PAIRS as u16);

        let sin_theta = sin_q15(theta_e);
        let cos_theta = cos_q15(theta_e);

        // ==============================================================
        // 3. Clarke 变换: Ia, Ib → Iα, Iβ
        // ==============================================================
        let i_alpha = ia_ma;
        let i_beta = (((ia_ma as i64 + 2 * ib_ma as i64) * 37837i64) >> 16) as i32;

        // ==============================================================
        // 4. Park 变换: Iα, Iβ → Id, Iq
        // ==============================================================
        let id =
            ((i_alpha as i64 * cos_theta as i64 + i_beta as i64 * sin_theta as i64) >> 15) as i32;
        let iq =
            ((-i_alpha as i64 * sin_theta as i64 + i_beta as i64 * cos_theta as i64) >> 15) as i32;

        // ==============================================================
        // 5. 电流 PI 控制器 (Id/Iq → Ud/Uq)
        // ==============================================================
        // Id_ref = 0（不弱磁）
        let id_error = 0 - id;
        let ud = pi_id.update(id_error);

        // Iq_ref 由速度环提供
        let iq_ref = CURRENT_IQ_REF.load(Ordering::Relaxed);
        let iq_error = iq_ref - iq;
        let uq = pi_iq.update(iq_error);

        OUTPUT_UD.store(ud, Ordering::Relaxed);
        OUTPUT_UQ.store(uq, Ordering::Relaxed);

        // ==============================================================
        // 6. 逆 Park 变换 (完整形式: Ud + Uq)
        // ==============================================================
        let u_alpha = ((ud as i64 * cos_theta as i64 - uq as i64 * sin_theta as i64) >> 15) as i32;
        let u_beta = ((ud as i64 * sin_theta as i64 + uq as i64 * cos_theta as i64) >> 15) as i32;

        // ==============================================================
        // 7. SVPWM → PWM 输出
        // ==============================================================
        let (da, db, dc) = svpwm(u_alpha, u_beta, pwm_wrap);

        slice_a.set_duty_a(da);
        slice_b.set_duty_a(db);
        slice_c.set_duty_a(dc);

        // ==============================================================
        // 8. 位置环 + 速度环 (@1 kHz)
        // ==============================================================
        if foc_counter.is_multiple_of(POSITION_LOOP_DIVIDER) {
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

            let speed_ref = if pos_error.abs() <= POSITION_DEADBAND {
                pid_pos.reset();
                pi_speed.reset();
                0
            } else {
                pid_pos.update(pos_error)
            };

            // ---- 速度 PI → Iq_ref (mA) ----
            let speed_error = speed_ref - actual_rpm;
            let iq_ref_new = pi_speed.update(speed_error);
            CURRENT_IQ_REF.store(iq_ref_new, Ordering::Relaxed);

            // ---- 日志 (~2 Hz) ----
            log_counter += 1;
            if log_counter.is_multiple_of(500) {
                let target_deg = target_pos * 360 / ENCODER_CPR as i32;
                let actual_deg = enc_count * 360 / ENCODER_CPR as i32;
                info!(
                    "pos:{}/{} ({}°/{}°) err:{} spd:{} Iq_ref:{} Id:{} Iq:{} Ud:{} Uq:{} seq:{}",
                    enc_count,
                    target_pos,
                    actual_deg,
                    target_deg,
                    pos_error,
                    actual_rpm,
                    iq_ref_new,
                    id,
                    iq,
                    ud,
                    uq,
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
    let pin_a = unsafe { pico_rs::gpio::Pin::<10>::steal() };
    let pin_b = unsafe { pico_rs::gpio::Pin::<11>::steal() };
    pin_a.clear_all_interrupts();
    pin_b.clear_all_interrupts();

    let a = pin_a.get() as u32;
    let b = pin_b.get() as u32;
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
