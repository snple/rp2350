//! # A4988 滑台高速往返性能测试
//!
//! 基于 RP2350 (Raspberry Pi Pico 2) + A4988 驱动，实现直线滑台的
//! **高速往返运动**测试程序。用于评估滑台在不同速度下的运行表现：
//! 是否丢步、振动、噪声、发热等。
//!
//! ---
//!
//! # 1. 测试流程
//!
//! 程序提供 **多轮递增速度** 的往返测试:
//!
//! 1. **预热** — 低速往返 2 次，确认接线和方向正确
//! 2. **速度扫描** — 从 `MIN_TEST_SPEED_HZ` 到 `MAX_TEST_SPEED_HZ`，
//!    以 `SPEED_STEP_HZ` 为步进，每个速度往返 `CYCLES_PER_SPEED` 次
//! 3. **极限测试** — 以最高配置速度持续往返，直到手动断电
//!
//! 每轮测试后打印统计信息：速度、实际步数、位置偏差（用于判断是否丢步）。
//!
//! ---
//!
//! # 2. 硬件接线
//!
//! 与 `stepper_a4988` 示例完全相同:
//!
//! | Pico GPIO | 功能 | A4988 引脚 |
//! |-----------|------|-----------|
//! | GP2 | STEP | STEP |
//! | GP3 | DIR | DIR |
//! | GP4 | ENABLE | ENABLE (低有效) |
//! | GP5 | MS1 | MS1 |
//! | GP6 | MS2 | MS2 |
//! | GP7 | MS3 | MS3 |
//! | GP8 | RESET | RESET (低有效) |
//! | GP9 | SLEEP | SLEEP (低有效) |
//!
//! ---
//!
//! # 3. 配置参数
//!
//! | 常量 | 默认值 | 说明 |
//! |------|--------|------|
//! | `TRAVEL_STEPS` | 3200 | 单程行程步数 (微步)，需根据滑台实际行程设置 |
//! | `MICROSTEPPING` | 16 | 微步细分 (1/2/4/8/16) |
//! | `START_SPEED_HZ` | 200 | 梯形加减速起步频率 |
//! | `MIN_TEST_SPEED_HZ` | 1000 | 速度扫描起始频率 |
//! | `MAX_TEST_SPEED_HZ` | 8000 | 速度扫描终止频率 |
//! | `SPEED_STEP_HZ` | 1000 | 速度扫描递增步长 |
//! | `ACCEL_STEPS` | 400 | 加速段步数 |
//! | `CYCLES_PER_SPEED` | 4 | 每个速度的往返次数 |
//! | `PAUSE_BETWEEN_MS` | 200 | 每次换向停顿时间 (ms) |
//!
//! ---
//!
//! # 4. 丢步检测原理
//!
//! 程序内部维护一个有符号绝对位置计数器 `POSITION`。
//! 每次完整往返后 (正转 N 步 + 反转 N 步)，理论位置应回到 0。
//! 如果 `POSITION != 0`，说明在该速度下发生了丢步。
//!
//! ---
//!
//! # 5. 编译运行
//!
//! ```bash
//! cd pico-rs-examples
//! cargo run --bin stepper_a4988_bench --target thumbv8m.main-none-eabihf
//! ```

#![no_std]
#![no_main]

#[path = "stepper_common.rs"]
mod stepper_common;
use stepper_common::{AxisConfig, AxisState, Direction};

use defmt::*;
use defmt_rtt as _;
use panic_probe as _;

use pico_rs::block::ImageDef;
use pico_rs::clocks::ClockBuilder;
use pico_rs::entry;
use pico_rs::gpio::{Gpio, Pins};
use pico_rs::interrupt;
use pico_rs::pac::{Interrupt, Peripherals};
use pico_rs::ticks::Ticks;
use pico_rs::timer::Timer0;

