//! # ST7789 TFT 彩屏 SPI 驱动示例 (1.14 寸 135×240, 中景园 ZJY114S0800TG01)
//!
//! 基于 RP2350 (Raspberry Pi Pico 2) 的 **ST7789V** 液晶屏驱动示例，
//! 使用 **SPI 4-wire 串行接口** 点亮 **135×240** RGB 全彩屏。
//!
//! 参考资料:
//! - `ST7789VW.pdf`        — Sitronix ST7789VW 控制器手册 (寄存器/命令/时序)
//! - `ZJY114S0800TG01.pdf` — 中景园 1.14 寸 135×240 模块规格书
//!   (含本模块官方 ST7789V 初始化序列, 见文档附录 "The appendix")
//!
//! ---
//!
//! # 1. 屏幕简介
//!
//! 本模块采用 **ST7789V** 驱动 IC，为 **SPI-4wire (4 线串行)** 接口：
//!
//! ```text
//!   MCU  ───────────────────────────────►  屏幕
//!   SCL  (串行时钟)   ─────────────────►  Pin3 SCL
//!   SDA  (串行数据)   ─────────────────►  Pin4 SDA
//!   DC   (数据/命令)  ─────────────────►  Pin6 DC
//!   RES  (复位, 低有效)────────────────►  Pin5 RES
//!   CS   (片选, 低有效)────────────────►  Pin7 CS
//!   BLK  (背光, 高亮) ─────────────────►  Pin8 BLK
//! ```
//!
//! | 引脚 | 名称 | 说明 |
//! |------|------|------|
//! | Pin1 | GND  | 电源地 |
//! | Pin2 | VCC  | 逻辑电源 2.4~3.3V |
//! | Pin3 | SCL  | SPI 时钟 (输入) |
//! | Pin4 | SDA  | SPI 数据 MOSI (输入) |
//! | Pin5 | RES  | 硬件复位 (低有效) |
//! | Pin6 | DC   | 数据/命令选择: 高=数据, 低=命令 |
//! | Pin7 | CS   | 片选 (低有效) |
//! | Pin8 | BLK  | 背光控制: 高=开, 低=关 |
//!
//! ## 1.1 DC 引脚如何区分"命令"和"数据"
//!
//! ST7789 采用 **4-wire + DC 引脚** 的写协议：每个 8 bit 字节通过 SDA 送入，
//! 同时由 DC 电平决定该字节是**命令**还是**参数/像素数据**：
//!
//! ```text
//!   DC=0 (低) ────► 该字节被当作【命令】写入命令寄存器
//!   DC=1 (高) ────► 该字节被当作【参数或像素数据】
//! ```
//!
//! 注意这与"第 9 位嵌入命令/数据"的 3-wire 方式不同，本屏为 4-wire，
//! 因此 SPI 数据宽度就是普通 **8 bit**，模式为 **Mode 0**。
//!
//! ## 1.2 本示例的功能演示
//!
//! 1. **硬件复位 + 官方初始化序列** (来自模块规格书附录)
//! 2. 整屏纯色切换 (验证数据线/背光/供电)
//! 3. 彩虹横条 (验证"列窗口 + 写像素")
//! 4. 棋盘格图案 (验证逐点绘制)
//! 5. 图形绘制：边框、十字、对角线、渐变色块
//! 6. 动画演示：缩放方块、来回滑动的彩条 (离屏帧缓冲合成，无闪烁)
//!
//! ---
//!
//! # 2. 硬件接线 (Pico 2 ⇄ ZJY114S0800TG01)
//!
//! | Pico 2 GPIO | 屏幕引脚 | SPI0 功能 | 说明 |
//! |-------------|---------|-----------|------|
//! | GP2         | Pin3 SCL | SCK  | SPI 时钟 |
//! | GP3         | Pin4 SDA | TX (MOSI) | 主机数据输出 |
//! | (GP0)       | —        | RX (MISO) | 未使用 (仅 HAL 初始化需要，悬空即可) |
//! | GP4         | Pin5 RES | GPIO 输出 | 复位 (低有效) |
//! | GP5         | Pin6 DC  | GPIO 输出 | 数据/命令选择 |
//! | GP6         | Pin7 CS  | GPIO 输出 | 片选 (低有效) |
//! | GP7         | Pin8 BLK | GPIO 输出 | 背光 (高=亮) |
//! | 3V3         | Pin2 VCC | —        | 逻辑电源 |
//! | GND         | Pin1 GND | —        | 地 |
//!
//!
//! ```text
//!   Pico 2 (RP2350)                  ZJY114 1.14" ST7789
//!   ──────────────                   ─────────────────────
//!   GP2 (SPI0 SCK)  ───────────────► Pin3 SCL   (时钟)
//!   GP3 (SPI0 MOSI) ───────────────► Pin4 SDA   (数据)
//!   GP4             ───────────────► Pin5 RES   (复位)
//!   GP5             ───────────────► Pin6 DC    (命令/数据)
//!   GP6             ───────────────► Pin7 CS    (片选)
//!   GP7             ───────────────► Pin8 BLK   (背光)
//!   3V3  ──────────────────────────► Pin2 VCC
//!   GND  ──────────────────────────► Pin1 GND
//! ```
//!
//! > **背光 (BLK) 说明**: 本模块背光为 2 颗白光 LED (约 20 mA, 3.0V)。
//! > 示例通过 GP7 输出高电平开启；若发现背光偏暗，可将 Pin8 BLK
//! > 直接接到 3.3V（或串 10Ω 限流）改用恒亮方式。
//!
//! > **MISO 说明**: 该屏是只写设备、无 MISO。RP2350 的 SPI 主机初始化需要
//! > 一个 RX 引脚，本示例使用 GP0（不连接任何东西）占位即可。
//!
//! ---
//!
//! # 3. SPI 通讯要点
//!
//! | 参数 | 取值 | 说明 |
//! |------|------|------|
//! | SPI 外设 | SPI0 | 使用 SPI0 (还可换 SPI1) |
//! | 模式 | Mode 0 | CPOL=0, CPHA=0 |
//! | 数据宽度 | 8 bit | 4-wire 串行接口逐字节传输 |
//! | 时钟频率 | 20 MHz | ST7789 通常支持 20~62 MHz，建议 ≤ 40 MHz 走线较短 |
//! | 颜色格式 | RGB565 | 每像素 2 字节，16 bit/像素 (COLMOD=0x05) |
//!
//! **SPI 时序 (写命令/写数据)**：
//! ```text
//!      ┌─┐ ┌─┐ ┌─┐ ┌─┐
//!  SCL ┘ └─┘ └─┘ └─┘ └───   Mode0: 空闲低, 上升沿采样
//!      ┌───────┐
//!  DC  │       └───────────   高=数据 / 低=命令
//!      ┌───────────────┐
//!  CS  │               └───   低有效, 整段事务保持低
//!      ┌───────────────────┐
//!  SDA │  D7 D6 ... D0      │  MSB 在前
//!      └───────────────────┘
//! ```
//!
//! ---
//!
//! # 4. ST7789 常用命令 (本示例用到)
//!
//! | 命令 | 名称 | 作用 |
//! |------|------|------|
//! | `0x01` | SWRESET | 软件复位 |
//! | `0x11` | SLPOUT  | 退出睡眠 |
//! | `0x36` | MADCTL  | 数据访问方向 (旋转/镜像/RGB-BGR) |
//! | `0x3A` | COLMOD  | 像素格式 (0x05=16bit RGB565) |
//! | `0x2A` | CASET   | 列地址窗口 (X) |
//! | `0x2B` | RASET   | 行地址窗口 (Y) |
//! | `0x2C` | RAMWR   | 写入像素数据 (连续) |
//! | `0x21` | INVON   | 显示反相开 |
//! | `0x29` | DISPON  | 显示开 |
//!
//! > `CASET` / `RASET` 用于裁剪出要更新的矩形"窗口"，
//! > 之后用 `RAMWR` 连续写入该窗口的像素即可。相比全屏重绘，
//! > 更新小区域时能大幅减少 SPI 数据量。
//!
//! ---
//!
//! # 5. 编译与烧录
//!
//! 在 `rp2350-examples/` 目录下（或 workspace 根目录）执行：
//!
//! ```bash
//! # 编译 + 通过调试器烧录运行 (已配置 probe-rs runner)
//! cargo run --bin st7789
//!
//! # 只编译检查
//! cargo check --bin st7789
//! ```
//!
//! 屏幕正常点亮后应依次看到：纯色切换 → 彩虹横条 → 棋盘格 →
//! 图形绘制 → 缩放动画 → 来回滑动的彩条。

