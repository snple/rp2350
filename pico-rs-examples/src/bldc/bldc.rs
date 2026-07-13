//! # 无传感器 BLDC 电调 — 开环 6 步换向示例
//!
//! 基于 RP2350 (Raspberry Pi Pico 2) 的无传感器三相无刷直流电机（BLDC）开环驱动，
//! 类似航模电调（ESC）。**无需霍尔传感器**，使用定时开环换向加渐进启动斜坡。
//!
//! **适用场景：** 快速验证硬件、简单低速旋转、不需要精确转速控制。
//! 闭环 BEMF + PI 转速控制版本见 `bldc_speed_ctrl.rs`。
//!
//! ---
//!
//! # 1. 三相无刷电机基本原理
//!
//! ## 1.1 电机结构
//!
//! BLDC 电机由**定子**（固定的电磁绕组，通常 3 相）和**转子**（永磁体）组成。
//! 通过依次给定子的不同相通电，产生旋转磁场，驱动转子跟随旋转。
//!
//! ```text
//!         ┌─────────────┐
//!         │   Phase A    │
//!         │    ┌───┐     │
//!         │    │ N │     │
//!    Phase B──►│   │◄── Phase C
//!         │    │ S │     │
//!         │    └───┘     │
//!         │   (rotor)    │
//!         └─────────────┘
//!           (stator)
//! ```
//!
//! ## 1.2 六步换向法（梯形驱动）
//!
//! 在任意时刻，三相中：
//! - **一相**通过 PWM 连接到电源正极（高侧导通） — 控制电流/力矩
//! - **一相**连接到地（低侧导通） — 提供电流回路
//! - **一相**悬空（浮空） — 用于反电动势检测（闭环版使用）
//!
//! 6 个换向步按顺序循环，每步跨越 60° 电角度，6 步 = 360° = 一个完整电周期：
//!
//! ```text
//!   Step │ Phase A │ Phase B │ Phase C │ 电流路径    │ 电角度
//!  ──────┼─────────┼─────────┼─────────┼────────────┼────────
//!    0   │  PWM ↑  │  GND ↓  │  Float  │  A → B     │  0°- 60°
//!    1   │  PWM ↑  │  Float  │  GND ↓  │  A → C     │  60°-120°
//!    2   │  Float  │  PWM ↑  │  GND ↓  │  B → C     │ 120°-180°
//!    3   │  GND ↓  │  PWM ↑  │  Float  │  B → A     │ 180°-240°
//!    4   │  GND ↓  │  Float  │  PWM ↑  │  C → A     │ 240°-300°
//!    5   │  Float  │  GND ↓  │  PWM ↑  │  C → B     │ 300°-360°
//! ```
//!
//! ## 1.3 电气转速与机械转速的关系
//!
//! ```text
//! 电周期 = 6 × 换向步周期
//! 电气转速 (ERPS) = 1 / 电周期
//! 机械转速 (RPM) = ERPS × 60 / 极对数
//! ```
//!
//! 例如：14 极电机（7 极对），换向步周期 = 2000 µs：
//! ```text
//! 电周期 = 6 × 2000 µs = 12000 µs = 0.012 s
//! ERPS = 1 / 0.012 = 83.3
//! RPM = 83.3 × 60 / 7 ≈ 714 RPM
//! ```
//!
//! ---
//!
//! # 2. 硬件电路
//!
//! ## 2.1 整体架构
//!
//! ```text
//!                           RP2350 (Pico 2)
//!                     ┌──────────────────────┐
//!                     │                      │
//!   RC Receiver ──────┤ GP6 (PWM Input)      │     ┌─────────────┐
//!   (可选油门输入)     │                      │     │  Gate Driver │
//!                     │ GP0 (PWM) ───────────────── AH ──→ FET ──┐
//!                     │ GP1 (GPIO) ──────────────── AL ──→ FET ──┤── Phase A
//!                     │                      │     └─────────────┘  │
//!                     │ GP2 (PWM) ───────────────── BH ──→ FET ──┤
//!                     │ GP3 (GPIO) ──────────────── BL ──→ FET ──┤── Phase B ─ Motor
//!                     │                      │     └─────────────┘  │
//!                     │ GP4 (PWM) ───────────────── CH ──→ FET ──┤
//!                     │ GP5 (GPIO) ──────────────── CL ──→ FET ──┘── Phase C
//!                     │                      │     └─────────────┘
//!                     └──────────────────────┘
//! ```
//!
//! ## 2.2 引脚分配
//!
//! | GPIO | 功能 | 外设 | 说明 |
//! |------|------|------|------|
//! | GP0 | Phase A 高侧 | PWM Slice 0 ChA | 20 kHz PWM 控制力矩 |
//! | GP1 | Phase A 低侧 | GPIO SIO 输出 | 开关地回路 |
//! | GP2 | Phase B 高侧 | PWM Slice 1 ChA | 20 kHz PWM |
//! | GP3 | Phase B 低侧 | GPIO SIO 输出 | 开关地回路 |
//! | GP4 | Phase C 高侧 | PWM Slice 2 ChA | 20 kHz PWM |
//! | GP5 | Phase C 低侧 | GPIO SIO 输出 | 开关地回路 |
//!
//! ## 2.3 功率级电路（每相半桥拓扑）
//!
//! ```text
//!     VBAT ─────────────────┐
//!                           │
//!                      ┌────┴────┐
//!                      │ Q_H     │  ← 高侧 N-MOSFET（需自举/电荷泵驱动）
//!       PWM ──→ Gate ──┤ (high)  │
//!                      │  Drain  │
//!                      └────┬────┘
//!                           │
//!                           ├──────── Phase Output ──→ 电机绕组
//!                           │
//!                      ┌────┴────┐
//!                      │ Q_L     │  ← 低侧 N-MOSFET（直接驱动）
//!       GPIO ──→ Gate ─┤ (low)   │
//!                      │  Source │
//!                      └────┬────┘
//!                           │
//!     GND ──────────────────┘
//! ```
//!
//! **推荐栅极驱动芯片：**
//!
//! | 芯片 | 类型 | 电压 | 特点 |
//! |------|------|------|------|
//! | IR2104 | 半桥驱动 | ≤600V | 经典方案，自举电路，低成本 |
//! | DRV8301 | 三相集成驱动 | ≤60V | SPI 配置，集成电流检测放大器 |
//! | FD6288 | 三相半桥驱动 | ≤250V | 低成本，适合航模 |
//!
//! > **⚠️ 安全警告：** 高侧 + 低侧同时导通会造成直通短路（shoot-through），
//! > 瞬间烧毁 MOSFET 和电源。栅极驱动器通常内置死区时间保护，软件层面也要确保
//! > 不会在同一相同时激活高低侧。本代码通过 6 步顺序表保证了这一点。
//!
//! ## 2.4 物料清单参考（12V / 10A 小型航模电机）
//!
//! | 物料 | 型号/规格 | 数量 |
//! |------|-----------|------|
//! | MCU | Raspberry Pi Pico 2 (RP2350A) | 1 |
//! | 栅极驱动 | IR2104 / FD6288 | 3 (单相) 或 1 (三相集成) |
//! | MOSFET | IRLZ44N / AOD4184 (Vds≥30V, Rds≤10mΩ) | 6 |
//! | 自举电容 | 10 µF / 25V 陶瓷 | 3 |
//! | 电源去耦 | 100 µF 电解 + 100 nF 陶瓷 | 各 1 |
//!
//! ---
//!
//! # 3. 核心架构总览
//!
//! | 模块 | 实现方式 | 说明 |
//! |------|---------|------|
//! | **换向逻辑** | `COMMUTATION_TABLE[6]` 查表 | 每步指定高侧 PWM / 低侧导通 / 浮空相 |
//! | **PWM 调速** | GP0/GP2/GP4 — PWM Slice 0/1/2 | 20 kHz，占空比控制力矩 |
//! | **低侧开关** | GP1/GP3/GP5 — GPIO SIO 输出 | 根据换向步开/关，提供电流回路 |
//! | **转子对齐** | Step 0 固定通电 500 ms | 将转子锁定到已知位置，确保首步有力矩 |
//! | **启动斜坡** | 定时器递减换向间隔 | 60 电周期，20 ms → 2 ms 线性加速 |
//! | **恒速运行** | 固定换向间隔 + 固定占空比 | 2 ms 步进，25% duty，无反馈 |
//! | **角度反馈** | 无 | 开环盲驱，不检测转子位置 |
//! | **速度反馈** | 无 | 无 PI 控制，转速取决于负载和电压 |
//!
//! ### 与其他驱动模式的对比
//!
//! | 区别点 | 开环 6 步（本文件） | BEMF 闭环 (`bldc_speed_ctrl.rs`) | 霍尔闭环 (`bldc_hall.rs`) |
//! |--------|-------------------|-------------------------------|-------------------------|
//! | 角度反馈 | 无 | BEMF 过零检测 (ADC) | 霍尔传感器 (GPIO 中断) |
//! | 速度控制 | 无（固定参数） | PI 闭环 | PI 闭环 |
//! | 启动方式 | 盲启动斜坡 | 开环斜坡 → BEMF 切换 | 直接霍尔换向 |
//! | 负载补偿 | 无 | 有 | 有 |
//! | 适用场景 | 硬件验证、轻载 | 航模、风扇 | 电动车、伺服 |
//!
//! ---
//!
//! # 4. 开环驱动工作流程
//!
//! ```text
//! ┌──────────────────────────────────────────────────────┐
//! │                    开环驱动流程                        │
//! ├──────────────────────────────────────────────────────┤
//! │                                                      │
//! │  ① 系统初始化                                         │
//! │    • 时钟 → 150 MHz                                   │
//! │    • PWM → 20 kHz, 3 个 slice                         │
//! │    • GPIO → 3 个低侧输出                               │
//! │    • Timer0 → 延时                                    │
//! │                                                      │
//! │  ② 转子对齐 (500 ms)                                  │
//! │    • 步骤 0 通电，低占空比                              │
//! │    • 转子锁定到已知位置                                 │
//! │                                                      │
//! │  ③ 开环加速斜坡                                       │
//! │    • 60 个电周期 (360 步)                              │
//! │    • 换向间隔: 20 ms → 2 ms (线性递减)                  │
//! │    • 固定 8% 占空比                                    │
//! │                                                      │
//! │  ④ 恒速运行                                           │
//! │    • 固定 25% 占空比                                   │
//! │    • 固定 2 ms 换向间隔                                │
//! │    • 无反馈，无负载补偿                                 │
//! │                                                      │
//! └──────────────────────────────────────────────────────┘
//! ```
//!
//! ---
//!
//! # 5. 参数说明
//!
//! | 参数 | 默认值 | 说明 |
//! |------|--------|------|
//! | `PWM_FREQ_HZ` | 20,000 | FET 开关频率 (Hz)，≥18kHz 不可闻 |
//! | `STARTUP_DUTY_PERCENT` | 8 | 启动占空比 (%)，低力矩避免失步 |
//! | `RUN_DUTY_PERCENT` | 25 | 运行占空比 (%)，控制输出转速 |
//! | `STARTUP_REVOLUTIONS` | 60 | 加速斜坡的电周期数 |
//! | `STARTUP_INTERVAL_US_MAX` | 20,000 | 初始换向间隔 (µs)，非常慢 |
//! | `STARTUP_INTERVAL_US_MIN` | 2,000 | 最终换向间隔 (µs)，加速完成 |
//! | `RUN_INTERVAL_US` | 2,000 | 恒速运行的换向间隔 (µs) |
//!
//! ## 针对不同电机的调参建议
//!
//! **小型航模电机 (2204/2206, 2300KV)：**
//! ```text
//! MOTOR_POLES = 14, STARTUP_DUTY = 8%, STARTUP_REVOLUTIONS = 40
//! ```
//!
//! **中型电机 (5010/5020, 360KV)：**
//! ```text
//! STARTUP_DUTY = 15%, STARTUP_REVOLUTIONS = 100, STARTUP_INTERVAL_US_MAX = 30000
//! ```
//!
//! **电动工具电机 (高速小极数)：**
//! ```text
//! MOTOR_POLES = 4, PWM_FREQ_HZ = 32000, RUN_INTERVAL_US = 1000
//! ```
//!
//! ---
//!
//! # 6. 局限性
//!
//! - **无负载补偿**：加载会导致减速甚至失步
//! - **无速度反馈**：实际转速取决于电压和负载
//! - **效率较低**：换向时机不与转子角度同步，力矩波动大
//! - **只适合轻载**：重载启动容易失步
//!
//! 需要闭环转速控制时请使用 `bldc_speed_ctrl.rs`，它通过 BEMF 过零检测 +
//! PI 控制器实现了负载自适应的稳定转速调节。
//!
//! ---
//!
//! # 7. 常见问题
//!
//! **Q: 电机不转/抖动**
//! 1. 检查换向表是否正确（高侧和低侧不能在同一相）
//! 2. 启动占空比是否足够（增大 `STARTUP_DUTY_PERCENT`）
//! 3. 启动是否太快（增大 `STARTUP_INTERVAL_US_MAX`）
//! 4. PWM 极性是否正确（高侧 PWM 高 = FET 导通？）
//! 5. 栅极驱动供电是否正常
//!
//! **Q: 电机高速时声音异常**
//! - PWM 频率在可听范围 → 确保 ≥ 18 kHz
//! - 换向间隔抖动 → 本开环版本无法消除，请用闭环版
//!
//! ---
//!
//! # 8. 编译运行
//!
//! ```bash
//! cd pico-rs-examples
//! cargo run --bin bldc --target thumbv8m.main-none-eabihf
//! ```
//!
//! ---
//!
//! # 9. 参考资料
//!
//! - [RP2350 Datasheet](https://datasheets.raspberrypi.com/rp2350/rp2350-datasheet.pdf)
//! - AN1078 (Microchip) — *Sensorless BLDC Motor Control Using Back-EMF Zero Crossing*
//! - AN885 (Microchip) — *Brushless DC (BLDC) Motor Fundamentals*
//! - TI SPRABQ7 — *Sensorless Trapezoidal Control of BLDC Motors*

