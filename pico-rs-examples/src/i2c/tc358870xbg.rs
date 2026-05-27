//! # TC358870XBG — HDMI 转 MIPI DSI 桥接芯片驱动示例
//!
//! 本示例演示如何使用 RP2350 (Raspberry Pi Pico 2) 通过 I2C 控制 Toshiba TC358870XBG
//! HDMI-to-MIPI DSI 桥接芯片，将 HDMI 输入视频信号转换并驱动一块
//! **TA039SXV01-V1868A** MIPI 4-lane 显示屏 (1080×1240)。
//!
//! ---
//!
//! # 1. 系统架构
//!
//! ```text
//!  ┌──────────┐    HDMI    ┌───────────────┐  MIPI DSI 4-lane ┌──────────────┐
//!  │  HDMI    │ ─────────→ │  TC358870XBG  │ ────────────────→│TA039SXV01    │
//!  │  信号源   │  TMDS×3    │  (桥接芯片)    │  Data×4 + CLK    │ 1080×1240    │
//!  └──────────┘  + CLK     └──────┬────────┘                  └──────────────┘
//!                                  │
//!                             I2C (SDA/SCL)
//!                                  │
//!                          ┌───────┴──────┐
//!                          │   RP2350     │
//!                          │  (Pico 2)    │
//!                          └──────────────┘
//! ```
//!
//! # 2. TC358870XBG 芯片概述
//!
//! ## 2.1 功能特性
//!
//! TC358870XBG 是 Toshiba (现 Nuvoton) 生产的 HDMI 1.4b 转 MIPI DSI 桥接芯片:
//!
//! | 特性 | 说明 |
//! |------|------|
//! | HDMI 输入 | HDMI 1.4b, 最高 1080p@60Hz |
//! | MIPI DSI 输出 | 最多 4 数据通道 (lane), 每通道最高 1 Gbps |
//! | 色彩深度 | 24-bit RGB888 |
//! | I2C 控制 | 7-bit 地址 0x0F (默认) |
//! | 内置 HDCP | 支持 HDCP 1.4 |
//! | 封装 | BGA |
//!
//! ## 2.2 I2C 地址
//!
//! TC358870XBG 默认 I2C 地址为 **0x0F** (7-bit)。
//! 注意：该芯片使用 **16-bit 寄存器地址**，分多个地址空间 (页)。
//!
//! ## 2.3 寄存器访问方式
//!
//! TC358870XBG 使用 16-bit 寄存器地址，数据宽度可为 8-bit、16-bit 或 32-bit。
//! I2C 写操作格式:
//!
//! ```text
//! [START] [Slave Addr + W] [Reg Addr High] [Reg Addr Low] [Data0] [Data1]... [STOP]
//! ```
//!
//! I2C 读操作格式:
//!
//! ```text
//! [START] [Slave Addr + W] [Reg Addr High] [Reg Addr Low]
//! [RESTART] [Slave Addr + R] [Data0] [Data1]... [STOP]
//! ```
//!
//! ---
//!
//! # 3. TC358870XBG 关键寄存器定义
//!
//! ## 3.1 系统控制寄存器 (System)
//!
//! | 地址 | 名称 | 宽度 | 说明 |
//! |------|------|------|------|
//! | 0x0000 | CHIPID | 16-bit | 芯片 ID，读回应为 0x4700 |
//! | 0x0002 | SYSCTL | 16-bit | 系统控制 (软复位等) |
//! | 0x0004 | CONFCTL0 | 16-bit | 配置控制 0 |
//! | 0x0006 | CONFCTL1 | 16-bit | 配置控制 1 |
//! | 0x0014 | FIFOCTL | 16-bit | FIFO 控制 |
//!
//! ## 3.2 HDMI 接收寄存器 (HDMI RX)
//!
//! | 地址 | 名称 | 宽度 | 说明 |
//! |------|------|------|------|
//! | 0x8520 | HDMI_INT0 | 8-bit | HDMI 中断状态 0 |
//! | 0x8521 | HDMI_INT1 | 8-bit | HDMI 中断状态 1 |
//! | 0x852F | SYS_STATUS | 8-bit | 系统状态 (bit0=HDMI信号检测) |
//! | 0x8540 | HPD_CTL | 8-bit | HPD (Hot Plug Detect) 控制 |
//! | 0x854A | INIT_END | 8-bit | 初始化结束标志 |
//! | 0x8543 | DDC_CTL | 8-bit | DDC 通道控制 |
//! | 0x8544 | HPD_CTL2 | 8-bit | HPD 控制 2 (DDC 5V 控制) |
//! | 0x8573 | PHY_CTL0 | 8-bit | HDMI PHY 控制 |
//! | 0x8600 | EDID_LEN | 16-bit | EDID 数据长度 |
//! | 0x8C00 | EDID_RAM | 256B | EDID 数据 RAM |
//!
//! ## 3.3 视频检测寄存器
//!
//! | 地址 | 名称 | 宽度 | 说明 |
//! |------|------|------|------|
//! | 0x8582 | DE_HSIZE | 16-bit | 检测到的水平有效像素数 |
//! | 0x858C | DE_VSIZE | 16-bit | 检测到的垂直有效行数 |
//!
//! ## 3.4 MIPI DSI TX 寄存器
//!
//! | 地址 | 名称 | 宽度 | 说明 |
//! |------|------|------|------|
//! | 0x0108 | DSI_TX_CLKEN | 32-bit | DSI TX 时钟使能 |
//! | 0x010C | PPI_CLKCTRL | 32-bit | PPI 时钟控制 |
//! | 0x0110 | PPI_LANEENABLE | 32-bit | PPI 通道使能 |
//! | 0x0114 | DSI_LANEENABLE | 32-bit | DSI 通道使能 |
//! | 0x0118 | PPI_STARTCNTRL | 32-bit | PPI 启动控制 |
//! | 0x0120 | DSI_STARTCNTRL | 32-bit | DSI 启动控制 |
//! | 0x0204 | LINEINITCNT | 32-bit | PHY 行初始化计数 |
//! | 0x0210 | LPTXTIMECNT | 32-bit | LP-TX 时间计数 |
//! | 0x0214 | TCLK_HEADERCNT | 32-bit | TCLK-PREPARE + TCLK-ZERO 计数 |
//! | 0x0218 | TCLK_TRAILCNT | 32-bit | TCLK-TRAIL 计数 |
//! | 0x021C | THS_HEADERCNT | 32-bit | THS-PREPARE + THS-ZERO 计数 |
//! | 0x0220 | TWAKEUP | 32-bit | TWAKEUP 时间计数 |
//! | 0x0224 | TCLK_POSTCNT | 32-bit | TCLK-POST 计数 |
//! | 0x0228 | THS_TRAILCNT | 32-bit | THS-TRAIL 计数 |
//! | 0x022C | HSTXVREGCNT | 32-bit | HS-TX 电压调节器计数 |
//! | 0x0230 | HSTXVREGEN | 32-bit | HS-TX 电压调节使能 |
//! | 0x0234 | BTA_COUNT | 32-bit | BTA 周期计数 |
//! | 0x0238 | DPHY_TX_ADJUST | 32-bit | D-PHY TX 调整 |
//! | 0x0518 | DSI_HSYNC_STOP_COUNT | 32-bit | DSI HSYNC 停止计数 |
//! | 0x0500 | DSITX_DT | 32-bit | DSI TX 数据类型 |
//! | 0x0504 | DSITX_PIX_FORMAT | 32-bit | DSI TX 像素格式 |
//! | 0x0510 | DSI_HACT | 32-bit | 水平有效像素 (按字节) |
//! | 0x0514 | DSI_VACT | 32-bit | 垂直有效行数 |
//! | 0x0518 | DSI_HSA | 32-bit | 水平同步脉冲宽度 |
//! | 0x051C | DSI_HBP | 32-bit | 水平后肩 |
//! | 0x0524 | DSI_VSA | 32-bit | 垂直同步脉冲宽度 |
//! | 0x0528 | DSI_VBP | 32-bit | 垂直后肩 |
//! | 0x052C | DSI_VFP | 32-bit | 垂直前肩 |
//!
//! ## 3.5 DSI 命令模式 (DCS 命令发送)
//!
//! | 地址 | 名称 | 宽度 | 说明 |
//! |------|------|------|------|
//! | 0x0600 | DCSCMD_TYPE | 16-bit | DCS 命令类型 |
//! | 0x0602 | DCSCMD_WC | 16-bit | DCS 命令字数 |
//! | 0x0610 | DCSCMD_DATA0 | 32-bit | DCS 命令数据 [3:0] |
//! | 0x0614 | DCSCMD_DATA1 | 32-bit | DCS 命令数据 [7:4] |
//! | ...... | ............. | ...... | 更多数据寄存器 |
//! | 0x0630 | DCSCMD_DATA8 | 32-bit | DCS 命令数据 [35:32] |
//! | 0x0600 | DCSCMD_CTRL | 16-bit | DCS 命令控制 (写0x0001发送) |
//!
//! ---
//!
//! # 4. TA039SXV01-V1868A 显示屏参数
//!
//! | 参数 | 值 |
//! |------|------|
//! | 分辨率 | 1080 × 1240 |
//! | 接口 | MIPI DSI 4-lane |
//! | 色彩 | RGB888 (24-bit) |
//! | 驱动 IC | 内置 (需通过 DCS 命令初始化) |
//!
//! ---
//!
//! # 5. 厂家初始化序列 (TA039SXV01-V1868A_MIPI 4lane_Init_code.txt) 解析
//!
//! 厂家提供的初始化代码是一系列 DCS (Display Command Set) 命令，
//! 通过 MIPI DSI 接口发送到显示屏的内置驱动 IC。
//!
//! ## 5.1 初始化代码格式
//!
//! 每一行的格式为: `命令字节, 参数1, 参数2, ...` (十六进制)
//! - 第一个字节是 DCS 命令号
//! - 后续字节是该命令的参数
//!
//! ## 5.2 关键命令解析
//!
//! ### 基础 DCS 标准命令
//!
//! | 命令 | 名称 | 说明 |
//! |------|------|------|
//! | 0x11 | Sleep Out | 退出休眠模式 (每次初始化必须) |
//! | 0x29 | Display On | 开启显示输出 |
//! | 0x2A | Column Address Set | 设置列地址范围 (水平: 0 ~ 1079) |
//! | 0x2B | Row Address Set | 设置行地址范围 (垂直: 0 ~ 1239) |
//! | 0x35 | Tearing Effect Line On | 开启 TE 信号 (用于帧同步) |
//! | 0x44 | Set Tear Scanline | 设置 TE 信号触发扫描行 (0x026C = 620) |
//! | 0x51 | Display Brightness | 设置亮度 (0xFF = 最大) |
//! | 0x8F | (厂商自定义) | MIPI 时序相关参数 |
//! | 0x60 | (厂商自定义) | 源极驱动控制 |
//!
//! ### 页选择命令 (0xF0)
//!
//! 显示屏 IC 使用 **分页寄存器** 机制，0xF0 命令用于切换寄存器页:
//!
//! | 命令 | 页 | 内容 |
//! |------|-----|------|
//! | F0 50 | Page 0x50 | 电源/时序配置 |
//! | F0 51 | Page 0x51 | 电源时序配置 2 |
//! | F0 52 | Page 0x52 | Gamma 校准 (伽马曲线) |
//! | F0 53 | Page 0x53 | 源极/栅极驱动配置 |
//! | F0 55 | Page 0x55 | 电源IC相关 |
//! | F0 58 | Page 0x58 | 高级电源/MIPI配置 |
//!
//! ### Page 0x50 — 电源/时序参数
//!
//! ```text
//! B0 0E 0E      → 电源控制: VGH/VGL 电压设置
//! B2 80         → VCOM 设置 (对比度相关)
//! B3 00 1E      → VGH/VGL 泵浦时序
//! B4 92 18 92   → 电源时钟分频
//! B6 00 3F 00   → 源极驱动控制
//! B7 00 00 00 00→ 时序控制
//! B8 02 62      → GIP (Gate In Panel) 控制
//! B9 11         → 电源模式
//! BB 08 08      → ESD 保护配置
//! C0 80         → 面板设置
//! C7 40         → VCOM 偏移
//! C9 55 F8 F8   → 时钟控制
//! ```
//!
//! ### Page 0x52 — Gamma 校准
//!
//! B0~B6 (正 gamma) 和 B7~BA (负 gamma) 定义了显示屏的亮度响应曲线:
//! - 每组包含 LUT (Look-Up Table) 数据点
//! - 控制从黑到白的灰度过渡平滑度
//! - 影响色彩准确性和对比度
//!
//! ### Page 0x53 — 源极/栅极驱动
//!
//! ```text
//! B0 00 00      → 源极驱动基础配置
//! B3 ...        → GIP Forward Scan 时序
//! B4 ...        → GIP Backward Scan 时序
//! B9, BA        → Gate 驱动时序 (CLK/STV)
//! BE            → GOA (Gate On Array) 配置
//! C1, C3        → GIP 信号映射 (正向/反向)
//! ```
//!
//! ### Page 0x58 — 高级电源
//!
//! ```text
//! C0 83 71 8E 00 → 电源IC参数
//! C1 ...         → 电压阶梯
//! C2 FF          → 电源使能
//! C9 ...         → Power On Sequence 时序
//! CC ...         → 高级时序补偿
//! ```
//!
//! ### 最终启动序列
//!
//! ```text
//! 11 00          → Sleep Out (退出休眠)
//! Delay 120ms    → 等待内部电源稳定 (MIPI 规范要求 ≥120ms)
//! 29 00          → Display On (开启显示)
//! ```
//!
//! ---
//!
//! # 6. 硬件接线
//!
//! ## 6.1 RP2350 ↔ TC358870XBG (I2C 控制)
//!
//! | Pico GPIO | 功能 | TC358870XBG 引脚 |
//! |-----------|------|-----------------|
//! | GP4 | I2C0 SDA | SDA |
//! | GP5 | I2C0 SCL | SCL |
//! | GP6 | RESET (输出) | RESX (低电平有效) |
//! | GP7 | INT (输入) | INT (中断输出, 低电平有效) |
//!
//! ## 6.2 TC358870XBG ↔ 显示屏
//!
//! | TC358870XBG | 显示屏 TA039SXV01 |
//! |-------------|-------------------|
//! | DSI_D0P/N | MIPI Data Lane 0 |
//! | DSI_D1P/N | MIPI Data Lane 1 |
//! | DSI_D2P/N | MIPI Data Lane 2 |
//! | DSI_D3P/N | MIPI Data Lane 3 |
//! | DSI_CKP/N | MIPI Clock Lane |
//!
//! ## 6.3 HDMI 输入
//!
//! | 信号 | TC358870XBG 引脚 |
//! |------|-----------------|
//! | HDMI_D0+/- | HDMI Data 0 差分对 |
//! | HDMI_D1+/- | HDMI Data 1 差分对 |
//! | HDMI_D2+/- | HDMI Data 2 差分对 |
//! | HDMI_CK+/- | HDMI Clock 差分对 |
//! | DDC_SDA | EDID/DDC 数据线 |
//! | DDC_SCL | EDID/DDC 时钟线 |
//! | HPD | Hot Plug Detect |
//! | CEC | Consumer Electronics Control (可选) |
//!
//! ---
//!
//! # 7. 初始化流程概述
//!
//! ```text
//! [RP2350 上电]
//!      │
//!      ▼
//! [硬件复位 TC358870XBG]  ← GP6 拉低 10ms 再拉高，等待 50ms
//!      │
//!      ▼
//! [读取芯片ID (0x0000)]   ← 确认 I2C 通信正常，应返回 0x4700
//!      │
//!      ▼
//! [软复位]                ← 写 SYSCTL 执行软复位
//!      │
//!      ▼
//! [配置 MIPI DSI TX]      ← 设置 4-lane, 时序参数
//!      │
//!      ▼
//! [发送屏幕初始化序列]     ← 通过 DSI 命令模式发送厂家 DCS 命令
//!      │
//!      ▼
//! [配置 HDMI RX]         ← 设置 EDID, HPD, 分辨率检测
//!      │
//!      ▼
//! [配置视频通路]          ← 将 HDMI RX 连接到 DSI TX
//!      │
//!      ▼
//! [启动视频传输]          ← 使能 DSI 视频模式输出
//!      │
//!      ▼
//! [主循环: 监控状态]      ← 检测 HDMI 信号、中断、错误
//! ```
//!
//! ---
//!
//! # 8. 编译运行
//!
//! ```bash
//! cargo run --bin tc358870xbg --target thumbv8m.main-none-eabihf
//! ```

