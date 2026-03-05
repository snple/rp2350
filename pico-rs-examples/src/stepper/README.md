# 步进电机控制

基于 RP2350 (Raspberry Pi Pico 2) 的步进电机控制示例。

**详细文档（原理、硬件电路、参数调优等）已写入源码注释中，请直接阅读源文件。**

| 示例 | 文件 | 说明 |
|------|------|------|
| `stepper` | [stepper.rs](stepper.rs) | 开环步进 — STEP/DIR 梯形加减速 + 四相全步进/半步进 |
| `stepper_closed` | [stepper_closed.rs](stepper_closed.rs) | 闭环步进 — 编码器反馈 + 位置 PID + 速度 PI + 堵转保护 |

## 开环 vs 闭环

| 特性 | 开环 (`stepper`) | 闭环 (`stepper_closed`) |
|------|-----------------|------------------------|
| 反馈 | 无 | 编码器 4 倍频 |
| 丢步检测 | ✗ | ✓ 实时检测补偿 |
| 堵转保护 | ✗ | ✓ 自动停机 |
| 适合场景 | 轻载/低速 | 高精度/重载 |
| 硬件 | 驱动器 + 电机 | 驱动器 + 电机 + 编码器 |

## 硬件接线

### STEP/DIR 模式 (A4988 / DRV8825 / TMC2209)

| GPIO | 功能 |
|------|------|
| GP2 | STEP 脉冲 |
| GP3 | DIR 方向 |
| GP4 | ENABLE (低有效) |
| GP10 | 编码器 A (闭环) |
| GP11 | 编码器 B (闭环) |

### 四相直接驱动 (ULN2003 + 28BYJ-48，仅开环)

| GPIO | 功能 |
|------|------|
| GP2 | IN1 (Phase A) |
| GP3 | IN2 (Phase B) |
| GP4 | IN3 (Phase C) |
| GP5 | IN4 (Phase D) |

## 编译

```bash
cd pico-rs-examples

# 开环步进 (STEP/DIR + 四相)
cargo run --bin stepper --target thumbv8m.main-none-eabihf

# 闭环步进 (编码器 + PID)
cargo run --bin stepper_closed --target thumbv8m.main-none-eabihf
```
