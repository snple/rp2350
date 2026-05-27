//! # UART 串口回显示例
//!
//! 基于 RP2350 (Raspberry Pi Pico 2) 的 **UART 阻塞式串口通信**示例，
//! 演示最基础的 UART 收发功能 (不使用 DMA)。
//!
//! ---
//!
//! # 1. 功能概述
//!
//! ## 1.1 UART 基础
//!
//! UART (Universal Asynchronous Receiver/Transmitter) 是最常见的异步串行通信协议:
//!
//! ```text
//!   数据帧格式 (8N1):
//!   ─────┐   ┌───┬───┬───┬───┬───┬───┬───┬───┐   ┌─────
//!   IDLE │ S │ D0│ D1│ D2│ D3│ D4│ D5│ D6│ D7│ P │ IDLE
//!   (高) │(低)│ LSB                       MSB │(高)│ (高)
//!   ─────┘   └───┴───┴───┴───┴───┴───┴───┴───┘   └─────
//!            │← 起始位    8 位数据       停止位 →│
//! ```
//!
//! - **异步**: 无时钟线，收发双方约定相同波特率
//! - **全双工**: TX 和 RX 独立工作，可同时收发
//! - **8N1**: 最常用格式 — 8 数据位, 无校验, 1 停止位
//!
//! ## 1.2 RP2350 UART 特性
//!
//! - 2 个独立 UART 控制器 (UART0, UART1)
//! - 硬件 FIFO: TX 和 RX 各 32 字节深度
//! - 支持 5~8 位数据位, 1~2 停止位, 奇/偶/无校验
//! - 支持硬件流控 (CTS/RTS)
//! - 波特率: 110 ~ 7.8 MHz (取决于系统时钟)
//!
//! ## 1.3 本示例演示内容
//!
//! 1. **串口初始化**: 配置 UART0, 115200 baud, 8N1
//! 2. **发送字符串**: 使用 `write()` 和 `write_str()` 发送欢迎信息
//! 3. **回显 (Echo)**: 接收到任意字符后立即回发 — 最经典的串口测试
//! 4. **非阻塞读取**: 使用 `read_byte()` 轮询接收
//! 5. **阻塞读取**: 使用 `read_byte_blocking()` 等待数据到达
//! 6. **行缓冲回显**: 接收一整行 (以回车结束) 后统一回发
//! 7. **`core::fmt::Write`**: 使用 `write!` 宏格式化输出
//!
//! ---
//!
//! # 2. 硬件接线
//!
//! ## 2.1 UART0 引脚
//!
//! | Pico GPIO | UART 功能 | 方向 | 说明 |
//! |-----------|----------|------|------|
//! | GP0 | UART0_TX | 输出 | Pico 发送 → 对端接收 |
//! | GP1 | UART0_RX | 输入 | 对端发送 → Pico 接收 |
//!
//! ## 2.2 连接 USB-TTL 串口适配器
//!
//! ```text
//!   Pico 2 (RP2350)           USB-TTL 适配器
//!   ──────────────            ──────────────
//!   GP0 (TX) ────────────────► RX
//!   GP1 (RX) ◄──────────────── TX
//!   GND ──────────────────────  GND
//!
//!   注意: TX ↔ RX 交叉连接!
//!   Pico TX → 适配器 RX
//!   Pico RX ← 适配器 TX
//! ```
//!
//! > **电压**: Pico GPIO 为 3.3V。使用 3.3V TTL 适配器，
//! > 或确保 5V 适配器有 3.3V 兼容输入。
//!
//! ## 2.3 UART0 可选引脚
//!
//! | 功能 | 可选 GPIO |
//! |------|-----------|
//! | TX | GP0, GP12, GP16, GP28 |
//! | RX | GP1, GP13, GP17, GP29 |
//! | CTS | GP2, GP14, GP18 |
//! | RTS | GP3, GP15, GP19 |
//!
//! ## 2.4 回环测试 (无需外部设备)
//!
//! 将 **GP0 (TX)** 和 **GP1 (RX)** 短接，
//! 发送的数据会立即被自己接收 — 适合验证 UART 基本功能。
//!
//! ---
//!
//! # 3. 配置参数
//!
//! | 参数 | 默认值 | 说明 |
//! |------|--------|------|
//! | `UART_BAUD` | 115200 | 波特率 (常用: 9600, 115200, 921600) |
//! | 数据位 | 8 | DataBits::Eight |
//! | 停止位 | 1 | StopBits::One |
//! | 校验 | 无 | Parity::None |
//!
//! ---
//!
//! # 4. 测试方法
//!
//! 在电脑上用串口终端软件连接 USB-TTL 适配器:
//!
//! ```bash
//! # Linux / macOS
//! picocom -b 115200 /dev/ttyUSB0
//! # 或
//! screen /dev/ttyUSB0 115200
//!
//! # Windows
//! # 使用 PuTTY 或 Tera Term, 选择对应 COM 口, 115200 baud
//! ```
//!
//! 程序启动后会打印欢迎信息，随后进入回显模式 — 输入任何字符会立即显示回来。
//!
//! ---
//!
//! # 5. 编译运行
//!
//! ```bash
//! cargo run --bin uart_echo --target thumbv8m.main-none-eabihf
//! ```

