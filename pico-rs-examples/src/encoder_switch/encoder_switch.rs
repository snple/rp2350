//! # 增量式旋转编码器开关示例
//!
//! 基于 RP2350 (Raspberry Pi Pico 2) 的 **5 脚增量式旋转编码器开关** 读取示例，
//! 使用 **GPIO 边沿中断** 实现四倍频 (4×) 正交解码 + 按键检测。
//!
//! ---
//!
//! # 1. 编码器开关引脚说明
//!
//! 5 脚增量式旋转编码器开关 (如 EC11) 具有以下引脚:
//!
//! ```text
//!        ┌───────────┐
//!        │  编码器开关 │
//!  A○────┤           ├────○D
//!        │           │
//!  C○────┤  (公共端)  ├────○E
//!        │           │
//!  B○────┤           │
//!        └───────────┘
//!
//!  编码器部分: A, B, C          按钮部分: D, E
//!  ─────────────────          ──────────────
//!  A - 编码器信号 A            D - 按钮引脚 1
//!  B - 编码器信号 B            E - 按钮引脚 2
//!  C - 编码器公共端 (接 GND)    (按下时 D-E 短接)
//! ```
//!
//! ## 工作原理
//!
//! **编码器部分 (A, B, C)**:
//! - C 为公共端，连接 GND
//! - A 和 B 输出两路相差 90° 的方波信号
//! - A、B 通过 GPIO 内部上拉到高电平
//! - 旋转时 A、B 交替连接到 C (GND)，产生低电平脉冲
//!
//! **按钮部分 (D, E)**:
//! - 按下旋钮时 D 和 E 导通
//! - E 连接 GND，D 通过 GPIO 内部上拉到高电平
//! - 按下时 D 被拉低
//!
//! ## 正交解码原理
//!
//! ```text
//!   顺时针旋转 (CW):                   逆时针旋转 (CCW):
//!
//!   A:  ─┐ ┌─┐ ┌─┐ ┌──             A:  ──┐ ┌─┐ ┌─┐ ┌─
//!        │ │ │ │ │ │                     │ │ │ │ │ │
//!        └─┘ └─┘ └─┘                     └─┘ └─┘ └─┘
//!
//!   B:  ──┐ ┌─┐ ┌─┐ ┌─             B:  ─┐ ┌─┐ ┌─┐ ┌──
//!         │ │ │ │ │ │                   │ │ │ │ │ │
//!         └─┘ └─┘ └─┘                   └─┘ └─┘ └─┘
//!
//!        A 超前 B → 正转              B 超前 A → 反转
//! ```
//!
//! ---
//!
//! # 2. 硬件接线
//!
//! | Pico GPIO | 编码器引脚 | 说明 |
//! |-----------|-----------|------|
//! | GP10      | A         | 编码器信号 A (内部上拉) |
//! | GP11      | B         | 编码器信号 B (内部上拉) |
//! | GND       | C         | 编码器公共端 (接地) |
//! | GP12      | D         | 按钮引脚 (内部上拉) |
//! | GND       | E         | 按钮公共端 (接地) |
//!
//! ```text
//!   Pico 2 (RP2350)               旋转编码器开关 (EC11 等)
//!   ──────────────                ────────────────────────
//!   GP10 ◄──────────────────────── A  (编码器信号 A)
//!   GP11 ◄──────────────────────── B  (编码器信号 B)
//!   GND  ────────────────────────── C  (编码器公共端)
//!   GP12 ◄──────────────────────── D  (按钮引脚)
//!   GND  ────────────────────────── E  (按钮公共端)
//! ```
//!
//! > **注意**: C 和 E 都接 GND，可以共用同一个 GND 引脚。
//! > A、B、D 使用 GPIO 内部上拉电阻，无需外部上拉。
//!
//! ---
//!
//! # 3. 配置参数
//!
//! | 参数 | 默认值 | 说明 |
//! |------|--------|------|
//! | `PIN_A` | 10 | 编码器信号 A 的 GPIO |
//! | `PIN_B` | 11 | 编码器信号 B 的 GPIO |
//! | `PIN_BTN` | 12 | 按钮引脚 D 的 GPIO |
//! | 主循环间隔 | 200 ms | 打印状态的间隔 |
//! | 消抖时间 | 50 ms | 按钮消抖延迟 |
//!
//! ---
//!
//! # 4. 编译运行
//!
//! ```bash
//! cargo run --bin encoder_switch --target thumbv8m.main-none-eabihf
//! ```

#![no_std]
#![no_main]

use core::sync::atomic::{AtomicBool, AtomicI32, AtomicU32, Ordering};

use defmt::*;
use defmt_rtt as _;

use pico_rs::bind;
use pico_rs::clocks::{ClockBuilder};
use pico_rs::main;
use pico_rs::gpio::{EdgeLevel, Pin, Pins};
use pico_rs::interrupt;
use pico_rs::pac::{Interrupt, Peripherals};
use pico_rs::ticks::Ticks;
use pico_rs::timer::Timer0;// ============================================================================
// 引脚配置
// ============================================================================

