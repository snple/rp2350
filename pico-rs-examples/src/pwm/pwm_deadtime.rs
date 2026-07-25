//! # 双切片 + 相位偏移法 PWM 互补输出（带死区控制）
//!
//! 基于 RP2350 (Raspberry Pi Pico 2) 实现半桥/全桥电机控制或开关电源所需的
//! **互补 PWM + 死区时间** 输出。
//!
//! RP2350 的 PWM 外设没有硬件死区插入功能（不像 STM32 高级定时器），
//! 因此本示例使用 **双切片相位偏移法** 在软件层面实现死区控制。
//!
//! **适用场景：**
//! - 三相 BLDC 电机半桥驱动（配合 IR2104/DRV8323 等栅极驱动器）
//! - DC-DC 同步整流
//! - 全桥 DC 电机 / 步进电机驱动
//!
//! ---
//!
//! # 1. 核心原理
//!
//! ## 1.1 什么是"双切片 + 相位偏移"？
//!
//! 使用两个独立的 PWM Slice（而非同一 Slice 的 A/B 通道），通过预装载其中一个
//! Slice 的计数器初始值，使其输出波形相对于另一个 Slice 产生固定的相位滞后。
//!
//! ```text
//!                    ┌─ Slice 0 (高侧/上管) ──► GPIO0
//!                    │
//!   PWM 外设 ────────┤
//!                    │
//!                    └─ Slice 1 (低侧/下管, 反相) ──► GPIO2
//!
//!   Slice 1 计数器 = (Slice 0 计数器 - DEAD_TIME) mod (WRAP+1)
//!   即 Slice 1 始终落后 Slice 0 固定步数 = 相位偏移 = 死区时间
//! ```
//!
//! ## 1.2 为什么不能只用同一切片的 A/B 通道？
//!
//! 同一 Slice 的 A/B 通道共享一个计数器、分频器和 TOP 值。
//! 如果只用 A/B 来做互补 + 死区：
//!
//! ```text
//!   A (normal):  HIGH when counter < CC_A
//!   B (inverted): HIGH when counter >= CC_B
//!
//!   设 CC_A = duty, CC_B = duty + dead:
//!
//!   counter:  0 ──────── CC_A ──── CC_B ──────── WRAP ──► 0
//!   A:        ‾‾‾‾‾‾‾‾‾‾\______\_______________/‾‾‾‾‾‾‾‾‾‾
//!   B (inv):  ___________/      /‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾\__________
//!                         ^^^^                    ^^^^
//!                   dead time (OK!)         NO dead time! ✗
//! ```
//!
//! 在 counter 回绕到 0 的边沿上，A 和 B 同时切换——无死区，可能直通！
//!
//! ## 1.3 本方案（双切片 + 相位偏移）
//!
//! 两个 Slice 独立计数，Slice 1 始终滞后 Slice 0 固定的 `DEAD_TIME` 步：
//!
//! ```text
//!   Slice0 counter: 0 ────── CC ──────────── WRAP ─► 0
//!   Slice0 (normal): ‾‾‾‾‾‾\____________________/‾‾‾‾‾
//!
//!   Slice1 counter: DEAD ────── CC+DEAD ─────── WRAP ─► DEAD
//!   Slice1 (inv):    ______/   \____________________/
//!                         ^^^^                     ^^^^
//!                    dead time (OK!)          dead time (OK!)
//! ```
//!
//! 两个边沿都有死区！
//!
//! ## 1.4 死区时间波形示意
//!
//! ```text
//!   高侧 (Slice 0, Ch A):
//!       ‾‾‾‾‾‾‾‾‾‾‾‾‾\_____________________/‾‾‾‾‾‾‾‾‾‾‾‾
//!
//!   低侧 (Slice 1, Ch A, 反相):
//!       ______________/                     \___________
//!                      ^^^^^^^^^^^^^^^^^^^^^^
//!                           死区 (两管均关断)
//!                      ^^^^                 ^^^^
//!                    上升沿死区            下降沿死区
//! ```
//!
//! ---
//!
//! # 2. 频率与死区计算
//!
//! ## 2.1 PWM 频率
//!
//! ```text
//!   边沿对齐模式:
//!   f_PWM = f_sys / (DIV × (WRAP + 1))
//!
//!   以 150 MHz 系统时钟, DIV=1, WRAP=7499 为例:
//!   f_PWM = 150_000_000 / (1 × 7500) = 20_000 Hz (20 kHz)
//!
//!   每步时间 = 1 / 150_000_000 ≈ 6.67 ns
//! ```
//!
//! ## 2.2 死区时间
//!
//! ```text
//!   t_dead = DEAD_TIME_COUNTS × (DIV / f_sys)
//!
//!   例如 DEAD_TIME_COUNTS = 150:
//!   t_dead = 150 × (1 / 150_000_000) = 1.0 µs
//!
//!   例如 DEAD_TIME_COUNTS = 75:
//!   t_dead = 75 × 6.67ns ≈ 500 ns
//! ```
//!
//! ---
//!
//! # 3. 硬件接线
//!
//! ```text
//!   RP2350 (Pico 2)                  半桥栅极驱动 (如 IR2104)
//!   ┌──────────────┐                 ┌─────────────────────┐
//!   │  GP0  ─────────────────────────► IN (高侧 PWM)       │
//!   │  GP2  ─────────────────────────► nSD (低侧/互补PWM)  │
//!   │  GND  ─────────────────────────► GND                 │
//!   └──────────────┘                 └─────────┬───────────┘
//!                                              │
//!                                        HO ──┤── 高侧 MOSFET
//!                                        LO ──┤── 低侧 MOSFET
//!                                              │
//!                                        VS ──┼── 半桥中点 → 电机/电感
//!   ```
//!
//! > **注意：** GP0 是 Slice 0 Channel A，GP2 是 Slice 1 Channel A。
//! > 不要使用同一 Slice 的两个通道（如 GP0 和 GP1），它们共享计数器！
//!
//! ---
//!
//! # 4. 参数说明
//!
//! | 参数 | 默认值 | 说明 |
//! |------|--------|------|
//! | `PWM_FREQ_HZ` | 20_000 | PWM 载波频率 |
//! | `WRAP` | 7_499 | TOP 值 = 7500 步/周期 |
//! | `DEAD_TIME_NS` | 1_000 | 死区时间 (纳秒) |
//! | `DEAD_TIME_COUNTS` | 自动计算 | 死区对应的计数步数 |
//! | `INITIAL_DUTY_PCT` | 50 | 初始占空比 (%) |
//!
//! ---
//!
//! # 5. 技术要点
//!
//! 1. **Slice 同步**：先 disable 两个 Slice，分别预装计数器（一个为 0，一个为
//!    `DEAD_TIME`），然后快速连续 enable。两次 enable 之间的软件延迟（若干 CPU
//!    周期）会被叠加到死区时间中，对大多数应用可忽略。
//!
//! 2. **死区方向的正确性**：确保 Slice 1 的计数器滞后于 Slice 0，这样才能保证
//!    在两个输出边沿上都产生死区间隙。
//!
//! 3. **占空比更新**：两个 Slice 的 compare 值需保持同步更新。本示例在每次修改
//!    占空比时同时写入两个 Slice。
//!
//! 4. **相位偏移的稳定性**：由于两个 Slice 使用完全相同的 DIV 和 WRAP，一旦同步
//!    后，相位偏移将长期保持稳定（不受时钟抖动影响）。
//!
//! ---
//!
//! # 6. 编译运行
//!
//! ```bash
//! cargo build --release --bin pwm_deadtime
//! ```
//!
//! 使用逻辑分析仪或示波器观察 GP0 和 GP2：
//! - 两个通道频率相同（20 kHz）
//! - GP2 滞后 GP0，且反相
//! - 在每次切换时两通道存在约 1µs 的同时低电平间隙（死区）

