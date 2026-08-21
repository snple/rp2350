//! # DRV8323 三相无刷电机驱动示例 — SPI 配置 + FOC 电流闭环 + SVPWM
//!
//! 基于 RP2350 (Raspberry Pi Pico 2) + **TI DRV8323** 三相智能栅极驱动器的
//! **电流模式 FOC（磁场定向控制）** 无刷电机驱动示例。
//!
//! DRV8323 集成了三相半桥栅极驱动器、双通道电流检测放大器、
//! 稳压器和保护电路，通过 SPI 接口配置内部寄存器。
//!
//! 本示例演示：
//! 1. DRV8323 SPI 寄存器读写（配置栅极驱动、电流放大增益、保护阈值）
//! 2. 故障检测与清除
//! 3. 增量式编码器 4 倍频解码
//! 4. Clarke → Park → Id/Iq 双电流 PI 环 + 速度 PI 环
//! 5. 逆 Park + SVPWM 三相调制输出
//!
//! 其他驱动模式见：
//! - `bldc.rs` — 开环 6 步梯形换向
//! - `bldc_speed_ctrl.rs` — 无传感器 BEMF 闭环
//! - `bldc_hall.rs` — 霍尔 6 步梯形换向 + PI 转速控制
//! - `bldc_foc.rs` — 电压模式 FOC（霍尔 + 角度插值 + SVPWM）
//! - `bldc_foc_current.rs` — 电流模式 FOC（编码器 + 外部电流传感器）
//!
//! ---
//!
//! # 1. DRV8323 芯片概述
//!
//! ## 1.1 功能框图
//!
//! ```text
//!  ┌────────────────────────────────────────────────────────────────────┐
//!  │                          DRV8323                                   │
//!  │                                                                    │
//!  │  ┌──────────┐   ┌──────────────────────┐   ┌──────────────────┐  │
//!  │  │ SPI 接口  │   │   栅极驱动控制逻辑     │   │ 3 × 半桥驱动器   │  │
//!  │  │ SCLK     │◄─►│  ┌───────────────┐   │──►│  GHA ──► HS_A    │  │
//!  │  │ SDI      │   │  │ 寄存器组      │   │   │  GLA ──► LS_A    │  │
//!  │  │ SDO      │   │  │ (配置+状态)   │   │   │  GHB ──► HS_B    │  │
//!  │  │ nSCS     │   │  └───────────────┘   │   │  GLB ──► LS_B    │  │
//!  │  └──────────┘   └──────────┬───────────┘   │  GHC ──► HS_C    │  │
//!  │                            │               │  GLC ──► LS_C    │  │
//!  │                            │               └────────┬─────────┘  │
//!  │  ┌──────────┐              │                        │            │
//!  │  │ 保护电路  │◄─────────────┤              ┌────────┴─────────┐  │
//!  │  │ OCP/OVP  │              │              │  3 × 半桥功率级   │  │
//!  │  │ OTSD     │              │              │  (外部 MOSFET)    │  │
//!  │  │ VDS 监控  │              │              │  SH_A ──► 电机 A │  │
//!  │  └─────┬────┘              │              │  SH_B ──► 电机 B │  │
//!  │        │                   │              │  SH_C ──► 电机 C │  │
//!  │        ▼                   │              └──────────────────┘  │
//!  │  nFAULT ───► MCU          │                                    │
//!  │                            │                                    │
//!  │  ┌──────────────────────┐  │                                    │
//!  │  │ 2 × 电流检测放大器    │◄─┘                                    │
//!  │  │ CSA Gain: 5/10/20/40 │  SN_A ──► SO_A ──► MCU ADC (Ia)     │
//!  │  │ 内置偏置 VREF/2      │  SN_B ──► SO_B ──► MCU ADC (Ib)     │
//!  │  └──────────────────────┘  SN_C ──► SO_C (可选)                │
//!  │                                                                  │
//!  │  ┌──────────┐                                                    │
//!  │  │ 稳压器    │──► VCC_MCU (3.3V / 可选 buck)                     │
//!  │  └──────────┘                                                    │
//!  └────────────────────────────────────────────────────────────────────┘
//! ```
//!
//! ## 1.2 DRV8323 主要特性
//!
//! | 特性 | 参数 |
//! |------|------|
//! | 输入电压范围 | 6V ~ 60V |
//! | 峰值驱动电流 | 1A source / 2A sink (DRV8323S) |
//! | SPI 时钟频率 | 最高 10 MHz |
//! | SPI 模式 | Mode 1 (CPOL=0, CPHA=1) |
//! | 电流放大增益 | 5V/V, 10V/V, 20V/V, 40V/V (可选) |
//! | 保护功能 | VDS 过流、过温关断、欠压锁定、VGS 监控 |
//! | 死区时间 | 可配置 50/100/200/400 ns |
//! | PWM 模式 | 6×PWM / 3×PWM / 1×PWM / 独立模式 |
//! | 封装 | 40-pin QFN (6mm × 6mm) |
//!
//! ## 1.3 DRV8323 vs 独立电流传感器方案
//!
//! | 特性 | DRV8323（本示例） | 独立方案（bldc_foc_current.rs） |
//! |------|------------------|-------------------------------|
//! | 电流检测 | 内置 CSA（增益可配） | 外部 ACS712/INA240 |
//! | 栅极驱动 | 内置（配置简单） | 需外部栅极驱动 IC |
//! | 保护 | OCP/OVP/OTSD 全集成 | 需外部保护电路 |
//! | 元件数量 | 少（高集成度） | 多 |
//! | SPI 配置 | 需要（本示例重点） | 不需要 |
//! | 成本 | 稍高（~$3-5） | 取决于方案 |
//! | 灵活性 | 受限于 DRV8323 方案 | 可自由搭配 |
//!
//! ---
//!
//! # 2. DRV8323 SPI 协议
//!
//! ## 2.1 SPI 配置
//!
//! - **SPI Mode 1** (CPOL=0, CPHA=1) — DRV8323 数据手册要求
//! - **16-bit 帧**：MSB 优先
//! - 最高时钟频率 **10 MHz**
//! - **nSCS** 低电平选中，每帧传输 16 bit
//!
//! ## 2.2 SPI 帧格式
//!
//! ```text
//!   写操作 (MCU → DRV8323):
//!   ┌───┬──────────┬──────────────┐
//!   │ 0 │ ADDR[3:0]│ DATA[10:0]   │
//!   └───┴──────────┴──────────────┘
//!   bit15 bit14-11   bit10-0
//!
//!     bit15   = 0 (Write)
//!     bit14:11 = 寄存器地址 (0x0 ~ 0x7)
//!     bit10:0  = 写入数据 (11 bit)
//!
//!   读操作 (MCU → DRV8323):
//!   ┌───┬──────────┬──────────────┐
//!   │ 1 │ ADDR[3:0]│ XXXX (don't care) │
//!   └───┴──────────┴──────────────┘
//!   bit15 bit14:11   bit10:0
//!
//!     bit15   = 1 (Read)
//!     bit14:11 = 寄存器地址
//!     bit10:0  = 无关（发送任意值）
//!
//!   读回应 (DRV8323 → MCU, 在同一帧 SDO 上):
//!   ┌───┬──────────┬──────────────┐
//!   │ F │ ADDR[3:0]│ DATA[10:0]   │
//!   └───┴──────────┴──────────────┘
//!     bit15   = Fault 指示 (1=有故障)
//!     bit14:11 = 回显寄存器地址
//!     bit10:0  = 寄存器数据
//! ```
//!
//! 每次 SPI 传输恰好 16 bit（1 个 u16 word）。
//!
//! ---
//!
//! # 3. DRV8323 寄存器映射
//!
//! ## 3.1 寄存器总览
//!
//! | 地址 | 名称 | R/W | 说明 |
//! |------|------|-----|------|
//! | 0x0 | Fault Status 1 | R | VDS_OCP + GDF 故障标志 |
//! | 0x1 | VGS Status 2 | R | VGS + OTW + OTSD + 欠压等状态 |
//! | 0x2 | Driver Control | R/W | PWM 模式、OCP 行为、CLR_FLT 等 |
//! | 0x3 | Gate Drive HS | R/W | 高侧栅极驱动电流配置 |
//! | 0x4 | Gate Drive LS | R/W | 低侧栅极驱动 + 死区时间 + OCP 模式 |
//! | 0x5 | OCP Control | R/W | VDS 过流阈值、去抖时间、重试模式 |
//! | 0x6 | CSA Control | R/W | 电流检测放大器增益、校准启用 |
//!
//! ## 3.2 Fault Status 1 (0x0) — 只读
//!
//! ```text
//! bit10    FAULT      — 逻辑 OR 所有故障
//! bit9     VDS_OCP    — VDS 过流 (任一相)
//! bit8     GDF        — 栅极驱动故障
//! bit7     UVLO       — 欠压锁定
//! bit6     OTSD       — 过温关断
//! bit5     VDS_HA     — 高侧 A VDS 过流
//! bit4     VDS_LA     — 低侧 A VDS 过流
//! bit3     VDS_HB     — 高侧 B VDS 过流
//! bit2     VDS_LB     — 低侧 B VDS 过流
//! bit1     VDS_HC     — 高侧 C VDS 过流
//! bit0     VDS_LC     — 低侧 C VDS 过流
//! ```
//!
//! ## 3.3 VGS Status 2 (0x1) — 只读
//!
//! ```text
//! bit10    SA_OC      — CSA 放大器 A 过流
//! bit9     SB_OC      — CSA 放大器 B 过流
//! bit8     SC_OC      — CSA 放大器 C 过流
//! bit7     OTW        — 过温警告 (尚未关断)
//! bit6     CPUV       — 电荷泵欠压
//! bit5     VGS_HA     — 高侧 A VGS 故障
//! bit4     VGS_LA     — 低侧 A VGS 故障
//! bit3     VGS_HB     — 高侧 B VGS 故障
//! bit2     VGS_LB     — 低侧 B VGS 故障
//! bit1     VGS_HC     — 高侧 C VGS 故障
//! bit0     VGS_LC     — 低侧 C VGS 故障
//! ```
//!
//! ## 3.4 Driver Control (0x2)
//!
//! ```text
//! bit10:9  DIS_CPUV + DIS_GDF — 禁用保护 (通常不动)
//! bit8     OTW_REP    — 过温警告是否报告到 nFAULT
//! bit7:6   PWM_MODE   — 00=6×PWM, 01=3×PWM, 10=1×PWM, 11=独立
//! bit5     1PWM_COM   — 1×PWM 模式换向控制
//! bit4     1PWM_DIR   — 1×PWM 模式方向
//! bit3:2   COAST + BRAKE — 滑行/刹车模式
//! bit1     CLR_FLT    — 写 1 清除故障锁定
//! bit0     —          — 保留
//! ```
//!
//! ## 3.5 Gate Drive HS (0x3) — 高侧栅极
//!
//! ```text
//! bit10    LOCK       — 寄存器锁定 (写保护)
//! bit9:8   IDRIVEP_HS — 高侧源电流: 00=10mA 01=30mA 10=60mA 11=80mA
//! bit7:4   IDRIVEN_HS — 高侧灌电流: 0000=20mA ... 1111=2000mA
//! (DRV8323S 具体值见数据手册表 12)
//! ```
//!
//! ## 3.6 Gate Drive LS (0x4) — 低侧栅极
//!
//! ```text
//! bit10    CBC        — 逐周期过流限制启用 (Cycle-By-Cycle)
//! bit9:8   TDRIVE     — 栅极驱动峰值源时间: 00=500ns 01=1μs 10=2μs 11=4μs
//! bit7:4   IDRIVEP_LS — 低侧源电流
//! bit3:0   IDRIVEN_LS — 低侧灌电流
//! ```
//!
//! ## 3.7 OCP Control (0x5) — 过流保护
//!
//! ```text
//! bit10    TRETRY     — 重试时间: 0=4ms, 1=50μs
//! bit9:8   DEAD_TIME  — 死区时间: 00=100ns 01=50ns 10=200ns 11=400ns
//! bit7:6   OCP_MODE   — 过流保护模式:
//!                         00=锁存关断 (需清除故障才恢复)
//!                         01=自动重试
//!                         10=仅报告 (不关断)
//!                         11=禁用
//! bit5:4   OCP_DEG    — 过流去抖时间: 00=1μs 01=2μs 10=4μs 11=8μs
//! bit3:0   VDS_LVL    — VDS 过流阈值:
//!                         0000=0.06V, 0001=0.07V, ... 1111=2.0V
//!                         (具体取决于所选 DRV832x 型号)
//! ```
//!
//! ## 3.8 CSA Control (0x6) — 电流检测放大器
//!
//! ```text
//! bit10    CSA_FET    — 使用 VDS 代替外部采样电阻 (0=SH-SP, 1=VDS)
//! bit9     VREF_DIV   — CSA 参考分压: 0=VREF, 1=VREF/2 (双向采样用)
//! bit8     LS_REF     — 0=SH 引脚参考, 1=SP 引脚参考
//! bit7:6   CSA_GAIN   — 放大增益:
//!                         00=5 V/V
//!                         01=10 V/V
//!                         10=20 V/V  ← 推荐（平衡灵敏度与范围）
//!                         11=40 V/V
//! bit5     DIS_SEN    — 禁用过流检测
//! bit4:3   CSA_CAL_A/B — 校准启用 (接内部短路，校准偏移)
//! bit2     CSA_CAL_C  — C 相校准
//! bit1:0   SEN_LVL    — CSA 过流检测阈值:
//!                         00=0.25V, 01=0.50V, 10=0.75V, 11=1.0V
//! ```
//!
//! ---
//!
//! # 4. 电流采样原理 (DRV8323 CSA)
//!
//! DRV8323 内置的电流检测放大器 (CSA) 在每个低侧 MOSFET 的源极串联
//! 一个采样电阻 (Rsense)，通过差分放大得到与相电流成正比的电压信号。
//!
//! ```text
//!                   ┌──────┐
//!   SH_A ───────────┤MOSFET├──── Motor Phase A
//!                   └──┬───┘
//!                      │
//!                   ┌──┴───┐
//!                   │Rsense│ (如 10mΩ)
//!                   └──┬───┘
//!                      │
//!           SN_A ──────┤──── GND
//!           SP_A ──────┘
//!
//!   DRV8323 内部:
//!     V_sense = (SP_A - SN_A) × CSA_GAIN
//!     SO_A = VREF/2 + V_sense    ← 输出到 MCU ADC
//!
//!   电流换算:
//!     I_phase = (ADC_voltage - VREF/2) / (Rsense × CSA_GAIN)
//!     I_phase_mA = (adc_raw - 2048) × 3300 / 4096 / (R_mΩ × GAIN / 1000)
//! ```
//!
//! ### 增益选择指南
//!
//! | CSA_GAIN | Rsense=10mΩ | Rsense=5mΩ | 说明 |
//! |----------|-------------|------------|------|
//! | 5 V/V | ±33A | ±66A | 大电流应用 |
//! | 10 V/V | ±16.5A | ±33A | 通用 |
//! | **20 V/V** | ±8.25A | ±16.5A | **推荐（精度好）** |
//! | 40 V/V | ±4.1A | ±8.25A | 小电流、高精度 |
//!
//! ---
//!
//! # 5. 控制链路
//!
//! 与 `bldc_foc_current.rs` 相同的四层级联结构，区别在于电流采样来自
//! DRV8323 内置 CSA，栅极驱动由 DRV8323 完成（MCU 只需输出 PWM）。
//!
//! ```text
//! ┌──────────────────────────────────────────────────────────────────────┐
//! │               DRV8323 + RP2350 FOC 控制链路                          │
//! │                                                                      │
//! │  ┌──────────┐     ┌──────────┐     ┌───────────┐                    │
//! │  │ 目标转速  │────▶│ 速度 PI  │────▶│ Iq_ref    │                    │
//! │  │ (RPM)    │     │ 控制器   │     │ (力矩指令) │                    │
//! │  └──────────┘     └──────────┘     └─────┬─────┘                    │
//! │                                          │  Id_ref = 0              │
//! │                                          ▼                          │
//! │  ┌──────────────┐  ┌──────────┐   ┌──────────┐                     │
//! │  │DRV8323 CSA   │  │ Clarke   │   │  Park    │                     │
//! │  │Ia,Ib (ADC)   │─▶│ abc→αβ   │──▶│  αβ→dq   │──▶ Id, Iq 实际     │
//! │  │(内置放大器)   │  └──────────┘   └──────────┘                     │
//! │  └──────────────┘                       │                           │
//! │                                         ▼                           │
//! │                                   ┌──────────┐                      │
//! │                                   │ Id PI    │──▶ Ud                │
//! │                                   │ Iq PI    │──▶ Uq                │
//! │                                   └────┬─────┘                      │
//! │                                        │                            │
//! │  ┌──────────┐                          ▼                            │
//! │  │ 编码器   │──▶ θe ──────▶ ┌──────────────┐                       │
//! │  │ (GPIO)   │              │ 逆 Park + SVPWM │                      │
//! │  └──────────┘              └───────┬──┬──┬──┘                      │
//! │                                    │  │  │                          │
//! │                                  INHA INHB INHC                     │
//! │                                    │  │  │                          │
//! │                              ┌─────┴──┴──┴─────┐                   │
//! │                              │   DRV8323        │                   │
//! │                              │  栅极驱动 + 保护  │                   │
//! │                              │  + 电流检测       │                   │
//! │                              └────────┬─────────┘                   │
//! │                                       │                             │
//! │                                  ┌────┴─────┐                      │
//! │                                  │ BLDC 电机  │                      │
//! │                                  └──────────┘                      │
//! └──────────────────────────────────────────────────────────────────────┘
//! ```
//!
//! ---
//!
//! # 6. 硬件接线
//!
//! ## 6.1 引脚分配
//!
//! | GPIO | 功能 | 外设 | 说明 |
//! |------|------|------|------|
//! | GP0 | INHA (A 相 PWM) | PWM Slice 0 ChA | 20 kHz SVPWM → DRV8323 INHA |
//! | GP2 | INHB (B 相 PWM) | PWM Slice 1 ChA | 20 kHz SVPWM → DRV8323 INHB |
//! | GP4 | INHC (C 相 PWM) | PWM Slice 2 ChA | 20 kHz SVPWM → DRV8323 INHC |
//! | GP1 | INLA (A 相低侧) | GPIO SIO 输出 | 常高 → DRV8323 INLA |
//! | GP3 | INLB (B 相低侧) | GPIO SIO 输出 | 常高 → DRV8323 INLB |
//! | GP5 | INLC (C 相低侧) | GPIO SIO 输出 | 常高 → DRV8323 INLC |
//! | GP6 | nFAULT | GPIO 输入 (上拉) | DRV8323 故障输出 (低有效) |
//! | GP7 | ENABLE | GPIO 输出 | DRV8323 启用 (高有效) |
//! | GP10 | Encoder A | GPIO 输入 + 中断 | 上拉，双边沿 |
//! | GP11 | Encoder B | GPIO 输入 + 中断 | 上拉，双边沿 |
//! | GP18 | SPI0 SCK | SPI0 | → DRV8323 SCLK |
//! | GP19 | SPI0 TX (MOSI) | SPI0 | → DRV8323 SDI |
//! | GP16 | SPI0 RX (MISO) | SPI0 | ← DRV8323 SDO |
//! | GP17 | SPI0 CS | GPIO 输出 | → DRV8323 nSCS (软件控制) |
//! | GP26 | SO_A (Ia) | ADC Ch0 | DRV8323 电流输出 A |
//! | GP27 | SO_B (Ib) | ADC Ch1 | DRV8323 电流输出 B |
//!
//! ## 6.2 接线图
//!
//! ```text
//!   RP2350 (Pico 2)               DRV8323 评估模块
//!   ──────────────                ────────────────
//!   GP0  (INHA PWM) ─────────── INHA
//!   GP1  (INLA)     ─────────── INLA
//!   GP2  (INHB PWM) ─────────── INHB
//!   GP3  (INLB)     ─────────── INLB
//!   GP4  (INHC PWM) ─────────── INHC
//!   GP5  (INLC)     ─────────── INLC
//!
//!   GP18 (SPI SCK)  ─────────── SCLK
//!   GP19 (SPI MOSI) ─────────── SDI
//!   GP16 (SPI MISO) ◄────────── SDO
//!   GP17 (SPI CS)   ─────────── nSCS
//!
//!   GP6  (nFAULT)   ◄────────── nFAULT (开漏, 需上拉)
//!   GP7  (ENABLE)   ─────────── ENABLE
//!
//!   GP26 (ADC0)     ◄────────── SO_A (电流输出 A)
//!   GP27 (ADC1)     ◄────────── SO_B (电流输出 B)
//!
//!   GP10 (Enc A)    ◄────────── 编码器 A
//!   GP11 (Enc B)    ◄────────── 编码器 B
//!
//!   3.3V ────────────────────── DVDD / VIO
//!   GND  ────────────────────── GND
//!
//!   外部电源:
//!   VM (6~60V) ──────────────── PVDD (电机供电)
//!
//!   电机:
//!   DRV8323 SH_A ────────────── 电机 Phase A
//!   DRV8323 SH_B ────────────── 电机 Phase B
//!   DRV8323 SH_C ────────────── 电机 Phase C
//!
//!   电流采样电阻 (焊在 DRV8323 评估板上):
//!   低侧 MOSFET Source ──┤Rsense├── GND
//!                        (10mΩ 典型)
//! ```
//!
//! > **注意**: DRV8323 使用 **6×PWM 模式** 时，高侧和低侧输入独立控制。
//! > 本示例简单地将低侧 INLx 常高，高侧 INHx 接 PWM。
//! > DRV8323 内部会插入可配置的死区时间。
//!
//! ---
//!
//! # 7. 参数说明
//!
//! | 参数 | 默认值 | 说明 |
//! |------|--------|------|
//! | `TARGET_RPM` | 3000 | 目标机械转速 |
//! | `MOTOR_POLES` | 14 | 电机磁极数 |
//! | `ENCODER_CPR` | 4000 | 编码器每转计数（4 倍频后） |
//! | `PWM_FREQ_HZ` | 20,000 | PWM 开关频率 |
//! | `FOC_LOOP_HZ` | 10,000 | FOC 电流环频率 |
//! | `CSA_GAIN` | 20 V/V | DRV8323 电流放大增益 |
//! | `R_SENSE_MOHM` | 10 | 采样电阻 (mΩ) |
//! | `SPI_BAUDRATE` | 5 MHz | SPI 时钟频率 |
//!
//! ---
//!
//! # 8. DRV8323 调试指南
//!
//! ## 8.1 初次上电检查
//!
//! ```text
//! 1. 先不连电机，只接 SPI + 电源
//! 2. 拉高 ENABLE，等待 >1ms
//! 3. 读 Fault Status 1 (0x0) 和 VGS Status 2 (0x1)
//!    → 应该全 0（无故障）
//! 4. 读 Driver Control (0x2) 确认 SPI 通信正常
//!    → 应返回默认值 0x000
//! 5. 写入配置参数，再读回验证
//! ```
//!
//! ## 8.2 常见故障及排查
//!
//! | 故障 | nFAULT | 寄存器 | 可能原因 |
//! |------|--------|--------|---------|
//! | VDS_OCP | 低 | 0x0 bit9 | 电流过大/短路/采样电阻虚焊 |
//! | GDF | 低 | 0x0 bit8 | 栅极驱动开路/MOSFET 损坏 |
//! | OTSD | 低 | 0x0 bit6 | 过温（散热不良） |
//! | UVLO | 低 | 0x0 bit7 | 电源电压低于 UVLO 阈值 |
//! | OTW | — | 0x1 bit7 | 温度警告（尚可运行） |
//! | CPUV | — | 0x1 bit6 | 电荷泵欠压（启动中正常） |
//!
//! ## 8.3 故障清除
//!
//! ```text
//! 方式 1: SPI 写 Driver Control (0x2) bit1 (CLR_FLT) = 1
//! 方式 2: 拉低 ENABLE → 等待 >1μs → 拉高 ENABLE
//! ```
//!
//! ---
//!
//! # 9. 编译运行
//!
//! ```bash
//! cd rp2350-examples
//! cargo run --bin bldc_drv8323 --target thumbv8m.main-none-eabihf
//! ```
//!
//! ---
//!
//! # 10. 参考资料
//!
//! - [DRV8323 Datasheet](https://www.ti.com/lit/ds/symlink/drv8323.pdf) — TI SLVSE97
//! - [DRV8323 EVM User Guide](https://www.ti.com/lit/ug/slvubc3/slvubc3.pdf)
//! - [RP2350 Datasheet](https://datasheets.raspberrypi.com/rp2350/rp2350-datasheet.pdf)
//! - *Field Oriented Control of AC Motors* — Texas Instruments (SPRABQ8)

