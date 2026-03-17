//! 步进电机控制 — 梯形加减速 + 多轴支持
//!
//! 提供步进电机的公共控制逻辑，支持:
//! - 单轴或多轴 (X/Y/Z) 独立控制
//! - 梯形加减速 (Trapezoidal Velocity Profile)
//! - 连续运转模式
//! - Timer 中断驱动的 STEP 脉冲生成 (两段式: 上升沿 + 下降沿)
//!
//! # 架构
//!
//! 每个步进轴对应一个 [`AxisState`] 实例（声明为 `static`），
//! 绑定到一个 Timer alarm 通道。RP2350 Timer0 有 4 个 alarm，
//! 因此单个 Timer0 即可独立驱动 **4 个轴**。若需更多轴可启用 Timer1。
//!
//! ```text
//!   Timer0
//!   ├── Alarm 0 → TIMER0_IRQ_0 → AXIS_X.handle_alarm_isr()
//!   ├── Alarm 1 → TIMER0_IRQ_1 → AXIS_Y.handle_alarm_isr()
//!   ├── Alarm 2 → TIMER0_IRQ_2 → AXIS_Z.handle_alarm_isr()
//!   └── Alarm 3 → (可用于其他用途)
//! ```
//!
//! # 使用示例
//!
//! ```ignore
//! use stepper_common::{AxisState, AxisConfig, Direction};
//!
//! // 声明轴 (全局 static，供 ISR 使用)
//! static AXIS_X: AxisState = AxisState::new();
//! static AXIS_Y: AxisState = AxisState::new();
//!
//! fn main() {
//!     // 配置各轴
//!     AXIS_X.configure(&AxisConfig {
//!         pin_step: 2, pin_dir: 3, alarm: 0,
//!         start_speed_hz: 200, accel_steps: 400, step_pulse_us: 2,
//!     });
//!     AXIS_Y.configure(&AxisConfig {
//!         pin_step: 10, pin_dir: 11, alarm: 1,
//!         start_speed_hz: 200, accel_steps: 400, step_pulse_us: 2,
//!     });
//!
//!     // 启动运动 (需在 interrupt::free 中访问 timer)
//!     interrupt::free(|cs| {
//!         if let Some(ref timer) = *TIMER.borrow(cs).borrow() {
//!             AXIS_X.start_move(timer, 3200, Direction::Forward, 8000);
//!             AXIS_Y.start_move(timer, 1600, Direction::Reverse, 4000);
//!         }
//!     });
//!
//!     // 等待完成
//!     AXIS_X.wait_done();
//!     AXIS_Y.wait_done();
//! }
//!
//! // ISR: 每个轴对应 1 个 timer alarm
//! fn TIMER0_IRQ_0() {
//!     interrupt::free(|cs| {
//!         if let Some(ref timer) = *TIMER.borrow(cs).borrow() {
//!             AXIS_X.handle_alarm_isr(timer);
//!         }
//!     });
//! }
//! fn TIMER0_IRQ_1() {
//!     interrupt::free(|cs| {
//!         if let Some(ref timer) = *TIMER.borrow(cs).borrow() {
//!             AXIS_Y.handle_alarm_isr(timer);
//!         }
//!     });
//! }
//! ```

use core::sync::atomic::{AtomicBool, AtomicI32, AtomicU8, AtomicU32, Ordering};

use pico_rs::gpio::Gpio;

#[inline(always)]
fn gpio() -> Gpio {
    // SAFETY: these examples configure GPIO direction and function once during
    // initialization, then use `Gpio` only for runtime SIO data-path access.
    unsafe { Gpio::new() }
}

// ============================================================================
// Timer 抽象 — 统一 Timer0 / Timer1 接口
// ============================================================================

/// 步进电机控制所需的 Timer 操作接口
///
/// Timer0 和 Timer1 均自动实现此 trait。
pub trait StepperTimer {
    fn get_time_us_32(&self) -> u32;
    fn set_alarm_safe(&self, alarm: u8, time: u32);
    fn enable_alarm_interrupt(&self, alarm: u8);
    fn clear_alarm_interrupt(&self, alarm: u8);
    fn clear_forced_alarm_interrupt(&self, alarm: u8);
}