#![no_std]
#![no_main]

use defmt::info;
use defmt_rtt as _;

use pico_rs::clocks::ClockBuilder;
use pico_rs::gpio::Pins;
use pico_rs::pwm::Pwm;
use pico_rs::timer::Timer0;
use pico_rs::{Peripherals, main};

// ------------------------------------------------------------------
// 可调参数
// ------------------------------------------------------------------

/// PWM 载波频率 (Hz)
const PWM_FREQ_HZ: u32 = 20_000;

/// 系统时钟 (Hz) — 与 ClockBuilder 默认值一致
const SYS_FREQ_HZ: u32 = 150_000_000;

/// 死区时间 (纳秒)
const DEAD_TIME_NS: u32 = 1_000;

/// 初始占空比 (百分比, 0~100)
const INITIAL_DUTY_PCT: u32 = 50;

/// 扫频周期 (ms) — 占空比从 10% 到 90% 来回变化
const SWEEP_PERIOD_MS: u32 = 4_000;

// ------------------------------------------------------------------
// 派生参数
// ------------------------------------------------------------------

/// WRAP 值：决定每个 PWM 周期的分辨率步数
/// f_PWM = SYS_FREQ_HZ / (DIV × (WRAP + 1))
/// 使用 DIV = 1 时的 WRAP 值
const WRAP: u16 = (SYS_FREQ_HZ / PWM_FREQ_HZ - 1) as u16;

