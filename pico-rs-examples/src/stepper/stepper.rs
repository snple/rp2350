//! # TMC2209 步进电机控制 — UART 配置 + 梯形加减速
//!
//! 基于 RP2350 (Raspberry Pi Pico 2) 的**步进电机控制器**，
//! 使用 **TMC2209** 驱动芯片，通过 **STEP/DIR** 接口驱动电机，
//! 并通过 **UART** 配置 TMC2209 的运行参数。
//!
//! ---
//!
//! # 1. 功能概述
//!
//! ## 1.1 TMC2209 特性
//!
//! - **静音驱动 (StealthChop2)**: 极低噪音运行，适合桌面 3D 打印机等场景
//! - **UART 配置**: 运行时动态调整电流、微步、StallGuard 等参数，无需硬件跳线
//! - **电流控制**: 通过 IHOLD_IRUN 寄存器精确设置运行电流和保持电流 (0~2A RMS)
//! - **微步细分**: 硬件支持最高 256 微步细分，并内置插值器可将低细分平滑到 256
//! - **StallGuard4**: 无传感器堵转检测，可用于替代限位开关的回零操作
//! - **CoolStep**: 根据负载自适应调节电流，降低功耗和发热
//!
//! ## 1.2 加减速控制
//!
//! 采用**梯形加减速**（Trapezoidal Velocity Profile）：
//! ```text
//!   速度
//!     ↑     ┌───────────────────┐
//!     │    ╱│                   │╲
//!     │   ╱ │                   │ ╲
//!     │  ╱  │     匀速区        │  ╲
//!     │ ╱   │  (max_speed_hz)   │   ╲
//!     │╱    │                   │    ╲
//!     └─────┴───────────────────┴─────→ 步数
//!       加速      恒速            减速
//! ```
//!
//! - **加速段**: 步进频率从 `START_SPEED_HZ` 线性增加到 `MAX_SPEED_HZ`
//! - **匀速段**: 保持最大步进频率
//! - **减速段**: 步进频率线性减小到起步频率，保证精确停止
//! - 加减速由 **Timer 中断**实现，每一步动态调整间隔时间
//!
//! ## 1.3 UART 可配置寄存器一览
//!
//! | 寄存器 | 地址 | 功能 | 说明 |
//! |--------|------|------|------|
//! | GCONF | 0x00 | 全局配置 | StealthChop/SpreadCycle 模式切换 |
//! | GSTAT | 0x01 | 全局状态 | 过温、短路等错误标志 (读后清除) |
//! | IFCNT | 0x02 | 接口计数器 | 每次成功 UART 写入 +1，用于校验通信 |
//! | IOIN | 0x06 | 输入状态 | 引脚状态 + 芯片版本号 (高 8 位) |
//! | IHOLD_IRUN | 0x10 | 电流设置 | 运行电流 / 保持电流 / 过渡时间 |
//! | TPOWERDOWN | 0x11 | 掉电延时 | 停止运动后降为保持电流的延时 |
//! | TSTEP | 0x12 | 步进时间 | 最近一次步进的实际时间 (只读) |
//! | TCOOLTHRS | 0x14 | CoolStep 阈值 | 低于此 TSTEP 时激活 CoolStep/StallGuard |
//! | SGTHRS | 0x40 | StallGuard 阈值 | 堵转检测灵敏度 (0-255) |
//! | SG_RESULT | 0x41 | StallGuard 结果 | 当前负载指示值 (只读, 0-510) |
//! | CHOPCONF | 0x6C | 斩波器配置 | 微步细分、插值、TOFF/HSTRT/HEND 等 |
//! | DRV_STATUS | 0x6F | 驱动状态 | 实际电流、过温、开路检测等 (只读) |
//! | PWMCONF | 0x70 | PWM 配置 | StealthChop 自动调谐参数 |
//!
//! ---
//!
//! # 2. 硬件接线
//!
//! ## 2.1 STEP/DIR 控制接口
//!
//! | Pico GPIO | 功能 | TMC2209 引脚 | 说明 |
//! |-----------|------|-------------|------|
//! | GP2 | STEP | STEP | 步进脉冲输出 (上升沿有效) |
//! | GP3 | DIR | DIR | 方向控制 (高/低 = CW/CCW) |
//! | GP4 | ENABLE | ENN | 使能控制 (**低有效**) |
//!
//! ## 2.2 UART 配置接口
//!
//! | Pico GPIO | 功能 | TMC2209 引脚 | 说明 |
//! |-----------|------|-------------|------|
//! | GP8 | UART1_TX | PDN_UART | 通过 1kΩ 电阻连接 |
//! | GP9 | UART1_RX | PDN_UART | 通过 1kΩ 电阻连接 |
//!
//! ```text
//!   Pico 2 (RP2350)                     TMC2209
//!   ──────────────                      ────────
//!   GP2 (STEP) ─────────────────────── STEP
//!   GP3 (DIR)  ─────────────────────── DIR
//!   GP4 (EN)   ─────────────────────── ENN  (低有效)
//!
//!                      1kΩ
//!   GP8 (UART1_TX) ──/\/\/──┐
//!                            ├──────── PDN_UART (单线半双工)
//!   GP9 (UART1_RX) ──/\/\/──┘
//!                      1kΩ
//!
//!   GND ────────────────────────────── GND
//!   3.3V ───────────────────────────── VCC_IO (逻辑电源)
//!
//!   外部 12~24V 电源 ────────────────── VM   (电机电源)
//!   外部 GND ────────────────────────── GND
//!
//!   步进电机 (4 线双极):
//!     线圈 A+ ──── OA1
//!     线圈 A- ──── OA2
//!     线圈 B+ ──── OB1
//!     线圈 B- ──── OB2
//! ```
//!
//! > **关于 UART 接线**:
//! > TMC2209 的 UART 是**单线半双工**接口 (PDN_UART 引脚)。
//! > Pico 的 TX 和 RX 各通过 1kΩ 电阻连接到 PDN_UART，形成"线或"拓扑。
//! > 发送数据时 Pico 会在 RX 上收到自己发送的回显，代码中需要读取并丢弃这些回显字节。
//!
//! ## 2.3 TMC2209 UART 地址设置 (MS1/MS2 引脚)
//!
//! 一条 UART 总线最多挂 4 个 TMC2209，通过 MS1/MS2 引脚设置不同地址：
//!
//! | MS1 | MS2 | UART 从机地址 |
//! |-----|-----|---------------|
//! | GND | GND | 0x00 (默认) |
//! | VCC | GND | 0x01 |
//! | GND | VCC | 0x02 |
//! | VCC | VCC | 0x03 |
//!
//! ---
//!
//! # 3. 配置参数说明
//!
//! | 常量 | 默认值 | 说明 |
//! |------|--------|------|
//! | `START_SPEED_HZ` | 100 | 起步频率 (Hz)，梯形加减速的起始速度 |
//! | `MAX_SPEED_HZ` | 2000 | 最大步进频率 (Hz)，梯形匀速区的目标速度 |
//! | `ACCEL_STEPS` | 200 | 加速段步数，数值越大加速越平缓 |
//! | `STEP_PULSE_US` | 5 | STEP 脉冲高电平宽度 (μs)，TMC2209 要求 ≥100ns |
//! | `MOTOR_STEPS_PER_REV` | 200 | 电机每转全步数 (1.8° 电机 = 200 步/转) |
//! | `MICROSTEPPING` | 16 | 微步细分数 (1/2/4/8/16/32/64/128/256) |
//! | `TMC_UART_BAUD` | 115200 | TMC2209 UART 通信波特率 |
//! | `TMC_SLAVE_ADDR` | 0x00 | TMC2209 从机地址 (与 MS1/MS2 对应) |
//! | `RUN_CURRENT` | 16 | 运行电流 (0-31)，16 ≈ 50% 满量程 |
//! | `HOLD_CURRENT` | 8 | 保持电流 (0-31)，8 ≈ 25% 满量程 |
//! | `IHOLD_DELAY` | 10 | 从运行电流过渡到保持电流的延时 (0-15) |
//!
//! ---
//!
//! # 4. 演示流程
//!
//! 程序启动后依次执行：
//!
//! 1. **UART 通信验证** — 读取 TMC2209 IOIN 寄存器，确认芯片版本号 (0x21)
//! 2. **芯片配置** — 通过 UART 写入 GCONF / IHOLD_IRUN / CHOPCONF / PWMCONF 等
//! 3. **Demo 1** — 梯形加减速正转 1 圈 (MOTOR_STEPS_PER_REV × MICROSTEPPING 步)
//! 4. **Demo 2** — 梯形加减速反转 1 圈
//! 5. **Demo 3** — 动态修改运行电流：先降低到 25% 跑半圈，再恢复正常跑半圈
//! 6. **Demo 4** — 连续运转 3 秒，每 0.5 秒读取 StallGuard 值和驱动状态
//! 7. **循环** — 无限交替正反转半圈
//!
//! ---
//!
//! # 5. 编译运行
//!
//! ```bash
//! cd pico-rs-examples
//! cargo run --bin stepper --target thumbv8m.main-none-eabihf
//! ```