macro_rules! impl_stepper_timer {
    ($Timer:ty) => {
        impl StepperTimer for $Timer {
            #[inline]
            fn get_time_us_32(&self) -> u32 {
                self.get_time_us_32()
            }
            #[inline]
            fn set_alarm_safe(&self, alarm: u8, time: u32) {
                self.set_alarm_safe(alarm, time)
            }
            #[inline]
            fn enable_alarm_interrupt(&self, alarm: u8) {
                self.enable_alarm_interrupt(alarm)
            }
            #[inline]
            fn clear_alarm_interrupt(&self, alarm: u8) {
                self.clear_alarm_interrupt(alarm)
            }
            #[inline]
            fn clear_forced_alarm_interrupt(&self, alarm: u8) {
                self.clear_forced_alarm_interrupt(alarm)
            }
        }
    };
}

impl_stepper_timer!(pico_rs::timer::Timer0);
impl_stepper_timer!(pico_rs::timer::Timer1);

// ============================================================================
// 公共类型
// ============================================================================

/// 运动方向
#[derive(Clone, Copy, PartialEq)]
pub enum Direction {
    Forward = 0,
    Reverse = 1,
}

/// 运动阶段
///
/// ```text
///   Idle → Accelerating → Cruising → Decelerating → Idle
///                           ↑                        ↑
///                      (步数不够时直接跳到减速)       |
///   Idle → Running ──────────────────────────────────┘
///          (连续运转模式, 外部调用 stop() 退出)
/// ```
#[derive(Clone, Copy, PartialEq)]
pub enum MotionPhase {
    Idle = 0,
    Accelerating = 1,
    Cruising = 2,
    Decelerating = 3,
    Running = 4,
}

impl MotionPhase {
    #[inline]
    fn from_u32(v: u32) -> Self {
        match v {
            1 => Self::Accelerating,
            2 => Self::Cruising,
            3 => Self::Decelerating,
            4 => Self::Running,
            _ => Self::Idle,
        }
    }
}

/// 轴配置参数
pub struct AxisConfig {
    /// STEP 引脚 GPIO 编号
    pub pin_step: u8,
    /// DIR 引脚 GPIO 编号
    pub pin_dir: u8,
    /// Timer alarm 编号 (0-3)
    pub alarm: u8,
    /// 起步频率 (Hz) — 梯形加减速的最低速度
    pub start_speed_hz: u32,
    /// 加速段总步数 — 从起步频率加速到目标频率所需步数
    pub accel_steps: u32,
    /// STEP 脉冲高电平宽度 (μs)
    pub step_pulse_us: u32,
}

/// Hz → μs 间隔转换
#[inline]
pub const fn speed_to_interval_us(hz: u32) -> u32 {
    if hz == 0 { 1_000_000 } else { 1_000_000 / hz }
}

// ============================================================================
// 轴状态 — 每个步进轴的完整运行时状态
// ============================================================================

/// 单个步进轴的运行时状态
///
/// 所有字段均为原子类型，可安全地在 ISR 与主线程间共享。
/// 使用 `const fn new()` 创建，可直接放在 `static` 中。
///
/// # 生命周期
///
/// 1. `static AXIS: AxisState = AxisState::new()` — 编译时创建
/// 2. `AXIS.configure(&config)` — 运行时设置引脚和参数 (ISR 使能前调用)
/// 3. `AXIS.start_move(timer, ...)` — 启动运动
/// 4. ISR 中调用 `AXIS.handle_alarm_isr(timer)` — 自动脉冲生成
/// 5. `AXIS.wait_done()` — 阻塞等待完成
pub struct AxisState {
    // ---- 配置 (configure() 设置，之后只读) ----
    pin_step: AtomicU8,
    pin_dir: AtomicU8,
    alarm: AtomicU8,
    start_speed_hz: AtomicU32,
    accel_steps: AtomicU32,
    step_pulse_us: AtomicU32,

