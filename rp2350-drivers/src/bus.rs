//! 通用总线抽象 trait。
//!
//! HAL（`rp2350`）不再内置 `I2cBus` / `SpiBus` / `UartBus` 等总线抽象 trait，
//! 需要抽象时由调用者（driver crate 或最终应用）自行定义并实现。
//!
//! 本 crate 的设备驱动（如 [`crate::tmc2209`]）依赖 [`UartBus`]，因此在此
//! 定义该 trait，并为 RP2350 的 UART 类型（`Uart0` / `Uart1` 及其流控变体）
//! 提供实现，保证驱动开箱即用。其他需要总线抽象的调用者可参考本模块
//! 自行定义 trait 并为具体类型实现。

use rp2350::gpio::{
    Uart0CtsPin, Uart0RtsPin, Uart0RxPin, Uart0TxPin, Uart1CtsPin, Uart1RtsPin, Uart1RxPin,
    Uart1TxPin,
};
use rp2350::uart::{Error, Uart0, Uart0WithFlowControl, Uart1, Uart1WithFlowControl};

/// 抽象 UART 总线 trait，供驱动（如 [`crate::tmc2209`]）解耦具体 UART 类型。
///
/// 任何实现本 trait 的总线（HAL UART、模拟总线、测试 mock）都能被驱动复用。
pub trait UartBus {
    /// 错误类型。
    type Error;

    /// 写字节到 UART。
    fn write(&self, data: &[u8]) -> Result<(), Self::Error>;

    /// 阻塞读取，填满整个缓冲区。
    fn read(&self, buffer: &mut [u8]) -> Result<(), Self::Error>;

    /// 非阻塞读取当前可用字节。
    ///
    /// 任一字节携带帧/奇偶/溢出错误时返回 `Err`。
    /// 需要忽略错误标志的原始字节访问请用 [`read_raw`](UartBus::read_raw)。
    fn read_available(&self, buffer: &mut [u8]) -> Result<usize, Self::Error>;

    /// 无错误检查地读取可用字节（用于半双工回显排出等场景）。
    fn read_raw(&self, buffer: &mut [u8]) -> usize;

    /// 阻塞等待所有挂起传输完成。
    fn flush(&self) -> Result<(), Self::Error>;
}

// ============================================================================
// 为普通 UART 类型实现 UartBus
// ============================================================================

macro_rules! impl_uart_bus {
    ($Uart:ident, $TxPin:ident, $RxPin:ident) => {
        impl<TX, RX> UartBus for $Uart<TX, RX>
        where
            TX: $TxPin,
            RX: $RxPin,
        {
            type Error = Error;

            #[inline]
            fn write(&self, data: &[u8]) -> Result<(), Self::Error> {
                self.write(data);
                Ok(())
            }

            #[inline]
            fn read(&self, buffer: &mut [u8]) -> Result<(), Self::Error> {
                self.read(buffer)
            }

            #[inline]
            fn read_available(&self, buffer: &mut [u8]) -> Result<usize, Self::Error> {
                let mut count = 0;

                for slot in buffer.iter_mut() {
                    match self.read_byte_with_error() {
                        Some(Ok(byte)) => {
                            *slot = byte;
                            count += 1;
                        }
                        Some(Err((_, err))) => return Err(err),
                        None => break,
                    }
                }

                Ok(count)
            }

            #[inline]
            fn read_raw(&self, buffer: &mut [u8]) -> usize {
                self.read_raw(buffer)
            }

            #[inline]
            fn flush(&self) -> Result<(), Self::Error> {
                self.flush();
                Ok(())
            }
        }
    };
}

impl_uart_bus!(Uart0, Uart0TxPin, Uart0RxPin);
impl_uart_bus!(Uart1, Uart1TxPin, Uart1RxPin);

// ============================================================================
// 为带硬件流控的 UART 变体实现 UartBus
// ============================================================================

macro_rules! impl_uart_bus_fc {
    ($Uart:ident, $Base:ident, $TxPin:ident, $RxPin:ident, $CtsPin:ident, $RtsPin:ident) => {
        impl<TX, RX, CTS, RTS> UartBus for $Uart<TX, RX, CTS, RTS>
        where
            TX: $TxPin,
            RX: $RxPin,
            CTS: $CtsPin,
            RTS: $RtsPin,
        {
            type Error = Error;

            // 流控变体通过 Deref 委托内层基础 UART 的固有方法。
            // 这里显式调用 `$Base::xxx(&**self, ...)` 以避免与方法名相同的
            // trait 方法（UartBus::write 等）发生解析歧义/递归。
            #[inline]
            fn write(&self, data: &[u8]) -> Result<(), Self::Error> {
                $Base::write(&**self, data);
                Ok(())
            }

            #[inline]
            fn read(&self, buffer: &mut [u8]) -> Result<(), Self::Error> {
                $Base::read(&**self, buffer)
            }

            #[inline]
            fn read_available(&self, buffer: &mut [u8]) -> Result<usize, Self::Error> {
                let mut count = 0;

                for slot in buffer.iter_mut() {
                    match $Base::read_byte_with_error(&**self) {
                        Some(Ok(byte)) => {
                            *slot = byte;
                            count += 1;
                        }
                        Some(Err((_, err))) => return Err(err),
                        None => break,
                    }
                }

                Ok(count)
            }

            #[inline]
            fn read_raw(&self, buffer: &mut [u8]) -> usize {
                $Base::read_raw(&**self, buffer)
            }

            #[inline]
            fn flush(&self) -> Result<(), Self::Error> {
                $Base::flush(&**self);
                Ok(())
            }
        }
    };
}

impl_uart_bus_fc!(
    Uart0WithFlowControl,
    Uart0,
    Uart0TxPin,
    Uart0RxPin,
    Uart0CtsPin,
    Uart0RtsPin
);
impl_uart_bus_fc!(
    Uart1WithFlowControl,
    Uart1,
    Uart1TxPin,
    Uart1RxPin,
    Uart1CtsPin,
    Uart1RtsPin
);
