//! # PIO 三路正交编码器示例 (中断驱动)
//!
//! 基于 RP2350 (Raspberry Pi Pico 2) 的 **PIO (可编程 I/O)** 实现，
//! 使用 **单个 PIO 块 (PIO0) 的 3 个状态机** 同时读取 3 个增量式旋转编码器。
//!
//! **使用 PIO0_IRQ_0 中断在 RX FIFO 非空时自动排空并解码**，
//! 主循环仅负责定期打印，CPU 大部分时间可进入休眠。
//!
//! ---
//!
//! # 1. 设计原理
//!
//! ## 为什么用 PIO？
//!
//! 传统 GPIO 中断方式在高转速下可能丢步，且占用 CPU 时间处理中断。
//! PIO 状态机以硬件速率持续采样引脚，**零 CPU 开销**，不会丢失任何边沿。
//!
//! ## 架构
//!
//! ```text
//!   PIO0 (单个 PIO 块, 共享 13 条指令)
//!   ┌──────────────────────────────────────────────────────┐
//!   │  SM0 ─── 采样 GP2/GP3 ─┐                            │
//!   │  SM1 ─── 采样 GP4/GP5 ─┼── RX FIFO 非空 ──► IRQ0   │
//!   │  SM2 ─── 采样 GP6/GP7 ─┘                            │
//!   └──────────────────────────────────────────────────────┘
//!                      │
//!                PIO0_IRQ_0 中断
//!                      │
//!                      ▼
//!         ISR: 排空 FIFO → 查表解码 → 原子累加位置
//!                      │
//!           CPU 主循环: 读取位置 → 定期打印
//! ```
//!
//! 每个状态机运行 **相同的 PIO 程序**（安装一次、共享执行），
//! 通过不同的 `IN_BASE` 引脚配置来读取不同编码器的 A/B 信号。
//!
//! ## PIO 程序工作流程
//!
//! ```text
//!   ┌──────────────┐
//!   │ 读取初始 A/B  │    (启动时执行一次)
//!   │   保存到 Y    │
//!   └──────┬───────┘
//!          ▼
//!   ┌──────────────┐
//!   │  采样 A/B     │◄──────────┐
//!   │  保存到 X     │           │
//!   └──────┬───────┘           │
//!          ▼                   │
//!   ┌──────────────┐    无变化  │
//!   │  X == Y ?    │───────────┘
//!   └──────┬───────┘
//!          │ 有变化
//!          ▼
//!   ┌──────────────────┐
//!   │ 打包 (old<<2|new) │
//!   │   push 到 FIFO    │
//!   │   Y = X (更新)    │
//!   └──────┬───────────┘
//!          └───► 回到采样
//! ```
//!
//! CPU 从 RX FIFO 读取 4-bit 跳变码，通过查找表解码旋转方向 (±1)，
//! 累加到各编码器的位置计数器。
//!
//! ---
//!
//! # 2. 硬件接线
//!
//! 每个编码器的 A、B 信号引脚必须是 **连续的 GPIO**。
//!
//! | Pico GPIO | 编码器 | 引脚 | 说明 |
//! |-----------|--------|------|------|
//! | GP2       | Enc 0  | A    | 编码器信号 A (内部上拉) |
//! | GP3       | Enc 0  | B    | 编码器信号 B (内部上拉) |
//! | GP4       | Enc 1  | A    | 编码器信号 A (内部上拉) |
//! | GP5       | Enc 1  | B    | 编码器信号 B (内部上拉) |
//! | GP6       | Enc 2  | A    | 编码器信号 A (内部上拉) |
//! | GP7       | Enc 2  | B    | 编码器信号 B (内部上拉) |
//! | GND       | 所有   | C    | 编码器公共端 (接地) |
//!
//! ```text
//!   Pico 2 (RP2350)              编码器 0        编码器 1        编码器 2
//!   ──────────────               ────────        ────────        ────────
//!   GP2 ◄─────────────────────── A
//!   GP3 ◄─────────────────────── B
//!   GP4 ◄──────────────────────────────────────── A
//!   GP5 ◄──────────────────────────────────────── B
//!   GP6 ◄──────────────────────────────────────────────────────── A
//!   GP7 ◄──────────────────────────────────────────────────────── B
//!   GND ──────────────────────── C ───────────── C ───────────── C
//! ```
//!
//! > **注意**: A 和 B 必须接在连续 GPIO 上 (如 GP2/GP3)，
//! > 因为 PIO `in pins, 2` 指令读取从 `IN_BASE` 开始的 2 个连续引脚。
//! > 编码器公共端 (C) 均接 GND，可共用。
//!
//! ---
//!
//! # 3. 配置参数
//!
//! | 参数 | 默认值 | 说明 |
//! |------|--------|------|
//! | `ENC0_BASE` | 2 | 编码器 0 的 A 引脚 GPIO (B = A+1) |
//! | `ENC1_BASE` | 4 | 编码器 1 的 A 引脚 GPIO |
//! | `ENC2_BASE` | 6 | 编码器 2 的 A 引脚 GPIO |
//! | PIO 时钟分频 | 1000 (150 kHz) | 约 30k 次/秒采样率 (手拧足够) |
//! | 打印间隔 | 200 ms | 串口输出间隔 |
//!
//! ---
//!
//! # 4. 编译运行
//!
//! ```bash
//! cargo run --bin pio_encoder --target thumbv8m.main-none-eabihf
//! ```

