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
// 数据类型
// ============================================================================

/// 电机静止时的行为模式 (对应 PWMCONF.freewheel)。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum StandstillMode {
    /// 正常操作
    Normal = 0b00,
    /// 续流 (所有 MOSFET 关断)
    Freewheeling = 0b01,
    /// 线圈通过低侧 MOSFET 短路 (被动制动)
    StrongBraking = 0b10,
    /// 线圈通过高侧 MOSFET 短路 (被动制动)
    Braking = 0b11,
}

impl StandstillMode {
    /// 转换为驱动寄存器中的二进制值。
    #[inline]
    pub const fn to_bits(self) -> u8 {
        self as u8
    }
}

impl From<u8> for StandstillMode {
    fn from(value: u8) -> Self {
        match value {
            0b00 => Self::Normal,
            0b01 => Self::Freewheeling,
            0b10 => Self::StrongBraking,
            0b11 => Self::Braking,
            _ => Self::Normal,
        }
    }
}

/// 微步分辨率描述。
///
/// 内部以"指数"表示：微步数 = 2^exponent。
/// 与驱动寄存器 MRES 的关系：`mres = 8 - exponent`。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct MicroStepResolution {
    exponent: u8,
}

impl MicroStepResolution {
    /// 从指数创建 (0 = 1 微步/全步, 8 = 256 微步)。
    #[inline]
    pub const fn from_exponent(exponent: u8) -> Self {
        assert!(exponent <= 8);
        Self { exponent }
    }

    /// 从微步数创建 (如 256, 128, 64, …, 1)。
    ///
    /// # Panics
    /// 如果 `microsteps` 不是 2 的幂或超过 256。
    #[inline]
    pub const fn from_microsteps(microsteps: u32) -> Self {
        assert!(microsteps.is_power_of_two() && microsteps <= 256);
        Self {
            exponent: microsteps.trailing_zeros() as u8,
        }
    }

    /// 指数值 (微步数 = 2^exponent)。
    #[inline]
    pub const fn exponent(self) -> u8 {
        self.exponent
    }

    /// 微步数量。
    #[inline]
    pub const fn microsteps(self) -> u32 {
        1 << self.exponent as u32
    }

    /// 对应的 MRES 驱动寄存器值。
    #[inline]
    pub const fn to_mres(self) -> u32 {
        (8 - self.exponent) as u32
    }

    /// 从 MRES 驱动寄存器值创建。
    #[inline]
    pub const fn from_mres(mres: u32) -> Self {
        Self {
            exponent: (8 - mres) as u8,
        }
    }
}

// ============================================================================
// 芯片参数
// ============================================================================

/// TMC2209 内部时钟频率 (Hz)。
///
/// 即数据手册中的 `f_CLK`。由出厂 OTP 校准至 12 MHz。
pub const INTERNAL_CLOCK_HZ: u32 = 12_000_000;

// ============================================================================
// TMC2209 寄存器地址
// ============================================================================

/// GCONF (0x00) — 全局配置寄存器
///
/// | 位 | 名称 | 说明 |
/// |----|------|------|
/// | 0 | i_scale_analog | 电流基准源选择 |
/// | 2 | en_spread_cycle | 0: StealthChop, 1: SpreadCycle |
/// | 6 | pdn_disable | PDN_UART 仅用于 UART 通信 (必须置 1) |
/// | 7 | mstep_reg_select | 微步来源: 0=引脚, 1=寄存器 |
/// | 8 | multistep_filt | STEP 脉冲边沿滤波 |
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
/// | 2 | MS1 | MS1 引脚状态 |
/// | 3 | MS2 | MS2 引脚状态 |
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
/// | [3:0] | toff | 斩波关断时间 |
/// | [6:4] | hstrt | 滞后起始值 |
/// | [10:7] | hend | 滞后结束值 |
/// | [16:15] | tbl | 消隐时间比较 |
/// | 17 | vsense | 检流电阻灵敏度 |
/// | [27:24] | mres | 微步分辨率 (0=256 ... 8=全步) |
/// | 28 | intpol | 256 微步插值启用 |
pub const REG_CHOPCONF: u8 = 0x6C;

/// DRV_STATUS (0x6F) — 驱动器状态 (只读)
///
/// | 位 | 名称 | 说明 |
/// |----|------|------|
/// | 0 | otpw | 过温预警 (~120°C) |
/// | 1 | ot | 过温关断 (~150°C) |
/// | [20:16] | cs_actual | 实际电流缩放值 |
/// | 30 | stealth | StealthChop 模式指示 |
/// | 31 | stst | 静止检测 |
pub const REG_DRV_STATUS: u8 = 0x6F;

/// PWMCONF (0x70) — StealthChop PWM 调谐参数
///
/// | 位 | 名称 | 说明 |
/// |----|------|------|
/// | [7:0] | pwm_ofs | PWM 初始偏移 |
/// | [15:8] | pwm_grad | PWM 梯度 |
/// | [17:16] | pwm_freq | PWM 频率 |
/// | 18 | pwm_autoscale | 自动调幅 |
/// | 19 | pwm_autograd | 自动调梯度 |
/// | [21:20] | freewheel | 静止模式 (IHOLD=0 时) |
/// | [27:24] | pwm_reg | 调节精度 |
/// | [31:28] | pwm_lim | PWM 限幅 |
pub const REG_PWMCONF: u8 = 0x70;

/// SLAVECONF (0x03) — 从机配置 (仅写)
///
/// | 位 | 名称 | 说明 |
/// |----|------|------|
/// | [11:8] | SENDDELAY | 读回复发送延时 |
pub const REG_SLAVECONF: u8 = 0x03;

/// TPWMTHRS (0x13) — StealthChop 切换到 SpreadCycle 的速度阈值
///
/// 当 `TSTEP < TPWMTHRS` (速度高于阈值) 时自动切换到 SpreadCycle。
/// 设为 0 则全程使用 StealthChop。
pub const REG_TPWMTHRS: u8 = 0x13;

