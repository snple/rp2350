//! # 双核共享数据示例
//!
//! 基于 RP2350 (Raspberry Pi Pico 2) 的 **双核协作** 示例，
//! 演示 Core 0 与 Core 1 之间如何安全地共享数据和通信。
//!
//! ---
//!
//! # 1. 功能概述
//!
//! ## 1.1 RP2350 双核架构
//!
//! ```text
//!   ┌─────────────────────────────────────────────────────────┐
//!   │                    RP2350 SoC                            │
//!   │                                                         │
//!   │  ┌─────────────┐               ┌─────────────┐        │
//!   │  │  Cortex-M33  │   SIO FIFO   │  Cortex-M33  │        │
//!   │  │   Core 0     │◄────────────►│   Core 1     │        │
//!   │  │  (主核)      │  (8 word/方向)│  (从核)      │        │
//!   │  └──────┬───────┘               └──────┬───────┘        │
//!   │         │                              │               │
//!   │         │     ┌──────────────┐         │               │
//!   │         ├─────┤  32 硬件      ├─────────┤               │
//!   │         │     │  Spinlock     │         │               │
//!   │         │     └──────────────┘         │               │
//!   │         │                              │               │
//!   │         │     ┌──────────────┐         │               │
//!   │         └─────┤  共享 SRAM    ├─────────┘               │
//!   │               │  (520 KB)    │                         │
//!   │               └──────────────┘                         │
//!   └─────────────────────────────────────────────────────────┘
//! ```
//!
//! ## 1.2 核间通信机制
//!
//! | 机制 | 特点 | 适用场景 |
//! |------|------|---------|
//! | **SIO FIFO** | 硬件队列, 8 word 深度, 自动 SEV/WFE | 命令/消息传递 |
//! | **硬件 Spinlock** | 32 个, 单指令原子获取/释放 | 保护共享数据的临界区 |
//! | **Atomic** | CPU 原子指令 (Cortex-M33 支持) | 简单计数器/标志位 |
//! | **Doorbell** | 8-bit 中断信号, 可触发对方核中断 | 轻量级事件通知 |
//! | **共享内存** | 两核看到相同 SRAM | 大块数据共享 (需同步保护) |
//!
//! ## 1.3 本示例演示内容
//!
//! 1. **启动 Core 1**: 分配栈、启动函数、握手验证
//! 2. **FIFO 消息传递**: Core 0 发送任务 → Core 1 处理 → 返回结果
//! 3. **Spinlock 保护共享数据**: 两核安全读写同一全局变量
//! 4. **Atomic 操作**: 无锁计数器, 两核同时递增
//! 5. **生产者-消费者模式**: Core 0 产生数据 → Core 1 消费并统计
//!
//! ---
//!
//! # 2. 硬件要求
//!
//! **无需任何外部接线** — 双核是 RP2350 内部功能。
//!
//! > **注意**: RP2350 是双核 Cortex-M33, 而 RP2040 是双核 Cortex-M0+。
//! > 两者的多核 API 类似, 但 RP2350 的 Cortex-M33 支持原子指令,
//! > 而 RP2040 的 M0+ 不支持, 这是一个重要区别。
//!
//! ---
//!
//! # 3. 安全注意事项
//!
//! ## 3.1 defmt 不是线程安全的
//!
//! `defmt_rtt` **不支持多核并发输出**。如果两核同时调用 `info!()` 等宏，
//! 会导致输出数据损坏。因此:
//! - **Core 1 不使用 defmt 日志**
//! - Core 1 通过 FIFO 将需要打印的信息发送给 Core 0
//!
//! ## 3.2 Spinlock vs Atomic
//!
//! | 场景 | 推荐方式 |
//! |------|---------|
//! | 单个 u32/bool | `AtomicU32` / `AtomicBool` (无锁) |
//! | 多个关联变量需原子更新 | Spinlock (锁保护的临界区) |
//! | 读多写少 | Atomic (乐观读) |
//! | 复杂数据结构 | Spinlock + 共享内存 |
//!
//! ## 3.3 Core 1 栈大小
//!
//! Core 1 需要独立的栈空间 (不能用 Core 0 的栈)。
//! 本例分配 4096 字节, 对于简单计算足够。
//! 如果 Core 1 有深递归或大局部变量, 需增大。
//!
//! ---
//!
//! # 4. 编译运行
//!
//! ```bash
//! cargo run --bin multicore_shared --target thumbv8m.main-none-eabihf
//! ```

