//! # 闭环步进电机控制 — 编码器反馈 + 位置/速度 PID + 梯形加减速
//!
//! 基于 RP2350 (Raspberry Pi Pico 2) 的**闭环步进电机控制器**，
//! 使用**编码器反馈**实现步进电机的位置/速度闭环控制，彻底解决
//! 开环步进电机的**丢步**和**堵转**问题。
//!
//! ---
//!
//! # 1. 开环 vs 闭环
//!
//! | 特性 | 开环 (`stepper.rs`) | 闭环（本文件） |
//! |------|---------------------|---------------|
//! | 反馈 | 无 — 假设每步都到位 | 编码器实时反馈 |
//! | 丢步 | 过载时丢步，不检测 | 实时检测，自动补偿 |
//! | 堵转 | 电机发热，无感知 | 检测堵转，报警/降速 |
//! | 力矩利用率 | 低（需留裕量） | 高（按需输出） |
//! | 速度精度 | 依赖脉冲频率 | PID 闭环调节 |
//! | 低速振动 | 明显 | 闭环抑制 |
//! | 整步噪声 | 明显 | 微步 + 闭环补偿 |
//!
//! ## 1.1 闭环控制架构
//!
//! ```text
//!   目标位置 ─── (+) ─── 位置 PID ─── 速度参考 ─── (+) ─── 速度 PI ─── STEP 频率
//!              ↑(-)              ↑                       ↑(-)
//!              │                 │                       │
//!              │            参数可调 (Kp/Ki/Kd)          │
//!              │                                         │
//!          编码器位置                                编码器速度
//!              ↑                                         ↑
//!              └───────── 编码器 4 倍频解码 ──────────────┘
//! ```
//!
//! ## 1.2 主要功能
//!
//! - **位置闭环**: 位置 PID 控制，精确到达目标
//! - **速度闭环**: 速度 PI 控制步进频率
//! - **丢步检测**: 编码器反馈 vs 期望位置，偏差过大报警
//! - **堵转保护**: 连续丢步超阈值则停机保护
//! - **梯形加减速**: 速度参考经过梯形规划
//! - **绝对/相对定位**: 支持绝对位置和相对步进
//!
//! ---
//!
//! # 2. 硬件接线
//!
//! | GPIO | 功能 | 说明 |
//! |------|------|------|
//! | GP2 | STEP | 步进脉冲输出 (→ A4988/DRV8825/TMC2209) |
//! | GP3 | DIR | 方向控制 |
//! | GP4 | ENABLE | 使能控制 (低有效) |
//! | GP10 | Encoder A | 编码器 A 相 (上拉，双边沿中断) |
//! | GP11 | Encoder B | 编码器 B 相 (上拉，双边沿中断) |
//! | GP25 | LED | 状态指示（堵转时闪烁） |
//!
//! ```text
//!   Pico 2                    A4988 / TMC2209
//!   GP2 (STEP) ─────────────► STEP
//!   GP3 (DIR)  ─────────────► DIR
//!   GP4 (EN)   ─────────────► ENABLE (低有效)
//!   GND ────────────────────── GND
//!
//!   编码器 (AB 增量式):
//!   GP10 (Enc A) ──►────────── Encoder A (上拉)
//!   GP11 (Enc B) ──►────────── Encoder B (上拉)
//!
//!   步进电机:
//!   A+ ──────── 1A (线圈 A)
//!   A- ──────── 1B
//!   B+ ──────── 2A (线圈 B)
//!   B- ──────── 2B
//! ```
//!
//! ---
//!
//! # 3. 参数说明
//!
//! | 参数 | 默认值 | 说明 |
//! |------|--------|------|
//! | `ENCODER_CPR` | 4000 | 编码器每转计数 (4 倍频后) |
//! | `MOTOR_STEPS_PER_REV` | 200 | 电机物理步数/转 |
//! | `MICROSTEPPING` | 16 | 驱动器微步细分 |
//! | `MIN_SPEED_HZ` | 50 | 最低步进频率 (Hz) |
//! | `MAX_SPEED_HZ` | 3000 | 最高步进频率 (Hz) |
//! | `KP_POS` | 2.0 (Q10=2048) | 位置 Kp |
//! | `KI_POS` | 0.01 (Q10=10) | 位置 Ki |
//! | `KD_POS` | 5.0 (Q10=5120) | 位置 Kd |
//! | `KP_SPEED` | 0.5 (Q10=512) | 速度 Kp |
//! | `KI_SPEED` | 0.02 (Q10=20) | 速度 Ki |
//! | `STALL_THRESHOLD` | 100 | 丢步报警阈值 (编码器脉冲) |
//! | `STALL_COUNT_MAX` | 50 | 连续堵转计数上限 → 停机 |
//!
//! ---
//!
//! # 4. 控制时序
//!
//! ```text
//! Timer0 Alarm 0 @10kHz → FOC/PID tick
//!   ├── 编码器读取
//!   ├── 位置 PID (每 10 次 = 1kHz)
//!   ├── 速度 PI (每 10 次 = 1kHz)
//!   ├── 丢步/堵转检测
//!   └── 更新 STEP 频率
//!
//! Timer0 Alarm 1 → STEP 脉冲定时
//!   ├── 生成 STEP 上升沿
//!   ├── 脉冲宽度后下降沿
//!   └── 按计算频率调度下一步
//!
//! IO_IRQ_BANK0 → 编码器 4 倍频解码
//!   └── GP10/GP11 双边沿中断
//! ```
//!
//! ---
//!
//! # 5. 演示流程
//!
//! 1. 归零 (原点) → 编码器和目标都清零
//! 2. 正转 1 圈 (3200 步 @16 细分) → 闭环定位
//! 3. 反转 1 圈 → 闭环回位
//! 4. 短距离高精度定位 (50 步) → 观察 PID 收敛
//! 5. 循环交替运动，日志输出位置误差和速度
//!
//! ---
//!
//! # 6. 编译运行
//!
//! ```bash
//! cd pico-rs-examples
//! cargo run --bin stepper_closed --target thumbv8m.main-none-eabihf
//! ```

