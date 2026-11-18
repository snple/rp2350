//! # 硬件真随机数生成器 (TRNG) 示例
//!
//! 基于 RP2350 (Raspberry Pi Pico 2) 的 **TRNG (True Random Number Generator)** 示例，
//! 演示如何使用硬件真随机数生成器产生密码学安全的随机数。
//!
//! ---
//!
//! # 1. 功能概述
//!
//! ## 1.1 TRNG 基本原理
//!
//! RP2350 内置硬件 TRNG，基于**环形振荡器 (Ring Oscillator, ROSC)** 的物理抖动
//! 产生真随机熵:
//!
//! ```text
//!   ROSC 物理噪声    Von Neumann     CRNGT 检测     EHR 寄存器
//!         │           去偏处理           │               │
//!   ┌─────▼─────┐   ┌──────┐    ┌──────▼─────┐   ┌────▼────┐
//!   │ Ring Osc  │──►│ VN   │───►│ Continuous │──►│ 192-bit │──► random_u32()
//!   │ Jitter    │   │ Debia│    │ RNG Test   │   │ Entropy │    random_u64()
//!   └───────────┘   └──────┘    └────────────┘   │ Register│    fill_random()
//!                                                 └─────────┘
//! ```
//!
//! - **Von Neumann 去偏**: 消除 ROSC 输出中的统计偏差
//! - **CRNGT (Continuous RNG Test)**: 连续随机数测试，确保连续两次输出不相同
//! - **Autocorrelation 测试**: 检测输出中的模式/相关性
//! - **EHR (Entropy Holding Register)**: 192-bit (6 × 32-bit) 熵池
//!
//! ## 1.2 与伪随机数的区别
//!
//! | 特性 | TRNG (本示例) | PRNG (软件) |
//! |------|--------------|-------------|
//! | 熵源 | 物理噪声 (不可预测) | 算法 + 种子 (可预测) |
//! | 安全性 | 密码学安全 | 种子泄露则不安全 |
//! | 速度 | 较慢 (~100μs/32bit) | 极快 (纳秒级) |
//! | 适用场景 | 密钥生成、nonce、IV | 游戏、仿真、测试 |
//!
//! ## 1.3 本示例演示内容
//!
//! 1. **随机 u32**: 生成 32 位随机整数
//! 2. **随机 u64**: 生成 64 位随机整数
//! 3. **随机字节填充**: 填充任意长度的缓冲区
//! 4. **随机范围**: 生成指定范围内的均匀随机数
//! 5. **原始熵读取**: 读取 192-bit EHR (Entropy Holding Register)
//! 6. **统计质量验证**: 简单的频率测试和卡方检验
//! 7. **错误处理**: Von Neumann / CRNGT / Autocorrelation 错误检测
//!
//! ---
//!
//! # 2. 硬件要求
//!
//! **无需任何外部接线** — TRNG 是 RP2350 内部集成的硬件模块。
//!
//! > **注意**: TRNG 是 RP2350 (Pico 2) 的新特性，RP2040 (Pico 1) 不支持。
//!
//! ---
//!
//! # 3. 安全注意事项
//!
//! - TRNG 输出适合作为密码学密钥、随机 nonce、初始化向量 (IV) 等
//! - 在安全关键应用中，应检查 `random_u32()` 等方法的 `Result` 返回值，
//!   确保没有发生 CRNGT/VN/Autocorrelation 错误
//! - 如果需要大量随机数 (>100KB/s)，建议用 TRNG 做种子，再通过
//!   ChaCha20 等 CSPRNG 扩展
//!
//! ---
//!
//! # 4. 编译运行
//!
//! ```bash
//! cargo run --bin trng --target thumbv8m.main-none-eabihf
//! ```

#![no_std]
#![no_main]

#[cfg(feature = "defmt")]
use defmt_rtt as _;
use rp2350_examples::log::*;

