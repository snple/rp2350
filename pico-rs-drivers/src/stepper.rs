//! 步进电机控制模块。
//!
//! 提供基于 RP2350 Timer alarm 的步进电机脉冲生成和梯形加减速控制。
//!
//! ## 核心思路
//!
//! - 每个轴对应一个独立的 [`AxisState`]
//! - 每个轴占用一个 Timer alarm
//! - 中断里做 STEP 上升沿 / 下降沿两段式脉冲输出
//! - 通过梯形速度曲线实现平滑启停
//!
//! ## 使用方式
//!
//! ```ignore
//! use pico_rs_drivers::stepper::{AxisConfig, AxisState, Direction};
//!
//! static AXIS_X: AxisState = AxisState::new(AxisConfig {
//!     pin_step: 2,
//!     pin_dir: 3,
//!     alarm: 0,
//!     start_speed_hz: 500,
//!     accel_steps: 200,
//!     step_pulse_us: 3,
//!     limit_pin: None,
//!     limit_active_high: true,
//!     index_pin: None,
//!     homing_backoff_index: 0,
//! });
//! ```

use core::sync::atomic::{AtomicBool, AtomicI32, AtomicU32, Ordering};

use pico_rs::Steal;
use pico_rs::gpio::Gpio;
use pico_rs::timer::{AlarmTimer, MonotonicTimer};

#[inline(always)]
fn gpio() -> Steal<Gpio> {
    // SAFETY: 对应 GPIO 均在初始化阶段设置完成，后续仅通过 SIO 访问数据寄存器。
    unsafe { Gpio::steal() }
}

/// 步进电机运动方向。
///
/// 协议中以 `FWD`/`REV` 字符串传输。
/// `Forward` 对应坐标轴正方向（位置 +1），`Reverse` 对应负方向（位置 -1）。
#[derive(Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Direction {
    /// 坐标轴正方向，每步位置 +1。
    Forward = 0,
    /// 坐标轴负方向，每步位置 -1。
    Reverse = 1,
}

impl Direction {
    /// 返回对应的协议字符串（`"FWD"` 或 `"REV"`）。
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Forward => "FWD",
            Self::Reverse => "REV",
        }
    }
}

/// 轴的当前运动阶段，描述梯形速度曲线的哪一段（或停止状态）。
///
/// 梯形曲线结构：
/// ```text
/// 速度
///  ^      _____________
///  |     /             \
///  |    /               \
///  |___/                 \___
///  +--+--------+---------+---> 步数
///   Accel   Cruise     Decel
/// ```
/// [`Running`](MotionPhase::Running) 是没有步数上限的连续运动模式（`AXIS_RUN` 命令）。
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum MotionPhase {
    /// 静止，无运动任务。
    Idle = 0,
    /// 加速段：速度从 起始速度 平滑升至 目标速度。
    Accelerating = 1,
    /// 巡航段：以恒定目标速度运动。
    Cruising = 2,
    /// 减速段：速度从目标速度平滑降回 起始速度。
    Decelerating = 3,
    /// 连续运动模式（`AXIS_RUN`）：无步数限制，不经过加减速。
    Running = 4,
    /// 归位模式：向限位开关方向连续运动，每步检测限位开关。
    Homing = 5,
    /// 归位回退模式：触发限位后反向运动，利用 INDEX 信号对齐到电气周期边界。
    HomingBackoff = 6,
}

impl MotionPhase {
    /// 将原子存储器中的 u32 数字还原为枚举值。
    /// 未识别值均映射到 [`MotionPhase::Idle`]。
    #[inline]
    fn from_u32(value: u32) -> Self {
        match value {
            1 => Self::Accelerating,
            2 => Self::Cruising,
            3 => Self::Decelerating,
            4 => Self::Running,
            5 => Self::Homing,
            6 => Self::HomingBackoff,
            _ => Self::Idle,
        }
    }
}

