//! # TMC2209 行程校准 — 双端探测 + 软限位 + INDEX 丢步检测
//!
//! ## 流程
//!
//! ```text
//! 1. probe_endstop(REV) → set_position(0)    // 归位近端
//! 2. probe_endstop(FWD) → 读 position()      // 探测远端，得 travel
//! 3. set_soft_limits(0, travel)              // 设置软限位保护
//! 4. 往返验证 + INDEX 丢步检测                // 安全范围内自由运动
//! ```
//!
//! ## 编译运行
//!
//! ```bash
//! cd pico-rs-examples
//! cargo run --bin stepper_homing --target thumbv8m.main-none-eabihf
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
use pico_rs::uart::{DataBits, Parity, StopBits, Uart1};

// ============================================================================
// 运动参数
// ============================================================================

/// 探测时的目标速度 (Hz)，即每秒脉冲数。
/// 10 kHz = 10,000 pulse/s，在 1/16 微步下对应约 31.25 full-step/s。
const PROBE_SPEED_HZ: u32 = 10000;

/// 加减速距离 (步数)。从零加速到目标速度所需的步数，
/// 值越大加速越柔和。设为较小值以缩短加速段，
/// 配合略大于它的 blanking_steps 覆盖加速+沉降。
const ACCEL_STEPS: u32 = 50;

/// STEP 引脚脉冲宽度 (μs)。TMC2209 要求 STEP 高电平至少 100ns，
/// 这里取 3μs 留有充足余量，确保可靠触发。
const STEP_PULSE_US: u32 = 3;

/// 微步细分设置。16 表示 1/16 微步，即一个全步分为 16 个微步。
/// TMC2209 在 UART 模式下支持 1/256，这里选 16 兼顾精度与力矩。
const MICROSTEPPING: u32 = 16;

// ============================================================================
// TMC2209 驱动参数
// ============================================================================

/// 运行电流 (IRUN)，单位约为 1/32 A (RMS)。
/// 12 表示约 0.375A RMS。根据电机规格和散热条件调整。
const RUN_CURRENT: u8 = 12;

/// 保持电流 (IHOLD)，电机静止时的电流。
/// 2 表示约 0.0625A RMS，远低于运行电流，避免长时间静止过热。
const HOLD_CURRENT: u8 = 2;

/// StallGuard 阈值。TMC2209 通过反电动势检测堵转，
/// 当 SG 值低于此阈值时判定堵转 (DIAG 引脚拉低)。
/// 值越大越敏感，典型范围 0-255，0 表示禁用。
const SG_THRESHOLD: u32 = 60;

/// TMC2209 UART 通信波特率。默认 115200，需与芯片 OTP 或硬件配置一致。
const TMC_UART_BAUD: u32 = 115_200;

/// INDEX 回退步数。检测到 INDEX 脉冲后反向移动的微步数，
/// 用于将电机停在 INDEX 信号的精确位置而非惯性滑过的位置。
const INDEX_BACKOFF: u32 = 4;

// ============================================================================
// 引脚分配 (RP2350 GPIO)
// ============================================================================

/// 电机启用 (ENABLE)。低电平启用，高电平禁用。
/// 上电初始拉高以禁止输出，初始化完成后拉低启用。
const PIN_ENABLE: u32 = 7;

/// TMC2209 UART 从机地址。0x00 为默认广播地址，
/// 如果总线上有多个 TMC2209，需通过 OTP 或 MS1/MS2 引脚设置不同地址。
const TMC_SLAVE_ADDR: u8 = 0x00;

/// 方向控制 (DIR)。高/低电平决定电机正/反转。
const PIN_DIR: u32 = 8;

/// 步进脉冲 (STEP)。每个上升沿 (或下降沿，取决于 TMC2209 配置)
/// 驱动电机前进一个微步。
const PIN_STEP: u32 = 9;

/// INDEX 信号输入。TMC2209 每经过一个完整的电气周期
/// (4 个全步) 输出一个 INDEX 脉冲，用于精确位置校准和丢步检测。
const PIN_INDEX: u32 = 10;

/// DIAG 信号输入。TMC2209 的多功能诊断输出，
/// 在 StallGuard 模式下用于指示堵转 (低电平 = 堵转)。
const PIN_DIAG: u32 = 11;

/// 引脚操作类型（ZST），编译期单态化零开销。
struct Pins;

