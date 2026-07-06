//! # 带霍尔传感器的 BLDC 电调 — 闭环换向 + PI 转速控制
//!
//! 基于 RP2350 (Raspberry Pi Pico 2) 的**有传感器**三相无刷直流电机驱动。
//! 使用 3 个霍尔传感器（120° 间隔）确定转子位置，实现精确换向和 PI 转速控制。
//!
//! 无传感器版本见 `bldc.rs`（开环）和 `bldc_speed_ctrl.rs`（BEMF 闭环）。
//!
//! ---
//!
//! # 1. 霍尔传感器 vs 无传感器
//!
//! | 特性 | 霍尔传感器 | 无传感器 (BEMF) |
//! |------|-----------|----------------|
//! | 零速启动 | ✅ 可直接启动 | ❌ 需开环盲启 |
//! | 低速性能 | ✅ 优秀 | ❌ BEMF 太弱 |
//! | 成本 | 稍高（3 个霍尔） | 低（仅电阻分压） |
//! | 接线 | 多 3 根信号线 | 少 |
//! | 换向精度 | 固定 60° 分辨率 | 连续过零检测 |
//! | 应用场景 | 电动车、云台、工业 | 航模、风扇 |
//!
//! ---
//!
//! # 2. 霍尔传感器工作原理
//!
//! 3 个霍尔传感器（HA, HB, HC）以 120° 电角度间隔安装在定子上。
//! 当转子永磁体经过时，霍尔传感器输出高或低电平。
//!
//! 3 个霍尔信号组合成 3-bit 编码（`hall_state = HA<<2 | HB<<1 | HC`），
//! 共有 6 个有效状态（001~110），对应 6 个换向步：
//!
//! ```text
//!  电角度   0°   60°  120°  180°  240°  300°  360°
//!          ┌────┬────┬────┬────┬────┬────┐
//!   HA     │ 1  │ 1  │ 1  │ 0  │ 0  │ 0  │
//!          ├────┼────┼────┼────┼────┼────┤
//!   HB     │ 0  │ 0  │ 1  │ 1  │ 1  │ 0  │
//!          ├────┼────┼────┼────┼────┼────┤
//!   HC     │ 1  │ 0  │ 0  │ 0  │ 1  │ 1  │
//!          └────┴────┴────┴────┴────┴────┘
//!   State:   5    4    6    2    3    1
//!          ═══════════════════════════════
//!   Step:    0    1    2    3    4    5
//! ```
//!
//! 注意：霍尔编码顺序因电机而异（取决于霍尔安装位置和磁极排列）。
//! 如果电机不转或反转，需要调整 `HALL_TO_STEP` 映射表。
//!
//! ---
//!
//! # 3. 系统架构
//!
//! ```text
//!  ┌─────────────┐     ┌──────────────┐     ┌──────────────┐
//!  │  目标转速     │────▶│ PI 控制器     │────▶│  PWM 占空比  │
//!  │  TARGET_RPM  │     │  @100Hz      │     │  (0-100%)    │
//!  └─────────────┘     └──────┬───────┘     └──────┬───────┘
//!                              │                    │
//!  ┌─────────────┐            │                    ▼
//!  │  实际转速     │◀───────────┘          ┌──────────────────┐
//!  │  (霍尔计时)   │                      │  6 步 PWM 换向    │
//!  └──────┬───────┘                      │  (由霍尔状态决定) │
//!         │                              └────────┬─────────┘
//!         │                                       │
//!    ┌────┴────┐                                  ▼
//!    │ Timer0  │                          ┌──────────────────┐
//!    │ 周期测量 │   ┌──────────────┐      │  电机 A, B, C    │
//!    └─────────┘   │ 霍尔传感器    │◀─────┤  + 永磁转子      │
//!                  │ HA, HB, HC   │      └──────────────────┘
//!                  └──────┬───────┘
//!                         │ GPIO 中断
//!                         │ (边沿触发)
//!                         ▼
//!                  ┌──────────────┐
//!                  │ IO_IRQ_BANK0 │
//!                  │ → 读取霍尔态  │
//!                  │ → 执行换向    │
//!                  │ → 记录时间戳  │
//!                  └──────────────┘
//! ```
//!
//! **核心流程：**
//! - 霍尔信号变化 → GPIO 中断 → 立即读取 3-bit 霍尔编码 → 查表换向 → 记录时间
//! - 主循环以 100Hz 运行 PI 控制器，根据霍尔换向间隔计算 RPM 并调整占空比
//!
//! ---
//!
//! # 4. 核心架构总览
//!
//! | 模块 | 实现方式 | 说明 |
//! |------|---------|------|
//! | **霍尔检测** | GP6/7/8 GPIO 中断 (双边沿) | 3-bit 编码 `HA<<2\|HB<<1\|HC`，6 个有效状态 |
//! | **换向逻辑** | `HALL_TO_STEP[8]` 查表 | 霍尔编码 → 6 步换向，GPIO 中断中立即执行 |
//! | **PWM 调速** | PWM Slice 0/1/2 @ 20 kHz | 高侧 PWM 控制力矩 |
//! | **低侧开关** | GP1/3/5 GPIO 输出 | 根据换向步开/关，提供电流回路 |
//! | **转速测量** | 霍尔中断时间差 + EMA | 步进周期 → RPM，α=0.25 指数平滑 |
//! | **PI 控制器** | Q10 定点, Timer0 @100 Hz | RPM 误差 → 占空比调整，带积分限幅 |
//! | **零速启动** | 直接读霍尔 → 正确换向 | 无需开环斜坡，上电即转 |
//! | **中断** | `IO_IRQ_BANK0` + `TIMER0_IRQ_0` | 霍尔换向 @GPIO + 速度 PI @100 Hz |
//!
//! ### 与其他驱动模式的对比
//!
//! | 区别点 | 霍尔 6 步（本文件） | 无传感器 BEMF (`bldc_speed_ctrl.rs`) | FOC 电压模式 (`bldc_foc.rs`) |
//! |--------|-------------------|-------------------------------|-------------------------|
//! | 电流波形 | 方波/梯形 | 方波/梯形 | 正弦波 |
//! | 换向精度 | 60° 固定 | 连续过零 | 连续角度 |
//! | 力矩脉动 | 大 | 大 | 极小 |
//! | 零速启动 | 支持 | 不支持 | 支持 |
//! | 硬件需求 | 3× 霍尔 | 3× 分压电阻 | 3× 霍尔 + SVPWM |
//!
//! ---
//!
//! # 5. 换向表详解
//!
//! 霍尔 3-bit 编码 → 换向步骤的映射（`HALL_TO_STEP`）：
//!
//! ```text
//!  Hall State │ HA │ HB │ HC │ 换向步 │ High │ Low │ Float │ 电流
//!  ───────────┼────┼────┼────┼───────┼──────┼─────┼───────┼──────
//!      5      │  1 │  0 │  1 │   0   │  A   │  B  │   C   │ A→B
//!      4      │  1 │  0 │  0 │   1   │  A   │  C  │   B   │ A→C
//!      6      │  1 │  1 │  0 │   2   │  B   │  C  │   A   │ B→C
//!      2      │  0 │  1 │  0 │   3   │  B   │  A  │   C   │ B→A
//!      3      │  0 │  1 │  1 │   4   │  C   │  A  │   B   │ C→A
//!      1      │  0 │  0 │  1 │   5   │  C   │  B  │   A   │ C→B
//!    0 / 7    │  — │  — │  — │ 无效  │  —   │  —  │   —   │ 错误
//! ```
//!
//! > **重要：** 以上映射是最常见的 120° 霍尔排列。如果电机反转或不转，
//! > 交换任意两相的接线，或修改 `HALL_TO_STEP` 表。
//!
//! ---
//!
//! # 6. 转速测量
//!
//! 每次霍尔中断记录时间戳，两次中断的时间差 = 一个换向步周期（60° 电角度）：
//!
//! ```text
//! 换向步周期 (µs) = T_now - T_last
//! 电周期 = 换向步周期 × 6
//! RPM = 60,000,000 / (电周期_µs × 极对数)
//! ```
//!
//! 使用指数移动平均（EMA, α=0.25）平滑：
//! ```text
//! step_period = step_period_old × 0.75 + measured × 0.25
//! ```
//!
//! ---
//!
//! # 7. PI 速度控制器
//!
//! 与 `bldc_speed_ctrl.rs` 相同的 Q10 定点 PI 控制器，100Hz 运行：
//!
//! ```text
//! error = target_rpm - actual_rpm
//! P = Kp × error
//! I += Ki × error  (限幅防 windup)
//! duty_adjust = (P + I) >> 10
//! ```
//!
//! 详细调参方法见 `bldc_speed_ctrl.rs` 文档。
//!
//! ---
//!
//! # 8. 优势：零速启动
//!
//! 与无传感器版本不同，霍尔版本**不需要开环启动斜坡**：
//! 1. 上电后立即读取霍尔状态 → 知道转子位置
//! 2. 直接按正确的换向步通电 → 产生力矩
//! 3. 转子开始旋转 → 霍尔信号变化 → 自动换向
//!
//! 这使得霍尔版本特别适合需要精确起停控制的应用（电动车、机器人、云台等）。
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
//! | GP6 | Hall A (HA) | GPIO 输入 + 中断 | 上拉，双边沿触发 |
//! | GP7 | Hall B (HB) | GPIO 输入 + 中断 | 上拉，双边沿触发 |
//! | GP8 | Hall C (HC) | GPIO 输入 + 中断 | 上拉，双边沿触发 |
//!
//! ## 9.2 霍尔传感器接线
//!
//! 霍尔传感器通常为开漏或推挽输出，供电 3.3V 或 5V（5V 需串联限流电阻或电平转换）。
//!
//! ```text
//!   3.3V ───┬───────────┬───────────┬──────────
//!           │           │           │
//!        [10kΩ]      [10kΩ]      [10kΩ]    ← 上拉电阻（开漏输出时需要）
//!           │           │           │
//!    HA ────┼─── GP6    │           │
//!           │    HB ────┼─── GP7    │
//!           │           │    HC ────┼─── GP8
//!           │           │           │
//!   Motor Hall Sensors (3.3V supply)
//! ```
//!
//! > **注意：** 如果霍尔传感器是 5V 供电，需要用电阻分压或电平转换器将输出
//! > 限制在 3.3V 以内，RP2350 的 GPIO 不耐 5V。
//!
//! ## 9.3 功率级
//!
//! 与 `bldc.rs` 完全相同：3 个半桥（6 个 MOSFET + 栅极驱动），详见其文档。
//!
//! ---
//!
//! # 10. 参数说明
//!
//! | 参数 | 默认值 | 说明 |
//! |------|--------|------|
//! | `TARGET_RPM` | 3,000 | 目标机械转速 (RPM) |
//! | `MOTOR_POLES` | 14 | 电机磁极数 |
//! | `STARTUP_DUTY_PERCENT` | 15 | 初始占空比 (%) |
//! | `MIN_DUTY_PERCENT` | 5 | PI 输出下限 (%) |
//! | `MAX_DUTY_PERCENT` | 95 | PI 输出上限 (%) |
//! | `KP_FP` | 512 | Kp × 1024 (Q10 定点) |
//! | `KI_FP` | 51 | Ki × 1024 (Q10 定点) |
//! | `INTEGRAL_MAX` | 100,000 | 积分限幅 |
//! | `SPEED_LOOP_INTERVAL_US` | 10,000 | PI 运行间隔 = 100 Hz |
//!
//! ## 霍尔映射表调整
//!
//! 如果电机不转或抖动，可能需要调整 `HALL_TO_STEP` 表。方法：
//! 1. 手动缓慢转动电机，通过 defmt 日志观察霍尔状态变化顺序
//! 2. 对照上面的理论表，重新排列映射
//! 3. 如果方向反了，将换向步 0↔5, 1↔4, 2↔3 互换即可
//!
//! ---
//!
//! # 11. 常见问题
//!
//! **Q: 电机抖动不转**
//! - 霍尔接线顺序不对 → 交换接线或修改 `HALL_TO_STEP`
//! - 霍尔供电问题 → 检查 3.3V 供电和上拉电阻
//! - 霍尔读数都是 0 或 7 → 接线断开或传感器损坏
//!
//! **Q: 电机反转**
//! - 交换任意两相电机线（如 A↔B）
//! - 或交换 `HALL_TO_STEP` 表中对应的步序
//!
//! **Q: 低速时换向不平滑**
//! - 增大 `STARTUP_DUTY_PERCENT`
//! - 检查霍尔信号是否有噪声（加去抖滤波电容 10-100nF）
//!
//! **Q: 与无传感器版本的选择？**
//! - 需要零速启动、低速控制、精确停止 → 用霍尔版
//! - 追求低成本、高速应用 → 用无传感器版
//! - 要求最佳性能 → 霍尔启动 + BEMF 闭环运行（混合方案，本例未实现）
//!
//! ---
//!
//! # 12. 编译运行
//!
//! ```bash
//! cd pico-rs-examples
//! cargo run --bin bldc_hall --target thumbv8m.main-none-eabihf
//! ```
//!
//! ---
//!
//! # 13. 参考资料
//!
//! - [RP2350 Datasheet](https://datasheets.raspberrypi.com/rp2350/rp2350-datasheet.pdf)
//! - AN1078 (Microchip) — *Sensorless BLDC Motor Control*
//! - AN885 (Microchip) — *Brushless DC Motor Fundamentals*
//! - TI DRV8301 Application Note — *3-Phase BLDC Motor Drive with Hall Sensors*

