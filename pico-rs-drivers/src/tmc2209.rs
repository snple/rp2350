//! TMC2209 步进电机驱动 — UART 通信与配置
//!
//! 提供 TMC2209 的寄存器级 UART 读写操作和高层配置函数。
//!
//! ## 设计要点
//!
//! - **不依赖具体 Timer 类型**: 所有需要延时的操作通过 [`DelayUs`] trait 注入
//! - **UART 抽象**: 通过 [`UartBus`] trait 解耦，不绑定特定 UART 外设
//! - **零开销封装**: [`Tmc2209`] 结构体仅持有 UART 句柄和从机地址，
//!   所有方法为 `&self`，可安全共享
//!
//! ## TMC2209 UART 协议
//!
//! TMC2209 使用自定义的单线半双工协议，基于标准 UART 8N1 帧格式:
//!
//! **写数据报** (8 字节, MCU → TMC2209):
//! ```text
//! ┌──────┬───────────┬───────────┬───────┬───────┬───────┬───────┬─────┐
//! │ SYNC │ SLAVE_ADDR│ REG|0x80  │ DATA3 │ DATA2 │ DATA1 │ DATA0 │ CRC │
//! │ 0x05 │  0x00~03  │  写标志   │ MSB   │       │       │ LSB   │     │
//! └──────┴───────────┴───────────┴───────┴───────┴───────┴───────┴─────┘
//! ```
//!
//! **读请求** (4 字节, MCU → TMC2209):
//! ```text
//! ┌──────┬───────────┬─────┬─────┐
//! │ SYNC │ SLAVE_ADDR│ REG │ CRC │
//! │ 0x05 │  0x00~03  │     │     │
//! └──────┴───────────┴─────┴─────┘
//! ```
//!
//! **读回复** (8 字节, TMC2209 → MCU):
//! ```text
//! ┌──────┬──────┬─────┬───────┬───────┬───────┬───────┬─────┐
//! │ SYNC │ 0xFF │ REG │ DATA3 │ DATA2 │ DATA1 │ DATA0 │ CRC │
//! │ 0x05 │ 主机 │     │ MSB   │       │       │ LSB   │     │
//! └──────┴──────┴─────┴───────┴───────┴───────┴───────┴─────┘
//! ```
//!
//! ## 使用示例
//!
//! ```ignore
//! use pico_rs_drivers::tmc2209::{Tmc2209, REG_IOIN};
//!
//! // 创建 TMC2209 句柄 (slave_addr 由 MS1/MS2 引脚决定)
//! let tmc = Tmc2209::new(uart, 0x00);
//!
//! // 读取芯片版本
//! if let Some(ioin) = tmc.read_reg(REG_IOIN, &delay) {
//!     let version = (ioin >> 24) & 0xFF;
//!     // TMC2209 version = 0x21
//! }
//!
//! // 完整配置芯片
//! tmc.configure(&delay, 16, 16, 8, 10, 10);
//! ```

use bitfield_struct::bitfield;
use pico_rs::delay::DelayUs;
use pico_rs::uart::UartBus;

// ============================================================================
// TMC2209 寄存器地址
// ============================================================================

/// GCONF (0x00) — 全局配置寄存器
///
/// | 位 | 名称 | 说明 |
/// |----|------|------|
/// | 6  | multistep_filt | STEP 脉冲边沿滤波 |
/// | 7  | pdn_disable | PDN_UART 仅用于 UART 通信 (必须置 1) |
pub const REG_GCONF: u8 = 0x00;

/// GSTAT (0x01) — 全局状态寄存器 (读后自动清除)
///
/// | 位 | 名称 | 说明 |
/// |----|------|------|
/// | 0  | reset | 芯片已复位 |
/// | 1  | drv_err | 驱动器过温/短路 |
/// | 2  | uv_cp | 电荷泵欠压 |
pub const REG_GSTAT: u8 = 0x01;

/// IFCNT (0x02) — UART 接口写入计数器
///
/// 每次成功的 UART 写操作 +1，用于校验通信完整性。
pub const REG_IFCNT: u8 = 0x02;

