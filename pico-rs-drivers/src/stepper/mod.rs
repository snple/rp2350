//! 步进电机控制模块。
//!
//! - [`tmc2209`] — TMC2209 专用扩展（DIAG 堵转检测 + INDEX 电气周期对齐）
//!
//! ## 命名约定
//!
//! 两个子模块提供**同名**的 [`AxisConfig`] 和 [`AxisState`] 类型。
//! 通过选择不同的导入路径即可在通用版和 TMC2209 版之间切换：
//!
//! ```ignore
//! // 通用版（A4988 等）
//! use pico_rs_drivers::stepper::{AxisConfig, AxisState, Direction};
//!
//! // TMC2209 版（带 DIAG/INDEX/归位）
//! use pico_rs_drivers::stepper::tmc2209::{AxisConfig, AxisState};
//! use pico_rs_drivers::stepper::Direction;
//! ```

use core::marker::PhantomData;
use core::sync::atomic::{AtomicBool, AtomicI32, AtomicU8, AtomicU32, Ordering};

use pico_rs::timer::{AlarmTimer, MonotonicTimer};

// ============================================================================
// AxisPins trait
// ============================================================================

/// 步进电机 STEP/DIR 引脚操作 trait。
///
/// 实现为零大小类型 (ZST)，编译期单态化，ISR 中零开销内联。
/// 双电机同步：在 `set_step`/`set_dir` 中操作多个 GPIO 即可。
pub trait AxisPins {
    /// 设置 STEP 引脚电平：`true` = 高，`false` = 低。
    fn set_step(high: bool);
    /// 设置 DIR 引脚电平：`true` = 高，`false` = 低。
    fn set_dir(high: bool);
}

// ============================================================================
// Direction
// ============================================================================

