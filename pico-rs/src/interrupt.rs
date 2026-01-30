//! Interrupt Management HAL for RP2350
//!
//! This module provides safe and ergonomic interrupt management for the RP2350
//! microcontroller. It wraps the Cortex-M NVIC (Nested Vectored Interrupt Controller)
//! and provides additional utilities for RP2350-specific interrupt handling.
//!
//! ## Features
//!
//! - NVIC control (enable, disable, pend, clear, priority)
//! - Critical sections for interrupt-safe code
//! - Re-export of interrupt numbers from PAC
//!
//! ## Example
//!
//! ```ignore
//! use pico_rs::interrupt::{self, Interrupt};
//! use pico_rs::pac::Interrupt as IrqNum;
//!
//! // Enable a specific interrupt
//! interrupt::enable(IrqNum::TIMER0_IRQ_0);
//!
//! // Set interrupt priority (0 = highest, 255 = lowest)
//! interrupt::set_priority(IrqNum::TIMER0_IRQ_0, 128);
//!
//! // Run code in a critical section (all interrupts disabled)
//! interrupt::free(|_| {
//!     // Interrupt-safe code here
//! });
//! ```
//!
//! ## Interrupt Handler Example
//!
//! ```ignore
//!
//! #[unsafe(no_mangle)]
//! fn TIMER0_IRQ_0() {
//!     // Handle timer0 alarm 0 interrupt
//!     // Remember to clear the interrupt flag!
//! }
//! ```

use cortex_m::peripheral::NVIC;

// Re-export the interrupt enum from PAC for convenience
pub use crate::pac::Interrupt;

/// Token type that proves we are in a critical section
#[derive(Clone, Copy)]
pub struct CriticalSection {
    _private: (),
}

impl CriticalSection {
    /// Create a new critical section token
    ///
    /// # Safety
    /// This should only be called when interrupts are actually disabled
    #[inline]
    pub(crate) unsafe fn new() -> Self {
        Self { _private: () }
    }
}

// ============================================================================
// NVIC Operations
// ============================================================================

/// Enable an interrupt in the NVIC
///
/// This allows the interrupt to be triggered. Note that the peripheral
/// must also have its interrupt enabled (e.g., timer alarm interrupt).
///
/// # Safety
/// Enabling an interrupt can cause the interrupt handler to run at any time.
/// Make sure the handler is properly set up before enabling.
///
/// # Example
/// ```ignore
/// use pico_rs::interrupt::{self, Interrupt};
/// use pico_rs::pac::Interrupt as IrqNum;
///
/// // Enable timer0 alarm 0 interrupt
/// unsafe { interrupt::enable(IrqNum::TIMER0_IRQ_0); }
/// ```
#[inline]
pub unsafe fn enable(interrupt: Interrupt) {
    // Safety: We're just enabling an interrupt in the NVIC
    unsafe {
        NVIC::unmask(interrupt);
    }
}

/// Disable an interrupt in the NVIC
///
/// This prevents the interrupt from triggering. The interrupt remains
/// pending if it was triggered while disabled.
///
/// # Example
/// ```ignore
/// use pico_rs::interrupt::{self, Interrupt};
/// use pico_rs::pac::Interrupt as IrqNum;
///
/// interrupt::disable(IrqNum::TIMER0_IRQ_0);
/// ```
#[inline]
pub fn disable(interrupt: Interrupt) {
    NVIC::mask(interrupt);
}

/// Check if an interrupt is enabled in the NVIC
#[inline]
pub fn is_enabled(interrupt: Interrupt) -> bool {
    NVIC::is_enabled(interrupt)
}

/// Check if an interrupt is pending
#[inline]
pub fn is_pending(interrupt: Interrupt) -> bool {
    NVIC::is_pending(interrupt)
}

/// Set an interrupt as pending
///
/// This will cause the interrupt handler to run when the interrupt
/// is enabled and has higher priority than the current execution.
#[inline]
pub fn pend(interrupt: Interrupt) {
    NVIC::pend(interrupt);
}

