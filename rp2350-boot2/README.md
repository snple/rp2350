# rp2350-boot2

RP2350 second stage boot loader ("boot2") — an optional XIP setup function.

On RP2350 the boot ROM already configures XIP well enough to boot, so boot2 is not
required. What the ROM does not do is make XIP fast. This crate provides a
position-independent setup function of at most 256 bytes that the image calls early
in startup to replace the ROM's conservative configuration with an aggressive one:
a high interface clock, a tuned RX sampling delay and — on quad-capable flash —
continuous quad-I/O reads instead of single-bit `03h` reads.

The function rewrites the QMI timing registers that govern instruction fetch, so it
cannot run while being fetched from XIP: `init()` copies it onto the stack and calls
it from there, which is what the Pico SDK's `crt0` does as well.

## Features

Exactly one variant must be selected:

- `generic-03h` (default) — anything that answers a plain `03h` serial read. Roughly
  three times slower, but the safest choice and it needs no status-register
  programming.
- `w25q80` — W25Q80-family quad-SPI flash, plus the parts the Pico SDK's
  `boot2_w25q080.S` lists as compatible (W25Q16JV, W25Q32JV, AT25SF081, S25FL132K0).

Extras, used by the variant recipes in `rp2350-rt`:

- `program-status-reg` (`w25q80` only) — program the quad-enable bit before switching
  to quad reads
- `binary-info` — emit a `BINARY_INFO_ID_RP_BOOT2_NAME` entry so `picotool` can report
  the variant

## Requirements

The `.boot2` output section and the `__boot2_start__` / `__boot2_end__` symbols come
from `rp2350-rt`'s linker scripts. The section must fit in 256 bytes; the linker fails
the build if it does not.

## Usage

Enabling the feature on the HAL is enough — `rp2350-rt` then calls `init` from its
reset handler, before `.data` is initialised:

```toml
[dependencies]
rp2350 = { version = "0.1", features = ["boot2-w25q80"] }
```

## Target

`no_std`, Cortex-M33 (`thumbv8m.main-none-eabihf`).

## License

Licensed under either of MIT or Apache-2.0, at your option.