/// 轴的静态配置参数，在初始化阶段一次性写入 [`AxisState`]。
pub struct AxisConfig {
    /// STEP 信号引脚编号。
    pub pin_step: u8,
    /// DIR 方向控制引脚编号。
    pub pin_dir: u8,
    /// Timer0 alarm 编号（对应 0/1/2 分别绑定 X/Y/Z 轴）。
    pub alarm: u8,
    /// 步进电机从静止起步的初始频率，单位 Hz（steps/s）。
    pub start_speed_hz: u32,
    /// 加减速段占用的步数，同时也是减速段的步数。
    pub accel_steps: u32,
    /// STEP 脉冲高电平宽度，单位 微秒（us）。A4988/TMC 系列均可接受 2us 以上。
    pub step_pulse_us: u32,
    /// 限位开关 / DIAG 引脚编号。`None` 表示未配置。
    pub limit_pin: Option<u8>,
    /// 限位信号极性：`true` = 高电平有效（物理限位开关和 TMC2209 DIAG 引脚均统一为高电平有效）。
    pub limit_active_high: bool,
    /// INDEX 引脚编号（TMC2209 INDEX 信号，每完整电气周期脉冲一次）。`None` 表示未配置。
    pub index_pin: Option<u8>,
    /// 归位回退时需要检测到的 INDEX 脉冲数（0 = 不回退）。
    pub homing_backoff_index: u32,
}

/// 将步进频率（Hz）转换为每步间隔时间（微秒）。
///
/// 公式：`interval_us = 1_000_000 / hz`。
/// 传入 0 Hz 时返回 1 秒（1_000_000 us）作为默认上限保护。
#[inline]
pub const fn speed_to_interval_us(hz: u32) -> u32 {
    // if hz == 0 { 1_000_000 } else { 1_000_000 / hz }
    if let Some(interval) = 1_000_000u32.checked_div(hz) {
        interval
    } else {
        1_000_000
    }
}

/// 单轴步进状态。
///
/// 静态配置字段（`pin_step`、`alarm` 等）在 [`configure`](AxisState::configure) 后不再变化，
/// 使用普通类型保存；动态运动状态字段使用原子类型以支持 ISR 和主循环跨核心安全读写。
///
/// 设计原则：
/// - 配置字段只写一次，之后只读——无需原子操作。
/// - 运动状态字段仅在中断中写，主循环只读。
/// - `position` 和 `motion_done` 对主循环可读，不需要屏障。
pub struct AxisState {
    // --- 静态配置（initialize once，之后只读）---
    /// STEP 脉冲输出引脚编号。
    pin_step: u8,
    /// DIR 方向控制引脚编号。
    pin_dir: u8,
    /// 所占用的 Timer0 alarm 编号（0/1/2）。
    alarm: u8,
    /// 起步频率，单位 Hz。
    start_speed_hz: u32,
    /// 加减速段步数。
    accel_steps: u32,
    /// STEP 脉冲高电平宽度，单位 us。
    step_pulse_us: u32,
    /// 限位开关 / DIAG 引脚编号。
    limit_pin: Option<u8>,
    /// 限位信号极性：统一使用高电平有效（true）。
    limit_active_high: bool,
    /// INDEX 引脚编号。
    index_pin: Option<u8>,
    /// 归位回退时需要检测到的 INDEX 脉冲数（0 = 不回退）。
    homing_backoff_index: u32,

