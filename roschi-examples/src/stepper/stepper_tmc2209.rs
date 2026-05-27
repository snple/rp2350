//! # TMC2209 步进电机控制 — UART 配置 + stepper_common 梯形加减速
//!
//! 基于 RP2350 (Raspberry Pi Pico 2) 的**步进电机控制器**，
//! 使用 **TMC2209** 驱动芯片，通过 **STEP/DIR** 接口驱动电机，
//! 并通过 **UART** 配置 TMC2209 的运行参数。
//!
//! 与 `stepper.rs` 示例的区别：本文件使用 [`stepper_common`] 公共模块
//! 实现步进脉冲生成和梯形加减速，使得运动控制逻辑可以跨驱动芯片复用。
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
//! 采用 **stepper_common** 模块的**梯形加减速**（Trapezoidal Velocity Profile）：
//! ```text
//!   速度
//!     ↑     ┌───────────────────┐
//!     │    ╱│                   │╲
//!     │   ╱ │                   │ ╲
//!     │  ╱  │      匀速区        │  ╲
//!     │ ╱   │  (MAX_SPEED_HZ)   │   ╲
//!     │╱    │                   │    ╲
//!     └─────┴───────────────────┴─────→ 步数
//!       加速      恒速            减速
//! ```
//!
//! - Timer 中断驱动，每步动态调整间隔时间
//! - 支持梯形加减速运动 (`start_move`) 和连续运转模式 (`start_continuous`)
//! - 多轴独立运行 (本例仅使用 1 轴，可扩展)
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
//! | GP5 | DIAG | DIAG | 堵转检测输出 (开漏, 高 = 堵转) |
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
//!   GP9 (UART1_RX) ──/\/\/──┘  │
//!                      1kΩ     /\
//!                              /\  1kΩ
//!   3.3V (VCC_IO) ──────/\/\/──┘  ← 上拉电阻 (UART 模式必须!)
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
//! ## 2.3 如何让 TMC2209 进入 UART 通信模式
//!
//! TMC2209 的 PDN_UART 引脚有**双重功能**：
//! - 当 PDN_UART = **低电平**时：芯片进入 **Power-Down 待机模式**（默认行为）
//! - 当 PDN_UART = **高电平**时：芯片保持活跃，PDN_UART 同时作为 **UART 通信接口**
//!
//! 要让 TMC2209 正常通过 UART 通信，必须满足**两个条件**：
//!
//! ### 条件 1 (硬件): PDN_UART 上拉电阻
//!
//! PDN_UART 引脚在**上电瞬间**被采样。如果此时为低电平，芯片直接进入待机模式，
//! **完全不响应 UART 命令**。这是最常见的"UART 不通"的原因。
//!
//! 解决方法：在 PDN_UART 上外接 **1kΩ 上拉电阻到 VCC_IO (3.3V)**，
//! 确保上电时 PDN_UART 为高电平：
//!
//! ```text
//!   VCC_IO (3.3V)
//!       │
//!      1kΩ  ← 上拉电阻 (确保上电时 PDN_UART = HIGH)
//!       │
//!       ├──────── PDN_UART (TMC2209)
//!       │
//!      1kΩ ── GP8 (UART1_TX)   Pico 发送
//!       │
//!      1kΩ ── GP9 (UART1_RX)   Pico 接收
//! ```
//!
//! > 注意：很多 TMC2209 模块（如 BigTreeTech/MKS 等）板载已有上拉电阻，
//! > 无需额外添加。请先查看模块原理图确认。
//!
//! ### 条件 2 (软件): 设置 GCONF 寄存器的 pdn_disable 位
//!
//! 上电后 PDN_UART 仍然默认用于 Power-Down 控制。需要通过 UART 写入
//! **GCONF (0x00) 寄存器的 bit 7 (pdn_disable = 1)**，
//! 告诉芯片"PDN_UART 引脚不再控制待机，仅用于 UART 通信"：
//!
//! ```rust
//! // GCONF bit 7: pdn_disable = 1 → PDN_UART 用于 UART 通信
//! // GCONF bit 6: multistep_filt = 1 → STEP 脉冲边沿滤波
//! let gconf: u32 = (1 << 6) | (1 << 7);
//! tmc_write_reg(uart, REG_GCONF, gconf);
//! ```
//!
//! 本示例中 `tmc_configure()` 的步骤 3 已自动完成此操作。
//!
//! ### 故障排查清单
//!
//! | 现象 | 可能原因 | 解决方法 |
//! |------|---------|----------|
//! | 完全无响应 | PDN_UART 无上拉，芯片处于待机 | 加 1kΩ 上拉到 VCC_IO |
//! | 完全无响应 | TX/RX 接线错误或缺少 1kΩ 串联电阻 | 检查接线和电阻 |
//! | 完全无响应 | VM (电机电源) 未供电 | TMC2209 需要 VM 才能工作 |
//! | CRC 校验失败 | 波特率不匹配 | 确认 115200 baud, 8N1 |
//! | 读到全 0xFF | UART 地址不匹配 | 检查 MS1/MS2 引脚与 TMC_SLAVE_ADDR |
//! | 版本号不是 0x21 | 芯片不是 TMC2209 | 确认芯片型号 |
//!
//! ## 2.4 TMC2209 UART 地址设置 (MS1/MS2 引脚)
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
//! # 3. StallGuard4 堵转检测
//!
//! TMC2209 内置 **StallGuard4** 无传感器堵转检测功能，通过监测电机线圈的
//! **反电动势 (back-EMF)** 来估算负载大小，**无需物理限位开关**即可判断电机是否堵转。
//! 典型应用包括无传感器回零 (sensorless homing) 和碰撞保护。
//!
//! ## 3.1 工作原理
//!
//! TMC2209 在每个斩波周期中测量线圈电流的衰减特征：
//! - 电机正常运转时，转子产生的反电动势会影响电流波形
//! - 电机堵转（转子被卡住）时，反电动势消失，电流波形发生变化
//!
//! 芯片将这一差异量化为 **SG_RESULT** 值 (0~510)：
//!
//! | SG_RESULT | 含义 |
//! |-----------|------|
//! | **~510** | 无负载（空转） |
//! | **中值** | 正常负载 |
//! | **接近 0** | 极大负载 / 已堵转 |
//!
//! > **注意**: StallGuard **仅在电机运动时有效**，静止时结果无意义。
//!
//! ## 3.2 三个关键寄存器
//!
//! ### SGTHRS (0x40) — 堵转阈值
//!
//! 触发条件: `SG_RESULT ≤ SGTHRS × 2`
//!
//! - 范围 0–255，**值越大越灵敏**（越容易触发堵转）
//! - 设为 **0** = 禁用堵转检测
//! - 本示例设为 **10**（保守值，不容易误触发）
//! - 当触发条件满足时，TMC2209 的 **DIAG 引脚**被拉高
//!
//! ```rust
//! // SGTHRS = 10 → 当 SG_RESULT ≤ 20 时触发堵转
//! tmc_write_reg(uart, REG_SGTHRS, 10);
//! ```
//!
//! ### TCOOLTHRS (0x14) — 速度激活阈值
//!
//! StallGuard 只在速度高于一定值时才激活（低速时反电动势太小，检测不可靠）：
//!
//! - 激活条件: `TSTEP ≤ TCOOLTHRS`（TSTEP 越小 = 速度越快）
//! - 本示例设为 `0xFFFFF`（最大值），表示**任何速度下都激活**
//! - 实际应用中建议设一个合理阈值，避免低速误触发
//!
//! ```rust
//! tmc_write_reg(uart, REG_TCOOLTHRS, 0xFFFFF);
//! ```
//!
//! ### SG_RESULT (0x41) — 只读结果
//!
//! 范围 0–510，通过 UART 轮询读取。Demo 4 中每 500ms 读取一次：
//!
//! ```rust
//! if let Some(sg) = tmc_read_sg_result(&uart, &timer) {
//!     info!("SG_RESULT = {}", sg & 0x3FF);
//! }
//! ```
//!
//! ## 3.3 两种检测方式
//!
//! | 方式 | 实现 | 延迟 | 适用场景 |
//! |------|------|------|----------|
//! | **UART 轮询 SG_RESULT** | Demo 4 | 取决于轮询间隔 (~500ms) | 调试、负载监控 |
//! | **DIAG 引脚中断** | Demo 5 / Demo 6 | 实时 (~μs 级) | 回零、碰撞急停 |
//!
//! 如果需要**实时堵转急停**（如无传感器回零），应将 TMC2209 的 **DIAG 引脚**
//! 接到 Pico 的一个 GPIO，配置为上升沿中断，在 ISR 中立即停止电机。
//! 本示例的 Demo 5 和 Demo 6 已实现此功能：
//!
//! ```text
//!   TMC2209 DIAG ──────────── Pico GP5 (内部上拉 + 上升沿中断)
//! ```
//!
//! ISR 中的处理流程:
//! 1. `check_and_clear(EdgeHigh)` — 确认并清除中断标志
//! 2. `AXIS.stop()` — 立即停止步进脉冲输出
//! 3. `STALL_DETECTED.store(true)` — 通知主循环
//!
//! ## 3.4 SGTHRS 调参方法
//!
//! StallGuard 阈值需要根据实际工况标定，建议在**匀速段**进行：
//!
//! | 步骤 | 操作 | 示例 SG_RESULT |
//! |------|------|----------------|
//! | 1 | 空载运行，记录典型值 | ~250 |
//! | 2 | 加载到正常工作负载 | ~80 |
//! | 3 | 手动堵住电机 | ~5–15 |
//! | 4 | 设 `SGTHRS = 堵转值 / 2`，留裕量 | SGTHRS = 10 |
//!
//! **注意事项**:
//! - 加速/减速过程中 SG_RESULT 会波动，应在匀速段检测
//! - 微步细分越高，SG_RESULT 分辨率越好
//! - 不同速度下 SG_RESULT 基线不同，建议在目标速度下标定
//! - 温度变化也会影响 SG_RESULT，调参时要留足裕量
//!
//! ## 3.5 与编码器闭环堵转检测的对比
//!
//! 本项目的 `stepper_closed.rs` 使用编码器反馈做软件堵转检测，
//! 与 StallGuard4 硬件检测各有优劣：
//!
//! | | StallGuard4 (TMC2209) | 编码器闭环 |
//! |---|---|---|
//! | 硬件成本 | 无额外硬件 | 需要编码器 |
//! | 检测精度 | 中等，受速度/温度影响 | 高，直接测位置 |
//! | 响应速度 | DIAG 中断: μs 级 | 取决于控制环周期 |
//! | 适用场景 | 回零、碰撞保护 | 精密定位、力控 |
//!
//! ---
//!
//! # 4. 配置参数说明
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
//! | `HOMING_SPEED_HZ` | 800 | 回零速度 (Hz)，低速提高堵转检测可靠性 |
//! | `HOMING_SGTHRS` | 30 | 回零 StallGuard 阈值，比正常运行更灵敏 |
//! | `HOMING_BACKOFF_STEPS` | 200 | 回零后反弹步数，离开机械限位 |
//!
//! ---
//!
//! # 5. 演示流程
//!
//! 程序启动后依次执行：
//!
//! 1. **UART 通信验证** — 读取 TMC2209 IOIN 寄存器，确认芯片版本号 (0x21)
//! 2. **芯片配置** — 通过 UART 写入 GCONF / IHOLD_IRUN / CHOPCONF / PWMCONF 等
//! 3. **Demo 1** — 梯形加减速正转 1 圈 (MOTOR_STEPS_PER_REV × MICROSTEPPING 步)
//! 4. **Demo 2** — 梯形加减速反转 1 圈
//! 5. **Demo 3** — 动态修改运行电流：先降低到 25% 跑半圈，再恢复正常跑半圈
//! 6. **Demo 4** — 连续运转 3 秒，每 0.5 秒读取 StallGuard 值和驱动状态 (UART 轮询)
//! 7. **Demo 5** — StallGuard 堵转检测: DIAG 引脚中断急停，手动堵住电机触发
//! 8. **Demo 6** — 无传感器回零: 低速运动至机械限位，反弹后设零点
//! 9. **Demo 7** — 读取全部关键寄存器
//! 10. **循环** — 无限交替正反转半圈
//!
//! ---
//!
//! # 6. 编译运行
//!
//! ```bash
//! cd roschi-examples
//! cargo run --bin stepper_tmc2209 --target thumbv8m.main-none-eabihf
//! ```