/// VACTUAL (0x22) — UART 直接速度控制
///
/// 非零值时电机直接以给定速度运动，忽略 STEP 引脚输入。
/// 0 表示回到 STEP 引脚控制。内部 12 MHz 时钟下：
/// `VACTUAL ≈ microsteps_per_sec / 0.715`
pub const REG_VACTUAL: u8 = 0x22;

/// COOLCONF (0x42) — CoolStep 智能电流控制
///
/// CoolStep 根据 StallGuard 结果动态调节电机电流以节省能量。
pub const REG_COOLCONF: u8 = 0x42;

/// MSCNT (0x6A) — 微步计数器 (只读, 0–1023)
///
/// 指示电机在当前电气周期内的微步位置。
pub const REG_MSCNT: u8 = 0x6A;

/// MSCURACT (0x6B) — 当前 A/B 相微步电流 (只读)
///
/// 从正弦波表中读出的原始值 (未经电流缩放)，范围 -255~255。
pub const REG_MSCURACT: u8 = 0x6B;

/// PWM_SCALE (0x71) — StealthChop 自动调幅结果 (只读)
///
/// 可监控自动 PWM 幅度调节的状态。
pub const REG_PWM_SCALE: u8 = 0x71;

/// PWM_AUTO (0x72) — 自动 PWM 梯度/偏移值 (只读)
///
/// 可用于为 PWMCONF 确定上电默认参数。
pub const REG_PWM_AUTO: u8 = 0x72;

/// FACTORY_CONF (0x07) — 工厂配置
///
/// | 位 | 名称 | 说明 |
/// |----|------|------|
/// | [4:0] | FCLKTRIM | 时钟频率调节 (出厂 OTP 预设为 12 MHz) |
/// | [9:8] | OTTRIM | 过温阈值调节 |
pub const REG_FACTORY_CONF: u8 = 0x07;

/// OTP_PROG (0x04) — OTP 编程 (仅写, 一次性)
///
/// <div class=\"warning\">OTP = One Time Programmable, 只能写一次！</div>
pub const REG_OTP_PROG: u8 = 0x04;

/// OTP_READ (0x05) — OTP 读出 (只读)
pub const REG_OTP_READ: u8 = 0x05;

// ============================================================================
// 寄存器 bitfield 类型
// ============================================================================

/// GCONF (0x00) — 全局配置
///
/// | 位 | 名称 | 说明 |
/// |----|------|------|
/// | 0 | i_scale_analog | 0: 内部 VREF, 1: AIN 模拟电流基准 |
/// | 1 | internal_rsense | 0: 外部检流电阻, 1: 内部检流电阻 |
/// | 2 | en_spread_cycle | 0: StealthChop, 1: SpreadCycle |
/// | 3 | shaft | 反转电机方向 |
/// | 4 | index_otpw | INDEX 引脚输出过温预警 |
/// | 5 | index_step | INDEX 引脚输出步进脉冲 |
/// | 6 | pdn_disable | PDN_UART 仅用于 UART 通信 (必须置 1) |
/// | 7 | mstep_reg_select | 微步来源: 0=MS1/MS2 引脚, 1=CHOPCONF.mres |
/// | 8 | multistep_filt | STEP 脉冲边沿滤波 |
/// | 9 | test_mode | 测试模式 (正常操作必须为 0) |
#[bitfield(u32)]
pub struct Gconf {
    /// 0: 内部 VREF, 1: AIN 模拟电流基准
    #[bits(1)]
    pub i_scale_analog: bool,
    /// 0: 外部检流电阻, 1: 内部检流电阻
    #[bits(1)]
    pub internal_rsense: bool,
    /// 0: StealthChop, 1: SpreadCycle
    #[bits(1)]
    pub en_spread_cycle: bool,
    /// 反转电机方向
    #[bits(1)]
    pub shaft: bool,
    /// INDEX 引脚输出过温预警
    #[bits(1)]
    pub index_otpw: bool,
    /// INDEX 引脚输出步进脉冲
    #[bits(1)]
    pub index_step: bool,
    /// PDN_UART 仅用于 UART 通信 (必须置 1)
    #[bits(1)]
    pub pdn_disable: bool,
    /// 微步分辨率来源: 0=MS1/MS2 引脚, 1=CHOPCONF.mres
    #[bits(1)]
    pub mstep_reg_select: bool,
    /// STEP 脉冲边沿滤波
    #[bits(1)]
    pub multistep_filt: bool,
    /// 测试模式 (正常操作必须为 0)
    #[bits(1)]
    pub test_mode: bool,
    #[bits(22)]
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
    #[bits(1)]
    __: u8,
    /// MS1 引脚状态
    #[bits(1)]
    pub ms1: bool,
    /// MS2 引脚状态
    #[bits(1)]
    pub ms2: bool,
    /// DIAG 引脚状态
    #[bits(1)]
    pub diag: bool,
    #[bits(1)]
    __: u8,
    /// PDN_UART 引脚状态
    #[bits(1)]
    pub pdn_uart: bool,
    /// STEP 引脚状态
    #[bits(1)]
    pub step: bool,
    /// Spread 启用引脚状态
    #[bits(1)]
    pub spread_en: bool,
    /// DIR 引脚状态
    #[bits(1)]
    pub dir: bool,
    #[bits(14)]
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
    /// 斩波关断时间 (0=禁用驱动, 1–15)
    #[bits(4)]
    pub toff: u8,
    /// 滞后起始值 (1–8 加到 HEND)
    #[bits(3)]
    pub hstrt: u8,
    /// 滞后结束值 (−3..12)
    #[bits(4)]
    pub hend: u8,
    #[bits(4)]
    __: u8,
    /// 比较器消隐时间 (0=16, 1=24, 2=32, 3=40 clocks)
    #[bits(2)]
    pub tbl: u8,
    /// 检流电阻电压灵敏度 (0: 低灵敏度 V_FS=0.325V, 1: 高灵敏度 V_FS=0.180V)
    #[bits(1)]
    pub vsense: bool,
    #[bits(6)]
    __: u8,
    /// 微步分辨率 (0=256 ... 8=全步)
    #[bits(4)]
    pub mres: u8,
    /// 256 微步插值启用
    #[bits(1)]
    pub intpol: bool,
    /// 双边沿步进脉冲 (与 multistep_filt 互斥)
    #[bits(1)]
    pub dedge: bool,
    /// 禁用对地短路保护
    #[bits(1)]
    pub diss2g: bool,
    /// 禁用低侧短路保护
    #[bits(1)]
    pub diss2vs: bool,
}

