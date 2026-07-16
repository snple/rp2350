//! # MCT8316Z 集成无刷电机驱动示例 — I2C 配置 + 无传感器 FOC
//!
//! 基于 RP2350 (Raspberry Pi Pico 2) + **TI MCT8316Z** 集成三相无刷电机驱动器的
//! 电机驱动示例。MCT8316Z 集成了 MOSFET 功率级、栅极驱动器、电流检测、
//! 和完整的无传感器 FOC 控制器，通过 **I2C 接口**配置内部寄存器。
//!
//! 本示例演示：
//! 1. MCT8316Z I2C 寄存器读写（配置电机参数、控制模式、保护阈值）
//! 2. 无传感器 FOC 启动与运行
//! 3. 故障检测、清除与状态监控
//! 4. 速度闭环控制（芯片内置 FOC + 速度环）
//! 5. RP2350 作为上位机通过 I2C 设定目标速度并监控运行状态
//!
//! 其他驱动模式见：
//! - `bldc.rs` — 开环 6 步梯形换向
//! - `bldc_drv8323.rs` — DRV8323 + MCU 侧 FOC
//! - `bldc_foc.rs` — 电压模式 FOC（MCU 侧实现）
//! - `bldc_foc_current.rs` — 电流模式 FOC（MCU 侧实现）
//!
//! ---
//!
//! # 1. MCT8316Z 芯片概述
//!
//! ## 1.1 功能框图
//!
//! ```text
//!  ┌─────────────────────────────────────────────────────────────────────┐
//!  │                        MCT8316Z                                     │
//!  │                                                                     │
//!  │  ┌───────────┐      ┌─────────────────────────────────┐            │
//!  │  │ I2C 接口   │◄───▶│         数字控制核心             │            │
//!  │  │ SDA, SCL   │     │  ┌─────────────────────────┐    │            │
//!  │  └───────────┘      │  │ 无传感器 FOC / 梯形控制  │    │            │
//!  │                     │  │  • 启动算法              │    │            │
//!  │  ┌───────────┐      │  │  • 电流控制环            │    │            │
//!  │  │ SPEED/     │────▶│  │  • 速度控制环            │    │            │
//!  │  │ DRVOFF     │     │  │  • BEMF 观测器           │    │            │
//!  │  │ DIR/FG     │     │  └─────────────────────────┘    │            │
//!  │  │ BRAKE      │     └────────────┬────────────────────┘            │
//!  │  └───────────┘                   │                                 │
//!  │                                  ▼                                 │
//!  │  ┌───────────┐      ┌─────────────────────────────────┐            │
//!  │  │ 内置电流   │◄────│    3 × 集成半桥功率级            │            │
//!  │  │ 检测       │     │    (N-MOSFET, RDS(on) ~160mΩ)  │            │
//!  │  │            │     │                                 │            │
//!  │  └───────────┘      │    OUT_A ──► 电机 Phase A       │            │
//!  │                     │    OUT_B ──► 电机 Phase B       │            │
//!  │  ┌───────────┐      │    OUT_C ──► 电机 Phase C       │            │
//!  │  │ 保护电路   │◄────│                                 │            │
//!  │  │ OCP/OVP   │     └─────────────────────────────────┘            │
//!  │  │ OTP/UVLO  │                                                    │
//!  │  └─────┬─────┘                                                    │
//!  │        ▼                                                          │
//!  │   nFAULT ──► MCU                                                  │
//!  │                                                                    │
//!  │  ┌───────────┐                                                    │
//!  │  │ LDO 3.3V  │──► DVDD (内部数字供电)                              │
//!  │  └───────────┘                                                    │
//!  └─────────────────────────────────────────────────────────────────────┘
//! ```
//!
//! ## 1.2 MCT8316Z 主要特性
//!
//! | 特性 | 参数 |
//! |------|------|
//! | 输入电压范围 | 4.5V ~ 35V |
//! | 持续输出电流 | 8A (封装限制，需散热) |
//! | 峰值电流 | 16A |
//! | MOSFET RDS(on) | ~160mΩ (HS+LS) |
//! | 控制模式 | 无传感器 FOC / 无传感器梯形 / 有传感器(霍尔) |
//! | 通信接口 | I2C (地址可配: 0x01~0x04) |
//! | PWM 调速输入 | SPEED 引脚 (模拟电压 / PWM / I2C 指令) |
//! | 内置保护 | OCP, OVP, OTP, UVLO, 开路负载, 卡转检测 |
//! | 封装 | 40-pin QFN (5mm × 5mm) |
//!
//! ## 1.3 MCT8316Z vs DRV8323 架构对比
//!
//! | 特性 | MCT8316Z（本示例） | DRV8323（bldc_drv8323.rs） |
//! |------|------------------|--------------------------|
//! | MOSFET | **内置**（全集成） | 外部（需自选 MOSFET） |
//! | FOC 控制器 | **内置**（芯片自主运行） | 外部（MCU 软件实现） |
//! | MCU 角色 | 配置 + 监控（轻量） | 实时 FOC 运算（重载） |
//! | MCU 实时性要求 | 低（I2C 配置即可） | 高（10kHz 电流环） |
//! | 通信接口 | I2C | SPI |
//! | 最大电压 | 35V | 60V |
//! | 最大电流 | 8A 持续 | 取决于外部 MOSFET |
//! | 位置传感器 | 可选（内置无传感器） | 需要（编码器/霍尔） |
//! | 灵活性 | 受限（固定算法） | 高（完全自定义） |
//! | PCB 面积 | 小（高集成度） | 大（需外部 MOSFET） |
//! | 典型应用 | 风扇、泵、小功率工具 | 伺服、机器人、大功率 |
//!
//! ---
//!
//! # 2. MCT8316Z I2C 协议
//!
//! ## 2.1 I2C 配置
//!
//! - **I2C 速率**: 标准模式 (100 kHz) 或快速模式 (400 kHz)
//! - **7-bit 地址**: 由 SDA 引脚上电时的外部电阻配置
//!   - 浮空 → 0x01, 下拉 10kΩ → 0x02, 下拉 3.3kΩ → 0x03, 下拉 1kΩ → 0x04
//! - **字节序**: 寄存器数据为大端 (Big-Endian)，高字节先传
//!
//! ## 2.2 写寄存器
//!
//! ```text
//! MCU → MCT8316Z:
//!   START + [I2C_ADDR<<1|W] + [REG_ADDR] + [DATA_MSB] + [DATA_MID] + [DATA_LSB] + STOP
//!
//! MCT8316Z 寄存器为 24-bit (3 字节)，按大端序传输。
//!
//! 示例：写 ISD_CONFIG (0x80)，设定加速斜率
//!   [0x80] [0x00] [0x12] [0x34]
//!    addr   data[23:16] [15:8] [7:0]
//! ```
//!
//! ## 2.3 读寄存器
//!
//! ```text
//! MCU → MCT8316Z:  START + [I2C_ADDR<<1|W] + [REG_ADDR] + REPEATED_START
//! MCT8316Z → MCU:  [I2C_ADDR<<1|R] + [DATA_MSB] + [DATA_MID] + [DATA_LSB] + STOP
//!
//! 示例：读 DEVICE_STATUS (0xE0)
//!   写: [0xE0]
//!   读: [status_b2] [status_b1] [status_b0]   (24-bit BE)
//! ```
//!
//! ## 2.4 EEPROM 写入流程
//!
//! MCT8316Z 的配置寄存器有两个版本：
//! - **Shadow Register** (RAM): 立即生效，掉电丢失
//! - **EEPROM**: 调用 EEPROM 写入后永久保存
//!
//! ```text
//! EEPROM 写入步骤:
//!   1. 写 Shadow Register 到目标值
//!   2. 向 ALGO_CTRL2 (0xA4) 写入 EEPROM 写启用密钥 (0xB6 到 bit[7:0])
//!   3. 向 ALGO_CTRL2 写入 EEPROM 写触发 (0xFE 到 bit[7:0])
//!   4. 等待 >10ms，芯片将 Shadow → EEPROM
//!   5. 写 0x00 关闭 EEPROM 写启用
//! ```
//!
//! ---
//!
//! # 3. MCT8316Z 寄存器映射
//!
//! MCT8316Z 拥有大量可配置寄存器。以下列出关键寄存器，完整列表见数据手册。
//!
//! ## 3.1 寄存器总览
//!
//! | 地址 | 名称 | R/W | 说明 |
//! |------|------|-----|------|
//! | 0x00~0x06 | Fault Status | R | 故障/警告状态 (7 个寄存器) |
//! | 0x60 | ISD_CONFIG | R/W | 初始速度检测配置 |
//! | 0x64 | REV_DRIVE_CONFIG | R/W | 反转/刹车/回生配置 |
//! | 0x68 | MOTOR_STARTUP1 | R/W | 启动参数 (对齐时间、电流) |
//! | 0x6C | MOTOR_STARTUP2 | R/W | 启动参数 (开环加速) |
//! | 0x70 | CLOSED_LOOP1 | R/W | 闭环参数 (换相超前角等) |
//! | 0x74 | CLOSED_LOOP2 | R/W | 闭环参数 (加减速斜率) |
//! | 0x78 | CLOSED_LOOP3 | R/W | 闭环参数 (速度环 PI) |
//! | 0x7C | CLOSED_LOOP4 | R/W | 闭环参数 (电流限制等) |
//! | 0x80 | SPEED_PROFILES1 | R/W | 速度档位 1 / 最大占空比 |
//! | 0x84 | SPEED_PROFILES2 | R/W | 速度档位 2~4 |
//! | 0x88 | SPEED_PROFILES3 | R/W | 速度档位 5~6 |
//! | 0x8C | SPEED_PROFILES4 | R/W | 开环/闭环电流限制 |
//! | 0x90 | FAULT_CONFIG1 | R/W | 故障保护配置 (OCP/OVP 阈值) |
//! | 0x94 | FAULT_CONFIG2 | R/W | 故障保护配置 (重试/锁定模式) |
//! | 0x98 | PIN_CONFIG | R/W | 引脚功能配置 |
//! | 0x9C | DEVICE_CONFIG1 | R/W | 设备配置 (PWM 频率等) |
//! | 0xA0 | DEVICE_CONFIG2 | R/W | 设备配置 (死区/电流增益等) |
//! | 0xA4 | ALGO_CTRL1 | R/W | 算法控制 (FOC/梯形选择等) |
//! | 0xA8 | ALGO_CTRL2 | R/W | 算法控制 + EEPROM 写入控制 |
//! | 0xE0 | DEV_STATUS | R | 设备运行状态 |
//! | 0xE4 | ALGO_STATUS | R | 算法运行状态 (当前状态机) |
//!
//! ## 3.2 Fault Status 寄存器组 (0x00~0x06)
//!
//! | 地址 | 名称 | 关键位域 |
//! |------|------|---------|
//! | 0x00 | IC_STATUS | FAULT(b23), OT_WARN(b22), SPI_ADDR_FLT(b18)... |
//! | 0x02 | STATUS1 | OVP(b22), UVLO(b21), OTP(b20), OCP(b18)... |
//! | 0x04 | STATUS2 | OTW(b23), OTS(b22), OTP_ERR(b21)... |
//! | 0x06 | STATUS3 | BK_FLT(b22), NO_MTR(b21), MTR_LCK(b20)... |
//!
//! ## 3.3 MOTOR_STARTUP1 (0x68) — 启动配置
//!
//! ```text
//! bit[23:22] CURR_ALIGN     — 对齐电流: 00=250mA, 01=500mA, 10=750mA, 11=1A
//! bit[21:19] ALIGN_TIME     — 对齐时间: 000=50ms ... 111=3s
//! bit[18:16] OL_ILIMIT      — 开环电流限制: 000=250mA ... 111=2A
//! bit[15:13] OL_ACC_A1      — 开环段1加速率
//! bit[12:10] OL_ACC_A2      — 开环段2加速率
//! bit[9:7]   SLOW_SLEW_EN   — 慢斜率启用 + 配置
//! bit[6:4]   OPEN_LOOP_SPD  — 进入闭环转速阈值
//! ```
//!
//! ## 3.4 CLOSED_LOOP1 (0x70) — 闭环控制配置
//!
//! ```text
//! bit[23:22] —               保留
//! bit[21:19] COMM_ADVANCE    — 换相超前角: 000=0°, 001=3.75°, ..., 111=26.25°
//! bit[18:15] CLOOP_DEC       — 闭环减速率
//! bit[14:12] BEMF_THR        — BEMF 积分阈值
//! bit[11:8]  MOTOR_KT        — 电机 Kt (力矩常数，用于 IPD)
//! bit[7:5]   FG_SEL          — FG 输出选择 (每转脉冲数)
//! bit[4]     CLOOP_DIS       — 闭环禁用 (仅开环)
//! bit[3:0]   —               保留
//! ```
//!
//! ## 3.5 CLOSED_LOOP3 (0x78) — 速度环 PI 参数
//!
//! ```text
//! bit[23:20] SPD_KP          — 速度环比例增益 (Kp)
//!                               0000=0.0625, 0001=0.125, ..., 1111=512
//! bit[19:16] SPD_KI          — 速度环积分增益 (Ki)
//!                               0000=0.0039, 0001=0.0078, ..., 1111=128
//! bit[15:12] —               保留
//! bit[11:8]  PWM_DUTY_MAX    — 最大 PWM 占空比
//! bit[7:4]   PWM_DUTY_MIN    — 最小 PWM 占空比
//! bit[3:0]   LOW_DUTY_THR    — 低占空比阈值 (低于此值停止)
//! ```
//!
//! ## 3.6 DEVICE_CONFIG1 (0x9C) — 设备配置
//!
//! ```text
//! bit[23:21] PWM_FREQ_SEL    — PWM 开关频率:
//!                               000=25kHz, 001=30kHz, 010=40kHz,
//!                               011=50kHz, 100=75kHz, 101=100kHz
//! bit[20:18] PWM_MODE_SEL    — PWM 模式: 000=PWM调速, 001=模拟调速, 010=I2C调速
//! bit[17:15] SSM_CONFIG      — 展频调制配置 (降 EMI)
//! bit[14:12] SLEW_RATE       — MOSFET 斜率控制
//! bit[11:9]  DEADTIME        — 死区时间配置
//! bit[8:7]   SD_MODE         — 慢衰减/混合衰减模式
//! bit[6:4]   BRAKE_TYPE      — 刹车类型
//! bit[3:0]   —               保留
//! ```
//!
//! ## 3.7 ALGO_CTRL1 (0xA4) — 算法控制
//!
//! ```text
//! bit[23:22] —               保留
//! bit[21]    FOC_EN          — FOC 启用: 0=梯形换向, 1=FOC 正弦换向
//! bit[20:18] SPEED_CMD       — I2C 速度指令 (当 PWM_MODE=I2C 时)
//!                               数值映射到不同速度档位
//! bit[17]    MOTOR_DIR       — 电机方向: 0=正转, 1=反转
//! bit[16]    DRVOFF_CMD      — 驱动关闭: 0=正常, 1=关闭输出
//! bit[15]    BRAKE_CMD       — 刹车: 0=正常, 1=刹车
//! bit[14:8]  SPD_CTRL_I2C    — I2C 速度控制值 (0~127, 映射到 0~100%)
//! bit[7:0]   —               保留/EEPROM 控制
//! ```
//!
//! ## 3.8 ALGO_CTRL2 (0xA8) — 高级算法控制
//!
//! ```text
//! bit[23:16] —               保留
//! bit[15:13] FAST_ISD_EN     — 快速初始速度检测启用
//! bit[12:8]  —               保留
//! bit[7:0]   EEPROM_CMD      — EEPROM 写入控制:
//!                               写 0xB6 → 写启用
//!                               写 0xFE → 触发 Shadow → EEPROM
//!                               写 0x00 → 关闭写启用
//! ```
//!
//! ## 3.9 DEV_STATUS (0xE0) — 设备状态
//!
//! ```text
//! bit[23:20] DEV_STATE       — 设备状态机:
//!                               0000=IDLE, 0001=START, 0010=RUN,
//!                               0011=ALIGN, 0100=IPD, 0101=OL_ACC,
//!                               0110=CL, 0111=BRAKE, 1000=STOP, ...
//! bit[19:16] SPD_CMD_STATUS  — 当前速度指令状态
//! bit[15:0]  BUS_CURRENT     — 总线电流 ADC 读数
//! ```
//!
//! ---
//!
//! # 4. 控制架构
//!
//! MCT8316Z 内部集成了完整的电机控制算法，MCU 只需作为**配置和监控**角色：
//!
//! ```text
//! ┌──────────────────────────────────────────────────────────┐
//! │                 MCT8316Z 内部控制链路                      │
//! │                                                          │
//! │  ┌──────────┐     ┌──────────────────┐                  │
//! │  │ SPEED    │────▶│ 速度指令解析       │                  │
//! │  │ 引脚/I2C │     │ (PWM/模拟/I2C)    │                  │
//! │  └──────────┘     └────────┬─────────┘                  │
//! │                            │                             │
//! │                            ▼                             │
//! │                   ┌────────────────┐                     │
//! │                   │   速度 PI 环    │                     │
//! │                   │  (SPD_KP/KI)   │                     │
//! │                   └───────┬────────┘                     │
//! │                           │                              │
//! │    ┌──────────┐           ▼                              │
//! │    │ BEMF     │   ┌──────────────┐                      │
//! │    │ 观测器   │──▶│ FOC / 梯形    │                      │
//! │    │ (无传感器)│   │ 换向控制器    │                      │
//! │    └──────────┘   └──────┬───────┘                      │
//! │                          │                               │
//! │    ┌──────────┐          ▼                               │
//! │    │ 内置电流  │  ┌──────────────┐                       │
//! │    │ 检测     │─▶│ 电流保护/限制  │                       │
//! │    └──────────┘  └──────┬───────┘                       │
//! │                         │                                │
//! │                    ┌────┴─────┐                          │
//! │                    │ 3×半桥    │                          │
//! │                    │ 功率级    │                          │
//! │                    └────┬─────┘                          │
//! │                         │                                │
//! └──────────────────────────────────────────────────────────┘
//!                           │
//!                      ┌────┴─────┐
//!                      │ BLDC 电机 │
//!                      └──────────┘
//!
//!   RP2350 (MCU) 角色:
//!   ┌───────────────────────────────────────┐
//!   │  1. 上电配置 MCT8316Z 寄存器 (I2C)     │
//!   │  2. 设定目标速度                       │
//!   │  3. 监控故障状态 + nFAULT              │
//!   │  4. 动态调速 (通过 I2C 或 SPEED 引脚)  │
//!   └───────────────────────────────────────┘
//! ```
//!
//! ---
//!
//! # 5. 硬件接线
//!
//! ## 5.1 引脚分配
//!
//! | GPIO | 功能 | 方向 | 说明 |
//! |------|------|------|------|
//! | GP4 | I2C0 SDA | 双向 | MCT8316Z I2C 数据 (需 4.7kΩ 上拉) |
//! | GP5 | I2C0 SCL | 输出 | MCT8316Z I2C 时钟 (需 4.7kΩ 上拉) |
//! | GP6 | nFAULT | 输入 | MCT8316Z 故障输出 (开漏, 低有效) |
//! | GP7 | DRVOFF | 输出 | MCT8316Z 驱动关断 (高=关断, 低=正常) |
//! | GP8 | SPEED (PWM) | 输出 | 可选: PWM 调速输入 |
//! | GP9 | FG | 输入 | 可选: 频率发生器 (转速反馈) |
//! | GP10 | DIR | 输出 | 可选: 电机方向控制 |
//! | GP11 | BRAKE | 输出 | 可选: 刹车控制 |
//!
//! ## 5.2 接线图
//!
//! ```text
//!   RP2350 (Pico 2)                  MCT8316Z 评估模块
//!   ──────────────                   ──────────────────
//!
//!   I2C 通信:
//!   GP4  (SDA)  ──────── 4.7kΩ ──┬── SDA
//!   GP5  (SCL)  ──────── 4.7kΩ ──┤── SCL
//!                        3.3V ───┘   (上拉)
//!
//!   控制信号:
//!   GP6  (nFAULT) ◄────────────── nFAULT (开漏, 3.3V 上拉)
//!   GP7  (DRVOFF)  ───────────── DRVOFF (高=关断输出)
//!   GP8  (SPEED)   ───────────── SPEED  (PWM 调速, 可选)
//!   GP9  (FG)     ◄────────────── FG     (转速脉冲输出, 可选)
//!   GP10 (DIR)     ───────────── DIR    (方向控制, 可选)
//!   GP11 (BRAKE)   ───────────── BRAKE  (刹车, 可选)
//!
//!   电源:
//!   VM (4.5~35V) ────────────── VM (电机电源)
//!   GND ─────────────────────── GND
//!
//!   电机 (3 相):
//!   MCT8316Z OUT_A ──────────── 电机 Phase A
//!   MCT8316Z OUT_B ──────────── 电机 Phase B
//!   MCT8316Z OUT_C ──────────── 电机 Phase C
//! ```
//!
//! > **注意**: MCT8316Z 的 I2C 接口电平为 3.3V，Pico 2 可直接连接。
//! > I2C 总线需要 4.7kΩ 上拉电阻。
//!
//! ---
//!
//! # 6. 参数说明
//!
//! | 参数 | 默认值 | 说明 |
//! |------|--------|------|
//! | `I2C_ADDR` | 0x01 | MCT8316Z I2C 地址 (SDA 浮空) |
//! | `I2C_BAUDRATE` | 400 kHz | I2C 快速模式 |
//! | `PWM_FREQ` | 25 kHz | 内部 PWM 频率 |
//! | `CSA_GAIN` | 0.15 V/A | 电流检测增益 (内部固定) |
//! | `FOC_EN` | true | 启用 FOC 模式 (否则梯形) |
//! | `OL_CURR` | 500 mA | 开环启动电流 |
//! | `ALIGN_TIME` | 500 ms | 转子对齐时间 |
//! | `SPEED_CMD` | I2C | 速度指令来源 |
//!
//! ---
//!
//! # 7. MCT8316Z 启动序列
//!
//! MCT8316Z 使用以下自动启动序列（芯片内部执行）：
//!
//! ```text
//!   ┌───────┐   ┌───────┐   ┌─────────┐   ┌──────────┐   ┌────────┐
//!   │ IDLE  │──▶│ ALIGN │──▶│ OL_ACC  │──▶│ CL (闭环) │──▶│  RUN   │
//!   └───────┘   └───────┘   └─────────┘   └──────────┘   └────────┘
//!     (等待      (转子对齐    (开环加速      (BEMF 锁定     (正常运行
//!      指令)      到 d 轴)     到切换速度)    → 闭环换向)    速度闭环)
//!
//!   时间线 (典型):
//!   0ms         50~500ms     500~2000ms    2000~3000ms     ∞
//!
//!   可选: IPD (初始位置检测) 可在对齐前执行
//! ```
//!
//! ---
//!
//! # 8. 调试指南
//!
//! ## 8.1 初次调试步骤
//!
//! ```text
//! 1. VM 只接低压 (<12V) 并空载
//! 2. 通过 I2C 读 IC_STATUS (0x00) 确认通信正常
//! 3. 配置 MOTOR_STARTUP 参数 (低电流、长对齐时间)
//! 4. 启用 FOC_EN，设低速度指令
//! 5. DRVOFF=0 → 启动电机
//! 6. 观察启动序列: ALIGN → OL_ACC → CL → RUN
//! 7. 成功后再提高速度/电流/电压
//! ```
//!
//! ## 8.2 常见故障排查
//!
//! | 症状 | 检查 | 可能原因 |
//! |------|------|---------|
//! | nFAULT 持续低 | 读 STATUS1/2/3 | OVP/OCP/OTP |
//! | 启动后立刻停 | STATUS3 MTR_LCK | 卡转检测触发,增大启动电流 |
//! | 启动振荡 | MOTOR_STARTUP1 | 对齐电流不足/对齐时间太短 |
//! | 闭环切换失败 | CLOSED_LOOP1 | BEMF 阈值不匹配,换相超前角不对 |
//! | I2C 无响应 | 地址配置 | SDA 上拉电阻值不对,检查地址 |
//! | 电机反转 | ALGO_CTRL1 | MOTOR_DIR 设反,或交换两相 |
//!
//! ---
//!
//! # 9. 编译运行
//!
//! ```bash
//! cd pico-rs-examples
//! cargo run --bin bldc_mct8316z --target thumbv8m.main-none-eabihf
//! ```
//!
//! ---
//!
//! # 10. 参考资料
//!
//! - [MCT8316Z Datasheet](https://www.ti.com/lit/ds/symlink/mct8316z.pdf) — TI SLVSGE8
//! - [MCT8316Z EVM User Guide](https://www.ti.com/lit/ug/slvuct2/slvuct2.pdf)
//! - [RP2350 Datasheet](https://datasheets.raspberrypi.com/rp2350/rp2350-datasheet.pdf)
//! - *Sensorless Trapezoidal Control of BLDC Motors* — TI SPRABQ4

