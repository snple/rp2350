//! RP2350 Hardware Register Definitions
//!
//! Auto-generated from RP2350.svd - DO NOT EDIT MANUALLY

#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(clippy::identity_op)]

use core::marker::PhantomData;
use core::ops::Deref;

pub mod common;

// ============================================================================
// Interrupt Handlers
// ============================================================================

unsafe extern "C" {
    fn TIMER0_IRQ_0();
    fn TIMER0_IRQ_1();
    fn TIMER0_IRQ_2();
    fn TIMER0_IRQ_3();
    fn TIMER1_IRQ_0();
    fn TIMER1_IRQ_1();
    fn TIMER1_IRQ_2();
    fn TIMER1_IRQ_3();
    fn PWM_IRQ_WRAP_0();
    fn PWM_IRQ_WRAP_1();
    fn DMA_IRQ_0();
    fn DMA_IRQ_1();
    fn DMA_IRQ_2();
    fn DMA_IRQ_3();
    fn USBCTRL_IRQ();
    fn PIO0_IRQ_0();
    fn PIO0_IRQ_1();
    fn PIO1_IRQ_0();
    fn PIO1_IRQ_1();
    fn PIO2_IRQ_0();
    fn PIO2_IRQ_1();
    fn IO_IRQ_BANK0();
    fn IO_IRQ_BANK0_NS();
    fn IO_IRQ_QSPI();
    fn IO_IRQ_QSPI_NS();
    fn SIO_IRQ_FIFO();
    fn SIO_IRQ_BELL();
    fn SIO_IRQ_FIFO_NS();
    fn SIO_IRQ_BELL_NS();
    fn SIO_IRQ_MTIMECMP();
    fn CLOCKS_IRQ();
    fn SPI0_IRQ();
    fn SPI1_IRQ();
    fn UART0_IRQ();
    fn UART1_IRQ();
    fn ADC_IRQ_FIFO();
    fn I2C0_IRQ();
    fn I2C1_IRQ();
    fn OTP_IRQ();
    fn TRNG_IRQ();
    fn PLL_SYS_IRQ();
    fn PLL_USB_IRQ();
    fn POWMAN_IRQ_POW();
    fn POWMAN_IRQ_TIMER();
    fn SPARE_IRQ_0();
    fn SPARE_IRQ_1();
    fn SPARE_IRQ_2();
    fn SPARE_IRQ_3();
    fn SPARE_IRQ_4();
    fn SPARE_IRQ_5();
}

/// Union type for interrupt vectors
#[doc(hidden)]
#[repr(C)]
pub union Vector {
    _handler: unsafe extern "C" fn(),
    _reserved: u32,
}

