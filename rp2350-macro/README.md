# rp2350-macro

Procedural macros for the [`rp2350`](https://crates.io/crates/rp2350) HAL.

## Macros

- `#[main]` — program entry point. Emits the PICOBIN `IMAGE_DEF` boot block, releases
  the hardware spinlocks and enables the DCP and GPIO co-processors before calling the
  user function.
- `#[exception]` — exception handler for Cortex-M exceptions
- `#[bind(IRQ)]` — bind a function to an interrupt
- `#[ramfunc]` — place a function in `.ramfunc` so it executes from RAM
- `pio_asm!` — PIO assembly compiler

## Usage

The macros are re-exported by the HAL, so depend on that crate and import from it:

```rust
use rp2350::entry;

#[entry]
fn main() -> ! {
    loop {}
}
```

## License

Licensed under either of MIT or Apache-2.0, at your option.
