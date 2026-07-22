//! # TMC2209 四电机同步控制
//!
//! 同时控制 4 个 TMC2209 步进电机。电机 3/4 共用 DIR/STEP 引脚，
//! 因此实际由 3 个 AxisState 驱动 3 个物理轴，电机 4 仅通过 UART
//! 配置电流和微步参数。
//!
//! 使用硬件 TRNG 生成真随机目标位置进行并行三轴往返丢步检测。
//!
//! ## 电机映射
//!
//! | 电机 | 地址   | DIR | STEP | INDEX | DIAG | AxisState |
//! |------|--------|-----|------|-------|------|-----------|
//! | 1    | 0x00   | 8   | 9    | 10    | 11   | AXIS1     |
//! | 2    | 0x01   | 12  | 13   | 14    | 15   | AXIS2     |
//! | 3    | 0x02   | 16  | 17   | 18    | 19   | AXIS3     |
//! | 4    | 0x03   | 16* | 17*  | 20    | 21   | (仅 UART) |
//!
//! \* 电机 3/4 物理共用 DIR/STEP，控制电机 3 即同时驱动两电机。
//!
//! ## 运动参数
//!
//! - 步进电机: 1.8° 步距角, 1/16 微步 → 3200 微步/转
//! - 丝杆导程: 2mm/转 → 1600 微步/mm
//! - 极限行程: 55mm (88,000 微步)
//! - 两端各留 5mm 安全边距 → 有效行程 45mm (72,000 微步)
//!
//! ## 流程
//!
//! ```text
//! 1. 初始化 4 个 TMC2209 (共享 UART 总线, 不同从机地址)
//! 2. 按干扰避免序列校准三个轴:
//!    Step 1: M1 归位 → +5mm offset → 设软限位
//!    Step 2: M2 归位 → +5mm offset → 设软限位
//!    Step 3: M3 归位 → +5mm offset → 设软限位
//!    Step 4: M2 重新归位 (修正 M3 运动引入的偏差)
//! 3. 三轴并行随机往返 + INDEX 丢步检测
//! ```
//!
//! ## 编译运行
//!
//! ```bash
//! cd pico-rs-examples
//! cargo run --bin stepper_multi_axis --target thumbv8m.main-none-eabihf
//! ```

#![no_std]
#![no_main]

use pico_rs_drivers::stepper::AxisPins;
use pico_rs_drivers::stepper::Direction;
use pico_rs_drivers::stepper::tmc2209::{AxisConfig, AxisState, TmcPins};
use pico_rs_drivers::tmc2209::Tmc2209;

use defmt::*;
use defmt_rtt as _;

use pico_rs::Steal;
use pico_rs::clocks::ClockBuilder;
use pico_rs::gpio::{self};
use pico_rs::gpioc;
use pico_rs::interrupt;
use pico_rs::main;
use pico_rs::pac::{Interrupt, Peripherals};
use pico_rs::ticks::Ticks;
use pico_rs::timer::Timer0;
use pico_rs::trng::Trng;
use pico_rs::uart::{DataBits, Parity, StopBits, Uart1};

// ============================================================================
// 运动参数 (所有电机共用)
// ============================================================================

const START_SPEED_HZ: u32 = 400;
const RUN_SPEED_HZ: u32 = 40000;
const ACCEL_STEPS: u32 = 100;
const BLANKING_STEPS: u32 = 200;
const STEP_PULSE_US: u32 = 3;
const MICROSTEPPING: u32 = 16;
const FWD_LEVEL: bool = false;

// ============================================================================
// TMC2209 驱动参数
// ============================================================================

const RUN_CURRENT: u8 = 16;
const HOLD_CURRENT: u8 = 4;
const IHOLD_DELAY: u8 = 10;
const SG_THRESHOLD: u32 = 90;
const TMC_UART_BAUD: u32 = 115_200;
const INDEX_BACKOFF: u32 = 10;

// ============================================================================
// 引脚分配
// ============================================================================

const PIN_ENABLE: u32 = 7;