    // ---- 运行时状态 ----
    motion_phase: AtomicU32,
    max_speed_hz: AtomicU32,
    step_interval_us: AtomicU32,
    steps_done: AtomicU32,
    steps_target: AtomicU32,
    decel_start_step: AtomicU32,
    direction: AtomicU32,
    position: AtomicI32,
    motion_done: AtomicBool,
    pulse_state: AtomicBool,
}

// Safety: all fields are atomic types, safe to share between threads/ISRs
unsafe impl Sync for AxisState {}

impl AxisState {
    /// 创建一个未配置的轴状态 (用于 static 初始化)
    pub const fn new() -> Self {
        Self {
            pin_step: AtomicU8::new(0),
            pin_dir: AtomicU8::new(0),
            alarm: AtomicU8::new(0),
            start_speed_hz: AtomicU32::new(200),
            accel_steps: AtomicU32::new(200),
            step_pulse_us: AtomicU32::new(5),

            motion_phase: AtomicU32::new(0),
            max_speed_hz: AtomicU32::new(1000),
            step_interval_us: AtomicU32::new(0),
            steps_done: AtomicU32::new(0),
            steps_target: AtomicU32::new(0),
            decel_start_step: AtomicU32::new(0),
            direction: AtomicU32::new(0),
            position: AtomicI32::new(0),
            motion_done: AtomicBool::new(true),
            pulse_state: AtomicBool::new(false),
        }
    }

    /// 配置轴参数 (必须在 ISR 使能前调用)
    pub fn configure(&self, config: &AxisConfig) {
        self.pin_step.store(config.pin_step, Ordering::Relaxed);
        self.pin_dir.store(config.pin_dir, Ordering::Relaxed);
        self.alarm.store(config.alarm, Ordering::Relaxed);
        self.start_speed_hz
            .store(config.start_speed_hz, Ordering::Relaxed);
        self.accel_steps
            .store(config.accel_steps, Ordering::Relaxed);
        self.step_pulse_us
            .store(config.step_pulse_us, Ordering::Relaxed);
    }

    // ====================================================================
    // 状态查询
    // ====================================================================

    /// 当前运动阶段
    #[inline]
    pub fn phase(&self) -> MotionPhase {
        MotionPhase::from_u32(self.motion_phase.load(Ordering::Relaxed))
    }

    /// 当前绝对位置 (有符号步数)
    #[inline]
    pub fn position(&self) -> i32 {
        self.position.load(Ordering::Relaxed)
    }

    /// 已完成步数
    #[inline]
    pub fn steps_done(&self) -> u32 {
        self.steps_done.load(Ordering::Relaxed)
    }

    /// 运动是否完成
    #[inline]
    pub fn is_done(&self) -> bool {
        self.motion_done.load(Ordering::Relaxed)
    }

    /// 设置绝对位置 (用于归零)
    #[inline]
    pub fn set_position(&self, pos: i32) {
        self.position.store(pos, Ordering::Relaxed);
    }

    // ====================================================================
    // 运动控制
    // ====================================================================

    /// 启动梯形加减速运动
    ///
    /// # 参数
    /// - `timer`: Timer 外设引用
    /// - `steps`: 目标步数 (微步)
    /// - `dir`: 运动方向
    /// - `max_speed_hz`: 本次运动的最大步进频率 (Hz)
    pub fn start_move<T: StepperTimer>(
        &self,
        timer: &T,
        steps: u32,
        dir: Direction,
        max_speed_hz: u32,
    ) {
        if steps == 0 {
            return;
        }

        let accel = self.accel_steps.load(Ordering::Relaxed);
        let start_speed = self.start_speed_hz.load(Ordering::Relaxed);
        let alarm = self.alarm.load(Ordering::Relaxed);
        let pin_dir = self.pin_dir.load(Ordering::Relaxed);

        self.max_speed_hz.store(max_speed_hz, Ordering::Relaxed);
        self.motion_done.store(false, Ordering::Relaxed);
        self.steps_done.store(0, Ordering::Relaxed);
        self.steps_target.store(steps, Ordering::Relaxed);
        self.direction.store(dir as u32, Ordering::Relaxed);
        self.pulse_state.store(false, Ordering::Relaxed);

        let accel_steps = accel.min(steps / 2);
        let decel_start = steps - accel_steps;
        self.decel_start_step.store(decel_start, Ordering::Relaxed);

        // 设置方向引脚
        let gpio = gpio();
        gpio.set(pin_dir, dir as u32 != 0);

        self.motion_phase
            .store(MotionPhase::Accelerating as u32, Ordering::Relaxed);

        let interval = speed_to_interval_us(start_speed);
        self.step_interval_us.store(interval, Ordering::Relaxed);

        let now = timer.get_time_us_32();
        timer.set_alarm_safe(alarm, now.wrapping_add(interval));
        timer.enable_alarm_interrupt(alarm);
    }