    // --- 动态运动状态（中断内更新，主循环只读）---
    /// 当前运动阶段（存储 [`MotionPhase`] 的离散化整数値）。
    motion_phase: AtomicU32,
    /// 当前目标巡航速度，单位 Hz。
    max_speed_hz: AtomicU32,
    /// 当前每步间隔，单位 us。加减速期间每步更新。
    step_interval_us: AtomicU32,
    /// 本次运动已完成的步数。
    steps_done: AtomicU32,
    /// 本次运动的目标步数（连续模式下为 0）。
    steps_target: AtomicU32,
    /// 开始减速的步号阈値（`steps_target - accel_steps`）。
    decel_start_step: AtomicU32,
    /// 当前运动方向（存储 [`Direction`] 的离散化整数値）。
    direction: AtomicU32,
    /// 当前逻辑坐标，单位步（正向运动 +1，反向运动 -1）。
    position: AtomicI32,
    /// 运动已完成标志（启动时置 false，最后一步完成后置 true）。
    motion_done: AtomicBool,
    /// STEP 引脚的当前电平（false = 低电平 / true = 高电平）。中断内切换。
    pulse_state: AtomicBool,
    /// INDEX 脉冲计数器（每个完整电气周期 +1）。
    index_count: AtomicU32,
    /// INDEX 引脚上一次采样状态。
    index_last_state: AtomicBool,
    /// 归位回退目标 INDEX 计数。
    homing_backoff_target: AtomicU32,
}

unsafe impl Sync for AxisState {}

impl AxisState {
    /// 根据静态配置创建轴状态对象。
    ///
    /// 所有配置字段在构造时写入，后续不再变化。`const fn` 便于直接在 `static` 中初始化。
    pub const fn new(config: AxisConfig) -> Self {
        Self {
            pin_step: config.pin_step,
            pin_dir: config.pin_dir,
            alarm: config.alarm,
            start_speed_hz: config.start_speed_hz,
            accel_steps: config.accel_steps,
            step_pulse_us: config.step_pulse_us,
            limit_pin: config.limit_pin,
            limit_active_high: config.limit_active_high,
            index_pin: config.index_pin,
            homing_backoff_index: config.homing_backoff_index,

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
            index_count: AtomicU32::new(0),
            index_last_state: AtomicBool::new(false),
            homing_backoff_target: AtomicU32::new(0),
        }
    }

    /// 返回当前运动阶段。可在主循环中读取以显示轴状态。
    #[inline]
    pub fn phase(&self) -> MotionPhase {
        MotionPhase::from_u32(self.motion_phase.load(Ordering::Relaxed))
    }

    /// 返回当前逻辑坐标（步）。中断内每完成一步自动累加。
    #[inline]
    pub fn position(&self) -> i32 {
        self.position.load(Ordering::Relaxed)
    }

    /// 返回本次运动已完成的步数。
    #[inline]
    pub fn steps_done(&self) -> u32 {
        self.steps_done.load(Ordering::Relaxed)
    }

    /// 返回轴是否处于可新廻状态（Idle）。
    /// `true` 表示无运动任务或任务已完成。
    #[inline]
    pub fn is_done(&self) -> bool {
        self.motion_done.load(Ordering::Relaxed)
    }

    /// 将逻辑坐标强制写入指定値，不产生任何实际运动。
    #[inline]
    pub fn set_position(&self, pos: i32) {
        self.position.store(pos, Ordering::Relaxed);
    }

    /// 阻塞等待运动完成（使用 WFI 省电）。
    pub fn wait_done(&self) {
        while !self.motion_done.load(Ordering::Relaxed) {
            pico_rs_rt::asm::wfi();
        }
    }

    /// 标记轴为即将开始运动状态。
    ///
    /// 在双核架构中，Core 0 在通过 FIFO 发送运动命令前调用此方法，
    /// 确保在 Core 1 实际处理命令前 `is_done()` 返回 `false`。
    #[inline]
    pub fn prepare_start(&self) {
        self.motion_done.store(false, Ordering::Relaxed);
    }

    /// 返回是否配置了限位开关。
    #[inline]
    pub fn has_limit_switch(&self) -> bool {
        self.limit_pin.is_some()
    }

    /// 返回限位 / DIAG 信号是否已触发。
    ///
    /// 根据 `limit_active_high` 设置匹配极性（物理限位开关和 TMC2209 DIAG
    /// 均统一为高电平有效）。
    #[inline]
    pub fn limit_triggered(&self) -> bool {
        match self.limit_pin {
            Some(pin) => gpio().get(pin) == self.limit_active_high,
            None => false,
        }
    }