#![no_std]
#![no_main]

use defmt::*;
use defmt_rtt as _;

use rp2350::clocks::ClockBuilder;
use rp2350::delay::DelayUs;
use rp2350::gpio::{DriveStrength, Pin, Pins};
use rp2350::main;
use rp2350::pac::Peripherals;
use rp2350::spi::Spi0;

// 内嵌中文字形位图（由同目录 gen_font.py 生成，只含用到的字）
mod font;

// ============================================================================
// 硬件配置 (按你的实际接线修改这里即可)
// ============================================================================

/// SPI 时钟频率 (Hz)。ST7789 支持较高时钟，但受杜邦线/布局影响，
/// 本示例使用 20 MHz 作为稳妥起点；走线良好可提升到 40 MHz。
const SPI_BAUDRATE: u32 = 20_000_000;

/// SPI0 引脚：SCK / MOSI / RX(未用，占位)
const PIN_SCK: u32 = 2;
const PIN_MOSI: u32 = 3;
const PIN_MISO: u32 = 0; // 屏幕无 MISO，仅占位

/// 控制引脚
const PIN_DC: u32 = 5; // 数据/命令 (高=数据, 低=命令)
const PIN_RES: u32 = 4; // 复位 (低有效)
const PIN_CS: u32 = 6; // 片选 (低有效)
const PIN_BLK: u32 = 7; // 背光 (高=开)

