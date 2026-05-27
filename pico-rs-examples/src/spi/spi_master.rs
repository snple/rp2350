//! # SPI 主机通信示例
//!
//! 基于 RP2350 (Raspberry Pi Pico 2) 的 **SPI 主机 (Master)** 通信示例，
//! 演示如何使用 pico-rs 的类型安全 SPI API 进行数据收发。
//!
//! ---
//!
//! # 1. 功能概述
//!
//! ## 1.1 SPI 基础
//!
//! SPI (Serial Peripheral Interface) 是一种全双工、同步的串行通信协议：
//!
//! ```text
//!   Master (Pico 2)            Slave (外设)
//!   ─────────────              ───────────
//!   SCK   ─────────────────►  SCK    (时钟，由主机产生)
//!   TX/MOSI ────────────────►  MOSI   (主→从 数据)
//!   RX/MISO ◄────────────────  MISO   (从→主 数据)
//!   CSn   ─────────────────►  CS     (片选，低有效)
//! ```
//!
//! - **全双工**: 每个时钟周期同时收发 1 位数据
//! - **主机控制时钟**: 从机不产生时钟，由主机 SCK 驱动
//! - **片选 (CS)**: 低有效，主机拉低选中的从机才参与通信
//!
//! ## 1.2 SPI 模式 (Mode 0 ~ Mode 3)
//!
//! | 模式 | CPOL | CPHA | 说明 |
//! |------|------|------|------|
//! | Mode0 | 0 | 0 | 空闲低电平，上升沿采样 (**最常用**) |
//! | Mode1 | 0 | 1 | 空闲低电平，下降沿采样 |
//! | Mode2 | 1 | 0 | 空闲高电平，下降沿采样 |
//! | Mode3 | 1 | 1 | 空闲高电平，上升沿采样 |
//!
//! 大多数 SPI 外设 (Flash、传感器、屏幕) 使用 **Mode 0** 或 **Mode 3**。
//!
//! ## 1.3 本示例演示内容
//!
//! 1. **SPI0 初始化**: 配置时钟频率、数据格式、SPI 模式
//! 2. **写操作 (write)**: 主机发送数据到从机
//! 3. **读操作 (read)**: 主机从从机读取数据
//! 4. **全双工传输 (transfer)**: 同时收发
//! 5. **手动 CS 控制**: GPIO 软件控制片选，支持多从机
//! 6. **修改波特率**: 运行时动态调整 SPI 时钟频率
//! 7. **切换 SPI 模式**: 演示不同 CPOL/CPHA 的配置
//!
//! ---
//!
//! # 2. 硬件接线
//!
//! ## 2.1 SPI0 引脚分配
//!
//! | Pico GPIO | SPI 功能 | 方向 | 说明 |
//! |-----------|---------|------|------|
//! | GP2 | SCK | 输出 | SPI 时钟 (Master → Slave) |
//! | GP3 | TX (MOSI) | 输出 | 主机发送数据 |
//! | GP4 | RX (MISO) | 输入 | 主机接收数据 |
//! | GP5 | CSn | 输出 | 片选 (GPIO 软件控制，低有效) |
//!
//! ```text
//!   Pico 2 (RP2350)                     SPI 从设备
//!   ──────────────                      ──────────
//!   GP2 (SCK)  ──────────────────────── SCK  / CLK
//!   GP3 (MOSI) ──────────────────────── MOSI / SDI / DIN
//!   GP4 (MISO) ◄─────────────────────── MISO / SDO / DOUT
//!   GP5 (CSn)  ──────────────────────── CS   / SS  / NSS (低有效)
//!
//!   3.3V ────────────────────────────── VCC
//!   GND  ────────────────────────────── GND
//! ```
//!
//! > **注意**: RP2350 GPIO 电平为 3.3V。如果外设需要 5V 逻辑，需要电平转换器。
//!
//! ## 2.2 SPI0 可选引脚
//!
//! | 功能 | 可选 GPIO |
//! |------|-----------|
//! | SCK | GP2, GP6, GP18, GP22 |
//! | TX (MOSI) | GP3, GP7, GP19, GP23 |
//! | RX (MISO) | GP0, GP4, GP16, GP20 |
//! | CSn | GP1, GP5, GP17, GP21 |
//!
//! ---
//!
//! # 3. 配置参数
//!
//! | 参数 | 默认值 | 说明 |
//! |------|--------|------|
//! | `SPI_BAUDRATE` | 1_000_000 | SPI 时钟频率 (Hz), 1 MHz |
//! | `SPI_MODE` | Mode0 | SPI 模式 (CPOL=0, CPHA=0) |
//! | `DATA_BITS` | 8 | 每帧数据位数 |
//!
//! ---
//!
//! # 4. 回环测试 (Loopback)
//!
//! **无需外部从设备**也可以测试 SPI 通信：
//! 将 **MOSI (GP3)** 和 **MISO (GP4)** 用杜邦线短接，
//! 发送的数据会通过 MOSI 输出后从 MISO 回到主机，
//! 程序会自动检测回环并验证数据一致性。
//!
//! ---
//!
//! # 5. 编译运行
//!
//! ```bash
//! cargo run --bin spi_master --target thumbv8m.main-none-eabihf
//! ```