/// 编码器信号 A 的 GPIO 编号 (编码器引脚 A)
const PIN_A: u32 = 10;

/// 编码器信号 B 的 GPIO 编号 (编码器引脚 B)
const PIN_B: u32 = 11;

/// 按钮引脚的 GPIO 编号 (编码器引脚 D)
/// 编码器引脚 C 接 GND (编码器公共端)
/// 编码器引脚 E 接 GND (按钮公共端)
const PIN_BTN: u32 = 12;

/// 按钮消抖时间 (微秒)
const DEBOUNCE_US: u32 = 50_000; // 50ms

// ============================================================================
// 共享状态 (主循环 ↔ ISR)
// ============================================================================

/// 编码器累计位置 (有符号, 4× 解码)
/// CW → 递增, CCW → 递减
static POSITION: AtomicI32 = AtomicI32::new(0);

/// 上一次 A/B 状态 (bit1 = A, bit0 = B)
static LAST_AB: AtomicI32 = AtomicI32::new(0);

/// 按键按下标志 (由 ISR 置 true, 主循环清除)
static BUTTON_PRESSED: AtomicBool = AtomicBool::new(false);

/// 上次按钮中断时间戳 (用于消抖, 低 32 位足够 — 约 71 分钟溢出, 远超 50ms 消抖窗口)
static LAST_BTN_TIME: AtomicU32 = AtomicU32::new(0);

// ============================================================================
// 正交解码查找表
// ============================================================================

/// 状态跳变表: `QUAD_TABLE[old_state << 2 | new_state]`
///
/// 返回值:  0 = 无效/无变化, +1 = CW, -1 = CCW
///
/// 索引 = (old_AB << 2) | new_AB, 共 16 项
const QUAD_TABLE: [i32; 16] = [
    0,  // 00 → 00: 无变化
    1,  // 00 → 01: CW
    -1, // 00 → 10: CCW
    0,  // 00 → 11: 无效 (跳跃)
    -1, // 01 → 00: CCW
    0,  // 01 → 01: 无变化
    0,  // 01 → 10: 无效
    1,  // 01 → 11: CW
    1,  // 10 → 00: CW
    0,  // 10 → 01: 无效
    0,  // 10 → 10: 无变化
    -1, // 10 → 11: CCW
    0,  // 11 → 00: 无效
    -1, // 11 → 01: CCW
    1,  // 11 → 10: CW
    0,  // 11 → 11: 无变化
];

#[main]
fn main() -> ! {
    info!("=== 5-Pin Rotary Encoder Switch Example ===");
    info!("Pins: A(encoder), B(encoder), C(GND), D(button), E(GND)");

    // ------------------------------------------------------------------
    // 1. 外设初始化
    // ------------------------------------------------------------------
    let dp = Peripherals::take().unwrap();
    let clocks = ClockBuilder::new().build(dp.CLOCKS, dp.XOSC, dp.PLL_SYS, dp.PLL_USB);
    let ticks = Ticks::new(dp.TICKS, &clocks);
    ticks.init_timer0();
    let timer = Timer0::init(dp.TIMER0);
    let pins = Pins::new(dp.IO_BANK0, dp.PADS_BANK0);

    // ------------------------------------------------------------------
    // 2. 配置编码器引脚
    //    A, B → GPIO 输入 + 内部上拉 (编码器公共端 C 接 GND)
    //    D   → GPIO 输入 + 内部上拉 (按钮公共端 E 接 GND)
    // ------------------------------------------------------------------

    // 编码器信号 A — 输入 + 内部上拉
    let pin_a = pins.pin::<PIN_A>();
    pin_a.into_input_pullup();

    // 编码器信号 B — 输入 + 内部上拉
    let pin_b = pins.pin::<PIN_B>();
    pin_b.into_input_pullup();

    // 按钮引脚 D — 输入 + 内部上拉 (按下时 D 被拉低到 E/GND)
    let pin_btn = pins.pin::<PIN_BTN>();
    pin_btn.into_input_pullup();

    info!(
        "Config: A=GP{}, B=GP{}, BTN=GP{} (C=GND, E=GND)",
        PIN_A, PIN_B, PIN_BTN
    );

    // ------------------------------------------------------------------
    // 3. 读取初始 A/B 状态
    // ------------------------------------------------------------------
    let init_a = pin_a.get() as i32;
    let init_b = pin_b.get() as i32;
    let init_state = (init_a << 1) | init_b;
    LAST_AB.store(init_state, Ordering::Relaxed);
    info!(
        "Initial state: A={}, B={} (AB={:#04b})",
        init_a, init_b, init_state
    );

    // ------------------------------------------------------------------
    // 4. 使能 GPIO 中断
    //    A, B: 双沿中断 (上升沿 + 下降沿) → 4× 解码
    //    D:    下降沿中断 (按下时触发)
    // ------------------------------------------------------------------
    pin_a.enable_interrupt(EdgeLevel::EdgeHigh);
    pin_a.enable_interrupt(EdgeLevel::EdgeLow);

    pin_b.enable_interrupt(EdgeLevel::EdgeHigh);
    pin_b.enable_interrupt(EdgeLevel::EdgeLow);

    pin_btn.enable_interrupt(EdgeLevel::EdgeLow);

    // 使能 NVIC 中 IO_IRQ_BANK0 中断
    unsafe {
        interrupt::enable(Interrupt::IO_IRQ_BANK0);
    }

    info!("Interrupts enabled (A/B: both edges, BTN: falling edge)");

    // ------------------------------------------------------------------
    // 5. 主循环 — 定期打印编码器状态
    // ------------------------------------------------------------------
    info!("");
    info!("Rotate the encoder knob — position and speed printed every 200ms");
    info!("Press the encoder button to reset position to 0");
    info!("");

    let mut last_position: i32 = 0;
    let mut last_time = timer.get_time_us();
    let mut print_count: u32 = 0;

    loop {
        timer.delay_ms(200);

        // 读取当前位置 (原子操作, 与 ISR 无竞争)
        let position = POSITION.load(Ordering::Relaxed);
        let now = timer.get_time_us();

        // 计算自上次以来的增量和时间差
        let delta = position - last_position;
        let dt_us = now.wrapping_sub(last_time);

        // 位置有变化时打印信息
        if delta != 0 {
            // 速度 = delta / dt (step/s)
            let speed = if dt_us > 0 {
                (delta as i64 * 1_000_000) / dt_us as i64
            } else {
                0
            };

            let direction = if delta > 0 { "CW " } else { "CCW" };

            info!(
                "[#{}] Pos: {}, Delta: {}, Speed: {} step/s ({})",
                print_count, position, delta, speed, direction
            );

            last_position = position;
            last_time = now;
            print_count += 1;
        }

        // 检查按键事件
        if BUTTON_PRESSED.swap(false, Ordering::Relaxed) {
            POSITION.store(0, Ordering::Relaxed);
            last_position = 0;
            info!("[Button] Position reset to 0");
        }
    }
}