// ============================================================================
// 屏幕几何参数 (ZJY114S0800TG01: 135(RGB) × 240)
// ============================================================================

/// 屏幕宽度 (像素)
const LCD_W: u16 = 135;
/// 屏幕高度 (像素)
const LCD_H: u16 = 240;

/// 列偏移：若画面整体左右偏移，可在此加补偿。
/// (ST7789V GRAM 为 240 列；本屏可视 135 列居中摆放，可视区起点约在
///  列 52，故 COL_OFFSET=52，使 CASET 落到 52..186 覆盖整个可视玻璃)
const COL_OFFSET: u16 = 52;
/// 行偏移：若画面整体上下偏移，可在此加补偿。
/// (GRAM 为 320 行；可视 240 行居中摆放，起点约在行 40，故 ROW_OFFSET=40)
const ROW_OFFSET: u16 = 40;

// ============================================================================
// 帧缓冲几何 (RGB565，行优先，每像素 2 字节且高字节在前，与 SPI 顺序一致)
// ============================================================================

/// 每像素字节数
const BYTES_PER_PIXEL: usize = 2;
/// 每一行像素字节数
const ROW_BYTES: usize = (LCD_W as usize) * BYTES_PER_PIXEL;
/// 整帧字节数 = 135 × 240 × 2
const FRAME_BYTES: usize = ROW_BYTES * (LCD_H as usize);

// ============================================================================
// RGB565 颜色 (红 5 bit | 绿 6 bit | 蓝 5 bit)
// ============================================================================

/// 16 位 RGB565 颜色类型
type Color = u16;

// 常用颜色常量
const COLOR_BLACK: Color = 0x0000;
const COLOR_WHITE: Color = 0xFFFF;
const COLOR_RED: Color = 0xF800;
const COLOR_GREEN: Color = 0x07E0;
const COLOR_BLUE: Color = 0x001F;
const COLOR_YELLOW: Color = 0xFFE0;
const COLOR_CYAN: Color = 0x07FF;
const COLOR_MAGENTA: Color = 0xF81F;
const COLOR_ORANGE: Color = 0xFD20;
const COLOR_GRAY: Color = 0x8410;

/// 由 R/G/B 8bit 分量构造 RGB565 颜色 (编译期可用)
const fn rgb565(r: u8, g: u8, b: u8) -> Color {
    ((r as u16 & 0xF8) << 8) | ((g as u16 & 0xFC) << 3) | ((b as u16) >> 3)
}

// ============================================================================
// ST7789 驱动 (绑定本示例的具体接线引脚)
// ============================================================================

/// 本示例 SPI0 实例的具体类型（由引脚号确定，编译期校验引脚功能）。
type LcdSpi = Spi0<Pin<PIN_SCK>, Pin<PIN_MOSI>, Pin<PIN_MISO>>;

/// 离屏 RGB565 帧缓冲（放 .bss，约 64.8 KB）。
///
/// 本示例只创建一个 [`St7789`] 实例、且只在主程序取一次可变引用长期独占，
/// 因此用 `static mut` 是安全的；若将来要在中断里共享需改用锁。
static mut FRAMEBUF: [u8; FRAME_BYTES] = [0u8; FRAME_BYTES];

/// ST7789 屏幕句柄。
///
/// 持有 SPI 总线 + DC/RES/CS/BLK 四个控制引脚。
/// 所有屏幕操作封装为方法，演示代码只需调用高层绘图函数。
pub struct St7789 {
    spi: LcdSpi,
    dc: Pin<PIN_DC>,
    res: Pin<PIN_RES>,
    cs: Pin<PIN_CS>,
    blk: Pin<PIN_BLK>,
    /// 离屏 RGB565 帧缓冲（行优先，高字节在前）
    fb: &'static mut [u8; FRAME_BYTES],
    /// 自上次 [`flush`](Self::flush) 以来改动的矩形（含端点；None=已同步）
    dirty: Option<(u16, u16, u16, u16)>,
}

impl St7789 {
    /// 组装屏幕句柄；`fb` 为本例唯一的离屏帧缓冲（主程序取一次后长期独占）。
    fn new(
        spi: LcdSpi,
        dc: Pin<PIN_DC>,
        res: Pin<PIN_RES>,
        cs: Pin<PIN_CS>,
        blk: Pin<PIN_BLK>,
        fb: &'static mut [u8; FRAME_BYTES],
    ) -> Self {
        Self {
            spi,
            dc,
            res,
            cs,
            blk,
            fb,
            dirty: None,
        }
    }

    // ------------------------------------------------------------------
    // 底层字节传输
    // ------------------------------------------------------------------

    /// 发送单个命令字节 (DC=0)
    fn write_cmd(&mut self, cmd: u8) {
        self.dc.set_low();
        self.cs.set_low();
        self.spi.write(&[cmd]);
        self.cs.set_high();
    }

