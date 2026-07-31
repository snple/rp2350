//! UART1
//!
//! This peripheral supports atomic SET/CLR/XOR register aliases.

#![allow(dead_code)]

pub const BASE: u32 = 0x40078000;

/// UARTDR Register
///
/// Data Register, UARTDR
///
/// Address offset: 0x0000
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **OE** `[11]` (RO)
///
///   Overrun error. This bit is set to 1 if data is received and the receive FIFO is already full. This is cleared to 0 once there is an empty space in the FIFO and a new character can be written to it.
///
/// - **BE** `[10]` (RO)
///
///   Break error. This bit is set to 1 if a break condition was detected, indicating that the received data input was held LOW for longer than a full-word transmission time (defined as start, data, parity and stop bits). In FIFO mode, this error is associated with the character at the top of the FIFO. When a break occurs, only one 0 character is loaded into the FIFO. The next character is only enabled after the receive data input goes to a 1 (marking state), and the next valid start bit is received.
///
/// - **PE** `[9]` (RO)
///
///   Parity error. When set to 1, it indicates that the parity of the received data character does not match the parity that the EPS and SPS bits in the Line Control Register, UARTLCR_H. In FIFO mode, this error is associated with the character at the top of the FIFO.
///
/// - **FE** `[8]` (RO)
///
///   Framing error. When set to 1, it indicates that the received character did not have a valid stop bit (a valid stop bit is 1). In FIFO mode, this error is associated with the character at the top of the FIFO.
///
/// - **DATA** `[7:0]` (RW)
///
///   Receive (read) data character. Transmit (write) data character.
#[repr(transparent)]
pub struct Uartdr {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Uartdr {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Uartdr {
    // ======== Field Constants ========

    /// **OE**
    ///
    /// Overrun error. This bit is set to 1 if data is received and the receive FIFO is already full. This is cleared to 0 once there is an empty space in the FIFO and a new character can be written to it.
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const OE_OFFSET: u32 = 11;
    pub const OE_MASK: u32 = 0x00000800;
    pub const OE: u32 = 1 << 11;

    /// **BE**
    ///
    /// Break error. This bit is set to 1 if a break condition was detected, indicating that the received data input was held LOW for longer than a full-word transmission time (defined as start, data, parity and stop bits). In FIFO mode, this error is associated with the character at the top of the FIFO. When a break occurs, only one 0 character is loaded into the FIFO. The next character is only enabled after the receive data input goes to a 1 (marking state), and the next valid start bit is received.
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const BE_OFFSET: u32 = 10;
    pub const BE_MASK: u32 = 0x00000400;
    pub const BE: u32 = 1 << 10;

    /// **PE**
    ///
    /// Parity error. When set to 1, it indicates that the parity of the received data character does not match the parity that the EPS and SPS bits in the Line Control Register, UARTLCR_H. In FIFO mode, this error is associated with the character at the top of the FIFO.
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const PE_OFFSET: u32 = 9;
    pub const PE_MASK: u32 = 0x00000200;
    pub const PE: u32 = 1 << 9;

    /// **FE**
    ///
    /// Framing error. When set to 1, it indicates that the received character did not have a valid stop bit (a valid stop bit is 1). In FIFO mode, this error is associated with the character at the top of the FIFO.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const FE_OFFSET: u32 = 8;
    pub const FE_MASK: u32 = 0x00000100;
    pub const FE: u32 = 1 << 8;

    /// **DATA**
    ///
    /// Receive (read) data character. Transmit (write) data character.
    ///
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const DATA_OFFSET: u32 = 0;
    pub const DATA_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// **OE** `[11]` (RO)
    ///
    /// Overrun error. This bit is set to 1 if data is received and the receive FIFO is already full. This is cleared to 0 once there is an empty space in the FIFO and a new character can be written to it.
    #[inline(always)]
    pub fn oe(&self) -> bool {
        (self.value.read_bits() & Self::OE_MASK) != 0
    }

    /// **BE** `[10]` (RO)
    ///
    /// Break error. This bit is set to 1 if a break condition was detected, indicating that the received data input was held LOW for longer than a full-word transmission time (defined as start, data, parity and stop bits). In FIFO mode, this error is associated with the character at the top of the FIFO. When a break occurs, only one 0 character is loaded into the FIFO. The next character is only enabled after the receive data input goes to a 1 (marking state), and the next valid start bit is received.
    #[inline(always)]
    pub fn be(&self) -> bool {
        (self.value.read_bits() & Self::BE_MASK) != 0
    }

    /// **PE** `[9]` (RO)
    ///
    /// Parity error. When set to 1, it indicates that the parity of the received data character does not match the parity that the EPS and SPS bits in the Line Control Register, UARTLCR_H. In FIFO mode, this error is associated with the character at the top of the FIFO.
    #[inline(always)]
    pub fn pe(&self) -> bool {
        (self.value.read_bits() & Self::PE_MASK) != 0
    }

    /// **FE** `[8]` (RO)
    ///
    /// Framing error. When set to 1, it indicates that the received character did not have a valid stop bit (a valid stop bit is 1). In FIFO mode, this error is associated with the character at the top of the FIFO.
    #[inline(always)]
    pub fn fe(&self) -> bool {
        (self.value.read_bits() & Self::FE_MASK) != 0
    }

    /// **DATA** `[7:0]` (RW)
    ///
    /// Receive (read) data character. Transmit (write) data character.
    #[inline(always)]
    pub fn data(&self) -> u32 {
        (self.value.read_bits() & Self::DATA_MASK) >> Self::DATA_OFFSET
    }

    /// Write **DATA** `[7:0]` (RW)
    ///
    /// Receive (read) data character. Transmit (write) data character.
    #[inline(always)]
    pub fn write_data(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DATA_MASK) | ((value << Self::DATA_OFFSET) & Self::DATA_MASK)
        });
    }
}

/// UARTDR value type for batch read/modify/write operations
///
/// Use `uartdr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `uartdr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct UartdrVal {
    bits: u32,
}

impl UartdrVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x00000000 }
    }

    /// Read **OE** `[11:11]`
    ///
    /// Overrun error. This bit is set to 1 if data is received and the receive FIFO is already full. This is cleared to 0 once there is an empty space in the FIFO and a new character can be written to it.
    #[inline(always)]
    pub const fn oe(&self) -> bool {
        (self.bits & Uartdr::OE_MASK) != 0
    }

    /// Read **BE** `[10:10]`
    ///
    /// Break error. This bit is set to 1 if a break condition was detected, indicating that the received data input was held LOW for longer than a full-word transmission time (defined as start, data, parity and stop bits). In FIFO mode, this error is associated with the character at the top of the FIFO. When a break occurs, only one 0 character is loaded into the FIFO. The next character is only enabled after the receive data input goes to a 1 (marking state), and the next valid start bit is received.
    #[inline(always)]
    pub const fn be(&self) -> bool {
        (self.bits & Uartdr::BE_MASK) != 0
    }

    /// Read **PE** `[9:9]`
    ///
    /// Parity error. When set to 1, it indicates that the parity of the received data character does not match the parity that the EPS and SPS bits in the Line Control Register, UARTLCR_H. In FIFO mode, this error is associated with the character at the top of the FIFO.
    #[inline(always)]
    pub const fn pe(&self) -> bool {
        (self.bits & Uartdr::PE_MASK) != 0
    }

    /// Read **FE** `[8:8]`
    ///
    /// Framing error. When set to 1, it indicates that the received character did not have a valid stop bit (a valid stop bit is 1). In FIFO mode, this error is associated with the character at the top of the FIFO.
    #[inline(always)]
    pub const fn fe(&self) -> bool {
        (self.bits & Uartdr::FE_MASK) != 0
    }

    /// Read **DATA** `[7:0]`
    ///
    /// Receive (read) data character. Transmit (write) data character.
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        (self.bits & Uartdr::DATA_MASK) >> Uartdr::DATA_OFFSET
    }

    /// Set **DATA** `[7:0]`
    ///
    /// Receive (read) data character. Transmit (write) data character.
    #[inline(always)]
    pub const fn with_data(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Uartdr::DATA_MASK)
                | ((value << Uartdr::DATA_OFFSET) & Uartdr::DATA_MASK),
        }
    }
}

impl Default for UartdrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Uartdr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> UartdrVal {
        UartdrVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: UartdrVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(UartdrVal) -> UartdrVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// UARTRSR Register
///
/// Receive Status Register/Error Clear Register, UARTRSR/UARTECR
///
/// Address offset: 0x0004
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **OE** `[3]` (RW)
///
///   Overrun error. This bit is set to 1 if data is received and the FIFO is already full. This bit is cleared to 0 by a write to UARTECR. The FIFO contents remain valid because no more data is written when the FIFO is full, only the contents of the shift register are overwritten. The CPU must now read the data, to empty the FIFO.
///
/// - **BE** `[2]` (RW)
///
///   Break error. This bit is set to 1 if a break condition was detected, indicating that the received data input was held LOW for longer than a full-word transmission time (defined as start, data, parity, and stop bits). This bit is cleared to 0 after a write to UARTECR. In FIFO mode, this error is associated with the character at the top of the FIFO. When a break occurs, only one 0 character is loaded into the FIFO. The next character is only enabled after the receive data input goes to a 1 (marking state) and the next valid start bit is received.
///
/// - **PE** `[1]` (RW)
///
///   Parity error. When set to 1, it indicates that the parity of the received data character does not match the parity that the EPS and SPS bits in the Line Control Register, UARTLCR_H. This bit is cleared to 0 by a write to UARTECR. In FIFO mode, this error is associated with the character at the top of the FIFO.
///
/// - **FE** `[0]` (RW)
///
///   Framing error. When set to 1, it indicates that the received character did not have a valid stop bit (a valid stop bit is 1). This bit is cleared to 0 by a write to UARTECR. In FIFO mode, this error is associated with the character at the top of the FIFO.
#[repr(transparent)]
pub struct Uartrsr {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Uartrsr {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Uartrsr {
    // ======== Field Constants ========

    /// **OE**
    ///
    /// Overrun error. This bit is set to 1 if data is received and the FIFO is already full. This bit is cleared to 0 by a write to UARTECR. The FIFO contents remain valid because no more data is written when the FIFO is full, only the contents of the shift register are overwritten. The CPU must now read the data, to empty the FIFO.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const OE_OFFSET: u32 = 3;
    pub const OE_MASK: u32 = 0x00000008;
    pub const OE: u32 = 1 << 3;

    /// **BE**
    ///
    /// Break error. This bit is set to 1 if a break condition was detected, indicating that the received data input was held LOW for longer than a full-word transmission time (defined as start, data, parity, and stop bits). This bit is cleared to 0 after a write to UARTECR. In FIFO mode, this error is associated with the character at the top of the FIFO. When a break occurs, only one 0 character is loaded into the FIFO. The next character is only enabled after the receive data input goes to a 1 (marking state) and the next valid start bit is received.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const BE_OFFSET: u32 = 2;
    pub const BE_MASK: u32 = 0x00000004;
    pub const BE: u32 = 1 << 2;

    /// **PE**
    ///
    /// Parity error. When set to 1, it indicates that the parity of the received data character does not match the parity that the EPS and SPS bits in the Line Control Register, UARTLCR_H. This bit is cleared to 0 by a write to UARTECR. In FIFO mode, this error is associated with the character at the top of the FIFO.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const PE_OFFSET: u32 = 1;
    pub const PE_MASK: u32 = 0x00000002;
    pub const PE: u32 = 1 << 1;

    /// **FE**
    ///
    /// Framing error. When set to 1, it indicates that the received character did not have a valid stop bit (a valid stop bit is 1). This bit is cleared to 0 by a write to UARTECR. In FIFO mode, this error is associated with the character at the top of the FIFO.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const FE_OFFSET: u32 = 0;
    pub const FE_MASK: u32 = 0x00000001;
    pub const FE: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **OE** `[3]` (RW)
    ///
    /// Overrun error. This bit is set to 1 if data is received and the FIFO is already full. This bit is cleared to 0 by a write to UARTECR. The FIFO contents remain valid because no more data is written when the FIFO is full, only the contents of the shift register are overwritten. The CPU must now read the data, to empty the FIFO.
    #[inline(always)]
    pub fn oe(&self) -> bool {
        (self.value.read_bits() & Self::OE_MASK) != 0
    }

    /// Set OE bit atomically (RW)
    #[inline(always)]
    pub fn set_oe(&self) {
        self.set_bits(Self::OE_MASK);
    }

    /// Clear OE bit atomically (RW)
    #[inline(always)]
    pub fn clr_oe(&self) {
        self.clr_bits(Self::OE_MASK);
    }

    /// **BE** `[2]` (RW)
    ///
    /// Break error. This bit is set to 1 if a break condition was detected, indicating that the received data input was held LOW for longer than a full-word transmission time (defined as start, data, parity, and stop bits). This bit is cleared to 0 after a write to UARTECR. In FIFO mode, this error is associated with the character at the top of the FIFO. When a break occurs, only one 0 character is loaded into the FIFO. The next character is only enabled after the receive data input goes to a 1 (marking state) and the next valid start bit is received.
    #[inline(always)]
    pub fn be(&self) -> bool {
        (self.value.read_bits() & Self::BE_MASK) != 0
    }

    /// Set BE bit atomically (RW)
    #[inline(always)]
    pub fn set_be(&self) {
        self.set_bits(Self::BE_MASK);
    }

    /// Clear BE bit atomically (RW)
    #[inline(always)]
    pub fn clr_be(&self) {
        self.clr_bits(Self::BE_MASK);
    }

    /// **PE** `[1]` (RW)
    ///
    /// Parity error. When set to 1, it indicates that the parity of the received data character does not match the parity that the EPS and SPS bits in the Line Control Register, UARTLCR_H. This bit is cleared to 0 by a write to UARTECR. In FIFO mode, this error is associated with the character at the top of the FIFO.
    #[inline(always)]
    pub fn pe(&self) -> bool {
        (self.value.read_bits() & Self::PE_MASK) != 0
    }

    /// Set PE bit atomically (RW)
    #[inline(always)]
    pub fn set_pe(&self) {
        self.set_bits(Self::PE_MASK);
    }

    /// Clear PE bit atomically (RW)
    #[inline(always)]
    pub fn clr_pe(&self) {
        self.clr_bits(Self::PE_MASK);
    }

    /// **FE** `[0]` (RW)
    ///
    /// Framing error. When set to 1, it indicates that the received character did not have a valid stop bit (a valid stop bit is 1). This bit is cleared to 0 by a write to UARTECR. In FIFO mode, this error is associated with the character at the top of the FIFO.
    #[inline(always)]
    pub fn fe(&self) -> bool {
        (self.value.read_bits() & Self::FE_MASK) != 0
    }

    /// Set FE bit atomically (RW)
    #[inline(always)]
    pub fn set_fe(&self) {
        self.set_bits(Self::FE_MASK);
    }

    /// Clear FE bit atomically (RW)
    #[inline(always)]
    pub fn clr_fe(&self) {
        self.clr_bits(Self::FE_MASK);
    }
}

/// UARTRSR value type for batch read/modify/write operations
///
/// Use `uartrsr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `uartrsr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct UartrsrVal {
    bits: u32,
}

impl UartrsrVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x00000000 }
    }

    /// Read **OE** `[3:3]`
    ///
    /// Overrun error. This bit is set to 1 if data is received and the FIFO is already full. This bit is cleared to 0 by a write to UARTECR. The FIFO contents remain valid because no more data is written when the FIFO is full, only the contents of the shift register are overwritten. The CPU must now read the data, to empty the FIFO.
    #[inline(always)]
    pub const fn oe(&self) -> bool {
        (self.bits & Uartrsr::OE_MASK) != 0
    }

    /// Read **BE** `[2:2]`
    ///
    /// Break error. This bit is set to 1 if a break condition was detected, indicating that the received data input was held LOW for longer than a full-word transmission time (defined as start, data, parity, and stop bits). This bit is cleared to 0 after a write to UARTECR. In FIFO mode, this error is associated with the character at the top of the FIFO. When a break occurs, only one 0 character is loaded into the FIFO. The next character is only enabled after the receive data input goes to a 1 (marking state) and the next valid start bit is received.
    #[inline(always)]
    pub const fn be(&self) -> bool {
        (self.bits & Uartrsr::BE_MASK) != 0
    }

    /// Read **PE** `[1:1]`
    ///
    /// Parity error. When set to 1, it indicates that the parity of the received data character does not match the parity that the EPS and SPS bits in the Line Control Register, UARTLCR_H. This bit is cleared to 0 by a write to UARTECR. In FIFO mode, this error is associated with the character at the top of the FIFO.
    #[inline(always)]
    pub const fn pe(&self) -> bool {
        (self.bits & Uartrsr::PE_MASK) != 0
    }

    /// Read **FE** `[0:0]`
    ///
    /// Framing error. When set to 1, it indicates that the received character did not have a valid stop bit (a valid stop bit is 1). This bit is cleared to 0 by a write to UARTECR. In FIFO mode, this error is associated with the character at the top of the FIFO.
    #[inline(always)]
    pub const fn fe(&self) -> bool {
        (self.bits & Uartrsr::FE_MASK) != 0
    }

    /// Set **OE** `[3:3]`
    ///
    /// Overrun error. This bit is set to 1 if data is received and the FIFO is already full. This bit is cleared to 0 by a write to UARTECR. The FIFO contents remain valid because no more data is written when the FIFO is full, only the contents of the shift register are overwritten. The CPU must now read the data, to empty the FIFO.
    #[inline(always)]
    pub const fn with_oe(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Uartrsr::OE_MASK
            } else {
                self.bits & !Uartrsr::OE_MASK
            },
        }
    }

    /// Set **BE** `[2:2]`
    ///
    /// Break error. This bit is set to 1 if a break condition was detected, indicating that the received data input was held LOW for longer than a full-word transmission time (defined as start, data, parity, and stop bits). This bit is cleared to 0 after a write to UARTECR. In FIFO mode, this error is associated with the character at the top of the FIFO. When a break occurs, only one 0 character is loaded into the FIFO. The next character is only enabled after the receive data input goes to a 1 (marking state) and the next valid start bit is received.
    #[inline(always)]
    pub const fn with_be(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Uartrsr::BE_MASK
            } else {
                self.bits & !Uartrsr::BE_MASK
            },
        }
    }

    /// Set **PE** `[1:1]`
    ///
    /// Parity error. When set to 1, it indicates that the parity of the received data character does not match the parity that the EPS and SPS bits in the Line Control Register, UARTLCR_H. This bit is cleared to 0 by a write to UARTECR. In FIFO mode, this error is associated with the character at the top of the FIFO.
    #[inline(always)]
    pub const fn with_pe(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Uartrsr::PE_MASK
            } else {
                self.bits & !Uartrsr::PE_MASK
            },
        }
    }

    /// Set **FE** `[0:0]`
    ///
    /// Framing error. When set to 1, it indicates that the received character did not have a valid stop bit (a valid stop bit is 1). This bit is cleared to 0 by a write to UARTECR. In FIFO mode, this error is associated with the character at the top of the FIFO.
    #[inline(always)]
    pub const fn with_fe(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Uartrsr::FE_MASK
            } else {
                self.bits & !Uartrsr::FE_MASK
            },
        }
    }
}

impl Default for UartrsrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Uartrsr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> UartrsrVal {
        UartrsrVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: UartrsrVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(UartrsrVal) -> UartrsrVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// UARTFR Register
///
/// Flag Register, UARTFR
///
/// Address offset: 0x0018
/// Reset value: 0x00000090
///
/// ## Fields
///
/// - **RI** `[8]` (RO)
///
///   Ring indicator. This bit is the complement of the UART ring indicator, nUARTRI, modem status input. That is, the bit is 1 when nUARTRI is LOW.
///
/// - **TXFE** `[7]` (RO)
///
///   Transmit FIFO empty. The meaning of this bit depends on the state of the FEN bit in the Line Control Register, UARTLCR_H. If the FIFO is disabled, this bit is set when the transmit holding register is empty. If the FIFO is enabled, the TXFE bit is set when the transmit FIFO is empty. This bit does not indicate if there is data in the transmit shift register.
///
/// - **RXFF** `[6]` (RO)
///
///   Receive FIFO full. The meaning of this bit depends on the state of the FEN bit in the UARTLCR_H Register. If the FIFO is disabled, this bit is set when the receive holding register is full. If the FIFO is enabled, the RXFF bit is set when the receive FIFO is full.
///
/// - **TXFF** `[5]` (RO)
///
///   Transmit FIFO full. The meaning of this bit depends on the state of the FEN bit in the UARTLCR_H Register. If the FIFO is disabled, this bit is set when the transmit holding register is full. If the FIFO is enabled, the TXFF bit is set when the transmit FIFO is full.
///
/// - **RXFE** `[4]` (RO)
///
///   Receive FIFO empty. The meaning of this bit depends on the state of the FEN bit in the UARTLCR_H Register. If the FIFO is disabled, this bit is set when the receive holding register is empty. If the FIFO is enabled, the RXFE bit is set when the receive FIFO is empty.
///
/// - **BUSY** `[3]` (RO)
///
///   UART busy. If this bit is set to 1, the UART is busy transmitting data. This bit remains set until the complete byte, including all the stop bits, has been sent from the shift register. This bit is set as soon as the transmit FIFO becomes non-empty, regardless of whether the UART is enabled or not.
///
/// - **DCD** `[2]` (RO)
///
///   Data carrier detect. This bit is the complement of the UART data carrier detect, nUARTDCD, modem status input. That is, the bit is 1 when nUARTDCD is LOW.
///
/// - **DSR** `[1]` (RO)
///
///   Data set ready. This bit is the complement of the UART data set ready, nUARTDSR, modem status input. That is, the bit is 1 when nUARTDSR is LOW.
///
/// - **CTS** `[0]` (RO)
///
///   Clear to send. This bit is the complement of the UART clear to send, nUARTCTS, modem status input. That is, the bit is 1 when nUARTCTS is LOW.
#[repr(transparent)]
pub struct Uartfr {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Uartfr {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Uartfr {
    // ======== Field Constants ========

    /// **RI**
    ///
    /// Ring indicator. This bit is the complement of the UART ring indicator, nUARTRI, modem status input. That is, the bit is 1 when nUARTRI is LOW.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const RI_OFFSET: u32 = 8;
    pub const RI_MASK: u32 = 0x00000100;
    pub const RI: u32 = 1 << 8;

    /// **TXFE**
    ///
    /// Transmit FIFO empty. The meaning of this bit depends on the state of the FEN bit in the Line Control Register, UARTLCR_H. If the FIFO is disabled, this bit is set when the transmit holding register is empty. If the FIFO is enabled, the TXFE bit is set when the transmit FIFO is empty. This bit does not indicate if there is data in the transmit shift register.
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const TXFE_OFFSET: u32 = 7;
    pub const TXFE_MASK: u32 = 0x00000080;
    pub const TXFE: u32 = 1 << 7;

    /// **RXFF**
    ///
    /// Receive FIFO full. The meaning of this bit depends on the state of the FEN bit in the UARTLCR_H Register. If the FIFO is disabled, this bit is set when the receive holding register is full. If the FIFO is enabled, the RXFF bit is set when the receive FIFO is full.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const RXFF_OFFSET: u32 = 6;
    pub const RXFF_MASK: u32 = 0x00000040;
    pub const RXFF: u32 = 1 << 6;

    /// **TXFF**
    ///
    /// Transmit FIFO full. The meaning of this bit depends on the state of the FEN bit in the UARTLCR_H Register. If the FIFO is disabled, this bit is set when the transmit holding register is full. If the FIFO is enabled, the TXFF bit is set when the transmit FIFO is full.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const TXFF_OFFSET: u32 = 5;
    pub const TXFF_MASK: u32 = 0x00000020;
    pub const TXFF: u32 = 1 << 5;

    /// **RXFE**
    ///
    /// Receive FIFO empty. The meaning of this bit depends on the state of the FEN bit in the UARTLCR_H Register. If the FIFO is disabled, this bit is set when the receive holding register is empty. If the FIFO is enabled, the RXFE bit is set when the receive FIFO is empty.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const RXFE_OFFSET: u32 = 4;
    pub const RXFE_MASK: u32 = 0x00000010;
    pub const RXFE: u32 = 1 << 4;