impl AxisPins for Pins {
    #[inline]
    fn set_step(high: bool) {
        unsafe { gpioc::set(PIN_STEP, high) };
    }
    #[inline]
    fn set_dir(high: bool) {
        unsafe { gpioc::set(PIN_DIR, high) };
    }
}

impl TmcPins for Pins {
    #[inline]
    fn diag() -> bool {
        unsafe { gpioc::get(PIN_DIAG) }
    }
    #[inline]
    fn index() -> bool {
        unsafe { gpioc::get(PIN_INDEX) }
    }
}

// ============================================================================
// 全局步进轴实例
// ============================================================================

/// 全局 AxisState，ISR 和主循环共享。
static AXIS: AxisState<Pins> = AxisState::new(AxisConfig {
    alarm: 0,
    start_speed_hz: 500,
    accel_steps: ACCEL_STEPS,
    step_pulse_us: STEP_PULSE_US,
    index_backoff: INDEX_BACKOFF,
    fwd_level: true,
    blanking_steps: 80,
});

/// 从 ISR 中安全获取 Timer0 的借用。
/// 使用 `unsafe` + `Steal` 是 embedded Rust 在中断上下文获取全局外设的惯用方式。
#[inline(always)]
fn timer() -> Steal<Timer0> {
    unsafe { Timer0::steal() }
}

// ============================================================================
// main
// ============================================================================