    /// 发送命令 + 参数 (命令期间 DC=0，参数期间 DC=1，CS 全程保持低)
    fn write_cmd_params(&mut self, cmd: u8, params: &[u8]) {
        self.dc.set_low();
        self.cs.set_low();
        self.spi.write(&[cmd]);
        self.dc.set_high();
        self.spi.write(params);
        self.cs.set_high();
    }

    // ------------------------------------------------------------------
    // 初始化
    // ------------------------------------------------------------------

    /// 硬件复位（RES 拉低 ≥10ms 再释放）
    fn reset(&mut self, delay: &impl DelayUs) {
        self.cs.set_high(); // 复位期间保持不片选
        self.res.set_high();
        delay.delay_ms(10);
        self.res.set_low(); // 进入复位
        delay.delay_ms(10);
        self.res.set_high(); // 释放复位
        delay.delay_ms(120); // 等待内部上电稳定
    }

    /// 屏幕初始化。
    ///
    /// 序列取自 **ZJY114S0800TG01 规格书附录**（针对 1.14 寸 135×240 面板
    /// 做过电压/伽马校准），如果你的面板不同，可只保留通用命令并自行校准。
    pub fn init(&mut self, delay: &impl DelayUs) {
        info!("ST7789: resetting...");
        self.reset(delay);

        info!("ST7789: applying init sequence...");

        // 退出睡眠模式 (须等待 > 120ms 稳定)
        self.write_cmd(0x11);
        delay.delay_ms(120);

        // MADCTL (0x36): 数据访问方向。
        //   0x00 = 竖屏正向 / RGB 顺序 (模块出厂默认，官方序列值)
        //   0xC0 = 上下左右镜像 (翻转 180°)
        //   0x08 = BGR 颜色顺序 (若红蓝对调可改此位)
        self.write_cmd_params(0x36, &[0x00]);

        // COLMOD (0x3A): 像素格式 = 0x05 (16 bit/pixel, RGB565)
        self.write_cmd_params(0x3A, &[0x05]);

        // ---- 以下为面板电压/时序/伽马校准 (来自模块官方初始化) ----

        // PORCTL (0xB2):  porch 设置
        self.write_cmd_params(0xB2, &[0x0C, 0x0C, 0x00, 0x33, 0x33]);
        // GCTRL (0xB7):   gate 控制
        self.write_cmd_params(0xB7, &[0x35]);
        // VCOMS (0xBB):  VCOM 设置
        self.write_cmd_params(0xBB, &[0x19]);
        // LCMCTRL (0xC0):  LCM 控制
        self.write_cmd_params(0xC0, &[0x2C]);
        // VDVVRHEN (0xC2):  VDV/VRH 使能
        self.write_cmd_params(0xC2, &[0x01]);
        // VRHS (0xC3):    VRH 设置
        self.write_cmd_params(0xC3, &[0x12]);
        // VDVS (0xC4):    VDV 设置
        self.write_cmd_params(0xC4, &[0x20]);
        // FRCTRL2 (0xC6):  帧率控制
        self.write_cmd_params(0xC6, &[0x0F]);
        // PWCTRL1 (0xD0):  电源控制 1
        self.write_cmd_params(0xD0, &[0xA4, 0xA1]);

        // PVGAMCTRL (0xE0): 正极性伽马校正 (14 字节)
        self.write_cmd_params(
            0xE0,
            &[
                0xD0, 0x04, 0x0D, 0x11, 0x13, 0x2B, 0x3F, 0x54, 0x4C, 0x18, 0x0D, 0x0B, 0x1F, 0x23,
            ],
        );
        // NVGAMCTRL (0xE1): 负极性伽马校正 (14 字节)
        self.write_cmd_params(
            0xE1,
            &[
                0xD0, 0x04, 0x0C, 0x11, 0x13, 0x2C, 0x3F, 0x44, 0x51, 0x2F, 0x1F, 0x1F, 0x20, 0x23,
            ],
        );

        // INVON (0x21): 显示反相开
        self.write_cmd(0x21);
        // DISPON (0x29): 显示开
        self.write_cmd(0x29);

        info!("ST7789: display on, backlight on");
        // 开启背光 (BLK 高电平)
        self.blk.set_high();
    }

    // ------------------------------------------------------------------
    // 帧缓冲：脏区跟踪、离屏写入、一次性上屏
    // ------------------------------------------------------------------

    /// 记录脏矩形并与已有脏区求并集（坐标须已在屏内）。
    fn mark_dirty(&mut self, x0: u16, y0: u16, x1: u16, y1: u16) {
        self.dirty = Some(match self.dirty {
            Some((a0, b0, a1, b1)) => (
                core::cmp::min(a0, x0),
                core::cmp::min(b0, y0),
                core::cmp::max(a1, x1),
                core::cmp::max(b1, y1),
            ),
            None => (x0, y0, x1, y1),
        });
    }