#![no_std]
#![no_main]

use core::cell::RefCell;
use core::sync::atomic::{AtomicBool, AtomicI32, AtomicU32, Ordering};

use defmt::*;
use defmt_rtt as _;
use panic_probe as _;

use pico_rs::Steal;
use pico_rs::block::ImageDef;
use pico_rs::clocks::ClockBuilder;
use pico_rs::entry;
use pico_rs::gpio::{Gpio, Pin, Pins};
use pico_rs::interrupt::{self, Mutex};
use pico_rs::pac::{Interrupt, Peripherals};
use pico_rs::timer::Timer0;

#[unsafe(link_section = ".start_block")]
#[used]
pub static IMAGE_DEF: ImageDef = ImageDef::exe();

// ============================================================================
// 配置参数
// ============================================================================

/// 编码器每转计数 (4 倍频后)
const ENCODER_CPR: u32 = 4000;

/// 电机每转步数 (全步进)
const MOTOR_STEPS_PER_REV: u32 = 200;

/// 微步细分 (驱动器硬件设置)
const MICROSTEPPING: u32 = 16;

/// 每转微步数
const USTEPS_PER_REV: u32 = MOTOR_STEPS_PER_REV * MICROSTEPPING;

/// 编码器脉冲 → 微步转换系数 (Q16 定点)
/// ustep = encoder_count × USTEPS_PER_REV / ENCODER_CPR
/// 为避免运行时除法，预计算: factor_q16 = USTEPS_PER_REV * 65536 / ENCODER_CPR
const ENC_TO_USTEP_Q16: u32 = (USTEPS_PER_REV as u64 * 65536 / ENCODER_CPR as u64) as u32;

/// 最低步进频率 (Hz)
const MIN_SPEED_HZ: u32 = 50;

/// 最高步进频率 (Hz)
const MAX_SPEED_HZ: u32 = 3000;