#![no_std]
#![no_main]

use core::fmt::Write;

use defmt::*;
use defmt_rtt as _;

use roschi::block::ImageDef;
use roschi::clocks::ClockBuilder;
use roschi::entry;
use roschi::gpio::Pins;
use roschi::pac::Peripherals;
use roschi::timer::Timer0;
use roschi::uart::{DataBits, Parity, StopBits, Uart0};

#[unsafe(link_section = ".start_block")]
#[used]
pub static IMAGE_DEF: ImageDef = ImageDef::exe();

// ============================================================================
// 配置参数
// ============================================================================

/// UART 波特率
/// 常用值: 9600, 19200, 38400, 57600, 115200, 230400, 460800, 921600
const UART_BAUD: u32 = 115_200;

/// 行缓冲区大小 (用于 Demo 3: 行模式回显)
const LINE_BUF_SIZE: usize = 128;

#[entry]
fn main() -> ! {
    info!("=== UART Echo Example ===");

    // ------------------------------------------------------------------
    // 1. 外设初始化
    // ------------------------------------------------------------------
    let dp = Peripherals::take().unwrap();

    // 系统时钟: 150 MHz
    let clocks = ClockBuilder::new().build(dp.CLOCKS, dp.XOSC, dp.PLL_SYS, dp.PLL_USB);
    info!("System clock: {} Hz", clocks.sys_freq());

    let pins = Pins::new(dp.IO_BANK0, dp.PADS_BANK0);
    let timer = Timer0::init(dp.TIMER0);

    // ------------------------------------------------------------------
    // 2. UART0 初始化
    //    GP0 = TX, GP1 = RX
    // ------------------------------------------------------------------
    let mut uart = Uart0::new(
        dp.UART0,
        pins.pin::<0>(), // TX
        pins.pin::<1>(), // RX
        UART_BAUD,
        clocks.peri_freq(),
    );
    uart.set_format(DataBits::Eight, StopBits::One, Parity::None);

    info!("UART0: {} baud, 8N1 (GP0=TX, GP1=RX)", UART_BAUD);

    // ------------------------------------------------------------------
    // 3. 发送欢迎信息 — 使用 write() 发送原始字节
    // ------------------------------------------------------------------
    uart.write(b"\r\n");
    uart.write(b"============================================\r\n");
    uart.write(b"  RP2350 UART Echo Example\r\n");
    uart.write(b"  Baud: 115200, Format: 8N1\r\n");
    uart.write(b"============================================\r\n");
    uart.write(b"\r\n");
    uart.flush();

    // ------------------------------------------------------------------
    // 4. 使用 core::fmt::Write trait — write! 宏格式化输出
    //    Uart0 实现了 core::fmt::Write，可以直接用 write!/writeln!
    // ------------------------------------------------------------------
    let _ = core::write!(
        uart,
        "System clock: {} MHz\r\n",
        clocks.sys_freq() / 1_000_000
    );
    let _ = core::write!(
        uart,
        "Peri clock:   {} MHz\r\n\r\n",
        clocks.peri_freq() / 1_000_000
    );
    uart.flush();

    info!("Welcome message sent");

    // ======================================================================
    // Demo 1: 非阻塞字符回显
    //
    // 使用 read_byte() 轮询 — 如果 RX FIFO 有数据返回 Some(byte)，
    // 没有则返回 None。适合在主循环中顺便检查。
    // ======================================================================
    uart.write(b"[Demo 1] Non-blocking echo (5 seconds)...\r\n");
    uart.write(b"Type characters - they will echo back immediately.\r\n\r\n");
    uart.flush();

    info!("Demo 1: Non-blocking echo for 5 seconds");

    let start = timer.get_time_us();
    let duration_us: u64 = 5_000_000; // 5 秒
    let mut echo_count: u32 = 0;

    while timer.get_time_us() - start < duration_us {
        // 非阻塞读取 — 不会卡在这里
        if let Some(byte) = uart.read_byte() {
            // 立即回发收到的字符
            uart.write_byte(byte);
            echo_count += 1;
        }
    }

    let _ = core::write!(
        uart,
        "\r\n[Demo 1] Echoed {} characters\r\n\r\n",
        echo_count
    );
    info!("Demo 1 done: {} chars echoed", echo_count);

    // ======================================================================
    // Demo 2: 阻塞读取 + 特殊字符处理
    //
    // read_byte_blocking() 会一直等到收到数据为止。
    // 这里演示对回车/退格等特殊字符的处理。
    // ======================================================================
    uart.write(b"[Demo 2] Blocking echo with special chars.\r\n");
    uart.write(b"Type characters. Press Enter to see line count.\r\n");
    uart.write(b"Press Ctrl+C (0x03) to advance to Demo 3.\r\n\r\n");
    uart.flush();

    info!("Demo 2: Blocking echo (press Ctrl+C to exit)");

    let mut line_count: u32 = 0;

    loop {
        let byte = uart.read_byte_blocking();

        match byte {
            // Ctrl+C (ETX) — 退出本演示
            0x03 => {
                uart.write(b"\r\n[Ctrl+C received]\r\n\r\n");
                break;
            }
            // 回车 (CR) — 换行并打印行计数
            b'\r' => {
                line_count += 1;
                let _ = core::write!(uart, "\r\n[line {}]\r\n", line_count);
            }
            // 退格 (BS / DEL) — 回退光标并擦除字符
            0x08 | 0x7F => {
                uart.write(b"\x08 \x08"); // 退格 + 空格覆盖 + 再退格
            }
            // 普通可打印字符 — 直接回显
            _ => {
                uart.write_byte(byte);
            }
        }
    }

    info!("Demo 2 done: {} lines", line_count);

    // ======================================================================
    // Demo 3: 行缓冲模式
    //
    // 收集一整行 (以回车结束)，然后将整行回发并加上行号。
    // 适合命令行交互式应用。
    // ======================================================================
    uart.write(b"[Demo 3] Line buffer mode.\r\n");
    uart.write(b"Type a line and press Enter. Type 'quit' to exit.\r\n\r\n");
    uart.flush();

    info!("Demo 3: Line buffer mode");

    let mut line_buf = [0u8; LINE_BUF_SIZE];
    let mut line_pos: usize = 0;
    let mut line_num: u32 = 0;

    loop {
        let byte = uart.read_byte_blocking();

        match byte {
            // 回车 — 处理完整行
            b'\r' => {
                uart.write(b"\r\n");
                line_num += 1;

                if line_pos > 0 {
                    // 回显: "[行号] > 内容"
                    let _ = core::write!(uart, "[{}] > ", line_num);
                    uart.write(&line_buf[..line_pos]);
                    uart.write(b"\r\n");

                    // 检查 "quit" 命令
                    if line_pos == 4
                        && line_buf[0] == b'q'
                        && line_buf[1] == b'u'
                        && line_buf[2] == b'i'
                        && line_buf[3] == b't'
                    {
                        uart.write(b"[Goodbye!]\r\n\r\n");
                        break;
                    }
                }

                line_pos = 0; // 重置缓冲区
            }
            // 退格
            0x08 | 0x7F => {
                if line_pos > 0 {
                    line_pos -= 1;
                    uart.write(b"\x08 \x08");
                }
            }
            // 普通字符 — 存入缓冲区并回显
            _ => {
                if line_pos < LINE_BUF_SIZE {
                    line_buf[line_pos] = byte;
                    line_pos += 1;
                    uart.write_byte(byte); // 本地回显
                }
            }
        }
    }

    info!("Demo 3 done: {} lines", line_num);

    // ======================================================================
    // 无限循环: 持续字符回显
    // ======================================================================
    uart.write(b"=== Continuous echo mode ===\r\n");
    uart.write(b"All received characters will be echoed back.\r\n\r\n");
    uart.flush();

    info!("Entering continuous echo loop");

    loop {
        let byte = uart.read_byte_blocking();
        uart.write_byte(byte);

        // 如果收到回车，自动补发换行 (\n)
        if byte == b'\r' {
            uart.write_byte(b'\n');
        }
    }
}
