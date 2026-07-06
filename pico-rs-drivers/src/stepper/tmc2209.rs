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
//!     homing_backoff_index: 0,
//! });
//! ```

use core::sync::atomic::{AtomicBool, AtomicU32, Ordering};

use pico_rs::gpioc;
use pico_rs::timer::{AlarmTimer, MonotonicTimer};

use super::{Direction, MotionPhase, speed_to_interval_us};

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
        let steps_per_index = 4 * microstepping;
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
    /// STEP 信号引脚编号。
    pub pin_step: u32,
    /// DIR 方向控制引脚编号。
    pub pin_dir: u32,
    /// Timer0 alarm 编号（对应 0/1/2 分别绑定 X/Y/Z 轴）。
    pub alarm: u8,
    /// 起步频率，单位 Hz。
    pub start_speed_hz: u32,
    /// 加减速段步数。
    pub accel_steps: u32,
    /// STEP 脉冲高电平宽度，单位 us。
    pub step_pulse_us: u32,
    /// 限位开关 / DIAG 引脚编号（高电平有效）。
    pub diag_pin: u32,
    /// INDEX 引脚编号（TMC2209 INDEX 信号，每完整电气周期脉冲一次）。
    pub index_pin: u32,
    /// 归位回退时需要检测到的 INDEX 脉冲数（0 = 不回退）。
    pub homing_backoff_index: u32,
}

// ============================================================================
// AxisState
// ============================================================================

/// TMC2209 单轴步进状态。
///
/// 包装通用 [`AxisState`](super::AxisState)，额外添加 DIAG/INDEX/归位功能。
pub struct AxisState {
    /// 通用步进轴状态（脉冲生成、梯形加减速）。
    axis: super::AxisState,
    /// DIAG 引脚编号。
    diag_pin: u32,
    /// INDEX 引脚编号。
    index_pin: u32,
    /// 归位回退时需要检测到的 INDEX 脉冲数（0 = 不回退）。
    homing_backoff_index: u32,
    /// INDEX 脉冲计数器（每个完整电气周期 +1）。
    index_count: AtomicU32,
    /// INDEX 引脚上一次采样状态。
    index_last_state: AtomicBool,
    /// 归位回退目标 INDEX 计数。
    homing_backoff_target: AtomicU32,
    /// 堵转锁存标志（DIAG 触发后被 ISR 置 true，`start_*` 时清零）。
    stall_latched: AtomicBool,
    /// 堵转检测使能（默认 true）。退离/回退等场景可临时关闭，避免 DIAG 误杀。
    stall_check_enabled: AtomicBool,
}

unsafe impl Sync for AxisState {}

impl AxisState {
    /// 根据静态配置创建 TMC2209 轴状态对象。
    pub const fn new(config: AxisConfig) -> Self {
        Self {
            axis: super::AxisState::new(super::AxisConfig {
                pin_step: config.pin_step,
                pin_dir: config.pin_dir,
                alarm: config.alarm,
                start_speed_hz: config.start_speed_hz,
                accel_steps: config.accel_steps,
                step_pulse_us: config.step_pulse_us,
            }),
            diag_pin: config.diag_pin,
            index_pin: config.index_pin,
            homing_backoff_index: config.homing_backoff_index,
            index_count: AtomicU32::new(0),
            index_last_state: AtomicBool::new(false),
            homing_backoff_target: AtomicU32::new(0),
            stall_latched: AtomicBool::new(false),
            stall_check_enabled: AtomicBool::new(true),
        }
    }

    // --- 委托给内部 AxisState 的方法 ---

    /// 返回当前运动阶段。
    #[inline]
    pub fn phase(&self) -> MotionPhase {
        self.axis.phase()
    }

    /// 返回当前逻辑坐标（步）。
    #[inline]
    pub fn position(&self) -> i32 {
        self.axis.position()
    }

    /// 返回本次运动已完成的步数。
    #[inline]
    pub fn steps_done(&self) -> u32 {
        self.axis.steps_done()
    }

    /// 返回轴是否处于可新建状态（Idle）。
    #[inline]
    pub fn is_done(&self) -> bool {
        self.axis.is_done()
    }

    /// 将逻辑坐标强制写入指定值。
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

    /// 启动有限步数的相对位移运动，带梯形加减速。
    pub fn start_move<T: AlarmTimer + MonotonicTimer>(
        &self,
        timer: &T,
        steps: u32,
        dir: Direction,
        max_speed_hz: u32,
    ) {
        self.stall_latched.store(false, Ordering::Relaxed);
        self.reset_index();
        self.axis.start_move(timer, steps, dir, max_speed_hz);
    }

