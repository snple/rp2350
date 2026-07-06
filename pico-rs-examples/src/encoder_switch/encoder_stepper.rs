//! # 编码器 + 步进电机手动控制示例（粗调/细调三档 · 软中断驱动）
//!
//! 基于 RP2350 (Raspberry Pi Pico 2) 的 **旋转编码器控制步进电机** 示例，
//! 使用 **GPIO 边沿中断** 实现四倍频 (4×) 正交解码 + 按键检测，
//! 通过 **软中断 (SPARE_IRQ_0)** 实现零延迟电机响应。
//! 旋转编码器每转一步，步进电机按当前档位运动相应步数；
//! 按下编码器按键在三个档位之间循环切换。
//!
//! ## 中断架构
//!
//! ```text
//!   IO_IRQ_BANK0 (优先级 0, 最高)
//!     │  正交解码: A/B 边沿 → 更新 RAW_COUNT / ENC_STEPS
//!     │  按钮: 消抖 → 设置 BUTTON_PRESSED
//!     │
//!     └──► pend SPARE_IRQ_0 (每个物理棒齿触发一次)
//!
//!   TIMER0_IRQ_0 (优先级 0, 最高)
//!     │  步进脉冲生成 (两相: 上升沿 → 下降沿)
//!     │
//!     ├──► pend SPARE_IRQ_0 (进入减速段 且 有未处理编码器步)
//!     └──► pend SPARE_IRQ_0 (运动完成 且 目标未到达)
//!
//!   SPARE_IRQ_0 (优先级 48, 较低)
//!     │  将编码器增量累加到电机目标位置
//!     │  计算 (目标 − 当前) = 剩余距离
//!     │  若同向加速/巡航: 动态延长运动距离 (无需停止)
//!     │  若正在减速 或 反向: 停止 → 以剩余距离重启
//!     └──► 启动/重启电机运动
//!
//!   主循环 (最低优先级)
//!     │  档位切换 (按键)
//!     └──► WFI 省电
//! ```
//!
//! > **目标位置追踪**: SWI 维护电机目标位置 (MOTOR_TARGET), 编码器每步累加
//! > gear_steps() 个微步。电机始终向目标位置运动。
//! >
//! > **运动延长**: 连续旋转时, 若电机在加速/巡航阶段, 动态延长运动距离,
//! > 无需停止电机; 若已进入减速段, 立即停止并以更新后的目标重启。
//! > 避免 "减速 → 停顿 → 加速" 循环, 实现快档模式下连续平滑运动。
//!
//! ---
//!
//! # 1. 编码器开关引脚说明
//!
//! 5 脚增量式旋转编码器开关 (如 EC11) 具有以下引脚:
//!
//! ```text
//!        ┌───────────┐
//!        │  编码器开关 │
//!  A○────┤           ├────○D
//!        │           │
//!  C○────┤  (公共端)  ├────○E
//!        │           │
//!  B○────┤           │
//!        └───────────┘
//!
//!  编码器部分: A, B, C          按钮部分: D, E
//!  ─────────────────          ──────────────
//!  A - 编码器信号 A            D - 按钮引脚 1
//!  B - 编码器信号 B            E - 按钮引脚 2
//!  C - 编码器公共端 (接 GND)    (按下时 D-E 短接)
//! ```
//!
//! ## 工作原理
//!
//! **编码器部分 (A, B, C)**:
//! - C 为公共端，连接 GND
//! - A 和 B 输出两路相差 90° 的方波信号
//! - A、B 通过 GPIO 内部上拉到高电平
//! - 旋转时 A、B 交替连接到 C (GND)，产生低电平脉冲
//!
//! **按钮部分 (D, E)**:
//! - 按下旋钮时 D 和 E 导通
//! - E 连接 GND，D 通过 GPIO 内部上拉到高电平
//! - 按下时 D 被拉低
//!
//! ## 正交解码原理
//!
//! ```text
//!   顺时针旋转 (CW):                   逆时针旋转 (CCW):
//!
//!   A:  ─┐ ┌─┐ ┌─┐ ┌──             A:  ──┐ ┌─┐ ┌─┐ ┌─
//!        │ │ │ │ │ │                     │ │ │ │ │ │
//!        └─┘ └─┘ └─┘                     └─┘ └─┘ └─┘
//!
//!   B:  ──┐ ┌─┐ ┌─┐ ┌─             B:  ─┐ ┌─┐ ┌─┐ ┌──
//!         │ │ │ │ │ │                   │ │ │ │ │ │
//!         └─┘ └─┘ └─┘                   └─┘ └─┘ └─┘
//!
//!        A 超前 B → 正转              B 超前 A → 反转
//! ```
//!
//! ---
//!
//! # 2. 三档调速
//!
//! | 档位 | 编码器 1 步 | 步进电机运动量 | 说明 |
//! |------|------------|---------------|------|
//! | 快档 (Fast) | 1 步 | 1 整圈 (360°) | 快速粗调 |
//! | 中档 (Medium) | 1 步 | 3.6° (2 全步) | 中等精度 |
//! | 慢档 (Slow) | 1 步 | 1 微步 | 最精细调整 |
//!
//! > 以 1.8° 步距角电机 + 16 微步细分为例：
//! > - 快档：1 步 = 200 × 16 = 3200 微步
//! > - 中档：3.6° / 1.8° × 16 = 32 微步
//! > - 慢档：1 微步
//!
//! ---
//!
//! # 3. 硬件接线
//!
//! ## 编码器部分
//!
//! | Pico GPIO | 编码器引脚 | 说明 |
//! |-----------|-----------|------|
//! | GP10 | A | 编码器信号 A (内部上拉) |
//! | GP11 | B | 编码器信号 B (内部上拉) |
//! | GND | C | 编码器公共端 (接地) |
//! | GP12 | D | 按钮引脚 (内部上拉) |
//! | GND | E | 按钮公共端 (接地) |
//!
//! ## 步进电机部分 (A4988 / TMC2209 均可)
//!
//! | Pico GPIO | 功能 | 驱动器引脚 |
//! |-----------|------|-----------|
//! | GP2 | STEP | STEP |
//! | GP3 | DIR | DIR |
//! | GP4 | ENABLE | ENN / ENABLE |
//!
//! ```text
//!   Pico 2 (RP2350)               旋转编码器开关 (EC11 等)
//!   ──────────────                ────────────────────────
//!   GP10 ◄────────────────────────── A  (编码器信号 A)
//!   GP11 ◄────────────────────────── B  (编码器信号 B)
//!   GND  ──────────────────────────── C  (编码器公共端)
//!   GP12 ◄────────────────────────── D  (按钮引脚)
//!   GND  ──────────────────────────── E  (按钮公共端)
//!
//!                                 步进电机驱动器 (A4988 / TMC2209)
//!                                 ──────────────────────────────
//!   GP2  ──────────────────────────► STEP
//!   GP3  ──────────────────────────► DIR
//!   GP4  ──────────────────────────► ENN / ENABLE
//! ```
//!
//! > **注意**: C 和 E 都接 GND，可以共用同一个 GND 引脚。
//! > A、B、D 使用 GPIO 内部上拉电阻，无需外部上拉。
//!
//! ---
//!
//! # 4. 按键硬件消抖 与 EMI 防护
//!
//! ## 为什么需要硬件消抖
//!
//! 步进电机运行时，电流脉冲通过共地/电源耦合到编码器按钮引脚，
//! 产生微秒级假 EdgeLow，软件无法可靠区分。硬件 RC 滤波器可以
//! 在进入 GPIO 前滤除高频干扰，是最根本的解决方案。
//!
//! ## RC 低通滤波器 (必加)
//!
//! 在按钮引脚 D 和 GPIO 之间串联电阻 + 并联电容：
//!
//! ```text
//!   编码器 D 引脚 ───[10kΩ]───┬─── GP12 (BTN)
//!                              │
//!                           [100nF]
//!                              │
//!                             GND
//! ```
//!
//! - R = 10kΩ, C = 100nF → τ = 1ms
//! - 截止频率 ≈ 1.6kHz，轻松滤除 20kHz 步进脉冲干扰
//! - 不影响按键响应（人手按下 >50ms）
//! - 同时拑平机械弹跳（按下和松开均有效）
//!
//! > 可选: 编码器信号 A, B 也可以加 RC 滤波，但 R 应较小 (1kΩ/10nF)，
//! > 避免影响 4× 解码时序。
//!
//! ## 布线建议
//!
//! 1. **分离地线**: 电机驱动器电源地和信号地分开走线,
//!    在 Pico GND 处单点汇聚，避免电机电流回流污染信号地
//! 2. **编码器走线远离电机**: 编码器引线与步进电机/STEP 信号线
//!    保持距离或垂直交叉，不要平行走线
//! 3. **驱动器电源旁路电容**: 在电机驱动器 VMOT 旁加 100μF 电解
//!    + 100nF 陶瓷电容，吸收电流尖峰
//! 4. **使用屏蔽线** (可选): 如果编码器引线 > 10cm,
//!    使用屏蔽线并将屏蔽层接 GND
//!
//! ---
//!
//! # 5. 配置参数
//!
//! | 参数 | 默认值 | 说明 |
//! |------|--------|------|
//! | `PIN_A` | 10 | 编码器信号 A 的 GPIO |
//! | `PIN_B` | 11 | 编码器信号 B 的 GPIO |
//! | `PIN_BTN` | 12 | 按钮引脚 D 的 GPIO (需加 RC 滤波) |
//! | `PIN_STEP` | 2 | 步进电机 STEP 引脚 |
//! | `PIN_DIR` | 3 | 步进电机 DIR 引脚 |
//! | `PIN_ENABLE` | 4 | 步进电机 ENABLE 引脚 |
//! | 软件消抖 | 50 ms | 按钮消抖延迟 (配合 RC 滤波使用) |
//!
//! ---
//!
//! # 6. 操作说明
//!
//! - **旋转编码器**：顺时针正转，逆时针反转，运动量取决于当前档位
//! - **按下编码器按键**：循环切换 快档 → 中档 → 慢档 → 快档 ...
//! - 编码器每一步变化通过软中断 (SPARE_IRQ_0) 立即更新电机目标位置
//! - 连续旋转时, 电机实时追踪目标位置, 减速阶段自动跳过以消除停顿
//! - 电机运动完成后自动检查未到达的目标位置，确保不丢步
//!
//! ---
//!
//! # 7. 编译运行
//!
//! ```bash
//! cargo run --bin encoder_stepper --target thumbv8m.main-none-eabihf
//! ```

