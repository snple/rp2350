//! TMC2209 专用步进电机控制模块。
//!
//! 在通用 [`AxisState`](super::AxisState) 基础上增加：
//!
//! - **DIAG 引脚**：堵转检测与归位限位
//! - **INDEX 引脚**：电气周期对齐与丢步校验
//! - **归位模式**：向 DIAG 方向运动直到触发 + INDEX 回退精确对齐
//!
//! ## 使用方式
//!
//! ```ignore
//! use pico_rs_drivers::stepper::tmc2209::{AxisConfig, AxisState};
//! use pico_rs_drivers::stepper::Direction;
//!
//! static AXIS: AxisState = AxisState::new(AxisConfig {
//!     pin_step: 2,
//!     pin_dir: 3,
//!     alarm: 0,
//!     start_speed_hz: 500,
//!     accel_steps: 200,
//!     step_pulse_us: 3,
//!     diag_pin: 5,
//!     index_pin: 6,
//!     homing_index_backoff: 0,
//! });
//! ```

use core::marker::PhantomData;
use core::sync::atomic::{AtomicBool, AtomicU32, Ordering};

use defmt::debug;
use pico_rs::timer::{AlarmTimer, MonotonicTimer};

use super::{AxisPins, Direction, MotionPhase, speed_to_interval_us};

// ============================================================================
// TmcPins trait
// ============================================================================

/// TMC2209 引脚操作 trait，扩展 [`AxisPins`] 增加 DIAG/INDEX。
///
/// 实现为零大小类型 (ZST)，编译期单态化零开销。
pub trait TmcPins: AxisPins {
    /// 读取 DIAG 引脚：`true` = 堵转/故障。
    fn diag() -> bool;
    /// 读取 INDEX 引脚：`true` = 高电平。
    fn index() -> bool;
}

// ============================================================================
// 常量
// ============================================================================"

/// 每个电气周期包含的整步数（2 相步进电机 = 4 整步）。
/// INDEX 信号每完成一个电气周期翻转一次。
const STEPS_PER_ELEC_CYCLE: u32 = 4;

// ============================================================================
// IndexCheck
// ============================================================================

/// INDEX 信号校验结果。
///
/// TMC2209 INDEX 信号每个电气周期（4 个全步）脉冲一次。
/// 通过比较预期 INDEX 脉冲数和实际计数值来判断是否丢步。
#[derive(Clone, Copy)]
pub struct IndexCheck {
    /// 预期 INDEX 脉冲数。
    pub expected: u32,
    /// 实际计数的 INDEX 脉冲数。
    pub actual: u32,
}

impl IndexCheck {
    /// 判断 INDEX 计数是否在容差范围内。
    ///
    /// `tolerance` 为允许的 INDEX 脉冲偏差（通常为 1，允许 ±1 个脉冲的边界对齐误差）。
    pub fn is_accurate(&self, tolerance: u32) -> bool {
        let diff = self.actual.abs_diff(self.expected);
        diff <= tolerance
    }

    /// 估算丢失的微步数。
    ///
    /// 仅在实际计数少于预期时有效（`actual < expected`）。
    /// 每个 INDEX 脉冲对应 `4 × microstepping` 个微步。
    pub fn missed_microsteps(&self, microstepping: u32) -> u32 {
        let steps_per_index = STEPS_PER_ELEC_CYCLE * microstepping;
        if self.actual >= self.expected {
            0
        } else {
            (self.expected - self.actual) * steps_per_index
        }
    }
}

// ============================================================================
// AxisConfig
// ============================================================================

/// TMC2209 轴静态配置参数。
pub struct AxisConfig {
    /// Timer0 alarm 编号（对应 0/1/2 分别绑定 X/Y/Z 轴）。
    pub alarm: u8,
    /// 起步频率，单位 Hz。
    pub start_speed_hz: u32,
    /// 加减速段步数。
    pub accel_steps: u32,
    /// STEP 脉冲高电平宽度，单位 us。
    pub step_pulse_us: u32,
    /// Forward 方向对应 DIR 引脚的电平。`true` = 高电平，`false` = 低电平。
    pub fwd_level: bool,
    /// 归位回退时需要检测到的 INDEX 脉冲数（0 = 不回退）。
    pub index_backoff: u32,
    /// DIAG 消隐步数。两种模式共用：
    /// - 探测模式：跳过前 N 步，给 BEMF 建立时间
    /// - 梯形运动：两端各留 N 步缓冲区，仅巡航中段检堵
    ///   应设为 `≥ accel_steps + 少量沉降步数`。
    pub blanking_steps: u32,
}