#![no_std]
#![no_main]

use core::cell::RefCell;
use core::sync::atomic::{AtomicBool, AtomicI32, AtomicU32, Ordering};

use defmt::*;
use defmt_rtt as _;
use panic_probe as _;

use pico_rs::block::ImageDef;
use pico_rs::clocks::ClockBuilder;
use pico_rs::entry;
use pico_rs::gpio::{Pins, Uart1RxPin, Uart1TxPin};
use pico_rs::interrupt::{self, Mutex};
use pico_rs::pac::{Interrupt, Peripherals};
use pico_rs::timer::Timer0;
use pico_rs::uart::{DataBits, Parity, StopBits, Uart1};

#[unsafe(link_section = ".start_block")]
#[used]
pub static IMAGE_DEF: ImageDef = ImageDef::exe();

// ============================================================================
// 配置参数
// ============================================================================

/// 起步脉冲频率 (Hz) — 梯形加减速起始速度
const START_SPEED_HZ: u32 = 100;

/// 最大脉冲频率 (Hz) — 梯形加减速最高速度
const MAX_SPEED_HZ: u32 = 2000;

/// 加速段总步数 — 从 START_SPEED_HZ 线性加速到 MAX_SPEED_HZ 所需步数
const ACCEL_STEPS: u32 = 200;

/// STEP 脉冲高电平宽度 (μs)
/// TMC2209 数据手册要求 STEP 高/低电平最短 100ns，此处取 5μs 留足余量
const STEP_PULSE_US: u32 = 5;

/// 电机每转全步数 (1.8° 步距角 = 200 全步/转)
const MOTOR_STEPS_PER_REV: u32 = 200;

/// 微步细分倍数 — 通过 UART 写入 TMC2209 CHOPCONF 寄存器的 MRES 字段
/// 支持: 1, 2, 4, 8, 16, 32, 64, 128, 256
const MICROSTEPPING: u32 = 16;

/// TMC2209 UART 波特率 (芯片默认支持自动检测，推荐 115200)
const TMC_UART_BAUD: u32 = 115_200;

/// TMC2209 UART 从机地址 — 由 MS1/MS2 引脚的高低电平决定 (0x00~0x03)
const TMC_SLAVE_ADDR: u8 = 0x00;

/// 运行电流 (IRUN, 0-31)
/// 满量程电流取决于 Rsense 和 Vref:  I_RMS = (IRUN + 1) / 32 × V_FS / (√2 × Rsense)
/// 对于典型 Rsense=0.11Ω 的 TMC2209 模块，IRUN=16 约对应 ~1.0A RMS
const RUN_CURRENT: u8 = 16;

/// 保持电流 (IHOLD, 0-31)
/// 电机静止时使用的电流，通常取运行电流的 50%~70% 可节省功耗和减少发热
const HOLD_CURRENT: u8 = 8;

/// 电流过渡延时 (IHOLDDELAY, 0-15)
/// 控制从 IRUN 降低到 IHOLD 的速度，值越大过渡越慢越平滑
const IHOLD_DELAY: u8 = 10;

/// 将步进频率 (Hz) 转换为步进间隔 (μs)
///
/// Timer 中断以此间隔触发，每次中断产生一个 STEP 脉冲。
/// 当 hz=0 时返回一个大值 (1 秒) 作为安全兜底。
const fn speed_to_interval_us(hz: u32) -> u32 {
    if hz == 0 { 1_000_000 } else { 1_000_000 / hz }
}

// ---- GPIO 引脚分配 ----

/// GP2 → TMC2209 STEP 引脚 (步进脉冲)
const PIN_STEP: u32 = 2;
/// GP3 → TMC2209 DIR 引脚 (方向控制)
const PIN_DIR: u32 = 3;
/// GP4 → TMC2209 ENN 引脚 (使能，低有效)
const PIN_ENABLE: u32 = 4;
// GP8 → UART1_TX (TMC2209 PDN_UART)  — 由 Uart1::new() 自动配置 AF
// GP9 → UART1_RX (TMC2209 PDN_UART)  — 由 Uart1::new() 自动配置 AF

// ============================================================================
// TMC2209 寄存器地址定义
// ============================================================================
// 参考: TMC2209 Datasheet Rev 1.09, Section 5 — Register Description

/// GCONF (0x00) — 全局配置
/// 控制 StealthChop/SpreadCycle 切换、方向反转、UART 使能等
const REG_GCONF: u8 = 0x00;

/// GSTAT (0x01) — 全局状态 (读后自动清除)
/// bit 0: reset   — 芯片复位后置 1
/// bit 1: drv_err — 驱动器错误 (过温/短路)
/// bit 2: uv_cp   — 电荷泵欠压
const REG_GSTAT: u8 = 0x01;

/// IFCNT (0x02) — UART 接口写入计数器
/// 每次 UART 成功写入寄存器 +1 (8 位，溢出回绕)
/// 用途: 写入前后读取此值，差值为 1 说明写入成功
const REG_IFCNT: u8 = 0x02;

/// IOIN (0x06) — 输入引脚状态 + 芯片版本
/// bits [7:0]   — ENN, MS1, MS2, DIAG, PDN_UART, STEP, DIR, 各引脚状态
/// bits [31:24] — VERSION (TMC2209 固定为 0x21)
const REG_IOIN: u8 = 0x06;

/// IHOLD_IRUN (0x10) — 保持/运行电流设置
/// bits [4:0]   — IHOLD  (保持电流, 0-31)
/// bits [12:8]  — IRUN   (运行电流, 0-31)
/// bits [19:16] — IHOLDDELAY (过渡延时, 0-15)
const REG_IHOLD_IRUN: u8 = 0x10;