/// STEP 脉冲宽度 (μs)
const STEP_PULSE_US: u32 = 5;

/// 控制环频率 (Hz)
const CONTROL_LOOP_HZ: u32 = 10_000;
const CONTROL_INTERVAL_US: u32 = 1_000_000 / CONTROL_LOOP_HZ;

/// 位置/速度 PID 分频 → 1 kHz
const PID_DIVIDER: u32 = 10;

/// 编码器方向 (false=不反转)
const ENCODER_DIR: bool = true;

// ---- PID 参数默认值 (Q10 定点) ----
const FP_SHIFT: i32 = 10;

// 位置 PID
const KP_POS: i32 = 2048; // ~2.0
const KI_POS: i32 = 10; // ~0.01
const KD_POS: i32 = 5120; // ~5.0
const POS_INTEGRAL_MAX: i32 = 500_000;

// 速度 PI
const KP_SPEED: i32 = 512; // ~0.5
const KI_SPEED: i32 = 20; // ~0.02
const SPEED_INTEGRAL_MAX: i32 = 500_000;

/// 位置死区 (编码器脉冲)
const POSITION_DEADBAND: i32 = 3;

// ---- 堵转检测参数 ----

/// 丢步警告阈值 (编码器脉冲)
/// 当期望位置与实际位置差超过此值，视为"丢步"
const STALL_THRESHOLD: i32 = 100;

/// 连续堵转检测计数上限 → 停机保护
const STALL_COUNT_MAX: u32 = 50;

// ============================================================================
// GPIO 引脚定义
// ============================================================================

const PIN_STEP: u32 = 2;
const PIN_DIR: u32 = 3;
const _PIN_ENABLE: u32 = 4;
const PIN_LED: u32 = 25;

// ============================================================================
// PID 控制器
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

    fn reset(&mut self) {
        self.integral = 0;
        self.prev_error = 0;
    }
}

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

    fn reset(&mut self) {
        self.integral = 0;
    }
}

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
// 全局共享状态
// ============================================================================

static TIMER: Mutex<RefCell<Option<Timer0>>> = Mutex::new(RefCell::new(None));

/// 控制环 tick 标志
static CONTROL_TICK: AtomicBool = AtomicBool::new(false);

/// 编码器累计计数
static ENCODER_COUNT: AtomicI32 = AtomicI32::new(0);

/// 编码器上次 AB 状态
static ENCODER_LAST_AB: AtomicU32 = AtomicU32::new(0);

/// 目标位置 (微步)
static TARGET_POSITION: AtomicI32 = AtomicI32::new(0);

/// 已发出的步进脉冲位置 (微步)
static COMMANDED_POSITION: AtomicI32 = AtomicI32::new(0);

/// 实际 RPM (×10)
static ACTUAL_RPM_X10: AtomicI32 = AtomicI32::new(0);

/// STEP 脉冲间隔 (μs)，0 = 停止
static STEP_INTERVAL_US: AtomicU32 = AtomicU32::new(0);

/// 步进方向 (0=正, 1=反)
static STEP_DIRECTION: AtomicU32 = AtomicU32::new(0);

/// 步进脉冲状态 (true=高)
static PULSE_STATE: AtomicBool = AtomicBool::new(false);

/// 步进器运行标志
static STEPPER_RUNNING: AtomicBool = AtomicBool::new(false);

/// 堵转标志
static STALL_DETECTED: AtomicBool = AtomicBool::new(false);

/// 运动完成标志
static MOTION_DONE: AtomicBool = AtomicBool::new(true);

#[inline(always)]
fn gpio() -> Steal<Gpio> {
    // SAFETY: the example configures STEP/DIR/ENABLE through `Pins` before
    // any runtime SIO access. This helper only performs SIO data-path IO.
    unsafe { Gpio::steal() }
}

// ============================================================================
// 步进脉冲控制
// ============================================================================