#[main]
fn main() -> ! {
    info!("=== TMC2209 Travel Calibration ===");

    // --- 外设初始化 ---
    // 获取所有外设的单例所有权 (embedded Rust 的 singleton 模式)
    let dp = Peripherals::take().unwrap();

    // 配置系统时钟：外部晶振 → PLL_SYS → 各总线时钟
    let clocks = ClockBuilder::new().build(dp.CLOCKS, dp.XOSC, dp.PLL_SYS, dp.PLL_USB);

    // 初始化定时器 tick 发生器，为 TIMER0 提供微秒级时基
    let ticks = Ticks::new(dp.TICKS, &clocks);
    ticks.init_timer0();
    let timer = Timer0::init(dp.TIMER0);

    // 获取 GPIO 引脚集合
    let pins = pico_rs::gpio::Pins::new(dp.IO_BANK0, dp.PADS_BANK0);

    // --- 步进电机控制引脚 ---
    // ENABLE: 初始拉高 → 电机禁用 (防止上电抖动)
    let _en = pins.pin::<PIN_ENABLE>();
    _en.into_output();
    unsafe { gpio::set_high(PIN_ENABLE) };

    // DIR: 初始拉低，对应 AxisConfig 中 fwd_level=true 时的反方向
    let _dir = pins.pin::<PIN_DIR>();
    _dir.into_output();
    unsafe { gpio::set_low(PIN_DIR) };

    // STEP: 初始拉低，脉冲为上升沿触发
    let _step = pins.pin::<PIN_STEP>();
    _step.into_output();
    unsafe { gpio::set_low(PIN_STEP) };

    // INDEX: 输入 + 内部上拉，TMC2209 开漏输出，需上拉读取高电平
    let _index = pins.pin::<PIN_INDEX>();
    _index.into_input_pullup();

    // DIAG: 输入 + 内部上拉，TMC2209 开漏输出
    let p_diag = pins.pin::<PIN_DIAG>();
    p_diag.into_input_pullup();

    // --- TMC2209 UART 通信 ---
    // UART1 使用 GPIO4 (TX) 和 GPIO5 (RX)，波特率 115200
    // TMC2209 的 PDN_UART 引脚需接 GND 以启用 UART 模式
    let uart = Uart1::new(
        dp.UART1,
        pins.pin::<4>(),
        pins.pin::<5>(),
        TMC_UART_BAUD,
        clocks.peri_freq(),
    );
    // 8 数据位 / 1 停止位 / 无校验 — TMC2209 标准格式
    uart.set_format(DataBits::Eight, StopBits::One, Parity::None);
    timer.delay_ms(100);

    // --- TMC2209 初始化 ---
    // 创建 TMC2209 通信实例，通过 UART 与芯片交互
    let tmc = Tmc2209::new(&uart, TMC_SLAVE_ADDR);

    // 读取 IOIN 寄存器验证通信：如果成功则打印芯片版本号
    match tmc.read_ioin(&timer) {
        Some(v) => info!(
            "TMC2209 addr={:#x} ver={:#04x} ✓",
            TMC_SLAVE_ADDR,
            v.version()
        ),
        None => {
            // 通信失败：可能地址不对、接线错误或芯片未上电
            warn!("TMC2209 not responding — halt");
            loop {
                // WFI = Wait For Interrupt，低功耗等待
                pico_rs_rt::asm::wfi();
            }
        }
    }

    // 配置 TMC2209 运行参数：
    //   - 微步细分 1/16
    //   - 运行电流 12 (≈0.375A RMS)
    //   - 保持电流 2 (≈0.0625A RMS)
    //   - 保持延迟 10 (约 2^10 = 1024 个时钟后降至 IHOLD)
    //   - StallGuard 阈值 80
    tmc.configure(
        &timer,
        MICROSTEPPING,
        RUN_CURRENT,
        HOLD_CURRENT,
        10,
        SG_THRESHOLD,
    );
    info!(
        "1/{} μstep, IRUN={}, IHOLD={}, SGTHRS={}",
        MICROSTEPPING, RUN_CURRENT, HOLD_CURRENT, SG_THRESHOLD
    );

    // 禁用 TCOOL 阈值 (设为最大值)，防止运动过程中意外触发 CoolStep
    tmc.set_tcool_threshold(0xFFFFF, &timer);

    // 停止任何残留的 VACTUAL 速度模式，确保处于位置模式
    tmc.stop_vactual(&timer);

    // 启用 TIMER0 中断 (alarm 0 用于步进脉冲时序)
    unsafe {
        interrupt::enable(Interrupt::TIMER0_IRQ_0);
    }

    // 拉低 ENABLE → 电机启用，可以开始运动
    unsafe { gpio::set_low(PIN_ENABLE) };

    timer.delay_ms(100);

    // ======================================================================
    // Step 1: 探测近端限位 → 标记 home=0
    // ======================================================================
    // 原理：向 Reverse 方向运动，直到 DIAG 引脚被 StallGuard 拉低 (堵转)，
    // 然后检测 INDEX 脉冲精确定位，再回退 INDEX_BACKOFF 步停在 INDEX 位置。
    info!("--- Step 1: Probe near-end ---");

    // 启动堵转探测：以 PROBE_SPEED_HZ 的速度向 Reverse 方向运动，
    // 碰到机械限位后电机堵转 → SG 值下降 → DIAG 拉低 → 自动停止
    AXIS.probe_endstop(&timer, Direction::Reverse, PROBE_SPEED_HZ);

    // 阻塞等待探测完成 (DIAG 触发 + INDEX 定位 + 回退全部完成)
    AXIS.wait_done();

    // 将当前位置设置为逻辑原点 0
    AXIS.set_position(0);
    info!(
        "home=0 (INDEX backoff={}, pulses={})",
        INDEX_BACKOFF,
        AXIS.index_count()
    );

    // 等待 1 秒，确保机械稳定
    timer.delay_ms(1000);

    // ======================================================================
    // Step 2: 探测远端限位 → 获得有效行程
    // ======================================================================
    // 原理同上，但方向改为 Forward，从 home=0 运动到远端机械限位。
    info!("--- Step 2: Probe far-end ---");
    AXIS.probe_endstop(&timer, Direction::Forward, PROBE_SPEED_HZ);
    AXIS.wait_done();

    // 此时 AXIS.position() 即为从 home=0 到远端的微步总数
    let travel = AXIS.position();
    info!("far-end reached, travel = {} μsteps", travel);

    // 换算为全步数和大致物理行程 (假设 GT2 同步带 + 20 齿同步轮)
    // GT2 节距 2mm，20 齿 = 40mm/转，典型 1.8° 步进电机 = 200 全步/转
    // 所以 100 全步 ≈ 20mm ≈ 2.0mm 显示
    let fs = travel.unsigned_abs() / MICROSTEPPING;
    info!(
        "effective range: {} μsteps = {} full-steps ≈ {}.{} mm (GT2 20T)",
        travel,
        fs,
        fs / 100,
        (fs % 100) / 10
    );

    // ======================================================================
    // Step 3: 设置软限位保护
    // ======================================================================
    // 将探测到的行程设置为软件限位，防止后续运动超出机械范围造成碰撞。
    // set_max_travel 会限制所有 move_to / move_by 的目标位置在 [0, travel] 内。
    AXIS.set_max_travel(travel);
    info!("max travel = {} steps", travel);

    timer.delay_ms(1000);

    // ======================================================================
    // Step 4: 随机往返验证 — 丢步 / 堵转检测
    // ======================================================================
    // 反复随机选取行程内的目标位置并移动，
    // 每次运动结束后检查 INDEX 脉冲计数 (丢步) 和 DIAG 堵转标志。
    info!("--- Step 4: Random move stress test ---");

    let max_steps = travel.unsigned_abs();
    let mut rng = Lcg::new(0xDEAD_BEEF);
    let mut iteration: u32 = 0;
    let mut stall_count: u32 = 0;
    let mut miss_count: u32 = 0;

    loop {
        iteration += 1;

        // 在 [0, max_steps] 范围内随机选取目标位置
        let target: i32 = rng.next_in_range(0, max_steps) as i32;
        let current = AXIS.position();
        let diff = target - current;
        let steps = diff.unsigned_abs();
        let dir = if diff >= 0 {
            Direction::Forward
        } else {
            Direction::Reverse
        };

        info!(
            "[{}] {} → {} ({} steps, {})",
            iteration, current, target, steps, dir
        );

        if steps > 0 {
            AXIS.start_move(&timer, steps, dir, PROBE_SPEED_HZ);
            AXIS.wait_done();

            // --- 堵转检测 ---
            // stall_occurred 是锁存标志，start_move 会自动清零，
            // 如果运动结束后为 true，说明本次运动触发了 StallGuard。
            if AXIS.stall_occurred() {
                stall_count += 1;
                warn!(
                    "[{}] STALL detected! (total stalls: {})",
                    iteration, stall_count
                );
            }

            // --- INDEX 丢步检测 ---
            // 比较实际 INDEX 脉冲数与理论值，允许 ±1 脉冲容差。
            // INDEX 每 4 个全步 (4×MICROSTEPPING 微步) 产生一个脉冲，
            // 整数除法截断 + 起始电角度随机性导致边界 ±1 是正常的。
            let idx = AXIS.check_index(steps, MICROSTEPPING);
            if !idx.is_accurate(1) {
                miss_count += 1;
                let actual = idx.actual as i32;
                let expected = idx.expected as i32;
                warn!(
                    "[{}] INDEX miss: expected {} pulses, got {} (diff={}, total misses: {})",
                    iteration,
                    expected,
                    actual,
                    actual - expected,
                    miss_count
                );
            }
        } else {
            info!("[{}] skip (already at target)", iteration);
        }

        // 每 100 次迭代输出一次汇总统计
        // defmt 不支持浮点格式化，用整数模拟一位小数：x * 1000 / total → 整数部分.小数部分
        if iteration.is_multiple_of(100) {
            let stall_pct = stall_count * 1000 / iteration;
            let miss_pct = miss_count * 1000 / iteration;
            info!(
                "--- stats: {} iters, {} stalls ({}.{} %), {} misses ({}.{} %) ---",
                iteration,
                stall_count,
                stall_pct / 10,
                stall_pct % 10,
                miss_count,
                miss_pct / 10,
                miss_pct % 10
            );
        }

        // 短暂停顿，避免电机过热
        timer.delay_ms(200);
    }
}

