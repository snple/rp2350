//! # TCAN4550 CAN 总线通信示例
//!
//! 基于 RP2350 (Raspberry Pi Pico 2) + TCAN4550 外部 CAN 控制器的 CAN 总线通信示例。
//!
//! TCAN4550 是 TI 的独立 CAN FD 控制器，内置 CAN 收发器，通过 SPI 接口与 MCU 通信。
//! 其内部集成了符合 Bosch MCAN 规范的 CAN 控制器核心，寄存器布局与 MCAN 一致。
//!
//! # 硬件连接
//!
//! ```text
//! RP2350 (Pico 2)          TCAN4550
//! ─────────────────         ──────────
//! GP2  (SCK)   ──────────►  SCLK
//! GP3  (MOSI)  ──────────►  SDI
//! GP4  (MISO)  ◄──────────  SDO
//! GP5  (CS)    ──────────►  nCS
//! GP6  (nINT)  ◄──────────  nINT (可选, 中断输出)
//! 3.3V         ──────────►  VIO  (数字 IO 供电)
//! GND          ──────────►  GND
//!                            VCC ← 5V (CAN 收发器供电)
//!                            CANH ───► CAN 总线
//!                            CANL ───► CAN 总线
//! ```
//!
//! # SPI 协议
//!
//! TCAN4550 使用 32-bit SPI 字进行通信 (SPI Mode 0, CPOL=0 CPHA=0)。
//! 每次 SPI 事务由一个 32-bit 头部开始，后跟 N 个 32-bit 数据字。
//!
//! ## SPI 头部格式 (32-bit)
//! ```text
//! [31]    R/W: 0 = 读, 1 = 写
//! [30:24] 保留 (0)
//! [23:8]  16-bit 地址
//! [7:0]   数据字数量 (N, 1-256, 0 代表 256)
//! ```
//!
//! # MCAN 寄存器地址空间
//!
//! TCAN4550 的寄存器分为两部分:
//! - 0x0000-0x0FFF: TCAN4550 设备专用寄存器 (SPI 状态、模式控制等)
//! - 0x1000-0x1FFF: MCAN 核心寄存器 (CAN 控制器配置、收发消息等)
//!
//! ## 消息 RAM (MRAM) 布局
//!
//! MCAN 使用片内 Message RAM 存储过滤器、RX FIFO、TX Buffer 等。
//! TCAN4550 的 MRAM 起始地址为 0x8000, 大小 2KB (0x8000-0x87FF)。
//!
//! # 运行
//!
//! ```bash
//! cargo run --bin tcan4550
//! ```

#![no_std]
#![no_main]
#![allow(dead_code)] // 寄存器定义预留给扩展使用

use defmt::*;
use defmt_rtt as _;

use pico_rs::block::ImageDef;
use pico_rs::clocks::{ClockBuilder, ClockResources};
use pico_rs::entry;
use pico_rs::gpio::{Pin, Pins};
use pico_rs::pac::Peripherals;
use pico_rs::spi::{FrameFormat, Mode, Spi0, SpiBus};
use pico_rs::timer::Timer0;

#[unsafe(link_section = ".start_block")]
#[used]
pub static IMAGE_DEF: ImageDef = ImageDef::exe();

// ============================================================================
// SPI 配置
// ============================================================================

/// TCAN4550 SPI 最大速率 18 MHz, 这里保守使用 2 MHz
const SPI_BAUDRATE: u32 = 2_000_000;

// ============================================================================
// TCAN4550 设备寄存器地址 (0x0000 - 0x0FFF)
// ============================================================================

/// 设备 ID1 寄存器 — 包含 "TCAN" ASCII 码 (0x5443414E)
const DEV_ID1: u16 = 0x0000;
/// 设备 ID2 寄存器 — 包含 "4550" ASCII 码 (0x34353530)
const DEV_ID2: u16 = 0x0004;
/// 设备版本与 SPI 版本寄存器
const DEV_REVISION: u16 = 0x0008;
/// 设备状态寄存器
const DEV_STATUS: u16 = 0x000C;
/// SPI 错误状态掩码
const SPI_ERR_STATUS: u16 = 0x0010;

/// 模式/引脚配置寄存器
const MODE_PINS: u16 = 0x0800;
/// 时间戳预分频寄存器
const TIMESTAMP_PRESCALER: u16 = 0x0804;
/// 测试与调试寄存器
const TEST_REG: u16 = 0x0808;
/// 中断寄存器
const DEV_IR: u16 = 0x0820;
/// 中断使能寄存器
const DEV_IE: u16 = 0x0830;

