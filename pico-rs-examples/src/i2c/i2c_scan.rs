//! # I2C 总线扫描器
//!
//! 基于 RP2350 (Raspberry Pi Pico 2) 的 **I2C 总线扫描器**，
//! 逐个尝试 7-bit 地址空间 (0x08~0x77) 中的所有设备地址，
//! 打印出在线设备的地址和总数。
//!
//! ---
//!
//! # 1. 功能概述
//!
//! ## 1.1 I2C 基础
//!
//! I2C (Inter-Integrated Circuit) 是一种**半双工、同步**的两线串行总线:
//!
//! ```text
//!         主机 (Pico 2)
//!         ┌──────────┐
//!    SDA ─┤          ├── 数据线 (双向)
//!    SCL ─┤          ├── 时钟线 (主机驱动)
//!         └──────────┘
//!              │ │
//!        ┌─────┘ └─────┐
//!   ┌────┴───┐   ┌────┴───┐    ┌────────┐
//!   │ 从机 1 │   │ 从机 2 │    │ 从机 N │
//!   │ (0x44) │   │ (0x68) │ …  │ (0xNN) │
//!   └────────┘   └────────┘    └────────┘
//! ```
//!
//! - **两线**: SDA (数据) + SCL (时钟)，都需要**上拉电阻** (通常 4.7kΩ)
//! - **7-bit 地址**: 每个从机有唯一地址 (0x00~0x7F)
//! - **多从机**: 同一总线可挂接多个从机，通过地址区分
//! - **主机控制时钟**: SCL 由主机产生，从机可以通过"时钟拉伸"暂停通信
//!
//! ## 1.2 I2C 地址保留范围
//!
//! | 地址范围 | 用途 |
//! |---------|------|
//! | 0x00~0x07 | 保留 (广播地址、CBUS 等) |
//! | 0x08~0x77 | 用户设备地址 (**本扫描器的搜索范围**) |
//! | 0x78~0x7F | 保留 (10-bit 地址扩展、设备 ID 等) |
//!
//! ## 1.3 扫描原理
//!
//! 对每个地址发送一个长度为 0 的写操作 (`i2c.write(addr, &[])`):
//! - **收到 ACK** → 该地址有设备在线
//! - **收到 NACK** → 该地址无响应
//! - **超时/错误** → 总线异常
//!
//! ## 1.4 常见 I2C 设备地址速查
//!
//! | 设备 | 典型地址 | 说明 |
//! |------|---------|------|
//! | SHT30 温湿度 | 0x44 / 0x45 | ADDR 引脚可选 |
//! | BMP280 气压 | 0x76 / 0x77 | SDO 引脚可选 |
//! | MPU6050 IMU | 0x68 / 0x69 | AD0 引脚可选 |
//! | SSD1306 OLED | 0x3C / 0x3D | 128×64 / 128×32 |
//! | PCA9685 PWM | 0x40~0x7F | 16 路 PWM 舵机驱动 |
//! | AT24C32 EEPROM | 0x50~0x57 | A0/A1/A2 引脚可选 |
//! | PCF8574 IO 扩展 | 0x20~0x27 | 8-bit GPIO 扩展器 |
//! | ADS1115 ADC | 0x48~0x4B | 16-bit 4 通道 ADC |
//! | DS3231 RTC | 0x68 | 实时时钟 (与 MPU6050 冲突!) |
//!
//! ---
//!
//! # 2. 硬件接线
//!
//! ## 2.1 I2C0 引脚
//!
//! | Pico GPIO | I2C 功能 | 说明 |
//! |-----------|---------|------|
//! | GP4 | SDA | 数据线 (需外部 4.7kΩ 上拉到 3.3V) |
//! | GP5 | SCL | 时钟线 (需外部 4.7kΩ 上拉到 3.3V) |
//!
//! ```text
//!   3.3V          3.3V
//!    │              │
//!   [4.7kΩ]       [4.7kΩ]
//!    │              │
//!    ├──── SDA ─────┤──── 从设备 SDA
//!    │              │
//!    │    ┌─── SCL ─┤──── 从设备 SCL
//!    │    │         │
//!   GP4  GP5       GND ──── 从设备 GND
//!   (SDA)(SCL)
//! ```
//!
//! > **上拉电阻**: I2C 是开漏总线，必须有上拉电阻。
//! > 许多 I2C 模块 (如常见的 SHT30/OLED 模块) 自带上拉电阻，
//! > 但如果总线上模块较多或走线较长，可能需要外加上拉。
//! > RP2350 GPIO 也有内置弱上拉 (~50kΩ)，短距离简单测试可用。
//!
//! ## 2.2 I2C0 可选引脚
//!
//! | 功能 | 可选 GPIO |
//! |------|-----------|
//! | SDA | GP0, GP4, GP8, GP12, GP16, GP20, GP24, GP28 |
//! | SCL | GP1, GP5, GP9, GP13, GP17, GP21, GP25, GP29 |
//!
//! ## 2.3 I2C1 可选引脚
//!
//! | 功能 | 可选 GPIO |
//! |------|-----------|
//! | SDA | GP2, GP6, GP10, GP14, GP18, GP22, GP26 |
//! | SCL | GP3, GP7, GP11, GP15, GP19, GP23, GP27 |
//!
//! ---
//!
//! # 3. 配置参数
//!
//! | 参数 | 默认值 | 说明 |
//! |------|--------|------|
//! | `I2C_BAUDRATE` | 100_000 | I2C 时钟频率 (Hz), 标准模式 100kHz |
//! | 扫描范围 | 0x08~0x77 | 有效 7-bit 从机地址范围 |
//!
//! ## 3.1 I2C 速度等级
//!
//! | 速度模式 | 频率 | 说明 |
//! |---------|------|------|
//! | Standard | 100 kHz | 兼容所有 I2C 设备 (**推荐扫描用**) |
//! | Fast | 400 kHz | 大多数现代设备支持 |
//! | Fast+ | 1 MHz | RP2350 HAL 支持，但部分设备不兼容 |
//!
//! > **扫描建议**: 使用 100 kHz 标准模式扫描，确保兼容所有设备。
//! > 发现设备后再针对性提速。
//!
//! ---
//!
//! # 4. 输出示例
//!
//! ```text
//! I2C Bus Scan Results:
//!      0  1  2  3  4  5  6  7  8  9  A  B  C  D  E  F
//! 00: -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- --
//! 10: -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- --
//! 20: -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- --
//! 30: -- -- -- -- -- -- -- -- -- -- -- -- 3C -- -- --
//! 40: -- -- -- -- 44 -- -- -- -- -- -- -- -- -- -- --
//! 50: -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- --
//! 60: -- -- -- -- -- -- -- -- 68 -- -- -- -- -- -- --
//! 70: -- -- -- -- -- -- 76 --
//!
//! Found 4 device(s): 0x3C 0x44 0x68 0x76
//! ```
//!
//! ---
//!
//! # 5. 编译运行
//!
//! ```bash
//! cargo run --bin i2c_scan --target thumbv8m.main-none-eabihf
//! ```