#![no_std]
#![no_main]

use defmt::*;
use defmt_rtt as _;

use pico_rs::Delay;
use pico_rs::block::ImageDef;
use pico_rs::clocks::ClockBuilder;
use pico_rs::entry;
use pico_rs::gpio::Pins;
use pico_rs::i2c::{I2c0, I2cBus};
use pico_rs::pac::Peripherals;
use pico_rs::rt::peripheral::Peripherals as CorePeripherals;
use pico_rs::timer::Timer0;

#[unsafe(link_section = ".start_block")]
#[used]
pub static IMAGE_DEF: ImageDef = ImageDef::exe();

// ============================================================================
// TC358870XBG 寄存器地址定义
// ============================================================================

// --- 系统控制 (System Control) ---
/// 芯片 ID 寄存器，读回 0x4700
const REG_CHIPID: u16 = 0x0000;
/// 系统控制寄存器 (bit0: 软复位)
const REG_SYSCTL: u16 = 0x0002;
/// 配置控制 0
const REG_CONFCTL0: u16 = 0x0004;
/// 配置控制 1
const REG_CONFCTL1: u16 = 0x0006;

// --- MIPI DSI TX ---
/// DSI TX 时钟使能
const REG_DSI_TX_CLKEN: u16 = 0x0108;
/// PPI 时钟控制
const REG_PPI_CLKCTRL: u16 = 0x010C;
/// PPI 通道使能
const REG_PPI_LANEENABLE: u16 = 0x0110;
/// DSI 通道使能
const REG_DSI_LANEENABLE: u16 = 0x0114;
/// PPI 启动控制
const REG_PPI_STARTCNTRL: u16 = 0x0118;
/// DSI 启动控制
const REG_DSI_STARTCNTRL: u16 = 0x0120;
/// PHY 行初始化计数
const REG_LINEINITCNT: u16 = 0x0210;
/// LP-TX 时间计数
const REG_LPTXTIMECNT: u16 = 0x0214;
/// TCLK_HEADERCNT (TCLK-PREPARE + TCLK-ZERO)
const REG_TCLK_HEADERCNT: u16 = 0x0218;
/// TCLK_TRAILCNT
const REG_TCLK_TRAILCNT: u16 = 0x021C;
/// THS_HEADERCNT (THS-PREPARE + THS-ZERO)
const REG_THS_HEADERCNT: u16 = 0x0220;
/// TWAKEUP 时间
const REG_TWAKEUP: u16 = 0x0224;
/// TCLK_POSTCNT
const REG_TCLK_POSTCNT: u16 = 0x0228;
/// THS_TRAILCNT
const REG_THS_TRAILCNT: u16 = 0x022C;
/// HS-TX 电压调节器计数
const REG_HSTXVREGCNT: u16 = 0x0230;
/// HS-TX 电压调节使能
const REG_HSTXVREGEN: u16 = 0x0234;
/// BTA (Bus Turn Around) 计数
const REG_BTA_COUNT: u16 = 0x0238;
/// D-PHY TX 调整
const REG_DPHY_TX_ADJUST: u16 = 0x023C;

