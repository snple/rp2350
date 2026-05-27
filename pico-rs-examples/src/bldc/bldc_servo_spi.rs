//! # SPI 从机电流模式伺服 — 寄存器接口 + 位置/角度/步进控制 + Id/Iq 电流闭环
//!
//! 基于 RP2350 (Raspberry Pi Pico 2) 的**电流模式无刷伺服控制器**，
//! 通过 **SPI Slave 寄存器接口**接收外部主控制器的位置/角度/步进指令，
//! 并支持运行时修改 PID/PI 参数。
//!
//! 本方案将无刷电机变为一个"智能执行器"，主控制器只需通过 SPI
//! 发送目标角度或步数，伺服自主完成**四环级联 FOC 闭环控制**。
//!
//! 控制链路：**位置 PID → 速度 PI → Id/Iq 双电流 PI → 逆 Park + SVPWM**
//!
//! 三种指令模式：
//! - **绝对位置** — 直接设编码器脉冲目标（高精度）
//! - **绝对角度** — 设目标角度 °（0.1° 分辨率），自动转换为脉冲
//! - **相对步进** — 类似步进电机，指定前进/后退步数
//!
//! 其他驱动模式见：
//! - `bldc_servo_i2c.rs` — I2C 从机电流模式伺服（寄存器接口）
//! - `bldc_servo.rs` — 电压模式伺服（演示序列版）
//! - `bldc_servo_current.rs` — 电流模式伺服（四环级联，演示序列版）
//! - `bldc_foc.rs` / `bldc_foc_current.rs` — 速度控制 FOC
//!
//! ---
//!
//! # 1. SPI 寄存器映射
//!
//! 寄存器空间 64 字节，与 I2C 版本 (`bldc_servo_i2c.rs`) **完全一致**，
//! 可直接复用主控端寄存器定义。
//!
//! | 偏移 | 名称 | R/W | 字节 | 说明 |
//! |------|------|-----|------|------|
//! | 0x00 | `TARGET_POS` | R/W | 4 | 目标位置 (i32 LE, 编码器脉冲) |
//! | 0x04 | `TARGET_ANGLE` | W | 4 | 目标角度 ×10 (i32 LE, 0.1° 单位，写入自动换算脉冲) |
//! | 0x08 | `REL_STEPS` | W | 4 | 相对步进 (i32 LE, 写入加到当前目标，自动清零) |
//! | 0x0C | `ACTUAL_POS` | R | 4 | 当前位置 (i32 LE, 编码器脉冲) |
//! | 0x10 | `ACTUAL_ANGLE` | R | 4 | 当前角度 ×10 (i32 LE, 0.1° 单位) |
//! | 0x14 | `ACTUAL_RPM` | R | 4 | 当前转速 (i32 LE) |
//! | 0x18 | `STATUS` | R | 1 | 状态位: bit0=到位, bit1=运动中 |
//! | 0x19 | `CMD` | W | 1 | 命令: 0x01=归零, 0x02=急停 (执行后自动清零) |
//! | 0x1A | `ACTUAL_ID` | R | 2 | 实测 Id 电流 (i16 LE, mA) |
//! | 0x1C | `ACTUAL_IQ` | R | 2 | 实测 Iq 电流 (i16 LE, mA) |
//! | 0x1E | `ACTUAL_IA` | R | 2 | 实测 Ia 相电流 (i16 LE, mA) |
//! | 0x20 | `KP_POS` | R/W | 2 | 位置 Kp (i16 LE, Q10 定点) |
//! | 0x22 | `KI_POS` | R/W | 2 | 位置 Ki (i16 LE, Q10) |
//! | 0x24 | `KD_POS` | R/W | 2 | 位置 Kd (i16 LE, Q10) |
//! | 0x26 | `KP_SPEED` | R/W | 2 | 速度 Kp (i16 LE, Q10) |
//! | 0x28 | `KI_SPEED` | R/W | 2 | 速度 Ki (i16 LE, Q10) |
//! | 0x2A | `KP_ID` | R/W | 2 | Id 电流 Kp (i16 LE, Q10) |
//! | 0x2C | `KI_ID` | R/W | 2 | Id 电流 Ki (i16 LE, Q10) |
//! | 0x2E | `KP_IQ` | R/W | 2 | Iq 电流 Kp (i16 LE, Q10) |
//! | 0x30 | `KI_IQ` | R/W | 2 | Iq 电流 Ki (i16 LE, Q10) |
//! | 0x32 | `MAX_RPM` | R/W | 2 | 最大转速 (i16 LE, RPM) |
//! | 0x34 | `IQ_REF_MAX` | R/W | 2 | Iq 参考最大值 (i16 LE, mA) |
//! | 0x36 | `U_MAX` | R/W | 2 | 电流 PI 电压输出上限 (i16 LE, Q15) |
//! | 0x38 | `DEADBAND` | R/W | 2 | 位置死区 (i16 LE, 编码器脉冲) |
//! | 0x3A | `CPR` | R | 2 | 编码器 CPR (u16 LE, 只读) |
//! | 0x3C | `FW_VER` | R | 2 | 固件版本 (u16 LE, 0x0201 = v2.1-SPI) |
//! | 0x3E | — | — | 2 | 保留 |
//!
//! ---
//!
//! # 2. SPI 通信协议
//!
//! ## 2.1 SPI 模式
//!
//! - **SPI Mode 0** (CPOL=0, CPHA=0)
//! - **8-bit 字长**
//! - 时钟频率由主机决定，从机自动跟随
//! - **CSn 低电平**选中从机，高电平结束事务
//!
//! ## 2.2 帧格式
//!
//! 每次 SPI 事务的**第一个字节**为命令字节：
//!
//! ```text
//! 命令字节:  [R/W | A6 A5 A4 A3 A2 A1 A0]
//!             bit7   bit6 ─────────── bit0
//!
//!   bit7 = 0 → 写寄存器 (Write)
//!   bit7 = 1 → 读寄存器 (Read)
//!   bit6:0   → 起始寄存器地址 (0x00 ~ 0x3F)
//! ```
//!
//! ## 2.3 写寄存器
//!
//! ```text
//! CSn ─┐                                        ┌── CSn
//!      └────────────────────────────────────────┘
//! MOSI:  [0x00|addr] [data0] [data1] [data2] [data3]
//! MISO:  (ignored)   (ignored) ...
//!
//! 示例：写目标角度 180.0° (= 1800 = 0x00000708, addr=0x04)
//!   MOSI: [0x04] [0x08] [0x07] [0x00] [0x00]
//!         cmd     1800 (i32 LE)
//! ```
//!
//! 地址自动递增，可连续写多个寄存器。CSn 上升沿触发写入处理。
//!
//! ## 2.4 读寄存器
//!
//! ```text
//! CSn ─┐                                        ┌── CSn
//!      └────────────────────────────────────────┘
//! MOSI:  [0x80|addr] [dummy] [dummy] [dummy] [dummy]
//! MISO:  (garbage)   [data0] [data1] [data2] [data3]
//!
//! 示例：读当前位置 (4 字节，addr=0x0C)
//!   MOSI: [0x8C] [0x00] [0x00] [0x00] [0x00]
//!   MISO: [  ?  ] [pos_b0] [pos_b1] [pos_b2] [pos_b3]
//!         忽略    i32 LE
//! ```
//!
//! 注意：读操作的**第一个 MISO 字节**是垃圾数据（命令字节传输时从机尚未
//! 准备好数据），从**第二个字节**开始才是有效的寄存器数据。
//!
//! ## 2.5 与 I2C 版本的区别
//!
//! | 特性 | I2C (`bldc_servo_i2c.rs`) | SPI（本文件） |
//! |------|---------------------------|---------------|
//! | 速度 | 100/400 kHz 标准/快速 | 可达数 MHz |
//! | 线数 | 2 线 (SDA + SCL) | 4 线 (SCK + MOSI + MISO + CSn) |
//! | 总线拓扑 | 多主多从 | 单主多从（每从设备一根 CSn） |
//! | 地址 | 7-bit 地址 (0x2A) | CSn 引脚选择 |
//! | 全双工 | 半双工 | 全双工 |
//! | 寄存器映射 | 完全一致 | 完全一致（64 字节） |
//! | 读操作首字节 | 有效 | 垃圾（需丢弃） |
//!
//! ---
//!
//! # 3. 三种指令模式
//!
//! ## 3.1 绝对位置模式（编码器脉冲）
//!
//! 写 `TARGET_POS` (0x00)，直接设定编码器脉冲目标：
//! ```text
//! 目标 2000 脉冲 → SPI 写: [0x00] [0xD0] [0x07] [0x00] [0x00]
//! ```
//!
//! ## 3.2 绝对角度模式（0.1° 分辨率）
//!
//! 写 `TARGET_ANGLE` (0x04)，单位为 0.1°，自动转换为脉冲：
//! ```text
//! 目标 180.0° → SPI 写: [0x04] + 1800 (i32 LE)
//! 内部转换: pos = 1800 × CPR / 3600 = 1800 × 4000 / 3600 = 2000 脉冲
//! ```
//!
//! ## 3.3 相对步进模式（类似步进电机）
//!
//! 写 `REL_STEPS` (0x08)，加到当前目标位置：
//! ```text
//! 前进 500 步 → SPI 写: [0x08] + 500 (i32 LE)
//! 后退 1000 步 → SPI 写: [0x08] + -1000 (i32 LE)
//! ```
//!
//! 写入后 `REL_STEPS` 自动清零，可连续发送步进指令。
//!
//! ---
//!
//! # 4. 主控制器示例代码
//!
//! ## 4.1 MicroPython (Raspberry Pi Pico / ESP32)
//!
//! ```python
//! from machine import SPI, Pin
//! import struct
//!
//! spi = SPI(0, baudrate=1_000_000, polarity=0, phase=0,
//!           sck=Pin(2), mosi=Pin(3), miso=Pin(0))
//! cs = Pin(1, Pin.OUT, value=1)
//!
//! def write_reg(addr, data):
//!     cs.value(0)
//!     spi.write(bytes([addr & 0x7F]) + data)
//!     cs.value(1)
//!
//! def read_reg(addr, n):
//!     cs.value(0)
//!     spi.write(bytes([addr | 0x80]))
//!     result = spi.read(n)
//!     cs.value(1)
//!     return result
//!
//! # 设目标角度 90.0°
//! write_reg(0x04, struct.pack('<i', 900))
//!
//! # 前进 500 步
//! write_reg(0x08, struct.pack('<i', 500))
//!
//! # 读当前位置
//! pos = struct.unpack('<i', read_reg(0x0C, 4))[0]
//! print(f"Position: {pos} pulses")
//!
//! # 读实测 Id/Iq 电流
//! id_ma = struct.unpack('<h', read_reg(0x1A, 2))[0]
//! iq_ma = struct.unpack('<h', read_reg(0x1C, 2))[0]
//! print(f"Id: {id_ma} mA, Iq: {iq_ma} mA")
//!
//! # 修改位置 Kp 为 2.0 (Q10 = 2048)
//! write_reg(0x20, struct.pack('<h', 2048))
//!
//! # 归零
//! write_reg(0x19, bytes([0x01]))
//! ```
//!
//! ## 4.2 Arduino / C
//!
//! ```c
//! #include <SPI.h>
//! #define CS_PIN 10
//!
//! void setup() {
//!     SPI.begin();
//!     pinMode(CS_PIN, OUTPUT);
//!     digitalWrite(CS_PIN, HIGH);
//! }
//!
//! void servo_write_i32(uint8_t reg, int32_t val) {
//!     digitalWrite(CS_PIN, LOW);
//!     SPI.transfer(reg & 0x7F);         // 命令字节 (写)
//!     SPI.transfer((uint8_t*)&val, 4);  // little-endian data
//!     digitalWrite(CS_PIN, HIGH);
//! }
//!
//! int32_t servo_read_i32(uint8_t reg) {
//!     int32_t val = 0;
//!     digitalWrite(CS_PIN, LOW);
//!     SPI.transfer(reg | 0x80);           // 命令字节 (读)
//!     for (int i = 0; i < 4; i++)
//!         ((uint8_t*)&val)[i] = SPI.transfer(0x00);  // dummy out, data in
//!     digitalWrite(CS_PIN, HIGH);
//!     return val;
//! }
//!
//! // 设角度 180°
//! servo_write_i32(0x04, 1800);
//!
//! // 读当前位置
//! int32_t pos = servo_read_i32(0x0C);
//! ```
//!
//! ---
//!
//! # 5. 核心架构总览
//!
//! | 模块 | 实现方式 | 说明 |
//! |------|---------|------|
//! | **SPI 从机** | SPI1, GP14(SCK) GP15(TX) GP12(RX) GP13(CSn), Mode0 | 64 字节寄存器映射，非阻塞轮询 |
//! | **角度反馈** | 编码器 GP10(A) + GP11(B) | GPIO 4 倍频解码 |
//! | **电流采样** | GP26(Ia) + GP27(Ib) ADC 12-bit | Clarke→Park 得 Id/Iq 实际值 |
//! | **位置 PID** | Q10 定点 PID, @1 kHz | 位置误差 → 速度指令，带 D 项抑制超调 |
//! | **速度 PI** | Q10 定点 PI, @1 kHz | 编码器差分求 RPM，输出 Iq_ref (mA) |
//! | **Id PI** | Q10 定点 PI, @10 kHz | Id_ref=0（不弱磁），输出 Ud |
//! | **Iq PI** | Q10 定点 PI, @10 kHz | Iq_ref 由速度环提供，输出 Uq |
//! | **逆 Park** | dq → αβ（完整形式） | Uα=Ud·cos−Uq·sin, Uβ=Ud·sin+Uq·cos |
//! | **SVPWM** | 中性点注入法 | αβ → 三相占空比 |
//! | **参数热修改** | SPI 写参数寄存器 | 主循环每 1ms 同步一次 |
//! | **中断** | `IO_IRQ_BANK0` + `TIMER0_IRQ_0` | 编码器 @GPIO + FOC tick @10 kHz |
//!
//! ---
//!
//! # 6. 硬件接线
//!
//! | GPIO | 功能 | 外设 | 说明 |
//! |------|------|------|------|
//! | GP0 | Phase A 高侧 | PWM Slice 0 ChA | 20 kHz SVPWM |
//! | GP1 | Phase A 低侧 | GPIO SIO 输出 | 常导通 |
//! | GP2 | Phase B 高侧 | PWM Slice 1 ChA | 20 kHz SVPWM |
//! | GP3 | Phase B 低侧 | GPIO SIO 输出 | 常导通 |
//! | GP4 | Phase C 高侧 | PWM Slice 2 ChA | 20 kHz SVPWM |
//! | GP5 | Phase C 低侧 | GPIO SIO 输出 | 常导通 |
//! | GP10 | Encoder A | GPIO 输入 + 中断 | 上拉，双边沿 |
//! | GP11 | Encoder B | GPIO 输入 + 中断 | 上拉，双边沿 |
//! | GP12 | SPI1 RX (MOSI) | SPI1 从机输入 | 主机 MOSI → 从机 RX |
//! | GP13 | SPI1 CSn | SPI1 片选 | 低电平选中 |
//! | GP14 | SPI1 SCK | SPI1 时钟 | 由主机提供 |
//! | GP15 | SPI1 TX (MISO) | SPI1 从机输出 | 从机 TX → 主机 MISO |
//! | GP26 | Ia 电流采样 | ADC Ch0 | 相 A 电流传感器 |
//! | GP27 | Ib 电流采样 | ADC Ch1 | 相 B 电流传感器 |
//!
//! ```text
//!   主控制器 (Master)         伺服 (Slave, 本 Pico)
//!   SCK  ────────────────── GP14 (SPI1 SCK)
//!   MOSI ────────────────── GP12 (SPI1 RX / 从机输入)
//!   MISO ────────────────── GP15 (SPI1 TX / 从机输出)
//!   CS   ────────────────── GP13 (SPI1 CSn)
//!   GND  ────────────────── GND
//!
//!   电流传感器 (如 ACS712-5A):
//!   相 A 电流 ──► ACS712 ──► Vout ──► GP26 (ADC Ch0)
//!   相 B 电流 ──► ACS712 ──► Vout ──► GP27 (ADC Ch1)
//!     0A → 1.65V (中点)，±5A → 0~3.3V
//! ```
//!
//! > **I2C 从机替代方案**: 如需 I2C 而非 SPI，参见 `bldc_servo_i2c.rs`。
//! > 寄存器映射完全一致，可复用主控端寄存器定义代码。
//!
//! ---
//!
//! # 7. 参数说明
//!
//! | 参数 | 默认值 | SPI 寄存器 | 说明 |
//! |------|--------|-----------|------|
//! | `MOTOR_POLES` | 14 | — | 电机磁极数（固定） |
//! | `ENCODER_CPR` | 4000 | 0x3A (R) | 编码器 CPR |
//! | `FOC_LOOP_HZ` | 10,000 | — | FOC / 电流环频率 |
//! | `KP_POS` | 1024 (~1.0) | 0x20 | 位置 Kp |
//! | `KI_POS` | 5 (~0.005) | 0x22 | 位置 Ki |
//! | `KD_POS` | 8192 (~8.0) | 0x24 | 位置 Kd |
//! | `KP_SPEED` | 307 (~0.3) | 0x26 | 速度 Kp |
//! | `KI_SPEED` | 20 (~0.02) | 0x28 | 速度 Ki |
//! | `KP_ID` | 512 (~0.5) | 0x2A | Id 电流 Kp |
//! | `KI_ID` | 51 (~0.05) | 0x2C | Id 电流 Ki |
//! | `KP_IQ` | 512 (~0.5) | 0x2E | Iq 电流 Kp |
//! | `KI_IQ` | 51 (~0.05) | 0x30 | Iq 电流 Ki |
//! | `MAX_SPEED_RPM` | 3000 | 0x32 | 速度限幅 |
//! | `IQ_REF_MAX` | 5000 mA | 0x34 | 速度环输出电流限幅 |
//! | `U_MAX` | 17000 (Q15) | 0x36 | 电流 PI 电压输出上限 |
//! | `DEADBAND` | 2 | 0x38 | 位置死区 |
//!
//! ---
//!
//! # 8. 编译运行
//!
//! ```bash
//! cd pico-rs-examples
//! cargo run --bin bldc_servo_spi --target thumbv8m.main-none-eabihf
//! ```
//!
//! ---
//!
//! # 9. 参考资料
//!
//! - [RP2350 Datasheet — SPI / PL022](https://datasheets.raspberrypi.com/rp2350/rp2350-datasheet.pdf)
//! - *Field Oriented Control of AC Motors* — Texas Instruments (SPRABQ8)
//! - *ARM PrimeCell SSP (PL022) Technical Reference Manual*
//! - *STM32 FOC PMSM SDK* — ST AN1292, AN4220