#![no_std]
#![no_main]

use defmt::*;
use defmt_rtt as _;

use pico_rs::block::ImageDef;
use pico_rs::clocks::ClockBuilder;
use pico_rs::entry;
use pico_rs::gpio::Pins;
use pico_rs::i2c::I2c0;
use pico_rs::pac::Peripherals;
use pico_rs::timer::Timer0;

#[unsafe(link_section = ".start_block")]
#[used]
pub static IMAGE_DEF: ImageDef = ImageDef::exe();

// ============================================================================
// 配置参数
// ============================================================================

/// I2C 时钟频率 — 使用标准模式 100kHz 确保最大兼容性
const I2C_BAUDRATE: u32 = 100_000;

/// 扫描起始地址 — 跳过保留地址 0x00~0x07
const SCAN_ADDR_START: u8 = 0x08;

/// 扫描结束地址 — 跳过保留地址 0x78~0x7F
const SCAN_ADDR_END: u8 = 0x77;

/// 最大可记录的设备数
const MAX_DEVICES: usize = 16;

#[entry]
fn main() -> ! {
    info!("=== I2C Bus Scanner ===");

    // ------------------------------------------------------------------
    // 1. 外设初始化
    // ------------------------------------------------------------------
    let dp = Peripherals::take().unwrap();

    let clocks = ClockBuilder::new().build(dp.CLOCKS, dp.XOSC, dp.PLL_SYS, dp.PLL_USB);
    info!("System clock: {} Hz", clocks.sys_freq());

    let pins = Pins::new(dp.IO_BANK0, dp.PADS_BANK0);
    let timer = Timer0::init(dp.TIMER0);

    // ------------------------------------------------------------------
    // 2. I2C0 初始化
    //    GP4 = SDA, GP5 = SCL
    //    Pico-rs I2C HAL 会自动配置引脚的上拉和 I2C 功能
    // ------------------------------------------------------------------
    let i2c = I2c0::new(
        dp.I2C0,
        pins.pin::<4>(), // SDA
        pins.pin::<5>(), // SCL
        I2C_BAUDRATE,
        &clocks,
    );

    info!(
        "I2C0: {} Hz (GP4=SDA, GP5=SCL)",
        I2C_BAUDRATE
    );
    info!("Scanning address range: {:#04x} ~ {:#04x}", SCAN_ADDR_START, SCAN_ADDR_END);

    // ======================================================================
    // 3. I2C 总线扫描
    //
    // 对每个 7-bit 地址尝试发送一个空写操作:
    //   - write(addr, &[]) 成功 → 设备在线 (ACK)
    //   - write(addr, &[]) 失败 → 无设备 (NACK/超时)
    // ======================================================================
    info!("");
    info!("I2C Bus Scan Results:");
    info!("     0  1  2  3  4  5  6  7  8  9  A  B  C  D  E  F");

    let mut found_addrs = [0u8; MAX_DEVICES];
    let mut found_count: usize = 0;

    // 按行扫描 (每行 16 个地址)
    let mut addr = 0u8;
    while addr < 0x80 {
        // 行前缀: "00: ", "10: ", "20: ", ...
        let row = addr >> 4;
        // 使用 defmt 打印行头 — 因为 defmt 不支持动态格式，我们用 match
        match row {
            0 => info!("00: -- -- -- -- -- -- -- -- {}", scan_row(&i2c, 0x08, 0x0F, &mut found_addrs, &mut found_count)),
            1 => info!("10: {}", scan_row(&i2c, 0x10, 0x1F, &mut found_addrs, &mut found_count)),
            2 => info!("20: {}", scan_row(&i2c, 0x20, 0x2F, &mut found_addrs, &mut found_count)),
            3 => info!("30: {}", scan_row(&i2c, 0x30, 0x3F, &mut found_addrs, &mut found_count)),
            4 => info!("40: {}", scan_row(&i2c, 0x40, 0x4F, &mut found_addrs, &mut found_count)),
            5 => info!("50: {}", scan_row(&i2c, 0x50, 0x5F, &mut found_addrs, &mut found_count)),
            6 => info!("60: {}", scan_row(&i2c, 0x60, 0x6F, &mut found_addrs, &mut found_count)),
            7 => info!("70: {}", scan_row(&i2c, 0x70, 0x77, &mut found_addrs, &mut found_count)),
            _ => {}
        }

        addr += 16;
    }

    // ======================================================================
    // 4. 打印扫描结果汇总
    // ======================================================================
    info!("");
    if found_count == 0 {
        info!("No devices found!");
        info!("");
        info!("Troubleshooting:");
        info!("  1. Check SDA/SCL wiring (GP4=SDA, GP5=SCL)");
        info!("  2. Verify pull-up resistors (4.7k to 3.3V)");
        info!("  3. Check device power supply");
        info!("  4. Verify device I2C address (check datasheet)");
    } else {
        info!("Found {} device(s):", found_count);
        for i in 0..found_count {
            let addr = found_addrs[i];
            let device_name = identify_device(addr);
            info!("  {:#04x} — {}", addr, device_name);
        }
    }

    // ======================================================================
    // 5. 无限循环: 定期重新扫描
    // ======================================================================
    info!("");
    info!("=== Re-scanning every 5 seconds ===");

    loop {
        timer.delay_ms(5000);

        info!("--- Re-scan ---");
        let mut count: u32 = 0;

        let mut addr = SCAN_ADDR_START;
        while addr <= SCAN_ADDR_END {
            if i2c.write(addr, &[]).is_ok() {
                count += 1;
                info!("  Device found: {:#04x}", addr);
            }
            addr += 1;
        }

        info!("  Total: {} device(s)", count);
    }
}