// ============================================================================
// 简易线性同余伪随机数生成器 (LCG)
// ============================================================================

/// LCG 伪随机数生成器，用于 `#![no_std]` 环境。
/// 使用 Numerical Recipes 参数 (a=1664525, c=1013904223, m=2^32)。
struct Lcg(u32);

impl Lcg {
    fn new(seed: u32) -> Self {
        Self(seed)
    }

    fn next(&mut self) -> u32 {
        self.0 = self.0.wrapping_mul(1664525).wrapping_add(1013904223);
        self.0
    }

    /// 生成 [min, max] 区间的随机整数 (含两端)
    fn next_in_range(&mut self, min: u32, max: u32) -> u32 {
        if min >= max {
            return min;
        }
        min + (self.next() % (max - min + 1))
    }
}

// ============================================================================
// TIMER0 Alarm 0 中断服务例程 (ISR)
// ============================================================================

/// TIMER0_IRQ_0 是步进脉冲生成的核心 ISR。
/// 每次 alarm 0 触发时调用 `handle_alarm_isr`，该函数：
/// 1. 清除中断标志
/// 2. 根据当前运动状态翻转 STEP 引脚或完成脉冲
/// 3. 设置下一次 alarm 触发时间 (实现精确的脉冲频率控制)
/// 4. 处理加减速曲线
/// 5. 监控 DIAG (堵转) 和 INDEX 信号
#[unsafe(no_mangle)]
#[allow(non_snake_case)]
fn TIMER0_IRQ_0() {
    let t = timer();
    AXIS.handle_alarm_isr(&t);
}