// --- DSI 视频参数 ---
/// DSI TX 数据类型 (Video Mode)
const REG_DSITX_DT: u16 = 0x0500;
/// DSI TX 像素格式
const REG_DSITX_PIX_FORMAT: u16 = 0x0504;
/// 水平有效像素 (字节数)
const REG_DSI_HACT: u16 = 0x0510;
/// 垂直有效行数
const REG_DSI_VACT: u16 = 0x0514;
/// 水平同步脉冲宽度 (HSA)
const REG_DSI_HSA: u16 = 0x0518;
/// 水平后肩 (HBP)
const REG_DSI_HBP: u16 = 0x051C;
/// 水平前肩 (HFP)
const REG_DSI_HFP: u16 = 0x0520;
/// 垂直同步脉冲宽度 (VSA)
const REG_DSI_VSA: u16 = 0x0524;
/// 垂直后肩 (VBP)
const REG_DSI_VBP: u16 = 0x0528;
/// 垂直前肩 (VFP)
const REG_DSI_VFP: u16 = 0x052C;

// --- DCS 命令发送 ---
/// DCS 命令类型控制
const REG_DCSCMD_TYPE: u16 = 0x0600;
/// DCS 命令字数 (Word Count)
const REG_DCSCMD_WC: u16 = 0x0602;
/// DCS 命令数据起始 (DATA0~DATA8: 0x0610~0x0630, 每个 32-bit)
const REG_DCSCMD_DATA0: u16 = 0x0610;
/// DCS 命令发送控制 (写 0x0001 触发发送)
const REG_DCSCMD_CTRL: u16 = 0x0600;

// --- HDMI RX ---
/// HDMI 中断状态 0
const REG_HDMI_INT0: u16 = 0x8520;
/// 系统状态 (bit0: HDMI 信号检测)
const REG_SYS_STATUS: u16 = 0x852F;
/// HPD 控制 (Hot Plug Detect)
const REG_HPD_CTL: u16 = 0x8540;
/// DDC 通道控制
const REG_DDC_CTL: u16 = 0x8543;
/// HPD 控制 2
const REG_HPD_CTL2: u16 = 0x8544;
/// 初始化结束标志
const REG_INIT_END: u16 = 0x854A;
/// HDMI PHY 控制
const REG_PHY_CTL0: u16 = 0x8573;
/// EDID 长度
const REG_EDID_LEN: u16 = 0x8600;
/// EDID RAM 起始
const REG_EDID_RAM: u16 = 0x8C00;

/// 视频检测 — 水平有效像素
const REG_DE_HSIZE: u16 = 0x8582;
/// 视频检测 — 垂直有效行数
const REG_DE_VSIZE: u16 = 0x858C;

// ============================================================================
// 显示屏参数 (TA039SXV01-V1868A, 1080×1240)
// ============================================================================

/// 水平分辨率 (像素)
const DISPLAY_HACTIVE: u16 = 1080;
/// 垂直分辨率 (行)
const DISPLAY_VACTIVE: u16 = 1240;
/// MIPI 数据通道数
const MIPI_LANES: u8 = 4;

/// 水平同步脉冲宽度 (像素时钟周期)
const HSA: u16 = 20;
/// 水平后肩 (像素时钟周期)
const HBP: u16 = 20;
/// 水平前肩 (像素时钟周期)
const HFP: u16 = 40;
/// 垂直同步脉冲宽度 (行)
const VSA: u16 = 2;
/// 垂直后肩 (行)
const VBP: u16 = 16;
/// 垂直前肩 (行)
const VFP: u16 = 20;

// ============================================================================
// TC358870XBG I2C 地址
// ============================================================================

/// TC358870XBG 默认 I2C 从机地址 (7-bit)
const TC358870_ADDR: u8 = 0x0F;

// ============================================================================
// GPIO 定义
// ============================================================================

/// TC358870XBG 硬件复位引脚 (低电平有效)
const PIN_RESET: u8 = 6;
/// TC358870XBG 中断引脚 (低电平有效, 输入)
const PIN_INT: u8 = 7;

// ============================================================================
// EDID 数据 — 1080×1240@60Hz
// ============================================================================
//
// EDID (Extended Display Identification Data) 是 HDMI 信号源用来识别显示器
// 能力的数据结构。TC358870XBG 内部有 EDID RAM，我们需要写入 EDID 告诉
// HDMI 源设备：本系统支持 1080×1240@60Hz 输入。
//
// 注意：实际 EDID 需要根据精确的时序参数计算，以下为参考模板。