#![no_std]
#![no_main]

use core::sync::atomic::{AtomicBool, AtomicI32, AtomicU32, Ordering};

use defmt::*;
use defmt_rtt as _;

use pico_rs::bind;
use pico_rs::clocks::ClockBuilder;
use pico_rs::main;
use pico_rs::gpio::{EdgeLevel, Pin, Pins};
use pico_rs::interrupt;
use pico_rs::pac::{Interrupt, Peripherals};
use pico_rs::ticks::Ticks;
use pico_rs::timer::Timer0;

use pico_rs_drivers::stepper::{AxisConfig, AxisState, Direction, MotionPhase};// ============================================================================
// 编码器引脚配置
// ============================================================================

/// 编码器信号 A 的 GPIO 编号 (编码器引脚 A)
const PIN_A: u32 = 10;

/// 编码器信号 B 的 GPIO 编号 (编码器引脚 B)
const PIN_B: u32 = 11;

/// 按钮引脚的 GPIO 编号 (编码器引脚 D)
/// 编码器引脚 C 接 GND (编码器公共端)
/// 编码器引脚 E 接 GND (按钮公共端)
const PIN_BTN: u32 = 12;

/// 按钮消抖时间 (微秒)
const DEBOUNCE_US: u32 = 50_000; // 50ms