// ============================================================================
// ============================================================================
// AxisState
// ============================================================================

/// TMC2209 单轴步进状态。
///
/// 泛型 `P` 实现 [`TmcPins`]，编译期单态化零开销。
/// 包装通用 [`AxisState`](super::AxisState)，额外添加 DIAG/INDEX/归位功能。
pub struct AxisState<P: TmcPins> {
    _pins: PhantomData<P>,
    /// 通用步进轴状态（脉冲生成、梯形加减速）。
    axis: super::AxisState<P>,

    // --- 静态配置（initialize once，之后只读）---
    /// 归位回退时需要检测到的 INDEX 脉冲数（0 = 不回退）。
    index_backoff: u32,
    /// DIAG 消隐步数（配置值，构造后不变）。探测模式和梯形运动共用此值。
    blanking_steps: u32,

    // --- 动态运动状态（中断内更新，主循环只读）---
    /// INDEX 脉冲计数器。
    index_count: AtomicU32,
    /// INDEX 引脚上一次采样状态。
    index_last_state: AtomicBool,
    /// 堵转锁存标志。
    stall_latched: AtomicBool,
    /// 归位回退目标 INDEX 计数。
    index_backoff_target: AtomicU32,
    /// 堵转检测启用（默认 true）。退离/回退等场景可临时关闭，避免 DIAG 误杀。
    stall_check_enabled: AtomicBool,
    /// 回退阶段 DIAG 已清除标志。
    backoff_diag_cleared: AtomicBool,
}

unsafe impl<P: TmcPins> Sync for AxisState<P> {}

impl<P: TmcPins> AxisState<P> {
    pub const fn new(config: AxisConfig) -> Self {
        Self {
            _pins: PhantomData,
            axis: super::AxisState::new(super::AxisConfig {
                alarm: config.alarm,
                start_speed_hz: config.start_speed_hz,
                accel_steps: config.accel_steps,
                step_pulse_us: config.step_pulse_us,
                fwd_level: config.fwd_level,
            }),
            index_backoff: config.index_backoff,
            blanking_steps: config.blanking_steps,
            index_count: AtomicU32::new(0),
            index_last_state: AtomicBool::new(false),
            stall_latched: AtomicBool::new(false),
            index_backoff_target: AtomicU32::new(0),
            stall_check_enabled: AtomicBool::new(true),
            backoff_diag_cleared: AtomicBool::new(false),
        }
    }

    // --- 内部 DIAG/INDEX 操作 ---

    #[inline]
    fn diag(&self) -> bool {
        P::diag()
    }
    #[inline]
    fn index(&self) -> bool {
        P::index()
    }

    fn reset_index(&self) {
        self.index_count.store(0, Ordering::Relaxed);
        self.stall_latched.store(false, Ordering::Relaxed);
        self.index_last_state.store(self.index(), Ordering::Relaxed);
    }

    fn detect_index_edge(&self) {
        let state = self.index();
        let last = self.index_last_state.swap(state, Ordering::Relaxed);
        if !state && last {
            self.index_count.fetch_add(1, Ordering::Relaxed);
        }
    }

    fn reset_stall_state(&self) {
        self.reset_index();
    }

    // --- 委托给内部 AxisState 的方法 ---

    #[inline]
    pub fn phase(&self) -> MotionPhase {
        self.axis.phase()
    }
    #[inline]
    pub fn position(&self) -> i32 {
        self.axis.position()
    }
    #[inline]
    pub fn steps_done(&self) -> u32 {
        self.axis.steps_done()
    }
    #[inline]
    pub fn is_done(&self) -> bool {
        self.axis.is_done()
    }
    #[inline]
    pub fn set_position(&self, pos: i32) {
        self.axis.set_position(pos);
    }

    /// 阻塞等待运动完成（使用 WFI 省电）。
    pub fn wait_done(&self) {
        self.axis.wait_done();
    }

    /// 标记轴为即将开始运动状态（双核架构用）。
    #[inline]
    pub fn prepare_start(&self) {
        self.axis.prepare_start();
    }

    /// 设置最大行程。委托给底层通用 [`AxisState`](super::AxisState)。
    #[inline]
    pub fn set_max_travel(&self, travel: i32) {
        self.axis.set_max_travel(travel);
    }