#![no_std]
#![no_main]

use core::sync::atomic::{AtomicI32, Ordering};

use defmt::*;
use defmt_rtt as _;

use rp2350::bind;
use rp2350::gpio::Pins;
use rp2350::interrupt;
use rp2350::main;
use rp2350::pac::{Interrupt, Peripherals};
use rp2350::pio;
use rp2350::pio::FifoJoin;
use rp2350::pio_asm;
use rp2350::timer::Timer0; // ============================================================================
// 引脚配置 (每个编码器的 A, B 必须是连续的 GPIO)
// ============================================================================

/// 编码器 0: A = GP2, B = GP3
const ENC0_BASE: u8 = 2;

/// 编码器 1: A = GP4, B = GP5
const ENC1_BASE: u8 = 4;

/// 编码器 2: A = GP6, B = GP7
const ENC2_BASE: u8 = 6;

/// 编码器数量
const NUM_ENCODERS: usize = 3;

/// 打印间隔 (微秒)
const PRINT_INTERVAL_US: u64 = 200_000;

// ============================================================================
// 共享状态 (主循环 ↔ ISR)
// ============================================================================

/// 编码器位置 (原子操作, ISR 递增/递减, 主循环读取)
static POSITIONS: [AtomicI32; NUM_ENCODERS] =
    [AtomicI32::new(0), AtomicI32::new(0), AtomicI32::new(0)];

// ============================================================================
// PIO0_IRQ_0 中断源位定义 (IRQ0_INTE 寄存器)
// ============================================================================

/// SM0 RX FIFO 非空 — bit 0
const IRQ_SM0_RXNEMPTY: u32 = 0;

/// SM1 RX FIFO 非空 — bit 1
const IRQ_SM1_RXNEMPTY: u32 = 1;

/// SM2 RX FIFO 非空 — bit 2
const IRQ_SM2_RXNEMPTY: u32 = 2;

// ============================================================================
// 正交解码查找表
// ============================================================================