// ============================================================================
// 步进电机配置
// ============================================================================

/// STEP 引脚
const PIN_STEP: u32 = 2;

/// DIR 引脚
const PIN_DIR: u32 = 3;

/// ENABLE 引脚 (低有效)
const PIN_ENABLE: u32 = 4;

/// 起步频率 (Hz)
const START_SPEED_HZ: u32 = 200;

/// 最大步进频率 (Hz)
const MAX_SPEED_HZ: u32 = 20_000;

/// 加速段步数
const ACCEL_STEPS: u32 = 200;

/// STEP 脉冲宽度 (μs)
const STEP_PULSE_US: u32 = 3;

/// 电机每转全步数 (1.8° = 200 步/转)
const MOTOR_STEPS_PER_REV: u32 = 200;

/// 微步细分倍数
const MICROSTEPPING: u32 = 16;

// ============================================================================
// 三档步数计算
// ============================================================================

/// 快档：编码器 1 步 = 1 整圈
const STEPS_FAST: u32 = MOTOR_STEPS_PER_REV * MICROSTEPPING; // 3200

/// 中档：编码器 1 步 = 3.6° = 2 全步
const STEPS_MEDIUM: u32 = 2 * MICROSTEPPING; // 32

/// 慢档：编码器 1 步 = 1 微步
const STEPS_SLOW: u32 = 1;