    /// 设置写入窗口 (坐标含端点，均包含边界像素)，自动叠加行列偏移。
    ///
    /// CASET(0x2A)=列范围 (X)，RASET(0x2B)=行范围 (Y)。
    fn set_window(&mut self, x0: u16, y0: u16, x1: u16, y1: u16) {
        let x0 = x0 + COL_OFFSET;
        let x1 = x1 + COL_OFFSET;
        let y0 = y0 + ROW_OFFSET;
        let y1 = y1 + ROW_OFFSET;

        self.write_cmd_params(
            0x2A,
            &[(x0 >> 8) as u8, x0 as u8, (x1 >> 8) as u8, x1 as u8],
        );
        self.write_cmd_params(
            0x2B,
            &[(y0 >> 8) as u8, y0 as u8, (y1 >> 8) as u8, y1 as u8],
        );
    }

    /// 把帧缓冲中的脏矩形整体推送到屏幕（含 RAMWR 0x2C），随后清空脏标记。
    ///
    /// 合成发生在离屏缓冲里，上屏由本方法一次性完成——屏幕只看到完整帧，
    /// 因此切换 demo / 动画都不会出现半清半画或整屏闪黑。
    pub fn flush(&mut self) {
        let Some((x0, y0, x1, y1)) = self.dirty else {
            return;
        };
        self.dirty = None;

        self.set_window(x0, y0, x1, y1);

        // RAMWR 命令 + 数据阶段，CS 全程保持低
        self.cs.set_low();
        self.dc.set_low(); // 命令阶段
        self.spi.write(&[0x2C]); // RAMWR: 写像素数据
        self.dc.set_high(); // 数据阶段

        // 满宽时各行在缓冲内首尾相接，可整段一次发送
        if x0 == 0 && x1 == LCD_W - 1 {
            let start = (y0 as usize) * ROW_BYTES;
            let end = (y1 as usize + 1) * ROW_BYTES;
            self.spi.write(&self.fb[start..end]);
        } else {
            let row_len = ((x1 - x0 + 1) as usize) * BYTES_PER_PIXEL;
            let col_off = (x0 as usize) * BYTES_PER_PIXEL;
            for y in y0..=y1 {
                let start = (y as usize) * ROW_BYTES + col_off;
                self.spi.write(&self.fb[start..start + row_len]);
            }
        }
        self.cs.set_high();
    }

    // ------------------------------------------------------------------
    // 绘图 API（都写进离屏帧缓冲并记录脏区；调用方最后调 flush() 上屏）
    // ------------------------------------------------------------------

    /// 填充整个屏幕为指定颜色（仅写缓冲）。
    pub fn fill_screen(&mut self, color: Color) {
        self.fill_rect(0, 0, LCD_W, LCD_H, color);
    }

    /// 画单个像素（仅写缓冲）。
    pub fn draw_pixel(&mut self, x: u16, y: u16, color: Color) {
        if x >= LCD_W || y >= LCD_H {
            return;
        }
        let i = (y as usize) * ROW_BYTES + (x as usize) * BYTES_PER_PIXEL;
        self.fb[i] = (color >> 8) as u8;
        self.fb[i + 1] = color as u8;
        self.mark_dirty(x, y, x, y);
    }

    /// 填充实心矩形 (x, y 为左上角，w/h 为宽高；仅写缓冲)。
    pub fn fill_rect(&mut self, x: u16, y: u16, w: u16, h: u16, color: Color) {
        if w == 0 || h == 0 || x >= LCD_W || y >= LCD_H {
            return;
        }
        // 用 saturating 运算防止 x+w 等溢出 u16
        let x1 = core::cmp::min(x.saturating_add(w.saturating_sub(1)), LCD_W - 1);
        let y1 = core::cmp::min(y.saturating_add(h.saturating_sub(1)), LCD_H - 1);

        let hi = (color >> 8) as u8;
        let lo = color as u8;
        let wpx = (x1 - x + 1) as usize;
        let col_off = (x as usize) * BYTES_PER_PIXEL;
        for yy in y..=y1 {
            let start = (yy as usize) * ROW_BYTES + col_off;
            let seg = &mut self.fb[start..start + wpx * BYTES_PER_PIXEL];
            let mut i = 0;
            while i < seg.len() {
                seg[i] = hi;
                seg[i + 1] = lo;
                i += BYTES_PER_PIXEL;
            }
        }
        self.mark_dirty(x, y, x1, y1);
    }

    /// 画水平直线。
    pub fn draw_hline(&mut self, x: u16, y: u16, len: u16, color: Color) {
        if y >= LCD_H || len == 0 {
            return;
        }
        let x1 = core::cmp::min(x.saturating_add(len.saturating_sub(1)), LCD_W - 1);
        if x1 < x {
            return;
        }
        self.fill_rect(x, y, x1 - x + 1, 1, color);
    }

    /// 画垂直直线。
    pub fn draw_vline(&mut self, x: u16, y: u16, len: u16, color: Color) {
        if x >= LCD_W || len == 0 {
            return;
        }
        let y1 = core::cmp::min(y.saturating_add(len.saturating_sub(1)), LCD_H - 1);
        if y1 < y {
            return;
        }
        self.fill_rect(x, y, 1, y1 - y + 1, color);
    }