#![no_std]
#![no_main]

use roschi_drivers::stepper::{AxisConfig, AxisState, Direction};

use core::sync::atomic::{AtomicBool, Ordering};
use defmt::*;
use defmt_rtt as _;

use roschi::Steal;
use roschi::block::ImageDef;
use roschi::clocks::ClockBuilder;
use roschi::entry;
use roschi::gpio::{EdgeLevel, Gpio, Pin, Pins, Uart1RxPin, Uart1TxPin};
use roschi::interrupt;
use roschi::pac::{Interrupt, Peripherals};
use roschi::ticks::Ticks;
use roschi::timer::Timer0;
use roschi::uart::{DataBits, Parity, StopBits, Uart1};

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

// ============================================================================
// GPIO 引脚分配
// ============================================================================

/// GP2 → TMC2209 STEP 引脚 (步进脉冲)
const PIN_STEP: u32 = 2;

/// GP3 → TMC2209 DIR 引脚 (方向控制)
const PIN_DIR: u32 = 3;

/// GP4 → TMC2209 ENN 引脚 (使能，低有效)
const PIN_ENABLE: u32 = 4;

/// GP5 → TMC2209 DIAG 引脚 (堵转检测输出，开漏，上升沿 = 堵转)
const PIN_DIAG: u32 = 5;