/// 4× 正交解码: 每个编码器物理棘齿 (detent) 产生 4 个计数
const ENCODER_COUNTS_PER_STEP: i32 = 4;

// ============================================================================
// 共享状态 (ISR 之间 / ISR ↔ 主循环)
// ============================================================================

/// 编码器原始累计计数 (有符号, 4× 解码)
/// CW → 递增, CCW → 递减
/// 由 GPIO ISR 更新, 用于物理步边界检测
static RAW_COUNT: AtomicI32 = AtomicI32::new(0);

/// 编码器物理步计数 (每个棘齿 = 4 个原始计数)
/// 由 GPIO ISR 在检测到物理步边界时更新
static ENC_STEPS: AtomicI32 = AtomicI32::new(0);

/// SWI 上次处理时的物理步快照
/// 用于计算自上次电机启动以来的增量
static LAST_PROCESSED_STEP: AtomicI32 = AtomicI32::new(0);

/// 电机目标位置 (微步单位)
/// 由 SWI 根据编码器增量和当前档位累加更新
/// 每个编码器物理步增加 gear_steps() 个微步
static MOTOR_TARGET: AtomicI32 = AtomicI32::new(0);

/// 电机当前运动方向 (true = Forward, false = Reverse)
/// 由 SWI 在 start_move 前设置, 用于反向检测
static MOTOR_DIR_FWD: AtomicBool = AtomicBool::new(true);

/// 上一次 A/B 状态 (bit1 = A, bit0 = B)
static LAST_AB: AtomicI32 = AtomicI32::new(0);

/// 按键按下标志 (由 GPIO ISR 置 true, 主循环清除)
static BUTTON_PRESSED: AtomicBool = AtomicBool::new(false);

/// 上次按钮中断时间戳 (用于消抖, 低 32 位足够 — 约 71 分钟溢出, 远超 50ms 消抖窗口)
static LAST_BTN_TIME: AtomicU32 = AtomicU32::new(0);

/// 当前档位 (0 = 快档, 1 = 中档, 2 = 慢档)
static GEAR: AtomicU32 = AtomicU32::new(0);

/// 步进轴状态
static AXIS: AxisState = AxisState::new(AxisConfig {
    pin_step: PIN_STEP,
    pin_dir: PIN_DIR,
    alarm: 0,
    start_speed_hz: START_SPEED_HZ,
    accel_steps: ACCEL_STEPS,
    step_pulse_us: STEP_PULSE_US,
});

// ============================================================================
// 正交解码查找表
// ============================================================================

/// 状态跳变表: `QUAD_TABLE[old_state << 2 | new_state]`
///
/// 返回值:  0 = 无效/无变化, +1 = CW, -1 = CCW
///
/// 索引 = (old_AB << 2) | new_AB, 共 16 项
const QUAD_TABLE: [i32; 16] = [
    0,  // 00 → 00: 无变化
    1,  // 00 → 01: CW
    -1, // 00 → 10: CCW
    0,  // 00 → 11: 无效 (跳跃)
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

/// 返回当前档位对应的步进电机步数
fn gear_steps(gear: u32) -> u32 {
    match gear {
        0 => STEPS_FAST,
        1 => STEPS_MEDIUM,
        _ => STEPS_SLOW,
    }
}

/// 返回当前档位名称
fn gear_name(gear: u32) -> &'static str {
    match gear {
        0 => "FAST (1 rev)",
        1 => "MEDIUM (3.6°)",
        _ => "SLOW (1 ustep)",
    }
}

/// 返回当前档位的最大速度 (慢档无需高速)
fn gear_max_speed(gear: u32) -> u32 {
    match gear {
        0 => MAX_SPEED_HZ, // 20 kHz — 快档全速
        1 => 4000,
        _ => 500,
    }
}