// ============================================================================
// MCAN 核心寄存器地址 (偏移 0x1000)
// ============================================================================

/// MCAN Core Release 寄存器
const MCAN_CREL: u16 = 0x1000;
/// MCAN 数据位时间与预分频寄存器
const MCAN_DBTP: u16 = 0x100C;
/// MCAN 测试寄存器
const MCAN_TEST: u16 = 0x1010;
/// MCAN RAM 看门狗
const MCAN_RWD: u16 = 0x1014;
/// MCAN CC 控制寄存器
const MCAN_CCCR: u16 = 0x1018;
/// MCAN 标称位时间与预分频寄存器
const MCAN_NBTP: u16 = 0x101C;
/// MCAN 时间戳计数器配置
const MCAN_TSCC: u16 = 0x1020;
/// MCAN 时间戳计数器值
const MCAN_TSCV: u16 = 0x1024;
/// MCAN 超时计数器配置
const MCAN_TOCC: u16 = 0x1028;
/// MCAN 超时计数器值
const MCAN_TOCV: u16 = 0x102C;
/// MCAN 协议状态寄存器
const MCAN_PSR: u16 = 0x1044;
/// MCAN 中断寄存器
const MCAN_IR: u16 = 0x1050;
/// MCAN 中断使能
const MCAN_IE: u16 = 0x1054;
/// MCAN 中断线选择
const MCAN_ILS: u16 = 0x1058;
/// MCAN 中断线使能
const MCAN_ILE: u16 = 0x105C;

/// 全局过滤器配置
const MCAN_GFC: u16 = 0x1080;
/// 标准 ID 过滤器配置
const MCAN_SIDFC: u16 = 0x1084;
/// 扩展 ID 过滤器配置
const MCAN_XIDFC: u16 = 0x1088;
/// 扩展 ID AND 掩码
const MCAN_XIDAM: u16 = 0x1090;

/// RX FIFO 0 配置
const MCAN_RXF0C: u16 = 0x10A0;
/// RX FIFO 0 状态
const MCAN_RXF0S: u16 = 0x10A4;
/// RX FIFO 0 确认
const MCAN_RXF0A: u16 = 0x10A8;
/// RX FIFO 1 配置
const MCAN_RXF1C: u16 = 0x10B0;
/// RX FIFO 1 状态
const MCAN_RXF1S: u16 = 0x10B4;

/// RX Buffer / FIFO 元素大小配置
const MCAN_RXESC: u16 = 0x10BC;

/// TX Buffer 配置
const MCAN_TXBC: u16 = 0x10C0;
/// TX FIFO/Queue 状态
const MCAN_TXFQS: u16 = 0x10C4;
/// TX Buffer 元素大小配置
const MCAN_TXESC: u16 = 0x10C8;
/// TX Buffer 请求挂起
const MCAN_TXBRP: u16 = 0x10CC;
/// TX Buffer 添加请求
const MCAN_TXBAR: u16 = 0x10D0;
/// TX Buffer 取消请求
const MCAN_TXBCR: u16 = 0x10D4;
/// TX Buffer 传输完成
const MCAN_TXBTO: u16 = 0x10D8;
/// TX Buffer 取消完成
const MCAN_TXBCF: u16 = 0x10DC;
/// TX Buffer 传输中断使能
const MCAN_TXBTIE: u16 = 0x10E0;
/// TX Buffer 取消完成中断使能
const MCAN_TXBCIE: u16 = 0x10E4;

// ============================================================================
// MRAM (Message RAM) 地址 — TCAN4550 MRAM 起始于 0x8000
// ============================================================================

const MRAM_BASE: u16 = 0x8000;

// MRAM 分区偏移 (相对于 MRAM_BASE)
// 布局: SID Filter(8) | XID Filter(0) | RX FIFO0(3) | RX FIFO1(0) | TX Event(0) | TX Buffer(2)

/// 标准 ID 过滤器: 每个 4 bytes, 最多 8 个
const MRAM_SID_FILTER_OFFSET: u16 = 0x0000;
const MRAM_SID_FILTER_NUM: u8 = 8;

/// RX FIFO 0: 每个元素 = 8 (header) + 8 (data, 8 bytes) = 16 bytes, 3 个元素
const MRAM_RX_FIFO0_OFFSET: u16 = 0x0020; // 8 * 4 = 0x20 after SID filters
const MRAM_RX_FIFO0_NUM: u8 = 3;

