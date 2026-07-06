//! # 无传感器 BLDC 电调 — 闭环 BEMF 过零检测 + PI 转速控制
//!
//! 基于 RP2350 (Raspberry Pi Pico 2) 的无传感器三相无刷直流电机闭环驱动。
//! 开环启动 + **BEMF 过零检测闭环换向** + **PI 速度控制器**，
//! 类似真实航模电调（ESC）但带有转速调节能力。
//!
//! 开环版本（无速度反馈）见 `bldc.rs`。
//!
//! ---
//!
//! # 1. 系统架构
//!
//! ```text
//!  ┌─────────────┐     ┌──────────────┐     ┌──────────────┐
//!  │  目标转速     │────▶│ PI 控制器     │────▶│  PWM 占空比  │
//!  │  TARGET_RPM  │     │  Kp=0.5      │     │  (0-100%)    │
//!  │  (常量或      │     │  Ki=0.05     │     └──────┬───────┘
//!  │   RC 输入)   │     └──────┬───────┘            │
//!  └─────────────┘            │                     ▼
//!                              │              ┌──────────────┐
//!  ┌─────────────┐            │              │  6 步 PWM     │
//!  │  实际转速     │◀───────────┘              │  换向         │
//!  │  (BEMF 测量)  │                          └──────┬───────┘
//!  └──────┬───────┘                                  │
//!         │                                          ▼
//!         │              ┌──────────────┐     ┌──────────────┐
//!         └──────────────│  BEMF 过零    │◀───┤  电机三相     │
//!                        │  检测 (ADC)   │    │  A, B, C     │
//!                        └──────────────┘    └──────────────┘
//! ```
//!
//! ---
//!
//! # 2. 核心架构总览
//!
//! | 模块 | 实现方式 | 说明 |
//! |------|---------|------|
//! | **换向逻辑** | 6 步换向表 | 高侧 PWM / 低侧导通 / 浮空相，与开环版相同 |
//! | **BEMF 采样** | GP26/27/28 ADC Ch0–2 | 电阻分压后采样浮空相电压 |
//! | **过零检测** | ADC 轮询 + 阈值比较 | 上升/下降方向由 `STEP_BEMF_RISING` 表确定 |
//! | **30° 延迟** | 步进周期 / 2 | 过零到最优换向时刻的等待 |
//! | **步进周期平滑** | EMA (α=0.25) | 防止噪声导致换向抖动 |
//! | **PI 控制器** | Q10 定点, Timer0 @100 Hz | 转速误差 → 占空比调整，带积分限幅 |
//! | **失步保护** | 连续 60 次无 ZC → 重启 | 关输出 → 等待 → 重置 PI → 开环重启 |
//! | **开环启动** | 80 电周期斜坡 | 加速到 BEMF 可检测转速后切换闭环 |
//! | **中断** | `TIMER0_IRQ_0` | PI 速度环 tick @100 Hz |
//!
//! ### 与其他驱动模式的对比
//!
//! | 区别点 | 开环 6 步 (`bldc.rs`) | BEMF 闭环（本文件） | 霍尔闭环 (`bldc_hall.rs`) |
//! |--------|-------------------|--------------------|-------------------------|
//! | 角度反馈 | 无 | BEMF ADC 过零 | 霍尔传感器 (GPIO 中断) |
//! | 零速启动 | 盲斜坡 | 开环斜坡过渡 | 直接换向 |
//! | 速度控制 | 无 | PI 闭环 | PI 闭环 |
//! | 额外硬件 | 无 | 3× 分压电阻 | 3× 霍尔传感器 |
//! | 低速性能 | 差 | 差（BEMF 弱） | 优秀 |
//!
//! ---
//!
//! # 3. 反电动势（BEMF）原理
//!
//! 电机旋转时，永磁体的运动在每个定子绕组上感应出一个与转速成正比的电压，
//! 这就是反电动势（Back-EMF）。
//!
//! 对于悬空相（未被驱动的那一相），BEMF 电压是一个近似梯形的波形。
//! 在每个换向步的中间点（30° 电角度处），BEMF 恰好穿越中性点电压（≈ VBAT/2）。
//! 这个穿越事件称为**过零点**（Zero Crossing, ZC）。
//!
//! ```text
//! 电压 ▲
//!      │    ╱╲         BEMF 波形（浮空相）
//!      │   ╱  ╲
//! VBAT │──╱────╲──── - - - - - ← BEMF 峰值
//!   /2 │─╱──────╲──── ZC ← 过零点（BEMF = VBAT/2）
//!      │╱        ╲
//!    0 └──────────╲──────────── → 时间
//!      │           ╲╱
//!      │
//!      ├──Step N──┼──Step N+1──┤
//!                 ▲
//!                 检测到 ZC 后等待 30° 再执行换向
//! ```
//!
//! ---
//!
//! # 4. BEMF 过零检测详解
//!
//! ## 4.1 浮空相选择
//!
//! 每个换向步中有一相是浮空的，`STEP_FLOAT` 表确定采样哪个 ADC 通道：
//! ```text
//! STEP_FLOAT = [2, 1, 0, 2, 1, 0]
//! // Step 0: C 浮空, Step 1: B 浮空, Step 2: A 浮空, ...
//! ```
//!
//! ## 4.2 过零方向
//!
//! `STEP_BEMF_RISING` 表确定 BEMF 是上升穿越还是下降穿越阈值：
//! ```text
//! STEP_BEMF_RISING = [false, true, false, true, false, true]
//! ```
//!
//! ## 4.3 检测逻辑
//!
//! ```text
//! 过零 = (上升沿预期 && 上次采样 < 阈值 && 当前采样 ≥ 阈值)
//!      || (下降沿预期 && 上次采样 > 阈值 && 当前采样 ≤ 阈值)
//! ```
//!
//! ---
//!
//! # 5. 换向定时与速度测量
//!
//! ## 5.1 30° 延迟
//!
//! 检测到过零点后，最佳换向时刻在 30° 电角度之后（即半个换向步周期）：
//!
//! ```text
//!   时间轴 →
//!   ─────┬──────────┬──────────┬─────
//!        │  Step N  │  Step N+1│
//!        │    ZC ───┤          │
//!        │    ├─30°─┤          │
//!        │    │delay│          │
//!        │    │     ↓ 执行换向  │
//!   ─────┴──────────┴──────────┴─────
//!
//!   delay_30deg = step_period_us / 2
//! ```
//!
//! ## 5.2 步进周期平滑
//!
//! 使用指数移动平均（EMA, α ≈ 0.25）避免噪声导致抖动：
//! ```text
//! step_period_new = step_period_old × 0.75 + measured_period × 0.25
//! ```
//!
//! ## 5.3 RPM 计算
//!
//! ```text
//! 电周期 = 6 × step_period
//! RPM = 60,000,000 / (电周期_µs × 极对数)
//! ```
//!
//! ---
//!
//! # 6. PI 速度控制器
//!
//! PI 控制器以 100 Hz 的固定速率运行（Timer0 Alarm 0 中断触发），
//! 将转速误差转换为 PWM 占空比调整量：
//!
//! ```text
//! ┌────────────────────────────────────────────────────────┐
//! │                   PI 控制器框图                         │
//! │                                                        │
//! │  target_rpm ──(+)──►┌──────┐                           │
//! │               ↑     │  Kp  ├──────────────┐            │
//! │          error│     └──────┘              │            │
//! │               │     ┌──────┐              ▼            │
//! │               ├────►│  Ki  ├─►[∫ clamp]──(+)──► duty  │
//! │               │     └──────┘                           │
//! │  actual_rpm ──(-)                                      │
//! │                                                        │
//! │  • Kp = 0.5  (快速响应)                                 │
//! │  • Ki = 0.05 (消除稳态误差)                              │
//! │  • 积分限幅: ±100,000 (防止 windup)                     │
//! │  • Duty 输出: 5% – 95%                                 │
//! └────────────────────────────────────────────────────────┘
//! ```
//!
//! **使用 Q10 定点运算**（所有增益 × 1024），避免浮点开销：
//! ```text
//! p_term = KP_FP * error                            // Q10
//! integral = (integral + KI_FP * error).clamp(...)   // Q10, 带限幅
//! output = (p_term + integral) >> 10                  // 转回整数
//! ```
//!
//! ## PI 参数调优步骤
//!
//! 1. **先调 Kp：** Ki 设为 0，从 Kp=0.1 开始逐步增大，直到转速能跟随但有稳态误差；
//!    振荡则减小。
//! 2. **再调 Ki：** 从 Ki=0.01 开始逐步增大直到稳态误差消除；
//!    低频振荡则减小。
//!
//! **症状诊断：**
//!
//! | 现象 | 可能原因 | 调整 |
//! |------|---------|------|
//! | 转速偏低，不到目标 | Ki 太小 | 增大 Ki |
//! | 转速在目标值附近振荡 | Kp 太大 | 减小 Kp |
//! | 加载后转速恢复慢 | Ki 太小 | 增大 Ki |
//! | 加载后过冲明显 | Ki 或 INTEGRAL_MAX 太大 | 减小 |
//! | 对油门变化响应迟钝 | Kp 太小 | 增大 Kp |
//!
//! ---
//!
//! # 7. 失步保护
//!
//! 如果连续 60 次换向步都未检测到过零点，认为电机已失步：
//! 1. 关闭所有输出（安全状态）
//! 2. 等待 500 ms（让电机停转）
//! 3. 重置 PI 控制器积分项
//! 4. 回到开环启动占空比
//! 5. 继续尝试 BEMF 检测
//!
//! ---
//!
//! # 8. 闭环控制工作流程
//!
//! ```text
//! ┌──────────────────────────────────────────────────────┐
//! │                 闭环转速控制流程                        │
//! ├──────────────────────────────────────────────────────┤
//! │                                                      │
//! │  阶段 1: 转子对齐 (500 ms, 10% 占空比)                │
//! │                                                      │
//! │  阶段 2: 开环启动斜坡 (80 电周期)                      │
//! │    • 换向间隔: 20 ms → 3 ms                           │
//! │    • 目的: 加速到 BEMF 可检测的速度                     │
//! │                                                      │
//! │  阶段 3: 闭环 BEMF + PI 控制                           │
//! │    ┌─────────────────────────────────────┐            │
//! │    │        主循环（每个换向步）           │            │
//! │    │  A. 换向 + 消隐等待 (100 µs)         │            │
//! │    │  B. ADC 轮询浮空相 BEMF → 检测过零   │            │
//! │    │  C. 等待 30° → 更新步进周期 → 算 RPM │            │
//! │    │  D. PI 控制器 (@100Hz) → 调整 duty  │            │
//! │    │  [失步保护: 60 次未检到 ZC → 重启]    │            │
//! │    └─────────────────────────────────────┘            │
//! └──────────────────────────────────────────────────────┘
//! ```
//!
//! ---
//!
//! # 9. 硬件接线
//!
//! ## 9.1 引脚分配
//!
//! | GPIO | 功能 | 外设 | 说明 |
//! |------|------|------|------|
//! | GP0 | Phase A 高侧 | PWM Slice 0 ChA | 20 kHz PWM |
//! | GP1 | Phase A 低侧 | GPIO SIO 输出 | 开关地回路 |
//! | GP2 | Phase B 高侧 | PWM Slice 1 ChA | 20 kHz PWM |
//! | GP3 | Phase B 低侧 | GPIO SIO 输出 | 开关地回路 |
//! | GP4 | Phase C 高侧 | PWM Slice 2 ChA | 20 kHz PWM |
//! | GP5 | Phase C 低侧 | GPIO SIO 输出 | 开关地回路 |
//! | GP26 | BEMF Phase A | ADC Channel 0 | 分压后采样 |
//! | GP27 | BEMF Phase B | ADC Channel 1 | 分压后采样 |
//! | GP28 | BEMF Phase C | ADC Channel 2 | 分压后采样 |
//!
//! ## 9.2 功率级与半桥拓扑
//!
//! 详见 `bldc.rs` 中的硬件电路说明。每相使用半桥（高侧 + 低侧 MOSFET），
//! 需要栅极驱动芯片（如 IR2104、FD6288、DRV8301）。
//!
//! > **⚠️ 安全警告：** 高侧 + 低侧同时导通会直通短路，烧毁 MOSFET。
//!
//! ## 9.3 BEMF 分压电路
//!
//! 每相需要一个电阻分压器，将电机端电压缩放到 ADC 安全范围（0–3.3 V）：
//!
//! ```text
//!     Phase Output ────┬───[R1]───┬─── GP26/27/28 (ADC)
//!                      │          │
//!                     [R2]       [C1]  (可选滤波电容 100pF-1nF)
//!                      │          │
//!     GND ─────────────┴──────────┘
//! ```
//!
//! **分压比计算：** `V_adc = V_phase × R2 / (R1 + R2)`
//!
//! | 电池电压 | R1 | R2 | 分压比 | 最大 ADC 电压 |
//! |----------|----|----|--------|--------------|
//! | 12V (3S) | 10 kΩ | 3.3 kΩ | 0.248 | 2.98 V |
//! | 22.2V (6S) | 22 kΩ | 3.3 kΩ | 0.130 | 2.89 V |
//! | 7.4V (2S) | 6.8 kΩ | 3.3 kΩ | 0.327 | 2.42 V |
//!
//! **BEMF 过零阈值计算：**
//! ```text
//! ADC_threshold = (VBAT/2) × (R2/(R1+R2)) × 4096 / 3.3
//! 示例 (12V, R1=10k, R2=3.3k): = 6.0 × 0.248 × 4096 / 3.3 ≈ 1845 ≈ 2048（代码默认）
//! ```
//!
//! ---
//!
//! # 10. 参数说明
//!
//! | 参数 | 默认值 | 说明 |
//! |------|--------|------|
//! | `TARGET_RPM` | 3,000 | 目标机械转速 (RPM) |
//! | `MOTOR_POLES` | 14 | 电机磁极数 |
//! | `STARTUP_DUTY_PERCENT` | 10 | 启动和失步恢复的占空比 (%) |
//! | `MIN_DUTY_PERCENT` | 5 | PI 输出下限 (%) |
//! | `MAX_DUTY_PERCENT` | 95 | PI 输出上限 (%) |
//! | `STARTUP_REVOLUTIONS` | 80 | 开环启动电周期数 |
//! | `STARTUP_INTERVAL_US_MIN` | 3,000 | 开环→闭环切换的换向间隔 (µs) |
//! | `BEMF_THRESHOLD` | 2048 | ADC 过零阈值 (12-bit, 0–4095) |
//! | `BLANKING_TIME_US` | 100 | 换向后消隐时间 (µs) |
//! | `KP_FP` | 512 | Kp × 1024 (Q10 定点) |
//! | `KI_FP` | 51 | Ki × 1024 (Q10 定点) |
//! | `INTEGRAL_MAX` | 100,000 | 积分项限幅 |
//! | `SPEED_LOOP_INTERVAL_US` | 10,000 | PI 运行间隔 = 100 Hz |
//!
//! ## 针对不同电机的调参建议
//!
//! **小型航模电机 (2204/2206, 2300KV)：**
//! ```text
//! MOTOR_POLES=14, STARTUP_DUTY=8%, STARTUP_REVOLUTIONS=40, TARGET_RPM=5000
//! ```
//!
//! **中型电机 (5010/5020, 360KV)：**
//! ```text
//! MOTOR_POLES=12, STARTUP_DUTY=15%, STARTUP_REVOLUTIONS=100,
//! STARTUP_INTERVAL_US_MAX=30000, TARGET_RPM=2000
//! ```
//!
//! **电动工具电机 (高速小极数)：**
//! ```text
//! MOTOR_POLES=4, PWM_FREQ_HZ=32000, STARTUP_INTERVAL_US_MIN=1000, TARGET_RPM=15000
//! ```
//!
//! ## BEMF 检测调优
//!
//! - **消隐时间 (`BLANKING_TIME_US`)：** 太短→采到开关噪声误触发；太长→错过过零点。
//!   建议范围 30–200 µs，高速失步时尝试减小。
//! - **阈值 (`BEMF_THRESHOLD`)：** 理论值 = VBAT/2 × 分压比 × 4096 / 3.3。
//!   实际可用 2048（中间值）通常效果不错。
//! - **开环→闭环切换 (`STARTUP_INTERVAL_US_MIN`)：** 高 KV 电机 3000–5000 µs；
//!   低 KV 电机可能需要 1000–2000 µs。启动后立即失步则增大 `STARTUP_REVOLUTIONS`。
//!
//! ---
//!
//! # 11. 常见问题
//!
//! **Q: 电机启动后立即停转**
//! - BEMF 检测失败 → 检查 ADC 接线和分压比
//! - 阈值不对 → 在开环模式下用 ADC 采样观察 BEMF 信号
//! - 消隐时间太长 → 减小 `BLANKING_TIME_US`
//!
//! **Q: 电机低速时抽搐**
//! - BEMF 信号太弱 → 降低 `STARTUP_INTERVAL_US_MIN` 让闭环切换在更高速度
//! - ADC 噪声 → 添加硬件滤波电容，或软件多次采样平均
//!
//! **Q: 电机高速时声音异常**
//! - PWM 频率在可听范围 → 确保 ≥ 18 kHz
//! - 换向抖动 → 增大 EMA 平滑系数（代码中 3:1 → 尝试 7:1）
//! - PI 振荡 → 减小 Kp
//!
//! **Q: 如何接入 RC 接收机油门？**
//!
//! 用 PWM 输入模式测量 RC PPM 信号（1000–2000 µs 脉宽），映射到 `TARGET_RPM`。
//!
//! ---
//!
//! # 12. defmt 日志输出示例
//!
//! ```text
//! INFO  BLDC ESC — closed-loop BEMF speed control
//! INFO  System clock: 150 MHz
//! INFO  PWM wrap = 7500 (20 kHz)
//! INFO  Phase 1: Rotor alignment (500 ms hold)...
//! INFO  Phase 2: Open-loop ramp (80 electrical revolutions)...
//! INFO  Phase 2 complete — transitioning to closed-loop BEMF control
//! INFO  Phase 3: Target 3000 RPM, 14 poles (7 pairs), PI Kp=0.5, Ki=0.05
//! INFO  RPM: 2847 (target 3000), duty: 28%, step: 1923us, misses: 0
//! INFO  RPM: 3002 (target 3000), duty: 25%, step: 1900us, misses: 0
//! ```
//!
//! ---
//!
//! # 13. 编译运行
//!
//! ```bash
//! cd pico-rs-examples
//! cargo run --bin bldc_speed_ctrl --target thumbv8m.main-none-eabihf
//! ```
//!
//! ---
//!
//! # 14. 参考资料
//!
//! - [RP2350 Datasheet](https://datasheets.raspberrypi.com/rp2350/rp2350-datasheet.pdf)
//! - AN1078 (Microchip) — *Sensorless BLDC Motor Control Using Back-EMF Zero Crossing*
//! - AN885 (Microchip) — *Brushless DC (BLDC) Motor Fundamentals*
//! - TI SPRABQ7 — *Sensorless Trapezoidal Control of BLDC Motors*
//!
//! ## 关键术语
//!
//! | 术语 | 英文 | 说明 |
//! |------|------|------|
//! | 反电动势 | Back-EMF / BEMF | 电机旋转时绕组感应电压 |
//! | 过零点 | Zero Crossing (ZC) | BEMF 穿越中性点的时刻 |
//! | 换向 | Commutation | 切换通电相的动作 |
//! | 消隐时间 | Blanking Time | 换向后忽略 ADC 采样的时间窗 |
//! | 电角度 | Electrical Angle | 一个电周期 = 360°，对应 6 步 |
//! | 极对数 | Pole Pairs | 磁极数 / 2 |
//! | KV 值 | Motor KV | 每伏特空载转速 (RPM/V) |
//! | 失步 | Desynchronization | 驱动磁场与转子失去同步 |
//! | 直通 | Shoot-through | 同相高低侧 FET 同时导通（短路） |
//! | 死区时间 | Dead Time | 高低侧切换间的安全间隔 |