#![no_std]
#![no_main]
#![allow(dead_code)] // MCT8316Z 寄存器定义预留给扩展使用

use defmt::*;
use defmt_rtt as _;

use pico_rs::Delay;
use pico_rs::clocks::ClockBuilder;
use pico_rs::gpio::Pins;
use pico_rs::i2c::{I2c0, I2cBus};
use pico_rs::main;
use pico_rs::pac::Peripherals;
use pico_rs::rt::peripheral::Peripherals as CorePeripherals;
use pico_rs::timer::Timer0; // ============================================================================
// 配置参数
// ============================================================================

/// MCT8316Z I2C 7-bit 地址
/// SDA 浮空 = 0x01, 10kΩ下拉 = 0x02, 3.3kΩ下拉 = 0x03, 1kΩ下拉 = 0x04
const MCT_I2C_ADDR: u8 = 0x01;

/// I2C 时钟频率 (Hz) — 快速模式 400 kHz
const I2C_BAUDRATE: u32 = 400_000;

/// 目标速度 (I2C 速度控制值, 0~127, 映射到 0~100% 占空比)
const TARGET_SPEED: u8 = 64; // ~50%

/// 状态轮询间隔 (ms)
const POLL_INTERVAL_MS: u32 = 500;

// ============================================================================
// MCT8316Z 寄存器地址定义
// ============================================================================