/// TX Buffer: 每个元素 = 8 (header) + 8 (data, 8 bytes) = 16 bytes, 2 个元素
const MRAM_TX_BUFFER_OFFSET: u16 = 0x0050; // 0x20 + 3 * 0x10 = 0x50
const MRAM_TX_BUFFER_NUM: u8 = 2;

// ============================================================================
// MCAN CCCR 寄存器位定义
// ============================================================================

const CCCR_INIT: u32 = 1 << 0; // 初始化模式请求
const CCCR_CCE: u32 = 1 << 1; // 配置变更使能
const CCCR_MON: u32 = 1 << 5; // 总线监听模式
const CCCR_DAR: u32 = 1 << 6; // 禁用自动重传
const CCCR_BRSE: u32 = 1 << 9; // 比特率切换使能 (CAN FD)
const CCCR_FDOE: u32 = 1 << 8; // CAN FD 操作使能

// ============================================================================
// MODE_PINS 寄存器位定义
// ============================================================================

/// 正常模式: 设备进入 Normal 操作模式 (MODE_SEL = 0b10)
const MODE_NORMAL: u32 = 0x00000060; // bit[6:5] = 0b11 → Normal mode, nWKRQ/INH/WAKE disabled
/// Standby 模式 (低功耗, CAN 收发功能关闭)
const MODE_STANDBY: u32 = 0x00000040; // bit[6:5] = 0b10

// ============================================================================
// SPI 读写操作
// ============================================================================

/// SPI 读取操作: 从 TCAN4550 寄存器读取 32-bit 值
///
/// 协议:
/// 1. 发送 32-bit header: [R/W=0][reserved][16-bit addr][8-bit word count=1]
/// 2. 读取 32-bit 数据
fn spi_read_word(spi: &impl SpiBus, cs: &Pin<5>, addr: u16) -> u32 {
    let header: [u8; 4] = [
        0x00 | ((addr >> 8) as u8 & 0xFF), // R/W=0 (read) | addr high
        (addr & 0xFF) as u8,               // addr low
        0x00,                              // reserved
        0x01,                              // word count = 1
    ];

    let mut rx = [0u8; 4];

    cs.set_low();
    spi.write(&header);
    spi.read(&mut rx);
    cs.set_high();

    u32::from_be_bytes(rx)
}

/// SPI 写入操作: 向 TCAN4550 寄存器写入 32-bit 值
fn spi_write_word(spi: &impl SpiBus, cs: &Pin<5>, addr: u16, data: u32) {
    let header: [u8; 4] = [
        0x61 | ((addr >> 8) as u8 & 0x1F), // R/W=1 (write, bit[7:5]=011) | addr high
        (addr & 0xFF) as u8,               // addr low
        0x00,                              // reserved
        0x01,                              // word count = 1
    ];

    let data_bytes: [u8; 4] = data.to_be_bytes();

    cs.set_low();
    spi.write(&header);
    spi.write(&data_bytes);
    cs.set_high();
}

/// SPI 读取多个连续 32-bit 字
fn spi_read_words(spi: &impl SpiBus, cs: &Pin<5>, addr: u16, buf: &mut [u32]) {
    let count = buf.len() as u8;
    let header: [u8; 4] = [
        0x00 | ((addr >> 8) as u8 & 0xFF),
        (addr & 0xFF) as u8,
        0x00,
        count,
    ];

    cs.set_low();
    spi.write(&header);
    for word in buf.iter_mut() {
        let mut rx = [0u8; 4];
        spi.read(&mut rx);
        *word = u32::from_be_bytes(rx);
    }
    cs.set_high();
}

/// SPI 写入多个连续 32-bit 字
fn spi_write_words(spi: &impl SpiBus, cs: &Pin<5>, addr: u16, data: &[u32]) {
    let count = data.len() as u8;
    let header: [u8; 4] = [
        0x61 | ((addr >> 8) as u8 & 0x1F),
        (addr & 0xFF) as u8,
        0x00,
        count,
    ];

    cs.set_low();
    spi.write(&header);
    for &word in data {
        spi.write(&word.to_be_bytes());
    }
    cs.set_high();
}

// ============================================================================
// MCAN 配置辅助
// ============================================================================

