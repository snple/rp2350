# rp2350-drivers

Driver collection for RP2350 microcontrollers, built on the
[`rp2350`](https://crates.io/crates/rp2350) HAL.

## Modules

| Module | Contents |
|--------|----------|
| `tmc2209` | TMC2209 stepper driver: register-level UART access and high-level configuration, no dependency on a concrete timer or UART type |
| `stepper` | Generic STEP/DIR axis control (`AxisConfig`, `AxisState`), plus a TMC2209 variant with DIAG stall detection, INDEX alignment and homing |
| `timer` | `MonotonicTimer` / `AlarmTimer` traits for drivers, implemented for RP2350's `Timer0` / `Timer1` |
| `bus` | `UartBus` abstraction used by the drivers, implemented for RP2350's UART types |

The bus and timer traits live here rather than in the HAL so that drivers can be
generic over them without the HAL committing to a particular abstraction; implement
them for your own types if you need to.

## Features

- `defmt` (default): `defmt` logging support
- `rp2350a` (default) / `rp2350b`: chip variant, forwarded to `rp2350`

## Usage

```toml
[dependencies]
rp2350-drivers = "0.1"
```

## Target

`no_std`, Cortex-M33 (`thumbv8m.main-none-eabihf`).

## License

Licensed under either of MIT or Apache-2.0, at your option.
