//! # TMC2209 步进电机控制 — 基于 pico_rs_drivers
//!
//! 使用 [`pico_rs_drivers::tmc2209::Tmc2209`] 驱动 TMC2209 芯片，
//! 使用 [`pico_rs_drivers::stepper::tmc2209::AxisState`] 实现 STEP/DIR 脉冲生成与梯形加减速。
//!
//! ---
//!
//! # 1. 硬件接线
//!
//! | Pico GPIO | 功能     | TMC2209 引脚 | 说明                        |
//! |-----------|----------|-------------|-----------------------------|
//! | GP4       | UART1_TX | PDN_UART    | 通过 1kΩ 电阻连接           |
//! | GP5       | UART1_RX | PDN_UART    | 通过 1kΩ 电阻连接           |
//! | GP7       | EN       | ENN          | 使能控制 (**低有效**)        |
//! | GP8/12    | DIR      | DIR          | 方向控制                    |
//! | GP9/13    | STEP     | STEP         | 步进脉冲输出                |
//! | GP10/14   | INDEX    | INDEX        | 电气周期脉冲 (可选)         |
//! | GP11/15   | DIAG     | DIAG         | 堵转/错误输出 (开漏, 需上拉) |
//!
//! PDN_UART 上必须外接 1kΩ 上拉电阻到 VCC_IO (3.3V)，否则芯片上电即进入待机模式，
//! 不响应 UART 命令。
//!
//! ---
//!
//! # 2. 配置参数
//!
//! | 参数 | 值 | 说明 |
//! |------|-----|------|
//! | 微步细分 | 1/16 | CHOPCONF.MRES = 4 |
//! | 运行电流 | ~325 mA RMS | IRUN = 4 (外部 Rsense = 0.11Ω) |
//! | 保持电流 | ~195 mA RMS | IHOLD = 2 |
//! | 堵转检测 | 开启 | SGTHRS = 10, DIAG 中断 |
//!
//! ---
//!
//! # 3. 编译运行
//!
//! ```bash
//! cd pico-rs-examples
//! cargo run --bin stepper_tmc2209 --target thumbv8m.main-none-eabihf
//! ```

#![no_std]
#![no_main]

use pico_rs_drivers::stepper::Direction;
use pico_rs_drivers::stepper::tmc2209::{AxisConfig, AxisState};
use pico_rs_drivers::tmc2209::{
    Gconf, IholdIr, REG_CHOPCONF, REG_IHOLD_IRUN, REG_PWMCONF, REG_SGTHRS, REG_TCOOLTHRS,
    REG_TPOWERDOWN, Tmc2209, microstepping_to_mres,
};

use defmt::*;
use defmt_rtt as _;

use pico_rs::clocks::ClockBuilder;
use pico_rs::main;
use pico_rs::gpio::{self, Pins};
use pico_rs::interrupt;
use pico_rs::pac::{Interrupt, Peripherals};
use pico_rs::ticks::Ticks;
use pico_rs::timer::Timer0;
use pico_rs::uart::{DataBits, Parity, StopBits, Uart1};// ============================================================================
// 运动控制参数
// ============================================================================

/// 起步频率 (Hz)
const START_SPEED_HZ: u32 = 100;

/// 最高运行频率 (Hz)
const MAX_SPEED_HZ: u32 = 2000;

/// 加减速段步数
const ACCEL_STEPS: u32 = 200;

/// STEP 脉冲高电平宽度 (μs)
const STEP_PULSE_US: u32 = 5;

/// 电机每转全步数 (1.8° = 200 步/转)
const MOTOR_STEPS_PER_REV: u32 = 200;

/// 微步细分数
const MICROSTEPPING: u32 = 16;

// ============================================================================
// TMC2209 UART 通信参数
// ============================================================================

/// UART 波特率
const TMC_UART_BAUD: u32 = 115_200;