/// MCT8316Z 寄存器地址
///
/// 所有寄存器为 24-bit (3 字节)，大端字节序。
/// 参见数据手册 Table 7-6.
mod mct_reg {
    // ---- 故障/状态寄存器 (只读) ----
    /// IC 状态 (FAULT, OT_WARN 等)
    pub const IC_STATUS: u8 = 0x00;
    /// 故障状态 1 (OVP, UVLO, OTP, OCP 等)
    pub const STATUS1: u8 = 0x02;
    /// 故障状态 2 (OTW, OTS, OTP_ERR 等)
    pub const STATUS2: u8 = 0x04;
    /// 故障状态 3 (BK_FLT, NO_MTR, MTR_LCK 等)
    pub const STATUS3: u8 = 0x06;

    // ---- 配置寄存器 (可读写) ----
    /// 初始速度检测配置
    pub const ISD_CONFIG: u8 = 0x60;
    /// 反转/刹车/回生配置
    pub const REV_DRIVE_CONFIG: u8 = 0x64;
    /// 电机启动参数 1 (对齐电流/时间、开环电流)
    pub const MOTOR_STARTUP1: u8 = 0x68;
    /// 电机启动参数 2 (开环加速率)
    pub const MOTOR_STARTUP2: u8 = 0x6C;
    /// 闭环参数 1 (换相超前角、BEMF 阈值等)
    pub const CLOSED_LOOP1: u8 = 0x70;
    /// 闭环参数 2 (加减速斜率)
    pub const CLOSED_LOOP2: u8 = 0x74;
    /// 闭环参数 3 (速度环 PI、占空比限制)
    pub const CLOSED_LOOP3: u8 = 0x78;
    /// 闭环参数 4 (电流限制设定)
    pub const CLOSED_LOOP4: u8 = 0x7C;
    /// 速度档位 1
    pub const SPEED_PROFILES1: u8 = 0x80;
    /// 速度档位 2
    pub const SPEED_PROFILES2: u8 = 0x84;
    /// 速度档位 3
    pub const SPEED_PROFILES3: u8 = 0x88;
    /// 速度档位 4 (开环/闭环电流限制)
    pub const SPEED_PROFILES4: u8 = 0x8C;
    /// 故障配置 1 (OCP/OVP 阈值)
    pub const FAULT_CONFIG1: u8 = 0x90;
    /// 故障配置 2 (重试/锁定模式)
    pub const FAULT_CONFIG2: u8 = 0x94;
    /// 引脚功能配置
    pub const PIN_CONFIG: u8 = 0x98;
    /// 设备配置 1 (PWM 频率、调速模式等)
    pub const DEVICE_CONFIG1: u8 = 0x9C;
    /// 设备配置 2 (死区、电流增益等)
    pub const DEVICE_CONFIG2: u8 = 0xA0;
    /// 算法控制 1 (FOC 启用、速度指令、方向等)
    pub const ALGO_CTRL1: u8 = 0xA4;
    /// 算法控制 2 (EEPROM 写入控制)
    pub const ALGO_CTRL2: u8 = 0xA8;