#![no_std]
#![no_main]

use core::cell::RefCell;
use core::sync::atomic::{AtomicBool, AtomicI32, AtomicU32, Ordering};

use defmt::*;
use defmt_rtt as _;

use pico_rs::adc::{Adc, AdcPin};
use pico_rs::block::ImageDef;
use pico_rs::clocks::ClockBuilder;
use pico_rs::entry;
use pico_rs::gpio::Pins;
use pico_rs::interrupt::{self, Mutex};
use pico_rs::pac::{Interrupt, Peripherals};
use pico_rs::pwm::Pwm;
use pico_rs::spi::Spi1WithCs;
use pico_rs::timer::Timer0;

#[unsafe(link_section = ".start_block")]
#[used]
pub static IMAGE_DEF: ImageDef = ImageDef::exe();

// ============================================================================
// 配置参数
// ============================================================================

/// PWM 开关频率 (20 kHz)
const PWM_FREQ_HZ: u32 = 20_000;

/// 电机磁极数
const MOTOR_POLES: u32 = 14;
const POLE_PAIRS: u32 = MOTOR_POLES / 2;

/// 编码器每转计数(CPR, 4 倍频后)
const ENCODER_CPR: u32 = 4000;

/// 编码器方向
const ENCODER_DIR: bool = true;