// ============================================================================
// TMC2209 电流参数 — 外部检流电阻 (GCONF.internal_rsense = 0, Rsense = 0.11Ω)
//
// I_RMS = (IRUN+1)/32 × 325mV/(√2 × 0.11Ω) ≈ (IRUN+1) × 65.3 mA
// IRUN = 4  → I_RMS ≈ 5 × 65.3 ≈ 326 mA
// IHOLD = 2  → I_RMS ≈ 3 × 65.3 ≈ 196 mA
// ============================================================================

/// 运行电流 IRUN (0–31), 外部 Rsense: ~325 mA
const RUN_CURRENT: u8 = 12;

/// 保持电流 IHOLD (0–31), 外部 Rsense: ~195 mA
const HOLD_CURRENT: u8 = 2;

/// 电流过渡延时 (0–15)
const IHOLD_DELAY: u8 = 10;

// ============================================================================
// StallGuard 堵转检测参数
// ============================================================================

/// 正常运行堵转阈值 (0–255, 值越大越灵敏)
const SG_THRESHOLD: u32 = 10;

/// 回零堵转阈值 (比正常更灵敏)
const HOMING_SG_THRESHOLD: u32 = 30;

/// 回零速度 (Hz)
const HOMING_SPEED_HZ: u32 = 800;

/// 回零后反弹步数
const HOMING_BACKOFF_STEPS: u32 = 200;

// ============================================================================
// 引脚定义
// ============================================================================

/// GP4 → UART1_TX → TMC2209 PDN_UART
/// GP5 → UART1_RX → TMC2209 PDN_UART
const PIN_ENABLE: u32 = 7; // GP7  → ENN (低有效)

// #1
const TMC_SLAVE_ADDR: u8 = 0x00; // TMC2209 从机地址
const PIN_DIR: u32 = 8; // GP8  → DIR
const PIN_STEP: u32 = 9; // GP9  → STEP
const PIN_INDEX: u32 = 10; // GP10 → INDEX
const PIN_DIAG: u32 = 11; // GP11 → DIAG

// #2
// const TMC_SLAVE_ADDR: u8 = 0x01; // TMC2209 从机地址
// const PIN_DIR: u8 = 12; // GP12  → DIR
// const PIN_STEP: u8 = 13; // GP13  → STEP
// const PIN_INDEX: u8 = 14; // GP14 → INDEX
// const PIN_DIAG: u8 = 15; // GP15 → DIAG

// ============================================================================
// 步进轴状态
// ============================================================================

static AXIS: AxisState = AxisState::new(AxisConfig {
    pin_step: PIN_STEP,
    pin_dir: PIN_DIR,
    alarm: 0,
    start_speed_hz: START_SPEED_HZ,
    accel_steps: ACCEL_STEPS,
    step_pulse_us: STEP_PULSE_US,
    diag_pin: PIN_DIAG,
    index_pin: PIN_INDEX,
    homing_backoff_index: 0, // 不使用 INDEX 回退
});

// ============================================================================
// 运动控制辅助函数
// ============================================================================

fn start_move(steps: u32, dir: Direction) {
    let timer = unsafe { Timer0::steal() };
    AXIS.start_move(&timer, steps, dir, MAX_SPEED_HZ);
}

fn start_continuous(dir: Direction, speed_hz: u32) {
    let timer = unsafe { Timer0::steal() };
    AXIS.start_continuous(&timer, dir, speed_hz);
}

fn stop_motion() {
    AXIS.stop();
}

fn wait_done() {
    AXIS.wait_done();
}

// ============================================================================
// 程序入口
// ============================================================================