/// Clear a pending interrupt
#[inline]
pub fn unpend(interrupt: Interrupt) {
    NVIC::unpend(interrupt);
}

/// Set the priority of an interrupt
///
/// Lower numbers mean higher priority. On Cortex-M33 (RP2350), priorities
/// are 0-255, but only the top 4 bits are implemented (0, 16, 32, ..., 240).
///
/// # Arguments
/// * `interrupt` - The interrupt to configure
/// * `priority` - Priority level (0 = highest, 255 = lowest)
///
/// # Example
/// ```ignore
/// use pico_rs::interrupt::{self, Interrupt};
/// use pico_rs::pac::Interrupt as IrqNum;
///
/// // Set high priority for timer interrupt
/// interrupt::set_priority(IrqNum::TIMER0_IRQ_0, 0);
/// ```
#[inline]
pub fn set_priority(interrupt: Interrupt, priority: u8) {
    // Safety: We're just setting the priority in the NVIC
    unsafe {
        let mut nvic = cortex_m::Peripherals::steal().NVIC;
        nvic.set_priority(interrupt, priority);
    }
}

/// Get the priority of an interrupt
#[inline]
pub fn get_priority(interrupt: Interrupt) -> u8 {
    // Safety: We're just reading the priority
    NVIC::get_priority(interrupt)
}

// ============================================================================
// Critical Sections
// ============================================================================

/// Execute a closure in a critical section (interrupts disabled)
///
/// All interrupts are disabled while the closure executes. This is useful
/// for protecting shared data from race conditions.
///
/// # Example
/// ```ignore
/// use pico_rs::interrupt;
///
/// static mut COUNTER: u32 = 0;
///
/// interrupt::free(|_cs| {
///     // Safe to access COUNTER here - no interrupts can preempt
///     unsafe { COUNTER += 1; }
/// });
/// ```
#[inline]
pub fn free<F, R>(f: F) -> R
where
    F: FnOnce(CriticalSection) -> R,
{
    // Disable interrupts
    let restore_state = crate::cortex_m33::register::primask::read_raw();
    cortex_m::interrupt::disable();

    // Create critical section token
    let cs = unsafe { CriticalSection::new() };

    // Execute the closure
    let result = f(cs);

    // Restore interrupt state
    unsafe { crate::cortex_m33::register::primask::write_raw(restore_state) };

    result
}

/// Check if we are currently in an interrupt handler
#[inline]
pub fn in_interrupt() -> bool {
    // Read the IPSR (Interrupt Program Status Register)
    // If non-zero, we're in an exception/interrupt handler
    let ipsr: u32;
    unsafe {
        #![allow(asm_sub_register)]
        core::arch::asm!("mrs {0}, IPSR", out(reg) ipsr, options(nomem, nostack, preserves_flags));
    }
    (ipsr & 0x1FF) != 0
}

/// Check if interrupts are currently enabled
#[inline]
pub fn are_enabled() -> bool {
    cortex_m::register::primask::read().is_active()
}

// ============================================================================
// Interrupt Groups
// ============================================================================

/// Timer0 interrupt numbers
pub mod timer0 {
    use super::Interrupt;

    /// Alarm 0 interrupt
    pub const TIMER0_IRQ_0: Interrupt = Interrupt::TIMER0_IRQ_0;
    /// Alarm 1 interrupt
    pub const TIMER0_IRQ_1: Interrupt = Interrupt::TIMER0_IRQ_1;
    /// Alarm 2 interrupt
    pub const TIMER0_IRQ_2: Interrupt = Interrupt::TIMER0_IRQ_2;
    /// Alarm 3 interrupt
    pub const TIMER0_IRQ_3: Interrupt = Interrupt::TIMER0_IRQ_3;

    /// All timer0 interrupts
    pub const ALL: [Interrupt; 4] = [TIMER0_IRQ_0, TIMER0_IRQ_1, TIMER0_IRQ_2, TIMER0_IRQ_3];
}