#![no_std]
#![no_main]

use defmt::*;
use defmt_rtt as _;

use pico_rs::clocks::ClockBuilder;
use pico_rs::main;
use pico_rs::gpio::Pins;
use pico_rs::pac::Peripherals;
use pico_rs::pwm::Pwm;
use pico_rs::timer::Timer0;// ============================================================================
// Configuration
// ============================================================================

/// PWM frequency for the FET gate drive (20 kHz — inaudible)
const PWM_FREQ_HZ: u32 = 20_000;

/// Startup duty cycle (low torque to avoid stall, ~8%)
const STARTUP_DUTY_PERCENT: u16 = 8;

/// Running duty cycle (~25% throttle — adjust based on motor/load)
const RUN_DUTY_PERCENT: u16 = 25;

/// Number of full electrical revolutions for the startup ramp
const STARTUP_REVOLUTIONS: u32 = 60;

/// Initial commutation interval during startup (microseconds)
/// Start very slowly so the rotor can follow the rotating field
const STARTUP_INTERVAL_US_MAX: u32 = 20_000; // 50 Hz commutation → ~8.3 ERPS

/// Final commutation interval at end of startup ramp (microseconds)
const STARTUP_INTERVAL_US_MIN: u32 = 2_000; // 500 Hz commutation → ~83 ERPS

