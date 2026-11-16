# rp2350-pac

Peripheral Access Crate (PAC) for the RP2350 microcontroller (Raspberry Pi Pico 2).

Every peripheral in the RP2350 datasheet is exposed as a typed register block with
safe accessors and matching bit-field types. The code is generated from the
RP2350 SVD file, so it tracks the register definitions of the chip rather than hand
written copies.

## What it provides

- Register blocks for all RP2350 peripherals (`GPIO`/`PADS_BANK0`/`IO_BANK0`, `UART0`,
  `SPI0`, `I2C0`, `PWM`, `ADC`, `DMA`, `PIO0`/`PIO1`, `TIMER0`, `USBCTRL_REGS`, …)
- `Peripherals::take()` for exclusive, singleton ownership of each peripheral
- `device.x`, emitted by the build script from the interrupt table, so that
  `rp2350-rt`'s linker script can resolve the interrupt handler symbols

## Usage

```toml
[dependencies]
rp2350-pac = "0.1"
```

```rust
use rp2350_pac::Peripherals;

let dp = Peripherals::take().unwrap();
// Route GPIO25 to the SIO function (funcsel 5 is SIO on RP2350).
dp.IO_BANK0.gpio25_ctrl.write_funcsel(5);
```

Most applications use the [`rp2350`](https://crates.io/crates/rp2350) HAL instead,
which re-exports this crate as `rp2350::pac`.

## Target

`no_std`, Cortex-M33 (`thumbv8m.main-none-eabihf`).

## License

Licensed under either of MIT or Apache-2.0, at your option.