/// 状态跳变表: `QUAD_TABLE[(old_AB << 2) | new_AB]`
///
/// 引脚状态编码: bit0 = A (IN_BASE), bit1 = B (IN_BASE+1)
///
/// 返回值: 0 = 无效/无变化, +1 = CW (A 超前 B), -1 = CCW (B 超前 A)
const QUAD_TABLE: [i32; 16] = [
    0,  //  0: 00→00 无变化
    1,  //  1: 00→01 CW
    -1, //  2: 00→10 CCW
    0,  //  3: 00→11 无效 (跳跃)
    -1, //  4: 01→00 CCW
    0,  //  5: 01→01 无变化
    0,  //  6: 01→10 无效
    1,  //  7: 01→11 CW
    1,  //  8: 10→00 CW
    0,  //  9: 10→01 无效
    0,  // 10: 10→10 无变化
    -1, // 11: 10→11 CCW
    0,  // 12: 11→00 无效
    -1, // 13: 11→01 CCW
    1,  // 14: 11→10 CW
    0,  // 15: 11→11 无变化
];

#[main]
fn main() -> ! {
    info!("=== PIO Triple Quadrature Encoder Example ===");
    info!("1 PIO block, 3 state machines, 3 encoders");

    // ------------------------------------------------------------------
    // 1. 外设初始化
    // ------------------------------------------------------------------
    let dp = Peripherals::take().unwrap();
    let timer = Timer0::init(dp.TIMER0);
    let pins = Pins::init(dp.IO_BANK0, dp.PADS_BANK0);

    // ------------------------------------------------------------------
    // 2. PIO0 初始化
    // ------------------------------------------------------------------
    let mut pio0 = pio::Pio0::init(dp.PIO0);

    // ------------------------------------------------------------------
    // 3. 配置编码器引脚并声明给 PIO0
    //    先设置内部上拉 (编码器公共端接 GND)，再 claim 给 PIO
    //    (claim_pin 只改变 funcsel，不影响 pad 上拉设置)
    // ------------------------------------------------------------------

    // 编码器 0: GP2 (A), GP3 (B)
    let mut enc0a = pins.pin::<2>();
    let mut enc0b = pins.pin::<3>();
    enc0a.set_pull_up();
    enc0b.set_pull_up();
    pio0.claim_pin(enc0a);
    pio0.claim_pin(enc0b);

    // 编码器 1: GP4 (A), GP5 (B)
    let mut enc1a = pins.pin::<4>();
    let mut enc1b = pins.pin::<5>();
    enc1a.set_pull_up();
    enc1b.set_pull_up();
    pio0.claim_pin(enc1a);
    pio0.claim_pin(enc1b);

    // 编码器 2: GP6 (A), GP7 (B)
    let mut enc2a = pins.pin::<6>();
    let mut enc2b = pins.pin::<7>();
    enc2a.set_pull_up();
    enc2b.set_pull_up();
    pio0.claim_pin(enc2a);
    pio0.claim_pin(enc2b);

    info!(
        "Pins: Enc0=GP{}/GP{}, Enc1=GP{}/GP{}, Enc2=GP{}/GP{}",
        ENC0_BASE,
        ENC0_BASE + 1,
        ENC1_BASE,
        ENC1_BASE + 1,
        ENC2_BASE,
        ENC2_BASE + 1
    );

    // ------------------------------------------------------------------
    // 4. PIO 正交编码器程序
    //
    //    原理: 持续采样 A/B 两个引脚，当状态发生变化时，
    //    将 4-bit 跳变码 (old_state << 2 | new_state) 推入 RX FIFO。
    //    CPU 侧通过查找表将跳变码解码为 +1 (CW) 或 -1 (CCW)。
    //
    //    使用 IN 左移模式将 old 和 new 状态拼接到 ISR 的低 4 位。
    // ------------------------------------------------------------------

    let program = pio_asm!(
        r#"
        .program quadrature_encoder

        ; ───────────────────────────────────────────────────────────
        ; 正交编码器 PIO 四倍频解码程序
        ;
        ; 读取 2 个连续 GPIO (A = IN_BASE, B = IN_BASE+1)
        ; 检测引脚状态变化后，将 4-bit 跳变码 (old<<2 | new)
        ; 推送到 RX FIFO，供 CPU 查表解码方向。
        ;
        ; 寄存器用途:
        ;   Y  = 上一次引脚状态 (old_AB, bits 1:0)
        ;   X  = 当前引脚状态 (new_AB, bits 1:0)
        ;   ISR = 工作寄存器 (采样/打包用)
        ;
        ; 配置要求:
        ;   - IN shift: 左移 (shift_right = false)
        ;   - autopush: 关闭
        ;   - IN_BASE: 编码器 A 引脚的 GPIO 号 (B = A+1)
        ;   - FIFO: join RX (8 深 RX FIFO)
        ; ───────────────────────────────────────────────────────────

            ; === 初始化 (仅执行一次) ===
            mov isr, null       ; 清空 ISR
            in pins, 2          ; 读取初始 A, B → ISR[1:0]
            mov y, isr          ; Y = 初始引脚状态 (old state)

        .wrap_target
        sample:
            ; === 采样当前引脚状态 ===
            mov isr, null       ; 清空 ISR
            in pins, 2          ; 读取 A, B → ISR[1:0]
            mov x, isr          ; X = 当前引脚状态 (new state)
            jmp x!=y, changed   ; 与上次不同 → 跳转处理
            jmp sample          ; 无变化 → 继续轮询

        changed:
            ; === 状态变化: 打包并推送 ===
            mov isr, null       ; 清空 ISR
            in y, 2             ; ISR[1:0] = old state (2 bits from Y)
            in x, 2             ; ISR = (old << 2) | new (左移拼接)
            push noblock        ; 推入 RX FIFO (FIFO 满时丢弃，不阻塞)
            mov y, x            ; 更新: old state = new state
        .wrap
        "#
    );

    info!(
        "PIO program: {} instructions (wrap {}-{})",
        program.instructions.len(),
        program.wrap_target,
        program.wrap
    );

    // 安装程序 (仅安装一次，3 个 SM 共享同一份程序内存)
    let installed = pio0.install_program(&program).unwrap();
    info!("Program installed at offset {}", installed.offset);

    // ------------------------------------------------------------------
    // 5. 配置 3 个状态机 (SM0 → Enc0, SM1 → Enc1, SM2 → Enc2)
    //
    //    所有 SM 共享相同配置，仅 IN_BASE 不同:
    //    - 时钟分频 = 1000 (150 MHz / 1000 = 150 kHz 指令速率)
    //      采样轮询环 ~5 条指令 → ~30k 次/秒采样，手拧编码器绰绰有余
    //      (EC11 编码器 20 脉冲/圈, 手拧 ~5 圈/秒 = 100 脉冲/秒, 4× = 400 边沿/秒)
    //    - IN shift = 左移, 无 autopush
    //    - FIFO join RX (8 深 RX FIFO, 不使用 TX)
    // ------------------------------------------------------------------

    // PIO 时钟分频: 150 MHz / 1000 = 150 kHz 指令速率
    // 手拧编码器完全足够, 且大幅降低 FIFO 填充速率
    const PIO_CLKDIV: u16 = 1000;

    // SM0 → 编码器 0 (GP2/GP3)
    pio0.sm0()
        .apply_program(&installed)
        .set_clkdiv(PIO_CLKDIV, 0)
        .set_in_pins(ENC0_BASE)
        .set_in_shift(false, false, 32)
        .set_fifo_join(FifoJoin::Rx)
        .init(installed.offset);

    // SM1 → 编码器 1 (GP4/GP5)
    pio0.sm1()
        .apply_program(&installed)
        .set_clkdiv(PIO_CLKDIV, 0)
        .set_in_pins(ENC1_BASE)
        .set_in_shift(false, false, 32)
        .set_fifo_join(FifoJoin::Rx)
        .init(installed.offset);

    // SM2 → 编码器 2 (GP6/GP7)
    pio0.sm2()
        .apply_program(&installed)
        .set_clkdiv(PIO_CLKDIV, 0)
        .set_in_pins(ENC2_BASE)
        .set_in_shift(false, false, 32)
        .set_fifo_join(FifoJoin::Rx)
        .init(installed.offset);

    // 同步启动所有 3 个状态机 (SM0 | SM1 | SM2 = 0b0111)
    pio0.enable_sm_mask_in_sync(0b0111);

    // ------------------------------------------------------------------
    // 6. 启用 PIO0 RX FIFO 中断
    //
    //    IRQ0_INTE 寄存器:
    //      bit 0 = SM0 RX FIFO 非空
    //      bit 1 = SM1 RX FIFO 非空
    //      bit 2 = SM2 RX FIFO 非空
    //
    //    当任一 SM 的 RX FIFO 有数据时，触发 PIO0_IRQ_0 中断，
    //    ISR 排空 FIFO 并通过查表累加位置。
    // ------------------------------------------------------------------
    pio0.set_irq0_source_enabled(IRQ_SM0_RXNEMPTY, true);
    pio0.set_irq0_source_enabled(IRQ_SM1_RXNEMPTY, true);
    pio0.set_irq0_source_enabled(IRQ_SM2_RXNEMPTY, true);

    unsafe {
        interrupt::enable(Interrupt::PIO0_IRQ_0);
    }

    info!("All 3 state machines started in sync");
    info!("PIO0_IRQ_0 enabled (RX FIFO not-empty for SM0/SM1/SM2)");
    info!("");
    info!("Rotate any encoder — positions printed every 200ms");
    info!("");

    // ------------------------------------------------------------------
    // 7. 主循环: 定期打印位置 (解码在 ISR 中完成)
    // ------------------------------------------------------------------

    let mut last_positions = [0i32; NUM_ENCODERS];
    let mut last_print_time = timer.time_us();

    loop {
        timer.delay_ms(10);

        let now = timer.time_us();
        if now.wrapping_sub(last_print_time) >= PRINT_INTERVAL_US {
            let positions = [
                POSITIONS[0].load(Ordering::Relaxed),
                POSITIONS[1].load(Ordering::Relaxed),
                POSITIONS[2].load(Ordering::Relaxed),
            ];

            if positions != last_positions {
                info!(
                    "Enc0: {}  |  Enc1: {}  |  Enc2: {}",
                    positions[0], positions[1], positions[2]
                );
                last_positions = positions;
            }
            last_print_time = now;
        }
    }
}

