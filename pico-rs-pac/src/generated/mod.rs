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

#[cfg(feature = "rt")]
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
#[cfg(feature = "rt")]
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

#[cfg(feature = "cortex-m")]
unsafe impl cortex_m::interrupt::InterruptNumber for Interrupt {
    #[inline(always)]
    fn number(self) -> u16 {
        self as u16
    }
}

// ============================================================================
// Peripheral Definitions
// ============================================================================

/// RESETS peripheral
pub struct RESETS {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for RESETS {}

impl RESETS {
    /// Pointer to the register block
    pub const PTR: *const resets::RegisterBlock = 0x40020000 as *const _;

    /// Return the pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const resets::RegisterBlock {
        Self::PTR
    }

    /// Steal an instance of this peripheral
    ///
    /// # Safety
    ///
    /// Ensure that the new instance of the peripheral cannot be used in a way
    /// that may race with any existing instances.
    #[inline]
    pub unsafe fn steal() -> Self {
        Self { _marker: PhantomData }
    }
}

impl Deref for RESETS {
    type Target = resets::RegisterBlock;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}

impl core::fmt::Debug for RESETS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RESETS").finish()
    }
}

/// RESETS register module
pub mod resets;

/// PSM peripheral
pub struct PSM {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for PSM {}

impl PSM {
    /// Pointer to the register block
    pub const PTR: *const psm::RegisterBlock = 0x40018000 as *const _;

    /// Return the pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const psm::RegisterBlock {
        Self::PTR
    }

    /// Steal an instance of this peripheral
    ///
    /// # Safety
    ///
    /// Ensure that the new instance of the peripheral cannot be used in a way
    /// that may race with any existing instances.
    #[inline]
    pub unsafe fn steal() -> Self {
        Self { _marker: PhantomData }
    }
}

impl Deref for PSM {
    type Target = psm::RegisterBlock;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}

impl core::fmt::Debug for PSM {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PSM").finish()
    }
}

/// PSM register module
pub mod psm;

/// CLOCKS peripheral
pub struct CLOCKS {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for CLOCKS {}

impl CLOCKS {
    /// Pointer to the register block
    pub const PTR: *const clocks::RegisterBlock = 0x40010000 as *const _;

    /// Return the pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const clocks::RegisterBlock {
        Self::PTR
    }

    /// Steal an instance of this peripheral
    ///
    /// # Safety
    ///
    /// Ensure that the new instance of the peripheral cannot be used in a way
    /// that may race with any existing instances.
    #[inline]
    pub unsafe fn steal() -> Self {
        Self { _marker: PhantomData }
    }
}

impl Deref for CLOCKS {
    type Target = clocks::RegisterBlock;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}

impl core::fmt::Debug for CLOCKS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLOCKS").finish()
    }
}

/// CLOCKS register module
pub mod clocks;

/// TICKS peripheral
pub struct TICKS {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for TICKS {}

impl TICKS {
    /// Pointer to the register block
    pub const PTR: *const ticks::RegisterBlock = 0x40108000 as *const _;

    /// Return the pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const ticks::RegisterBlock {
        Self::PTR
    }

    /// Steal an instance of this peripheral
    ///
    /// # Safety
    ///
    /// Ensure that the new instance of the peripheral cannot be used in a way
    /// that may race with any existing instances.
    #[inline]
    pub unsafe fn steal() -> Self {
        Self { _marker: PhantomData }
    }
}

impl Deref for TICKS {
    type Target = ticks::RegisterBlock;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}

impl core::fmt::Debug for TICKS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TICKS").finish()
    }
}

/// TICKS register module
pub mod ticks;

/// PADS_BANK0 peripheral
pub struct PADS_BANK0 {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for PADS_BANK0 {}

impl PADS_BANK0 {
    /// Pointer to the register block
    pub const PTR: *const pads_bank0::RegisterBlock = 0x40038000 as *const _;

    /// Return the pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const pads_bank0::RegisterBlock {
        Self::PTR
    }

    /// Steal an instance of this peripheral
    ///
    /// # Safety
    ///
    /// Ensure that the new instance of the peripheral cannot be used in a way
    /// that may race with any existing instances.
    #[inline]
    pub unsafe fn steal() -> Self {
        Self { _marker: PhantomData }
    }
}

impl Deref for PADS_BANK0 {
    type Target = pads_bank0::RegisterBlock;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}

impl core::fmt::Debug for PADS_BANK0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PADS_BANK0").finish()
    }
}

/// PADS_BANK0 register module
pub mod pads_bank0;

/// PADS_QSPI peripheral
pub struct PADS_QSPI {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for PADS_QSPI {}

impl PADS_QSPI {
    /// Pointer to the register block
    pub const PTR: *const pads_qspi::RegisterBlock = 0x40040000 as *const _;

    /// Return the pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const pads_qspi::RegisterBlock {
        Self::PTR
    }

    /// Steal an instance of this peripheral
    ///
    /// # Safety
    ///
    /// Ensure that the new instance of the peripheral cannot be used in a way
    /// that may race with any existing instances.
    #[inline]
    pub unsafe fn steal() -> Self {
        Self { _marker: PhantomData }
    }
}

impl Deref for PADS_QSPI {
    type Target = pads_qspi::RegisterBlock;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}

impl core::fmt::Debug for PADS_QSPI {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PADS_QSPI").finish()
    }
}

/// PADS_QSPI register module
pub mod pads_qspi;

/// IO_QSPI peripheral
pub struct IO_QSPI {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for IO_QSPI {}

impl IO_QSPI {
    /// Pointer to the register block
    pub const PTR: *const io_qspi::RegisterBlock = 0x40030000 as *const _;

    /// Return the pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const io_qspi::RegisterBlock {
        Self::PTR
    }

    /// Steal an instance of this peripheral
    ///
    /// # Safety
    ///
    /// Ensure that the new instance of the peripheral cannot be used in a way
    /// that may race with any existing instances.
    #[inline]
    pub unsafe fn steal() -> Self {
        Self { _marker: PhantomData }
    }
}

impl Deref for IO_QSPI {
    type Target = io_qspi::RegisterBlock;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}

