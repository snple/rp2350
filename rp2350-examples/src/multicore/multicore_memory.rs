//! # 双核共享内存测试
//!
//! 基于 RP2350 (Raspberry Pi Pico 2) 的 **共享 SRAM** 读写测试，
//! 验证双核通过共享内存交换数据的正确性和性能。
//!
//! ---
//!
//! # 1. 功能概述
//!
//! 与 `multicore_shared` 不同，本示例**不通过 FIFO 传递数据**，
//! 而是将数据直接放在共享 SRAM 中，Core 0 和 Core 1 直接读写。
//! FIFO 仅用于同步信号（通知对方"数据已就绪"）。
//!
//! ## 1.1 演示内容
//!
//! | Demo | 名称 | 内容 |
//! |------|------|------|
//! | 1 | 批量数据传输 | Core 0 写 1KB 到共享 buf → Core 1 校验 checksum |
//! | 2 | 乒乓缓冲 | 双 buffer，一核写另一核读，交替进行 |
//! | 3 | 并发写+Spinlock | 两核同时写同一个 buffer 的不同区域 |
//! | 4 | 内存一致性 | 验证 compiler_fence / atomic fence 的必要性 |
//! | 5 | 带宽测试 | 测量共享内存读写吞吐量 |
//! | 6 | Atomic Release-Acquire | 纯 Atomic 标志位同步 (不用 FIFO) |
//! | 7 | Atomic 环形缓冲 | 无锁 SPSC ring buffer (Atomic 读写指针) |
//!
//! ## 1.2 与 `multicore_shared` 的区别
//!
//! | 特性 | multicore_shared | multicore_memory |
//! |------|-----------------|------------------|
//! | 数据路径 | FIFO 消息传递 | 共享 SRAM 直接读写 |
//! | 数据量 | 单字 (4 bytes) | KB 级别 buffer |
//! | FIFO 用途 | 传递数据 | 仅传递同步信号 |
//! | 适用场景 | 命令/小数据 | 图像帧、传感器数据、音频 buffer |
//!
//! ---
//!
//! # 2. 硬件要求
//!
//! 无需任何外部接线。共享 SRAM 是 RP2350 内部资源。
//!
//! ---
//!
//! # 3. 内存模型说明
//!
//! ## 3.1 RP2350 SRAM 布局
//!
//! RP2350 有 520 KB SRAM，分布在 8 个 bank (SRAM0-SRAM7) +
//! 2 个 4KB scratch bank (SRAM8-SRAM9)：
//!
//! ```text
//!   0x2000_0000  ┌────────────┐
//!                │  SRAM0-7   │  512 KB (striped, 4-way)
//!                │  (主 RAM)   │
//!   0x2008_0000  ├────────────┤
//!                │  SRAM8     │  4 KB (scratch)
//!   0x2008_1000  ├────────────┤
//!                │  SRAM9     │  4 KB (scratch)
//!   0x2008_2000  └────────────┘
//! ```
//!
//! 两个核心看到**完全相同的 SRAM 地址空间** — 这是真正的共享内存。
//!
//! ## 3.2 缓存一致性
//!
//! Cortex-M33 **没有硬件缓存一致性协议**。如果启用了 cache，
//! 需要在核心间传递数据前执行 cache 维护操作。本示例默认
//! 不使用 cache 来简化问题。
//!
//! ## 3.3 内存屏障
//!
//! ARMv8-M 是弱内存序架构。即使没有 cache，以下情况仍需内存屏障：
//!
//! - Core 0 写入数据 → 发 FIFO 通知 → Core 1 读数据
//!   **必须在写数据和发 FIFO 之间插入 `compiler_fence`**，
//!   否则编译器可能重排指令顺序。
//!
//! - 本示例使用 FIFO 的硬件语义：Core 1 的 `fifo.read()` 返回
//!   时，Core 0 的 `fifo.write()` 已完成，形成了隐式的
//!   happens-before 关系。但对于纯共享内存（无 FIFO），
//!   必须显式使用 fence。
//!
//! ---
//!
//! # 4. 编译运行
//!
//! ```bash
//! cargo run --bin multicore_memory --target thumbv8m.main-none-eabihf
//! ```

#![no_std]
#![no_main]

use core::cell::UnsafeCell;
use core::sync::atomic::{AtomicBool, AtomicU32, Ordering, compiler_fence};

