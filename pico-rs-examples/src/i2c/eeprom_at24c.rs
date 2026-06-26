//! # AT24C EEPROM I2C 读写示例
//!
//! 基于 RP2350 (Raspberry Pi Pico 2)，通过 I2C0 对 **AT24C 系列串行 EEPROM**
//! 进行字节写、页写、跨页写、随机读、顺序读操作，
//! 演示完整的写完成等待 (ACK 轮询) 和数据校验流程。
//!
//! 适配型号：AT24C32 / AT24C64 / AT24C128 / **AT24C256** (默认) / AT24C512
//! （均使用 2 字节地址；AT24C01/02/04/08/16 使用 1 字节地址，需修改驱动函数）
//!
//! ---
//!
//! # 1. AT24C EEPROM 基础
//!
//! ## 1.1 器件概述
//!
//! AT24C 系列是 Microchip (原 Atmel) 生产的 **I2C 串行 EEPROM**，
//! 以低引脚数、低功耗、无需外部时钟著称，广泛用于参数存储、序列号等场景。
//!
//! | 型号 | 容量 | 地址字节数 | 页大小 | 写周期 |
//! |------|------|-----------|--------|--------|
//! | AT24C01 | 128 B | 1 | 8 B | ≤ 5 ms |
//! | AT24C02 | 256 B | 1 | 8 B | ≤ 5 ms |
//! | AT24C04 | 512 B | 1 | 16 B | ≤ 5 ms |
//! | AT24C08 | 1 KB | 1 | 16 B | ≤ 5 ms |
//! | AT24C16 | 2 KB | 1 | 16 B | ≤ 5 ms |
//! | AT24C32 | 4 KB | **2** | 32 B | ≤ 5 ms |
//! | AT24C64 | 8 KB | **2** | 32 B | ≤ 5 ms |
//! | AT24C128 | 16 KB | **2** | 64 B | ≤ 5 ms |
//! | AT24C256 | 32 KB | **2** | 64 B | ≤ 5 ms |
//! | AT24C512 | 64 KB | **2** | 128 B | ≤ 5 ms |
//!
//! > **本例针对 AT24C256 (32 KB, 2 字节地址, 64 字节页大小)**，
//! > 修改 `PAGE_SIZE` 和 `TOTAL_SIZE` 常量即可适配其他型号。
//!
//! ## 1.2 I2C 设备地址
//!
//! AT24C 系列 7-bit I2C 地址格式（第 8 位为 R/W，已由 HAL 处理）：
//!
//! ```text
//! bit 6  5  4  3  2  1  0
//!     1  0  1  0  A2 A1 A0
//! ```
//!
//! - 固定前缀：**0b1010**（高 4 位固定）
//! - **A2、A1、A0**：对应芯片上的 3 个地址引脚，接 GND=0，接 VCC=1
//! - 7-bit 地址范围：**0x50 ~ 0x57**（同一总线最多并联 8 片）
//!
//! | A2 | A1 | A0 | 7-bit 地址 |
//! |----|----|----|-----------|
//! | 0  | 0  | 0  | **0x50（默认，本例使用）** |
//! | 0  | 0  | 1  | 0x51 |
//! | 0  | 1  | 0  | 0x52 |
//! | 0  | 1  | 1  | 0x53 |
//! | 1  | 0  | 0  | 0x54 |
//! | 1  | 0  | 1  | 0x55 |
//! | 1  | 1  | 0  | 0x56 |
//! | 1  | 1  | 1  | 0x57 |
//!
//! ## 1.3 操作时序详解
//!
//! ### 字节写 (Byte Write)
//!
//! 每次写入 1 个字节，写完后 EEPROM 进入内部编程状态（最长 5 ms）：
//!
//! ```text
//! [START]
//! [1010 A2 A1 A0 | 0]   ← 器件地址 + 写标志
//! [ADDR_HIGH]            ← 存储地址高字节 (bit15~bit8)
//! [ADDR_LOW]             ← 存储地址低字节 (bit7~bit0)
//! [DATA]                 ← 写入 1 字节数据
//! [STOP]
//! ↓
//! (内部编程，最长 5 ms，此期间 EEPROM 不响应 I2C)
//! ```
//!
//! ### 页写 (Page Write)
//!
//! 一次 I2C 写事务最多可写入 **一整页** 数据（AT24C256 为 64 字节），
//! 比逐字节写效率高得多（节省 N-1 次写周期等待）：
//!
//! ```text
//! [START]
//! [1010 A2 A1 A0 | 0]   ← 器件地址 + 写
//! [ADDR_HIGH]            ← 页内起始地址高字节
//! [ADDR_LOW]             ← 页内起始地址低字节
//! [DATA_0]               ← 第 0 字节数据
//! [DATA_1]               ← 第 1 字节数据
//!  ...                   ← ...（最多写满一页，不超过 64 字节）
//! [DATA_N]
//! [STOP]
//! ↓
//! (内部编程，最长 5 ms)
//! ```
//!
//! > ⚠️ **页边界限制**：写操作不能跨越页边界！
//! > 若地址计数器到达页末尾会**自动回绕到页首**，覆盖本页前面的数据。
//! > 必须将大批量写操作拆分成对齐的页写，见 `eeprom_write_any`。
//!
//! ### 随机读 (Random Read)
//!
//! 先"假写"地址（不含数据，仅用于设置内部地址指针），
//! 再发重复起始 + 读命令取出数据：
//!
//! ```text
//! [START]
//! [1010 A2 A1 A0 | 0]   ← 器件地址 + 写（设置读地址）
//! [ADDR_HIGH]            ← 要读取的地址高字节
//! [ADDR_LOW]             ← 要读取的地址低字节
//! [RESTART]
//! [1010 A2 A1 A0 | 1]   ← 器件地址 + 读
//! [DATA]                 ← 读出 1 字节（主机发 NACK 终止）
//! [STOP]
//! ```
//!
//! ### 顺序读 (Sequential Read)
//!
//! 与随机读类似，但主机持续回复 ACK，EEPROM 自动递增地址，
//! 连续输出数据，直到主机发送 NACK + STOP 终止：
//!
//! ```text
//! [START]
//! [1010 A2 A1 A0 | 0]   ← 器件地址 + 写（设置起始地址）
//! [ADDR_HIGH]
//! [ADDR_LOW]
//! [RESTART]
//! [1010 A2 A1 A0 | 1]   ← 器件地址 + 读
//! [DATA_0] [ACK]         ← 主机 ACK，继续读
//! [DATA_1] [ACK]
//!  ...
//! [DATA_N] [NACK]        ← 主机 NACK，结束
//! [STOP]
//! ```
//!
//! 以上读操作均通过 `I2cBus::write_read()` 实现（先写地址再重复起始读数据）。
//!
//! ## 1.4 写完成等待：ACK 轮询 (Acknowledge Polling)
//!
//! EEPROM 写入后进入内部编程状态，期间不响应 I2C（返回 NACK）。
//! 编程完成后才开始正常响应（返回 ACK）。
//!
//! **两种等待方式对比**：
//!
//! | 方式 | 实现 | 优点 | 缺点 |
//! |------|------|------|------|
//! | 固定延时 | `delay_ms(5)` | 简单 | 浪费时间（实际写周期通常 2~3 ms） |
//! | ACK 轮询 | 循环发 I2C 直到 ACK | 最短等待时间 | 稍复杂 |
//!
//! 本例使用 **ACK 轮询**：
//!
//! ```text
//! loop {
//!     发送空写操作（零字节）
//!     if 收到 ACK → EEPROM 就绪，退出
//!     else        → 等待 100 μs 再试
//! }
//! ```
//!
//! ---
//!
//! # 2. 硬件接线
//!
//! ## 2.1 AT24C256 芯片引脚
//!
//! DIP-8 / SOIC-8 封装引脚定义：
//!
//! ```text
//!          ┌─── 8-pin ───┐
//!   A0 ───┤1            8├─── VCC (1.7V ~ 5.5V)
//!   A1 ───┤2            7├─── WP  (写保护)
//!   A2 ───┤3            6├─── SCL (时钟)
//!  GND ───┤4            5├─── SDA (数据)
//!          └─────────────┘
//! ```
//!
//! | 引脚 | 名称 | 说明 |
//! |------|------|------|
//! | 1 | A0 | 地址位 0（接 GND 或 VCC） |
//! | 2 | A1 | 地址位 1（接 GND 或 VCC） |
//! | 3 | A2 | 地址位 2（接 GND 或 VCC） |
//! | 4 | GND | 电源地 |
//! | 5 | SDA | I2C 数据线（需上拉到 VCC） |
//! | 6 | SCL | I2C 时钟线（需上拉到 VCC） |
//! | 7 | WP | 写保护（**接 GND = 允许写**，接 VCC = 禁止写） |
//! | 8 | VCC | 电源正 (3.3V) |
//!
//! ## 2.2 Pico 2 接线图
//!
//! ```text
//!   Pico 2 (RP2350)              AT24C256
//!   ┌────────────────┐           ┌────────┐
//!   │            GP4 ├── SDA ───┤5  SDA  │
//!   │            GP5 ├── SCL ───┤6  SCL  │
//!   │           3.3V ├── VCC ───┤8  VCC  │
//!   │            GND ├── GND ───┤4  GND  │
//!   │                │    GND ──┤1  A0   │ (地址引脚接 GND → 地址 0x50)
//!   │                │    GND ──┤2  A1   │
//!   │                │    GND ──┤3  A2   │
//!   │                │    GND ──┤7  WP   │ (接 GND → 允许写操作!)
//!   └────────────────┘           └────────┘
//!
//!   上拉电阻：SDA、SCL 各接 4.7 kΩ 到 3.3V
//! ```
//!
//! > **WP (Write Protect) 引脚注意事项**：
//! > - 接 GND → **允许写**（本例必须如此）
//! > - 接 VCC → 禁止写，所有写操作均被 NACK（但读操作不受影响）
//! > - 许多 AT24C 模块将 WP 通过上拉电阻接到 VCC，导致写入失败。
//! >   如果写入总是超时，请检查 WP 引脚接线！
//!
//! ---
//!
//! # 3. 配置参数
//!
//! | 参数 | 默认值 | 说明 |
//! |------|--------|------|
//! | `EEPROM_ADDR` | 0x50 | EEPROM I2C 地址 (A2=A1=A0=GND) |
//! | `PAGE_SIZE` | 64 | AT24C256 页大小（字节），改为 32 适配 AT24C32/64 |
//! | `TOTAL_SIZE` | 32768 | AT24C256 总容量（字节） |
//! | `MAX_POLL_RETRIES` | 200 | ACK 轮询最大次数（防止写保护时死循环） |
//! | I2C 频率 | 400_000 | 400 kHz 快速模式 |
//!
//! ---
//!
//! # 4. 输出示例
//!
//! ```text
//! === AT24C256 EEPROM Example ===
//! System clock: 150000000 Hz
//! I2C0: 400000 Hz (GP4=SDA, GP5=SCL)
//! EEPROM detected at 0x50
//!
//! --- Test 1: Byte Write & Random Read ---
//! Writing 0xA5 to address 0x0000...
//! Write complete (ACK polling: 3 retries)
//! Read back: 0xA5  [OK]
//!
//! --- Test 2: Page Write (64 bytes) ---
//! Writing 64 bytes to address 0x0040 (page 1)...
//! Page write complete (ACK polling: 2 retries)
//! Page verify OK: all 64 bytes match
//!
//! --- Test 3: Cross-page Write (128 bytes from 0x0078) ---
//! Writing 128 bytes from address 0x0078 (crosses page boundaries)...
//! Cross-page write complete
//! Cross-page verify OK: all 128 bytes match
//!
//! --- Test 4: Sequential Read Dump (128 bytes from 0x0000) ---
//! 0000: A5 FF FF FF FF FF FF FF  FF FF FF FF FF FF FF FF
//! 0010: FF FF FF FF FF FF FF FF  FF FF FF FF FF FF FF FF
//! 0020: FF FF FF FF FF FF FF FF  FF FF FF FF FF FF FF FF
//! 0030: FF FF FF FF FF FF FF FF  FF FF FF FF FF FF FF FF
//! 0040: 00 01 02 03 04 05 06 07  08 09 0A 0B 0C 0D 0E 0F
//! 0050: 10 11 12 13 14 15 16 17  18 19 1A 1B 1C 1D 1E 1F
//! 0060: 20 21 22 23 24 25 26 27  28 29 2A 2B 2C 2D 2E 2F
//! 0070: 30 31 32 33 34 35 36 37  38 39 3A 3B 3C 3D 3E 3F
//!
//! === All tests completed! ===
//! EEPROM total capacity: 32768 bytes (32 KB)
//! ```
//!
//! ---
//!
//! # 5. 编译运行
//!
//! ```bash
//! cargo run --bin eeprom_at24c --target thumbv8m.main-none-eabihf
//! ```