#![no_std]
#![no_main]

use core::cell::RefCell;
use core::sync::atomic::{AtomicBool, AtomicU8, AtomicU16, AtomicU32, Ordering};

use defmt::*;
use defmt_rtt as _;

use pico_rs::clocks::ClockBuilder;
use pico_rs::main;
use pico_rs::gpio::{EdgeLevel, Pin, Pins};
use pico_rs::interrupt::{self, Mutex};
use pico_rs::pac::{Interrupt, Peripherals};
use pico_rs::pwm::Pwm;
use pico_rs::timer::Timer0;// ============================================================================
// 配置 — 根据电机和应用调整
// ============================================================================

/// PWM 开关频率（20 kHz，不可闻）
const PWM_FREQ_HZ: u32 = 20_000;

/// 目标机械转速 (RPM)
const TARGET_RPM: u32 = 3000;

/// 电机磁极数（永磁体数量）。常见：4, 6, 8, 12, 14
const MOTOR_POLES: u32 = 14;

/// 初始占空比 (%) — 上电后立即以此占空比通电
const STARTUP_DUTY_PERCENT: u16 = 15;

/// PI 控制占空比范围 (%)
const MIN_DUTY_PERCENT: u16 = 5;
const MAX_DUTY_PERCENT: u16 = 95;