// --- 电机 1: GP8–11, UART addr 0x00 ---
const M1_ADDR: u8 = 0x00;
const M1_DIR: u32 = 8;
const M1_STEP: u32 = 9;
const M1_INDEX: u32 = 10;
const M1_DIAG: u32 = 11;

// --- 电机 2: GP12–15, UART addr 0x01 ---
const M2_ADDR: u8 = 0x01;
const M2_DIR: u32 = 12;
const M2_STEP: u32 = 13;
const M2_INDEX: u32 = 14;
const M2_DIAG: u32 = 15;

// --- 电机 3: GP16–19, UART addr 0x02 ---
const M3_ADDR: u8 = 0x02;
const M3_DIR: u32 = 16;
const M3_STEP: u32 = 17;
const M3_INDEX: u32 = 18;
const M3_DIAG: u32 = 19;

// --- 电机 4: UART addr 0x03, DIR/STEP 与电机 3 共用 ---
const M4_ADDR: u8 = 0x03;
// DIR=16, STEP=17 与 M3 共享, INDEX=20, DIAG=21 暂忽略

// ============================================================================
// Pin trait 实现 — 零开销编译期单态化
// ============================================================================

struct Mot1Pins;

impl AxisPins for Mot1Pins {
    #[inline]
    fn set_step(high: bool) {
        unsafe { gpioc::set(M1_STEP, high) };
    }
    #[inline]
    fn set_dir(high: bool) {
        unsafe { gpioc::set(M1_DIR, high) };
    }
}

impl TmcPins for Mot1Pins {
    #[inline]
    fn diag() -> bool {
        unsafe { gpioc::get(M1_DIAG) }
    }
    #[inline]
    fn index() -> bool {
        unsafe { gpioc::get(M1_INDEX) }
    }
}

struct Mot2Pins;

impl AxisPins for Mot2Pins {
    #[inline]
    fn set_step(high: bool) {
        unsafe { gpioc::set(M2_STEP, high) };
    }
    #[inline]
    fn set_dir(high: bool) {
        unsafe { gpioc::set(M2_DIR, high) };
    }
}

impl TmcPins for Mot2Pins {
    #[inline]
    fn diag() -> bool {
        unsafe { gpioc::get(M2_DIAG) }
    }
    #[inline]
    fn index() -> bool {
        unsafe { gpioc::get(M2_INDEX) }
    }
}

struct Mot3Pins;

impl AxisPins for Mot3Pins {
    #[inline]
    fn set_step(high: bool) {
        unsafe { gpioc::set(M3_STEP, high) };
    }
    #[inline]
    fn set_dir(high: bool) {
        unsafe { gpioc::set(M3_DIR, high) };
    }
}

impl TmcPins for Mot3Pins {
    #[inline]
    fn diag() -> bool {
        unsafe { gpioc::get(M3_DIAG) }
    }
    #[inline]
    fn index() -> bool {
        unsafe { gpioc::get(M3_INDEX) }
    }
}

// ============================================================================
// 全局步进轴实例 — 三个独立 AxisState，各用不同 Timer0 alarm
// ============================================================================

static AXIS1: AxisState<Mot1Pins> = AxisState::new(AxisConfig {
    alarm: 0,
    start_speed_hz: START_SPEED_HZ,
    accel_steps: ACCEL_STEPS,
    step_pulse_us: STEP_PULSE_US,
    index_backoff: INDEX_BACKOFF,
    fwd_level: FWD_LEVEL,
    blanking_steps: BLANKING_STEPS,
});

static AXIS2: AxisState<Mot2Pins> = AxisState::new(AxisConfig {
    alarm: 1,
    start_speed_hz: START_SPEED_HZ,
    accel_steps: ACCEL_STEPS,
    step_pulse_us: STEP_PULSE_US,
    index_backoff: INDEX_BACKOFF,
    fwd_level: FWD_LEVEL,
    blanking_steps: BLANKING_STEPS,
});

static AXIS3: AxisState<Mot3Pins> = AxisState::new(AxisConfig {
    alarm: 2,
    start_speed_hz: START_SPEED_HZ,
    accel_steps: ACCEL_STEPS,
    step_pulse_us: STEP_PULSE_US,
    index_backoff: INDEX_BACKOFF,
    fwd_level: FWD_LEVEL,
    blanking_steps: BLANKING_STEPS,
});