#![no_std]
#![no_main]

use defmt::*;
use defmt_rtt as _;

use pico_rs::Delay;
use pico_rs::block::ImageDef;
use pico_rs::clocks::{ClockBuilder, ClockResources};
use pico_rs::entry;
use pico_rs::gpio::Pins;
use pico_rs::i2c::{Error, I2c1, I2cBus};
use pico_rs::pac::Peripherals;
use pico_rs::rt::peripheral::Peripherals as CorePeripherals;

/// RP2350 启动块 —— 告知 Boot ROM 本镜像是合法的 ARM 可执行文件
#[unsafe(link_section = ".start_block")]
#[used]
pub static IMAGE_DEF: ImageDef = ImageDef::exe();

// ============================================================================
// 配置参数
// ============================================================================

/// EEPROM I2C 设备地址
/// A2=A1=A0=GND → 0b1010_000 = 0x50
/// 若地址引脚接 VCC，请相应修改，例如 A0=VCC 时为 0x51
const EEPROM_ADDR: u8 = 0x50;

/// AT24C256 页大小（字节）
/// 一次页写不能超过此值，且不能跨越页边界。
/// - AT24C32 / AT24C64  → 32
/// - AT24C128 / AT24C256 → 64
/// - AT24C512            → 128
const PAGE_SIZE: usize = 64;