#[main]
fn main() -> ! {
    info!("=== Encoder + Stepper Motor Control (3-Gear) ===");
    info!("Rotate encoder to move motor, press button to switch gear");

    // ------------------------------------------------------------------
    // 1. 外设初始化
    // ------------------------------------------------------------------
    let dp = Peripherals::take().unwrap();
    let clocks = ClockBuilder::new().build(dp.CLOCKS, dp.XOSC, dp.PLL_SYS, dp.PLL_USB);
    let ticks = Ticks::new(dp.TICKS, &clocks);
    ticks.init_timer0();
    let _timer = Timer0::init(dp.TIMER0);
    let pins = Pins::new(dp.IO_BANK0, dp.PADS_BANK0);

    // ------------------------------------------------------------------
    // 2. 配置编码器引脚
    //    A, B → GPIO 输入 + 内部上拉 (编码器公共端 C 接 GND)
    //    D   → GPIO 输入 + 内部上拉 (按钮公共端 E 接 GND)
    // ------------------------------------------------------------------

    // 编码器信号 A — 输入 + 内部上拉
    let pin_a = pins.pin::<PIN_A>();
    pin_a.into_input_pullup();

    // 编码器信号 B — 输入 + 内部上拉
    let pin_b = pins.pin::<PIN_B>();
    pin_b.into_input_pullup();

    // 按钮引脚 D — 输入 + 内部上拉 (按下时 D 被拉低到 E/GND)
    let pin_btn = pins.pin::<PIN_BTN>();
    pin_btn.into_input_pullup();

    info!(
        "Config: A=GP{}, B=GP{}, BTN=GP{} (C=GND, E=GND)",
        PIN_A, PIN_B, PIN_BTN
    );

    let init_a = pin_a.get() as i32;
    let init_b = pin_b.get() as i32;
    LAST_AB.store((init_a << 1) | init_b, Ordering::Relaxed);

    // 编码器双沿中断 (上升沿 + 下降沿) → 4× 解码
    pin_a.enable_interrupt(EdgeLevel::EdgeHigh);
    pin_a.enable_interrupt(EdgeLevel::EdgeLow);
    pin_b.enable_interrupt(EdgeLevel::EdgeHigh);
    pin_b.enable_interrupt(EdgeLevel::EdgeLow);

    // 按钮下降沿中断 (按下时触发)
    // 硬件 RC 滤波 (10kΩ + 100nF) 已滤除 EMI 和松开弹跳, 软件仅做基本消抖
    pin_btn.enable_interrupt(EdgeLevel::EdgeLow);

    // ------------------------------------------------------------------
    // 3. 步进电机 GPIO 配置
    // ------------------------------------------------------------------
    let p_step = pins.pin::<PIN_STEP>();
    p_step.into_output();
    p_step.set_low();

    let p_dir = pins.pin::<PIN_DIR>();
    p_dir.into_output();
    p_dir.set_low();

    let p_en = pins.pin::<PIN_ENABLE>();
    p_en.into_output();
    p_en.set_low(); // 使能驱动器

    info!("Encoder: A=GP{}, B=GP{}, BTN=GP{}", PIN_A, PIN_B, PIN_BTN);
    info!(
        "Stepper: STEP=GP{}, DIR=GP{}, EN=GP{}",
        PIN_STEP, PIN_DIR, PIN_ENABLE
    );
    info!(
        "Motor: {} full steps/rev, {}x microstepping = {} usteps/rev",
        MOTOR_STEPS_PER_REV,
        MICROSTEPPING,
        MOTOR_STEPS_PER_REV * MICROSTEPPING
    );

    // ------------------------------------------------------------------
    // 4. 使能中断
    //    IO_IRQ_BANK0:  编码器正交解码 + 按钮检测 (优先级 0, 最高)
    //    TIMER0_IRQ_0:  步进电机脉冲生成        (优先级 0, 最高)
    //    SPARE_IRQ_0:   软中断 — 电机控制逻辑    (优先级 48, 较低)
    //
    //    优先级设计:
    //    - GPIO 和 Timer 需要最高优先级以保证时序精度
    //    - SWI 优先级较低, 确保不会阻塞编码器解码和脉冲生成
    //    - 主循环优先级最低, 仅处理档位切换
    // ------------------------------------------------------------------
    unsafe {
        interrupt::set_priority(Interrupt::SPARE_IRQ_0, 48);
        interrupt::enable(Interrupt::IO_IRQ_BANK0);
        interrupt::enable(Interrupt::TIMER0_IRQ_0);
        interrupt::enable(Interrupt::SPARE_IRQ_0);
    }

    info!("Interrupts enabled (GPIO: prio 0, Timer: prio 0, SWI: prio 48)");

    // ------------------------------------------------------------------
    // 5. 主循环 — 仅处理按键档位切换, 电机控制由 SWI 驱动
    // ------------------------------------------------------------------
    info!(
        "Gear: {} | Rotate to move, press to switch gear",
        gear_name(0)
    );

    loop {
        // WFI: 等待任意中断唤醒, 省电
        pico_rs_rt::asm::wfi();

        // 检查按键 → 切换档位
        if BUTTON_PRESSED.swap(false, Ordering::Relaxed) {
            let new_gear = (GEAR.load(Ordering::Relaxed) + 1) % 3;
            GEAR.store(new_gear, Ordering::Relaxed);
            info!("Gear switched: {}", gear_name(new_gear));
        }
    }
}