/// 死区时间对应的计数步数
/// t_dead = DEAD_TIME_COUNTS × DIV / f_sys
/// 当 DIV = 1: 每步 ≈ 6.67ns @ 150MHz
const DEAD_TIME_COUNTS: u16 = {
    // counts = t_dead_ns * f_sys_hz / 1_000_000_000 / DIV
    // DIV = 1 when SYS_FREQ_HZ / (WRAP+1) = PWM_FREQ_HZ
    let counts: u64 = (DEAD_TIME_NS as u64) * (SYS_FREQ_HZ as u64) / 1_000_000_000;
    if counts > 65535 {
        // clamp to u16 max - but that would mean > 437µs dead time, unreasonable
        65535
    } else if counts == 0 {
        1 // ensure at least 1 count of dead time
    } else {
        counts as u16
    }
};

/// 周期 (纳秒) — 编译期计算
const PERIOD_NS: u32 = (1_000_000_000u64 / PWM_FREQ_HZ as u64) as u32;

/// 每步分辨率 (纳秒) — 编译期计算
const STEP_NS: u32 = (1_000_000_000u64 / SYS_FREQ_HZ as u64) as u32;

// ------------------------------------------------------------------
// 编译期校验
// ------------------------------------------------------------------

// 确保死区不会吃掉全部占空比
const _CHECK_DEADTIME_RANGE: () = {
    // 最大占空比对应的 CC 值必须大于死区步数，否则低侧无法开启
    // CC_max ≈ WRAP (99.9%)，只需检查 dead time < WRAP
    assert!(
        DEAD_TIME_COUNTS < WRAP,
        "Dead time must be less than PWM period"
    );
};

// 确保 WRAP + 1 = SYS_FREQ / PWM_FREQ (DIV=1 时)
const _CHECK_WRAP_CONSISTENCY: () = {
    assert!(
        (WRAP as u32) + 1 == SYS_FREQ_HZ / PWM_FREQ_HZ,
        "WRAP must equal SYS_FREQ_HZ / PWM_FREQ_HZ - 1 for DIV=1"
    );
};

// ============================================================================