/// AT24C256 总存储容量（字节）
/// - AT24C32  → 4096
/// - AT24C64  → 8192
/// - AT24C128 → 16384
/// - AT24C256 → 32768
/// - AT24C512 → 65536
const TOTAL_SIZE: u32 = 32768;

/// ACK 轮询最大重试次数
/// 防止 EEPROM 写保护（WP 接高）或断线时陷入死循环。
/// 以 100 μs 间隔轮询，200 次 = 20 ms，远超 5 ms 写周期上限。
const MAX_POLL_RETRIES: u32 = 200;

// ============================================================================
// EEPROM 驱动函数
// ============================================================================

/// 等待 EEPROM 写周期完成（ACK 轮询）
///
/// EEPROM 在内部编程期间会对所有 I2C 操作回复 NACK。
/// 本函数以 100 μs 间隔反复发送空写操作，直到收到 ACK（表示就绪）。
///
/// # 返回
/// - `Ok(retries)` — 写入完成，返回实际轮询次数（可用于性能调试）
/// - `Err(())` — 超过最大重试次数：可能是写保护（WP=HIGH）或硬件故障
fn eeprom_wait_ready<B: I2cBus>(i2c: &B, delay: &Delay) -> Result<u32, ()> {
    for retries in 0..MAX_POLL_RETRIES {
        // 发送零字节写操作，仅用于检测 ACK/NACK
        if i2c.write(EEPROM_ADDR, &[]).is_ok() {
            return Ok(retries);
        }
        // 写周期尚未结束，等待 100 μs 后重试
        delay.delay_us(100);
    }
    Err(())
}