#![no_std]
#![no_main]

use core::cell::RefCell;
use core::sync::atomic::{AtomicBool, AtomicU32, Ordering};

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
// Configuration — Tune these for your motor and application
// ============================================================================

/// PWM switching frequency (20 kHz — above audible range)
const PWM_FREQ_HZ: u32 = 20_000;

/// Target mechanical speed in RPM
const TARGET_RPM: u32 = 3000;

/// Number of motor poles (permanent magnets). Common: 4, 6, 8, 12, 14
const MOTOR_POLES: u32 = 14;

/// Startup duty cycle (% — keep low to avoid stalling)
const STARTUP_DUTY_PERCENT: u16 = 10;

/// Duty limits during closed-loop control (%)
const MIN_DUTY_PERCENT: u16 = 5;
const MAX_DUTY_PERCENT: u16 = 95;

/// Open-loop startup ramp: number of full electrical revolutions
const STARTUP_REVOLUTIONS: u32 = 80;

/// Open-loop ramp: initial and final commutation interval (µs)
const STARTUP_INTERVAL_US_MAX: u32 = 20_000; // very slow start
const STARTUP_INTERVAL_US_MIN: u32 = 3_000; // hand-off to closed-loop

/// ADC threshold for BEMF zero crossing.
/// With 12-bit ADC (0–4095), midpoint ≈ 2048 corresponds to VBAT/2.
const BEMF_THRESHOLD: u16 = 2048;