    /// 启动连续运转模式
    ///
    /// 以恒定速度持续运行，直到调用 [`stop()`](Self::stop) 停止。
    /// 不使用梯形加减速。
    ///
    /// # 参数
    /// - `timer`: Timer 外设引用
    /// - `dir`: 运动方向
    /// - `speed_hz`: 步进频率 (Hz)
    pub fn start_continuous<T: StepperTimer>(&self, timer: &T, dir: Direction, speed_hz: u32) {
        let alarm = self.alarm.load(Ordering::Relaxed);
        let pin_dir = self.pin_dir.load(Ordering::Relaxed);

        self.motion_done.store(false, Ordering::Relaxed);
        self.steps_done.store(0, Ordering::Relaxed);
        self.steps_target.store(0, Ordering::Relaxed);
        self.direction.store(dir as u32, Ordering::Relaxed);
        self.pulse_state.store(false, Ordering::Relaxed);

        let gpio = gpio();
        gpio.set(pin_dir, dir as u32 != 0);

        self.motion_phase
            .store(MotionPhase::Running as u32, Ordering::Relaxed);

        let interval = speed_to_interval_us(speed_hz);
        self.step_interval_us.store(interval, Ordering::Relaxed);

        let now = timer.get_time_us_32();
        timer.set_alarm_safe(alarm, now.wrapping_add(interval));
        timer.enable_alarm_interrupt(alarm);
    }

    /// 立即停止运动 (急停)
    ///
    /// 将状态机置为 Idle 并拉低 STEP 引脚。
    /// 不执行减速过程。
    pub fn stop(&self) {
        self.motion_phase
            .store(MotionPhase::Idle as u32, Ordering::Relaxed);
        self.motion_done.store(true, Ordering::Relaxed);

        gpio().set_low(self.pin_step.load(Ordering::Relaxed));
    }

    /// 阻塞等待运动完成 (使用 WFI 省电)
    pub fn wait_done(&self) {
        while !self.motion_done.load(Ordering::Relaxed) {
            pico_rs_rt::asm::wfi();
        }
    }

    // ====================================================================
    // ISR 处理 — 在 Timer alarm 中断中调用
    // ====================================================================