/// 字节写：向指定地址写入 1 字节数据
///
/// 写完成后 EEPROM 进入内部编程状态（最长 5 ms），
/// 调用方须随后调用 `eeprom_wait_ready` 等待写周期结束，
/// 才能进行下一次写或读操作。
///
/// # I2C 时序
/// `[DEVICE|W] [ADDR_H] [ADDR_L] [DATA]`
///
/// # 参数
/// - `addr`: 16-bit 字节地址（AT24C256 有效范围 0x0000 ~ 0x7FFF）
/// - `data`: 要写入的字节值
fn eeprom_write_byte<B: I2cBus>(i2c: &B, addr: u16, data: u8) -> Result<(), B::Error> {
    let buf = [
        (addr >> 8) as u8,   // 地址高字节（bit15~bit8）
        (addr & 0xFF) as u8, // 地址低字节（bit7~bit0）
        data,                // 待写数据
    ];
    i2c.write(EEPROM_ADDR, &buf)
}

/// 页写：向指定地址连续写入最多 PAGE_SIZE 字节
///
/// 一次页写仅触发 **一次写周期**，效率远高于逐字节写。
///
/// # ⚠️ 页边界约束
///
/// 1. `data.len()` 不能超过 `PAGE_SIZE`（64 字节），超出部分会被静默截断
/// 2. 起始地址到末尾地址必须在**同一页内**：
///    `addr / PAGE_SIZE == (addr + data.len() - 1) / PAGE_SIZE`
/// 3. 若需写入跨越页边界的数据，请调用 `eeprom_write_any`（自动分页）
///
/// # I2C 时序
/// `[DEVICE|W] [ADDR_H] [ADDR_L] [DATA_0] [DATA_1] ... [DATA_N]`
fn eeprom_write_page<B: I2cBus>(i2c: &B, addr: u16, data: &[u8]) -> Result<(), B::Error> {
    // 构造写缓冲区：[地址高字节, 地址低字节, 数据...]
    // 使用固定大小栈数组，避免在 no_std 环境下进行堆分配
    let mut buf = [0u8; 2 + PAGE_SIZE];
    let len = data.len().min(PAGE_SIZE);

    buf[0] = (addr >> 8) as u8; // 地址高字节
    buf[1] = (addr & 0xFF) as u8; // 地址低字节
    buf[2..2 + len].copy_from_slice(&data[..len]);

    i2c.write(EEPROM_ADDR, &buf[..2 + len])
}