/// Timer1 interrupt numbers
pub mod timer1 {
    use super::Interrupt;

    /// Alarm 0 interrupt
    pub const TIMER0_IRQ_0: Interrupt = Interrupt::TIMER1_IRQ_0;
    /// Alarm 1 interrupt
    pub const TIMER0_IRQ_1: Interrupt = Interrupt::TIMER1_IRQ_1;
    /// Alarm 2 interrupt
    pub const TIMER0_IRQ_2: Interrupt = Interrupt::TIMER1_IRQ_2;
    /// Alarm 3 interrupt
    pub const TIMER0_IRQ_3: Interrupt = Interrupt::TIMER1_IRQ_3;

    /// All timer1 interrupts
    pub const ALL: [Interrupt; 4] = [TIMER0_IRQ_0, TIMER0_IRQ_1, TIMER0_IRQ_2, TIMER0_IRQ_3];
}

/// PWM interrupt numbers
pub mod pwm {
    use super::Interrupt;

    /// PWM wrap interrupt 0 (slices 0-7)
    pub const PWM_IRQ_WRAP_0: Interrupt = Interrupt::PWM_IRQ_WRAP_0;
    /// PWM wrap interrupt 1 (slices 8-11)
    pub const PWM_IRQ_WRAP_1: Interrupt = Interrupt::PWM_IRQ_WRAP_1;

    /// All PWM interrupts
    pub const ALL: [Interrupt; 2] = [PWM_IRQ_WRAP_0, PWM_IRQ_WRAP_1];
}

/// DMA interrupt numbers
pub mod dma {
    use super::Interrupt;

    /// DMA interrupt 0
    pub const DMA_IRQ_0: Interrupt = Interrupt::DMA_IRQ_0;
    /// DMA interrupt 1
    pub const DMA_IRQ_1: Interrupt = Interrupt::DMA_IRQ_1;
    /// DMA interrupt 2
    pub const DMA_IRQ_2: Interrupt = Interrupt::DMA_IRQ_2;
    /// DMA interrupt 3
    pub const DMA_IRQ_3: Interrupt = Interrupt::DMA_IRQ_3;

    /// All DMA interrupts
    pub const ALL: [Interrupt; 4] = [DMA_IRQ_0, DMA_IRQ_1, DMA_IRQ_2, DMA_IRQ_3];
}

/// PIO interrupt numbers
pub mod pio {
    use super::Interrupt;

    /// PIO0 interrupt 0
    pub const PIO0_IRQ_0: Interrupt = Interrupt::PIO0_IRQ_0;
    /// PIO0 interrupt 1
    pub const PIO0_IRQ_1: Interrupt = Interrupt::PIO0_IRQ_1;
    /// PIO1 interrupt 0
    pub const PIO1_IRQ_0: Interrupt = Interrupt::PIO1_IRQ_0;
    /// PIO1 interrupt 1
    pub const PIO1_IRQ_1: Interrupt = Interrupt::PIO1_IRQ_1;
    /// PIO2 interrupt 0
    pub const PIO2_IRQ_0: Interrupt = Interrupt::PIO2_IRQ_0;
    /// PIO2 interrupt 1
    pub const PIO2_IRQ_1: Interrupt = Interrupt::PIO2_IRQ_1;

    /// All PIO interrupts
    pub const ALL: [Interrupt; 6] = [
        PIO0_IRQ_0, PIO0_IRQ_1, PIO1_IRQ_0, PIO1_IRQ_1, PIO2_IRQ_0, PIO2_IRQ_1,
    ];
}

/// GPIO interrupt numbers
pub mod gpio {
    use super::Interrupt;

    /// IO Bank0 interrupt (secure)
    pub const IO_IRQ_BANK0: Interrupt = Interrupt::IO_IRQ_BANK0;
    /// IO Bank0 interrupt (non-secure)
    pub const IO_IRQ_BANK0_NS: Interrupt = Interrupt::IO_IRQ_BANK0_NS;
    /// IO QSPI interrupt (secure)
    pub const IO_IRQ_QSPI: Interrupt = Interrupt::IO_IRQ_QSPI;
    /// IO QSPI interrupt (non-secure)
    pub const IO_IRQ_QSPI_NS: Interrupt = Interrupt::IO_IRQ_QSPI_NS;