    /// 启动不限步数的连续运动，无加减速。
    pub fn start_continuous<T: AlarmTimer + MonotonicTimer>(
        &self,
        timer: &T,
        dir: Direction,
        speed_hz: u32,
    ) {
        self.stall_latched.store(false, Ordering::Relaxed);
        self.stall_check_enabled.store(true, Ordering::Relaxed);
        self.reset_index();
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
        unsafe { gpioc::get(self.diag_pin) }
    }

    /// 返回本次运动是否发生过堵转（DIAG 锁存标志）。
    ///
    /// 由 ISR 在检测到 DIAG 时自动置位，调用 `start_move` / `start_continuous` /
    /// `start_homing` 时自动清零。
    #[inline]
    pub fn stall_occurred(&self) -> bool {
        self.stall_latched.load(Ordering::Relaxed)
    }

    /// 临时关闭堵转检测（不影响 `stall_occurred()` 锁存读取）。
    ///
    /// 用于退离/回退等场景：堵转后电机静止时 DIAG 持续为高，
    /// 此时启动反向运动会立即触发误判。关闭后 ISR 跳过 DIAG 检查。
    ///
    /// `start_continuous` 和 `start_homing` 会自动恢复为使能状态，
    /// `start_move` 不会自动恢复——由调用者通过本方法和
    /// [`enable_stall_check`] 控制。
    #[inline]
    pub fn disable_stall_check(&self) {
        self.stall_check_enabled.store(false, Ordering::Relaxed);
    }

    /// 恢复堵转检测使能。`start_continuous` / `start_homing` 会自动调用，
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

    /// 重置 INDEX 计数器并快照当前 INDEX 引脚电平。
    fn reset_index(&self) {
        self.index_count.store(0, Ordering::Relaxed);
        let state = unsafe { gpioc::get(self.index_pin) };
        self.index_last_state.store(state, Ordering::Relaxed);
    }

    /// INDEX 下降沿检测（由 ISR 在每步完成后调用）。
    #[inline(always)]
    fn detect_index_edge(&self) {
        let state = unsafe { gpioc::get(self.index_pin) };
        let last = self.index_last_state.swap(state, Ordering::Relaxed);
        if !state && last {
            self.index_count.fetch_add(1, Ordering::Relaxed);
        }
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
        let steps_per_index = 4 * microstepping.max(1);
        let expected = total_steps / steps_per_index;
        let actual = self.index_count.load(Ordering::Relaxed);
        IndexCheck { expected, actual }
    }

    // --- 归位 ---

    /// 启动归位运动：向反方向连续运动直到触碰限位开关。
    ///
    /// - 如果 DIAG 已触发且无 INDEX 回退，直接置零并返回 Idle。
    /// - 如果 DIAG 已触发且有 INDEX 回退，进入 [`HomingBackoff`](MotionPhase::HomingBackoff)。
    /// - 否则从起步速度加速至目标速度，向反方向运动，ISR 每步检测 DIAG 信号。
    pub fn start_homing<T: AlarmTimer + MonotonicTimer>(&self, timer: &T, speed_hz: u32) {
        self.stall_latched.store(false, Ordering::Relaxed);
        self.stall_check_enabled.store(true, Ordering::Relaxed);

        // 已在限位位置
        if self.diag_triggered() {
            if self.homing_backoff_index > 0 {
                self.start_homing_backoff(timer, speed_hz);
                return;
            }
            self.axis.position.store(0, Ordering::Relaxed);
            self.axis
                .motion_phase
                .store(MotionPhase::Idle as u8, Ordering::Relaxed);
            self.axis.motion_done.store(true, Ordering::Relaxed);
            return;
        }

        self.axis.motion_done.store(false, Ordering::Relaxed);
        self.axis.steps_done.store(0, Ordering::Relaxed);
        self.axis.steps_target.store(0, Ordering::Relaxed);
        self.axis.max_speed_hz.store(speed_hz, Ordering::Relaxed);
        self.axis
            .effective_accel_steps
            .store(self.axis.accel_steps, Ordering::Relaxed);
        self.reset_index();
        self.axis
            .direction
            .store(Direction::Reverse as u8, Ordering::Relaxed);
        self.axis.pulse_state.store(false, Ordering::Relaxed);

        unsafe { gpioc::set(self.axis.pin_dir, Direction::Reverse as u32 != 0) };

        self.axis
            .motion_phase
            .store(MotionPhase::Homing as u8, Ordering::Relaxed);

        let interval = speed_to_interval_us(self.axis.start_speed_hz);
        self.axis
            .step_interval_us
            .store(interval, Ordering::Relaxed);

        let now = timer.get_time_us_32();
        timer.set_alarm_safe(self.axis.alarm, now.wrapping_add(interval));
        timer.enable_alarm_interrupt(self.axis.alarm);
    }