    /// 画空心矩形边框。
    pub fn draw_rect(&mut self, x: u16, y: u16, w: u16, h: u16, color: Color) {
        if x >= LCD_W || y >= LCD_H || w == 0 || h == 0 {
            return;
        }
        // 先把矩形裁剪到屏幕内，保证后续坐标运算不越界
        let w = core::cmp::min(w, LCD_W - x);
        let h = core::cmp::min(h, LCD_H - y);
        // 上下两条水平边
        self.draw_hline(x, y, w, color);
        self.draw_hline(x, y + h - 1, w, color);
        // 左右两条垂直边 (跳过端点, 避免重复)
        if h > 2 {
            self.draw_vline(x, y + 1, h - 2, color);
            self.draw_vline(x + w - 1, y + 1, h - 2, color);
        }
    }

    // ------------------------------------------------------------------
    // 中文文本绘制（1bit 位图字库 font.rs）
    // ------------------------------------------------------------------

    /// 在 (x, y) 处直接写一个像素（越界忽略，不标记脏区）。
    fn put_px(&mut self, x: i32, y: i32, color: Color) {
        if x < 0 || y < 0 || x as u32 >= LCD_W as u32 || y as u32 >= LCD_H as u32 {
            return;
        }
        let i = (y as usize) * ROW_BYTES + (x as usize) * BYTES_PER_PIXEL;
        self.fb[i] = (color >> 8) as u8;
        self.fb[i + 1] = color as u8;
    }

    /// 以 (x, y) 为左上角绘制一行文本（支持内嵌中文位图字库）；`scale` 为放大倍数。
    /// 每字占 `GLYPH_SIZE×scale` 像素、字间距 `scale`。返回文本总宽(px)。
    pub fn draw_text(&mut self, x: i32, y: i32, s: &str, color: Color, scale: u32) -> i32 {
        let sc = scale as i32;
        let gsize = font::GLYPH_SIZE as i32;
        let step = gsize * sc; // 每字步进(不含字间距)
        let gap = sc; // 字间距
        let total = s.chars().count() as i32;
        let mut cx = x;

        for (i, ch) in s.chars().enumerate() {
            let Some(data) = font::find(ch) else {
                continue;
            };
            // 逐位 blit：1=前景色，0=保持背景透明
            for ry in 0..gsize {
                for rx in 0..gsize {
                    let on = data[(ry as usize) * font::GLYPH_ROW_BYTES + (rx / 8) as usize]
                        & (0x80u8 >> ((rx % 8) as u32))
                        != 0;
                    if !on {
                        continue;
                    }
                    for sy in 0..scale {
                        for sx in 0..scale {
                            self.put_px(cx + rx * sc + sx as i32, y + ry * sc + sy as i32, color);
                        }
                    }
                }
            }
            // 记录该字区域到脏区（裁到屏内）
            let gx0 = cx.clamp(0, LCD_W as i32 - 1) as u16;
            let gy0 = y.clamp(0, LCD_H as i32 - 1) as u16;
            let gx1 = (cx + step).clamp(0, LCD_W as i32 - 1) as u16;
            let gy1 = (y + step).clamp(0, LCD_H as i32 - 1) as u16;
            self.mark_dirty(gx0, gy0, gx1, gy1);

            cx += step;
            if i as i32 != total - 1 {
                cx += gap;
            }
        }
        cx - x
    }
}

// ============================================================================
// 演示图案
// ============================================================================

/// Demo 1: 整屏纯色切换，用于快速验证接线与颜色通道。
fn demo_pure_colors(lcd: &mut St7789, delay: &impl DelayUs) {
    let tests: [(&str, Color); 6] = [
        ("RED", COLOR_RED),
        ("GREEN", COLOR_GREEN),
        ("BLUE", COLOR_BLUE),
        ("WHITE", COLOR_WHITE),
        ("BLACK", COLOR_BLACK),
        ("GRAY", COLOR_GRAY),
    ];

    for (name, color) in tests.iter() {
        info!("fill screen: {}", name);
        lcd.fill_screen(*color);
        lcd.flush(); // 整帧一次性上屏，切换不闪
        delay.delay_ms(700);
    }
}

/// Demo 2: 彩虹横条 (水平分隔 8 个色带)。
fn demo_rainbow_bars(lcd: &mut St7789) {
    const BAND: u16 = 8; // 横条数量
    const H: u16 = LCD_H / BAND; // 每带高度

    let colors = [
        COLOR_RED,
        COLOR_ORANGE,
        COLOR_YELLOW,
        COLOR_GREEN,
        COLOR_CYAN,
        COLOR_BLUE,
        COLOR_MAGENTA,
        COLOR_WHITE,
    ];

    info!("demo: rainbow bars");
    for (i, color) in colors.iter().enumerate() {
        lcd.fill_rect(0, (i as u16) * H, LCD_W, H, *color);
    }
    lcd.flush();
}

