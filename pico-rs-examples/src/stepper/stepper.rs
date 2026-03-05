//! # 开环步进电机控制 — 梯形加减速 + 多种步进模式
//!
//! 基于 RP2350 (Raspberry Pi Pico 2) 的**开环步进电机控制器**，
//! 支持 **STEP/DIR 驱动接口** (A4988 / DRV8825 / TMC2209 等) 和
//! **四相直接驱动接口** (ULN2003 + 28BYJ-48 等)。
//!
//! ---
//!
//! # 1. 功能概述
//!
//! ## 1.1 驱动模式
//!
//! | 模式 | 方式 | 驱动器芯片 | 说明 |
//! |------|------|-----------|------|
//! | **STEP/DIR** | 2 根信号线 | A4988 / DRV8825 / TMC2209 | 脉冲+方向，最常见 |
//! | **四相全步进** | 4 根 GPIO | ULN2003 + 28BYJ-48 | 两相同时激励，力矩大 |
//! | **四相半步进** | 4 根 GPIO | ULN2003 + 28BYJ-48 | 细分一倍，运行更平滑 |
//!
//! ## 1.2 加减速控制
//!
//! 采用**梯形加减速**（Trapezoidal Velocity Profile）：
//! ```text
//!   速度
//!     ↑     ┌───────────────────┐
//!     │    ╱│                   │╲
//!     │   ╱ │                   │ ╲
//!     │  ╱  │     匀速区        │  ╲
//!     │ ╱   │  (max_speed_hz)   │   ╲
//!     │╱    │                   │    ╲
//!     └─────┴───────────────────┴─────→ 步数
//!       加速      恒速            减速
//! ```
//!
//! - **加速段**: 步进频率从 `start_speed_hz` 线性增加到 `max_speed_hz`
//! - **匀速段**: 保持最大步进频率
//! - **减速段**: 步进频率线性减小到 0，保证精确停止
//!
//! 加减速由 Timer 中断实现，每一步动态调整间隔时间。
//!
//! ## 1.3 运动指令
//!
//! - **相对步进**: `move_steps(±N)` — 从当前位置移动 N 步
//! - **连续运转**: `run(direction, speed)` — 以指定速度持续运转
//! - **急停**: `emergency_stop()` — 立即停止
//! - **软停**: 自动梯形减速后停止
//!
//! ---
//!
//! # 2. 硬件接线
//!
//! ## 2.1 STEP/DIR 模式 (A4988 / DRV8825 / TMC2209)
//!
//! | GPIO | 功能 | 说明 |
//! |------|------|------|
//! | GP2 | STEP | 步进脉冲输出 |
//! | GP3 | DIR | 方向控制 |
//! | GP4 | ENABLE | 使能控制 (低有效) |
//!
//! ```text
//!   Pico 2                    A4988 / DRV8825
//!   GP2 (STEP) ─────────────► STEP
//!   GP3 (DIR)  ─────────────► DIR
//!   GP4 (EN)   ─────────────► ENABLE (低有效)
//!   GND ────────────────────── GND
//!   5V/3.3V ────────────────── VDD
//!
//!   电机电源 (12~24V) ────────── VMOT
//!   GND ──────────────────────── GND
//!
//!   步进电机:
//!   A+ ──────── 1A (线圈 A)
//!   A- ──────── 1B
//!   B+ ──────── 2A (线圈 B)
//!   B- ──────── 2B
//! ```
//!
//! ## 2.2 四相直接驱动模式 (ULN2003 + 28BYJ-48)
//!
//! | GPIO | 功能 | ULN2003 输入 | 电机线圈 |
//! |------|------|-------------|---------|
//! | GP2 | Phase A (IN1) | IN1 | 蓝色 |
//! | GP3 | Phase B (IN2) | IN2 | 粉色 |
//! | GP4 | Phase C (IN3) | IN3 | 黄色 |
//! | GP5 | Phase D (IN4) | IN4 | 橙色 |
//!
//! ```text
//!   Pico 2                 ULN2003          28BYJ-48
//!   GP2 (IN1) ──────────► IN1 ──► OUT1 ──── 蓝 (A)
//!   GP3 (IN2) ──────────► IN2 ──► OUT2 ──── 粉 (B)
//!   GP4 (IN3) ──────────► IN3 ──► OUT3 ──── 黄 (C)
//!   GP5 (IN4) ──────────► IN4 ──► OUT4 ──── 橙 (D)
//!   5V ─────────────────────────────────────  红 (VCC)
//!   GND ────────────────── GND
//! ```
//!
//! ---
//!
//! # 3. 参数说明
//!
//! | 参数 | 默认值 | 说明 |
//! |------|--------|------|
//! | `START_SPEED_HZ` | 100 | 起步频率 (Hz) |
//! | `MAX_SPEED_HZ` | 2000 | 最大步进频率 (Hz) |
//! | `ACCEL_STEPS` | 200 | 加速段总步数 |
//! | `STEP_PULSE_US` | 5 | STEP 脉冲宽度 (μs)，驱动器要求 ≥1μs |
//! | `MOTOR_STEPS_PER_REV` | 200 | 电机每转步数 (1.8° 电机 = 200 步) |
//! | `MICROSTEPPING` | 16 | 微步细分 (1/2/4/8/16/32) |
//!
//! ---
//!
//! # 4. 演示流程
//!
//! 程序启动后依次演示：
//! 1. STEP/DIR 模式 — 梯形加减速正转 1600 步 (1 圈 @16 细分)
//! 2. STEP/DIR 模式 — 反转 1600 步
//! 3. STEP/DIR 模式 — 连续运转 3 秒后停止
//! 4. 四相全步进 — 顺时针 200 步
//! 5. 四相半步进 — 逆时针 400 步
//!
//! ---
//!
//! # 5. 编译运行
//!
//! ```bash
//! cd pico-rs-examples
//! cargo run --bin stepper --target thumbv8m.main-none-eabihf
//! ```

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
use pico_rs::timer::Timer0;

