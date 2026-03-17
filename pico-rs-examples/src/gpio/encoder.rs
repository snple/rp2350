//! # 旋转编码器读取示例
//!
//! 基于 RP2350 (Raspberry Pi Pico 2) 的 **增量式旋转编码器** 读取示例，
//! 使用 **GPIO 边沿中断** 实现四倍频 (4×) 正交解码。
//!
//! ---
//!
//! # 1. 功能概述
//!
//! ## 1.1 增量式旋转编码器原理
//!
//! 增量式旋转编码器输出两路相差 90° 的方波信号 (A 和 B):
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
//! ## 1.2 正交解码状态表
//!
//! 4× 解码 (在 A 和 B 的每个边沿都更新计数):
//!
//! | 旧状态 (AB) | 新状态 (AB) | 方向 |
//! |------------|------------|------|
//! | 00 | 01 | +1 (CW) |
//! | 01 | 11 | +1 (CW) |
//! | 11 | 10 | +1 (CW) |
//! | 10 | 00 | +1 (CW) |
//! | 00 | 10 | -1 (CCW) |
//! | 10 | 11 | -1 (CCW) |
//! | 11 | 01 | -1 (CCW) |
//! | 01 | 00 | -1 (CCW) |
//!
//! 其他跳变 (如 00→11) 视为无效/噪声，忽略。
//!
//! ## 1.3 本示例的解码方案
//!
//! 使用 **GPIO 中断方案**:
//! - 对 A、B 两个引脚分别注册**双沿** (上升沿 + 下降沿) 中断
//! - 在 ISR 中读取当前 A、B 电平，与上一次状态比对
//! - 查表得到旋转方向，更新原子计数器
//! - 主循环定期读取计数器并打印
//!
//! **优点**: 4× 解码精度，CPU 仅在信号变化时被短暂中断。
//!
//! ## 1.4 本示例演示内容
//!
//! 1. **基本位置计数**: CW +1, CCW -1, 打印累计位置
//! 2. **旋转速度估算**: 通过相邻读取间的位置差计算转速
//! 3. **按键检测**: 大多数旋转编码器自带按键 (SW 引脚)
//!
//! ---
//!
//! # 2. 硬件接线
//!
//! ## 2.1 引脚分配
//!
//! | Pico GPIO | 编码器引脚 | 说明 |
//! |-----------|-----------|------|
//! | GP10 | A (CLK) | A 通道信号 |
//! | GP11 | B (DT) | B 通道信号 |
//! | GP12 | SW | 按键 (可选，低有效) |
//! | 3.3V | VCC (+) | 电源 (部分编码器也可用 5V) |
//! | GND | GND | 接地 |
//!
//! ```text
//!   Pico 2 (RP2350)               旋转编码器 (KY-040 等)
//!   ──────────────                ─────────────────────
//!   GP10 ◄──────────────────────── CLK (A)
//!   GP11 ◄──────────────────────── DT  (B)
//!   GP12 ◄──────────────────────── SW  (按键, 可选)
//!   3.3V ────────────────────────── VCC (+)
//!   GND  ────────────────────────── GND
//! ```
//!
//! > **上拉电阻**: 大多数 KY-040 模块自带 10kΩ 上拉。
//! > 如果使用裸编码器，可使用 GPIO 内置上拉 (`into_input_pullup()`)。
//!
//! ## 2.2 其他可选 GPIO
//!
//! 编码器只需要普通 GPIO，任何引脚均可。只需修改下方常量。
//!
//! ---
//!
//! # 3. 配置参数
//!
//! | 参数 | 默认值 | 说明 |
//! |------|--------|------|
//! | `PIN_A` | 10 | 编码器 A 通道 GPIO |
//! | `PIN_B` | 11 | 编码器 B 通道 GPIO |
//! | `PIN_SW` | 12 | 编码器按键 GPIO (可选) |
//! | 主循环间隔 | 200 ms | 打印状态的间隔 |
//!
//! ---
//!
//! # 4. 常见旋转编码器规格
//!
//! | 型号 | 脉冲/转 (PPR) | 每转计数 (4×) | 说明 |
//! |------|--------------|-------------|------|
//! | KY-040 | 20 | 80 | 最常见的廉价编码器模块 |
//! | EC11 | 20~30 | 80~120 | 面板安装型，带按键 |
//! | 600 PPR 光电 | 600 | 2400 | 高精度电机用 |
//!
//! ---
//!
//! # 5. 编译运行
//!
//! ```bash
//! cargo run --bin encoder --target thumbv8m.main-none-eabihf
//! ```

#![no_std]
#![no_main]

use core::sync::atomic::{AtomicBool, AtomicI32, Ordering};

use defmt::*;
use defmt_rtt as _;
use panic_probe as _;

use pico_rs::block::ImageDef;
use pico_rs::entry;
use pico_rs::gpio::{EdgeLevel, Pin, Pins};
use pico_rs::interrupt;
use pico_rs::pac::{Interrupt, Peripherals};
use pico_rs::timer::Timer0;