#![no_std]
#![no_main]

use defmt::*;
use defmt_rtt as _;

use pico_rs::block::ImageDef;
use pico_rs::clocks::ClockBuilder;
use pico_rs::entry;
use pico_rs::gpio::Pins;
use pico_rs::pac::Peripherals;
use pico_rs::spi::{FrameFormat, Mode, Spi0};
use pico_rs::timer::Timer0;

#[unsafe(link_section = ".start_block")]
#[used]
pub static IMAGE_DEF: ImageDef = ImageDef::exe();

// ============================================================================
// 配置参数
// ============================================================================

/// SPI 时钟频率 (Hz)
/// RP2350 SPI 最高支持 sys_clk/2 = 75 MHz (150 MHz 系统时钟时)
/// 大多数外设推荐 1~10 MHz 即可
const SPI_BAUDRATE: u32 = 1_000_000;

/// 片选引脚 — 使用 GPIO 软件控制 (非硬件 CSn)
/// 软件控制 CS 的优势: 可在一次传输中灵活控制 CS 的拉高拉低时机
const _PIN_CS: u32 = 5;

#[entry]
fn main() -> ! {
    info!("=== SPI Master Example ===");

    // ------------------------------------------------------------------
    // 1. 外设初始化
    // ------------------------------------------------------------------
    let dp = Peripherals::take().unwrap();

    // 配置系统时钟: 150 MHz
    let clocks = ClockBuilder::new().build(dp.CLOCKS, dp.XOSC, dp.PLL_SYS, dp.PLL_USB);
    info!("System clock: {} Hz", clocks.sys_freq());

    let pins = Pins::new(dp.IO_BANK0, dp.PADS_BANK0);
    let timer = Timer0::init(dp.TIMER0);

    // ------------------------------------------------------------------
    // 2. SPI0 初始化
    //    GP2 = SCK, GP3 = TX (MOSI), GP4 = RX (MISO)
    //    不使用硬件 CSn，用 GP5 GPIO 软件控制片选
    // ------------------------------------------------------------------
    let mut spi = Spi0::new(
        dp.SPI0,
        pins.pin::<2>(), // SCK
        pins.pin::<3>(), // TX (MOSI)
        pins.pin::<4>(), // RX (MISO)
        SPI_BAUDRATE,
        clocks.peri_freq(),
    );

    // 设置 SPI 格式: 8 位数据, Mode0 (CPOL=0, CPHA=0), Motorola 标准帧格式
    spi.set_format(8, Mode::Mode0, FrameFormat::MotorolaSpi);

    let actual_baud = spi.set_baudrate(SPI_BAUDRATE, clocks.peri_freq());
    info!(
        "SPI0 initialized: requested {} Hz, actual {} Hz",
        SPI_BAUDRATE, actual_baud
    );

    // ------------------------------------------------------------------
    // 3. 片选 (CS) — GPIO 软件控制
    // ------------------------------------------------------------------
    let cs = pins.pin::<5>();
    cs.into_output();
    cs.set_high(); // CS 空闲为高 (未选中)

    info!("Pin config: GP2(SCK) GP3(MOSI) GP4(MISO) GP5(CS)");

    // ======================================================================
    // 演示 1: 基本写操作 — 向从机发送数据
    // ======================================================================
    info!("--- Demo 1: SPI Write ---");
    {
        let tx_data: [u8; 4] = [0xDE, 0xAD, 0xBE, 0xEF];

        cs.set_low(); // 拉低 CS 选中从机
        spi.write(&tx_data); // 发送 4 字节
        spi.flush(); // 等待发送完成
        cs.set_high(); // 释放 CS

        info!(
            "Sent {} bytes: [{:#04x}, {:#04x}, {:#04x}, {:#04x}]",
            tx_data.len(),
            tx_data[0],
            tx_data[1],
            tx_data[2],
            tx_data[3]
        );
    }
    timer.delay_ms(100);

    // ======================================================================
    // 演示 2: 基本读操作 — 从从机接收数据
    // ======================================================================
    info!("--- Demo 2: SPI Read ---");
    {
        let mut rx_buf = [0u8; 4];

        cs.set_low();
        spi.read(&mut rx_buf); // 读取 4 字节 (同时发送 0x00 作为时钟驱动)
        cs.set_high();

        info!(
            "Received {} bytes: [{:#04x}, {:#04x}, {:#04x}, {:#04x}]",
            rx_buf.len(),
            rx_buf[0],
            rx_buf[1],
            rx_buf[2],
            rx_buf[3]
        );
    }
    timer.delay_ms(100);

    // ======================================================================
    // 演示 3: 全双工传输 — 同时收发
    //
    // 如果 MOSI 和 MISO 短接 (回环测试)，收到的数据应该等于发送的数据。
    // ======================================================================
    info!("--- Demo 3: SPI Full-Duplex Transfer ---");
    {
        let tx_data: [u8; 8] = [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08];
        let mut rx_buf = [0u8; 8];

        cs.set_low();
        spi.transfer(&mut rx_buf, &tx_data); // 同时发送 tx_data 并接收到 rx_buf
        cs.set_high();

        info!(
            "TX: [{:#04x}, {:#04x}, {:#04x}, {:#04x}, {:#04x}, {:#04x}, {:#04x}, {:#04x}]",
            tx_data[0],
            tx_data[1],
            tx_data[2],
            tx_data[3],
            tx_data[4],
            tx_data[5],
            tx_data[6],
            tx_data[7]
        );
        info!(
            "RX: [{:#04x}, {:#04x}, {:#04x}, {:#04x}, {:#04x}, {:#04x}, {:#04x}, {:#04x}]",
            rx_buf[0], rx_buf[1], rx_buf[2], rx_buf[3], rx_buf[4], rx_buf[5], rx_buf[6], rx_buf[7]
        );

        // 回环检测: 如果 MOSI-MISO 短接，TX==RX
        if tx_data == rx_buf {
            info!("Loopback detected: TX == RX (MOSI-MISO shorted)");
        } else {
            info!("Normal mode: TX != RX (external slave connected or open)");
        }
    }
    timer.delay_ms(100);

    // ======================================================================
    // 演示 4: 寄存器式通信模式
    //
    // 许多 SPI 外设 (传感器、Flash) 采用 "命令 + 数据" 协议:
    //   - 先发送寄存器地址 / 命令字节
    //   - 再读取或写入数据
    //   - CS 在整个事务期间保持低电平
    // ======================================================================
    info!("--- Demo 4: Register-style Read (cmd + response) ---");
    {
        let cmd: [u8; 1] = [0x9F]; // 示例: SPI Flash 的 JEDEC ID 读取命令
        let mut response = [0u8; 3];

        cs.set_low();
        spi.write(&cmd); // 发送命令
        spi.read(&mut response); // 读取 3 字节回复
        cs.set_high();

        info!(
            "Command {:#04x} → response: [{:#04x}, {:#04x}, {:#04x}]",
            cmd[0], response[0], response[1], response[2]
        );
        info!("(For SPI Flash: Manufacturer ID, Memory Type, Capacity)");
    }
    timer.delay_ms(100);

    // ======================================================================
    // 演示 5: 动态修改波特率
    // ======================================================================
    info!("--- Demo 5: Baudrate Change ---");
    {
        // 提升到 4 MHz
        let new_baud = spi.set_baudrate(4_000_000, clocks.peri_freq());
        info!("Baudrate changed to {} Hz (requested 4 MHz)", new_baud);

        let tx = [0xAA, 0x55, 0xAA, 0x55];
        cs.set_low();
        spi.write(&tx);
        spi.flush();
        cs.set_high();

        info!("Sent 4 bytes at {} Hz", new_baud);

        // 恢复到原始波特率
        let restored = spi.set_baudrate(SPI_BAUDRATE, clocks.peri_freq());
        info!("Baudrate restored to {} Hz", restored);
    }
    timer.delay_ms(100);

    // ======================================================================
    // 演示 6: 切换 SPI 模式
    // ======================================================================
    info!("--- Demo 6: SPI Mode Switch ---");
    {
        let modes = [
            (Mode::Mode0, "Mode0 (CPOL=0, CPHA=0)"),
            (Mode::Mode1, "Mode1 (CPOL=0, CPHA=1)"),
            (Mode::Mode2, "Mode2 (CPOL=1, CPHA=0)"),
            (Mode::Mode3, "Mode3 (CPOL=1, CPHA=1)"),
        ];

        for (mode, name) in modes.iter() {
            spi.set_mode(*mode);
            info!("Switched to {}", name);

            let tx = [0x42];
            cs.set_low();
            spi.write(&tx);
            spi.flush();
            cs.set_high();

            timer.delay_ms(10);
        }

        // 恢复 Mode0
        spi.set_mode(Mode::Mode0);
        info!("Restored to Mode0");
    }
    timer.delay_ms(100);

    // ======================================================================
    // 演示 7: 16 位字传输
    // ======================================================================
    info!("--- Demo 7: 16-bit Word Transfer ---");
    {
        // 切换到 16 位帧格式
        spi.set_format(16, Mode::Mode0, FrameFormat::MotorolaSpi);

        cs.set_low();
        spi.write_word(0x1234); // 发送 16 位字
        spi.write_word(0x5678);
        spi.flush();
        let w1 = spi.read_word(); // 读取 16 位字
        cs.set_high();

        info!("Sent words: 0x1234, 0x5678; Read: {:#06x}", w1);

        // 恢复 8 位帧格式
        spi.set_format(8, Mode::Mode0, FrameFormat::MotorolaSpi);
        info!("Restored to 8-bit frame format");
    }

    // ======================================================================
    // 无限循环: 定期发送心跳数据
    // ======================================================================
    info!("=== Entering heartbeat loop ===");

    let mut counter: u32 = 0;
    loop {
        let tx = [
            (counter >> 24) as u8,
            (counter >> 16) as u8,
            (counter >> 8) as u8,
            counter as u8,
        ];
        let mut rx = [0u8; 4];

        cs.set_low();
        spi.transfer(&mut rx, &tx);
        cs.set_high();

        info!(
            "[#{}] TX={:#010x} RX={:#010x}",
            counter,
            u32::from_be_bytes(tx),
            u32::from_be_bytes(rx)
        );

        counter = counter.wrapping_add(1);
        timer.delay_ms(1000);
    }
}