// ============================================================================
// GPIO 中断处理函数
// ============================================================================

/// IO_IRQ_BANK0 中断服务程序
///
/// **编码器 A/B 边沿触发**:
///   1. 读取当前 A、B 电平
///   2. 与上一次状态查表得到方向 (+1 CW / -1 CCW)
///   3. 原子更新 POSITION
///
/// **按钮 D 下降沿触发**:
///   消抖后设置 BUTTON_PRESSED 标志
#[bind(IO_IRQ_BANK0)]
fn encoder_button_isr() {
    // SAFETY: GPIO 子系统已在 main() 中通过 Pins::new() 初始化。
    // ISR 中仅访问中断状态寄存器和读取引脚电平, 不做重新配置。
    let a = unsafe { Pin::<PIN_A>::steal() };
    let b = unsafe { Pin::<PIN_B>::steal() };
    let btn = unsafe { Pin::<PIN_BTN>::steal() };

    // --- 先清除中断标志并保存结果 (check_and_clear 会清除标志, 只能调用一次) ---
    let a_fired = a.check_and_clear(EdgeLevel::EdgeHigh) | a.check_and_clear(EdgeLevel::EdgeLow);
    let b_fired = b.check_and_clear(EdgeLevel::EdgeHigh) | b.check_and_clear(EdgeLevel::EdgeLow);

    if a_fired || b_fired {
        // 读取当前 A、B 电平
        let cur_a = a.get() as i32;
        let cur_b = b.get() as i32;
        let new_state = (cur_a << 1) | cur_b;

        // 获取上一次状态
        let old_state = LAST_AB.swap(new_state, Ordering::Relaxed);

        // 查表: 索引 = (old << 2) | new
        let idx = ((old_state << 2) | new_state) as usize & 0x0F;
        let direction = QUAD_TABLE[idx];

        // 原子更新位置
        if direction != 0 {
            POSITION.fetch_add(direction, Ordering::Relaxed);
        }
    }

    // --- 按钮中断处理 (带消抖 + 电平确认) ---
    if btn.check_and_clear(EdgeLevel::EdgeLow) {
        // 确认按钮确实处于按下状态 (引脚为低电平)
        // 编码器旋转时相邻引脚串扰可能产生短暂毛刺触发 EdgeLow,
        // 但此时引脚已恢复高电平, 可过滤掉
        if !btn.get() {
            let now = {
                let timer = unsafe { Timer0::steal() };
                timer.get_time_us_32()
            };

            let last = LAST_BTN_TIME.load(Ordering::Relaxed);
            if now.wrapping_sub(last) >= DEBOUNCE_US {
                LAST_BTN_TIME.store(now, Ordering::Relaxed);
                BUTTON_PRESSED.store(true, Ordering::Relaxed);
            }
        }
    }
}