/// TPOWERDOWN (0x11) — 停止后电流降低延时
/// 电机停止运动后，经过 TPOWERDOWN × 2^18 个时钟周期，电流从 IRUN 降为 IHOLD
/// 值 20 ≈ 1.4 秒 (@内部 12MHz 时钟)
const REG_TPOWERDOWN: u8 = 0x11;

/// TSTEP (0x12) — 最近一步的实际步进间隔 (只读)
/// 单位: 内部时钟周期，可用于速度参考
#[allow(dead_code)]
const REG_TSTEP: u8 = 0x12;

/// TCOOLTHRS (0x14) — CoolStep / StallGuard 激活的速度阈值
/// 当 TSTEP ≤ TCOOLTHRS 时 (即速度高于此阈值) 才激活 CoolStep 和 StallGuard
/// 设为 0xFFFFF (最大值) 表示任何速度下都激活
const REG_TCOOLTHRS: u8 = 0x14;

/// SGTHRS (0x40) — StallGuard 堵转检测阈值
/// SG_RESULT ≤ SGTHRS × 2 时触发堵转标志 (DIAG 引脚拉高)
/// 值越大越容易触发 (越灵敏)，0 = 禁用
const REG_SGTHRS: u8 = 0x40;

/// SG_RESULT (0x41) — StallGuard 结果值 (只读)
/// 范围 0~510，值越小表示负载越大。
/// 0 = 最大负载 (已堵转)，510 = 无负载
const REG_SG_RESULT: u8 = 0x41;

/// CHOPCONF (0x6C) — 斩波器配置
/// bits [3:0]   — TOFF: 慢衰减关断时间 (0=驱动器关闭, ≥1 为有效)
/// bits [6:4]   — HSTRT: 滞环比较器起始值
/// bits [10:7]  — HEND: 滞环比较器结束值 (偏移 -3)
/// bit  15      — TBL: 空白时间选择 (0=16, 1=24, 2=36, 3=54 时钟)
/// bits [27:24] — MRES: 微步分辨率 (0=256 ... 8=全步)
/// bit  28      — intpol: 插值使能 (将低细分插值到 256 微步)
/// bit  29      — dedge: 双沿步进 (STEP 上升沿和下降沿都有效)
const REG_CHOPCONF: u8 = 0x6C;

/// DRV_STATUS (0x6F) — 驱动器状态 (只读)
/// bits [4:0]   — SG_RESULT 低 5 位
/// bits [20:16] — CS_ACTUAL: 实际电流缩放值
/// bit  24      — stealth: StealthChop 模式指示
/// bit  25      — stst: 静止检测 (standstill)
/// bit  26      — olb: 开路检测 线圈 B
/// bit  27      — ola: 开路检测 线圈 A
/// bit  28      — s2gb: 线圈 B 对地短路
/// bit  29      — s2ga: 线圈 A 对地短路
/// bit  30      — otpw: 过温预警 (~120°C)
/// bit  31      — ot: 过温关断 (~150°C)
const REG_DRV_STATUS: u8 = 0x6F;

/// PWMCONF (0x70) — StealthChop PWM 调谐参数
/// bits [7:0]   — PWM_OFS: PWM 偏移量 (初始值)
/// bits [15:8]  — PWM_GRAD: PWM 梯度 (速度相关的 PWM 调节)
/// bits [17:16] — pwm_freq: PWM 频率选择 (0=fCLK/1024, 1=fCLK/683, 2=fCLK/512, 3=fCLK/410)
/// bit  18      — pwm_autoscale: StealthChop 自动电流调节
/// bit  19      — pwm_autograd: 自动梯度调节
/// bits [23:20] — freewheel: 静止时自由滑行模式 (0=正常, 1=自由滑行, 2=短接LS, 3=短接HS)
/// bits [27:24] — PWM_REG: 电流调节速度
/// bits [31:28] — PWM_LIM: PWM 自动缩放限制
const REG_PWMCONF: u8 = 0x70;

// ============================================================================
// TMC2209 UART 协议实现
// ============================================================================
//
// TMC2209 使用自定义的单线半双工协议 (非标准 UART 应用层):
//
//   物理层: 标准 UART 8N1, 默认 115200 baud
//   数据帧: 大端字节序
//
//   写数据报 (8 字节, MCU → TMC2209):
//   ┌──────┬───────────┬───────────┬───────┬───────┬───────┬───────┬─────┐
//   │ SYNC │ SLAVE_ADDR│ REG|0x80  │ DATA3 │ DATA2 │ DATA1 │ DATA0 │ CRC │
//   │ 0x05 │  0x00~03  │  写标志   │ MSB   │       │       │ LSB   │     │
//   └──────┴───────────┴───────────┴───────┴───────┴───────┴───────┴─────┘
//
//   读请求 (4 字节, MCU → TMC2209):
//   ┌──────┬───────────┬─────┬─────┐
//   │ SYNC │ SLAVE_ADDR│ REG │ CRC │
//   │ 0x05 │  0x00~03  │     │     │
//   └──────┴───────────┴─────┴─────┘
//
//   读回复 (8 字节, TMC2209 → MCU):
//   ┌──────┬──────┬─────┬───────┬───────┬───────┬───────┬─────┐
//   │ SYNC │ 0xFF │ REG │ DATA3 │ DATA2 │ DATA1 │ DATA0 │ CRC │
//   │ 0x05 │ 主机 │     │ MSB   │       │       │ LSB   │     │
//   └──────┴──────┴─────┴───────┴───────┴───────┴───────┴─────┘
//
//   CRC: 多项式 x^8 + x^2 + x + 1 (0x07)，初始值 0，逐位计算

/// TMC2209 同步字节 — 每个数据报的第一个字节
const TMC_SYNC: u8 = 0x05;

/// 计算 TMC2209 CRC8 校验值
///
/// 算法: CRC-8，生成多项式 0x07 (x^8 + x^2 + x + 1)，初始值 0x00。
/// **注意**: TMC2209 使用 LSB-first 位处理顺序 (先处理低位)，
/// 这与标准 CRC-8 的 MSB-first 不同。
fn tmc_crc8(data: &[u8]) -> u8 {
    let mut crc: u8 = 0;
    for &byte in data {
        let mut current = byte;
        for _ in 0..8 {
            // 比较 CRC 最高位和数据当前最低位
            if (crc >> 7) ^ (current & 0x01) != 0 {
                crc = (crc << 1) ^ 0x07; // 异或生成多项式
            } else {
                crc <<= 1;
            }
            current >>= 1; // LSB-first: 从低位向高位处理
        }
    }
    crc
}