/// Blanking time after each commutation (µs).
/// Switching transients must settle before ADC sampling is meaningful.
const BLANKING_TIME_US: u64 = 100;

/// PI controller gains in Q10 fixed-point (value × 1024).
///   Kp = 0.5  → 512
///   Ki = 0.05 → 51
const KP_FP: i32 = 512;
const KI_FP: i32 = 51;
const FP_SHIFT: i32 = 10; // right-shift to convert back

/// Anti-windup clamp for the integral term
const INTEGRAL_MAX: i32 = 100_000;

/// Speed control loop period (µs) — timer alarm interval
const SPEED_LOOP_INTERVAL_US: u32 = 10_000; // 100 Hz

// ============================================================================
// 6-step commutation tables
// ============================================================================

/// Phase driven HIGH (PWM): 0=A, 1=B, 2=C
const STEP_HIGH: [u8; 6] = [0, 0, 1, 1, 2, 2];

/// Phase driven LOW (GND)
const STEP_LOW: [u8; 6] = [1, 2, 2, 0, 0, 1];

/// Phase FLOATING (for BEMF sampling) — maps to ADC channel index
///   Phase A → ADC ch 0 (GP26)
///   Phase B → ADC ch 1 (GP27)
///   Phase C → ADC ch 2 (GP28)
const STEP_FLOAT: [u8; 6] = [2, 1, 0, 2, 1, 0];