    // ---- 运行时状态 (只读) ----
    /// 设备运行状态
    pub const DEV_STATUS: u8 = 0xE0;
    /// 算法运行状态
    pub const ALGO_STATUS: u8 = 0xE4;
}

// ============================================================================
// IC_STATUS (0x00) 位定义
// ============================================================================

mod ic_status {
    /// 全局故障标志 (任意故障 = 1)
    pub const FAULT: u32 = 1 << 23;
    /// 过温警告 (尚未关断)
    pub const OT_WARN: u32 = 1 << 22;
    /// SPI/I2C 地址故障
    pub const SPI_ADDR_FLT: u32 = 1 << 18;
    /// SPI/I2C 通信故障
    pub const SPI_PARITY_FLT: u32 = 1 << 17;
}

// ============================================================================
// STATUS1 (0x02) 位定义
// ============================================================================

mod status1 {
    /// 过压保护
    pub const OVP: u32 = 1 << 22;
    /// 欠压锁定
    pub const UVLO: u32 = 1 << 21;
    /// 内部过温保护
    pub const OTP: u32 = 1 << 20;
    /// 过流保护 (供电侧)
    pub const SUPPLY_OCP: u32 = 1 << 18;
    /// A 相高侧过流
    pub const OCP_HA: u32 = 1 << 17;
    /// A 相低侧过流
    pub const OCP_LA: u32 = 1 << 16;
    /// B 相高侧过流
    pub const OCP_HB: u32 = 1 << 15;
    /// B 相低侧过流
    pub const OCP_LB: u32 = 1 << 14;
    /// C 相高侧过流
    pub const OCP_HC: u32 = 1 << 13;
    /// C 相低侧过流
    pub const OCP_LC: u32 = 1 << 12;
}