/// FOC / 电流环频率 (Hz)
const FOC_LOOP_HZ: u32 = 10_000;
const FOC_LOOP_INTERVAL_US: u32 = 1_000_000 / FOC_LOOP_HZ;

/// 位置/速度环分频 → 1 kHz
const POSITION_LOOP_DIVIDER: u32 = 10;

// ---- 位置 PID 参数默认值 (Q10 定点) ----
const KP_POS_DEFAULT: i16 = 1024; // ~1.0
const KI_POS_DEFAULT: i16 = 5; // ~0.005
const KD_POS_DEFAULT: i16 = 8192; // ~8.0
const POS_INTEGRAL_MAX: i32 = 500_000;
const MAX_SPEED_RPM_DEFAULT: i16 = 3000;
const POSITION_DEADBAND_DEFAULT: i16 = 2;

// ---- 速度 PI 参数默认值 (Q10 定点) ----
/// 输出单位为 mA (Iq_ref)
const KP_SPEED_DEFAULT: i16 = 307; // ~0.3
const KI_SPEED_DEFAULT: i16 = 20; // ~0.02
const SPEED_INTEGRAL_MAX: i32 = 500_000;

/// Iq 参考最大值 (mA)
const IQ_REF_MAX_DEFAULT: i16 = 5000;

