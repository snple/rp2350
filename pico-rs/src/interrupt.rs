//! RP2350 Interrupt definitions
//!
//! This module defines the interrupt vector table for the RP2350.

#![allow(non_camel_case_types)]

// External interrupt handlers - these are provided by device.x with DefaultHandler
extern "C" {
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
    fn SW0_IRQ();
    fn SW1_IRQ();
    fn SW2_IRQ();
    fn SW3_IRQ();
    fn SW4_IRQ();
    fn SW5_IRQ();
}

/// Union type for interrupt vectors
#[repr(C)]
pub union Vector {
    _handler: unsafe extern "C" fn(),
    _reserved: u32,
}

/// RP2350 interrupt vector table (52 interrupts)
#[cfg(feature = "rt")]
#[doc(hidden)]
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
pub static __INTERRUPTS: [Vector; 52] = [
    Vector {
        _handler: TIMER0_IRQ_0,
    },
    Vector {
        _handler: TIMER0_IRQ_1,
    },
    Vector {
        _handler: TIMER0_IRQ_2,
    },
    Vector {
        _handler: TIMER0_IRQ_3,
    },
    Vector {
        _handler: TIMER1_IRQ_0,
    },
    Vector {
        _handler: TIMER1_IRQ_1,
    },
    Vector {
        _handler: TIMER1_IRQ_2,
    },
    Vector {
        _handler: TIMER1_IRQ_3,
    },
    Vector {
        _handler: PWM_IRQ_WRAP_0,
    },
    Vector {
        _handler: PWM_IRQ_WRAP_1,
    },
    Vector {
        _handler: DMA_IRQ_0,
    },
    Vector {
        _handler: DMA_IRQ_1,
    },
    Vector {
        _handler: DMA_IRQ_2,
    },
    Vector {
        _handler: DMA_IRQ_3,
    },
    Vector {
        _handler: USBCTRL_IRQ,
    },
    Vector {
        _handler: PIO0_IRQ_0,
    },
    Vector {
        _handler: PIO0_IRQ_1,
    },
    Vector {
        _handler: PIO1_IRQ_0,
    },
    Vector {
        _handler: PIO1_IRQ_1,
    },
    Vector {
        _handler: PIO2_IRQ_0,
    },
    Vector {
        _handler: PIO2_IRQ_1,
    },
    Vector {
        _handler: IO_IRQ_BANK0,
    },
    Vector {
        _handler: IO_IRQ_BANK0_NS,
    },
    Vector {
        _handler: IO_IRQ_QSPI,
    },
    Vector {
        _handler: IO_IRQ_QSPI_NS,
    },
    Vector {
        _handler: SIO_IRQ_FIFO,
    },
    Vector {
        _handler: SIO_IRQ_BELL,
    },
    Vector {
        _handler: SIO_IRQ_FIFO_NS,
    },
    Vector {
        _handler: SIO_IRQ_BELL_NS,
    },
    Vector {
        _handler: SIO_IRQ_MTIMECMP,
    },
    Vector {
        _handler: CLOCKS_IRQ,
    },
    Vector { _handler: SPI0_IRQ },
    Vector { _handler: SPI1_IRQ },
    Vector {
        _handler: UART0_IRQ,
    },
    Vector {
        _handler: UART1_IRQ,
    },
    Vector {
        _handler: ADC_IRQ_FIFO,
    },
    Vector { _handler: I2C0_IRQ },
    Vector { _handler: I2C1_IRQ },
    Vector { _handler: OTP_IRQ },
    Vector { _handler: TRNG_IRQ },
    Vector { _reserved: 0 }, // Reserved
    Vector { _reserved: 0 }, // Reserved
    Vector {
        _handler: PLL_SYS_IRQ,
    },
    Vector {
        _handler: PLL_USB_IRQ,
    },
    Vector {
        _handler: POWMAN_IRQ_POW,
    },
    Vector {
        _handler: POWMAN_IRQ_TIMER,
    },
    Vector { _handler: SW0_IRQ },
    Vector { _handler: SW1_IRQ },
    Vector { _handler: SW2_IRQ },
    Vector { _handler: SW3_IRQ },
    Vector { _handler: SW4_IRQ },
    Vector { _handler: SW5_IRQ },
];