/// 通过 UART 写入 TMC2209 寄存器
///
/// 构造 8 字节写数据报并发送，然后读取并丢弃单线半双工产生的回显。
///
/// # 参数
/// - `uart`: UART1 外设引用
/// - `reg`: 目标寄存器地址 (0x00~0x7F)
/// - `value`: 要写入的 32 位值
fn tmc_write_reg<TX: Uart1TxPin, RX: Uart1RxPin>(uart: &Uart1<TX, RX>, reg: u8, value: u32) {
    // 构造写数据报: [SYNC, ADDR, REG|0x80, DATA3..DATA0, CRC]
    let mut datagram = [0u8; 8];
    datagram[0] = TMC_SYNC;
    datagram[1] = TMC_SLAVE_ADDR;
    datagram[2] = reg | 0x80; // bit7=1 表示写操作
    datagram[3] = (value >> 24) as u8; // DATA[31:24] MSB
    datagram[4] = (value >> 16) as u8; // DATA[23:16]
    datagram[5] = (value >> 8) as u8; // DATA[15:8]
    datagram[6] = value as u8; // DATA[7:0]  LSB
    datagram[7] = tmc_crc8(&datagram[..7]);

    uart.write(&datagram);
    uart.flush();

    // 丢弃回显:
    // 单线半双工拓扑下，Pico TX → 1kΩ → PDN_UART ← 1kΩ ← Pico RX
    // 发送 8 字节后 RX 线上会收到相同的 8 字节回显
    for _ in 0..8 {
        let _ = tmc_read_byte_timeout(uart, 2000);
    }
}

/// 通过 UART 读取 TMC2209 寄存器
///
/// 发送 4 字节读请求，丢弃 4 字节回显，然后等待 TMC2209 返回 8 字节回复，
/// 并校验 CRC 和帧头。
///
/// # 参数
/// - `uart`: UART1 外设引用
/// - `timer`: 定时器引用 (用于等待 TMC2209 回复)
/// - `reg`: 目标寄存器地址
///
/// # 返回
/// - `Some(value)`: 读取成功，返回 32 位寄存器值
/// - `None`: 超时、CRC 校验失败、或帧头异常
fn tmc_read_reg<TX: Uart1TxPin, RX: Uart1RxPin>(
    uart: &Uart1<TX, RX>,
    timer: &Timer0,
    reg: u8,
) -> Option<u32> {
    // 构造读请求: [SYNC, ADDR, REG, CRC]
    let mut request = [0u8; 4];
    request[0] = TMC_SYNC;
    request[1] = TMC_SLAVE_ADDR;
    request[2] = reg; // bit7=0 表示读操作
    request[3] = tmc_crc8(&request[..3]);

    uart.write(&request);
    uart.flush();

    // 丢弃 4 字节回显 (自己发送的请求)
    for _ in 0..4 {
        let _ = tmc_read_byte_timeout(uart, 2000);
    }

    // 等待 TMC2209 处理请求并发送回复
    // TMC2209 典型回复延迟 ~75μs，此处取 200μs 留余量
    timer.delay_us(200);

    // 接收 8 字节回复: [SYNC, 0xFF, REG, DATA3..DATA0, CRC]
    let mut reply = [0u8; 8];
    for slot in reply.iter_mut() {
        match tmc_read_byte_timeout(uart, 5000) {
            Some(b) => *slot = b,
            None => return None, // 超时，可能接线有问题
        }
    }

    // CRC 校验 — 对前 7 字节计算 CRC，与第 8 字节比对
    let crc = tmc_crc8(&reply[..7]);
    if crc != reply[7] {
        warn!(
            "TMC2209 CRC error: computed {:#04x}, received {:#04x}",
            crc, reply[7]
        );
        return None;
    }

    // 帧头校验 — 回复的 SYNC 应为 0x05，地址字节固定为 0xFF
    if reply[0] != TMC_SYNC || reply[1] != 0xFF {
        warn!(
            "TMC2209 bad reply header: [{:#04x}, {:#04x}]",
            reply[0], reply[1]
        );
        return None;
    }

    // 组装 32 位寄存器值 (大端序)
    let value = (reply[3] as u32) << 24
        | (reply[4] as u32) << 16
        | (reply[5] as u32) << 8
        | (reply[6] as u32);

    Some(value)
}

/// UART 单字节读取 (带超时)
///
/// 在循环中轮询 UART RX FIFO，每次迭代间插入短暂自旋延时。
/// `timeout_loops` 控制最大轮询次数 (每次约 ~100 个 spin_loop 周期)。
fn tmc_read_byte_timeout<TX: Uart1TxPin, RX: Uart1RxPin>(
    uart: &Uart1<TX, RX>,
    timeout_loops: u32,
) -> Option<u8> {
    for _ in 0..timeout_loops {
        if let Some(byte) = uart.read_byte() {
            return Some(byte);
        }
        // 短自旋延时，避免紧密循环占满总线
        for _ in 0..100 {
            core::hint::spin_loop();
        }
    }
    None
}

// ============================================================================
// TMC2209 高级配置函数
// ============================================================================