// ---- 电流 PI 参数默认值 (Q10 定点) ----
/// Id PI (d 轴 — 磁通)
const KP_ID_DEFAULT: i16 = 512; // ~0.5
const KI_ID_DEFAULT: i16 = 51; // ~0.05

/// Iq PI (q 轴 — 力矩)
const KP_IQ_DEFAULT: i16 = 512; // ~0.5
const KI_IQ_DEFAULT: i16 = 51; // ~0.05

/// 电流积分限幅
const CURRENT_INTEGRAL_MAX: i32 = 500_000;

/// 电压输出限幅 (Q15)
const U_MAX_DEFAULT: i16 = 17000;

/// Q10 定点移位
const FP_SHIFT: i32 = 10;

// ---- 电流传感器参数 ----
/// 传感器灵敏度 (mV/A)
const CURRENT_SENSE_MV_PER_A: i32 = 400;

/// ADC 中点（0A 时的 ADC 读数，理想值 2048）
const ADC_ZERO_CURRENT: i32 = 2048;

// ============================================================================
// SPI 从机配置
// ============================================================================

/// 固件版本 (v2.1 — SPI 电流模式)
const FW_VERSION: u16 = 0x0201;

/// SPI 命令字节 R/W 位掩码
const SPI_CMD_READ: u8 = 0x80;

// ============================================================================
// 寄存器地址 (64 字节，与 I2C 版本一致)
// ============================================================================

const REG_TARGET_POS: usize = 0x00;
const REG_TARGET_ANGLE: usize = 0x04;
const REG_REL_STEPS: usize = 0x08;
const REG_ACTUAL_POS: usize = 0x0C;
const REG_ACTUAL_ANGLE: usize = 0x10;
const REG_ACTUAL_RPM: usize = 0x14;
const REG_STATUS: usize = 0x18;
const REG_CMD: usize = 0x19;
const REG_ACTUAL_ID: usize = 0x1A;
const REG_ACTUAL_IQ: usize = 0x1C;
const REG_ACTUAL_IA: usize = 0x1E;
const REG_KP_POS: usize = 0x20;
const REG_KI_POS: usize = 0x22;
const REG_KD_POS: usize = 0x24;
const REG_KP_SPEED: usize = 0x26;
const REG_KI_SPEED: usize = 0x28;
const REG_KP_ID: usize = 0x2A;
const REG_KI_ID: usize = 0x2C;
const REG_KP_IQ: usize = 0x2E;
const REG_KI_IQ: usize = 0x30;
const REG_MAX_RPM: usize = 0x32;
const REG_IQ_REF_MAX: usize = 0x34;
const REG_U_MAX: usize = 0x36;
const REG_DEADBAND: usize = 0x38;
const REG_CPR: usize = 0x3A;
const REG_FW_VER: usize = 0x3C;
const REG_SIZE: usize = 0x40; // 64 bytes

// ============================================================================
// SPI 从机状态机
// ============================================================================

#[derive(Clone, Copy, PartialEq)]
enum SpiState {
    /// 等待命令字节
    Idle,
    /// 正在接收写数据
    Write,
    /// 正在发送读数据（主机发 dummy，从机回寄存器）
    Read,
}

// ============================================================================
// 寄存器文件
// ============================================================================

struct SpiRegs {
    data: [u8; REG_SIZE],
    /// 当前寄存器指针
    ptr: u8,
    /// SPI 事务状态
    state: SpiState,
    /// 写事务起始寄存器地址
    write_base: u8,
    /// 已收到的数据字节数（不含命令字节）
    write_count: usize,
}

impl SpiRegs {
    /// 初始化寄存器文件，写入默认参数
    fn new() -> Self {
        let mut r = Self {
            data: [0u8; REG_SIZE],
            ptr: 0,
            state: SpiState::Idle,
            write_base: 0,
            write_count: 0,
        };
        // 位置 PID
        r.wr_i16(REG_KP_POS, KP_POS_DEFAULT);
        r.wr_i16(REG_KI_POS, KI_POS_DEFAULT);
        r.wr_i16(REG_KD_POS, KD_POS_DEFAULT);
        // 速度 PI
        r.wr_i16(REG_KP_SPEED, KP_SPEED_DEFAULT);
        r.wr_i16(REG_KI_SPEED, KI_SPEED_DEFAULT);
        // 电流 PI
        r.wr_i16(REG_KP_ID, KP_ID_DEFAULT);
        r.wr_i16(REG_KI_ID, KI_ID_DEFAULT);
        r.wr_i16(REG_KP_IQ, KP_IQ_DEFAULT);
        r.wr_i16(REG_KI_IQ, KI_IQ_DEFAULT);
        // 限幅
        r.wr_i16(REG_MAX_RPM, MAX_SPEED_RPM_DEFAULT);
        r.wr_i16(REG_IQ_REF_MAX, IQ_REF_MAX_DEFAULT);
        r.wr_i16(REG_U_MAX, U_MAX_DEFAULT);
        r.wr_i16(REG_DEADBAND, POSITION_DEADBAND_DEFAULT);
        // 只读
        r.wr_u16(REG_CPR, ENCODER_CPR as u16);
        r.wr_u16(REG_FW_VER, FW_VERSION);
        r
    }

    // ---- 寄存器读写辅助 ----

    fn rd_i16(&self, offset: usize) -> i16 {
        i16::from_le_bytes([self.data[offset], self.data[offset + 1]])
    }

    fn rd_i32(&self, offset: usize) -> i32 {
        i32::from_le_bytes([
            self.data[offset],
            self.data[offset + 1],
            self.data[offset + 2],
            self.data[offset + 3],
        ])
    }

    fn wr_i16(&mut self, offset: usize, val: i16) {
        let b = val.to_le_bytes();
        self.data[offset] = b[0];
        self.data[offset + 1] = b[1];
    }

    fn wr_u16(&mut self, offset: usize, val: u16) {
        let b = val.to_le_bytes();
        self.data[offset] = b[0];
        self.data[offset + 1] = b[1];
    }

    fn wr_i32(&mut self, offset: usize, val: i32) {
        let b = val.to_le_bytes();
        self.data[offset] = b[0];
        self.data[offset + 1] = b[1];
        self.data[offset + 2] = b[2];
        self.data[offset + 3] = b[3];
    }

    // ---- 指针操作 ----

    /// 读取当前指针位置的字节，指针自增
    fn read_next(&mut self) -> u8 {
        let addr = (self.ptr as usize) % REG_SIZE;
        let val = self.data[addr];
        self.ptr = ((addr + 1) % REG_SIZE) as u8;
        val
    }

    /// 在当前指针位置写入字节，指针自增
    fn write_next(&mut self, byte: u8) {
        let addr = (self.ptr as usize) % REG_SIZE;
        self.data[addr] = byte;
        self.ptr = ((addr + 1) % REG_SIZE) as u8;
        self.write_count += 1;
    }

    // ---- SPI 事务处理 ----

    /// 处理收到的命令字节（SPI 事务的第一个字节）
    ///
    /// 返回 true 表示是读操作（需要预填 TX FIFO）
    fn process_cmd(&mut self, cmd: u8) -> bool {
        let addr = cmd & 0x7F;
        let is_read = cmd & SPI_CMD_READ != 0;

        self.ptr = addr;
        self.write_base = addr;
        self.write_count = 0;

        if is_read {
            self.state = SpiState::Read;
            self.update_status(); // 刷新只读寄存器
            true
        } else {
            self.state = SpiState::Write;
            false
        }
    }