/// IOIN (0x06) — 输入引脚状态 + 芯片版本号
///
/// | 位 | 名称 | 说明 |
/// |----|------|------|
/// | [31:24] | VERSION | 芯片版本 (TMC2209 = 0x21) |
/// | 0 | ENN | ENN 引脚状态 |
/// | 1 | MS1 | MS1 引脚状态 |
/// | 2 | MS2 | MS2 引脚状态 |
pub const REG_IOIN: u8 = 0x06;

/// IHOLD_IRUN (0x10) — 保持/运行电流设置
///
/// | 位 | 名称 | 说明 |
/// |----|------|------|
/// | [4:0] | IHOLD | 保持电流 (0–31) |
/// | [12:8] | IRUN | 运行电流 (0–31) |
/// | [19:16] | IHOLDDELAY | 电流过渡延时 (0–15) |
pub const REG_IHOLD_IRUN: u8 = 0x10;

/// TPOWERDOWN (0x11) — 停止后电流降低延时
///
/// 单位: 约 2^18 个时钟周期 (~2.6 ms @ 12 MHz)。设 0 则不掉电。
pub const REG_TPOWERDOWN: u8 = 0x11;

/// TSTEP (0x12) — 最近一步的实际步进间隔 (只读)
///
/// 用于 CoolStep 负载自适应。值越小 = 速度越快。
pub const REG_TSTEP: u8 = 0x12;

/// TCOOLTHRS (0x14) — CoolStep / StallGuard 激活的速度阈值
///
/// StallGuard 激活条件: `TSTEP ≤ TCOOLTHRS`。
/// 设为最大值 (0xFFFFF) 表示任何速度下都激活。
pub const REG_TCOOLTHRS: u8 = 0x14;

/// SGTHRS (0x40) — StallGuard 堵转检测阈值
///
/// 触发条件: `SG_RESULT ≤ SGTHRS × 2`。
/// 值越大越灵敏。设为 0 禁用堵转检测。
pub const REG_SGTHRS: u8 = 0x40;

/// SG_RESULT (0x41) — StallGuard 结果值 (只读, 0–510)
///
/// ~510 = 空载，接近 0 = 堵转。仅在电机运动时有效。
pub const REG_SG_RESULT: u8 = 0x41;

/// CHOPCONF (0x6C) — 斩波器配置
///
/// | 位 | 名称 | 说明 |
/// |----|------|------|
/// | [3:0] | TOFF | 斩波关断时间 |
/// | [7:4] | HSTRT | 滞后起始值 |
/// | [14:8] | HEND | 滞后结束值 |
/// | 15 | TBL | 消隐时间比较 |
/// | [27:24] | MRES | 微步分辨率 (0=256 ... 8=全步) |
/// | 28 | intpol | 256 微步插值使能 |
pub const REG_CHOPCONF: u8 = 0x6C;

/// DRV_STATUS (0x6F) — 驱动器状态 (只读)
///
/// | 位 | 名称 | 说明 |
/// |----|------|------|
/// | [20:16] | CS_ACTUAL | 实际电流缩放值 |
/// | 25 | stst | 静止检测 |
/// | 29 | ola | A 相开路 |
/// | 30 | olb | B 相开路 |
/// | 26 | otpw | 过温预警 (~120°C) |
/// | 25 | ot | 过温关断 (~150°C) |
pub const REG_DRV_STATUS: u8 = 0x6F;

/// PWMCONF (0x70) — StealthChop PWM 调谐参数
///
/// | 位 | 名称 | 说明 |
/// |----|------|------|
/// | [7:0] | PWM_OFS | PWM 初始偏移 |
/// | [15:8] | PWM_GRAD | PWM 梯度 |
/// | [17:16] | PWM_FREQ | PWM 频率 |
/// | 18 | PWM_AUTOSCALE | 自动调幅 |
/// | 19 | PWM_AUTOGRAD | 自动调梯度 |
/// | [27:24] | PWM_LIM | PWM 限幅 |
/// | [31:28] | PWM_REG | 调节精度 |
pub const REG_PWMCONF: u8 = 0x70;

// ============================================================================
// 寄存器 bitfield 类型
// ============================================================================