#[inline(always)]
fn timer() -> Steal<Timer0> {
    unsafe { Timer0::steal() }
}

// ============================================================================
// 辅助宏：初始化一组电机的 GPIO 引脚
// ============================================================================

macro_rules! init_motor_pins {
    ($pins:expr, $dir:expr, $step:expr, $index:expr, $diag:expr) => {{
        let mut _d = $pins.pin::<$dir>();
        _d.into_output();
        unsafe { gpio::set_low($dir) };

        let mut _s = $pins.pin::<$step>();
        _s.into_output();
        unsafe { gpio::set_low($step) };

        let mut _i = $pins.pin::<$index>();
        _i.into_input_pullup();

        let mut _g = $pins.pin::<$diag>();
        _g.into_input_pullup();
    }};
}

// ============================================================================
// 辅助宏：配置 TMC2209 并验证通讯
// ============================================================================

macro_rules! tmc_init {
    ($uart:expr, $addr:expr, $timer:expr, $label:expr) => {{
        let tmc = Tmc2209::new($uart, $addr);

        match tmc.read_ioin($timer) {
            Some(v) => info!("[{}] addr={:#x} ver={:#04x} ✓", $label, $addr, v.version()),
            None => {
                warn!("[{}] TMC2209 not responding — halt", $label);
                loop {
                    pico_rs_rt::asm::wfi();
                }
            }
        }

        tmc.configure(
            $timer,
            MICROSTEPPING,
            RUN_CURRENT,
            HOLD_CURRENT,
            IHOLD_DELAY,
            SG_THRESHOLD,
        );
        tmc.set_tcool_threshold(0xFFFFF, $timer);
        tmc.stop_vactual($timer);
        info!(
            "[{}] configured: 1/{} μstep, IRUN={}, IHOLD={}, SGTHRS={}",
            $label, MICROSTEPPING, RUN_CURRENT, HOLD_CURRENT, SG_THRESHOLD
        );

        tmc
    }};
}

// ============================================================================
// 行程常量
//
// 步进电机: 1.8° 步距角 (200 全步/转), 1/16 微步, 丝杆导程 2mm
//   1 转 = 200 × 16 = 3200 微步 = 2mm → 1600 微步/mm
//   极限行程 55mm → 88000 微步
//   两端各留 5mm 安全边距 → 有效行程 45mm → 72000 微步
// ============================================================================
// 1mm 对应微步数
const STEPS_PER_MM: u32 = 200 * MICROSTEPPING / 2;

// 极限行程 (55mm)
const MAX_TRAVEL: u32 = 55 * STEPS_PER_MM;

/// 近端安全边距 (5mm)
const HOME_OFFSET: u32 = 5 * STEPS_PER_MM;

/// 有效行程 (45mm)
const EFFECTIVE_TRAVEL: u32 = MAX_TRAVEL - 2 * HOME_OFFSET;

// ============================================================================
// 辅助宏：StallGuard 近端归位 → 堵转 → INDEX 精确定位 → 回退
// ============================================================================

macro_rules! home_axis {
    ($axis:expr, $timer:expr, $label:expr) => {{
        info!("[{}] probing near-end (REV)...", $label);
        $axis.probe_endstop($timer, Direction::Reverse, RUN_SPEED_HZ);
        $axis.wait_done();
        $axis.set_position(0);
        info!("[{}] home=0, index pulses={}", $label, $axis.index_count());
    }};
}

// ============================================================================
// 辅助宏：归位 → 前移至预留位 → 设软限位 (校准标准步骤)
// ============================================================================

macro_rules! home_and_offset {
    ($axis:expr, $timer:expr, $label:expr, $offset:expr) => {{
        home_axis!($axis, $timer, $label);
        info!("[{}] moving {} steps to home offset...", $label, $offset);
        $axis.start_move($timer, $offset, Direction::Forward, RUN_SPEED_HZ);
        $axis.wait_done();
        $axis.set_position(0);
        $axis.set_max_travel(EFFECTIVE_TRAVEL as i32);
        info!(
            "[{}] positioned at offset, soft limits: [0, {}]",
            $label, EFFECTIVE_TRAVEL
        );
    }};
}