/// 随机读：从指定地址读取 1 字节数据
///
/// 使用 write_read（先写地址后重复起始读数据），
/// 地址写入和数据读出之间无 STOP，只有 RESTART，符合 EEPROM 要求。
///
/// # I2C 时序
/// `[DEVICE|W] [ADDR_H] [ADDR_L] [RESTART] [DEVICE|R] [DATA] [NACK] [STOP]`
fn eeprom_read_byte<B: I2cBus>(i2c: &B, addr: u16) -> Result<u8, B::Error> {
    let addr_buf = [(addr >> 8) as u8, (addr & 0xFF) as u8];
    let mut buf = [0u8; 1];
    i2c.write_read(EEPROM_ADDR, &addr_buf, &mut buf)?;
    Ok(buf[0])
}

/// 顺序读：从指定地址连续读取多字节数据
///
/// AT24C256 支持跨页连续读，内部地址指针自动递增。
/// 到达最大地址（0x7FFF）后会**回绕到 0x0000**，继续读取。
///
/// 本函数可一次读取任意长度，无页边界限制（仅写操作有页边界约束）。
///
/// # I2C 时序
/// `[DEVICE|W] [ADDR_H] [ADDR_L] [RESTART] [DEVICE|R] [D0][ACK] [D1][ACK] ... [DN][NACK] [STOP]`
fn eeprom_read_seq<B: I2cBus>(i2c: &B, addr: u16, buf: &mut [u8]) -> Result<(), B::Error> {
    let addr_buf = [(addr >> 8) as u8, (addr & 0xFF) as u8];
    i2c.write_read(EEPROM_ADDR, &addr_buf, buf)
}