// ============================================================================
// STATUS3 (0x06) 位定义
// ============================================================================

mod status3 {
    /// 刹车故障
    pub const BK_FLT: u32 = 1 << 22;
    /// 无电机检测
    pub const NO_MTR: u32 = 1 << 21;
    /// 电机卡转锁定
    pub const MTR_LCK: u32 = 1 << 20;
    /// 异常速度
    pub const ABN_SPEED: u32 = 1 << 19;
    /// 异常 BEMF
    pub const ABN_BEMF: u32 = 1 << 18;
}

// ============================================================================
// MOTOR_STARTUP1 (0x68) 位域
// ============================================================================

mod startup1 {
    /// 对齐电流选择 (bit[23:22])
    pub const CURR_ALIGN_SHIFT: u8 = 22;
    /// 250mA
    pub const CURR_ALIGN_250MA: u32 = 0x0 << 22;
    /// 500mA
    pub const CURR_ALIGN_500MA: u32 = 0x1 << 22;
    /// 750mA
    pub const CURR_ALIGN_750MA: u32 = 0x2 << 22;
    /// 1A
    pub const CURR_ALIGN_1A: u32 = 0x3 << 22;

    /// 对齐时间 (bit[21:19])
    pub const ALIGN_TIME_SHIFT: u8 = 19;
    /// 50ms
    pub const ALIGN_TIME_50MS: u32 = 0x0 << 19;
    /// 100ms
    pub const ALIGN_TIME_100MS: u32 = 0x1 << 19;
    /// 200ms
    pub const ALIGN_TIME_200MS: u32 = 0x2 << 19;
    /// 500ms
    pub const ALIGN_TIME_500MS: u32 = 0x3 << 19;
    /// 1s
    pub const ALIGN_TIME_1S: u32 = 0x4 << 19;
    /// 2s
    pub const ALIGN_TIME_2S: u32 = 0x5 << 19;
    /// 3s
    pub const ALIGN_TIME_3S: u32 = 0x6 << 19;

    /// 开环电流限制 (bit[18:16])
    pub const OL_ILIMIT_SHIFT: u8 = 16;
    /// 250mA
    pub const OL_ILIMIT_250MA: u32 = 0x0 << 16;
    /// 500mA
    pub const OL_ILIMIT_500MA: u32 = 0x1 << 16;
    /// 750mA
    pub const OL_ILIMIT_750MA: u32 = 0x2 << 16;
    /// 1A
    pub const OL_ILIMIT_1A: u32 = 0x3 << 16;
    /// 1.25A
    pub const OL_ILIMIT_1P25A: u32 = 0x4 << 16;
    /// 1.5A
    pub const OL_ILIMIT_1P5A: u32 = 0x5 << 16;
    /// 1.75A
    pub const OL_ILIMIT_1P75A: u32 = 0x6 << 16;
    /// 2A
    pub const OL_ILIMIT_2A: u32 = 0x7 << 16;
}

// ============================================================================
// CLOSED_LOOP1 (0x70) 位域
// ============================================================================

mod closed_loop1 {
    /// 换相超前角 (bit[21:19])
    pub const COMM_ADVANCE_SHIFT: u8 = 19;
    /// 0°
    pub const COMM_ADVANCE_0: u32 = 0x0 << 19;
    /// 3.75°
    pub const COMM_ADVANCE_3P75: u32 = 0x1 << 19;
    /// 7.5°
    pub const COMM_ADVANCE_7P5: u32 = 0x2 << 19;
    /// 11.25°
    pub const COMM_ADVANCE_11P25: u32 = 0x3 << 19;
    /// 15°
    pub const COMM_ADVANCE_15: u32 = 0x4 << 19;
    /// 18.75°
    pub const COMM_ADVANCE_18P75: u32 = 0x5 << 19;
    /// 22.5°
    pub const COMM_ADVANCE_22P5: u32 = 0x6 << 19;
    /// 26.25°
    pub const COMM_ADVANCE_26P25: u32 = 0x7 << 19;

    /// 禁用闭环 (仅开环)
    pub const CLOOP_DIS: u32 = 1 << 4;
}

// ============================================================================
// CLOSED_LOOP3 (0x78) 位域 — 速度环 PI
// ============================================================================

mod closed_loop3 {
    /// 速度 Kp (bit[23:20])
    pub const SPD_KP_SHIFT: u8 = 20;
    /// 速度 Ki (bit[19:16])
    pub const SPD_KI_SHIFT: u8 = 16;
}

// ============================================================================
// DEVICE_CONFIG1 (0x9C) 位域
// ============================================================================

mod dev_config1 {
    /// PWM 频率选择 (bit[23:21])
    pub const PWM_FREQ_SHIFT: u8 = 21;
    /// 25 kHz
    pub const PWM_FREQ_25K: u32 = 0x0 << 21;
    /// 30 kHz
    pub const PWM_FREQ_30K: u32 = 0x1 << 21;
    /// 40 kHz
    pub const PWM_FREQ_40K: u32 = 0x2 << 21;
    /// 50 kHz
    pub const PWM_FREQ_50K: u32 = 0x3 << 21;
    /// 75 kHz
    pub const PWM_FREQ_75K: u32 = 0x4 << 21;
    /// 100 kHz
    pub const PWM_FREQ_100K: u32 = 0x5 << 21;

    /// 速度输入模式 (bit[20:18])
    pub const PWM_MODE_SHIFT: u8 = 18;
    /// PWM 调速
    pub const PWM_MODE_PWM: u32 = 0x0 << 18;
    /// 模拟电压调速
    pub const PWM_MODE_ANALOG: u32 = 0x1 << 18;
    /// I2C 指令调速
    pub const PWM_MODE_I2C: u32 = 0x2 << 18;
}

// ============================================================================
// ALGO_CTRL1 (0xA4) 位域
// ============================================================================

mod algo_ctrl1 {
    /// FOC 启用 (bit21): 0=梯形, 1=FOC
    pub const FOC_EN: u32 = 1 << 21;
    /// 电机方向 (bit17): 0=正转, 1=反转
    pub const MOTOR_DIR: u32 = 1 << 17;
    /// 驱动关闭 (bit16): 0=正常, 1=关断
    pub const DRVOFF_CMD: u32 = 1 << 16;
    /// 刹车 (bit15): 0=正常, 1=刹车
    pub const BRAKE_CMD: u32 = 1 << 15;
    /// I2C 速度控制值偏移 (bit[14:8])
    pub const SPD_CTRL_SHIFT: u8 = 8;
    pub const SPD_CTRL_MASK: u32 = 0x7F << 8;
}

// ============================================================================
// ALGO_CTRL2 (0xA8) 位域 — EEPROM 控制
// ============================================================================

mod algo_ctrl2 {
    /// EEPROM 写启用密钥
    pub const EEPROM_WRITE_KEY: u8 = 0xB6;
    /// EEPROM 写触发
    pub const EEPROM_WRITE_TRIGGER: u8 = 0xFE;
    /// EEPROM 写关闭
    pub const EEPROM_WRITE_CLOSE: u8 = 0x00;
}