/// 简化的 128 字节 EDID 块 — 描述 1080×1240@60Hz 显示能力
/// 实际使用时需根据 EDID 规范精确计算校验和
static EDID_DATA: [u8; 128] = [
    // Header (8 bytes)
    0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00,
    // Manufacturer ID (2B) + Product Code (2B) + Serial (4B)
    0x1E, 0x6D, // Manufacturer: "GM" (示例)
    0x01, 0x00, // Product code
    0x01, 0x00, 0x00, 0x00, // Serial
    // Week (1B) + Year (1B) + EDID Version (1B) + Revision (1B)
    0x01, 0x1E, // Week 1, Year 2024
    0x01, 0x03, // EDID 1.3
    // Basic Display (5 bytes)
    0x80, // Digital input
    0x00, 0x00, // H/V image size (未指定)
    0x78, // Gamma 2.2
    0x0A, // Features: RGB color, preferred timing in DTD1
    // Chromaticity (10 bytes)
    0xEE, 0x95, 0xA3, 0x54, 0x4C, 0x99, 0x26, 0x0F, 0x50, 0x54,
    // Established Timings (3 bytes) — 不声明标准时序
    0x00, 0x00, 0x00, // Standard Timings (16 bytes) — 全部未使用
    0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01,
    // Detailed Timing Descriptor 1 — 1080×1240@60Hz
    //   Pixel clock = (1080+20+20+40) × (1240+2+16+20) × 60 = ~99.5 MHz
    //   Pixel clock in EDID = 99500 kHz / 10 = 9950 → 0x26DE
    0xDE, 0x26, // Pixel clock: 99.50 MHz (little-endian ÷10 kHz)
    0x38, // H active lower 8 bits: 1080 & 0xFF = 0x38
    0x50, // H blanking lower 8 bits: (20+20+40)=80 → 0x50
    0x40, // H active upper[7:4]=0x4(1080>>8), H blank upper[3:0]=0x0
    0xD8, // V active lower 8 bits: 1240 & 0xFF = 0xD8
    0x26, // V blanking lower 8 bits: (2+16+20)=38 → 0x26
    0x40, // V active upper[7:4]=0x4(1240>>8), V blank upper[3:0]=0x0
    0x14, // H front porch lower: 20 = 0x14
    0x14, // H sync width lower: 20 = 0x14
    0x02, // V front porch lower[7:4]=2>>0=2, V sync lower[3:0]=0
    0x00, // Upper nibbles of front porch / sync
    0x00, 0x00, // Image size (mm) — 未指定
    0x00, 0x00, // Border
    0x18, // Non-interlaced, normal display, digital separate
    // Descriptor 2 — Display Name
    0x00, 0x00, 0x00, 0xFC, 0x00, b'T', b'A', b'0', b'3', b'9', b'S', b'X', b'V', b'0', b'1', b' ',
    0x0A, 0x20, // Descriptor 3 — Display Range Limits
    0x00, 0x00, 0x00, 0xFD, 0x00, 0x38, 0x4C, 0x1E, 0x51, 0x11, 0x00, 0x0A, 0x20, 0x20, 0x20, 0x20,
    0x20, 0x20, // Descriptor 4 — Unused (18 bytes)
    0x00, 0x00, 0x00, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00,
    // Extension count (1 byte) + Checksum (1 byte) = 总计 128 bytes
    0x00, // No extensions
    0x00, // Checksum (需运行时或离线计算填入)
    0x00, // Padding — 补齐到 128 字节
];

// ============================================================================
// 厂家初始化序列 — TA039SXV01-V1868A (MIPI 4-lane)
// ============================================================================
//
// 从 TA039SXV01-V1868A_MIPI 4lane_Init_code.txt 解析而来。
// 每条命令由 DCS 命令号 + 参数组成，通过 TC358870XBG 的 DSI 命令模式发送。
//
// 格式: (DCS命令, &[参数列表])
// 特殊: DELAY 标记表示需要延时

/// 显示屏初始化命令序列
/// 每个元素: (DCS 命令字节, 参数切片)
/// 一些命令需要特殊处理 (如 Sleep Out 后需 120ms 延时)
struct DcsCmd {
    cmd: u8,
    params: &'static [u8],
    delay_ms: u32,
}