/// GCONF (0x00) — 全局配置
#[bitfield(u32)]
pub struct Gconf {
    /// 0 = StealthChop, 1 = SpreadCycle
    #[bits(1)]
    pub en_spread_cycle: bool,
    #[bits(3)]
    __: u8,
    /// 反转电机方向
    #[bits(1)]
    pub shaft: bool,
    #[bits(1)]
    __: u8,
    /// STEP 脉冲边沿滤波
    #[bits(1)]
    pub multistep_filt: bool,
    /// PDN_UART 仅用于 UART 通信 (必须置 1)
    #[bits(1)]
    pub pdn_disable: bool,
    #[bits(24)]
    __: u32,
}

/// GSTAT (0x01) — 全局状态 (读后自动清除)
#[bitfield(u32)]
pub struct Gstat {
    /// 芯片已复位
    #[bits(1)]
    pub reset: bool,
    /// 驱动器过温/短路
    #[bits(1)]
    pub drv_err: bool,
    /// 电荷泵欠压
    #[bits(1)]
    pub uv_cp: bool,
    #[bits(29)]
    __: u32,
}

/// IOIN (0x06) — 输入引脚状态 + 芯片版本号
#[bitfield(u32)]
pub struct Ioin {
    /// ENN 引脚状态
    #[bits(1)]
    pub enn: bool,
    /// MS1 引脚状态
    #[bits(1)]
    pub ms1: bool,
    /// MS2 引脚状态
    #[bits(1)]
    pub ms2: bool,
    #[bits(1)]
    __: u8,
    /// STEP 引脚状态
    #[bits(1)]
    pub step: bool,
    /// DIR 引脚状态
    #[bits(1)]
    pub dir: bool,
    /// Spread 使能引脚状态
    #[bits(1)]
    pub spread_en: bool,
    /// 索引引脚状态
    #[bits(1)]
    pub index: bool,
    #[bits(16)]
    __: u32,
    /// 芯片版本号 (TMC2209 = 0x21)
    #[bits(8)]
    pub version: u8,
}

/// IHOLD_IRUN (0x10) — 保持/运行电流
#[bitfield(u32)]
pub struct IholdIr {
    /// 保持电流 (0–31)
    #[bits(5)]
    pub ihold: u8,
    #[bits(3)]
    __: u8,
    /// 运行电流 (0–31)
    #[bits(5)]
    pub irun: u8,
    #[bits(3)]
    __: u8,
    /// 电流过渡延时 (0–15)
    #[bits(4)]
    pub iholddelay: u8,
    #[bits(12)]
    __: u32,
}

/// CHOPCONF (0x6C) — 斩波器配置
#[bitfield(u32)]
pub struct Chopconf {
    /// 斩波关断时间
    #[bits(4)]
    pub toff: u8,
    /// 滞后起始值
    #[bits(3)]
    pub hstrt: u8,
    /// 滞后结束值
    #[bits(4)]
    pub hend: u8,
    #[bits(4)]
    __: u8,
    /// 消隐时间比较
    #[bits(2)]
    pub tbl: u8,
    #[bits(7)]
    __: u16,
    /// 微步分辨率 (0=256 ... 8=全步)
    #[bits(4)]
    pub mres: u8,
    /// 256 微步插值使能
    #[bits(1)]
    pub intpol: bool,
    #[bits(3)]
    __: u8,
}

/// DRV_STATUS (0x6F) — 驱动器状态 (只读)
#[bitfield(u32)]
pub struct DrvStatus {
    #[bits(16)]
    __: u32,
    /// 实际电流缩放值
    #[bits(5)]
    pub cs_actual: u8,
    #[bits(4)]
    __: u8,
    /// 过温关断 (~150°C)
    #[bits(1)]
    pub ot: bool,
    /// 过温预警 (~120°C)
    #[bits(1)]
    pub otpw: bool,
    /// StallGuard A 相
    #[bits(1)]
    pub s2ga: bool,
    /// StallGuard B 相
    #[bits(1)]
    pub s2gb: bool,
    /// A 相开路
    #[bits(1)]
    pub ola: bool,
    /// B 相开路
    #[bits(1)]
    pub olb: bool,
    /// 静止检测
    #[bits(1)]
    pub stst: bool,
}