    /// CSn 上升沿（事务结束）后的写动作处理
    fn on_cs_deassert(&mut self) {
        if self.state == SpiState::Write && self.write_count > 0 {
            let base = self.write_base as usize;
            let count = self.write_count;

            // 根据写入的起始寄存器触发动作
            match base {
                REG_TARGET_POS if count >= 4 => {
                    let pos = self.rd_i32(REG_TARGET_POS);
                    TARGET_POSITION.store(pos, Ordering::Relaxed);
                }
                REG_TARGET_ANGLE if count >= 4 => {
                    let angle_x10 = self.rd_i32(REG_TARGET_ANGLE);
                    let pos = (angle_x10 as i64 * ENCODER_CPR as i64 / 3600) as i32;
                    self.wr_i32(REG_TARGET_POS, pos);
                    TARGET_POSITION.store(pos, Ordering::Relaxed);
                }
                REG_REL_STEPS if count >= 4 => {
                    let steps = self.rd_i32(REG_REL_STEPS);
                    let current = TARGET_POSITION.load(Ordering::Relaxed);
                    let new_target = current.wrapping_add(steps);
                    self.wr_i32(REG_TARGET_POS, new_target);
                    TARGET_POSITION.store(new_target, Ordering::Relaxed);
                    self.wr_i32(REG_REL_STEPS, 0);
                }
                _ => {}
            }

            // CMD 寄存器
            let write_end = base + count;
            if base <= REG_CMD && write_end > REG_CMD {
                let cmd = self.data[REG_CMD];
                if cmd != 0 {
                    match cmd {
                        0x01 => {
                            ENCODER_COUNT.store(0, Ordering::Relaxed);
                            TARGET_POSITION.store(0, Ordering::Relaxed);
                            self.wr_i32(REG_TARGET_POS, 0);
                        }
                        0x02 => {
                            let current = ENCODER_COUNT.load(Ordering::Relaxed);
                            TARGET_POSITION.store(current, Ordering::Relaxed);
                            self.wr_i32(REG_TARGET_POS, current);
                        }
                        _ => {}
                    }
                    self.data[REG_CMD] = 0;
                }
            }
        }

        self.state = SpiState::Idle;
        self.write_count = 0;
    }

    /// 更新只读状态寄存器
    fn update_status(&mut self) {
        let pos = ENCODER_COUNT.load(Ordering::Relaxed);
        let target = TARGET_POSITION.load(Ordering::Relaxed);
        let rpm = ACTUAL_RPM.load(Ordering::Relaxed);

        self.wr_i32(REG_ACTUAL_POS, pos);

        let angle_x10 = (pos as i64 * 3600 / ENCODER_CPR as i64) as i32;
        self.wr_i32(REG_ACTUAL_ANGLE, angle_x10);

        self.wr_i32(REG_ACTUAL_RPM, rpm);

        self.wr_i16(REG_ACTUAL_ID, CURRENT_ID.load(Ordering::Relaxed) as i16);
        self.wr_i16(REG_ACTUAL_IQ, CURRENT_IQ.load(Ordering::Relaxed) as i16);
        self.wr_i16(REG_ACTUAL_IA, CURRENT_IA_MA.load(Ordering::Relaxed) as i16);

        let reached = (pos - target).abs() <= self.rd_i16(REG_DEADBAND) as i32;
        let moving = rpm.unsigned_abs() > 5;
        self.data[REG_STATUS] = (reached as u8) | ((moving as u8) << 1);

        self.wr_i32(REG_TARGET_POS, target);
    }
}

// ============================================================================
// 定点三角函数 — 256 点 Q15 正弦表
// ============================================================================

const SIN_TABLE: [i16; 256] = [
    0, 804, 1608, 2410, 3212, 4011, 4808, 5602, 6393, 7179, 7962, 8739, 9512, 10278, 11039, 11793,
    12539, 13279, 14010, 14732, 15446, 16151, 16846, 17530, 18204, 18868, 19519, 20159, 20787,
    21403, 22005, 22594, 23170, 23731, 24279, 24811, 25329, 25832, 26319, 26790, 27245, 27683,
    28105, 28510, 28898, 29268, 29621, 29956, 30273, 30571, 30852, 31113, 31356, 31580, 31785,
    31971, 32137, 32285, 32412, 32521, 32609, 32678, 32728, 32757, 32767, 32757, 32728, 32678,
    32609, 32521, 32412, 32285, 32137, 31971, 31785, 31580, 31356, 31113, 30852, 30571, 30273,
    29956, 29621, 29268, 28898, 28510, 28105, 27683, 27245, 26790, 26319, 25832, 25329, 24811,
    24279, 23731, 23170, 22594, 22005, 21403, 20787, 20159, 19519, 18868, 18204, 17530, 16846,
    16151, 15446, 14732, 14010, 13279, 12539, 11793, 11039, 10278, 9512, 8739, 7962, 7179, 6393,
    5602, 4808, 4011, 3212, 2410, 1608, 804, 0, -804, -1608, -2410, -3212, -4011, -4808, -5602,
    -6393, -7179, -7962, -8739, -9512, -10278, -11039, -11793, -12539, -13279, -14010, -14732,
    -15446, -16151, -16846, -17530, -18204, -18868, -19519, -20159, -20787, -21403, -22005, -22594,
    -23170, -23731, -24279, -24811, -25329, -25832, -26319, -26790, -27245, -27683, -28105, -28510,
    -28898, -29268, -29621, -29956, -30273, -30571, -30852, -31113, -31356, -31580, -31785, -31971,
    -32137, -32285, -32412, -32521, -32609, -32678, -32728, -32757, -32767, -32757, -32728, -32678,
    -32609, -32521, -32412, -32285, -32137, -31971, -31785, -31580, -31356, -31113, -30852, -30571,
    -30273, -29956, -29621, -29268, -28898, -28510, -28105, -27683, -27245, -26790, -26319, -25832,
    -25329, -24811, -24279, -23731, -23170, -22594, -22005, -21403, -20787, -20159, -19519, -18868,
    -18204, -17530, -16846, -16151, -15446, -14732, -14010, -13279, -12539, -11793, -11039, -10278,
    -9512, -8739, -7962, -7179, -6393, -5602, -4808, -4011, -3212, -2410, -1608, -804,
];

#[inline]
fn sin_q15(angle_q16: u16) -> i32 {
    SIN_TABLE[(angle_q16 >> 8) as usize] as i32
}

#[inline]
fn cos_q15(angle_q16: u16) -> i32 {
    sin_q15(angle_q16.wrapping_add(16384))
}

// ============================================================================
// SVPWM — 中性点注入法
// ============================================================================