static DISPLAY_INIT_SEQUENCE: &[DcsCmd] = &[
    // === 基础参数设置 ===
    // 设置列地址: 0x0000 ~ 0x0437 (0~1079)
    DcsCmd {
        cmd: 0x2A,
        params: &[0x00, 0x00, 0x04, 0x37],
        delay_ms: 0,
    },
    // 设置行地址: 0x0000 ~ 0x04D7 (0~1239)
    DcsCmd {
        cmd: 0x2B,
        params: &[0x00, 0x00, 0x04, 0xD7],
        delay_ms: 0,
    },
    // MIPI 时序参数 (厂商自定义)
    DcsCmd {
        cmd: 0x8F,
        params: &[0x04, 0x88, 0x77],
        delay_ms: 0,
    },
    // 设置亮度为最大 (0xFF)
    DcsCmd {
        cmd: 0x51,
        params: &[0xFF],
        delay_ms: 0,
    },
    // 开启 Tearing Effect 信号 (TE), 参数 00 = V-Blank only
    DcsCmd {
        cmd: 0x35,
        params: &[0x00],
        delay_ms: 0,
    },
    // 源极驱动控制
    DcsCmd {
        cmd: 0x60,
        params: &[0x80],
        delay_ms: 0,
    },
    // 设置 TE 触发扫描行: 0x026C = 620
    DcsCmd {
        cmd: 0x44,
        params: &[0x02, 0x6C],
        delay_ms: 0,
    },
    // === Page 0x50 — 电源/时序配置 ===
    DcsCmd {
        cmd: 0xF0,
        params: &[0x50],
        delay_ms: 0,
    },
    // 电源控制: VGH/VGL 设置
    DcsCmd {
        cmd: 0xB0,
        params: &[0x0E, 0x0E],
        delay_ms: 0,
    },
    // VCOM 设置 (影响对比度)
    DcsCmd {
        cmd: 0xB2,
        params: &[0x80],
        delay_ms: 0,
    },
    // VGH/VGL 泵浦时序
    DcsCmd {
        cmd: 0xB3,
        params: &[0x00, 0x1E],
        delay_ms: 0,
    },
    // 电源时钟分频
    DcsCmd {
        cmd: 0xB4,
        params: &[0x92, 0x18, 0x92],
        delay_ms: 0,
    },
    // 源极驱动
    DcsCmd {
        cmd: 0xB6,
        params: &[0x00, 0x3F, 0x00],
        delay_ms: 0,
    },
    // 时序控制
    DcsCmd {
        cmd: 0xB7,
        params: &[0x00, 0x00, 0x00, 0x00],
        delay_ms: 0,
    },
    // GIP 控制
    DcsCmd {
        cmd: 0xB8,
        params: &[0x02, 0x62],
        delay_ms: 0,
    },
    // 电源模式
    DcsCmd {
        cmd: 0xB9,
        params: &[0x11],
        delay_ms: 0,
    },
    // ESD 保护
    DcsCmd {
        cmd: 0xBB,
        params: &[0x08, 0x08],
        delay_ms: 0,
    },
    // 面板设置
    DcsCmd {
        cmd: 0xC0,
        params: &[0x80],
        delay_ms: 0,
    },
    // VCOM 偏移
    DcsCmd {
        cmd: 0xC7,
        params: &[0x40],
        delay_ms: 0,
    },
    // 时钟控制
    DcsCmd {
        cmd: 0xC9,
        params: &[0x55, 0xF8, 0xF8],
        delay_ms: 0,
    },
    // === Page 0x51 — 电源时序 2 ===
    DcsCmd {
        cmd: 0xF0,
        params: &[0x51],
        delay_ms: 0,
    },
    DcsCmd {
        cmd: 0xB0,
        params: &[0x28],
        delay_ms: 0,
    },
    DcsCmd {
        cmd: 0xB1,
        params: &[0x28],
        delay_ms: 0,
    },
    DcsCmd {
        cmd: 0xB2,
        params: &[0x1C],
        delay_ms: 0,
    },
    DcsCmd {
        cmd: 0xB3,
        params: &[0x1C],
        delay_ms: 0,
    },
    DcsCmd {
        cmd: 0xC0,
        params: &[0x2D],
        delay_ms: 0,
    },
    DcsCmd {
        cmd: 0xC1,
        params: &[0x18, 0x18],
        delay_ms: 0,
    },
    DcsCmd {
        cmd: 0xC4,
        params: &[0x28],
        delay_ms: 0,
    },
    DcsCmd {
        cmd: 0xC9,
        params: &[0x1B],
        delay_ms: 0,
    },
    DcsCmd {
        cmd: 0xD1,
        params: &[0x4A, 0x4A, 0x80, 0x4F, 0x69],
        delay_ms: 0,
    },
    DcsCmd {
        cmd: 0xD2,
        params: &[0x05],
        delay_ms: 0,
    },
    DcsCmd {
        cmd: 0xD3,
        params: &[0x11, 0x00, 0x00, 0x64],
        delay_ms: 0,
    },
    DcsCmd {
        cmd: 0xD7,
        params: &[0xA8, 0x5A, 0xAA, 0xAA],
        delay_ms: 0,
    },
    DcsCmd {
        cmd: 0xD8,
        params: &[0xA8, 0x5A, 0xAA, 0xAA],
        delay_ms: 0,
    },
    DcsCmd {
        cmd: 0xDD,
        params: &[0xAA, 0xAA, 0xAA, 0xAA],
        delay_ms: 0,
    },
    // === Page 0x52 — Gamma 校准 ===
    DcsCmd {
        cmd: 0xF0,
        params: &[0x52],
        delay_ms: 0,
    },
    // Gamma 参考电压
    DcsCmd {
        cmd: 0xB0,
        params: &[0x21, 0x66, 0x69],
        delay_ms: 0,
    },
    // 正 Gamma 曲线 (红)
    DcsCmd {
        cmd: 0xBA,
        params: &[
            0xFF, 0x47, 0x58, 0x68, 0x79, 0xFF, 0x8B, 0x94, 0x98, 0x9D, 0x0F, 0xA1, 0xA5,
        ],
        delay_ms: 0,
    },
    // 正 Gamma 曲线 (绿)
    DcsCmd {
        cmd: 0xB9,
        params: &[
            0xAA, 0x15, 0x3C, 0x60, 0x82, 0xAA, 0x93, 0xA3, 0xC4, 0xE5, 0xFF, 0x05, 0x15, 0x26,
            0x36,
        ],
        delay_ms: 0,
    },
    // 正 Gamma 曲线 (蓝)
    DcsCmd {
        cmd: 0xB8,
        params: &[
            0x00, 0x00, 0xA8, 0xC0, 0xCF, 0x55, 0x08, 0x2D, 0x45, 0x5F, 0x55, 0x8B, 0xAE, 0xCC,
            0xE7,
        ],
        delay_ms: 0,
    },
    // 负 Gamma 曲线数据组
    DcsCmd {
        cmd: 0xB7,
        params: &[
            0xFA, 0xEC, 0xFB, 0x0B, 0x1A, 0xFF, 0x29, 0x32, 0x36, 0x3A, 0x0F, 0x3E, 0x40,
        ],
        delay_ms: 0,
    },
    DcsCmd {
        cmd: 0xB6,
        params: &[
            0xA5, 0xD5, 0xFA, 0x1B, 0x3B, 0xAA, 0x4A, 0x59, 0x76, 0x94, 0xAA, 0xB1, 0xBF, 0xCF,
            0xDD,
        ],
        delay_ms: 0,
    },
    DcsCmd {
        cmd: 0xB5,
        params: &[
            0x00, 0x00, 0x9E, 0xAA, 0xB2, 0x50, 0xD4, 0xF3, 0x12, 0x2B, 0x55, 0x54, 0x75, 0x90,
            0xA9,
        ],
        delay_ms: 0,
    },
    DcsCmd {
        cmd: 0xB4,
        params: &[
            0xFF, 0x1B, 0x2C, 0x3E, 0x4F, 0xFF, 0x60, 0x6A, 0x6F, 0x74, 0x0F, 0x78, 0x7B,
        ],
        delay_ms: 0,
    },
    DcsCmd {
        cmd: 0xB3,
        params: &[
            0xA9, 0xEF, 0x15, 0x38, 0x5A, 0xAA, 0x6A, 0x7A, 0x9A, 0xB9, 0xEA, 0xD9, 0xE9, 0xFA,
            0x0A,
        ],
        delay_ms: 0,
    },
    DcsCmd {
        cmd: 0xB2,
        params: &[
            0x00, 0x00, 0x81, 0x9E, 0xA3, 0x54, 0xE3, 0x11, 0x2B, 0x44, 0x55, 0x6D, 0x8D, 0xAA,
            0xC3,
        ],
        delay_ms: 0,
    },
    // === Page 0x53 — 源极/栅极驱动 ===
    DcsCmd {
        cmd: 0xF0,
        params: &[0x53],
        delay_ms: 0,
    },
    // 源极驱动基础配置
    DcsCmd {
        cmd: 0xB0,
        params: &[0x00, 0x00],
        delay_ms: 0,
    },
    // GIP Forward Scan 时序
    DcsCmd {
        cmd: 0xB3,
        params: &[0xA8, 0x10, 0x3C, 0x0A, 0x03, 0x77, 0x77],
        delay_ms: 0,
    },
    // GIP Backward Scan 时序
    DcsCmd {
        cmd: 0xB4,
        params: &[0x26, 0x00, 0x00, 0x05, 0x00, 0x77, 0x77],
        delay_ms: 0,
    },
    // Gate 驱动时序 CLK/STV (正向)
    DcsCmd {
        cmd: 0xB9,
        params: &[
            0x24, 0x90, 0x00, 0x77, 0x17, 0x31, 0x22, 0x90, 0x00, 0x77, 0x17, 0x31,
        ],
        delay_ms: 0,
    },
    // Gate 驱动时序 CLK/STV (反向)
    DcsCmd {
        cmd: 0xBA,
        params: &[
            0x20, 0x90, 0x00, 0x77, 0x03, 0x10, 0x21, 0x90, 0x00, 0x77, 0x03, 0x10,
        ],
        delay_ms: 0,
    },
    // GOA 配置
    DcsCmd {
        cmd: 0xBE,
        params: &[0x20, 0x09, 0x00, 0x00, 0x2C, 0x20, 0x46, 0x00, 0x00, 0x2C],
        delay_ms: 0,
    },
    // GIP 信号映射 (正向扫描)
    DcsCmd {
        cmd: 0xC1,
        params: &[
            0x13, 0x12, 0x0B, 0x0A, 0x02, 0x09, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00,
        ],
        delay_ms: 0,
    },
    // GIP 信号映射 (反向扫描)
    DcsCmd {
        cmd: 0xC3,
        params: &[
            0x00, 0x00, 0x00, 0x00, 0x00, 0x08, 0x09, 0x03, 0x0A, 0x0B, 0x12, 0x13,
        ],
        delay_ms: 0,
    },
    // === Page 0x55 — 电源 IC ===
    DcsCmd {
        cmd: 0xF0,
        params: &[0x55],
        delay_ms: 0,
    },
    DcsCmd {
        cmd: 0xB1,
        params: &[0xD0, 0x00],
        delay_ms: 0,
    },
    DcsCmd {
        cmd: 0xC3,
        params: &[0x0F, 0x9F, 0x0F, 0x1F, 0xBF, 0x1F, 0x3F, 0xBF, 0x00],
        delay_ms: 0,
    },
    DcsCmd {
        cmd: 0xC4,
        params: &[0x00, 0x67, 0x67, 0x7F, 0x7F, 0x00, 0x00, 0xBF, 0x3F],
        delay_ms: 0,
    },
    DcsCmd {
        cmd: 0xC5,
        params: &[0x00, 0x3F, 0xBF, 0x1F, 0x9F, 0x00, 0x1F, 0xBF, 0x1F],
        delay_ms: 0,
    },
    DcsCmd {
        cmd: 0xC6,
        params: &[0x00, 0xBF, 0x3F, 0x00, 0x9F, 0x1F, 0x00, 0x7F, 0x7F],
        delay_ms: 0,
    },
    // === Page 0x58 — 高级电源/MIPI ===
    DcsCmd {
        cmd: 0xF0,
        params: &[0x58],
        delay_ms: 0,
    },
    DcsCmd {
        cmd: 0xC0,
        params: &[0x83, 0x71, 0x8E, 0x00],
        delay_ms: 0,
    },
    DcsCmd {
        cmd: 0xC1,
        params: &[0xBF, 0x7F, 0x3F, 0x2F, 0x1F, 0x0F, 0x07, 0x00],
        delay_ms: 0,
    },
    DcsCmd {
        cmd: 0xC2,
        params: &[0xFF],
        delay_ms: 0,
    },
    DcsCmd {
        cmd: 0xC3,
        params: &[0x7F, 0x05],
        delay_ms: 0,
    },
    DcsCmd {
        cmd: 0xC4,
        params: &[0x09],
        delay_ms: 0,
    },
    DcsCmd {
        cmd: 0xC5,
        params: &[0x00, 0x09, 0x09, 0x09, 0x09, 0x00, 0x09, 0x09, 0x09, 0x09],
        delay_ms: 0,
    },
    DcsCmd {
        cmd: 0xC6,
        params: &[0xFF, 0xFF],
        delay_ms: 0,
    },
    DcsCmd {
        cmd: 0xC7,
        params: &[0x5A],
        delay_ms: 0,
    },
    DcsCmd {
        cmd: 0xC9,
        params: &[
            0x7F, 0x3F, 0x2F, 0x1F, 0x0F, 0x07, 0x05, 0x04, 0x03, 0x02, 0x01, 0x00,
        ],
        delay_ms: 0,
    },
    DcsCmd {
        cmd: 0xCA,
        params: &[0xFF, 0x8F],
        delay_ms: 0,
    },
    DcsCmd {
        cmd: 0xCB,
        params: &[0x67, 0x44, 0x34, 0x01, 0x00, 0x00],
        delay_ms: 0,
    },
    DcsCmd {
        cmd: 0xCC,
        params: &[
            0x5A, 0xE9, 0x1F, 0xDC, 0x8B, 0x01, 0x1B, 0xC8, 0xAC, 0x9C, 0x00, 0x89, 0x70, 0x52,
            0x00,
        ],
        delay_ms: 0,
    },
    // === 最终启动 ===
    // Sleep Out — 退出休眠模式
    DcsCmd {
        cmd: 0x11,
        params: &[0x00],
        delay_ms: 120,
    },
    // Display On — 开启显示
    DcsCmd {
        cmd: 0x29,
        params: &[0x00],
        delay_ms: 20,
    },
];