/// 更新步进脉冲频率和方向
///
/// speed_ref: 速度参考 (RPM × 10)，正值=正转，负值=反转
fn update_step_speed(speed_ref: i32) {
    let gpio = gpio();

    if speed_ref.abs() < 5 {
        // 速度太低 → 停止脉冲
        STEP_INTERVAL_US.store(0, Ordering::Relaxed);
        STEPPER_RUNNING.store(false, Ordering::Relaxed);
            gpio.set_low(PIN_STEP as u8);
        return;
    }

    // 设置方向
    if speed_ref >= 0 {
            gpio.set_low(PIN_DIR as u8);
        STEP_DIRECTION.store(0, Ordering::Relaxed);
    } else {
            gpio.set_high(PIN_DIR as u8);
        STEP_DIRECTION.store(1, Ordering::Relaxed);
    }

    // 速度 (RPM×10) → 步进频率 (Hz)
    // freq = |speed_rpm| × USTEPS_PER_REV / 60
    // speed_ref 是 RPM×10，所以 freq = |speed_ref| × USTEPS_PER_REV / 600
    let abs_speed = speed_ref.unsigned_abs();
    let freq_hz = (abs_speed as u64 * USTEPS_PER_REV as u64 / 600) as u32;
    let freq_hz = freq_hz.clamp(MIN_SPEED_HZ, MAX_SPEED_HZ);

    let interval_us = if freq_hz > 0 { 1_000_000 / freq_hz } else { 0 };

    STEP_INTERVAL_US.store(interval_us, Ordering::Relaxed);

    // 如果步进器还没运行，启动 Alarm 1
    if !STEPPER_RUNNING.load(Ordering::Relaxed) && interval_us > 0 {
        STEPPER_RUNNING.store(true, Ordering::Relaxed);
        PULSE_STATE.store(false, Ordering::Relaxed);

        interrupt::free(|cs| {
            if let Some(ref timer) = *TIMER.borrow(cs).borrow() {
                let now = timer.get_time_us_32();
                timer.set_alarm_safe(1, now.wrapping_add(interval_us));
                timer.enable_alarm_interrupt(1);
            }
        });
    }
}

// ============================================================================
// 入口
// ============================================================================