/// 完整配置 TMC2209 驱动器
///
/// 执行以下步骤:
/// 1. 读取 IOIN 寄存器验证 UART 通信和芯片版本
/// 2. 清除 GSTAT 错误标志
/// 3. 配置 GCONF: StealthChop 模式 + UART 使能
/// 4. 配置 IHOLD_IRUN: 运行/保持电流
/// 5. 配置 TPOWERDOWN: 停止后电流降低延时
/// 6. 配置 CHOPCONF: 微步细分 + 256 微步插值
/// 7. 配置 PWMCONF: StealthChop 自动调谐参数
/// 8. 配置 SGTHRS: StallGuard 堵转检测阈值
/// 9. 配置 TCOOLTHRS: CoolStep/StallGuard 激活条件
/// 10. 回读 CHOPCONF 校验写入结果
/// 11. 读取 IFCNT 确认总写入次数
fn tmc_configure<TX: Uart1TxPin, RX: Uart1RxPin>(uart: &Uart1<TX, RX>, timer: &Timer0) {
    // ---- 步骤 1: 验证通信，读取芯片版本 ----
    if let Some(ioin) = tmc_read_reg(uart, timer, REG_IOIN) {
        let version = (ioin >> 24) & 0xFF;
        info!("TMC2209 version: {:#04x} (expect 0x21)", version);
    } else {
        warn!("Failed to read TMC2209 IOIN — check UART wiring and power!");
    }

    // ---- 步骤 2: 清除 GSTAT 错误标志 ----
    // 写入 0x07 清除所有 3 个标志位 (reset, drv_err, uv_cp)
    tmc_write_reg(uart, REG_GSTAT, 0x07);
    timer.delay_ms(10);

    // ---- 步骤 3: GCONF — 全局配置 ----
    //
    // 我们设置:
    //   bit 0  I_scale_analog  = 0  → 使用内部电流参考 (不使用 VREF 引脚)
    //   bit 2  en_spreadcycle  = 0  → 使用 StealthChop (静音模式)
    //   bit 3  shaft           = 0  → 正常方向 (不反转)
    //   bit 6  multistep_filt  = 1  → STEP 脉冲边沿滤波，抑制干扰
    //   bit 7  pdn_disable     = 1  → 禁用 PDN_UART 的自动待机功能，改为 UART 通信
    //
    // 注意: pdn_disable=1 是使用 UART 的必要条件!
    let gconf: u32 = (1 << 6)  // multistep_filt
                   | (1 << 7); // pdn_disable
    tmc_write_reg(uart, REG_GCONF, gconf);
    timer.delay_ms(10);

    // ---- 步骤 4: IHOLD_IRUN — 电流设置 ----
    //
    // 电流公式 (以 Rsense=0.11Ω 为例):
    //   I_RMS = (value + 1) / 32 × 0.325V / (√2 × 0.11Ω) ≈ (value + 1) × 65.3mA
    //   IRUN=16  → ~1.11A RMS
    //   IHOLD=8  → ~0.59A RMS
    //
    // 寄存器布局:
    //   bits [4:0]   = IHOLD      (保持电流)
    //   bits [12:8]  = IRUN       (运行电流)
    //   bits [19:16] = IHOLDDELAY (过渡时间: 2^18 clk × IHOLDDELAY 每 step)
    let ihold_irun: u32 =
        (HOLD_CURRENT as u32) | ((RUN_CURRENT as u32) << 8) | ((IHOLD_DELAY as u32) << 16);
    tmc_write_reg(uart, REG_IHOLD_IRUN, ihold_irun);
    timer.delay_ms(10);

    // ---- 步骤 5: TPOWERDOWN — 停止后电流降低延时 ----
    // 值 20 × 2^18 / 12MHz ≈ 1.4 秒
    tmc_write_reg(uart, REG_TPOWERDOWN, 20);
    timer.delay_ms(10);

    // ---- 步骤 6: CHOPCONF — 斩波器 + 微步配置 ----
    //
    // MRES 编码: 0→256, 1→128, 2→64, 3→32, 4→16, 5→8, 6→4, 7→2, 8→全步
    let mres = match MICROSTEPPING {
        256 => 0,
        128 => 1,
        64 => 2,
        32 => 3,
        16 => 4,
        8 => 5,
        4 => 6,
        2 => 7,
        _ => 8, // 全步进
    };
    //
    // TOFF=3  : 慢衰减时间 (推荐 2~5, 越大越安静但纹波增大)
    // HSTRT=4 : 滞环起始值 (与 HEND 配合控制过零电流)
    // HEND=1  : 滞环结束值 (偏移 -3, 实际为 -2)
    // TBL=1   : 空白时间 = 24 clk (推荐值, 抑制开关噪声)
    // intpol=1: 256 微步插值 (即使 MRES 设置为 16 微步, PWM 也内部插值到 256)
    let chopconf: u32 = 3          // TOFF
        | (4 << 4)                 // HSTRT
        | (1 << 7)                 // HEND
        | (1 << 15)                // TBL
        | (mres << 24)             // MRES
        | (1 << 28); // intpol
    tmc_write_reg(uart, REG_CHOPCONF, chopconf);
    timer.delay_ms(10);

    // ---- 步骤 7: PWMCONF — StealthChop 自动调谐 ----
    //
    // PWM_OFS=36       : 初始 PWM 偏移量 (经验值)
    // PWM_GRAD=14      : 速度相关梯度 (经验值)
    // pwm_freq=0       : PWM 频率 = fCLK/1024 ≈ 12kHz (安静)
    // pwm_autoscale=1  : 启用自动电流缩放 (StealthChop 核心功能)
    // pwm_autograd=1   : 启用自动梯度适应
    // PWM_REG=8        : 电流调节卷积速度
    // PWM_LIM=12       : PWM 自动缩放上限
    let pwmconf: u32 = 36          // PWM_OFS
        | (14 << 8)                // PWM_GRAD
        | (0 << 16)                // pwm_freq
        | (1 << 18)                // pwm_autoscale
        | (1 << 19)                // pwm_autograd
        | (8 << 24)                // PWM_REG
        | (12 << 28); // PWM_LIM
    tmc_write_reg(uart, REG_PWMCONF, pwmconf);
    timer.delay_ms(10);

    // ---- 步骤 8: SGTHRS — StallGuard 堵转检测阈值 ----
    // SGTHRS=10 是比较保守的值，不容易误触发
    // 实际应用中需要根据电机和负载情况调整
    tmc_write_reg(uart, REG_SGTHRS, 10);
    timer.delay_ms(10);

    // ---- 步骤 9: TCOOLTHRS — CoolStep/StallGuard 激活速度阈值 ----
    // 设为最大值 0xFFFFF，表示任意速度下都激活 CoolStep 和 StallGuard
    tmc_write_reg(uart, REG_TCOOLTHRS, 0xFFFFF);
    timer.delay_ms(10);

    // ---- 步骤 10: 回读 CHOPCONF 校验写入 ----
    if let Some(readback) = tmc_read_reg(uart, timer, REG_CHOPCONF) {
        info!("TMC2209 CHOPCONF readback: {:#010x}", readback);
        let readback_mres = (readback >> 24) & 0x0F;
        if readback_mres == mres as u32 {
            info!("  MRES verified: {} microstepping", MICROSTEPPING);
        } else {
            warn!("  MRES mismatch! read={}, expected={}", readback_mres, mres);
        }
    }

    // ---- 步骤 11: 读 IFCNT 确认写入计数 ----
    if let Some(ifcnt) = tmc_read_reg(uart, timer, REG_IFCNT) {
        info!("TMC2209 IFCNT = {} (successful UART writes)", ifcnt & 0xFF);
    }

    info!("TMC2209 configuration complete");
}

/// 动态修改 TMC2209 运行/保持电流
///
/// 可在电机运行中调用，立即生效。
/// 用途: 降低空载电流以省电，或在高负载时临时提升电流。
fn tmc_set_run_current<TX: Uart1TxPin, RX: Uart1RxPin>(
    uart: &Uart1<TX, RX>,
    run_current: u8,
    hold_current: u8,
) {
    let ihold_irun: u32 =
        (hold_current as u32) | ((run_current as u32) << 8) | ((IHOLD_DELAY as u32) << 16);
    tmc_write_reg(uart, REG_IHOLD_IRUN, ihold_irun);
}

/// 读取 TMC2209 驱动状态寄存器 (DRV_STATUS)
///
/// 返回的 32 位值包含: 实际电流、过温、开路检测、StallGuard 等信息。
/// 详见 `REG_DRV_STATUS` 的位定义。
fn tmc_read_drv_status<TX: Uart1TxPin, RX: Uart1RxPin>(
    uart: &Uart1<TX, RX>,
    timer: &Timer0,
) -> Option<u32> {
    tmc_read_reg(uart, timer, REG_DRV_STATUS)
}

/// 读取 StallGuard 结果值 (SG_RESULT, 0~510)
///
/// 值越小 = 负载越大，0 = 堵转。
/// 仅在速度超过 TCOOLTHRS 设定的阈值时有效。
fn tmc_read_sg_result<TX: Uart1TxPin, RX: Uart1RxPin>(
    uart: &Uart1<TX, RX>,
    timer: &Timer0,
) -> Option<u32> {
    tmc_read_reg(uart, timer, REG_SG_RESULT)
}

// ============================================================================
// 运动控制 — 步进状态机
// ============================================================================

/// 运动阶段枚举
///
/// 梯形加减速的状态机:
/// ```text
///   Idle → Accelerating → Cruising → Decelerating → Idle
///                           ↑                        ↑
///                      (steps 不够时直接跳到减速)     |
///   Idle → Running ──────────────────────────────────┘
///          (连续运转模式, 外部调用 stop_motion() 退出)
/// ```
#[derive(Clone, Copy, PartialEq)]
enum MotionPhase {
    /// 空闲 — 电机静止
    Idle,
    /// 加速中 — 步进间隔逐步缩短 (频率增大)
    Accelerating,
    /// 匀速 — 保持最大步进频率
    Cruising,
    /// 减速中 — 步进间隔逐步增大 (频率降低)
    Decelerating,
    /// 连续运转 — 恒速运行直到外部停止
    Running,
}