/// RP2350 interrupt vector table (52 interrupts)
#[doc(hidden)]
#[unsafe(link_section = ".vector_table.interrupts")]
#[unsafe(no_mangle)]
pub static __INTERRUPTS: [Vector; 52] = [
    Vector { _handler: TIMER0_IRQ_0 },
    Vector { _handler: TIMER0_IRQ_1 },
    Vector { _handler: TIMER0_IRQ_2 },
    Vector { _handler: TIMER0_IRQ_3 },
    Vector { _handler: TIMER1_IRQ_0 },
    Vector { _handler: TIMER1_IRQ_1 },
    Vector { _handler: TIMER1_IRQ_2 },
    Vector { _handler: TIMER1_IRQ_3 },
    Vector { _handler: PWM_IRQ_WRAP_0 },
    Vector { _handler: PWM_IRQ_WRAP_1 },
    Vector { _handler: DMA_IRQ_0 },
    Vector { _handler: DMA_IRQ_1 },
    Vector { _handler: DMA_IRQ_2 },
    Vector { _handler: DMA_IRQ_3 },
    Vector { _handler: USBCTRL_IRQ },
    Vector { _handler: PIO0_IRQ_0 },
    Vector { _handler: PIO0_IRQ_1 },
    Vector { _handler: PIO1_IRQ_0 },
    Vector { _handler: PIO1_IRQ_1 },
    Vector { _handler: PIO2_IRQ_0 },
    Vector { _handler: PIO2_IRQ_1 },
    Vector { _handler: IO_IRQ_BANK0 },
    Vector { _handler: IO_IRQ_BANK0_NS },
    Vector { _handler: IO_IRQ_QSPI },
    Vector { _handler: IO_IRQ_QSPI_NS },
    Vector { _handler: SIO_IRQ_FIFO },
    Vector { _handler: SIO_IRQ_BELL },
    Vector { _handler: SIO_IRQ_FIFO_NS },
    Vector { _handler: SIO_IRQ_BELL_NS },
    Vector { _handler: SIO_IRQ_MTIMECMP },
    Vector { _handler: CLOCKS_IRQ },
    Vector { _handler: SPI0_IRQ },
    Vector { _handler: SPI1_IRQ },
    Vector { _handler: UART0_IRQ },
    Vector { _handler: UART1_IRQ },
    Vector { _handler: ADC_IRQ_FIFO },
    Vector { _handler: I2C0_IRQ },
    Vector { _handler: I2C1_IRQ },
    Vector { _handler: OTP_IRQ },
    Vector { _handler: TRNG_IRQ },
    Vector { _reserved: 0 }, // Reserved
    Vector { _reserved: 0 }, // Reserved
    Vector { _handler: PLL_SYS_IRQ },
    Vector { _handler: PLL_USB_IRQ },
    Vector { _handler: POWMAN_IRQ_POW },
    Vector { _handler: POWMAN_IRQ_TIMER },
    Vector { _handler: SPARE_IRQ_0 },
    Vector { _handler: SPARE_IRQ_1 },
    Vector { _handler: SPARE_IRQ_2 },
    Vector { _handler: SPARE_IRQ_3 },
    Vector { _handler: SPARE_IRQ_4 },
    Vector { _handler: SPARE_IRQ_5 },
];

/// RP2350 Interrupt enumeration
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Interrupt {
    /// 0 - TIMER0 interrupt
    TIMER0_IRQ_0 = 0,
    /// 1 - TIMER0 interrupt
    TIMER0_IRQ_1 = 1,
    /// 2 - TIMER0 interrupt
    TIMER0_IRQ_2 = 2,
    /// 3 - TIMER0 interrupt
    TIMER0_IRQ_3 = 3,
    /// 4 - TIMER1 interrupt
    TIMER1_IRQ_0 = 4,
    /// 5 - TIMER1 interrupt
    TIMER1_IRQ_1 = 5,
    /// 6 - TIMER1 interrupt
    TIMER1_IRQ_2 = 6,
    /// 7 - TIMER1 interrupt
    TIMER1_IRQ_3 = 7,
    /// 8 - PWM interrupt
    PWM_IRQ_WRAP_0 = 8,
    /// 9 - PWM interrupt
    PWM_IRQ_WRAP_1 = 9,
    /// 10 - DMA interrupt
    DMA_IRQ_0 = 10,
    /// 11 - DMA interrupt
    DMA_IRQ_1 = 11,
    /// 12 - DMA interrupt
    DMA_IRQ_2 = 12,
    /// 13 - DMA interrupt
    DMA_IRQ_3 = 13,
    /// 14 - USB interrupt
    USBCTRL_IRQ = 14,
    /// 15 - PIO0 interrupt
    PIO0_IRQ_0 = 15,
    /// 16 - PIO0 interrupt
    PIO0_IRQ_1 = 16,
    /// 17 - PIO1 interrupt
    PIO1_IRQ_0 = 17,
    /// 18 - PIO1 interrupt
    PIO1_IRQ_1 = 18,
    /// 19 - PIO2 interrupt
    PIO2_IRQ_0 = 19,
    /// 20 - PIO2 interrupt
    PIO2_IRQ_1 = 20,
    /// 21 - IO_BANK0 interrupt
    IO_IRQ_BANK0 = 21,
    /// 22 - IO_BANK0 interrupt
    IO_IRQ_BANK0_NS = 22,
    /// 23 - IO_QSPI interrupt
    IO_IRQ_QSPI = 23,
    /// 24 - IO_QSPI interrupt
    IO_IRQ_QSPI_NS = 24,
    /// 25 - SIO interrupt
    SIO_IRQ_FIFO = 25,
    /// 26 - SIO interrupt
    SIO_IRQ_BELL = 26,
    /// 27 - SIO interrupt
    SIO_IRQ_FIFO_NS = 27,
    /// 28 - SIO interrupt
    SIO_IRQ_BELL_NS = 28,
    /// 29 - SIO interrupt
    SIO_IRQ_MTIMECMP = 29,
    /// 30 - CLOCKS interrupt
    CLOCKS_IRQ = 30,
    /// 31 - SPI0 interrupt
    SPI0_IRQ = 31,
    /// 32 - SPI1 interrupt
    SPI1_IRQ = 32,
    /// 33 - UART0 interrupt
    UART0_IRQ = 33,
    /// 34 - UART1 interrupt
    UART1_IRQ = 34,
    /// 35 - ADC interrupt
    ADC_IRQ_FIFO = 35,
    /// 36 - I2C0 interrupt
    I2C0_IRQ = 36,
    /// 37 - I2C1 interrupt
    I2C1_IRQ = 37,
    /// 38 - OTP interrupt
    OTP_IRQ = 38,
    /// 39 - TRNG interrupt
    TRNG_IRQ = 39,
    /// 42 - PLL_SYS interrupt
    PLL_SYS_IRQ = 42,
    /// 43 - PLL_USB interrupt
    PLL_USB_IRQ = 43,
    /// 44 - POWMAN interrupt
    POWMAN_IRQ_POW = 44,
    /// 45 - POWMAN interrupt
    POWMAN_IRQ_TIMER = 45,
    /// 46 - SPARE_IRQ interrupt
    SPARE_IRQ_0 = 46,
    /// 47 - SPARE_IRQ interrupt
    SPARE_IRQ_1 = 47,
    /// 48 - SPARE_IRQ interrupt
    SPARE_IRQ_2 = 48,
    /// 49 - SPARE_IRQ interrupt
    SPARE_IRQ_3 = 49,
    /// 50 - SPARE_IRQ interrupt
    SPARE_IRQ_4 = 50,
    /// 51 - SPARE_IRQ interrupt
    SPARE_IRQ_5 = 51,
}