/// Demo 3: 棋盘格 (黑白交替 16px 方格)。
fn demo_checkerboard(lcd: &mut St7789) {
    const CELL: u16 = 16;

    info!("demo: checkerboard");
    for y in (0..LCD_H).step_by(CELL as usize) {
        for x in (0..LCD_W).step_by(CELL as usize) {
            let black = ((x / CELL) + (y / CELL)).is_multiple_of(2);
            lcd.fill_rect(
                x,
                y,
                CELL,
                CELL,
                if black { COLOR_BLACK } else { COLOR_WHITE },
            );
        }
    }
    lcd.flush();
}

/// Demo 4: 图形绘制 —— 彩色边框、对角线、网格与点阵。
fn demo_shapes(lcd: &mut St7789) {
    info!("demo: shapes");
    lcd.fill_screen(COLOR_BLACK);

    // 由外到内的彩色边框
    lcd.draw_rect(1, 1, LCD_W - 2, LCD_H - 2, COLOR_RED);
    lcd.draw_rect(5, 5, LCD_W - 10, LCD_H - 10, COLOR_GREEN);
    lcd.draw_rect(9, 9, LCD_W - 18, LCD_H - 18, COLOR_BLUE);

    // 两条对角线
    for i in 0..core::cmp::min(LCD_W, LCD_H) {
        lcd.draw_pixel(i, i, COLOR_YELLOW);
        lcd.draw_pixel(LCD_W - 1 - i, i, COLOR_CYAN);
    }

    // 水平/垂直中线
    lcd.draw_hline(0, LCD_H / 2, LCD_W, COLOR_WHITE);
    lcd.draw_vline(LCD_W / 2, 0, LCD_H, COLOR_WHITE);

    // 四角与中心的定位点 (便于观察坐标方向是否正常)
    let dots = [
        (0, 0),
        (LCD_W - 1, 0),
        (0, LCD_H - 1),
        (LCD_W - 1, LCD_H - 1),
        (LCD_W / 2, LCD_H / 2),
    ];
    for (x, y) in dots.iter() {
        lcd.fill_rect(*x, *y, 3, 3, COLOR_MAGENTA);
    }
    lcd.flush();
}

/// Demo 5b: 显示中文文字（内嵌 1bit 位图字库，仅含少量字符）。
fn demo_chinese(lcd: &mut St7789) {
    info!("demo: chinese text");
    const TEXT: &str = "屏幕测试";
    const SCALE: u32 = 1; // 放大倍数：1 = 原生 16×16 点阵
    lcd.fill_screen(COLOR_BLACK);

    let gsize = font::GLYPH_SIZE as i32 * SCALE as i32;
    let n = TEXT.chars().count() as i32;
    // 总宽 = 每字宽 + 字间距(取 SCALE)
    let tw = n * gsize + (n - 1) * SCALE as i32;
    let x = ((LCD_W as i32) - tw) / 2;
    let y = ((LCD_H as i32) - gsize) / 2;

    lcd.draw_text(x, y, TEXT, COLOR_YELLOW, SCALE);
    lcd.flush();
}

/// Demo 5: 由中心向外放大的方块动画。
fn demo_growing_square(lcd: &mut St7789, delay: &impl DelayUs) {
    info!("demo: growing square");
    const MAX: u16 = 100;

    // 黑底一次性铺好并上屏；之后每帧只重画"旧方块擦除 + 新方块"的并集区域
    lcd.fill_screen(COLOR_BLACK);
    lcd.flush();

    let mut size: u16 = 4;
    let mut frame = 0u16;
    let mut prev: Option<(u16, u16, u16)> = None; // 上一帧 (x, y, size)

    while size <= MAX {
        let x = (LCD_W - size) / 2;
        let y = (LCD_H - size) / 2;

        // 在离屏缓冲里擦掉上一帧方块
        if let Some((px, py, ps)) = prev {
            lcd.fill_rect(px, py, ps, ps, COLOR_BLACK);
        }

        // 颜色随帧数变化
        let color = rgb565(
            ((frame * 3) & 0xFF) as u8,
            ((frame * 7) & 0xFF) as u8,
            ((frame * 11) & 0xFF) as u8,
        );
        lcd.fill_rect(x, y, size, size, color);
        lcd.flush(); // 脏区自动合并为旧∪新，只推送这一小块

        prev = Some((x, y, size));
        size += 4;
        frame += 1;
        delay.delay_ms(60);
    }
}