/// DRV_STATUS (0x6F) — 驱动器状态 (只读)
#[bitfield(u32)]
pub struct DrvStatus {
    /// 过温预警 (~120°C)
    #[bits(1)]
    pub otpw: bool,
    /// 过温关断 (~150°C)
    #[bits(1)]
    pub ot: bool,
    /// A 相对地短路
    #[bits(1)]
    pub s2ga: bool,
    /// B 相对地短路
    #[bits(1)]
    pub s2gb: bool,
    /// A 相低侧短路
    #[bits(1)]
    pub s2vsa: bool,
    /// B 相低侧短路
    #[bits(1)]
    pub s2vsb: bool,
    /// A 相开路
    #[bits(1)]
    pub ola: bool,
    /// B 相开路
    #[bits(1)]
    pub olb: bool,
    /// 温度超过 120°C
    #[bits(1)]
    pub t120: bool,
    /// 温度超过 143°C
    #[bits(1)]
    pub t143: bool,
    /// 温度超过 150°C
    #[bits(1)]
    pub t150: bool,
    /// 温度超过 157°C
    #[bits(1)]
    pub t157: bool,
    #[bits(4)]
    __: u8,
    /// 实际电流缩放值 (0–31)
    #[bits(5)]
    pub cs_actual: u8,
    #[bits(5)]
    __: u8,
    #[bits(4)]
    __: u8,
    /// StealthChop 模式指示 (1=StealthChop, 0=SpreadCycle)
    #[bits(1)]
    pub stealth: bool,
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
    /// 静止模式 (仅当 IHOLD=0 时生效)
    #[bits(2)]
    pub freewheel: u8,
    #[bits(2)]
    __: u8,
    /// 调节精度
    #[bits(4)]
    pub pwm_reg: u8,
    /// PWM 限幅
    #[bits(4)]
    pub pwm_lim: u8,
}

/// TSTEP (0x12) — 步进间隔测量值 (只读)
///
/// 以 1/f_CLK 为单位，表示两次 1/256 微步之间的时间。
/// 值越小 = 速度越快。溢出或静止时为 `2^20 - 1`。
#[bitfield(u32)]
pub struct Tstep {
    /// 步进间隔 (0..1_048_575)
    #[bits(20)]
    pub tstep: u32,
    #[bits(12)]
    __: u32,
}

/// TPWMTHRS (0x13) — StealthChop 速度切换阈值
#[bitfield(u32)]
pub struct Tpwmthrs {
    /// 当 TSTEP 小于此值时切换到 SpreadCycle
    #[bits(20)]
    pub threshold: u32,
    #[bits(12)]
    __: u32,
}

/// TCOOLTHRS (0x14) — CoolStep/StallGuard 速度阈值
///
/// `TSTEP ≤ TCOOLTHRS` 时启用 CoolStep 和 DIAG 堵转输出。
#[bitfield(u32)]
pub struct Tcoolthrs {
    #[bits(20)]
    pub threshold: u32,
    #[bits(12)]
    __: u32,
}

/// VACTUAL (0x22) — UART 速度控制
///
/// 非零值时电机按此速度直接运动，忽略 STEP 引脚。
/// 有符号值：正 = 正转，负 = 反转。
#[bitfield(u32)]
pub struct Vactual {
    /// 电机速度 (有符号, 最大 ±2^23-1 微步/t)
    #[bits(24)]
    pub velocity: i32,
    #[bits(8)]
    __: u32,
}

/// SGTHRS (0x40) — StallGuard 堵转阈值
///
/// 触发条件: `SG_RESULT ≤ SGTHRS × 2`。
#[bitfield(u32)]
pub struct Sgthrs {
    /// 堵转阈值 (0–255, 0 = 禁用)
    #[bits(8)]
    pub threshold: u8,
    #[bits(24)]
    __: u32,
}

/// SG_RESULT (0x41) — StallGuard 结果 (只读, 0–510)
///
/// 值越大负载越轻 (~510 = 空载, ~0 = 堵转)。
#[bitfield(u32)]
pub struct SgResult {
    /// StallGuard 值
    #[bits(10)]
    pub result: u16,
    #[bits(22)]
    __: u32,
}

/// COOLCONF (0x42) — CoolStep 智能电流控制
#[bitfield(u32)]
pub struct Coolconf {
    /// 最小 StallGuard 值。低于此值则增加电流 (0–15, 0 = 禁用 CoolStep)
    #[bits(4)]
    pub semin: u8,
    #[bits(1)]
    __: u8,
    /// 电流增加步长: 0=1, 1=2, 2=4, 3=8
    #[bits(2)]
    pub seup: u8,
    #[bits(1)]
    __: u8,
    /// 电流减小上限 (0–15)。SG ≥ (SEMIN + SEMAX + 1)×32 时减小电流
    #[bits(4)]
    pub semax: u8,
    #[bits(1)]
    __: u8,
    /// 电流减小速度: 0=每32次, 1=每8次, 2=每2次, 3=每次
    #[bits(2)]
    pub sedn: u8,
    /// 最小电流: 0=IRUN/2, 1=IRUN/4
    #[bits(1)]
    pub seimin: bool,
    #[bits(16)]
    __: u32,
}

/// MSCNT (0x6A) — 微步计数器 (只读)
#[bitfield(u32)]
pub struct Mscnt {
    /// 当前微步位置 (0–1023)
    #[bits(10)]
    pub count: u16,
    #[bits(22)]
    __: u32,
}

/// MSCURACT (0x6B) — 当前 A/B 相微步电流 (只读)
#[bitfield(u32)]
pub struct Mscuract {
    /// B 相电流 (−255..255)
    #[bits(9)]
    pub cur_b: i16,
    #[bits(7)]
    __: u16,
    /// A 相电流 (−255..255)
    #[bits(9)]
    pub cur_a: i16,
    #[bits(7)]
    __: u16,
}