impl core::fmt::Debug for IO_QSPI {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IO_QSPI").finish()
    }
}

/// IO_QSPI register module
pub mod io_qspi;

/// IO_BANK0 peripheral
pub struct IO_BANK0 {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for IO_BANK0 {}

impl IO_BANK0 {
    /// Pointer to the register block
    pub const PTR: *const io_bank0::RegisterBlock = 0x40028000 as *const _;

    /// Return the pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const io_bank0::RegisterBlock {
        Self::PTR
    }

    /// Steal an instance of this peripheral
    ///
    /// # Safety
    ///
    /// Ensure that the new instance of the peripheral cannot be used in a way
    /// that may race with any existing instances.
    #[inline]
    pub unsafe fn steal() -> Self {
        Self { _marker: PhantomData }
    }
}

impl Deref for IO_BANK0 {
    type Target = io_bank0::RegisterBlock;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}

impl core::fmt::Debug for IO_BANK0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IO_BANK0").finish()
    }
}

/// IO_BANK0 register module
pub mod io_bank0;

/// SYSINFO peripheral
pub struct SYSINFO {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for SYSINFO {}

impl SYSINFO {
    /// Pointer to the register block
    pub const PTR: *const sysinfo::RegisterBlock = 0x40000000 as *const _;

    /// Return the pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const sysinfo::RegisterBlock {
        Self::PTR
    }

    /// Steal an instance of this peripheral
    ///
    /// # Safety
    ///
    /// Ensure that the new instance of the peripheral cannot be used in a way
    /// that may race with any existing instances.
    #[inline]
    pub unsafe fn steal() -> Self {
        Self { _marker: PhantomData }
    }
}

impl Deref for SYSINFO {
    type Target = sysinfo::RegisterBlock;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}

impl core::fmt::Debug for SYSINFO {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYSINFO").finish()
    }
}

/// SYSINFO register module
pub mod sysinfo;

/// SHA-256 hash function implementation
pub struct SHA256 {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for SHA256 {}

impl SHA256 {
    /// Pointer to the register block
    pub const PTR: *const sha256::RegisterBlock = 0x400f8000 as *const _;

    /// Return the pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const sha256::RegisterBlock {
        Self::PTR
    }

    /// Steal an instance of this peripheral
    ///
    /// # Safety
    ///
    /// Ensure that the new instance of the peripheral cannot be used in a way
    /// that may race with any existing instances.
    #[inline]
    pub unsafe fn steal() -> Self {
        Self { _marker: PhantomData }
    }
}

impl Deref for SHA256 {
    type Target = sha256::RegisterBlock;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}

impl core::fmt::Debug for SHA256 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SHA256").finish()
    }
}

/// SHA-256 hash function implementation
pub mod sha256;

/// FIFO status and write access for HSTX
pub struct HSTX_FIFO {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for HSTX_FIFO {}

impl HSTX_FIFO {
    /// Pointer to the register block
    pub const PTR: *const hstx_fifo::RegisterBlock = 0x50600000 as *const _;

    /// Return the pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const hstx_fifo::RegisterBlock {
        Self::PTR
    }

    /// Steal an instance of this peripheral
    ///
    /// # Safety
    ///
    /// Ensure that the new instance of the peripheral cannot be used in a way
    /// that may race with any existing instances.
    #[inline]
    pub unsafe fn steal() -> Self {
        Self { _marker: PhantomData }
    }
}

impl Deref for HSTX_FIFO {
    type Target = hstx_fifo::RegisterBlock;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}

impl core::fmt::Debug for HSTX_FIFO {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HSTX_FIFO").finish()
    }
}

/// FIFO status and write access for HSTX
pub mod hstx_fifo;

/// Control interface to HSTX. For FIFO write access and status, see the HSTX_F...
pub struct HSTX_CTRL {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for HSTX_CTRL {}

impl HSTX_CTRL {
    /// Pointer to the register block
    pub const PTR: *const hstx_ctrl::RegisterBlock = 0x400c0000 as *const _;

    /// Return the pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const hstx_ctrl::RegisterBlock {
        Self::PTR
    }

    /// Steal an instance of this peripheral
    ///
    /// # Safety
    ///
    /// Ensure that the new instance of the peripheral cannot be used in a way
    /// that may race with any existing instances.
    #[inline]
    pub unsafe fn steal() -> Self {
        Self { _marker: PhantomData }
    }
}

impl Deref for HSTX_CTRL {
    type Target = hstx_ctrl::RegisterBlock;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}

impl core::fmt::Debug for HSTX_CTRL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HSTX_CTRL").finish()
    }
}

/// Control interface to HSTX. For FIFO write access and status, see the HSTX_F...
pub mod hstx_ctrl;

/// Cortex-M33 EPPB vendor register block for RP2350
pub struct EPPB {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for EPPB {}

impl EPPB {
    /// Pointer to the register block
    pub const PTR: *const eppb::RegisterBlock = 0xe0080000 as *const _;

    /// Return the pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const eppb::RegisterBlock {
        Self::PTR
    }

    /// Steal an instance of this peripheral
    ///
    /// # Safety
    ///
    /// Ensure that the new instance of the peripheral cannot be used in a way
    /// that may race with any existing instances.
    #[inline]
    pub unsafe fn steal() -> Self {
        Self { _marker: PhantomData }
    }
}

impl Deref for EPPB {
    type Target = eppb::RegisterBlock;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}

impl core::fmt::Debug for EPPB {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EPPB").finish()
    }
}

/// Cortex-M33 EPPB vendor register block for RP2350
pub mod eppb;

/// TEAL registers accessible through the debug interface
pub struct PPB {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for PPB {}

impl PPB {
    /// Pointer to the register block
    pub const PTR: *const ppb::RegisterBlock = 0xe0000000 as *const _;

    /// Return the pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const ppb::RegisterBlock {
        Self::PTR
    }

    /// Steal an instance of this peripheral
    ///
    /// # Safety
    ///
    /// Ensure that the new instance of the peripheral cannot be used in a way
    /// that may race with any existing instances.
    #[inline]
    pub unsafe fn steal() -> Self {
        Self { _marker: PhantomData }
    }
}