// ============================================================================
// DEV_STATUS (0xE0) 位域
// ============================================================================

mod dev_status {
    /// 设备状态机 (bit[23:20])
    pub const DEV_STATE_SHIFT: u8 = 20;
    pub const DEV_STATE_MASK: u32 = 0xF << 20;

    // 状态机值
    pub const STATE_IDLE: u32 = 0x0;
    pub const STATE_START: u32 = 0x1;
    pub const STATE_RUN: u32 = 0x2;
    pub const STATE_ALIGN: u32 = 0x3;
    pub const STATE_IPD: u32 = 0x4;
    pub const STATE_OL_ACC: u32 = 0x5;
    pub const STATE_CL: u32 = 0x6;
    pub const STATE_BRAKE: u32 = 0x7;
    pub const STATE_STOP: u32 = 0x8;
    pub const STATE_FAULT: u32 = 0x9;
}

// ============================================================================
// MCT8316Z I2C 驱动封装
// ============================================================================

/// MCT8316Z I2C 驱动器
///
/// 封装了 I2C 寄存器读写操作。MCT8316Z 寄存器为 24-bit (3 字节)，大端序。
///
/// I2C 写: [REG_ADDR] + [DATA_MSB] + [DATA_MID] + [DATA_LSB]
/// I2C 读: Write [REG_ADDR] + Repeated Start + Read [3 bytes]
struct Mct8316z<'a, I2C: I2cBus> {
    i2c: &'a I2C,
    addr: u8,
}

impl<'a, I2C: I2cBus> Mct8316z<'a, I2C> {
    /// 创建 MCT8316Z 驱动实例
    fn new(i2c: &'a I2C, addr: u8) -> Self {
        Self { i2c, addr }
    }

    /// 写 MCT8316Z 寄存器 (24-bit, 大端序)
    ///
    /// I2C 事务: START + [addr|W] + [reg] + [data_b2] + [data_b1] + [data_b0] + STOP
    fn write_reg(&self, reg: u8, data: u32) -> bool {
        let buf = [
            reg,
            ((data >> 16) & 0xFF) as u8, // MSB
            ((data >> 8) & 0xFF) as u8,  // MID
            (data & 0xFF) as u8,         // LSB
        ];
        self.i2c.write(self.addr, &buf).is_ok()
    }

    /// 读 MCT8316Z 寄存器 (24-bit, 大端序)
    ///
    /// I2C 事务:
    ///   Write: [reg]
    ///   Repeated Start + Read: [data_b2] [data_b1] [data_b0]
    fn read_reg(&self, reg: u8) -> Option<u32> {
        let mut buf = [0u8; 3];
        match self.i2c.write_read(self.addr, &[reg], &mut buf) {
            Ok(_) => {
                let val = ((buf[0] as u32) << 16) | ((buf[1] as u32) << 8) | (buf[2] as u32);
                Some(val)
            }
            Err(_) => None,
        }
    }

    /// 修改寄存器中的某些位域 (read-modify-write)
    ///
    /// `mask`: 要修改的位对应为 1
    /// `value`: 要写入的新值 (在 mask 对应位)
    fn modify_reg(&self, reg: u8, mask: u32, value: u32) -> bool {
        if let Some(current) = self.read_reg(reg) {
            let new_val = (current & !mask) | (value & mask);
            self.write_reg(reg, new_val)
        } else {
            false
        }
    }

    /// 检查 I2C 通信是否正常（通过读取 IC_STATUS）
    fn check_communication(&self) -> bool {
        self.read_reg(mct_reg::IC_STATUS).is_some()
    }

    /// 读取故障状态并打印详情
    ///
    /// 返回 IC_STATUS 中的 FAULT 位是否置位
    fn check_faults(&self) -> bool {
        let ic_status = self.read_reg(mct_reg::IC_STATUS).unwrap_or(0);
        let has_fault = (ic_status & ic_status::FAULT) != 0;

        if has_fault {
            warn!("MCT8316Z FAULT! IC_STATUS=0x{:06x}", ic_status);

            if ic_status & ic_status::OT_WARN != 0 {
                warn!("  OT_WARN (过温警告)");
            }

            // 读 STATUS1
            if let Some(s1) = self.read_reg(mct_reg::STATUS1)
                && s1 != 0
            {
                warn!("  STATUS1=0x{:06x}", s1);
                if s1 & status1::OVP != 0 {
                    warn!("    OVP (过压)");
                }
                if s1 & status1::UVLO != 0 {
                    warn!("    UVLO (欠压)");
                }
                if s1 & status1::OTP != 0 {
                    warn!("    OTP (过温关断)");
                }
                if s1 & status1::SUPPLY_OCP != 0 {
                    warn!("    SUPPLY_OCP (供电过流)");
                }
            }

            // 读 STATUS3 (电机相关故障)
            if let Some(s3) = self.read_reg(mct_reg::STATUS3)
                && s3 != 0
            {
                warn!("  STATUS3=0x{:06x}", s3);
                if s3 & status3::NO_MTR != 0 {
                    warn!("    NO_MTR (未检测到电机)");
                }
                if s3 & status3::MTR_LCK != 0 {
                    warn!("    MTR_LCK (电机卡转)");
                }
                if s3 & status3::ABN_SPEED != 0 {
                    warn!("    ABN_SPEED (异常速度)");
                }
                if s3 & status3::ABN_BEMF != 0 {
                    warn!("    ABN_BEMF (异常反电动势)");
                }
            }
        }

        has_fault
    }

    /// 清除故障 — 通过将 DRVOFF 拉高再拉低来复位
    ///
    /// MCT8316Z 不像 DRV8323 有 CLR_FLT 位，
    /// 清除故障的方式是:
    ///   1. DRVOFF=1 (关闭驱动)
    ///   2. 等待 >1ms
    ///   3. DRVOFF=0 (重新启用)
    ///
    /// 或通过 I2C 写 ALGO_CTRL1 的 DRVOFF_CMD 位
    fn clear_fault_via_i2c(&self) -> bool {
        // 设 DRVOFF_CMD = 1
        if !self.modify_reg(
            mct_reg::ALGO_CTRL1,
            algo_ctrl1::DRVOFF_CMD,
            algo_ctrl1::DRVOFF_CMD,
        ) {
            return false;
        }
        // 需要外部延时，这里仅设置标志
        // 调用者需在此后延时 >1ms 再调用 enable_drive()
        true
    }

    /// 启用驱动输出 (DRVOFF_CMD = 0)
    fn enable_drive(&self) -> bool {
        self.modify_reg(mct_reg::ALGO_CTRL1, algo_ctrl1::DRVOFF_CMD, 0)
    }

    /// 禁用驱动输出 (DRVOFF_CMD = 1)
    fn disable_drive(&self) -> bool {
        self.modify_reg(
            mct_reg::ALGO_CTRL1,
            algo_ctrl1::DRVOFF_CMD,
            algo_ctrl1::DRVOFF_CMD,
        )
    }

    /// 设定目标速度 (I2C 调速模式)
    ///
    /// `speed`: 0~127, 映射到 0~100% 占空比
    fn set_speed(&self, speed: u8) -> bool {
        let speed_val = (speed.min(127) as u32) << algo_ctrl1::SPD_CTRL_SHIFT;
        self.modify_reg(mct_reg::ALGO_CTRL1, algo_ctrl1::SPD_CTRL_MASK, speed_val)
    }

