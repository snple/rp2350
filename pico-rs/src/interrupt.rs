//! Interrupt Management HAL for RP2350
//!
//! This module provides safe and ergonomic interrupt management for the RP2350
//! microcontroller. It wraps the Cortex-M33 NVIC (Nested Vectored Interrupt Controller)
//! and provides additional utilities for RP2350-specific interrupt handling.
//!
//! ## Dual-Core Interrupt Architecture
//!
//! RP2350 has two Cortex-M33 cores, each with its own **independent NVIC**. The NVIC
//! registers reside on the Private Peripheral Bus (PPB), so reads and writes to
//! `0xE000_E100` always access the *current core's* NVIC — there is no way to
//! directly poke another core's NVIC from software.
//!
//! **Key implications:**
//!
//! - `enable()` / `disable()` / `set_priority()` etc. only affect the **calling core**。
//! - 中断在哪个核心触发，取决于哪个核心的 NVIC 启用了该中断。
//! - 如果两个核心启用相同 IRQ，则两个核心都会收到（电平触发时先应答者获胜）。
//! - 要将中断独占路由到某一核心，只需在该核心上启用，另一个核心上不启用即可。
//!
//! ## 推荐的双核中断模式
//!
//! 每个核心在自己的初始化代码中配置自己的 NVIC：
//!
//! ```ignore
//! // Core 0 main — 处理 USB 和 UART
//! #[entry]
//! fn main() -> ! {
//!     unsafe {
//!         interrupt::enable(Interrupt::USBCTRL_IRQ);
//!         interrupt::enable(Interrupt::UART0_IRQ);
//!     }
//!     // launch Core 1...
//! }
//!
//! // Core 1 entry — 处理 DMA 和 PIO
//! fn core1_main() -> ! {
//!     unsafe {
//!         interrupt::enable(Interrupt::DMA_IRQ_0);
//!         interrupt::enable(Interrupt::PIO0_IRQ_0);
//!     }
//!     loop { pico_rs_rt::asm::wfe(); }
//! }
//! ```
//!
//! ## 跨核触发中断
//!
//! 无需远程操作另一个核心的 NVIC。要触发另一个核心的中断，使用硬件机制：
//!
//! - **Doorbell**（`SIO_IRQ_BELL`）：8 bit 标志位，写入即触发对方核心中断
//! - **FIFO**（`SIO_IRQ_FIFO`）：发送数据同时触发对方核心中断
//! - **Spare IRQ**：6 个软件中断，用 `pend()` 触发当前核心
//!
//! ```ignore
//! use pico_rs::multicore::Doorbell;
//!
//! // Core 0: 通知 Core 1 有工作要处理
//! Doorbell::new().signal();
//!
//! // Core 1: 在 SIO_IRQ_BELL handler 中处理
//! #[unsafe(no_mangle)]
//! fn SIO_IRQ_BELL() {
//!     let db = Doorbell::new();
//!     db.clear_local(1);
//!     // 处理来自 Core 0 的请求...
//! }
//! ```
//!
//! ## 优先级系统（RP2350）
//!
//! RP2350 的 Cortex-M33 在每个 8-bit IPR 寄存器字段的**高 4 位**实现优先级，
//! 共提供 **16 个有效级别**。数字越小优先级越高。
//!
//! | 级别 | 8-bit 写入值 | 十六进制 |
//! |------|-------------|---------|
//! |  0   |   0         | 0x00    | ← 最高优先级（复位后所有中断的默认值）
//! |  1   |  16         | 0x10    |
//! |  2   |  32         | 0x20    |
//! |  3   |  48         | 0x30    |
//! |  4   |  64         | 0x40    |
//! |  5   |  80         | 0x50    |
//! |  6   |  96         | 0x60    |
//! |  7   | 112         | 0x70    |
//! |  8   | 128         | 0x80    | ← 常用中间值
//! |  9   | 144         | 0x90    |
//! | 10   | 160         | 0xA0    |
//! | 11   | 176         | 0xB0    |
//! | 12   | 192         | 0xC0    |
//! | 13   | 208         | 0xD0    |
//! | 14   | 224         | 0xE0    |
//! | 15   | 240         | 0xF0    | ← 最低优先级
//!
//! **重要细节：**
//! - 写入不对齐的值（如 100）硬件会自动截断为 `value & 0xF0`（即 96）。
//!   建议始终使用 16 的倍数，以避免歧义。
//! - 复位后所有中断优先级寄存器为 **0x00**（最高优先级）。
//! - **`set_priority()` 必须在 `enable()` 之前调用**，否则中断的第一次触发
//!   仍以优先级 0（最高）运行。
//!
//! ## 抢占行为
//!
//! Cortex-M33 默认 PRIGROUP=0（AIRCR 寄存器），表示所有 4 个优先级位都用于
//! **抢占优先级**，无子优先级位。效果：
//!
//! - 优先级数值更小的中断可以**抢占**正在运行的更大数值中断。
//! - 同等优先级的中断**不能相互抢占**（等待当前 ISR 返回后才处理）。
//!
//! ```ignore
//! // 典型优先级分配示例：
//! // USBCTRL_IRQ  → 0   (最高，确保 USB 总线及时响应)
//! // DMA_IRQ_0    → 32  (高，数据流)
//! // TIMER0_IRQ_0 → 64  (中)
//! // SPARE_IRQ_0  → 128 (低，应用层事件处理)
//! // SysTick 优先级通过 SCB 的 SHPR3 寄存器单独配置，范围相同
//!
//! interrupt::set_priority(Interrupt::USBCTRL_IRQ, 0);
//! interrupt::set_priority(Interrupt::SPARE_IRQ_0, 128);
//! // 设置优先级之后再 enable，保证第一次触发就以正确优先级运行
//! unsafe {
//!     interrupt::enable(Interrupt::USBCTRL_IRQ);
//!     interrupt::enable(Interrupt::SPARE_IRQ_0);
//! }
//! ```
//!
//! ## 使用 `#[bind]` 宏绑定中断
//!
//! `bind` 宏将一个函数绑定到指定的中断向量，生成 `#[no_mangle]` 的 ISR 入口：
//!
//! ```ignore
//! use pico_rs::bind;
//!
//! #[bind(SPARE_IRQ_0)]
//! fn event_handler() {
//!     // 处理应用层事件
//! }
//! ```
//!
//! **优先级必须在 `enable()` 之前单独设置**（不在宏内处理）：
//!
//! ```ignore
//! interrupt::set_priority(Interrupt::SPARE_IRQ_0, 128); // 先设优先级
//! unsafe { interrupt::enable(Interrupt::SPARE_IRQ_0); } // 再 enable
//! ```
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
//! // Enable a specific interrupt (on the current core)
//! // 必须先设置优先级，再 enable，否则第一次触发以优先级 0 运行
//! interrupt::set_priority(IrqNum::TIMER0_IRQ_0, 64); // 优先级级别 4
//! unsafe { interrupt::enable(IrqNum::TIMER0_IRQ_0); }
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