#[cfg(feature = "defmt")]
use defmt_rtt as _;
use rp2350_examples::log::*;

use rp2350::clocks::ClockBuilder;
use rp2350::main;
use rp2350::multicore::{self, Fifo, Multicore, Stack};
use rp2350::pac::Peripherals;
use rp2350::sio::Spinlock;
use rp2350::timer::Timer0;

// ============================================================================
// Core 1 栈
// ============================================================================

static mut CORE1_STACK: Stack<4096> = Stack::new();

// ============================================================================
// 共享内存区域
// ============================================================================

/// 共享数据缓冲区 (1 KB)
///
/// 使用 `UnsafeCell` 声明可变共享语义。
/// 两个核心都可以读写这块内存，必须通过 spinlock 或协议同步。
///
/// 在 `.bss` 段，启动时自动清零。
#[repr(C, align(32))]
struct SharedBuffer {
    data: UnsafeCell<[u32; 256]>, // 256 × 4 = 1024 bytes
}

// SAFETY: RP2350 双核共享同一 SRAM 地址空间，
// 此 static 在两核上看到相同物理内存。
// 安全性由 spinlock / FIFO 协议保证。
unsafe impl Sync for SharedBuffer {}

static SHARED_BUF: SharedBuffer = SharedBuffer {
    data: UnsafeCell::new([0u32; 256]),
};

/// 乒乓缓冲：两个独立的 512 字节缓冲区
#[repr(C, align(32))]
struct PingPongBuffers {
    buf_a: UnsafeCell<[u32; 128]>, // 128 × 4 = 512 bytes
    buf_b: UnsafeCell<[u32; 128]>,
}

unsafe impl Sync for PingPongBuffers {}

static PINGPONG: PingPongBuffers = PingPongBuffers {
    buf_a: UnsafeCell::new([0u32; 128]),
    buf_b: UnsafeCell::new([0u32; 128]),
};

// ============================================================================
// 同步原语
// ============================================================================

/// Core 0 已写入数据的标志 (由 Core 0 设置, Core 1 读取)
static DATA_READY: AtomicBool = AtomicBool::new(false);

/// Core 1 已完成处理的标志 (由 Core 1 设置, Core 0 读取)
static DATA_DONE: AtomicBool = AtomicBool::new(false);

/// 无锁环形缓冲区 (SPSC: single-producer, single-consumer)
const RINGBUF_CAP: usize = 64;

#[repr(C, align(32))]
struct AtomicRingBuf {
    data: UnsafeCell<[u32; RINGBUF_CAP]>,
    /// Core 0 (producer) 写入指针
    write_idx: AtomicU32,
    /// Core 1 (consumer) 读取指针
    read_idx: AtomicU32,
}

unsafe impl Sync for AtomicRingBuf {}

static RINGBUF: AtomicRingBuf = AtomicRingBuf {
    data: UnsafeCell::new([0u32; RINGBUF_CAP]),
    write_idx: AtomicU32::new(0),
    read_idx: AtomicU32::new(0),
};

/// Ring buffer producer 计划投放的总条目数（Core 0 在启动前设置）
static RINGBUF_TOTAL: AtomicU32 = AtomicU32::new(0);

/// Spinlock 分配
const SPINLOCK_BUF: u8 = 1; // 保护 SHARED_BUF 的并发写

// ============================================================================
// FIFO 协议
// ============================================================================

/// FIFO 命令 (仅用于同步, 数据在共享内存中)
///
/// Core 0 → Core 1:
const CMD_WRITE_DONE: u32 = 0xA000_0001; // "我已写入共享 buffer，请校验"
const CMD_PINGPONG_A: u32 = 0xA000_0002; // "请处理 buf_a"
const CMD_PINGPONG_B: u32 = 0xA000_0003; // "请处理 buf_b"
const CMD_FILL_PATTERN: u32 = 0xA000_0004; // "请在 buffer 中填充指定模式"
const CMD_ATOMIC_FLAG_WAIT: u32 = 0xA000_0005; // "进入 Atomic 标志位 spin-wait 模式"
const CMD_RINGBUF_CONSUME: u32 = 0xA000_0006; // "进入 ring buffer consumer 模式"
const CMD_FIFO_BACK: u32 = 0xA000_0007; // "回到 FIFO 模式"
const CMD_STOP: u32 = 0xFFFF_FFFF;

/// Core 1 → Core 0:
const RSP_DONE: u32 = 0xB000_0003;