#[unsafe(link_section = ".start_block")]
#[used]
pub static IMAGE_DEF: ImageDef = ImageDef::exe();

// ============================================================================
// 配置参数
// ============================================================================

/// 起步脉冲频率 (Hz) — 梯形加减速起始速度
const START_SPEED_HZ: u32 = 100;

/// 最大脉冲频率 (Hz) — 梯形加减速最高速度
const MAX_SPEED_HZ: u32 = 2000;

/// 加速段总步数 — 从 START_SPEED 加速到 MAX_SPEED 需要的步数
const ACCEL_STEPS: u32 = 200;

/// STEP 脉冲宽度 (μs)，驱动器要求 ≥1μs，推荐 ≥2μs
const STEP_PULSE_US: u32 = 5;

/// 电机每转步数（全步进，1.8° 电机 = 200）
const MOTOR_STEPS_PER_REV: u32 = 200;

/// 微步细分 (STEP/DIR 驱动器硬件设置)
const MICROSTEPPING: u32 = 16;

/// 步进脉冲间隔计算: interval_us = 1_000_000 / freq_hz
const fn speed_to_interval_us(hz: u32) -> u32 {
    if hz == 0 { 1_000_000 } else { 1_000_000 / hz }
}

// ---- GPIO 引脚分配 ----

/// STEP/DIR 模式引脚
const PIN_STEP: u32 = 2;
const PIN_DIR: u32 = 3;
const PIN_ENABLE: u32 = 4;

/// 四相驱动模式引脚 (IN1~IN4)
const PIN_IN1: u32 = 2;
const PIN_IN2: u32 = 3;
const PIN_IN3: u32 = 4;
const PIN_IN4: u32 = 5;

// ============================================================================
// 步进状态机
// ============================================================================

/// 运动阶段
#[derive(Clone, Copy, PartialEq)]
enum MotionPhase {
    /// 空闲
    Idle,
    /// 加速中
    Accelerating,
    /// 匀速运行
    Cruising,
    /// 减速中
    Decelerating,
    /// 连续运转模式
    Running,
}

// ============================================================================
// 全局共享状态 (Timer 中断使用)
// ============================================================================

static TIMER: Mutex<RefCell<Option<Timer0>>> = Mutex::new(RefCell::new(None));

/// 运动阶段
static MOTION_PHASE: AtomicU32 = AtomicU32::new(0); // 0=Idle

/// 当前步进间隔 (μs)
static STEP_INTERVAL_US: AtomicU32 = AtomicU32::new(0);

/// 已走步数
static STEPS_DONE: AtomicU32 = AtomicU32::new(0);

/// 目标步数 (0 = 连续运转)
static STEPS_TARGET: AtomicU32 = AtomicU32::new(0);

/// 减速起始步数
static DECEL_START_STEP: AtomicU32 = AtomicU32::new(0);

/// 方向 (0=正, 1=反)
static DIRECTION: AtomicU32 = AtomicU32::new(0);