/// Commutation interval during normal running (microseconds)
/// For a 14-pole motor: 2000 µs → 500 comm/s → 500/3 ≈ 167 ERPS ≈ 2381 RPM
const RUN_INTERVAL_US: u32 = 2_000;

// ============================================================================
// 6-step commutation table
// ============================================================================

/// Which phase is driven high (PWM) at each commutation step
/// 0 = Phase A, 1 = Phase B, 2 = Phase C
const STEP_HIGH: [u8; 6] = [0, 0, 1, 1, 2, 2];

/// Which phase is driven low (GND) at each commutation step
const STEP_LOW: [u8; 6] = [1, 2, 2, 0, 0, 1];

// ============================================================================
// Entry
// ============================================================================

#[main]
fn main() -> ! {
    info!("BLDC ESC — sensorless open-loop 6-step commutation");

    // ------------------------------------------------------------------
    // Peripheral & clock init
    // ------------------------------------------------------------------
    let dp = Peripherals::take().unwrap();
    let clocks = ClockBuilder::new().build(dp.CLOCKS, dp.XOSC, dp.PLL_SYS, dp.PLL_USB);
    let sys_freq = clocks.sys_freq();
    info!("System clock: {} MHz", sys_freq / 1_000_000);

    let pins = Pins::init(dp.IO_BANK0, dp.PADS_BANK0);
    let timer = Timer0::init(dp.TIMER0);

    // ------------------------------------------------------------------
    // PWM setup — 20 kHz on slices 0, 1, 2 (channel A each)
    //   GPIO 0 → Slice 0 ChA (Phase A high-side)
    //   GPIO 2 → Slice 1 ChA (Phase B high-side)
    //   GPIO 4 → Slice 2 ChA (Phase C high-side)
    // ------------------------------------------------------------------
    let pwm_wrap: u16 = (sys_freq / PWM_FREQ_HZ) as u16;
    info!("PWM wrap = {} ({}  kHz)", pwm_wrap, PWM_FREQ_HZ / 1000);

    let pwm = Pwm::init(dp.PWM);

    // Phase A — PWM slice 0, channel A
    let slice_a = pwm.slice::<0>().output_a(pins.pin::<0>());
    slice_a.set_wrap(pwm_wrap);
    slice_a.set_duty_a(0);
    slice_a.enable();

    // Phase B — PWM slice 1, channel A
    let slice_b = pwm.slice::<1>().output_a(pins.pin::<2>());
    slice_b.set_wrap(pwm_wrap);
    slice_b.set_duty_a(0);
    slice_b.enable();

    // Phase C — PWM slice 2, channel A
    let slice_c = pwm.slice::<2>().output_a(pins.pin::<4>());
    slice_c.set_wrap(pwm_wrap);
    slice_c.set_duty_a(0);
    slice_c.enable();

    // ------------------------------------------------------------------
    // Low-side GPIO outputs
    //   GPIO 1 → Phase A low-side
    //   GPIO 3 → Phase B low-side
    //   GPIO 5 → Phase C low-side
    // ------------------------------------------------------------------
    let al = pins.pin::<1>();
    al.into_output();
    al.set_low();

    let bl = pins.pin::<3>();
    bl.into_output();
    bl.set_low();

    let cl = pins.pin::<5>();
    cl.into_output();
    cl.set_low();

    // ------------------------------------------------------------------
    // Helper closures
    // ------------------------------------------------------------------

    // Convert a duty percentage (0-100) to a compare value
    let duty_from_percent =
        |percent: u16| -> u16 { ((pwm_wrap as u32) * (percent as u32) / 100) as u16 };

    // Apply one commutation step:
    //   - Activate the high-side PWM for `high_phase`
    //   - Ground the low-side for `low_phase`
    //   - Float everything else
    let commutate = |step: usize, duty: u16| {
        let high = STEP_HIGH[step];
        let low = STEP_LOW[step];

        // --- High-side PWM duty ---
        slice_a.set_duty_a(if high == 0 { duty } else { 0 });
        slice_b.set_duty_a(if high == 1 { duty } else { 0 });
        slice_c.set_duty_a(if high == 2 { duty } else { 0 });

        // --- Low-side GPIO ---
        if low == 0 {
            al.set_high();
        } else {
            al.set_low();
        }
        if low == 1 {
            bl.set_high();
        } else {
            bl.set_low();
        }
        if low == 2 {
            cl.set_high();
        } else {
            cl.set_low();
        }
    };

    // Turn all outputs off (safe state)
    let _all_off = || {
        slice_a.set_duty_a(0);
        slice_b.set_duty_a(0);
        slice_c.set_duty_a(0);
        al.set_low();
        bl.set_low();
        cl.set_low();
    };

    // ------------------------------------------------------------------
    // Alignment pulse — lock rotor to a known electrical position
    // ------------------------------------------------------------------
    info!("Rotor alignment...");
    let align_duty = duty_from_percent(STARTUP_DUTY_PERCENT);
    commutate(0, align_duty);
    timer.delay_ms(500); // hold for 500 ms to let rotor settle

    // ------------------------------------------------------------------
    // Open-loop startup ramp
    //
    // Gradually decrease the commutation interval (= increase speed)
    // over STARTUP_REVOLUTIONS full electrical revolutions.
    // Each electrical revolution = 6 commutation steps.
    // ------------------------------------------------------------------
    info!(
        "Starting open-loop ramp ({} revolutions)...",
        STARTUP_REVOLUTIONS
    );
    let startup_duty = duty_from_percent(STARTUP_DUTY_PERCENT);
    let total_steps = STARTUP_REVOLUTIONS * 6;
    let mut step: usize = 0;

    for i in 0..total_steps {
        // Linear ramp from STARTUP_INTERVAL_US_MAX → STARTUP_INTERVAL_US_MIN
        let progress = i;
        let interval = STARTUP_INTERVAL_US_MAX
            - (STARTUP_INTERVAL_US_MAX - STARTUP_INTERVAL_US_MIN) * progress / total_steps;

        commutate(step, startup_duty);
        timer.delay_us(interval as u64);

        step = (step + 1) % 6;
    }

    info!("Startup complete — entering run mode");

    // ------------------------------------------------------------------
    // Normal running — constant commutation rate & duty cycle
    //
    // In a real ESC you would:
    //   1. Monitor back-EMF zero crossings on the floating phase
    //   2. Commutate at the detected timing (closed-loop)
    //   3. Accept throttle input (e.g. RC PWM on another GPIO)
    //
    // This example keeps it simple with fixed open-loop timing.
    // ------------------------------------------------------------------
    let run_duty = duty_from_percent(RUN_DUTY_PERCENT);
    info!(
        "Running: duty={}%, interval={}us",
        RUN_DUTY_PERCENT, RUN_INTERVAL_US
    );

    loop {
        commutate(step, run_duty);
        timer.delay_us(RUN_INTERVAL_US as u64);
        step = (step + 1) % 6;
    }
}