use core::cell::UnsafeCell;
use core::sync::atomic::{AtomicU8, Ordering};
use pico_rs_rt::peripheral::NVIC;

// Re-export the interrupt enum from PAC for convenience
pub use crate::pac::Interrupt;

// ============================================================================
// NVIC Operations
// ============================================================================

/// Enable an interrupt in the **current core's** NVIC
///
/// This allows the interrupt to be triggered on the calling core only.
/// Note that the peripheral must also have its interrupt enabled
/// (e.g., timer alarm interrupt).
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
/// // Enable timer0 alarm 0 interrupt on whichever core calls this
/// unsafe { interrupt::enable(IrqNum::TIMER0_IRQ_0); }
/// ```
#[inline]
pub unsafe fn enable(interrupt: Interrupt) {
    unsafe {
        NVIC::enable(interrupt as u16);
    }
}

/// Disable an interrupt in the **current core's** NVIC
///
/// This prevents the interrupt from triggering on the calling core.
/// The interrupt remains pending if it was triggered while disabled.
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
    NVIC::disable(interrupt as u16);
}

/// Check if an interrupt is enabled in the **current core's** NVIC
#[inline]
pub fn is_enabled(interrupt: Interrupt) -> bool {
    NVIC::is_enabled(interrupt as u16)
}

/// Check if an interrupt is pending in the **current core's** NVIC
#[inline]
pub fn is_pending(interrupt: Interrupt) -> bool {
    NVIC::is_pending(interrupt as u16)
}

/// Set an interrupt as pending in the **current core's** NVIC
///
/// This will cause the interrupt handler to run when the interrupt
/// is enabled and has higher priority than the current execution.
#[inline]
pub fn pend(interrupt: Interrupt) {
    NVIC::pend(interrupt as u16);
}

/// Clear a pending interrupt in the **current core's** NVIC
#[inline]
pub fn unpend(interrupt: Interrupt) {
    NVIC::unpend(interrupt as u16);
}