/// 当前绝对位置 (步数)
static POSITION: AtomicI32 = AtomicI32::new(0);

/// 运动完成标志
static MOTION_DONE: AtomicBool = AtomicBool::new(true);

/// 脉冲状态 (true=高电平, false=低电平) — 用于生成脉冲
static PULSE_STATE: AtomicBool = AtomicBool::new(false);

// ============================================================================
// 梯形加减速核心算法
// ============================================================================

/// 根据当前步数计算步进间隔 (μs)
///
/// 梯形速度规划：线性加减速
fn compute_interval(step: u32, total_steps: u32) -> u32 {
    let phase = phase_from_u32(MOTION_PHASE.load(Ordering::Relaxed));

    let start_interval = speed_to_interval_us(START_SPEED_HZ);
    let max_interval = speed_to_interval_us(MAX_SPEED_HZ);

    match phase {
        MotionPhase::Accelerating => {
            // 线性插值: start_interval → max_interval (间隔越小速度越快)
            if ACCEL_STEPS == 0 {
                return max_interval;
            }
            let progress = step.min(ACCEL_STEPS);
            let interval = start_interval
                - ((start_interval - max_interval) as u64 * progress as u64 / ACCEL_STEPS as u64)
                    as u32;
            interval
        }
        MotionPhase::Cruising | MotionPhase::Running => max_interval,
        MotionPhase::Decelerating => {
            let decel_start = DECEL_START_STEP.load(Ordering::Relaxed);
            let steps_into_decel = step.saturating_sub(decel_start);
            let decel_steps = total_steps.saturating_sub(decel_start);
            if decel_steps == 0 {
                return start_interval;
            }
            let progress = steps_into_decel.min(decel_steps);
            let interval = max_interval
                + ((start_interval - max_interval) as u64 * progress as u64 / decel_steps as u64)
                    as u32;
            interval
        }
        MotionPhase::Idle => start_interval,
    }
}

fn phase_to_u32(phase: MotionPhase) -> u32 {
    match phase {
        MotionPhase::Idle => 0,
        MotionPhase::Accelerating => 1,
        MotionPhase::Cruising => 2,
        MotionPhase::Decelerating => 3,
        MotionPhase::Running => 4,
    }
}

fn phase_from_u32(val: u32) -> MotionPhase {
    match val {
        1 => MotionPhase::Accelerating,
        2 => MotionPhase::Cruising,
        3 => MotionPhase::Decelerating,
        4 => MotionPhase::Running,
        _ => MotionPhase::Idle,
    }
}

// ============================================================================
// 四相步进序列表
// ============================================================================

/// 全步进序列 (两相激励) — 力矩最大
/// A, B, C, D 对应 IN1~IN4 的高低电平
const FULL_STEP_SEQ: [[bool; 4]; 4] = [
    [true, false, true, false], // AB
    [false, true, true, false], // BC
    [false, true, false, true], // CD
    [true, false, false, true], // DA
];

/// 半步进序列 (全步进 + 单相交替) — 更平滑
const HALF_STEP_SEQ: [[bool; 4]; 8] = [
    [true, false, false, false], // A
    [true, false, true, false],  // AB
    [false, false, true, false], // B
    [false, true, true, false],  // BC
    [false, true, false, false], // C
    [false, true, false, true],  // CD
    [false, false, false, true], // D
    [true, false, false, true],  // DA
];

// ============================================================================
// STEP/DIR 驱动函数
// ============================================================================

/// 启动梯形加减速运动 (STEP/DIR 模式)
///
/// - `steps`: 目标步数 (正数)
/// - `dir`: 方向 (0=正转, 1=反转)
fn start_trapezoidal_move(steps: u32, dir: u32) {
    if steps == 0 {
        return;
    }

    MOTION_DONE.store(false, Ordering::Relaxed);
    STEPS_DONE.store(0, Ordering::Relaxed);
    STEPS_TARGET.store(steps, Ordering::Relaxed);
    DIRECTION.store(dir, Ordering::Relaxed);
    PULSE_STATE.store(false, Ordering::Relaxed);

    // 计算减速起始点
    let accel_steps = ACCEL_STEPS.min(steps / 2);
    let decel_steps = accel_steps;
    let decel_start = steps - decel_steps;
    DECEL_START_STEP.store(decel_start, Ordering::Relaxed);

    // 设置方向引脚
    let sio = unsafe { pico_rs::pac::SIO::steal() };
    if dir == 0 {
        sio.gpio_out_clr.write_bits(1 << PIN_DIR);
    } else {
        sio.gpio_out_set.write_bits(1 << PIN_DIR);
    }

    // 开始加速
    MOTION_PHASE.store(phase_to_u32(MotionPhase::Accelerating), Ordering::Relaxed);

    let interval = speed_to_interval_us(START_SPEED_HZ);
    STEP_INTERVAL_US.store(interval, Ordering::Relaxed);

    // 启动定时中断
    interrupt::free(|cs| {
        if let Some(ref timer) = *TIMER.borrow(cs).borrow() {
            let now = timer.get_time_us_32();
            timer.set_alarm(0, now.wrapping_add(interval));
            timer.enable_alarm_interrupt(0);
        }
    });
}