    /// 启动有限步数的相对位移运动，带梯形加减速。
    ///
    /// 轴忙时直接返回。步数受 [`max_travel`](super::AxisState::set_max_travel) 约束。
    pub fn start_move<T: AlarmTimer + MonotonicTimer>(
        &self,
        timer: &T,
        steps: u32,
        dir: Direction,
        max_speed_hz: u32,
    ) {
        if self.axis.is_busy() {
            return;
        }
        self.reset_stall_state();
        self.axis.start_move(timer, steps, dir, max_speed_hz);
    }

    /// 启动不限步数的连续运动，无加减速。
    ///
    /// 轴忙时直接返回。如果设置了最大行程，自动转为有限步数的 `start_move`。
    pub fn start_continuous<T: AlarmTimer + MonotonicTimer>(
        &self,
        timer: &T,
        dir: Direction,
        speed_hz: u32,
    ) {
        if self.axis.is_busy() {
            return;
        }
        if let Some(max_steps) = self.axis.continuous_max_steps(dir) {
            if max_steps == 0 {
                self.axis.motion_done.store(true, Ordering::Relaxed);
                return;
            }
            self.start_move(timer, max_steps, dir, speed_hz);
            return;
        }
        // 无限位
        self.stall_check_enabled.store(true, Ordering::Relaxed);
        self.reset_stall_state();
        self.axis.start_continuous(timer, dir, speed_hz);
    }

    /// 动态更新目标步数和减速起始点。
    pub fn set_target_steps(&self, new_total_steps: u32) {
        self.axis.set_target_steps(new_total_steps);
    }

    /// 立即停止轴运动。
    pub fn stop(&self) {
        self.axis.stop();
    }

    // --- DIAG ---

    /// 返回 DIAG 引脚当前电平（`true` = 堵转/故障）。
    #[inline(always)]
    pub fn diag_triggered(&self) -> bool {
        self.diag()
    }

    /// 返回本次运动是否发生过堵转（DIAG 锁存标志）。
    ///
    /// 由 ISR 在检测到 DIAG 时自动置位，调用 `start_move` / `start_continuous` /
    /// `probe_endstop` 时自动清零。
    #[inline]
    pub fn stall_occurred(&self) -> bool {
        self.stall_latched.load(Ordering::Relaxed)
    }

    /// 临时关闭堵转检测（不影响 `stall_occurred()` 锁存读取）。
    ///
    /// 用于退离/回退等场景：堵转后电机静止时 DIAG 持续为高，
    /// 此时启动反向运动会立即触发误判。关闭后 ISR 跳过 DIAG 检查。
    ///
    /// `start_continuous` 和 `probe_endstop` 会自动恢复为启用状态，
    /// `start_move` 不会自动恢复——由调用者通过本方法和
    /// [`enable_stall_check`] 控制。
    #[inline]
    pub fn disable_stall_check(&self) {
        self.stall_check_enabled.store(false, Ordering::Relaxed);
    }

    /// 恢复堵转检测启用。`start_continuous` / `probe_endstop` 会自动调用，
    /// `start_move` 不会——由调用者决定。
    #[inline]
    pub fn enable_stall_check(&self) {
        self.stall_check_enabled.store(true, Ordering::Relaxed);
    }

    // --- INDEX ---

    /// 返回当前 INDEX 脉冲计数。
    #[inline]
    pub fn index_count(&self) -> u32 {
        self.index_count.load(Ordering::Relaxed)
    }

    /// 使用 INDEX 信号校验本次运动的步进精度，检测是否丢步。
    ///
    /// ## 调用时机
    ///
    /// 运动完成后（`is_done() == true`）。
    ///
    /// ## 计算原理
    ///
    /// TMC2209 INDEX 信号每 4 个全步（一个完整电气周期）输出一个脉冲。
    /// `expected = total_steps / (4 × microstepping)`
    ///
    /// ## 示例
    ///
    /// ```ignore
    /// AXIS.start_move(&timer, 800, Direction::Forward, 5000);
    /// AXIS.wait_done();
    /// let check = AXIS.check_index(800, 4);
    /// if !check.is_accurate(1) {
    ///     warn!("丢步! 预期 {} INDEX, 实际 {}", check.expected, check.actual);
    /// }
    /// ```
    pub fn check_index(&self, total_steps: u32, microstepping: u32) -> IndexCheck {
        let steps_per_index = STEPS_PER_ELEC_CYCLE * microstepping.max(1);
        let expected = total_steps / steps_per_index;
        let actual = self.index_count.load(Ordering::Relaxed);
        IndexCheck { expected, actual }
    }