impl Deref for PPB {
    type Target = ppb::RegisterBlock;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}

impl core::fmt::Debug for PPB {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PPB").finish()
    }
}

/// TEAL registers accessible through the debug interface
pub mod ppb;

/// PPB_NS peripheral
pub struct PPB_NS {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for PPB_NS {}

impl PPB_NS {
    /// Pointer to the register block
    pub const PTR: *const ppb_ns::RegisterBlock = 0xe0020000 as *const _;

    /// Return the pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const ppb_ns::RegisterBlock {
        Self::PTR
    }

    /// Steal an instance of this peripheral
    ///
    /// # Safety
    ///
    /// Ensure that the new instance of the peripheral cannot be used in a way
    /// that may race with any existing instances.
    #[inline]
    pub unsafe fn steal() -> Self {
        Self { _marker: PhantomData }
    }
}

impl Deref for PPB_NS {
    type Target = ppb_ns::RegisterBlock;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}

impl core::fmt::Debug for PPB_NS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PPB_NS").finish()
    }
}

/// PPB_NS register module
pub mod ppb_ns;

/// QSPI Memory Interface. Provides a memory-mapped interface to up to two SPI/...
pub struct QMI {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for QMI {}

impl QMI {
    /// Pointer to the register block
    pub const PTR: *const qmi::RegisterBlock = 0x400d0000 as *const _;

    /// Return the pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const qmi::RegisterBlock {
        Self::PTR
    }

    /// Steal an instance of this peripheral
    ///
    /// # Safety
    ///
    /// Ensure that the new instance of the peripheral cannot be used in a way
    /// that may race with any existing instances.
    #[inline]
    pub unsafe fn steal() -> Self {
        Self { _marker: PhantomData }
    }
}

impl Deref for QMI {
    type Target = qmi::RegisterBlock;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}

impl core::fmt::Debug for QMI {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("QMI").finish()
    }
}

/// QSPI Memory Interface. Provides a memory-mapped interface to up to two SPI/...
pub mod qmi;

/// QSPI flash execute-in-place block
pub struct XIP_CTRL {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for XIP_CTRL {}

impl XIP_CTRL {
    /// Pointer to the register block
    pub const PTR: *const xip_ctrl::RegisterBlock = 0x400c8000 as *const _;

    /// Return the pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const xip_ctrl::RegisterBlock {
        Self::PTR
    }

    /// Steal an instance of this peripheral
    ///
    /// # Safety
    ///
    /// Ensure that the new instance of the peripheral cannot be used in a way
    /// that may race with any existing instances.
    #[inline]
    pub unsafe fn steal() -> Self {
        Self { _marker: PhantomData }
    }
}

impl Deref for XIP_CTRL {
    type Target = xip_ctrl::RegisterBlock;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}

impl core::fmt::Debug for XIP_CTRL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("XIP_CTRL").finish()
    }
}

/// QSPI flash execute-in-place block
pub mod xip_ctrl;

/// Auxiliary DMA access to XIP FIFOs, via fast AHB bus access
pub struct XIP_AUX {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for XIP_AUX {}

impl XIP_AUX {
    /// Pointer to the register block
    pub const PTR: *const xip_aux::RegisterBlock = 0x50500000 as *const _;

    /// Return the pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const xip_aux::RegisterBlock {
        Self::PTR
    }

    /// Steal an instance of this peripheral
    ///
    /// # Safety
    ///
    /// Ensure that the new instance of the peripheral cannot be used in a way
    /// that may race with any existing instances.
    #[inline]
    pub unsafe fn steal() -> Self {
        Self { _marker: PhantomData }
    }
}

impl Deref for XIP_AUX {
    type Target = xip_aux::RegisterBlock;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}

impl core::fmt::Debug for XIP_AUX {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("XIP_AUX").finish()
    }
}

/// Auxiliary DMA access to XIP FIFOs, via fast AHB bus access
pub mod xip_aux;

/// Register block for various chip control signals
pub struct SYSCFG {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for SYSCFG {}

impl SYSCFG {
    /// Pointer to the register block
    pub const PTR: *const syscfg::RegisterBlock = 0x40008000 as *const _;

    /// Return the pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const syscfg::RegisterBlock {
        Self::PTR
    }

    /// Steal an instance of this peripheral
    ///
    /// # Safety
    ///
    /// Ensure that the new instance of the peripheral cannot be used in a way
    /// that may race with any existing instances.
    #[inline]
    pub unsafe fn steal() -> Self {
        Self { _marker: PhantomData }
    }
}

impl Deref for SYSCFG {
    type Target = syscfg::RegisterBlock;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}

impl core::fmt::Debug for SYSCFG {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYSCFG").finish()
    }
}

/// Register block for various chip control signals
pub mod syscfg;

/// Controls the crystal oscillator
pub struct XOSC {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for XOSC {}

impl XOSC {
    /// Pointer to the register block
    pub const PTR: *const xosc::RegisterBlock = 0x40048000 as *const _;

    /// Return the pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const xosc::RegisterBlock {
        Self::PTR
    }

    /// Steal an instance of this peripheral
    ///
    /// # Safety
    ///
    /// Ensure that the new instance of the peripheral cannot be used in a way
    /// that may race with any existing instances.
    #[inline]
    pub unsafe fn steal() -> Self {
        Self { _marker: PhantomData }
    }
}

impl Deref for XOSC {
    type Target = xosc::RegisterBlock;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}

impl core::fmt::Debug for XOSC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("XOSC").finish()
    }
}

/// Controls the crystal oscillator
pub mod xosc;

/// PLL_SYS peripheral
pub struct PLL_SYS {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for PLL_SYS {}

impl PLL_SYS {
    /// Pointer to the register block
    pub const PTR: *const pll_sys::RegisterBlock = 0x40050000 as *const _;

    /// Return the pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const pll_sys::RegisterBlock {
        Self::PTR
    }

    /// Steal an instance of this peripheral
    ///
    /// # Safety
    ///
    /// Ensure that the new instance of the peripheral cannot be used in a way
    /// that may race with any existing instances.
    #[inline]
    pub unsafe fn steal() -> Self {
        Self { _marker: PhantomData }
    }
}