/// Set the priority of an interrupt in the **current core's** NVIC
///
/// RP2350 Cortex-M33 实现了 **4 位优先级**（高 4 位有效），共 16 个级别。
/// 数字越小优先级越高。写入值会被截断为 `value & 0xF0`，**请使用 16 的倍数**。
///
/// | 8-bit 值 | 级别 | 说明 |
/// |---------|------|------|
/// |   0     |  0   | 最高优先级（复位默认值）|
/// |  16     |  1   | |
/// |  32     |  2   | |
/// |  48     |  3   | |
/// |  64     |  4   | |
/// |  80     |  5   | |
/// |  96     |  6   | |
/// | 112     |  7   | |
/// | 128     |  8   | 常用中间值 |
/// | 144..224|9..14 | |
/// | 240     | 15   | 最低优先级 |
///
/// **必须在 `enable()` 之前调用**，否则中断的第一次触发以复位默认值（0）运行。
///
/// # Arguments
/// * `interrupt` - 要配置的中断
/// * `priority`  - 优先级值，使用 16 的倍数（0 = 最高，240 = 最低）
///
/// # Safety
/// This function accesses the NVIC registers of the current core. The caller must ensure
/// that setting the priority does not cause any timing or stability issues. Additionally,
/// it is recommended to call this function before `enable()` to ensure the interrupt is
/// triggered with the correct priority from its first occurrence.
///
/// # Example
/// ```ignore
/// use pico_rs::interrupt::{self, Interrupt};
///
/// // 先设优先级，再 enable
/// interrupt::set_priority(Interrupt::SPARE_IRQ_0, 128); // 级别 8
/// unsafe { interrupt::enable(Interrupt::SPARE_IRQ_0); }
/// ```
#[inline]
pub unsafe fn set_priority(interrupt: Interrupt, priority: u8) {
    unsafe {
        NVIC::set_priority(interrupt as u16, priority);
    }
}

/// Get the priority of an interrupt from the **current core's** NVIC
#[inline]
pub fn get_priority(interrupt: Interrupt) -> u8 {
    NVIC::get_priority(interrupt as u16)
}

// ============================================================================
// Additional Utilities
// ============================================================================