// ============================================================================
// 全局共享状态 — Timer 中断与主循环之间的通信
// ============================================================================
// 所有状态使用原子类型，Timer ISR 和 main 线程通过原子操作无锁通信。
// Timer0 外设放在 Mutex<RefCell> 中，通过 interrupt::free() 临界区访问。

/// Timer0 外设 — 在 main 中初始化，移入全局供 ISR 使用
static TIMER: Mutex<RefCell<Option<Timer0>>> = Mutex::new(RefCell::new(None));

/// 当前运动阶段 (MotionPhase 编码为 u32: 0=Idle, 1=Accel, 2=Cruise, 3=Decel, 4=Run)
static MOTION_PHASE: AtomicU32 = AtomicU32::new(0);
/// 当前步进间隔 (μs) — ISR 用此值设置下一次 alarm
static STEP_INTERVAL_US: AtomicU32 = AtomicU32::new(0);
/// 已完成步数计数器 — ISR 每步 +1
static STEPS_DONE: AtomicU32 = AtomicU32::new(0);
/// 目标步数 — 0 表示连续运转模式
static STEPS_TARGET: AtomicU32 = AtomicU32::new(0);
/// 减速起始步数 — 到达此步数时从匀速切换到减速
static DECEL_START_STEP: AtomicU32 = AtomicU32::new(0);
/// 运动方向 — 0=正转 (DIR 低电平), 1=反转 (DIR 高电平)
static DIRECTION: AtomicU32 = AtomicU32::new(0);
/// 绝对位置 (带符号步数) — 正转 +1, 反转 -1
static POSITION: AtomicI32 = AtomicI32::new(0);
/// 运动完成标志 — main 轮询此标志等待运动结束
static MOTION_DONE: AtomicBool = AtomicBool::new(true);
/// STEP 脉冲状态 — true=高电平阶段, false=低电平阶段
/// ISR 两段式脉冲: 第一次中断拉高 STEP，STEP_PULSE_US 后第二次中断拉低
static PULSE_STATE: AtomicBool = AtomicBool::new(false);

// ============================================================================
// 梯形加减速核心算法
// ============================================================================

/// 根据当前步数和运动阶段计算步进间隔 (μs)
///
/// 梯形速度规划原理:
///
/// ```text
///   间隔(μs)
///     ↑  start_interval (100Hz → 10000μs)
///     │  ╲
///     │   ╲  加速段: 间隔线性减小 → 速度线性增大
///     │    ╲
///     │     └─────────────────────┐ max_interval (2000Hz → 500μs)
///     │                           │ 匀速段: 保持最小间隔
///     │                           ├─────────────┐
///     │                           │  减速段:    ╱
///     │                           │ 间隔增大  ╱
///     │                           │         ╱
///     └───────────────────────────┴────────→ 步数
/// ```
fn compute_interval(step: u32, total_steps: u32) -> u32 {
    let phase = phase_from_u32(MOTION_PHASE.load(Ordering::Relaxed));

    // 起步间隔 (低速) 和 最大速度间隔 (高速)
    let start_interval = speed_to_interval_us(START_SPEED_HZ); // 10000μs @ 100Hz
    let max_interval = speed_to_interval_us(MAX_SPEED_HZ); //   500μs @ 2000Hz

    match phase {
        MotionPhase::Accelerating => {
            // 线性插值: start_interval → max_interval
            // 间隔越小速度越快，所以是"减小间隔"
            if ACCEL_STEPS == 0 {
                return max_interval;
            }
            let progress = step.min(ACCEL_STEPS);
            start_interval
                - ((start_interval - max_interval) as u64 * progress as u64 / ACCEL_STEPS as u64)
                    as u32
        }
        MotionPhase::Cruising | MotionPhase::Running => {
            // 匀速或连续运转: 保持最高速度
            max_interval
        }
        MotionPhase::Decelerating => {
            // 与加速段对称: max_interval → start_interval
            let decel_start = DECEL_START_STEP.load(Ordering::Relaxed);
            let steps_into_decel = step.saturating_sub(decel_start);
            let decel_steps = total_steps.saturating_sub(decel_start);
            if decel_steps == 0 {
                return start_interval;
            }
            let progress = steps_into_decel.min(decel_steps);
            max_interval
                + ((start_interval - max_interval) as u64 * progress as u64 / decel_steps as u64)
                    as u32
        }
        MotionPhase::Idle => start_interval,
    }
}

/// MotionPhase → u32 编码 (用于原子存储)
fn phase_to_u32(phase: MotionPhase) -> u32 {
    match phase {
        MotionPhase::Idle => 0,
        MotionPhase::Accelerating => 1,
        MotionPhase::Cruising => 2,
        MotionPhase::Decelerating => 3,
        MotionPhase::Running => 4,
    }
}

/// u32 → MotionPhase 解码
fn phase_from_u32(val: u32) -> MotionPhase {
    match val {
        1 => MotionPhase::Accelerating,
        2 => MotionPhase::Cruising,
        3 => MotionPhase::Decelerating,
        4 => MotionPhase::Running,
        _ => MotionPhase::Idle,
    }
}

// ============================================================================
// STEP/DIR 运动控制函数
// ============================================================================

/// 启动梯形加减速运动
///
/// 设置运动参数并启动 Timer 中断，ISR 会自动执行 加速→匀速→减速→停止 的完整过程。
///
/// # 参数
/// - `steps`: 目标步数 (微步)
/// - `dir`: 方向，0=正转 (DIR 低), 1=反转 (DIR 高)
fn start_trapezoidal_move(steps: u32, dir: u32) {
    if steps == 0 {
        return;
    }

    // 初始化运动状态
    MOTION_DONE.store(false, Ordering::Relaxed);
    STEPS_DONE.store(0, Ordering::Relaxed);
    STEPS_TARGET.store(steps, Ordering::Relaxed);
    DIRECTION.store(dir, Ordering::Relaxed);
    PULSE_STATE.store(false, Ordering::Relaxed);

    // 计算减速起始点:
    //   加/减速各占一半步数，如果总步数不够则 accel=decel=steps/2
    //   decel_start = steps - decel_steps
    let accel_steps = ACCEL_STEPS.min(steps / 2);
    let decel_start = steps - accel_steps;
    DECEL_START_STEP.store(decel_start, Ordering::Relaxed);

    // 设置方向引脚 (DIR)
    let sio = unsafe { pico_rs::pac::SIO::steal() };
    if dir == 0 {
        sio.gpio_out_clr.write_bits(1 << PIN_DIR);
    } else {
        sio.gpio_out_set.write_bits(1 << PIN_DIR);
    }

    // 从"加速"阶段开始
    MOTION_PHASE.store(phase_to_u32(MotionPhase::Accelerating), Ordering::Relaxed);

    // 初始间隔 = 起步频率对应的间隔
    let interval = speed_to_interval_us(START_SPEED_HZ);
    STEP_INTERVAL_US.store(interval, Ordering::Relaxed);

    // 启动 Timer alarm — 第一次中断将产生第一个 STEP 脉冲
    interrupt::free(|cs| {
        if let Some(ref timer) = *TIMER.borrow(cs).borrow() {
            let now = timer.get_time_us_32();
            timer.set_alarm(0, now.wrapping_add(interval));
            timer.enable_alarm_interrupt(0);
        }
    });
}