    /// 启动归位回退：触发限位后反向低速运动，直到检测到指定数量的 INDEX 脉冲。
    fn start_homing_backoff<T: AlarmTimer + MonotonicTimer>(&self, timer: &T, speed_hz: u32) {
        self.axis.motion_done.store(false, Ordering::Relaxed);
        self.axis.steps_done.store(0, Ordering::Relaxed);
        self.axis.steps_target.store(0, Ordering::Relaxed);
        self.reset_index();
        self.homing_backoff_target
            .store(self.homing_backoff_index, Ordering::Relaxed);
        self.axis
            .direction
            .store(Direction::Forward as u8, Ordering::Relaxed);
        self.axis.pulse_state.store(false, Ordering::Relaxed);

        unsafe { gpioc::set(self.axis.pin_dir, Direction::Forward as u32 != 0) };

        self.axis
            .motion_phase
            .store(MotionPhase::HomingBackoff as u8, Ordering::Relaxed);

        // 回退速度取归位速度的一半，确保低速精确对齐
        let backoff_speed = (speed_hz / 2).max(1);
        let interval = speed_to_interval_us(backoff_speed);
        self.axis
            .step_interval_us
            .store(interval, Ordering::Relaxed);

        let now = timer.get_time_us_32();
        timer.set_alarm_safe(self.axis.alarm, now.wrapping_add(interval));
        timer.enable_alarm_interrupt(self.axis.alarm);
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
        if phase_raw == 0 {
            return; // Idle
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
            MotionPhase::Homing => self.handle_homing_step(timer, step),
            MotionPhase::HomingBackoff => self.handle_homing_backoff_step(timer),
            _ => {
                // 通用阶段：先检测 DIAG 堵转（如果使能），再调度下一步
                if self.stall_check_enabled.load(Ordering::Relaxed) && self.diag_triggered() {
                    self.stall_latched.store(true, Ordering::Relaxed);
                    self.axis.motion_phase.store(0, Ordering::Relaxed); // Idle
                    self.axis.motion_done.store(true, Ordering::Relaxed);
                    return;
                }
                let total = self.axis.steps_target.load(Ordering::Relaxed);
                self.axis
                    .schedule_next_by_phase(timer, step, total, phase_raw);
            }
        }
    }

    /// 归位模式下每步处理：检测限位 + 加速。
    fn handle_homing_step<T: AlarmTimer + MonotonicTimer>(&self, timer: &T, step: u32) {
        if self.diag_triggered() {
            // 限位开关触发
            if self.homing_backoff_index > 0 {
                // 切换到回退模式
                self.reset_index();
                self.homing_backoff_target
                    .store(self.homing_backoff_index, Ordering::Relaxed);
                self.axis
                    .direction
                    .store(Direction::Forward as u8, Ordering::Relaxed);
                unsafe { gpioc::set(self.axis.pin_dir, Direction::Forward as u32 != 0) };
                self.axis
                    .motion_phase
                    .store(MotionPhase::HomingBackoff as u8, Ordering::Relaxed);
                // 回退速度减半以提高精度
                let interval = self.axis.step_interval_us.load(Ordering::Relaxed) * 2;
                self.axis
                    .step_interval_us
                    .store(interval, Ordering::Relaxed);
                let now = timer.get_time_us_32();
                timer.set_alarm_safe(self.axis.alarm, now.wrapping_add(interval));
                return;
            }
            self.axis.position.store(0, Ordering::Relaxed);
            self.axis
                .motion_phase
                .store(MotionPhase::Idle as u8, Ordering::Relaxed);
            self.axis.motion_done.store(true, Ordering::Relaxed);
            return;
        }

        // 继续归位运动（带加速），复用通用加速算法
        let new_interval = self
            .axis
            .compute_interval(step, 0, MotionPhase::Accelerating);
        self.axis
            .step_interval_us
            .store(new_interval, Ordering::Relaxed);
        let now = timer.get_time_us_32();
        timer.set_alarm_safe(self.axis.alarm, now.wrapping_add(new_interval));
    }

    /// 归位回退模式下每步处理：检测 INDEX 脉冲计数。
    fn handle_homing_backoff_step<T: AlarmTimer + MonotonicTimer>(&self, timer: &T) {
        let target = self.homing_backoff_target.load(Ordering::Relaxed);
        if self.index_count.load(Ordering::Relaxed) >= target {
            self.axis.position.store(0, Ordering::Relaxed);
            self.axis
                .motion_phase
                .store(MotionPhase::Idle as u8, Ordering::Relaxed);
            self.axis.motion_done.store(true, Ordering::Relaxed);
            return;
        }
        let interval = self.axis.step_interval_us.load(Ordering::Relaxed);
        let now = timer.get_time_us_32();
        timer.set_alarm_safe(self.axis.alarm, now.wrapping_add(interval));
    }
}
