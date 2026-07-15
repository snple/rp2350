# pico-rs

Hardware Abstraction Layer for RP2350 microcontrollers.

`pico-rs` provides a `no_std` HAL for RP2350-based boards, along with the
runtime and PAC re-exports needed to bring up a bare-metal application on the
Raspberry Pi Pico 2 family.

## What it includes

- GPIO, UART, SPI, I2C, PWM, ADC, DMA, timer, watchdog, TRNG, and multicore APIs
- Re-exports of `pico-rs-pac` as `pico_rs::pac` and `pico-rs-rt` as `pico_rs::rt`
- Attribute macros such as `#[entry]`, `#[exception]`, and `#[bind]`
- Optional `defmt` integration and a built-in panic handler for bare-metal targets

## Features

- `defmt`: enable `defmt` logging support
- `rp2350a`: target RP2350A with GPIO 0-29 (default)
- `rp2350b`: target RP2350B with GPIO 0-47
- `critical-section-impl`: register this crate as the `critical-section` provider
- `panic-handler`: enable the built-in panic handler on `target_os = "none"`

Default features: `defmt`, `rp2350a`, `critical-section-impl`, `panic-handler`.

## Quick start

Add the HAL to your embedded crate. If you want the same `defmt` logging setup as
the workspace blink example, add `defmt-rtt` as well:

```toml
[dependencies]
pico-rs = { path = "../pico-rs" }
defmt-rtt = "1.2"
```

Complete blink example with comments:

```rust
#![no_std]
#![no_main]

use defmt::*;
use defmt_rtt as _;

use pico_rs::Delay;
use pico_rs::block::ImageDef;
use pico_rs::entry;
use pico_rs::gpio::Pins;
use pico_rs::pac::Peripherals;
use pico_rs::rt::peripheral::Peripherals as CorePeripherals;

/// RP2350 Boot Block - tells the Boot ROM this is a valid ARM executable
#[unsafe(link_section = ".start_block")]
#[used]
pub static IMAGE_DEF: ImageDef = ImageDef::exe();

/// RP2350 default boot clock frequency
/// At boot, RP2350 runs from ROSC at approximately 12 MHz
const CLOCK_FREQ_HZ: u32 = 12_000_000;

#[entry]
fn main() -> ! {
    info!("RP2350 started!");
    info!("Blink example with GPIO LED control...");

    // Take device peripherals from PAC
    let dp = Peripherals::take().unwrap();

    // Initialize GPIO pins by consuming IO_BANK0 and PADS_BANK0
    let pins = Pins::init(dp.IO_BANK0, dp.PADS_BANK0);

    // Configure GPIO 25 (LED) as output
    let led = pins.pin::<25>();
    led.into_output();

    info!("GPIO 25 configured as output");

    // Take core peripherals (NVIC, SCB, SYST, MPU, FPU, SAU)
    let core_peripherals = CorePeripherals::take().unwrap();

    // Initialize SysTick-based delay
    let delay = Delay::init(core_peripherals.SYST, CLOCK_FREQ_HZ);

    let mut led_on = false;
    loop {
        // Toggle LED
        led.toggle();
        led_on = !led_on;

        if led_on {
            info!("LED ON");
        } else {
            info!("LED OFF");
        }

        // Delay for 500ms
        delay.delay_ms(500);
    }
}
```

## Common imports

`pico-rs` re-exports the most commonly used types from the HAL root, so simple
applications can often import directly from the crate:

```rust
use pico_rs::{Clocks, Delay, Gpio, Pins, Timer0, Uart0, Watchdog};
```

For lower-level access, use:

```rust
use pico_rs::pac;
use pico_rs::rt;
```

## Notes

- This crate is intended for bare-metal RP2350 targets such as `thumbv8m.main-none-eabihf`.
- If your final binary defines its own `#[panic_handler]`, disable the `panic-handler` feature.
- If you are targeting RP2350B, disable the default chip feature and enable `rp2350b` instead.

## Examples

The workspace contains runnable examples covering GPIO, clocks, DMA, PIO, USB,
BLDC control, and more in the `pico-rs-examples` crate.