    /// Timer alarm ISR 处理函数
    ///
    /// 在对应的 Timer alarm 中断中调用此方法，实现 STEP 脉冲生成和
    /// 梯形加减速。每个轴对应一个 alarm，ISR 只需清除中断并调用此方法。
    ///
    /// # 脉冲生成原理 (两段式中断)
    ///
    /// ```text
    ///   时间: ──┬──────┬────────────────────────┬──────┬─────
    ///           │ ISR1 │                        │ ISR2 │
    ///   STEP: __/‾‾‾‾‾‾\________________________/‾‾‾‾‾‾\____
    ///           │←脉宽→│←── step_interval_us ──→│
    /// ```
    ///
    /// - 第 1 次中断: 拉高 STEP (上升沿驱动电机)，设 alarm = now + 脉宽
    /// - 第 2 次中断: 拉低 STEP，计步，更新速度，设 alarm = now + 间隔
    pub fn handle_alarm_isr<T: StepperTimer>(&self, timer: &T) {
        let alarm = self.alarm.load(Ordering::Relaxed);

        // 清除中断 (自然中断 + 强制中断)
        timer.clear_alarm_interrupt(alarm);
        timer.clear_forced_alarm_interrupt(alarm);

        let phase = MotionPhase::from_u32(self.motion_phase.load(Ordering::Relaxed));
        if phase == MotionPhase::Idle {
            return;
        }

        let pin_step = self.pin_step.load(Ordering::Relaxed);
        let gpio = gpio();

        let pulse_high = self.pulse_state.load(Ordering::Relaxed);

        if !pulse_high {
            // ---- 第 1 阶段: STEP 上升沿 ----
            gpio.set_high(pin_step);
            self.pulse_state.store(true, Ordering::Relaxed);

            let pulse_us = self.step_pulse_us.load(Ordering::Relaxed);
            let now = timer.get_time_us_32();
            timer.set_alarm_safe(alarm, now.wrapping_add(pulse_us));
            return;
        }

        // ---- 第 2 阶段: STEP 下降沿 + 状态更新 ----
        gpio.set_low(pin_step);
        self.pulse_state.store(false, Ordering::Relaxed);

        let step = self.steps_done.fetch_add(1, Ordering::Relaxed) + 1;
        let total = self.steps_target.load(Ordering::Relaxed);
        let dir = self.direction.load(Ordering::Relaxed);

        // 更新绝对位置
        if dir == 0 {
            self.position.fetch_add(1, Ordering::Relaxed);
        } else {
            self.position.fetch_sub(1, Ordering::Relaxed);
        }

        // 连续运转模式: 不检查步数
        if phase == MotionPhase::Running {
            let interval = self.step_interval_us.load(Ordering::Relaxed);
            let now = timer.get_time_us_32();
            timer.set_alarm_safe(alarm, now.wrapping_add(interval));
            return;
        }

        // 梯形模式: 到达目标 → 停止
        if total > 0 && step >= total {
            self.motion_phase
                .store(MotionPhase::Idle as u32, Ordering::Relaxed);
            self.motion_done.store(true, Ordering::Relaxed);
            return;
        }

        // 梯形模式: 阶段切换
        let accel = self.accel_steps.load(Ordering::Relaxed);
        let decel_start = self.decel_start_step.load(Ordering::Relaxed);

        let new_phase = if step < accel.min(decel_start) {
            MotionPhase::Accelerating
        } else if step >= decel_start {
            MotionPhase::Decelerating
        } else {
            MotionPhase::Cruising
        };
        self.motion_phase.store(new_phase as u32, Ordering::Relaxed);

        // 计算下一步间隔
        let interval = self.compute_interval(step, total, new_phase);
        self.step_interval_us.store(interval, Ordering::Relaxed);

        let now = timer.get_time_us_32();
        timer.set_alarm_safe(alarm, now.wrapping_add(interval));
    }

    // ====================================================================
    // 内部: 梯形间隔计算
    // ====================================================================

    fn compute_interval(&self, step: u32, total_steps: u32, phase: MotionPhase) -> u32 {
        let start_speed = self.start_speed_hz.load(Ordering::Relaxed);
        let max_speed = self.max_speed_hz.load(Ordering::Relaxed);
        let accel = self.accel_steps.load(Ordering::Relaxed);

        let start_interval = speed_to_interval_us(start_speed);
        let max_interval = speed_to_interval_us(max_speed);

        match phase {
            MotionPhase::Accelerating => {
                if accel == 0 {
                    return max_interval;
                }
                let progress = step.min(accel);
                start_interval
                    - ((start_interval - max_interval) as u64 * progress as u64 / accel as u64)
                        as u32
            }
            MotionPhase::Cruising | MotionPhase::Running => max_interval,
            MotionPhase::Decelerating => {
                let decel_start = self.decel_start_step.load(Ordering::Relaxed);
                let steps_into_decel = step.saturating_sub(decel_start);
                let decel_steps = total_steps.saturating_sub(decel_start);
                if decel_steps == 0 {
                    return start_interval;
                }
                let progress = steps_into_decel.min(decel_steps);
                max_interval
                    + ((start_interval - max_interval) as u64 * progress as u64
                        / decel_steps as u64) as u32
            }
            MotionPhase::Idle => start_interval,
        }
    }
}