    /// **BUSY**
    ///
    /// UART busy. If this bit is set to 1, the UART is busy transmitting data. This bit remains set until the complete byte, including all the stop bits, has been sent from the shift register. This bit is set as soon as the transmit FIFO becomes non-empty, regardless of whether the UART is enabled or not.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const BUSY_OFFSET: u32 = 3;
    pub const BUSY_MASK: u32 = 0x00000008;
    pub const BUSY: u32 = 1 << 3;

    /// **DCD**
    ///
    /// Data carrier detect. This bit is the complement of the UART data carrier detect, nUARTDCD, modem status input. That is, the bit is 1 when nUARTDCD is LOW.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const DCD_OFFSET: u32 = 2;
    pub const DCD_MASK: u32 = 0x00000004;
    pub const DCD: u32 = 1 << 2;

    /// **DSR**
    ///
    /// Data set ready. This bit is the complement of the UART data set ready, nUARTDSR, modem status input. That is, the bit is 1 when nUARTDSR is LOW.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const DSR_OFFSET: u32 = 1;
    pub const DSR_MASK: u32 = 0x00000002;
    pub const DSR: u32 = 1 << 1;

    /// **CTS**
    ///
    /// Clear to send. This bit is the complement of the UART clear to send, nUARTCTS, modem status input. That is, the bit is 1 when nUARTCTS is LOW.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const CTS_OFFSET: u32 = 0;
    pub const CTS_MASK: u32 = 0x00000001;
    pub const CTS: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **RI** `[8]` (RO)
    ///
    /// Ring indicator. This bit is the complement of the UART ring indicator, nUARTRI, modem status input. That is, the bit is 1 when nUARTRI is LOW.
    #[inline(always)]
    pub fn ri(&self) -> bool {
        (self.value.read_bits() & Self::RI_MASK) != 0
    }

    /// **TXFE** `[7]` (RO)
    ///
    /// Transmit FIFO empty. The meaning of this bit depends on the state of the FEN bit in the Line Control Register, UARTLCR_H. If the FIFO is disabled, this bit is set when the transmit holding register is empty. If the FIFO is enabled, the TXFE bit is set when the transmit FIFO is empty. This bit does not indicate if there is data in the transmit shift register.
    #[inline(always)]
    pub fn txfe(&self) -> bool {
        (self.value.read_bits() & Self::TXFE_MASK) != 0
    }

    /// **RXFF** `[6]` (RO)
    ///
    /// Receive FIFO full. The meaning of this bit depends on the state of the FEN bit in the UARTLCR_H Register. If the FIFO is disabled, this bit is set when the receive holding register is full. If the FIFO is enabled, the RXFF bit is set when the receive FIFO is full.
    #[inline(always)]
    pub fn rxff(&self) -> bool {
        (self.value.read_bits() & Self::RXFF_MASK) != 0
    }

    /// **TXFF** `[5]` (RO)
    ///
    /// Transmit FIFO full. The meaning of this bit depends on the state of the FEN bit in the UARTLCR_H Register. If the FIFO is disabled, this bit is set when the transmit holding register is full. If the FIFO is enabled, the TXFF bit is set when the transmit FIFO is full.
    #[inline(always)]
    pub fn txff(&self) -> bool {
        (self.value.read_bits() & Self::TXFF_MASK) != 0
    }

    /// **RXFE** `[4]` (RO)
    ///
    /// Receive FIFO empty. The meaning of this bit depends on the state of the FEN bit in the UARTLCR_H Register. If the FIFO is disabled, this bit is set when the receive holding register is empty. If the FIFO is enabled, the RXFE bit is set when the receive FIFO is empty.
    #[inline(always)]
    pub fn rxfe(&self) -> bool {
        (self.value.read_bits() & Self::RXFE_MASK) != 0
    }

    /// **BUSY** `[3]` (RO)
    ///
    /// UART busy. If this bit is set to 1, the UART is busy transmitting data. This bit remains set until the complete byte, including all the stop bits, has been sent from the shift register. This bit is set as soon as the transmit FIFO becomes non-empty, regardless of whether the UART is enabled or not.
    #[inline(always)]
    pub fn busy(&self) -> bool {
        (self.value.read_bits() & Self::BUSY_MASK) != 0
    }

    /// **DCD** `[2]` (RO)
    ///
    /// Data carrier detect. This bit is the complement of the UART data carrier detect, nUARTDCD, modem status input. That is, the bit is 1 when nUARTDCD is LOW.
    #[inline(always)]
    pub fn dcd(&self) -> bool {
        (self.value.read_bits() & Self::DCD_MASK) != 0
    }

    /// **DSR** `[1]` (RO)
    ///
    /// Data set ready. This bit is the complement of the UART data set ready, nUARTDSR, modem status input. That is, the bit is 1 when nUARTDSR is LOW.
    #[inline(always)]
    pub fn dsr(&self) -> bool {
        (self.value.read_bits() & Self::DSR_MASK) != 0
    }

    /// **CTS** `[0]` (RO)
    ///
    /// Clear to send. This bit is the complement of the UART clear to send, nUARTCTS, modem status input. That is, the bit is 1 when nUARTCTS is LOW.
    #[inline(always)]
    pub fn cts(&self) -> bool {
        (self.value.read_bits() & Self::CTS_MASK) != 0
    }
}

/// UARTFR value type for batch read/modify/write operations
///
/// Use `uartfr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `uartfr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct UartfrVal {
    bits: u32,
}

impl UartfrVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x00000090 }
    }

    /// Read **RI** `[8:8]`
    ///
    /// Ring indicator. This bit is the complement of the UART ring indicator, nUARTRI, modem status input. That is, the bit is 1 when nUARTRI is LOW.
    #[inline(always)]
    pub const fn ri(&self) -> bool {
        (self.bits & Uartfr::RI_MASK) != 0
    }

    /// Read **TXFE** `[7:7]`
    ///
    /// Transmit FIFO empty. The meaning of this bit depends on the state of the FEN bit in the Line Control Register, UARTLCR_H. If the FIFO is disabled, this bit is set when the transmit holding register is empty. If the FIFO is enabled, the TXFE bit is set when the transmit FIFO is empty. This bit does not indicate if there is data in the transmit shift register.
    #[inline(always)]
    pub const fn txfe(&self) -> bool {
        (self.bits & Uartfr::TXFE_MASK) != 0
    }

    /// Read **RXFF** `[6:6]`
    ///
    /// Receive FIFO full. The meaning of this bit depends on the state of the FEN bit in the UARTLCR_H Register. If the FIFO is disabled, this bit is set when the receive holding register is full. If the FIFO is enabled, the RXFF bit is set when the receive FIFO is full.
    #[inline(always)]
    pub const fn rxff(&self) -> bool {
        (self.bits & Uartfr::RXFF_MASK) != 0
    }

    /// Read **TXFF** `[5:5]`
    ///
    /// Transmit FIFO full. The meaning of this bit depends on the state of the FEN bit in the UARTLCR_H Register. If the FIFO is disabled, this bit is set when the transmit holding register is full. If the FIFO is enabled, the TXFF bit is set when the transmit FIFO is full.
    #[inline(always)]
    pub const fn txff(&self) -> bool {
        (self.bits & Uartfr::TXFF_MASK) != 0
    }

    /// Read **RXFE** `[4:4]`
    ///
    /// Receive FIFO empty. The meaning of this bit depends on the state of the FEN bit in the UARTLCR_H Register. If the FIFO is disabled, this bit is set when the receive holding register is empty. If the FIFO is enabled, the RXFE bit is set when the receive FIFO is empty.
    #[inline(always)]
    pub const fn rxfe(&self) -> bool {
        (self.bits & Uartfr::RXFE_MASK) != 0
    }

    /// Read **BUSY** `[3:3]`
    ///
    /// UART busy. If this bit is set to 1, the UART is busy transmitting data. This bit remains set until the complete byte, including all the stop bits, has been sent from the shift register. This bit is set as soon as the transmit FIFO becomes non-empty, regardless of whether the UART is enabled or not.
    #[inline(always)]
    pub const fn busy(&self) -> bool {
        (self.bits & Uartfr::BUSY_MASK) != 0
    }

    /// Read **DCD** `[2:2]`
    ///
    /// Data carrier detect. This bit is the complement of the UART data carrier detect, nUARTDCD, modem status input. That is, the bit is 1 when nUARTDCD is LOW.
    #[inline(always)]
    pub const fn dcd(&self) -> bool {
        (self.bits & Uartfr::DCD_MASK) != 0
    }

    /// Read **DSR** `[1:1]`
    ///
    /// Data set ready. This bit is the complement of the UART data set ready, nUARTDSR, modem status input. That is, the bit is 1 when nUARTDSR is LOW.
    #[inline(always)]
    pub const fn dsr(&self) -> bool {
        (self.bits & Uartfr::DSR_MASK) != 0
    }

    /// Read **CTS** `[0:0]`
    ///
    /// Clear to send. This bit is the complement of the UART clear to send, nUARTCTS, modem status input. That is, the bit is 1 when nUARTCTS is LOW.
    #[inline(always)]
    pub const fn cts(&self) -> bool {
        (self.bits & Uartfr::CTS_MASK) != 0
    }
}

impl Default for UartfrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Uartfr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> UartfrVal {
        UartfrVal::from_bits(self.value.read_bits())
    }
}

/// UARTILPR Register
///
/// IrDA Low-Power Counter Register, UARTILPR
///
/// Address offset: 0x0020
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **ILPDVSR** `[7:0]` (RW)
///
///   8-bit low-power divisor value. These bits are cleared to 0 at reset.
#[repr(transparent)]
pub struct Uartilpr {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Uartilpr {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Uartilpr {
    // ======== Field Constants ========

    /// **ILPDVSR**
    ///
    /// 8-bit low-power divisor value. These bits are cleared to 0 at reset.
    ///
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const ILPDVSR_OFFSET: u32 = 0;
    pub const ILPDVSR_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// **ILPDVSR** `[7:0]` (RW)
    ///
    /// 8-bit low-power divisor value. These bits are cleared to 0 at reset.
    #[inline(always)]
    pub fn ilpdvsr(&self) -> u32 {
        (self.value.read_bits() & Self::ILPDVSR_MASK) >> Self::ILPDVSR_OFFSET
    }

    /// Write **ILPDVSR** `[7:0]` (RW)
    ///
    /// 8-bit low-power divisor value. These bits are cleared to 0 at reset.
    #[inline(always)]
    pub fn write_ilpdvsr(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ILPDVSR_MASK) | ((value << Self::ILPDVSR_OFFSET) & Self::ILPDVSR_MASK)
        });
    }
}

/// UARTILPR value type for batch read/modify/write operations
///
/// Use `uartilpr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `uartilpr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct UartilprVal {
    bits: u32,
}

impl UartilprVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x00000000 }
    }

    /// Read **ILPDVSR** `[7:0]`
    ///
    /// 8-bit low-power divisor value. These bits are cleared to 0 at reset.
    #[inline(always)]
    pub const fn ilpdvsr(&self) -> u32 {
        (self.bits & Uartilpr::ILPDVSR_MASK) >> Uartilpr::ILPDVSR_OFFSET
    }

    /// Set **ILPDVSR** `[7:0]`
    ///
    /// 8-bit low-power divisor value. These bits are cleared to 0 at reset.
    #[inline(always)]
    pub const fn with_ilpdvsr(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Uartilpr::ILPDVSR_MASK)
                | ((value << Uartilpr::ILPDVSR_OFFSET) & Uartilpr::ILPDVSR_MASK),
        }
    }
}

impl Default for UartilprVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Uartilpr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> UartilprVal {
        UartilprVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: UartilprVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(UartilprVal) -> UartilprVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// UARTIBRD Register
///
/// Integer Baud Rate Register, UARTIBRD
///
/// Address offset: 0x0024
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **BAUD_DIVINT** `[15:0]` (RW)
///
///   The integer baud rate divisor. These bits are cleared to 0 on reset.
#[repr(transparent)]
pub struct Uartibrd {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Uartibrd {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Uartibrd {
    // ======== Field Constants ========

    /// **BAUD_DIVINT**
    ///
    /// The integer baud rate divisor. These bits are cleared to 0 on reset.
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const BAUD_DIVINT_OFFSET: u32 = 0;
    pub const BAUD_DIVINT_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **BAUD_DIVINT** `[15:0]` (RW)
    ///
    /// The integer baud rate divisor. These bits are cleared to 0 on reset.
    #[inline(always)]
    pub fn baud_divint(&self) -> u32 {
        (self.value.read_bits() & Self::BAUD_DIVINT_MASK) >> Self::BAUD_DIVINT_OFFSET
    }

    /// Write **BAUD_DIVINT** `[15:0]` (RW)
    ///
    /// The integer baud rate divisor. These bits are cleared to 0 on reset.
    #[inline(always)]
    pub fn write_baud_divint(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::BAUD_DIVINT_MASK)
                | ((value << Self::BAUD_DIVINT_OFFSET) & Self::BAUD_DIVINT_MASK)
        });
    }
}

/// UARTIBRD value type for batch read/modify/write operations
///
/// Use `uartibrd.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `uartibrd.write_val(val)`.
#[derive(Clone, Copy)]
pub struct UartibrdVal {
    bits: u32,
}

impl UartibrdVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x00000000 }
    }

    /// Read **BAUD_DIVINT** `[15:0]`
    ///
    /// The integer baud rate divisor. These bits are cleared to 0 on reset.
    #[inline(always)]
    pub const fn baud_divint(&self) -> u32 {
        (self.bits & Uartibrd::BAUD_DIVINT_MASK) >> Uartibrd::BAUD_DIVINT_OFFSET
    }

    /// Set **BAUD_DIVINT** `[15:0]`
    ///
    /// The integer baud rate divisor. These bits are cleared to 0 on reset.
    #[inline(always)]
    pub const fn with_baud_divint(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Uartibrd::BAUD_DIVINT_MASK)
                | ((value << Uartibrd::BAUD_DIVINT_OFFSET) & Uartibrd::BAUD_DIVINT_MASK),
        }
    }
}

impl Default for UartibrdVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Uartibrd {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> UartibrdVal {
        UartibrdVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: UartibrdVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(UartibrdVal) -> UartibrdVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// UARTFBRD Register
///
/// Fractional Baud Rate Register, UARTFBRD
///
/// Address offset: 0x0028
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **BAUD_DIVFRAC** `[5:0]` (RW)
///
///   The fractional baud rate divisor. These bits are cleared to 0 on reset.
#[repr(transparent)]
pub struct Uartfbrd {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Uartfbrd {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Uartfbrd {
    // ======== Field Constants ========

    /// **BAUD_DIVFRAC**
    ///
    /// The fractional baud rate divisor. These bits are cleared to 0 on reset.
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const BAUD_DIVFRAC_OFFSET: u32 = 0;
    pub const BAUD_DIVFRAC_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **BAUD_DIVFRAC** `[5:0]` (RW)
    ///
    /// The fractional baud rate divisor. These bits are cleared to 0 on reset.
    #[inline(always)]
    pub fn baud_divfrac(&self) -> u32 {
        (self.value.read_bits() & Self::BAUD_DIVFRAC_MASK) >> Self::BAUD_DIVFRAC_OFFSET
    }

    /// Write **BAUD_DIVFRAC** `[5:0]` (RW)
    ///
    /// The fractional baud rate divisor. These bits are cleared to 0 on reset.
    #[inline(always)]
    pub fn write_baud_divfrac(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::BAUD_DIVFRAC_MASK)
                | ((value << Self::BAUD_DIVFRAC_OFFSET) & Self::BAUD_DIVFRAC_MASK)
        });
    }
}

/// UARTFBRD value type for batch read/modify/write operations
///
/// Use `uartfbrd.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `uartfbrd.write_val(val)`.
#[derive(Clone, Copy)]
pub struct UartfbrdVal {
    bits: u32,
}

impl UartfbrdVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x00000000 }
    }

    /// Read **BAUD_DIVFRAC** `[5:0]`
    ///
    /// The fractional baud rate divisor. These bits are cleared to 0 on reset.
    #[inline(always)]
    pub const fn baud_divfrac(&self) -> u32 {
        (self.bits & Uartfbrd::BAUD_DIVFRAC_MASK) >> Uartfbrd::BAUD_DIVFRAC_OFFSET
    }

    /// Set **BAUD_DIVFRAC** `[5:0]`
    ///
    /// The fractional baud rate divisor. These bits are cleared to 0 on reset.
    #[inline(always)]
    pub const fn with_baud_divfrac(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Uartfbrd::BAUD_DIVFRAC_MASK)
                | ((value << Uartfbrd::BAUD_DIVFRAC_OFFSET) & Uartfbrd::BAUD_DIVFRAC_MASK),
        }
    }
}

impl Default for UartfbrdVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Uartfbrd {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> UartfbrdVal {
        UartfbrdVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: UartfbrdVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(UartfbrdVal) -> UartfbrdVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// UARTLCR_H Register
///
/// Line Control Register, UARTLCR_H
///
/// Address offset: 0x002c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **SPS** `[7]` (RW)
///
///   Stick parity select. 0 = stick parity is disabled 1 = either: * if the EPS bit is 0 then the parity bit is transmitted and checked as a 1 * if the EPS bit is 1 then the parity bit is transmitted and checked as a 0. This bit has no effect when the PEN bit disables parity checking and generation.
///
/// - **WLEN** `[6:5]` (RW)
///
///   Word length. These bits indicate the number of data bits transmitted or received in a frame as follows: b11 = 8 bits b10 = 7 bits b01 = 6 bits b00 = 5 bits.
///
/// - **FEN** `[4]` (RW)
///
///   Enable FIFOs: 0 = FIFOs are disabled (character mode) that is, the FIFOs become 1-byte-deep holding registers 1 = transmit and receive FIFO buffers are enabled (FIFO mode).
///
/// - **STP2** `[3]` (RW)
///
///   Two stop bits select. If this bit is set to 1, two stop bits are transmitted at the end of the frame. The receive logic does not check for two stop bits being received.
///
/// - **EPS** `[2]` (RW)
///
///   Even parity select. Controls the type of parity the UART uses during transmission and reception: 0 = odd parity. The UART generates or checks for an odd number of 1s in the data and parity bits. 1 = even parity. The UART generates or checks for an even number of 1s in the data and parity bits. This bit has no effect when the PEN bit disables parity checking and generation.
///
/// - **PEN** `[1]` (RW)
///
///   Parity enable: 0 = parity is disabled and no parity bit added to the data frame 1 = parity checking and generation is enabled.
///
/// - **BRK** `[0]` (RW)
///
///   Send break. If this bit is set to 1, a low-level is continually output on the UARTTXD output, after completing transmission of the current character. For the proper execution of the break command, the software must set this bit for at least two complete frames. For normal use, this bit must be cleared to 0.
#[repr(transparent)]
pub struct UartlcrH {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for UartlcrH {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl UartlcrH {
    // ======== Field Constants ========

    /// **SPS**
    ///
    /// Stick parity select. 0 = stick parity is disabled 1 = either: * if the EPS bit is 0 then the parity bit is transmitted and checked as a 1 * if the EPS bit is 1 then the parity bit is transmitted and checked as a 0. This bit has no effect when the PEN bit disables parity checking and generation.
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const SPS_OFFSET: u32 = 7;
    pub const SPS_MASK: u32 = 0x00000080;
    pub const SPS: u32 = 1 << 7;

    /// **WLEN**
    ///
    /// Word length. These bits indicate the number of data bits transmitted or received in a frame as follows: b11 = 8 bits b10 = 7 bits b01 = 6 bits b00 = 5 bits.
    ///
    /// Bits `[6:5]`, mask `0x00000060`
    pub const WLEN_OFFSET: u32 = 5;
    pub const WLEN_MASK: u32 = 0x00000060;

    /// **FEN**
    ///
    /// Enable FIFOs: 0 = FIFOs are disabled (character mode) that is, the FIFOs become 1-byte-deep holding registers 1 = transmit and receive FIFO buffers are enabled (FIFO mode).
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const FEN_OFFSET: u32 = 4;
    pub const FEN_MASK: u32 = 0x00000010;
    pub const FEN: u32 = 1 << 4;

    /// **STP2**
    ///
    /// Two stop bits select. If this bit is set to 1, two stop bits are transmitted at the end of the frame. The receive logic does not check for two stop bits being received.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const STP2_OFFSET: u32 = 3;
    pub const STP2_MASK: u32 = 0x00000008;
    pub const STP2: u32 = 1 << 3;

    /// **EPS**
    ///
    /// Even parity select. Controls the type of parity the UART uses during transmission and reception: 0 = odd parity. The UART generates or checks for an odd number of 1s in the data and parity bits. 1 = even parity. The UART generates or checks for an even number of 1s in the data and parity bits. This bit has no effect when the PEN bit disables parity checking and generation.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const EPS_OFFSET: u32 = 2;
    pub const EPS_MASK: u32 = 0x00000004;
    pub const EPS: u32 = 1 << 2;

    /// **PEN**
    ///
    /// Parity enable: 0 = parity is disabled and no parity bit added to the data frame 1 = parity checking and generation is enabled.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const PEN_OFFSET: u32 = 1;
    pub const PEN_MASK: u32 = 0x00000002;
    pub const PEN: u32 = 1 << 1;

    /// **BRK**
    ///
    /// Send break. If this bit is set to 1, a low-level is continually output on the UARTTXD output, after completing transmission of the current character. For the proper execution of the break command, the software must set this bit for at least two complete frames. For normal use, this bit must be cleared to 0.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const BRK_OFFSET: u32 = 0;
    pub const BRK_MASK: u32 = 0x00000001;
    pub const BRK: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **SPS** `[7]` (RW)
    ///
    /// Stick parity select. 0 = stick parity is disabled 1 = either: * if the EPS bit is 0 then the parity bit is transmitted and checked as a 1 * if the EPS bit is 1 then the parity bit is transmitted and checked as a 0. This bit has no effect when the PEN bit disables parity checking and generation.
    #[inline(always)]
    pub fn sps(&self) -> bool {
        (self.value.read_bits() & Self::SPS_MASK) != 0
    }

    /// Set SPS bit atomically (RW)
    #[inline(always)]
    pub fn set_sps(&self) {
        self.set_bits(Self::SPS_MASK);
    }

    /// Clear SPS bit atomically (RW)
    #[inline(always)]
    pub fn clr_sps(&self) {
        self.clr_bits(Self::SPS_MASK);
    }

    /// **WLEN** `[6:5]` (RW)
    ///
    /// Word length. These bits indicate the number of data bits transmitted or received in a frame as follows: b11 = 8 bits b10 = 7 bits b01 = 6 bits b00 = 5 bits.
    #[inline(always)]
    pub fn wlen(&self) -> u32 {
        (self.value.read_bits() & Self::WLEN_MASK) >> Self::WLEN_OFFSET
    }

    /// Write **WLEN** `[6:5]` (RW)
    ///
    /// Word length. These bits indicate the number of data bits transmitted or received in a frame as follows: b11 = 8 bits b10 = 7 bits b01 = 6 bits b00 = 5 bits.
    #[inline(always)]
    pub fn write_wlen(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::WLEN_MASK) | ((value << Self::WLEN_OFFSET) & Self::WLEN_MASK)
        });
    }

    /// **FEN** `[4]` (RW)
    ///
    /// Enable FIFOs: 0 = FIFOs are disabled (character mode) that is, the FIFOs become 1-byte-deep holding registers 1 = transmit and receive FIFO buffers are enabled (FIFO mode).
    #[inline(always)]
    pub fn fen(&self) -> bool {
        (self.value.read_bits() & Self::FEN_MASK) != 0
    }

    /// Set FEN bit atomically (RW)
    #[inline(always)]
    pub fn set_fen(&self) {
        self.set_bits(Self::FEN_MASK);
    }