// ============================================================================
// 程序入口
// ============================================================================

#[main]
fn main() -> ! {
    info!("=== TMC2209 Multi-Axis (4 motors, 3 axes) ===");

    // --- 外设初始化 ---
    let dp = Peripherals::take().unwrap();
    let clocks = ClockBuilder::new().build(dp.CLOCKS, dp.XOSC, dp.PLL_SYS, dp.PLL_USB);

    let mut ticks = Ticks::init(dp.TICKS, &clocks);
    ticks.init_timer0();
    let timer = Timer0::init(dp.TIMER0);

    let pins = pico_rs::gpio::Pins::init(dp.IO_BANK0, dp.PADS_BANK0);

    // --- GPIO: ENABLE ---
    let mut _en = pins.pin::<PIN_ENABLE>();
    _en.into_output();
    _en.set_high();

    // --- GPIO: 各电机引脚 ---
    init_motor_pins!(pins, M1_DIR, M1_STEP, M1_INDEX, M1_DIAG);
    init_motor_pins!(pins, M2_DIR, M2_STEP, M2_INDEX, M2_DIAG);
    init_motor_pins!(pins, M3_DIR, M3_STEP, M3_INDEX, M3_DIAG);
    // 电机 4 的 INDEX/DIAG 只初始化为输入 (防浮空)
    {
        let mut _i4 = pins.pin::<20>();
        _i4.into_input_pullup();
        let mut _g4 = pins.pin::<21>();
        _g4.into_input_pullup();
    }
    // 注意: 电机 4 的 DIR/STEP 与电机 3 共用 GPIO16/17，不重复初始化

    // --- UART ---
    let mut uart = Uart1::init(
        dp.UART1,
        pins.pin::<4>(),
        pins.pin::<5>(),
        TMC_UART_BAUD,
        clocks.peri_freq(),
    );
    uart.set_format(DataBits::Eight, StopBits::One, Parity::None);
    timer.delay_ms(100);

    // --- 初始化 4 个 TMC2209 ---
    let _tmc1 = tmc_init!(&uart, M1_ADDR, &timer, "M1");
    let _tmc2 = tmc_init!(&uart, M2_ADDR, &timer, "M2");
    let _tmc3 = tmc_init!(&uart, M3_ADDR, &timer, "M3");
    let _tmc4 = tmc_init!(&uart, M4_ADDR, &timer, "M4");

    // --- 中断 ---
    unsafe {
        interrupt::enable(Interrupt::TIMER0_IRQ_0);
        interrupt::enable(Interrupt::TIMER0_IRQ_1);
        interrupt::enable(Interrupt::TIMER0_IRQ_2);
    }

    // --- 启用全部电机 ---
    unsafe { gpio::set_low(PIN_ENABLE) };
    timer.delay_ms(100);

    // ======================================================================
    // 行程校准 — 按干扰避免序列进行
    //
    // M2/M3 行程存在干涉，必须按下列顺序归位：
    //   1. M1 归位 → 移到预留位 → 设软限位
    //   2. M2 归位 → 移到预留位 → 设软限位
    //   3. M3 归位 → 移到预留位 → 设软限位
    //   4. M2 重新归位 (修正 M3 运动可能引入的偏差)
    // ======================================================================

    info!("--- Calibrating all axes (interference-aware sequence) ---");

    // Step 1: M1 近端归位 → 前移 5mm → position=0, max_travel=45mm
    home_and_offset!(AXIS1, &timer, "M1", HOME_OFFSET);
    timer.delay_ms(500);

    // Step 2: M2 近端归位 → 前移 5mm → position=0, max_travel=45mm
    home_and_offset!(AXIS2, &timer, "M2", HOME_OFFSET);
    timer.delay_ms(500);

    // Step 3: M3 近端归位 → 前移 5mm → position=0, max_travel=45mm
    home_and_offset!(AXIS3, &timer, "M3", HOME_OFFSET);
    timer.delay_ms(500);

    // Step 4: M2 重新归位 (修正 M3 运动可能引入的偏差)
    home_and_offset!(AXIS2, &timer, "M2", HOME_OFFSET);
    timer.delay_ms(500);

    info!(
        "calibration done: all axes at home, max_travel={}",
        EFFECTIVE_TRAVEL
    );
    timer.delay_ms(1000);

    // ======================================================================
    // 三轴并行连续运动到最大行程位置
    //
    // 各轴从当前位置 (0) 以连续运动模式向远端匀速运动。
    // 由于已设 max_travel = EFFECTIVE_TRAVEL，start_continuous 自动
    // 转为有限梯形运动到达限位，无需手动指定步数。
    // ======================================================================

    info!("--- Moving all axes to max travel (continuous) ---");

    AXIS1.start_continuous(&timer, Direction::Forward, RUN_SPEED_HZ);
    AXIS2.start_continuous(&timer, Direction::Forward, RUN_SPEED_HZ);
    AXIS3.start_continuous(&timer, Direction::Forward, RUN_SPEED_HZ);

    AXIS1.wait_done();
    AXIS2.wait_done();
    AXIS3.wait_done();

    info!(
        "all axes at max travel: M1={}, M2={}, M3={}",
        AXIS1.position(),
        AXIS2.position(),
        AXIS3.position(),
    );
    timer.delay_ms(1000);

    // ======================================================================
    // 三轴并行返回近端 (测试反向连续运动)
    // ======================================================================

    info!("--- Moving all axes back to home (continuous) ---");

    AXIS1.start_continuous(&timer, Direction::Reverse, RUN_SPEED_HZ);
    AXIS2.start_continuous(&timer, Direction::Reverse, RUN_SPEED_HZ);
    AXIS3.start_continuous(&timer, Direction::Reverse, RUN_SPEED_HZ);

    AXIS1.wait_done();
    AXIS2.wait_done();
    AXIS3.wait_done();

    info!(
        "all axes back home: M1={}, M2={}, M3={}",
        AXIS1.position(),
        AXIS2.position(),
        AXIS3.position(),
    );
    timer.delay_ms(1000);

    // ======================================================================
    // 三轴并行随机往返 + 丢步检测
    // ======================================================================

    info!("--- Parallel random move stress test ---");

    let trng = Trng::init(dp.TRNG);

    let max_steps = EFFECTIVE_TRAVEL as u32;

    let mut iteration: u32 = 0;
    let mut stalls: [u32; 3] = [0; 3];
    let mut misses: [u32; 3] = [0; 3];

    // 统计随机数错误次数 (极罕见)
    let mut trng_errors: u32 = 0;

    loop {
        iteration += 1;

        // 使用硬件 TRNG 为每个轴生成真随机目标位置
        // random_range(max) 返回 [0, max)，+1 得到 [0, max_steps]
        let target1 = match trng.random_range(max_steps + 1) {
            Ok(v) => v as i32,
            Err(_) => {
                trng_errors += 1;
                continue;
            }
        };
        let target2 = match trng.random_range(max_steps + 1) {
            Ok(v) => v as i32,
            Err(_) => {
                trng_errors += 1;
                continue;
            }
        };
        let target3 = match trng.random_range(max_steps + 1) {
            Ok(v) => v as i32,
            Err(_) => {
                trng_errors += 1;
                continue;
            }
        };

        let pos1 = AXIS1.position();
        let pos2 = AXIS2.position();
        let pos3 = AXIS3.position();

        let (steps1, dir1) = move_params(pos1, target1);
        let (steps2, dir2) = move_params(pos2, target2);
        let (steps3, dir3) = move_params(pos3, target3);

        info!(
            "[{}] M1: {}→{} ({}), M2: {}→{} ({}), M3: {}→{} ({})",
            iteration, pos1, target1, steps1, pos2, target2, steps2, pos3, target3, steps3,
        );

        // 三轴同时启动运动 (非阻塞 — 由各自 ISR 驱动脉冲)
        if steps1 > 0 {
            AXIS1.start_move(&timer, steps1, dir1, RUN_SPEED_HZ);
        }
        if steps2 > 0 {
            AXIS2.start_move(&timer, steps2, dir2, RUN_SPEED_HZ);
        }
        if steps3 > 0 {
            AXIS3.start_move(&timer, steps3, dir3, RUN_SPEED_HZ);
        }

        // 等待三轴全部完成
        AXIS1.wait_done();
        AXIS2.wait_done();
        AXIS3.wait_done();

        // ---- 三轴诊断 ----

        // StallGuard 堵转检测
        if AXIS1.stall_occurred() {
            stalls[0] += 1;
            warn!("[{}] M1 STALL! (total: {})", iteration, stalls[0]);
        }
        if AXIS2.stall_occurred() {
            stalls[1] += 1;
            warn!("[{}] M2 STALL! (total: {})", iteration, stalls[1]);
        }
        if AXIS3.stall_occurred() {
            stalls[2] += 1;
            warn!("[{}] M3 STALL! (total: {})", iteration, stalls[2]);
        }

        // INDEX 丢步检测
        if steps1 > 0 {
            let idx = AXIS1.check_index(steps1, MICROSTEPPING);
            if !idx.is_accurate(1) {
                misses[0] += 1;
                warn!(
                    "[{}] M1 INDEX miss: exp={} got={} (misses: {})",
                    iteration, idx.expected, idx.actual, misses[0]
                );
            }
        }
        if steps2 > 0 {
            let idx = AXIS2.check_index(steps2, MICROSTEPPING);
            if !idx.is_accurate(1) {
                misses[1] += 1;
                warn!(
                    "[{}] M2 INDEX miss: exp={} got={} (misses: {})",
                    iteration, idx.expected, idx.actual, misses[1]
                );
            }
        }
        if steps3 > 0 {
            let idx = AXIS3.check_index(steps3, MICROSTEPPING);
            if !idx.is_accurate(1) {
                misses[2] += 1;
                warn!(
                    "[{}] M3 INDEX miss: exp={} got={} (misses: {})",
                    iteration, idx.expected, idx.actual, misses[2]
                );
            }
        }

        // ---- 每 100 次迭代输出汇总 ----
        if iteration.is_multiple_of(100) {
            let s1 = stalls[0] * 1000 / iteration;
            let s2 = stalls[1] * 1000 / iteration;
            let s3 = stalls[2] * 1000 / iteration;
            let m1 = misses[0] * 1000 / iteration;
            let m2 = misses[1] * 1000 / iteration;
            let m3 = misses[2] * 1000 / iteration;
            info!("--- stats @ {} iters ---", iteration);
            info!(
                "  stalls: M1={}.{}% M2={}.{}% M3={}.{}%",
                s1 / 10,
                s1 % 10,
                s2 / 10,
                s2 % 10,
                s3 / 10,
                s3 % 10
            );
            info!(
                "  misses: M1={}.{}% M2={}.{}% M3={}.{}%",
                m1 / 10,
                m1 % 10,
                m2 / 10,
                m2 % 10,
                m3 / 10,
                m3 % 10
            );
            if trng_errors > 0 {
                info!("  trng_errors: {}", trng_errors);
            }
        }

        timer.delay_ms(200);
    }
}

// ============================================================================
// 辅助函数：计算运动参数
// ============================================================================

fn move_params(current: i32, target: i32) -> (u32, Direction) {
    let diff = target - current;
    let steps = diff.unsigned_abs();
    let dir = if diff >= 0 {
        Direction::Forward
    } else {
        Direction::Reverse
    };
    (steps, dir)
}

// ============================================================================
// TIMER0 Alarm 中断服务例程 — 每个轴独立 ISR
// ============================================================================

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
fn TIMER0_IRQ_0() {
    let t = timer();
    AXIS1.handle_alarm_isr(&t);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
fn TIMER0_IRQ_1() {
    let t = timer();
    AXIS2.handle_alarm_isr(&t);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
fn TIMER0_IRQ_2() {
    let t = timer();
    AXIS3.handle_alarm_isr(&t);
}