#[unsafe(link_section = ".start_block")]
#[used]
pub static IMAGE_DEF: ImageDef = ImageDef::exe();

// ============================================================================
// 配置参数
// ============================================================================

/// 编码器 A 通道 GPIO 编号
const PIN_A: u8 = 10;

/// 编码器 B 通道 GPIO 编号
const PIN_B: u8 = 11;

/// 编码器按键 GPIO 编号 (按下接地)
const PIN_SW: u8 = 12;

// ============================================================================
// 共享状态 (主循环 ↔ ISR)
// ============================================================================

/// 编码器累计位置 (有符号, 4× 解码)
/// CW → 递增, CCW → 递减
static POSITION: AtomicI32 = AtomicI32::new(0);

/// 上一次 A/B 状态 (bit1 = A, bit0 = B), 存为 u8 但只用低 2 位
/// 用 AtomicI32 避免引入 AtomicU8 (thumbv8m 可能不支持)
static LAST_AB: AtomicI32 = AtomicI32::new(0);

/// 按键按下标志 (由 ISR 置 true, 主循环清除)
static BUTTON_PRESSED: AtomicBool = AtomicBool::new(false);

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

#[entry]
fn main() -> ! {
    info!("=== Rotary Encoder Example (GPIO Interrupt, 4x Decode) ===");

    // ------------------------------------------------------------------
    // 1. 外设初始化
    // ------------------------------------------------------------------
    let dp = Peripherals::take().unwrap();
    let timer = Timer0::init(dp.TIMER0);
    let pins = Pins::new(dp.IO_BANK0, dp.PADS_BANK0);

    // ------------------------------------------------------------------
    // 2. 配置编码器引脚
    // ------------------------------------------------------------------

    // A 通道 — 输入 + 内置上拉
    let pin_a = pins.pin::<PIN_A>();
    pin_a.into_input_pullup();

    // B 通道 — 输入 + 内置上拉
    let pin_b = pins.pin::<PIN_B>();
    pin_b.into_input_pullup();

    // 按键 — 输入 + 内置上拉 (按下接地)
    let pin_sw = pins.pin::<PIN_SW>();
    pin_sw.into_input_pullup();

    info!("Pins: A=GP{}, B=GP{}, SW=GP{}", PIN_A, PIN_B, PIN_SW);

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
    //    A 和 B 都注册**双沿** (上升沿 + 下降沿) 中断 → 4× 解码
    //    SW 注册下降沿中断 (按下时触发)
    // ------------------------------------------------------------------
    pin_a.enable_interrupt(EdgeLevel::EdgeHigh);
    pin_a.enable_interrupt(EdgeLevel::EdgeLow);

    pin_b.enable_interrupt(EdgeLevel::EdgeHigh);
    pin_b.enable_interrupt(EdgeLevel::EdgeLow);

    pin_sw.enable_interrupt(EdgeLevel::EdgeLow);

    // 使能 NVIC 中 IO_IRQ_BANK0 中断
    unsafe {
        interrupt::enable(Interrupt::IO_IRQ_BANK0);
    }

    info!("Interrupts enabled (A: both edges, B: both edges, SW: falling)");

    // ------------------------------------------------------------------
    // 5. 主循环 — 定期打印编码器状态
    // ------------------------------------------------------------------
    info!("");
    info!("Rotate the encoder — position and speed will be printed every 200ms");
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

        // 如果位置有变化或按键被按下, 打印信息
        if delta != 0 {
            // 速度 = delta / dt (step/s)
            let speed = if dt_us > 0 {
                (delta as i64 * 1_000_000) / dt_us as i64
            } else {
                0
            };

            let direction = if delta > 0 { "CW " } else { "CCW" };

            info!(
                "[#{}] Position: {}, Delta: {}, Speed: {} step/s ({})",
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
/// 在 A 或 B 的任何边沿触发时:
///   1. 读取当前 A、B 电平
///   2. 与上一次状态查表得到方向
///   3. 原子更新 POSITION
///
/// 在 SW 下降沿触发时:
///   设置 BUTTON_PRESSED 标志
#[allow(non_snake_case)]
#[unsafe(no_mangle)]
extern "C" fn IO_IRQ_BANK0() {
    // SAFETY: GPIO 子系统已在 main() 中通过 Pins::new() 初始化。
    // ISR 中仅访问中断状态寄存器和读取引脚电平, 不做重新配置。
    let a = unsafe { Pin::<PIN_A>::new() };
    let b = unsafe { Pin::<PIN_B>::new() };
    let sw = unsafe { Pin::<PIN_SW>::new() };

    // --- 编码器 A/B 中断处理 ---
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

    // --- 按键中断处理 ---
    if sw.check_and_clear(EdgeLevel::EdgeLow) {
        BUTTON_PRESSED.store(true, Ordering::Relaxed);
    }
}