/// 启动连续运转 (STEP/DIR 模式)
fn start_continuous_run(dir: u32, speed_hz: u32) {
    MOTION_DONE.store(false, Ordering::Relaxed);
    STEPS_DONE.store(0, Ordering::Relaxed);
    STEPS_TARGET.store(0, Ordering::Relaxed);
    DIRECTION.store(dir, Ordering::Relaxed);
    PULSE_STATE.store(false, Ordering::Relaxed);

    let sio = unsafe { pico_rs::pac::SIO::steal() };
    if dir == 0 {
        sio.gpio_out_clr.write_bits(1 << PIN_DIR);
    } else {
        sio.gpio_out_set.write_bits(1 << PIN_DIR);
    }

    MOTION_PHASE.store(phase_to_u32(MotionPhase::Running), Ordering::Relaxed);

    let interval = speed_to_interval_us(speed_hz);
    STEP_INTERVAL_US.store(interval, Ordering::Relaxed);

    interrupt::free(|cs| {
        if let Some(ref timer) = *TIMER.borrow(cs).borrow() {
            let now = timer.get_time_us_32();
            timer.set_alarm(0, now.wrapping_add(interval));
            timer.enable_alarm_interrupt(0);
        }
    });
}

/// 停止运动
fn stop_motion() {
    MOTION_PHASE.store(phase_to_u32(MotionPhase::Idle), Ordering::Relaxed);
    MOTION_DONE.store(true, Ordering::Relaxed);

    // 确保 STEP 引脚低电平
    let sio = unsafe { pico_rs::pac::SIO::steal() };
    sio.gpio_out_clr.write_bits(1 << PIN_STEP);
}

/// 等待运动完成
fn wait_motion_done() {
    while !MOTION_DONE.load(Ordering::Relaxed) {
        pico_rs_rt::asm::wfi();
    }
}

// ============================================================================
// 四相直接驱动函数
// ============================================================================

/// 四相步进 — 全步进或半步进
///
/// - `steps`: 步数
/// - `dir`: 方向 (0=CW, 1=CCW)
/// - `delay_ms`: 每步延时 (ms)
/// - `half_step`: true 为半步进, false 为全步进
fn drive_four_phase(timer: &Timer0, steps: u32, dir: u32, delay_ms: u32, half_step: bool) {
    let sio = unsafe { pico_rs::pac::SIO::steal() };

    let seq: &[[bool; 4]] = if half_step {
        &HALF_STEP_SEQ
    } else {
        &FULL_STEP_SEQ
    };
    let seq_len = seq.len();

    let pins = [PIN_IN1, PIN_IN2, PIN_IN3, PIN_IN4];

    // 静态步序索引
    static PHASE_IDX: AtomicU32 = AtomicU32::new(0);
    let mut idx = PHASE_IDX.load(Ordering::Relaxed) as usize;

    for step in 0..steps {
        // 根据方向前进或后退
        if dir == 0 {
            idx = (idx + 1) % seq_len;
        } else {
            idx = (idx + seq_len - 1) % seq_len;
        }

        // 设置四个 GPIO
        let pattern = seq[idx];
        for (i, &on) in pattern.iter().enumerate() {
            if on {
                sio.gpio_out_set.write_bits(1 << pins[i]);
            } else {
                sio.gpio_out_clr.write_bits(1 << pins[i]);
            }
        }

        // 更新位置
        if dir == 0 {
            POSITION.fetch_add(1, Ordering::Relaxed);
        } else {
            POSITION.fetch_sub(1, Ordering::Relaxed);
        }

        // 延时
        timer.delay_ms(delay_ms);

        if step % 50 == 0 {
            let mode = if half_step { "half" } else { "full" };
            info!(
                "[4-phase {}] step {}/{}, pos={}",
                mode,
                step,
                steps,
                POSITION.load(Ordering::Relaxed)
            );
        }
    }

    // 断电（关闭所有线圈，省电 + 防止过热）
    for &pin in &pins {
        sio.gpio_out_clr.write_bits(1 << pin);
    }

    PHASE_IDX.store(idx as u32, Ordering::Relaxed);
    info!("[4-phase] done, pos={}", POSITION.load(Ordering::Relaxed));
}