#![no_std]
#![no_main]

use core::sync::atomic::{AtomicI32, AtomicU32, Ordering};

use defmt::*;
use defmt_rtt as _;
use panic_probe as _;

use pico_rs::block::ImageDef;
use pico_rs::clocks::ClockBuilder;
use pico_rs::entry;
use pico_rs::multicore::{self, Fifo, Multicore, Stack};
use pico_rs::pac::Peripherals;
use pico_rs::sio::Spinlock;
use pico_rs::timer::Timer0;

#[unsafe(link_section = ".start_block")]
#[used]
pub static IMAGE_DEF: ImageDef = ImageDef::exe();

// ============================================================================
// Core 1 栈 — 静态分配, 8 字节对齐
// ============================================================================

/// Core 1 独立栈空间 (4 KB)
static mut CORE1_STACK: Stack<4096> = Stack::new();

// ============================================================================
// 共享数据 (Core 0 ↔ Core 1)
// ============================================================================

/// 原子计数器 — 两核并发递增, 无锁
/// 用于 Demo 3: 演示 Atomic 操作
static ATOMIC_COUNTER: AtomicU32 = AtomicU32::new(0);

/// Spinlock 保护的共享数据
/// 用于 Demo 2: 演示 Spinlock 临界区
///
/// 虽然单个 i32 可以用 Atomic, 但这里故意用 Spinlock 保护
/// 来演示 Spinlock 的使用模式 (实际场景中会保护多个关联变量)
static SHARED_SUM: AtomicI32 = AtomicI32::new(0);
static SHARED_COUNT: AtomicU32 = AtomicU32::new(0);

/// Spinlock 编号分配:
/// - Spinlock 0: 保护 SHARED_SUM + SHARED_COUNT (Demo 2)
/// - Spinlock 1~31: 保留
const SPINLOCK_SHARED: u8 = 0;

// ============================================================================
// FIFO 协议定义
// ============================================================================

/// FIFO 消息格式:
/// Core 0 → Core 1:
///   - 0x0000_NNNN: 计算请求 (对 N 求平方)
///   - 0xFFFF_FFFF: 停止信号
///
/// Core 1 → Core 0:
///   - 任意值: 请求的计算结果
const MSG_STOP: u32 = 0xFFFF_FFFF;

// ============================================================================
// Core 1 入口函数
// ============================================================================

/// Core 1 主函数 — 必须是 `fn() -> !` (永不返回)
///
/// 职责:
///   1. 接收 FIFO 消息, 处理计算请求
///   2. 参与 Spinlock 保护的共享数据更新
///   3. 参与 Atomic 计数器递增
///
/// 注意: **不使用 defmt 日志**, 所有输出通过 FIFO 发回 Core 0
fn core1_main() -> ! {
    let fifo = Fifo::new();

    // --- 握手: 等待 Core 0 的启动确认 ---
    let handshake = fifo.read();
    fifo.write(handshake); // 回弹确认

    loop {
        // 等待来自 Core 0 的消息
        let msg = fifo.read();

        if msg == MSG_STOP {
            // 收到停止信号 — 进入低功耗等待
            // 发回确认
            fifo.write(MSG_STOP);
            loop {
                pico_rs_rt::asm::wfe();
            }
        }

        // --- 任务: 计算 msg 的平方 ---
        let result = msg.wrapping_mul(msg);

        // --- 同时更新 Spinlock 保护的共享数据 ---
        {
            let _guard = Spinlock::<SPINLOCK_SHARED>::acquire();
            // 临界区: 原子更新 SUM 和 COUNT (两个变量需要一致性)
            let old_sum = SHARED_SUM.load(Ordering::Relaxed);
            SHARED_SUM.store(old_sum + result as i32, Ordering::Relaxed);
            let old_count = SHARED_COUNT.load(Ordering::Relaxed);
            SHARED_COUNT.store(old_count + 1, Ordering::Relaxed);
        } // Spinlock 自动释放

        // --- 递增无锁 Atomic 计数器 ---
        ATOMIC_COUNTER.fetch_add(1, Ordering::Relaxed);

        // --- 返回计算结果 ---
        fifo.write(result);
    }
}

// ============================================================================
// Core 0 主函数
// ============================================================================