#[unsafe(link_section = ".start_block")]
#[used]
pub static IMAGE_DEF: ImageDef = ImageDef::exe();

// ============================================================================
// 测试配置参数 — 根据实际滑台调整
// ============================================================================

/// 单程行程总微步数
///
/// 示例: NEMA17 (200步/转) × 16微步 × 导程丝杠 (如2mm/转)
///   3200 微步 = 1 转 = 2mm 行程
///   若滑台行程 40mm → TRAVEL_STEPS = 3200 × 20 = 64000
///
/// **⚠ 务必根据实际滑台行程设置此值，过大会撞限位！**
const TRAVEL_STEPS: u32 = 3200 * 2;

/// 微步细分 (1/2/4/8/16)
const MICROSTEPPING: u32 = 16;

/// 梯形加减速起步频率 (Hz)
const START_SPEED_HZ: u32 = 200;

/// 速度扫描: 起始频率 (Hz)
const MIN_TEST_SPEED_HZ: u32 = 1000;

/// 速度扫描: 终止频率 (Hz)
const MAX_TEST_SPEED_HZ: u32 = 8000;

/// 速度扫描: 递增步长 (Hz)
const SPEED_STEP_HZ: u32 = 1000;

/// 加速段总步数 (从 START_SPEED_HZ → 目标速度)
const ACCEL_STEPS: u32 = 400;

/// STEP 脉冲高电平宽度 (μs)，A4988 要求 ≥1μs
const STEP_PULSE_US: u32 = 2;

/// 每个速度的往返次数
const CYCLES_PER_SPEED: u32 = 4;

/// 换向停顿时间 (ms) — 给滑台惯性一个停稳的时间
const PAUSE_BETWEEN_MS: u32 = 200;

// ============================================================================
// GPIO 引脚分配 (与 stepper_a4988 相同)
// ============================================================================

const PIN_STEP: u32 = 2;
const PIN_DIR: u32 = 3;
const PIN_ENABLE: u32 = 4;
const PIN_MS1: u32 = 5;
const PIN_MS2: u32 = 6;
const PIN_MS3: u32 = 7;
const PIN_RESET: u32 = 8;
const PIN_SLEEP: u32 = 9;

// ============================================================================
// 全局共享状态
// ============================================================================

/// 步进轴状态
static AXIS: AxisState = AxisState::new();

#[inline(always)]
fn gpio() -> Gpio {
    // SAFETY: all used pins are configured through `Pins` before runtime SIO
    // access. This helper only touches SIO data registers.
    unsafe { Gpio::new() }
}

// ============================================================================
// A4988 微步配置
// ============================================================================

fn set_microstepping(microsteps: u32) {
    let gpio = gpio();
    let (ms1, ms2, ms3) = match microsteps {
        1 => (false, false, false),
        2 => (true, false, false),
        4 => (false, true, false),
        8 => (true, true, false),
        16 => (true, true, true),
        _ => {
            warn!("不支持 1/{} 微步，使用全步", microsteps);
            (false, false, false)
        }
    };
    gpio.set(PIN_MS1 as u8, ms1);
    gpio.set(PIN_MS2 as u8, ms2);
    gpio.set(PIN_MS3 as u8, ms3);
    info!("微步: 1/{}", microsteps);
}

// ============================================================================
// 运动控制
// ============================================================================

/// 启动梯形加减速运动
fn start_move(steps: u32, dir: u32, max_speed: u32) {
    let direction = if dir == 0 {
        Direction::Forward
    } else {
        Direction::Reverse
    };
    let timer = unsafe { Timer0::steal() };
    AXIS.start_move(&timer, steps, direction, max_speed);
}

/// 阻塞等待运动完成
fn wait_done() {
    AXIS.wait_done();
}

// ============================================================================
// 入口
// ============================================================================