/// PI 控制器增益 (Q10 定点，值 × 1024)
///   Kp = 0.5  → 512
///   Ki = 0.05 → 51
const KP_FP: i32 = 512;
const KI_FP: i32 = 51;
const FP_SHIFT: i32 = 10;

/// 积分项限幅（防止 windup）
const INTEGRAL_MAX: i32 = 100_000;

/// PI 速度控制循环周期 (µs) — Timer0 Alarm 中断间隔
const SPEED_LOOP_INTERVAL_US: u32 = 10_000; // 100 Hz

/// 最大允许的换向步周期 (µs) — 超过此值认为电机几乎停止
const MAX_STEP_PERIOD_US: u32 = 200_000; // 对应极低转速

// ============================================================================
// 6步换向表
// ============================================================================

/// 每步哪相驱动高（PWM）：0=A, 1=B, 2=C
const STEP_HIGH: [u8; 6] = [0, 0, 1, 1, 2, 2];

/// 每步哪相驱动低（GND）
const STEP_LOW: [u8; 6] = [1, 2, 2, 0, 0, 1];

// ============================================================================
// 霍尔传感器 → 换向步映射表
// ============================================================================

/// 霍尔 3-bit 编码 → 换向步索引
///
/// hall_state = (HA << 2) | (HB << 1) | HC
///
/// 索引 0 和 7 为无效状态（全 0 或全 1 不应出现）。
/// 有效状态：1, 2, 3, 4, 5, 6 → 映射到换向步 0~5。
///
/// **注意：** 此表对应最常见的 120° 霍尔排列。
/// 如果电机不转或方向不对，需要调整此表。
///
/// ```text
///  Index:  0     1     2     3     4     5     6     7
///  State: 000   001   010   011   100   101   110   111
///  Step:  INV    5     3     4     1     0     2    INV
/// ```
const HALL_TO_STEP: [u8; 8] = [
    0xFF, // 0b000 — 无效
    5,    // 0b001 — HC=1
    3,    // 0b010 — HB=1
    4,    // 0b011 — HB=1, HC=1
    1,    // 0b100 — HA=1
    0,    // 0b101 — HA=1, HC=1
    2,    // 0b110 — HA=1, HB=1
    0xFF, // 0b111 — 无效
];