// ============================================================================
// I2C 寄存器读写辅助函数
// ============================================================================

/// 向 TC358870XBG 写入 8-bit 寄存器
///
/// I2C 格式: [reg_addr_hi, reg_addr_lo, data]
fn tc_write_reg8(i2c: &impl I2cBus, reg: u16, val: u8) {
    let buf = [
        (reg >> 8) as u8,   // 寄存器地址高字节
        (reg & 0xFF) as u8, // 寄存器地址低字节
        val,
    ];
    if i2c.write(TC358870_ADDR, &buf).is_err() {
        error!("I2C write8 failed: reg={:#06X} val={:#04X}", reg, val);
    }
}

/// 向 TC358870XBG 写入 16-bit 寄存器 (小端序)
///
/// I2C 格式: [reg_addr_hi, reg_addr_lo, data_lo, data_hi]
fn tc_write_reg16(i2c: &impl I2cBus, reg: u16, val: u16) {
    let buf = [
        (reg >> 8) as u8,
        (reg & 0xFF) as u8,
        (val & 0xFF) as u8,        // 数据低字节
        ((val >> 8) & 0xFF) as u8, // 数据高字节
    ];
    if i2c.write(TC358870_ADDR, &buf).is_err() {
        error!("I2C write16 failed: reg={:#06X} val={:#06X}", reg, val);
    }
}

/// 向 TC358870XBG 写入 32-bit 寄存器 (小端序)
///
/// I2C 格式: [reg_addr_hi, reg_addr_lo, d0, d1, d2, d3]
fn tc_write_reg32(i2c: &impl I2cBus, reg: u16, val: u32) {
    let buf = [
        (reg >> 8) as u8,
        (reg & 0xFF) as u8,
        (val & 0xFF) as u8,
        ((val >> 8) & 0xFF) as u8,
        ((val >> 16) & 0xFF) as u8,
        ((val >> 24) & 0xFF) as u8,
    ];
    if i2c.write(TC358870_ADDR, &buf).is_err() {
        error!("I2C write32 failed: reg={:#06X} val={:#010X}", reg, val);
    }
}

/// 从 TC358870XBG 读取 8-bit 寄存器
///
/// I2C: 先写寄存器地址，再重复起始读 1 字节
fn tc_read_reg8(i2c: &impl I2cBus, reg: u16) -> u8 {
    let addr_buf = [(reg >> 8) as u8, (reg & 0xFF) as u8];
    let mut data = [0u8; 1];
    if i2c.write_read(TC358870_ADDR, &addr_buf, &mut data).is_err() {
        error!("I2C read8 failed: reg={:#06X}", reg);
        return 0;
    }
    data[0]
}

/// 从 TC358870XBG 读取 16-bit 寄存器 (小端序)
fn tc_read_reg16(i2c: &impl I2cBus, reg: u16) -> u16 {
    let addr_buf = [(reg >> 8) as u8, (reg & 0xFF) as u8];
    let mut data = [0u8; 2];
    if i2c.write_read(TC358870_ADDR, &addr_buf, &mut data).is_err() {
        error!("I2C read16 failed: reg={:#06X}", reg);
        return 0;
    }
    u16::from_le_bytes(data)
}

// ============================================================================
// EDID 校验和计算
// ============================================================================

/// 计算 EDID 校验和 — 128 字节块的所有字节之和为 0 (mod 256)
fn edid_checksum(edid: &[u8; 128]) -> u8 {
    let sum: u8 = edid[..127].iter().fold(0u8, |acc, &b| acc.wrapping_add(b));
    0u8.wrapping_sub(sum)
}

// ============================================================================
// 通过 TC358870XBG 发送 DCS 命令到屏幕
// ============================================================================