impl Deref for PLL_SYS {
    type Target = pll_sys::RegisterBlock;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}

impl core::fmt::Debug for PLL_SYS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PLL_SYS").finish()
    }
}

/// PLL_SYS register module
pub mod pll_sys;

/// PLL_USB peripheral
pub struct PLL_USB {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for PLL_USB {}

impl PLL_USB {
    /// Pointer to the register block
    pub const PTR: *const pll_usb::RegisterBlock = 0x40058000 as *const _;

    /// Return the pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const pll_usb::RegisterBlock {
        Self::PTR
    }

    /// Steal an instance of this peripheral
    ///
    /// # Safety
    ///
    /// Ensure that the new instance of the peripheral cannot be used in a way
    /// that may race with any existing instances.
    #[inline]
    pub unsafe fn steal() -> Self {
        Self { _marker: PhantomData }
    }
}

impl Deref for PLL_USB {
    type Target = pll_usb::RegisterBlock;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}

impl core::fmt::Debug for PLL_USB {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PLL_USB").finish()
    }
}

/// PLL_USB register module
pub mod pll_usb;

/// Hardware access control registers
pub struct ACCESSCTRL {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for ACCESSCTRL {}

impl ACCESSCTRL {
    /// Pointer to the register block
    pub const PTR: *const accessctrl::RegisterBlock = 0x40060000 as *const _;

    /// Return the pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const accessctrl::RegisterBlock {
        Self::PTR
    }

    /// Steal an instance of this peripheral
    ///
    /// # Safety
    ///
    /// Ensure that the new instance of the peripheral cannot be used in a way
    /// that may race with any existing instances.
    #[inline]
    pub unsafe fn steal() -> Self {
        Self { _marker: PhantomData }
    }
}

impl Deref for ACCESSCTRL {
    type Target = accessctrl::RegisterBlock;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}

impl core::fmt::Debug for ACCESSCTRL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ACCESSCTRL").finish()
    }
}

/// Hardware access control registers
pub mod accessctrl;

/// UART0 peripheral
pub struct UART0 {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for UART0 {}

impl UART0 {
    /// Pointer to the register block
    pub const PTR: *const uart0::RegisterBlock = 0x40070000 as *const _;

    /// Return the pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const uart0::RegisterBlock {
        Self::PTR
    }

    /// Steal an instance of this peripheral
    ///
    /// # Safety
    ///
    /// Ensure that the new instance of the peripheral cannot be used in a way
    /// that may race with any existing instances.
    #[inline]
    pub unsafe fn steal() -> Self {
        Self { _marker: PhantomData }
    }
}

impl Deref for UART0 {
    type Target = uart0::RegisterBlock;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}

impl core::fmt::Debug for UART0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UART0").finish()
    }
}

/// UART0 register module
pub mod uart0;

/// UART1 peripheral
pub struct UART1 {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for UART1 {}

impl UART1 {
    /// Pointer to the register block
    pub const PTR: *const uart1::RegisterBlock = 0x40078000 as *const _;

    /// Return the pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const uart1::RegisterBlock {
        Self::PTR
    }

    /// Steal an instance of this peripheral
    ///
    /// # Safety
    ///
    /// Ensure that the new instance of the peripheral cannot be used in a way
    /// that may race with any existing instances.
    #[inline]
    pub unsafe fn steal() -> Self {
        Self { _marker: PhantomData }
    }
}

impl Deref for UART1 {
    type Target = uart1::RegisterBlock;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}

impl core::fmt::Debug for UART1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UART1").finish()
    }
}

/// UART1 register module
pub mod uart1;

/// ROSC peripheral
pub struct ROSC {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for ROSC {}

impl ROSC {
    /// Pointer to the register block
    pub const PTR: *const rosc::RegisterBlock = 0x400e8000 as *const _;

    /// Return the pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const rosc::RegisterBlock {
        Self::PTR
    }

    /// Steal an instance of this peripheral
    ///
    /// # Safety
    ///
    /// Ensure that the new instance of the peripheral cannot be used in a way
    /// that may race with any existing instances.
    #[inline]
    pub unsafe fn steal() -> Self {
        Self { _marker: PhantomData }
    }
}

impl Deref for ROSC {
    type Target = rosc::RegisterBlock;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}

impl core::fmt::Debug for ROSC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ROSC").finish()
    }
}

/// ROSC register module
pub mod rosc;

/// Controls vreg, bor, lposc, chip resets & xosc startup, powman and provides ...
pub struct POWMAN {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for POWMAN {}

impl POWMAN {
    /// Pointer to the register block
    pub const PTR: *const powman::RegisterBlock = 0x40100000 as *const _;

    /// Return the pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const powman::RegisterBlock {
        Self::PTR
    }

    /// Steal an instance of this peripheral
    ///
    /// # Safety
    ///
    /// Ensure that the new instance of the peripheral cannot be used in a way
    /// that may race with any existing instances.
    #[inline]
    pub unsafe fn steal() -> Self {
        Self { _marker: PhantomData }
    }
}

impl Deref for POWMAN {
    type Target = powman::RegisterBlock;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}

impl core::fmt::Debug for POWMAN {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("POWMAN").finish()
    }
}

/// Controls vreg, bor, lposc, chip resets & xosc startup, powman and provides ...
pub mod powman;

/// WATCHDOG peripheral
pub struct WATCHDOG {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for WATCHDOG {}

impl WATCHDOG {
    /// Pointer to the register block
    pub const PTR: *const watchdog::RegisterBlock = 0x400d8000 as *const _;

    /// Return the pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const watchdog::RegisterBlock {
        Self::PTR
    }

    /// Steal an instance of this peripheral
    ///
    /// # Safety
    ///
    /// Ensure that the new instance of the peripheral cannot be used in a way
    /// that may race with any existing instances.
    #[inline]
    pub unsafe fn steal() -> Self {
        Self { _marker: PhantomData }
    }
}

impl Deref for WATCHDOG {
    type Target = watchdog::RegisterBlock;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}