// ============================================================================
// Timer0 ISR — 步进脉冲生成 (优先级 0)
// ============================================================================

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
fn TIMER0_IRQ_0() {
    let timer = unsafe { Timer0::steal() };
    AXIS.handle_alarm_isr(&timer);

    if AXIS.is_done() {
        // 运动完成: 若目标未到达 或 有未处理的编码器步, 触发 SWI 继续
        if MOTOR_TARGET.load(Ordering::Relaxed) != AXIS.position()
            || ENC_STEPS.load(Ordering::Relaxed) != LAST_PROCESSED_STEP.load(Ordering::Relaxed)
        {
            interrupt::pend(Interrupt::SPARE_IRQ_0);
        }
    } else if AXIS.phase() == MotionPhase::Decelerating {
        // 电机进入减速段: 若有未处理的编码器步, 触发 SWI
        // SWI 会停止当前运动并以更新后的目标重启,
        // 避免 “减速 → 停顿 → 加速” 的不跟手感
        if ENC_STEPS.load(Ordering::Relaxed) != LAST_PROCESSED_STEP.load(Ordering::Relaxed) {
            interrupt::pend(Interrupt::SPARE_IRQ_0);
        }
    }
}

// ============================================================================
// 软中断 (SPARE_IRQ_0) — 电机控制逻辑 (优先级 48)
//
// 触发时机:
//   1. 编码器 ISR 检测到物理棒齿后 pend
//   2. Timer ISR 检测到减速段 且 有未处理编码器步时 pend
//   3. Timer ISR 运动完成 且 目标未到达时 pend
//
// 核心逻辑: 目标位置追踪
//   - 每个编码器物理步累加 gear_steps() 个微步到 MOTOR_TARGET
//   - 电机始终向 MOTOR_TARGET 运动
//   - 同向加速/巡航时动态延长运动距离 (无需停止电机)
//   - 进入减速时停止并重启 ("skip decel"), 反向时立即停止并反向运动
// ============================================================================

#[bind(SPARE_IRQ_0)]
fn motor_control_swi() {
    // ---- 1. 将新的编码器物理步增量累加到电机目标位置 ----
    let cur_enc = ENC_STEPS.load(Ordering::Relaxed);
    let last_enc = LAST_PROCESSED_STEP.load(Ordering::Relaxed);
    let enc_delta = cur_enc - last_enc;

    if enc_delta != 0 {
        LAST_PROCESSED_STEP.store(cur_enc, Ordering::Relaxed);
        let gear = GEAR.load(Ordering::Relaxed);
        let usteps = enc_delta * gear_steps(gear) as i32;
        MOTOR_TARGET.fetch_add(usteps, Ordering::Relaxed);
    }

    // ---- 2. 计算电机当前位置到目标的剩余距离 ----
    let target = MOTOR_TARGET.load(Ordering::Relaxed);
    let current = AXIS.position();
    let remaining = target - current;

    if remaining == 0 {
        return;
    }

    // ---- 3. 决定是否需要停止当前运动并重启 ----
    if !AXIS.is_done() {
        let want_forward = remaining > 0;
        let cur_forward = MOTOR_DIR_FWD.load(Ordering::Relaxed);

        if want_forward == cur_forward && AXIS.phase() != MotionPhase::Decelerating {
            // 同方向且仍在加速/巡航: 动态延长运动距离, 无需停止电机
            // steps_done + remaining = 新的总目标步数, 减速点自动后移
            let new_total = AXIS.steps_done() + remaining.unsigned_abs();
            AXIS.set_target_steps(new_total);
            return;
        }

        // 方向改变 或 进入减速阶段: 停止当前运动, 重新规划路径
        // stop() 为即时停止, 仅微秒级延迟, 立即以更新后的目标重启
        AXIS.stop();
    }

    // ---- 4. 启动电机运动到目标位置 ----
    // stop() 后重新读取位置 (停止前 Timer ISR 可能多走了几步)
    let current = AXIS.position();
    let remaining = MOTOR_TARGET.load(Ordering::Relaxed) - current;
    if remaining == 0 {
        return;
    }

    let dir = if remaining > 0 {
        Direction::Forward
    } else {
        Direction::Reverse
    };
    MOTOR_DIR_FWD.store(remaining > 0, Ordering::Relaxed);

    let gear = GEAR.load(Ordering::Relaxed);
    let max_speed = gear_max_speed(gear);

    if enc_delta != 0 {
        info!(
            "[{}] enc={} target={} pos={} move={} {}",
            gear_name(gear),
            enc_delta,
            MOTOR_TARGET.load(Ordering::Relaxed),
            current,
            remaining.unsigned_abs(),
            dir.as_str()
        );
    }

    let t = unsafe { Timer0::steal() };
    AXIS.start_move(&t, remaining.unsigned_abs(), dir, max_speed);
}