unsafe impl roschi_rt::peripheral::nvic::InterruptNumber for Interrupt {
    #[inline(always)]
    fn number(self) -> u16 {
        self as u16
    }
}

// ============================================================================
// Peripheral Wrapper Macro
// ============================================================================

/// Helper macro to create peripheral wrappers
macro_rules! peripheral {
    ($name:ident, $reg:ty, $base:expr) => {
        impl $name {
            /// Pointer to the peripheral's register block
            pub const PTR: *const $reg = $base as *const $reg;

            /// Steal an instance of this peripheral
            ///
            /// # Safety
            ///
            /// Ensure that the new instance of the peripheral cannot be used in a way
            /// that may race with any existing instances.
            #[inline]
            pub const unsafe fn steal() -> Self {
                Self {
                    _marker: PhantomData,
                }
            }
        }

        impl Deref for $name {
            type Target = $reg;

            #[inline(always)]
            fn deref(&self) -> &Self::Target {
                unsafe { &*Self::PTR }
            }
        }

        impl core::fmt::Debug for $name {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct(stringify!($name)).finish()
            }
        }
    };
}

// ============================================================================
// Peripheral Definitions
// ============================================================================

/// RESETS peripheral
pub struct RESETS {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for RESETS {}

peripheral!(RESETS, resets::RegisterBlock, 0x40020000);

/// RESETS register module
pub mod resets;

/// PSM peripheral
pub struct PSM {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for PSM {}

peripheral!(PSM, psm::RegisterBlock, 0x40018000);

/// PSM register module
pub mod psm;

/// CLOCKS peripheral
pub struct CLOCKS {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for CLOCKS {}

peripheral!(CLOCKS, clocks::RegisterBlock, 0x40010000);

/// CLOCKS register module
pub mod clocks;

/// TICKS peripheral
pub struct TICKS {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for TICKS {}

peripheral!(TICKS, ticks::RegisterBlock, 0x40108000);

/// TICKS register module
pub mod ticks;

/// PADS_BANK0 peripheral
pub struct PADS_BANK0 {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for PADS_BANK0 {}

peripheral!(PADS_BANK0, pads_bank0::RegisterBlock, 0x40038000);

/// PADS_BANK0 register module
pub mod pads_bank0;

/// PADS_QSPI peripheral
pub struct PADS_QSPI {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for PADS_QSPI {}

peripheral!(PADS_QSPI, pads_qspi::RegisterBlock, 0x40040000);

/// PADS_QSPI register module
pub mod pads_qspi;

/// IO_QSPI peripheral
pub struct IO_QSPI {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for IO_QSPI {}

peripheral!(IO_QSPI, io_qspi::RegisterBlock, 0x40030000);

/// IO_QSPI register module
pub mod io_qspi;

/// IO_BANK0 peripheral
pub struct IO_BANK0 {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for IO_BANK0 {}

peripheral!(IO_BANK0, io_bank0::RegisterBlock, 0x40028000);

/// IO_BANK0 register module
pub mod io_bank0;

/// SYSINFO peripheral
pub struct SYSINFO {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for SYSINFO {}

peripheral!(SYSINFO, sysinfo::RegisterBlock, 0x40000000);

/// SYSINFO register module
pub mod sysinfo;

/// SHA-256 hash function implementation
pub struct SHA256 {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for SHA256 {}

peripheral!(SHA256, sha256::RegisterBlock, 0x400f8000);

/// SHA-256 hash function implementation
pub mod sha256;

/// FIFO status and write access for HSTX
pub struct HSTX_FIFO {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for HSTX_FIFO {}

peripheral!(HSTX_FIFO, hstx_fifo::RegisterBlock, 0x50600000);

/// FIFO status and write access for HSTX
pub mod hstx_fifo;

/// Control interface to HSTX. For FIFO write access and status, see the HSTX_FIFO register block.
pub struct HSTX_CTRL {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for HSTX_CTRL {}

peripheral!(HSTX_CTRL, hstx_ctrl::RegisterBlock, 0x400c0000);

/// Control interface to HSTX. For FIFO write access and status, see the HSTX_FIFO register block.
pub mod hstx_ctrl;

/// Cortex-M33 EPPB vendor register block for RP2350
pub struct EPPB {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for EPPB {}

peripheral!(EPPB, eppb::RegisterBlock, 0xe0080000);

/// Cortex-M33 EPPB vendor register block for RP2350
pub mod eppb;

/// TEAL registers accessible through the debug interface
pub struct PPB {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for PPB {}

peripheral!(PPB, ppb::RegisterBlock, 0xe0000000);

/// TEAL registers accessible through the debug interface
pub mod ppb;

/// PPB_NS peripheral
pub struct PPB_NS {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for PPB_NS {}

peripheral!(PPB_NS, ppb_ns::RegisterBlock, 0xe0020000);

/// PPB_NS register module
pub mod ppb_ns;

/// QSPI Memory Interface.
pub struct QMI {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for QMI {}

peripheral!(QMI, qmi::RegisterBlock, 0x400d0000);

/// QSPI Memory Interface.
pub mod qmi;

/// QSPI flash execute-in-place block
pub struct XIP_CTRL {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for XIP_CTRL {}

peripheral!(XIP_CTRL, xip_ctrl::RegisterBlock, 0x400c8000);

/// QSPI flash execute-in-place block
pub mod xip_ctrl;

/// Auxiliary DMA access to XIP FIFOs, via fast AHB bus access
pub struct XIP_AUX {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for XIP_AUX {}

peripheral!(XIP_AUX, xip_aux::RegisterBlock, 0x50500000);

/// Auxiliary DMA access to XIP FIFOs, via fast AHB bus access
pub mod xip_aux;

/// Register block for various chip control signals
pub struct SYSCFG {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for SYSCFG {}

peripheral!(SYSCFG, syscfg::RegisterBlock, 0x40008000);

/// Register block for various chip control signals
pub mod syscfg;

/// Controls the crystal oscillator
pub struct XOSC {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for XOSC {}

peripheral!(XOSC, xosc::RegisterBlock, 0x40048000);

/// Controls the crystal oscillator
pub mod xosc;

/// PLL_SYS peripheral
pub struct PLL_SYS {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for PLL_SYS {}

peripheral!(PLL_SYS, pll_sys::RegisterBlock, 0x40050000);

/// PLL_SYS register module
pub mod pll_sys;

/// PLL_USB peripheral
pub struct PLL_USB {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for PLL_USB {}

peripheral!(PLL_USB, pll_usb::RegisterBlock, 0x40058000);

/// PLL_USB register module
pub mod pll_usb;

/// Hardware access control registers
pub struct ACCESSCTRL {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for ACCESSCTRL {}

peripheral!(ACCESSCTRL, accessctrl::RegisterBlock, 0x40060000);

/// Hardware access control registers
pub mod accessctrl;

/// UART0 peripheral
pub struct UART0 {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for UART0 {}

peripheral!(UART0, uart0::RegisterBlock, 0x40070000);

/// UART0 register module
pub mod uart0;

/// UART1 peripheral
pub struct UART1 {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for UART1 {}

peripheral!(UART1, uart1::RegisterBlock, 0x40078000);

/// UART1 register module
pub mod uart1;

/// ROSC peripheral
pub struct ROSC {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for ROSC {}

peripheral!(ROSC, rosc::RegisterBlock, 0x400e8000);

/// ROSC register module
pub mod rosc;

/// Controls vreg, bor, lposc, chip resets & xosc startup, powman and provides scratch register for general use and for bootcode use
pub struct POWMAN {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for POWMAN {}

peripheral!(POWMAN, powman::RegisterBlock, 0x40100000);

/// Controls vreg, bor, lposc, chip resets & xosc startup, powman and provides scratch register for general use and for bootcode use
pub mod powman;

/// WATCHDOG peripheral
pub struct WATCHDOG {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for WATCHDOG {}

peripheral!(WATCHDOG, watchdog::RegisterBlock, 0x400d8000);

/// WATCHDOG register module
pub mod watchdog;

/// DMA with separate read and write masters
pub struct DMA {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for DMA {}

peripheral!(DMA, dma::RegisterBlock, 0x50000000);

/// DMA with separate read and write masters
pub mod dma;

/// Controls time and alarms
pub struct TIMER0 {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for TIMER0 {}

peripheral!(TIMER0, timer0::RegisterBlock, 0x400b0000);

/// Controls time and alarms
pub mod timer0;

/// TIMER1 peripheral
pub struct TIMER1 {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for TIMER1 {}

peripheral!(TIMER1, timer1::RegisterBlock, 0x400b8000);

/// TIMER1 register module
pub mod timer1;

/// Simple PWM
pub struct PWM {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for PWM {}

peripheral!(PWM, pwm::RegisterBlock, 0x400a8000);

/// Simple PWM
pub mod pwm;

/// Control and data interface to SAR ADC
pub struct ADC {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for ADC {}

peripheral!(ADC, adc::RegisterBlock, 0x400a0000);

/// Control and data interface to SAR ADC
pub mod adc;

/// DW_apb_i2c address block
pub struct I2C0 {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for I2C0 {}

peripheral!(I2C0, i2c0::RegisterBlock, 0x40090000);

/// DW_apb_i2c address block
pub mod i2c0;

/// I2C1 peripheral
pub struct I2C1 {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for I2C1 {}

peripheral!(I2C1, i2c1::RegisterBlock, 0x40098000);

/// I2C1 register module
pub mod i2c1;

/// SPI0 peripheral
pub struct SPI0 {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for SPI0 {}

peripheral!(SPI0, spi0::RegisterBlock, 0x40080000);

/// SPI0 register module
pub mod spi0;

/// SPI1 peripheral
pub struct SPI1 {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for SPI1 {}

peripheral!(SPI1, spi1::RegisterBlock, 0x40088000);

/// SPI1 register module
pub mod spi1;

/// Programmable IO block
pub struct PIO0 {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for PIO0 {}

peripheral!(PIO0, pio0::RegisterBlock, 0x50200000);

/// Programmable IO block
pub mod pio0;

/// PIO1 peripheral
pub struct PIO1 {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for PIO1 {}

peripheral!(PIO1, pio1::RegisterBlock, 0x50300000);

/// PIO1 register module
pub mod pio1;

/// PIO2 peripheral
pub struct PIO2 {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for PIO2 {}

peripheral!(PIO2, pio2::RegisterBlock, 0x50400000);

/// PIO2 register module
pub mod pio2;

/// Register block for busfabric control signals and performance counters
pub struct BUSCTRL {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for BUSCTRL {}

peripheral!(BUSCTRL, busctrl::RegisterBlock, 0x40068000);

/// Register block for busfabric control signals and performance counters
pub mod busctrl;

/// Single-cycle IO block
pub struct SIO {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for SIO {}

peripheral!(SIO, sio::RegisterBlock, 0xd0000000);

/// Single-cycle IO block
pub mod sio;

/// SIO_NS peripheral
pub struct SIO_NS {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for SIO_NS {}

peripheral!(SIO_NS, sio_ns::RegisterBlock, 0xd0020000);

/// SIO_NS register module
pub mod sio_ns;

/// Additional registers mapped adjacent to the bootram, for use by the bootrom.
pub struct BOOTRAM {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for BOOTRAM {}

peripheral!(BOOTRAM, bootram::RegisterBlock, 0x400e0000);

/// Additional registers mapped adjacent to the bootram, for use by the bootrom.
pub mod bootram;

/// Coresight block - RP specific registers
pub struct CORESIGHT_TRACE {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for CORESIGHT_TRACE {}

peripheral!(CORESIGHT_TRACE, coresight_trace::RegisterBlock, 0x50700000);

/// Coresight block - RP specific registers
pub mod coresight_trace;

/// USB FS/LS controller device registers
pub struct USB {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for USB {}

peripheral!(USB, usb::RegisterBlock, 0x50110000);

/// USB FS/LS controller device registers
pub mod usb;

/// ARM TrustZone RNG register block
pub struct TRNG {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for TRNG {}

peripheral!(TRNG, trng::RegisterBlock, 0x400f0000);

/// ARM TrustZone RNG register block
pub mod trng;

/// Glitch detector controls
pub struct GLITCH_DETECTOR {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for GLITCH_DETECTOR {}

peripheral!(GLITCH_DETECTOR, glitch_detector::RegisterBlock, 0x40158000);

/// Glitch detector controls
pub mod glitch_detector;

/// SNPS OTP control IF (SBPI and RPi wrapper control)
pub struct OTP {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for OTP {}

peripheral!(OTP, otp::RegisterBlock, 0x40120000);

/// SNPS OTP control IF (SBPI and RPi wrapper control)
pub mod otp;

/// Predefined OTP data layout for RP2350
pub struct OTP_DATA {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for OTP_DATA {}

peripheral!(OTP_DATA, otp_data::RegisterBlock, 0x40130000);

/// Predefined OTP data layout for RP2350
pub mod otp_data;

/// Predefined OTP data layout for RP2350
pub struct OTP_DATA_RAW {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for OTP_DATA_RAW {}

peripheral!(OTP_DATA_RAW, otp_data_raw::RegisterBlock, 0x40134000);

/// Predefined OTP data layout for RP2350
pub mod otp_data_raw;

/// For managing simulation testbenches
pub struct TBMAN {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for TBMAN {}

peripheral!(TBMAN, tbman::RegisterBlock, 0x40160000);

/// For managing simulation testbenches
pub mod tbman;

/// DPRAM layout for USB device.
pub struct USB_DPRAM {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for USB_DPRAM {}

peripheral!(USB_DPRAM, usb_dpram::RegisterBlock, 0x50100000);

/// DPRAM layout for USB device.
pub mod usb_dpram;

/// SPARE_IRQ peripheral
pub struct SPARE_IRQ {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for SPARE_IRQ {}

peripheral!(SPARE_IRQ, spare_irq::RegisterBlock, 0x00000000);

/// SPARE_IRQ register module
pub mod spare_irq;

// ============================================================================
// Peripherals - All device peripherals
// ============================================================================

use core::sync::atomic::{AtomicBool, Ordering};

/// Guard to ensure peripherals are taken only once
static TAKEN: AtomicBool = AtomicBool::new(false);

/// All the peripherals.
#[allow(non_snake_case)]
pub struct Peripherals {
    /// RESETS peripheral
    pub RESETS: RESETS,
    /// PSM peripheral
    pub PSM: PSM,
    /// CLOCKS peripheral
    pub CLOCKS: CLOCKS,
    /// TICKS peripheral
    pub TICKS: TICKS,
    /// PADS_BANK0 peripheral
    pub PADS_BANK0: PADS_BANK0,
    /// PADS_QSPI peripheral
    pub PADS_QSPI: PADS_QSPI,
    /// IO_QSPI peripheral
    pub IO_QSPI: IO_QSPI,
    /// IO_BANK0 peripheral
    pub IO_BANK0: IO_BANK0,
    /// SYSINFO peripheral
    pub SYSINFO: SYSINFO,
    /// SHA-256 hash function implementation
    pub SHA256: SHA256,
    /// FIFO status and write access for HSTX
    pub HSTX_FIFO: HSTX_FIFO,
    /// Control interface to HSTX. For FIFO write access and status, see the HSTX_FIFO register block.
    pub HSTX_CTRL: HSTX_CTRL,
    /// Cortex-M33 EPPB vendor register block for RP2350
    pub EPPB: EPPB,
    /// TEAL registers accessible through the debug interface
    pub PPB: PPB,
    /// PPB_NS peripheral
    pub PPB_NS: PPB_NS,
    /// QSPI Memory Interface.
    pub QMI: QMI,
    /// QSPI flash execute-in-place block
    pub XIP_CTRL: XIP_CTRL,
    /// Auxiliary DMA access to XIP FIFOs, via fast AHB bus access
    pub XIP_AUX: XIP_AUX,
    /// Register block for various chip control signals
    pub SYSCFG: SYSCFG,
    /// Controls the crystal oscillator
    pub XOSC: XOSC,
    /// PLL_SYS peripheral
    pub PLL_SYS: PLL_SYS,
    /// PLL_USB peripheral
    pub PLL_USB: PLL_USB,
    /// Hardware access control registers
    pub ACCESSCTRL: ACCESSCTRL,
    /// UART0 peripheral
    pub UART0: UART0,
    /// UART1 peripheral
    pub UART1: UART1,
    /// ROSC peripheral
    pub ROSC: ROSC,
    /// Controls vreg, bor, lposc, chip resets & xosc startup, powman and provides scratch register for general use and for bootcode use
    pub POWMAN: POWMAN,
    /// WATCHDOG peripheral
    pub WATCHDOG: WATCHDOG,
    /// DMA with separate read and write masters
    pub DMA: DMA,
    /// Controls time and alarms
    pub TIMER0: TIMER0,
    /// TIMER1 peripheral
    pub TIMER1: TIMER1,
    /// Simple PWM
    pub PWM: PWM,
    /// Control and data interface to SAR ADC
    pub ADC: ADC,
    /// DW_apb_i2c address block
    pub I2C0: I2C0,
    /// I2C1 peripheral
    pub I2C1: I2C1,
    /// SPI0 peripheral
    pub SPI0: SPI0,
    /// SPI1 peripheral
    pub SPI1: SPI1,
    /// Programmable IO block
    pub PIO0: PIO0,
    /// PIO1 peripheral
    pub PIO1: PIO1,
    /// PIO2 peripheral
    pub PIO2: PIO2,
    /// Register block for busfabric control signals and performance counters
    pub BUSCTRL: BUSCTRL,
    /// Single-cycle IO block
    pub SIO: SIO,
    /// SIO_NS peripheral
    pub SIO_NS: SIO_NS,
    /// Additional registers mapped adjacent to the bootram, for use by the bootrom.
    pub BOOTRAM: BOOTRAM,
    /// Coresight block - RP specific registers
    pub CORESIGHT_TRACE: CORESIGHT_TRACE,
    /// USB FS/LS controller device registers
    pub USB: USB,
    /// ARM TrustZone RNG register block
    pub TRNG: TRNG,
    /// Glitch detector controls
    pub GLITCH_DETECTOR: GLITCH_DETECTOR,
    /// SNPS OTP control IF (SBPI and RPi wrapper control)
    pub OTP: OTP,
    /// Predefined OTP data layout for RP2350
    pub OTP_DATA: OTP_DATA,
    /// Predefined OTP data layout for RP2350
    pub OTP_DATA_RAW: OTP_DATA_RAW,
    /// For managing simulation testbenches
    pub TBMAN: TBMAN,
    /// DPRAM layout for USB device.
    pub USB_DPRAM: USB_DPRAM,
    /// SPARE_IRQ peripheral
    pub SPARE_IRQ: SPARE_IRQ,
}

impl Peripherals {
    /// Returns all the peripherals *once*.
    #[inline]
    pub fn take() -> Option<Self> {
        if TAKEN.swap(true, Ordering::AcqRel) {
            None
        } else {
            Some(unsafe { Self::steal() })
        }
    }

