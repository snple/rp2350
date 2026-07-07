//! # TMC2209 堵转限位 — 以 StallGuard 替代物理限位器
//!
//! 电机沿一个方向持续运行，触碰末端 → 堵转触发 → 停止 → 停顿 → 反向运行，
//! 如此往复。利用 TMC2209 的 DIAG 堵转输出替代机械限位开关。
//!
//! ## 编译运行
//!
//! ```bash
//! cd pico-rs-examples
//! cargo run --bin stepper_stall_homing --target thumbv8m.main-none-eabihf
//! ```

#![no_std]
#![no_main]

use pico_rs_drivers::stepper::Direction;
use pico_rs_drivers::stepper::tmc2209::{AxisConfig, AxisState};
use pico_rs_drivers::tmc2209::Tmc2209;

use defmt::*;
use defmt_rtt as _;

use pico_rs::Steal;
use pico_rs::clocks::ClockBuilder;
use pico_rs::gpio::{self, Pins};
use pico_rs::interrupt;
use pico_rs::main;
use pico_rs::pac::{Interrupt, Peripherals};
use pico_rs::ticks::Ticks;
use pico_rs::timer::Timer0;
use pico_rs::uart::{DataBits, Parity, StopBits, Uart1}; // ============================================================================
// 参数
// ============================================================================

/// 堵转检测时的运行频率 (Hz)。不宜太快，避免撞击末端时损伤机械。
///
/// ╔═══════════════════════════════════════════════════════════════╗
/// ║  重要：SG_THRESHOLD 灵敏度受多因素影响                      ║
/// ╚═══════════════════════════════════════════════════════════════╝
///
/// StallGuard2 的 SG_RESULT 由多种因素共同决定，无法给出一组通用值：
///
///   • 运行速度 (本参数)       高速 → SG_RESULT ↑（BEMF 增强）
///   • 电机电流 IRUN           大电流 → SG_RESULT ↑（信号更强）
///   • 驱动电压 VCC            高电压 → SG_RESULT ↑
///   • 电机型号/电感量         不同电机 SG_RESULT 差异巨大
///   • 机械负载                负载 ↑ → SG_RESULT ↓
///   • 微步细分数               影响 TSTEP 和 BEMF 采样时机
///
/// 因此每次更换电机、改变 STALL_SPEED_HZ、调整 IRUN 或 VCC 后，
/// 都必须依靠下方的诊断代码重新校准 SG_THRESHOLD，不可跨条件套用。
///
/// 校准方法：见下方诊断代码，启动后直接读日志输出即可。
const STALL_SPEED_HZ: u32 = 10000;

/// 加减速段步数 (start_continuous 不使用加减速，此处保留备用)
const ACCEL_STEPS: u32 = 200;

/// STEP 脉冲高电平宽度 (μs)
const STEP_PULSE_US: u32 = 3;

/// 微步细分数
///
/// TSTEP 与 DIAG 激活的关系（重要！）:
///
/// TSTEP 是 TMC2209 内部测量的 "两个 1/256 微步之间的时间"，
/// 单位是 fCLK 周期 (fCLK = 12MHz 内部时钟)。
/// TSTEP 始终以 1/256 微步为基准，与 CHOPCONF.mres 无关。
///
///   TSTEP = fCLK / (STEP_freq_Hz × 微步折算因子)
///
/// 其中 微步折算因子 = 256 / 实际微步数_per_全步。
/// 例如 16 微步 (mres=4): 因子 = 256/16 = 16。
/// 例如  4 微步 (mres=6): 因子 = 256/4  = 64。
///
/// 计算示例 (fCLK=12MHz, STEP=3000Hz):
///   16 微步: TSTEP = 12,000,000 / (3000 × 16) = 250
///    4 微步: TSTEP = 12,000,000 / (3000 × 64) = 62.5 → 62
///
/// DIAG 堵转输出激活条件: TSTEP ≤ TCOOLTHRS
/// configure() 默认写 TCOOLTHRS=100:
///   16 微步: 250 ≤ 100? ❌ DIAG 静默 → 必须手动设 TCOOLTHRS ≥ 250
///    4 微步:  62 ≤ 100? ✓ DIAG 正常
///
/// 因此 16 微步时调用 set_tcool_threshold(0xFFFFF) 强制全速激活。
const MICROSTEPPING: u32 = 16;