// ============================================================================
// 全局共享状态（main ↔ 霍尔 ISR / Timer ISR）
// ============================================================================

/// Timer 外设 — 通过 Mutex 共享给 ISR
static TIMER: Mutex<RefCell<Option<Timer0>>> = Mutex::new(RefCell::new(None));

/// 当前霍尔状态（3-bit: HA<<2 | HB<<1 | HC）
static HALL_STATE: AtomicU8 = AtomicU8::new(0);

/// 霍尔中断发生标志 — 通知主循环需要换向
static HALL_CHANGED: AtomicBool = AtomicBool::new(false);

/// 当前换向步（0~5），由霍尔 ISR 根据 HALL_TO_STEP 查表得到
static CURRENT_STEP: AtomicU8 = AtomicU8::new(0);

/// 平滑后的换向步周期 (µs)
static STEP_PERIOD_US: AtomicU32 = AtomicU32::new(MAX_STEP_PERIOD_US);

/// 上次霍尔中断的时间戳 (µs)
static LAST_HALL_TIME_US: AtomicU32 = AtomicU32::new(0);

/// 霍尔中断计数（用于 RPM 计算和日志）
static HALL_COUNT: AtomicU32 = AtomicU32::new(0);

/// 实测 RPM
static ACTUAL_RPM: AtomicU32 = AtomicU32::new(0);