// GP8 → UART1_TX (TMC2209 PDN_UART)  — 由 Uart1::new() 自动配置 AF
// GP9 → UART1_RX (TMC2209 PDN_UART)  — 由 Uart1::new() 自动配置 AF

// ============================================================================
// 堵转检测 / 无传感器回零参数
// ============================================================================

/// 回零速度 (Hz) — 低速运行以提高 StallGuard 检测可靠性
const HOMING_SPEED_HZ: u32 = 800;

/// 回零 StallGuard 阈值 — 比正常运行时更灵敏 (值越大越灵敏)
/// 正常运行 SGTHRS=10，回零时提高到 30 以确保碰到限位后快速触发
const HOMING_SGTHRS: u32 = 30;

/// 回零后反弹步数 — 离开机械限位，避免堵转状态持续
const HOMING_BACKOFF_STEPS: u32 = 200;

// ============================================================================
// TMC2209 寄存器地址定义
// ============================================================================

/// GCONF (0x00) — 全局配置
const REG_GCONF: u8 = 0x00;

/// GSTAT (0x01) — 全局状态 (读后自动清除)
const REG_GSTAT: u8 = 0x01;

/// IFCNT (0x02) — UART 接口写入计数器
const REG_IFCNT: u8 = 0x02;

/// IOIN (0x06) — 输入引脚状态 + 芯片版本
/// bits [31:24] — VERSION (TMC2209 固定为 0x21)
const REG_IOIN: u8 = 0x06;

/// IHOLD_IRUN (0x10) — 保持/运行电流设置
/// bits [4:0]   — IHOLD  (保持电流, 0-31)
/// bits [12:8]  — IRUN   (运行电流, 0-31)
/// bits [19:16] — IHOLDDELAY (过渡延时, 0-15)
const REG_IHOLD_IRUN: u8 = 0x10;