/// PWMCONF (0x70) — StealthChop PWM 配置
#[bitfield(u32)]
pub struct Pwmconf {
    /// PWM 初始偏移
    #[bits(8)]
    pub pwm_ofs: u8,
    /// PWM 梯度
    #[bits(8)]
    pub pwm_grad: u8,
    /// PWM 频率
    #[bits(2)]
    pub pwm_freq: u8,
    /// 自动调幅
    #[bits(1)]
    pub pwm_autoscale: bool,
    /// 自动调梯度
    #[bits(1)]
    pub pwm_autograd: bool,
    #[bits(4)]
    __: u8,
    /// PWM 限幅
    #[bits(4)]
    pub pwm_lim: u8,
    /// 调节精度
    #[bits(4)]
    pub pwm_reg: u8,
}

// ============================================================================
// 协议常量
// ============================================================================

/// TMC2209 UART 同步字节 (所有数据报以此开头)
const TMC_SYNC: u8 = 0x05;

/// TMC2209 UART 读请求字节数
const READ_REQUEST_LEN: usize = 4;

/// TMC2209 UART 写数据报字节数
const WRITE_DATAGRAM_LEN: usize = 8;

/// TMC2209 UART 回复数据报字节数
const REPLY_DATAGRAM_LEN: usize = 8;

/// 读操作中，TMC2209 回复前的典型延迟 (μs)
const REPLY_DELAY_US: u32 = 200;

/// 单字节读取的超时时间 (μs)
const BYTE_TIMEOUT_US: u32 = 5_000;

/// 轮询间隔 (μs)，用于超时循环中的忙等待粒度
const POLL_INTERVAL_US: u32 = 10;

/// 默认 IHOLDDELAY 值 (电流过渡延时)
const IHOLD_DELAY_DEFAULT: u8 = 10;

/// 微步细分值与 MRES 编码的映射表
const MRES_TABLE: [(u32, u32); 9] = [
    (256, 0),
    (128, 1),
    (64, 2),
    (32, 3),
    (16, 4),
    (8, 5),
    (4, 6),
    (2, 7),
    (1, 8),
];

// ============================================================================
// CRC8
// ============================================================================

/// 计算 TMC2209 CRC-8 校验值。
///
/// 生成多项式: `0x07` (x^8 + x^2 + x + 1)，初始值 `0x00`。
/// TMC2209 采用 LSB-first 位处理顺序。
///
/// # 示例
///
/// ```
/// use pico_rs_drivers::tmc2209::calc_crc8;
/// assert_eq!(calc_crc8(&[0x05, 0x00, 0x06]), 0x54); // 读 IOIN 请求
/// ```
#[inline]
pub fn calc_crc8(data: &[u8]) -> u8 {
    let mut crc: u8 = 0;
    for &byte in data {
        let mut current = byte;
        for _ in 0..8 {
            if (crc >> 7) ^ (current & 0x01) != 0 {
                crc = (crc << 1) ^ 0x07;
            } else {
                crc <<= 1;
            }
            current >>= 1;
        }
    }
    crc
}

// ============================================================================
// Tmc2209 struct
// ============================================================================

/// TMC2209 步进电机驱动器句柄。
///
/// 封装对共享 UART 总线的引用和从机地址，提供寄存器读写和芯片配置功能。
///
/// 同一条 UART 总线上可挂多个 TMC2209（最多 4 个），
/// 通过 MS1/MS2 引脚设置不同从机地址即可创建多个实例共享同一个 `&Uart`：
///
/// ```ignore
/// let tmc_x = Tmc2209::new(&uart, 0x00); // MS1=GND, MS2=GND
/// let tmc_y = Tmc2209::new(&uart, 0x01); // MS1=VCC, MS2=GND
/// let tmc_z = Tmc2209::new(&uart, 0x02); // MS1=GND, MS2=VCC
/// // 三个轴共享一条 UART 总线
/// ```
///
/// # 类型参数
///
/// - `U`: 实现 [`UartBus`] 的 UART 类型 (如 `Uart1<TX, RX>`)
pub struct Tmc2209<'a, U: UartBus> {
    uart: &'a U,
    slave_addr: u8,
}