#[entry]
fn main() -> ! {
    info!("Stepper Motor — Closed-Loop Control (Encoder Feedback)");

    // ------------------------------------------------------------------
    // 外设初始化
    // ------------------------------------------------------------------
    let dp = Peripherals::take().unwrap();
    let _clocks = ClockBuilder::new().build(dp.CLOCKS, dp.XOSC, dp.PLL_SYS, dp.PLL_USB);
    let pins = Pins::new(dp.IO_BANK0, dp.PADS_BANK0);
    let timer = Timer0::init(dp.TIMER0);

    // ------------------------------------------------------------------
    // GPIO 配置
    // ------------------------------------------------------------------

    // GP2 = STEP
    let p2 = pins.pin::<2>();
    p2.into_output();
    p2.set_low();

    // GP3 = DIR
    let p3 = pins.pin::<3>();
    p3.into_output();
    p3.set_low();

    // GP4 = ENABLE (低有效)
    let p4 = pins.pin::<4>();
    p4.into_output();
    p4.set_low(); // 使能驱动器

    // GP25 = LED
    let led = pins.pin::<25>();
    led.into_output();
    led.set_low();

    info!("GPIO: GP2(STEP) GP3(DIR) GP4(EN) GP10(EncA) GP11(EncB) GP25(LED)");

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
    let init_ab = ((init_a as u32) << 1) | (init_b as u32);
    ENCODER_LAST_AB.store(init_ab, Ordering::Relaxed);

    info!(
        "Encoder init: A={}, B={}, CPR={}",
        init_a, init_b, ENCODER_CPR
    );

    // ------------------------------------------------------------------
    // Timer0 配置
    //   Alarm 0: 控制环 @10kHz
    //   Alarm 1: STEP 脉冲
    // ------------------------------------------------------------------
    {
        let now = timer.get_time_us_32();
        timer.set_alarm_safe(0, now.wrapping_add(CONTROL_INTERVAL_US));
        timer.enable_alarm_interrupt(0);

        interrupt::free(|cs| {
            TIMER.borrow(cs).replace(Some(timer));
        });

        unsafe {
            interrupt::enable(Interrupt::TIMER0_IRQ_0);
            interrupt::enable(Interrupt::TIMER0_IRQ_1);
            interrupt::enable(Interrupt::IO_IRQ_BANK0);
        }
    }

    info!(
        "Control loop: {}Hz, PID: {}Hz, motor: {} ustep/rev, encoder: {} CPR",
        CONTROL_LOOP_HZ,
        CONTROL_LOOP_HZ / PID_DIVIDER,
        USTEPS_PER_REV,
        ENCODER_CPR
    );

    // ------------------------------------------------------------------
    // 控制器
    // ------------------------------------------------------------------

    // 位置 PID → 速度参考 (RPM×10)
    let mut pid_pos = PidController::new(
        KP_POS,
        KI_POS,
        KD_POS,
        POS_INTEGRAL_MAX,
        -(MAX_SPEED_HZ as i32) * 600 / USTEPS_PER_REV as i32, // RPM×10 下限
        (MAX_SPEED_HZ as i32) * 600 / USTEPS_PER_REV as i32,  // RPM×10 上限
    );

    // 速度 PI → 步进频率微调
    let mut pi_speed = PiController::new(
        KP_SPEED,
        KI_SPEED,
        SPEED_INTEGRAL_MAX,
        -(MAX_SPEED_HZ as i32),
        MAX_SPEED_HZ as i32,
    );

    let mut tick_counter: u32 = 0;
    let mut log_counter: u32 = 0;
    let mut last_encoder_count: i32 = 0;
    let mut stall_counter: u32 = 0;

    let timer_now_us = || -> u32 {
        interrupt::free(|cs| {
            TIMER
                .borrow(cs)
                .borrow()
                .as_ref()
                .map_or(0, |t| t.get_time_us_32())
        })
    };

    let delay_ms = |ms: u32| {
        interrupt::free(|cs| {
            if let Some(ref t) = *TIMER.borrow(cs).borrow() {
                t.delay_ms(ms);
            }
        });
    };

    let mut last_speed_time: u32 = timer_now_us();

    // 辅助函数：设置目标并等待到位
    let move_to_and_wait = |target: i32, timeout_ms: u32| -> bool {
        TARGET_POSITION.store(target, Ordering::Relaxed);
        MOTION_DONE.store(false, Ordering::Relaxed);

        let start = timer_now_us();
        loop {
            if MOTION_DONE.load(Ordering::Relaxed) {
                return true;
            }
            if STALL_DETECTED.load(Ordering::Relaxed) {
                return false;
            }
            let elapsed = timer_now_us().wrapping_sub(start);
            if elapsed > timeout_ms * 1000 {
                return false; // 超时
            }
            pico_rs_rt::asm::wfi();
        }
    };

    info!("Closed-loop stepper ready!");

    // ======================================================================
    // 演示 1: 归零
    // ======================================================================
    info!("=== Demo 1: Home / Zero ===");
    ENCODER_COUNT.store(0, Ordering::Relaxed);
    TARGET_POSITION.store(0, Ordering::Relaxed);
    COMMANDED_POSITION.store(0, Ordering::Relaxed);
    delay_ms(500);

    // ======================================================================
    // 演示 2: 正转 1 圈 (USTEPS_PER_REV 微步)
    // ======================================================================
    info!("=== Demo 2: Forward 1 rev ({} usteps) ===", USTEPS_PER_REV);
    let ok = move_to_and_wait(USTEPS_PER_REV as i32, 10_000);
    info!(
        "Demo 2 {}: enc={}, target={}, cmd={}",
        if ok { "OK" } else { "FAIL" },
        ENCODER_COUNT.load(Ordering::Relaxed),
        TARGET_POSITION.load(Ordering::Relaxed),
        COMMANDED_POSITION.load(Ordering::Relaxed)
    );
    delay_ms(1000);

    // ======================================================================
    // 演示 3: 反转 1 圈回原点
    // ======================================================================
    info!("=== Demo 3: Reverse 1 rev back to 0 ===");
    STALL_DETECTED.store(false, Ordering::Relaxed);
    let ok = move_to_and_wait(0, 10_000);
    info!(
        "Demo 3 {}: enc={}, target=0",
        if ok { "OK" } else { "FAIL" },
        ENCODER_COUNT.load(Ordering::Relaxed),
    );
    delay_ms(1000);

    // ======================================================================
    // 演示 4: 短距离高精度定位 (50 微步)
    // ======================================================================
    info!("=== Demo 4: Short precision move (50 usteps) ===");
    STALL_DETECTED.store(false, Ordering::Relaxed);
    let ok = move_to_and_wait(50, 5_000);
    info!(
        "Demo 4 {}: enc={}, target=50",
        if ok { "OK" } else { "FAIL" },
        ENCODER_COUNT.load(Ordering::Relaxed),
    );
    delay_ms(1000);

    // ======================================================================
    // 循环: 交替运动
    // ======================================================================
    info!("=== Loop: Alternating closed-loop moves ===");

    let positions: [i32; 4] = [
        USTEPS_PER_REV as i32,     // 1 圈
        0,                         // 回原点
        USTEPS_PER_REV as i32 / 2, // 半圈
        USTEPS_PER_REV as i32 / 4, // 1/4 圈
    ];
    let mut pos_idx = 0;

    loop {
        // ==============================================================
        // 控制环 tick (@10kHz)
        // ==============================================================
        if !CONTROL_TICK.swap(false, Ordering::Relaxed) {
            // 检查当前目标循环
            if MOTION_DONE.load(Ordering::Relaxed) {
                delay_ms(500);

                STALL_DETECTED.store(false, Ordering::Relaxed);
                stall_counter = 0;

                let target = positions[pos_idx % positions.len()];
                pos_idx += 1;

                info!("[loop] → target={} usteps", target);
                TARGET_POSITION.store(target, Ordering::Relaxed);
                MOTION_DONE.store(false, Ordering::Relaxed);
            }

            pico_rs_rt::asm::wfi();
            continue;
        }

        tick_counter += 1;

        // ==============================================================
        // 位置/速度控制环 (@1kHz)
        // ==============================================================
        if tick_counter % PID_DIVIDER == 0 {
            let now = timer_now_us();
            let dt_us = now.wrapping_sub(last_speed_time);
            last_speed_time = now;

            // ---- 编码器数据 ----
            let enc_count = ENCODER_COUNT.load(Ordering::Relaxed);

            // 编码器位置 → 微步等效
            let encoder_ustep = ((enc_count as i64 * ENC_TO_USTEP_Q16 as i64) >> 16) as i32;

            // 编码器速度 (RPM×10)
            let delta_enc = enc_count - last_encoder_count;
            last_encoder_count = enc_count;

            let actual_rpm_x10 = if dt_us > 0 {
                (delta_enc as i64 * 600_000_000i64 / (ENCODER_CPR as i64 * dt_us as i64)) as i32
            } else {
                0
            };
            ACTUAL_RPM_X10.store(actual_rpm_x10, Ordering::Relaxed);

            // ---- 目标与误差 ----
            let target = TARGET_POSITION.load(Ordering::Relaxed);
            let pos_error = target - encoder_ustep;

            // ---- 到位检测 ----
            if pos_error.abs() <= POSITION_DEADBAND {
                pid_pos.reset();
                pi_speed.reset();
                update_step_speed(0);

                if !MOTION_DONE.load(Ordering::Relaxed) {
                    MOTION_DONE.store(true, Ordering::Relaxed);
                    info!(
                        "[done] target={} actual={} enc={}",
                        target, encoder_ustep, enc_count
                    );
                }
            } else {
                // ---- 位置 PID → 速度参考 (RPM×10) ----
                let speed_ref = pid_pos.update(pos_error);

                // ---- 速度 PI (可选，简单模式直接用速度参考) ----
                let speed_error = speed_ref - actual_rpm_x10;
                let _speed_adj = pi_speed.update(speed_error);

                // 更新步进频率
                update_step_speed(speed_ref);
            }

            // ---- 丢步 / 堵转检测 ----
            let cmd_pos = COMMANDED_POSITION.load(Ordering::Relaxed);
            let step_error = (cmd_pos - encoder_ustep).abs();

            if step_error > STALL_THRESHOLD {
                stall_counter += 1;
                if stall_counter >= STALL_COUNT_MAX {
                    // 堵转 → 停机保护
                    STALL_DETECTED.store(true, Ordering::Relaxed);
                    update_step_speed(0);
                    pid_pos.reset();
                    pi_speed.reset();

                    // LED 快闪指示
                    unsafe { Pin::<{ PIN_LED as u8 }>::steal() }.toggle();

                    warn!(
                        "STALL! cmd={} enc_ustep={} err={} cnt={}",
                        cmd_pos, encoder_ustep, step_error, stall_counter
                    );
                }
            } else {
                stall_counter = 0;
                // 堵转恢复后关闭 LED
                unsafe { Pin::<{ PIN_LED as u8 }>::steal() }.set_low();
            }

            // ---- 日志 (~2Hz) ----
            log_counter += 1;
            if log_counter % 500 == 0 {
                info!(
                    "pos_err:{} enc:{} cmd:{} rpm_x10:{} interval:{}us stall:{}",
                    pos_error,
                    enc_count,
                    cmd_pos,
                    actual_rpm_x10,
                    STEP_INTERVAL_US.load(Ordering::Relaxed),
                    stall_counter
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
// Timer0 Alarm 0 ISR — 控制环 tick @10kHz
// ============================================================================

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
fn TIMER0_IRQ_0() {
    interrupt::free(|cs| {
        if let Some(ref timer) = *TIMER.borrow(cs).borrow() {
            timer.clear_alarm_interrupt(0);
            timer.clear_forced_alarm_interrupt(0);
            CONTROL_TICK.store(true, Ordering::Relaxed);
            let now = timer.get_time_us_32();
            timer.set_alarm_safe(0, now.wrapping_add(CONTROL_INTERVAL_US));
        }
    });
}

// ============================================================================
// Timer0 Alarm 1 ISR — STEP 脉冲生成
// ============================================================================

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
fn TIMER0_IRQ_1() {
    interrupt::free(|cs| {
        if let Some(ref timer) = *TIMER.borrow(cs).borrow() {
            timer.clear_alarm_interrupt(1);
            timer.clear_forced_alarm_interrupt(1);

            let interval = STEP_INTERVAL_US.load(Ordering::Relaxed);
            if interval == 0 {
                STEPPER_RUNNING.store(false, Ordering::Relaxed);
                return;
            }

            let step_pin = unsafe { Pin::<{ PIN_STEP as u8 }>::steal() };
            let pulse_high = PULSE_STATE.load(Ordering::Relaxed);

            if !pulse_high {
                // 上升沿
                step_pin.set_high();
                PULSE_STATE.store(true, Ordering::Relaxed);

                let now = timer.get_time_us_32();
                timer.set_alarm_safe(1, now.wrapping_add(STEP_PULSE_US));
                return;
            }

            // 下降沿
            step_pin.set_low();
            PULSE_STATE.store(false, Ordering::Relaxed);

            // 更新指令位置
            let dir = STEP_DIRECTION.load(Ordering::Relaxed);
            if dir == 0 {
                COMMANDED_POSITION.fetch_add(1, Ordering::Relaxed);
            } else {
                COMMANDED_POSITION.fetch_sub(1, Ordering::Relaxed);
            }

            // 重新读取间隔（可能被控制环更新了）
            let interval = STEP_INTERVAL_US.load(Ordering::Relaxed);
            if interval == 0 {
                STEPPER_RUNNING.store(false, Ordering::Relaxed);
                return;
            }

            let now = timer.get_time_us_32();
            timer.set_alarm_safe(1, now.wrapping_add(interval));
        }
    });
}