/// 通过 TC358870XBG DSI 命令模式接口发送一条 DCS 命令到屏幕
///
/// TC358870XBG 有专用的 DCS 命令发送通路:
///   1. 将 DCS payload 写入 DCSCMD_DATA 寄存器 (0x0610~0x0630)
///   2. 设置命令类型 (短命令/长命令)
///   3. 设置字数 (Word Count)
///   4. 触发发送
///
/// # DCS 命令分类
///
/// - **Short Write (0 参数)**: DT=0x05, WC=1 (仅命令字节)
/// - **Short Write (1 参数)**: DT=0x15, WC=2 (命令+1参数)
/// - **Long Write (≥2 参数)**: DT=0x39, WC=1+N (命令+参数)
fn send_dcs_command(i2c: &impl I2cBus, cmd: u8, params: &[u8]) {
    let total_len = 1 + params.len(); // DCS命令字节 + 参数
    let (dt, wc) = if params.is_empty() {
        // Short Write, no parameter — DCS命令本身作为唯一字节
        (0x05u16, 1u16)
    } else if params.len() == 1 {
        // Short Write, 1 parameter
        (0x15u16, 2u16)
    } else {
        // Long Write — Generic Long Write
        (0x39u16, total_len as u16)
    };

    // 组装 DCS payload: [cmd, param0, param1, ...]
    // 将 payload 写入 DATA 寄存器 (每个 DATA 寄存器 32-bit = 4 bytes)
    let mut payload = [0u8; 40]; // 最大支持 40 字节
    payload[0] = cmd;
    let copy_len = params.len().min(payload.len() - 1);
    payload[1..1 + copy_len].copy_from_slice(&params[..copy_len]);

    // 写入 DCSCMD_DATA 寄存器 (0x0610 ~ 0x0630)
    let num_data_regs = (total_len + 3) / 4; // 向上取整到 4 字节边界
    for i in 0..num_data_regs {
        let offset = i * 4;
        let val = u32::from_le_bytes([
            payload[offset],
            payload.get(offset + 1).copied().unwrap_or(0),
            payload.get(offset + 2).copied().unwrap_or(0),
            payload.get(offset + 3).copied().unwrap_or(0),
        ]);
        tc_write_reg32(i2c, REG_DCSCMD_DATA0 + (i as u16) * 4, val);
    }

    // 设置命令类型 (Data Type)
    tc_write_reg16(i2c, REG_DCSCMD_TYPE, dt);

    // 设置字数 (Word Count)
    tc_write_reg16(i2c, REG_DCSCMD_WC, wc);

    // 触发发送 — 写 0x0001 到 DCSCMD_CTRL
    tc_write_reg16(i2c, REG_DCSCMD_CTRL, 0x0001);

    // 等待命令发送完成 — 简单延时
    // 实际应用中可轮询状态寄存器
    pico_rs_rt::asm::delay(150_000 * 5); // ~5ms @ 150MHz
}

// ============================================================================
// 主函数
// ============================================================================