// ============================================================================
// GPIO 中断 — 编码器正交解码 + 按钮检测
// ============================================================================

#[bind(IO_IRQ_BANK0)]
fn encoder_button_isr() {
    // SAFETY: GPIO 子系统已在 main() 中通过 Pins::new() 初始化。
    // ISR 中仅访问中断状态寄存器和读取引脚电平, 不做重新配置。
    let a = unsafe { Pin::<PIN_A>::steal() };
    let b = unsafe { Pin::<PIN_B>::steal() };
    let btn = unsafe { Pin::<PIN_BTN>::steal() };

    // --- 先清除中断标志并保存结果 (check_and_clear 会清除标志, 只能调用一次) ---
    let a_fired = a.check_and_clear(EdgeLevel::EdgeHigh) | a.check_and_clear(EdgeLevel::EdgeLow);
    let b_fired = b.check_and_clear(EdgeLevel::EdgeHigh) | b.check_and_clear(EdgeLevel::EdgeLow);

    if a_fired || b_fired {
        // 读取当前 A、B 电平
        let cur_a = a.get() as i32;
        let cur_b = b.get() as i32;
        let new_state = (cur_a << 1) | cur_b;

        // 获取上一次状态
        let old_state = LAST_AB.swap(new_state, Ordering::Relaxed);

        // 查表: 索引 = (old << 2) | new
        let idx = ((old_state << 2) | new_state) as usize & 0x0F;
        let direction = QUAD_TABLE[idx];

        // 原子更新原始计数, 然后检测物理棘齿边界
        if direction != 0 {
            let new_raw = RAW_COUNT.fetch_add(direction, Ordering::Relaxed) + direction;
            // 检测物理步边界: 每 4 个原始计数 = 1 个物理棘齿 (detent)
            // 仅当跨越棒齿边界时才触发软中断, 每个物理步只触发一次
            let old_step = (new_raw - direction) / ENCODER_COUNTS_PER_STEP;
            let new_step = new_raw / ENCODER_COUNTS_PER_STEP;
            if new_step != old_step {
                ENC_STEPS.store(new_step, Ordering::Relaxed);
                interrupt::pend(Interrupt::SPARE_IRQ_0);
            }
        }
    }

    // --- 按钮中断处理 (带消抖 + 电平确认) ---
    if btn.check_and_clear(EdgeLevel::EdgeLow) {
        // 确认按钮确实处于按下状态 (引脚为低电平)
        // 编码器旋转时相邻引脚串扰可能产生短暂毛刺触发 EdgeLow,
        // 但此时引脚已恢复高电平, 可过滤掉
        if !btn.get() {
            let now = {
                let timer = unsafe { Timer0::steal() };
                timer.get_time_us_32()
            };

            let last = LAST_BTN_TIME.load(Ordering::Relaxed);
            if now.wrapping_sub(last) >= DEBOUNCE_US {
                LAST_BTN_TIME.store(now, Ordering::Relaxed);
                BUTTON_PRESSED.store(true, Ordering::Relaxed);
            }
        }
    }
}
