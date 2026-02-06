//! GPIO Pin Function Example - Demonstrates type-safe pin function assignment
//!
//! This example demonstrates the type-safe GPIO pin function API:
//! - Setting pins to specific peripheral functions (UART, SPI, I2C, PWM)
//! - Using trait-based function constraints
//! - Pin function validation at compile time
//!
//! Note: This example shows the API usage. Actual peripheral initialization
//! would require additional setup.
//!
//! Run with: cargo run --release --bin gpio_test

#![no_std]
#![no_main]

use defmt::*;
use defmt_rtt as _;
use panic_probe as _;

use pico_rs::Delay;
use pico_rs::block::ImageDef;
use pico_rs::entry;
use pico_rs::gpio::{
    self, Pin0, Pin1, Pin4, Pin5, Pin8, Pin9, Pin10, Pin11, Pin12, Pin13, Pin16, Pin17, Pin18,
    Pin19, Pin25,
};

#[unsafe(link_section = ".start_block")]
#[used]
pub static IMAGE_DEF: ImageDef = ImageDef::secure_exe();

const CLOCK_FREQ_HZ: u32 = 12_000_000;

#[entry]
fn main() -> ! {
    info!("GPIO Pin Function Example");
    info!("==========================");

    gpio::init_gpio();
    let mut delay = Delay::new(CLOCK_FREQ_HZ);

    // ========================================================================
    // UART Pin Configuration Example
    // ========================================================================
    info!("");
    info!("--- UART Pin Configuration ---");

    // UART0: TX on GPIO0, RX on GPIO1 (valid functions based on hardware)
    let uart0_tx = Pin0::new();
    let uart0_rx = Pin1::new();

    // Convert pins to UART function using type-safe methods
    // These methods only exist on pins that support the specific function
    let _uart0_tx_pin = uart0_tx.into_uart0_tx();
    let _uart0_rx_pin = uart0_rx.into_uart0_rx();
    info!("UART0: TX=GPIO0, RX=GPIO1 configured");

    // UART1: TX on GPIO4, RX on GPIO5
    let uart1_tx = Pin4::new();
    let uart1_rx = Pin5::new();
    let _uart1_tx_pin = uart1_tx.into_uart1_tx();
    let _uart1_rx_pin = uart1_rx.into_uart1_rx();
    info!("UART1: TX=GPIO4, RX=GPIO5 configured");

    // ========================================================================
    // I2C Pin Configuration Example
    // ========================================================================
    info!("");
    info!("--- I2C Pin Configuration ---");

    // I2C0: SDA on GPIO8, SCL on GPIO9
    let i2c0_sda = Pin8::new();
    let i2c0_scl = Pin9::new();
    let _i2c0_sda_pin = i2c0_sda.into_i2c0_sda();
    let _i2c0_scl_pin = i2c0_scl.into_i2c0_scl();
    info!("I2C0: SDA=GPIO8, SCL=GPIO9 configured");

    // I2C1: SDA on GPIO10, SCL on GPIO11
    let i2c1_sda = Pin10::new();
    let i2c1_scl = Pin11::new();
    let _i2c1_sda_pin = i2c1_sda.into_i2c1_sda();
    let _i2c1_scl_pin = i2c1_scl.into_i2c1_scl();
    info!("I2C1: SDA=GPIO10, SCL=GPIO11 configured");

    // ========================================================================
    // SPI Pin Configuration Example
    // ========================================================================
    info!("");
    info!("--- SPI Pin Configuration ---");

    // SPI0: RX=GPIO16, CSn=GPIO17, SCK=GPIO18, TX=GPIO19
    let spi0_rx = Pin16::new();
    let spi0_csn = Pin17::new();
    let spi0_sck = Pin18::new();
    let spi0_tx = Pin19::new();

    let _spi0_rx_pin = spi0_rx.into_spi0_rx();
    let _spi0_csn_pin = spi0_csn.into_spi0_csn();
    let _spi0_sck_pin = spi0_sck.into_spi0_sck();
    let _spi0_tx_pin = spi0_tx.into_spi0_tx();
    info!("SPI0: RX=GPIO16, CSn=GPIO17, SCK=GPIO18, TX=GPIO19 configured");

    // ========================================================================
    // PWM Pin Configuration Example
    // ========================================================================
    info!("");
    info!("--- PWM Pin Configuration ---");

    // PWM channels are assigned based on GPIO number:
    // GPIO n uses PWM slice (n / 2) % 8, channel A if n is even, B if odd
    //
    // Examples:
    // - GPIO0 -> PWM0A, GPIO1 -> PWM0B
    // - GPIO2 -> PWM1A, GPIO3 -> PWM1B
    // - GPIO12 -> PWM6A, GPIO13 -> PWM6B
    // - GPIO25 -> PWM4B (25/2 = 12, 12 % 8 = 4, odd = channel B)

    // We'll use GPIO25 (LED) for PWM demonstration
    let led = Pin25::new();
    let _pwm_led = led.into_pwm();
    info!("GPIO25 (LED) configured as PWM channel B");

    // Additional PWM pins
    let pwm_pin0 = Pin12::new();
    let pwm_pin1 = Pin13::new();
    let _pwm0a = pwm_pin0.into_pwm();
    let _pwm0b = pwm_pin1.into_pwm();
    info!("GPIO12 -> PWM A, GPIO13 -> PWM B configured");

    // ========================================================================
    // Summary
    // ========================================================================
    info!("");
    info!("=== Summary ===");
    info!("Pin function assignment is type-safe:");
    info!("  - Only valid functions compile");
    info!("  - Invalid pin/function combos are compile errors");
    info!("  - Hardware function tables embedded in code");
    info!("");
    info!("Configured peripherals:");
    info!("  UART0: GPIO0(TX), GPIO1(RX)");
    info!("  UART1: GPIO4(TX), GPIO5(RX)");
    info!("  I2C0:  GPIO8(SDA), GPIO9(SCL)");
    info!("  I2C1:  GPIO10(SDA), GPIO11(SCL)");
    info!("  SPI0:  GPIO16(RX), GPIO17(CSn), GPIO18(SCK), GPIO19(TX)");
    info!("  PWM:   GPIO12(A), GPIO13(B), GPIO25(B)");

    info!("");
    info!("Test complete!");

    loop {
        delay.delay_ms(1000);
    }
}