    /// Clear FEN bit atomically (RW)
    #[inline(always)]
    pub fn clr_fen(&self) {
        self.clr_bits(Self::FEN_MASK);
    }

    /// **STP2** `[3]` (RW)
    ///
    /// Two stop bits select. If this bit is set to 1, two stop bits are transmitted at the end of the frame. The receive logic does not check for two stop bits being received.
    #[inline(always)]
    pub fn stp2(&self) -> bool {
        (self.value.read_bits() & Self::STP2_MASK) != 0
    }

    /// Set STP2 bit atomically (RW)
    #[inline(always)]
    pub fn set_stp2(&self) {
        self.set_bits(Self::STP2_MASK);
    }

    /// Clear STP2 bit atomically (RW)
    #[inline(always)]
    pub fn clr_stp2(&self) {
        self.clr_bits(Self::STP2_MASK);
    }

    /// **EPS** `[2]` (RW)
    ///
    /// Even parity select. Controls the type of parity the UART uses during transmission and reception: 0 = odd parity. The UART generates or checks for an odd number of 1s in the data and parity bits. 1 = even parity. The UART generates or checks for an even number of 1s in the data and parity bits. This bit has no effect when the PEN bit disables parity checking and generation.
    #[inline(always)]
    pub fn eps(&self) -> bool {
        (self.value.read_bits() & Self::EPS_MASK) != 0
    }

    /// Set EPS bit atomically (RW)
    #[inline(always)]
    pub fn set_eps(&self) {
        self.set_bits(Self::EPS_MASK);
    }

    /// Clear EPS bit atomically (RW)
    #[inline(always)]
    pub fn clr_eps(&self) {
        self.clr_bits(Self::EPS_MASK);
    }

    /// **PEN** `[1]` (RW)
    ///
    /// Parity enable: 0 = parity is disabled and no parity bit added to the data frame 1 = parity checking and generation is enabled.
    #[inline(always)]
    pub fn pen(&self) -> bool {
        (self.value.read_bits() & Self::PEN_MASK) != 0
    }

    /// Set PEN bit atomically (RW)
    #[inline(always)]
    pub fn set_pen(&self) {
        self.set_bits(Self::PEN_MASK);
    }

    /// Clear PEN bit atomically (RW)
    #[inline(always)]
    pub fn clr_pen(&self) {
        self.clr_bits(Self::PEN_MASK);
    }

    /// **BRK** `[0]` (RW)
    ///
    /// Send break. If this bit is set to 1, a low-level is continually output on the UARTTXD output, after completing transmission of the current character. For the proper execution of the break command, the software must set this bit for at least two complete frames. For normal use, this bit must be cleared to 0.
    #[inline(always)]
    pub fn brk(&self) -> bool {
        (self.value.read_bits() & Self::BRK_MASK) != 0
    }

    /// Set BRK bit atomically (RW)
    #[inline(always)]
    pub fn set_brk(&self) {
        self.set_bits(Self::BRK_MASK);
    }

    /// Clear BRK bit atomically (RW)
    #[inline(always)]
    pub fn clr_brk(&self) {
        self.clr_bits(Self::BRK_MASK);
    }
}

/// UARTLCR_H value type for batch read/modify/write operations
///
/// Use `uartlcrh.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `uartlcrh.write_val(val)`.
#[derive(Clone, Copy)]
pub struct UartlcrHVal {
    bits: u32,
}

impl UartlcrHVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x00000000 }
    }

    /// Read **SPS** `[7:7]`
    ///
    /// Stick parity select. 0 = stick parity is disabled 1 = either: * if the EPS bit is 0 then the parity bit is transmitted and checked as a 1 * if the EPS bit is 1 then the parity bit is transmitted and checked as a 0. This bit has no effect when the PEN bit disables parity checking and generation.
    #[inline(always)]
    pub const fn sps(&self) -> bool {
        (self.bits & UartlcrH::SPS_MASK) != 0
    }

    /// Read **WLEN** `[6:5]`
    ///
    /// Word length. These bits indicate the number of data bits transmitted or received in a frame as follows: b11 = 8 bits b10 = 7 bits b01 = 6 bits b00 = 5 bits.
    #[inline(always)]
    pub const fn wlen(&self) -> u32 {
        (self.bits & UartlcrH::WLEN_MASK) >> UartlcrH::WLEN_OFFSET
    }

    /// Read **FEN** `[4:4]`
    ///
    /// Enable FIFOs: 0 = FIFOs are disabled (character mode) that is, the FIFOs become 1-byte-deep holding registers 1 = transmit and receive FIFO buffers are enabled (FIFO mode).
    #[inline(always)]
    pub const fn fen(&self) -> bool {
        (self.bits & UartlcrH::FEN_MASK) != 0
    }

    /// Read **STP2** `[3:3]`
    ///
    /// Two stop bits select. If this bit is set to 1, two stop bits are transmitted at the end of the frame. The receive logic does not check for two stop bits being received.
    #[inline(always)]
    pub const fn stp2(&self) -> bool {
        (self.bits & UartlcrH::STP2_MASK) != 0
    }

    /// Read **EPS** `[2:2]`
    ///
    /// Even parity select. Controls the type of parity the UART uses during transmission and reception: 0 = odd parity. The UART generates or checks for an odd number of 1s in the data and parity bits. 1 = even parity. The UART generates or checks for an even number of 1s in the data and parity bits. This bit has no effect when the PEN bit disables parity checking and generation.
    #[inline(always)]
    pub const fn eps(&self) -> bool {
        (self.bits & UartlcrH::EPS_MASK) != 0
    }

    /// Read **PEN** `[1:1]`
    ///
    /// Parity enable: 0 = parity is disabled and no parity bit added to the data frame 1 = parity checking and generation is enabled.
    #[inline(always)]
    pub const fn pen(&self) -> bool {
        (self.bits & UartlcrH::PEN_MASK) != 0
    }

    /// Read **BRK** `[0:0]`
    ///
    /// Send break. If this bit is set to 1, a low-level is continually output on the UARTTXD output, after completing transmission of the current character. For the proper execution of the break command, the software must set this bit for at least two complete frames. For normal use, this bit must be cleared to 0.
    #[inline(always)]
    pub const fn brk(&self) -> bool {
        (self.bits & UartlcrH::BRK_MASK) != 0
    }

    /// Set **SPS** `[7:7]`
    ///
    /// Stick parity select. 0 = stick parity is disabled 1 = either: * if the EPS bit is 0 then the parity bit is transmitted and checked as a 1 * if the EPS bit is 1 then the parity bit is transmitted and checked as a 0. This bit has no effect when the PEN bit disables parity checking and generation.
    #[inline(always)]
    pub const fn with_sps(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | UartlcrH::SPS_MASK
            } else {
                self.bits & !UartlcrH::SPS_MASK
            },
        }
    }

    /// Set **WLEN** `[6:5]`
    ///
    /// Word length. These bits indicate the number of data bits transmitted or received in a frame as follows: b11 = 8 bits b10 = 7 bits b01 = 6 bits b00 = 5 bits.
    #[inline(always)]
    pub const fn with_wlen(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !UartlcrH::WLEN_MASK)
                | ((value << UartlcrH::WLEN_OFFSET) & UartlcrH::WLEN_MASK),
        }
    }

    /// Set **FEN** `[4:4]`
    ///
    /// Enable FIFOs: 0 = FIFOs are disabled (character mode) that is, the FIFOs become 1-byte-deep holding registers 1 = transmit and receive FIFO buffers are enabled (FIFO mode).
    #[inline(always)]
    pub const fn with_fen(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | UartlcrH::FEN_MASK
            } else {
                self.bits & !UartlcrH::FEN_MASK
            },
        }
    }

    /// Set **STP2** `[3:3]`
    ///
    /// Two stop bits select. If this bit is set to 1, two stop bits are transmitted at the end of the frame. The receive logic does not check for two stop bits being received.
    #[inline(always)]
    pub const fn with_stp2(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | UartlcrH::STP2_MASK
            } else {
                self.bits & !UartlcrH::STP2_MASK
            },
        }
    }

    /// Set **EPS** `[2:2]`
    ///
    /// Even parity select. Controls the type of parity the UART uses during transmission and reception: 0 = odd parity. The UART generates or checks for an odd number of 1s in the data and parity bits. 1 = even parity. The UART generates or checks for an even number of 1s in the data and parity bits. This bit has no effect when the PEN bit disables parity checking and generation.
    #[inline(always)]
    pub const fn with_eps(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | UartlcrH::EPS_MASK
            } else {
                self.bits & !UartlcrH::EPS_MASK
            },
        }
    }

    /// Set **PEN** `[1:1]`
    ///
    /// Parity enable: 0 = parity is disabled and no parity bit added to the data frame 1 = parity checking and generation is enabled.
    #[inline(always)]
    pub const fn with_pen(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | UartlcrH::PEN_MASK
            } else {
                self.bits & !UartlcrH::PEN_MASK
            },
        }
    }

    /// Set **BRK** `[0:0]`
    ///
    /// Send break. If this bit is set to 1, a low-level is continually output on the UARTTXD output, after completing transmission of the current character. For the proper execution of the break command, the software must set this bit for at least two complete frames. For normal use, this bit must be cleared to 0.
    #[inline(always)]
    pub const fn with_brk(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | UartlcrH::BRK_MASK
            } else {
                self.bits & !UartlcrH::BRK_MASK
            },
        }
    }
}

impl Default for UartlcrHVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl UartlcrH {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> UartlcrHVal {
        UartlcrHVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: UartlcrHVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(UartlcrHVal) -> UartlcrHVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// UARTCR Register
///
/// Control Register, UARTCR
///
/// Address offset: 0x0030
/// Reset value: 0x00000300
///
/// ## Fields
///
/// - **CTSEN** `[15]` (RW)
///
///   CTS hardware flow control enable. If this bit is set to 1, CTS hardware flow control is enabled. Data is only transmitted when the nUARTCTS signal is asserted.
///
/// - **RTSEN** `[14]` (RW)
///
///   RTS hardware flow control enable. If this bit is set to 1, RTS hardware flow control is enabled. Data is only requested when there is space in the receive FIFO for it to be received.
///
/// - **OUT2** `[13]` (RW)
///
///   This bit is the complement of the UART Out2 (nUARTOut2) modem status output. That is, when the bit is programmed to a 1, the output is 0. For DTE this can be used as Ring Indicator (RI).
///
/// - **OUT1** `[12]` (RW)
///
///   This bit is the complement of the UART Out1 (nUARTOut1) modem status output. That is, when the bit is programmed to a 1 the output is 0. For DTE this can be used as Data Carrier Detect (DCD).
///
/// - **RTS** `[11]` (RW)
///
///   Request to send. This bit is the complement of the UART request to send, nUARTRTS, modem status output. That is, when the bit is programmed to a 1 then nUARTRTS is LOW.
///
/// - **DTR** `[10]` (RW)
///
///   Data transmit ready. This bit is the complement of the UART data transmit ready, nUARTDTR, modem status output. That is, when the bit is programmed to a 1 then nUARTDTR is LOW.
///
/// - **RXE** `[9]` (RW)
///
///   Receive enable. If this bit is set to 1, the receive section of the UART is enabled. Data reception occurs for either UART signals or SIR signals depending on the setting of the SIREN bit. When the UART is disabled in the middle of reception, it completes the current character before stopping.
///
/// - **TXE** `[8]` (RW)
///
///   Transmit enable. If this bit is set to 1, the transmit section of the UART is enabled. Data transmission occurs for either UART signals, or SIR signals depending on the setting of the SIREN bit. When the UART is disabled in the middle of transmission, it completes the current character before stopping.
///
/// - **LBE** `[7]` (RW)
///
///   Loopback enable. If this bit is set to 1 and the SIREN bit is set to 1 and the SIRTEST bit in the Test Control Register, UARTTCR is set to 1, then the nSIROUT path is inverted, and fed through to the SIRIN path. The SIRTEST bit in the test register must be set to 1 to override the normal half-duplex SIR operation. This must be the requirement for accessing the test registers during normal operation, and SIRTEST must be cleared to 0 when loopback testing is finished. This feature reduces the amount of external coupling required during system test. If this bit is set to 1, and the SIRTEST bit is set to 0, the UARTTXD path is fed through to the UARTRXD path. In either SIR mode or UART mode, when this bit is set, the modem outputs are also fed through to the modem inputs. This bit is cleared to 0 on reset, to disable loopback.
///
/// - **SIRLP** `[2]` (RW)
///
///   SIR low-power IrDA mode. This bit selects the IrDA encoding mode. If this bit is cleared to 0, low-level bits are transmitted as an active high pulse with a width of 3 / 16th of the bit period. If this bit is set to 1, low-level bits are transmitted with a pulse width that is 3 times the period of the IrLPBaud16 input signal, regardless of the selected bit rate. Setting this bit uses less power, but might reduce transmission distances.
///
/// - **SIREN** `[1]` (RW)
///
///   SIR enable: 0 = IrDA SIR ENDEC is disabled. nSIROUT remains LOW (no light pulse generated), and signal transitions on SIRIN have no effect. 1 = IrDA SIR ENDEC is enabled. Data is transmitted and received on nSIROUT and SIRIN. UARTTXD remains HIGH, in the marking state. Signal transitions on UARTRXD or modem status inputs have no effect. This bit has no effect if the UARTEN bit disables the UART.
///
/// - **UARTEN** `[0]` (RW)
///
///   UART enable: 0 = UART is disabled. If the UART is disabled in the middle of transmission or reception, it completes the current character before stopping. 1 = the UART is enabled. Data transmission and reception occurs for either UART signals or SIR signals depending on the setting of the SIREN bit.
#[repr(transparent)]
pub struct Uartcr {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Uartcr {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Uartcr {
    // ======== Field Constants ========

    /// **CTSEN**
    ///
    /// CTS hardware flow control enable. If this bit is set to 1, CTS hardware flow control is enabled. Data is only transmitted when the nUARTCTS signal is asserted.
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const CTSEN_OFFSET: u32 = 15;
    pub const CTSEN_MASK: u32 = 0x00008000;
    pub const CTSEN: u32 = 1 << 15;

    /// **RTSEN**
    ///
    /// RTS hardware flow control enable. If this bit is set to 1, RTS hardware flow control is enabled. Data is only requested when there is space in the receive FIFO for it to be received.
    ///
    /// Bits `[14:14]`, mask `0x00004000`
    pub const RTSEN_OFFSET: u32 = 14;
    pub const RTSEN_MASK: u32 = 0x00004000;
    pub const RTSEN: u32 = 1 << 14;

    /// **OUT2**
    ///
    /// This bit is the complement of the UART Out2 (nUARTOut2) modem status output. That is, when the bit is programmed to a 1, the output is 0. For DTE this can be used as Ring Indicator (RI).
    ///
    /// Bits `[13:13]`, mask `0x00002000`
    pub const OUT2_OFFSET: u32 = 13;
    pub const OUT2_MASK: u32 = 0x00002000;
    pub const OUT2: u32 = 1 << 13;

    /// **OUT1**
    ///
    /// This bit is the complement of the UART Out1 (nUARTOut1) modem status output. That is, when the bit is programmed to a 1 the output is 0. For DTE this can be used as Data Carrier Detect (DCD).
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const OUT1_OFFSET: u32 = 12;
    pub const OUT1_MASK: u32 = 0x00001000;
    pub const OUT1: u32 = 1 << 12;

    /// **RTS**
    ///
    /// Request to send. This bit is the complement of the UART request to send, nUARTRTS, modem status output. That is, when the bit is programmed to a 1 then nUARTRTS is LOW.
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const RTS_OFFSET: u32 = 11;
    pub const RTS_MASK: u32 = 0x00000800;
    pub const RTS: u32 = 1 << 11;

    /// **DTR**
    ///
    /// Data transmit ready. This bit is the complement of the UART data transmit ready, nUARTDTR, modem status output. That is, when the bit is programmed to a 1 then nUARTDTR is LOW.
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const DTR_OFFSET: u32 = 10;
    pub const DTR_MASK: u32 = 0x00000400;
    pub const DTR: u32 = 1 << 10;

    /// **RXE**
    ///
    /// Receive enable. If this bit is set to 1, the receive section of the UART is enabled. Data reception occurs for either UART signals or SIR signals depending on the setting of the SIREN bit. When the UART is disabled in the middle of reception, it completes the current character before stopping.
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const RXE_OFFSET: u32 = 9;
    pub const RXE_MASK: u32 = 0x00000200;
    pub const RXE: u32 = 1 << 9;

    /// **TXE**
    ///
    /// Transmit enable. If this bit is set to 1, the transmit section of the UART is enabled. Data transmission occurs for either UART signals, or SIR signals depending on the setting of the SIREN bit. When the UART is disabled in the middle of transmission, it completes the current character before stopping.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const TXE_OFFSET: u32 = 8;
    pub const TXE_MASK: u32 = 0x00000100;
    pub const TXE: u32 = 1 << 8;

    /// **LBE**
    ///
    /// Loopback enable. If this bit is set to 1 and the SIREN bit is set to 1 and the SIRTEST bit in the Test Control Register, UARTTCR is set to 1, then the nSIROUT path is inverted, and fed through to the SIRIN path. The SIRTEST bit in the test register must be set to 1 to override the normal half-duplex SIR operation. This must be the requirement for accessing the test registers during normal operation, and SIRTEST must be cleared to 0 when loopback testing is finished. This feature reduces the amount of external coupling required during system test. If this bit is set to 1, and the SIRTEST bit is set to 0, the UARTTXD path is fed through to the UARTRXD path. In either SIR mode or UART mode, when this bit is set, the modem outputs are also fed through to the modem inputs. This bit is cleared to 0 on reset, to disable loopback.
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const LBE_OFFSET: u32 = 7;
    pub const LBE_MASK: u32 = 0x00000080;
    pub const LBE: u32 = 1 << 7;

    /// **SIRLP**
    ///
    /// SIR low-power IrDA mode. This bit selects the IrDA encoding mode. If this bit is cleared to 0, low-level bits are transmitted as an active high pulse with a width of 3 / 16th of the bit period. If this bit is set to 1, low-level bits are transmitted with a pulse width that is 3 times the period of the IrLPBaud16 input signal, regardless of the selected bit rate. Setting this bit uses less power, but might reduce transmission distances.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const SIRLP_OFFSET: u32 = 2;
    pub const SIRLP_MASK: u32 = 0x00000004;
    pub const SIRLP: u32 = 1 << 2;

    /// **SIREN**
    ///
    /// SIR enable: 0 = IrDA SIR ENDEC is disabled. nSIROUT remains LOW (no light pulse generated), and signal transitions on SIRIN have no effect. 1 = IrDA SIR ENDEC is enabled. Data is transmitted and received on nSIROUT and SIRIN. UARTTXD remains HIGH, in the marking state. Signal transitions on UARTRXD or modem status inputs have no effect. This bit has no effect if the UARTEN bit disables the UART.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const SIREN_OFFSET: u32 = 1;
    pub const SIREN_MASK: u32 = 0x00000002;
    pub const SIREN: u32 = 1 << 1;

    /// **UARTEN**
    ///
    /// UART enable: 0 = UART is disabled. If the UART is disabled in the middle of transmission or reception, it completes the current character before stopping. 1 = the UART is enabled. Data transmission and reception occurs for either UART signals or SIR signals depending on the setting of the SIREN bit.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const UARTEN_OFFSET: u32 = 0;
    pub const UARTEN_MASK: u32 = 0x00000001;
    pub const UARTEN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **CTSEN** `[15]` (RW)
    ///
    /// CTS hardware flow control enable. If this bit is set to 1, CTS hardware flow control is enabled. Data is only transmitted when the nUARTCTS signal is asserted.
    #[inline(always)]
    pub fn ctsen(&self) -> bool {
        (self.value.read_bits() & Self::CTSEN_MASK) != 0
    }

    /// Set CTSEN bit atomically (RW)
    #[inline(always)]
    pub fn set_ctsen(&self) {
        self.set_bits(Self::CTSEN_MASK);
    }

    /// Clear CTSEN bit atomically (RW)
    #[inline(always)]
    pub fn clr_ctsen(&self) {
        self.clr_bits(Self::CTSEN_MASK);
    }

    /// **RTSEN** `[14]` (RW)
    ///
    /// RTS hardware flow control enable. If this bit is set to 1, RTS hardware flow control is enabled. Data is only requested when there is space in the receive FIFO for it to be received.
    #[inline(always)]
    pub fn rtsen(&self) -> bool {
        (self.value.read_bits() & Self::RTSEN_MASK) != 0
    }

    /// Set RTSEN bit atomically (RW)
    #[inline(always)]
    pub fn set_rtsen(&self) {
        self.set_bits(Self::RTSEN_MASK);
    }

    /// Clear RTSEN bit atomically (RW)
    #[inline(always)]
    pub fn clr_rtsen(&self) {
        self.clr_bits(Self::RTSEN_MASK);
    }

    /// **OUT2** `[13]` (RW)
    ///
    /// This bit is the complement of the UART Out2 (nUARTOut2) modem status output. That is, when the bit is programmed to a 1, the output is 0. For DTE this can be used as Ring Indicator (RI).
    #[inline(always)]
    pub fn out2(&self) -> bool {
        (self.value.read_bits() & Self::OUT2_MASK) != 0
    }

    /// Set OUT2 bit atomically (RW)
    #[inline(always)]
    pub fn set_out2(&self) {
        self.set_bits(Self::OUT2_MASK);
    }

    /// Clear OUT2 bit atomically (RW)
    #[inline(always)]
    pub fn clr_out2(&self) {
        self.clr_bits(Self::OUT2_MASK);
    }

    /// **OUT1** `[12]` (RW)
    ///
    /// This bit is the complement of the UART Out1 (nUARTOut1) modem status output. That is, when the bit is programmed to a 1 the output is 0. For DTE this can be used as Data Carrier Detect (DCD).
    #[inline(always)]
    pub fn out1(&self) -> bool {
        (self.value.read_bits() & Self::OUT1_MASK) != 0
    }

    /// Set OUT1 bit atomically (RW)
    #[inline(always)]
    pub fn set_out1(&self) {
        self.set_bits(Self::OUT1_MASK);
    }

    /// Clear OUT1 bit atomically (RW)
    #[inline(always)]
    pub fn clr_out1(&self) {
        self.clr_bits(Self::OUT1_MASK);
    }

    /// **RTS** `[11]` (RW)
    ///
    /// Request to send. This bit is the complement of the UART request to send, nUARTRTS, modem status output. That is, when the bit is programmed to a 1 then nUARTRTS is LOW.
    #[inline(always)]
    pub fn rts(&self) -> bool {
        (self.value.read_bits() & Self::RTS_MASK) != 0
    }

    /// Set RTS bit atomically (RW)
    #[inline(always)]
    pub fn set_rts(&self) {
        self.set_bits(Self::RTS_MASK);
    }

    /// Clear RTS bit atomically (RW)
    #[inline(always)]
    pub fn clr_rts(&self) {
        self.clr_bits(Self::RTS_MASK);
    }

    /// **DTR** `[10]` (RW)
    ///
    /// Data transmit ready. This bit is the complement of the UART data transmit ready, nUARTDTR, modem status output. That is, when the bit is programmed to a 1 then nUARTDTR is LOW.
    #[inline(always)]
    pub fn dtr(&self) -> bool {
        (self.value.read_bits() & Self::DTR_MASK) != 0
    }

    /// Set DTR bit atomically (RW)
    #[inline(always)]
    pub fn set_dtr(&self) {
        self.set_bits(Self::DTR_MASK);
    }

    /// Clear DTR bit atomically (RW)
    #[inline(always)]
    pub fn clr_dtr(&self) {
        self.clr_bits(Self::DTR_MASK);
    }

    /// **RXE** `[9]` (RW)
    ///
    /// Receive enable. If this bit is set to 1, the receive section of the UART is enabled. Data reception occurs for either UART signals or SIR signals depending on the setting of the SIREN bit. When the UART is disabled in the middle of reception, it completes the current character before stopping.
    #[inline(always)]
    pub fn rxe(&self) -> bool {
        (self.value.read_bits() & Self::RXE_MASK) != 0
    }