/// 任意长度写入（自动处理页边界分片）
///
/// 将 `data` 写入从 `start_addr` 开始的连续地址空间，
/// 内部自动按 64 字节页对齐，分多次页写完成，每次写后等待 ACK 轮询就绪。
///
/// # 示例：128 字节从地址 0x0078 写入
///
/// ```text
/// 页 1 (0x0040~0x007F)：剩余 8 字节 → 写 0x0078~0x007F
/// 页 2 (0x0080~0x00BF)：整页 64 字节 → 写 0x0080~0x00BF
/// 页 3 (0x00C0~0x00FF)：前 56 字节 → 写 0x00C0~0x00F7
/// ```
///
/// # 参数
/// - `start_addr`: 写入起始地址
/// - `data`: 待写入数据（任意长度）
/// - `delay`: 延时驱动，用于 ACK 轮询间隔等待
fn eeprom_write_any<B: I2cBus<Error = Error>>(
    i2c: &B,
    start_addr: u16,
    data: &[u8],
    delay: &Delay,
) -> Result<(), Error> {
    let mut written = 0usize;
    let mut addr = start_addr;

    while written < data.len() {
        // 计算当前地址在页内的偏移，以及到本页末尾的剩余字节数
        // 例如：addr = 0x0078，PAGE_SIZE = 64
        //   page_offset = 0x78 % 64 = 56
        //   bytes_to_page_end = 64 - 56 = 8
        let page_offset = (addr as usize) % PAGE_SIZE;
        let bytes_to_page_end = PAGE_SIZE - page_offset;
        let bytes_remaining = data.len() - written;

        // 本次实际写入字节数：不超过页末尾，也不超过剩余数据量
        let chunk_len = bytes_to_page_end.min(bytes_remaining);

        // 执行本次页写
        eeprom_write_page(i2c, addr, &data[written..written + chunk_len])?;

        // 等待 EEPROM 内部编程完成（ACK 轮询）
        eeprom_wait_ready(i2c, delay).map_err(|_| Error::Timeout)?;

        written += chunk_len;
        addr = addr.wrapping_add(chunk_len as u16);
    }

    Ok(())
}

// ============================================================================
// 主程序
// ============================================================================