// ============================================================================
// 辅助函数
// ============================================================================

/// 扫描 I2C 地址行内的一段范围，返回该行的格式化结果字符串索引
///
/// 将发现的设备地址记录到 `found_addrs`，并更新 `found_count`。
fn scan_row<SDA, SCL>(
    i2c: &I2c0<SDA, SCL>,
    start: u8,
    end: u8,
    found_addrs: &mut [u8],
    found_count: &mut usize,
) -> ScanRowResult
where
    SDA: pico_rs::gpio::I2c0SdaPin,
    SCL: pico_rs::gpio::I2c0SclPin,
{
    let mut result = ScanRowResult::new();

    let mut addr = start;
    while addr <= end {
        if addr < SCAN_ADDR_START || addr > SCAN_ADDR_END {
            result.set(addr & 0x0F, ScanEntry::Reserved);
        } else if i2c.write(addr, &[]).is_ok() {
            result.set(addr & 0x0F, ScanEntry::Found(addr));
            if *found_count < found_addrs.len() {
                found_addrs[*found_count] = addr;
                *found_count += 1;
            }
        } else {
            result.set(addr & 0x0F, ScanEntry::Empty);
        }
        addr += 1;
    }

    result
}

/// 扫描结果条目
#[derive(Clone, Copy)]
enum ScanEntry {
    Empty,
    Found(u8),
    Reserved,
}