fn svpwm(u_alpha: i32, u_beta: i32, pwm_wrap: u16) -> (u16, u16, u16) {
    const SQRT3_Q15: i32 = 56756; // √3 × 32768

    let ua = u_alpha;
    let ub = (-u_alpha + ((SQRT3_Q15 * u_beta) >> 15)) / 2;
    let uc = (-u_alpha - ((SQRT3_Q15 * u_beta) >> 15)) / 2;

    let v_max = ua.max(ub).max(uc);
    let v_min = ua.min(ub).min(uc);
    let offset = -(v_max + v_min) / 2;

    let half_wrap = pwm_wrap as i32 / 2;

    let da = ((ua + offset) * half_wrap / 32767 + half_wrap).clamp(0, pwm_wrap as i32) as u16;
    let db = ((ub + offset) * half_wrap / 32767 + half_wrap).clamp(0, pwm_wrap as i32) as u16;
    let dc = ((uc + offset) * half_wrap / 32767 + half_wrap).clamp(0, pwm_wrap as i32) as u16;

    (da, db, dc)
}

// ============================================================================
// ADC → 电流换算
// ============================================================================

#[inline]
fn adc_to_current_ma(adc_raw: u16) -> i32 {
    let diff = adc_raw as i32 - ADC_ZERO_CURRENT;
    (diff * 806) / CURRENT_SENSE_MV_PER_A
}

// ============================================================================
// PI 控制器（通用）
// ============================================================================

struct PiController {
    kp: i32,
    ki: i32,
    integral: i32,
    integral_max: i32,
    output_max: i32,
    output_min: i32,
}

impl PiController {
    const fn new(kp: i32, ki: i32, integral_max: i32, output_min: i32, output_max: i32) -> Self {
        Self {
            kp,
            ki,
            integral: 0,
            integral_max,
            output_max,
            output_min,
        }
    }

    #[inline]
    fn update(&mut self, error: i32) -> i32 {
        let p = self.kp * error;
        self.integral =
            (self.integral + self.ki * error).clamp(-self.integral_max, self.integral_max);
        let output = (p + self.integral) >> FP_SHIFT;
        output.clamp(self.output_min, self.output_max)
    }

    fn reset(&mut self) {
        self.integral = 0;
    }
}

// ============================================================================
// PID 控制器（位置环专用，带 D 项）
// ============================================================================

struct PidController {
    kp: i32,
    ki: i32,
    kd: i32,
    integral: i32,
    integral_max: i32,
    prev_error: i32,
    output_max: i32,
    output_min: i32,
}

impl PidController {
    const fn new(
        kp: i32,
        ki: i32,
        kd: i32,
        integral_max: i32,
        output_min: i32,
        output_max: i32,
    ) -> Self {
        Self {
            kp,
            ki,
            kd,
            integral: 0,
            integral_max,
            prev_error: 0,
            output_max,
            output_min,
        }
    }

    #[inline]
    fn update(&mut self, error: i32) -> i32 {
        let p = self.kp * error;
        self.integral =
            (self.integral + self.ki * error).clamp(-self.integral_max, self.integral_max);
        let d = self.kd * (error - self.prev_error);
        self.prev_error = error;
        let output = (p + self.integral + d) >> FP_SHIFT;
        output.clamp(self.output_min, self.output_max)
    }

    fn reset(&mut self) {
        self.integral = 0;
        self.prev_error = 0;
    }
}

// ============================================================================
// 全局共享状态
// ============================================================================

static TIMER: Mutex<RefCell<Option<Timer0>>> = Mutex::new(RefCell::new(None));
static ADC_PERIPH: Mutex<RefCell<Option<Adc>>> = Mutex::new(RefCell::new(None));

/// FOC tick 标志
static FOC_TICK: AtomicBool = AtomicBool::new(false);

/// 编码器累计计数
static ENCODER_COUNT: AtomicI32 = AtomicI32::new(0);

/// 编码器上次 AB 状态
static ENCODER_LAST_AB: AtomicU32 = AtomicU32::new(0);

/// 目标位置（编码器脉冲）
static TARGET_POSITION: AtomicI32 = AtomicI32::new(0);

/// Iq 参考值 (mA, 由速度环输出，电流环消费)
static CURRENT_IQ_REF: AtomicI32 = AtomicI32::new(0);

/// 实测 RPM
static ACTUAL_RPM: AtomicI32 = AtomicI32::new(0);

/// 实测电流 (用于寄存器状态回读)
static CURRENT_IA_MA: AtomicI32 = AtomicI32::new(0);
static CURRENT_ID: AtomicI32 = AtomicI32::new(0);
static CURRENT_IQ: AtomicI32 = AtomicI32::new(0);

// ============================================================================
// 编码器 4 倍频解码表
// ============================================================================

const ENCODER_TABLE: [i32; 16] = [
    0, 1, 0, -1, // prev=00
    -1, 0, 1, 0, // prev=01
    0, -1, 0, 1, // prev=11
    1, 0, -1, 0, // prev=10
];

// ============================================================================
// 入口
// ============================================================================