#[entry]
fn main() -> ! {
    info!("=== TC358870XBG HDMI-to-MIPI DSI 桥接驱动 ===");
    info!(
        "显示屏: TA039SXV01-V1868A ({}×{}, MIPI {}-lane)",
        DISPLAY_HACTIVE, DISPLAY_VACTIVE, MIPI_LANES
    );

    // ------------------------------------------------------------------
    // 1. RP2350 外设初始化
    // ------------------------------------------------------------------
    let dp = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();

    let clocks = ClockBuilder::new().build(dp.CLOCKS, dp.XOSC, dp.PLL_SYS, dp.PLL_USB);
    info!("系统时钟: {} MHz", clocks.sys_freq() / 1_000_000);

    let pins = Pins::new(dp.IO_BANK0, dp.PADS_BANK0);
    let delay = Delay::new(core.SYST, clocks.sys_freq());
    let _timer = Timer0::init(dp.TIMER0);

    // ------------------------------------------------------------------
    // 2. GPIO 初始化 — 复位引脚 & 中断引脚
    // ------------------------------------------------------------------
    let reset_binding = pins.pin::<PIN_RESET>();
    let reset_pin = reset_binding.into_output();
    let int_binding = pins.pin::<PIN_INT>();
    let _int_pin = int_binding.into_input();

    // ------------------------------------------------------------------
    // 3. 硬件复位 TC358870XBG
    //    RESX 低电平有效 → 拉低至少 10ms → 拉高 → 等待 50ms 稳定
    // ------------------------------------------------------------------
    info!("硬件复位 TC358870XBG...");
    reset_pin.set_low();
    delay.delay_ms(10);
    reset_pin.set_high();
    delay.delay_ms(50);
    info!("复位完成");

    // ------------------------------------------------------------------
    // 4. I2C0 初始化 — 100 kHz (TC358870XBG I2C 标准模式)
    //    GP4 = SDA, GP5 = SCL
    // ------------------------------------------------------------------
    let i2c = I2c0::new(
        dp.I2C0,
        pins.pin::<4>(), // SDA
        pins.pin::<5>(), // SCL
        100_000,         // 100 kHz 标准模式
        &clocks,
    );
    info!("I2C0 初始化完成 (100 kHz, GP4=SDA, GP5=SCL)");

    // ------------------------------------------------------------------
    // 5. 读取芯片 ID — 验证 I2C 通信
    //    REG_CHIPID (0x0000) 应返回 0x4700
    // ------------------------------------------------------------------
    let chip_id = tc_read_reg16(&i2c, REG_CHIPID);
    info!("TC358870XBG 芯片 ID: {:#06X}", chip_id);
    if chip_id != 0x4700 {
        error!("芯片 ID 不匹配! 期望 0x4700, 实际 {:#06X}", chip_id);
        error!("请检查: I2C 接线、上拉电阻、TC358870XBG 电源");
        loop {
            pico_rs_rt::asm::wfe();
        }
    }
    info!("芯片 ID 验证通过 ✓");

    // ------------------------------------------------------------------
    // 6. 软复位
    //    SYSCTL (0x0002) bit0 = 1 → 软件复位
    // ------------------------------------------------------------------
    info!("执行软复位...");
    tc_write_reg16(&i2c, REG_SYSCTL, 0x0001);
    delay.delay_ms(10);
    tc_write_reg16(&i2c, REG_SYSCTL, 0x0000);
    delay.delay_ms(10);

    // ==================================================================
    // 7. 配置 MIPI DSI TX — 4-lane, D-PHY 时序
    // ==================================================================
    info!("配置 MIPI DSI TX...");

    // 7.1 使能 DSI TX 时钟
    tc_write_reg32(&i2c, REG_DSI_TX_CLKEN, 0x00000001);

    // 7.2 PPI 时钟控制
    tc_write_reg32(&i2c, REG_PPI_CLKCTRL, 0x00000002);

    // 7.3 PPI 通道使能 — 4 数据通道 + 1 时钟通道 = 0x1F
    //     bit[0] = CLK lane, bit[1:4] = Data lane 0~3
    tc_write_reg32(&i2c, REG_PPI_LANEENABLE, 0x0000001F);

    // 7.4 DSI 通道使能 — 同上
    tc_write_reg32(&i2c, REG_DSI_LANEENABLE, 0x0000001F);

    // 7.5 D-PHY 时序参数
    //     这些值依赖于 DSI 时钟频率，以下为 ~500 Mbps/lane 的典型值
    //     实际需根据显示屏要求和 MIPI D-PHY 规范调整

    // PHY 初始化计数 — LP-11 状态持续时间
    tc_write_reg32(&i2c, REG_LINEINITCNT, 0x00002C88);

    // LP-TX 时间计数
    tc_write_reg32(&i2c, REG_LPTXTIMECNT, 0x00000005);

    // TCLK-PREPARE + TCLK-ZERO 计数
    tc_write_reg32(&i2c, REG_TCLK_HEADERCNT, 0x00002006);

    // TCLK-TRAIL 计数
    tc_write_reg32(&i2c, REG_TCLK_TRAILCNT, 0x00000003);

    // THS-PREPARE + THS-ZERO 计数
    tc_write_reg32(&i2c, REG_THS_HEADERCNT, 0x00000606);

    // TWAKEUP 时间 — 需 ≥ 1ms
    tc_write_reg32(&i2c, REG_TWAKEUP, 0x00004A88);

    // TCLK-POST 计数
    tc_write_reg32(&i2c, REG_TCLK_POSTCNT, 0x0000000A);

    // THS-TRAIL 计数
    tc_write_reg32(&i2c, REG_THS_TRAILCNT, 0x00000004);

    // HS-TX 电压调节器稳定时间
    tc_write_reg32(&i2c, REG_HSTXVREGCNT, 0x00000005);

    // HS-TX 电压调节使能 — 所有通道
    tc_write_reg32(&i2c, REG_HSTXVREGEN, 0x0000001F);

    // BTA 计数
    tc_write_reg32(&i2c, REG_BTA_COUNT, 0x00000008);

    // D-PHY TX 调整
    tc_write_reg32(&i2c, REG_DPHY_TX_ADJUST, 0x00000002);

    info!("DSI TX D-PHY 时序配置完成");

    // 7.6 启动 PPI 和 DSI
    tc_write_reg32(&i2c, REG_PPI_STARTCNTRL, 0x00000001);
    tc_write_reg32(&i2c, REG_DSI_STARTCNTRL, 0x00000001);

    info!("DSI TX 通道已启动");

    // ==================================================================
    // 8. 通过 DSI 命令模式发送屏幕初始化序列
    // ==================================================================
    info!(
        "发送屏幕初始化序列 ({} 条命令)...",
        DISPLAY_INIT_SEQUENCE.len()
    );

    for (idx, dcs) in DISPLAY_INIT_SEQUENCE.iter().enumerate() {
        // 发送 DCS 命令
        send_dcs_command(&i2c, dcs.cmd, dcs.params);

        // 如果需要延时 (如 Sleep Out 后 120ms)
        if dcs.delay_ms > 0 {
            info!("  CMD {:#04X}: 等待 {}ms", dcs.cmd, dcs.delay_ms);
            delay.delay_ms(dcs.delay_ms);
        }

        // 每 10 条命令打印一次进度
        if (idx + 1) % 10 == 0 {
            info!(
                "  已发送 {}/{} 条命令",
                idx + 1,
                DISPLAY_INIT_SEQUENCE.len()
            );
        }
    }
    info!("屏幕初始化序列发送完成 ✓");

    // ==================================================================
    // 9. 配置 DSI 视频模式参数 — 匹配 1080×1240 时序
    // ==================================================================
    info!("配置 DSI 视频模式参数...");

    // 9.1 像素格式 — RGB888, Packed Pixel Stream (DT = 0x3E)
    tc_write_reg32(&i2c, REG_DSITX_DT, 0x003E);

    // 9.2 像素格式寄存器 — RGB888
    tc_write_reg32(&i2c, REG_DSITX_PIX_FORMAT, 0x0003);

    // 9.3 水平有效像素 (按字节): 1080 × 3 (RGB888) = 3240 = 0x0CA8
    tc_write_reg32(&i2c, REG_DSI_HACT, (DISPLAY_HACTIVE as u32) * 3);

    // 9.4 垂直有效行数
    tc_write_reg32(&i2c, REG_DSI_VACT, DISPLAY_VACTIVE as u32);

    // 9.5 水平同步脉冲 (HSA) — 按字节: HSA_pixels × 3
    tc_write_reg32(&i2c, REG_DSI_HSA, (HSA as u32) * 3);

    // 9.6 水平后肩 (HBP) — 按字节: HBP_pixels × 3
    tc_write_reg32(&i2c, REG_DSI_HBP, (HBP as u32) * 3);

    // 9.7 水平前肩 (HFP) — 按字节: HFP_pixels × 3
    tc_write_reg32(&i2c, REG_DSI_HFP, (HFP as u32) * 3);

    // 9.8 垂直同步脉冲 (VSA)
    tc_write_reg32(&i2c, REG_DSI_VSA, VSA as u32);

    // 9.9 垂直后肩 (VBP)
    tc_write_reg32(&i2c, REG_DSI_VBP, VBP as u32);

    // 9.10 垂直前肩 (VFP)
    tc_write_reg32(&i2c, REG_DSI_VFP, VFP as u32);

    info!(
        "DSI 视频参数: {}×{}, HSA={} HBP={} HFP={} VSA={} VBP={} VFP={}",
        DISPLAY_HACTIVE, DISPLAY_VACTIVE, HSA, HBP, HFP, VSA, VBP, VFP
    );

    // ==================================================================
    // 10. 配置 HDMI 接收器
    // ==================================================================
    info!("配置 HDMI 接收器...");

    // 10.1 写入 EDID 到 TC358870XBG 内部 RAM
    //      EDID 告诉 HDMI 源我们支持 1080×1240@60Hz
    info!("写入 EDID 数据 (128 bytes)...");

    // 计算并修正校验和
    let mut edid = EDID_DATA;
    edid[127] = edid_checksum(&edid);

    // 将 EDID 写入 RAM (0x8C00 开始, 每次写 1 字节)
    for (i, &byte) in edid.iter().enumerate() {
        tc_write_reg8(&i2c, REG_EDID_RAM + i as u16, byte);
    }

    // 设置 EDID 长度
    tc_write_reg16(&i2c, REG_EDID_LEN, 128);
    info!("EDID 写入完成");

    // 10.2 配置 HPD (Hot Plug Detect)
    //      HPD 拉高告诉 HDMI 源设备: "显示器已连接"
    tc_write_reg8(&i2c, REG_HPD_CTL, 0x01); // 使能 HPD 输出
    tc_write_reg8(&i2c, REG_HPD_CTL2, 0x01); // DDC 5V 使能

    // 10.3 DDC 通道使能 (用于 EDID 传输)
    tc_write_reg8(&i2c, REG_DDC_CTL, 0x00); // 自动 DDC 应答

    // 10.4 HDMI PHY 配置
    tc_write_reg8(&i2c, REG_PHY_CTL0, 0x01); // 使能 HDMI PHY

    // 10.5 初始化结束标志
    tc_write_reg8(&i2c, REG_INIT_END, 0x01);

    info!("HDMI 接收器配置完成");

    // ==================================================================
    // 11. 配置视频通路 — HDMI RX → DSI TX
    // ==================================================================
    info!("配置视频通路...");

    // 配置控制 0 — 使能 HDMI→DSI 视频通路
    //   bit[0] = 1: 使能视频输出
    //   bit[2] = 1: HDMI 视频通路使能
    //   bit[4] = 1: DSI 视频模式 (非命令模式)
    tc_write_reg16(&i2c, REG_CONFCTL0, 0x0015);

    // 配置控制 1
    tc_write_reg16(&i2c, REG_CONFCTL1, 0x0001);

    info!("视频通路已使能: HDMI RX → DSI TX (Video Mode)");

    // ==================================================================
    // 12. 主循环 — 监控 HDMI 信号状态
    // ==================================================================
    info!("");
    info!("======================================");
    info!("  初始化完成! 等待 HDMI 输入信号...");
    info!("======================================");
    info!("");
    info!("请将 HDMI 源设备设置为 1080×1240@60Hz 输出");
    info!("(如不支持该分辨率，部分源设备可能自动选择最接近的分辨率)");
    info!("");

    let mut hdmi_connected = false;
    let mut poll_counter = 0u32;

    loop {
        // 每 ~500ms 检查一次 HDMI 状态
        delay.delay_ms(500);
        poll_counter = poll_counter.wrapping_add(1);

        // 读取系统状态 — bit0 表示 HDMI 信号是否存在
        let status = tc_read_reg8(&i2c, REG_SYS_STATUS);
        let signal_detected = (status & 0x01) != 0;

        if signal_detected && !hdmi_connected {
            // HDMI 信号刚刚接入
            hdmi_connected = true;
            info!("✓ HDMI 信号检测到!");

            // 读取检测到的分辨率
            delay.delay_ms(200); // 等待信号稳定
            let h_size = tc_read_reg16(&i2c, REG_DE_HSIZE);
            let v_size = tc_read_reg16(&i2c, REG_DE_VSIZE);
            info!("  检测到的分辨率: {}×{}", h_size, v_size);

            if h_size == DISPLAY_HACTIVE && v_size == DISPLAY_VACTIVE {
                info!("  ✓ 分辨率匹配! 视频正在输出到屏幕");
            } else {
                warn!(
                    "  ⚠ 分辨率不匹配! 期望 {}×{}, 实际 {}×{}",
                    DISPLAY_HACTIVE, DISPLAY_VACTIVE, h_size, v_size
                );
                warn!("  请调整 HDMI 源分辨率设置");
            }
        } else if !signal_detected && hdmi_connected {
            // HDMI 信号断开
            hdmi_connected = false;
            warn!("✗ HDMI 信号丢失 — 等待重新连接...");
        }

        // 读取并清除 HDMI 中断
        let int0 = tc_read_reg8(&i2c, REG_HDMI_INT0);
        if int0 != 0 {
            debug!("HDMI INT0: {:#04X}", int0);
        }

        // 每 10 秒打印一次心跳 (20 × 500ms)
        if poll_counter % 20 == 0 {
            if hdmi_connected {
                info!("[心跳] HDMI 已连接, 视频输出中...");
            } else {
                info!("[心跳] 等待 HDMI 信号... ({}s)", poll_counter / 2);
            }
        }
    }
}