/// 一行扫描结果 (最多 16 列)
struct ScanRowResult {
    entries: [ScanEntry; 16],
    count: u8,
}

impl ScanRowResult {
    fn new() -> Self {
        Self {
            entries: [ScanEntry::Reserved; 16],
            count: 0,
        }
    }

    fn set(&mut self, col: u8, entry: ScanEntry) {
        if (col as usize) < 16 {
            self.entries[col as usize] = entry;
            if col >= self.count {
                self.count = col + 1;
            }
        }
    }
}

impl defmt::Format for ScanRowResult {
    fn format(&self, fmt: defmt::Formatter) {
        for i in 0..self.count as usize {
            if i > 0 {
                defmt::write!(fmt, " ");
            }
            match self.entries[i] {
                ScanEntry::Empty => defmt::write!(fmt, "--"),
                ScanEntry::Found(addr) => defmt::write!(fmt, "{:02x}", addr),
                ScanEntry::Reserved => defmt::write!(fmt, "--"),
            }
        }
    }
}

/// 根据 I2C 地址猜测设备类型 (常见设备速查)
fn identify_device(addr: u8) -> &'static str {
    match addr {
        0x20..=0x27 => "PCF8574 / MCP23008 (IO Expander)",
        0x3C | 0x3D => "SSD1306 / SH1106 (OLED Display)",
        0x38..=0x3B | 0x3E | 0x3F => "PCF8574A (IO Expander)",
        0x40 => "PCA9685 (PWM) / INA219 (Current Sensor) / HDC1080",
        0x44 | 0x45 => "SHT30/SHT31/SHT35 (Temp/Humidity)",
        0x48..=0x4B => "ADS1115/ADS1015 (ADC) / TMP102 (Temp)",
        0x50..=0x57 => "AT24C32/AT24C256 (EEPROM)",
        0x60 => "MCP4725 (DAC) / ATECC508A (Crypto)",
        0x68 => "MPU6050/MPU9250 (IMU) / DS1307/DS3231 (RTC)",
        0x69 => "MPU6050/MPU9250 (IMU, alt addr)",
        0x76 => "BMP280/BME280/BME680 (Pressure/Env)",
        0x77 => "BMP280/BME280 (alt addr) / BMP180",
        _ => "Unknown device",
    }
}