impl<'a, U: UartBus> Tmc2209<'a, U> {
    /// 创建新的 TMC2209 句柄。
    ///
    /// 持有 UART 的共享引用，因此同一条总线可创建多个不同地址的实例。
    ///
    /// # 参数
    ///
    /// - `uart`: 对实现 [`UartBus`] 的 UART 实例的共享引用
    /// - `slave_addr`: TMC2209 从机地址 (0x00–0x03)，由 MS1/MS2 引脚决定
    ///
    /// | MS1 | MS2 | 从机地址 |
    /// |-----|-----|----------|
    /// | GND | GND | 0x00 (默认) |
    /// | VCC | GND | 0x01 |
    /// | GND | VCC | 0x02 |
    /// | VCC | VCC | 0x03 |
    pub fn new(uart: &'a U, slave_addr: u8) -> Self {
        Self { uart, slave_addr }
    }

    /// 获取 UART 的引用。
    #[inline]
    pub fn uart(&self) -> &U {
        self.uart
    }

    /// 获取从机地址。
    #[inline]
    pub fn slave_addr(&self) -> u8 {
        self.slave_addr
    }

    // ========================================================================
    // 寄存器读写
    // ========================================================================

    /// 写入 TMC2209 寄存器。
    ///
    /// 构造 8 字节写数据报并发送，
    /// 然后读取并丢弃单线半双工拓扑产生的回显字节。
    ///
    /// # 参数
    ///
    /// - `reg`: 寄存器地址 (0x00–0x7F)
    /// - `value`: 32 位寄存器值
    /// - `delay`: 实现 [`DelayUs`] 的延时源 (用于回显排出超时)
    pub fn write_reg(&self, reg: u8, value: u32, delay: &impl DelayUs) {
        let mut datagram = [0u8; WRITE_DATAGRAM_LEN];
        datagram[0] = TMC_SYNC;
        datagram[1] = self.slave_addr;
        datagram[2] = reg | 0x80; // bit7 = 1 表示写操作
        datagram[3] = (value >> 24) as u8;
        datagram[4] = (value >> 16) as u8;
        datagram[5] = (value >> 8) as u8;
        datagram[6] = value as u8;
        datagram[7] = calc_crc8(&datagram[..7]);

        let _ = self.uart.write(&datagram);
        let _ = self.uart.flush();

        // 丢弃回显 (单线半双工: TX 发出的字节会被 RX 收到)
        self.drain_bytes(WRITE_DATAGRAM_LEN, delay, BYTE_TIMEOUT_US);
    }

    /// 读取 TMC2209 寄存器。
    ///
    /// 发送 4 字节读请求，丢弃回显，等待 8 字节回复，校验 CRC 和帧头。
    ///
    /// # 参数
    ///
    /// - `reg`: 寄存器地址 (0x00–0x7F)
    /// - `delay`: 实现 [`DelayUs`] 的延时源 (用于等待回复)
    ///
    /// # 返回
    ///
    /// - `Some(value)`: 读取成功，返回 32 位寄存器值
    /// - `None`: 通信失败 (CRC 错误 / 超时 / 帧头不符)
    pub fn read_reg(&self, reg: u8, delay: &impl DelayUs) -> Option<u32> {
        // 构造读请求
        let mut request = [0u8; READ_REQUEST_LEN];
        request[0] = TMC_SYNC;
        request[1] = self.slave_addr;
        request[2] = reg;
        request[3] = calc_crc8(&request[..3]);

        let _ = self.uart.write(&request);
        let _ = self.uart.flush();

        // 丢弃 4 字节回显
        self.drain_bytes(READ_REQUEST_LEN, delay, BYTE_TIMEOUT_US);

        // 等待 TMC2209 回复 (~75μs 典型延迟)
        delay.delay_us(REPLY_DELAY_US);

        // 接收 8 字节回复
        let mut reply = [0u8; REPLY_DATAGRAM_LEN];
        for slot in reply.iter_mut() {
            *slot = self.read_byte_timeout(delay, BYTE_TIMEOUT_US)?;
        }

        // CRC 校验
        let crc = calc_crc8(&reply[..7]);
        if crc != reply[7] {
            #[cfg(feature = "defmt")]
            defmt::warn!(
                "TMC2209 CRC error: computed {:#04x}, received {:#04x}",
                crc,
                reply[7]
            );
            return None;
        }

        // 帧头校验
        if reply[0] != TMC_SYNC || reply[1] != 0xFF {
            #[cfg(feature = "defmt")]
            defmt::warn!(
                "TMC2209 bad reply header: [{:#04x}, {:#04x}]",
                reply[0],
                reply[1]
            );
            return None;
        }

        let value = (reply[3] as u32) << 24
            | (reply[4] as u32) << 16
            | (reply[5] as u32) << 8
            | (reply[6] as u32);

        Some(value)
    }