#[entry]
fn main() -> ! {
    info!("BLDC Servo SPI Current Mode — 4-loop FOC via SPI slave");

    // ------------------------------------------------------------------
    // 外设初始化
    // ------------------------------------------------------------------
    let dp = Peripherals::take().unwrap();
    let clocks = ClockBuilder::new()
        .enable_adc()
        .build(dp.CLOCKS, dp.XOSC, dp.PLL_SYS, dp.PLL_USB);
    let sys_freq = clocks.sys_freq();
    info!("System clock: {} MHz", sys_freq / 1_000_000);

    let pins = Pins::new(dp.IO_BANK0, dp.PADS_BANK0);
    let timer = Timer0::init(dp.TIMER0);

    // ------------------------------------------------------------------
    // PWM — 20 kHz，3 个 slice
    // ------------------------------------------------------------------
    let pwm_wrap: u16 = (sys_freq / PWM_FREQ_HZ) as u16;
    info!("PWM wrap = {} ({} kHz)", pwm_wrap, PWM_FREQ_HZ / 1000);

    let pwm = Pwm::new(dp.PWM);

    let slice_a = pwm.slice::<0>().output_a(pins.pin::<0>());
    slice_a.set_wrap(pwm_wrap);
    slice_a.set_duty_a(0);
    slice_a.enable();

    let slice_b = pwm.slice::<1>().output_a(pins.pin::<2>());
    slice_b.set_wrap(pwm_wrap);
    slice_b.set_duty_a(0);
    slice_b.enable();

    let slice_c = pwm.slice::<2>().output_a(pins.pin::<4>());
    slice_c.set_wrap(pwm_wrap);
    slice_c.set_duty_a(0);
    slice_c.enable();

    // ------------------------------------------------------------------
    // 低侧 GPIO — 常导通
    // ------------------------------------------------------------------
    let al = pins.pin::<1>();
    al.into_output();
    al.set_high();

    let bl = pins.pin::<3>();
    bl.into_output();
    bl.set_high();

    let cl = pins.pin::<5>();
    cl.into_output();
    cl.set_high();

    // ------------------------------------------------------------------
    // SPI1 从机 — GP14(SCK), GP15(TX/MISO), GP12(RX/MOSI), GP13(CSn)
    // ------------------------------------------------------------------
    let spi = Spi1WithCs::new_slave(
        dp.SPI1,
        pins.pin::<14>(), // SCK
        pins.pin::<15>(), // TX (MISO — 从机输出)
        pins.pin::<12>(), // RX (MOSI — 从机输入)
        pins.pin::<13>(), // CSn
        sys_freq,
    );
    info!("SPI1 slave initialized: GP14(SCK) GP15(TX) GP12(RX) GP13(CSn)");

    // ------------------------------------------------------------------
    // ADC — 两相电流采样 (Ia on GP26, Ib on GP27)
    // ------------------------------------------------------------------
    let adc = Adc::new(dp.ADC);
    let adc_ia = pins.pin::<26>().into_adc_input();
    let adc_ib = pins.pin::<27>().into_adc_input();
    info!(
        "ADC initialized: Ia=Ch{}, Ib=Ch{}",
        adc_ia.channel(),
        adc_ib.channel()
    );

    // ------------------------------------------------------------------
    // 编码器 — GPIO 中断 4 倍频正交解码
    // ------------------------------------------------------------------
    let enc_a = pins.pin::<10>();
    enc_a.into_input_pullup();
    enc_a.enable_interrupt(pico_rs::gpio::EdgeLevel::EdgeHigh);
    enc_a.enable_interrupt(pico_rs::gpio::EdgeLevel::EdgeLow);

    let enc_b = pins.pin::<11>();
    enc_b.into_input_pullup();
    enc_b.enable_interrupt(pico_rs::gpio::EdgeLevel::EdgeHigh);
    enc_b.enable_interrupt(pico_rs::gpio::EdgeLevel::EdgeLow);

    let init_a = enc_a.get();
    let init_b = enc_b.get();
    info!("Encoder init: A={}, B={}", init_a, init_b);

    let init_ab = ((init_a as u32) << 1) | (init_b as u32);
    ENCODER_LAST_AB.store(init_ab, Ordering::Relaxed);

    // ------------------------------------------------------------------
    // Timer0 — FOC tick @10kHz
    // ------------------------------------------------------------------
    {
        let now = timer.get_time_us_32();
        timer.set_alarm(0, now.wrapping_add(FOC_LOOP_INTERVAL_US));
        timer.enable_alarm_interrupt(0);

        interrupt::free(|cs| {
            TIMER.borrow(cs).replace(Some(timer));
            ADC_PERIPH.borrow(cs).replace(Some(adc));
        });

        unsafe {
            interrupt::enable(Interrupt::TIMER0_IRQ_0);
            interrupt::enable(Interrupt::IO_IRQ_BANK0);
        }
    }

    info!(
        "Servo ready: SPI1 slave, {}Hz FOC/current, {}Hz pos/speed, {} CPR",
        FOC_LOOP_HZ,
        FOC_LOOP_HZ / POSITION_LOOP_DIVIDER,
        ENCODER_CPR
    );

    // ------------------------------------------------------------------
    // 控制器实例
    // ------------------------------------------------------------------

    // 位置环 (PID) → 速度参考 (RPM)
    let mut pid_pos = PidController::new(
        KP_POS_DEFAULT as i32,
        KI_POS_DEFAULT as i32,
        KD_POS_DEFAULT as i32,
        POS_INTEGRAL_MAX,
        -(MAX_SPEED_RPM_DEFAULT as i32),
        MAX_SPEED_RPM_DEFAULT as i32,
    );

    // 速度环 (PI) → Iq_ref (mA)
    let mut pi_speed = PiController::new(
        KP_SPEED_DEFAULT as i32,
        KI_SPEED_DEFAULT as i32,
        SPEED_INTEGRAL_MAX,
        -(IQ_REF_MAX_DEFAULT as i32),
        IQ_REF_MAX_DEFAULT as i32,
    );

    // 电流环 (Id/Iq PI) → Ud/Uq (Q15)
    let mut pi_id = PiController::new(
        KP_ID_DEFAULT as i32,
        KI_ID_DEFAULT as i32,
        CURRENT_INTEGRAL_MAX,
        -(U_MAX_DEFAULT as i32),
        U_MAX_DEFAULT as i32,
    );
    let mut pi_iq = PiController::new(
        KP_IQ_DEFAULT as i32,
        KI_IQ_DEFAULT as i32,
        CURRENT_INTEGRAL_MAX,
        -(U_MAX_DEFAULT as i32),
        U_MAX_DEFAULT as i32,
    );

    let mut spi_regs = SpiRegs::new();

    let mut foc_counter: u32 = 0;
    let mut log_counter: u32 = 0;
    let mut last_encoder_count: i32 = 0;

    // Timer 辅助
    let timer_now_us = || -> u32 {
        interrupt::free(|cs| {
            TIMER
                .borrow(cs)
                .borrow()
                .as_ref()
                .map_or(0, |t| t.get_time_us_32())
        })
    };

    let mut last_speed_time: u32 = timer_now_us();

    // ------------------------------------------------------------------
    // 主循环 — SPI 轮询 + 电流环 @10kHz + 位置/速度环 @1kHz
    // ------------------------------------------------------------------
    loop {
        // ==============================================================
        // SPI 从机轮询（非阻塞）
        // ==============================================================

        // 从 RX FIFO 取出所有已接收的字节
        while let Some(word) = spi.try_read_word() {
            let byte = word as u8;

            match spi_regs.state {
                SpiState::Idle => {
                    // 命令字节：解析 R/W + 地址
                    let is_read = spi_regs.process_cmd(byte);
                    if is_read {
                        // 读操作：预装 TX FIFO（最多 8 字节）
                        for _ in 0..8 {
                            if spi.is_tx_full() {
                                break;
                            }
                            let b = spi_regs.read_next();
                            spi.write_word(b as u16);
                        }
                    }
                }
                SpiState::Write => {
                    // 写数据字节存入寄存器
                    spi_regs.write_next(byte);
                }
                SpiState::Read => {
                    // 主机发送 dummy 来时钟数据；持续补充 TX FIFO
                    if !spi.is_tx_full() {
                        let b = spi_regs.read_next();
                        spi.write_word(b as u16);
                    }
                }
            }
        }

        // 检测事务结束：RX FIFO 空 + SPI 不忙 + 处于非 Idle 状态
        if spi_regs.state != SpiState::Idle && !spi.is_busy() && spi.is_rx_empty() {
            spi_regs.on_cs_deassert();
        }

        // ==============================================================
        // FOC tick 检查
        // ==============================================================
        if !FOC_TICK.swap(false, Ordering::Relaxed) {
            continue;
        }

        foc_counter += 1;

        // ==============================================================
        // 1. 采样两相电流 (ADC)
        // ==============================================================
        let (ia_raw, ib_raw) = interrupt::free(|cs| {
            if let Some(ref mut adc) = *ADC_PERIPH.borrow(cs).borrow_mut() {
                let ia = adc.read(&adc_ia);
                let ib = adc.read(&adc_ib);
                (ia, ib)
            } else {
                (ADC_ZERO_CURRENT as u16, ADC_ZERO_CURRENT as u16)
            }
        });

        let ia_ma = adc_to_current_ma(ia_raw);
        let ib_ma = adc_to_current_ma(ib_raw);
        CURRENT_IA_MA.store(ia_ma, Ordering::Relaxed);

        // ==============================================================
        // 2. 编码器 → 电角度 θe
        // ==============================================================
        let enc_count = ENCODER_COUNT.load(Ordering::Relaxed);

        let count_in_rev = enc_count.rem_euclid(ENCODER_CPR as i32) as u32;
        let theta_mech = ((count_in_rev as u64 * 65536u64) / ENCODER_CPR as u64) as u16;
        let theta_e = theta_mech.wrapping_mul(POLE_PAIRS as u16);

        let sin_theta = sin_q15(theta_e);
        let cos_theta = cos_q15(theta_e);

        // ==============================================================
        // 3. Clarke 变换: Ia, Ib → Iα, Iβ
        // ==============================================================
        let i_alpha = ia_ma;
        let i_beta = (((ia_ma as i64 + 2 * ib_ma as i64) * 37837i64) >> 16) as i32;

        // ==============================================================
        // 4. Park 变换: Iα, Iβ → Id, Iq
        // ==============================================================
        let id =
            ((i_alpha as i64 * cos_theta as i64 + i_beta as i64 * sin_theta as i64) >> 15) as i32;
        let iq =
            ((-i_alpha as i64 * sin_theta as i64 + i_beta as i64 * cos_theta as i64) >> 15) as i32;

        CURRENT_ID.store(id, Ordering::Relaxed);
        CURRENT_IQ.store(iq, Ordering::Relaxed);

        // ==============================================================
        // 5. 电流 PI 控制器 (Id/Iq → Ud/Uq)
        // ==============================================================
        let id_error = 0 - id;
        let ud = pi_id.update(id_error);

        let iq_ref = CURRENT_IQ_REF.load(Ordering::Relaxed);
        let iq_error = iq_ref - iq;
        let uq = pi_iq.update(iq_error);

        // ==============================================================
        // 6. 逆 Park 变换 (完整形式: Ud + Uq)
        // ==============================================================
        let u_alpha = ((ud as i64 * cos_theta as i64 - uq as i64 * sin_theta as i64) >> 15) as i32;
        let u_beta = ((ud as i64 * sin_theta as i64 + uq as i64 * cos_theta as i64) >> 15) as i32;

        // ==============================================================
        // 7. SVPWM → PWM 输出
        // ==============================================================
        let (da, db, dc) = svpwm(u_alpha, u_beta, pwm_wrap);

        slice_a.set_duty_a(da);
        slice_b.set_duty_a(db);
        slice_c.set_duty_a(dc);

        // ==============================================================
        // 8. 位置环 + 速度环 (@1 kHz)
        // ==============================================================
        if foc_counter % POSITION_LOOP_DIVIDER == 0 {
            let now = timer_now_us();
            let dt_us = now.wrapping_sub(last_speed_time);
            last_speed_time = now;

            // ---- 从寄存器同步参数 ----
            pid_pos.kp = spi_regs.rd_i16(REG_KP_POS) as i32;
            pid_pos.ki = spi_regs.rd_i16(REG_KI_POS) as i32;
            pid_pos.kd = spi_regs.rd_i16(REG_KD_POS) as i32;

            let max_speed = spi_regs.rd_i16(REG_MAX_RPM) as i32;
            pid_pos.output_max = max_speed;
            pid_pos.output_min = -max_speed;

            pi_speed.kp = spi_regs.rd_i16(REG_KP_SPEED) as i32;
            pi_speed.ki = spi_regs.rd_i16(REG_KI_SPEED) as i32;

            let iq_ref_max = spi_regs.rd_i16(REG_IQ_REF_MAX) as i32;
            pi_speed.output_max = iq_ref_max;
            pi_speed.output_min = -iq_ref_max;

            pi_id.kp = spi_regs.rd_i16(REG_KP_ID) as i32;
            pi_id.ki = spi_regs.rd_i16(REG_KI_ID) as i32;
            pi_iq.kp = spi_regs.rd_i16(REG_KP_IQ) as i32;
            pi_iq.ki = spi_regs.rd_i16(REG_KI_IQ) as i32;

            let u_max = spi_regs.rd_i16(REG_U_MAX) as i32;
            pi_id.output_max = u_max;
            pi_id.output_min = -u_max;
            pi_iq.output_max = u_max;
            pi_iq.output_min = -u_max;

            let deadband = spi_regs.rd_i16(REG_DEADBAND) as i32;

            // ---- 实际速度计算 ----
            let delta_count = enc_count - last_encoder_count;
            last_encoder_count = enc_count;

            let actual_rpm = if dt_us > 0 {
                (delta_count as i64 * 60_000_000i64 / (ENCODER_CPR as i64 * dt_us as i64)) as i32
            } else {
                0
            };
            ACTUAL_RPM.store(actual_rpm, Ordering::Relaxed);

            // ---- 位置 PID → 速度参考 ----
            let target_pos = TARGET_POSITION.load(Ordering::Relaxed);
            let pos_error = target_pos - enc_count;

            let speed_ref;
            if pos_error.abs() <= deadband {
                pid_pos.reset();
                pi_speed.reset();
                speed_ref = 0;
            } else {
                speed_ref = pid_pos.update(pos_error);
            }

            // ---- 速度 PI → Iq_ref (mA) ----
            let speed_error = speed_ref - actual_rpm;
            let iq_ref_new = pi_speed.update(speed_error);
            CURRENT_IQ_REF.store(iq_ref_new, Ordering::Relaxed);

            // ---- 日志 (~2 Hz) ----
            log_counter += 1;
            if log_counter % 500 == 0 {
                let target_deg = target_pos * 360 / ENCODER_CPR as i32;
                let actual_deg = enc_count * 360 / ENCODER_CPR as i32;
                info!(
                    "pos:{}/{} ({}°/{}°) err:{} spd:{} Iq_ref:{} Id:{} Iq:{} Ud:{} Uq:{}",
                    enc_count,
                    target_pos,
                    actual_deg,
                    target_deg,
                    pos_error,
                    actual_rpm,
                    iq_ref_new,
                    id,
                    iq,
                    ud,
                    uq
                );
            }
        }
    }
}