impl core::fmt::Debug for WATCHDOG {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WATCHDOG").finish()
    }
}

/// WATCHDOG register module
pub mod watchdog;

/// DMA with separate read and write masters
pub struct DMA {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for DMA {}

impl DMA {
    /// Pointer to the register block
    pub const PTR: *const dma::RegisterBlock = 0x50000000 as *const _;

    /// Return the pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const dma::RegisterBlock {
        Self::PTR
    }

    /// Steal an instance of this peripheral
    ///
    /// # Safety
    ///
    /// Ensure that the new instance of the peripheral cannot be used in a way
    /// that may race with any existing instances.
    #[inline]
    pub unsafe fn steal() -> Self {
        Self { _marker: PhantomData }
    }
}

impl Deref for DMA {
    type Target = dma::RegisterBlock;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}

impl core::fmt::Debug for DMA {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA").finish()
    }
}

/// DMA with separate read and write masters
pub mod dma;

/// Controls time and alarms time is a 64 bit value indicating the time since p...
pub struct TIMER0 {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for TIMER0 {}

impl TIMER0 {
    /// Pointer to the register block
    pub const PTR: *const timer0::RegisterBlock = 0x400b0000 as *const _;

    /// Return the pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const timer0::RegisterBlock {
        Self::PTR
    }

    /// Steal an instance of this peripheral
    ///
    /// # Safety
    ///
    /// Ensure that the new instance of the peripheral cannot be used in a way
    /// that may race with any existing instances.
    #[inline]
    pub unsafe fn steal() -> Self {
        Self { _marker: PhantomData }
    }
}

impl Deref for TIMER0 {
    type Target = timer0::RegisterBlock;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}

impl core::fmt::Debug for TIMER0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMER0").finish()
    }
}

/// Controls time and alarms time is a 64 bit value indicating the time since p...
pub mod timer0;

/// TIMER1 peripheral
pub struct TIMER1 {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for TIMER1 {}

impl TIMER1 {
    /// Pointer to the register block
    pub const PTR: *const timer1::RegisterBlock = 0x400b8000 as *const _;

    /// Return the pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const timer1::RegisterBlock {
        Self::PTR
    }

    /// Steal an instance of this peripheral
    ///
    /// # Safety
    ///
    /// Ensure that the new instance of the peripheral cannot be used in a way
    /// that may race with any existing instances.
    #[inline]
    pub unsafe fn steal() -> Self {
        Self { _marker: PhantomData }
    }
}

impl Deref for TIMER1 {
    type Target = timer1::RegisterBlock;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}

impl core::fmt::Debug for TIMER1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMER1").finish()
    }
}

/// TIMER1 register module
pub mod timer1;

/// Simple PWM
pub struct PWM {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for PWM {}

impl PWM {
    /// Pointer to the register block
    pub const PTR: *const pwm::RegisterBlock = 0x400a8000 as *const _;

    /// Return the pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const pwm::RegisterBlock {
        Self::PTR
    }

    /// Steal an instance of this peripheral
    ///
    /// # Safety
    ///
    /// Ensure that the new instance of the peripheral cannot be used in a way
    /// that may race with any existing instances.
    #[inline]
    pub unsafe fn steal() -> Self {
        Self { _marker: PhantomData }
    }
}

impl Deref for PWM {
    type Target = pwm::RegisterBlock;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}

impl core::fmt::Debug for PWM {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWM").finish()
    }
}

/// Simple PWM
pub mod pwm;

/// Control and data interface to SAR ADC
pub struct ADC {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for ADC {}

impl ADC {
    /// Pointer to the register block
    pub const PTR: *const adc::RegisterBlock = 0x400a0000 as *const _;

    /// Return the pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const adc::RegisterBlock {
        Self::PTR
    }

    /// Steal an instance of this peripheral
    ///
    /// # Safety
    ///
    /// Ensure that the new instance of the peripheral cannot be used in a way
    /// that may race with any existing instances.
    #[inline]
    pub unsafe fn steal() -> Self {
        Self { _marker: PhantomData }
    }
}

impl Deref for ADC {
    type Target = adc::RegisterBlock;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}

impl core::fmt::Debug for ADC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC").finish()
    }
}

/// Control and data interface to SAR ADC
pub mod adc;

/// DW_apb_i2c address block List of configuration constants for the Synopsys I...
pub struct I2C0 {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for I2C0 {}

impl I2C0 {
    /// Pointer to the register block
    pub const PTR: *const i2c0::RegisterBlock = 0x40090000 as *const _;

    /// Return the pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const i2c0::RegisterBlock {
        Self::PTR
    }

    /// Steal an instance of this peripheral
    ///
    /// # Safety
    ///
    /// Ensure that the new instance of the peripheral cannot be used in a way
    /// that may race with any existing instances.
    #[inline]
    pub unsafe fn steal() -> Self {
        Self { _marker: PhantomData }
    }
}

impl Deref for I2C0 {
    type Target = i2c0::RegisterBlock;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}

impl core::fmt::Debug for I2C0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2C0").finish()
    }
}

/// DW_apb_i2c address block List of configuration constants for the Synopsys I...
pub mod i2c0;

/// I2C1 peripheral
pub struct I2C1 {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for I2C1 {}

impl I2C1 {
    /// Pointer to the register block
    pub const PTR: *const i2c1::RegisterBlock = 0x40098000 as *const _;

    /// Return the pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const i2c1::RegisterBlock {
        Self::PTR
    }

    /// Steal an instance of this peripheral
    ///
    /// # Safety
    ///
    /// Ensure that the new instance of the peripheral cannot be used in a way
    /// that may race with any existing instances.
    #[inline]
    pub unsafe fn steal() -> Self {
        Self { _marker: PhantomData }
    }
}

impl Deref for I2C1 {
    type Target = i2c1::RegisterBlock;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}

impl core::fmt::Debug for I2C1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2C1").finish()
    }
}

/// I2C1 register module
pub mod i2c1;

/// SPI0 peripheral
pub struct SPI0 {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for SPI0 {}

impl SPI0 {
    /// Pointer to the register block
    pub const PTR: *const spi0::RegisterBlock = 0x40080000 as *const _;