#[entry]
fn main() -> ! {
    info!("=== AT24C256 EEPROM Example ===");

    // ------------------------------------------------------------------
    // 1. 外设初始化
    // ------------------------------------------------------------------
    let dp = Peripherals::take().unwrap();

    // 初始化时钟系统（150 MHz 系统时钟，48 MHz USB 时钟）
    let clocks = ClockBuilder::new().build(ClockResources { clocks: dp.CLOCKS, xosc: dp.XOSC, pll_sys: dp.PLL_SYS, pll_usb: dp.PLL_USB });
    info!("System clock: {} Hz", clocks.sys_freq());

    // 初始化 GPIO 管理器
    let pins = Pins::new(dp.IO_BANK0, dp.PADS_BANK0);

    // 初始化 I2C1：GP2=SDA，GP3=SCL，400 kHz 快速模式
    // HAL 会自动配置引脚为 I2C 功能（开漏 + 内部上拉）
    let i2c = I2c1::new(
        dp.I2C1,
        pins.pin::<2>(), // SDA
        pins.pin::<3>(), // SCL
        400_000,         // 400 kHz
        &clocks,
    );
    info!("I2C1: 400000 Hz (GP2=SDA, GP3=SCL)");

    // 初始化 SysTick 延时（用于 ACK 轮询等待）
    let core = CorePeripherals::take().unwrap();
    let delay = Delay::new(core.SYST, clocks.sys_freq());

    // ------------------------------------------------------------------
    // 2. EEPROM 在线检测
    // ------------------------------------------------------------------
    // 发送零字节写操作检测设备是否应答（ACK）
    match i2c.write(EEPROM_ADDR, &[]) {
        Ok(_) => {
            info!("EEPROM detected at {:#04x}", EEPROM_ADDR);
        }
        Err(_) => {
            error!("EEPROM not found at {:#04x}!", EEPROM_ADDR);
            error!("Troubleshooting checklist:");
            error!("  [1] GP4=SDA / GP5=SCL wiring");
            error!("  [2] Pull-up resistors (4.7k to 3.3V) on SDA & SCL");
            error!("  [3] VCC (3.3V) and GND connected");
            error!("  [4] WP pin → GND (HIGH = write protected, still readable)");
            error!(
                "  [5] Address pins A2/A1/A0 match EEPROM_ADDR={:#04x}",
                EEPROM_ADDR
            );
            loop {}
        }
    }

    // ======================================================================
    // 测试 1：字节写 & 随机读
    //
    // 向地址 0x0000 写入 1 字节 0xA5，等待写周期完成后读回校验。
    // ======================================================================
    info!("");
    info!("--- Test 1: Byte Write & Random Read ---");

    let test_addr: u16 = 0x0000;
    let test_byte: u8 = 0xA5;

    info!(
        "Writing {:#04x} to address {:#06x}...",
        test_byte, test_addr
    );

    if let Err(e) = eeprom_write_byte(&i2c, test_addr, test_byte) {
        error!("Write failed: {}", e);
        error!("Hint: WP pin may be HIGH (connected to VCC).");
        error!("      Connect WP to GND to enable write operations.");
        loop {}
    }

    // 等待 EEPROM 内部编程完成（ACK 轮询）
    match eeprom_wait_ready(&i2c, &delay) {
        Ok(retries) => info!("Write complete (ACK polling: {} retries)", retries),
        Err(_) => {
            error!("Write timeout! EEPROM may be write-protected (WP=HIGH).");
            loop {}
        }
    }

    // 随机读：从地址 0x0000 读回 1 字节并校验
    match eeprom_read_byte(&i2c, test_addr) {
        Ok(val) if val == test_byte => {
            info!("Read back: {:#04x}  [OK]", val);
        }
        Ok(val) => {
            error!(
                "Read back: {:#04x}  [MISMATCH! expected {:#04x}]",
                val, test_byte
            );
            if val == 0xFF {
                error!("⚠  Readback is 0xFF — classic Write-Protect symptom!");
                error!("⚠  Connect the WP pin to GND to enable writes.");
                error!("⚠  WP=HIGH (or floating) → entire EEPROM is read-only.");
            }
        }
        Err(e) => {
            error!("Read failed: {}", e);
        }
    }

    // ======================================================================
    // 测试 2：页写 (Page Write)
    //
    // 向页 1（地址 0x0040~0x007F）写入 64 字节序列数据 [0x00..0x3F]，
    // 演示一次写周期写满整页，效率远高于逐字节写。
    // ======================================================================
    info!("");
    info!("--- Test 2: Page Write (64 bytes) ---");

    // 页 1 起始地址：PAGE_SIZE * 1 = 64 = 0x0040
    let page_addr: u16 = PAGE_SIZE as u16;
    info!(
        "Writing {} bytes to address {:#06x} (page 1)...",
        PAGE_SIZE, page_addr
    );

    // 构造测试数据：[0x00, 0x01, 0x02, ..., 0x3F]
    let mut page_data = [0u8; PAGE_SIZE];
    for (i, b) in page_data.iter_mut().enumerate() {
        *b = i as u8;
    }

    if let Err(e) = eeprom_write_page(&i2c, page_addr, &page_data) {
        error!("Page write failed: {}", e);
        loop {}
    }

    match eeprom_wait_ready(&i2c, &delay) {
        Ok(retries) => info!("Page write complete (ACK polling: {} retries)", retries),
        Err(_) => {
            error!("Write timeout!");
            loop {}
        }
    }

    // 顺序读回整页，逐字节校验
    let mut read_buf = [0u8; PAGE_SIZE];
    match eeprom_read_seq(&i2c, page_addr, &mut read_buf) {
        Ok(_) => {
            let mut all_ok = true;
            let mut all_ff = true;
            for i in 0..PAGE_SIZE {
                if read_buf[i] != page_data[i] {
                    error!(
                        "Mismatch at +{}: got {:#04x}, expected {:#04x}",
                        i, read_buf[i], page_data[i]
                    );
                    all_ok = false;
                }
                if read_buf[i] != 0xFF {
                    all_ff = false;
                }
            }
            if all_ok {
                info!("Page verify OK: all {} bytes match", PAGE_SIZE);
            }
            if !all_ok && all_ff {
                error!("⚠  All bytes read 0xFF — WP pin is probably HIGH!");
                error!("⚠  Connect WP pin to GND to enable writes.");
            }
        }
        Err(e) => {
            error!("Page read failed: {}", e);
        }
    }

    // ======================================================================
    // 测试 3：跨页连续写 (eeprom_write_any)
    //
    // 从地址 0x0078 写入 128 字节，故意跨越页边界：
    //   页 1 尾部 (0x0078~0x007F)：8 字节
    //   页 2 完整 (0x0080~0x00BF)：64 字节
    //   页 3 头部 (0x00C0~0x00F7)：56 字节
    // eeprom_write_any 自动完成分页写入和等待，调用方无需关心页边界。
    // ======================================================================
    info!("");
    info!("--- Test 3: Cross-page Write (128 bytes from 0x0078) ---");

    let cross_addr: u16 = 0x0078;
    let cross_len: usize = 128;
    info!(
        "Writing {} bytes from {:#06x} (crosses page boundaries)...",
        cross_len, cross_addr
    );
    info!("  Page 1 tail: 0x0078~0x007F (8 bytes)");
    info!("  Page 2 full: 0x0080~0x00BF (64 bytes)");
    info!("  Page 3 head: 0x00C0~0x00F7 (56 bytes)");

    // 构造测试数据：[0x80, 0x81, ..., 0xFF, 0x00, 0x01, ...]
    let mut cross_data = [0u8; 128];
    for (i, b) in cross_data.iter_mut().enumerate() {
        *b = (0x80u16.wrapping_add(i as u16)) as u8;
    }

    match eeprom_write_any(&i2c, cross_addr, &cross_data, &delay) {
        Ok(_) => info!("Cross-page write complete"),
        Err(e) => {
            error!("Cross-page write failed: {}", e);
            loop {}
        }
    }

    // 顺序读回并逐字节校验
    let mut verify_buf = [0u8; 128];
    match eeprom_read_seq(&i2c, cross_addr, &mut verify_buf) {
        Ok(_) => {
            let mut mismatches = 0u32;
            for i in 0..cross_len {
                if verify_buf[i] != cross_data[i] {
                    mismatches += 1;
                    // 仅打印前 3 个不匹配，避免日志过多
                    if mismatches <= 3 {
                        error!(
                            "Mismatch at +{}: got {:#04x}, expected {:#04x}",
                            i, verify_buf[i], cross_data[i]
                        );
                    }
                }
            }
            if mismatches == 0 {
                info!("Cross-page verify OK: all {} bytes match", cross_len);
            } else {
                error!("Cross-page verify FAILED: {} mismatches", mismatches);
            }
        }
        Err(e) => {
            error!("Verify read failed: {}", e);
        }
    }

    // ======================================================================
    // 测试 4：顺序读转储（读取前 128 字节并格式化输出）
    //
    // 以 16 字节一行的十六进制格式打印前 128 字节内容，
    // 可用于直观验证写入结果。
    // ======================================================================
    info!("");
    info!("--- Test 4: Sequential Read Dump (128 bytes from 0x0000) ---");

    let mut dump_buf = [0u8; 128];
    match eeprom_read_seq(&i2c, 0x0000, &mut dump_buf) {
        Ok(_) => {
            // 每行 16 字节，中间以空格分隔两组 8 字节（方便阅读）
            // defmt 不支持运行时动态格式化，需展开每行
            for row in 0..8usize {
                let o = row * 16;
                let d = &dump_buf[o..o + 16];
                info!(
                    "{:04x}: {:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x}  \
                           {:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x}",
                    o as u16,
                    d[0],
                    d[1],
                    d[2],
                    d[3],
                    d[4],
                    d[5],
                    d[6],
                    d[7],
                    d[8],
                    d[9],
                    d[10],
                    d[11],
                    d[12],
                    d[13],
                    d[14],
                    d[15]
                );
            }
        }
        Err(e) => {
            error!("Sequential read failed: {}", e);
        }
    }

    // ======================================================================
    // 完成
    // ======================================================================
    info!("");
    info!("=== All tests completed! ===");
    info!(
        "EEPROM total capacity: {} bytes ({} KB)",
        TOTAL_SIZE,
        TOTAL_SIZE / 1024
    );
    info!("Entering idle loop...");

    loop {
        delay.delay_ms(1000);
    }
}
