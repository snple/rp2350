# BLDC 无刷电机控制

基于 RP2350 (Raspberry Pi Pico 2) 的三相无刷直流电机驱动示例集合。

**详细文档（原理、硬件电路、参数调优、FAQ 等）已写入源码注释中，请直接阅读源文件。**

| 示例 | 文件 | 说明 |
|------|------|------|
| `bldc` | [bldc.rs](bldc.rs) | 开环 6 步梯形换向，固定占空比 |
| `bldc_speed_ctrl` | [bldc_speed_ctrl.rs](bldc_speed_ctrl.rs) | 无传感器 BEMF 过零检测 + PI 转速控制 |
| `bldc_hall` | [bldc_hall.rs](bldc_hall.rs) | 霍尔传感器 6 步换向 + PI 转速控制 |
| `bldc_foc` | [bldc_foc.rs](bldc_foc.rs) | FOC 磁场定向控制 — 霍尔 + 角度插值 + SVPWM |
| `bldc_foc_current` | [bldc_foc_current.rs](bldc_foc_current.rs) | 电流模式 FOC — 编码器 + 双电流环 + SVPWM |
| `bldc_servo` | [bldc_servo.rs](bldc_servo.rs) | 无刷伺服 — 编码器 + 位置/速度/电压三环 FOC |
| `bldc_servo_current` | [bldc_servo_current.rs](bldc_servo_current.rs) | 电流模式无刷伺服 — 编码器 + 位置/速度/电流四环 FOC |
| `bldc_servo_i2c` | [bldc_servo_i2c.rs](bldc_servo_i2c.rs) | I2C 从机电流模式伺服 — 寄存器接口 + 四环级联 + 位置/角度/步进 |
| `bldc_servo_spi` | [bldc_servo_spi.rs](bldc_servo_spi.rs) | SPI 从机电流模式伺服 — 寄存器接口 + 四环级联 + 位置/角度/步进 |

## 编译

```bash
cd rp2350-examples

# 开环 6 步
cargo run --bin bldc --target thumbv8m.main-none-eabihf

# 无传感器闭环
cargo run --bin bldc_speed_ctrl --target thumbv8m.main-none-eabihf

# 霍尔 6 步
cargo run --bin bldc_hall --target thumbv8m.main-none-eabihf

# FOC 电压模式（霍尔）
cargo run --bin bldc_foc --target thumbv8m.main-none-eabihf

# FOC 电流模式（编码器）
cargo run --bin bldc_foc_current --target thumbv8m.main-none-eabihf

# 无刷伺服（编码器 + 位置控制）
cargo run --bin bldc_servo --target thumbv8m.main-none-eabihf

# 电流模式伺服（编码器 + 电流环 + 位置控制）
cargo run --bin bldc_servo_current --target thumbv8m.main-none-eabihf

# I2C 从机电流模式伺服（四环级联 + 寄存器接口 + 位置/角度/步进）
cargo run --bin bldc_servo_i2c --target thumbv8m.main-none-eabihf

# SPI 从机电流模式伺服（四环级联 + 寄存器接口 + 位置/角度/步进）
cargo run --bin bldc_servo_spi --target thumbv8m.main-none-eabihf
```