// ============================================================================
// PIO0_IRQ_0 中断处理函数
// ============================================================================

/// PIO0 IRQ0 中断服务程序
///
/// 当任一 SM 的 RX FIFO 非空时触发。
/// 排空所有 SM 的 FIFO，查表解码后原子累加到 POSITIONS。
#[bind(PIO0_IRQ_0)]
fn pio0_encoder_isr() {
    // SAFETY: PIO0 已在 main() 中通过 Pio0::init() 初始化。
    // ISR 在单核 Cortex-M33 上不会与主循环并发访问 PIO 寄存器
    // (主循环只读原子变量, 不访问 PIO)。
    let pio0 = unsafe { pio::Pio0::steal() };

    // SM0 — 编码器 0
    while let Some(val) = pio0.sm0().rx_fifo_try_read() {
        POSITIONS[0].fetch_add(QUAD_TABLE[(val & 0x0F) as usize], Ordering::Relaxed);
    }

    // SM1 — 编码器 1
    while let Some(val) = pio0.sm1().rx_fifo_try_read() {
        POSITIONS[1].fetch_add(QUAD_TABLE[(val & 0x0F) as usize], Ordering::Relaxed);
    }

    // SM2 — 编码器 2
    while let Some(val) = pio0.sm2().rx_fifo_try_read() {
        POSITIONS[2].fetch_add(QUAD_TABLE[(val & 0x0F) as usize], Ordering::Relaxed);
    }
}