#[entry]
fn main() -> ! {
    info!("=== A4988 滑台高速往返性能测试 ===");

    // ------------------------------------------------------------------
    // 外设初始化
    // ------------------------------------------------------------------
    let dp = Peripherals::take().unwrap();
    let clocks = ClockBuilder::new().build(dp.CLOCKS, dp.XOSC, dp.PLL_SYS, dp.PLL_USB);
    info!("sys_clk = {} Hz", clocks.sys_freq());

    let ticks = Ticks::new(dp.TICKS, &clocks);
    ticks.init_timer0();
    let timer = Timer0::init(dp.TIMER0);

    let pins = Pins::new(dp.IO_BANK0, dp.PADS_BANK0);

    // ------------------------------------------------------------------
    // GPIO 配置
    // ------------------------------------------------------------------
    let p2 = pins.pin::<2>();
    p2.into_output();
    p2.set_low(); // STEP

    let p3 = pins.pin::<3>();
    p3.into_output();
    p3.set_low(); // DIR

    let p4 = pins.pin::<4>();
    p4.into_output();
    p4.set_high(); // ENABLE 先禁用

    let p5 = pins.pin::<5>();
    p5.into_output();
    p5.set_low(); // MS1
    let p6 = pins.pin::<6>();
    p6.into_output();
    p6.set_low(); // MS2
    let p7 = pins.pin::<7>();
    p7.into_output();
    p7.set_low(); // MS3

    let p8 = pins.pin::<8>();
    p8.into_output();
    p8.set_low(); // RESET (先复位)
    let p9 = pins.pin::<9>();
    p9.into_output();
    p9.set_low(); // SLEEP (先休眠)

    // ------------------------------------------------------------------
    // A4988 上电序列
    // ------------------------------------------------------------------
    // SLEEP → HIGH (唤醒)
        gpio().set_high(PIN_SLEEP as u8);
    timer.delay_ms(2);

    // RESET → HIGH (解除复位)
        gpio().set_high(PIN_RESET as u8);
    timer.delay_ms(1);

    // 微步配置
    set_microstepping(MICROSTEPPING);
    timer.delay_ms(1);

    // ------------------------------------------------------------------
    // 轴配置 + Timer 中断
    // ------------------------------------------------------------------
    AXIS.configure(&AxisConfig {
        pin_step: PIN_STEP as u8,
        pin_dir: PIN_DIR as u8,
        alarm: 0,
        start_speed_hz: START_SPEED_HZ,
        accel_steps: ACCEL_STEPS,
        step_pulse_us: STEP_PULSE_US,
    });

    unsafe {
        interrupt::enable(Interrupt::TIMER0_IRQ_0);
    }

    // 延时辅助闭包
    let delay_ms = |ms: u32| {
        timer.delay_ms(ms);
    };

    // 获取当前时间 (μs) 辅助闭包
    let now_us = || -> u32 { timer.get_time_us_32() };

    // 使能 A4988
        gpio().set_low(PIN_ENABLE as u8);
    info!("A4988 已使能");
    delay_ms(100);

    // ==================================================================
    // 阶段 1: 低速预热 — 确认方向和行程正确
    // ==================================================================
    info!("========================================");
    info!("  阶段 1: 低速预热 (500 Hz)");
    info!("========================================");

    let warmup_speed: u32 = 500;
    for i in 0..2u32 {
        info!("[预热 {}/2] 正转 {} 步", i + 1, TRAVEL_STEPS);
        start_move(TRAVEL_STEPS, 0, warmup_speed);
        wait_done();
        delay_ms(PAUSE_BETWEEN_MS);

        info!("[预热 {}/2] 反转 {} 步", i + 1, TRAVEL_STEPS);
        start_move(TRAVEL_STEPS, 1, warmup_speed);
        wait_done();
        delay_ms(PAUSE_BETWEEN_MS);

        let pos = AXIS.position();
        info!("[预热 {}/2] 位置偏差: {} 步", i + 1, pos);
    }

    let pos_after_warmup = AXIS.position();
    if pos_after_warmup != 0 {
        warn!(
            "⚠ 预热后位置偏差 = {}，可能已丢步或接线有误！",
            pos_after_warmup
        );
    } else {
        info!("✓ 预热完成，位置偏差 = 0");
    }
    delay_ms(500);

    // ==================================================================
    // 阶段 2: 速度递增扫描
    // ==================================================================
    info!("========================================");
    info!(
        "  阶段 2: 速度扫描 {} → {} Hz (步进 {})",
        MIN_TEST_SPEED_HZ, MAX_TEST_SPEED_HZ, SPEED_STEP_HZ
    );
    info!("========================================");

    AXIS.set_position(0);

    let mut speed = MIN_TEST_SPEED_HZ;
    while speed <= MAX_TEST_SPEED_HZ {
        info!("--- 测试速度: {} Hz ---", speed);

        let mut total_steps_fwd: u32 = 0;
        let mut total_steps_rev: u32 = 0;
        let mut cycle_lost = false;

        // 记录本速度开始时间
        let t_start = now_us();

        for c in 0..CYCLES_PER_SPEED {
            // 正转
            start_move(TRAVEL_STEPS, 0, speed);
            wait_done();
            total_steps_fwd += AXIS.steps_done();
            delay_ms(PAUSE_BETWEEN_MS);

            // 反转
            start_move(TRAVEL_STEPS, 1, speed);
            wait_done();
            total_steps_rev += AXIS.steps_done();
            delay_ms(PAUSE_BETWEEN_MS);

            let pos = AXIS.position();
            if pos != 0 && !cycle_lost {
                warn!("  第 {} 轮出现位置偏差: {}", c + 1, pos);
                cycle_lost = true;
            }
        }

        let t_end = now_us();
        let elapsed_ms = t_end.wrapping_sub(t_start) / 1000;
        let pos = AXIS.position();

        info!(
            "  {} Hz | {}轮 | 正转{}步 反转{}步 | 偏差={} | 耗时 {}ms",
            speed, CYCLES_PER_SPEED, total_steps_fwd, total_steps_rev, pos, elapsed_ms,
        );

        if pos != 0 {
            warn!("  ✗ 丢步! 偏差 = {} 步", pos);
            // 发生丢步后归零位置继续测试下一速度
            AXIS.set_position(0);
        } else {
            info!("  ✓ 无丢步");
        }

        delay_ms(500);
        speed += SPEED_STEP_HZ;
    }

    // ==================================================================
    // 阶段 3: 极限速度持续往返
    // ==================================================================
    info!("========================================");
    info!("  阶段 3: 极限速度 {} Hz 持续往返", MAX_TEST_SPEED_HZ);
    info!("========================================");

    AXIS.set_position(0);
    let mut cycle: u32 = 0;

    loop {
        cycle += 1;

        // 正转
        start_move(TRAVEL_STEPS, 0, MAX_TEST_SPEED_HZ);
        wait_done();
        delay_ms(PAUSE_BETWEEN_MS);

        // 反转
        start_move(TRAVEL_STEPS, 1, MAX_TEST_SPEED_HZ);
        wait_done();
        delay_ms(PAUSE_BETWEEN_MS);

        let pos = AXIS.position();

        // 每 10 轮打印一次统计
        if cycle % 10 == 0 {
            info!(
                "[极限] 第 {} 轮 | {} Hz | 位置偏差={}",
                cycle, MAX_TEST_SPEED_HZ, pos
            );
            if pos != 0 {
                warn!("  ✗ 累计丢步 = {}", pos);
            }
        }
    }
}

// ============================================================================
// Timer0 ISR — STEP 脉冲生成 + 梯形加减速
// ============================================================================

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
fn TIMER0_IRQ_0() {
    let timer = unsafe { Timer0::steal() };
    AXIS.handle_alarm_isr(&timer);
}