    /// Set RXE bit atomically (RW)
    #[inline(always)]
    pub fn set_rxe(&self) {
        self.set_bits(Self::RXE_MASK);
    }

    /// Clear RXE bit atomically (RW)
    #[inline(always)]
    pub fn clr_rxe(&self) {
        self.clr_bits(Self::RXE_MASK);
    }

    /// **TXE** `[8]` (RW)
    ///
    /// Transmit enable. If this bit is set to 1, the transmit section of the UART is enabled. Data transmission occurs for either UART signals, or SIR signals depending on the setting of the SIREN bit. When the UART is disabled in the middle of transmission, it completes the current character before stopping.
    #[inline(always)]
    pub fn txe(&self) -> bool {
        (self.value.read_bits() & Self::TXE_MASK) != 0
    }

    /// Set TXE bit atomically (RW)
    #[inline(always)]
    pub fn set_txe(&self) {
        self.set_bits(Self::TXE_MASK);
    }

    /// Clear TXE bit atomically (RW)
    #[inline(always)]
    pub fn clr_txe(&self) {
        self.clr_bits(Self::TXE_MASK);
    }

    /// **LBE** `[7]` (RW)
    ///
    /// Loopback enable. If this bit is set to 1 and the SIREN bit is set to 1 and the SIRTEST bit in the Test Control Register, UARTTCR is set to 1, then the nSIROUT path is inverted, and fed through to the SIRIN path. The SIRTEST bit in the test register must be set to 1 to override the normal half-duplex SIR operation. This must be the requirement for accessing the test registers during normal operation, and SIRTEST must be cleared to 0 when loopback testing is finished. This feature reduces the amount of external coupling required during system test. If this bit is set to 1, and the SIRTEST bit is set to 0, the UARTTXD path is fed through to the UARTRXD path. In either SIR mode or UART mode, when this bit is set, the modem outputs are also fed through to the modem inputs. This bit is cleared to 0 on reset, to disable loopback.
    #[inline(always)]
    pub fn lbe(&self) -> bool {
        (self.value.read_bits() & Self::LBE_MASK) != 0
    }

    /// Set LBE bit atomically (RW)
    #[inline(always)]
    pub fn set_lbe(&self) {
        self.set_bits(Self::LBE_MASK);
    }

    /// Clear LBE bit atomically (RW)
    #[inline(always)]
    pub fn clr_lbe(&self) {
        self.clr_bits(Self::LBE_MASK);
    }

    /// **SIRLP** `[2]` (RW)
    ///
    /// SIR low-power IrDA mode. This bit selects the IrDA encoding mode. If this bit is cleared to 0, low-level bits are transmitted as an active high pulse with a width of 3 / 16th of the bit period. If this bit is set to 1, low-level bits are transmitted with a pulse width that is 3 times the period of the IrLPBaud16 input signal, regardless of the selected bit rate. Setting this bit uses less power, but might reduce transmission distances.
    #[inline(always)]
    pub fn sirlp(&self) -> bool {
        (self.value.read_bits() & Self::SIRLP_MASK) != 0
    }

    /// Set SIRLP bit atomically (RW)
    #[inline(always)]
    pub fn set_sirlp(&self) {
        self.set_bits(Self::SIRLP_MASK);
    }

    /// Clear SIRLP bit atomically (RW)
    #[inline(always)]
    pub fn clr_sirlp(&self) {
        self.clr_bits(Self::SIRLP_MASK);
    }

    /// **SIREN** `[1]` (RW)
    ///
    /// SIR enable: 0 = IrDA SIR ENDEC is disabled. nSIROUT remains LOW (no light pulse generated), and signal transitions on SIRIN have no effect. 1 = IrDA SIR ENDEC is enabled. Data is transmitted and received on nSIROUT and SIRIN. UARTTXD remains HIGH, in the marking state. Signal transitions on UARTRXD or modem status inputs have no effect. This bit has no effect if the UARTEN bit disables the UART.
    #[inline(always)]
    pub fn siren(&self) -> bool {
        (self.value.read_bits() & Self::SIREN_MASK) != 0
    }

    /// Set SIREN bit atomically (RW)
    #[inline(always)]
    pub fn set_siren(&self) {
        self.set_bits(Self::SIREN_MASK);
    }

    /// Clear SIREN bit atomically (RW)
    #[inline(always)]
    pub fn clr_siren(&self) {
        self.clr_bits(Self::SIREN_MASK);
    }

    /// **UARTEN** `[0]` (RW)
    ///
    /// UART enable: 0 = UART is disabled. If the UART is disabled in the middle of transmission or reception, it completes the current character before stopping. 1 = the UART is enabled. Data transmission and reception occurs for either UART signals or SIR signals depending on the setting of the SIREN bit.
    #[inline(always)]
    pub fn uarten(&self) -> bool {
        (self.value.read_bits() & Self::UARTEN_MASK) != 0
    }

    /// Set UARTEN bit atomically (RW)
    #[inline(always)]
    pub fn set_uarten(&self) {
        self.set_bits(Self::UARTEN_MASK);
    }

    /// Clear UARTEN bit atomically (RW)
    #[inline(always)]
    pub fn clr_uarten(&self) {
        self.clr_bits(Self::UARTEN_MASK);
    }
}

/// UARTCR value type for batch read/modify/write operations
///
/// Use `uartcr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `uartcr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct UartcrVal {
    bits: u32,
}

impl UartcrVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x00000300 }
    }

    /// Read **CTSEN** `[15:15]`
    ///
    /// CTS hardware flow control enable. If this bit is set to 1, CTS hardware flow control is enabled. Data is only transmitted when the nUARTCTS signal is asserted.
    #[inline(always)]
    pub const fn ctsen(&self) -> bool {
        (self.bits & Uartcr::CTSEN_MASK) != 0
    }

    /// Read **RTSEN** `[14:14]`
    ///
    /// RTS hardware flow control enable. If this bit is set to 1, RTS hardware flow control is enabled. Data is only requested when there is space in the receive FIFO for it to be received.
    #[inline(always)]
    pub const fn rtsen(&self) -> bool {
        (self.bits & Uartcr::RTSEN_MASK) != 0
    }

    /// Read **OUT2** `[13:13]`
    ///
    /// This bit is the complement of the UART Out2 (nUARTOut2) modem status output. That is, when the bit is programmed to a 1, the output is 0. For DTE this can be used as Ring Indicator (RI).
    #[inline(always)]
    pub const fn out2(&self) -> bool {
        (self.bits & Uartcr::OUT2_MASK) != 0
    }

    /// Read **OUT1** `[12:12]`
    ///
    /// This bit is the complement of the UART Out1 (nUARTOut1) modem status output. That is, when the bit is programmed to a 1 the output is 0. For DTE this can be used as Data Carrier Detect (DCD).
    #[inline(always)]
    pub const fn out1(&self) -> bool {
        (self.bits & Uartcr::OUT1_MASK) != 0
    }

    /// Read **RTS** `[11:11]`
    ///
    /// Request to send. This bit is the complement of the UART request to send, nUARTRTS, modem status output. That is, when the bit is programmed to a 1 then nUARTRTS is LOW.
    #[inline(always)]
    pub const fn rts(&self) -> bool {
        (self.bits & Uartcr::RTS_MASK) != 0
    }

    /// Read **DTR** `[10:10]`
    ///
    /// Data transmit ready. This bit is the complement of the UART data transmit ready, nUARTDTR, modem status output. That is, when the bit is programmed to a 1 then nUARTDTR is LOW.
    #[inline(always)]
    pub const fn dtr(&self) -> bool {
        (self.bits & Uartcr::DTR_MASK) != 0
    }

    /// Read **RXE** `[9:9]`
    ///
    /// Receive enable. If this bit is set to 1, the receive section of the UART is enabled. Data reception occurs for either UART signals or SIR signals depending on the setting of the SIREN bit. When the UART is disabled in the middle of reception, it completes the current character before stopping.
    #[inline(always)]
    pub const fn rxe(&self) -> bool {
        (self.bits & Uartcr::RXE_MASK) != 0
    }

    /// Read **TXE** `[8:8]`
    ///
    /// Transmit enable. If this bit is set to 1, the transmit section of the UART is enabled. Data transmission occurs for either UART signals, or SIR signals depending on the setting of the SIREN bit. When the UART is disabled in the middle of transmission, it completes the current character before stopping.
    #[inline(always)]
    pub const fn txe(&self) -> bool {
        (self.bits & Uartcr::TXE_MASK) != 0
    }

    /// Read **LBE** `[7:7]`
    ///
    /// Loopback enable. If this bit is set to 1 and the SIREN bit is set to 1 and the SIRTEST bit in the Test Control Register, UARTTCR is set to 1, then the nSIROUT path is inverted, and fed through to the SIRIN path. The SIRTEST bit in the test register must be set to 1 to override the normal half-duplex SIR operation. This must be the requirement for accessing the test registers during normal operation, and SIRTEST must be cleared to 0 when loopback testing is finished. This feature reduces the amount of external coupling required during system test. If this bit is set to 1, and the SIRTEST bit is set to 0, the UARTTXD path is fed through to the UARTRXD path. In either SIR mode or UART mode, when this bit is set, the modem outputs are also fed through to the modem inputs. This bit is cleared to 0 on reset, to disable loopback.
    #[inline(always)]
    pub const fn lbe(&self) -> bool {
        (self.bits & Uartcr::LBE_MASK) != 0
    }

    /// Read **SIRLP** `[2:2]`
    ///
    /// SIR low-power IrDA mode. This bit selects the IrDA encoding mode. If this bit is cleared to 0, low-level bits are transmitted as an active high pulse with a width of 3 / 16th of the bit period. If this bit is set to 1, low-level bits are transmitted with a pulse width that is 3 times the period of the IrLPBaud16 input signal, regardless of the selected bit rate. Setting this bit uses less power, but might reduce transmission distances.
    #[inline(always)]
    pub const fn sirlp(&self) -> bool {
        (self.bits & Uartcr::SIRLP_MASK) != 0
    }

    /// Read **SIREN** `[1:1]`
    ///
    /// SIR enable: 0 = IrDA SIR ENDEC is disabled. nSIROUT remains LOW (no light pulse generated), and signal transitions on SIRIN have no effect. 1 = IrDA SIR ENDEC is enabled. Data is transmitted and received on nSIROUT and SIRIN. UARTTXD remains HIGH, in the marking state. Signal transitions on UARTRXD or modem status inputs have no effect. This bit has no effect if the UARTEN bit disables the UART.
    #[inline(always)]
    pub const fn siren(&self) -> bool {
        (self.bits & Uartcr::SIREN_MASK) != 0
    }

    /// Read **UARTEN** `[0:0]`
    ///
    /// UART enable: 0 = UART is disabled. If the UART is disabled in the middle of transmission or reception, it completes the current character before stopping. 1 = the UART is enabled. Data transmission and reception occurs for either UART signals or SIR signals depending on the setting of the SIREN bit.
    #[inline(always)]
    pub const fn uarten(&self) -> bool {
        (self.bits & Uartcr::UARTEN_MASK) != 0
    }

    /// Set **CTSEN** `[15:15]`
    ///
    /// CTS hardware flow control enable. If this bit is set to 1, CTS hardware flow control is enabled. Data is only transmitted when the nUARTCTS signal is asserted.
    #[inline(always)]
    pub const fn with_ctsen(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Uartcr::CTSEN_MASK
            } else {
                self.bits & !Uartcr::CTSEN_MASK
            },
        }
    }

    /// Set **RTSEN** `[14:14]`
    ///
    /// RTS hardware flow control enable. If this bit is set to 1, RTS hardware flow control is enabled. Data is only requested when there is space in the receive FIFO for it to be received.
    #[inline(always)]
    pub const fn with_rtsen(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Uartcr::RTSEN_MASK
            } else {
                self.bits & !Uartcr::RTSEN_MASK
            },
        }
    }

    /// Set **OUT2** `[13:13]`
    ///
    /// This bit is the complement of the UART Out2 (nUARTOut2) modem status output. That is, when the bit is programmed to a 1, the output is 0. For DTE this can be used as Ring Indicator (RI).
    #[inline(always)]
    pub const fn with_out2(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Uartcr::OUT2_MASK
            } else {
                self.bits & !Uartcr::OUT2_MASK
            },
        }
    }

    /// Set **OUT1** `[12:12]`
    ///
    /// This bit is the complement of the UART Out1 (nUARTOut1) modem status output. That is, when the bit is programmed to a 1 the output is 0. For DTE this can be used as Data Carrier Detect (DCD).
    #[inline(always)]
    pub const fn with_out1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Uartcr::OUT1_MASK
            } else {
                self.bits & !Uartcr::OUT1_MASK
            },
        }
    }

    /// Set **RTS** `[11:11]`
    ///
    /// Request to send. This bit is the complement of the UART request to send, nUARTRTS, modem status output. That is, when the bit is programmed to a 1 then nUARTRTS is LOW.
    #[inline(always)]
    pub const fn with_rts(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Uartcr::RTS_MASK
            } else {
                self.bits & !Uartcr::RTS_MASK
            },
        }
    }

    /// Set **DTR** `[10:10]`
    ///
    /// Data transmit ready. This bit is the complement of the UART data transmit ready, nUARTDTR, modem status output. That is, when the bit is programmed to a 1 then nUARTDTR is LOW.
    #[inline(always)]
    pub const fn with_dtr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Uartcr::DTR_MASK
            } else {
                self.bits & !Uartcr::DTR_MASK
            },
        }
    }

    /// Set **RXE** `[9:9]`
    ///
    /// Receive enable. If this bit is set to 1, the receive section of the UART is enabled. Data reception occurs for either UART signals or SIR signals depending on the setting of the SIREN bit. When the UART is disabled in the middle of reception, it completes the current character before stopping.
    #[inline(always)]
    pub const fn with_rxe(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Uartcr::RXE_MASK
            } else {
                self.bits & !Uartcr::RXE_MASK
            },
        }
    }

    /// Set **TXE** `[8:8]`
    ///
    /// Transmit enable. If this bit is set to 1, the transmit section of the UART is enabled. Data transmission occurs for either UART signals, or SIR signals depending on the setting of the SIREN bit. When the UART is disabled in the middle of transmission, it completes the current character before stopping.
    #[inline(always)]
    pub const fn with_txe(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Uartcr::TXE_MASK
            } else {
                self.bits & !Uartcr::TXE_MASK
            },
        }
    }

    /// Set **LBE** `[7:7]`
    ///
    /// Loopback enable. If this bit is set to 1 and the SIREN bit is set to 1 and the SIRTEST bit in the Test Control Register, UARTTCR is set to 1, then the nSIROUT path is inverted, and fed through to the SIRIN path. The SIRTEST bit in the test register must be set to 1 to override the normal half-duplex SIR operation. This must be the requirement for accessing the test registers during normal operation, and SIRTEST must be cleared to 0 when loopback testing is finished. This feature reduces the amount of external coupling required during system test. If this bit is set to 1, and the SIRTEST bit is set to 0, the UARTTXD path is fed through to the UARTRXD path. In either SIR mode or UART mode, when this bit is set, the modem outputs are also fed through to the modem inputs. This bit is cleared to 0 on reset, to disable loopback.
    #[inline(always)]
    pub const fn with_lbe(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Uartcr::LBE_MASK
            } else {
                self.bits & !Uartcr::LBE_MASK
            },
        }
    }

    /// Set **SIRLP** `[2:2]`
    ///
    /// SIR low-power IrDA mode. This bit selects the IrDA encoding mode. If this bit is cleared to 0, low-level bits are transmitted as an active high pulse with a width of 3 / 16th of the bit period. If this bit is set to 1, low-level bits are transmitted with a pulse width that is 3 times the period of the IrLPBaud16 input signal, regardless of the selected bit rate. Setting this bit uses less power, but might reduce transmission distances.
    #[inline(always)]
    pub const fn with_sirlp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Uartcr::SIRLP_MASK
            } else {
                self.bits & !Uartcr::SIRLP_MASK
            },
        }
    }

    /// Set **SIREN** `[1:1]`
    ///
    /// SIR enable: 0 = IrDA SIR ENDEC is disabled. nSIROUT remains LOW (no light pulse generated), and signal transitions on SIRIN have no effect. 1 = IrDA SIR ENDEC is enabled. Data is transmitted and received on nSIROUT and SIRIN. UARTTXD remains HIGH, in the marking state. Signal transitions on UARTRXD or modem status inputs have no effect. This bit has no effect if the UARTEN bit disables the UART.
    #[inline(always)]
    pub const fn with_siren(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Uartcr::SIREN_MASK
            } else {
                self.bits & !Uartcr::SIREN_MASK
            },
        }
    }

    /// Set **UARTEN** `[0:0]`
    ///
    /// UART enable: 0 = UART is disabled. If the UART is disabled in the middle of transmission or reception, it completes the current character before stopping. 1 = the UART is enabled. Data transmission and reception occurs for either UART signals or SIR signals depending on the setting of the SIREN bit.
    #[inline(always)]
    pub const fn with_uarten(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Uartcr::UARTEN_MASK
            } else {
                self.bits & !Uartcr::UARTEN_MASK
            },
        }
    }
}

impl Default for UartcrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Uartcr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> UartcrVal {
        UartcrVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: UartcrVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(UartcrVal) -> UartcrVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// UARTIFLS Register
///
/// Interrupt FIFO Level Select Register, UARTIFLS
///
/// Address offset: 0x0034
/// Reset value: 0x00000012
///
/// ## Fields
///
/// - **RXIFLSEL** `[5:3]` (RW)
///
///   Receive interrupt FIFO level select. The trigger points for the receive interrupt are as follows: b000 = Receive FIFO becomes >= 1 / 8 full b001 = Receive FIFO becomes >= 1 / 4 full b010 = Receive FIFO becomes >= 1 / 2 full b011 = Receive FIFO becomes >= 3 / 4 full b100 = Receive FIFO becomes >= 7 / 8 full b101-b111 = reserved.
///
/// - **TXIFLSEL** `[2:0]` (RW)
///
///   Transmit interrupt FIFO level select. The trigger points for the transmit interrupt are as follows: b000 = Transmit FIFO becomes <= 1 / 8 full b001 = Transmit FIFO becomes <= 1 / 4 full b010 = Transmit FIFO becomes <= 1 / 2 full b011 = Transmit FIFO becomes <= 3 / 4 full b100 = Transmit FIFO becomes <= 7 / 8 full b101-b111 = reserved.
#[repr(transparent)]
pub struct Uartifls {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Uartifls {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Uartifls {
    // ======== Field Constants ========

    /// **RXIFLSEL**
    ///
    /// Receive interrupt FIFO level select. The trigger points for the receive interrupt are as follows: b000 = Receive FIFO becomes >= 1 / 8 full b001 = Receive FIFO becomes >= 1 / 4 full b010 = Receive FIFO becomes >= 1 / 2 full b011 = Receive FIFO becomes >= 3 / 4 full b100 = Receive FIFO becomes >= 7 / 8 full b101-b111 = reserved.
    ///
    /// Bits `[5:3]`, mask `0x00000038`
    pub const RXIFLSEL_OFFSET: u32 = 3;
    pub const RXIFLSEL_MASK: u32 = 0x00000038;

    /// **TXIFLSEL**
    ///
    /// Transmit interrupt FIFO level select. The trigger points for the transmit interrupt are as follows: b000 = Transmit FIFO becomes <= 1 / 8 full b001 = Transmit FIFO becomes <= 1 / 4 full b010 = Transmit FIFO becomes <= 1 / 2 full b011 = Transmit FIFO becomes <= 3 / 4 full b100 = Transmit FIFO becomes <= 7 / 8 full b101-b111 = reserved.
    ///
    /// Bits `[2:0]`, mask `0x00000007`
    pub const TXIFLSEL_OFFSET: u32 = 0;
    pub const TXIFLSEL_MASK: u32 = 0x00000007;

    // ======== Field Accessors ========

    /// **RXIFLSEL** `[5:3]` (RW)
    ///
    /// Receive interrupt FIFO level select. The trigger points for the receive interrupt are as follows: b000 = Receive FIFO becomes >= 1 / 8 full b001 = Receive FIFO becomes >= 1 / 4 full b010 = Receive FIFO becomes >= 1 / 2 full b011 = Receive FIFO becomes >= 3 / 4 full b100 = Receive FIFO becomes >= 7 / 8 full b101-b111 = reserved.
    #[inline(always)]
    pub fn rxiflsel(&self) -> u32 {
        (self.value.read_bits() & Self::RXIFLSEL_MASK) >> Self::RXIFLSEL_OFFSET
    }

    /// Write **RXIFLSEL** `[5:3]` (RW)
    ///
    /// Receive interrupt FIFO level select. The trigger points for the receive interrupt are as follows: b000 = Receive FIFO becomes >= 1 / 8 full b001 = Receive FIFO becomes >= 1 / 4 full b010 = Receive FIFO becomes >= 1 / 2 full b011 = Receive FIFO becomes >= 3 / 4 full b100 = Receive FIFO becomes >= 7 / 8 full b101-b111 = reserved.
    #[inline(always)]
    pub fn write_rxiflsel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::RXIFLSEL_MASK) | ((value << Self::RXIFLSEL_OFFSET) & Self::RXIFLSEL_MASK)
        });
    }

    /// **TXIFLSEL** `[2:0]` (RW)
    ///
    /// Transmit interrupt FIFO level select. The trigger points for the transmit interrupt are as follows: b000 = Transmit FIFO becomes <= 1 / 8 full b001 = Transmit FIFO becomes <= 1 / 4 full b010 = Transmit FIFO becomes <= 1 / 2 full b011 = Transmit FIFO becomes <= 3 / 4 full b100 = Transmit FIFO becomes <= 7 / 8 full b101-b111 = reserved.
    #[inline(always)]
    pub fn txiflsel(&self) -> u32 {
        (self.value.read_bits() & Self::TXIFLSEL_MASK) >> Self::TXIFLSEL_OFFSET
    }

    /// Write **TXIFLSEL** `[2:0]` (RW)
    ///
    /// Transmit interrupt FIFO level select. The trigger points for the transmit interrupt are as follows: b000 = Transmit FIFO becomes <= 1 / 8 full b001 = Transmit FIFO becomes <= 1 / 4 full b010 = Transmit FIFO becomes <= 1 / 2 full b011 = Transmit FIFO becomes <= 3 / 4 full b100 = Transmit FIFO becomes <= 7 / 8 full b101-b111 = reserved.
    #[inline(always)]
    pub fn write_txiflsel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::TXIFLSEL_MASK) | ((value << Self::TXIFLSEL_OFFSET) & Self::TXIFLSEL_MASK)
        });
    }
}

/// UARTIFLS value type for batch read/modify/write operations
///
/// Use `uartifls.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `uartifls.write_val(val)`.
#[derive(Clone, Copy)]
pub struct UartiflsVal {
    bits: u32,
}