/// PWM_SCALE (0x71) — StealthChop 自动调幅结果 (只读)
#[bitfield(u32)]
pub struct PwmScale {
    /// 实际 PWM 占空比总和
    #[bits(8)]
    pub pwm_scale_sum: u8,
    #[bits(8)]
    __: u16,
    /// 自动幅度调节偏移 (−255..255)
    #[bits(9)]
    pub pwm_scale_auto: i16,
    #[bits(7)]
    __: u16,
}

/// PWM_AUTO (0x72) — 自动 PWM 梯度/偏移值 (只读)
///
/// 可用于为 PWMCONF 确定上电默认值。
#[bitfield(u32)]
pub struct PwmAuto {
    /// 自动确定的 PWM 偏移
    #[bits(8)]
    pub pwm_ofs_auto: u8,
    #[bits(8)]
    __: u16,
    /// 自动确定的 PWM 梯度
    #[bits(8)]
    pub pwm_grad_auto: u8,
    #[bits(8)]
    __: u16,
}

/// SLAVECONF (0x03) — 从机配置 (仅写)
#[bitfield(u32)]
pub struct Slaveconf {
    #[bits(8)]
    __: u32,
    /// 读回复发送延时 (0–15)，对应 1×8 到 15×8 bit times
    #[bits(4)]
    pub send_delay: u8,
    #[bits(20)]
    __: u32,
}

/// TPOWERDOWN (0x11) — 停止后电流降低延时
///
/// 单位: 约 2^18 个时钟周期。0 = 无延时立即掉电。
#[bitfield(u32)]
pub struct Tpowerdown {
    /// 延时值 (0–255)
    #[bits(8)]
    pub delay: u8,
    #[bits(24)]
    __: u32,
}

/// IFCNT (0x02) — UART 写入计数器 (只读)
#[bitfield(u32)]
pub struct Ifcnt {
    /// 写入计数 (0–255, 溢出回绕)
    #[bits(8)]
    pub count: u8,
    #[bits(24)]
    __: u32,
}

/// FACTORY_CONF (0x07) — 工厂配置
#[bitfield(u32)]
pub struct FactoryConf {
    /// 时钟频率调节 (0–31, 出厂 OTP 预设)
    #[bits(5)]
    pub fclktrim: u8,
    #[bits(3)]
    __: u8,
    /// 过温阈值调节
    #[bits(2)]
    pub ottrim: u8,
    #[bits(22)]
    __: u32,
}

/// OTP_READ (0x05) — OTP 只读
#[bitfield(u32)]
pub struct OtpRead {
    /// 出厂时钟频率调节值 (0–31)
    #[bits(5)]
    pub otp_fclktrim: u8,
    /// 过温阈值
    #[bits(1)]
    pub otp_ottrim: u8,
    /// 内部检流电阻默认值
    #[bits(1)]
    pub otp_internal_rsense: bool,
    /// 消隐时间默认值
    #[bits(1)]
    pub otp_tbl: u8,
    /// PWM_GRAD 默认值编码
    #[bits(4)]
    pub otp_pwm_grad: u8,
    /// pwm_autograd 默认值
    #[bits(1)]
    pub otp_pwm_autograd: bool,
    /// TPWM_THRS 默认值编码
    #[bits(3)]
    pub otp_tpwmthrs: u8,
    /// PWM_OFS 默认值 (0 = 36, 1 = 0)
    #[bits(1)]
    pub otp_pwm_ofs: u8,
    /// PWM_REG 默认值
    #[bits(1)]
    pub otp_pwm_reg: u8,
    /// PWM_FREQ 默认值
    #[bits(1)]
    pub otp_pwm_freq: u8,
    /// IHOLDDELAY 默认值编码
    #[bits(2)]
    pub otp_ihold_delay: u8,
    /// IHOLD 默认值编码
    #[bits(2)]
    pub otp_ihold: u8,
    /// 默认 chopper 模式 (1=SpreadCycle, 0=StealthChop)
    #[bits(1)]
    pub otp_en_spread_cycle: bool,
    #[bits(8)]
    __: u32,
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

    /// 读取 StallGuard 堵转阈值 (SGTHRS)。
    ///
    /// 堵转判定条件: `SG_RESULT ≤ SGTHRS × 2`。
    /// 设为 0 禁用堵转检测。
    pub fn read_sg_threshold(&self, delay: &impl DelayUs) -> Option<u16> {
        self.read_reg(REG_SGTHRS, delay).map(|v| v as u16)
    }

    /// 检测电机是否堵转（软件轮询方式）。
    ///
    /// 同时读取 SG_RESULT 和 SGTHRS，判断是否触发 StallGuard 堵转条件。
    /// 仅在电机运动时有效。
    ///
    /// 堵转判定: `SG_RESULT ≤ SGTHRS × 2`
    ///
    /// # 返回
    ///
    /// - `true`: 堵转
    /// - `false`: 正常运行，或通信失败（保守返回 false）
    ///
    /// # 示例
    ///
    /// ```ignore
    /// // 运动循环中轮询堵转状态
    /// if tmc.is_stalled(&delay) {
    ///     // 立即停止电机
    /// }
    /// ```
    ///
    /// # 硬件替代方案
    ///
    /// 如需避免软件轮询开销，可将 TMC2209 的 **DIAG 引脚** 连接到 MCU 的一个
    /// GPIO 中断引脚。DIAG 是开漏输出（需外部上拉），一旦发生堵转、驱动错误或
    /// 过温，引脚会被拉低。通过 MCU 外部中断可零延迟感知堵转事件。
    ///
    /// DIAG 引脚的堵转输出功能无需额外寄存器配置——只要 SGTHRS 和 TCOOLTHRS
    /// 已正确设置（[`configure`] 已包含），堵转时 DIAG 自动拉低。
    pub fn is_stalled(&self, delay: &impl DelayUs) -> bool {
        let sg_result = match self.read_sg_result(delay) {
            Some(v) => v,
            None => return false,
        };
        let sg_threshold = match self.read_sg_threshold(delay) {
            Some(v) => v,
            None => return false,
        };
        sg_result <= sg_threshold.saturating_mul(2)
    }