    /// 返回当前 INDEX 脉冲计数。用于步进精度校验。
    #[inline]
    pub fn index_count(&self) -> u32 {
        self.index_count.load(Ordering::Relaxed)
    }

    /// 重置 INDEX 计数器并快照当前 INDEX 引脚电平。
    fn reset_index(&self) {
        self.index_count.store(0, Ordering::Relaxed);
        if let Some(pin) = self.index_pin {
            let state = gpio().get(pin);
            self.index_last_state.store(state, Ordering::Relaxed);
        }
    }

    /// 启动归位运动：向反方向连续运动直到触碰限位开关。
    ///
    /// - 如果未配置限位开关，直接返回 Idle。
    /// - 如果限位开关已触发且无 INDEX 回退，直接置零并返回 Idle。
    /// - 如果限位开关已触发且有 INDEX 回退，进入 [`HomingBackoff`](MotionPhase::HomingBackoff)。
    /// - 否则从起步速度加速至目标速度，向反方向运动，ISR 每步检测限位开关。
    pub fn start_homing<T: AlarmTimer + MonotonicTimer>(&self, timer: &T, speed_hz: u32) {
        if self.limit_pin.is_none() {
            self.motion_phase
                .store(MotionPhase::Idle as u32, Ordering::Relaxed);
            self.motion_done.store(true, Ordering::Relaxed);
            return;
        }

        // 已在限位位置
        if self.limit_triggered() {
            if self.homing_backoff_index > 0 && self.index_pin.is_some() {
                // 需要 INDEX 回退对齐
                self.start_homing_backoff(timer, speed_hz);
                return;
            }
            self.position.store(0, Ordering::Relaxed);
            self.motion_phase
                .store(MotionPhase::Idle as u32, Ordering::Relaxed);
            self.motion_done.store(true, Ordering::Relaxed);
            return;
        }

        self.motion_done.store(false, Ordering::Relaxed);
        self.steps_done.store(0, Ordering::Relaxed);
        self.steps_target.store(0, Ordering::Relaxed);
        self.max_speed_hz.store(speed_hz, Ordering::Relaxed);
        self.reset_index();
        self.direction
            .store(Direction::Reverse as u32, Ordering::Relaxed);
        self.pulse_state.store(false, Ordering::Relaxed);

        gpio().set(self.pin_dir, Direction::Reverse as u32 != 0);

        self.motion_phase
            .store(MotionPhase::Homing as u32, Ordering::Relaxed);

        // 从起步速度开始，ISR 中逐步加速到目标速度
        let interval = speed_to_interval_us(self.start_speed_hz);
        self.step_interval_us.store(interval, Ordering::Relaxed);

        let now = timer.get_time_us_32();
        timer.set_alarm_safe(self.alarm, now.wrapping_add(interval));
        timer.enable_alarm_interrupt(self.alarm);
    }

    /// 启动归位回退：触发限位后反向低速运动，直到检测到指定数量的 INDEX 脉冲。
    ///
    /// INDEX 信号每完整电气周期脉冲一次，利用该信号对齐到精确的电气周期边界，
    /// 消除限位开关机械抖动带来的定位误差。
    fn start_homing_backoff<T: AlarmTimer + MonotonicTimer>(&self, timer: &T, speed_hz: u32) {
        self.motion_done.store(false, Ordering::Relaxed);
        self.steps_done.store(0, Ordering::Relaxed);
        self.steps_target.store(0, Ordering::Relaxed);
        self.reset_index();
        self.homing_backoff_target
            .store(self.homing_backoff_index, Ordering::Relaxed);
        self.direction
            .store(Direction::Forward as u32, Ordering::Relaxed);
        self.pulse_state.store(false, Ordering::Relaxed);

        gpio().set(self.pin_dir, Direction::Forward as u32 != 0);

        self.motion_phase
            .store(MotionPhase::HomingBackoff as u32, Ordering::Relaxed);

        // 回退速度取归位速度的一半，确保低速精确对齐
        let backoff_speed = (speed_hz / 2).max(1);
        let interval = speed_to_interval_us(backoff_speed);
        self.step_interval_us.store(interval, Ordering::Relaxed);

        let now = timer.get_time_us_32();
        timer.set_alarm_safe(self.alarm, now.wrapping_add(interval));
        timer.enable_alarm_interrupt(self.alarm);
    }

