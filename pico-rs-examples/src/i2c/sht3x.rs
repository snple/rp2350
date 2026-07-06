//! # SHT30 I2C Example
//!
//! Read temperature and humidity from an SHT30 sensor over I2C
//! on a Raspberry Pi Pico 2 (RP2350).

#![no_std]
#![no_main]

use defmt::*;
use defmt_rtt as _;

use pico_rs::Delay;
use pico_rs::clocks::ClockBuilder;
use pico_rs::gpio::Pins;
use pico_rs::i2c::{Error, I2c0, I2cBus};
use pico_rs::main;
use pico_rs::pac::Peripherals;
use pico_rs::rt::peripheral::Peripherals as CorePeripherals;

#[main]
fn main() -> ! {
    info!("RP2350 started!");
    info!("SHT30 I2C example...");

    // Take device peripherals from PAC
    let dp = Peripherals::take().unwrap();

    // Initialize clock system (150 MHz sys, 48 MHz USB)
    let clocks = ClockBuilder::new().build(dp.CLOCKS, dp.XOSC, dp.PLL_SYS, dp.PLL_USB);
    info!("Clock system initialized: sys={} Hz", clocks.sys_freq());

    // Initialize GPIO pins by consuming IO_BANK0 and PADS_BANK0
    let pins = Pins::new(dp.IO_BANK0, dp.PADS_BANK0);

    // Initialize I2C0 on GPIO 4 (SDA) and GPIO 5 (SCL)
    let i2c = I2c0::new(dp.I2C0, pins.pin::<4>(), pins.pin::<5>(), 400_000, &clocks);

    // Take core peripherals (NVIC, SCB, SYST, MPU, FPU, SAU)
    let core_peripherals = CorePeripherals::take().unwrap();

    // Initialize SysTick-based delay
    let delay = Delay::new(core_peripherals.SYST, clocks.sys_freq());

    loop {
        match sht30_read(&i2c, &delay) {
            Ok((temp_c_x100, rh_pct_x100)) => {
                info!("Temp x100: {} C, RH x100: {} %", temp_c_x100, rh_pct_x100);
            }
            Err(e) => {
                info!("SHT30 read error: {}", e);
            }
        }

        delay.delay_ms(1000);
    }
}

const SHT30_ADDR: u8 = 0x44;
const SHT30_CMD_MEAS_HIGHREP: [u8; 2] = [0x24, 0x00];

fn sht30_read<B>(i2c: &B, delay: &Delay) -> Result<(i32, i32), Error>
where
    B: I2cBus<Error = Error>,
{
    let mut buf = [0u8; 6];

    i2c.write(SHT30_ADDR, &SHT30_CMD_MEAS_HIGHREP)?;
    delay.delay_ms(15);
    i2c.read(SHT30_ADDR, &mut buf)?;

    if crc8(&buf[0..2]) != buf[2] || crc8(&buf[3..5]) != buf[5] {
        return Err(Error::Abort);
    }

    let raw_t = u16::from_be_bytes([buf[0], buf[1]]);
    let raw_rh = u16::from_be_bytes([buf[3], buf[4]]);

    let temp_c = (((raw_t as i32) * 17500) / 65535) - 4500;
    let rh_pct = ((raw_rh as i32) * 10000) / 65535;

    Ok((temp_c, rh_pct))
}

fn crc8(data: &[u8]) -> u8 {
    let mut crc: u8 = 0xFF;
    for &byte in data {
        crc ^= byte;
        for _ in 0..8 {
            if (crc & 0x80) != 0 {
                crc = (crc << 1) ^ 0x31;
            } else {
                crc <<= 1;
            }
        }
    }
    crc
}