/// 进入 MCAN 初始化模式 (CCCR.INIT=1, CCCR.CCE=1)
fn mcan_enter_init(spi: &impl SpiBus, cs: &Pin<5>, timer: &Timer0) {
    // 请求初始化模式
    spi_write_word(spi, cs, MCAN_CCCR, CCCR_INIT);
    timer.delay_ms(1);

    // 等待 INIT 位置位
    for _ in 0..100 {
        let cccr = spi_read_word(spi, cs, MCAN_CCCR);
        if cccr & CCCR_INIT != 0 {
            break;
        }
        timer.delay_ms(1);
    }

    // 使能配置变更
    spi_write_word(spi, cs, MCAN_CCCR, CCCR_INIT | CCCR_CCE);
    timer.delay_ms(1);
}

/// 退出 MCAN 初始化模式, 进入正常操作
fn mcan_exit_init(spi: &impl SpiBus, cs: &Pin<5>, timer: &Timer0) {
    // 清除 INIT 和 CCE, 进入正常模式
    spi_write_word(spi, cs, MCAN_CCCR, 0);
    timer.delay_ms(1);

    // 等待 INIT 位清零
    for _ in 0..100 {
        let cccr = spi_read_word(spi, cs, MCAN_CCCR);
        if cccr & CCCR_INIT == 0 {
            info!("MCAN 已进入正常模式");
            return;
        }
        timer.delay_ms(1);
    }
    warn!("MCAN 退出初始化超时");
}

// ============================================================================
// 主程序
// ============================================================================