    /// Unchecked version of `Peripherals::take`.
    ///
    /// # Safety
    ///
    /// Each of the returned peripherals must be used at most once.
    #[inline]
    pub unsafe fn steal() -> Self {
        unsafe {
            Peripherals {
                RESETS: RESETS::steal(),
                PSM: PSM::steal(),
                CLOCKS: CLOCKS::steal(),
                TICKS: TICKS::steal(),
                PADS_BANK0: PADS_BANK0::steal(),
                PADS_QSPI: PADS_QSPI::steal(),
                IO_QSPI: IO_QSPI::steal(),
                IO_BANK0: IO_BANK0::steal(),
                SYSINFO: SYSINFO::steal(),
                SHA256: SHA256::steal(),
                HSTX_FIFO: HSTX_FIFO::steal(),
                HSTX_CTRL: HSTX_CTRL::steal(),
                EPPB: EPPB::steal(),
                PPB: PPB::steal(),
                PPB_NS: PPB_NS::steal(),
                QMI: QMI::steal(),
                XIP_CTRL: XIP_CTRL::steal(),
                XIP_AUX: XIP_AUX::steal(),
                SYSCFG: SYSCFG::steal(),
                XOSC: XOSC::steal(),
                PLL_SYS: PLL_SYS::steal(),
                PLL_USB: PLL_USB::steal(),
                ACCESSCTRL: ACCESSCTRL::steal(),
                UART0: UART0::steal(),
                UART1: UART1::steal(),
                ROSC: ROSC::steal(),
                POWMAN: POWMAN::steal(),
                WATCHDOG: WATCHDOG::steal(),
                DMA: DMA::steal(),
                TIMER0: TIMER0::steal(),
                TIMER1: TIMER1::steal(),
                PWM: PWM::steal(),
                ADC: ADC::steal(),
                I2C0: I2C0::steal(),
                I2C1: I2C1::steal(),
                SPI0: SPI0::steal(),
                SPI1: SPI1::steal(),
                PIO0: PIO0::steal(),
                PIO1: PIO1::steal(),
                PIO2: PIO2::steal(),
                BUSCTRL: BUSCTRL::steal(),
                SIO: SIO::steal(),
                SIO_NS: SIO_NS::steal(),
                BOOTRAM: BOOTRAM::steal(),
                CORESIGHT_TRACE: CORESIGHT_TRACE::steal(),
                USB: USB::steal(),
                TRNG: TRNG::steal(),
                GLITCH_DETECTOR: GLITCH_DETECTOR::steal(),
                OTP: OTP::steal(),
                OTP_DATA: OTP_DATA::steal(),
                OTP_DATA_RAW: OTP_DATA_RAW::steal(),
                TBMAN: TBMAN::steal(),
                USB_DPRAM: USB_DPRAM::steal(),
                SPARE_IRQ: SPARE_IRQ::steal(),
            }
        }
    }

    /// Release the peripherals back to the pool
    ///
    /// After calling this, `take()` will succeed again.
    #[inline]
    pub fn release(self) {
        TAKEN.store(false, Ordering::Release);
    }
}