impl UartiflsVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x00000012 }
    }

    /// Read **RXIFLSEL** `[5:3]`
    ///
    /// Receive interrupt FIFO level select. The trigger points for the receive interrupt are as follows: b000 = Receive FIFO becomes >= 1 / 8 full b001 = Receive FIFO becomes >= 1 / 4 full b010 = Receive FIFO becomes >= 1 / 2 full b011 = Receive FIFO becomes >= 3 / 4 full b100 = Receive FIFO becomes >= 7 / 8 full b101-b111 = reserved.
    #[inline(always)]
    pub const fn rxiflsel(&self) -> u32 {
        (self.bits & Uartifls::RXIFLSEL_MASK) >> Uartifls::RXIFLSEL_OFFSET
    }

    /// Read **TXIFLSEL** `[2:0]`
    ///
    /// Transmit interrupt FIFO level select. The trigger points for the transmit interrupt are as follows: b000 = Transmit FIFO becomes <= 1 / 8 full b001 = Transmit FIFO becomes <= 1 / 4 full b010 = Transmit FIFO becomes <= 1 / 2 full b011 = Transmit FIFO becomes <= 3 / 4 full b100 = Transmit FIFO becomes <= 7 / 8 full b101-b111 = reserved.
    #[inline(always)]
    pub const fn txiflsel(&self) -> u32 {
        (self.bits & Uartifls::TXIFLSEL_MASK) >> Uartifls::TXIFLSEL_OFFSET
    }

    /// Set **RXIFLSEL** `[5:3]`
    ///
    /// Receive interrupt FIFO level select. The trigger points for the receive interrupt are as follows: b000 = Receive FIFO becomes >= 1 / 8 full b001 = Receive FIFO becomes >= 1 / 4 full b010 = Receive FIFO becomes >= 1 / 2 full b011 = Receive FIFO becomes >= 3 / 4 full b100 = Receive FIFO becomes >= 7 / 8 full b101-b111 = reserved.
    #[inline(always)]
    pub const fn with_rxiflsel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Uartifls::RXIFLSEL_MASK)
                | ((value << Uartifls::RXIFLSEL_OFFSET) & Uartifls::RXIFLSEL_MASK),
        }
    }

    /// Set **TXIFLSEL** `[2:0]`
    ///
    /// Transmit interrupt FIFO level select. The trigger points for the transmit interrupt are as follows: b000 = Transmit FIFO becomes <= 1 / 8 full b001 = Transmit FIFO becomes <= 1 / 4 full b010 = Transmit FIFO becomes <= 1 / 2 full b011 = Transmit FIFO becomes <= 3 / 4 full b100 = Transmit FIFO becomes <= 7 / 8 full b101-b111 = reserved.
    #[inline(always)]
    pub const fn with_txiflsel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Uartifls::TXIFLSEL_MASK)
                | ((value << Uartifls::TXIFLSEL_OFFSET) & Uartifls::TXIFLSEL_MASK),
        }
    }
}

impl Default for UartiflsVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Uartifls {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> UartiflsVal {
        UartiflsVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: UartiflsVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(UartiflsVal) -> UartiflsVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// UARTIMSC Register
///
/// Interrupt Mask Set/Clear Register, UARTIMSC
///
/// Address offset: 0x0038
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **OEIM** `[10]` (RW)
///
///   Overrun error interrupt mask. A read returns the current mask for the UARTOEINTR interrupt. On a write of 1, the mask of the UARTOEINTR interrupt is set. A write of 0 clears the mask.
///
/// - **BEIM** `[9]` (RW)
///
///   Break error interrupt mask. A read returns the current mask for the UARTBEINTR interrupt. On a write of 1, the mask of the UARTBEINTR interrupt is set. A write of 0 clears the mask.
///
/// - **PEIM** `[8]` (RW)
///
///   Parity error interrupt mask. A read returns the current mask for the UARTPEINTR interrupt. On a write of 1, the mask of the UARTPEINTR interrupt is set. A write of 0 clears the mask.
///
/// - **FEIM** `[7]` (RW)
///
///   Framing error interrupt mask. A read returns the current mask for the UARTFEINTR interrupt. On a write of 1, the mask of the UARTFEINTR interrupt is set. A write of 0 clears the mask.
///
/// - **RTIM** `[6]` (RW)
///
///   Receive timeout interrupt mask. A read returns the current mask for the UARTRTINTR interrupt. On a write of 1, the mask of the UARTRTINTR interrupt is set. A write of 0 clears the mask.
///
/// - **TXIM** `[5]` (RW)
///
///   Transmit interrupt mask. A read returns the current mask for the UARTTXINTR interrupt. On a write of 1, the mask of the UARTTXINTR interrupt is set. A write of 0 clears the mask.
///
/// - **RXIM** `[4]` (RW)
///
///   Receive interrupt mask. A read returns the current mask for the UARTRXINTR interrupt. On a write of 1, the mask of the UARTRXINTR interrupt is set. A write of 0 clears the mask.
///
/// - **DSRMIM** `[3]` (RW)
///
///   nUARTDSR modem interrupt mask. A read returns the current mask for the UARTDSRINTR interrupt. On a write of 1, the mask of the UARTDSRINTR interrupt is set. A write of 0 clears the mask.
///
/// - **DCDMIM** `[2]` (RW)
///
///   nUARTDCD modem interrupt mask. A read returns the current mask for the UARTDCDINTR interrupt. On a write of 1, the mask of the UARTDCDINTR interrupt is set. A write of 0 clears the mask.
///
/// - **CTSMIM** `[1]` (RW)
///
///   nUARTCTS modem interrupt mask. A read returns the current mask for the UARTCTSINTR interrupt. On a write of 1, the mask of the UARTCTSINTR interrupt is set. A write of 0 clears the mask.
///
/// - **RIMIM** `[0]` (RW)
///
///   nUARTRI modem interrupt mask. A read returns the current mask for the UARTRIINTR interrupt. On a write of 1, the mask of the UARTRIINTR interrupt is set. A write of 0 clears the mask.
#[repr(transparent)]
pub struct Uartimsc {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Uartimsc {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Uartimsc {
    // ======== Field Constants ========

    /// **OEIM**
    ///
    /// Overrun error interrupt mask. A read returns the current mask for the UARTOEINTR interrupt. On a write of 1, the mask of the UARTOEINTR interrupt is set. A write of 0 clears the mask.
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const OEIM_OFFSET: u32 = 10;
    pub const OEIM_MASK: u32 = 0x00000400;
    pub const OEIM: u32 = 1 << 10;

    /// **BEIM**
    ///
    /// Break error interrupt mask. A read returns the current mask for the UARTBEINTR interrupt. On a write of 1, the mask of the UARTBEINTR interrupt is set. A write of 0 clears the mask.
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const BEIM_OFFSET: u32 = 9;
    pub const BEIM_MASK: u32 = 0x00000200;
    pub const BEIM: u32 = 1 << 9;

    /// **PEIM**
    ///
    /// Parity error interrupt mask. A read returns the current mask for the UARTPEINTR interrupt. On a write of 1, the mask of the UARTPEINTR interrupt is set. A write of 0 clears the mask.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const PEIM_OFFSET: u32 = 8;
    pub const PEIM_MASK: u32 = 0x00000100;
    pub const PEIM: u32 = 1 << 8;

    /// **FEIM**
    ///
    /// Framing error interrupt mask. A read returns the current mask for the UARTFEINTR interrupt. On a write of 1, the mask of the UARTFEINTR interrupt is set. A write of 0 clears the mask.
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const FEIM_OFFSET: u32 = 7;
    pub const FEIM_MASK: u32 = 0x00000080;
    pub const FEIM: u32 = 1 << 7;

    /// **RTIM**
    ///
    /// Receive timeout interrupt mask. A read returns the current mask for the UARTRTINTR interrupt. On a write of 1, the mask of the UARTRTINTR interrupt is set. A write of 0 clears the mask.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const RTIM_OFFSET: u32 = 6;
    pub const RTIM_MASK: u32 = 0x00000040;
    pub const RTIM: u32 = 1 << 6;

    /// **TXIM**
    ///
    /// Transmit interrupt mask. A read returns the current mask for the UARTTXINTR interrupt. On a write of 1, the mask of the UARTTXINTR interrupt is set. A write of 0 clears the mask.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const TXIM_OFFSET: u32 = 5;
    pub const TXIM_MASK: u32 = 0x00000020;
    pub const TXIM: u32 = 1 << 5;

    /// **RXIM**
    ///
    /// Receive interrupt mask. A read returns the current mask for the UARTRXINTR interrupt. On a write of 1, the mask of the UARTRXINTR interrupt is set. A write of 0 clears the mask.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const RXIM_OFFSET: u32 = 4;
    pub const RXIM_MASK: u32 = 0x00000010;
    pub const RXIM: u32 = 1 << 4;

    /// **DSRMIM**
    ///
    /// nUARTDSR modem interrupt mask. A read returns the current mask for the UARTDSRINTR interrupt. On a write of 1, the mask of the UARTDSRINTR interrupt is set. A write of 0 clears the mask.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const DSRMIM_OFFSET: u32 = 3;
    pub const DSRMIM_MASK: u32 = 0x00000008;
    pub const DSRMIM: u32 = 1 << 3;

    /// **DCDMIM**
    ///
    /// nUARTDCD modem interrupt mask. A read returns the current mask for the UARTDCDINTR interrupt. On a write of 1, the mask of the UARTDCDINTR interrupt is set. A write of 0 clears the mask.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const DCDMIM_OFFSET: u32 = 2;
    pub const DCDMIM_MASK: u32 = 0x00000004;
    pub const DCDMIM: u32 = 1 << 2;

    /// **CTSMIM**
    ///
    /// nUARTCTS modem interrupt mask. A read returns the current mask for the UARTCTSINTR interrupt. On a write of 1, the mask of the UARTCTSINTR interrupt is set. A write of 0 clears the mask.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const CTSMIM_OFFSET: u32 = 1;
    pub const CTSMIM_MASK: u32 = 0x00000002;
    pub const CTSMIM: u32 = 1 << 1;

    /// **RIMIM**
    ///
    /// nUARTRI modem interrupt mask. A read returns the current mask for the UARTRIINTR interrupt. On a write of 1, the mask of the UARTRIINTR interrupt is set. A write of 0 clears the mask.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const RIMIM_OFFSET: u32 = 0;
    pub const RIMIM_MASK: u32 = 0x00000001;
    pub const RIMIM: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **OEIM** `[10]` (RW)
    ///
    /// Overrun error interrupt mask. A read returns the current mask for the UARTOEINTR interrupt. On a write of 1, the mask of the UARTOEINTR interrupt is set. A write of 0 clears the mask.
    #[inline(always)]
    pub fn oeim(&self) -> bool {
        (self.value.read_bits() & Self::OEIM_MASK) != 0
    }

    /// Set OEIM bit atomically (RW)
    #[inline(always)]
    pub fn set_oeim(&self) {
        self.set_bits(Self::OEIM_MASK);
    }

    /// Clear OEIM bit atomically (RW)
    #[inline(always)]
    pub fn clr_oeim(&self) {
        self.clr_bits(Self::OEIM_MASK);
    }

    /// **BEIM** `[9]` (RW)
    ///
    /// Break error interrupt mask. A read returns the current mask for the UARTBEINTR interrupt. On a write of 1, the mask of the UARTBEINTR interrupt is set. A write of 0 clears the mask.
    #[inline(always)]
    pub fn beim(&self) -> bool {
        (self.value.read_bits() & Self::BEIM_MASK) != 0
    }

    /// Set BEIM bit atomically (RW)
    #[inline(always)]
    pub fn set_beim(&self) {
        self.set_bits(Self::BEIM_MASK);
    }

    /// Clear BEIM bit atomically (RW)
    #[inline(always)]
    pub fn clr_beim(&self) {
        self.clr_bits(Self::BEIM_MASK);
    }

    /// **PEIM** `[8]` (RW)
    ///
    /// Parity error interrupt mask. A read returns the current mask for the UARTPEINTR interrupt. On a write of 1, the mask of the UARTPEINTR interrupt is set. A write of 0 clears the mask.
    #[inline(always)]
    pub fn peim(&self) -> bool {
        (self.value.read_bits() & Self::PEIM_MASK) != 0
    }

    /// Set PEIM bit atomically (RW)
    #[inline(always)]
    pub fn set_peim(&self) {
        self.set_bits(Self::PEIM_MASK);
    }

    /// Clear PEIM bit atomically (RW)
    #[inline(always)]
    pub fn clr_peim(&self) {
        self.clr_bits(Self::PEIM_MASK);
    }

    /// **FEIM** `[7]` (RW)
    ///
    /// Framing error interrupt mask. A read returns the current mask for the UARTFEINTR interrupt. On a write of 1, the mask of the UARTFEINTR interrupt is set. A write of 0 clears the mask.
    #[inline(always)]
    pub fn feim(&self) -> bool {
        (self.value.read_bits() & Self::FEIM_MASK) != 0
    }

    /// Set FEIM bit atomically (RW)
    #[inline(always)]
    pub fn set_feim(&self) {
        self.set_bits(Self::FEIM_MASK);
    }

    /// Clear FEIM bit atomically (RW)
    #[inline(always)]
    pub fn clr_feim(&self) {
        self.clr_bits(Self::FEIM_MASK);
    }

    /// **RTIM** `[6]` (RW)
    ///
    /// Receive timeout interrupt mask. A read returns the current mask for the UARTRTINTR interrupt. On a write of 1, the mask of the UARTRTINTR interrupt is set. A write of 0 clears the mask.
    #[inline(always)]
    pub fn rtim(&self) -> bool {
        (self.value.read_bits() & Self::RTIM_MASK) != 0
    }

    /// Set RTIM bit atomically (RW)
    #[inline(always)]
    pub fn set_rtim(&self) {
        self.set_bits(Self::RTIM_MASK);
    }

    /// Clear RTIM bit atomically (RW)
    #[inline(always)]
    pub fn clr_rtim(&self) {
        self.clr_bits(Self::RTIM_MASK);
    }

    /// **TXIM** `[5]` (RW)
    ///
    /// Transmit interrupt mask. A read returns the current mask for the UARTTXINTR interrupt. On a write of 1, the mask of the UARTTXINTR interrupt is set. A write of 0 clears the mask.
    #[inline(always)]
    pub fn txim(&self) -> bool {
        (self.value.read_bits() & Self::TXIM_MASK) != 0
    }

    /// Set TXIM bit atomically (RW)
    #[inline(always)]
    pub fn set_txim(&self) {
        self.set_bits(Self::TXIM_MASK);
    }

    /// Clear TXIM bit atomically (RW)
    #[inline(always)]
    pub fn clr_txim(&self) {
        self.clr_bits(Self::TXIM_MASK);
    }

    /// **RXIM** `[4]` (RW)
    ///
    /// Receive interrupt mask. A read returns the current mask for the UARTRXINTR interrupt. On a write of 1, the mask of the UARTRXINTR interrupt is set. A write of 0 clears the mask.
    #[inline(always)]
    pub fn rxim(&self) -> bool {
        (self.value.read_bits() & Self::RXIM_MASK) != 0
    }

    /// Set RXIM bit atomically (RW)
    #[inline(always)]
    pub fn set_rxim(&self) {
        self.set_bits(Self::RXIM_MASK);
    }

    /// Clear RXIM bit atomically (RW)
    #[inline(always)]
    pub fn clr_rxim(&self) {
        self.clr_bits(Self::RXIM_MASK);
    }

    /// **DSRMIM** `[3]` (RW)
    ///
    /// nUARTDSR modem interrupt mask. A read returns the current mask for the UARTDSRINTR interrupt. On a write of 1, the mask of the UARTDSRINTR interrupt is set. A write of 0 clears the mask.
    #[inline(always)]
    pub fn dsrmim(&self) -> bool {
        (self.value.read_bits() & Self::DSRMIM_MASK) != 0
    }

    /// Set DSRMIM bit atomically (RW)
    #[inline(always)]
    pub fn set_dsrmim(&self) {
        self.set_bits(Self::DSRMIM_MASK);
    }

    /// Clear DSRMIM bit atomically (RW)
    #[inline(always)]
    pub fn clr_dsrmim(&self) {
        self.clr_bits(Self::DSRMIM_MASK);
    }

    /// **DCDMIM** `[2]` (RW)
    ///
    /// nUARTDCD modem interrupt mask. A read returns the current mask for the UARTDCDINTR interrupt. On a write of 1, the mask of the UARTDCDINTR interrupt is set. A write of 0 clears the mask.
    #[inline(always)]
    pub fn dcdmim(&self) -> bool {
        (self.value.read_bits() & Self::DCDMIM_MASK) != 0
    }

    /// Set DCDMIM bit atomically (RW)
    #[inline(always)]
    pub fn set_dcdmim(&self) {
        self.set_bits(Self::DCDMIM_MASK);
    }

    /// Clear DCDMIM bit atomically (RW)
    #[inline(always)]
    pub fn clr_dcdmim(&self) {
        self.clr_bits(Self::DCDMIM_MASK);
    }

    /// **CTSMIM** `[1]` (RW)
    ///
    /// nUARTCTS modem interrupt mask. A read returns the current mask for the UARTCTSINTR interrupt. On a write of 1, the mask of the UARTCTSINTR interrupt is set. A write of 0 clears the mask.
    #[inline(always)]
    pub fn ctsmim(&self) -> bool {
        (self.value.read_bits() & Self::CTSMIM_MASK) != 0
    }

    /// Set CTSMIM bit atomically (RW)
    #[inline(always)]
    pub fn set_ctsmim(&self) {
        self.set_bits(Self::CTSMIM_MASK);
    }

    /// Clear CTSMIM bit atomically (RW)
    #[inline(always)]
    pub fn clr_ctsmim(&self) {
        self.clr_bits(Self::CTSMIM_MASK);
    }

    /// **RIMIM** `[0]` (RW)
    ///
    /// nUARTRI modem interrupt mask. A read returns the current mask for the UARTRIINTR interrupt. On a write of 1, the mask of the UARTRIINTR interrupt is set. A write of 0 clears the mask.
    #[inline(always)]
    pub fn rimim(&self) -> bool {
        (self.value.read_bits() & Self::RIMIM_MASK) != 0
    }

    /// Set RIMIM bit atomically (RW)
    #[inline(always)]
    pub fn set_rimim(&self) {
        self.set_bits(Self::RIMIM_MASK);
    }

    /// Clear RIMIM bit atomically (RW)
    #[inline(always)]
    pub fn clr_rimim(&self) {
        self.clr_bits(Self::RIMIM_MASK);
    }
}

/// UARTIMSC value type for batch read/modify/write operations
///
/// Use `uartimsc.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `uartimsc.write_val(val)`.
#[derive(Clone, Copy)]
pub struct UartimscVal {
    bits: u32,
}

impl UartimscVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x00000000 }
    }

    /// Read **OEIM** `[10:10]`
    ///
    /// Overrun error interrupt mask. A read returns the current mask for the UARTOEINTR interrupt. On a write of 1, the mask of the UARTOEINTR interrupt is set. A write of 0 clears the mask.
    #[inline(always)]
    pub const fn oeim(&self) -> bool {
        (self.bits & Uartimsc::OEIM_MASK) != 0
    }

    /// Read **BEIM** `[9:9]`
    ///
    /// Break error interrupt mask. A read returns the current mask for the UARTBEINTR interrupt. On a write of 1, the mask of the UARTBEINTR interrupt is set. A write of 0 clears the mask.
    #[inline(always)]
    pub const fn beim(&self) -> bool {
        (self.bits & Uartimsc::BEIM_MASK) != 0
    }

    /// Read **PEIM** `[8:8]`
    ///
    /// Parity error interrupt mask. A read returns the current mask for the UARTPEINTR interrupt. On a write of 1, the mask of the UARTPEINTR interrupt is set. A write of 0 clears the mask.
    #[inline(always)]
    pub const fn peim(&self) -> bool {
        (self.bits & Uartimsc::PEIM_MASK) != 0
    }

    /// Read **FEIM** `[7:7]`
    ///
    /// Framing error interrupt mask. A read returns the current mask for the UARTFEINTR interrupt. On a write of 1, the mask of the UARTFEINTR interrupt is set. A write of 0 clears the mask.
    #[inline(always)]
    pub const fn feim(&self) -> bool {
        (self.bits & Uartimsc::FEIM_MASK) != 0
    }

    /// Read **RTIM** `[6:6]`
    ///
    /// Receive timeout interrupt mask. A read returns the current mask for the UARTRTINTR interrupt. On a write of 1, the mask of the UARTRTINTR interrupt is set. A write of 0 clears the mask.
    #[inline(always)]
    pub const fn rtim(&self) -> bool {
        (self.bits & Uartimsc::RTIM_MASK) != 0
    }

    /// Read **TXIM** `[5:5]`
    ///
    /// Transmit interrupt mask. A read returns the current mask for the UARTTXINTR interrupt. On a write of 1, the mask of the UARTTXINTR interrupt is set. A write of 0 clears the mask.
    #[inline(always)]
    pub const fn txim(&self) -> bool {
        (self.bits & Uartimsc::TXIM_MASK) != 0
    }

    /// Read **RXIM** `[4:4]`
    ///
    /// Receive interrupt mask. A read returns the current mask for the UARTRXINTR interrupt. On a write of 1, the mask of the UARTRXINTR interrupt is set. A write of 0 clears the mask.
    #[inline(always)]
    pub const fn rxim(&self) -> bool {
        (self.bits & Uartimsc::RXIM_MASK) != 0
    }

    /// Read **DSRMIM** `[3:3]`
    ///
    /// nUARTDSR modem interrupt mask. A read returns the current mask for the UARTDSRINTR interrupt. On a write of 1, the mask of the UARTDSRINTR interrupt is set. A write of 0 clears the mask.
    #[inline(always)]
    pub const fn dsrmim(&self) -> bool {
        (self.bits & Uartimsc::DSRMIM_MASK) != 0
    }

    /// Read **DCDMIM** `[2:2]`
    ///
    /// nUARTDCD modem interrupt mask. A read returns the current mask for the UARTDCDINTR interrupt. On a write of 1, the mask of the UARTDCDINTR interrupt is set. A write of 0 clears the mask.
    #[inline(always)]
    pub const fn dcdmim(&self) -> bool {
        (self.bits & Uartimsc::DCDMIM_MASK) != 0
    }

    /// Read **CTSMIM** `[1:1]`
    ///
    /// nUARTCTS modem interrupt mask. A read returns the current mask for the UARTCTSINTR interrupt. On a write of 1, the mask of the UARTCTSINTR interrupt is set. A write of 0 clears the mask.
    #[inline(always)]
    pub const fn ctsmim(&self) -> bool {
        (self.bits & Uartimsc::CTSMIM_MASK) != 0
    }

    /// Read **RIMIM** `[0:0]`
    ///
    /// nUARTRI modem interrupt mask. A read returns the current mask for the UARTRIINTR interrupt. On a write of 1, the mask of the UARTRIINTR interrupt is set. A write of 0 clears the mask.
    #[inline(always)]
    pub const fn rimim(&self) -> bool {
        (self.bits & Uartimsc::RIMIM_MASK) != 0
    }