/// 堵转后停顿时间 (ms)
const STALL_PAUSE_MS: u32 = 500;

/// 堵转后退离末端的距离（步数），防止皮带弹跳反复触发
const BACKOFF_STEPS: u32 = 400;

/// 退离时的速度 (Hz)
const BACKOFF_SPEED_HZ: u32 = 1000;

/// 运行电流 IRUN (0–31)。堵转限位应偏低，让电机容易堵转。
///
///   IRUN │  4   6   8  10  12  14  16  18  20
///   ─────┼─────────────────────────────────────
///   mA   │ 280 390 500 610 720 830 940 1.0 1.2
///
/// StallGuard 的信号质量与 IRUN 正相关。IRUN 太低时：
///   - SG_RESULT 信噪比差，数值异常偏低
///   - 可能出现自由运转 SG < 堵转 SG 的"倒挂"现象
///
/// 建议 IRUN 至少 ≥ 10，让 BEMF 信号足够强。
const RUN_CURRENT: u8 = 12;

/// 保持电流 IHOLD (0–31)
const HOLD_CURRENT: u8 = 2;

/// StallGuard 堵转检测阈值 (0–255)。越大越灵敏。
/// 触发条件: SG_RESULT ≤ SGTHRS × 2
///
/// ╔═══════════════════════════════════════════════════════════════╗
///
///  ⚠ SG_RESULT 受速度、电流、电压、电机型号、负载共同影响，
///     同一台电机改一个条件 SG_RESULT 就可能变 2~5 倍。
///
///  ⚠ 每次改变 STALL_SPEED_HZ、IRUN、电源电压或电机后，
///     都必须靠下方诊断日志重新校准，不可借用旧值。
///
/// ╚═══════════════════════════════════════════════════════════════╝
///
/// 快速校准流程（启动后看日志）：
///   step 1: 诊断自动输出 自由运转 SG_RESULT (min/max)
///   step 2: 用手堵转，诊断输出 堵转 SG_RESULT (min)
///   step 3: SGTHRS = 堵转 min / 2 + 5
///   step 4: 确认 自由运转 min > SGTHRS × 2（否则降低 SGTHRS）
///
/// 初始值 80 较为保守，在 10 kHz / IRUN=12 下大概率需要调低。
const SG_THRESHOLD: u32 = 80;

/// UART 波特率
const TMC_UART_BAUD: u32 = 115_200;

// ============================================================================
// 引脚定义
// ============================================================================

const PIN_ENABLE: u32 = 7;

// #1 (电机 1: GP8–11, addr 0x00)
const TMC_SLAVE_ADDR: u8 = 0x00;
const PIN_DIR: u32 = 8;
const PIN_STEP: u32 = 9;
const PIN_INDEX: u32 = 10;
const PIN_DIAG: u32 = 11;

// ============================================================================
// 步进轴状态
// ============================================================================

static AXIS: AxisState = AxisState::new(AxisConfig {
    pin_step: PIN_STEP,
    pin_dir: PIN_DIR,
    alarm: 0,
    start_speed_hz: 500,
    accel_steps: ACCEL_STEPS,
    step_pulse_us: STEP_PULSE_US,
    diag_pin: PIN_DIAG,
    index_pin: PIN_INDEX,
    index_backoff: 0,
    fwd_level: false,
    blanking_steps: 200,
});

#[inline(always)]
fn timer() -> Steal<Timer0> {
    unsafe { Timer0::steal() }
}

// ============================================================================
// 程序入口
// ============================================================================