    /// Return the pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const spi0::RegisterBlock {
        Self::PTR
    }

    /// Steal an instance of this peripheral
    ///
    /// # Safety
    ///
    /// Ensure that the new instance of the peripheral cannot be used in a way
    /// that may race with any existing instances.
    #[inline]
    pub unsafe fn steal() -> Self {
        Self { _marker: PhantomData }
    }
}

impl Deref for SPI0 {
    type Target = spi0::RegisterBlock;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}

impl core::fmt::Debug for SPI0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI0").finish()
    }
}

/// SPI0 register module
pub mod spi0;

/// SPI1 peripheral
pub struct SPI1 {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for SPI1 {}

impl SPI1 {
    /// Pointer to the register block
    pub const PTR: *const spi1::RegisterBlock = 0x40088000 as *const _;

    /// Return the pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const spi1::RegisterBlock {
        Self::PTR
    }

    /// Steal an instance of this peripheral
    ///
    /// # Safety
    ///
    /// Ensure that the new instance of the peripheral cannot be used in a way
    /// that may race with any existing instances.
    #[inline]
    pub unsafe fn steal() -> Self {
        Self { _marker: PhantomData }
    }
}

impl Deref for SPI1 {
    type Target = spi1::RegisterBlock;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}

impl core::fmt::Debug for SPI1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI1").finish()
    }
}

/// SPI1 register module
pub mod spi1;

/// Programmable IO block
pub struct PIO0 {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for PIO0 {}

impl PIO0 {
    /// Pointer to the register block
    pub const PTR: *const pio0::RegisterBlock = 0x50200000 as *const _;

    /// Return the pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const pio0::RegisterBlock {
        Self::PTR
    }

    /// Steal an instance of this peripheral
    ///
    /// # Safety
    ///
    /// Ensure that the new instance of the peripheral cannot be used in a way
    /// that may race with any existing instances.
    #[inline]
    pub unsafe fn steal() -> Self {
        Self { _marker: PhantomData }
    }
}

impl Deref for PIO0 {
    type Target = pio0::RegisterBlock;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}

impl core::fmt::Debug for PIO0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PIO0").finish()
    }
}

/// Programmable IO block
pub mod pio0;

/// PIO1 peripheral
pub struct PIO1 {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for PIO1 {}

impl PIO1 {
    /// Pointer to the register block
    pub const PTR: *const pio1::RegisterBlock = 0x50300000 as *const _;

    /// Return the pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const pio1::RegisterBlock {
        Self::PTR
    }

    /// Steal an instance of this peripheral
    ///
    /// # Safety
    ///
    /// Ensure that the new instance of the peripheral cannot be used in a way
    /// that may race with any existing instances.
    #[inline]
    pub unsafe fn steal() -> Self {
        Self { _marker: PhantomData }
    }
}

impl Deref for PIO1 {
    type Target = pio1::RegisterBlock;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}

impl core::fmt::Debug for PIO1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PIO1").finish()
    }
}

/// PIO1 register module
pub mod pio1;

/// PIO2 peripheral
pub struct PIO2 {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for PIO2 {}

impl PIO2 {
    /// Pointer to the register block
    pub const PTR: *const pio2::RegisterBlock = 0x50400000 as *const _;

    /// Return the pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const pio2::RegisterBlock {
        Self::PTR
    }

    /// Steal an instance of this peripheral
    ///
    /// # Safety
    ///
    /// Ensure that the new instance of the peripheral cannot be used in a way
    /// that may race with any existing instances.
    #[inline]
    pub unsafe fn steal() -> Self {
        Self { _marker: PhantomData }
    }
}

impl Deref for PIO2 {
    type Target = pio2::RegisterBlock;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}

impl core::fmt::Debug for PIO2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PIO2").finish()
    }
}

/// PIO2 register module
pub mod pio2;

/// Register block for busfabric control signals and performance counters
pub struct BUSCTRL {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for BUSCTRL {}

impl BUSCTRL {
    /// Pointer to the register block
    pub const PTR: *const busctrl::RegisterBlock = 0x40068000 as *const _;

    /// Return the pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const busctrl::RegisterBlock {
        Self::PTR
    }

    /// Steal an instance of this peripheral
    ///
    /// # Safety
    ///
    /// Ensure that the new instance of the peripheral cannot be used in a way
    /// that may race with any existing instances.
    #[inline]
    pub unsafe fn steal() -> Self {
        Self { _marker: PhantomData }
    }
}

impl Deref for BUSCTRL {
    type Target = busctrl::RegisterBlock;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}

impl core::fmt::Debug for BUSCTRL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BUSCTRL").finish()
    }
}

/// Register block for busfabric control signals and performance counters
pub mod busctrl;

/// Single-cycle IO block Provides core-local and inter-core hardware for the t...
pub struct SIO {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for SIO {}

impl SIO {
    /// Pointer to the register block
    pub const PTR: *const sio::RegisterBlock = 0xd0000000 as *const _;

    /// Return the pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const sio::RegisterBlock {
        Self::PTR
    }

    /// Steal an instance of this peripheral
    ///
    /// # Safety
    ///
    /// Ensure that the new instance of the peripheral cannot be used in a way
    /// that may race with any existing instances.
    #[inline]
    pub unsafe fn steal() -> Self {
        Self { _marker: PhantomData }
    }
}

impl Deref for SIO {
    type Target = sio::RegisterBlock;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}

impl core::fmt::Debug for SIO {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SIO").finish()
    }
}

/// Single-cycle IO block Provides core-local and inter-core hardware for the t...
pub mod sio;

/// SIO_NS peripheral
pub struct SIO_NS {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for SIO_NS {}

impl SIO_NS {
    /// Pointer to the register block
    pub const PTR: *const sio_ns::RegisterBlock = 0xd0020000 as *const _;

    /// Return the pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const sio_ns::RegisterBlock {
        Self::PTR
    }