// ============================================================================
// 入口
// ============================================================================

#[entry]
fn main() -> ! {
    info!("Stepper Motor — Open-Loop Control");

    // ------------------------------------------------------------------
    // 外设初始化
    // ------------------------------------------------------------------
    let dp = Peripherals::take().unwrap();
    let _clocks = ClockBuilder::new().build(dp.CLOCKS, dp.XOSC, dp.PLL_SYS, dp.PLL_USB);
    let pins = Pins::new(dp.IO_BANK0, dp.PADS_BANK0);
    let timer = Timer0::init(dp.TIMER0);

    // ------------------------------------------------------------------
    // GPIO 配置 — STEP/DIR/ENABLE + 四相 IN1~IN4
    // ------------------------------------------------------------------

    // GP2 = STEP / IN1
    let p2 = pins.pin::<2>();
    p2.into_output();
    p2.set_low();

    // GP3 = DIR / IN2
    let p3 = pins.pin::<3>();
    p3.into_output();
    p3.set_low();

    // GP4 = ENABLE / IN3
    let p4 = pins.pin::<4>();
    p4.into_output();
    p4.set_low();

    // GP5 = IN4 (四相模式)
    let p5 = pins.pin::<5>();
    p5.into_output();
    p5.set_low();

    info!("GPIO configured: GP2(STEP/IN1) GP3(DIR/IN2) GP4(EN/IN3) GP5(IN4)");

    let steps_per_rev = MOTOR_STEPS_PER_REV * MICROSTEPPING;

    // ------------------------------------------------------------------
    // Timer 中断配置
    // ------------------------------------------------------------------
    interrupt::free(|cs| {
        TIMER.borrow(cs).replace(Some(timer));
    });
    unsafe {
        interrupt::enable(Interrupt::TIMER0_IRQ_0);
    }

    // 使用定时器延时需要临界区访问
    let delay_ms = |ms: u32| {
        interrupt::free(|cs| {
            if let Some(ref t) = *TIMER.borrow(cs).borrow() {
                t.delay_ms(ms);
            }
        });
    };

    // ======================================================================
    // 演示 1: STEP/DIR — 梯形加减速正转 1 圈
    // ======================================================================
    info!(
        "=== Demo 1: STEP/DIR forward {} steps (1 rev) ===",
        steps_per_rev
    );

    // 使能驱动器 (ENABLE 低有效)
    let sio = unsafe { pico_rs::pac::SIO::steal() };
    sio.gpio_out_clr.write_bits(1 << PIN_ENABLE);

    start_trapezoidal_move(steps_per_rev, 0);
    wait_motion_done();

    info!(
        "Demo 1 done. Position: {} steps",
        POSITION.load(Ordering::Relaxed)
    );
    delay_ms(1000);

    // ======================================================================
    // 演示 2: STEP/DIR — 反转 1 圈
    // ======================================================================
    info!("=== Demo 2: STEP/DIR reverse {} steps ===", steps_per_rev);

    start_trapezoidal_move(steps_per_rev, 1);
    wait_motion_done();

    info!(
        "Demo 2 done. Position: {} steps",
        POSITION.load(Ordering::Relaxed)
    );
    delay_ms(1000);

    // ======================================================================
    // 演示 3: STEP/DIR — 连续运转 3 秒后停止
    // ======================================================================
    info!("=== Demo 3: STEP/DIR continuous run 3s ===");

    start_continuous_run(0, 1000);
    delay_ms(3000);
    stop_motion();

    info!(
        "Demo 3 done. Position: {} steps",
        POSITION.load(Ordering::Relaxed)
    );
    delay_ms(1000);

    // ======================================================================
    // 演示 4: 四相全步进 — 顺时针 200 步
    // ======================================================================
    info!("=== Demo 4: 4-phase full-step CW 200 steps ===");

    // 禁用 STEP/DIR 的 ENABLE，切换为四相模式
    sio.gpio_out_set.write_bits(1 << PIN_ENABLE);

    POSITION.store(0, Ordering::Relaxed);

    interrupt::free(|cs| {
        if let Some(ref t) = *TIMER.borrow(cs).borrow() {
            drive_four_phase(t, 200, 0, 5, false);
        }
    });

    delay_ms(1000);

    // ======================================================================
    // 演示 5: 四相半步进 — 逆时针 400 步
    // ======================================================================
    info!("=== Demo 5: 4-phase half-step CCW 400 steps ===");

    POSITION.store(0, Ordering::Relaxed);

    interrupt::free(|cs| {
        if let Some(ref t) = *TIMER.borrow(cs).borrow() {
            drive_four_phase(t, 400, 1, 3, true);
        }
    });

    delay_ms(1000);

    // ======================================================================
    // 循环: 交替正反转
    // ======================================================================
    info!("=== Loop: alternating STEP/DIR ===");

    // 重新使能 STEP/DIR 驱动器
    sio.gpio_out_clr.write_bits(1 << PIN_ENABLE);
    POSITION.store(0, Ordering::Relaxed);

    let mut forward = true;
    loop {
        let dir = if forward { 0 } else { 1 };
        let dir_str = if forward { "FWD" } else { "REV" };
        info!("[loop] {} {} steps", dir_str, steps_per_rev / 2);

        start_trapezoidal_move(steps_per_rev / 2, dir);
        wait_motion_done();

        info!("[loop] pos={} steps", POSITION.load(Ordering::Relaxed));

        delay_ms(500);
        forward = !forward;
    }
}

