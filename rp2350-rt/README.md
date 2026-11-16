# rp2350-rt

Runtime support for RP2350 Cortex-M33 microcontrollers.

`rp2350-rt` is the small layer between `cargo`/the linker and an application: it
provides the reset handler and vector table, the core register and instruction
wrappers, and the linker scripts.

## What it provides

- Low-level assembly instructions (`WFI`, `WFE`, `SEV`, `NOP`, …)
- Core register access (`PRIMASK`, `CONTROL`, `MSP`, `PSP`, `MSPLIM`, …)
- Core peripherals (`NVIC`, `SCB`, `SysTick`, `MPU`, `FPU`, `SAU`)
- Interrupt enable/disable helpers and a critical-section implementation
- Runtime startup code (`Reset`, vector table) and `link.x` / `link_ram.x`

## Features

- `rt` (default): startup code, vector table and linker script
- `run-from-ram`: copy `.text` and `.rodata` into RAM at startup
- `boot2`: call a linked `rp2350-boot2` blob from `Reset` to set the QMI up for fast
  XIP; `boot2-w25q80` / `boot2-generic-03h` select the flash part and imply it

## Startup sequence

`Reset` runs, in order: `run-from-ram` copy → `.ramfunc` copy → `boot2` blob call →
`__pre_init` → `.data` copy → `.bss` zero → `MSPLIM`/FPU setup → `main`.

## Linker scripts

The crate ships `link.x` and `link_ram.x`. They `INCLUDE device.x`, which is **not**
provided here but by `rp2350-pac`'s build script, so any binary using these scripts
must also depend on `rp2350-pac`. The `rp2350` HAL does that automatically.

Both scripts also define the 256-byte `.boot2` output section that `rp2350-boot2`
fills.

## Usage

```toml
[dependencies]
rp2350-rt = "0.1"
```

Most applications use the [`rp2350`](https://crates.io/crates/rp2350) HAL, which
re-exports this crate as `rp2350::rt`.

## Target

`no_std`, Cortex-M33 (`thumbv8m.main-none-eabihf`).

## License

Licensed under either of MIT or Apache-2.0, at your option.
