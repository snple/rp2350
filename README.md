# rp2350

> RP2350（Raspberry Pi Pico 2 / Pico 2 W）的 Rust `no_std` 嵌入式支持：HAL、PAC、运行时、
> boot2、过程宏、USB 设备栈与驱动。

[![CI](https://github.com/snple/rp2350/actions/workflows/ci.yml/badge.svg)](https://github.com/snple/rp2350/actions/workflows/ci.yml)
[![crates.io](https://img.shields.io/crates/v/rp2350.svg)](https://crates.io/crates/rp2350)
[![docs.rs](https://docs.rs/rp2350/badge.svg)](https://docs.rs/rp2350)
[![license](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](#许可证)

## 特性

- **外设覆盖**：GPIO、UART、SPI、I2C、PWM、ADC、DMA、Timer、Watchdog、TRNG、SHA-256、
  DCP、PIO、Multicore、QSPI、OTP、Clocks/PLL
- **PAC 由 SVD 生成**：寄存器定义与 RP2350 datasheet 同步，`cargo xtask gen-pac` 一条命令重新生成
- **零成本所有权模型**：每个外设由 HAL 结构体独占，`steal()` / `regs()` 留出逃生口
- **可选 boot2**：256 字节 XIP 配置 blob，让 flash 从单比特 `03h` 读切到 quad 连续读
- **USB 设备栈**：CDC ACM、HID（键盘/鼠标/电源设备）、GS_USB、i2c-tiny-usb、UAC 1.0
- **驱动**：TMC2209（UART）、通用 STEP/DIR 步进轴控制、DRV8323 / MCT8316Z 等
- 支持 `rp2350a`（30 GPIO）与 `rp2350b`（48 GPIO）两种芯片型号
- 78 个可运行的示例二进制，覆盖几乎所有片上外设

## Crate 结构

本仓库是一个 workspace，按「只做一件事」分层：

| Crate | 说明 |
|-------|------|
| [`rp2350`](rp2350/) | **HAL**：唯一手写的用户接口层，每个 PAC 外设对应一个 HAL 模块 |
| [`rp2350-pac`](rp2350-pac/) | **PAC**：由 `RP2350.svd` 生成，提供寄存器块、字段常量与访问器。勿手改 |
| [`rp2350-rt`](rp2350-rt/) | **运行时**：`Reset`、向量表、`link.x` / `link_ram.x`、核心寄存器与指令封装 |
| [`rp2350-boot2`](rp2350-boot2/) | **可选 boot2**：位置无关的 XIP 配置 blob（≤ 256 字节），拷到 RAM 执行 |
| [`rp2350-macro`](rp2350-macro/) | **过程宏**：`#[main]`、`#[exception]`、`#[bind]`、`#[ramfunc]`、`pio_asm!` |
| [`rp2350-usb`](rp2350-usb/) | **USB 设备栈**：设备状态机、控制传输、描述符、类实现（通用核心在仓库外的 [`usb-stack`](../usb-stack/)） |
| [`rp2350-drivers`](rp2350-drivers/) | **设备驱动**：TMC2209、步进轴、`MonotonicTimer` / `UartBus` 抽象 |
| [`rp2350-examples`](rp2350-examples/) | 各外设的独立二进制示例 |
| [`xtask`](xtask/) | 构建辅助（`cargo xtask gen-pac`） |

依赖方向是单向的：`macro` / `rt` / `boot2` → `pac` → `rp2350` → `drivers` / `usb` → `examples`。

## 快速开始

```toml
[dependencies]
rp2350 = "0.1"
defmt = "1.2"
defmt-rtt = "1.3"
```

```rust
#![no_std]
#![no_main]

use defmt::*;
use defmt_rtt as _;

use rp2350::Delay;
use rp2350::gpio::Pins;
use rp2350::main;
use rp2350::pac::Peripherals;
use rp2350::rt::peripheral::Peripherals as CorePeripherals;

const CLOCK_FREQ_HZ: u32 = 150_000_000;

#[main]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let pins = Pins::init(dp.IO_BANK0, dp.PADS_BANK0);

    let mut led = pins.pin::<25>();
    led.into_output();

    let cp = CorePeripherals::take().unwrap();
    let delay = Delay::init(cp.SYST, CLOCK_FREQ_HZ);

    loop {
        led.toggle();
        info!("LED toggled");
        delay.delay_ms(500);
    }
}
```

`#[main]` 会自动生成 boot ROM 需要的 `IMAGE_DEF` boot block，并清空 spinlock、使能 DCP 与
GPIO 协处理器，因此用户代码不需要手写 `.start_block`。

自己的工程还需要三份脚手架文件，可直接从 [`rp2350-examples`](rp2350-examples/) 拷贝：

- `memory.x`：flash / RAM 布局
- `build.rs`：把 `memory.x` 放进 `OUT_DIR`、`-Tdefmt.x`
- `.cargo/config.toml`：默认 target 与 `-Tlink.x`、`--nmagic`、`target-cpu=cortex-m33`

## 构建与烧写示例

需要 `thumbv8m.main-none-eabihf` target（`rust-toolchain.toml` 已声明）与
[`probe-rs`](https://probe.rs/)：

```bash
rustup target add thumbv8m.main-none-eabihf
cargo install probe-rs-tools

cd rp2350-examples
cargo run --bin blink
```

`rp2350-examples/.cargo/config.toml` 已把 runner 配成
`probe-rs run --chip RP235x --protocol swd`；换用 `rp2350tool` 时改那一行即可。

打开 boot2（提升 XIP 速度，默认关闭）：

```bash
cargo run --bin blink --features boot2-w25q80
# 或不支持 quad 读的 flash：
cargo run --bin blink --features boot2-generic-03h
```

### 芯片型号

`rp2350a`（GPIO 0-29）与 `rp2350b`（GPIO 0-47）互斥，默认 `rp2350a`。切到 RP2350B 时要把
每个特征的默认值重新显式列出，因为 `--no-default-features` 会一并关掉 `defmt` 等：

```bash
cargo build -p rp2350-examples --no-default-features \
  --features rp2350b,defmt,critical-section-impl,panic-handler
```

## 重新生成 PAC

`RP2350.svd` 更新后：

```bash
cargo xtask gen-pac            # 重新生成 rp2350-pac/src/generated
cargo xtask gen-pac --check    # 只校验已提交的生成代码是否与 SVD 一致（CI 用）
```

## 文档

| 文档 | 内容 |
|------|------|
| [`docs/boot2.md`](docs/boot2.md) | boot2 做了什么、为什么可选、256 字节限制、如何验证 |
| [`docs/pwm.md`](docs/pwm.md) | PWM 硬件模型，输出/输入模式的频率、占空比、测速换算 |
| [`docs/usb_servo_protocol.md`](docs/usb_servo_protocol.md) | 双舵机控制器上位机协议 |
| [`docs/style-guide.md`](docs/style-guide.md) | 项目设计约定：模块骨架、所有权模型、错误处理 |
| [`rp2350/README.md`](rp2350/README.md) | HAL 的特征列表与完整示例 |
| [`rp2350-usb/README.md`](rp2350-usb/README.md) | USB 栈的模块划分与快速上手 |

各 crate 的 API 文档见 [docs.rs](https://docs.rs/rp2350)。

## 开发

```bash
cargo fmt --all -- --check
cargo build --workspace --target thumbv8m.main-none-eabihf --exclude xtask
cargo clippy --workspace --target thumbv8m.main-none-eabihf --exclude xtask -- -D warnings
cargo clippy -p xtask -- -D warnings
```

CI 还会额外校验 `cargo xtask gen-pac --check`、drivers 的 `--no-default-features` 构建，
以及 RP2350B 变体的完整构建，见 [`.github/workflows/ci.yml`](.github/workflows/ci.yml)。

## 许可证

本项目采用 MIT 或 Apache-2.0 双许可，二选一：

- [LICENSE-MIT](LICENSE-MIT)
- [LICENSE-APACHE](LICENSE-APACHE)