/// Expected BEMF direction at each step:
///   true  = BEMF rising through threshold
///   false = BEMF falling through threshold
const STEP_BEMF_RISING: [bool; 6] = [false, true, false, true, false, true];

// ============================================================================
// Global shared state (main loop ↔ timer ISR)
// ============================================================================

/// Timer peripheral — shared via Mutex for ISR access
static TIMER: Mutex<RefCell<Option<Timer0>>> = Mutex::new(RefCell::new(None));

/// Speed-loop tick flag — set by timer ISR, consumed by main loop
static SPEED_LOOP_TICK: AtomicBool = AtomicBool::new(false);

/// Measured mechanical RPM — updated by main loop
static ACTUAL_RPM: AtomicU32 = AtomicU32::new(0);

// ============================================================================
// Entry
// ============================================================================

#[main]
fn main() -> ! {
    info!("BLDC ESC — closed-loop BEMF speed control");

    // ------------------------------------------------------------------
    // Peripheral & clock init
    // ------------------------------------------------------------------
    let dp = Peripherals::take().unwrap();
    let clocks = ClockBuilder::new().build(dp.CLOCKS, dp.XOSC, dp.PLL_SYS, dp.PLL_USB);
    let sys_freq = clocks.sys_freq();
    info!("System clock: {} MHz", sys_freq / 1_000_000);

    let pins = Pins::new(dp.IO_BANK0, dp.PADS_BANK0);
    let timer = Timer0::init(dp.TIMER0);

    // ------------------------------------------------------------------
    // ADC setup — BEMF voltage dividers on GP26 / GP27 / GP28
    // ------------------------------------------------------------------
    let mut adc = Adc::new(dp.ADC);
    let bemf_a = pins.pin::<26>().into_adc_input(); // ADC ch 0 — Phase A
    let bemf_b = pins.pin::<27>().into_adc_input(); // ADC ch 1 — Phase B
    let bemf_c = pins.pin::<28>().into_adc_input(); // ADC ch 2 — Phase C

    // ------------------------------------------------------------------
    // PWM setup — 20 kHz on slices 0, 1, 2 (channel A each)
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
    // Low-side GPIO outputs (directly switching the low-side FETs)
    // ------------------------------------------------------------------
    let al = pins.pin::<1>();
    al.into_output();
    al.set_low();

    let bl = pins.pin::<3>();
    bl.into_output();
    bl.set_low();

    let cl = pins.pin::<5>();
    cl.into_output();
    cl.set_low();

    // ------------------------------------------------------------------
    // Helper closures
    // ------------------------------------------------------------------

    // Convert duty percentage (0–100) to PWM compare value
    let duty_from_percent =
        |percent: u16| -> u16 { ((pwm_wrap as u32) * (percent as u32) / 100) as u16 };

    // Apply one 6-step commutation state
    let commutate = |step: usize, duty: u16| {
        let high = STEP_HIGH[step];
        let low = STEP_LOW[step];

        // High-side: PWM only on the active phase, others off
        slice_a.set_duty_a(if high == 0 { duty } else { 0 });
        slice_b.set_duty_a(if high == 1 { duty } else { 0 });
        slice_c.set_duty_a(if high == 2 { duty } else { 0 });

        // Low-side: one phase grounded, others floating
        if low == 0 {
            al.set_high();
        } else {
            al.set_low();
        }
        if low == 1 {
            bl.set_high();
        } else {
            bl.set_low();
        }
        if low == 2 {
            cl.set_high();
        } else {
            cl.set_low();
        }
    };

    // Disable all outputs — safe state
    let all_off = || {
        slice_a.set_duty_a(0);
        slice_b.set_duty_a(0);
        slice_c.set_duty_a(0);
        al.set_low();
        bl.set_low();
        cl.set_low();
    };

    // Read BEMF ADC for the floating phase at this commutation step
    let mut read_bemf = |step: usize| -> u16 {
        match STEP_FLOAT[step] {
            0 => adc.read(&bemf_a),
            1 => adc.read(&bemf_b),
            _ => adc.read(&bemf_c),
        }
    };

    // Detect if a zero crossing occurred between `prev` and `sample`
    let is_zero_crossing = |step: usize, sample: u16, prev: u16| -> bool {
        if STEP_BEMF_RISING[step] {
            prev < BEMF_THRESHOLD && sample >= BEMF_THRESHOLD
        } else {
            prev > BEMF_THRESHOLD && sample <= BEMF_THRESHOLD
        }
    };

    // ------------------------------------------------------------------
    // Timer interrupt setup — periodic tick for the PI speed loop
    // ------------------------------------------------------------------
    {
        let now = timer.get_time_us_32();
        timer.set_alarm(0, now.wrapping_add(SPEED_LOOP_INTERVAL_US));
        timer.enable_alarm_interrupt(0);

        // Move timer into the global Mutex so the ISR can re-arm it
        interrupt::free(|cs| {
            TIMER.borrow(cs).replace(Some(timer));
        });

        unsafe {
            interrupt::enable(Interrupt::TIMER0_IRQ_0);
        }
    }

    // Timer access wrappers (go through the Mutex)
    let timer_delay_us = |us: u64| {
        interrupt::free(|cs| {
            if let Some(ref t) = *TIMER.borrow(cs).borrow() {
                t.delay_us(us);
            }
        });
    };
    let timer_delay_ms = |ms: u32| {
        interrupt::free(|cs| {
            if let Some(ref t) = *TIMER.borrow(cs).borrow() {
                t.delay_ms(ms);
            }
        });
    };
    let timer_now_us = || -> u32 {
        interrupt::free(|cs| {
            TIMER
                .borrow(cs)
                .borrow()
                .as_ref()
                .map_or(0, |t| t.get_time_us_32())
        })
    };

    // ------------------------------------------------------------------
    // ===== PHASE 1: Rotor Alignment =====
    // Lock the rotor to step 0 so we know the starting position
    // ------------------------------------------------------------------
    info!("Phase 1: Rotor alignment (500 ms hold)...");
    let align_duty = duty_from_percent(STARTUP_DUTY_PERCENT);
    commutate(0, align_duty);
    timer_delay_ms(500);

    // ------------------------------------------------------------------
    // ===== PHASE 2: Open-Loop Startup Ramp =====
    // Gradually accelerate with fixed duty until BEMF is strong enough
    // to detect zero crossings reliably.
    // ------------------------------------------------------------------
    info!(
        "Phase 2: Open-loop ramp ({} electrical revolutions)...",
        STARTUP_REVOLUTIONS
    );
    let startup_duty = duty_from_percent(STARTUP_DUTY_PERCENT);
    let total_ramp_steps = STARTUP_REVOLUTIONS * 6;
    let mut step: usize = 0;

    for i in 0..total_ramp_steps {
        // Linear interpolation: interval decreases as i increases
        let interval = STARTUP_INTERVAL_US_MAX
            - (STARTUP_INTERVAL_US_MAX - STARTUP_INTERVAL_US_MIN) * i / total_ramp_steps;

        commutate(step, startup_duty);
        timer_delay_us(interval as u64);
        step = (step + 1) % 6;
    }

    info!("Phase 2 complete — transitioning to closed-loop BEMF control");

    // ------------------------------------------------------------------
    // ===== PHASE 3: Closed-Loop BEMF + PI Speed Control =====
    //
    // Main control loop:
    //   1. Commutate to current step
    //   2. Wait blanking period
    //   3. Poll BEMF on floating phase until zero crossing
    //   4. Wait 30° (half step period) then advance step
    //   5. On PI tick: compute RPM error → adjust duty
    // ------------------------------------------------------------------
    let pole_pairs = MOTOR_POLES / 2;

    // PI controller state
    let mut integral: i32 = 0;
    let mut current_duty_percent: u16 = STARTUP_DUTY_PERCENT;
    let mut current_duty: u16 = duty_from_percent(current_duty_percent);

    // Timing state
    let mut last_zc_time: u32 = timer_now_us();
    let mut step_period_us: u32 = STARTUP_INTERVAL_US_MIN;
    let mut prev_bemf: u16 = BEMF_THRESHOLD;
    let mut zc_count: u32 = 0;
    let mut consecutive_misses: u32 = 0;

    info!(
        "Phase 3: Target {} RPM, {} poles ({} pairs), PI Kp={}, Ki={}",
        TARGET_RPM,
        MOTOR_POLES,
        pole_pairs,
        KP_FP as f32 / 1024.0,
        KI_FP as f32 / 1024.0,
    );

    loop {
        // ==============================================================
        // Step A: Commutate & blanking
        // ==============================================================
        commutate(step, current_duty);
        timer_delay_us(BLANKING_TIME_US);

        // ==============================================================
        // Step B: BEMF zero-crossing detection (polling)
        // ==============================================================
        let timeout = step_period_us * 2; // safety: 2× expected period
        let poll_start = timer_now_us();
        let mut zc_detected = false;

        loop {
            let sample = read_bemf(step);

            if is_zero_crossing(step, sample, prev_bemf) {
                zc_detected = true;
                prev_bemf = sample;
                break;
            }
            prev_bemf = sample;

            // Timeout — no ZC found, fall back to open-loop timing
            if timer_now_us().wrapping_sub(poll_start) > timeout {
                break;
            }
        }

        // ==============================================================
        // Step C: Timing update & 30° delay
        // ==============================================================
        if zc_detected {
            consecutive_misses = 0;
            let now = timer_now_us();
            let zc_period = now.wrapping_sub(last_zc_time);
            last_zc_time = now;
            zc_count += 1;

            // Exponential moving average (α ≈ 0.25)
            step_period_us = (step_period_us * 3 + zc_period) / 4;

            // Wait 30 electrical degrees before commutating
            // (ZC occurs at the midpoint → wait half a step)
            let delay_30deg = step_period_us / 2;
            timer_delay_us(delay_30deg as u64);

            // Update RPM every electrical revolution (6 ZCs)
            if zc_count.is_multiple_of(6) {
                let elec_period_us = step_period_us as u64 * 6;
                if elec_period_us > 0 {
                    let rpm = 60_000_000u64 / (elec_period_us * pole_pairs as u64);
                    ACTUAL_RPM.store(rpm as u32, Ordering::Relaxed);
                }
            }
        } else {
            // Open-loop fallback: use predicted timing
            consecutive_misses += 1;
            let elapsed = timer_now_us().wrapping_sub(poll_start);
            let remaining = step_period_us.saturating_sub(elapsed);
            if remaining > 0 {
                timer_delay_us(remaining as u64);
            }

            // If too many consecutive misses, motor may have stalled
            if consecutive_misses > 60 {
                warn!("Lost sync — too many missed ZC, restarting...");
                all_off();
                timer_delay_ms(500);
                // Reset state and restart at open-loop
                consecutive_misses = 0;
                step_period_us = STARTUP_INTERVAL_US_MIN;
                current_duty_percent = STARTUP_DUTY_PERCENT;
                current_duty = duty_from_percent(current_duty_percent);
                integral = 0;
            }
        }

        // Advance to next commutation step
        step = (step + 1) % 6;

        // ==============================================================
        // Step D: PI speed controller (runs at 100 Hz)
        // ==============================================================
        if SPEED_LOOP_TICK.swap(false, Ordering::Relaxed) {
            let actual_rpm = ACTUAL_RPM.load(Ordering::Relaxed);
            let error = TARGET_RPM as i32 - actual_rpm as i32;

            // --- Proportional ---
            let p_term = KP_FP * error;

            // --- Integral with anti-windup ---
            integral = (integral + KI_FP * error).clamp(-INTEGRAL_MAX, INTEGRAL_MAX);

            // --- PI output → duty adjustment ---
            let output = (p_term + integral) >> FP_SHIFT;

            let new_duty = (current_duty_percent as i32 + output)
                .clamp(MIN_DUTY_PERCENT as i32, MAX_DUTY_PERCENT as i32)
                as u16;

            if new_duty != current_duty_percent {
                current_duty_percent = new_duty;
                current_duty = duty_from_percent(current_duty_percent);
            }

            // Periodic status log (every ~1 second)
            if zc_count.is_multiple_of(600) && zc_count > 0 {
                info!(
                    "RPM: {} (target {}), duty: {}%, step: {}us, misses: {}",
                    actual_rpm,
                    TARGET_RPM,
                    current_duty_percent,
                    step_period_us,
                    consecutive_misses
                );
            }
        }
    }
}

// ============================================================================
// Timer ISR — sets the speed-loop tick flag at 100 Hz
// ============================================================================

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
fn TIMER0_IRQ_0() {
    interrupt::free(|cs| {
        if let Some(ref timer) = *TIMER.borrow(cs).borrow() {
            timer.clear_alarm_interrupt(0);
            SPEED_LOOP_TICK.store(true, Ordering::Relaxed);
            let now = timer.get_time_us_32();
            timer.set_alarm(0, now.wrapping_add(SPEED_LOOP_INTERVAL_US));
        }
    });
}