    /// Set **OEIM** `[10:10]`
    ///
    /// Overrun error interrupt mask. A read returns the current mask for the UARTOEINTR interrupt. On a write of 1, the mask of the UARTOEINTR interrupt is set. A write of 0 clears the mask.
    #[inline(always)]
    pub const fn with_oeim(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Uartimsc::OEIM_MASK
            } else {
                self.bits & !Uartimsc::OEIM_MASK
            },
        }
    }

    /// Set **BEIM** `[9:9]`
    ///
    /// Break error interrupt mask. A read returns the current mask for the UARTBEINTR interrupt. On a write of 1, the mask of the UARTBEINTR interrupt is set. A write of 0 clears the mask.
    #[inline(always)]
    pub const fn with_beim(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Uartimsc::BEIM_MASK
            } else {
                self.bits & !Uartimsc::BEIM_MASK
            },
        }
    }

    /// Set **PEIM** `[8:8]`
    ///
    /// Parity error interrupt mask. A read returns the current mask for the UARTPEINTR interrupt. On a write of 1, the mask of the UARTPEINTR interrupt is set. A write of 0 clears the mask.
    #[inline(always)]
    pub const fn with_peim(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Uartimsc::PEIM_MASK
            } else {
                self.bits & !Uartimsc::PEIM_MASK
            },
        }
    }

    /// Set **FEIM** `[7:7]`
    ///
    /// Framing error interrupt mask. A read returns the current mask for the UARTFEINTR interrupt. On a write of 1, the mask of the UARTFEINTR interrupt is set. A write of 0 clears the mask.
    #[inline(always)]
    pub const fn with_feim(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Uartimsc::FEIM_MASK
            } else {
                self.bits & !Uartimsc::FEIM_MASK
            },
        }
    }

    /// Set **RTIM** `[6:6]`
    ///
    /// Receive timeout interrupt mask. A read returns the current mask for the UARTRTINTR interrupt. On a write of 1, the mask of the UARTRTINTR interrupt is set. A write of 0 clears the mask.
    #[inline(always)]
    pub const fn with_rtim(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Uartimsc::RTIM_MASK
            } else {
                self.bits & !Uartimsc::RTIM_MASK
            },
        }
    }

    /// Set **TXIM** `[5:5]`
    ///
    /// Transmit interrupt mask. A read returns the current mask for the UARTTXINTR interrupt. On a write of 1, the mask of the UARTTXINTR interrupt is set. A write of 0 clears the mask.
    #[inline(always)]
    pub const fn with_txim(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Uartimsc::TXIM_MASK
            } else {
                self.bits & !Uartimsc::TXIM_MASK
            },
        }
    }

    /// Set **RXIM** `[4:4]`
    ///
    /// Receive interrupt mask. A read returns the current mask for the UARTRXINTR interrupt. On a write of 1, the mask of the UARTRXINTR interrupt is set. A write of 0 clears the mask.
    #[inline(always)]
    pub const fn with_rxim(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Uartimsc::RXIM_MASK
            } else {
                self.bits & !Uartimsc::RXIM_MASK
            },
        }
    }

    /// Set **DSRMIM** `[3:3]`
    ///
    /// nUARTDSR modem interrupt mask. A read returns the current mask for the UARTDSRINTR interrupt. On a write of 1, the mask of the UARTDSRINTR interrupt is set. A write of 0 clears the mask.
    #[inline(always)]
    pub const fn with_dsrmim(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Uartimsc::DSRMIM_MASK
            } else {
                self.bits & !Uartimsc::DSRMIM_MASK
            },
        }
    }

    /// Set **DCDMIM** `[2:2]`
    ///
    /// nUARTDCD modem interrupt mask. A read returns the current mask for the UARTDCDINTR interrupt. On a write of 1, the mask of the UARTDCDINTR interrupt is set. A write of 0 clears the mask.
    #[inline(always)]
    pub const fn with_dcdmim(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Uartimsc::DCDMIM_MASK
            } else {
                self.bits & !Uartimsc::DCDMIM_MASK
            },
        }
    }

    /// Set **CTSMIM** `[1:1]`
    ///
    /// nUARTCTS modem interrupt mask. A read returns the current mask for the UARTCTSINTR interrupt. On a write of 1, the mask of the UARTCTSINTR interrupt is set. A write of 0 clears the mask.
    #[inline(always)]
    pub const fn with_ctsmim(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Uartimsc::CTSMIM_MASK
            } else {
                self.bits & !Uartimsc::CTSMIM_MASK
            },
        }
    }

    /// Set **RIMIM** `[0:0]`
    ///
    /// nUARTRI modem interrupt mask. A read returns the current mask for the UARTRIINTR interrupt. On a write of 1, the mask of the UARTRIINTR interrupt is set. A write of 0 clears the mask.
    #[inline(always)]
    pub const fn with_rimim(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Uartimsc::RIMIM_MASK
            } else {
                self.bits & !Uartimsc::RIMIM_MASK
            },
        }
    }
}

impl Default for UartimscVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Uartimsc {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> UartimscVal {
        UartimscVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: UartimscVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(UartimscVal) -> UartimscVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// UARTRIS Register
///
/// Raw Interrupt Status Register, UARTRIS
///
/// Address offset: 0x003c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **OERIS** `[10]` (RO)
///
///   Overrun error interrupt status. Returns the raw interrupt state of the UARTOEINTR interrupt.
///
/// - **BERIS** `[9]` (RO)
///
///   Break error interrupt status. Returns the raw interrupt state of the UARTBEINTR interrupt.
///
/// - **PERIS** `[8]` (RO)
///
///   Parity error interrupt status. Returns the raw interrupt state of the UARTPEINTR interrupt.
///
/// - **FERIS** `[7]` (RO)
///
///   Framing error interrupt status. Returns the raw interrupt state of the UARTFEINTR interrupt.
///
/// - **RTRIS** `[6]` (RO)
///
///   Receive timeout interrupt status. Returns the raw interrupt state of the UARTRTINTR interrupt. a
///
/// - **TXRIS** `[5]` (RO)
///
///   Transmit interrupt status. Returns the raw interrupt state of the UARTTXINTR interrupt.
///
/// - **RXRIS** `[4]` (RO)
///
///   Receive interrupt status. Returns the raw interrupt state of the UARTRXINTR interrupt.
///
/// - **DSRRMIS** `[3]` (RO)
///
///   nUARTDSR modem interrupt status. Returns the raw interrupt state of the UARTDSRINTR interrupt.
///
/// - **DCDRMIS** `[2]` (RO)
///
///   nUARTDCD modem interrupt status. Returns the raw interrupt state of the UARTDCDINTR interrupt.
///
/// - **CTSRMIS** `[1]` (RO)
///
///   nUARTCTS modem interrupt status. Returns the raw interrupt state of the UARTCTSINTR interrupt.
///
/// - **RIRMIS** `[0]` (RO)
///
///   nUARTRI modem interrupt status. Returns the raw interrupt state of the UARTRIINTR interrupt.
#[repr(transparent)]
pub struct Uartris {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Uartris {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Uartris {
    // ======== Field Constants ========

    /// **OERIS**
    ///
    /// Overrun error interrupt status. Returns the raw interrupt state of the UARTOEINTR interrupt.
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const OERIS_OFFSET: u32 = 10;
    pub const OERIS_MASK: u32 = 0x00000400;
    pub const OERIS: u32 = 1 << 10;

    /// **BERIS**
    ///
    /// Break error interrupt status. Returns the raw interrupt state of the UARTBEINTR interrupt.
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const BERIS_OFFSET: u32 = 9;
    pub const BERIS_MASK: u32 = 0x00000200;
    pub const BERIS: u32 = 1 << 9;

    /// **PERIS**
    ///
    /// Parity error interrupt status. Returns the raw interrupt state of the UARTPEINTR interrupt.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const PERIS_OFFSET: u32 = 8;
    pub const PERIS_MASK: u32 = 0x00000100;
    pub const PERIS: u32 = 1 << 8;

    /// **FERIS**
    ///
    /// Framing error interrupt status. Returns the raw interrupt state of the UARTFEINTR interrupt.
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const FERIS_OFFSET: u32 = 7;
    pub const FERIS_MASK: u32 = 0x00000080;
    pub const FERIS: u32 = 1 << 7;

    /// **RTRIS**
    ///
    /// Receive timeout interrupt status. Returns the raw interrupt state of the UARTRTINTR interrupt. a
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const RTRIS_OFFSET: u32 = 6;
    pub const RTRIS_MASK: u32 = 0x00000040;
    pub const RTRIS: u32 = 1 << 6;

    /// **TXRIS**
    ///
    /// Transmit interrupt status. Returns the raw interrupt state of the UARTTXINTR interrupt.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const TXRIS_OFFSET: u32 = 5;
    pub const TXRIS_MASK: u32 = 0x00000020;
    pub const TXRIS: u32 = 1 << 5;

    /// **RXRIS**
    ///
    /// Receive interrupt status. Returns the raw interrupt state of the UARTRXINTR interrupt.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const RXRIS_OFFSET: u32 = 4;
    pub const RXRIS_MASK: u32 = 0x00000010;
    pub const RXRIS: u32 = 1 << 4;

    /// **DSRRMIS**
    ///
    /// nUARTDSR modem interrupt status. Returns the raw interrupt state of the UARTDSRINTR interrupt.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const DSRRMIS_OFFSET: u32 = 3;
    pub const DSRRMIS_MASK: u32 = 0x00000008;
    pub const DSRRMIS: u32 = 1 << 3;

    /// **DCDRMIS**
    ///
    /// nUARTDCD modem interrupt status. Returns the raw interrupt state of the UARTDCDINTR interrupt.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const DCDRMIS_OFFSET: u32 = 2;
    pub const DCDRMIS_MASK: u32 = 0x00000004;
    pub const DCDRMIS: u32 = 1 << 2;

    /// **CTSRMIS**
    ///
    /// nUARTCTS modem interrupt status. Returns the raw interrupt state of the UARTCTSINTR interrupt.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const CTSRMIS_OFFSET: u32 = 1;
    pub const CTSRMIS_MASK: u32 = 0x00000002;
    pub const CTSRMIS: u32 = 1 << 1;

    /// **RIRMIS**
    ///
    /// nUARTRI modem interrupt status. Returns the raw interrupt state of the UARTRIINTR interrupt.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const RIRMIS_OFFSET: u32 = 0;
    pub const RIRMIS_MASK: u32 = 0x00000001;
    pub const RIRMIS: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **OERIS** `[10]` (RO)
    ///
    /// Overrun error interrupt status. Returns the raw interrupt state of the UARTOEINTR interrupt.
    #[inline(always)]
    pub fn oeris(&self) -> bool {
        (self.value.read_bits() & Self::OERIS_MASK) != 0
    }

    /// **BERIS** `[9]` (RO)
    ///
    /// Break error interrupt status. Returns the raw interrupt state of the UARTBEINTR interrupt.
    #[inline(always)]
    pub fn beris(&self) -> bool {
        (self.value.read_bits() & Self::BERIS_MASK) != 0
    }

    /// **PERIS** `[8]` (RO)
    ///
    /// Parity error interrupt status. Returns the raw interrupt state of the UARTPEINTR interrupt.
    #[inline(always)]
    pub fn peris(&self) -> bool {
        (self.value.read_bits() & Self::PERIS_MASK) != 0
    }

    /// **FERIS** `[7]` (RO)
    ///
    /// Framing error interrupt status. Returns the raw interrupt state of the UARTFEINTR interrupt.
    #[inline(always)]
    pub fn feris(&self) -> bool {
        (self.value.read_bits() & Self::FERIS_MASK) != 0
    }

    /// **RTRIS** `[6]` (RO)
    ///
    /// Receive timeout interrupt status. Returns the raw interrupt state of the UARTRTINTR interrupt. a
    #[inline(always)]
    pub fn rtris(&self) -> bool {
        (self.value.read_bits() & Self::RTRIS_MASK) != 0
    }

    /// **TXRIS** `[5]` (RO)
    ///
    /// Transmit interrupt status. Returns the raw interrupt state of the UARTTXINTR interrupt.
    #[inline(always)]
    pub fn txris(&self) -> bool {
        (self.value.read_bits() & Self::TXRIS_MASK) != 0
    }

    /// **RXRIS** `[4]` (RO)
    ///
    /// Receive interrupt status. Returns the raw interrupt state of the UARTRXINTR interrupt.
    #[inline(always)]
    pub fn rxris(&self) -> bool {
        (self.value.read_bits() & Self::RXRIS_MASK) != 0
    }

    /// **DSRRMIS** `[3]` (RO)
    ///
    /// nUARTDSR modem interrupt status. Returns the raw interrupt state of the UARTDSRINTR interrupt.
    #[inline(always)]
    pub fn dsrrmis(&self) -> bool {
        (self.value.read_bits() & Self::DSRRMIS_MASK) != 0
    }

    /// **DCDRMIS** `[2]` (RO)
    ///
    /// nUARTDCD modem interrupt status. Returns the raw interrupt state of the UARTDCDINTR interrupt.
    #[inline(always)]
    pub fn dcdrmis(&self) -> bool {
        (self.value.read_bits() & Self::DCDRMIS_MASK) != 0
    }

    /// **CTSRMIS** `[1]` (RO)
    ///
    /// nUARTCTS modem interrupt status. Returns the raw interrupt state of the UARTCTSINTR interrupt.
    #[inline(always)]
    pub fn ctsrmis(&self) -> bool {
        (self.value.read_bits() & Self::CTSRMIS_MASK) != 0
    }

    /// **RIRMIS** `[0]` (RO)
    ///
    /// nUARTRI modem interrupt status. Returns the raw interrupt state of the UARTRIINTR interrupt.
    #[inline(always)]
    pub fn rirmis(&self) -> bool {
        (self.value.read_bits() & Self::RIRMIS_MASK) != 0
    }
}

/// UARTRIS value type for batch read/modify/write operations
///
/// Use `uartris.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `uartris.write_val(val)`.
#[derive(Clone, Copy)]
pub struct UartrisVal {
    bits: u32,
}

impl UartrisVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x00000000 }
    }

    /// Read **OERIS** `[10:10]`
    ///
    /// Overrun error interrupt status. Returns the raw interrupt state of the UARTOEINTR interrupt.
    #[inline(always)]
    pub const fn oeris(&self) -> bool {
        (self.bits & Uartris::OERIS_MASK) != 0
    }

    /// Read **BERIS** `[9:9]`
    ///
    /// Break error interrupt status. Returns the raw interrupt state of the UARTBEINTR interrupt.
    #[inline(always)]
    pub const fn beris(&self) -> bool {
        (self.bits & Uartris::BERIS_MASK) != 0
    }

    /// Read **PERIS** `[8:8]`
    ///
    /// Parity error interrupt status. Returns the raw interrupt state of the UARTPEINTR interrupt.
    #[inline(always)]
    pub const fn peris(&self) -> bool {
        (self.bits & Uartris::PERIS_MASK) != 0
    }

    /// Read **FERIS** `[7:7]`
    ///
    /// Framing error interrupt status. Returns the raw interrupt state of the UARTFEINTR interrupt.
    #[inline(always)]
    pub const fn feris(&self) -> bool {
        (self.bits & Uartris::FERIS_MASK) != 0
    }

    /// Read **RTRIS** `[6:6]`
    ///
    /// Receive timeout interrupt status. Returns the raw interrupt state of the UARTRTINTR interrupt. a
    #[inline(always)]
    pub const fn rtris(&self) -> bool {
        (self.bits & Uartris::RTRIS_MASK) != 0
    }

    /// Read **TXRIS** `[5:5]`
    ///
    /// Transmit interrupt status. Returns the raw interrupt state of the UARTTXINTR interrupt.
    #[inline(always)]
    pub const fn txris(&self) -> bool {
        (self.bits & Uartris::TXRIS_MASK) != 0
    }

    /// Read **RXRIS** `[4:4]`
    ///
    /// Receive interrupt status. Returns the raw interrupt state of the UARTRXINTR interrupt.
    #[inline(always)]
    pub const fn rxris(&self) -> bool {
        (self.bits & Uartris::RXRIS_MASK) != 0
    }

    /// Read **DSRRMIS** `[3:3]`
    ///
    /// nUARTDSR modem interrupt status. Returns the raw interrupt state of the UARTDSRINTR interrupt.
    #[inline(always)]
    pub const fn dsrrmis(&self) -> bool {
        (self.bits & Uartris::DSRRMIS_MASK) != 0
    }

    /// Read **DCDRMIS** `[2:2]`
    ///
    /// nUARTDCD modem interrupt status. Returns the raw interrupt state of the UARTDCDINTR interrupt.
    #[inline(always)]
    pub const fn dcdrmis(&self) -> bool {
        (self.bits & Uartris::DCDRMIS_MASK) != 0
    }

    /// Read **CTSRMIS** `[1:1]`
    ///
    /// nUARTCTS modem interrupt status. Returns the raw interrupt state of the UARTCTSINTR interrupt.
    #[inline(always)]
    pub const fn ctsrmis(&self) -> bool {
        (self.bits & Uartris::CTSRMIS_MASK) != 0
    }

    /// Read **RIRMIS** `[0:0]`
    ///
    /// nUARTRI modem interrupt status. Returns the raw interrupt state of the UARTRIINTR interrupt.
    #[inline(always)]
    pub const fn rirmis(&self) -> bool {
        (self.bits & Uartris::RIRMIS_MASK) != 0
    }
}

impl Default for UartrisVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Uartris {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> UartrisVal {
        UartrisVal::from_bits(self.value.read_bits())
    }
}

/// UARTMIS Register
///
/// Masked Interrupt Status Register, UARTMIS
///
/// Address offset: 0x0040
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **OEMIS** `[10]` (RO)
///
///   Overrun error masked interrupt status. Returns the masked interrupt state of the UARTOEINTR interrupt.
///
/// - **BEMIS** `[9]` (RO)
///
///   Break error masked interrupt status. Returns the masked interrupt state of the UARTBEINTR interrupt.
///
/// - **PEMIS** `[8]` (RO)
///
///   Parity error masked interrupt status. Returns the masked interrupt state of the UARTPEINTR interrupt.
///
/// - **FEMIS** `[7]` (RO)
///
///   Framing error masked interrupt status. Returns the masked interrupt state of the UARTFEINTR interrupt.
///
/// - **RTMIS** `[6]` (RO)
///
///   Receive timeout masked interrupt status. Returns the masked interrupt state of the UARTRTINTR interrupt.
///
/// - **TXMIS** `[5]` (RO)
///
///   Transmit masked interrupt status. Returns the masked interrupt state of the UARTTXINTR interrupt.
///
/// - **RXMIS** `[4]` (RO)
///
///   Receive masked interrupt status. Returns the masked interrupt state of the UARTRXINTR interrupt.
///
/// - **DSRMMIS** `[3]` (RO)
///
///   nUARTDSR modem masked interrupt status. Returns the masked interrupt state of the UARTDSRINTR interrupt.
///
/// - **DCDMMIS** `[2]` (RO)
///
///   nUARTDCD modem masked interrupt status. Returns the masked interrupt state of the UARTDCDINTR interrupt.
///
/// - **CTSMMIS** `[1]` (RO)
///
///   nUARTCTS modem masked interrupt status. Returns the masked interrupt state of the UARTCTSINTR interrupt.
///
/// - **RIMMIS** `[0]` (RO)
///
///   nUARTRI modem masked interrupt status. Returns the masked interrupt state of the UARTRIINTR interrupt.
#[repr(transparent)]
pub struct Uartmis {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Uartmis {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Uartmis {
    // ======== Field Constants ========

    /// **OEMIS**
    ///
    /// Overrun error masked interrupt status. Returns the masked interrupt state of the UARTOEINTR interrupt.
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const OEMIS_OFFSET: u32 = 10;
    pub const OEMIS_MASK: u32 = 0x00000400;
    pub const OEMIS: u32 = 1 << 10;

    /// **BEMIS**
    ///
    /// Break error masked interrupt status. Returns the masked interrupt state of the UARTBEINTR interrupt.
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const BEMIS_OFFSET: u32 = 9;
    pub const BEMIS_MASK: u32 = 0x00000200;
    pub const BEMIS: u32 = 1 << 9;

    /// **PEMIS**
    ///
    /// Parity error masked interrupt status. Returns the masked interrupt state of the UARTPEINTR interrupt.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const PEMIS_OFFSET: u32 = 8;
    pub const PEMIS_MASK: u32 = 0x00000100;
    pub const PEMIS: u32 = 1 << 8;

    /// **FEMIS**
    ///
    /// Framing error masked interrupt status. Returns the masked interrupt state of the UARTFEINTR interrupt.
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const FEMIS_OFFSET: u32 = 7;
    pub const FEMIS_MASK: u32 = 0x00000080;
    pub const FEMIS: u32 = 1 << 7;

    /// **RTMIS**
    ///
    /// Receive timeout masked interrupt status. Returns the masked interrupt state of the UARTRTINTR interrupt.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const RTMIS_OFFSET: u32 = 6;
    pub const RTMIS_MASK: u32 = 0x00000040;
    pub const RTMIS: u32 = 1 << 6;

    /// **TXMIS**
    ///
    /// Transmit masked interrupt status. Returns the masked interrupt state of the UARTTXINTR interrupt.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const TXMIS_OFFSET: u32 = 5;
    pub const TXMIS_MASK: u32 = 0x00000020;
    pub const TXMIS: u32 = 1 << 5;

    /// **RXMIS**
    ///
    /// Receive masked interrupt status. Returns the masked interrupt state of the UARTRXINTR interrupt.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const RXMIS_OFFSET: u32 = 4;
    pub const RXMIS_MASK: u32 = 0x00000010;
    pub const RXMIS: u32 = 1 << 4;

    /// **DSRMMIS**
    ///
    /// nUARTDSR modem masked interrupt status. Returns the masked interrupt state of the UARTDSRINTR interrupt.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const DSRMMIS_OFFSET: u32 = 3;
    pub const DSRMMIS_MASK: u32 = 0x00000008;
    pub const DSRMMIS: u32 = 1 << 3;

    /// **DCDMMIS**
    ///
    /// nUARTDCD modem masked interrupt status. Returns the masked interrupt state of the UARTDCDINTR interrupt.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const DCDMMIS_OFFSET: u32 = 2;
    pub const DCDMMIS_MASK: u32 = 0x00000004;
    pub const DCDMMIS: u32 = 1 << 2;

    /// **CTSMMIS**
    ///
    /// nUARTCTS modem masked interrupt status. Returns the masked interrupt state of the UARTCTSINTR interrupt.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const CTSMMIS_OFFSET: u32 = 1;
    pub const CTSMMIS_MASK: u32 = 0x00000002;
    pub const CTSMMIS: u32 = 1 << 1;

    /// **RIMMIS**
    ///
    /// nUARTRI modem masked interrupt status. Returns the masked interrupt state of the UARTRIINTR interrupt.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const RIMMIS_OFFSET: u32 = 0;
    pub const RIMMIS_MASK: u32 = 0x00000001;
    pub const RIMMIS: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **OEMIS** `[10]` (RO)
    ///
    /// Overrun error masked interrupt status. Returns the masked interrupt state of the UARTOEINTR interrupt.
    #[inline(always)]
    pub fn oemis(&self) -> bool {
        (self.value.read_bits() & Self::OEMIS_MASK) != 0
    }

    /// **BEMIS** `[9]` (RO)
    ///
    /// Break error masked interrupt status. Returns the masked interrupt state of the UARTBEINTR interrupt.
    #[inline(always)]
    pub fn bemis(&self) -> bool {
        (self.value.read_bits() & Self::BEMIS_MASK) != 0
    }

    /// **PEMIS** `[8]` (RO)
    ///
    /// Parity error masked interrupt status. Returns the masked interrupt state of the UARTPEINTR interrupt.
    #[inline(always)]
    pub fn pemis(&self) -> bool {
        (self.value.read_bits() & Self::PEMIS_MASK) != 0
    }

    /// **FEMIS** `[7]` (RO)
    ///
    /// Framing error masked interrupt status. Returns the masked interrupt state of the UARTFEINTR interrupt.
    #[inline(always)]
    pub fn femis(&self) -> bool {
        (self.value.read_bits() & Self::FEMIS_MASK) != 0
    }

    /// **RTMIS** `[6]` (RO)
    ///
    /// Receive timeout masked interrupt status. Returns the masked interrupt state of the UARTRTINTR interrupt.
    #[inline(always)]
    pub fn rtmis(&self) -> bool {
        (self.value.read_bits() & Self::RTMIS_MASK) != 0
    }

    /// **TXMIS** `[5]` (RO)
    ///
    /// Transmit masked interrupt status. Returns the masked interrupt state of the UARTTXINTR interrupt.
    #[inline(always)]
    pub fn txmis(&self) -> bool {
        (self.value.read_bits() & Self::TXMIS_MASK) != 0
    }

    /// **RXMIS** `[4]` (RO)
    ///
    /// Receive masked interrupt status. Returns the masked interrupt state of the UARTRXINTR interrupt.
    #[inline(always)]
    pub fn rxmis(&self) -> bool {
        (self.value.read_bits() & Self::RXMIS_MASK) != 0
    }

    /// **DSRMMIS** `[3]` (RO)
    ///
    /// nUARTDSR modem masked interrupt status. Returns the masked interrupt state of the UARTDSRINTR interrupt.
    #[inline(always)]
    pub fn dsrmmis(&self) -> bool {
        (self.value.read_bits() & Self::DSRMMIS_MASK) != 0
    }

    /// **DCDMMIS** `[2]` (RO)
    ///
    /// nUARTDCD modem masked interrupt status. Returns the masked interrupt state of the UARTDCDINTR interrupt.
    #[inline(always)]
    pub fn dcdmmis(&self) -> bool {
        (self.value.read_bits() & Self::DCDMMIS_MASK) != 0
    }

    /// **CTSMMIS** `[1]` (RO)
    ///
    /// nUARTCTS modem masked interrupt status. Returns the masked interrupt state of the UARTCTSINTR interrupt.
    #[inline(always)]
    pub fn ctsmmis(&self) -> bool {
        (self.value.read_bits() & Self::CTSMMIS_MASK) != 0
    }