#![no_std]
#![no_main]
#![allow(dead_code)] // DRV8323 寄存器定义预留给扩展使用

use core::cell::RefCell;
use core::sync::atomic::{AtomicBool, AtomicI32, AtomicU32, Ordering};

use defmt::*;
use defmt_rtt as _;

use rp2350::adc::{Adc, AdcPin};
use rp2350::clocks::ClockBuilder;
use rp2350::gpio::{Pin, Pins};
use rp2350::interrupt::{self, Mutex};
use rp2350::main;
use rp2350::pac::{Interrupt, Peripherals};
use rp2350::pwm::Pwm;
use rp2350::spi::{FrameFormat, Mode, Spi0};
use rp2350::timer::Timer0; // ============================================================================
// 配置参数
// ============================================================================

/// PWM 开关频率 (20 kHz)
const PWM_FREQ_HZ: u32 = 20_000;

/// 目标机械转速 (RPM)
const TARGET_RPM: i32 = 3000;

/// 电机磁极数
const MOTOR_POLES: u32 = 14;
const POLE_PAIRS: u32 = MOTOR_POLES / 2;

/// 编码器每转计数 (CPR, 4 倍频后)
const ENCODER_CPR: u32 = 4000;

/// 编码器方向：true = 默认方向，false = 反向
const ENCODER_DIR: bool = true;