    // --- 末端探测（归位 / 行程校准） ---

    /// 向指定方向运动，直到 DIAG 堵转触发，INDEX 回退精确对齐。
    ///
    /// 纯探测——不修改位置坐标。调用者探测后自行决定：
    ///
    /// ```ignore
    /// // 探测近端，标记为 home
    /// AXIS.probe_endstop(&timer, Direction::Reverse, 2000);
    /// AXIS.wait_done();
    /// AXIS.set_position(0);
    ///
    /// // 探测远端，计算行程
    /// AXIS.probe_endstop(&timer, Direction::Forward, 2000);
    /// AXIS.wait_done();
    /// let travel = AXIS.position();
    ///
    /// // 设置行程保护
    /// AXIS.set_max_travel(travel);
    /// ```
    ///
    /// INDEX 回退量由 [`AxisConfig::homing_index_backoff`] 控制，
    /// 设为 0 则跳过回退，DIAG 触发即停止。
    pub fn probe_endstop<T: AlarmTimer + MonotonicTimer>(
        &self,
        timer: &T,
        dir: Direction,
        speed_hz: u32,
    ) {
        if self.axis.is_busy() {
            return;
        }
        self.stall_check_enabled.store(true, Ordering::Relaxed);
        self.reset_stall_state();

        // 不提前检查 DIAG——直接开始探测。
        // 若在限位处：第一步会触发 DIAG → handle_probing_step 自动 backoff
        // 若不在限位：正常探测到远端
        // 两种情况的 backoff 方向都由探测方向推导，保证正确。

        self.axis.motion_done.store(false, Ordering::Relaxed);
        self.axis.steps_done.store(0, Ordering::Relaxed);
        self.axis.steps_target.store(0, Ordering::Relaxed);
        self.axis.max_speed_hz.store(speed_hz, Ordering::Relaxed);
        self.axis
            .effective_accel_steps
            .store(self.axis.accel_steps, Ordering::Relaxed);

        self.axis.begin_motion(
            timer,
            dir,
            MotionPhase::Probing,
            speed_to_interval_us(self.axis.start_speed_hz),
        );
    }

    // --- ISR ---

    /// Timer alarm 中断服务函数（ISR）。
    ///
    /// 在通用脉冲逻辑基础上，每步完成后额外检测 INDEX 信号和 DIAG 堵转。
    /// 归位和归位回退阶段由本方法独立处理，其他阶段委托给
    /// [`AxisState::schedule_next_by_phase`]。
    pub fn handle_alarm_isr<T: AlarmTimer + MonotonicTimer>(&self, timer: &T) {
        timer.clear_alarm_interrupt(self.axis.alarm);
        timer.clear_forced_alarm_interrupt(self.axis.alarm);

        let phase_raw = self.axis.motion_phase.load(Ordering::Relaxed);
        if phase_raw == MotionPhase::Idle as u8 {
            return;
        }

        // 脉冲翻转（上升沿自动安排脉冲宽度）
        if !self.axis.toggle_pulse(timer) {
            return;
        }

        // 下降沿：步完成
        let (step, _total) = self.axis.advance_step();

        // INDEX 下降沿检测
        self.detect_index_edge();

        match MotionPhase::from_u8(phase_raw) {
            MotionPhase::Probing => self.handle_probing_step(timer, step),
            MotionPhase::ProbingBackoff => self.handle_probing_backoff_step(timer),
            _ => {
                // DIAG 堵转检测：仅巡航阶段 + 两端缓冲区之外。
                //
                // SG 值在加速→巡航、巡航→减速两个过渡点都不稳定，
                // 容易假阳性。因此只在巡航中段检堵：
                //   step ∈ [blanking_steps, total - blanking_steps]
                // 即前后各留 blanking_steps 作为缓冲区。
                //
                // 建议 blanking_steps ≥ accel_steps + 沉降步数，
                // 确保缓冲区完全覆盖加速/减速段及过渡。
                let is_cruising = phase_raw == MotionPhase::Cruising as u8;
                let total = self.axis.steps_target.load(Ordering::Relaxed);
                let in_window = step >= self.blanking_steps
                    && step <= total.saturating_sub(self.blanking_steps);
                if is_cruising
                    && in_window
                    && self.stall_check_enabled.load(Ordering::Relaxed)
                    && self.diag_triggered()
                {
                    self.stall_latched.store(true, Ordering::Relaxed);
                    self.axis.set_idle();
                    return;
                }
                self.axis
                    .schedule_next_by_phase(timer, step, total, phase_raw);
            }
        }
    }