    /// 读取 IHOLD_IRUN 寄存器 (类型化)。
    pub fn read_ihold_irun(&self, delay: &impl DelayUs) -> Option<IholdIr> {
        self.read_reg(REG_IHOLD_IRUN, delay).map(IholdIr::from)
    }

    /// 写入 IHOLD_IRUN 寄存器。
    pub fn write_ihold_irun(&self, val: &IholdIr, delay: &impl DelayUs) {
        self.write_reg(REG_IHOLD_IRUN, (*val).into(), delay);
    }

    /// 写入 CHOPCONF 寄存器。
    pub fn write_chopconf(&self, val: &Chopconf, delay: &impl DelayUs) {
        self.write_reg(REG_CHOPCONF, (*val).into(), delay);
    }

    /// 写入 PWMCONF 寄存器。
    pub fn write_pwmconf(&self, val: &Pwmconf, delay: &impl DelayUs) {
        self.write_reg(REG_PWMCONF, (*val).into(), delay);
    }

    /// 读取 TSTEP 寄存器 (步进间隔，只读)。
    ///
    /// 值越小 = 速度越快。溢出或静止时为 `2^20 - 1`。
    /// 以 1/f_CLK 为单位，始终对应 1/256 微步。
    pub fn read_tstep(&self, delay: &impl DelayUs) -> Option<Tstep> {
        self.read_reg(REG_TSTEP, delay).map(Tstep::from)
    }

    /// 读取 PWM_SCALE 寄存器 (自动调幅结果，只读)。
    pub fn read_pwm_scale(&self, delay: &impl DelayUs) -> Option<PwmScale> {
        self.read_reg(REG_PWM_SCALE, delay).map(PwmScale::from)
    }

    /// 读取 PWM_AUTO 寄存器 (自动梯度/偏移值，只读)。
    pub fn read_pwm_auto(&self, delay: &impl DelayUs) -> Option<PwmAuto> {
        self.read_reg(REG_PWM_AUTO, delay).map(PwmAuto::from)
    }

    /// 读取 MSCNT 寄存器 (微步计数器，只读)。
    ///
    /// 返回当前电机在电气周期内的微步位置 (0–1023)。
    /// 可用于替代编码器做位置反馈。
    pub fn read_mscnt(&self, delay: &impl DelayUs) -> Option<Mscnt> {
        self.read_reg(REG_MSCNT, delay).map(Mscnt::from)
    }

    /// 读取 MSCURACT 寄存器 (A/B 相微步电流，只读)。
    pub fn read_mscuract(&self, delay: &impl DelayUs) -> Option<Mscuract> {
        self.read_reg(REG_MSCURACT, delay).map(Mscuract::from)
    }

    /// 读取 COOLCONF 寄存器 (CoolStep 配置，类型化)。
    pub fn read_coolconf(&self, delay: &impl DelayUs) -> Option<Coolconf> {
        self.read_reg(REG_COOLCONF, delay).map(Coolconf::from)
    }

    /// 写入 COOLCONF 寄存器。
    pub fn write_coolconf(&self, val: &Coolconf, delay: &impl DelayUs) {
        self.write_reg(REG_COOLCONF, (*val).into(), delay);
    }

    /// 读取 FACTORY_CONF 寄存器 (类型化)。
    pub fn read_factory_conf(&self, delay: &impl DelayUs) -> Option<FactoryConf> {
        self.read_reg(REG_FACTORY_CONF, delay)
            .map(FactoryConf::from)
    }

    /// 读取 OTP_READ 寄存器 (类型化)。
    pub fn read_otp(&self, delay: &impl DelayUs) -> Option<OtpRead> {
        self.read_reg(REG_OTP_READ, delay).map(OtpRead::from)
    }

    /// 读取 IFCNT (类型化)。
    pub fn read_ifcnt_typed(&self, delay: &impl DelayUs) -> Option<Ifcnt> {
        self.read_reg(REG_IFCNT, delay).map(Ifcnt::from)
    }

    /// 清除 GSTAT 错误标志。
    ///
    /// 写入 `0x07` 清除 reset、drv_err、uv_cp 三个标志位。
    pub fn clear_gstat(&self, delay: &impl DelayUs) {
        self.write_reg(REG_GSTAT, 0x07, delay);
    }

    /// 清除驱动错误标志 (GSTAT.drv_err)。
    ///
    /// GSTAT 是 R+WC 寄存器，写入 bit 为 1 以清除对应标志。
    /// 仅在驱动错误条件已消除时才能成功清除。
    pub fn clear_drv_err(&self, delay: &impl DelayUs) {
        let mut gstat = Gstat::default();
        gstat.set_drv_err(true);
        self.write_reg(REG_GSTAT, gstat.into(), delay);
    }

    // ========================================================================
    // 配置方法
    // ========================================================================

    /// 完整配置 TMC2209 驱动器。
    ///
    /// 执行以下步骤:
    /// 1. 清除 GSTAT 错误标志
    /// 2. 配置 GCONF: StealthChop + UART 启用 + 寄存器微步
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

        // 步骤 2: GCONF — StealthChop + UART 模式 + 寄存器微步
        let mut gconf = Gconf::default();
        gconf.set_pdn_disable(true); // UART 通信必须
        gconf.set_mstep_reg_select(true); // 微步由 CHOPCONF.mres 控制
        gconf.set_multistep_filt(true); // STEP 脉冲滤波
        self.write_gconf(&gconf, delay);
        delay.delay_ms(10);

        // 步骤 3: IHOLD_IRUN — 电流设置
        let mut ihold_irun = IholdIr::default();
        ihold_irun.set_ihold(hold_current);
        ihold_irun.set_irun(run_current);
        ihold_irun.set_iholddelay(ihold_delay);
        self.write_ihold_irun(&ihold_irun, delay);
        delay.delay_ms(10);