/// 启动连续运转模式
///
/// 以指定速度持续运行，直到调用 `stop_motion()` 停止。
/// 不使用梯形加减速，直接以目标频率启动 (适用于低速或允许急起急停的场景)。
///
/// # 参数
/// - `dir`: 方向，0=正转, 1=反转
/// - `speed_hz`: 步进频率 (Hz)
fn start_continuous_run(dir: u32, speed_hz: u32) {
    MOTION_DONE.store(false, Ordering::Relaxed);
    STEPS_DONE.store(0, Ordering::Relaxed);
    STEPS_TARGET.store(0, Ordering::Relaxed); // 0 = 无限步数
    DIRECTION.store(dir, Ordering::Relaxed);
    PULSE_STATE.store(false, Ordering::Relaxed);

    let sio = unsafe { pico_rs::pac::SIO::steal() };
    if dir == 0 {
        sio.gpio_out_clr.write_bits(1 << PIN_DIR);
    } else {
        sio.gpio_out_set.write_bits(1 << PIN_DIR);
    }

    MOTION_PHASE.store(phase_to_u32(MotionPhase::Running), Ordering::Relaxed);

    let interval = speed_to_interval_us(speed_hz);
    STEP_INTERVAL_US.store(interval, Ordering::Relaxed);

    interrupt::free(|cs| {
        if let Some(ref timer) = *TIMER.borrow(cs).borrow() {
            let now = timer.get_time_us_32();
            timer.set_alarm(0, now.wrapping_add(interval));
            timer.enable_alarm_interrupt(0);
        }
    });
}

/// 立即停止运动 (急停)
///
/// 将状态机置为 Idle 并拉低 STEP 引脚。
/// 注意: 这是急停，不会执行减速过程。高速运行时急停可能导致丢步。
fn stop_motion() {
    MOTION_PHASE.store(phase_to_u32(MotionPhase::Idle), Ordering::Relaxed);
    MOTION_DONE.store(true, Ordering::Relaxed);

    // 确保 STEP 引脚为低电平
    let sio = unsafe { pico_rs::pac::SIO::steal() };
    sio.gpio_out_clr.write_bits(1 << PIN_STEP);
}

/// 阻塞等待当前运动完成
///
/// 使用 WFI (Wait For Interrupt) 降低 CPU 功耗，每次中断唤醒后检查完成标志。
fn wait_motion_done() {
    while !MOTION_DONE.load(Ordering::Relaxed) {
        pico_rs_rt::asm::wfi();
    }
}

// ============================================================================
// 程序入口
// ============================================================================

#[entry]
fn main() -> ! {
    info!("Stepper Motor — TMC2209 UART Control");

    // ------------------------------------------------------------------
    // 1. 外设初始化
    // ------------------------------------------------------------------
    let dp = Peripherals::take().unwrap();

    // 配置系统时钟: 150MHz sys_clk, 48MHz USB
    let clocks = ClockBuilder::new().build(dp.CLOCKS, dp.XOSC, dp.PLL_SYS, dp.PLL_USB);
    let pins = Pins::new(dp.IO_BANK0, dp.PADS_BANK0);
    let timer = Timer0::init(dp.TIMER0);

    // ------------------------------------------------------------------
    // 2. GPIO 配置 — STEP / DIR / ENABLE
    // ------------------------------------------------------------------
    // GP2 = STEP (步进脉冲输出)
    let p2 = pins.pin::<2>();
    p2.into_output();
    p2.set_low();

    // GP3 = DIR (方向控制)
    let p3 = pins.pin::<3>();
    p3.into_output();
    p3.set_low();

    // GP4 = ENN (使能控制)
    // TMC2209 ENN 为低有效: 低=使能, 高=禁用
    // 初始化时先禁用驱动器，待配置完成后再使能
    let p4 = pins.pin::<4>();
    p4.into_output();
    p4.set_high();

    info!("GPIO: GP2(STEP) GP3(DIR) GP4(ENN)");

    // ------------------------------------------------------------------
    // 3. UART1 配置 — TMC2209 通信
    //    GP8 = UART1_TX → 1kΩ → PDN_UART
    //    GP9 = UART1_RX → 1kΩ → PDN_UART
    // ------------------------------------------------------------------
    let uart = Uart1::new(
        dp.UART1,
        pins.pin::<8>(), // TX
        pins.pin::<9>(), // RX
        TMC_UART_BAUD,
        clocks.peri_freq(),
    );
    // TMC2209 UART 格式: 8 数据位, 无校验, 1 停止位 (8N1)
    uart.set_format(DataBits::Eight, StopBits::One, Parity::None);

    info!("UART1: {} baud, 8N1 (GP8=TX, GP9=RX)", TMC_UART_BAUD);

    // 等待 TMC2209 上电稳定 (数据手册推荐 ≥10ms)
    timer.delay_ms(100);

    // ------------------------------------------------------------------
    // 4. 通过 UART 配置 TMC2209
    // ------------------------------------------------------------------
    info!("=== Configuring TMC2209 via UART ===");
    tmc_configure(&uart, &timer);

    // 每转总微步数 = 全步数 × 微步倍数
    let steps_per_rev = MOTOR_STEPS_PER_REV * MICROSTEPPING;

    // ------------------------------------------------------------------
    // 5. Timer0 中断配置 — 用于 STEP 脉冲生成
    // ------------------------------------------------------------------
    // 将 timer 所有权移入全局 Mutex，供 ISR 使用
    interrupt::free(|cs| {
        TIMER.borrow(cs).replace(Some(timer));
    });
    // 使能 Timer0 Alarm0 中断 (NVIC)
    unsafe {
        interrupt::enable(Interrupt::TIMER0_IRQ_0);
    }

    // 延时辅助闭包 — 在临界区内访问 TIMER，确保与 ISR 不冲突
    let delay_ms = |ms: u32| {
        interrupt::free(|cs| {
            if let Some(ref t) = *TIMER.borrow(cs).borrow() {
                t.delay_ms(ms);
            }
        });
    };

    // ------------------------------------------------------------------
    // 6. 使能 TMC2209 驱动器 (ENN 拉低)
    // ------------------------------------------------------------------
    let sio = unsafe { pico_rs::pac::SIO::steal() };
    sio.gpio_out_clr.write_bits(1 << PIN_ENABLE);
    info!("TMC2209 enabled (ENN=LOW)");
    delay_ms(100);

    // ======================================================================
    // 演示 1: 梯形加减速正转 1 圈
    // ======================================================================
    info!(
        "=== Demo 1: Forward {} steps (1 rev @ {}x microstepping) ===",
        steps_per_rev, MICROSTEPPING
    );

    start_trapezoidal_move(steps_per_rev, 0);
    wait_motion_done();

    info!(
        "Demo 1 done. Position: {} steps",
        POSITION.load(Ordering::Relaxed)
    );
    delay_ms(1000);

    // ======================================================================
    // 演示 2: 梯形加减速反转 1 圈
    // ======================================================================
    info!("=== Demo 2: Reverse {} steps ===", steps_per_rev);

    start_trapezoidal_move(steps_per_rev, 1);
    wait_motion_done();

    info!(
        "Demo 2 done. Position: {} steps",
        POSITION.load(Ordering::Relaxed)
    );
    delay_ms(1000);

    // ======================================================================
    // 演示 3: 动态修改运行电流
    //   先降低电流到 ~25% 跑半圈，再恢复正常电流跑回来
    // ======================================================================
    info!("=== Demo 3: Dynamic current adjustment ===");

    // 降低电流: IRUN=8 (~25%), IHOLD=4
    info!("Reducing run current to 8 (~25%)");
    tmc_set_run_current(&uart, 8, 4);
    delay_ms(100);

    start_trapezoidal_move(steps_per_rev / 2, 0);
    wait_motion_done();
    delay_ms(500);

    // 恢复正常电流
    info!(
        "Restoring run current to {} (~{}%)",
        RUN_CURRENT,
        RUN_CURRENT as u32 * 100 / 31
    );
    tmc_set_run_current(&uart, RUN_CURRENT, HOLD_CURRENT);
    delay_ms(100);

    start_trapezoidal_move(steps_per_rev / 2, 1);
    wait_motion_done();

    info!(
        "Demo 3 done. Position: {} steps",
        POSITION.load(Ordering::Relaxed)
    );
    delay_ms(1000);

    // ======================================================================
    // 演示 4: 连续运转 3 秒 + 实时读取 StallGuard 和驱动状态
    // ======================================================================
    info!("=== Demo 4: Continuous run 3s + StallGuard monitoring ===");

    start_continuous_run(0, 1000); // 正转, 1000 Hz

    // 每 0.5 秒读取一次状态 (共 6 次 = 3 秒)
    for i in 0..6u32 {
        delay_ms(500);

        // UART 读取 需要在临界区内访问 timer (用于延时等待回复)
        interrupt::free(|cs| {
            if let Some(ref t) = *TIMER.borrow(cs).borrow() {
                // 读取 StallGuard 结果 (0~510, 越小负载越大)
                if let Some(sg) = tmc_read_sg_result(&uart, t) {
                    info!("[{}] SG_RESULT = {}", i, sg & 0x3FF);
                }

                // 读取驱动状态
                if let Some(status) = tmc_read_drv_status(&uart, t) {
                    let cs_actual = (status >> 16) & 0x1F; // 实际电流缩放值
                    let ola = (status >> 29) & 1; // 开路检测 线圈 A
                    let olb = (status >> 30) & 1; // 开路检测 线圈 B
                    let stst = (status >> 31) & 1; // 静止标志
                    info!(
                        "[{}] DRV_STATUS: cs_actual={}, stst={}, olA={}, olB={}",
                        i, cs_actual, stst, ola, olb
                    );
                }
            }
        });
    }

    stop_motion();

    info!(
        "Demo 4 done. Position: {} steps",
        POSITION.load(Ordering::Relaxed)
    );
    delay_ms(1000);

    // ======================================================================
    // 无限循环: 交替正反转半圈
    // ======================================================================
    info!("=== Loop: alternating forward/reverse ===");

    POSITION.store(0, Ordering::Relaxed);

    let mut forward = true;
    loop {
        let dir = if forward { 0 } else { 1 };
        let dir_str = if forward { "FWD" } else { "REV" };
        info!("[loop] {} {} steps", dir_str, steps_per_rev / 2);

        start_trapezoidal_move(steps_per_rev / 2, dir);
        wait_motion_done();

        info!("[loop] pos={} steps", POSITION.load(Ordering::Relaxed));

        delay_ms(500);
        forward = !forward;
    }
}