    // ========================================================================
    // 寄存器快捷方法
    // ========================================================================

    /// 读取 IOIN 寄存器 (类型化)。
    pub fn read_ioin(&self, delay: &impl DelayUs) -> Option<Ioin> {
        self.read_reg(REG_IOIN, delay).map(Ioin::from)
    }

    /// 读取 GCONF 寄存器 (类型化)。
    pub fn read_gconf(&self, delay: &impl DelayUs) -> Option<Gconf> {
        self.read_reg(REG_GCONF, delay).map(Gconf::from)
    }

    /// 写入 GCONF 寄存器。
    pub fn write_gconf(&self, gconf: &Gconf, delay: &impl DelayUs) {
        self.write_reg(REG_GCONF, (*gconf).into(), delay);
    }

    /// 读取 GSTAT 寄存器 (类型化，读后自动清除)。
    pub fn read_gstat(&self, delay: &impl DelayUs) -> Option<Gstat> {
        self.read_reg(REG_GSTAT, delay).map(Gstat::from)
    }

    /// 读取 IFCNT 寄存器 (接口写入计数)。
    ///
    /// 每成功执行一次 UART 写入，该计数器 +1。
    pub fn read_ifcnt(&self, delay: &impl DelayUs) -> Option<u8> {
        self.read_reg(REG_IFCNT, delay).map(|v| v as u8)
    }

    /// 读取 CHOPCONF 寄存器 (类型化)。
    pub fn read_chopconf(&self, delay: &impl DelayUs) -> Option<Chopconf> {
        self.read_reg(REG_CHOPCONF, delay).map(Chopconf::from)
    }

    /// 读取 PWMCONF 寄存器 (类型化)。
    pub fn read_pwmconf(&self, delay: &impl DelayUs) -> Option<Pwmconf> {
        self.read_reg(REG_PWMCONF, delay).map(Pwmconf::from)
    }

    /// 读取 DRV_STATUS 寄存器 (类型化)。
    pub fn read_drv_status(&self, delay: &impl DelayUs) -> Option<DrvStatus> {
        self.read_reg(REG_DRV_STATUS, delay).map(DrvStatus::from)
    }

    /// 读取 SG_RESULT 寄存器 (StallGuard 结果，0–510)。
    ///
    /// 仅在电机运动时有效。~510 = 空载，接近 0 = 堵转。
    pub fn read_sg_result(&self, delay: &impl DelayUs) -> Option<u16> {
        self.read_reg(REG_SG_RESULT, delay)
            .map(|v| (v & 0x3FF) as u16)
    }

    /// 读取 IHOLD_IRUN 寄存器 (类型化)。
    pub fn read_ihold_irun(&self, delay: &impl DelayUs) -> Option<IholdIr> {
        self.read_reg(REG_IHOLD_IRUN, delay).map(IholdIr::from)
    }

    /// 清除 GSTAT 错误标志。
    ///
    /// 写入 `0x07` 清除 reset、drv_err、uv_cp 三个标志位。
    pub fn clear_gstat(&self, delay: &impl DelayUs) {
        self.write_reg(REG_GSTAT, 0x07, delay);
    }

    // ========================================================================
    // 配置方法
    // ========================================================================