    /// 启动有限步数的相对位移运动，带梯形加减速。
    ///
    /// 该方法必须在轴当前已 Idle（小循环检查 `is_done()`）的情况下调用。
    ///
    /// - `steps == 0`：直接将轴置为 Idle 并返回，不开启定时器。
    /// - 加速步数不超过 `steps / 2`，确保对程运动不会先加后减透支。
    pub fn start_move<T: AlarmTimer + MonotonicTimer>(
        &self,
        timer: &T,
        steps: u32,
        dir: Direction,
        max_speed_hz: u32,
    ) {
        if steps == 0 {
            self.motion_phase
                .store(MotionPhase::Idle as u32, Ordering::Relaxed);
            self.motion_done.store(true, Ordering::Relaxed);
            return;
        }

        self.max_speed_hz.store(max_speed_hz, Ordering::Relaxed);
        self.motion_done.store(false, Ordering::Relaxed);
        self.steps_done.store(0, Ordering::Relaxed);
        self.steps_target.store(steps, Ordering::Relaxed);
        self.reset_index();
        self.direction.store(dir as u32, Ordering::Relaxed);
        self.pulse_state.store(false, Ordering::Relaxed);

        let accel_steps = self.accel_steps.min(steps / 2);
        let decel_start = steps - accel_steps;
        self.decel_start_step.store(decel_start, Ordering::Relaxed);

        gpio().set(self.pin_dir, dir as u32 != 0);

        self.motion_phase
            .store(MotionPhase::Accelerating as u32, Ordering::Relaxed);

        let interval = speed_to_interval_us(self.start_speed_hz);
        self.step_interval_us.store(interval, Ordering::Relaxed);

        let now = timer.get_time_us_32();
        timer.set_alarm_safe(self.alarm, now.wrapping_add(interval));
        timer.enable_alarm_interrupt(self.alarm);
    }

    /// 启动不限步数的连续运动，无加减速，固定速度运转。
    ///
    /// 坐标仍然每步累加。需调用 [`stop()`](Self::stop) 终止运动。
    pub fn start_continuous<T: AlarmTimer + MonotonicTimer>(
        &self,
        timer: &T,
        dir: Direction,
        speed_hz: u32,
    ) {
        self.motion_done.store(false, Ordering::Relaxed);
        self.steps_done.store(0, Ordering::Relaxed);
        self.steps_target.store(0, Ordering::Relaxed);
        self.reset_index();
        self.direction.store(dir as u32, Ordering::Relaxed);
        self.pulse_state.store(false, Ordering::Relaxed);

        gpio().set(self.pin_dir, dir as u32 != 0);

        self.motion_phase
            .store(MotionPhase::Running as u32, Ordering::Relaxed);

        let interval = speed_to_interval_us(speed_hz);
        self.step_interval_us.store(interval, Ordering::Relaxed);

        let now = timer.get_time_us_32();
        timer.set_alarm_safe(self.alarm, now.wrapping_add(interval));
        timer.enable_alarm_interrupt(self.alarm);
    }