/// TPOWERDOWN (0x11) — 停止后电流降低延时
const REG_TPOWERDOWN: u8 = 0x11;

/// TSTEP (0x12) — 最近一步的实际步进间隔 (只读)
#[allow(dead_code)]
const REG_TSTEP: u8 = 0x12;

/// TCOOLTHRS (0x14) — CoolStep / StallGuard 激活的速度阈值
const REG_TCOOLTHRS: u8 = 0x14;

/// SGTHRS (0x40) — StallGuard 堵转检测阈值
const REG_SGTHRS: u8 = 0x40;

/// SG_RESULT (0x41) — StallGuard 结果值 (只读, 0-510)
const REG_SG_RESULT: u8 = 0x41;

/// CHOPCONF (0x6C) — 斩波器配置
/// bits [27:24] — MRES: 微步分辨率 (0=256 ... 8=全步)
/// bit  28      — intpol: 插值使能
const REG_CHOPCONF: u8 = 0x6C;

/// DRV_STATUS (0x6F) — 驱动器状态 (只读)
/// bits [20:16] — CS_ACTUAL: 实际电流缩放值
/// bit  25      — stst: 静止检测
/// bit  30      — otpw: 过温预警 (~120°C)
/// bit  31      — ot: 过温关断 (~150°C)
const REG_DRV_STATUS: u8 = 0x6F;

/// PWMCONF (0x70) — StealthChop PWM 调谐参数
const REG_PWMCONF: u8 = 0x70;

// ============================================================================
// TMC2209 UART 协议实现
// ============================================================================
//
// TMC2209 使用自定义的单线半双工协议:
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

/// TMC2209 同步字节
const TMC_SYNC: u8 = 0x05;