/// Demo 6: 一个来回滑动的渐变彩条。
///
/// 只重绘彩条经过的局部区域（擦上一帧位置 → 画当前位置），
/// 不做整屏清除，避免每帧全屏闪黑造成明显闪烁。
fn demo_sliding_bar(lcd: &mut St7789, delay: &impl DelayUs) {
    const BAR_W: u16 = 20;
    const BAR_H: u16 = 60;
    const Y: u16 = (LCD_H - BAR_H) / 2;
    const MAX_X: u16 = LCD_W - BAR_W;
    const STEP: u16 = 2;

    // 黑底一次性铺好并上屏，之后只更新彩条经过的区域
    lcd.fill_screen(COLOR_BLACK);
    lcd.flush();

    // 先向右、再向左，各一趟
    for pass in 0..2u16 {
        let color = if pass == 0 { COLOR_YELLOW } else { COLOR_CYAN };
        let dir: i16 = if pass == 0 { 1 } else { -1 };
        let mut x: i16 = if pass == 0 { 0 } else { MAX_X as i16 };
        let mut prev: Option<i16> = None;

        loop {
            // 离屏擦掉上一帧彩条，再画当前位置；脏区自动并集，一次小区域推送
            if let Some(p) = prev {
                lcd.fill_rect(p as u16, Y, BAR_W, BAR_H, COLOR_BLACK);
            }
            lcd.fill_rect(x as u16, Y, BAR_W, BAR_H, color);
            lcd.flush();
            prev = Some(x);

            delay.delay_ms(20);

            x += dir * STEP as i16;
            if x < 0 || x as u16 > MAX_X {
                break;
            }
        }
    }
}

// ============================================================================
// 主程序
// ============================================================================

#[main]
fn main() -> ! {
    info!("=== ST7789 TFT SPI Example ===");
    info!("ZJY114S0800TG01 1.14\" 135x240 ST7789V via SPI0");

    // ------------------------------------------------------------------
    // 1. 系统初始化 (时钟 150 MHz + GPIO)
    // ------------------------------------------------------------------
    let dp = Peripherals::take().unwrap();

    let clocks = ClockBuilder::new().build(dp.CLOCKS, dp.XOSC, dp.PLL_SYS, dp.PLL_USB);
    info!("System clock: {} Hz", clocks.sys_freq());

    let pins = Pins::init(dp.IO_BANK0, dp.PADS_BANK0);

    // SysTick 延时
    let delay = rp2350_examples::common::board::init_delay(&clocks);

    // ------------------------------------------------------------------
    // 2. SPI0 初始化 (SCK=GP2, MOSI=GP3, MISO=GP0 占位)
    // ------------------------------------------------------------------
    let spi = Spi0::init(
        dp.SPI0,
        pins.pin::<PIN_SCK>(),
        pins.pin::<PIN_MOSI>(),
        pins.pin::<PIN_MISO>(),
        SPI_BAUDRATE,
        clocks.peri_freq(),
    );
    info!(
        "SPI0 ready: SCK=GP{}, MOSI=GP{}, {} Hz (8-bit, Mode0)",
        PIN_SCK, PIN_MOSI, SPI_BAUDRATE
    );

    // ------------------------------------------------------------------
    // 3. 控制引脚配置为输出 (DC/RES/CS 默认非选中, 背光默认关闭)
    // ------------------------------------------------------------------
    let mut dc = pins.pin::<PIN_DC>();
    dc.into_output().set_high(); // DC 默认高 (数据态)
    let mut res = pins.pin::<PIN_RES>();
    res.into_output().set_high(); // 默认不复位
    let mut cs = pins.pin::<PIN_CS>();
    cs.into_output().set_high(); // CS 默认不片选
    let mut blk = pins.pin::<PIN_BLK>();
    blk.into_output().set_low(); // 背光默认关
    blk.set_drive_strength(DriveStrength::_8mA); // 背光驱动电流加大

    info!(
        "Control pins: DC=GP{} RES=GP{} CS=GP{} BLK=GP{}",
        PIN_DC, PIN_RES, PIN_CS, PIN_BLK
    );

    // ------------------------------------------------------------------
    // 4. 组装屏幕句柄并初始化
    // ------------------------------------------------------------------
    // 取出唯一的离屏帧缓冲 (64.8 KB, .bss)；本程序仅此一处创建 &'static mut，
    // 之后由 lcd 长期独占，因此是安全的。
    let fb: &'static mut [u8; FRAME_BYTES] = unsafe { &mut *core::ptr::addr_of_mut!(FRAMEBUF) };

    let mut lcd = St7789::new(spi, dc, res, cs, blk, fb);
    lcd.init(&delay);

    // 铺黑并上屏，覆盖上电后的随机花屏，获得干净的起始画面
    lcd.fill_screen(COLOR_BLACK);
    lcd.flush();

    info!("ST7789 initialized: {}x{}", LCD_W, LCD_H);

    // ------------------------------------------------------------------
    // 5. 演示程序
    // ------------------------------------------------------------------
    demo_pure_colors(&mut lcd, &delay);
    delay.delay_ms(300);

    demo_rainbow_bars(&mut lcd);
    delay.delay_ms(1500);

    demo_checkerboard(&mut lcd);
    delay.delay_ms(1500);

    demo_shapes(&mut lcd);
    delay.delay_ms(1500);

    demo_chinese(&mut lcd);
    delay.delay_ms(10000);

    demo_growing_square(&mut lcd, &delay);
    delay.delay_ms(500);

    info!("=== Entering sliding-bar demo loop ===");

    // ------------------------------------------------------------------
    // 6. 无限循环：反复播放滑动彩条动画
    // ------------------------------------------------------------------
    loop {
        demo_sliding_bar(&mut lcd, &delay);
    }
}