    /// Steal an instance of this peripheral
    ///
    /// # Safety
    ///
    /// Ensure that the new instance of the peripheral cannot be used in a way
    /// that may race with any existing instances.
    #[inline]
    pub unsafe fn steal() -> Self {
        Self { _marker: PhantomData }
    }
}

impl Deref for SIO_NS {
    type Target = sio_ns::RegisterBlock;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}

impl core::fmt::Debug for SIO_NS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SIO_NS").finish()
    }
}

/// SIO_NS register module
pub mod sio_ns;

/// Additional registers mapped adjacent to the bootram, for use by the bootrom.
pub struct BOOTRAM {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for BOOTRAM {}

impl BOOTRAM {
    /// Pointer to the register block
    pub const PTR: *const bootram::RegisterBlock = 0x400e0000 as *const _;

    /// Return the pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const bootram::RegisterBlock {
        Self::PTR
    }

    /// Steal an instance of this peripheral
    ///
    /// # Safety
    ///
    /// Ensure that the new instance of the peripheral cannot be used in a way
    /// that may race with any existing instances.
    #[inline]
    pub unsafe fn steal() -> Self {
        Self { _marker: PhantomData }
    }
}

impl Deref for BOOTRAM {
    type Target = bootram::RegisterBlock;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}

impl core::fmt::Debug for BOOTRAM {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BOOTRAM").finish()
    }
}

/// Additional registers mapped adjacent to the bootram, for use by the bootrom.
pub mod bootram;

/// Coresight block - RP specific registers
pub struct CORESIGHT_TRACE {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for CORESIGHT_TRACE {}

impl CORESIGHT_TRACE {
    /// Pointer to the register block
    pub const PTR: *const coresight_trace::RegisterBlock = 0x50700000 as *const _;

    /// Return the pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const coresight_trace::RegisterBlock {
        Self::PTR
    }

    /// Steal an instance of this peripheral
    ///
    /// # Safety
    ///
    /// Ensure that the new instance of the peripheral cannot be used in a way
    /// that may race with any existing instances.
    #[inline]
    pub unsafe fn steal() -> Self {
        Self { _marker: PhantomData }
    }
}

impl Deref for CORESIGHT_TRACE {
    type Target = coresight_trace::RegisterBlock;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}

impl core::fmt::Debug for CORESIGHT_TRACE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORESIGHT_TRACE").finish()
    }
}

/// Coresight block - RP specific registers
pub mod coresight_trace;

/// USB FS/LS controller device registers
pub struct USB {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for USB {}

impl USB {
    /// Pointer to the register block
    pub const PTR: *const usb::RegisterBlock = 0x50110000 as *const _;

    /// Return the pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const usb::RegisterBlock {
        Self::PTR
    }

    /// Steal an instance of this peripheral
    ///
    /// # Safety
    ///
    /// Ensure that the new instance of the peripheral cannot be used in a way
    /// that may race with any existing instances.
    #[inline]
    pub unsafe fn steal() -> Self {
        Self { _marker: PhantomData }
    }
}

impl Deref for USB {
    type Target = usb::RegisterBlock;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}

impl core::fmt::Debug for USB {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USB").finish()
    }
}

/// USB FS/LS controller device registers
pub mod usb;

/// ARM TrustZone RNG register block
pub struct TRNG {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for TRNG {}

impl TRNG {
    /// Pointer to the register block
    pub const PTR: *const trng::RegisterBlock = 0x400f0000 as *const _;

    /// Return the pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const trng::RegisterBlock {
        Self::PTR
    }

    /// Steal an instance of this peripheral
    ///
    /// # Safety
    ///
    /// Ensure that the new instance of the peripheral cannot be used in a way
    /// that may race with any existing instances.
    #[inline]
    pub unsafe fn steal() -> Self {
        Self { _marker: PhantomData }
    }
}

impl Deref for TRNG {
    type Target = trng::RegisterBlock;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}

impl core::fmt::Debug for TRNG {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TRNG").finish()
    }
}

/// ARM TrustZone RNG register block
pub mod trng;

/// Glitch detector controls
pub struct GLITCH_DETECTOR {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for GLITCH_DETECTOR {}

impl GLITCH_DETECTOR {
    /// Pointer to the register block
    pub const PTR: *const glitch_detector::RegisterBlock = 0x40158000 as *const _;

    /// Return the pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const glitch_detector::RegisterBlock {
        Self::PTR
    }

    /// Steal an instance of this peripheral
    ///
    /// # Safety
    ///
    /// Ensure that the new instance of the peripheral cannot be used in a way
    /// that may race with any existing instances.
    #[inline]
    pub unsafe fn steal() -> Self {
        Self { _marker: PhantomData }
    }
}

impl Deref for GLITCH_DETECTOR {
    type Target = glitch_detector::RegisterBlock;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}

impl core::fmt::Debug for GLITCH_DETECTOR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GLITCH_DETECTOR").finish()
    }
}

/// Glitch detector controls
pub mod glitch_detector;

/// SNPS OTP control IF (SBPI and RPi wrapper control)
pub struct OTP {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for OTP {}

impl OTP {
    /// Pointer to the register block
    pub const PTR: *const otp::RegisterBlock = 0x40120000 as *const _;

    /// Return the pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const otp::RegisterBlock {
        Self::PTR
    }

    /// Steal an instance of this peripheral
    ///
    /// # Safety
    ///
    /// Ensure that the new instance of the peripheral cannot be used in a way
    /// that may race with any existing instances.
    #[inline]
    pub unsafe fn steal() -> Self {
        Self { _marker: PhantomData }
    }
}

impl Deref for OTP {
    type Target = otp::RegisterBlock;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}

impl core::fmt::Debug for OTP {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OTP").finish()
    }
}

/// SNPS OTP control IF (SBPI and RPi wrapper control)
pub mod otp;

/// Predefined OTP data layout for RP2350
pub struct OTP_DATA {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for OTP_DATA {}

impl OTP_DATA {
    /// Pointer to the register block
    pub const PTR: *const otp_data::RegisterBlock = 0x40130000 as *const _;

    /// Return the pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const otp_data::RegisterBlock {
        Self::PTR
    }