/// 步进电机运动方向。
///
/// 协议中以 `FWD`/`REV` 字符串传输。
/// `Forward` 对应坐标轴正方向（位置 +1），`Reverse` 对应负方向（位置 -1）。
#[derive(Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
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

    pub fn from_u8(value: u8) -> Self {
        match value {
            1 => Self::Reverse,
            _ => Self::Forward,
        }
    }

    pub fn backward(self) -> Self {
        match self {
            Self::Forward => Self::Reverse,
            Self::Reverse => Self::Forward,
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
/// [`Running`](MotionPhase::Running) 是没有步数上限的连续运动模式。
///
/// [`Probing`](MotionPhase::Probing) 和 [`ProbingBackoff`](MotionPhase::ProbingBackoff)
/// 仅在 [`tmc2209::AxisState`] 中使用，依赖 DIAG/INDEX 引脚。
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum MotionPhase {
    /// 静止，无运动任务。
    Idle = 0,
    /// 加速段：速度从起始速度平滑升至目标速度。
    Accelerating = 1,
    /// 巡航段：以恒定目标速度运动。
    Cruising = 2,
    /// 减速段：速度从目标速度平滑降回起始速度。
    Decelerating = 3,
    /// 连续运动模式：无步数限制，不经过加减速。
    Running = 4,
    /// 探测模式：向限位开关方向连续运动，每步检测限位开关。
    Probing = 5,
    /// 探测回退模式：触发限位后反向运动，利用 INDEX 信号对齐到电气周期边界。
    ProbingBackoff = 6,
}

impl MotionPhase {
    /// 将原子存储器中的 u8 数字还原为枚举值。
    /// 未识别值均映射到 [`MotionPhase::Idle`]。
    #[inline]
    pub fn from_u8(value: u8) -> Self {
        match value {
            1 => Self::Accelerating,
            2 => Self::Cruising,
            3 => Self::Decelerating,
            4 => Self::Running,
            5 => Self::Probing,
            6 => Self::ProbingBackoff,
            _ => Self::Idle,
        }
    }
}

/// Running 模式下 `set_speed` 平滑过渡时，每步间隔的最大变化量（微秒）。
/// 值越大过渡越快（但可能产生速度跳变感），建议 1~5 µs。
const SPEED_RAMP_DELTA_US: u32 = 2;

/// 将步进频率（Hz）转换为每步间隔时间（微秒）。
///
/// ## 公式
///
/// ```text
/// interval_us = 1_000_000 / hz
/// ```
///
/// 推导：1 Hz = 1 step/s，每步间隔 = 1 s = 1,000,000 µs。
/// n Hz 时每步间隔 = 1/n s = 1,000,000/n µs。
///
/// 传入 0 Hz 时返回 1 秒（1_000_000 µs）作为默认上限保护。
#[inline]
pub const fn speed_to_interval_us(hz: u32) -> u32 {
    if let Some(interval) = 1_000_000u32.checked_div(hz) {
        interval
    } else {
        1_000_000
    }
}

// ============================================================================
// AxisConfig — 通用轴配置
// ============================================================================

/// 轴的静态配置参数，在初始化阶段一次性写入 [`AxisState`]。
pub struct AxisConfig {
    /// Timer0 alarm 编号（对应 0/1/2 分别绑定 X/Y/Z 轴）。
    pub alarm: u8,
    /// 步进电机从静止起步的初始频率，单位 Hz（steps/s）。
    pub start_speed_hz: u32,
    /// 加减速段占用的步数，同时也是减速段的步数。
    pub accel_steps: u32,
    /// STEP 脉冲高电平宽度，单位微秒（us）。A4988/TMC 系列均可接受 2us 以上。
    pub step_pulse_us: u32,
    /// Forward 方向对应 DIR 引脚的电平。`true` = 高电平，`false` = 低电平。
    pub fwd_level: bool,
}

// ============================================================================
// AxisState — 通用单轴步进状态
// ============================================================================

/// 单轴步进状态（通用版）。
///
/// 泛型参数 `P` 实现 [`AxisPins`]，提供 STEP/DIR 的 GPIO 操作。
/// 编译期单态化，ISR 中零开销内联。
///
/// 如需 DIAG/INDEX/归位功能，请使用 [`tmc2209::AxisState`]。
pub struct AxisState<P: AxisPins> {
    _pins: PhantomData<P>,
    // ========================================================================
    // 静态配置（initialize once，之后只读）
    // ========================================================================
    /// 所占用的 Timer0 alarm 编号（0/1/2）。
    pub(crate) alarm: u8,
    /// 起步频率，单位 Hz。
    pub(crate) start_speed_hz: u32,
    /// 加减速段步数。
    pub(crate) accel_steps: u32,
    /// STEP 脉冲高电平宽度，单位 us。
    step_pulse_us: u32,
    /// Forward 方向对应 DIR 引脚的电平。`true` = 高电平，`false` = 低电平。
    fwd_level: bool,

    // ========================================================================
    // 动态运动状态（中断内更新，主循环只读）
    // ========================================================================
    /// 当前运动方向（存储 [`Direction`] 的离散化整数值）。
    pub(crate) direction: AtomicU8,
    /// 当前运动阶段（存储 [`MotionPhase`] 的离散化整数值）。
    pub(crate) motion_phase: AtomicU8,
    /// 当前目标巡航速度，单位 Hz。
    pub(crate) max_speed_hz: AtomicU32,
    /// 当前每步间隔，单位 us。加减速期间每步更新。
    pub(crate) step_interval_us: AtomicU32,
    /// 本次运动已完成的步数。
    pub(crate) steps_done: AtomicU32,
    /// 本次运动的目标步数（连续模式下为 0）。
    pub(crate) steps_target: AtomicU32,
    /// 本次运动实际使用的加速步数（= `min(accel_steps, steps/2)`），在 `start_move` 时写入。
    pub(crate) effective_accel_steps: AtomicU32,
    /// 开始减速的步号阈值（`steps_target - effective_accel_steps + 1`）。
    pub(crate) decel_start_step: AtomicU32,
    /// 当前逻辑坐标，单位步（正向运动 +1，反向运动 -1）。
    pub(crate) position: AtomicI32,
    /// 最大行程（微步）。0 = 无限位；>0 时位置约束在 [0, max_travel]。
    pub(crate) max_travel: AtomicI32,
    /// 运动已完成标志（启动时置 false，最后一步完成后置 true）。
    pub(crate) motion_done: AtomicBool,
    /// STEP 引脚的当前电平（false = 低电平 / true = 高电平）。中断内切换。
    pub(crate) pulse_state: AtomicBool,
    /// Running 模式下平滑调速的目标间隔（us）。与 `step_interval_us` 不同时触发过渡。
    target_interval_us: AtomicU32,
    /// Running 模式下达到目标速度后的稳定步数计数（用于 DIAG 重启用判断）。
    pub(crate) speed_settle_count: AtomicU32,
    /// 减速起始时的实际 step interval (us)。
    ///
    /// `0` 表示从 [`max_speed_hz`] 对应的 `max_interval` 开始减速（正常梯形减速）；
    /// 非零表示从该值开始减速，由 [`smooth_stop`] 在中途减速时写入，
    /// 避免电机尚未达到最高速度时产生速度跳变。
    pub(crate) decel_start_interval: AtomicU32,
}

unsafe impl<P: AxisPins> Sync for AxisState<P> {}

impl<P: AxisPins> AxisState<P> {
    /// 根据静态配置创建轴状态对象。
    ///
    /// 所有配置字段在构造时写入，后续不再变化。`const fn` 便于直接在 `static` 中初始化。
    pub const fn new(config: AxisConfig) -> Self {
        Self {
            _pins: PhantomData,
            alarm: config.alarm,
            start_speed_hz: config.start_speed_hz,
            accel_steps: config.accel_steps,
            step_pulse_us: config.step_pulse_us,
            fwd_level: config.fwd_level,

            direction: AtomicU8::new(0),
            motion_phase: AtomicU8::new(0),
            max_speed_hz: AtomicU32::new(1000),
            step_interval_us: AtomicU32::new(0),
            steps_done: AtomicU32::new(0),
            steps_target: AtomicU32::new(0),
            effective_accel_steps: AtomicU32::new(0),
            decel_start_step: AtomicU32::new(0),
            position: AtomicI32::new(0),
            max_travel: AtomicI32::new(0),
            motion_done: AtomicBool::new(true),
            pulse_state: AtomicBool::new(false),
            target_interval_us: AtomicU32::new(0),
            speed_settle_count: AtomicU32::new(0),
            decel_start_interval: AtomicU32::new(0),
        }
    }

    // ========================================================================
    // 状态查询
    // ========================================================================

    /// 返回当前运动阶段。可在主循环中读取以显示轴状态。
    #[inline]
    pub fn phase(&self) -> MotionPhase {
        MotionPhase::from_u8(self.motion_phase.load(Ordering::Relaxed))
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

    /// 返回本次运动的目标步数（`start_continuous` 连续模式为 0）。
    #[inline]
    pub fn steps_target(&self) -> u32 {
        self.steps_target.load(Ordering::Relaxed)
    }

    /// 返回轴是否处于可新建状态（Idle）。
    /// `true` 表示无运动任务或任务已完成。
    #[inline]
    pub fn is_done(&self) -> bool {
        self.motion_done.load(Ordering::Relaxed)
    }

    /// 轴是否正在运动中（phase 不为 Idle）。
    ///
    /// 与 [`is_done`] 不同：`prepare_start` 会将 `motion_done` 置 false
    /// 但 phase 保持 Idle，因此用 phase 判断更准确。
    #[inline]
    pub fn is_busy(&self) -> bool {
        self.motion_phase.load(Ordering::Relaxed) != MotionPhase::Idle as u8
    }

    // ========================================================================
    // 位置管理
    // ========================================================================

    /// 将逻辑坐标强制写入指定值，不产生任何实际运动。
    #[inline]
    pub fn set_position(&self, pos: i32) {
        self.position.store(pos, Ordering::Relaxed);
    }

    /// 阻塞等待运动完成（使用 WFI 省电）。
    #[inline]
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

    // ========================================================================
    // 行程约束
    // ========================================================================

    /// 设置最大行程（微步）。
    ///
    /// 设置后所有运动被约束在 `[0, max_travel]` 范围内：
    /// `start_move` 超出自动截断，`start_continuous` 自动转为有限运动。
    /// 设 0 取消限位。
    #[inline]
    pub fn set_max_travel(&self, travel: i32) {
        self.max_travel.store(travel, Ordering::Relaxed);
    }

    // ========================================================================
    // GPIO 操作
    // ========================================================================

    #[inline(always)]
    fn apply_dir(&self, dir: Direction) {
        let level = match dir {
            Direction::Forward => self.fwd_level,
            Direction::Reverse => !self.fwd_level,
        };
        P::set_dir(level);
    }

    #[inline(always)]
    fn apply_step(&self, high: bool) {
        P::set_step(high);
    }

    // ========================================================================
    // 行程辅助
    // ========================================================================

    /// 受最大行程约束后的有效步数。
    #[inline]
    pub(crate) fn clamp_steps(&self, steps: u32, dir: Direction) -> u32 {
        match self.continuous_max_steps(dir) {
            Some(max) => steps.min(max),
            None => steps,
        }
    }

    /// 受最大行程约束后，连续运动可用的最大步数。
    ///
    /// 返回 `None` 表示无限位；`Some(0)` 表示已在限位、无法运动；
    /// `Some(n)` 表示可运动 n 步。
    #[inline]
    pub(crate) fn continuous_max_steps(&self, dir: Direction) -> Option<u32> {
        let limit = self.max_travel.load(Ordering::Relaxed);
        if limit <= 0 {
            return None;
        }
        let cur = self.position();
        let steps = match dir {
            Direction::Forward => limit.saturating_sub(cur).max(0) as u32,
            Direction::Reverse => cur.max(0) as u32,
        };
        Some(steps)
    }

    // ========================================================================
    // 运动控制（内部）
    // ========================================================================

    /// 设置梯形加减速的加速步数和减速起始点。
    ///
    /// 加速步数 = `min(accel_steps, total_steps / 2)`，
    /// 减速起始 = `total_steps - accel_steps + 1`。
    ///
    /// ## 为什么 `+1`？
    ///
    /// 加速段通过 `step < accel_steps` 判断完成步数 1..(accel-1) 处于加速段，
    /// 共 `accel-1` 次内插计算。加上 `begin_motion` 中预设的初始间隔，
    /// 合计 `accel` 次速度变化的过渡。
    ///
    /// 减速段通过 `step >= decel_start` 判断。为与加速段对称，
    /// `decel_start = total - accel + 1` 使减速段覆盖 `total - decel_start`
    /// `= accel - 1` 次内插计算，与加速段的内插次数一致。
    ///
    /// 验证示例 (total=1000, accel=200)：
    /// - 加速：step 1..199 加速 → 199 次内插 + 1 次预设 = 200 过渡
    /// - 减速：decel_start=801, step 801..999 减速 → 199 次内插 = 199 过渡
    ///
    /// 同时清零 `decel_start_interval`，标记为正常梯形减速
    ///（从 `max_speed_hz` 对应的 `max_interval` 起始）。
    #[inline]
    fn setup_trapezoid(&self, total_steps: u32) {
        let accel_steps = self.accel_steps.min(total_steps / 2);
        self.effective_accel_steps
            .store(accel_steps, Ordering::Relaxed);
        self.decel_start_step.store(
            total_steps.saturating_sub(accel_steps).saturating_add(1),
            Ordering::Relaxed,
        );
        self.decel_start_interval.store(0, Ordering::Relaxed);
    }

    /// 将轴置为 Idle（仅更新阶段和完成标志，不操作 GPIO）。
    #[inline]
    fn set_idle(&self) {
        self.motion_phase
            .store(MotionPhase::Idle as u8, Ordering::Relaxed);
        self.motion_done.store(true, Ordering::Relaxed);
    }

    /// 按给定间隔调度下一次定时器报警。
    #[inline(always)]
    fn schedule_next_alarm<T: AlarmTimer + MonotonicTimer>(&self, timer: &T, interval_us: u32) {
        let now = timer.get_time_us_32();
        timer.set_alarm_safe(self.alarm, now.wrapping_add(interval_us));
    }

    /// 启动运动：设置方向、相位、间隔并启用定时器中断。
    ///
    /// 将方向、脉冲状态、GPIO、运动阶段、步间隔一次性写入，
    /// 然后设置首步 alarm 并启用中断。`start_move`、`start_continuous`、
    /// 以及 `tmc2209` 的 `probe_endstop` / `do_backoff` 均通过本方法收尾。
    #[inline]
    fn begin_motion<T: AlarmTimer + MonotonicTimer>(
        &self,
        timer: &T,
        dir: Direction,
        phase: MotionPhase,
        start_interval_us: u32,
    ) {
        self.direction.store(dir as u8, Ordering::Relaxed);
        self.pulse_state.store(false, Ordering::Relaxed);
        self.apply_dir(dir);
        self.motion_phase.store(phase as u8, Ordering::Relaxed);
        self.step_interval_us
            .store(start_interval_us, Ordering::Relaxed);
        self.target_interval_us
            .store(start_interval_us, Ordering::Relaxed);
        self.speed_settle_count.store(0, Ordering::Relaxed);
        self.schedule_next_alarm(timer, start_interval_us);
        timer.enable_alarm_interrupt(self.alarm);
    }

    // ========================================================================
    // 运动控制（公开 API）
    // ========================================================================

    /// 启动有限步数的相对位移运动，带梯形加减速。
    ///
    /// 轴忙时（运动中或探测中）直接返回，不覆盖当前运动状态。
    /// 目标位置超出 [`max_travel`] 时步数自动截断。
    ///
    /// - `steps == 0`：直接将轴置为 Idle 并返回，不开启定时器。
    /// - 加速步数不超过 `steps / 2`，确保对称运动不会先加后减透支。
    pub fn start_move<T: AlarmTimer + MonotonicTimer>(
        &self,
        timer: &T,
        steps: u32,
        dir: Direction,
        max_speed_hz: u32,
    ) {
        if self.is_busy() {
            return;
        }
        let steps = self.clamp_steps(steps, dir);
        if steps == 0 {
            self.set_idle();
            return;
        }

        self.max_speed_hz.store(max_speed_hz, Ordering::Relaxed);
        self.motion_done.store(false, Ordering::Relaxed);
        self.steps_done.store(0, Ordering::Relaxed);
        self.steps_target.store(steps, Ordering::Relaxed);
        self.setup_trapezoid(steps);

        self.begin_motion(
            timer,
            dir,
            MotionPhase::Accelerating,
            speed_to_interval_us(self.start_speed_hz),
        );
    }

    /// 启动不限步数的连续运动，无加减速，固定速度运转。
    ///
    /// 轴忙时直接返回。如果设置了 [`max_travel`]，自动转为有限步数的 `start_move`。
    pub fn start_continuous<T: AlarmTimer + MonotonicTimer>(
        &self,
        timer: &T,
        dir: Direction,
        speed_hz: u32,
    ) {
        if self.is_busy() {
            return;
        }
        if let Some(max_steps) = self.continuous_max_steps(dir) {
            if max_steps == 0 {
                self.motion_done.store(true, Ordering::Relaxed);
                return;
            }
            self.start_move(timer, max_steps, dir, speed_hz);
            return;
        }
        // 无限位
        self.motion_done.store(false, Ordering::Relaxed);
        self.steps_done.store(0, Ordering::Relaxed);
        self.steps_target.store(0, Ordering::Relaxed);
        self.begin_motion(
            timer,
            dir,
            MotionPhase::Running,
            speed_to_interval_us(speed_hz),
        );
    }

    /// 在当前目标步数基础上增加额外步数，推迟减速起始点。
    ///
    /// 用于运动中实时延长运动距离的场景（如摇杆连续输入），
    /// 无需停止电机即可直接延长。
    ///
    /// 如果在减速阶段调用，且新的减速起始点大于当前步数，
    /// ISR 会自动将阶段切回巡航阶段，继续匀速运动。
    ///
    /// 注意：`Running` 模式（`start_continuous`）下此方法无效，
    /// 因为 Running 模式无步数限制、不经梯形调度。请改用
    /// [`smooth_stop`] 停止后再重新 [`start_continuous`] 换向。
    #[inline]
    pub fn extend_target(&self, extra_steps: u32) {
        let current = self.steps_target.load(Ordering::Relaxed);
        let new_total = current.saturating_add(extra_steps);
        // 先写 steps_target，防止 ISR 在旧 target 处提前完成运动
        self.steps_target.store(new_total, Ordering::Relaxed);
        self.setup_trapezoid(new_total);
    }

    /// 实时调整 Running 模式下的运动速度，带平滑过渡。
    ///
    /// 仅在 [`start_continuous`] 启动的连续运动中生效。
    /// 内部更新 `target_interval_us`，ISR 每步以固定斜率向目标逼近，
    /// 避免速度跳变。同时更新 `max_speed_hz`，梯形模式调用时也可影响
    /// 加减速计算。
    ///
    /// TMC2209 版会在调速期间自动屏蔽 DIAG 堵转检测，
    /// 速度稳定 `blanking_steps` 步后自动恢复。
    #[inline]
    pub fn set_speed(&self, speed_hz: u32) {
        let interval = speed_to_interval_us(speed_hz);
        self.target_interval_us.store(interval, Ordering::Relaxed);
        self.max_speed_hz.store(speed_hz, Ordering::Relaxed);
    }

    /// 立即停止轴运动，将阶段设为 [`MotionPhase::Idle`] 并拉低 STEP 引脚。
    /// 不等待减速完成，适合 ESTOP 场景。ISR 下次触发时检测到 Idle 后直接返回。
    #[inline]
    pub fn stop(&self) {
        self.set_idle();
        self.apply_step(false);
    }

    /// 平滑停止：立即进入减速阶段，按梯形曲线降速至停止。
    ///
    /// 不同于 [`stop`] 的急停——`stop` 瞬间将阶段设为 Idle，
    /// 电机因惯性滑行停止；`smooth_stop` 让电机按当前的
    /// 加减速参数从当前位置平稳减速到停止，减少机械冲击。
    ///
    /// 适用于 `start_move` 的梯形模式和 `start_continuous` 的连续模式。
    /// 探测模式（Probing/ProbingBackoff）下调用无效，请使用 [`stop`]。
    ///
    /// ## 实现说明
    ///
    /// 将目标步数设为 `当前步数 + accel_steps`，减速起始点设为
    /// `当前步数 + 1`，强制阶段为 Decelerating。
    ///
    /// 关键：保存当前的 `step_interval_us` 到 `decel_start_interval`，
    /// 减速内插从实际当前速度（而非 `max_interval`）起始，
    /// 避免加速中途调用时产生短暂加速尖峰。
    ///
    /// ISR 下次触发时会按 `accel_steps` 步完成从当前速度到起始速度的平滑降速。
    #[inline]
    pub fn smooth_stop(&self) {
        let phase_raw = self.motion_phase.load(Ordering::Relaxed);
        if phase_raw == MotionPhase::Idle as u8
            || phase_raw == MotionPhase::Probing as u8
            || phase_raw == MotionPhase::ProbingBackoff as u8
        {
            return;
        }

        let cur_step = self.steps_done.load(Ordering::Relaxed);
        let cur_interval = self.step_interval_us.load(Ordering::Relaxed);
        let decel_dist = self.accel_steps.max(1);
        let new_target = cur_step + decel_dist;

        // 原子写入顺序：先写 steps_target 防止 ISR 在旧 target 处提前 done
        self.steps_target.store(new_target, Ordering::Relaxed);
        self.effective_accel_steps.store(0, Ordering::Relaxed);
        self.decel_start_step.store(cur_step + 1, Ordering::Relaxed);
        // 保存当前实际速度作为减速起点，避免加速中途调用时跳到 max_interval
        self.decel_start_interval
            .store(cur_interval, Ordering::Relaxed);
        self.motion_phase
            .store(MotionPhase::Decelerating as u8, Ordering::Relaxed);
    }

    // ========================================================================
    // ISR 基础构件
    // ========================================================================

    /// STEP 引脚脉冲翻转。返回 `true` 表示下降沿（步完成），`false` 表示上升沿。
    ///
    /// 上升沿时自动设置脉冲宽度的下一次 alarm。
    #[inline(always)]
    pub(crate) fn toggle_pulse<T: AlarmTimer + MonotonicTimer>(&self, timer: &T) -> bool {
        let pulse_high = self.pulse_state.load(Ordering::Relaxed);
        if !pulse_high {
            // 上升沿：拉高 STEP，安排脉冲宽度后的下一次 alarm
            self.apply_step(true);
            self.pulse_state.store(true, Ordering::Relaxed);
            let now = timer.get_time_us_32();
            timer.set_alarm_safe(self.alarm, now.wrapping_add(self.step_pulse_us));
            return false;
        }
        // 下降沿：拉低 STEP，一步完成
        self.apply_step(false);
        self.pulse_state.store(false, Ordering::Relaxed);
        true
    }

    /// 步完成时累加步数计数器并更新逻辑坐标。
    ///
    /// 返回 `(current_step, total_steps)`，其中 `current_step` 是当前步号（从 1 开始），
    /// `total_steps` 是本次运动的目标步数（连续模式为 0）。
    #[inline(always)]
    pub(crate) fn advance_step(&self) -> (u32, u32) {
        let step = self.steps_done.fetch_add(1, Ordering::Relaxed) + 1;
        let total = self.steps_target.load(Ordering::Relaxed);
        let dir = self.direction.load(Ordering::Relaxed);
        if dir == Direction::Forward as u8 {
            self.position.fetch_add(1, Ordering::Relaxed);
        } else {
            self.position.fetch_sub(1, Ordering::Relaxed);
        }
        (step, total)
    }

    // ========================================================================
    // 速度计算
    // ========================================================================

    /// 根据当前步号和运动阶段计算下一步的间隔时间（us）。
    ///
    /// 所有计算均为纯整数算术（提升到 u64 后取整），避免浮点。
    ///
    /// ## 加速段
    ///
    /// 线性内插公式（progress ∈ [0, divisor]）：
    ///
    /// ```text
    /// interval(progress) = start_interval
    ///     - (start_interval - max_interval) × progress / divisor
    /// ```
    ///
    /// - `progress = 0` → `start_interval`（起步速度）
    /// - `progress = divisor` → `max_interval`（巡航速度）
    /// - `divisor = accel_steps - 1`：共 `accel-1` 次内插，加上预设初始值合计 `accel` 次过渡
    ///
    /// ## 减速段
    ///
    /// 线性内插公式（progress ∈ [0, divisor]）：
    ///
    /// ```text
    /// interval(progress) = decel_start_interval
    ///     + (start_interval - decel_start_interval) × progress / divisor
    /// ```
    ///
    /// - `progress = 0` → `decel_start_interval`（当前实际速度）
    /// - `progress = divisor` → `start_interval`（起步速度，之后急停）
    ///
    /// `decel_start_interval` 取值规则：
    /// - `0`：正常梯形减速，使用 `max_interval`（= `speed_to_interval_us(max_speed_hz)`）
    /// - 非零：`smooth_stop` 中途减速，使用当时的 `step_interval_us`，
    ///   避免电机尚未达到最高速度时产生速度跳变
    pub(crate) fn compute_interval(&self, step: u32, total_steps: u32, phase: MotionPhase) -> u32 {
        let start_speed = self.start_speed_hz;
        let max_speed = self.max_speed_hz.load(Ordering::Relaxed);

        let start_interval = speed_to_interval_us(start_speed);
        let max_interval = speed_to_interval_us(max_speed);

        match phase {
            MotionPhase::Accelerating => {
                let accel = self.effective_accel_steps.load(Ordering::Relaxed);
                if accel <= 1 {
                    return max_interval;
                }

                // divisor = accel - 1 使得 progress/divisor ∈ [0, 1]
                let divisor = accel - 1;
                // step 是 1-based 已完成步数。progress = step 确保第一步 (step=1)
                // 就开始加速；step ≥ accel 时 clamped 到 divisor，全速运行。
                let progress = step.min(divisor);
                // interval = start - (start - max) × progress / divisor
                let computed = start_interval
                    - ((start_interval.saturating_sub(max_interval)) as u64 * progress as u64
                        / divisor as u64) as u32;
                // 防御性 clamp：归位模式无阶段切换，step 可远超 accel，
                // progress 被 min(divisor) 限制，理论上不会 overshoot，但保留安全网
                computed.max(max_interval)
            }
            // 恒定速度阶段。Probing / ProbingBackoff 不经过此路径
            // （各自在 tmc2209 中自行调度），保留分支以消除 unreachable 风险。
            MotionPhase::Cruising
            | MotionPhase::Running
            | MotionPhase::Probing
            | MotionPhase::ProbingBackoff => max_interval,
            MotionPhase::Decelerating => {
                let decel_start = self.decel_start_step.load(Ordering::Relaxed);
                let steps_into_decel = step.saturating_sub(decel_start);
                let decel_steps = total_steps.saturating_sub(decel_start);
                if decel_steps <= 1 {
                    return start_interval;
                }

                // 减速起始速度：优先用 decel_start_interval（smooth_stop 场景），
                // 若为 0 则回退到 max_interval（正常梯形减速）
                let raw_start = self.decel_start_interval.load(Ordering::Relaxed);
                let decel_start_interval = if raw_start == 0 {
                    max_interval
                } else {
                    raw_start
                };

                // divisor = decel_steps - 1，progress/divisor ∈ [0, 1]
                let divisor = decel_steps - 1;
                let progress = steps_into_decel.min(divisor);
                // interval = decel_start_interval
                //     + (start_interval - decel_start_interval) × progress / divisor
                //
                // 注意：当 decel_start_interval < start_interval 时（减速），
                // start_interval - decel_start_interval > 0，interval 递增（速度递减）；
                // 当 decel_start_interval > start_interval 时（理论上不会发生，
                // 电机起步速度应 ≤ 减速起始速度），saturating_sub 返回 0，匀速。
                decel_start_interval
                    + ((start_interval.saturating_sub(decel_start_interval)) as u64
                        * progress as u64
                        / divisor as u64) as u32
            }
            MotionPhase::Idle => start_interval,
        }
    }

    // ========================================================================
    // ISR 调度
    // ========================================================================

    /// ISR 优化版调度：直接传入已读取的 phase 原始值，避免重复原子加载和枚举转换。
    ///
    /// ## Running 模式 — 平滑调速
    ///
    /// 每步以固定步长 `SPEED_RAMP_DELTA_US`（默认 2 µs）向目标逼近：
    ///
    /// ```text
    /// interval_next = clamp(interval_current ± Δ, target)
    /// ```
    ///
    /// 调速耗时 ≈ |I_current² - I_target²| / (2 × Δ)（近似积分）。
    /// 例如 100 Hz → 1000 Hz：I₁=10000, I₂=1000, Δ=2 → ~24.75 s。
    /// 达到目标后 `speed_settle_count` 递增，供上层判断速度稳定。
    ///
    /// ## 梯形模式 — 阶段切换
    ///
    /// 根据已完成步数 `step` 判断当前处于加速/巡航/减速哪一段：
    ///
    /// ```text
    /// phase = Accelerating   if step < min(accel_steps, decel_start_step)
    ///         Decelerating   if step >= decel_start_step
    ///         Cruising       otherwise
    /// ```
    pub(crate) fn schedule_next_by_phase<T: AlarmTimer + MonotonicTimer>(
        &self,
        timer: &T,
        step: u32,
        total: u32,
        phase_raw: u8,
    ) {
        // Running — fast path: smooth speed ramp
        if phase_raw == MotionPhase::Running as u8 {
            let current = self.step_interval_us.load(Ordering::Relaxed);
            let target = self.target_interval_us.load(Ordering::Relaxed);
            let interval = if current != target {
                // 每步向目标逼近 SPEED_RAMP_DELTA_US（需保证不小于 1）
                let delta = SPEED_RAMP_DELTA_US.max(1);
                if current > target {
                    current.saturating_sub(delta).max(target)
                } else {
                    current.saturating_add(delta).min(target)
                }
            } else {
                current
            };
            self.step_interval_us.store(interval, Ordering::Relaxed);
            if interval == target {
                // 已达目标速度：递增稳定计数
                self.speed_settle_count.fetch_add(1, Ordering::Relaxed);
            } else {
                // 过渡中：清零稳定计数
                self.speed_settle_count.store(0, Ordering::Relaxed);
            }
            self.schedule_next_alarm(timer, interval);
            return;
        }

        if total > 0 && step >= total {
            self.set_idle();
            return;
        }

        let accel = self.effective_accel_steps.load(Ordering::Relaxed);
        let decel_start = self.decel_start_step.load(Ordering::Relaxed);

        // 阶段判断：step 为 1-based 已完成步数。
        // - step < accel：仍在加速段（step 1..accel-1，共 accel-1 次内插）
        // - step >= decel_start：已进入减速段
        // - 其他：巡航段
        let new_phase = if step < accel.min(decel_start) {
            MotionPhase::Accelerating
        } else if step >= decel_start {
            MotionPhase::Decelerating
        } else {
            MotionPhase::Cruising
        };
        self.motion_phase.store(new_phase as u8, Ordering::Relaxed);

        let interval = self.compute_interval(step, total, new_phase);
        self.step_interval_us.store(interval, Ordering::Relaxed);
        self.schedule_next_alarm(timer, interval);
    }

    // ========================================================================
    // ISR
    // ========================================================================

    /// Timer alarm 中断服务函数（ISR）。应在对应轴的 `TIMER0_IRQx` 部署中调用。
    ///
    /// 执行路径（最少操作）：
    /// 1. 上升沿：`STEP ↑` → `set_alarm(now + pulse_width)` → 返回
    /// 2. 下降沿：`STEP ↓` → `advance_step` → `schedule_next_by_phase`（复用已读 phase）
    pub fn handle_alarm_isr<T: AlarmTimer + MonotonicTimer>(&self, timer: &T) {
        timer.clear_alarm_interrupt(self.alarm);
        timer.clear_forced_alarm_interrupt(self.alarm);

        // 一次 atomic load，避免两次 MotionPhase::from_u8 转换
        let phase_raw = self.motion_phase.load(Ordering::Relaxed);
        if phase_raw == MotionPhase::Idle as u8 {
            return;
        }

        if !self.toggle_pulse(timer) {
            return;
        }

        let (step, total) = self.advance_step();
        self.schedule_next_by_phase(timer, step, total, phase_raw);
    }
}

// ============================================================================
// 子模块
// ============================================================================

pub mod tmc2209;