    /// All GPIO interrupts
    pub const ALL: [Interrupt; 4] = [IO_IRQ_BANK0, IO_IRQ_BANK0_NS, IO_IRQ_QSPI, IO_IRQ_QSPI_NS];
}

/// SIO (inter-core) interrupt numbers
pub mod sio {
    use super::Interrupt;

    /// FIFO interrupt (secure)
    pub const FIFO: Interrupt = Interrupt::SIO_IRQ_FIFO;
    /// Bell interrupt (secure)
    pub const BELL: Interrupt = Interrupt::SIO_IRQ_BELL;
    /// FIFO interrupt (non-secure)
    pub const FIFO_NS: Interrupt = Interrupt::SIO_IRQ_FIFO_NS;
    /// Bell interrupt (non-secure)
    pub const BELL_NS: Interrupt = Interrupt::SIO_IRQ_BELL_NS;
    /// Machine timer compare interrupt
    pub const MTIMECMP: Interrupt = Interrupt::SIO_IRQ_MTIMECMP;

    /// All SIO interrupts
    pub const ALL: [Interrupt; 5] = [FIFO, BELL, FIFO_NS, BELL_NS, MTIMECMP];
}

/// Clock interrupt numbers
pub mod clocks {
    use super::Interrupt;

    /// Clock interrupt
    pub const CLOCKS_IRQ: Interrupt = Interrupt::CLOCKS_IRQ;

    /// All clock interrupts
    pub const ALL: [Interrupt; 1] = [CLOCKS_IRQ];
}

/// SPI interrupt numbers
pub mod spi {
    use super::Interrupt;

    /// SPI0 interrupt
    pub const SPI0_IRQ: Interrupt = Interrupt::SPI0_IRQ;
    /// SPI1 interrupt
    pub const SPI1_IRQ: Interrupt = Interrupt::SPI1_IRQ;

    /// All SPI interrupts
    pub const ALL: [Interrupt; 2] = [SPI0_IRQ, SPI1_IRQ];
}

/// UART interrupt numbers
pub mod uart {
    use super::Interrupt;

    /// UART0 interrupt
    pub const UART0_IRQ: Interrupt = Interrupt::UART0_IRQ;
    /// UART1 interrupt
    pub const UART1_IRQ: Interrupt = Interrupt::UART1_IRQ;

    /// All UART interrupts
    pub const ALL: [Interrupt; 2] = [UART0_IRQ, UART1_IRQ];
}

/// ADC interrupt numbers
pub mod adc {
    use super::Interrupt;

    /// ADC interrupt
    pub const ADC_IRQ_FIFO: Interrupt = Interrupt::ADC_IRQ_FIFO;

    /// All ADC interrupts
    pub const ALL: [Interrupt; 1] = [ADC_IRQ_FIFO];
}

/// I2C interrupt numbers
pub mod i2c {
    use super::Interrupt;

    /// I2C0 interrupt
    pub const I2C0_IRQ: Interrupt = Interrupt::I2C0_IRQ;
    /// I2C1 interrupt
    pub const I2C1_IRQ: Interrupt = Interrupt::I2C1_IRQ;

    /// All I2C interrupts
    pub const ALL: [Interrupt; 2] = [I2C0_IRQ, I2C1_IRQ];
}

/// OTP interrupt numbers
pub mod otp {
    use super::Interrupt;

    /// OTP interrupt
    pub const OTP_IRQ: Interrupt = Interrupt::OTP_IRQ;

    /// All OTP interrupts
    pub const ALL: [Interrupt; 1] = [OTP_IRQ];
}

/// TRNG interrupt numbers
pub mod trng {
    use super::Interrupt;