    /// 完整配置 TMC2209 驱动器。
    ///
    /// 执行以下步骤:
    /// 1. 清除 GSTAT 错误标志
    /// 2. 配置 GCONF: StealthChop + UART 使能
    /// 3. 配置 IHOLD_IRUN: 运行/保持电流
    /// 4. 配置 TPOWERDOWN: 停止后电流降低延时
    /// 5. 配置 CHOPCONF: 微步细分 + 256 微步插值
    /// 6. 配置 PWMCONF: StealthChop 自动调谐
    /// 7. 配置 SGTHRS: StallGuard 堵转检测阈值
    /// 8. 配置 TCOOLTHRS: CoolStep/StallGuard 激活条件
    ///
    /// # 参数
    ///
    /// - `delay`: 延时源
    /// - `microstepping`: 微步细分数 (1/2/4/8/16/32/64/128/256)
    /// - `run_current`: 运行电流 IRUN (0–31)
    /// - `hold_current`: 保持电流 IHOLD (0–31)
    /// - `ihold_delay`: 电流过渡延时 (0–15)
    /// - `sg_threshold`: StallGuard 堵转阈值 (0–255, 0=禁用)
    ///
    /// # 示例
    ///
    /// ```ignore
    /// // 16 微步，运行电流 50%，保持电流 25%
    /// tmc.configure(&delay, 16, 16, 8, 10, 10);
    /// ```
    pub fn configure(
        &self,
        delay: &impl DelayUs,
        microstepping: u32,
        run_current: u8,
        hold_current: u8,
        ihold_delay: u8,
        sg_threshold: u32,
    ) {
        // 步骤 1: 清除 GSTAT
        self.clear_gstat(delay);
        delay.delay_ms(10);

        // 步骤 2: GCONF — StealthChop + UART 使能
        let gconf: u32 = (1 << 6) | (1 << 7);
        self.write_reg(REG_GCONF, gconf, delay);
        delay.delay_ms(10);

        // 步骤 3: IHOLD_IRUN — 电流设置
        let ihold_irun: u32 =
            (hold_current as u32) | ((run_current as u32) << 8) | ((ihold_delay as u32) << 16);
        self.write_reg(REG_IHOLD_IRUN, ihold_irun, delay);
        delay.delay_ms(10);

        // 步骤 4: TPOWERDOWN
        self.write_reg(REG_TPOWERDOWN, 20, delay);
        delay.delay_ms(10);

        // 步骤 5: CHOPCONF — 斩波器 + 微步配置
        let mres = microstepping_to_mres(microstepping);
        let chopconf: u32 = 3 | (4 << 4) | (1 << 7) | (1 << 15) | (mres << 24) | (1 << 28);
        self.write_reg(REG_CHOPCONF, chopconf, delay);
        delay.delay_ms(10);

        // 步骤 6: PWMCONF — StealthChop 自动调谐
        let pwmconf: u32 = (36 | (14 << 8)) | (1 << 18) | (1 << 19) | (8 << 24) | (12 << 28);
        self.write_reg(REG_PWMCONF, pwmconf, delay);
        delay.delay_ms(10);

        // 步骤 7: SGTHRS
        self.set_sg_threshold(sg_threshold, delay);
        delay.delay_ms(10);

        // 步骤 8: TCOOLTHRS — 任意速度下都激活
        self.write_reg(REG_TCOOLTHRS, 0xFFFFF, delay);
        delay.delay_ms(10);
    }

    /// 动态修改运行/保持电流。
    ///
    /// 可在电机运行时调用，无需停止。
    ///
    /// # 参数
    ///
    /// - `run_current`: 运行电流 IRUN (0–31)
    /// - `hold_current`: 保持电流 IHOLD (0–31)
    pub fn set_run_current(&self, run_current: u8, hold_current: u8, delay: &impl DelayUs) {
        let ihold_irun: u32 = (hold_current as u32)
            | ((run_current as u32) << 8)
            | ((IHOLD_DELAY_DEFAULT as u32) << 16);
        self.write_reg(REG_IHOLD_IRUN, ihold_irun, delay);
    }

    /// 动态修改 StallGuard 堵转阈值。
    ///
    /// - 值越大越灵敏 (越容易触发堵转)
    /// - 设为 0 禁用堵转检测
    /// - 触发条件: `SG_RESULT ≤ threshold × 2`
    pub fn set_sg_threshold(&self, threshold: u32, delay: &impl DelayUs) {
        self.write_reg(REG_SGTHRS, threshold, delay);
    }

    /// 设置 CoolStep / StallGuard 激活的速度阈值。
    ///
    /// StallGuard 激活条件: `TSTEP ≤ tcool_thrs`。
    /// 设为最大值 (`0xFFFFF`) 表示任何速度下都激活。
    pub fn set_tcool_threshold(&self, tcool_thrs: u32, delay: &impl DelayUs) {
        self.write_reg(REG_TCOOLTHRS, tcool_thrs, delay);
    }