// ============================================================================
// GPIO 中断 — 编码器 4 倍频解码
// ============================================================================

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
fn IO_IRQ_BANK0() {
    let pin_a = unsafe { pico_rs::gpio::Pin::<10>::steal() };
    let pin_b = unsafe { pico_rs::gpio::Pin::<11>::steal() };
    pin_a.clear_all_interrupts();
    pin_b.clear_all_interrupts();

    let a = pin_a.get() as u32;
    let b = pin_b.get() as u32;
    let curr_ab = (a << 1) | b;

    let prev_ab = ENCODER_LAST_AB.swap(curr_ab, Ordering::Relaxed);
    let idx = ((prev_ab << 2) | curr_ab) as usize;
    let dir = ENCODER_TABLE[idx & 0xF];

    if dir != 0 {
        let step = if ENCODER_DIR { dir } else { -dir };
        ENCODER_COUNT.fetch_add(step, Ordering::Relaxed);
    }
}

// ============================================================================
// Timer0 ISR — FOC tick @10kHz
// ============================================================================

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
fn TIMER0_IRQ_0() {
    interrupt::free(|cs| {
        if let Some(ref timer) = *TIMER.borrow(cs).borrow() {
            timer.clear_alarm_interrupt(0);
            FOC_TICK.store(true, Ordering::Relaxed);
            let now = timer.get_time_us_32();
            timer.set_alarm(0, now.wrapping_add(FOC_LOOP_INTERVAL_US));
        }
    });
}