// ============================================================================
// Timer0 ISR (Interrupt Service Routine) — STEP 脉冲生成 + 梯形加减速
// ============================================================================
//
// 脉冲生成原理 (两段式中断):
//
//   时间轴: ──┬──────┬────────────────────────┬──────┬─────
//             │ ISR1 │                        │ ISR2 │
//             │  ↓   │                        │  ↓   │
//   STEP:  ___/‾‾‾‾‾‾\________________________/‾‾‾‾‾‾\____
//             │←5μs→│←── step_interval_us ──→│
//             │(脉宽)│    (频率决定)          │
//
//   ISR 第 1 次: PULSE_STATE=false → 拉高 STEP，设 alarm = now + STEP_PULSE_US
//   ISR 第 2 次: PULSE_STATE=true  → 拉低 STEP，计步，更新速度，设 alarm = now + interval
//

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
fn TIMER0_IRQ_0() {
    interrupt::free(|cs| {
        if let Some(ref timer) = *TIMER.borrow(cs).borrow() {
            // 清除中断标志 (必须在处理前清除，否则会重复触发)
            timer.clear_alarm_interrupt(0);

            let phase = phase_from_u32(MOTION_PHASE.load(Ordering::Relaxed));
            if phase == MotionPhase::Idle {
                return; // 空闲状态，不处理
            }

            let sio = unsafe { pico_rs::pac::SIO::steal() };
            let pulse_high = PULSE_STATE.load(Ordering::Relaxed);

            if !pulse_high {
                // ---- 第 1 阶段: 产生 STEP 上升沿 ----
                sio.gpio_out_set.write_bits(1 << PIN_STEP);
                PULSE_STATE.store(true, Ordering::Relaxed);

                // 设置 alarm: STEP_PULSE_US 后执行第 2 阶段 (拉低)
                let now = timer.get_time_us_32();
                timer.set_alarm(0, now.wrapping_add(STEP_PULSE_US));
                return;
            }

            // ---- 第 2 阶段: STEP 下降沿 + 更新运动状态 ----
            sio.gpio_out_clr.write_bits(1 << PIN_STEP);
            PULSE_STATE.store(false, Ordering::Relaxed);

            // 计步 (fetch_add 返回旧值，所以 +1 得到当前步数)
            let step = STEPS_DONE.fetch_add(1, Ordering::Relaxed) + 1;
            let total = STEPS_TARGET.load(Ordering::Relaxed);
            let dir = DIRECTION.load(Ordering::Relaxed);

            // 更新绝对位置
            if dir == 0 {
                POSITION.fetch_add(1, Ordering::Relaxed);
            } else {
                POSITION.fetch_sub(1, Ordering::Relaxed);
            }

            // ---- 连续运转模式: 不检查步数，直接设置下一步 alarm ----
            if phase == MotionPhase::Running {
                let interval = STEP_INTERVAL_US.load(Ordering::Relaxed);
                let now = timer.get_time_us_32();
                timer.set_alarm(0, now.wrapping_add(interval));
                return;
            }

            // ---- 梯形模式: 检查是否到达目标 ----
            if total > 0 && step >= total {
                MOTION_PHASE.store(phase_to_u32(MotionPhase::Idle), Ordering::Relaxed);
                MOTION_DONE.store(true, Ordering::Relaxed);
                sio.gpio_out_clr.write_bits(1 << PIN_STEP);
                return;
            }

            // ---- 梯形模式: 根据当前步数决定运动阶段 ----
            let decel_start = DECEL_START_STEP.load(Ordering::Relaxed);
            let new_phase = if step < ACCEL_STEPS.min(decel_start) {
                MotionPhase::Accelerating // 还未达到加速目标
            } else if step >= decel_start {
                MotionPhase::Decelerating // 进入减速区间
            } else {
                MotionPhase::Cruising // 加速完成，匀速运行
            };

            MOTION_PHASE.store(phase_to_u32(new_phase), Ordering::Relaxed);

            // 根据阶段计算本步间隔并设置 alarm
            let interval = compute_interval(step, total);
            STEP_INTERVAL_US.store(interval, Ordering::Relaxed);

            let now = timer.get_time_us_32();
            timer.set_alarm(0, now.wrapping_add(interval));
        }
    });
}