#[main]
fn main() -> ! {
    info!("=== TMC2209 Stepper (pico_rs_drivers) ===");
    info!(
        "Pins: UART1 GP4(TX)/GP5(RX)  EN=GP{}  DIR=GP{}  STEP=GP{}  INDEX=GP{}  DIAG=GP{}",
        PIN_ENABLE, PIN_DIR, PIN_STEP, PIN_INDEX, PIN_DIAG
    );
    info!(
        "Params: 1/{} microstep, ext_rsense=0.11Ω, IRUN={} (~325mA), IHOLD={} (~195mA), SGTHRS={}",
        MICROSTEPPING, RUN_CURRENT, HOLD_CURRENT, SG_THRESHOLD
    );

    // ------------------------------------------------------------------
    // 1. 外设初始化
    // ------------------------------------------------------------------
    let dp = Peripherals::take().unwrap();

    let clocks = ClockBuilder::new().build(dp.CLOCKS, dp.XOSC, dp.PLL_SYS, dp.PLL_USB);
    info!("Clock: sys={} Hz", clocks.sys_freq());

    let ticks = Ticks::new(dp.TICKS, &clocks);
    ticks.init_timer0();

    let timer = Timer0::init(dp.TIMER0);
    info!("Timer0 initialized");

    let pins = Pins::new(dp.IO_BANK0, dp.PADS_BANK0);

    // ------------------------------------------------------------------
    // 2. GPIO 配置
    // ------------------------------------------------------------------

    // ENN (低有效): 初始拉高 = 禁用
    let _en = pins.pin::<PIN_ENABLE>();
    _en.into_output();
    unsafe { gpio::set_high(PIN_ENABLE) };

    // DIR: 初始低
    let _dir = pins.pin::<PIN_DIR>();
    _dir.into_output();
    unsafe { gpio::set_low(PIN_DIR) };

    // STEP: 初始低
    let _step = pins.pin::<PIN_STEP>();
    _step.into_output();
    unsafe { gpio::set_low(PIN_STEP) };

    // INDEX: 输入 (由 AxisState 读取)
    let _index = pins.pin::<PIN_INDEX>();
    _index.into_input_pullup();

    // DIAG: 开漏输出, 上拉 + 上升沿中断
    let p_diag = pins.pin::<PIN_DIAG>();
    p_diag.into_input_pullup();

    info!("GPIO configured");

    // ------------------------------------------------------------------
    // 3. UART1 初始化 — TMC2209 通信
    // ------------------------------------------------------------------
    let uart = Uart1::new(
        dp.UART1,
        pins.pin::<4>(), // GP4 = TX
        pins.pin::<5>(), // GP5 = RX
        TMC_UART_BAUD,
        clocks.peri_freq(),
    );
    uart.set_format(DataBits::Eight, StopBits::One, Parity::None);
    info!("UART1: {} baud, 8N1", TMC_UART_BAUD);

    timer.delay_ms(100);

    // ======================================================================
    // 阶段零: 引脚连通性测试
    // ======================================================================
    info!("=== Phase 0: Pin connectivity test ===");

    let tmc = Tmc2209::new(&uart, TMC_SLAVE_ADDR);

    // 验证芯片能通信
    let ioin_init = tmc.read_ioin(&timer);
    if ioin_init.is_none() {
        warn!("Cannot communicate with TMC2209 — check UART wiring and PDN_UART pull-up!");
        info!("Halting.");
        loop {
            pico_rs_rt::asm::wfi();
        }
    }
    let ver = ioin_init.unwrap().version();
    info!("Chip version: {:#04x} (expect 0x21)", ver);

    // --- 测试 ENN (GPxx) ---
    info!("--- ENN (GP{}) test ---", PIN_ENABLE);
    unsafe { gpio::set_high(PIN_ENABLE) };
    timer.delay_ms(2);
    match tmc.read_ioin(&timer) {
        Some(v) => info!(
            "  ENN=HIGH (GP{} high) → IOIN.ENN={} (expect true) {}",
            PIN_ENABLE,
            v.enn(),
            if v.enn() { "✓" } else { "✗" }
        ),
        None => warn!("  read failed"),
    }
    unsafe { gpio::set_low(PIN_ENABLE) };
    timer.delay_ms(2);
    match tmc.read_ioin(&timer) {
        Some(v) => info!(
            "  ENN=LOW  (GP{} low)  → IOIN.ENN={} (expect false) {}",
            PIN_ENABLE,
            v.enn(),
            if !v.enn() { "✓" } else { "✗" }
        ),
        None => warn!("  read failed"),
    }

    // --- 测试 DIR (GPxx) ---
    info!("--- DIR (GP{}) test ---", PIN_DIR);
    unsafe { gpio::set_high(PIN_DIR) };
    timer.delay_ms(2);
    match tmc.read_ioin(&timer) {
        Some(v) => info!(
            "  DIR=HIGH (GP{} high) → IOIN.DIR={} (expect true) {}",
            PIN_DIR,
            v.dir(),
            if v.dir() { "✓" } else { "✗" }
        ),
        None => warn!("  read failed"),
    }
    unsafe { gpio::set_low(PIN_DIR) };
    timer.delay_ms(2);
    match tmc.read_ioin(&timer) {
        Some(v) => info!(
            "  DIR=LOW  (GP{} low)  → IOIN.DIR={} (expect false) {}",
            PIN_DIR,
            v.dir(),
            if !v.dir() { "✓" } else { "✗" }
        ),
        None => warn!("  read failed"),
    }

    // --- 测试 STEP (GPxx) ---
    info!("--- STEP (GP{}) test ---", PIN_STEP);
    unsafe { gpio::set_high(PIN_STEP) };
    timer.delay_ms(2);
    match tmc.read_ioin(&timer) {
        Some(v) => info!(
            "  STEP=HIGH (GP{} high) → IOIN.STEP={} (expect true) {}",
            PIN_STEP,
            v.step(),
            if v.step() { "✓" } else { "✗" }
        ),
        None => warn!("  read failed"),
    }
    unsafe { gpio::set_low(PIN_STEP) };
    timer.delay_ms(2);
    match tmc.read_ioin(&timer) {
        Some(v) => info!(
            "  STEP=LOW  (GP{} low)  → IOIN.STEP={} (expect false) {}",
            PIN_STEP,
            v.step(),
            if !v.step() { "✓" } else { "✗" }
        ),
        None => warn!("  read failed"),
    }

    // --- 测试 INDEX (GPxx, TMC2209 输出 → Pico 输入) ---
    info!("--- INDEX (GP{}) test (Pico-side read) ---", PIN_INDEX);
    let idx_state = unsafe { gpio::get(PIN_INDEX) };
    info!(
        "  INDEX pin = {} (TMC2209 output, expect stable HIGH or LOW)",
        idx_state
    );

    // --- 测试 DIAG (GPxx, TMC2209 开漏输出 → Pico 上拉输入) ---
    info!("--- DIAG (GP{}) test (Pico-side read) ---", PIN_DIAG);
    let diag_state = unsafe { gpio::get(PIN_DIAG) };
    info!(
        "  DIAG pin = {} (open-drain w/ pull-up, expect HIGH) {}",
        diag_state,
        if diag_state {
            "✓"
        } else {
            "✗ LOW — possible stall/error or floating"
        }
    );

    // 恢复初始状态: ENN=HIGH (禁用), DIR=LOW, STEP=LOW
    unsafe { gpio::set_high(PIN_ENABLE) };
    unsafe { gpio::set_low(PIN_DIR) };
    unsafe { gpio::set_low(PIN_STEP) };
    info!("Pin test complete. ENN=HIGH (disabled), DIR=LOW, STEP=LOW");
    timer.delay_ms(50);

    // ======================================================================
    // 阶段一: 配置 TMC2209
    // ======================================================================
    info!("=== Phase 1: Configure TMC2209 ===");

    // 1.1 验证通信 — 读取芯片版本和引脚状态
    match tmc.read_ioin(&timer) {
        Some(ioin) => {
            let ver = ioin.version();
            info!(
                "IOIN={:#010x}  version={:#04x} {}  ENN={}  MS1={}  MS2={}  STEP={}  DIR={}",
                u32::from(ioin),
                ver,
                if ver == 0x21 { "✓" } else { "✗" },
                ioin.enn(),
                ioin.ms1(),
                ioin.ms2(),
                ioin.step(),
                ioin.dir()
            );
        }
        None => {
            warn!("Failed to read TMC2209 — check UART wiring and PDN_UART pull-up!");
        }
    }

    timer.delay_ms(10);

    // 1.2 清除错误标志
    tmc.clear_gstat(&timer);
    timer.delay_ms(10);
    if let Some(gstat) = tmc.read_gstat(&timer) {
        info!(
            "GSTAT={:#04x}  reset={}  drv_err={}  uv_cp={}",
            u32::from(gstat),
            gstat.reset(),
            gstat.drv_err(),
            gstat.uv_cp()
        );
    }

    // 1.3 逐寄存器显式配置 + 回读验证 (不使用 configure() 以便定位问题)
    //     pdn_disable=1 → PDN_UART 仅 UART 通信
    //     mstep_reg_select=1 → 微步来自 CHOPCONF.MRES
    //     multistep_filt=1 → STEP 脉冲边沿滤波
    //     internal_rsense=0 → 使用外部检流电阻 (0.11Ω)
    //     i_scale_analog=0 → 使用内部 VREF (非 AIN 引脚)
    let gconf_val = Gconf::new()
        .with_pdn_disable(true)
        .with_mstep_reg_select(true)
        .with_multistep_filt(true)
        .with_internal_rsense(false)
        .with_i_scale_analog(false);
    tmc.write_gconf(&gconf_val, &timer);
    timer.delay_ms(5);
    match tmc.read_gconf(&timer) {
        Some(v) => info!(
            "GCONF write→read: {:#06x}  pdn_dis={}  mstep_reg={}  multistep={}  int_rsense={}  i_scale_analog={}",
            u32::from(v),
            v.pdn_disable(),
            v.mstep_reg_select(),
            v.multistep_filt(),
            v.internal_rsense(),
            v.i_scale_analog()
        ),
        None => warn!("GCONF readback FAILED"),
    }

    // --- 诊断: 先读默认值 ---
    match tmc.read_reg(REG_IHOLD_IRUN, &timer) {
        Some(v) => info!("IHOLD_IRUN default (pre-write): {:#010x}", v),
        None => warn!("IHOLD_IRUN default read FAILED"),
    }
    match tmc.read_reg(REG_TPOWERDOWN, &timer) {
        Some(v) => info!("TPOWERDOWN default (pre-write): {}", v),
        None => warn!("TPOWERDOWN default read FAILED"),
    }

    // IHOLD_IRUN — 直接写 raw register，不通过 helper
    let ihold_irun_val: u32 =
        (HOLD_CURRENT as u32) | ((RUN_CURRENT as u32) << 8) | ((IHOLD_DELAY as u32) << 16);
    info!(
        "Writing IHOLD_IRUN = {:#010x} (IHOLD={} IRUN={} DELAY={})",
        ihold_irun_val, HOLD_CURRENT, RUN_CURRENT, IHOLD_DELAY
    );
    tmc.write_reg(REG_IHOLD_IRUN, ihold_irun_val, &timer);
    timer.delay_ms(5);
    match tmc.read_reg(REG_IHOLD_IRUN, &timer) {
        Some(v) => {
            let ir = IholdIr::from(v);
            info!(
                "IHOLD_IRUN write→read: {:#010x}  IHOLD={}  IRUN={}  DELAY={} {}",
                v,
                ir.ihold(),
                ir.irun(),
                ir.iholddelay(),
                if ir.irun() == RUN_CURRENT && ir.ihold() == HOLD_CURRENT {
                    "✓"
                } else {
                    "✗ MISMATCH"
                }
            );
        }
        None => warn!("IHOLD_IRUN readback FAILED"),
    }

    // TPOWERDOWN
    tmc.write_reg(REG_TPOWERDOWN, 20, &timer);
    timer.delay_ms(5);
    match tmc.read_reg(REG_TPOWERDOWN, &timer) {
        Some(v) => info!(
            "TPOWERDOWN write→read: {} (expect 20) {}",
            v,
            if v == 20 { "✓" } else { "✗" }
        ),
        None => warn!("TPOWERDOWN readback FAILED"),
    }

    // CHOPCONF — 低电流优化: TOFF=4 延长关断时间, TBL=2 延长消隐
    //   HSTRT=5 / HEND=3 标准回差避免方向不对称
    let mres = microstepping_to_mres(MICROSTEPPING);
    let chopconf_val: u32 = 4 | (5 << 4) | (3 << 7) | (2 << 15) | (mres << 24) | (1 << 28);
    tmc.write_reg(REG_CHOPCONF, chopconf_val, &timer);
    timer.delay_ms(5);
    match tmc.read_reg(REG_CHOPCONF, &timer) {
        Some(v) => info!(
            "CHOPCONF write→read: {:#010x}  TOFF={}  MRES={} {}",
            v,
            v & 0xF,
            (v >> 24) & 0xF,
            if v == chopconf_val { "✓" } else { "✗" }
        ),
        None => warn!("CHOPCONF readback FAILED"),
    }

    // PWMCONF — PWM_OFS=30 匹配低电流 (原 36 对 ~300mA 偏高)
    let pwmconf_val: u32 = 30 | (14 << 8) | (1 << 18) | (1 << 19) | (8 << 24) | (12 << 28);
    tmc.write_reg(REG_PWMCONF, pwmconf_val, &timer);
    timer.delay_ms(5);
    match tmc.read_reg(REG_PWMCONF, &timer) {
        Some(v) => info!(
            "PWMCONF write→read: {:#010x} {}",
            v,
            if v == pwmconf_val { "✓" } else { "✗" }
        ),
        None => warn!("PWMCONF readback FAILED"),
    }

    // SGTHRS
    tmc.write_reg(REG_SGTHRS, SG_THRESHOLD, &timer);
    timer.delay_ms(5);
    match tmc.read_reg(REG_SGTHRS, &timer) {
        Some(v) => info!(
            "SGTHRS write→read: {} (expect {}) {}",
            v,
            SG_THRESHOLD,
            if v == SG_THRESHOLD { "✓" } else { "✗" }
        ),
        None => warn!("SGTHRS readback FAILED"),
    }

    // TCOOLTHRS
    tmc.write_reg(REG_TCOOLTHRS, 0xFFFFF, &timer);
    timer.delay_ms(10);
    match tmc.read_reg(REG_TCOOLTHRS, &timer) {
        Some(v) => info!(
            "TCOOLTHRS write→read: {:#x} (expect 0xFFFFF) {}",
            v,
            if v == 0xFFFFF { "✓" } else { "✗" }
        ),
        None => warn!("TCOOLTHRS readback FAILED"),
    }

    // VACTUAL=0 — 关闭 UART 速度控制模式，确保 STEP/DIR 正常
    tmc.stop_vactual(&timer);
    timer.delay_ms(10);

    // 最终校验: 再次读取 IHOLD_IRUN
    match tmc.read_reg(REG_IHOLD_IRUN, &timer) {
        Some(v) => {
            let ir = IholdIr::from(v);
            info!(
                "IHOLD_IRUN final: {:#010x}  IHOLD={}  IRUN={}  DELAY={}",
                v,
                ir.ihold(),
                ir.irun(),
                ir.iholddelay()
            );
        }
        None => warn!("IHOLD_IRUN final readback FAILED"),
    }
    if let Some(ifcnt) = tmc.read_ifcnt(&timer) {
        info!("IFCNT = {}", ifcnt);
    }

    let steps_per_rev = MOTOR_STEPS_PER_REV * MICROSTEPPING;
    info!(
        "Steps/rev: {} ({} full × {} microstep)",
        steps_per_rev, MOTOR_STEPS_PER_REV, MICROSTEPPING
    );

    // ------------------------------------------------------------------
    // 4. 中断配置
    // ------------------------------------------------------------------
    unsafe {
        interrupt::enable(Interrupt::TIMER0_IRQ_0);
    }
    info!("Interrupts enabled");

    // ------------------------------------------------------------------
    // 5. 使能电机
    // ------------------------------------------------------------------
    unsafe { gpio::set_low(PIN_ENABLE) };
    timer.delay_ms(10);

    // 回读 IOIN 确认 ENN 引脚状态
    if let Some(ioin) = tmc.read_ioin(&timer) {
        info!(
            "Motor enabled: IOIN ENN={} (expect 0) {}",
            ioin.enn(),
            if !ioin.enn() {
                "✓"
            } else {
                "✗ ENN still high — check GP7 wiring!"
            }
        );
    }
    // 再读 GSTAT 确认无驱动错误
    if let Some(gstat) = tmc.read_gstat(&timer)
        && gstat.drv_err()
    {
        warn!("DRV_ERR flag set — possible over-temperature or short circuit!");
    }
    timer.delay_ms(100);

    // ======================================================================
    // 阶段二: 驱动演示
    // ======================================================================

    // --- Demo 1: 正转 1 圈 (梯形加减速) ---
    info!("=== Demo 1: Forward {} steps (1 rev) ===", steps_per_rev);
    start_move(steps_per_rev, Direction::Forward);
    wait_done();
    info!("Demo 1 done. pos={}", AXIS.position());
    timer.delay_ms(1000);

    // --- Demo 2: 反转 1 圈 ---
    info!("=== Demo 2: Reverse {} steps ===", steps_per_rev);
    start_move(steps_per_rev, Direction::Reverse);
    wait_done();
    info!("Demo 2 done. pos={}", AXIS.position());
    timer.delay_ms(1000);

    // --- Demo 3: 动态降电流 ---
    info!("=== Demo 3: Dynamic current reduction ===");
    info!("Reducing to IRUN=2 (~195mA)");
    tmc.set_run_current(2, 1, &timer);
    timer.delay_ms(50);

    start_move(steps_per_rev / 2, Direction::Forward);
    wait_done();

    info!("Restoring IRUN={} (~325mA)", RUN_CURRENT);
    tmc.set_run_current(RUN_CURRENT, HOLD_CURRENT, &timer);
    timer.delay_ms(50);

    start_move(steps_per_rev / 2, Direction::Reverse);
    wait_done();
    info!("Demo 3 done. pos={}", AXIS.position());
    timer.delay_ms(1000);

    // --- Demo 4: 连续运转 + StallGuard 轮询 ---
    info!("=== Demo 4: Continuous run + StallGuard polling ===");
    start_continuous(Direction::Forward, 1000);

    for i in 0..6u32 {
        timer.delay_ms(500);

        if let Some(sg) = tmc.read_sg_result(&timer) {
            info!("[{}] SG_RESULT = {}", i, sg);
        }
        if let Some(status) = tmc.read_drv_status(&timer) {
            info!(
                "[{}] DRV_STATUS: cs={} stst={} ola={} olb={} otpw={} ot={}",
                i,
                status.cs_actual(),
                status.stst(),
                status.ola(),
                status.olb(),
                status.otpw(),
                status.ot()
            );
        }
    }

    stop_motion();
    info!(
        "Demo 4 done. {} steps in 3s. pos={}",
        AXIS.steps_done(),
        AXIS.position()
    );
    timer.delay_ms(1000);

    // --- Demo 5: 堵转急停 (DIAG 中断) ---
    info!("=== Demo 5: Stall detection (DIAG interrupt) ===");
    info!("Block the motor shaft to trigger a stall!");

    tmc.set_sg_threshold(HOMING_SG_THRESHOLD, &timer);
    timer.delay_ms(50);

    start_continuous(Direction::Forward, HOMING_SPEED_HZ);

    let mut timeout = 0u32;
    while !AXIS.stall_occurred() && timeout < 20 {
        timer.delay_ms(500);
        timeout += 1;
        if let Some(sg) = tmc.read_sg_result(&timer) {
            info!("[stall] SG_RESULT = {}", sg);
        }
    }
    stop_motion();

    if AXIS.stall_occurred() {
        info!("STALL DETECTED at pos={}!", AXIS.position());
        info!("Backing off {} steps...", HOMING_BACKOFF_STEPS);
        timer.delay_ms(200);

        start_move(HOMING_BACKOFF_STEPS, Direction::Reverse);
        wait_done();
        info!("Demo 5 done. pos={}", AXIS.position());
    } else {
        info!("Demo 5 timeout — no stall in 10s");
    }

    tmc.set_sg_threshold(SG_THRESHOLD, &timer);
    timer.delay_ms(1000);

    // --- Demo 6: 无传感器回零 (利用 DIAG + 限位检测) ---
    info!("=== Demo 6: Sensorless homing ===");
    info!("Motor moves forward until hitting hard stop...");

    tmc.set_sg_threshold(HOMING_SG_THRESHOLD, &timer);
    timer.delay_ms(50);

    // 使用 start_homing 向 Reverse 方向 (反方向 = 向限位)
    AXIS.start_homing(&timer, HOMING_SPEED_HZ);

    timeout = 0;
    while !AXIS.is_done() && timeout < 20 {
        timer.delay_ms(500);
        timeout += 1;
    }
    stop_motion();

    if AXIS.position() == 0 {
        info!("Homing complete: at limit, pos=0");
    } else {
        info!("Homing timeout — position={}", AXIS.position());
    }

    tmc.set_sg_threshold(SG_THRESHOLD, &timer);
    timer.delay_ms(1000);

    // --- Demo 7: 寄存器转储 ---
    info!("=== Demo 7: Register dump ===");
    if let Some(gconf) = tmc.read_gconf(&timer) {
        info!(
            "  GCONF      = {:#06x}  pdn_dis={}  int_rsense={}  multistep={}",
            u32::from(gconf),
            gconf.pdn_disable(),
            gconf.internal_rsense(),
            gconf.multistep_filt()
        );
    }
    if let Some(ioin) = tmc.read_ioin(&timer) {
        info!(
            "  IOIN       = {:#010x}  version={:#04x}",
            u32::from(ioin),
            ioin.version()
        );
    }
    if let Some(chopconf) = tmc.read_chopconf(&timer) {
        info!("  CHOPCONF   = {:#010x}", u32::from(chopconf));
    }
    if let Some(pwmconf) = tmc.read_pwmconf(&timer) {
        info!("  PWMCONF    = {:#010x}", u32::from(pwmconf));
    }
    if let Some(drv) = tmc.read_drv_status(&timer) {
        info!("  DRV_STATUS = {:#010x}", u32::from(drv));
    }
    timer.delay_ms(1000);

    // ======================================================================
    // 循环: 交替正反转
    // ======================================================================
    info!("=== Loop: alternating forward/reverse ===");
    AXIS.set_position(0);

    let mut forward = true;
    loop {
        let dir = if forward {
            Direction::Forward
        } else {
            Direction::Reverse
        };
        info!("[loop] {} {} steps", dir.as_str(), steps_per_rev / 2);

        start_move(steps_per_rev / 2, dir);
        wait_done();
        info!("[loop] pos={}", AXIS.position());

        timer.delay_ms(500);
        forward = !forward;
    }
}

// ============================================================================
// Timer0 ISR — STEP 脉冲生成
// ============================================================================

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
fn TIMER0_IRQ_0() {
    let timer = unsafe { Timer0::steal() };
    AXIS.handle_alarm_isr(&timer);
}