/// RP2350 Interrupt enumeration
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Interrupt {
    /// Timer 0 IRQ 0
    TIMER0_IRQ_0 = 0,
    /// Timer 0 IRQ 1
    TIMER0_IRQ_1 = 1,
    /// Timer 0 IRQ 2
    TIMER0_IRQ_2 = 2,
    /// Timer 0 IRQ 3
    TIMER0_IRQ_3 = 3,
    /// Timer 1 IRQ 0
    TIMER1_IRQ_0 = 4,
    /// Timer 1 IRQ 1
    TIMER1_IRQ_1 = 5,
    /// Timer 1 IRQ 2
    TIMER1_IRQ_2 = 6,
    /// Timer 1 IRQ 3
    TIMER1_IRQ_3 = 7,
    /// PWM IRQ Wrap 0
    PWM_IRQ_WRAP_0 = 8,
    /// PWM IRQ Wrap 1
    PWM_IRQ_WRAP_1 = 9,
    /// DMA IRQ 0
    DMA_IRQ_0 = 10,
    /// DMA IRQ 1
    DMA_IRQ_1 = 11,
    /// DMA IRQ 2
    DMA_IRQ_2 = 12,
    /// DMA IRQ 3
    DMA_IRQ_3 = 13,
    /// USB Controller IRQ
    USBCTRL_IRQ = 14,
    /// PIO 0 IRQ 0
    PIO0_IRQ_0 = 15,
    /// PIO 0 IRQ 1
    PIO0_IRQ_1 = 16,
    /// PIO 1 IRQ 0
    PIO1_IRQ_0 = 17,
    /// PIO 1 IRQ 1
    PIO1_IRQ_1 = 18,
    /// PIO 2 IRQ 0
    PIO2_IRQ_0 = 19,
    /// PIO 2 IRQ 1
    PIO2_IRQ_1 = 20,
    /// IO Bank 0 IRQ
    IO_IRQ_BANK0 = 21,
    /// IO Bank 0 NS IRQ
    IO_IRQ_BANK0_NS = 22,
    /// IO QSPI IRQ
    IO_IRQ_QSPI = 23,
    /// IO QSPI NS IRQ
    IO_IRQ_QSPI_NS = 24,
    /// SIO FIFO IRQ
    SIO_IRQ_FIFO = 25,
    /// SIO Bell IRQ
    SIO_IRQ_BELL = 26,
    /// SIO FIFO NS IRQ
    SIO_IRQ_FIFO_NS = 27,
    /// SIO Bell NS IRQ
    SIO_IRQ_BELL_NS = 28,
    /// SIO MTIMECMP IRQ
    SIO_IRQ_MTIMECMP = 29,
    /// Clocks IRQ
    CLOCKS_IRQ = 30,
    /// SPI 0 IRQ
    SPI0_IRQ = 31,
    /// SPI 1 IRQ
    SPI1_IRQ = 32,
    /// UART 0 IRQ
    UART0_IRQ = 33,
    /// UART 1 IRQ
    UART1_IRQ = 34,
    /// ADC FIFO IRQ
    ADC_IRQ_FIFO = 35,
    /// I2C 0 IRQ
    I2C0_IRQ = 36,
    /// I2C 1 IRQ
    I2C1_IRQ = 37,
    /// OTP IRQ
    OTP_IRQ = 38,
    /// TRNG IRQ
    TRNG_IRQ = 39,
    /// PLL SYS IRQ
    PLL_SYS_IRQ = 42,
    /// PLL USB IRQ
    PLL_USB_IRQ = 43,
    /// Power Manager Power IRQ
    POWMAN_IRQ_POW = 44,
    /// Power Manager Timer IRQ
    POWMAN_IRQ_TIMER = 45,
    /// Software IRQ 0
    SW0_IRQ = 46,
    /// Software IRQ 1
    SW1_IRQ = 47,
    /// Software IRQ 2
    SW2_IRQ = 48,
    /// Software IRQ 3
    SW3_IRQ = 49,
    /// Software IRQ 4
    SW4_IRQ = 50,
    /// Software IRQ 5
    SW5_IRQ = 51,
}

unsafe impl cortex_m::interrupt::InterruptNumber for Interrupt {
    #[inline(always)]
    fn number(self) -> u16 {
        self as u16
    }
}