    /// TRNG interrupt
    pub const TRNG_IRQ: Interrupt = Interrupt::TRNG_IRQ;

    /// All TRNG interrupts
    pub const ALL: [Interrupt; 1] = [TRNG_IRQ];
}

/// PLL interrupt numbers
pub mod pll {
    use super::Interrupt;

    /// PLL_SYS interrupt
    pub const PLL_SYS_IRQ: Interrupt = Interrupt::PLL_SYS_IRQ;

    /// PLL_USB interrupt
    pub const PLL_USB_IRQ: Interrupt = Interrupt::PLL_USB_IRQ;

    /// All PLL interrupts
    pub const ALL: [Interrupt; 2] = [PLL_SYS_IRQ, PLL_USB_IRQ];
}

/// Power management interrupt numbers
pub mod powman {
    use super::Interrupt;

    /// Power management power interrupt
    pub const POWMAN_IRQ_POW: Interrupt = Interrupt::POWMAN_IRQ_POW;

    /// Power management timer interrupt
    pub const POWMAN_IRQ_TIMER: Interrupt = Interrupt::POWMAN_IRQ_TIMER;

    /// All power management interrupts
    pub const ALL: [Interrupt; 1] = [POWMAN_IRQ_POW];
}

/// Spare/User interrupt numbers (for software-triggered tasks)
///
/// RP2350 provides 6 spare interrupts that can be used as software interrupts.
/// These are ideal for implementing deferred task execution.
pub mod spare {
    use super::*;

    /// Number of available spare interrupts
    pub const COUNT: usize = 6;

    /// Spare SPARE_IRQ 0
    pub const SPARE_IRQ_0: Interrupt = Interrupt::SPARE_IRQ_0;
    /// Spare SPARE_IRQ 1
    pub const SPARE_IRQ_1: Interrupt = Interrupt::SPARE_IRQ_1;
    /// Spare SPARE_IRQ 2
    pub const SPARE_IRQ_2: Interrupt = Interrupt::SPARE_IRQ_2;
    /// Spare SPARE_IRQ 3
    pub const SPARE_IRQ_3: Interrupt = Interrupt::SPARE_IRQ_3;
    /// Spare SPARE_IRQ 4
    pub const SPARE_IRQ_4: Interrupt = Interrupt::SPARE_IRQ_4;
    /// Spare SPARE_IRQ 5
    pub const SPARE_IRQ_5: Interrupt = Interrupt::SPARE_IRQ_5;

    /// All spare interrupts
    pub const ALL: [Interrupt; COUNT] = [
        SPARE_IRQ_0,
        SPARE_IRQ_1,
        SPARE_IRQ_2,
        SPARE_IRQ_3,
        SPARE_IRQ_4,
        SPARE_IRQ_5,
    ];
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Disable all interrupts and return a guard that re-enables them on drop
///
/// This is useful for RAII-style critical sections.
///
/// # Example
/// ```ignore
/// use pico_rs::interrupt;
///
/// {
///     let _guard = interrupt::disable_all();
///     // All interrupts disabled here
///     // ...
/// } // Interrupts automatically restored here
/// ```
pub fn disable_all() -> InterruptGuard {
    let was_enabled = are_enabled();
    cortex_m::interrupt::disable();
    InterruptGuard { was_enabled }
}

/// Guard that re-enables interrupts when dropped
pub struct InterruptGuard {
    was_enabled: bool,
}

impl Drop for InterruptGuard {
    fn drop(&mut self) {
        if self.was_enabled {
            unsafe { cortex_m::interrupt::enable() };
        }
    }
}

// ============================================================================
// Processor ID Detection
// ============================================================================

/// Get the current processor ID (0 or 1)
///
/// This is useful for determining which core is executing the current code,
/// especially in interrupt handlers.
#[inline]
pub fn current_core() -> u32 {
    // Read CPUID from SIO
    // Safety: Reading CPUID is always safe
    unsafe {
        let sio = &*crate::pac::SIO::PTR;
        sio.cpuid.cpuid()
    }
}