/// FOC 电流环频率 (Hz)
const FOC_LOOP_HZ: u32 = 10_000;
const FOC_LOOP_INTERVAL_US: u32 = 1_000_000 / FOC_LOOP_HZ;

/// 速度环分频（相对于 FOC 循环）
/// 100 → 速度环 100 Hz
const SPEED_LOOP_DIVIDER: u32 = 100;

// ---- SPI 配置 ----
/// DRV8323 SPI 最高 10 MHz，这里使用 5 MHz
const DRV_SPI_BAUDRATE: u32 = 5_000_000;

// ---- 电流 PI 参数 (Q10 定点) ----
const KP_ID: i32 = 512; // ~0.5
const KI_ID: i32 = 51; // ~0.05
const KP_IQ: i32 = 512;
const KI_IQ: i32 = 51;
const CURRENT_INTEGRAL_MAX: i32 = 500_000;
const U_MAX: i32 = 17000;

// ---- 速度 PI 参数 (Q10 定点) ----
const KP_SPEED: i32 = 307; // ~0.3
const KI_SPEED: i32 = 20; // ~0.02
const SPEED_INTEGRAL_MAX: i32 = 500_000;
const IQ_REF_MAX: i32 = 5000;

/// Q10 定点移位
const FP_SHIFT: i32 = 10;