        // 步骤 4: TPOWERDOWN
        self.write_reg(REG_TPOWERDOWN, 20, delay);
        delay.delay_ms(10);

        // 步骤 5: CHOPCONF — 斩波器 + 微步配置
        let mut chopconf = Chopconf::default();
        chopconf.set_toff(3);
        chopconf.set_hstrt(4);
        chopconf.set_hend(1);
        chopconf.set_tbl(1); // 24 clocks blanking
        chopconf.set_mres(microstepping_to_mres(microstepping) as u8);
        chopconf.set_intpol(true);
        self.write_chopconf(&chopconf, delay);
        delay.delay_ms(10);

        // 步骤 6: PWMCONF — StealthChop 自动调谐
        let mut pwmconf = Pwmconf::default();
        pwmconf.set_pwm_ofs(36);
        pwmconf.set_pwm_grad(14);
        pwmconf.set_pwm_freq(0); // 0b00: fPWM = 2/1024 fCLK
        pwmconf.set_pwm_autoscale(true);
        pwmconf.set_pwm_autograd(true);
        pwmconf.set_pwm_reg(8);
        pwmconf.set_pwm_lim(12);
        self.write_pwmconf(&pwmconf, delay);
        delay.delay_ms(10);

        // 步骤 7: SGTHRS
        self.set_sg_threshold(sg_threshold, delay);
        delay.delay_ms(10);

        // 步骤 8: TCOOLTHRS — 只在 ≥~500 Hz 时激活 StallGuard
        // 避免 StealthChop 刚起步电流自整定未稳定时误判堵转。
        // TSTEP = fCLK / (speed_hz × 256)，设 TCOOLTHRS=100 → 激活速度 ≈ 470 Hz。
        self.write_reg(REG_TCOOLTHRS, 100, delay);
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
        let mut ihold_irun = IholdIr::default();
        ihold_irun.set_ihold(hold_current);
        ihold_irun.set_irun(run_current);
        ihold_irun.set_iholddelay(IHOLD_DELAY_DEFAULT);
        self.write_ihold_irun(&ihold_irun, delay);
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

    // ========================================================================
    // CoolStep 配置
    // ========================================================================

    /// 配置 CoolStep 智能电流控制。
    ///
    /// CoolStep 根据电机负载 (由 StallGuard 检测) 自动调节电流：
    /// - 负载轻时降低电流以节省能量
    /// - 负载重时增加电流以防止丢步
    ///
    /// 该功能仅在 StealthChop 模式下有效，且要求电机速度高于 TCOOLTHRS 阈值。
    ///
    /// # 参数
    ///
    /// - `semin`: 最小 StallGuard 阈值 (0–15)。SG 低于此值时增加电流。0 = 禁用 CoolStep。
    /// - `semax`: 最大 StallGuard 阈值 (0–15)。SG 高于 (SEMIN+SEMAX+1)×32 时降低电流。
    /// - `seup`: 电流增加步长 (0=1, 1=2, 2=4, 3=8)。值越小越平滑。
    /// - `sedn`: 电流降低速度 (0=每32次, 1=每8次, 2=每2次, 3=每次)。值越小越平滑。
    /// - `seimin`: 最小电流限制。`false` = IRUN/2, `true` = IRUN/4。
    ///
    /// # 建议起始值
    ///
    /// ```ignore
    /// tmc.configure_coolstep(&delay, 5, 2, 0, 0, false);
    /// ```
    pub fn configure_coolstep(
        &self,
        delay: &impl DelayUs,
        semin: u8,
        semax: u8,
        seup: u8,
        sedn: u8,
        seimin: bool,
    ) {
        let mut coolconf = Coolconf::default();
        coolconf.set_semin(semin);
        coolconf.set_semax(semax);
        coolconf.set_seup(seup);
        coolconf.set_sedn(sedn);
        coolconf.set_seimin(seimin);
        self.write_coolconf(&coolconf, delay);
    }

    // ========================================================================
    // 速度控制 (VACTUAL)
    // ========================================================================

    /// 通过 UART 直接设置电机速度 (忽略 STEP 引脚)。
    ///
    /// 这是 TMC2209 的 UART 速度模式：写入非零值后电机自动按该速度运动，
    /// 无需外部脉冲。写入 0 则回到 STEP 引脚控制。
    ///
    /// # 参数
    ///
    /// - `velocity`: 速度值。内部 12 MHz 时钟下：
    ///   `VACTUAL ≈ microsteps_per_sec / 0.715`
    ///   正值 = 正转，负值 = 反转，0 = 停止并回到 STEP 控制。
    ///
    /// # 示例
    ///
    /// ```ignore
    /// // 10000 微步/秒 = 约 VACTUAL 13986
    /// tmc.set_vactual(13986, &delay);
    /// ```
    pub fn set_vactual(&self, velocity: i32, delay: &impl DelayUs) {
        let val = Vactual::new().with_velocity(velocity);
        self.write_reg(REG_VACTUAL, val.into(), delay);
    }

    /// 停止 UART 速度模式，回到 STEP 引脚控制。
    pub fn stop_vactual(&self, delay: &impl DelayUs) {
        self.write_reg(REG_VACTUAL, 0, delay);
    }

    // ========================================================================
    // 静止行为 (Freewheel)
    // ========================================================================

    /// 设置电机静止时的行为 (PWMCONF.freewheel)。
    ///
    /// 仅在 IHOLD=0 时生效。可选择续流或被动制动。
    pub fn set_standstill_mode(&self, mode: StandstillMode, delay: &impl DelayUs) {
        if let Some(mut pwm) = self.read_pwmconf(delay) {
            pwm.set_freewheel(mode.to_bits());
            self.write_pwmconf(&pwm, delay);
        }
    }

    /// 设置 StealthChop 切换到 SpreadCycle 的速度阈值。
    ///
    /// 当电机速度快于此阈值 (`TSTEP < TPWMTHRS`) 时自动切换到 SpreadCycle。
    /// 设为 0 则全程使用 StealthChop。
    pub fn set_tpwmthrs(&self, threshold: u32, delay: &impl DelayUs) {
        self.write_reg(REG_TPWMTHRS, threshold & 0xFFFFF, delay);
    }