/// 计算 TMC2209 CRC8 校验值
///
/// CRC-8，生成多项式 0x07 (x^8 + x^2 + x + 1)，初始值 0x00。
/// TMC2209 使用 LSB-first 位处理顺序。
fn tmc_crc8(data: &[u8]) -> u8 {
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

/// 通过 UART 写入 TMC2209 寄存器
///
/// 构造 8 字节写数据报并发送，然后读取并丢弃单线半双工产生的回显。
fn tmc_write_reg<TX: Uart1TxPin, RX: Uart1RxPin>(uart: &Uart1<TX, RX>, reg: u8, value: u32) {
    let mut datagram = [0u8; 8];
    datagram[0] = TMC_SYNC;
    datagram[1] = TMC_SLAVE_ADDR;
    datagram[2] = reg | 0x80; // bit7=1 表示写操作
    datagram[3] = (value >> 24) as u8;
    datagram[4] = (value >> 16) as u8;
    datagram[5] = (value >> 8) as u8;
    datagram[6] = value as u8;
    datagram[7] = tmc_crc8(&datagram[..7]);

    uart.write(&datagram);
    uart.flush();

    // 丢弃回显 (单线半双工拓扑下 TX 发的字节会被 RX 收到)
    for _ in 0..8 {
        let _ = tmc_read_byte_timeout(uart, 2000);
    }
}

/// 通过 UART 读取 TMC2209 寄存器
///
/// 发送 4 字节读请求，丢弃 4 字节回显，等待 8 字节回复，校验 CRC。
fn tmc_read_reg<TX: Uart1TxPin, RX: Uart1RxPin>(
    uart: &Uart1<TX, RX>,
    timer: &Timer0,
    reg: u8,
) -> Option<u32> {
    let mut request = [0u8; 4];
    request[0] = TMC_SYNC;
    request[1] = TMC_SLAVE_ADDR;
    request[2] = reg;
    request[3] = tmc_crc8(&request[..3]);

    uart.write(&request);
    uart.flush();

    // 丢弃 4 字节回显
    for _ in 0..4 {
        let _ = tmc_read_byte_timeout(uart, 2000);
    }

    // 等待 TMC2209 回复 (~75μs 典型延迟)
    timer.delay_us(200);

    // 接收 8 字节回复
    let mut reply = [0u8; 8];
    for slot in reply.iter_mut() {
        match tmc_read_byte_timeout(uart, 5000) {
            Some(b) => *slot = b,
            None => return None,
        }
    }

    // CRC 校验
    let crc = tmc_crc8(&reply[..7]);
    if crc != reply[7] {
        warn!(
            "TMC2209 CRC error: computed {:#04x}, received {:#04x}",
            crc, reply[7]
        );
        return None;
    }

    // 帧头校验
    if reply[0] != TMC_SYNC || reply[1] != 0xFF {
        warn!(
            "TMC2209 bad reply header: [{:#04x}, {:#04x}]",
            reply[0], reply[1]
        );
        return None;
    }

    let value = (reply[3] as u32) << 24
        | (reply[4] as u32) << 16
        | (reply[5] as u32) << 8
        | (reply[6] as u32);

    Some(value)
}

/// UART 单字节读取 (带超时)
fn tmc_read_byte_timeout<TX: Uart1TxPin, RX: Uart1RxPin>(
    uart: &Uart1<TX, RX>,
    timeout_loops: u32,
) -> Option<u8> {
    for _ in 0..timeout_loops {
        if let Some(byte) = uart.read_byte() {
            return Some(byte);
        }
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
/// 1. 读取 IOIN 验证通信和芯片版本
/// 2. 清除 GSTAT 错误标志
/// 3. 配置 GCONF: StealthChop + UART 使能
/// 4. 配置 IHOLD_IRUN: 运行/保持电流
/// 5. 配置 TPOWERDOWN: 停止后电流降低延时
/// 6. 配置 CHOPCONF: 微步细分 + 256 微步插值
/// 7. 配置 PWMCONF: StealthChop 自动调谐
/// 8. 配置 SGTHRS: StallGuard 堵转检测阈值
/// 9. 配置 TCOOLTHRS: CoolStep/StallGuard 激活条件
/// 10. 回读 CHOPCONF 校验写入
/// 11. 读取 IFCNT 确认写入次数
fn tmc_configure<TX: Uart1TxPin, RX: Uart1RxPin>(uart: &Uart1<TX, RX>, timer: &Timer0) {
    // 步骤 1: 验证通信，读取芯片版本
    if let Some(ioin) = tmc_read_reg(uart, timer, REG_IOIN) {
        let version = (ioin >> 24) & 0xFF;
        info!("TMC2209 version: {:#04x} (expect 0x21)", version);
    } else {
        warn!("Failed to read TMC2209 IOIN — check UART wiring and power!");
    }

    // 步骤 2: 清除 GSTAT 错误标志
    tmc_write_reg(uart, REG_GSTAT, 0x07);
    timer.delay_ms(10);

    // 步骤 3: GCONF — StealthChop + UART 使能
    //   bit 6  multistep_filt = 1 → STEP 脉冲边沿滤波
    //   bit 7  pdn_disable    = 1 → 使用 UART 通信 (必要!)
    let gconf: u32 = (1 << 6) | (1 << 7);
    tmc_write_reg(uart, REG_GCONF, gconf);
    timer.delay_ms(10);

    // 步骤 4: IHOLD_IRUN — 电流设置
    let ihold_irun: u32 =
        (HOLD_CURRENT as u32) | ((RUN_CURRENT as u32) << 8) | ((IHOLD_DELAY as u32) << 16);
    tmc_write_reg(uart, REG_IHOLD_IRUN, ihold_irun);
    timer.delay_ms(10);

    // 步骤 5: TPOWERDOWN — 停止后 ~1.4 秒降为保持电流
    tmc_write_reg(uart, REG_TPOWERDOWN, 20);
    timer.delay_ms(10);

    // 步骤 6: CHOPCONF — 斩波器 + 微步配置
    let mres = match MICROSTEPPING {
        256 => 0,
        128 => 1,
        64 => 2,
        32 => 3,
        16 => 4,
        8 => 5,
        4 => 6,
        2 => 7,
        _ => 8,
    };
    // TOFF=3, HSTRT=4, HEND=1, TBL=1, intpol=1 (256 微步插值)
    let chopconf: u32 = 3 | (4 << 4) | (1 << 7) | (1 << 15) | (mres << 24) | (1 << 28);
    tmc_write_reg(uart, REG_CHOPCONF, chopconf);
    timer.delay_ms(10);

    // 步骤 7: PWMCONF — StealthChop 自动调谐
    let pwmconf: u32 = 36 | (14 << 8) | (0 << 16) | (1 << 18) | (1 << 19) | (8 << 24) | (12 << 28);
    tmc_write_reg(uart, REG_PWMCONF, pwmconf);
    timer.delay_ms(10);

    // 步骤 8: SGTHRS — StallGuard 阈值 (保守值)
    tmc_write_reg(uart, REG_SGTHRS, 10);
    timer.delay_ms(10);

    // 步骤 9: TCOOLTHRS — 任意速度下都激活
    tmc_write_reg(uart, REG_TCOOLTHRS, 0xFFFFF);
    timer.delay_ms(10);

    // 步骤 10: 回读 CHOPCONF 校验
    if let Some(readback) = tmc_read_reg(uart, timer, REG_CHOPCONF) {
        info!("TMC2209 CHOPCONF readback: {:#010x}", readback);
        let readback_mres = (readback >> 24) & 0x0F;
        if readback_mres == mres as u32 {
            info!("  MRES verified: 1/{} microstepping", MICROSTEPPING);
        } else {
            warn!("  MRES mismatch! read={}, expected={}", readback_mres, mres);
        }
    }

    // 步骤 11: 读 IFCNT 确认写入计数
    if let Some(ifcnt) = tmc_read_reg(uart, timer, REG_IFCNT) {
        info!("TMC2209 IFCNT = {} (successful UART writes)", ifcnt & 0xFF);
    }

    info!("TMC2209 configuration complete");
}

/// 动态修改运行/保持电流
fn tmc_set_run_current<TX: Uart1TxPin, RX: Uart1RxPin>(
    uart: &Uart1<TX, RX>,
    run_current: u8,
    hold_current: u8,
) {
    let ihold_irun: u32 =
        (hold_current as u32) | ((run_current as u32) << 8) | ((IHOLD_DELAY as u32) << 16);
    tmc_write_reg(uart, REG_IHOLD_IRUN, ihold_irun);
}

/// 读取驱动状态寄存器
fn tmc_read_drv_status<TX: Uart1TxPin, RX: Uart1RxPin>(
    uart: &Uart1<TX, RX>,
    timer: &Timer0,
) -> Option<u32> {
    tmc_read_reg(uart, timer, REG_DRV_STATUS)
}

/// 读取 StallGuard 结果值 (0~510)
fn tmc_read_sg_result<TX: Uart1TxPin, RX: Uart1RxPin>(
    uart: &Uart1<TX, RX>,
    timer: &Timer0,
) -> Option<u32> {
    tmc_read_reg(uart, timer, REG_SG_RESULT)
}

/// 动态修改 StallGuard 堵转阈值
///
/// 触发条件: SG_RESULT ≤ threshold × 2，值越大越灵敏，0 = 禁用
fn tmc_set_sgthrs<TX: Uart1TxPin, RX: Uart1RxPin>(uart: &Uart1<TX, RX>, threshold: u32) {
    tmc_write_reg(uart, REG_SGTHRS, threshold);
}

// ============================================================================
// 全局共享状态
// ============================================================================

/// 步进轴状态 — 使用 stepper_common 公共模块
static AXIS: AxisState = AxisState::new(AxisConfig {
    pin_step: PIN_STEP as u8,
    pin_dir: PIN_DIR as u8,
    alarm: 0,
    start_speed_hz: START_SPEED_HZ,
    accel_steps: ACCEL_STEPS,
    step_pulse_us: STEP_PULSE_US,
    limit_pin: None,
    limit_active_high: false,
    index_pin: None,
    homing_backoff_index: 0,
});

/// 堵转检测标志 — 由 DIAG 引脚中断 (IO_IRQ_BANK0) 置位
static STALL_DETECTED: AtomicBool = AtomicBool::new(false);

#[inline(always)]
fn gpio() -> Steal<Gpio> {
    unsafe { Gpio::steal() }
}

// ============================================================================
// 运动控制辅助函数
// ============================================================================

fn start_trapezoidal_move(steps: u32, dir: u32) {
    let direction = if dir == 0 {
        Direction::Forward
    } else {
        Direction::Reverse
    };
    let timer = unsafe { Timer0::steal() };
    AXIS.start_move(&timer, steps, direction, MAX_SPEED_HZ);
}

fn start_continuous_run(dir: u32, speed_hz: u32) {
    let direction = if dir == 0 {
        Direction::Forward
    } else {
        Direction::Reverse
    };
    let timer = unsafe { Timer0::steal() };
    AXIS.start_continuous(&timer, direction, speed_hz);
}

fn stop_motion() {
    AXIS.stop();
}

fn wait_motion_done() {
    AXIS.wait_done();
}

// ============================================================================
// 程序入口
// ============================================================================

#[entry]
fn main() -> ! {
    info!("Stepper Motor — TMC2209 UART Control (stepper_common)");

    // ------------------------------------------------------------------
    // 1. 外设初始化
    // ------------------------------------------------------------------
    let dp = Peripherals::take().unwrap();

    let clocks = ClockBuilder::new().build(dp.CLOCKS, dp.XOSC, dp.PLL_SYS, dp.PLL_USB);
    info!("Clock system initialized: sys={} Hz", clocks.sys_freq());

    let ticks = Ticks::new(dp.TICKS, &clocks);
    ticks.init_timer0();

    let timer = Timer0::init(dp.TIMER0);
    info!("Timer0 initialized");

    let pins = Pins::new(dp.IO_BANK0, dp.PADS_BANK0);

    // ------------------------------------------------------------------
    // 2. GPIO 配置 — STEP / DIR / ENABLE
    // ------------------------------------------------------------------
    let p2 = pins.pin::<2>();
    p2.into_output();
    p2.set_low();

    let p3 = pins.pin::<3>();
    p3.into_output();
    p3.set_low();

    // ENN 低有效: 初始禁用
    let p4 = pins.pin::<4>();
    p4.into_output();
    p4.set_high();

    // DIAG 开漏输出: 上拉 + 上升沿中断 (堵转时 DIAG 拉高)
    let p5 = pins.pin::<5>();
    p5.into_input_pullup();
    p5.enable_interrupt(EdgeLevel::EdgeHigh);

    info!("GPIO: GP2(STEP) GP3(DIR) GP4(ENN) GP5(DIAG)");

    // ------------------------------------------------------------------
    // 3. UART1 配置 — TMC2209 通信
    //    GP8 = UART1_TX → 1kΩ → PDN_UART
    //    GP9 = UART1_RX → 1kΩ → PDN_UART
    // ------------------------------------------------------------------
    let uart = Uart1::new(
        dp.UART1,
        pins.pin::<8>(),
        pins.pin::<9>(),
        TMC_UART_BAUD,
        clocks.peri_freq(),
    );
    uart.set_format(DataBits::Eight, StopBits::One, Parity::None);
    info!("UART1: {} baud, 8N1 (GP8=TX, GP9=RX)", TMC_UART_BAUD);

    // 等待 TMC2209 上电稳定
    timer.delay_ms(100);

    // ------------------------------------------------------------------
    // 4. 通过 UART 配置 TMC2209
    // ------------------------------------------------------------------
    info!("=== Configuring TMC2209 via UART ===");
    tmc_configure(&uart, &timer);

    let steps_per_rev = MOTOR_STEPS_PER_REV * MICROSTEPPING;
    info!(
        "Steps per revolution: {} ({} full steps x {} microsteps)",
        steps_per_rev, MOTOR_STEPS_PER_REV, MICROSTEPPING
    );

    // ------------------------------------------------------------------
    // 6. Timer0 + IO_IRQ_BANK0 中断配置
    // ------------------------------------------------------------------
    unsafe {
        interrupt::enable(Interrupt::TIMER0_IRQ_0);
        interrupt::enable(Interrupt::IO_IRQ_BANK0);
    }

    let delay_ms = |ms: u32| {
        timer.delay_ms(ms);
    };

    // ------------------------------------------------------------------
    // 7. 使能 TMC2209 (ENN 拉低)
    // ------------------------------------------------------------------
    gpio().set_low(PIN_ENABLE as u8);
    info!("TMC2209 enabled (ENN=LOW)");
    delay_ms(100);

    // ======================================================================
    // 演示 1: 梯形加减速正转 1 圈
    // ======================================================================
    info!(
        "=== Demo 1: Forward {} steps (1 rev @ 1/{} microstepping) ===",
        steps_per_rev, MICROSTEPPING
    );

    start_trapezoidal_move(steps_per_rev, 0);
    wait_motion_done();

    info!("Demo 1 done. Position: {} steps", AXIS.position());
    delay_ms(1000);

    // ======================================================================
    // 演示 2: 梯形加减速反转 1 圈
    // ======================================================================
    info!("=== Demo 2: Reverse {} steps ===", steps_per_rev);

    start_trapezoidal_move(steps_per_rev, 1);
    wait_motion_done();

    info!("Demo 2 done. Position: {} steps", AXIS.position());
    delay_ms(1000);

    // ======================================================================
    // 演示 3: 动态修改运行电流
    //   先降低电流到 ~25% 跑半圈，再恢复正常跑回来
    // ======================================================================
    info!("=== Demo 3: Dynamic current adjustment ===");

    info!("Reducing run current to 8 (~25%)");
    tmc_set_run_current(&uart, 8, 4);
    delay_ms(100);

    start_trapezoidal_move(steps_per_rev / 2, 0);
    wait_motion_done();
    delay_ms(500);

    info!(
        "Restoring run current to {} (~{}%)",
        RUN_CURRENT,
        RUN_CURRENT as u32 * 100 / 31
    );
    tmc_set_run_current(&uart, RUN_CURRENT, HOLD_CURRENT);
    delay_ms(100);

    start_trapezoidal_move(steps_per_rev / 2, 1);
    wait_motion_done();

    info!("Demo 3 done. Position: {} steps", AXIS.position());
    delay_ms(1000);

    // ======================================================================
    // 演示 4: 连续运转 3 秒 + 实时读取 StallGuard 和驱动状态
    // ======================================================================
    info!("=== Demo 4: Continuous run 3s + StallGuard monitoring ===");

    start_continuous_run(0, 1000);

    for i in 0..6u32 {
        delay_ms(500);

        if let Some(sg) = tmc_read_sg_result(&uart, &timer) {
            info!("[{}] SG_RESULT = {}", i, sg & 0x3FF);
        }

        if let Some(status) = tmc_read_drv_status(&uart, &timer) {
            let cs_actual = (status >> 16) & 0x1F;
            let stst = (status >> 31) & 1;
            let ola = (status >> 29) & 1;
            let olb = (status >> 30) & 1;
            let otpw = (status >> 26) & 1;
            let ot = (status >> 25) & 1;
            info!(
                "[{}] DRV_STATUS: cs_actual={}, stst={}, olA={}, olB={}, otpw={}, ot={}",
                i, cs_actual, stst, ola, olb, otpw, ot
            );
        }
    }

    stop_motion();

    let steps_run = AXIS.steps_done();
    info!(
        "Demo 4 done. Ran {} steps in 3s (~{} Hz). Position: {}",
        steps_run,
        steps_run / 3,
        AXIS.position()
    );
    delay_ms(1000);

    // ======================================================================
    // 演示 5: StallGuard 堵转检测 (DIAG 引脚中断)
    //   连续运转，手动堵住电机轴触发堵转中断，自动停机
    // ======================================================================
    info!("=== Demo 5: StallGuard stall detection (DIAG interrupt) ===");
    info!("Block the motor shaft to trigger a stall!");

    // 提高 SGTHRS 灵敏度用于堵转检测演示
    tmc_set_sgthrs(&uart, HOMING_SGTHRS);
    delay_ms(100);

    // 清除堵转标志
    STALL_DETECTED.store(false, Ordering::Relaxed);

    start_continuous_run(0, HOMING_SPEED_HZ);

    // 等待堵转或超时 (最多 10 秒)
    let mut timeout = 0u32;
    while !STALL_DETECTED.load(Ordering::Relaxed) && timeout < 20 {
        delay_ms(500);
        timeout += 1;

        if let Some(sg) = tmc_read_sg_result(&uart, &timer) {
            info!("[stall_detect] SG_RESULT = {}", sg & 0x3FF);
        }
    }

    // AXIS.stop() 已在 ISR 中调用，此处确保停止
    stop_motion();

    if STALL_DETECTED.load(Ordering::Relaxed) {
        info!("STALL DETECTED at position {} steps!", AXIS.position());
        info!("Backing off {} steps...", HOMING_BACKOFF_STEPS);

        delay_ms(200);
        STALL_DETECTED.store(false, Ordering::Relaxed);

        start_trapezoidal_move(HOMING_BACKOFF_STEPS, 1);
        wait_motion_done();

        info!(
            "Demo 5 done. Position after backoff: {} steps",
            AXIS.position()
        );
    } else {
        info!("Demo 5: timeout — no stall detected in 10s");
        info!("Tip: block the motor shaft during the test");
    }

    // 恢复正常 SGTHRS
    tmc_set_sgthrs(&uart, 10);
    delay_ms(1000);

    // ======================================================================
    // 演示 6: 无传感器回零 (Sensorless Homing)
    //   低速向一个方向运动直到碰到机械限位 (堵转)，
    //   然后反弹并将当前位置设为零点
    // ======================================================================
    info!("=== Demo 6: Sensorless homing ===");
    info!("Motor will move forward until hitting a hard stop.");
    info!("If no hard stop exists, it will timeout after 10s.");

    // 步骤 1: 设置回零参数 — 提高灵敏度
    tmc_set_sgthrs(&uart, HOMING_SGTHRS);
    delay_ms(100);

    // 步骤 2: 清除堵转标志，开始低速运动
    STALL_DETECTED.store(false, Ordering::Relaxed);
    start_continuous_run(0, HOMING_SPEED_HZ);

    // 步骤 3: 等待堵转 (DIAG 中断自动停机)
    let mut timeout = 0u32;
    while !STALL_DETECTED.load(Ordering::Relaxed) && timeout < 20 {
        delay_ms(500);
        timeout += 1;
    }
    stop_motion();

    if STALL_DETECTED.load(Ordering::Relaxed) {
        info!("Home position found! (stall at {} steps)", AXIS.position());

        // 步骤 4: 反弹离开限位
        delay_ms(200);
        STALL_DETECTED.store(false, Ordering::Relaxed);

        start_trapezoidal_move(HOMING_BACKOFF_STEPS, 1);
        wait_motion_done();

        // 步骤 5: 将当前位置设为零点
        AXIS.set_position(0);
        info!("Zero position set. Homing complete!");
    } else {
        info!("Homing timeout — no hard stop detected");
    }

    // 恢复正常 SGTHRS
    tmc_set_sgthrs(&uart, 10);
    delay_ms(1000);

    // ======================================================================
    // 演示 7: 读取全部关键寄存器
    // ======================================================================
    info!("=== Demo 7: Register dump ===");

    if let Some(gconf) = tmc_read_reg(&uart, &timer, REG_GCONF) {
        info!("  GCONF     = {:#010x}", gconf);
    }
    if let Some(chopconf) = tmc_read_reg(&uart, &timer, REG_CHOPCONF) {
        info!("  CHOPCONF  = {:#010x}", chopconf);
    }
    if let Some(pwmconf) = tmc_read_reg(&uart, &timer, REG_PWMCONF) {
        info!("  PWMCONF   = {:#010x}", pwmconf);
    }
    if let Some(ioin) = tmc_read_reg(&uart, &timer, REG_IOIN) {
        let version = (ioin >> 24) & 0xFF;
        let enn = ioin & 1;
        let ms1 = (ioin >> 1) & 1;
        let ms2 = (ioin >> 2) & 1;
        info!(
            "  IOIN      = {:#010x} (version={:#04x}, ENN={}, MS1={}, MS2={})",
            ioin, version, enn, ms1, ms2
        );
    }
    if let Some(drv_status) = tmc_read_drv_status(&uart, &timer) {
        info!("  DRV_STATUS= {:#010x}", drv_status);
    }

    delay_ms(1000);

    // ======================================================================
    // 无限循环: 交替正反转半圈
    // ======================================================================
    info!("=== Loop: alternating forward/reverse ===");

    AXIS.set_position(0);

    let mut forward = true;
    loop {
        let dir = if forward { 0 } else { 1 };
        let dir_str = if forward { "FWD" } else { "REV" };
        info!("[loop] {} {} steps", dir_str, steps_per_rev / 2);

        start_trapezoidal_move(steps_per_rev / 2, dir);
        wait_motion_done();

        info!("[loop] pos={} steps", AXIS.position());

        delay_ms(500);
        forward = !forward;
    }
}

// ============================================================================
// Timer0 ISR — STEP 脉冲生成 (由 stepper_common 处理)
// ============================================================================

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
fn TIMER0_IRQ_0() {
    let timer = unsafe { Timer0::steal() };
    AXIS.handle_alarm_isr(&timer);
}

// ============================================================================
// IO_IRQ_BANK0 ISR — TMC2209 DIAG 引脚堵转检测
// ============================================================================

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
fn IO_IRQ_BANK0() {
    // GP5 = TMC2209 DIAG 引脚
    let pin = unsafe { Pin::<5>::steal() };
    if pin.check_and_clear(EdgeLevel::EdgeHigh) {
        // 堵转检测: 立即停止电机 + 设置标志
        AXIS.stop();
        STALL_DETECTED.store(true, Ordering::Relaxed);
    }
}