// ---- DRV8323 电流检测参数 ----

/// 采样电阻阻值 (mΩ)
/// 推荐 10 mΩ (中等电流) 或 5 mΩ (大电流)
const R_SENSE_MOHM: i32 = 10;

/// DRV8323 CSA 放大增益
/// 可选值: 5, 10, 20, 40 (V/V)
/// 20 V/V 是推荐默认值，平衡灵敏度与可测范围
const CSA_GAIN: i32 = 20;

/// ADC 中点（0A 时的 ADC 读数）
/// DRV8323 CSA 偏置 = VREF/2 ≈ 1.65V → ADC ≈ 2048
const ADC_ZERO_CURRENT: i32 = 2048;

// ============================================================================
// DRV8323 SPI 寄存器定义
// ============================================================================

/// DRV8323 寄存器地址
mod drv_reg {
    /// Fault Status 1 — 只读故障标志
    pub const FAULT_STATUS_1: u8 = 0x0;
    /// VGS Status 2 — 只读状态/VGS故障
    pub const VGS_STATUS_2: u8 = 0x1;
    /// Driver Control — PWM 模式、故障清除等
    pub const DRIVER_CONTROL: u8 = 0x2;
    /// Gate Drive HS — 高侧栅极驱动电流
    pub const GATE_DRIVE_HS: u8 = 0x3;
    /// Gate Drive LS — 低侧栅极驱动 + CBC
    pub const GATE_DRIVE_LS: u8 = 0x4;
    /// OCP Control — VDS 过流保护配置
    pub const OCP_CONTROL: u8 = 0x5;
    /// CSA Control — 电流检测放大器配置
    pub const CSA_CONTROL: u8 = 0x6;
}

/// Fault Status 1 (0x0) 位定义
mod fault1 {
    pub const FAULT: u16 = 1 << 10;
    pub const VDS_OCP: u16 = 1 << 9;
    pub const GDF: u16 = 1 << 8;
    pub const UVLO: u16 = 1 << 7;
    pub const OTSD: u16 = 1 << 6;
    pub const VDS_HA: u16 = 1 << 5;
    pub const VDS_LA: u16 = 1 << 4;
    pub const VDS_HB: u16 = 1 << 3;
    pub const VDS_LB: u16 = 1 << 2;
    pub const VDS_HC: u16 = 1 << 1;
    pub const VDS_LC: u16 = 1 << 0;
}

/// VGS Status 2 (0x1) 位定义
mod vgs_status {
    pub const SA_OC: u16 = 1 << 10;
    pub const SB_OC: u16 = 1 << 9;
    pub const SC_OC: u16 = 1 << 8;
    pub const OTW: u16 = 1 << 7;
    pub const CPUV: u16 = 1 << 6;
    pub const VGS_HA: u16 = 1 << 5;
    pub const VGS_LA: u16 = 1 << 4;
    pub const VGS_HB: u16 = 1 << 3;
    pub const VGS_LB: u16 = 1 << 2;
    pub const VGS_HC: u16 = 1 << 1;
    pub const VGS_LC: u16 = 1 << 0;
}

/// Driver Control (0x2) 位定义
mod drv_ctrl {
    /// 禁用 CPUV 故障
    pub const DIS_CPUV: u16 = 1 << 9;
    /// 禁用 GDF 故障
    pub const DIS_GDF: u16 = 1 << 8;
    /// OTW 报告到 nFAULT
    pub const OTW_REP: u16 = 1 << 7;
    /// PWM 模式: 00=6×PWM, 01=3×PWM, 10=1×PWM, 11=独立
    pub const PWM_MODE_SHIFT: u8 = 5;
    pub const PWM_MODE_MASK: u16 = 0x3 << 5;
    /// 6×PWM 模式 (高侧低侧分别由 INHx/INLx 控制)
    pub const PWM_MODE_6X: u16 = 0x0 << 5;
    /// 3×PWM 模式 (只需 3 个 INHx，低侧自动互补)
    pub const PWM_MODE_3X: u16 = 0x1 << 5;
    /// 清除故障
    pub const CLR_FLT: u16 = 1 << 0;
}