    /// 设置 SLAVECONF 的 send_delay。
    ///
    /// 在多从机总线上可能需要调整读回复延时。
    ///
    /// | 值 | bit times |
    /// |----|-----------|
    /// | 0–1 | 1×8  |
    /// | 2–3 | 3×8  |
    /// | …   | …     |
    /// | 14–15 | 15×8 |
    pub fn set_send_delay(&self, delay_val: u8, delay: &impl DelayUs) {
        let val = Slaveconf::default().with_send_delay(delay_val);
        self.write_reg(REG_SLAVECONF, val.into(), delay);
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
    /// 通过 [`UartBus::read_raw`] 轮询 RX，每次尝试读取 1 字节。
    /// 在 `timeout_us` 微秒内超时则返回 `None`。
    ///
    /// 使用 `read_raw` 而非 `read_available` 以忽略帧错误标志：
    /// 单线半双工电阻网络下回显和 TMC2209 回复都可能触发虚假的 FE。
    fn read_byte_timeout(&self, delay: &impl DelayUs, timeout_us: u32) -> Option<u8> {
        let mut remaining = timeout_us;
        let mut buf = [0u8; 1];
        while remaining > 0 {
            let n = self.uart.read_raw(&mut buf);
            if n > 0 {
                return Some(buf[0]);
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
    ///
    /// 使用 `read_raw` 忽略帧错误：电阻网络下回显可能带有
    /// 虚假的 framing error，若用 `read_available` 会提前退出导致
    /// 回显残留并污染后续 TMC2209 回复帧。
    fn drain_bytes(&self, count: usize, delay: &impl DelayUs, timeout_us: u32) {
        let mut buf = [0u8; 8];
        let mut drained = 0;
        let mut remaining = timeout_us;
        while drained < count && remaining > 0 {
            let n = self.uart.read_raw(&mut buf[..(count - drained).min(8)]);
            if n > 0 {
                drained += n;
            } else {
                delay.delay_us(POLL_INTERVAL_US);
                remaining = remaining.saturating_sub(POLL_INTERVAL_US);
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

// ============================================================================
// 电流计算
// ============================================================================

/// 根据目标 RMS 电流和检流电阻计算 vsense 和 IRUN 设置值。
///
/// 这是 [`Tmc2209::set_run_current`] 的高级替代——直接由物理参数计算
/// 最优的寄存器写入值。
///
/// ## 推导
///
/// TMC2209 数据手册电流公式 (vsense=0, V_FS=0.325V):
///
/// ```text
/// I_RMS = (CS + 1) / 32 × V_FS / (RSENSE + 0.02) / √2
/// ```
///
/// 反解 CS:
///
/// ```text
/// CS = 32 × √2 × I_RMS × (RSENSE + 0.02) / V_FS - 1
/// ```
///
/// 其中 `+0.02` 为 TMC2209 内部 20 mΩ 寄生电阻。
/// 若 CS < 16 则自动切换到高灵敏度模式 (vsense=1, V_FS=0.180V)
/// 重新计算以获得更高精度。
///
/// # 参数
///
/// - `rsense`: 外部检流电阻值 (Ω)。例如 0.11Ω → `0.11`。
/// - `milliamps`: 目标 RMS 电流 (mA)。例如 800 mA → `800.0`。
///
/// # 返回
///
/// `(vsense, current_scale)`:
/// - `vsense`: `true` = 高灵敏度模式 (V_FS=0.180V)，写入 `CHOPCONF.vsense`
/// - `current_scale`: IRUN/IHOLD 的写入值 (0–31)
///
/// # 检流电阻选择参考
///
/// | RSENSE (Ω) | 最大 RMS 电流 (IRUN=31, vsense=0) |
/// |------------|-------------------------------------|
/// | 0.10       | ~1.92 A (适合 2A 电机)             |
/// | 0.11       | ~1.77 A (适合 1.5A 电机)           |
/// | 0.22       | ~0.96 A (适合 1A 电机)             |
/// | 0.33       | ~0.66 A (适合 700mA 电机)          |
/// | 0.47       | ~0.47 A (适合 500mA 电机)          |
///
/// # 示例
///
/// ```
/// use pico_rs_drivers::tmc2209::rms_current_to_vsense_cs;
///
/// // 0.11Ω 检流电阻, 800mA RMS → vsense=false, cs≈13
/// let (vsense, cs) = rms_current_to_vsense_cs(0.11, 800.0);
/// ```
pub fn rms_current_to_vsense_cs(rsense: f32, milliamps: f32) -> (bool, u8) {
    let amps = milliamps / 1000.0;

    // 标准模式: V_FS = 0.325V
    let mut cs = (32.0 * core::f32::consts::SQRT_2 * amps * (rsense + 0.02) / 0.325 - 1.0) as u8;

    // 如果 CS 太小，切换到高灵敏度模式
    if cs < 16 {
        cs = (32.0 * core::f32::consts::SQRT_2 * amps * (rsense + 0.02) / 0.180 - 1.0) as u8;
        (true, cs.min(31))
    } else {
        (false, cs.min(31))
    }
}

/// 将 CS (IRUN/IHOLD 寄存器值) 和 vsense 设置转换回 RMS 电流 (mA)。
///
/// 用于从 `DRV_STATUS.cs_actual` 反算实际电机电流。
///
/// ## 推导
///
/// 数据手册公式正向:
///
/// ```text
/// I_RMS = (CS + 1) / 32 × V_FS / (RSENSE + 0.02) / √2
/// ```
///
/// 其中 V_FS = 0.325V (vsense=0) 或 0.180V (vsense=1)。
/// 返回值乘以 1000 转换为 mA。
///
/// # 参数
///
/// - `rsense`: 外部检流电阻值 (Ω)
/// - `vsense`: 高灵敏度模式 (`CHOPCONF.vsense`)
/// - `cs`: 电流缩放值 (0–31)
///
/// # 示例
///
/// ```
/// use pico_rs_drivers::tmc2209::vsense_cs_to_rms_current;
///
/// let ma = vsense_cs_to_rms_current(0.11, false, 13);
/// // ma ≈ 800.0
/// ```
pub fn vsense_cs_to_rms_current(rsense: f32, vsense: bool, cs: u8) -> f32 {
    let v_fs = if vsense { 0.180 } else { 0.325 };
    (cs as f32 + 1.0) / 32.0 * v_fs / (rsense + 0.02) / core::f32::consts::SQRT_2 * 1000.0
}

// ============================================================================
// 斩波频率计算
// ============================================================================

/// 将斩波频率 (Hz) 转换为 TOFF 寄存器值。
///
/// ## 推导
///
/// TMC2209 斩波器周期由 off-time 决定：
///
/// ```text
/// t_OFF = (TOFF × 32 + 24) / f_CLK
/// f_CHOP = 1 / (4 × t_OFF) = f_CLK / (4 × (TOFF × 32 + 24))
/// ```
///
/// 反解 TOFF:
///
/// ```text
/// TOFF = (f_CLK / (4 × f_CHOP) - 24) / 32
/// ```
///
/// 代码实现: `(fclk / (hz * 4.0) - 24.0) / 32.0`
///
/// TMC2209 仅支持离散的 15 个斩波频率，范围约 5.95 kHz ~ 53.57 kHz。
/// 此函数返回最接近的 TOFF 值。
///
/// # 返回
///
/// `Some(toff)` 如果找到有效值 (1–15)，`None` 如果频率太高 (需要 TOFF=0，会禁用驱动)。
///
/// # 示例
///
/// ```
/// use pico_rs_drivers::tmc2209::{chopper_hz_to_toff, INTERNAL_CLOCK_HZ};
///
/// // 约 25 kHz 斩波频率
/// let toff = chopper_hz_to_toff(INTERNAL_CLOCK_HZ as f32, 25_000.0);
/// assert_eq!(toff, Some(3));
/// ```
pub fn chopper_hz_to_toff(fclk: f32, hz: f32) -> Option<u8> {
    // TOFF = (f_CLK / (4 × f_CHOP) - 24) / 32
    let toff_f = (fclk / (hz * 4.0) - 24.0) / 32.0;
    let toff = (round_f32(toff_f) as u8).min(15);
    if toff == 0 { None } else { Some(toff) }
}

/// 将 TOFF 寄存器值转换为对应的斩波频率 (Hz)。
///
/// ## 推导
///
/// ```text
/// f_CHOP = f_CLK / (4 × (TOFF × 32 + 24))
/// ```
///
/// # 返回
///
/// `Some(hz)` 如果 TOFF 在有效范围 (1–15)，否则 `None`。
///
/// # 示例
///
/// ```
/// use pico_rs_drivers::tmc2209::{toff_to_chopper_hz, INTERNAL_CLOCK_HZ};
///
/// // TOFF=3 → f_CHOP = 12e6 / (4 × (3×32+24)) = 12e6 / 480 = 25 kHz
/// let hz = toff_to_chopper_hz(INTERNAL_CLOCK_HZ as f32, 3);
/// assert!((hz.unwrap() - 25_000.0).abs() < 1.0);
/// ```
pub fn toff_to_chopper_hz(fclk: f32, toff: u8) -> Option<f32> {
    if !(1..=15).contains(&toff) {
        return None;
    }
    let toff_secs = (toff as f32 * 32.0 + 24.0) / fclk;
    Some(1.0 / toff_secs / 4.0)
}

/// 将微步/秒速度转换为 VACTUAL 寄存器值。
///
/// ## 推导
///
/// TMC2209 数据手册 VACTUAL 速度公式:
///
/// ```text
/// v[µSteps/s] = VACTUAL × f_CLK / 2^24
/// ```
///
/// 其中 f_CLK = 12 MHz (内部时钟)。反解:
///
/// ```text
/// VACTUAL = v[µSteps/s] × 2^24 / f_CLK
///         = v[µSteps/s] × 16_777_216 / 12_000_000
///         = v[µSteps/s] / 0.715255737...
/// ```
///
/// 常量 `0.7152557` = f_CLK / 2^24，截断至 7 位小数，
/// 精度误差 < 0.04 µSteps/s。
///
/// # 示例
///
/// ```
/// use pico_rs_drivers::tmc2209::microsteps_per_sec_to_vactual;
///
/// // 10000 微步/秒 → VACTUAL ≈ 13986
/// // 校验: 13986 × 0.7152557 ≈ 10000.0 ✓
/// let vactual = microsteps_per_sec_to_vactual(10_000);
/// ```
#[inline]
pub fn microsteps_per_sec_to_vactual(microsteps_per_sec: i32) -> i32 {
    round_f32((microsteps_per_sec as f32) / 0.7152557) as i32
}

/// 将 VACTUAL 寄存器值转换为微步/秒速度。
///
/// ## 推导
///
/// ```text
/// v[µSteps/s] = VACTUAL × f_CLK / 2^24
///             = VACTUAL × 0.715255737...
/// ```
#[inline]
pub fn vactual_to_microsteps_per_sec(vactual: i32) -> f32 {
    (vactual as f32) * 0.7152557
}

// ============================================================================
// 内部辅助函数
// ============================================================================

/// `no_std` 友好的 f32 四舍五入 (round half away from zero)。
///
/// 通过 `±0.5` 偏移后向零截断实现标准舍入：
///
/// ```text
/// f ≥ 0: round(f) = trunc(f + 0.5)
/// f < 0: round(f) = trunc(f - 0.5)
/// ```
///
/// 验证:
///
/// ```text
/// f =  1.2 → trunc( 1.7) =  1.0 ✓
/// f =  1.5 → trunc( 2.0) =  2.0 ✓
/// f = -1.2 → trunc(-1.7) = -1.0 ✓
/// f = -1.5 → trunc(-2.0) = -2.0 ✓ (half away from zero)
/// ```
#[inline]
fn round_f32(f: f32) -> f32 {
    if f >= 0.0 {
        (f + 0.5) as i32 as f32
    } else {
        (f - 0.5) as i32 as f32
    }
}