// ============================================================================
// 工具函数
// ============================================================================

/// 计算 u32 数组的简单 checksum (XOR 累加)
fn checksum_u32(data: &[u32]) -> u32 {
    let mut cs: u32 = 0;
    for &w in data {
        cs ^= w;
        cs = cs.wrapping_add(w);
    }
    cs
}

/// 用确定性模式填充 u32 数组
fn fill_pattern(data: &mut [u32], seed: u32) {
    for (i, w) in data.iter_mut().enumerate() {
        *w = seed.wrapping_mul(i as u32 + 1).wrapping_add(i as u32);
    }
}

// ============================================================================
// Core 1 入口函数
// ============================================================================

fn core1_main() -> ! {
    // Core 1 不走 startup::init()，需手动开启跨核 LDREX/STREX
    rp2350::ppb::Actlr::enable_crosscore_atomics();

    let fifo = unsafe { Fifo::steal() };

    // 握手
    let handshake = fifo.read();
    fifo.write(handshake);

    loop {
        let cmd = fifo.read();

        match cmd {
            // ----------------------------------------------------------
            // Demo 1: 校验共享 buffer 的 checksum
            // ----------------------------------------------------------
            CMD_WRITE_DONE => {
                compiler_fence(Ordering::SeqCst);

                let data = unsafe { &*SHARED_BUF.data.get() };
                let cs = checksum_u32(data);

                compiler_fence(Ordering::SeqCst);

                fifo.write(cs);
            }

            // ----------------------------------------------------------
            // Demo 2: 乒乓缓冲 — 处理 buf_a
            // ----------------------------------------------------------
            CMD_PINGPONG_A => {
                compiler_fence(Ordering::SeqCst);

                let data = unsafe { &*PINGPONG.buf_a.get() };
                let cs = checksum_u32(data);

                // Core 1 将自己的结果写回 buf_b (供 Core 0 验证)
                let buf_b = unsafe { &mut *PINGPONG.buf_b.get() };
                for (i, w) in buf_b.iter_mut().enumerate() {
                    *w = data[i].wrapping_mul(2).wrapping_add(cs);
                }

                compiler_fence(Ordering::SeqCst);
                fifo.write(RSP_DONE);
            }

            // ----------------------------------------------------------
            // Demo 2: 乒乓缓冲 — 处理 buf_b
            // ----------------------------------------------------------
            CMD_PINGPONG_B => {
                compiler_fence(Ordering::SeqCst);

                let data = unsafe { &*PINGPONG.buf_b.get() };
                let cs = checksum_u32(data);

                let buf_a = unsafe { &mut *PINGPONG.buf_a.get() };
                for (i, w) in buf_a.iter_mut().enumerate() {
                    *w = data[i].wrapping_add(cs).wrapping_mul(3);
                }

                compiler_fence(Ordering::SeqCst);
                fifo.write(RSP_DONE);
            }

            // ----------------------------------------------------------
            // Demo 3: 并发写 — 在 spinlock 保护下写入
            // ----------------------------------------------------------
            CMD_FILL_PATTERN => {
                let _guard = Spinlock::<SPINLOCK_BUF>::acquire();

                let data = unsafe { &mut *SHARED_BUF.data.get() };

                // Core 1 写入后半部分 (索引 128..256)
                let seed: u32 = 0xDEAD_BEEF;
                #[allow(clippy::needless_range_loop)]
                for i in 128..256 {
                    data[i] = seed.wrapping_mul(i as u32 + 1);
                }

                compiler_fence(Ordering::SeqCst);
                // spinlock 在 _guard drop 时释放
                fifo.write(RSP_DONE);
            }

            // ----------------------------------------------------------
            // Demo 6: Atomic Release-Acquire 标志位同步
            //
            // Core 1 不再阻塞在 fifo.read(), 而是 spin-wait
            // 共享的 Atomic 标志位。这是"纯共享内存"同步模式。
            // ----------------------------------------------------------
            CMD_ATOMIC_FLAG_WAIT => {
                fifo.write(RSP_DONE); // 确认进入 atomic 模式

                // 等待 Core 0 设置 DATA_READY (Release store)
                while !DATA_READY.load(Ordering::Acquire) {
                    rp2350_rt::asm::wfe();
                }

                // Acquire 确保了: 看到 DATA_READY==true 时,
                // Core 0 对 SHARED_BUF 的所有写入都可见
                compiler_fence(Ordering::Acquire);

                let data = unsafe { &*SHARED_BUF.data.get() };
                let cs = checksum_u32(data);

                // 将结果写入 buffer 前半部分供 Core 0 验证
                let first_word = unsafe { &mut *SHARED_BUF.data.get() };
                first_word[0] = cs;

                compiler_fence(Ordering::Release);

                // 通知 Core 0: 处理完成 (Release store)
                DATA_READY.store(false, Ordering::Relaxed);
                DATA_DONE.store(true, Ordering::Release);
                // 唤醒 Core 0 的 WFE
                rp2350_rt::asm::sev();

                // 回到 FIFO 模式
                // (Core 0 会在收到 DATA_DONE 后发 CMD_FIFO_BACK)
            }

            // ----------------------------------------------------------
            // Demo 7: 无锁 Ring Buffer Consumer
            // ----------------------------------------------------------
            CMD_RINGBUF_CONSUME => {
                let data = unsafe { &*RINGBUF.data.get() };
                let total = RINGBUF_TOTAL.load(Ordering::Acquire);

                let mut consumed = 0u32;
                let mut sum: u32 = 0;

                loop {
                    let w = RINGBUF.write_idx.load(Ordering::Acquire);
                    let r = RINGBUF.read_idx.load(Ordering::Relaxed);

                    if r >= w {
                        // 队列空 — 所有条目已消费完？
                        if consumed >= total {
                            break;
                        }
                        // 还没产完，等待 producer 的 SEV
                        rp2350_rt::asm::wfe();
                        continue;
                    }

                    let val = data[(r as usize) % RINGBUF_CAP];
                    sum = sum.wrapping_add(val);

                    // Release: 确保读取完成后再更新 read_idx
                    RINGBUF.read_idx.store(r + 1, Ordering::Release);
                    consumed += 1;
                }

                // 返回 sum 给 Core 0
                fifo.write(sum);
            }

            // ----------------------------------------------------------
            // 回到 FIFO 模式
            // ----------------------------------------------------------
            CMD_FIFO_BACK => {
                // 已经在 fifo.read() 循环中，无需操作
                fifo.write(RSP_DONE);
            }

            // ----------------------------------------------------------
            // 停止信号
            // ----------------------------------------------------------
            CMD_STOP => {
                fifo.write(CMD_STOP);
                loop {
                    rp2350_rt::asm::wfe();
                }
            }

            _ => {
                // 未知命令，回显
                fifo.write(cmd);
            }
        }
    }
}