#[main]
fn main() -> ! {
    info!("=== TMC2209 Stall Homing ===");

    // --- 外设初始化 ---
    let dp = Peripherals::take().unwrap();
    let clocks = ClockBuilder::new().build(dp.CLOCKS, dp.XOSC, dp.PLL_SYS, dp.PLL_USB);

    let ticks = Ticks::new(dp.TICKS, &clocks);
    ticks.init_timer0();

    let timer = Timer0::init(dp.TIMER0);
    let pins = Pins::new(dp.IO_BANK0, dp.PADS_BANK0);

    // --- GPIO ---
    let _en = pins.pin::<PIN_ENABLE>();
    _en.into_output();
    unsafe { gpio::set_high(PIN_ENABLE) };

    let _dir = pins.pin::<PIN_DIR>();
    _dir.into_output();
    unsafe { gpio::set_low(PIN_DIR) };

    let _step = pins.pin::<PIN_STEP>();
    _step.into_output();
    unsafe { gpio::set_low(PIN_STEP) };

    let _index = pins.pin::<PIN_INDEX>();
    _index.into_input_pullup();

    let p_diag = pins.pin::<PIN_DIAG>();
    p_diag.into_input_pullup();

    // --- UART ---
    let uart = Uart1::new(
        dp.UART1,
        pins.pin::<4>(),
        pins.pin::<5>(),
        TMC_UART_BAUD,
        clocks.peri_freq(),
    );
    uart.set_format(DataBits::Eight, StopBits::One, Parity::None);
    timer.delay_ms(100);

    // --- 配置 TMC2209 ---
    let tmc = Tmc2209::new(&uart, TMC_SLAVE_ADDR);

    match tmc.read_ioin(&timer) {
        Some(v) => info!("Chip addr={:#x} ver={:#04x} ✓", TMC_SLAVE_ADDR, v.version()),
        None => {
            warn!("TMC2209 not responding — halt");
            loop {
                pico_rs_rt::asm::wfi();
            }
        }
    }

    tmc.configure(
        &timer,
        MICROSTEPPING,
        RUN_CURRENT,
        HOLD_CURRENT,
        10,
        SG_THRESHOLD,
    );
    info!(
        "Configured: 1/{} μstep, IRUN={}, IHOLD={}, SGTHRS={}",
        MICROSTEPPING, RUN_CURRENT, HOLD_CURRENT, SG_THRESHOLD,
    );

    if let Some(sg) = tmc.read_sg_threshold(&timer) {
        info!("SGTHRS readback={} (stall if SG_RESULT ≤ {})", sg, sg * 2);
    }

    // 16 微步下 TSTEP=250 > TCOOLTHRS=100，DIAG 被禁用。设 max 使其全速激活。
    tmc.set_tcool_threshold(0xFFFFF, &timer);

    // 关闭 VACTUAL 模式 — 防止上次 UART 控速残留导致电机自转
    tmc.stop_vactual(&timer);

    // --- 中断 ---
    unsafe {
        interrupt::enable(Interrupt::TIMER0_IRQ_0);
    }

    // --- 使能 ---
    unsafe { gpio::set_low(PIN_ENABLE) };
    timer.delay_ms(100);

    // ======================================================================
    // 快速诊断：采样 SG_RESULT 辅助调阈值
    //
    // ╔═══════════════════════════════════════════════════════════════╗
    // ║  诊断速度 = STALL_SPEED_HZ，确保阈值对标实际工况           ║
    // ╚═══════════════════════════════════════════════════════════════╝
    //
    // StallGuard2 的 SG_RESULT 随速度变化（BEMF 正比于转速）。
    // 如果诊断速度和主循环不同，读出来的 SG_RESULT 对调阈值毫无意义。
    //
    // 预期输出解读:
    //
    //   SG_RESULT free-run: min=XXX max=YYY
    //     → 自由运转时的 SG_RESULT 范围。阈值必须 >= min，否则误触发。
    //
    //   SG_RESULT during stall: min=ZZZ
    //     → 堵转时的 SG_RESULT。阈值必须介于 ZZZ/2 ~ min/2 之间。
    //
    //   ⚠ SGTHRS=XX 太灵敏!
    //     → SGTHRS × 2 ≥ min，自由运转就会触发 DIAG，需要降低 SGTHRS。
    //
    //   ⚠ 堵转时 SG=ZZZ > 阈值 NNN! SGTHRS 太低
    //     → 堵转也达不到阈值，需要提高 SGTHRS。
    // ======================================================================
    info!("Diag: quick SG_RESULT sample @ {} Hz...", STALL_SPEED_HZ);
    AXIS.start_continuous(&timer, Direction::Forward, STALL_SPEED_HZ);
    timer.delay_ms(300);

    let mut sg_min: u16 = 65535;
    let mut sg_max: u16 = 0;
    for _ in 0..5 {
        if let Some(sg) = tmc.read_sg_result(&timer) {
            sg_min = sg_min.min(sg);
            sg_max = sg_max.max(sg);
        }
        timer.delay_ms(150);
    }
    AXIS.stop();
    timer.delay_ms(100);

    let suggested = (sg_min as u32).saturating_mul(4) / 10;
    info!(
        "SG_RESULT free-run: min={} max={}  → 建议 SGTHRS={} (当前={})",
        sg_min, sg_max, suggested, SG_THRESHOLD,
    );
    if SG_THRESHOLD as u16 * 2 >= sg_min {
        warn!("⚠ SGTHRS={} 太灵敏! 正常运转就会触发", SG_THRESHOLD);
    }

    // 尝试堵转采样：用手抓停电机，看 SG_RESULT 实际降到多少
    info!("Diag: STALL the motor NOW (block by hand) to sample stall SG_RESULT...");
    AXIS.start_continuous(&timer, Direction::Forward, STALL_SPEED_HZ);
    timer.delay_ms(500); // 给时间用手堵住

    let mut stall_sg: u16 = 65535;
    for _ in 0..10 {
        if let Some(sg) = tmc.read_sg_result(&timer) {
            stall_sg = stall_sg.min(sg);
            if sg <= SG_THRESHOLD as u16 * 2 {
                info!("  SG={}  ≤  {}  ← 堵转触发!", sg, SG_THRESHOLD as u16 * 2);
            }
        }
        timer.delay_ms(100);
    }
    AXIS.stop();
    timer.delay_ms(100);

    if stall_sg < 65535 {
        let optimal = (stall_sg as u32 / 2).saturating_add(5);
        info!(
            "SG_RESULT during stall: min={}  → 建议 SGTHRS={} (当前={})",
            stall_sg, optimal, SG_THRESHOLD,
        );
        if stall_sg > SG_THRESHOLD as u16 * 2 {
            warn!(
                "⚠ 堵转时 SG={} > 阈值 {}! SGTHRS 太低，DIAG 永远不会触发！",
                stall_sg,
                SG_THRESHOLD as u16 * 2,
            );
        }
    } else {
        warn!("未能读取堵转 SG_RESULT，请检查 UART 连接");
    }

    // ======================================================================
    // 堵转限位往返
    // ======================================================================

    info!(
        "Stall speed: {} Hz, pause: {} ms, backoff: {} steps @ {} Hz",
        STALL_SPEED_HZ, STALL_PAUSE_MS, BACKOFF_STEPS, BACKOFF_SPEED_HZ,
    );
    info!("Touch the end-stop or block the motor to trigger stall → reverse.");

    let mut direction = Direction::Forward;
    let mut lap: u32 = 0;

    loop {
        lap += 1;
        info!(
            "[lap {}] {} moving @ {} Hz ...",
            lap,
            direction.as_str(),
            STALL_SPEED_HZ,
        );

        AXIS.start_continuous(&timer, direction, STALL_SPEED_HZ);
        AXIS.wait_done();

        // 堵转触发后 Timer ISR 检测到 DIAG → stop + motion_done=true → 到达此处
        let pos = AXIS.position();
        info!(
            "[lap {}] STALL at pos={} — back off {} steps @ {} Hz",
            lap, pos, BACKOFF_STEPS, BACKOFF_SPEED_HZ,
        );

        // 反向低速退离末端，防止皮带回弹反复触发。
        //
        // TMC2209 的 DIAG 是锁存信号：一旦堵转触发，DIAG 保持 HIGH
        // 直到 ENN 拉高（禁用驱动器）才能复位。因此退离时必须临时
        // 关闭堵转检测（disable_stall_check），退离完成后再循环 ENN
        // 清除 DIAG 锁存。
        let back_dir = match direction {
            Direction::Forward => Direction::Reverse,
            Direction::Reverse => Direction::Forward,
        };

        AXIS.disable_stall_check();
        AXIS.start_move(&timer, BACKOFF_STEPS, back_dir, BACKOFF_SPEED_HZ);
        AXIS.wait_done();

        info!(
            "[lap {}] backoff done  pos={} — pause {} ms",
            lap,
            AXIS.position(),
            STALL_PAUSE_MS
        );
        timer.delay_ms(STALL_PAUSE_MS);

        // 折返
        direction = back_dir;
    }
}

// ============================================================================
// ISR
// ============================================================================

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
fn TIMER0_IRQ_0() {
    let t = timer();
    AXIS.handle_alarm_isr(&t);
}