    /// **RIMMIS** `[0]` (RO)
    ///
    /// nUARTRI modem masked interrupt status. Returns the masked interrupt state of the UARTRIINTR interrupt.
    #[inline(always)]
    pub fn rimmis(&self) -> bool {
        (self.value.read_bits() & Self::RIMMIS_MASK) != 0
    }
}

/// UARTMIS value type for batch read/modify/write operations
///
/// Use `uartmis.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `uartmis.write_val(val)`.
#[derive(Clone, Copy)]
pub struct UartmisVal {
    bits: u32,
}

impl UartmisVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x00000000 }
    }

    /// Read **OEMIS** `[10:10]`
    ///
    /// Overrun error masked interrupt status. Returns the masked interrupt state of the UARTOEINTR interrupt.
    #[inline(always)]
    pub const fn oemis(&self) -> bool {
        (self.bits & Uartmis::OEMIS_MASK) != 0
    }

    /// Read **BEMIS** `[9:9]`
    ///
    /// Break error masked interrupt status. Returns the masked interrupt state of the UARTBEINTR interrupt.
    #[inline(always)]
    pub const fn bemis(&self) -> bool {
        (self.bits & Uartmis::BEMIS_MASK) != 0
    }

    /// Read **PEMIS** `[8:8]`
    ///
    /// Parity error masked interrupt status. Returns the masked interrupt state of the UARTPEINTR interrupt.
    #[inline(always)]
    pub const fn pemis(&self) -> bool {
        (self.bits & Uartmis::PEMIS_MASK) != 0
    }

    /// Read **FEMIS** `[7:7]`
    ///
    /// Framing error masked interrupt status. Returns the masked interrupt state of the UARTFEINTR interrupt.
    #[inline(always)]
    pub const fn femis(&self) -> bool {
        (self.bits & Uartmis::FEMIS_MASK) != 0
    }

    /// Read **RTMIS** `[6:6]`
    ///
    /// Receive timeout masked interrupt status. Returns the masked interrupt state of the UARTRTINTR interrupt.
    #[inline(always)]
    pub const fn rtmis(&self) -> bool {
        (self.bits & Uartmis::RTMIS_MASK) != 0
    }

    /// Read **TXMIS** `[5:5]`
    ///
    /// Transmit masked interrupt status. Returns the masked interrupt state of the UARTTXINTR interrupt.
    #[inline(always)]
    pub const fn txmis(&self) -> bool {
        (self.bits & Uartmis::TXMIS_MASK) != 0
    }

    /// Read **RXMIS** `[4:4]`
    ///
    /// Receive masked interrupt status. Returns the masked interrupt state of the UARTRXINTR interrupt.
    #[inline(always)]
    pub const fn rxmis(&self) -> bool {
        (self.bits & Uartmis::RXMIS_MASK) != 0
    }

    /// Read **DSRMMIS** `[3:3]`
    ///
    /// nUARTDSR modem masked interrupt status. Returns the masked interrupt state of the UARTDSRINTR interrupt.
    #[inline(always)]
    pub const fn dsrmmis(&self) -> bool {
        (self.bits & Uartmis::DSRMMIS_MASK) != 0
    }

    /// Read **DCDMMIS** `[2:2]`
    ///
    /// nUARTDCD modem masked interrupt status. Returns the masked interrupt state of the UARTDCDINTR interrupt.
    #[inline(always)]
    pub const fn dcdmmis(&self) -> bool {
        (self.bits & Uartmis::DCDMMIS_MASK) != 0
    }

    /// Read **CTSMMIS** `[1:1]`
    ///
    /// nUARTCTS modem masked interrupt status. Returns the masked interrupt state of the UARTCTSINTR interrupt.
    #[inline(always)]
    pub const fn ctsmmis(&self) -> bool {
        (self.bits & Uartmis::CTSMMIS_MASK) != 0
    }

    /// Read **RIMMIS** `[0:0]`
    ///
    /// nUARTRI modem masked interrupt status. Returns the masked interrupt state of the UARTRIINTR interrupt.
    #[inline(always)]
    pub const fn rimmis(&self) -> bool {
        (self.bits & Uartmis::RIMMIS_MASK) != 0
    }
}

impl Default for UartmisVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Uartmis {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> UartmisVal {
        UartmisVal::from_bits(self.value.read_bits())
    }
}

/// UARTICR Register
///
/// Interrupt Clear Register, UARTICR
///
/// Address offset: 0x0044
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **OEIC** `[10]` (RW)
///
///   Overrun error interrupt clear. Clears the UARTOEINTR interrupt.
///
/// - **BEIC** `[9]` (RW)
///
///   Break error interrupt clear. Clears the UARTBEINTR interrupt.
///
/// - **PEIC** `[8]` (RW)
///
///   Parity error interrupt clear. Clears the UARTPEINTR interrupt.
///
/// - **FEIC** `[7]` (RW)
///
///   Framing error interrupt clear. Clears the UARTFEINTR interrupt.
///
/// - **RTIC** `[6]` (RW)
///
///   Receive timeout interrupt clear. Clears the UARTRTINTR interrupt.
///
/// - **TXIC** `[5]` (RW)
///
///   Transmit interrupt clear. Clears the UARTTXINTR interrupt.
///
/// - **RXIC** `[4]` (RW)
///
///   Receive interrupt clear. Clears the UARTRXINTR interrupt.
///
/// - **DSRMIC** `[3]` (RW)
///
///   nUARTDSR modem interrupt clear. Clears the UARTDSRINTR interrupt.
///
/// - **DCDMIC** `[2]` (RW)
///
///   nUARTDCD modem interrupt clear. Clears the UARTDCDINTR interrupt.
///
/// - **CTSMIC** `[1]` (RW)
///
///   nUARTCTS modem interrupt clear. Clears the UARTCTSINTR interrupt.
///
/// - **RIMIC** `[0]` (RW)
///
///   nUARTRI modem interrupt clear. Clears the UARTRIINTR interrupt.
#[repr(transparent)]
pub struct Uarticr {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Uarticr {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Uarticr {
    // ======== Field Constants ========

    /// **OEIC**
    ///
    /// Overrun error interrupt clear. Clears the UARTOEINTR interrupt.
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const OEIC_OFFSET: u32 = 10;
    pub const OEIC_MASK: u32 = 0x00000400;
    pub const OEIC: u32 = 1 << 10;

    /// **BEIC**
    ///
    /// Break error interrupt clear. Clears the UARTBEINTR interrupt.
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const BEIC_OFFSET: u32 = 9;
    pub const BEIC_MASK: u32 = 0x00000200;
    pub const BEIC: u32 = 1 << 9;

    /// **PEIC**
    ///
    /// Parity error interrupt clear. Clears the UARTPEINTR interrupt.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const PEIC_OFFSET: u32 = 8;
    pub const PEIC_MASK: u32 = 0x00000100;
    pub const PEIC: u32 = 1 << 8;

    /// **FEIC**
    ///
    /// Framing error interrupt clear. Clears the UARTFEINTR interrupt.
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const FEIC_OFFSET: u32 = 7;
    pub const FEIC_MASK: u32 = 0x00000080;
    pub const FEIC: u32 = 1 << 7;

    /// **RTIC**
    ///
    /// Receive timeout interrupt clear. Clears the UARTRTINTR interrupt.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const RTIC_OFFSET: u32 = 6;
    pub const RTIC_MASK: u32 = 0x00000040;
    pub const RTIC: u32 = 1 << 6;

    /// **TXIC**
    ///
    /// Transmit interrupt clear. Clears the UARTTXINTR interrupt.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const TXIC_OFFSET: u32 = 5;
    pub const TXIC_MASK: u32 = 0x00000020;
    pub const TXIC: u32 = 1 << 5;

    /// **RXIC**
    ///
    /// Receive interrupt clear. Clears the UARTRXINTR interrupt.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const RXIC_OFFSET: u32 = 4;
    pub const RXIC_MASK: u32 = 0x00000010;
    pub const RXIC: u32 = 1 << 4;

    /// **DSRMIC**
    ///
    /// nUARTDSR modem interrupt clear. Clears the UARTDSRINTR interrupt.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const DSRMIC_OFFSET: u32 = 3;
    pub const DSRMIC_MASK: u32 = 0x00000008;
    pub const DSRMIC: u32 = 1 << 3;

    /// **DCDMIC**
    ///
    /// nUARTDCD modem interrupt clear. Clears the UARTDCDINTR interrupt.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const DCDMIC_OFFSET: u32 = 2;
    pub const DCDMIC_MASK: u32 = 0x00000004;
    pub const DCDMIC: u32 = 1 << 2;

    /// **CTSMIC**
    ///
    /// nUARTCTS modem interrupt clear. Clears the UARTCTSINTR interrupt.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const CTSMIC_OFFSET: u32 = 1;
    pub const CTSMIC_MASK: u32 = 0x00000002;
    pub const CTSMIC: u32 = 1 << 1;

    /// **RIMIC**
    ///
    /// nUARTRI modem interrupt clear. Clears the UARTRIINTR interrupt.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const RIMIC_OFFSET: u32 = 0;
    pub const RIMIC_MASK: u32 = 0x00000001;
    pub const RIMIC: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **OEIC** `[10]` (RW)
    ///
    /// Overrun error interrupt clear. Clears the UARTOEINTR interrupt.
    #[inline(always)]
    pub fn oeic(&self) -> bool {
        (self.value.read_bits() & Self::OEIC_MASK) != 0
    }

    /// Set OEIC bit atomically (RW)
    #[inline(always)]
    pub fn set_oeic(&self) {
        self.set_bits(Self::OEIC_MASK);
    }

    /// Clear OEIC bit atomically (RW)
    #[inline(always)]
    pub fn clr_oeic(&self) {
        self.clr_bits(Self::OEIC_MASK);
    }

    /// **BEIC** `[9]` (RW)
    ///
    /// Break error interrupt clear. Clears the UARTBEINTR interrupt.
    #[inline(always)]
    pub fn beic(&self) -> bool {
        (self.value.read_bits() & Self::BEIC_MASK) != 0
    }

    /// Set BEIC bit atomically (RW)
    #[inline(always)]
    pub fn set_beic(&self) {
        self.set_bits(Self::BEIC_MASK);
    }

    /// Clear BEIC bit atomically (RW)
    #[inline(always)]
    pub fn clr_beic(&self) {
        self.clr_bits(Self::BEIC_MASK);
    }

    /// **PEIC** `[8]` (RW)
    ///
    /// Parity error interrupt clear. Clears the UARTPEINTR interrupt.
    #[inline(always)]
    pub fn peic(&self) -> bool {
        (self.value.read_bits() & Self::PEIC_MASK) != 0
    }

    /// Set PEIC bit atomically (RW)
    #[inline(always)]
    pub fn set_peic(&self) {
        self.set_bits(Self::PEIC_MASK);
    }

    /// Clear PEIC bit atomically (RW)
    #[inline(always)]
    pub fn clr_peic(&self) {
        self.clr_bits(Self::PEIC_MASK);
    }

    /// **FEIC** `[7]` (RW)
    ///
    /// Framing error interrupt clear. Clears the UARTFEINTR interrupt.
    #[inline(always)]
    pub fn feic(&self) -> bool {
        (self.value.read_bits() & Self::FEIC_MASK) != 0
    }

    /// Set FEIC bit atomically (RW)
    #[inline(always)]
    pub fn set_feic(&self) {
        self.set_bits(Self::FEIC_MASK);
    }

    /// Clear FEIC bit atomically (RW)
    #[inline(always)]
    pub fn clr_feic(&self) {
        self.clr_bits(Self::FEIC_MASK);
    }

    /// **RTIC** `[6]` (RW)
    ///
    /// Receive timeout interrupt clear. Clears the UARTRTINTR interrupt.
    #[inline(always)]
    pub fn rtic(&self) -> bool {
        (self.value.read_bits() & Self::RTIC_MASK) != 0
    }

    /// Set RTIC bit atomically (RW)
    #[inline(always)]
    pub fn set_rtic(&self) {
        self.set_bits(Self::RTIC_MASK);
    }

    /// Clear RTIC bit atomically (RW)
    #[inline(always)]
    pub fn clr_rtic(&self) {
        self.clr_bits(Self::RTIC_MASK);
    }

    /// **TXIC** `[5]` (RW)
    ///
    /// Transmit interrupt clear. Clears the UARTTXINTR interrupt.
    #[inline(always)]
    pub fn txic(&self) -> bool {
        (self.value.read_bits() & Self::TXIC_MASK) != 0
    }

    /// Set TXIC bit atomically (RW)
    #[inline(always)]
    pub fn set_txic(&self) {
        self.set_bits(Self::TXIC_MASK);
    }

    /// Clear TXIC bit atomically (RW)
    #[inline(always)]
    pub fn clr_txic(&self) {
        self.clr_bits(Self::TXIC_MASK);
    }

    /// **RXIC** `[4]` (RW)
    ///
    /// Receive interrupt clear. Clears the UARTRXINTR interrupt.
    #[inline(always)]
    pub fn rxic(&self) -> bool {
        (self.value.read_bits() & Self::RXIC_MASK) != 0
    }

    /// Set RXIC bit atomically (RW)
    #[inline(always)]
    pub fn set_rxic(&self) {
        self.set_bits(Self::RXIC_MASK);
    }

    /// Clear RXIC bit atomically (RW)
    #[inline(always)]
    pub fn clr_rxic(&self) {
        self.clr_bits(Self::RXIC_MASK);
    }

    /// **DSRMIC** `[3]` (RW)
    ///
    /// nUARTDSR modem interrupt clear. Clears the UARTDSRINTR interrupt.
    #[inline(always)]
    pub fn dsrmic(&self) -> bool {
        (self.value.read_bits() & Self::DSRMIC_MASK) != 0
    }

    /// Set DSRMIC bit atomically (RW)
    #[inline(always)]
    pub fn set_dsrmic(&self) {
        self.set_bits(Self::DSRMIC_MASK);
    }

    /// Clear DSRMIC bit atomically (RW)
    #[inline(always)]
    pub fn clr_dsrmic(&self) {
        self.clr_bits(Self::DSRMIC_MASK);
    }

    /// **DCDMIC** `[2]` (RW)
    ///
    /// nUARTDCD modem interrupt clear. Clears the UARTDCDINTR interrupt.
    #[inline(always)]
    pub fn dcdmic(&self) -> bool {
        (self.value.read_bits() & Self::DCDMIC_MASK) != 0
    }

    /// Set DCDMIC bit atomically (RW)
    #[inline(always)]
    pub fn set_dcdmic(&self) {
        self.set_bits(Self::DCDMIC_MASK);
    }

    /// Clear DCDMIC bit atomically (RW)
    #[inline(always)]
    pub fn clr_dcdmic(&self) {
        self.clr_bits(Self::DCDMIC_MASK);
    }

    /// **CTSMIC** `[1]` (RW)
    ///
    /// nUARTCTS modem interrupt clear. Clears the UARTCTSINTR interrupt.
    #[inline(always)]
    pub fn ctsmic(&self) -> bool {
        (self.value.read_bits() & Self::CTSMIC_MASK) != 0
    }

    /// Set CTSMIC bit atomically (RW)
    #[inline(always)]
    pub fn set_ctsmic(&self) {
        self.set_bits(Self::CTSMIC_MASK);
    }

    /// Clear CTSMIC bit atomically (RW)
    #[inline(always)]
    pub fn clr_ctsmic(&self) {
        self.clr_bits(Self::CTSMIC_MASK);
    }

    /// **RIMIC** `[0]` (RW)
    ///
    /// nUARTRI modem interrupt clear. Clears the UARTRIINTR interrupt.
    #[inline(always)]
    pub fn rimic(&self) -> bool {
        (self.value.read_bits() & Self::RIMIC_MASK) != 0
    }

    /// Set RIMIC bit atomically (RW)
    #[inline(always)]
    pub fn set_rimic(&self) {
        self.set_bits(Self::RIMIC_MASK);
    }

    /// Clear RIMIC bit atomically (RW)
    #[inline(always)]
    pub fn clr_rimic(&self) {
        self.clr_bits(Self::RIMIC_MASK);
    }
}

/// UARTICR value type for batch read/modify/write operations
///
/// Use `uarticr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `uarticr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct UarticrVal {
    bits: u32,
}

impl UarticrVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x00000000 }
    }

    /// Read **OEIC** `[10:10]`
    ///
    /// Overrun error interrupt clear. Clears the UARTOEINTR interrupt.
    #[inline(always)]
    pub const fn oeic(&self) -> bool {
        (self.bits & Uarticr::OEIC_MASK) != 0
    }

    /// Read **BEIC** `[9:9]`
    ///
    /// Break error interrupt clear. Clears the UARTBEINTR interrupt.
    #[inline(always)]
    pub const fn beic(&self) -> bool {
        (self.bits & Uarticr::BEIC_MASK) != 0
    }

    /// Read **PEIC** `[8:8]`
    ///
    /// Parity error interrupt clear. Clears the UARTPEINTR interrupt.
    #[inline(always)]
    pub const fn peic(&self) -> bool {
        (self.bits & Uarticr::PEIC_MASK) != 0
    }

    /// Read **FEIC** `[7:7]`
    ///
    /// Framing error interrupt clear. Clears the UARTFEINTR interrupt.
    #[inline(always)]
    pub const fn feic(&self) -> bool {
        (self.bits & Uarticr::FEIC_MASK) != 0
    }

    /// Read **RTIC** `[6:6]`
    ///
    /// Receive timeout interrupt clear. Clears the UARTRTINTR interrupt.
    #[inline(always)]
    pub const fn rtic(&self) -> bool {
        (self.bits & Uarticr::RTIC_MASK) != 0
    }

    /// Read **TXIC** `[5:5]`
    ///
    /// Transmit interrupt clear. Clears the UARTTXINTR interrupt.
    #[inline(always)]
    pub const fn txic(&self) -> bool {
        (self.bits & Uarticr::TXIC_MASK) != 0
    }

    /// Read **RXIC** `[4:4]`
    ///
    /// Receive interrupt clear. Clears the UARTRXINTR interrupt.
    #[inline(always)]
    pub const fn rxic(&self) -> bool {
        (self.bits & Uarticr::RXIC_MASK) != 0
    }

    /// Read **DSRMIC** `[3:3]`
    ///
    /// nUARTDSR modem interrupt clear. Clears the UARTDSRINTR interrupt.
    #[inline(always)]
    pub const fn dsrmic(&self) -> bool {
        (self.bits & Uarticr::DSRMIC_MASK) != 0
    }

    /// Read **DCDMIC** `[2:2]`
    ///
    /// nUARTDCD modem interrupt clear. Clears the UARTDCDINTR interrupt.
    #[inline(always)]
    pub const fn dcdmic(&self) -> bool {
        (self.bits & Uarticr::DCDMIC_MASK) != 0
    }

    /// Read **CTSMIC** `[1:1]`
    ///
    /// nUARTCTS modem interrupt clear. Clears the UARTCTSINTR interrupt.
    #[inline(always)]
    pub const fn ctsmic(&self) -> bool {
        (self.bits & Uarticr::CTSMIC_MASK) != 0
    }

    /// Read **RIMIC** `[0:0]`
    ///
    /// nUARTRI modem interrupt clear. Clears the UARTRIINTR interrupt.
    #[inline(always)]
    pub const fn rimic(&self) -> bool {
        (self.bits & Uarticr::RIMIC_MASK) != 0
    }

    /// Set **OEIC** `[10:10]`
    ///
    /// Overrun error interrupt clear. Clears the UARTOEINTR interrupt.
    #[inline(always)]
    pub const fn with_oeic(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Uarticr::OEIC_MASK
            } else {
                self.bits & !Uarticr::OEIC_MASK
            },
        }
    }

    /// Set **BEIC** `[9:9]`
    ///
    /// Break error interrupt clear. Clears the UARTBEINTR interrupt.
    #[inline(always)]
    pub const fn with_beic(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Uarticr::BEIC_MASK
            } else {
                self.bits & !Uarticr::BEIC_MASK
            },
        }
    }

    /// Set **PEIC** `[8:8]`
    ///
    /// Parity error interrupt clear. Clears the UARTPEINTR interrupt.
    #[inline(always)]
    pub const fn with_peic(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Uarticr::PEIC_MASK
            } else {
                self.bits & !Uarticr::PEIC_MASK
            },
        }
    }

    /// Set **FEIC** `[7:7]`
    ///
    /// Framing error interrupt clear. Clears the UARTFEINTR interrupt.
    #[inline(always)]
    pub const fn with_feic(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Uarticr::FEIC_MASK
            } else {
                self.bits & !Uarticr::FEIC_MASK
            },
        }
    }

    /// Set **RTIC** `[6:6]`
    ///
    /// Receive timeout interrupt clear. Clears the UARTRTINTR interrupt.
    #[inline(always)]
    pub const fn with_rtic(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Uarticr::RTIC_MASK
            } else {
                self.bits & !Uarticr::RTIC_MASK
            },
        }
    }

    /// Set **TXIC** `[5:5]`
    ///
    /// Transmit interrupt clear. Clears the UARTTXINTR interrupt.
    #[inline(always)]
    pub const fn with_txic(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Uarticr::TXIC_MASK
            } else {
                self.bits & !Uarticr::TXIC_MASK
            },
        }
    }

    /// Set **RXIC** `[4:4]`
    ///
    /// Receive interrupt clear. Clears the UARTRXINTR interrupt.
    #[inline(always)]
    pub const fn with_rxic(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Uarticr::RXIC_MASK
            } else {
                self.bits & !Uarticr::RXIC_MASK
            },
        }
    }

    /// Set **DSRMIC** `[3:3]`
    ///
    /// nUARTDSR modem interrupt clear. Clears the UARTDSRINTR interrupt.
    #[inline(always)]
    pub const fn with_dsrmic(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Uarticr::DSRMIC_MASK
            } else {
                self.bits & !Uarticr::DSRMIC_MASK
            },
        }
    }

    /// Set **DCDMIC** `[2:2]`
    ///
    /// nUARTDCD modem interrupt clear. Clears the UARTDCDINTR interrupt.
    #[inline(always)]
    pub const fn with_dcdmic(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Uarticr::DCDMIC_MASK
            } else {
                self.bits & !Uarticr::DCDMIC_MASK
            },
        }
    }

    /// Set **CTSMIC** `[1:1]`
    ///
    /// nUARTCTS modem interrupt clear. Clears the UARTCTSINTR interrupt.
    #[inline(always)]
    pub const fn with_ctsmic(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Uarticr::CTSMIC_MASK
            } else {
                self.bits & !Uarticr::CTSMIC_MASK
            },
        }
    }

    /// Set **RIMIC** `[0:0]`
    ///
    /// nUARTRI modem interrupt clear. Clears the UARTRIINTR interrupt.
    #[inline(always)]
    pub const fn with_rimic(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Uarticr::RIMIC_MASK
            } else {
                self.bits & !Uarticr::RIMIC_MASK
            },
        }
    }
}

impl Default for UarticrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Uarticr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> UarticrVal {
        UarticrVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: UarticrVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(UarticrVal) -> UarticrVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// UARTDMACR Register
///
/// DMA Control Register, UARTDMACR
///
/// Address offset: 0x0048
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **DMAONERR** `[2]` (RW)
///
///   DMA on error. If this bit is set to 1, the DMA receive request outputs, UARTRXDMASREQ or UARTRXDMABREQ, are disabled when the UART error interrupt is asserted.
///
/// - **TXDMAE** `[1]` (RW)
///
///   Transmit DMA enable. If this bit is set to 1, DMA for the transmit FIFO is enabled.
///
/// - **RXDMAE** `[0]` (RW)
///
///   Receive DMA enable. If this bit is set to 1, DMA for the receive FIFO is enabled.
#[repr(transparent)]
pub struct Uartdmacr {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Uartdmacr {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Uartdmacr {
    // ======== Field Constants ========