/// Check if we are currently in an interrupt handler
#[inline]
pub fn in_interrupt() -> bool {
    // Read the IPSR (Interrupt Program Status Register)
    // If non-zero, we're in an exception/interrupt handler
    let ipsr: u32;
    #[allow(asm_sub_register)]
    unsafe {
        core::arch::asm!("mrs {0}, IPSR", out(reg) ipsr, options(nomem, nostack, preserves_flags));
    }
    (ipsr & 0x1FF) != 0
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

/// USB interrupt numbers
pub mod usb {
    use super::Interrupt;

    /// USB controller interrupt
    pub const USBCTRL_IRQ: Interrupt = Interrupt::USBCTRL_IRQ;

    /// All USB interrupts
    pub const ALL: [Interrupt; 1] = [USBCTRL_IRQ];
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
    let was_enabled = pico_rs_rt::interrupt::is_enabled();
    pico_rs_rt::interrupt::disable();
    InterruptGuard { was_enabled }
}

/// Guard that re-enables interrupts when dropped
pub struct InterruptGuard {
    was_enabled: bool,
}

impl Drop for InterruptGuard {
    fn drop(&mut self) {
        if self.was_enabled {
            unsafe { pico_rs_rt::interrupt::enable() };
        }
    }
}

// ============================================================================
// Processor ID Detection
// ============================================================================

/// 获取当前执行核心的 ID
///
/// 在中断处理程序中可用来确定当前代码在哪个核心上运行。
#[inline]
pub fn current_core() -> crate::multicore::CoreId {
    crate::multicore::core_id()
}

/// Token proving code is running in a critical section (interrupts disabled)
#[derive(Clone, Copy)]
pub struct CriticalSection {
    _marker: core::marker::PhantomData<*const ()>,
}

impl CriticalSection {
    /// Create a new critical section token
    ///
    /// # Safety
    ///
    /// Must only be called when interrupts are actually disabled.
    #[inline(always)]
    pub(crate) unsafe fn new() -> Self {
        Self {
            _marker: core::marker::PhantomData,
        }
    }
}

/// A mutual exclusion primitive for protecting shared data in interrupt handlers
///
/// This is similar to `std::sync::Mutex` but designed for use in interrupt
/// handlers on single-core systems. Data can only be accessed within a
/// critical section (with interrupts disabled).
///
/// # Example
///
/// ```ignore
/// use core::cell::RefCell;
/// use pico_rs_rt::interrupt::{self, Mutex};
///
/// static COUNTER: Mutex<RefCell<u32>> = Mutex::new(RefCell::new(0));
///
/// fn main() {
///     interrupt::free(|cs| {
///         *COUNTER.borrow(cs).borrow_mut() += 1;
///     });
/// }
/// ```
pub struct Mutex<T> {
    data: UnsafeCell<T>,
}

// Safety: We only allow access through a CriticalSection token,
// which proves interrupts are disabled. On a single-core system,
// this prevents any data races.
unsafe impl<T> Sync for Mutex<T> where T: Send {}

impl<T> Mutex<T> {
    /// Create a new Mutex containing the given value
    #[inline(always)]
    pub const fn new(value: T) -> Self {
        Self {
            data: UnsafeCell::new(value),
        }
    }

    /// Borrow the data inside the mutex
    ///
    /// This requires a `CriticalSection` token proving that interrupts
    /// are disabled.
    #[inline(always)]
    pub fn borrow(&self, _cs: CriticalSection) -> &T {
        // Safety: We have a CriticalSection token, proving interrupts are
        // disabled. On a single-core system, this means no other code can
        // be accessing this data.
        unsafe { &*self.data.get() }
    }

    /// Get a raw pointer to the contained data
    ///
    /// This is useful when you need to pass the data to C code or
    /// perform other low-level operations.
    #[inline(always)]
    pub fn as_ptr(&self) -> *mut T {
        self.data.get()
    }
}

/// Execute a closure with interrupts disabled
///
/// This is a critical section - the closure runs atomically with respect
/// to all configurable interrupts. After the closure returns, the previous
/// interrupt state is restored.
///
/// Implemented directly via the same spinlock-31 and per-core nesting counter
/// used by `critical_section::Impl`, so nesting between `interrupt::free` and
/// `critical_section::with` (e.g. from `defmt`) is fully coherent and
/// deadlock-free.
///
/// # Example
///
/// ```ignore
/// use pico_rs_rt::interrupt;
///
/// static mut COUNTER: u32 = 0;
///
/// let value = interrupt::free(|_cs| {
///     // Safe to access COUNTER - interrupts are disabled
///     unsafe {
///         COUNTER += 1;
///         COUNTER
///     }
/// });
/// ```
#[inline]
pub fn free<F, R>(f: F) -> R
where
    F: FnOnce(CriticalSection) -> R,
{
    // 直接调用本模块的底层实现，与 critical_section::Impl（当启用时）
    // 共享同一套 CS_NESTING 计数器和 spinlock-31，两条路径完全一致，不会死锁。
    let primask = cs_acquire();
    let result = f(unsafe { CriticalSection::new() });
    unsafe { cs_release(primask) };
    result
}

// ============================================================================
// Critical Section Core — spinlock-31 + per-core nesting counter
//
// This lives here so that `free()` can call the same logic directly,
// and `critical_section_impl` (when enabled) is just a thin wrapper that
// registers the same acquire/release pair as the `critical_section::Impl`.
// ============================================================================

/// 每个核的临界区嵌套计数器（索引为核 ID 0/1）
pub(crate) static CS_NESTING: [AtomicU8; 2] = [AtomicU8::new(0), AtomicU8::new(0)];

#[inline]
fn cs_spinlock_acquire() {
    // 读 SIO spinlock-31：返回非零表示获取成功（硬件保证原子性）
    while crate::sio::regs().spinlock31.spinlock31() == 0 {
        core::hint::spin_loop();
    }
}

#[inline]
fn cs_spinlock_release() {
    // 写任意值释放 spinlock-31
    crate::sio::regs().spinlock31.write_spinlock31(1);
}

/// 进入临界区：禁用本核中断 + 获取 spinlock-31（支持嵌套）
///
/// 返回调用前的 PRIMASK 原始值，供 [`cs_release`] 恢复。
#[inline]
pub(crate) fn cs_acquire() -> u32 {
    // 1. 保存当前中断状态
    let primask = pico_rs_rt::register::primask::read_raw();
    // 2. 禁用本核中断
    pico_rs_rt::interrupt::disable();
    // 3. 更新嵌套计数器，只有从 0 变为 1 时才真正获取 spinlock-31
    let core = crate::sio::regs().cpuid.cpuid() as usize;
    let prev = CS_NESTING[core].fetch_add(1, Ordering::Acquire);
    if prev == 0 {
        cs_spinlock_acquire();
    }
    primask
}

/// 退出临界区：释放 spinlock-31 + 恢复中断状态（支持嵌套）
///
/// # Safety
/// `primask` 必须是对应 [`cs_acquire`] 调用返回的值。
#[inline]
pub(crate) unsafe fn cs_release(primask: u32) {
    // 1. 更新嵌套计数器，只有从 1 变为 0 时才真正释放 spinlock-31
    let core = crate::sio::regs().cpuid.cpuid() as usize;
    let prev = CS_NESTING[core].fetch_sub(1, Ordering::Release);
    if prev == 1 {
        cs_spinlock_release();
    }
    // 2. 恢复之前的中断状态
    unsafe { pico_rs_rt::register::primask::write_raw(primask) };
}