    /// 动态更新当前运动的目标步数和减速起始点。
    ///
    /// 仅在电机处于加速或巡航阶段时调用有效。
    /// 内部先更新 `steps_target`（防止 ISR 在旧 target 处提前判定运动完成），
    /// 再更新 `decel_start_step`。
    ///
    /// 用于编码器连续输入场景：无需停止电机，直接延长运动距离。
    pub fn set_target_steps(&self, new_total_steps: u32) {
        // 先写 steps_target，防止 ISR 在旧 target 处提前完成运动
        self.steps_target.store(new_total_steps, Ordering::Relaxed);
        let accel_steps = self.accel_steps.min(new_total_steps / 2);
        self.decel_start_step
            .store(new_total_steps - accel_steps, Ordering::Relaxed);
    }

    /// 立即停止轴运动，将阶段设为 [`MotionPhase::Idle`] 并拉低 STEP 引脚。
    /// 不等待减速完成，适合 ESTOP 场景。ISR 下次触发时检测到 Idle 后直接返回。
    pub fn stop(&self) {
        self.motion_phase
            .store(MotionPhase::Idle as u32, Ordering::Relaxed);
        self.motion_done.store(true, Ordering::Relaxed);
        gpio().set_low(self.pin_step);
    }

    /// Timer alarm 中断服务函数（ISR）。应在对应轴的 `TIMER0_IRQx` 部署中调用。
    ///
    /// 两段式脉冲逻辑：
    ///
    /// 1. **上升沿（`pulse_state == false`）**：拉高 STEP，安排 脉冲宽度 (微秒) 后的下个 alarm。
    /// 2. **下降沿（`pulse_state == true`）**：拉低 STEP，累加步数和坐标，根据当前阶段调整
    ///    速度原并设置下一步的 alarm。若步数达到目标则进入 Idle。
    pub fn handle_alarm_isr<T: AlarmTimer + MonotonicTimer>(&self, timer: &T) {
        timer.clear_alarm_interrupt(self.alarm);
        timer.clear_forced_alarm_interrupt(self.alarm);

        let phase = MotionPhase::from_u32(self.motion_phase.load(Ordering::Relaxed));
        if phase == MotionPhase::Idle {
            return;
        }

        let pulse_high = self.pulse_state.load(Ordering::Relaxed);

        if !pulse_high {
            gpio().set_high(self.pin_step);
            self.pulse_state.store(true, Ordering::Relaxed);

            let now = timer.get_time_us_32();
            timer.set_alarm_safe(self.alarm, now.wrapping_add(self.step_pulse_us));
            return;
        }

        gpio().set_low(self.pin_step);
        self.pulse_state.store(false, Ordering::Relaxed);

        let step = self.steps_done.fetch_add(1, Ordering::Relaxed) + 1;
        let total = self.steps_target.load(Ordering::Relaxed);
        let dir = self.direction.load(Ordering::Relaxed);

        if dir == 0 {
            self.position.fetch_add(1, Ordering::Relaxed);
        } else {
            self.position.fetch_sub(1, Ordering::Relaxed);
        }

        // INDEX 脉冲检测（下降沿计数）
        if let Some(idx_pin) = self.index_pin {
            let state = gpio().get(idx_pin);
            let last = self.index_last_state.swap(state, Ordering::Relaxed);
            if !state && last {
                self.index_count.fetch_add(1, Ordering::Relaxed);
            }
        }

        // 归位模式：每步检测限位开关 / DIAG 信号
        if phase == MotionPhase::Homing {
            if self.limit_triggered() {
                // 限位开关触发，检查是否需要 INDEX 回退对齐
                if self.homing_backoff_index > 0 && self.index_pin.is_some() {
                    // 切换到回退模式：反向运动，利用 INDEX 信号精确对齐
                    self.reset_index();
                    self.homing_backoff_target
                        .store(self.homing_backoff_index, Ordering::Relaxed);
                    self.direction
                        .store(Direction::Forward as u32, Ordering::Relaxed);
                    gpio().set(self.pin_dir, Direction::Forward as u32 != 0);
                    self.motion_phase
                        .store(MotionPhase::HomingBackoff as u32, Ordering::Relaxed);
                    // 回退速度减半以提高精度
                    let interval = self.step_interval_us.load(Ordering::Relaxed) * 2;
                    self.step_interval_us.store(interval, Ordering::Relaxed);
                    let now = timer.get_time_us_32();
                    timer.set_alarm_safe(self.alarm, now.wrapping_add(interval));
                    return;
                }
                self.position.store(0, Ordering::Relaxed);
                self.motion_phase
                    .store(MotionPhase::Idle as u32, Ordering::Relaxed);
                self.motion_done.store(true, Ordering::Relaxed);
                return;
            }
            // 继续归位运动（带加速）
            let interval = self.step_interval_us.load(Ordering::Relaxed);
            let min_interval = speed_to_interval_us(self.max_speed_hz.load(Ordering::Relaxed));
            let accel = self.accel_steps;
            let new_interval = if accel > 0 && interval > min_interval {
                let start_interval = speed_to_interval_us(self.start_speed_hz);
                let progress = step.min(accel);
                start_interval
                    - ((start_interval - min_interval) as u64 * progress as u64 / accel as u64)
                        as u32
            } else {
                min_interval
            };
            self.step_interval_us.store(new_interval, Ordering::Relaxed);
            let now = timer.get_time_us_32();
            timer.set_alarm_safe(self.alarm, now.wrapping_add(new_interval));
            return;
        }

        // 归位回退模式：反向运动直到检测到目标数量的 INDEX 脉冲
        if phase == MotionPhase::HomingBackoff {
            let target = self.homing_backoff_target.load(Ordering::Relaxed);
            if self.index_count.load(Ordering::Relaxed) >= target {
                self.position.store(0, Ordering::Relaxed);
                self.motion_phase
                    .store(MotionPhase::Idle as u32, Ordering::Relaxed);
                self.motion_done.store(true, Ordering::Relaxed);
                return;
            }
            let interval = self.step_interval_us.load(Ordering::Relaxed);
            let now = timer.get_time_us_32();
            timer.set_alarm_safe(self.alarm, now.wrapping_add(interval));
            return;
        }

        if phase == MotionPhase::Running {
            let interval = self.step_interval_us.load(Ordering::Relaxed);
            let now = timer.get_time_us_32();
            timer.set_alarm_safe(self.alarm, now.wrapping_add(interval));
            return;
        }

        if total > 0 && step >= total {
            self.motion_phase
                .store(MotionPhase::Idle as u32, Ordering::Relaxed);
            self.motion_done.store(true, Ordering::Relaxed);
            return;
        }

        let accel = self.accel_steps;
        let decel_start = self.decel_start_step.load(Ordering::Relaxed);

        let new_phase = if step < accel.min(decel_start) {
            MotionPhase::Accelerating
        } else if step >= decel_start {
            MotionPhase::Decelerating
        } else {
            MotionPhase::Cruising
        };
        self.motion_phase.store(new_phase as u32, Ordering::Relaxed);

        let interval = self.compute_interval(step, total, new_phase);
        self.step_interval_us.store(interval, Ordering::Relaxed);

        let now = timer.get_time_us_32();
        timer.set_alarm_safe(self.alarm, now.wrapping_add(interval));
    }

    /// 根据当前步号和运动阶段计算下一步的间隔时间（us）。
    ///
    /// - 加速段：起始间隔 → 巡航间隔 求线性内插。
    /// - 巡航 / Running：返回固定巡航间隔。
    /// - 减速段：巡航间隔 → 起始间隔 求线性内插（全速停止）。
    ///
    /// 内插使用整数算术（运转到 u64 后再取整）避免浮点。
    fn compute_interval(&self, step: u32, total_steps: u32, phase: MotionPhase) -> u32 {
        let start_speed = self.start_speed_hz;
        let max_speed = self.max_speed_hz.load(Ordering::Relaxed);
        let accel = self.accel_steps;

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
            MotionPhase::Cruising
            | MotionPhase::Running
            | MotionPhase::Homing
            | MotionPhase::HomingBackoff => max_interval,
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