    /// 设定电机方向
    ///
    /// `reverse`: true = 反转, false = 正转
    fn set_direction(&self, reverse: bool) -> bool {
        let val = if reverse { algo_ctrl1::MOTOR_DIR } else { 0 };
        self.modify_reg(mct_reg::ALGO_CTRL1, algo_ctrl1::MOTOR_DIR, val)
    }

    /// 刹车
    fn brake(&self) -> bool {
        self.modify_reg(
            mct_reg::ALGO_CTRL1,
            algo_ctrl1::BRAKE_CMD,
            algo_ctrl1::BRAKE_CMD,
        )
    }

    /// 释放刹车
    fn release_brake(&self) -> bool {
        self.modify_reg(mct_reg::ALGO_CTRL1, algo_ctrl1::BRAKE_CMD, 0)
    }

    /// 获取设备状态机当前状态
    ///
    /// 返回值: STATE_IDLE, STATE_RUN, STATE_FAULT 等
    fn device_state(&self) -> Option<u32> {
        self.read_reg(mct_reg::DEV_STATUS)
            .map(|v| (v & dev_status::DEV_STATE_MASK) >> dev_status::DEV_STATE_SHIFT)
    }

    /// 获取设备状态名称 (用于日志)
    fn state_name(state: u32) -> &'static str {
        match state {
            0x0 => "IDLE",
            0x1 => "START",
            0x2 => "RUN",
            0x3 => "ALIGN",
            0x4 => "IPD",
            0x5 => "OL_ACC",
            0x6 => "CL",
            0x7 => "BRAKE",
            0x8 => "STOP",
            0x9 => "FAULT",
            _ => "UNKNOWN",
        }
    }

    /// 完整配置 MCT8316Z
    ///
    /// 配置内容:
    /// - 无传感器 FOC 模式
    /// - I2C 调速
    /// - PWM 25kHz
    /// - 适中的启动参数
    /// - 合理的保护阈值
    fn configure(&self) -> bool {
        info!("Configuring MCT8316Z...");

        // ------------------------------------------------------------------
        // DEVICE_CONFIG1 (0x9C):
        //   - PWM 频率 = 25 kHz
        //   - 速度输入 = I2C 模式 (MCU 通过 I2C 设速度)
        // ------------------------------------------------------------------
        let dc1_mask: u32 = 0x7 << dev_config1::PWM_FREQ_SHIFT | 0x7 << dev_config1::PWM_MODE_SHIFT;
        let dc1_val: u32 = dev_config1::PWM_FREQ_25K | dev_config1::PWM_MODE_I2C;
        if !self.modify_reg(mct_reg::DEVICE_CONFIG1, dc1_mask, dc1_val) {
            error!("Failed to write DEVICE_CONFIG1");
            return false;
        }

        // ------------------------------------------------------------------
        // MOTOR_STARTUP1 (0x68):
        //   - 对齐电流 = 500mA (适中，避免过大振动)
        //   - 对齐时间 = 500ms (给足时间让转子对齐)
        //   - 开环电流 = 750mA
        // ------------------------------------------------------------------
        let su1_mask: u32 = 0x3 << startup1::CURR_ALIGN_SHIFT
            | 0x7 << startup1::ALIGN_TIME_SHIFT
            | 0x7 << startup1::OL_ILIMIT_SHIFT;
        let su1_val: u32 =
            startup1::CURR_ALIGN_500MA | startup1::ALIGN_TIME_500MS | startup1::OL_ILIMIT_750MA;
        if !self.modify_reg(mct_reg::MOTOR_STARTUP1, su1_mask, su1_val) {
            error!("Failed to write MOTOR_STARTUP1");
            return false;
        }

        // ------------------------------------------------------------------
        // CLOSED_LOOP1 (0x70):
        //   - 换相超前角 = 7.5° (通常需根据电机调整)
        //   - 闭环启用 (CLOOP_DIS = 0)
        // ------------------------------------------------------------------
        let cl1_mask: u32 = 0x7 << 19 | closed_loop1::CLOOP_DIS;
        let cl1_val: u32 = closed_loop1::COMM_ADVANCE_7P5; // CLOOP_DIS=0
        if !self.modify_reg(mct_reg::CLOSED_LOOP1, cl1_mask, cl1_val) {
            error!("Failed to write CLOSED_LOOP1");
            return false;
        }

        // ------------------------------------------------------------------
        // CLOSED_LOOP3 (0x78):
        //   - 速度 Kp = 4 (编码 0100, 中等增益)
        //   - 速度 Ki = 4 (编码 0100, 中等增益)
        // ------------------------------------------------------------------
        let cl3_mask: u32 = 0xF << closed_loop3::SPD_KP_SHIFT | 0xF << closed_loop3::SPD_KI_SHIFT;
        let cl3_val: u32 =
            (0x4 << closed_loop3::SPD_KP_SHIFT) | (0x4 << closed_loop3::SPD_KI_SHIFT);
        if !self.modify_reg(mct_reg::CLOSED_LOOP3, cl3_mask, cl3_val) {
            error!("Failed to write CLOSED_LOOP3");
            return false;
        }

        // ------------------------------------------------------------------
        // ALGO_CTRL1 (0xA4):
        //   - FOC 启用
        //   - 方向 = 正转
        //   - DRVOFF = 1 (先保持关闭，等配置完再开)
        //   - 速度 = 0 (待后续设定)
        // ------------------------------------------------------------------
        let ac1_mask: u32 = algo_ctrl1::FOC_EN
            | algo_ctrl1::MOTOR_DIR
            | algo_ctrl1::DRVOFF_CMD
            | algo_ctrl1::SPD_CTRL_MASK;
        let ac1_val: u32 = algo_ctrl1::FOC_EN | algo_ctrl1::DRVOFF_CMD; // FOC + 先关闭驱动
        if !self.modify_reg(mct_reg::ALGO_CTRL1, ac1_mask, ac1_val) {
            error!("Failed to write ALGO_CTRL1");
            return false;
        }

        info!("MCT8316Z configuration complete");

        // 回读验证关键寄存器
        if let Some(dc1) = self.read_reg(mct_reg::DEVICE_CONFIG1) {
            info!("  DEVICE_CONFIG1 = 0x{:06x}", dc1);
        }
        if let Some(su1) = self.read_reg(mct_reg::MOTOR_STARTUP1) {
            info!("  MOTOR_STARTUP1 = 0x{:06x}", su1);
        }
        if let Some(cl1) = self.read_reg(mct_reg::CLOSED_LOOP1) {
            info!("  CLOSED_LOOP1   = 0x{:06x}", cl1);
        }
        if let Some(ac1) = self.read_reg(mct_reg::ALGO_CTRL1) {
            info!("  ALGO_CTRL1     = 0x{:06x}", ac1);
        }

        true
    }

    /// 将 Shadow 寄存器写入 EEPROM (掉电永久保存)
    ///
    /// 步骤:
    /// 1. 写入 EEPROM 写启用密钥 (0xB6)
    /// 2. 触发 EEPROM 写入 (0xFE)
    /// 3. 等待 >10ms
    /// 4. 关闭 EEPROM 写启用 (0x00)
    fn save_to_eeprom(&self, delay: &Delay) -> bool {
        info!("Saving configuration to EEPROM...");

        // 写启用
        if !self.modify_reg(
            mct_reg::ALGO_CTRL2,
            0xFF,
            algo_ctrl2::EEPROM_WRITE_KEY as u32,
        ) {
            error!("EEPROM write-enable failed");
            return false;
        }

        // 触发写入
        if !self.modify_reg(
            mct_reg::ALGO_CTRL2,
            0xFF,
            algo_ctrl2::EEPROM_WRITE_TRIGGER as u32,
        ) {
            error!("EEPROM write-trigger failed");
            return false;
        }

        // 等待写入完成
        delay.delay_ms(20);

        // 关闭写启用
        self.modify_reg(
            mct_reg::ALGO_CTRL2,
            0xFF,
            algo_ctrl2::EEPROM_WRITE_CLOSE as u32,
        );

        info!("EEPROM save complete");
        true
    }
}