// ============================================================================
// Timer0 ISR — STEP 脉冲生成 + 梯形加减速
// ============================================================================

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
fn TIMER0_IRQ_0() {
    interrupt::free(|cs| {
        if let Some(ref timer) = *TIMER.borrow(cs).borrow() {
            timer.clear_alarm_interrupt(0);

            let phase = phase_from_u32(MOTION_PHASE.load(Ordering::Relaxed));
            if phase == MotionPhase::Idle {
                return;
            }

            let sio = unsafe { pico_rs::pac::SIO::steal() };

            // ---- STEP 脉冲生成 (上升沿 + 保持) ----
            let pulse_high = PULSE_STATE.load(Ordering::Relaxed);

            if !pulse_high {
                // 上升沿 → STEP 高电平
                sio.gpio_out_set.write_bits(1 << PIN_STEP);
                PULSE_STATE.store(true, Ordering::Relaxed);

                // 下一次中断: 脉冲宽度后拉低
                let now = timer.get_time_us_32();
                timer.set_alarm(0, now.wrapping_add(STEP_PULSE_US));
                return;
            }

            // ---- 下降沿 → STEP 低电平，准备下一步 ----
            sio.gpio_out_clr.write_bits(1 << PIN_STEP);
            PULSE_STATE.store(false, Ordering::Relaxed);

            // 计步
            let step = STEPS_DONE.fetch_add(1, Ordering::Relaxed) + 1;
            let total = STEPS_TARGET.load(Ordering::Relaxed);
            let dir = DIRECTION.load(Ordering::Relaxed);

            // 更新绝对位置
            if dir == 0 {
                POSITION.fetch_add(1, Ordering::Relaxed);
            } else {
                POSITION.fetch_sub(1, Ordering::Relaxed);
            }

            // ---- 连续运转模式 ----
            if phase == MotionPhase::Running {
                let interval = STEP_INTERVAL_US.load(Ordering::Relaxed);
                let now = timer.get_time_us_32();
                timer.set_alarm(0, now.wrapping_add(interval));
                return;
            }

            // ---- 梯形模式: 相位转换 ----
            if total > 0 && step >= total {
                // 目标到达 → 停止
                MOTION_PHASE.store(phase_to_u32(MotionPhase::Idle), Ordering::Relaxed);
                MOTION_DONE.store(true, Ordering::Relaxed);
                sio.gpio_out_clr.write_bits(1 << PIN_STEP);
                return;
            }

            // 检查是否应该切换阶段
            let decel_start = DECEL_START_STEP.load(Ordering::Relaxed);
            let new_phase = if step < ACCEL_STEPS.min(decel_start) {
                MotionPhase::Accelerating
            } else if step >= decel_start {
                MotionPhase::Decelerating
            } else {
                MotionPhase::Cruising
            };

            MOTION_PHASE.store(phase_to_u32(new_phase), Ordering::Relaxed);

            // 计算当前步的间隔
            let interval = compute_interval(step, total);
            STEP_INTERVAL_US.store(interval, Ordering::Relaxed);

            // 调度下一步脉冲
            let now = timer.get_time_us_32();
            timer.set_alarm(0, now.wrapping_add(interval));
        }
    });
}