/// OCP Control (0x5) 位定义
mod ocp_ctrl {
    /// 死区时间
    pub const DEAD_TIME_SHIFT: u8 = 8;
    /// 100ns 死区
    pub const DEAD_TIME_100NS: u16 = 0x0 << 8;
    /// 50ns 死区
    pub const DEAD_TIME_50NS: u16 = 0x1 << 8;
    /// 200ns 死区
    pub const DEAD_TIME_200NS: u16 = 0x2 << 8;
    /// 400ns 死区
    pub const DEAD_TIME_400NS: u16 = 0x3 << 8;

    /// OCP 模式
    pub const OCP_MODE_SHIFT: u8 = 6;
    /// 锁存关断 (需 CLR_FLT 清除)
    pub const OCP_LATCH: u16 = 0x0 << 6;
    /// 自动重试
    pub const OCP_RETRY: u16 = 0x1 << 6;
    /// 仅报告 (不关断)
    pub const OCP_REPORT_ONLY: u16 = 0x2 << 6;
    /// 禁用
    pub const OCP_DISABLED: u16 = 0x3 << 6;

    /// OCP 去抖时间
    pub const OCP_DEG_SHIFT: u8 = 4;
    pub const OCP_DEG_1US: u16 = 0x0 << 4;
    pub const OCP_DEG_2US: u16 = 0x1 << 4;
    pub const OCP_DEG_4US: u16 = 0x2 << 4;
    pub const OCP_DEG_8US: u16 = 0x3 << 4;

    /// VDS 过流阈值 (低 4 位)
    /// 具体电压取决于芯片型号，一般:
    /// 0x0=0.06V, 0x1=0.07V, ... 0x9=0.2V, ... 0xF=2.0V
    pub const VDS_LVL_MASK: u16 = 0x0F;
}

/// CSA Control (0x6) 位定义
mod csa_ctrl {
    /// 使用 VDS 代替外部 Rsense (通常 = 0)
    pub const CSA_FET: u16 = 1 << 10;
    /// VREF 分压: 1 = VREF/2 (双向电流检测)
    pub const VREF_DIV: u16 = 1 << 9;
    /// 参考引脚: 0 = SH 引脚
    pub const LS_REF: u16 = 1 << 8;

    /// CSA 增益
    pub const CSA_GAIN_SHIFT: u8 = 6;
    pub const CSA_GAIN_5: u16 = 0x0 << 6;
    pub const CSA_GAIN_10: u16 = 0x1 << 6;
    pub const CSA_GAIN_20: u16 = 0x2 << 6;
    pub const CSA_GAIN_40: u16 = 0x3 << 6;

    /// 禁用过流检测
    pub const DIS_SEN: u16 = 1 << 5;
    /// 校准启用 A
    pub const CSA_CAL_A: u16 = 1 << 4;
    /// 校准启用 B
    pub const CSA_CAL_B: u16 = 1 << 3;
    /// 校准启用 C
    pub const CSA_CAL_C: u16 = 1 << 2;

    /// CSA 过流检测阈值
    pub const SEN_LVL_SHIFT: u8 = 0;
    pub const SEN_LVL_025V: u16 = 0x0; // 0.25V
    pub const SEN_LVL_050V: u16 = 0x1; // 0.50V
    pub const SEN_LVL_075V: u16 = 0x2; // 0.75V
    pub const SEN_LVL_100V: u16 = 0x3; // 1.00V
}

// ============================================================================
// DRV8323 SPI 驱动封装
// ============================================================================

/// DRV8323 SPI 驱动器
///
/// 封装了 SPI 读写寄存器操作。使用 GPIO 软件控制 nSCS 片选。
///
/// DRV8323 SPI 帧格式为 16-bit:
/// - 写: [0 | ADDR(4bit) | DATA(11bit)]
/// - 读: [1 | ADDR(4bit) | x(11bit)]  →  SDO 返回 [FAULT | ADDR | DATA]
struct Drv8323<'a> {
    spi: &'a Spi0<Pin<18>, Pin<19>, Pin<16>>,
    cs: rp2350::gpio::Pin<17>,
}

impl<'a> Drv8323<'a> {
    /// 创建 DRV8323 驱动实例
    fn new(spi: &'a Spi0<Pin<18>, Pin<19>, Pin<16>>, mut cs: rp2350::gpio::Pin<17>) -> Self {
        cs.into_output();
        cs.set_high(); // nSCS 空闲为高
        Self { spi, cs }
    }

    /// 写 DRV8323 寄存器
    ///
    /// 帧格式: [0 | ADDR[3:0] | DATA[10:0]]
    /// bit15 = 0 (写), bit14:11 = 地址, bit10:0 = 数据
    fn write_reg(&mut self, addr: u8, data: u16) {
        let frame: u16 = ((addr as u16 & 0x0F) << 11) | (data & 0x7FF);
        let tx = frame.to_be_bytes(); // MSB 优先
        self.cs.set_low();
        self.spi.write(&tx);
        self.spi.flush();
        self.cs.set_high();
    }

    /// 读 DRV8323 寄存器
    ///
    /// 发送: [1 | ADDR[3:0] | 0x000]
    /// 返回 SDO: [FAULT | ADDR[3:0] | DATA[10:0]]
    fn read_reg(&mut self, addr: u8) -> u16 {
        let frame: u16 = (1 << 15) | ((addr as u16 & 0x0F) << 11);
        let tx = frame.to_be_bytes();
        let mut rx = [0u8; 2];
        self.cs.set_low();
        self.spi.transfer(&mut rx, &tx).expect("DRV8323 SPI read");
        self.cs.set_high();
        let raw = u16::from_be_bytes(rx);
        raw & 0x7FF // 只取低 11 位 (寄存器数据)
    }

    /// 检查读回数据的 FAULT 标志位 (bit15)
    fn read_reg_with_fault(&mut self, addr: u8) -> (u16, bool) {
        let frame: u16 = (1 << 15) | ((addr as u16 & 0x0F) << 11);
        let tx = frame.to_be_bytes();
        let mut rx = [0u8; 2];
        self.cs.set_low();
        self.spi
            .transfer(&mut rx, &tx)
            .expect("DRV8323 SPI read w/ fault");
        self.cs.set_high();
        let raw = u16::from_be_bytes(rx);
        let fault = (raw & 0x8000) != 0;
        let data = raw & 0x7FF;
        (data, fault)
    }

    /// 清除所有故障
    ///
    /// 向 Driver Control (0x2) 写入 CLR_FLT 位
    fn clear_faults(&mut self) {
        let current = self.read_reg(drv_reg::DRIVER_CONTROL);
        self.write_reg(drv_reg::DRIVER_CONTROL, current | drv_ctrl::CLR_FLT);
    }

    /// 读取 Fault Status 1 寄存器并打印故障信息
    fn check_faults(&mut self) -> u16 {
        let fs1 = self.read_reg(drv_reg::FAULT_STATUS_1);
        if fs1 & fault1::FAULT != 0 {
            warn!("DRV8323 FAULT detected! Status1=0x{:03x}", fs1);
            if fs1 & fault1::VDS_OCP != 0 {
                warn!("  VDS_OCP (过流)");
            }
            if fs1 & fault1::GDF != 0 {
                warn!("  GDF (栅极驱动故障)");
            }
            if fs1 & fault1::UVLO != 0 {
                warn!("  UVLO (欠压)");
            }
            if fs1 & fault1::OTSD != 0 {
                warn!("  OTSD (过温关断)");
            }
        }
        let fs2 = self.read_reg(drv_reg::VGS_STATUS_2);
        if fs2 & vgs_status::OTW != 0 {
            warn!("DRV8323 OTW (过温警告)");
        }
        if fs2 & vgs_status::CPUV != 0 {
            warn!("DRV8323 CPUV (电荷泵欠压)");
        }
        fs1
    }