    /// **DMAONERR**
    ///
    /// DMA on error. If this bit is set to 1, the DMA receive request outputs, UARTRXDMASREQ or UARTRXDMABREQ, are disabled when the UART error interrupt is asserted.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const DMAONERR_OFFSET: u32 = 2;
    pub const DMAONERR_MASK: u32 = 0x00000004;
    pub const DMAONERR: u32 = 1 << 2;

    /// **TXDMAE**
    ///
    /// Transmit DMA enable. If this bit is set to 1, DMA for the transmit FIFO is enabled.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const TXDMAE_OFFSET: u32 = 1;
    pub const TXDMAE_MASK: u32 = 0x00000002;
    pub const TXDMAE: u32 = 1 << 1;

    /// **RXDMAE**
    ///
    /// Receive DMA enable. If this bit is set to 1, DMA for the receive FIFO is enabled.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const RXDMAE_OFFSET: u32 = 0;
    pub const RXDMAE_MASK: u32 = 0x00000001;
    pub const RXDMAE: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **DMAONERR** `[2]` (RW)
    ///
    /// DMA on error. If this bit is set to 1, the DMA receive request outputs, UARTRXDMASREQ or UARTRXDMABREQ, are disabled when the UART error interrupt is asserted.
    #[inline(always)]
    pub fn dmaonerr(&self) -> bool {
        (self.value.read_bits() & Self::DMAONERR_MASK) != 0
    }

    /// Set DMAONERR bit atomically (RW)
    #[inline(always)]
    pub fn set_dmaonerr(&self) {
        self.set_bits(Self::DMAONERR_MASK);
    }

    /// Clear DMAONERR bit atomically (RW)
    #[inline(always)]
    pub fn clr_dmaonerr(&self) {
        self.clr_bits(Self::DMAONERR_MASK);
    }

    /// **TXDMAE** `[1]` (RW)
    ///
    /// Transmit DMA enable. If this bit is set to 1, DMA for the transmit FIFO is enabled.
    #[inline(always)]
    pub fn txdmae(&self) -> bool {
        (self.value.read_bits() & Self::TXDMAE_MASK) != 0
    }

    /// Set TXDMAE bit atomically (RW)
    #[inline(always)]
    pub fn set_txdmae(&self) {
        self.set_bits(Self::TXDMAE_MASK);
    }

    /// Clear TXDMAE bit atomically (RW)
    #[inline(always)]
    pub fn clr_txdmae(&self) {
        self.clr_bits(Self::TXDMAE_MASK);
    }

    /// **RXDMAE** `[0]` (RW)
    ///
    /// Receive DMA enable. If this bit is set to 1, DMA for the receive FIFO is enabled.
    #[inline(always)]
    pub fn rxdmae(&self) -> bool {
        (self.value.read_bits() & Self::RXDMAE_MASK) != 0
    }

    /// Set RXDMAE bit atomically (RW)
    #[inline(always)]
    pub fn set_rxdmae(&self) {
        self.set_bits(Self::RXDMAE_MASK);
    }

    /// Clear RXDMAE bit atomically (RW)
    #[inline(always)]
    pub fn clr_rxdmae(&self) {
        self.clr_bits(Self::RXDMAE_MASK);
    }
}

/// UARTDMACR value type for batch read/modify/write operations
///
/// Use `uartdmacr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `uartdmacr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct UartdmacrVal {
    bits: u32,
}

impl UartdmacrVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x00000000 }
    }

    /// Read **DMAONERR** `[2:2]`
    ///
    /// DMA on error. If this bit is set to 1, the DMA receive request outputs, UARTRXDMASREQ or UARTRXDMABREQ, are disabled when the UART error interrupt is asserted.
    #[inline(always)]
    pub const fn dmaonerr(&self) -> bool {
        (self.bits & Uartdmacr::DMAONERR_MASK) != 0
    }

    /// Read **TXDMAE** `[1:1]`
    ///
    /// Transmit DMA enable. If this bit is set to 1, DMA for the transmit FIFO is enabled.
    #[inline(always)]
    pub const fn txdmae(&self) -> bool {
        (self.bits & Uartdmacr::TXDMAE_MASK) != 0
    }

    /// Read **RXDMAE** `[0:0]`
    ///
    /// Receive DMA enable. If this bit is set to 1, DMA for the receive FIFO is enabled.
    #[inline(always)]
    pub const fn rxdmae(&self) -> bool {
        (self.bits & Uartdmacr::RXDMAE_MASK) != 0
    }

    /// Set **DMAONERR** `[2:2]`
    ///
    /// DMA on error. If this bit is set to 1, the DMA receive request outputs, UARTRXDMASREQ or UARTRXDMABREQ, are disabled when the UART error interrupt is asserted.
    #[inline(always)]
    pub const fn with_dmaonerr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Uartdmacr::DMAONERR_MASK
            } else {
                self.bits & !Uartdmacr::DMAONERR_MASK
            },
        }
    }

    /// Set **TXDMAE** `[1:1]`
    ///
    /// Transmit DMA enable. If this bit is set to 1, DMA for the transmit FIFO is enabled.
    #[inline(always)]
    pub const fn with_txdmae(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Uartdmacr::TXDMAE_MASK
            } else {
                self.bits & !Uartdmacr::TXDMAE_MASK
            },
        }
    }

    /// Set **RXDMAE** `[0:0]`
    ///
    /// Receive DMA enable. If this bit is set to 1, DMA for the receive FIFO is enabled.
    #[inline(always)]
    pub const fn with_rxdmae(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Uartdmacr::RXDMAE_MASK
            } else {
                self.bits & !Uartdmacr::RXDMAE_MASK
            },
        }
    }
}

impl Default for UartdmacrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Uartdmacr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> UartdmacrVal {
        UartdmacrVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: UartdmacrVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(UartdmacrVal) -> UartdmacrVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// UARTPERIPHID0 Register
///
/// UARTPeriphID0 Register
///
/// Address offset: 0x0fe0
/// Reset value: 0x00000011
///
/// ## Fields
///
/// - **PARTNUMBER0** `[7:0]` (RO)
///
///   These bits read back as 0x11
#[repr(transparent)]
pub struct Uartperiphid0 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Uartperiphid0 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Uartperiphid0 {
    // ======== Field Constants ========

    /// **PARTNUMBER0**
    ///
    /// These bits read back as 0x11
    ///
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const PARTNUMBER0_OFFSET: u32 = 0;
    pub const PARTNUMBER0_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// **PARTNUMBER0** `[7:0]` (RO)
    ///
    /// These bits read back as 0x11
    #[inline(always)]
    pub fn partnumber0(&self) -> u32 {
        (self.value.read_bits() & Self::PARTNUMBER0_MASK) >> Self::PARTNUMBER0_OFFSET
    }
}

/// UARTPERIPHID0 value type for batch read/modify/write operations
///
/// Use `uartperiphid0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `uartperiphid0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Uartperiphid0Val {
    bits: u32,
}

impl Uartperiphid0Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x00000011 }
    }

    /// Read **PARTNUMBER0** `[7:0]`
    ///
    /// These bits read back as 0x11
    #[inline(always)]
    pub const fn partnumber0(&self) -> u32 {
        (self.bits & Uartperiphid0::PARTNUMBER0_MASK) >> Uartperiphid0::PARTNUMBER0_OFFSET
    }
}

impl Default for Uartperiphid0Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Uartperiphid0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Uartperiphid0Val {
        Uartperiphid0Val::from_bits(self.value.read_bits())
    }
}

/// UARTPERIPHID1 Register
///
/// UARTPeriphID1 Register
///
/// Address offset: 0x0fe4
/// Reset value: 0x00000010
///
/// ## Fields
///
/// - **DESIGNER0** `[7:4]` (RO)
///
///   These bits read back as 0x1
///
/// - **PARTNUMBER1** `[3:0]` (RO)
///
///   These bits read back as 0x0
#[repr(transparent)]
pub struct Uartperiphid1 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Uartperiphid1 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Uartperiphid1 {
    // ======== Field Constants ========

    /// **DESIGNER0**
    ///
    /// These bits read back as 0x1
    ///
    /// Bits `[7:4]`, mask `0x000000f0`
    pub const DESIGNER0_OFFSET: u32 = 4;
    pub const DESIGNER0_MASK: u32 = 0x000000f0;

    /// **PARTNUMBER1**
    ///
    /// These bits read back as 0x0
    ///
    /// Bits `[3:0]`, mask `0x0000000f`
    pub const PARTNUMBER1_OFFSET: u32 = 0;
    pub const PARTNUMBER1_MASK: u32 = 0x0000000f;

    // ======== Field Accessors ========

    /// **DESIGNER0** `[7:4]` (RO)
    ///
    /// These bits read back as 0x1
    #[inline(always)]
    pub fn designer0(&self) -> u32 {
        (self.value.read_bits() & Self::DESIGNER0_MASK) >> Self::DESIGNER0_OFFSET
    }

    /// **PARTNUMBER1** `[3:0]` (RO)
    ///
    /// These bits read back as 0x0
    #[inline(always)]
    pub fn partnumber1(&self) -> u32 {
        (self.value.read_bits() & Self::PARTNUMBER1_MASK) >> Self::PARTNUMBER1_OFFSET
    }
}

/// UARTPERIPHID1 value type for batch read/modify/write operations
///
/// Use `uartperiphid1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `uartperiphid1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Uartperiphid1Val {
    bits: u32,
}

impl Uartperiphid1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x00000010 }
    }

    /// Read **DESIGNER0** `[7:4]`
    ///
    /// These bits read back as 0x1
    #[inline(always)]
    pub const fn designer0(&self) -> u32 {
        (self.bits & Uartperiphid1::DESIGNER0_MASK) >> Uartperiphid1::DESIGNER0_OFFSET
    }

    /// Read **PARTNUMBER1** `[3:0]`
    ///
    /// These bits read back as 0x0
    #[inline(always)]
    pub const fn partnumber1(&self) -> u32 {
        (self.bits & Uartperiphid1::PARTNUMBER1_MASK) >> Uartperiphid1::PARTNUMBER1_OFFSET
    }
}

impl Default for Uartperiphid1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Uartperiphid1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Uartperiphid1Val {
        Uartperiphid1Val::from_bits(self.value.read_bits())
    }
}

/// UARTPERIPHID2 Register
///
/// UARTPeriphID2 Register
///
/// Address offset: 0x0fe8
/// Reset value: 0x00000034
///
/// ## Fields
///
/// - **REVISION** `[7:4]` (RO)
///
///   This field depends on the revision of the UART: r1p0 0x0 r1p1 0x1 r1p3 0x2 r1p4 0x2 r1p5 0x3
///
/// - **DESIGNER1** `[3:0]` (RO)
///
///   These bits read back as 0x4
#[repr(transparent)]
pub struct Uartperiphid2 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Uartperiphid2 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Uartperiphid2 {
    // ======== Field Constants ========

    /// **REVISION**
    ///
    /// This field depends on the revision of the UART: r1p0 0x0 r1p1 0x1 r1p3 0x2 r1p4 0x2 r1p5 0x3
    ///
    /// Bits `[7:4]`, mask `0x000000f0`
    pub const REVISION_OFFSET: u32 = 4;
    pub const REVISION_MASK: u32 = 0x000000f0;

    /// **DESIGNER1**
    ///
    /// These bits read back as 0x4
    ///
    /// Bits `[3:0]`, mask `0x0000000f`
    pub const DESIGNER1_OFFSET: u32 = 0;
    pub const DESIGNER1_MASK: u32 = 0x0000000f;

    // ======== Field Accessors ========

    /// **REVISION** `[7:4]` (RO)
    ///
    /// This field depends on the revision of the UART: r1p0 0x0 r1p1 0x1 r1p3 0x2 r1p4 0x2 r1p5 0x3
    #[inline(always)]
    pub fn revision(&self) -> u32 {
        (self.value.read_bits() & Self::REVISION_MASK) >> Self::REVISION_OFFSET
    }

    /// **DESIGNER1** `[3:0]` (RO)
    ///
    /// These bits read back as 0x4
    #[inline(always)]
    pub fn designer1(&self) -> u32 {
        (self.value.read_bits() & Self::DESIGNER1_MASK) >> Self::DESIGNER1_OFFSET
    }
}

/// UARTPERIPHID2 value type for batch read/modify/write operations
///
/// Use `uartperiphid2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `uartperiphid2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Uartperiphid2Val {
    bits: u32,
}

impl Uartperiphid2Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x00000034 }
    }

    /// Read **REVISION** `[7:4]`
    ///
    /// This field depends on the revision of the UART: r1p0 0x0 r1p1 0x1 r1p3 0x2 r1p4 0x2 r1p5 0x3
    #[inline(always)]
    pub const fn revision(&self) -> u32 {
        (self.bits & Uartperiphid2::REVISION_MASK) >> Uartperiphid2::REVISION_OFFSET
    }

    /// Read **DESIGNER1** `[3:0]`
    ///
    /// These bits read back as 0x4
    #[inline(always)]
    pub const fn designer1(&self) -> u32 {
        (self.bits & Uartperiphid2::DESIGNER1_MASK) >> Uartperiphid2::DESIGNER1_OFFSET
    }
}

impl Default for Uartperiphid2Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Uartperiphid2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Uartperiphid2Val {
        Uartperiphid2Val::from_bits(self.value.read_bits())
    }
}

/// UARTPERIPHID3 Register
///
/// UARTPeriphID3 Register
///
/// Address offset: 0x0fec
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **CONFIGURATION** `[7:0]` (RO)
///
///   These bits read back as 0x00
#[repr(transparent)]
pub struct Uartperiphid3 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Uartperiphid3 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Uartperiphid3 {
    // ======== Field Constants ========

    /// **CONFIGURATION**
    ///
    /// These bits read back as 0x00
    ///
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const CONFIGURATION_OFFSET: u32 = 0;
    pub const CONFIGURATION_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// **CONFIGURATION** `[7:0]` (RO)
    ///
    /// These bits read back as 0x00
    #[inline(always)]
    pub fn configuration(&self) -> u32 {
        (self.value.read_bits() & Self::CONFIGURATION_MASK) >> Self::CONFIGURATION_OFFSET
    }
}

/// UARTPERIPHID3 value type for batch read/modify/write operations
///
/// Use `uartperiphid3.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `uartperiphid3.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Uartperiphid3Val {
    bits: u32,
}

impl Uartperiphid3Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x00000000 }
    }

    /// Read **CONFIGURATION** `[7:0]`
    ///
    /// These bits read back as 0x00
    #[inline(always)]
    pub const fn configuration(&self) -> u32 {
        (self.bits & Uartperiphid3::CONFIGURATION_MASK) >> Uartperiphid3::CONFIGURATION_OFFSET
    }
}

impl Default for Uartperiphid3Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Uartperiphid3 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Uartperiphid3Val {
        Uartperiphid3Val::from_bits(self.value.read_bits())
    }
}

/// UARTPCELLID0 Register
///
/// UARTPCellID0 Register
///
/// Address offset: 0x0ff0
/// Reset value: 0x0000000d
///
/// ## Fields
///
/// - **UARTPCELLID0** `[7:0]` (RO)
///
///   These bits read back as 0x0D
#[repr(transparent)]
pub struct Uartpcellid0 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Uartpcellid0 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Uartpcellid0 {
    // ======== Field Constants ========

    /// **UARTPCELLID0**
    ///
    /// These bits read back as 0x0D
    ///
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const UARTPCELLID0_OFFSET: u32 = 0;
    pub const UARTPCELLID0_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// **UARTPCELLID0** `[7:0]` (RO)
    ///
    /// These bits read back as 0x0D
    #[inline(always)]
    pub fn uartpcellid0(&self) -> u32 {
        (self.value.read_bits() & Self::UARTPCELLID0_MASK) >> Self::UARTPCELLID0_OFFSET
    }
}

/// UARTPCELLID0 value type for batch read/modify/write operations
///
/// Use `uartpcellid0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `uartpcellid0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Uartpcellid0Val {
    bits: u32,
}

impl Uartpcellid0Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x0000000d }
    }

    /// Read **UARTPCELLID0** `[7:0]`
    ///
    /// These bits read back as 0x0D
    #[inline(always)]
    pub const fn uartpcellid0(&self) -> u32 {
        (self.bits & Uartpcellid0::UARTPCELLID0_MASK) >> Uartpcellid0::UARTPCELLID0_OFFSET
    }
}

impl Default for Uartpcellid0Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Uartpcellid0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Uartpcellid0Val {
        Uartpcellid0Val::from_bits(self.value.read_bits())
    }
}

/// UARTPCELLID1 Register
///
/// UARTPCellID1 Register
///
/// Address offset: 0x0ff4
/// Reset value: 0x000000f0
///
/// ## Fields
///
/// - **UARTPCELLID1** `[7:0]` (RO)
///
///   These bits read back as 0xF0
#[repr(transparent)]
pub struct Uartpcellid1 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Uartpcellid1 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Uartpcellid1 {
    // ======== Field Constants ========

    /// **UARTPCELLID1**
    ///
    /// These bits read back as 0xF0
    ///
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const UARTPCELLID1_OFFSET: u32 = 0;
    pub const UARTPCELLID1_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// **UARTPCELLID1** `[7:0]` (RO)
    ///
    /// These bits read back as 0xF0
    #[inline(always)]
    pub fn uartpcellid1(&self) -> u32 {
        (self.value.read_bits() & Self::UARTPCELLID1_MASK) >> Self::UARTPCELLID1_OFFSET
    }
}

/// UARTPCELLID1 value type for batch read/modify/write operations
///
/// Use `uartpcellid1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `uartpcellid1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Uartpcellid1Val {
    bits: u32,
}

impl Uartpcellid1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x000000f0 }
    }

    /// Read **UARTPCELLID1** `[7:0]`
    ///
    /// These bits read back as 0xF0
    #[inline(always)]
    pub const fn uartpcellid1(&self) -> u32 {
        (self.bits & Uartpcellid1::UARTPCELLID1_MASK) >> Uartpcellid1::UARTPCELLID1_OFFSET
    }
}

impl Default for Uartpcellid1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Uartpcellid1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Uartpcellid1Val {
        Uartpcellid1Val::from_bits(self.value.read_bits())
    }
}

/// UARTPCELLID2 Register
///
/// UARTPCellID2 Register
///
/// Address offset: 0x0ff8
/// Reset value: 0x00000005
///
/// ## Fields
///
/// - **UARTPCELLID2** `[7:0]` (RO)
///
///   These bits read back as 0x05
#[repr(transparent)]
pub struct Uartpcellid2 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Uartpcellid2 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Uartpcellid2 {
    // ======== Field Constants ========

    /// **UARTPCELLID2**
    ///
    /// These bits read back as 0x05
    ///
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const UARTPCELLID2_OFFSET: u32 = 0;
    pub const UARTPCELLID2_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// **UARTPCELLID2** `[7:0]` (RO)
    ///
    /// These bits read back as 0x05
    #[inline(always)]
    pub fn uartpcellid2(&self) -> u32 {
        (self.value.read_bits() & Self::UARTPCELLID2_MASK) >> Self::UARTPCELLID2_OFFSET
    }
}

/// UARTPCELLID2 value type for batch read/modify/write operations
///
/// Use `uartpcellid2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `uartpcellid2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Uartpcellid2Val {
    bits: u32,
}

impl Uartpcellid2Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x00000005 }
    }

    /// Read **UARTPCELLID2** `[7:0]`
    ///
    /// These bits read back as 0x05
    #[inline(always)]
    pub const fn uartpcellid2(&self) -> u32 {
        (self.bits & Uartpcellid2::UARTPCELLID2_MASK) >> Uartpcellid2::UARTPCELLID2_OFFSET
    }
}

impl Default for Uartpcellid2Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Uartpcellid2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Uartpcellid2Val {
        Uartpcellid2Val::from_bits(self.value.read_bits())
    }
}

/// UARTPCELLID3 Register
///
/// UARTPCellID3 Register
///
/// Address offset: 0x0ffc
/// Reset value: 0x000000b1
///
/// ## Fields
///
/// - **UARTPCELLID3** `[7:0]` (RO)
///
///   These bits read back as 0xB1
#[repr(transparent)]
pub struct Uartpcellid3 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Uartpcellid3 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Uartpcellid3 {
    // ======== Field Constants ========

    /// **UARTPCELLID3**
    ///
    /// These bits read back as 0xB1
    ///
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const UARTPCELLID3_OFFSET: u32 = 0;
    pub const UARTPCELLID3_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// **UARTPCELLID3** `[7:0]` (RO)
    ///
    /// These bits read back as 0xB1
    #[inline(always)]
    pub fn uartpcellid3(&self) -> u32 {
        (self.value.read_bits() & Self::UARTPCELLID3_MASK) >> Self::UARTPCELLID3_OFFSET
    }
}

/// UARTPCELLID3 value type for batch read/modify/write operations
///
/// Use `uartpcellid3.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `uartpcellid3.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Uartpcellid3Val {
    bits: u32,
}

impl Uartpcellid3Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x000000b1 }
    }

    /// Read **UARTPCELLID3** `[7:0]`
    ///
    /// These bits read back as 0xB1
    #[inline(always)]
    pub const fn uartpcellid3(&self) -> u32 {
        (self.bits & Uartpcellid3::UARTPCELLID3_MASK) >> Uartpcellid3::UARTPCELLID3_OFFSET
    }
}

impl Default for Uartpcellid3Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Uartpcellid3 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Uartpcellid3Val {
        Uartpcellid3Val::from_bits(self.value.read_bits())
    }
}

/// Register block
#[repr(C)]
pub struct RegisterBlock {
    /// 0x0000
    /// Data Register, UARTDR
    pub uartdr: Uartdr,
    /// 0x0004
    /// Receive Status Register/Error Clear Register, UARTRSR/UARTECR
    pub uartrsr: Uartrsr,
    _reserved0: [u8; 16],
    /// 0x0018
    /// Flag Register, UARTFR
    pub uartfr: Uartfr,
    _reserved1: [u8; 4],
    /// 0x0020
    /// IrDA Low-Power Counter Register, UARTILPR
    pub uartilpr: Uartilpr,
    /// 0x0024
    /// Integer Baud Rate Register, UARTIBRD
    pub uartibrd: Uartibrd,
    /// 0x0028
    /// Fractional Baud Rate Register, UARTFBRD
    pub uartfbrd: Uartfbrd,
    /// 0x002c
    /// Line Control Register, UARTLCR_H
    pub uartlcr_h: UartlcrH,
    /// 0x0030
    /// Control Register, UARTCR
    pub uartcr: Uartcr,
    /// 0x0034
    /// Interrupt FIFO Level Select Register, UARTIFLS
    pub uartifls: Uartifls,
    /// 0x0038
    /// Interrupt Mask Set/Clear Register, UARTIMSC
    pub uartimsc: Uartimsc,
    /// 0x003c
    /// Raw Interrupt Status Register, UARTRIS
    pub uartris: Uartris,
    /// 0x0040
    /// Masked Interrupt Status Register, UARTMIS
    pub uartmis: Uartmis,
    /// 0x0044
    /// Interrupt Clear Register, UARTICR
    pub uarticr: Uarticr,
    /// 0x0048
    /// DMA Control Register, UARTDMACR
    pub uartdmacr: Uartdmacr,
    _reserved2: [u8; 3988],
    /// 0x0fe0
    /// UARTPeriphID0 Register
    pub uartperiphid0: Uartperiphid0,
    /// 0x0fe4
    /// UARTPeriphID1 Register
    pub uartperiphid1: Uartperiphid1,
    /// 0x0fe8
    /// UARTPeriphID2 Register
    pub uartperiphid2: Uartperiphid2,
    /// 0x0fec
    /// UARTPeriphID3 Register
    pub uartperiphid3: Uartperiphid3,
    /// 0x0ff0
    /// UARTPCellID0 Register
    pub uartpcellid0: Uartpcellid0,
    /// 0x0ff4
    /// UARTPCellID1 Register
    pub uartpcellid1: Uartpcellid1,
    /// 0x0ff8
    /// UARTPCellID2 Register
    pub uartpcellid2: Uartpcellid2,
    /// 0x0ffc
    /// UARTPCellID3 Register
    pub uartpcellid3: Uartpcellid3,
}