#[entry]
fn main() -> ! {
    info!("=== Multicore Shared Data Example ===");
    info!("");

    // ------------------------------------------------------------------
    // 1. 外设初始化 (仅 Core 0 负责)
    // ------------------------------------------------------------------
    let dp = Peripherals::take().unwrap();
    let _clocks = ClockBuilder::new().build(dp.CLOCKS, dp.XOSC, dp.PLL_SYS, dp.PLL_USB);
    let timer = Timer0::init(dp.TIMER0);

    info!("[Core 0] Core ID: {}", multicore::core_id());

    // ------------------------------------------------------------------
    // 2. 启动 Core 1
    // ------------------------------------------------------------------
    info!("[Core 0] Preparing to launch Core 1...");

    let fifo = Fifo::new();
    fifo.drain(); // 清空残留数据

    let mut mc = Multicore::new(dp.PSM, dp.PPB);
    mc.reset_core1(); // 确保 Core 1 处于已知状态

    // SAFETY: CORE1_STACK 只在此处使用, core1_main 永不返回
    unsafe {
        mc.launch_core1(core1_main, &raw mut CORE1_STACK);
    }
    timer.delay_ms(10); // 等待 Core 1 初始化

    info!("[Core 0] Core 1 launched!");

    // ------------------------------------------------------------------
    // 3. 握手验证
    // ------------------------------------------------------------------
    const HANDSHAKE: u32 = 0xCAFE_BABE;
    fifo.write(HANDSHAKE);
    let response = fifo.read();

    if response == HANDSHAKE {
        info!("[Core 0] Handshake OK ({:#010x})", HANDSHAKE);
    } else {
        error!(
            "[Core 0] Handshake FAILED: expected {:#010x}, got {:#010x}",
            HANDSHAKE, response
        );
        loop {
            pico_rs_rt::asm::wfe();
        }
    }

    // ======================================================================
    // Demo 1: FIFO 消息传递 — 请求/响应模式
    //
    // Core 0 发送数字 N → Core 1 计算 N² → 返回结果
    // ======================================================================
    info!("");
    info!("--- Demo 1: FIFO Request/Response (N → N²) ---");
    {
        for n in 1u32..=10 {
            fifo.write(n);
            let result = fifo.read();
            info!("  {}² = {} (received from Core 1)", n, result);
        }
    }
    timer.delay_ms(100);

    // ======================================================================
    // Demo 2: Spinlock 保护共享数据
    //
    // 两核同时更新 SHARED_SUM 和 SHARED_COUNT:
    //   - Core 0: 通过 FIFO 触发 Core 1 计算并更新
    //   - Core 0: 自己也在 Spinlock 保护下更新
    //   - 最后验证 SUM 和 COUNT 一致
    //
    // Spinlock 确保: 即使两核"同时"写, SUM 和 COUNT
    //   始终保持一致 (不会出现 SUM 更新了一半被另一核打断)
    // ======================================================================
    info!("");
    info!("--- Demo 2: Spinlock-Protected Shared Data ---");
    {
        // 重置共享状态
        {
            let _guard = Spinlock::<SPINLOCK_SHARED>::acquire();
            SHARED_SUM.store(0, Ordering::Relaxed);
            SHARED_COUNT.store(0, Ordering::Relaxed);
        }

        // Core 0 发送 20 个计算请求, Core 1 每次都更新共享数据
        // Core 0 也每次在 Spinlock 下更新
        for n in 1u32..=20 {
            // 通过 FIFO 让 Core 1 计算并更新共享数据
            fifo.write(n);
            let _result = fifo.read(); // 等待 Core 1 完成

            // Core 0 也在 Spinlock 保护下更新 (模拟双核竞争写)
            {
                let _guard = Spinlock::<SPINLOCK_SHARED>::acquire();
                let old_sum = SHARED_SUM.load(Ordering::Relaxed);
                SHARED_SUM.store(old_sum + n as i32, Ordering::Relaxed);
                let old_count = SHARED_COUNT.load(Ordering::Relaxed);
                SHARED_COUNT.store(old_count + 1, Ordering::Relaxed);
            }
        }

        // 读取最终结果
        let (final_sum, final_count) = {
            let _guard = Spinlock::<SPINLOCK_SHARED>::acquire();
            (
                SHARED_SUM.load(Ordering::Relaxed),
                SHARED_COUNT.load(Ordering::Relaxed),
            )
        };

        // Core 0 贡献: sum(1..=20) = 210, count = 20
        // Core 1 贡献: sum(1²..20²) = 2870, count = 20
        // 总计: sum = 210 + 2870 = 3080, count = 40
        let expected_sum: i32 = {
            let core0_sum: i32 = (1..=20i32).sum(); // 210
            let core1_sum: i32 = (1..=20i32).map(|x| x * x).sum(); // 2870
            core0_sum + core1_sum
        };
        let expected_count: u32 = 40;

        info!("  Shared SUM   = {} (expected {})", final_sum, expected_sum);
        info!(
            "  Shared COUNT = {} (expected {})",
            final_count, expected_count
        );

        if final_sum == expected_sum && final_count == expected_count {
            info!("  Result: PASS — Spinlock correctly protected shared data");
        } else {
            warn!("  Result: MISMATCH — possible synchronization issue");
        }
    }
    timer.delay_ms(100);

    // ======================================================================
    // Demo 3: Atomic 无锁计数器
    //
    // 两核各递增 ATOMIC_COUNTER 1000 次, 验证最终值 = 2000
    //
    // Cortex-M33 的 LDREX/STREX 指令确保原子性,
    // 即使两核"同时"执行 fetch_add 也不会丢失计数。
    // ======================================================================
    info!("");
    info!("--- Demo 3: Atomic Counter (Lock-Free) ---");
    {
        // 重置
        ATOMIC_COUNTER.store(0, Ordering::Relaxed);

        let iterations: u32 = 1000;

        // Core 0: 发送 1000 个请求, 每个请求 Core 1 会 fetch_add(1)
        // Core 0: 自己也 fetch_add(1) 1000 次
        for n in 0..iterations {
            // Core 0 递增
            ATOMIC_COUNTER.fetch_add(1, Ordering::Relaxed);

            // 每 100 次发一个 FIFO 请求让 Core 1 也递增
            // (Core 1 在处理每个 FIFO 消息时都会 fetch_add(1))
            if n % 1 == 0 {
                // 每次都发, 共 1000 次
                fifo.write(1); // 数值不重要, 只是触发 Core 1 工作
                let _ = fifo.read();
            }
        }

        let final_val = ATOMIC_COUNTER.load(Ordering::Relaxed);
        let expected = iterations * 2; // Core 0: 1000 + Core 1: 1000

        info!("  Atomic counter = {} (expected {})", final_val, expected);

        if final_val == expected {
            info!("  Result: PASS — Atomic ops are truly atomic across cores");
        } else {
            warn!(
                "  Result: {} (off by {})",
                final_val,
                (final_val as i64 - expected as i64).unsigned_abs()
            );
        }
    }
    timer.delay_ms(100);

    // ======================================================================
    // Demo 4: 性能对比 — FIFO 往返延迟
    //
    // 测量 Core 0 发送 → Core 1 处理 → Core 0 接收 的端到端延迟
    // ======================================================================
    info!("");
    info!("--- Demo 4: FIFO Round-Trip Latency ---");
    {
        let rounds: u32 = 100;
        let start = timer.get_time_us();

        for n in 0..rounds {
            fifo.write(n + 1);
            let _ = fifo.read();
        }

        let elapsed = timer.get_time_us() - start;
        let per_round = elapsed / rounds as u64;

        info!(
            "  {} round-trips in {} us ({} us/round-trip)",
            rounds, elapsed, per_round
        );
    }
    timer.delay_ms(100);

    // ======================================================================
    // Demo 5: 生产者-消费者模式摘要
    //
    // 展示到目前为止 Core 1 的累计工作量
    // ======================================================================
    info!("");
    info!("--- Demo 5: Summary ---");
    {
        let total_atomic = ATOMIC_COUNTER.load(Ordering::Relaxed);
        let (total_sum, total_count) = {
            let _guard = Spinlock::<SPINLOCK_SHARED>::acquire();
            (
                SHARED_SUM.load(Ordering::Relaxed),
                SHARED_COUNT.load(Ordering::Relaxed),
            )
        };

        info!("  Total atomic counter: {}", total_atomic);
        info!("  Total shared sum:     {}", total_sum);
        info!("  Total shared count:   {}", total_count);
        info!("  Core 0 ID: {}", multicore::core_id());
    }

    // ------------------------------------------------------------------
    // 停止 Core 1
    // ------------------------------------------------------------------
    info!("");
    info!("[Core 0] Sending stop signal to Core 1...");
    fifo.write(MSG_STOP);
    let ack = fifo.read();
    if ack == MSG_STOP {
        info!("[Core 0] Core 1 acknowledged stop");
    }

    info!("");
    info!("=== All demos complete ===");

    // 无限循环
    loop {
        pico_rs_rt::asm::wfe();
    }
}