    /// Steal an instance of this peripheral
    ///
    /// # Safety
    ///
    /// Ensure that the new instance of the peripheral cannot be used in a way
    /// that may race with any existing instances.
    #[inline]
    pub unsafe fn steal() -> Self {
        Self { _marker: PhantomData }
    }
}

impl Deref for OTP_DATA {
    type Target = otp_data::RegisterBlock;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}

impl core::fmt::Debug for OTP_DATA {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OTP_DATA").finish()
    }
}

/// Predefined OTP data layout for RP2350
pub mod otp_data;

/// Predefined OTP data layout for RP2350
pub struct OTP_DATA_RAW {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for OTP_DATA_RAW {}

impl OTP_DATA_RAW {
    /// Pointer to the register block
    pub const PTR: *const otp_data_raw::RegisterBlock = 0x40134000 as *const _;

    /// Return the pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const otp_data_raw::RegisterBlock {
        Self::PTR
    }

    /// Steal an instance of this peripheral
    ///
    /// # Safety
    ///
    /// Ensure that the new instance of the peripheral cannot be used in a way
    /// that may race with any existing instances.
    #[inline]
    pub unsafe fn steal() -> Self {
        Self { _marker: PhantomData }
    }
}

impl Deref for OTP_DATA_RAW {
    type Target = otp_data_raw::RegisterBlock;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}

impl core::fmt::Debug for OTP_DATA_RAW {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OTP_DATA_RAW").finish()
    }
}

/// Predefined OTP data layout for RP2350
pub mod otp_data_raw;

/// For managing simulation testbenches
pub struct TBMAN {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for TBMAN {}

impl TBMAN {
    /// Pointer to the register block
    pub const PTR: *const tbman::RegisterBlock = 0x40160000 as *const _;

    /// Return the pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const tbman::RegisterBlock {
        Self::PTR
    }

    /// Steal an instance of this peripheral
    ///
    /// # Safety
    ///
    /// Ensure that the new instance of the peripheral cannot be used in a way
    /// that may race with any existing instances.
    #[inline]
    pub unsafe fn steal() -> Self {
        Self { _marker: PhantomData }
    }
}

impl Deref for TBMAN {
    type Target = tbman::RegisterBlock;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}

impl core::fmt::Debug for TBMAN {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TBMAN").finish()
    }
}

/// For managing simulation testbenches
pub mod tbman;

/// DPRAM layout for USB device.
pub struct USB_DPRAM {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for USB_DPRAM {}

impl USB_DPRAM {
    /// Pointer to the register block
    pub const PTR: *const usb_dpram::RegisterBlock = 0x50100000 as *const _;

    /// Return the pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const usb_dpram::RegisterBlock {
        Self::PTR
    }

    /// Steal an instance of this peripheral
    ///
    /// # Safety
    ///
    /// Ensure that the new instance of the peripheral cannot be used in a way
    /// that may race with any existing instances.
    #[inline]
    pub unsafe fn steal() -> Self {
        Self { _marker: PhantomData }
    }
}

impl Deref for USB_DPRAM {
    type Target = usb_dpram::RegisterBlock;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}

impl core::fmt::Debug for USB_DPRAM {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USB_DPRAM").finish()
    }
}

/// DPRAM layout for USB device.
pub mod usb_dpram;

/// SPARE_IRQ peripheral
pub struct SPARE_IRQ {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for SPARE_IRQ {}

impl SPARE_IRQ {
    /// Pointer to the register block
    pub const PTR: *const spare_irq::RegisterBlock = 0x00000000 as *const _;

    /// Return the pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const spare_irq::RegisterBlock {
        Self::PTR
    }

    /// Steal an instance of this peripheral
    ///
    /// # Safety
    ///
    /// Ensure that the new instance of the peripheral cannot be used in a way
    /// that may race with any existing instances.
    #[inline]
    pub unsafe fn steal() -> Self {
        Self { _marker: PhantomData }
    }
}

impl Deref for SPARE_IRQ {
    type Target = spare_irq::RegisterBlock;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}

impl core::fmt::Debug for SPARE_IRQ {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPARE_IRQ").finish()
    }
}

/// SPARE_IRQ register module
pub mod spare_irq;

// ============================================================================
// Peripherals - All device peripherals
// ============================================================================

#[unsafe(no_mangle)]
static mut DEVICE_PERIPHERALS: bool = false;

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
    /// Control interface to HSTX. For FIFO write access and status, see the HSTX_F...
    pub HSTX_CTRL: HSTX_CTRL,
    /// Cortex-M33 EPPB vendor register block for RP2350
    pub EPPB: EPPB,
    /// TEAL registers accessible through the debug interface
    pub PPB: PPB,
    /// PPB_NS peripheral
    pub PPB_NS: PPB_NS,
    /// QSPI Memory Interface. Provides a memory-mapped interface to up to two SPI/...
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
    /// Controls vreg, bor, lposc, chip resets & xosc startup, powman and provides ...
    pub POWMAN: POWMAN,
    /// WATCHDOG peripheral
    pub WATCHDOG: WATCHDOG,
    /// DMA with separate read and write masters
    pub DMA: DMA,
    /// Controls time and alarms time is a 64 bit value indicating the time since p...
    pub TIMER0: TIMER0,
    /// TIMER1 peripheral
    pub TIMER1: TIMER1,
    /// Simple PWM
    pub PWM: PWM,
    /// Control and data interface to SAR ADC
    pub ADC: ADC,
    /// DW_apb_i2c address block List of configuration constants for the Synopsys I...
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
    /// Single-cycle IO block Provides core-local and inter-core hardware for the t...
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
    #[cfg(feature = "critical-section")]
    #[inline]
    pub fn take() -> Option<Self> {
        critical_section::with(|_| {
            if unsafe { DEVICE_PERIPHERALS } {
                return None;
            }
            Some(unsafe { Peripherals::steal() })
        })
    }

    /// Unchecked version of `Peripherals::take`.
    ///
    /// # Safety
    ///
    /// Each of the returned peripherals must be used at most once.
    #[inline]
    pub unsafe fn steal() -> Self {
        unsafe {
            DEVICE_PERIPHERALS = true;
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
}