    /// 探测模式下每步处理：检测限位 + 加速。
    ///
    /// 前 `blanking_steps` 步跳过 DIAG 检测，给电机时间建立 BEMF。
    /// 与正常模式共用 `blanking_steps` 参数。
    ///
    /// 注意：探测模式**不检查** [`stall_check_enabled`]，始终检堵。
    /// 因为归位是安全关键操作，不应被用户临时禁用。
    fn handle_probing_step<T: AlarmTimer + MonotonicTimer>(&self, timer: &T, step: u32) {
        if step >= self.blanking_steps && self.diag_triggered() {
            debug!(
                "probe DIAG@step={} pos={} dir={}",
                step,
                self.axis.position(),
                self.axis.direction.load(Ordering::Relaxed),
            );
            if self.index_backoff > 0 {
                let dir = Direction::from_u8(self.axis.direction.load(Ordering::Relaxed));
                self.do_backoff(dir.backward(), timer);
                return;
            }
            // 无 INDEX 回退：直接停止
            self.axis.set_idle();
            return;
        }

        // 继续运动（带加速）
        let new_interval = self
            .axis
            .compute_interval(step, 0, MotionPhase::Accelerating);
        self.axis
            .step_interval_us
            .store(new_interval, Ordering::Relaxed);
        self.axis.schedule_next_alarm(timer, new_interval);
    }

    /// 启动 INDEX 回退：向 `back_dir` 方向低速运动，直到检测到 N 个 INDEX 脉冲。
    ///
    /// 由 [`handle_probing_step`]（探测中 DIAG 触发）和
    /// [`probe_endstop`]（已在限位）共同调用。
    fn do_backoff<T: AlarmTimer + MonotonicTimer>(&self, back_dir: Direction, timer: &T) {
        self.reset_index();
        self.index_backoff_target
            .store(self.index_backoff, Ordering::Relaxed);
        // 回退启动时 DIAG 必然为高（刚从限位离开），标记未清除
        self.backoff_diag_cleared.store(false, Ordering::Relaxed);
        // 使用起步速度而非当前探测速度：从静止以低速启动，避免丢步
        self.axis.begin_motion(
            timer,
            back_dir,
            MotionPhase::ProbingBackoff,
            speed_to_interval_us(self.axis.start_speed_hz),
        );
    }

    /// 回退模式下每步处理：检测 INDEX 脉冲计数，同时检测 DIAG 防止对面限位撞击。
    ///
    /// 回退启动时 DIAG 必然为高（刚从限位离开），因此不能立即检测 DIAG。
    /// 使用 `backoff_diag_cleared` 标志：DIAG 第一次变低后置 true，
    /// 之后若 DIAG 再次变高则说明撞上对面限位，立即停止。
    fn handle_probing_backoff_step<T: AlarmTimer + MonotonicTimer>(&self, timer: &T) {
        let diag = self.diag_triggered();
        if !diag {
            // DIAG 已清除：标记"曾离开过限位"
            if !self.backoff_diag_cleared.load(Ordering::Relaxed) {
                debug!("backoff DIAG cleared pos={}", self.axis.position());
            }
            self.backoff_diag_cleared.store(true, Ordering::Relaxed);
        } else if self.backoff_diag_cleared.load(Ordering::Relaxed) {
            // DIAG 高 且 之前已清除过 → 撞上了对面限位
            debug!(
                "backoff DIAG re-triggered (opposite endstop) pos={} INDEX={}",
                self.axis.position(),
                self.index_count.load(Ordering::Relaxed),
            );
            self.axis.set_idle();
            return;
        }
        let target = self.index_backoff_target.load(Ordering::Relaxed);
        if self.index_count.load(Ordering::Relaxed) >= target {
            debug!(
                "backoff done INDEX>={} pos={}",
                target,
                self.axis.position(),
            );
            self.axis.set_idle();
            return;
        }
        let interval = self.axis.step_interval_us.load(Ordering::Relaxed);
        self.axis.schedule_next_alarm(timer, interval);
    }
}