/// 当前 PWM 占空比 compare 值（由 PI 控制器设置，ISR 读取用于换向）
static CURRENT_DUTY: AtomicU16 = AtomicU16::new(0);

/// PI 速度控制循环 tick 标志
static SPEED_LOOP_TICK: AtomicBool = AtomicBool::new(false);

// ============================================================================
// PWM Slice 全局访问（ISR 中通过 unsafe 直接操作寄存器）
// ============================================================================

/// PWM wrap 值（系统频率 / PWM频率）
static PWM_WRAP: AtomicU16 = AtomicU16::new(0);

// ============================================================================
// 入口
// ============================================================================

#[main]
fn main() -> ! {
    info!("BLDC ESC — Hall sensor closed-loop control");

    // ------------------------------------------------------------------
    // 外设与时钟初始化
    // ------------------------------------------------------------------
    let dp = Peripherals::take().unwrap();
    let clocks = ClockBuilder::new().build(dp.CLOCKS, dp.XOSC, dp.PLL_SYS, dp.PLL_USB);
    let sys_freq = clocks.sys_freq();
    info!("System clock: {} MHz", sys_freq / 1_000_000);

    let pins = Pins::new(dp.IO_BANK0, dp.PADS_BANK0);
    let timer = Timer0::init(dp.TIMER0);

    // ------------------------------------------------------------------
    // PWM 初始化 — 20 kHz，slice 0/1/2 的 channel A
    // ------------------------------------------------------------------
    let pwm_wrap: u16 = (sys_freq / PWM_FREQ_HZ) as u16;
    PWM_WRAP.store(pwm_wrap, Ordering::Relaxed);
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
    // 低侧 GPIO 输出
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
    // 霍尔传感器 GPIO 输入（上拉 + 双边沿中断）
    //   GP6 = Hall A (HA)
    //   GP7 = Hall B (HB)
    //   GP8 = Hall C (HC)
    // ------------------------------------------------------------------
    let hall_a = pins.pin::<6>();
    hall_a.into_input_pullup();
    hall_a.enable_interrupt(EdgeLevel::EdgeHigh);
    hall_a.enable_interrupt(EdgeLevel::EdgeLow);

    let hall_b = pins.pin::<7>();
    hall_b.into_input_pullup();
    hall_b.enable_interrupt(EdgeLevel::EdgeHigh);
    hall_b.enable_interrupt(EdgeLevel::EdgeLow);

    let hall_c = pins.pin::<8>();
    hall_c.into_input_pullup();
    hall_c.enable_interrupt(EdgeLevel::EdgeHigh);
    hall_c.enable_interrupt(EdgeLevel::EdgeLow);

    // ------------------------------------------------------------------
    // 辅助闭包
    // ------------------------------------------------------------------

    // 占空比百分比转 PWM compare 值
    let duty_from_percent =
        |percent: u16| -> u16 { ((pwm_wrap as u32) * (percent as u32) / 100) as u16 };

    // 执行一个换向步
    let commutate = |step: usize, duty: u16| {
        let high = STEP_HIGH[step];
        let low = STEP_LOW[step];

        // 高侧 PWM
        slice_a.set_duty_a(if high == 0 { duty } else { 0 });
        slice_b.set_duty_a(if high == 1 { duty } else { 0 });
        slice_c.set_duty_a(if high == 2 { duty } else { 0 });

        // 低侧 GPIO
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

    // 关闭所有输出
    let all_off = || {
        slice_a.set_duty_a(0);
        slice_b.set_duty_a(0);
        slice_c.set_duty_a(0);
        al.set_low();
        bl.set_low();
        cl.set_low();
    };

    // ------------------------------------------------------------------
    // 读取当前霍尔状态并执行首次换向
    // ------------------------------------------------------------------
    let read_hall = || -> u8 {
        let ha = hall_a.get() as u8;
        let hb = hall_b.get() as u8;
        let hc = hall_c.get() as u8;
        (ha << 2) | (hb << 1) | hc
    };

    let initial_hall = read_hall();
    let initial_step = HALL_TO_STEP[initial_hall as usize];
    info!(
        "Initial hall state: 0b{:03b} → step {}",
        initial_hall, initial_step
    );

    if initial_step == 0xFF {
        error!("Invalid hall state 0b{:03b} — check wiring!", initial_hall);
        // 安全状态：保持输出关闭
        loop {
            timer.delay_ms(1000);
        }
    }

    // 存储初始状态
    HALL_STATE.store(initial_hall, Ordering::Relaxed);
    CURRENT_STEP.store(initial_step, Ordering::Relaxed);

    let startup_duty = duty_from_percent(STARTUP_DUTY_PERCENT);
    CURRENT_DUTY.store(startup_duty, Ordering::Relaxed);

    // 首次换向：按当前霍尔位置通电
    commutate(initial_step as usize, startup_duty);
    info!(
        "Initial commutation applied, duty={}%",
        STARTUP_DUTY_PERCENT
    );

    // ------------------------------------------------------------------
    // Timer0 Alarm 中断 — PI 速度控制循环 @100Hz
    // ------------------------------------------------------------------
    {
        let now = timer.get_time_us_32();
        LAST_HALL_TIME_US.store(now, Ordering::Relaxed);
        timer.set_alarm(0, now.wrapping_add(SPEED_LOOP_INTERVAL_US));
        timer.enable_alarm_interrupt(0);

        interrupt::free(|cs| {
            TIMER.borrow(cs).replace(Some(timer));
        });

        unsafe {
            interrupt::enable(Interrupt::TIMER0_IRQ_0);
        }
    }

    // ------------------------------------------------------------------
    // 启用 GPIO 中断（霍尔传感器）
    // ------------------------------------------------------------------
    unsafe {
        interrupt::enable(Interrupt::IO_IRQ_BANK0);
    }
    info!("Hall sensor interrupts enabled — motor running");

    // ------------------------------------------------------------------
    // Timer 访问包装器（经过 Mutex）
    // ------------------------------------------------------------------
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
    // 主循环 — 换向处理 + PI 控制
    // ------------------------------------------------------------------
    let pole_pairs = MOTOR_POLES / 2;
    let mut integral: i32 = 0;
    let mut current_duty_percent: u16 = STARTUP_DUTY_PERCENT;
    let mut log_counter: u32 = 0;

    loop {
        // ==============================================================
        // A: 处理霍尔换向事件
        // ==============================================================
        if HALL_CHANGED.swap(false, Ordering::Relaxed) {
            let step = CURRENT_STEP.load(Ordering::Relaxed);
            if step < 6 {
                let duty = CURRENT_DUTY.load(Ordering::Relaxed);
                commutate(step as usize, duty);
            } else {
                // 无效霍尔状态 → 安全关闭
                all_off();
            }
        }

        // ==============================================================
        // B: PI 速度控制器 (@100Hz, Timer Alarm 触发)
        // ==============================================================
        if SPEED_LOOP_TICK.swap(false, Ordering::Relaxed) {
            // 从霍尔中断获取步进周期, 计算 RPM
            let step_period = STEP_PERIOD_US.load(Ordering::Relaxed);

            let actual_rpm = if step_period > 0 && step_period < MAX_STEP_PERIOD_US {
                let elec_period_us = step_period as u64 * 6;
                (60_000_000u64 / (elec_period_us * pole_pairs as u64)) as u32
            } else {
                0 // 电机停止或极低速
            };
            ACTUAL_RPM.store(actual_rpm, Ordering::Relaxed);

            // --- 检查是否超时（电机可能已停转）---
            let now = timer_now_us();
            let last_hall = LAST_HALL_TIME_US.load(Ordering::Relaxed);
            let since_last = now.wrapping_sub(last_hall);
            if since_last > MAX_STEP_PERIOD_US {
                // 电机可能停转，用超时后的 0 RPM
                ACTUAL_RPM.store(0, Ordering::Relaxed);
            }

            let actual_rpm = ACTUAL_RPM.load(Ordering::Relaxed);
            let error = TARGET_RPM as i32 - actual_rpm as i32;

            // --- 比例项 ---
            let p_term = KP_FP * error;

            // --- 积分项（带限幅防 windup）---
            integral = (integral + KI_FP * error).clamp(-INTEGRAL_MAX, INTEGRAL_MAX);

            // --- PI 输出 → 占空比调整 ---
            let output = (p_term + integral) >> FP_SHIFT;

            let new_duty = (current_duty_percent as i32 + output)
                .clamp(MIN_DUTY_PERCENT as i32, MAX_DUTY_PERCENT as i32)
                as u16;

            if new_duty != current_duty_percent {
                current_duty_percent = new_duty;
                let duty_val = duty_from_percent(current_duty_percent);
                CURRENT_DUTY.store(duty_val, Ordering::Relaxed);

                // 立即更新当前换向步的占空比
                let step = CURRENT_STEP.load(Ordering::Relaxed);
                if step < 6 {
                    commutate(step as usize, duty_val);
                }
            }

            // 周期日志（约每 1 秒）
            log_counter += 1;
            if log_counter.is_multiple_of(100) {
                let hall_cnt = HALL_COUNT.load(Ordering::Relaxed);
                let hall_state = HALL_STATE.load(Ordering::Relaxed);
                info!(
                    "RPM: {} (target {}), duty: {}%, step_period: {}us, hall: 0b{:03b}, count: {}",
                    actual_rpm, TARGET_RPM, current_duty_percent, step_period, hall_state, hall_cnt
                );
            }
        }

        // 短暂等待，避免空转烧 CPU（WFI 更理想但此处用简单延迟）
        // 注：调用 delay 需经过 Mutex
        interrupt::free(|cs| {
            if let Some(ref t) = *TIMER.borrow(cs).borrow() {
                t.delay_us(100);
            }
        });
    }
}

// ============================================================================
// 霍尔传感器 GPIO 中断 — IO_IRQ_BANK0
//
// 任意霍尔信号边沿变化时触发，读取 3-bit 霍尔编码，查表换向，记录时间戳。
// ============================================================================

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
fn IO_IRQ_BANK0() {
    // 在 ISR 中重建 Pin 引用（GPIO 子系统已在 main 中初始化）
    let ha = unsafe { Pin::<6>::steal() };
    let hb = unsafe { Pin::<7>::steal() };
    let hc = unsafe { Pin::<8>::steal() };

    // 清除所有 3 个霍尔引脚的中断标志
    ha.clear_all_interrupts();
    hb.clear_all_interrupts();
    hc.clear_all_interrupts();

    // 读取当前霍尔状态
    let hall_state = ((ha.get() as u8) << 2) | ((hb.get() as u8) << 1) | (hc.get() as u8);
    HALL_STATE.store(hall_state, Ordering::Relaxed);

    // 查表获取换向步
    let step = HALL_TO_STEP[hall_state as usize];
    CURRENT_STEP.store(step, Ordering::Relaxed);

    // 计时：测量换向步周期
    interrupt::free(|cs| {
        if let Some(ref timer) = *TIMER.borrow(cs).borrow() {
            let now = timer.get_time_us_32();
            let last = LAST_HALL_TIME_US.load(Ordering::Relaxed);
            let period = now.wrapping_sub(last);
            LAST_HALL_TIME_US.store(now, Ordering::Relaxed);

            // 有效周期才更新（过滤噪声/去抖）
            if period > 50 && period < MAX_STEP_PERIOD_US {
                let old = STEP_PERIOD_US.load(Ordering::Relaxed);
                // EMA 平滑: new = old * 0.75 + measured * 0.25
                let smoothed = (old * 3 + period) / 4;
                STEP_PERIOD_US.store(smoothed, Ordering::Relaxed);
            }
        }
    });

    HALL_COUNT.fetch_add(1, Ordering::Relaxed);

    // 通知主循环执行换向
    HALL_CHANGED.store(true, Ordering::Relaxed);
}

// ============================================================================
// Timer0 ISR — PI 速度控制 tick @100Hz
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