    /// 配置 DRV8323 为 6×PWM 模式 + CSA 增益 20 V/V + 合理的保护参数
    ///
    /// 推荐在 ENABLE 拉高后、电机启动前调用
    fn configure(&mut self) {
        // ------------------------------------------------------------------
        // Driver Control (0x2):
        //   - 6×PWM 模式 (PWM_MODE = 00)
        //   - 不禁用保护
        //   - CLR_FLT = 1 (清除启动残留故障)
        // ------------------------------------------------------------------
        self.write_reg(
            drv_reg::DRIVER_CONTROL,
            drv_ctrl::PWM_MODE_6X | drv_ctrl::CLR_FLT,
        );

        // ------------------------------------------------------------------
        // Gate Drive HS (0x3):
        //   - LOCK = 0 (不锁定寄存器)
        //   - IDRIVEP_HS = 01 (30mA source) — bit9:8
        //   - IDRIVEN_HS = 0010 (60mA sink) — bit7:4
        //   推荐中等驱动电流，平衡开关速度与 EMI
        // ------------------------------------------------------------------
        self.write_reg(drv_reg::GATE_DRIVE_HS, (0x01 << 8) | (0x02 << 4));

        // ------------------------------------------------------------------
        // Gate Drive LS (0x4):
        //   - CBC = 1 (启用逐周期过流限制) — bit10
        //   - TDRIVE = 01 (1μs 峰值源时间) — bit9:8
        //   - IDRIVEP_LS = 01 (30mA source) — bit7:4
        //   - IDRIVEN_LS = 0010 (60mA sink) — bit3:0
        // ------------------------------------------------------------------
        self.write_reg(
            drv_reg::GATE_DRIVE_LS,
            (1 << 10) | (0x01 << 8) | (0x01 << 4) | 0x02,
        );

        // ------------------------------------------------------------------
        // OCP Control (0x5):
        //   - DEAD_TIME = 00 (100ns) — bit9:8
        //   - OCP_MODE = 01 (自动重试) — bit7:6
        //   - OCP_DEG = 01 (2μs 去抖) — bit5:4
        //   - VDS_LVL = 0x09 (~0.2V 阈值) — bit3:0
        //     → 对于 10mΩ Rsense: 0.2V / 10mΩ = 20A 触发
        // ------------------------------------------------------------------
        self.write_reg(
            drv_reg::OCP_CONTROL,
            ocp_ctrl::DEAD_TIME_100NS | ocp_ctrl::OCP_RETRY | ocp_ctrl::OCP_DEG_2US | 0x09, // VDS_LVL
        );

        // ------------------------------------------------------------------
        // CSA Control (0x6):
        //   - CSA_FET = 0 (使用外部 Rsense)
        //   - VREF_DIV = 1 (VREF/2, 双向电流检测)
        //   - LS_REF = 0
        //   - CSA_GAIN = 10 (20 V/V) — bit7:6
        //   - DIS_SEN = 0 (启用过流检测)
        //   - SEN_LVL = 01 (0.50V 阈值)
        // ------------------------------------------------------------------
        self.write_reg(
            drv_reg::CSA_CONTROL,
            csa_ctrl::VREF_DIV | csa_ctrl::CSA_GAIN_20 | csa_ctrl::SEN_LVL_050V,
        );

        info!("DRV8323 configuration complete");

        // 回读验证
        let dc = self.read_reg(drv_reg::DRIVER_CONTROL);
        let hs = self.read_reg(drv_reg::GATE_DRIVE_HS);
        let ls = self.read_reg(drv_reg::GATE_DRIVE_LS);
        let ocp = self.read_reg(drv_reg::OCP_CONTROL);
        let csa = self.read_reg(drv_reg::CSA_CONTROL);
        info!(
            "  DrvCtrl=0x{:03x} GateHS=0x{:03x} GateLS=0x{:03x} OCP=0x{:03x} CSA=0x{:03x}",
            dc, hs, ls, ocp, csa
        );
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

/// 将 αβ 坐标系电压转换为三相 PWM 占空比
///
/// 使用中性点注入 (zero-sequence injection) 方法：
/// 1. 计算三相电压 ua, ub, uc
/// 2. 求 offset = -(max + min) / 2 (中性点偏移)
/// 3. 注入偏移后映射到 PWM 占空比
///
/// 相比标准 SPWM，利用率提高 ~15.47%（线性范围 → Vdc/√3）
fn svpwm(u_alpha: i32, u_beta: i32, pwm_wrap: u16) -> (u16, u16, u16) {
    const SQRT3_Q15: i32 = 56756; // √3 × 32768

    // 逆 Clarke：αβ → abc
    let ua = u_alpha;
    let ub = (-u_alpha + ((SQRT3_Q15 * u_beta) >> 15)) / 2;
    let uc = (-u_alpha - ((SQRT3_Q15 * u_beta) >> 15)) / 2;

    // 中性点注入
    let v_max = ua.max(ub).max(uc);
    let v_min = ua.min(ub).min(uc);
    let offset = -(v_max + v_min) / 2;

    // 映射到 PWM 占空比
    let half_wrap = pwm_wrap as i32 / 2;
    let da = ((ua + offset) * half_wrap / 32767 + half_wrap).clamp(0, pwm_wrap as i32) as u16;
    let db = ((ub + offset) * half_wrap / 32767 + half_wrap).clamp(0, pwm_wrap as i32) as u16;
    let dc = ((uc + offset) * half_wrap / 32767 + half_wrap).clamp(0, pwm_wrap as i32) as u16;

    (da, db, dc)
}

// ============================================================================
// ADC → 电流换算 (DRV8323 CSA)
// ============================================================================

/// 将 ADC 原始值转换为电流 (mA, 有符号)
///
/// DRV8323 CSA 输出:
///   V_out = VREF/2 + I × R_sense × CSA_GAIN
///
/// ADC 读数:
///   adc_raw = V_out × 4096 / 3.3V
///
/// 电流换算:
///   I = (adc_raw - 2048) × 3300 / 4096 / (R_sense_mΩ × CSA_GAIN / 1000)
///     = (adc_raw - 2048) × 3300 × 1000 / 4096 / (R_sense_mΩ × CSA_GAIN)
///     = (adc_raw - 2048) × 3_300_000 / 4096 / (R_sense_mΩ × CSA_GAIN)
///
/// 对于 R_sense=10mΩ, GAIN=20:
///   I_mA = (adc_raw - 2048) × 3_300_000 / 4096 / 200
///        = (adc_raw - 2048) × 806 / 200
///        ≈ (adc_raw - 2048) × 4.03
#[inline]
fn adc_to_current_ma(adc_raw: u16) -> i32 {
    let diff = adc_raw as i32 - ADC_ZERO_CURRENT;
    // 通用公式：diff × 3_300_000 / 4096 / (R_mΩ × GAIN)
    // 简化: diff × 806 / (R_mΩ × GAIN / 1000)
    // 这里用整型近似: diff × 806_000 / (R_SENSE_MOHM × CSA_GAIN)
    (diff as i64 * 806_000i64 / (R_SENSE_MOHM as i64 * CSA_GAIN as i64)) as i32
}

// ============================================================================
// PI 控制器（通用）
// ============================================================================

/// Q10 定点 PI 控制器
///
/// 带积分限幅 + 输出限幅，防止积分饱和 (windup)。
/// 同时用于电流内环 (Id/Iq PI) 和速度外环 (Speed PI)。
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

    /// 执行一步 PI 计算
    ///
    /// output = (Kp × error + integral) >> FP_SHIFT
    /// integral += Ki × error (带限幅)
    #[inline]
    fn update(&mut self, error: i32) -> i32 {
        let p = self.kp * error;
        self.integral =
            (self.integral + self.ki * error).clamp(-self.integral_max, self.integral_max);
        let output = (p + self.integral) >> FP_SHIFT;
        output.clamp(self.output_min, self.output_max)
    }

    /// 复位积分器
    #[allow(dead_code)]
    fn reset(&mut self) {
        self.integral = 0;
    }
}

// ============================================================================
// 全局共享状态
// ============================================================================

static TIMER: Mutex<RefCell<Option<Timer0>>> = Mutex::new(RefCell::new(None));
static ADC: Mutex<RefCell<Option<Adc>>> = Mutex::new(RefCell::new(None));

/// FOC tick 标志 (由 Timer0 ISR 置位，主循环清除)
static FOC_TICK: AtomicBool = AtomicBool::new(false);

/// 编码器原始计数
static ENCODER_COUNT: AtomicI32 = AtomicI32::new(0);

/// 实测 RPM
static ACTUAL_RPM: AtomicI32 = AtomicI32::new(0);

/// Iq 参考值 (mA, 由速度环输出)
static CURRENT_IQ_REF: AtomicI32 = AtomicI32::new(0);

/// 编码器上次 AB 状态 (2-bit: A<<1 | B)
static ENCODER_LAST_AB: AtomicU32 = AtomicU32::new(0);

// ============================================================================
// 编码器 4 倍频解码表
// ============================================================================

/// 正交解码状态转移表
///
/// 索引 = (prev_AB << 2) | curr_AB
/// 值: +1 = 正转, -1 = 反转, 0 = 无变化或无效
const ENCODER_TABLE: [i32; 16] = [
    0, 1, 0, -1, // prev=00
    -1, 0, 1, 0, // prev=01
    0, -1, 0, 1, // prev=11
    1, 0, -1, 0, // prev=10
];

// ============================================================================
// 入口
// ============================================================================

#[main]
fn main() -> ! {
    info!("BLDC FOC + DRV8323 — SPI config + encoder + current loop + SVPWM");

    // ------------------------------------------------------------------
    // 外设初始化
    // ------------------------------------------------------------------
    let dp = Peripherals::take().unwrap();
    let clocks = ClockBuilder::new()
        .enable_adc()
        .build(dp.CLOCKS, dp.XOSC, dp.PLL_SYS, dp.PLL_USB);
    let sys_freq = clocks.sys_freq();
    info!("System clock: {} MHz", sys_freq / 1_000_000);

    let pins = Pins::init(dp.IO_BANK0, dp.PADS_BANK0);
    let timer = Timer0::init(dp.TIMER0);

    // ------------------------------------------------------------------
    // DRV8323 SPI 初始化 (SPI0)
    //   GP18 = SCK, GP19 = TX (MOSI→SDI), GP16 = RX (MISO←SDO)
    //   GP17 = nSCS (GPIO 软件控制)
    // ------------------------------------------------------------------
    let mut spi = Spi0::init(
        dp.SPI0,
        pins.pin::<18>(), // SCK
        pins.pin::<19>(), // TX (MOSI)
        pins.pin::<16>(), // RX (MISO)
        DRV_SPI_BAUDRATE,
        clocks.peri_freq(),
    );
    // DRV8323 要求 SPI Mode 1 (CPOL=0, CPHA=1), 16-bit 帧
    spi.set_format(16, Mode::Mode1, FrameFormat::MotorolaSpi);
    let actual_baud = spi.set_baudrate(DRV_SPI_BAUDRATE, clocks.peri_freq());
    info!(
        "SPI0 initialized: {} Hz (requested {} Hz)",
        actual_baud, DRV_SPI_BAUDRATE
    );

    let cs_pin = pins.pin::<17>();

    // ------------------------------------------------------------------
    // DRV8323 ENABLE 引脚 (GP7) — 拉高启用芯片
    // ------------------------------------------------------------------
    let mut drv_enable = pins.pin::<7>();
    drv_enable.into_output();
    drv_enable.set_low();
    timer.delay_ms(10); // 确保 ENABLE 初始为低

    // 拉高 ENABLE，等待 DRV8323 就绪 (tWAKE ≈ 1ms)
    drv_enable.set_high();
    timer.delay_ms(2);
    info!("DRV8323 enabled");

    // ------------------------------------------------------------------
    // nFAULT 输入引脚 (GP6) — 开漏输出，需上拉
    // ------------------------------------------------------------------
    let mut nfault = pins.pin::<6>();
    nfault.into_input_pullup();

    // ------------------------------------------------------------------
    // 配置 DRV8323 寄存器
    // ------------------------------------------------------------------
    let mut drv = Drv8323::new(&spi, cs_pin);

    // 初次上电检查故障状态
    let faults = drv.check_faults();
    if faults != 0 {
        warn!("Clearing startup faults...");
        drv.clear_faults();
        timer.delay_ms(1);
    }

    // 配置 DRV8323: 6×PWM, CSA 增益 20, 死区 100ns, OCP 自动重试
    drv.configure();

    // 验证配置写入
    let csa_readback = drv.read_reg(drv_reg::CSA_CONTROL);
    info!("CSA Control readback: 0x{:03x}", csa_readback);

    // 检查 nFAULT: 高 = 正常, 低 = 有故障
    if !nfault.get() {
        error!("nFAULT is LOW after configuration — check hardware!");
        drv.check_faults();
    } else {
        info!("nFAULT=HIGH — DRV8323 ready");
    }

    // 释放 SPI（主循环中不再使用，仅故障时重新访问）
    // drv 离开作用域后 spi 归还

    // ------------------------------------------------------------------
    // PWM — 20 kHz，3 个 slice
    // 注意: DRV8323 在 6×PWM 模式下
    //   INHx 接 PWM (控制占空比)
    //   INLx 接 GPIO 常高 (低侧持续导通)
    //   DRV8323 内部插入死区
    // ------------------------------------------------------------------
    let pwm_wrap: u16 = (sys_freq / PWM_FREQ_HZ) as u16;
    info!("PWM wrap = {} ({} kHz)", pwm_wrap, PWM_FREQ_HZ / 1000);

    let pwm = Pwm::init(dp.PWM);

    // Phase A: GP0 (INHA)
    let mut slice_a = pwm.slice::<0>().output_a(pins.pin::<0>());
    slice_a.set_wrap(pwm_wrap);
    slice_a.set_duty_a(0);
    slice_a.enable();

    // Phase B: GP2 (INHB)
    let mut slice_b = pwm.slice::<1>().output_a(pins.pin::<2>());
    slice_b.set_wrap(pwm_wrap);
    slice_b.set_duty_a(0);
    slice_b.enable();

    // Phase C: GP4 (INHC)
    let mut slice_c = pwm.slice::<2>().output_a(pins.pin::<4>());
    slice_c.set_wrap(pwm_wrap);
    slice_c.set_duty_a(0);
    slice_c.enable();

    // ------------------------------------------------------------------
    // 低侧 INLx — GPIO 常高 (DRV8323 6×PWM 模式下低侧需保持导通)
    // ------------------------------------------------------------------
    let mut inla = pins.pin::<1>();
    inla.into_output();
    inla.set_high();

    let mut inlb = pins.pin::<3>();
    inlb.into_output();
    inlb.set_high();

    let mut inlc = pins.pin::<5>();
    inlc.into_output();
    inlc.set_high();

    // ------------------------------------------------------------------
    // ADC — 采样 DRV8323 CSA 输出 (SO_A → GP26, SO_B → GP27)
    // ------------------------------------------------------------------
    let adc = Adc::init(dp.ADC);
    #[cfg(not(feature = "rp2350b"))]
    let (adc_ia, adc_ib) = (
        pins.pin::<26>().into_adc_input(),
        pins.pin::<27>().into_adc_input(),
    );
    #[cfg(feature = "rp2350b")]
    let (adc_ia, adc_ib) = (
        pins.pin::<40>().into_adc_input(),
        pins.pin::<41>().into_adc_input(),
    );
    info!(
        "ADC initialized: SO_A=Ch{}, SO_B=Ch{}",
        adc_ia.channel(),
        adc_ib.channel()
    );

    // ------------------------------------------------------------------
    // 编码器 — GPIO 中断 4 倍频正交解码 (GP10=A, GP11=B)
    // ------------------------------------------------------------------
    let mut enc_a = pins.pin::<10>();
    enc_a.into_input_pullup();
    enc_a.enable_interrupt(rp2350::gpio::EdgeLevel::EdgeHigh);
    enc_a.enable_interrupt(rp2350::gpio::EdgeLevel::EdgeLow);

    let mut enc_b = pins.pin::<11>();
    enc_b.into_input_pullup();
    enc_b.enable_interrupt(rp2350::gpio::EdgeLevel::EdgeHigh);
    enc_b.enable_interrupt(rp2350::gpio::EdgeLevel::EdgeLow);

    // 读取初始编码器状态
    let init_a = enc_a.get();
    let init_b = enc_b.get();
    let init_ab = ((init_a as u32) << 1) | (init_b as u32);
    ENCODER_LAST_AB.store(init_ab, Ordering::Relaxed);
    info!("Encoder init: A={}, B={}", init_a, init_b);

    // ------------------------------------------------------------------
    // Timer0 — FOC tick @10kHz
    // ------------------------------------------------------------------
    {
        let now = timer.time_us_32();
        timer.set_alarm(0, now.wrapping_add(FOC_LOOP_INTERVAL_US));
        timer.enable_alarm_interrupt(0);

        interrupt::free(|cs| {
            TIMER.borrow(cs).replace(Some(timer));
            ADC.borrow(cs).replace(Some(adc));
        });

        unsafe {
            interrupt::enable(Interrupt::TIMER0_IRQ_0);
            interrupt::enable(Interrupt::IO_IRQ_BANK0);
        }
    }

    info!(
        "FOC running: {}Hz current loop, target {} RPM",
        FOC_LOOP_HZ, TARGET_RPM
    );
    info!(
        "DRV8323 CSA: GAIN={}V/V, Rsense={}mΩ",
        CSA_GAIN, R_SENSE_MOHM
    );

    // ------------------------------------------------------------------
    // PI 控制器实例
    // ------------------------------------------------------------------
    let mut pi_id = PiController::new(KP_ID, KI_ID, CURRENT_INTEGRAL_MAX, -U_MAX, U_MAX);
    let mut pi_iq = PiController::new(KP_IQ, KI_IQ, CURRENT_INTEGRAL_MAX, -U_MAX, U_MAX);
    let mut pi_speed = PiController::new(
        KP_SPEED,
        KI_SPEED,
        SPEED_INTEGRAL_MAX,
        -IQ_REF_MAX,
        IQ_REF_MAX,
    );

    let mut foc_counter: u32 = 0;
    let mut log_counter: u32 = 0;
    let mut last_encoder_count: i32 = 0;
    let mut fault_check_counter: u32 = 0;

    // 获取当前时间的辅助闭包
    let timer_now_us = || -> u32 {
        interrupt::free(|cs| {
            TIMER
                .borrow(cs)
                .borrow()
                .as_ref()
                .map_or(0, |t| t.time_us_32())
        })
    };

    let mut last_speed_time: u32 = timer_now_us();

    // ------------------------------------------------------------------
    // 主循环 — FOC 电流环 @10kHz + 速度环 @100Hz
    // ------------------------------------------------------------------
    loop {
        if !FOC_TICK.swap(false, Ordering::Relaxed) {
            continue;
        }

        foc_counter += 1;

        // ==============================================================
        // 1. 采样两相电流 (DRV8323 CSA → ADC)
        // ==============================================================
        let (ia_raw, ib_raw) = interrupt::free(|cs| {
            if let Some(ref mut adc) = *ADC.borrow(cs).borrow_mut() {
                let ia = adc.read(&adc_ia);
                let ib = adc.read(&adc_ib);
                (ia, ib)
            } else {
                (ADC_ZERO_CURRENT as u16, ADC_ZERO_CURRENT as u16)
            }
        });

        let ia_ma = adc_to_current_ma(ia_raw);
        let ib_ma = adc_to_current_ma(ib_raw);

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

        // ==============================================================
        // 5. 电流 PI 控制器
        // ==============================================================
        let id_error = 0 - id; // Id_ref = 0 (不弱磁)
        let ud = pi_id.update(id_error);

        let iq_ref = CURRENT_IQ_REF.load(Ordering::Relaxed);
        let iq_error = iq_ref - iq;
        let uq = pi_iq.update(iq_error);

        // ==============================================================
        // 6. 逆 Park + SVPWM → 三相 PWM
        // ==============================================================
        let u_alpha = ((ud as i64 * cos_theta as i64 - uq as i64 * sin_theta as i64) >> 15) as i32;
        let u_beta = ((ud as i64 * sin_theta as i64 + uq as i64 * cos_theta as i64) >> 15) as i32;

        let (da, db, dc) = svpwm(u_alpha, u_beta, pwm_wrap);

        slice_a.set_duty_a(da);
        slice_b.set_duty_a(db);
        slice_c.set_duty_a(dc);

        // ==============================================================
        // 7. 速度 PI (每 SPEED_LOOP_DIVIDER 次执行一次 → 100 Hz)
        // ==============================================================
        if foc_counter.is_multiple_of(SPEED_LOOP_DIVIDER) {
            let now = timer_now_us();
            let dt_us = now.wrapping_sub(last_speed_time);
            last_speed_time = now;

            // 编码器差分 → RPM
            let delta_count = enc_count - last_encoder_count;
            last_encoder_count = enc_count;

            let actual_rpm = if dt_us > 0 {
                (delta_count as i64 * 60_000_000i64 / (ENCODER_CPR as i64 * dt_us as i64)) as i32
            } else {
                0
            };
            ACTUAL_RPM.store(actual_rpm, Ordering::Relaxed);

            // 速度 PI → Iq_ref
            let speed_error = TARGET_RPM - actual_rpm;
            let iq_ref_new = pi_speed.update(speed_error);
            CURRENT_IQ_REF.store(iq_ref_new, Ordering::Relaxed);

            // 周期性故障检查 (~1 Hz)
            fault_check_counter += 1;
            if fault_check_counter.is_multiple_of(100) {
                // 通过 nFAULT 引脚快速检查
                if !nfault.get() {
                    warn!("nFAULT LOW — DRV8323 fault detected!");
                    // 可在此通过 SPI 读取详细故障信息
                    // 此处需要重新获取 SPI 访问权
                }
            }

            // 日志 (~1 Hz)
            log_counter += 1;
            if log_counter.is_multiple_of(100) {
                info!(
                    "RPM:{} target:{} Iq_ref:{} Id:{} Iq:{} Ud:{} Uq:{} enc:{}",
                    actual_rpm, TARGET_RPM, iq_ref_new, id, iq, ud, uq, enc_count
                );
                info!(
                    "  Ia:{}mA Ib:{}mA (raw: {}, {})",
                    ia_ma, ib_ma, ia_raw, ib_raw
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
    // 清除所有编码器引脚中断
    let pin_a = unsafe { rp2350::gpio::Pin::<10>::steal() };
    let pin_b = unsafe { rp2350::gpio::Pin::<11>::steal() };
    pin_a.clear_all_interrupts();
    pin_b.clear_all_interrupts();

    // 读取当前 AB 状态
    let a = pin_a.get() as u32;
    let b = pin_b.get() as u32;
    let curr_ab = (a << 1) | b;

    // 查表获取方向
    let prev_ab = ENCODER_LAST_AB.swap(curr_ab, Ordering::Relaxed);
    let idx = ((prev_ab << 2) | curr_ab) as usize;
    let dir = ENCODER_TABLE[idx & 0xF];

    // 更新位置计数
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
            let now = timer.time_us_32();
            timer.set_alarm(0, now.wrapping_add(FOC_LOOP_INTERVAL_US));
        }
    });
}