#[entry]
fn main() -> ! {
    info!("=== TCAN4550 CAN 总线示例 ===");

    // 初始化系统时钟
    let dp = Peripherals::take().unwrap();
    let clocks = ClockBuilder::new().build(ClockResources { clocks: dp.CLOCKS, xosc: dp.XOSC, pll_sys: dp.PLL_SYS, pll_usb: dp.PLL_USB });
    info!("系统时钟: {} Hz", clocks.sys_freq());

    // 初始化 GPIO 和定时器
    let pins = Pins::new(dp.IO_BANK0, dp.PADS_BANK0);
    let timer = Timer0::init(dp.TIMER0);

    // 初始化 SPI0 (Mode 0)
    let spi = Spi0::new(
        dp.SPI0,
        pins.pin::<2>(), // SCK
        pins.pin::<3>(), // TX (MOSI → SDI)
        pins.pin::<4>(), // RX (MISO ← SDO)
        SPI_BAUDRATE,
        clocks.peri_freq(),
    );
    spi.set_format(8, Mode::Mode0, FrameFormat::MotorolaSpi);
    info!("SPI0 已初始化: {} Hz", spi.baudrate(clocks.peri_freq()));

    // CS 引脚 (手动控制)
    let cs = pins.pin::<5>();
    cs.into_output();
    cs.set_high();

    // nINT 引脚 (中断输入, 可选)
    let _nint = pins.pin::<6>();
    _nint.into_input_pullup();

    // LED 指示灯
    let led = pins.pin::<25>();
    led.into_output();
    led.set_low();

    info!("引脚配置: GP2(SCK) GP3(MOSI) GP4(MISO) GP5(CS) GP6(nINT)");
    timer.delay_ms(100);

    // ========================================================================
    // 步骤 1: 读取设备 ID, 验证 SPI 通信
    // ========================================================================
    info!("--- 步骤 1: 读取 TCAN4550 设备 ID ---");

    let id1 = spi_read_word(&spi, &cs, DEV_ID1);
    let id2 = spi_read_word(&spi, &cs, DEV_ID2);
    let rev = spi_read_word(&spi, &cs, DEV_REVISION);

    info!("DEV_ID1: {:#010x} (期望 0x5443414E = \"TCAN\")", id1);
    info!("DEV_ID2: {:#010x} (期望 0x34353530 = \"4550\")", id2);
    info!("DEV_REV: {:#010x}", rev);

    if id1 != 0x5443414E || id2 != 0x34353530 {
        error!("TCAN4550 设备 ID 校验失败! 请检查 SPI 连接。");
        error!("  - 确认 MOSI/MISO 未接反");
        error!("  - 确认 VIO 供电正常 (3.3V)");
        error!("  - 确认 CS 引脚正确");
        loop {
            led.toggle();
            timer.delay_ms(100); // 快闪表示错误
        }
    }
    info!("TCAN4550 设备 ID 校验通过!");

    // ========================================================================
    // 步骤 2: 清除 SPI 错误状态
    // ========================================================================
    info!("--- 步骤 2: 清除 SPI 错误状态 ---");

    let spi_status = spi_read_word(&spi, &cs, SPI_ERR_STATUS);
    info!("SPI 错误状态: {:#010x}", spi_status);
    // 写 1 清零
    if spi_status != 0 {
        spi_write_word(&spi, &cs, SPI_ERR_STATUS, spi_status);
        info!("SPI 错误状态已清除");
    }

    // ========================================================================
    // 步骤 3: 将设备切换到 Standby 模式, 以便配置 MCAN
    // ========================================================================
    info!("--- 步骤 3: 进入 Standby 模式 ---");

    spi_write_word(&spi, &cs, MODE_PINS, MODE_STANDBY);
    timer.delay_ms(5);

    let mode = spi_read_word(&spi, &cs, MODE_PINS);
    info!("MODE_PINS: {:#010x}", mode);

    // ========================================================================
    // 步骤 4: 进入 MCAN 初始化模式
    // ========================================================================
    info!("--- 步骤 4: 进入 MCAN 初始化模式 ---");

    mcan_enter_init(&spi, &cs, &timer);

    let cccr = spi_read_word(&spi, &cs, MCAN_CCCR);
    info!(
        "CCCR: {:#010x} (INIT={}, CCE={})",
        cccr,
        (cccr & CCCR_INIT) >> 0,
        (cccr & CCCR_CCE) >> 1
    );

    let crel = spi_read_word(&spi, &cs, MCAN_CREL);
    info!("MCAN Core Release: {:#010x}", crel);

    // ========================================================================
    // 步骤 5: 配置 CAN 位时序 (500 kbit/s, 采样点 ~80%)
    // ========================================================================
    //
    // TCAN4550 MCAN 时钟 = 40 MHz
    //
    // 标称位时序 (Classic CAN / CAN FD 仲裁段):
    //   比特率 = 500 kbit/s
    //   Tq = 40 MHz / (NBRP+1) = 40 MHz / 1 = 40 MHz
    //   位时间 = NTSEG1 + NTSEG2 + 1(同步段) = 63 + 16 + 1 = 80 Tq
    //   比特率 = 40 MHz / 80 = 500 kbit/s
    //   采样点 = (1 + NTSEG1) / (1 + NTSEG1 + NTSEG2) = 64 / 80 = 80%
    //   SJW = 16
    //
    info!("--- 步骤 5: 配置 CAN 位时序 ---");

    // NBTP: NSJW[31:25]=15 | NBRP[24:16]=0 | NTSEG1[15:8]=63 | NTSEG2[6:0]=15
    let nbtp: u32 = (15 << 25)   // NSJW = 16 (值-1)
                  | (0 << 16)    // NBRP = 1  (值-1=0, 预分频 1)
                  | (63 << 8)    // NTSEG1 = 64 (值-1)
                  | (15 << 0); // NTSEG2 = 16 (值-1)
    spi_write_word(&spi, &cs, MCAN_NBTP, nbtp);
    info!("NBTP: {:#010x} (500 kbit/s, 采样点 80%)", nbtp);

    // 数据段位时序 (CAN FD 数据段): 2 Mbit/s
    //   Tq = 40 MHz / 1 = 40 MHz
    //   位时间 = DTSEG1 + DTSEG2 + 1 = 15 + 4 + 1 = 20 Tq
    //   比特率 = 40 MHz / 20 = 2 Mbit/s
    //   采样点 = 16 / 20 = 80%
    //   SJW = 4
    let dbtp: u32 = (3 << 0)    // DSJW = 4  (值-1)
                  | (0 << 16)   // DBRP = 1  (值-1=0)
                  | (14 << 8)   // DTSEG1 = 15 (值-1)
                  | (3 << 4); // DTSEG2 = 4  (值-1)
    spi_write_word(&spi, &cs, MCAN_DBTP, dbtp);
    info!("DBTP: {:#010x} (2 Mbit/s CAN FD 数据段)", dbtp);

    // ========================================================================
    // 步骤 6: 配置 MRAM 布局
    // ========================================================================
    info!("--- 步骤 6: 配置 Message RAM 布局 ---");

    // 标准 ID 过滤器: 从 MRAM_BASE + MRAM_SID_FILTER_OFFSET 开始, 8 个元素
    let sidfc = ((MRAM_SID_FILTER_OFFSET as u32) << 2) // FLSSA (word-aligned address)
              | (MRAM_SID_FILTER_NUM as u32) << 16; // LSS (element count)
    spi_write_word(&spi, &cs, MCAN_SIDFC, sidfc);
    info!(
        "SIDFC: {:#010x} ({} 标准 ID 过滤器)",
        sidfc, MRAM_SID_FILTER_NUM
    );

    // 扩展 ID 过滤器: 不使用
    spi_write_word(&spi, &cs, MCAN_XIDFC, 0);

    // RX FIFO 0: 从偏移处开始, 3 个元素
    let rxf0c = ((MRAM_RX_FIFO0_OFFSET as u32) << 2) | (MRAM_RX_FIFO0_NUM as u32) << 16;
    spi_write_word(&spi, &cs, MCAN_RXF0C, rxf0c);
    info!(
        "RXF0C: {:#010x} ({} 个 RX FIFO 0 元素)",
        rxf0c, MRAM_RX_FIFO0_NUM
    );

    // RX FIFO 1: 不使用
    spi_write_word(&spi, &cs, MCAN_RXF1C, 0);

    // TX Buffer: 从偏移处开始, 2 个专用 TX Buffer
    let txbc = ((MRAM_TX_BUFFER_OFFSET as u32) << 2) | (MRAM_TX_BUFFER_NUM as u32) << 24; // NDTB = 2 (dedicated TX buffers)
    spi_write_word(&spi, &cs, MCAN_TXBC, txbc);
    info!("TXBC: {:#010x} ({} 个 TX Buffer)", txbc, MRAM_TX_BUFFER_NUM);

    // RX/TX 元素大小: 8 bytes 数据场 (Classic CAN 最大)
    spi_write_word(&spi, &cs, MCAN_RXESC, 0x00000000); // 000 = 8 bytes
    spi_write_word(&spi, &cs, MCAN_TXESC, 0x00000000); // 000 = 8 bytes

    // ========================================================================
    // 步骤 7: 配置全局过滤器 — 接受所有标准帧到 RX FIFO 0
    // ========================================================================
    info!("--- 步骤 7: 配置过滤器 ---");

    // GFC: 拒绝所有不匹配的远程帧, 接受所有不匹配的标准帧到 RX FIFO 0
    // ANFS[5:4]=00 (接受到 RX FIFO 0), ANFE[3:2]=00 (接受到 RX FIFO 0)
    // RRFS[1]=1 (拒绝远程帧 标准), RRFE[0]=1 (拒绝远程帧 扩展)
    spi_write_word(&spi, &cs, MCAN_GFC, 0x00000003);
    info!("GFC: 接受所有标准/扩展帧到 RX FIFO 0, 拒绝远程帧");

    // 配置一个标准 ID 过滤器示例: 接受 ID=0x100 到 RX FIFO 0
    //
    // 标准过滤器元素格式 (32-bit):
    //   [31:30] SFT  = 0b10 (Classic filter)
    //   [29:27] SFEC = 0b001 (Store in RX FIFO 0)
    //   [26:16] SFID1 = 0x100 (匹配 ID)
    //   [15:11] reserved
    //   [10:0]  SFID2 = 0x7FF (掩码, 精确匹配)
    let sid_filter_0: u32 = (0b10 << 30)    // SFT = Classic filter
                          | (0b001 << 27)   // SFEC = Store in RX FIFO 0
                          | (0x100 << 16)   // SFID1 = 0x100
                          | 0x7FF; // SFID2 = 0x7FF (所有位都必须匹配)

    spi_write_word(&spi, &cs, MRAM_BASE + MRAM_SID_FILTER_OFFSET, sid_filter_0);
    info!("标准 ID 过滤器 0: 接受 ID=0x100");

    // ========================================================================
    // 步骤 8: 配置 CCCR — 使能 CAN FD 与位率切换 (可选)
    // ========================================================================
    info!("--- 步骤 8: 配置操作模式 ---");

    // 保持 INIT+CCE, 加上:
    // - DAR: 禁用自动重传 (调试方便)
    // - FDOE + BRSE: 使能 CAN FD (可选, 如果只需要 Classic CAN 可去掉)
    let cccr_val = CCCR_INIT | CCCR_CCE | CCCR_DAR;
    // 如需 CAN FD: let cccr_val = CCCR_INIT | CCCR_CCE | CCCR_DAR | CCCR_FDOE | CCCR_BRSE;
    spi_write_word(&spi, &cs, MCAN_CCCR, cccr_val);
    info!("CCCR: {:#010x} (Classic CAN, 禁用自动重传)", cccr_val);

    // ========================================================================
    // 步骤 9: 配置中断
    // ========================================================================
    info!("--- 步骤 9: 配置中断 ---");

    // 使能 RX FIFO 0 新消息中断 (bit 0) 和 TX 完成中断 (bit 9)
    let ie_val: u32 = (1 << 0)   // RF0NE - RX FIFO 0 New Message
                    | (1 << 9); // TCE   - Transmission Completed
    spi_write_word(&spi, &cs, MCAN_IE, ie_val);

    // 所有中断分配到 INT line 0
    spi_write_word(&spi, &cs, MCAN_ILS, 0);

    // 使能中断线 0
    spi_write_word(&spi, &cs, MCAN_ILE, 0x00000001);

    // TX Buffer 传输完成中断使能 (buffer 0 和 1)
    spi_write_word(&spi, &cs, MCAN_TXBTIE, 0x00000003);

    // 清除所有挂起的 MCAN 中断
    spi_write_word(&spi, &cs, MCAN_IR, 0xFFFFFFFF);

    // 清除设备中断
    spi_write_word(&spi, &cs, DEV_IR, 0xFFFFFFFF);
    // 使能设备中断 (MCAN 中断映射到 nINT 引脚)
    spi_write_word(&spi, &cs, DEV_IE, 0x00000002); // bit 1: MCAN INT 使能

    info!("中断已配置: RX FIFO 0 新消息 + TX 完成");

    // ========================================================================
    // 步骤 10: 退出初始化模式, 切换设备到 Normal 模式
    // ========================================================================
    info!("--- 步骤 10: 启动 CAN 通信 ---");

    mcan_exit_init(&spi, &cs, &timer);

    // 将 TCAN4550 设备切换到 Normal 模式 (使能收发器)
    spi_write_word(&spi, &cs, MODE_PINS, MODE_NORMAL);
    timer.delay_ms(5);

    let dev_status = spi_read_word(&spi, &cs, DEV_STATUS);
    info!("设备状态: {:#010x}", dev_status);

    let psr = spi_read_word(&spi, &cs, MCAN_PSR);
    info!("协议状态 (PSR): {:#010x}", psr);

    info!("TCAN4550 初始化完成! CAN 总线已就绪。");
    led.set_high(); // LED 常亮表示初始化成功

    // ========================================================================
    // 主循环: 发送 CAN 消息并轮询接收
    // ========================================================================
    info!("=== 进入主循环: 每 1 秒发送一帧, 持续检查接收 ===");

    let mut tx_count: u32 = 0;
    let mut rx_count: u32 = 0;

    loop {
        // ------------------------------------------------------------------
        // 发送 CAN 帧 (Standard ID = 0x100, 8 bytes)
        // ------------------------------------------------------------------
        //
        // TX Buffer 元素格式 (MRAM, 4 个 32-bit 字):
        //   Word 0 (T0): [28:18] ID (standard, 左对齐到 bit28:18)
        //                [31] ESI, [30] XTD, [29] RTR
        //   Word 1 (T1): [19:16] DLC (数据长度码)
        //                [23] FDF, [22] BRS, [21] EFC, [20] reserved
        //                [7:0] MM (Message Marker)
        //   Word 2-3:    数据字节 (大端序)
        //
        let can_id: u32 = 0x100;
        let data: [u8; 8] = [
            0xCA,
            0xFE,
            (tx_count >> 24) as u8,
            (tx_count >> 16) as u8,
            (tx_count >> 8) as u8,
            (tx_count & 0xFF) as u8,
            0xBE,
            0xEF,
        ];

        // T0: Standard ID, 非扩展帧, 非远程帧
        let t0: u32 = (can_id & 0x7FF) << 18; // ID 在 bit[28:18]
        // T1: DLC=8, 非 FD 帧
        let t1: u32 = 8 << 16; // DLC = 8

        // 数据字 (大端序打包)
        let d0: u32 = ((data[0] as u32) << 24)
            | ((data[1] as u32) << 16)
            | ((data[2] as u32) << 8)
            | (data[3] as u32);
        let d1: u32 = ((data[4] as u32) << 24)
            | ((data[5] as u32) << 16)
            | ((data[6] as u32) << 8)
            | (data[7] as u32);

        // 写入 TX Buffer 0
        let tx_buf_addr = MRAM_BASE + MRAM_TX_BUFFER_OFFSET;
        spi_write_words(&spi, &cs, tx_buf_addr, &[t0, t1, d0, d1]);

        // 请求发送 TX Buffer 0
        spi_write_word(&spi, &cs, MCAN_TXBAR, 0x00000001);

        info!(
            "[TX #{}] ID=0x{:03x} DLC=8 Data=[{:#04x},{:#04x},{:#04x},{:#04x},{:#04x},{:#04x},{:#04x},{:#04x}]",
            tx_count,
            can_id,
            data[0],
            data[1],
            data[2],
            data[3],
            data[4],
            data[5],
            data[6],
            data[7]
        );
        tx_count = tx_count.wrapping_add(1);

        // 等待发送完成
        for _ in 0..100 {
            let ir = spi_read_word(&spi, &cs, MCAN_IR);
            if ir & (1 << 9) != 0 {
                // 清除 TC 中断标志
                spi_write_word(&spi, &cs, MCAN_IR, 1 << 9);
                info!("  TX 完成");
                break;
            }
            timer.delay_ms(1);
        }

        // ------------------------------------------------------------------
        // 轮询 RX FIFO 0 接收
        // ------------------------------------------------------------------
        loop {
            let rxf0s = spi_read_word(&spi, &cs, MCAN_RXF0S);
            let fill_level = rxf0s & 0x7F; // bit[6:0] = Fill Level

            if fill_level == 0 {
                break; // 无新消息
            }

            let get_index = (rxf0s >> 8) & 0x3F; // bit[13:8] = Get Index

            // 计算 RX 元素在 MRAM 中的地址
            // 每个元素 16 bytes = 4 words
            let rx_elem_addr = MRAM_BASE + MRAM_RX_FIFO0_OFFSET + (get_index as u16) * 16;

            let mut rx_words = [0u32; 4];
            spi_read_words(&spi, &cs, rx_elem_addr, &mut rx_words);

            // 解析 RX 帧
            let r0 = rx_words[0];
            let r1 = rx_words[1];

            let xtd = (r0 >> 30) & 1;
            let rx_id = if xtd != 0 {
                r0 & 0x1FFFFFFF // 扩展 ID
            } else {
                (r0 >> 18) & 0x7FF // 标准 ID
            };
            let rtr = (r0 >> 29) & 1;
            let dlc = (r1 >> 16) & 0xF;
            let _data_len = dlc_to_len(dlc as u8);

            let mut rx_data = [0u8; 8];
            // 从 word 2-3 提取数据
            rx_data[0] = (rx_words[2] >> 24) as u8;
            rx_data[1] = (rx_words[2] >> 16) as u8;
            rx_data[2] = (rx_words[2] >> 8) as u8;
            rx_data[3] = rx_words[2] as u8;
            rx_data[4] = (rx_words[3] >> 24) as u8;
            rx_data[5] = (rx_words[3] >> 16) as u8;
            rx_data[6] = (rx_words[3] >> 8) as u8;
            rx_data[7] = rx_words[3] as u8;

            info!(
                "[RX #{}] ID=0x{:03x} XTD={} RTR={} DLC={} Data=[{:#04x},{:#04x},{:#04x},{:#04x},{:#04x},{:#04x},{:#04x},{:#04x}]",
                rx_count,
                rx_id,
                xtd,
                rtr,
                dlc,
                rx_data[0],
                rx_data[1],
                rx_data[2],
                rx_data[3],
                rx_data[4],
                rx_data[5],
                rx_data[6],
                rx_data[7]
            );
            rx_count = rx_count.wrapping_add(1);

            // 确认 RX FIFO 0: 释放已读元素
            spi_write_word(&spi, &cs, MCAN_RXF0A, get_index);
        }

        // 清除 RX FIFO 0 新消息中断标志
        let ir = spi_read_word(&spi, &cs, MCAN_IR);
        if ir & 1 != 0 {
            spi_write_word(&spi, &cs, MCAN_IR, 1);
        }

        // 定期检查总线状态
        if tx_count % 10 == 0 {
            let psr = spi_read_word(&spi, &cs, MCAN_PSR);
            let lec = psr & 0x7; // Last Error Code
            let act = (psr >> 3) & 0x3; // Activity
            let ep = (psr >> 5) & 1; // Error Passive
            let ew = (psr >> 6) & 1; // Warning Status
            let bo = (psr >> 7) & 1; // Bus Off
            info!(
                "  PSR: LEC={} ACT={} EP={} EW={} BO={}",
                lec, act, ep, ew, bo
            );

            if bo != 0 {
                warn!("总线离线 (Bus Off)! 请检查 CAN 总线连接和终端电阻。");
            }
        }

        led.toggle();
        timer.delay_ms(1000);
    }
}

/// 将 DLC 码转换为数据长度 (字节数)
fn dlc_to_len(dlc: u8) -> u8 {
    match dlc {
        0..=8 => dlc,
        9 => 12,
        10 => 16,
        11 => 20,
        12 => 24,
        13 => 32,
        14 => 48,
        15 => 64,
        _ => 0,
    }
}