    /// 设置停止后电流降低延时。
    ///
    /// # 参数
    ///
    /// - `time`: 延时值 (0 = 禁用掉电)
    ///
    /// 单位: 约 2^18 个内部时钟周期 (~2.6 ms/单位 @ 12 MHz 内部时钟)。
    pub fn set_power_down_delay(&self, time: u32, delay: &impl DelayUs) {
        self.write_reg(REG_TPOWERDOWN, time, delay);
    }

    /// 芯片版本号提取。
    ///
    /// 读取 IOIN 寄存器并提取 VERSION 字段。
    /// TMC2209 的版本号为 `0x21`。
    pub fn read_version(&self, delay: &impl DelayUs) -> Option<u8> {
        self.read_ioin(delay).map(|ioin| ioin.version())
    }

    /// 芯片身份验证。
    ///
    /// 读取 IOIN 并检查版本号是否为 `0x21` (TMC2209)。
    pub fn verify_chip(&self, delay: &impl DelayUs) -> bool {
        self.read_version(delay) == Some(0x21)
    }

    // ========================================================================
    // 内部辅助方法
    // ========================================================================

    /// 非阻塞单字节读取 (带超时)。
    ///
    /// 通过 [`UartBus::read_available`] 轮询 RX，每次尝试读取 1 字节。
    /// 在 `timeout_us` 微秒内超时则返回 `None`。
    fn read_byte_timeout(&self, delay: &impl DelayUs, timeout_us: u32) -> Option<u8> {
        let mut remaining = timeout_us;
        let mut buf = [0u8; 1];
        while remaining > 0 {
            match self.uart.read_available(&mut buf) {
                Ok(n) if n > 0 => return Some(buf[0]),
                _ => {}
            }
            delay.delay_us(POLL_INTERVAL_US);
            remaining = remaining.saturating_sub(POLL_INTERVAL_US);
        }
        None
    }

    /// 从 RX FIFO 中排出指定数量的字节 (丢弃回显)。
    ///
    /// TMC2209 单线半双工拓扑下，MCU 发送的数据会被自身 RX 接收，
    /// 产生回显字节。写入数据报或读请求后须丢弃与发送等量的回显。
    ///
    /// 带超时保护：若 `timeout_us` 微秒内未排完指定字节则提前返回，
    /// 避免因硬件异常导致死循环。
    fn drain_bytes(&self, count: usize, delay: &impl DelayUs, timeout_us: u32) {
        let mut buf = [0u8; 8];
        let mut drained = 0;
        let mut remaining = timeout_us;
        while drained < count && remaining > 0 {
            match self
                .uart
                .read_available(&mut buf[..(count - drained).min(8)])
            {
                Ok(n) if n > 0 => drained += n,
                Ok(0) => {
                    delay.delay_us(POLL_INTERVAL_US);
                    remaining = remaining.saturating_sub(POLL_INTERVAL_US);
                }
                _ => break,
            }
        }
    }
}

// ============================================================================
// 辅助函数
// ============================================================================

/// 将微步细分数转换为 MRES 编码。
///
/// 支持的微步值: 1, 2, 4, 8, 16, 32, 64, 128, 256。
/// 不支持的输入会回退为全步 (MRES=8)。
///
/// | 微步 | MRES |
/// |------|------|
/// | 256  | 0    |
/// | 128  | 1    |
/// | 64   | 2    |
/// | 32   | 3    |
/// | 16   | 4    |
/// | 8    | 5    |
/// | 4    | 6    |
/// | 2    | 7    |
/// | 1    | 8    |
#[inline]
pub fn microstepping_to_mres(microstepping: u32) -> u32 {
    for &(ms, mres) in &MRES_TABLE {
        if ms == microstepping {
            return mres;
        }
    }
    8 // 默认全步
}

/// 解析微步细分数，返回人类可读的字符串描述。
///
/// 用于调试和日志输出。
#[cfg(feature = "defmt")]
pub fn microstepping_label(microstepping: u32) -> &'static str {
    match microstepping {
        256 => "1/256",
        128 => "1/128",
        64 => "1/64",
        32 => "1/32",
        16 => "1/16",
        8 => "1/8",
        4 => "1/4",
        2 => "1/2",
        1 => "full-step",
        _ => "unknown",
    }
}