#[main]
fn main() -> ! {
    info!("PWM Dead-Time Control — Dual Slice + Phase Shift");

    // ------------------------------------------------------------------
    // 外设初始化
    // ------------------------------------------------------------------
    let dp = Peripherals::take().unwrap();
    let clocks = ClockBuilder::new().build(dp.CLOCKS, dp.XOSC, dp.PLL_SYS, dp.PLL_USB);
    let sys_freq = clocks.sys_freq();
    info!("System clock: {} Hz", sys_freq);

    let pins = Pins::init(dp.IO_BANK0, dp.PADS_BANK0);
    let timer = Timer0::init(dp.TIMER0);

    // ------------------------------------------------------------------
    // 参数打印
    // ------------------------------------------------------------------
    info!("PWM frequency: {} Hz", PWM_FREQ_HZ);
    info!("WRAP (TOP): {}", WRAP);
    info!(
        "Dead time: {} ns ({} counts)",
        DEAD_TIME_NS, DEAD_TIME_COUNTS
    );
    info!("Period: {} ns, Resolution: {} ns/step", PERIOD_NS, STEP_NS);

    // ------------------------------------------------------------------
    // PWM 初始化
    // ------------------------------------------------------------------

    // 使用 Slice 0 Channel A (GPIO 0) — 高侧/上管
    // 使用 Slice 1 Channel A (GPIO 2) — 低侧/下管（反相输出）
    let pwm = Pwm::init(dp.PWM);

    //
    // Slice 0: 高侧（正常输出）
    //
    let mut slice_hi = pwm.slice::<0>().output_a(pins.pin::<0>());
    slice_hi.set_wrap(WRAP);
    slice_hi.set_div(1, 0); // DIV = 1.0
    slice_hi.set_duty_a(0);
    slice_hi.disable(); // 先禁用，等待同步

    //
    // Slice 1: 低侧（反相输出）
    //
    let mut slice_lo = pwm.slice::<1>().output_a(pins.pin::<2>());
    slice_lo.set_wrap(WRAP);
    slice_lo.set_div(1, 0); // DIV = 1.0 — 与高侧完全相同
    slice_lo.set_duty_a(0);

    // 低侧反相：当 counter >= CC 时输出 HIGH
    slice_lo.set_invert_a(true);

    slice_lo.disable(); // 先禁用，等待同步

    // ------------------------------------------------------------------
    // 双切片相位同步
    //
    // 关键在于：Slice 1 的计数器预装载 DEAD_TIME_COUNTS，
    // 使其始终滞后 Slice 0 固定的步数。
    //
    // 步骤：
    //   1. 两个 Slice 均处于 disable 状态
    //   2. Slice 0 计数器 → 0
    //   3. Slice 1 计数器 → DEAD_TIME_COUNTS
    //   4. 快速连续 enable 两个 Slice
    // ------------------------------------------------------------------
    info!(
        "Synchronizing slices with {} count phase offset...",
        DEAD_TIME_COUNTS
    );

    // 确保都已禁用
    slice_hi.disable();
    slice_lo.disable();

    // 预装载计数器值
    slice_hi.set_counter(0);
    slice_lo.set_counter(DEAD_TIME_COUNTS);

    // 快速连续启用（软件延迟约 2-3 个 CPU 周期，叠加到死区中）
    slice_hi.enable();
    slice_lo.enable();

    // ------------------------------------------------------------------
    // 设置初始占空比
    // ------------------------------------------------------------------
    let initial_cc = ((WRAP as u32 + 1) * INITIAL_DUTY_PCT / 100) as u16;
    info!("Initial duty: {}% (CC = {})", INITIAL_DUTY_PCT, initial_cc);
    slice_hi.set_duty_a(initial_cc);
    slice_lo.set_duty_a(initial_cc);

    // ------------------------------------------------------------------
    // 验证配置
    // ------------------------------------------------------------------
    let actual_freq = slice_hi.frequency(sys_freq);
    info!("Actual PWM frequency: {} Hz", actual_freq);

    info!("GPIO0 (Slice 0 ChA): High-side, non-inverted");
    info!(
        "GPIO2 (Slice 1 ChA): Low-side, inverted, lagging by {} counts",
        DEAD_TIME_COUNTS
    );
    info!("System ready. Connect logic analyzer to GPIO0 and GPIO2.");

    // ------------------------------------------------------------------
    // 主循环：占空比扫频（10% → 90% → 10%）
    // 用于在示波器/逻辑分析仪上观察死区
    // ------------------------------------------------------------------
    let min_duty_pct: u32 = 10;
    let max_duty_pct: u32 = 90;
    let sweep_range = max_duty_pct - min_duty_pct;

    loop {
        // 上升阶段：10% → 90%
        for step in 0..=100 {
            let pct = min_duty_pct + (sweep_range * step / 100);
            let cc = ((WRAP as u32 + 1) * pct / 100) as u16;

            // 两个 Slice 同步更新占空比
            slice_hi.set_duty_a(cc);
            slice_lo.set_duty_a(cc);

            timer.delay_ms(SWEEP_PERIOD_MS / 200);
        }

        // 下降阶段：90% → 10%
        for step in 0..=100 {
            let pct = max_duty_pct - (sweep_range * step / 100);
            let cc = ((WRAP as u32 + 1) * pct / 100) as u16;

            slice_hi.set_duty_a(cc);
            slice_lo.set_duty_a(cc);

            timer.delay_ms(SWEEP_PERIOD_MS / 200);
        }
    }
}