// ============================================================================
// 入口
// ============================================================================

#[main]
fn main() -> ! {
    info!("BLDC MCT8316Z — I2C config + sensorless FOC");

    // ------------------------------------------------------------------
    // 外设初始化
    // ------------------------------------------------------------------
    let dp = Peripherals::take().unwrap();
    let clocks = ClockBuilder::new().build(dp.CLOCKS, dp.XOSC, dp.PLL_SYS, dp.PLL_USB);
    let sys_freq = clocks.sys_freq();
    info!("System clock: {} MHz", sys_freq / 1_000_000);

    let pins = Pins::init(dp.IO_BANK0, dp.PADS_BANK0);

    // SysTick delay
    let core = CorePeripherals::take().unwrap();
    let delay = Delay::init(core.SYST, clocks.sys_freq());

    // ------------------------------------------------------------------
    // I2C0 初始化 — GP4 (SDA), GP5 (SCL), 400 kHz
    // ------------------------------------------------------------------
    let i2c = I2c0::init(
        dp.I2C0,
        pins.pin::<4>(), // SDA
        pins.pin::<5>(), // SCL
        I2C_BAUDRATE,
        &clocks,
    );
    info!("I2C0 initialized: {} kHz", I2C_BAUDRATE / 1000);

    // ------------------------------------------------------------------
    // 控制引脚初始化
    // ------------------------------------------------------------------

    // nFAULT — 开漏输出，需上拉，低有效
    let mut nfault = pins.pin::<6>();
    nfault.into_input_pullup();

    // DRVOFF — 高有效 (高=关断驱动输出)
    // 上电默认拉高，保证电机不会意外启动
    let mut drvoff = pins.pin::<7>();
    drvoff.into_output();
    drvoff.set_high(); // 先关断驱动

    // DIR — 方向控制 (可选引脚控制)
    let mut dir_pin = pins.pin::<10>();
    dir_pin.into_output();
    dir_pin.set_low(); // 正转

    // BRAKE — 刹车控制 (可选)
    let mut brake_pin = pins.pin::<11>();
    brake_pin.into_output();
    brake_pin.set_low(); // 不刹车

    // Timer for general timing
    let _timer = Timer0::init(dp.TIMER0);

    // ------------------------------------------------------------------
    // MCT8316Z 初始化
    // ------------------------------------------------------------------
    let mct = Mct8316z::new(&i2c, MCT_I2C_ADDR);

    // 等待 MCT8316Z 上电就绪 (~10ms)
    delay.delay_ms(20);

    // 检查 I2C 通信
    if !mct.check_communication() {
        error!("MCT8316Z I2C communication failed!");
        error!(
            "Check: I2C address (0x{:02x}), wiring, pull-ups",
            MCT_I2C_ADDR
        );
        loop {
            delay.delay_ms(1000);
        }
    }
    info!(
        "MCT8316Z I2C communication OK (addr=0x{:02x})",
        MCT_I2C_ADDR
    );

    // 检查初始故障状态
    if mct.check_faults() {
        warn!("Clearing startup faults...");
        // 通过 DRVOFF 引脚复位故障
        drvoff.set_high();
        delay.delay_ms(5);
        drvoff.set_low();
        delay.delay_ms(10);

        // 再次检查
        if mct.check_faults() {
            error!("Faults persist after clear — check hardware");
        }
    }

    // ------------------------------------------------------------------
    // 配置 MCT8316Z
    //   - FOC 模式
    //   - I2C 调速
    //   - 25kHz PWM
    //   - 启动参数: 500mA 对齐, 500ms, 750mA 开环
    // ------------------------------------------------------------------
    if !mct.configure() {
        error!("MCT8316Z configuration failed");
        loop {
            delay.delay_ms(1000);
        }
    }

    // 读取所有故障寄存器，确认配置后无故障
    if mct.check_faults() {
        error!("Fault detected after configuration");
    }

    // ------------------------------------------------------------------
    // 启动电机
    // ------------------------------------------------------------------
    info!("Starting motor...");

    // 设定目标速度
    mct.set_speed(TARGET_SPEED);
    info!(
        "Target speed set: {}/127 (~{}%)",
        TARGET_SPEED,
        TARGET_SPEED as u32 * 100 / 127
    );

    // 启用驱动输出 (通过 I2C 清除 DRVOFF_CMD 位)
    mct.enable_drive();

    // 同时释放硬件 DRVOFF 引脚
    drvoff.set_low();
    info!("Drive enabled, DRVOFF=LOW");

    // 检查 nFAULT
    if !nfault.get() {
        error!("nFAULT LOW immediately after enable — check hardware!");
        mct.check_faults();
    }

    // ------------------------------------------------------------------
    // 主循环 — 状态监控 + 日志
    //
    // MCT8316Z 内部自主运行 FOC，MCU 只需监控状态
    // ------------------------------------------------------------------
    info!("Entering monitor loop ({}ms interval)", POLL_INTERVAL_MS);

    let mut loop_count: u32 = 0;
    let mut last_state: u32 = 0xFF; // 不存在的状态，确保首次打印

    loop {
        delay.delay_ms(POLL_INTERVAL_MS);
        loop_count += 1;

        // ==============================================================
        // 1. 检查 nFAULT 引脚 (快速故障检测)
        // ==============================================================
        if !nfault.get() {
            warn!("nFAULT LOW detected!");
            mct.check_faults();

            // 可选: 尝试自动恢复
            // mct.clear_fault_via_i2c();
            // delay.delay_ms(5);
            // mct.enable_drive();
        }

        // ==============================================================
        // 2. 读取设备状态机
        // ==============================================================
        if let Some(state) = mct.device_state()
            && state != last_state
        {
            info!(
                "State: {} → {} ({})",
                Mct8316z::<I2c0<pico_rs::gpio::Pin<4>, pico_rs::gpio::Pin<5>>>::state_name(
                    last_state
                ),
                Mct8316z::<I2c0<pico_rs::gpio::Pin<4>, pico_rs::gpio::Pin<5>>>::state_name(state),
                state
            );
            last_state = state;
        }

        // ==============================================================
        // 3. 读取完整状态 (每隔 ~2 秒)
        // ==============================================================
        if loop_count.is_multiple_of(2000 / POLL_INTERVAL_MS)
            && let Some(dev_stat) = mct.read_reg(mct_reg::DEV_STATUS)
        {
            let state = (dev_stat >> dev_status::DEV_STATE_SHIFT) & 0xF;
            let bus_current = dev_stat & 0xFFFF;
            info!(
                "DEV_STATUS: state={} bus_current_raw={} nFAULT={}",
                Mct8316z::<I2c0<pico_rs::gpio::Pin<4>, pico_rs::gpio::Pin<5>>>::state_name(state),
                bus_current,
                nfault.get()
            );
        }

        // ==============================================================
        // 4. 演示: 动态变速 (每 10 秒切换速度)
        //
        // 可选功能演示 — 去掉注释即可启用
        // ==============================================================
        // if loop_count % (10_000 / POLL_INTERVAL_MS) == 0 {
        //     let speeds = [32, 64, 96, 127]; // 25%, 50%, 75%, 100%
        //     let idx = ((loop_count / (10_000 / POLL_INTERVAL_MS)) as usize) % speeds.len();
        //     let new_speed = speeds[idx];
        //     mct.set_speed(new_speed);
        //     info!("Speed changed to {}/127 (~{}%)", new_speed, new_speed as u32 * 100 / 127);
        // }
    }
}