// ============================================================================
// Core 0 主函数
// ============================================================================

#[main]
fn main() -> ! {
    info!("=== Multicore Memory Sharing Test ===");
    info!("");

    // ------------------------------------------------------------------
    // 1. 外设初始化
    // ------------------------------------------------------------------
    let dp = Peripherals::take().unwrap();
    let _clocks = ClockBuilder::new().build(dp.CLOCKS, dp.XOSC, dp.PLL_SYS, dp.PLL_USB);
    let timer = Timer0::init(dp.TIMER0);

    info!("[Core 0] {}", multicore::core_id());

    // ------------------------------------------------------------------
    // 2. 启动 Core 1
    // ------------------------------------------------------------------
    let fifo = unsafe { Fifo::steal() };
    fifo.drain();

    let mut mc = Multicore::init(dp.PSM, dp.PPB);

    unsafe {
        mc.launch_core1(core1_main, &raw mut CORE1_STACK);
    }
    timer.delay_ms(10);

    // 握手
    const HANDSHAKE: u32 = 0xCAFE_BABE;
    fifo.write(HANDSHAKE);
    let response = fifo.read();
    if response == HANDSHAKE {
        info!("[Core 0] Handshake OK");
    } else {
        error!("[Core 0] Handshake FAILED");
        loop {
            rp2350_rt::asm::wfe();
        }
    }

    // ======================================================================
    // Demo 1: 批量数据传输 + Checksum 校验
    //
    // Core 0 填充 1KB 共享 buffer → 通知 Core 1 → Core 1 计算 checksum 返回
    // Core 0 也计算 checksum 并比对 → 验证数据完整性
    // ======================================================================
    info!("");
    info!("--- Demo 1: Bulk Data Transfer (1 KB) ---");
    {
        // Core 0 填充 buffer
        let data = unsafe { &mut *SHARED_BUF.data.get() };
        fill_pattern(data, 0x1234_5678);

        // 本地计算 checksum
        let expected_cs = checksum_u32(data);

        // 确保写入在通知 Core 1 之前完成
        compiler_fence(Ordering::SeqCst);

        // 通知 Core 1 读取并校验
        fifo.write(CMD_WRITE_DONE);

        let cs_from_core1 = fifo.read();

        info!(
            "  Core 0 checksum: {:08x}, Core 1 checksum: {:08x}",
            expected_cs, cs_from_core1
        );

        if expected_cs == cs_from_core1 {
            info!("  Result: PASS — shared memory data integrity verified");
        } else {
            error!("  Result: FAIL — checksum mismatch!");
        }
    }
    timer.delay_ms(100);

    // ======================================================================
    // Demo 2: 乒乓缓冲
    //
    // Core 0 填充 buf_a → Core 1 处理 → 结果存入 buf_b
    // Core 0 验证 buf_b → Core 0 填充 buf_b → Core 1 处理 → 结果存入 buf_a
    // 交替进行 4 轮, 验证双向数据传递
    // ======================================================================
    info!("");
    info!("--- Demo 2: Ping-Pong Buffers ---");
    {
        let rounds = 4u32;
        let mut ok = 0u32;

        for r in 0..rounds {
            // Phase 1: Core 0 → buf_a → Core 1 → buf_b
            {
                let buf_a = unsafe { &mut *PINGPONG.buf_a.get() };
                fill_pattern(buf_a, r * 1000 + 1);

                // 记录 buf_a 的 checksum 用于后续验证
                let cs_a = checksum_u32(buf_a);

                compiler_fence(Ordering::SeqCst);
                fifo.write(CMD_PINGPONG_A);
                let _ = fifo.read(); // 等待 RSP_DONE

                // 验证 Core 1 写入 buf_b 的结果
                let buf_b = unsafe { &*PINGPONG.buf_b.get() };
                let mut all_ok = true;
                for (i, &w) in buf_b.iter().enumerate() {
                    let expected = buf_a[i].wrapping_mul(2).wrapping_add(cs_a);
                    if w != expected {
                        all_ok = false;
                        if i < 4 {
                            warn!(
                                "    Round {} buf_b[{}]: got {:08x}, expected {:08x}",
                                r, i, w, expected
                            );
                        }
                    }
                }
                if all_ok {
                    ok += 1;
                }
            }

            // Phase 2: Core 0 → buf_b → Core 1 → buf_a
            {
                let buf_b = unsafe { &mut *PINGPONG.buf_b.get() };
                fill_pattern(buf_b, r * 1000 + 2);

                let cs_b = checksum_u32(buf_b);

                compiler_fence(Ordering::SeqCst);
                fifo.write(CMD_PINGPONG_B);
                let _ = fifo.read(); // 等待 RSP_DONE

                let buf_a = unsafe { &*PINGPONG.buf_a.get() };
                let mut all_ok = true;
                for (i, &w) in buf_a.iter().enumerate() {
                    let expected = buf_b[i].wrapping_add(cs_b).wrapping_mul(3);
                    if w != expected {
                        all_ok = false;
                        if i < 4 {
                            warn!(
                                "    Round {} buf_a[{}]: got {:08x}, expected {:08x}",
                                r, i, w, expected
                            );
                        }
                    }
                }
                if all_ok {
                    ok += 1;
                }
            }
        }

        let total_phases = rounds * 2;
        info!("  {}/{} phases passed", ok, total_phases);
        if ok == total_phases {
            info!("  Result: PASS — ping-pong buffers work correctly");
        } else {
            warn!("  Result: {} failures detected", total_phases - ok);
        }
    }
    timer.delay_ms(100);

    // ======================================================================
    // Demo 3: 并发写 (Spinlock 保护的共享 buffer)
    //
    // Core 0 写 buffer[0..128], Core 1 写 buffer[128..256]
    // 两核同时进行, spinlock 保证写入不交错
    // 最后验证整个 buffer 的内容正确
    // ======================================================================
    info!("");
    info!("--- Demo 3: Concurrent Write to Shared Buffer ---");
    {
        let data = unsafe { &mut *SHARED_BUF.data.get() };

        // 清零 buffer
        data.fill(0);

        // 通知 Core 1 开始写入后半部分
        compiler_fence(Ordering::SeqCst);
        fifo.write(CMD_FILL_PATTERN);

        // Core 0 同时写入前半部分 (spinlock)
        {
            let _guard = Spinlock::<SPINLOCK_BUF>::acquire();

            let seed: u32 = 0xCAFE_F00D;
            #[allow(clippy::needless_range_loop)]
            for i in 0..128 {
                data[i] = seed.wrapping_mul(i as u32 + 1);
            }

            compiler_fence(Ordering::SeqCst);
        }

        // 等待 Core 1 完成
        let _ = fifo.read(); // RSP_DONE

        // 验证整个 buffer
        let mut errors = 0u32;
        // 验证 Core 0 写入区域
        let seed0: u32 = 0xCAFE_F00D;
        #[allow(clippy::needless_range_loop)]
        for i in 0..128 {
            let expected = seed0.wrapping_mul(i as u32 + 1);
            if data[i] != expected {
                if errors < 4 {
                    warn!(
                        "  buf[{}]: got {:08x}, expected {:08x}",
                        i, data[i], expected
                    );
                }
                errors += 1;
            }
        }
        // 验证 Core 1 写入区域
        let seed1: u32 = 0xDEAD_BEEF;
        #[allow(clippy::needless_range_loop)]
        for i in 128..256 {
            let expected = seed1.wrapping_mul(i as u32 + 1);
            if data[i] != expected {
                if errors < 4 {
                    warn!(
                        "  buf[{}]: got {:08x}, expected {:08x}",
                        i, data[i], expected
                    );
                }
                errors += 1;
            }
        }

        if errors == 0 {
            info!("  Result: PASS — concurrent writes correctly synchronized");
        } else {
            error!("  Result: FAIL — {} errors in shared buffer", errors);
        }
    }
    timer.delay_ms(100);

    // ======================================================================
    // Demo 4: 内存一致性 — 验证 compiler_fence 的必要性
    //
    // 对比测试：使用 fence vs 不使用 fence
    // 注意: 在 ARMv8-M + FIFO 同步的场景下，fence 是否真的需要
    // 取决于具体场景。本 demo 构造一个临界场景来演示。
    // ======================================================================
    info!("");
    info!("--- Demo 4: Memory Ordering / Fence ---");
    {
        let data = unsafe { &mut *SHARED_BUF.data.get() };

        // 场景: 无 fence 写入后立即发 FIFO
        // 在 LTO + 高优化级别下，编译器可能重排指令
        // 这里我们验证有 fence 的情况总是正确

        for trial in 0..10u32 {
            // 写入已知模式
            let seed = trial * 100;
            fill_pattern(data, seed);
            let expected = checksum_u32(data);

            // 关键: 在发 FIFO 之前加 fence，确保所有写操作已发出
            compiler_fence(Ordering::SeqCst);

            // 通知 Core 1
            fifo.write(CMD_WRITE_DONE);
            let cs = fifo.read();

            if cs != expected {
                error!(
                    "  Trial {}: checksum mismatch (got {:08x}, expected {:08x})",
                    trial, cs, expected
                );
            }
        }

        // 额外测试: 使用 Atomic 标志位代替 FIFO 同步
        // 这种情况下必须使用正确的 Ordering
        info!("  Testing Atomic flag synchronization...");

        data.fill(0xAAAAAAAA);
        compiler_fence(Ordering::SeqCst);
        DATA_READY.store(true, Ordering::Release);

        // 由于 Core 1 主循环不在轮询标志位，这里只是演示 API 用法
        // 真实场景中 Core 1 应 spin-wait DATA_READY
        info!("  DATA_READY flag set (Release ordering) — Core 1 would see the writes");

        // 重置
        DATA_READY.store(false, Ordering::Relaxed);
        DATA_DONE.store(false, Ordering::Relaxed);

        info!("  Result: fence demo complete (verify with -O3 / LTO builds)");
    }
    timer.delay_ms(100);

    // ======================================================================
    // Demo 5: 共享内存带宽测试
    //
    // Core 0 连续写入大量数据到共享 buffer，
    // 测量纯写入、纯读取、memcpy 的吞吐量。
    // ======================================================================
    info!("");
    info!("--- Demo 5: Shared Memory Bandwidth ---");
    {
        let data = unsafe { &mut *SHARED_BUF.data.get() };
        let buf_size = 1024u64; // bytes
        let iterations = 1000u64;
        let total_bytes = buf_size * iterations;

        // --- 5a: 纯写入 ---
        {
            let start = timer.time_us();

            for iter in 0..iterations {
                let seed = iter as u32;
                for (i, w) in data.iter_mut().enumerate() {
                    *w = seed.wrapping_mul(i as u32 + 1);
                }
                compiler_fence(Ordering::SeqCst);
            }

            let elapsed = timer.time_us() - start;
            let mbps = (total_bytes * 1_000_000 / elapsed) as u32;
            info!(
                "  Write: {} KB in {} us ({} MB/s)",
                total_bytes / 1024,
                elapsed,
                mbps
            );
        }

        // --- 5b: 纯读取 ---
        {
            let mut sink: u32 = 0;

            let start = timer.time_us();

            for _ in 0..iterations {
                for &w in data.iter() {
                    sink ^= w;
                }
                compiler_fence(Ordering::SeqCst);
            }

            let elapsed = timer.time_us() - start;
            let mbps = (total_bytes * 1_000_000 / elapsed) as u32;

            // 防止编译器优化掉 sink
            core::hint::black_box(sink);

            info!(
                "  Read:  {} KB in {} us ({} MB/s) [sink={:08x}]",
                total_bytes / 1024,
                elapsed,
                mbps,
                sink
            );
        }

        // --- 5c: 写后读 (模拟生产-消费) ---
        {
            let start = timer.time_us();

            for iter in 0..iterations {
                let seed = iter as u32 * 13;
                for (i, w) in data.iter_mut().enumerate() {
                    *w = seed.wrapping_add(i as u32);
                }
                compiler_fence(Ordering::SeqCst);

                let mut sink: u32 = 0;
                for &w in data.iter() {
                    sink ^= w;
                }
                core::hint::black_box(sink);
            }

            let elapsed = timer.time_us() - start;
            let mbps = (2u64 * total_bytes * 1_000_000 / elapsed) as u32;
            info!(
                "  R/W:   {} KB in {} us ({} MB/s effective)",
                total_bytes / 1024,
                elapsed,
                mbps
            );
        }
    }
    timer.delay_ms(100);

    // ======================================================================
    // Demo 6: Atomic Release-Acquire 标志位同步
    //
    // 这是"纯共享内存"同步模式 — 完全不依赖 FIFO 传递数据就绪信息。
    //
    // 协议:
    //   1. Core 0 发送 CMD_ATOMIC_FLAG_WAIT → Core 1 进入 spin-wait 模式
    //   2. Core 0 写 SHARED_BUF, 然后 DATA_READY.store(true, Release)
    //   3. Core 1 的 DATA_READY.load(Acquire) 返回 true, 此时所有写可见
    //   4. Core 1 读 SHARED_BUF, 计算 checksum, 写回 buf[0]
    //   5. Core 1 设置 DATA_DONE.store(true, Release)
    //   6. Core 0 的 DATA_DONE.load(Acquire) 返回 true, 读取结果验证
    //
    // 关键: Release-Acquire 对确保 happens-before, 不依赖 FIFO 硬件同步
    // ======================================================================
    info!("");
    info!("--- Demo 6: Atomic Release-Acquire Flags (No FIFO) ---");
    {
        // 步骤 1: 让 Core 1 进入 Atomic 标志位 spin-wait 模式
        fifo.write(CMD_ATOMIC_FLAG_WAIT);
        let _ = fifo.read(); // RSP_DONE: Core 1 已就绪

        // 步骤 2: Core 0 写入数据
        let data = unsafe { &mut *SHARED_BUF.data.get() };
        fill_pattern(data, 0xDEAD_BEEF);
        let expected_cs = checksum_u32(data);

        // 步骤 3: 确保所有写入完成，然后 Release store 通知 Core 1
        // Release: 保证之前的所有内存写入不会被重排到此 store 之后
        compiler_fence(Ordering::Release);
        DATA_READY.store(true, Ordering::Release);
        // 关键: store 不会自动产生 SEV，必须显式发送事件唤醒 Core 1 的 WFE
        rp2350_rt::asm::sev();

        // 步骤 4-5: Core 1 此时在处理... 我们 spin-wait DONE 标志
        // Acquire: 看到 DONE==true 后，Core 1 的所有写入都保证可见
        while !DATA_DONE.load(Ordering::Acquire) {
            rp2350_rt::asm::wfe();
        }

        // 步骤 6: 验证 Core 1 的结果 (Core 1 把 checksum 写到了 buf[0])
        let cs_from_core1 = unsafe { (*SHARED_BUF.data.get())[0] };

        info!(
            "  Core 0 checksum: {:08x}, Core 1 checksum: {:08x}",
            expected_cs, cs_from_core1
        );

        if expected_cs == cs_from_core1 {
            info!("  Result: PASS — Release-Acquire sync works across cores");
        } else {
            error!("  Result: FAIL — data not visible across cores!");
        }

        // 清理标志位
        DATA_READY.store(false, Ordering::Relaxed);
        DATA_DONE.store(false, Ordering::Relaxed);

        // 让 Core 1 回到 FIFO 模式
        fifo.write(CMD_FIFO_BACK);
        let _ = fifo.read(); // RSP_DONE
    }
    timer.delay_ms(100);

    // ======================================================================
    // Demo 7: 无锁 Atomic Ring Buffer (SPSC)
    //
    // Core 0 (producer) 写入 ring buffer, Core 1 (consumer) 读取。
    // 使用 AtomicU32 读写指针实现无锁同步。
    //
    // 内存序约定:
    //   - Producer write_idx.store(..., Release): 确保数据写入先于指针更新
    //   - Consumer write_idx.load(..., Acquire): 确保看到指针更新后数据可见
    //   - Consumer read_idx.store(..., Release): 确保读取完成后才更新
    //   - Producer read_idx.load(..., Acquire): 确保看到消费者已完成
    // ======================================================================
    info!("");
    info!("--- Demo 7: Lock-Free Atomic Ring Buffer (SPSC) ---");
    {
        // 重置 ring buffer
        RINGBUF.write_idx.store(0, Ordering::Relaxed);
        RINGBUF.read_idx.store(0, Ordering::Relaxed);

        // Core 0 (producer): 快速写入 N 个元素
        let total_items: u32 = 200;

        // 告知 Core 1 期望的条目数，然后让 Core 1 进入 consumer 模式
        RINGBUF_TOTAL.store(total_items, Ordering::Release);
        fifo.write(CMD_RINGBUF_CONSUME);
        let base_seed: u32 = 0xCAFE_0000;
        let mut expected_sum: u32 = 0;

        let ring_data = unsafe { &mut *RINGBUF.data.get() };

        for i in 0..total_items {
            let val = base_seed.wrapping_add(i);
            expected_sum = expected_sum.wrapping_add(val);

            // 等待空间 (如果 buffer 满了就 spin)
            loop {
                let r = RINGBUF.read_idx.load(Ordering::Acquire);
                let w = RINGBUF.write_idx.load(Ordering::Relaxed);
                if w - r < RINGBUF_CAP as u32 {
                    break;
                }
                // buffer 满 — 等待 consumer
                rp2350_rt::asm::wfe();
            }

            let w = RINGBUF.write_idx.load(Ordering::Relaxed);
            ring_data[(w as usize) % RINGBUF_CAP] = val;

            // Release: 确保数据写入完成后再更新 write_idx
            RINGBUF.write_idx.store(w + 1, Ordering::Release);

            // 唤醒 Core 1 (SEV)
            rp2350_rt::asm::sev();
        }

        // 等待 Core 1 消费完并返回 sum
        let sum_from_core1 = fifo.read();

        info!(
            "  Produced {} items, expected sum: {:08x}, consumer sum: {:08x}",
            total_items, expected_sum, sum_from_core1
        );

        if expected_sum == sum_from_core1 {
            info!("  Result: PASS — lock-free ring buffer works correctly");

            // 显示最终状态
            let w = RINGBUF.write_idx.load(Ordering::Relaxed);
            let r = RINGBUF.read_idx.load(Ordering::Relaxed);
            info!("  Final: write_idx={}, read_idx={} (all consumed)", w, r);
        } else {
            error!(
                "  Result: FAIL — sum mismatch (off by {})",
                expected_sum.wrapping_sub(sum_from_core1)
            );
        }
    }
    timer.delay_ms(100);

    // ======================================================================
    // 停止 Core 1
    // ======================================================================
    info!("");
    info!("[Core 0] Stopping Core 1...");
    fifo.write(CMD_STOP);
    let ack = fifo.read();
    if ack == CMD_STOP {
        info!("[Core 0] Core 1 acknowledged stop");
    }

    info!("");
    info!("=== All memory tests complete ===");

    loop {
        rp2350_rt::asm::wfe();
    }
}