use rp2350::main;
use rp2350::pac::Peripherals;
use rp2350::timer::Timer0;
use rp2350::trng::Trng;
#[main]
fn main() -> ! {
    info!("=== TRNG (True Random Number Generator) Example ===");

    // ------------------------------------------------------------------
    // 1. 外设初始化
    // ------------------------------------------------------------------
    let dp = Peripherals::take().unwrap();
    let timer = Timer0::init(dp.TIMER0);

    // ------------------------------------------------------------------
    // 2. TRNG 初始化
    //    Trng::init() 会自动配置 ROSC 采样参数并启用 TRNG 模块
    // ------------------------------------------------------------------
    let trng = Trng::init(dp.TRNG);
    info!("TRNG initialized (hardware entropy source)");

    // ======================================================================
    // Demo 1: 生成随机 u32
    // ======================================================================
    info!("--- Demo 1: Random u32 ---");
    {
        for i in 0..10u32 {
            match trng.random_u32() {
                Ok(val) => info!("[{}] random_u32 = {:#010x} ({})", i, val, val),
                Err(_e) => warn!("[{}] TRNG error generating u32", i),
            }
        }
    }
    timer.delay_ms(100);

    // ======================================================================
    // Demo 2: 生成随机 u64
    // ======================================================================
    info!("--- Demo 2: Random u64 ---");
    {
        for i in 0..5u32 {
            match trng.random_u64() {
                Ok(val) => info!("[{}] random_u64 = {:#018x}", i, val),
                Err(_e) => warn!("[{}] TRNG error generating u64", i),
            }
        }
    }
    timer.delay_ms(100);

    // ======================================================================
    // Demo 3: 填充随机字节缓冲区
    //
    // fill_random() 将缓冲区填满随机字节，适用于:
    //   - AES 密钥 (16/32 字节)
    //   - 随机 nonce (12 字节)
    //   - UUID 生成 (16 字节)
    // ======================================================================
    info!("--- Demo 3: Fill Random Buffer ---");
    {
        // 模拟生成 AES-128 密钥 (16 字节)
        let mut key = [0u8; 16];
        match trng.fill_random(&mut key) {
            Ok(()) => {
                info!("AES-128 key (16 bytes):");
                info!(
                    "  [{:#04x}, {:#04x}, {:#04x}, {:#04x}, {:#04x}, {:#04x}, {:#04x}, {:#04x},",
                    key[0], key[1], key[2], key[3], key[4], key[5], key[6], key[7]
                );
                info!(
                    "   {:#04x}, {:#04x}, {:#04x}, {:#04x}, {:#04x}, {:#04x}, {:#04x}, {:#04x}]",
                    key[8], key[9], key[10], key[11], key[12], key[13], key[14], key[15]
                );
            }
            Err(_e) => warn!("TRNG error filling key buffer"),
        }

        // 模拟生成 UUID v4 (16 字节, 特定位设置版本和变体)
        let mut uuid = [0u8; 16];
        if trng.fill_random(&mut uuid).is_ok() {
            uuid[6] = (uuid[6] & 0x0F) | 0x40; // 版本 4
            uuid[8] = (uuid[8] & 0x3F) | 0x80; // 变体 1
            info!(
                "UUID v4: {:02x}{:02x}{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
                uuid[0],
                uuid[1],
                uuid[2],
                uuid[3],
                uuid[4],
                uuid[5],
                uuid[6],
                uuid[7],
                uuid[8],
                uuid[9],
                uuid[10],
                uuid[11],
                uuid[12],
                uuid[13],
                uuid[14],
                uuid[15]
            );
        }
    }
    timer.delay_ms(100);

    // ======================================================================
    // Demo 4: 随机范围 — 均匀分布
    //
    // random_range(max) 返回 [0, max) 范围内的均匀随机数
    // 内部使用 rejection sampling 消除取模偏差
    // ======================================================================
    info!("--- Demo 4: Random Range ---");
    {
        // 模拟掷骰子 10 次 (1~6)
        info!("Dice rolls (1-6):");
        for i in 0..10u32 {
            match trng.random_range(6) {
                Ok(val) => info!("  Roll {}: {}", i + 1, val + 1),
                Err(_e) => warn!("  Roll {}: TRNG error", i + 1),
            }
        }

        // 模拟抽扑克牌 (0~51)
        info!("Card draws (0-51):");
        let suits = ["Spades", "Hearts", "Diamonds", "Clubs"];
        let ranks = [
            "A", "2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K",
        ];

        for i in 0..5u32 {
            match trng.random_range(52) {
                Ok(card) => {
                    let suit = suits[(card / 13) as usize];
                    let rank = ranks[(card % 13) as usize];
                    info!("  Draw {}: {} of {}", i + 1, rank, suit);
                }
                Err(_e) => warn!("  Draw {}: TRNG error", i + 1),
            }
        }
    }
    timer.delay_ms(100);

    // ======================================================================
    // Demo 5: 读取原始 EHR (Entropy Holding Register)
    //
    // EHR 包含 192-bit (6 × 32-bit) 的原始熵数据。
    // 每次读取后 TRNG 会自动开始收集新的熵。
    // ======================================================================
    info!("--- Demo 5: Raw Entropy (EHR, 192-bit) ---");
    {
        match trng.read_ehr() {
            Ok(ehr) => {
                info!("EHR[0] = {:#010x}", ehr[0]);
                info!("EHR[1] = {:#010x}", ehr[1]);
                info!("EHR[2] = {:#010x}", ehr[2]);
                info!("EHR[3] = {:#010x}", ehr[3]);
                info!("EHR[4] = {:#010x}", ehr[4]);
                info!("EHR[5] = {:#010x}", ehr[5]);
                info!("Total: 192 bits of raw entropy");
            }
            Err(_e) => warn!("TRNG error reading EHR"),
        }
    }
    timer.delay_ms(100);

    // ======================================================================
    // Demo 6: 简单统计质量测试
    //
    // 生成大量随机数并检查:
    //   1) 位分布: 0 和 1 的比例应该接近 50/50
    //   2) 字节分布: 256 个可能值的出现频率应该近似均匀
    //
    // 注: 这不是正式的 NIST SP 800-22 测试套件，仅用于快速验证。
    // ======================================================================
    info!("--- Demo 6: Statistical Quality Check ---");
    {
        const SAMPLE_COUNT: u32 = 256;
        let mut zeros: u32 = 0;
        let mut ones: u32 = 0;
        let mut byte_counts = [0u32; 16]; // 按高 4 位分桶 (简化版)

        for _ in 0..SAMPLE_COUNT {
            if let Ok(val) = trng.random_u32() {
                // 统计 0/1 位数
                ones += val.count_ones();
                zeros += val.count_zeros();

                // 按最低字节的高 4 位分桶
                let bucket = ((val & 0xFF) >> 4) as usize;
                byte_counts[bucket] += 1;
            }
        }

        let total_bits = zeros + ones;
        let one_pct = (ones * 100).checked_div(total_bits).unwrap_or(0);

        info!(
            "Bit distribution: 1s={}%, 0s={}% (ideal: 50/50)",
            one_pct,
            100 - one_pct
        );
        info!(
            "  Total bits: {}, ones: {}, zeros: {}",
            total_bits, ones, zeros
        );

        // 简单均匀性检查: 16 个桶的最大偏差
        let expected = SAMPLE_COUNT / 16; // 每桶期望值
        let mut max_deviation: u32 = 0;
        for count in byte_counts.iter() {
            let dev = (*count).abs_diff(expected);
            if dev > max_deviation {
                max_deviation = dev;
            }
        }

        info!(
            "Byte bucket uniformity: expected={}/bucket, max deviation={}",
            expected, max_deviation
        );

        if (45..=55).contains(&one_pct) && max_deviation < expected {
            info!("Quality: PASS (reasonable for quick check)");
        } else {
            warn!("Quality: MARGINAL — consider running more samples");
        }
    }
    timer.delay_ms(100);

    // ======================================================================
    // Demo 7: 性能测量
    //
    // 测量生成随机数的速度
    // ======================================================================
    info!("--- Demo 7: Performance Measurement ---");
    {
        let count: u32 = 100;

        // 测量 random_u32 速度
        let start = timer.time_us();
        for _ in 0..count {
            let _ = trng.random_u32();
        }
        let elapsed = timer.time_us() - start;
        let per_call = elapsed / count as u64;

        info!(
            "random_u32: {} calls in {} us ({} us/call)",
            count, elapsed, per_call
        );

        // 测量 fill_random 速度 (64 字节)
        let mut buf = [0u8; 64];
        let start = timer.time_us();
        for _ in 0..count {
            let _ = trng.fill_random(&mut buf);
        }
        let elapsed = timer.time_us() - start;
        let bytes_per_sec = (count as u64 * 64 * 1_000_000)
            .checked_div(elapsed)
            .unwrap_or(0);
        info!(
            "fill_random(64B): {} calls in {} us ({} bytes/sec)",
            count, elapsed, bytes_per_sec
        );
    }

    // ======================================================================
    // 无限循环: 每秒生成并打印一个随机数
    // ======================================================================
    info!("=== Continuous random generation ===");

    let mut counter: u32 = 0;
    loop {
        match trng.random_u32() {
            Ok(val) => {
                info!("[#{}] {:#010x} (decimal: {})", counter, val, val);
            }
            Err(_e) => warn!("[#{}] TRNG error", counter),
        }

        counter = counter.wrapping_add(1);
        timer.delay_ms(1000);
    }
}
