//! SPI0

#![allow(dead_code)]

pub const BASE: u32 = 0x40080000;

/// SSPCR0 Register
///
/// Control register 0, SSPCR0 on page 3-4
///
/// Address offset: 0x0000
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **SCR** `[15:8]` (RW)
///   Serial clock rate. The value SCR is used to generate the transmit and receive bit rate of the PrimeCell SSP. The bit rate is: F SSPCLK CPSDVSR x (1+SCR) where CPSDVSR is an even value from 2-254, programmed through the SSPCPSR register and SCR is a value from 0-255.
/// - **SPH** `[7]` (RW)
///   SSPCLKOUT phase, applicable to Motorola SPI frame format only. See Motorola SPI frame format on page 2-10.
/// - **SPO** `[6]` (RW)
///   SSPCLKOUT polarity, applicable to Motorola SPI frame format only. See Motorola SPI frame format on page 2-10.
/// - **FRF** `[5:4]` (RW)
///   Frame format: 00 Motorola SPI frame format. 01 TI synchronous serial frame format. 10 National Microwire frame format. 11 Reserved, undefined operation.
/// - **DSS** `[3:0]` (RW)
///   Data Size Select: 0000 Reserved, undefined operation. 0001 Reserved, undefined operation. 0010 Reserved, undefined operation. 0011 4-bit data. 0100 5-bit data. 0101 6-bit data. 0110 7-bit data. 0111 8-bit data. 1000 9-bit data. 1001 10-bit data. 1010 11-bit data. 1011 12-bit data. 1100 13-bit data. 1101 14-bit data. 1110 15-bit data. 1111 16-bit data.
#[repr(transparent)]
pub struct Sspcr0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Sspcr0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Sspcr0 {
    // ======== Field Constants ========

    /// **SCR**
    /// Serial clock rate. The value SCR is used to generate the transmit and receive bit rate of the PrimeCell SSP. The bit rate is: F SSPCLK CPSDVSR x (1+SCR) where CPSDVSR is an even value from 2-254, programmed through the SSPCPSR register and SCR is a value from 0-255.
    /// Bits `[15:8]`, mask `0x0000ff00`
    pub const SCR_OFFSET: u32 = 8;
    pub const SCR_MASK: u32 = 0x0000ff00;

    /// **SPH**
    /// SSPCLKOUT phase, applicable to Motorola SPI frame format only. See Motorola SPI frame format on page 2-10.
    /// Bits `[7:7]`, mask `0x00000080`
    pub const SPH_OFFSET: u32 = 7;
    pub const SPH_MASK: u32 = 0x00000080;
    pub const SPH: u32 = 1 << 7;

    /// **SPO**
    /// SSPCLKOUT polarity, applicable to Motorola SPI frame format only. See Motorola SPI frame format on page 2-10.
    /// Bits `[6:6]`, mask `0x00000040`
    pub const SPO_OFFSET: u32 = 6;
    pub const SPO_MASK: u32 = 0x00000040;
    pub const SPO: u32 = 1 << 6;

    /// **FRF**
    /// Frame format: 00 Motorola SPI frame format. 01 TI synchronous serial frame format. 10 National Microwire frame format. 11 Reserved, undefined operation.
    /// Bits `[5:4]`, mask `0x00000030`
    pub const FRF_OFFSET: u32 = 4;
    pub const FRF_MASK: u32 = 0x00000030;

    /// **DSS**
    /// Data Size Select: 0000 Reserved, undefined operation. 0001 Reserved, undefined operation. 0010 Reserved, undefined operation. 0011 4-bit data. 0100 5-bit data. 0101 6-bit data. 0110 7-bit data. 0111 8-bit data. 1000 9-bit data. 1001 10-bit data. 1010 11-bit data. 1011 12-bit data. 1100 13-bit data. 1101 14-bit data. 1110 15-bit data. 1111 16-bit data.
    /// Bits `[3:0]`, mask `0x0000000f`
    pub const DSS_OFFSET: u32 = 0;
    pub const DSS_MASK: u32 = 0x0000000f;

    // ======== Field Accessors ========

    /// **SCR** `[15:8]` (RW)
    /// Serial clock rate. The value SCR is used to generate the transmit and receive bit rate of the PrimeCell SSP. The bit rate is: F SSPCLK CPSDVSR x (1+SCR) where CPSDVSR is an even value from 2-254, programmed through the SSPCPSR register and SCR is a value from 0-255.
    #[inline(always)]
    pub fn scr(&self) -> u32 {
        (self.value.read_bits() & Self::SCR_MASK) >> Self::SCR_OFFSET
    }

    /// Write **SCR** `[15:8]` (RW)
    /// Serial clock rate. The value SCR is used to generate the transmit and receive bit rate of the PrimeCell SSP. The bit rate is: F SSPCLK CPSDVSR x (1+SCR) where CPSDVSR is an even value from 2-254, programmed through the SSPCPSR register and SCR is a value from 0-255.
    #[inline(always)]
    pub fn write_scr(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::SCR_MASK) | ((value << Self::SCR_OFFSET) & Self::SCR_MASK));
    }

    /// **SPH** `[7]` (RW)
    /// SSPCLKOUT phase, applicable to Motorola SPI frame format only. See Motorola SPI frame format on page 2-10.
    #[inline(always)]
    pub fn sph(&self) -> bool {
        (self.value.read_bits() & Self::SPH_MASK) != 0
    }

    /// Set SPH bit atomically (RW)
    #[inline(always)]
    pub fn set_sph(&self) {
        self.value.set_bits(Self::SPH_MASK);
    }

    /// Clear SPH bit atomically (RW)
    #[inline(always)]
    pub fn clr_sph(&self) {
        self.value.clr_bits(Self::SPH_MASK);
    }

    /// **SPO** `[6]` (RW)
    /// SSPCLKOUT polarity, applicable to Motorola SPI frame format only. See Motorola SPI frame format on page 2-10.
    #[inline(always)]
    pub fn spo(&self) -> bool {
        (self.value.read_bits() & Self::SPO_MASK) != 0
    }

    /// Set SPO bit atomically (RW)
    #[inline(always)]
    pub fn set_spo(&self) {
        self.value.set_bits(Self::SPO_MASK);
    }

    /// Clear SPO bit atomically (RW)
    #[inline(always)]
    pub fn clr_spo(&self) {
        self.value.clr_bits(Self::SPO_MASK);
    }

    /// **FRF** `[5:4]` (RW)
    /// Frame format: 00 Motorola SPI frame format. 01 TI synchronous serial frame format. 10 National Microwire frame format. 11 Reserved, undefined operation.
    #[inline(always)]
    pub fn frf(&self) -> u32 {
        (self.value.read_bits() & Self::FRF_MASK) >> Self::FRF_OFFSET
    }

    /// Write **FRF** `[5:4]` (RW)
    /// Frame format: 00 Motorola SPI frame format. 01 TI synchronous serial frame format. 10 National Microwire frame format. 11 Reserved, undefined operation.
    #[inline(always)]
    pub fn write_frf(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::FRF_MASK) | ((value << Self::FRF_OFFSET) & Self::FRF_MASK));
    }

    /// **DSS** `[3:0]` (RW)
    /// Data Size Select: 0000 Reserved, undefined operation. 0001 Reserved, undefined operation. 0010 Reserved, undefined operation. 0011 4-bit data. 0100 5-bit data. 0101 6-bit data. 0110 7-bit data. 0111 8-bit data. 1000 9-bit data. 1001 10-bit data. 1010 11-bit data. 1011 12-bit data. 1100 13-bit data. 1101 14-bit data. 1110 15-bit data. 1111 16-bit data.
    #[inline(always)]
    pub fn dss(&self) -> u32 {
        (self.value.read_bits() & Self::DSS_MASK) >> Self::DSS_OFFSET
    }

    /// Write **DSS** `[3:0]` (RW)
    /// Data Size Select: 0000 Reserved, undefined operation. 0001 Reserved, undefined operation. 0010 Reserved, undefined operation. 0011 4-bit data. 0100 5-bit data. 0101 6-bit data. 0110 7-bit data. 0111 8-bit data. 1000 9-bit data. 1001 10-bit data. 1010 11-bit data. 1011 12-bit data. 1100 13-bit data. 1101 14-bit data. 1110 15-bit data. 1111 16-bit data.
    #[inline(always)]
    pub fn write_dss(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::DSS_MASK) | ((value << Self::DSS_OFFSET) & Self::DSS_MASK));
    }

}

/// SSPCR0 value type for batch read/modify/write operations
///
/// Use `sspcr0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `sspcr0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Sspcr0Val {
    bits: u32,
}

impl Sspcr0Val {
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

    /// Read **SCR** `[15:8]`
    /// Serial clock rate. The value SCR is used to generate the transmit and receive bit rate of the PrimeCell SSP. The bit rate is: F SSPCLK CPSDVSR x (1+SCR) where CPSDVSR is an even value from 2-254, programmed through the SSPCPSR register and SCR is a value from 0-255.
    #[inline(always)]
    pub const fn scr(&self) -> u32 {
        (self.bits & Sspcr0::SCR_MASK) >> Sspcr0::SCR_OFFSET
    }

    /// Read **SPH** `[7:7]`
    /// SSPCLKOUT phase, applicable to Motorola SPI frame format only. See Motorola SPI frame format on page 2-10.
    #[inline(always)]
    pub const fn sph(&self) -> bool {
        (self.bits & Sspcr0::SPH_MASK) != 0
    }

    /// Read **SPO** `[6:6]`
    /// SSPCLKOUT polarity, applicable to Motorola SPI frame format only. See Motorola SPI frame format on page 2-10.
    #[inline(always)]
    pub const fn spo(&self) -> bool {
        (self.bits & Sspcr0::SPO_MASK) != 0
    }

    /// Read **FRF** `[5:4]`
    /// Frame format: 00 Motorola SPI frame format. 01 TI synchronous serial frame format. 10 National Microwire frame format. 11 Reserved, undefined operation.
    #[inline(always)]
    pub const fn frf(&self) -> u32 {
        (self.bits & Sspcr0::FRF_MASK) >> Sspcr0::FRF_OFFSET
    }

    /// Read **DSS** `[3:0]`
    /// Data Size Select: 0000 Reserved, undefined operation. 0001 Reserved, undefined operation. 0010 Reserved, undefined operation. 0011 4-bit data. 0100 5-bit data. 0101 6-bit data. 0110 7-bit data. 0111 8-bit data. 1000 9-bit data. 1001 10-bit data. 1010 11-bit data. 1011 12-bit data. 1100 13-bit data. 1101 14-bit data. 1110 15-bit data. 1111 16-bit data.
    #[inline(always)]
    pub const fn dss(&self) -> u32 {
        (self.bits & Sspcr0::DSS_MASK) >> Sspcr0::DSS_OFFSET
    }

    /// Set **SCR** `[15:8]`
    /// Serial clock rate. The value SCR is used to generate the transmit and receive bit rate of the PrimeCell SSP. The bit rate is: F SSPCLK CPSDVSR x (1+SCR) where CPSDVSR is an even value from 2-254, programmed through the SSPCPSR register and SCR is a value from 0-255.
    #[inline(always)]
    pub const fn with_scr(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Sspcr0::SCR_MASK) | ((value << Sspcr0::SCR_OFFSET) & Sspcr0::SCR_MASK)
        }
    }

    /// Set **SPH** `[7:7]`
    /// SSPCLKOUT phase, applicable to Motorola SPI frame format only. See Motorola SPI frame format on page 2-10.
    #[inline(always)]
    pub const fn with_sph(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sspcr0::SPH_MASK
            } else {
                self.bits & !Sspcr0::SPH_MASK
            }
        }
    }

    /// Set **SPO** `[6:6]`
    /// SSPCLKOUT polarity, applicable to Motorola SPI frame format only. See Motorola SPI frame format on page 2-10.
    #[inline(always)]
    pub const fn with_spo(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sspcr0::SPO_MASK
            } else {
                self.bits & !Sspcr0::SPO_MASK
            }
        }
    }

    /// Set **FRF** `[5:4]`
    /// Frame format: 00 Motorola SPI frame format. 01 TI synchronous serial frame format. 10 National Microwire frame format. 11 Reserved, undefined operation.
    #[inline(always)]
    pub const fn with_frf(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Sspcr0::FRF_MASK) | ((value << Sspcr0::FRF_OFFSET) & Sspcr0::FRF_MASK)
        }
    }

    /// Set **DSS** `[3:0]`
    /// Data Size Select: 0000 Reserved, undefined operation. 0001 Reserved, undefined operation. 0010 Reserved, undefined operation. 0011 4-bit data. 0100 5-bit data. 0101 6-bit data. 0110 7-bit data. 0111 8-bit data. 1000 9-bit data. 1001 10-bit data. 1010 11-bit data. 1011 12-bit data. 1100 13-bit data. 1101 14-bit data. 1110 15-bit data. 1111 16-bit data.
    #[inline(always)]
    pub const fn with_dss(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Sspcr0::DSS_MASK) | ((value << Sspcr0::DSS_OFFSET) & Sspcr0::DSS_MASK)
        }
    }

}

impl Default for Sspcr0Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Sspcr0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Sspcr0Val {
        Sspcr0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Sspcr0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Sspcr0Val) -> Sspcr0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// SSPCR1 Register
///
/// Control register 1, SSPCR1 on page 3-5
///
/// Address offset: 0x0004
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **SOD** `[3]` (RW)
///   Slave-mode output disable. This bit is relevant only in the slave mode, MS=1. In multiple-slave systems, it is possible for an PrimeCell SSP master to broadcast a message to all slaves in the system while ensuring that only one slave drives data onto its serial output line. In such systems the RXD lines from multiple slaves could be tied together. To operate in such systems, the SOD bit can be set if the PrimeCell SSP slave is not supposed to drive the SSPTXD line: 0 SSP can drive the SSPTXD output in slave mode. 1 SSP must not drive the SSPTXD output in slave mode.
/// - **MS** `[2]` (RW)
///   Master or slave mode select. This bit can be modified only when the PrimeCell SSP is disabled, SSE=0: 0 Device configured as master, default. 1 Device configured as slave.
/// - **SSE** `[1]` (RW)
///   Synchronous serial port enable: 0 SSP operation disabled. 1 SSP operation enabled.
/// - **LBM** `[0]` (RW)
///   Loop back mode: 0 Normal serial port operation enabled. 1 Output of transmit serial shifter is connected to input of receive serial shifter internally.
#[repr(transparent)]
pub struct Sspcr1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Sspcr1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Sspcr1 {
    // ======== Field Constants ========

    /// **SOD**
    /// Slave-mode output disable. This bit is relevant only in the slave mode, MS=1. In multiple-slave systems, it is possible for an PrimeCell SSP master to broadcast a message to all slaves in the system while ensuring that only one slave drives data onto its serial output line. In such systems the RXD lines from multiple slaves could be tied together. To operate in such systems, the SOD bit can be set if the PrimeCell SSP slave is not supposed to drive the SSPTXD line: 0 SSP can drive the SSPTXD output in slave mode. 1 SSP must not drive the SSPTXD output in slave mode.
    /// Bits `[3:3]`, mask `0x00000008`
    pub const SOD_OFFSET: u32 = 3;
    pub const SOD_MASK: u32 = 0x00000008;
    pub const SOD: u32 = 1 << 3;

    /// **MS**
    /// Master or slave mode select. This bit can be modified only when the PrimeCell SSP is disabled, SSE=0: 0 Device configured as master, default. 1 Device configured as slave.
    /// Bits `[2:2]`, mask `0x00000004`
    pub const MS_OFFSET: u32 = 2;
    pub const MS_MASK: u32 = 0x00000004;
    pub const MS: u32 = 1 << 2;

    /// **SSE**
    /// Synchronous serial port enable: 0 SSP operation disabled. 1 SSP operation enabled.
    /// Bits `[1:1]`, mask `0x00000002`
    pub const SSE_OFFSET: u32 = 1;
    pub const SSE_MASK: u32 = 0x00000002;
    pub const SSE: u32 = 1 << 1;

    /// **LBM**
    /// Loop back mode: 0 Normal serial port operation enabled. 1 Output of transmit serial shifter is connected to input of receive serial shifter internally.
    /// Bits `[0:0]`, mask `0x00000001`
    pub const LBM_OFFSET: u32 = 0;
    pub const LBM_MASK: u32 = 0x00000001;
    pub const LBM: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **SOD** `[3]` (RW)
    /// Slave-mode output disable. This bit is relevant only in the slave mode, MS=1. In multiple-slave systems, it is possible for an PrimeCell SSP master to broadcast a message to all slaves in the system while ensuring that only one slave drives data onto its serial output line. In such systems the RXD lines from multiple slaves could be tied together. To operate in such systems, the SOD bit can be set if the PrimeCell SSP slave is not supposed to drive the SSPTXD line: 0 SSP can drive the SSPTXD output in slave mode. 1 SSP must not drive the SSPTXD output in slave mode.
    #[inline(always)]
    pub fn sod(&self) -> bool {
        (self.value.read_bits() & Self::SOD_MASK) != 0
    }

    /// Set SOD bit atomically (RW)
    #[inline(always)]
    pub fn set_sod(&self) {
        self.value.set_bits(Self::SOD_MASK);
    }

    /// Clear SOD bit atomically (RW)
    #[inline(always)]
    pub fn clr_sod(&self) {
        self.value.clr_bits(Self::SOD_MASK);
    }

    /// **MS** `[2]` (RW)
    /// Master or slave mode select. This bit can be modified only when the PrimeCell SSP is disabled, SSE=0: 0 Device configured as master, default. 1 Device configured as slave.
    #[inline(always)]
    pub fn ms(&self) -> bool {
        (self.value.read_bits() & Self::MS_MASK) != 0
    }

    /// Set MS bit atomically (RW)
    #[inline(always)]
    pub fn set_ms(&self) {
        self.value.set_bits(Self::MS_MASK);
    }

    /// Clear MS bit atomically (RW)
    #[inline(always)]
    pub fn clr_ms(&self) {
        self.value.clr_bits(Self::MS_MASK);
    }

    /// **SSE** `[1]` (RW)
    /// Synchronous serial port enable: 0 SSP operation disabled. 1 SSP operation enabled.
    #[inline(always)]
    pub fn sse(&self) -> bool {
        (self.value.read_bits() & Self::SSE_MASK) != 0
    }

    /// Set SSE bit atomically (RW)
    #[inline(always)]
    pub fn set_sse(&self) {
        self.value.set_bits(Self::SSE_MASK);
    }

    /// Clear SSE bit atomically (RW)
    #[inline(always)]
    pub fn clr_sse(&self) {
        self.value.clr_bits(Self::SSE_MASK);
    }

    /// **LBM** `[0]` (RW)
    /// Loop back mode: 0 Normal serial port operation enabled. 1 Output of transmit serial shifter is connected to input of receive serial shifter internally.
    #[inline(always)]
    pub fn lbm(&self) -> bool {
        (self.value.read_bits() & Self::LBM_MASK) != 0
    }

    /// Set LBM bit atomically (RW)
    #[inline(always)]
    pub fn set_lbm(&self) {
        self.value.set_bits(Self::LBM_MASK);
    }

    /// Clear LBM bit atomically (RW)
    #[inline(always)]
    pub fn clr_lbm(&self) {
        self.value.clr_bits(Self::LBM_MASK);
    }

}

/// SSPCR1 value type for batch read/modify/write operations
///
/// Use `sspcr1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `sspcr1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Sspcr1Val {
    bits: u32,
}

impl Sspcr1Val {
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

    /// Read **SOD** `[3:3]`
    /// Slave-mode output disable. This bit is relevant only in the slave mode, MS=1. In multiple-slave systems, it is possible for an PrimeCell SSP master to broadcast a message to all slaves in the system while ensuring that only one slave drives data onto its serial output line. In such systems the RXD lines from multiple slaves could be tied together. To operate in such systems, the SOD bit can be set if the PrimeCell SSP slave is not supposed to drive the SSPTXD line: 0 SSP can drive the SSPTXD output in slave mode. 1 SSP must not drive the SSPTXD output in slave mode.
    #[inline(always)]
    pub const fn sod(&self) -> bool {
        (self.bits & Sspcr1::SOD_MASK) != 0
    }

    /// Read **MS** `[2:2]`
    /// Master or slave mode select. This bit can be modified only when the PrimeCell SSP is disabled, SSE=0: 0 Device configured as master, default. 1 Device configured as slave.
    #[inline(always)]
    pub const fn ms(&self) -> bool {
        (self.bits & Sspcr1::MS_MASK) != 0
    }

    /// Read **SSE** `[1:1]`
    /// Synchronous serial port enable: 0 SSP operation disabled. 1 SSP operation enabled.
    #[inline(always)]
    pub const fn sse(&self) -> bool {
        (self.bits & Sspcr1::SSE_MASK) != 0
    }

    /// Read **LBM** `[0:0]`
    /// Loop back mode: 0 Normal serial port operation enabled. 1 Output of transmit serial shifter is connected to input of receive serial shifter internally.
    #[inline(always)]
    pub const fn lbm(&self) -> bool {
        (self.bits & Sspcr1::LBM_MASK) != 0
    }

    /// Set **SOD** `[3:3]`
    /// Slave-mode output disable. This bit is relevant only in the slave mode, MS=1. In multiple-slave systems, it is possible for an PrimeCell SSP master to broadcast a message to all slaves in the system while ensuring that only one slave drives data onto its serial output line. In such systems the RXD lines from multiple slaves could be tied together. To operate in such systems, the SOD bit can be set if the PrimeCell SSP slave is not supposed to drive the SSPTXD line: 0 SSP can drive the SSPTXD output in slave mode. 1 SSP must not drive the SSPTXD output in slave mode.
    #[inline(always)]
    pub const fn with_sod(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sspcr1::SOD_MASK
            } else {
                self.bits & !Sspcr1::SOD_MASK
            }
        }
    }

    /// Set **MS** `[2:2]`
    /// Master or slave mode select. This bit can be modified only when the PrimeCell SSP is disabled, SSE=0: 0 Device configured as master, default. 1 Device configured as slave.
    #[inline(always)]
    pub const fn with_ms(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sspcr1::MS_MASK
            } else {
                self.bits & !Sspcr1::MS_MASK
            }
        }
    }

    /// Set **SSE** `[1:1]`
    /// Synchronous serial port enable: 0 SSP operation disabled. 1 SSP operation enabled.
    #[inline(always)]
    pub const fn with_sse(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sspcr1::SSE_MASK
            } else {
                self.bits & !Sspcr1::SSE_MASK
            }
        }
    }

    /// Set **LBM** `[0:0]`
    /// Loop back mode: 0 Normal serial port operation enabled. 1 Output of transmit serial shifter is connected to input of receive serial shifter internally.
    #[inline(always)]
    pub const fn with_lbm(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sspcr1::LBM_MASK
            } else {
                self.bits & !Sspcr1::LBM_MASK
            }
        }
    }

}

impl Default for Sspcr1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Sspcr1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Sspcr1Val {
        Sspcr1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Sspcr1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Sspcr1Val) -> Sspcr1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// SSPDR Register
///
/// Data register, SSPDR on page 3-6
///
/// Address offset: 0x0008
///
/// ## Fields
///
/// - **DATA** `[15:0]` (RW)
///   Transmit/Receive FIFO: Read Receive FIFO. Write Transmit FIFO. You must right-justify data when the PrimeCell SSP is programmed for a data size that is less than 16 bits. Unused bits at the top are ignored by transmit logic. The receive logic automatically right-justifies.
#[repr(transparent)]
pub struct Sspdr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Sspdr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Sspdr {
    // ======== Field Constants ========

    /// **DATA**
    /// Transmit/Receive FIFO: Read Receive FIFO. Write Transmit FIFO. You must right-justify data when the PrimeCell SSP is programmed for a data size that is less than 16 bits. Unused bits at the top are ignored by transmit logic. The receive logic automatically right-justifies.
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const DATA_OFFSET: u32 = 0;
    pub const DATA_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **DATA** `[15:0]` (RW)
    /// Transmit/Receive FIFO: Read Receive FIFO. Write Transmit FIFO. You must right-justify data when the PrimeCell SSP is programmed for a data size that is less than 16 bits. Unused bits at the top are ignored by transmit logic. The receive logic automatically right-justifies.
    #[inline(always)]
    pub fn data(&self) -> u32 {
        (self.value.read_bits() & Self::DATA_MASK) >> Self::DATA_OFFSET
    }

    /// Write **DATA** `[15:0]` (RW)
    /// Transmit/Receive FIFO: Read Receive FIFO. Write Transmit FIFO. You must right-justify data when the PrimeCell SSP is programmed for a data size that is less than 16 bits. Unused bits at the top are ignored by transmit logic. The receive logic automatically right-justifies.
    #[inline(always)]
    pub fn write_data(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::DATA_MASK) | ((value << Self::DATA_OFFSET) & Self::DATA_MASK));
    }

}

/// SSPDR value type for batch read/modify/write operations
///
/// Use `sspdr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `sspdr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SspdrVal {
    bits: u32,
}

impl SspdrVal {
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

    /// Read **DATA** `[15:0]`
    /// Transmit/Receive FIFO: Read Receive FIFO. Write Transmit FIFO. You must right-justify data when the PrimeCell SSP is programmed for a data size that is less than 16 bits. Unused bits at the top are ignored by transmit logic. The receive logic automatically right-justifies.
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        (self.bits & Sspdr::DATA_MASK) >> Sspdr::DATA_OFFSET
    }

    /// Set **DATA** `[15:0]`
    /// Transmit/Receive FIFO: Read Receive FIFO. Write Transmit FIFO. You must right-justify data when the PrimeCell SSP is programmed for a data size that is less than 16 bits. Unused bits at the top are ignored by transmit logic. The receive logic automatically right-justifies.
    #[inline(always)]
    pub const fn with_data(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Sspdr::DATA_MASK) | ((value << Sspdr::DATA_OFFSET) & Sspdr::DATA_MASK)
        }
    }

}

impl Default for SspdrVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Sspdr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SspdrVal {
        SspdrVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SspdrVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SspdrVal) -> SspdrVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// SSPSR Register
///
/// Status register, SSPSR on page 3-7
///
/// Address offset: 0x000c
/// Reset value: 0x00000003
///
/// ## Fields
///
/// - **BSY** `[4]` (RO)
///   PrimeCell SSP busy flag, RO: 0 SSP is idle. 1 SSP is currently transmitting and/or receiving a frame or the transmit FIFO is not empty.
/// - **RFF** `[3]` (RO)
///   Receive FIFO full, RO: 0 Receive FIFO is not full. 1 Receive FIFO is full.
/// - **RNE** `[2]` (RO)
///   Receive FIFO not empty, RO: 0 Receive FIFO is empty. 1 Receive FIFO is not empty.
/// - **TNF** `[1]` (RO)
///   Transmit FIFO not full, RO: 0 Transmit FIFO is full. 1 Transmit FIFO is not full.
/// - **TFE** `[0]` (RO)
///   Transmit FIFO empty, RO: 0 Transmit FIFO is not empty. 1 Transmit FIFO is empty.
#[repr(transparent)]
pub struct Sspsr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Sspsr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Sspsr {
    // ======== Field Constants ========

    /// **BSY**
    /// PrimeCell SSP busy flag, RO: 0 SSP is idle. 1 SSP is currently transmitting and/or receiving a frame or the transmit FIFO is not empty.
    /// Bits `[4:4]`, mask `0x00000010`
    pub const BSY_OFFSET: u32 = 4;
    pub const BSY_MASK: u32 = 0x00000010;
    pub const BSY: u32 = 1 << 4;

    /// **RFF**
    /// Receive FIFO full, RO: 0 Receive FIFO is not full. 1 Receive FIFO is full.
    /// Bits `[3:3]`, mask `0x00000008`
    pub const RFF_OFFSET: u32 = 3;
    pub const RFF_MASK: u32 = 0x00000008;
    pub const RFF: u32 = 1 << 3;

    /// **RNE**
    /// Receive FIFO not empty, RO: 0 Receive FIFO is empty. 1 Receive FIFO is not empty.
    /// Bits `[2:2]`, mask `0x00000004`
    pub const RNE_OFFSET: u32 = 2;
    pub const RNE_MASK: u32 = 0x00000004;
    pub const RNE: u32 = 1 << 2;

    /// **TNF**
    /// Transmit FIFO not full, RO: 0 Transmit FIFO is full. 1 Transmit FIFO is not full.
    /// Bits `[1:1]`, mask `0x00000002`
    pub const TNF_OFFSET: u32 = 1;
    pub const TNF_MASK: u32 = 0x00000002;
    pub const TNF: u32 = 1 << 1;

    /// **TFE**
    /// Transmit FIFO empty, RO: 0 Transmit FIFO is not empty. 1 Transmit FIFO is empty.
    /// Bits `[0:0]`, mask `0x00000001`
    pub const TFE_OFFSET: u32 = 0;
    pub const TFE_MASK: u32 = 0x00000001;
    pub const TFE: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **BSY** `[4]` (RO)
    /// PrimeCell SSP busy flag, RO: 0 SSP is idle. 1 SSP is currently transmitting and/or receiving a frame or the transmit FIFO is not empty.
    #[inline(always)]
    pub fn bsy(&self) -> bool {
        (self.value.read_bits() & Self::BSY_MASK) != 0
    }

    /// **RFF** `[3]` (RO)
    /// Receive FIFO full, RO: 0 Receive FIFO is not full. 1 Receive FIFO is full.
    #[inline(always)]
    pub fn rff(&self) -> bool {
        (self.value.read_bits() & Self::RFF_MASK) != 0
    }

    /// **RNE** `[2]` (RO)
    /// Receive FIFO not empty, RO: 0 Receive FIFO is empty. 1 Receive FIFO is not empty.
    #[inline(always)]
    pub fn rne(&self) -> bool {
        (self.value.read_bits() & Self::RNE_MASK) != 0
    }

    /// **TNF** `[1]` (RO)
    /// Transmit FIFO not full, RO: 0 Transmit FIFO is full. 1 Transmit FIFO is not full.
    #[inline(always)]
    pub fn tnf(&self) -> bool {
        (self.value.read_bits() & Self::TNF_MASK) != 0
    }

    /// **TFE** `[0]` (RO)
    /// Transmit FIFO empty, RO: 0 Transmit FIFO is not empty. 1 Transmit FIFO is empty.
    #[inline(always)]
    pub fn tfe(&self) -> bool {
        (self.value.read_bits() & Self::TFE_MASK) != 0
    }

}

/// SSPSR value type for batch read/modify/write operations
///
/// Use `sspsr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `sspsr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SspsrVal {
    bits: u32,
}

impl SspsrVal {
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
        Self { bits: 0x00000003 }
    }

    /// Read **BSY** `[4:4]`
    /// PrimeCell SSP busy flag, RO: 0 SSP is idle. 1 SSP is currently transmitting and/or receiving a frame or the transmit FIFO is not empty.
    #[inline(always)]
    pub const fn bsy(&self) -> bool {
        (self.bits & Sspsr::BSY_MASK) != 0
    }

    /// Read **RFF** `[3:3]`
    /// Receive FIFO full, RO: 0 Receive FIFO is not full. 1 Receive FIFO is full.
    #[inline(always)]
    pub const fn rff(&self) -> bool {
        (self.bits & Sspsr::RFF_MASK) != 0
    }

    /// Read **RNE** `[2:2]`
    /// Receive FIFO not empty, RO: 0 Receive FIFO is empty. 1 Receive FIFO is not empty.
    #[inline(always)]
    pub const fn rne(&self) -> bool {
        (self.bits & Sspsr::RNE_MASK) != 0
    }

    /// Read **TNF** `[1:1]`
    /// Transmit FIFO not full, RO: 0 Transmit FIFO is full. 1 Transmit FIFO is not full.
    #[inline(always)]
    pub const fn tnf(&self) -> bool {
        (self.bits & Sspsr::TNF_MASK) != 0
    }

    /// Read **TFE** `[0:0]`
    /// Transmit FIFO empty, RO: 0 Transmit FIFO is not empty. 1 Transmit FIFO is empty.
    #[inline(always)]
    pub const fn tfe(&self) -> bool {
        (self.bits & Sspsr::TFE_MASK) != 0
    }

}

impl Default for SspsrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Sspsr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SspsrVal {
        SspsrVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SspsrVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SspsrVal) -> SspsrVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// SSPCPSR Register
///
/// Clock prescale register, SSPCPSR on page 3-8
///
/// Address offset: 0x0010
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **CPSDVSR** `[7:0]` (RW)
///   Clock prescale divisor. Must be an even number from 2-254, depending on the frequency of SSPCLK. The least significant bit always returns zero on reads.
#[repr(transparent)]
pub struct Sspcpsr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Sspcpsr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Sspcpsr {
    // ======== Field Constants ========

    /// **CPSDVSR**
    /// Clock prescale divisor. Must be an even number from 2-254, depending on the frequency of SSPCLK. The least significant bit always returns zero on reads.
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const CPSDVSR_OFFSET: u32 = 0;
    pub const CPSDVSR_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// **CPSDVSR** `[7:0]` (RW)
    /// Clock prescale divisor. Must be an even number from 2-254, depending on the frequency of SSPCLK. The least significant bit always returns zero on reads.
    #[inline(always)]
    pub fn cpsdvsr(&self) -> u32 {
        (self.value.read_bits() & Self::CPSDVSR_MASK) >> Self::CPSDVSR_OFFSET
    }

    /// Write **CPSDVSR** `[7:0]` (RW)
    /// Clock prescale divisor. Must be an even number from 2-254, depending on the frequency of SSPCLK. The least significant bit always returns zero on reads.
    #[inline(always)]
    pub fn write_cpsdvsr(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::CPSDVSR_MASK) | ((value << Self::CPSDVSR_OFFSET) & Self::CPSDVSR_MASK));
    }

}

/// SSPCPSR value type for batch read/modify/write operations
///
/// Use `sspcpsr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `sspcpsr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SspcpsrVal {
    bits: u32,
}

impl SspcpsrVal {
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

    /// Read **CPSDVSR** `[7:0]`
    /// Clock prescale divisor. Must be an even number from 2-254, depending on the frequency of SSPCLK. The least significant bit always returns zero on reads.
    #[inline(always)]
    pub const fn cpsdvsr(&self) -> u32 {
        (self.bits & Sspcpsr::CPSDVSR_MASK) >> Sspcpsr::CPSDVSR_OFFSET
    }

    /// Set **CPSDVSR** `[7:0]`
    /// Clock prescale divisor. Must be an even number from 2-254, depending on the frequency of SSPCLK. The least significant bit always returns zero on reads.
    #[inline(always)]
    pub const fn with_cpsdvsr(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Sspcpsr::CPSDVSR_MASK) | ((value << Sspcpsr::CPSDVSR_OFFSET) & Sspcpsr::CPSDVSR_MASK)
        }
    }

}

impl Default for SspcpsrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Sspcpsr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SspcpsrVal {
        SspcpsrVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SspcpsrVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SspcpsrVal) -> SspcpsrVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// SSPIMSC Register
///
/// Interrupt mask set or clear register, SSPIMSC on page 3-9
///
/// Address offset: 0x0014
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **TXIM** `[3]` (RW)
///   Transmit FIFO interrupt mask: 0 Transmit FIFO half empty or less condition interrupt is masked. 1 Transmit FIFO half empty or less condition interrupt is not masked.
/// - **RXIM** `[2]` (RW)
///   Receive FIFO interrupt mask: 0 Receive FIFO half full or less condition interrupt is masked. 1 Receive FIFO half full or less condition interrupt is not masked.
/// - **RTIM** `[1]` (RW)
///   Receive timeout interrupt mask: 0 Receive FIFO not empty and no read prior to timeout period interrupt is masked. 1 Receive FIFO not empty and no read prior to timeout period interrupt is not masked.
/// - **RORIM** `[0]` (RW)
///   Receive overrun interrupt mask: 0 Receive FIFO written to while full condition interrupt is masked. 1 Receive FIFO written to while full condition interrupt is not masked.
#[repr(transparent)]
pub struct Sspimsc {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Sspimsc {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Sspimsc {
    // ======== Field Constants ========

    /// **TXIM**
    /// Transmit FIFO interrupt mask: 0 Transmit FIFO half empty or less condition interrupt is masked. 1 Transmit FIFO half empty or less condition interrupt is not masked.
    /// Bits `[3:3]`, mask `0x00000008`
    pub const TXIM_OFFSET: u32 = 3;
    pub const TXIM_MASK: u32 = 0x00000008;
    pub const TXIM: u32 = 1 << 3;

    /// **RXIM**
    /// Receive FIFO interrupt mask: 0 Receive FIFO half full or less condition interrupt is masked. 1 Receive FIFO half full or less condition interrupt is not masked.
    /// Bits `[2:2]`, mask `0x00000004`
    pub const RXIM_OFFSET: u32 = 2;
    pub const RXIM_MASK: u32 = 0x00000004;
    pub const RXIM: u32 = 1 << 2;

    /// **RTIM**
    /// Receive timeout interrupt mask: 0 Receive FIFO not empty and no read prior to timeout period interrupt is masked. 1 Receive FIFO not empty and no read prior to timeout period interrupt is not masked.
    /// Bits `[1:1]`, mask `0x00000002`
    pub const RTIM_OFFSET: u32 = 1;
    pub const RTIM_MASK: u32 = 0x00000002;
    pub const RTIM: u32 = 1 << 1;

    /// **RORIM**
    /// Receive overrun interrupt mask: 0 Receive FIFO written to while full condition interrupt is masked. 1 Receive FIFO written to while full condition interrupt is not masked.
    /// Bits `[0:0]`, mask `0x00000001`
    pub const RORIM_OFFSET: u32 = 0;
    pub const RORIM_MASK: u32 = 0x00000001;
    pub const RORIM: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **TXIM** `[3]` (RW)
    /// Transmit FIFO interrupt mask: 0 Transmit FIFO half empty or less condition interrupt is masked. 1 Transmit FIFO half empty or less condition interrupt is not masked.
    #[inline(always)]
    pub fn txim(&self) -> bool {
        (self.value.read_bits() & Self::TXIM_MASK) != 0
    }

    /// Set TXIM bit atomically (RW)
    #[inline(always)]
    pub fn set_txim(&self) {
        self.value.set_bits(Self::TXIM_MASK);
    }

    /// Clear TXIM bit atomically (RW)
    #[inline(always)]
    pub fn clr_txim(&self) {
        self.value.clr_bits(Self::TXIM_MASK);
    }

    /// **RXIM** `[2]` (RW)
    /// Receive FIFO interrupt mask: 0 Receive FIFO half full or less condition interrupt is masked. 1 Receive FIFO half full or less condition interrupt is not masked.
    #[inline(always)]
    pub fn rxim(&self) -> bool {
        (self.value.read_bits() & Self::RXIM_MASK) != 0
    }

    /// Set RXIM bit atomically (RW)
    #[inline(always)]
    pub fn set_rxim(&self) {
        self.value.set_bits(Self::RXIM_MASK);
    }

    /// Clear RXIM bit atomically (RW)
    #[inline(always)]
    pub fn clr_rxim(&self) {
        self.value.clr_bits(Self::RXIM_MASK);
    }

    /// **RTIM** `[1]` (RW)
    /// Receive timeout interrupt mask: 0 Receive FIFO not empty and no read prior to timeout period interrupt is masked. 1 Receive FIFO not empty and no read prior to timeout period interrupt is not masked.
    #[inline(always)]
    pub fn rtim(&self) -> bool {
        (self.value.read_bits() & Self::RTIM_MASK) != 0
    }

    /// Set RTIM bit atomically (RW)
    #[inline(always)]
    pub fn set_rtim(&self) {
        self.value.set_bits(Self::RTIM_MASK);
    }

    /// Clear RTIM bit atomically (RW)
    #[inline(always)]
    pub fn clr_rtim(&self) {
        self.value.clr_bits(Self::RTIM_MASK);
    }

    /// **RORIM** `[0]` (RW)
    /// Receive overrun interrupt mask: 0 Receive FIFO written to while full condition interrupt is masked. 1 Receive FIFO written to while full condition interrupt is not masked.
    #[inline(always)]
    pub fn rorim(&self) -> bool {
        (self.value.read_bits() & Self::RORIM_MASK) != 0
    }

    /// Set RORIM bit atomically (RW)
    #[inline(always)]
    pub fn set_rorim(&self) {
        self.value.set_bits(Self::RORIM_MASK);
    }

    /// Clear RORIM bit atomically (RW)
    #[inline(always)]
    pub fn clr_rorim(&self) {
        self.value.clr_bits(Self::RORIM_MASK);
    }

}

/// SSPIMSC value type for batch read/modify/write operations
///
/// Use `sspimsc.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `sspimsc.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SspimscVal {
    bits: u32,
}

impl SspimscVal {
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

    /// Read **TXIM** `[3:3]`
    /// Transmit FIFO interrupt mask: 0 Transmit FIFO half empty or less condition interrupt is masked. 1 Transmit FIFO half empty or less condition interrupt is not masked.
    #[inline(always)]
    pub const fn txim(&self) -> bool {
        (self.bits & Sspimsc::TXIM_MASK) != 0
    }

    /// Read **RXIM** `[2:2]`
    /// Receive FIFO interrupt mask: 0 Receive FIFO half full or less condition interrupt is masked. 1 Receive FIFO half full or less condition interrupt is not masked.
    #[inline(always)]
    pub const fn rxim(&self) -> bool {
        (self.bits & Sspimsc::RXIM_MASK) != 0
    }

    /// Read **RTIM** `[1:1]`
    /// Receive timeout interrupt mask: 0 Receive FIFO not empty and no read prior to timeout period interrupt is masked. 1 Receive FIFO not empty and no read prior to timeout period interrupt is not masked.
    #[inline(always)]
    pub const fn rtim(&self) -> bool {
        (self.bits & Sspimsc::RTIM_MASK) != 0
    }

    /// Read **RORIM** `[0:0]`
    /// Receive overrun interrupt mask: 0 Receive FIFO written to while full condition interrupt is masked. 1 Receive FIFO written to while full condition interrupt is not masked.
    #[inline(always)]
    pub const fn rorim(&self) -> bool {
        (self.bits & Sspimsc::RORIM_MASK) != 0
    }

    /// Set **TXIM** `[3:3]`
    /// Transmit FIFO interrupt mask: 0 Transmit FIFO half empty or less condition interrupt is masked. 1 Transmit FIFO half empty or less condition interrupt is not masked.
    #[inline(always)]
    pub const fn with_txim(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sspimsc::TXIM_MASK
            } else {
                self.bits & !Sspimsc::TXIM_MASK
            }
        }
    }

    /// Set **RXIM** `[2:2]`
    /// Receive FIFO interrupt mask: 0 Receive FIFO half full or less condition interrupt is masked. 1 Receive FIFO half full or less condition interrupt is not masked.
    #[inline(always)]
    pub const fn with_rxim(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sspimsc::RXIM_MASK
            } else {
                self.bits & !Sspimsc::RXIM_MASK
            }
        }
    }

    /// Set **RTIM** `[1:1]`
    /// Receive timeout interrupt mask: 0 Receive FIFO not empty and no read prior to timeout period interrupt is masked. 1 Receive FIFO not empty and no read prior to timeout period interrupt is not masked.
    #[inline(always)]
    pub const fn with_rtim(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sspimsc::RTIM_MASK
            } else {
                self.bits & !Sspimsc::RTIM_MASK
            }
        }
    }

    /// Set **RORIM** `[0:0]`
    /// Receive overrun interrupt mask: 0 Receive FIFO written to while full condition interrupt is masked. 1 Receive FIFO written to while full condition interrupt is not masked.
    #[inline(always)]
    pub const fn with_rorim(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sspimsc::RORIM_MASK
            } else {
                self.bits & !Sspimsc::RORIM_MASK
            }
        }
    }

}

impl Default for SspimscVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Sspimsc {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SspimscVal {
        SspimscVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SspimscVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SspimscVal) -> SspimscVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// SSPRIS Register
///
/// Raw interrupt status register, SSPRIS on page 3-10
///
/// Address offset: 0x0018
/// Reset value: 0x00000008
///
/// ## Fields
///
/// - **TXRIS** `[3]` (RO)
///   Gives the raw interrupt state, prior to masking, of the SSPTXINTR interrupt
/// - **RXRIS** `[2]` (RO)
///   Gives the raw interrupt state, prior to masking, of the SSPRXINTR interrupt
/// - **RTRIS** `[1]` (RO)
///   Gives the raw interrupt state, prior to masking, of the SSPRTINTR interrupt
/// - **RORRIS** `[0]` (RO)
///   Gives the raw interrupt state, prior to masking, of the SSPRORINTR interrupt
#[repr(transparent)]
pub struct Sspris {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Sspris {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Sspris {
    // ======== Field Constants ========

    /// **TXRIS**
    /// Gives the raw interrupt state, prior to masking, of the SSPTXINTR interrupt
    /// Bits `[3:3]`, mask `0x00000008`
    pub const TXRIS_OFFSET: u32 = 3;
    pub const TXRIS_MASK: u32 = 0x00000008;
    pub const TXRIS: u32 = 1 << 3;

    /// **RXRIS**
    /// Gives the raw interrupt state, prior to masking, of the SSPRXINTR interrupt
    /// Bits `[2:2]`, mask `0x00000004`
    pub const RXRIS_OFFSET: u32 = 2;
    pub const RXRIS_MASK: u32 = 0x00000004;
    pub const RXRIS: u32 = 1 << 2;

    /// **RTRIS**
    /// Gives the raw interrupt state, prior to masking, of the SSPRTINTR interrupt
    /// Bits `[1:1]`, mask `0x00000002`
    pub const RTRIS_OFFSET: u32 = 1;
    pub const RTRIS_MASK: u32 = 0x00000002;
    pub const RTRIS: u32 = 1 << 1;

    /// **RORRIS**
    /// Gives the raw interrupt state, prior to masking, of the SSPRORINTR interrupt
    /// Bits `[0:0]`, mask `0x00000001`
    pub const RORRIS_OFFSET: u32 = 0;
    pub const RORRIS_MASK: u32 = 0x00000001;
    pub const RORRIS: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **TXRIS** `[3]` (RO)
    /// Gives the raw interrupt state, prior to masking, of the SSPTXINTR interrupt
    #[inline(always)]
    pub fn txris(&self) -> bool {
        (self.value.read_bits() & Self::TXRIS_MASK) != 0
    }

    /// **RXRIS** `[2]` (RO)
    /// Gives the raw interrupt state, prior to masking, of the SSPRXINTR interrupt
    #[inline(always)]
    pub fn rxris(&self) -> bool {
        (self.value.read_bits() & Self::RXRIS_MASK) != 0
    }

    /// **RTRIS** `[1]` (RO)
    /// Gives the raw interrupt state, prior to masking, of the SSPRTINTR interrupt
    #[inline(always)]
    pub fn rtris(&self) -> bool {
        (self.value.read_bits() & Self::RTRIS_MASK) != 0
    }

    /// **RORRIS** `[0]` (RO)
    /// Gives the raw interrupt state, prior to masking, of the SSPRORINTR interrupt
    #[inline(always)]
    pub fn rorris(&self) -> bool {
        (self.value.read_bits() & Self::RORRIS_MASK) != 0
    }

}

/// SSPRIS value type for batch read/modify/write operations
///
/// Use `sspris.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `sspris.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SsprisVal {
    bits: u32,
}

impl SsprisVal {
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
        Self { bits: 0x00000008 }
    }

    /// Read **TXRIS** `[3:3]`
    /// Gives the raw interrupt state, prior to masking, of the SSPTXINTR interrupt
    #[inline(always)]
    pub const fn txris(&self) -> bool {
        (self.bits & Sspris::TXRIS_MASK) != 0
    }

    /// Read **RXRIS** `[2:2]`
    /// Gives the raw interrupt state, prior to masking, of the SSPRXINTR interrupt
    #[inline(always)]
    pub const fn rxris(&self) -> bool {
        (self.bits & Sspris::RXRIS_MASK) != 0
    }

    /// Read **RTRIS** `[1:1]`
    /// Gives the raw interrupt state, prior to masking, of the SSPRTINTR interrupt
    #[inline(always)]
    pub const fn rtris(&self) -> bool {
        (self.bits & Sspris::RTRIS_MASK) != 0
    }

    /// Read **RORRIS** `[0:0]`
    /// Gives the raw interrupt state, prior to masking, of the SSPRORINTR interrupt
    #[inline(always)]
    pub const fn rorris(&self) -> bool {
        (self.bits & Sspris::RORRIS_MASK) != 0
    }

}

impl Default for SsprisVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Sspris {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SsprisVal {
        SsprisVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SsprisVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SsprisVal) -> SsprisVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// SSPMIS Register
///
/// Masked interrupt status register, SSPMIS on page 3-11
///
/// Address offset: 0x001c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **TXMIS** `[3]` (RO)
///   Gives the transmit FIFO masked interrupt state, after masking, of the SSPTXINTR interrupt
/// - **RXMIS** `[2]` (RO)
///   Gives the receive FIFO masked interrupt state, after masking, of the SSPRXINTR interrupt
/// - **RTMIS** `[1]` (RO)
///   Gives the receive timeout masked interrupt state, after masking, of the SSPRTINTR interrupt
/// - **RORMIS** `[0]` (RO)
///   Gives the receive over run masked interrupt status, after masking, of the SSPRORINTR interrupt
#[repr(transparent)]
pub struct Sspmis {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Sspmis {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Sspmis {
    // ======== Field Constants ========

    /// **TXMIS**
    /// Gives the transmit FIFO masked interrupt state, after masking, of the SSPTXINTR interrupt
    /// Bits `[3:3]`, mask `0x00000008`
    pub const TXMIS_OFFSET: u32 = 3;
    pub const TXMIS_MASK: u32 = 0x00000008;
    pub const TXMIS: u32 = 1 << 3;

    /// **RXMIS**
    /// Gives the receive FIFO masked interrupt state, after masking, of the SSPRXINTR interrupt
    /// Bits `[2:2]`, mask `0x00000004`
    pub const RXMIS_OFFSET: u32 = 2;
    pub const RXMIS_MASK: u32 = 0x00000004;
    pub const RXMIS: u32 = 1 << 2;

    /// **RTMIS**
    /// Gives the receive timeout masked interrupt state, after masking, of the SSPRTINTR interrupt
    /// Bits `[1:1]`, mask `0x00000002`
    pub const RTMIS_OFFSET: u32 = 1;
    pub const RTMIS_MASK: u32 = 0x00000002;
    pub const RTMIS: u32 = 1 << 1;

    /// **RORMIS**
    /// Gives the receive over run masked interrupt status, after masking, of the SSPRORINTR interrupt
    /// Bits `[0:0]`, mask `0x00000001`
    pub const RORMIS_OFFSET: u32 = 0;
    pub const RORMIS_MASK: u32 = 0x00000001;
    pub const RORMIS: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **TXMIS** `[3]` (RO)
    /// Gives the transmit FIFO masked interrupt state, after masking, of the SSPTXINTR interrupt
    #[inline(always)]
    pub fn txmis(&self) -> bool {
        (self.value.read_bits() & Self::TXMIS_MASK) != 0
    }

    /// **RXMIS** `[2]` (RO)
    /// Gives the receive FIFO masked interrupt state, after masking, of the SSPRXINTR interrupt
    #[inline(always)]
    pub fn rxmis(&self) -> bool {
        (self.value.read_bits() & Self::RXMIS_MASK) != 0
    }

    /// **RTMIS** `[1]` (RO)
    /// Gives the receive timeout masked interrupt state, after masking, of the SSPRTINTR interrupt
    #[inline(always)]
    pub fn rtmis(&self) -> bool {
        (self.value.read_bits() & Self::RTMIS_MASK) != 0
    }

    /// **RORMIS** `[0]` (RO)
    /// Gives the receive over run masked interrupt status, after masking, of the SSPRORINTR interrupt
    #[inline(always)]
    pub fn rormis(&self) -> bool {
        (self.value.read_bits() & Self::RORMIS_MASK) != 0
    }

}

/// SSPMIS value type for batch read/modify/write operations
///
/// Use `sspmis.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `sspmis.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SspmisVal {
    bits: u32,
}

impl SspmisVal {
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

    /// Read **TXMIS** `[3:3]`
    /// Gives the transmit FIFO masked interrupt state, after masking, of the SSPTXINTR interrupt
    #[inline(always)]
    pub const fn txmis(&self) -> bool {
        (self.bits & Sspmis::TXMIS_MASK) != 0
    }

    /// Read **RXMIS** `[2:2]`
    /// Gives the receive FIFO masked interrupt state, after masking, of the SSPRXINTR interrupt
    #[inline(always)]
    pub const fn rxmis(&self) -> bool {
        (self.bits & Sspmis::RXMIS_MASK) != 0
    }

    /// Read **RTMIS** `[1:1]`
    /// Gives the receive timeout masked interrupt state, after masking, of the SSPRTINTR interrupt
    #[inline(always)]
    pub const fn rtmis(&self) -> bool {
        (self.bits & Sspmis::RTMIS_MASK) != 0
    }

    /// Read **RORMIS** `[0:0]`
    /// Gives the receive over run masked interrupt status, after masking, of the SSPRORINTR interrupt
    #[inline(always)]
    pub const fn rormis(&self) -> bool {
        (self.bits & Sspmis::RORMIS_MASK) != 0
    }

}

impl Default for SspmisVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Sspmis {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SspmisVal {
        SspmisVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SspmisVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SspmisVal) -> SspmisVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// SSPICR Register
///
/// Interrupt clear register, SSPICR on page 3-11
///
/// Address offset: 0x0020
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **RTIC** `[1]` (RW)
///   Clears the SSPRTINTR interrupt
/// - **RORIC** `[0]` (RW)
///   Clears the SSPRORINTR interrupt
#[repr(transparent)]
pub struct Sspicr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Sspicr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Sspicr {
    // ======== Field Constants ========

    /// **RTIC**
    /// Clears the SSPRTINTR interrupt
    /// Bits `[1:1]`, mask `0x00000002`
    pub const RTIC_OFFSET: u32 = 1;
    pub const RTIC_MASK: u32 = 0x00000002;
    pub const RTIC: u32 = 1 << 1;

    /// **RORIC**
    /// Clears the SSPRORINTR interrupt
    /// Bits `[0:0]`, mask `0x00000001`
    pub const RORIC_OFFSET: u32 = 0;
    pub const RORIC_MASK: u32 = 0x00000001;
    pub const RORIC: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **RTIC** `[1]` (RW)
    /// Clears the SSPRTINTR interrupt
    #[inline(always)]
    pub fn rtic(&self) -> bool {
        (self.value.read_bits() & Self::RTIC_MASK) != 0
    }

    /// Set RTIC bit atomically (RW)
    #[inline(always)]
    pub fn set_rtic(&self) {
        self.value.set_bits(Self::RTIC_MASK);
    }

    /// Clear RTIC bit atomically (RW)
    #[inline(always)]
    pub fn clr_rtic(&self) {
        self.value.clr_bits(Self::RTIC_MASK);
    }

    /// **RORIC** `[0]` (RW)
    /// Clears the SSPRORINTR interrupt
    #[inline(always)]
    pub fn roric(&self) -> bool {
        (self.value.read_bits() & Self::RORIC_MASK) != 0
    }

    /// Set RORIC bit atomically (RW)
    #[inline(always)]
    pub fn set_roric(&self) {
        self.value.set_bits(Self::RORIC_MASK);
    }

    /// Clear RORIC bit atomically (RW)
    #[inline(always)]
    pub fn clr_roric(&self) {
        self.value.clr_bits(Self::RORIC_MASK);
    }

}

/// SSPICR value type for batch read/modify/write operations
///
/// Use `sspicr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `sspicr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SspicrVal {
    bits: u32,
}

impl SspicrVal {
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

    /// Read **RTIC** `[1:1]`
    /// Clears the SSPRTINTR interrupt
    #[inline(always)]
    pub const fn rtic(&self) -> bool {
        (self.bits & Sspicr::RTIC_MASK) != 0
    }

    /// Read **RORIC** `[0:0]`
    /// Clears the SSPRORINTR interrupt
    #[inline(always)]
    pub const fn roric(&self) -> bool {
        (self.bits & Sspicr::RORIC_MASK) != 0
    }

    /// Set **RTIC** `[1:1]`
    /// Clears the SSPRTINTR interrupt
    #[inline(always)]
    pub const fn with_rtic(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sspicr::RTIC_MASK
            } else {
                self.bits & !Sspicr::RTIC_MASK
            }
        }
    }

    /// Set **RORIC** `[0:0]`
    /// Clears the SSPRORINTR interrupt
    #[inline(always)]
    pub const fn with_roric(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sspicr::RORIC_MASK
            } else {
                self.bits & !Sspicr::RORIC_MASK
            }
        }
    }

}

impl Default for SspicrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Sspicr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SspicrVal {
        SspicrVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SspicrVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SspicrVal) -> SspicrVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// SSPDMACR Register
///
/// DMA control register, SSPDMACR on page 3-12
///
/// Address offset: 0x0024
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **TXDMAE** `[1]` (RW)
///   Transmit DMA Enable. If this bit is set to 1, DMA for the transmit FIFO is enabled.
/// - **RXDMAE** `[0]` (RW)
///   Receive DMA Enable. If this bit is set to 1, DMA for the receive FIFO is enabled.
#[repr(transparent)]
pub struct Sspdmacr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Sspdmacr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Sspdmacr {
    // ======== Field Constants ========

    /// **TXDMAE**
    /// Transmit DMA Enable. If this bit is set to 1, DMA for the transmit FIFO is enabled.
    /// Bits `[1:1]`, mask `0x00000002`
    pub const TXDMAE_OFFSET: u32 = 1;
    pub const TXDMAE_MASK: u32 = 0x00000002;
    pub const TXDMAE: u32 = 1 << 1;

    /// **RXDMAE**
    /// Receive DMA Enable. If this bit is set to 1, DMA for the receive FIFO is enabled.
    /// Bits `[0:0]`, mask `0x00000001`
    pub const RXDMAE_OFFSET: u32 = 0;
    pub const RXDMAE_MASK: u32 = 0x00000001;
    pub const RXDMAE: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **TXDMAE** `[1]` (RW)
    /// Transmit DMA Enable. If this bit is set to 1, DMA for the transmit FIFO is enabled.
    #[inline(always)]
    pub fn txdmae(&self) -> bool {
        (self.value.read_bits() & Self::TXDMAE_MASK) != 0
    }

    /// Set TXDMAE bit atomically (RW)
    #[inline(always)]
    pub fn set_txdmae(&self) {
        self.value.set_bits(Self::TXDMAE_MASK);
    }

    /// Clear TXDMAE bit atomically (RW)
    #[inline(always)]
    pub fn clr_txdmae(&self) {
        self.value.clr_bits(Self::TXDMAE_MASK);
    }

    /// **RXDMAE** `[0]` (RW)
    /// Receive DMA Enable. If this bit is set to 1, DMA for the receive FIFO is enabled.
    #[inline(always)]
    pub fn rxdmae(&self) -> bool {
        (self.value.read_bits() & Self::RXDMAE_MASK) != 0
    }

    /// Set RXDMAE bit atomically (RW)
    #[inline(always)]
    pub fn set_rxdmae(&self) {
        self.value.set_bits(Self::RXDMAE_MASK);
    }

    /// Clear RXDMAE bit atomically (RW)
    #[inline(always)]
    pub fn clr_rxdmae(&self) {
        self.value.clr_bits(Self::RXDMAE_MASK);
    }

}

/// SSPDMACR value type for batch read/modify/write operations
///
/// Use `sspdmacr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `sspdmacr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SspdmacrVal {
    bits: u32,
}

impl SspdmacrVal {
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

    /// Read **TXDMAE** `[1:1]`
    /// Transmit DMA Enable. If this bit is set to 1, DMA for the transmit FIFO is enabled.
    #[inline(always)]
    pub const fn txdmae(&self) -> bool {
        (self.bits & Sspdmacr::TXDMAE_MASK) != 0
    }

    /// Read **RXDMAE** `[0:0]`
    /// Receive DMA Enable. If this bit is set to 1, DMA for the receive FIFO is enabled.
    #[inline(always)]
    pub const fn rxdmae(&self) -> bool {
        (self.bits & Sspdmacr::RXDMAE_MASK) != 0
    }

    /// Set **TXDMAE** `[1:1]`
    /// Transmit DMA Enable. If this bit is set to 1, DMA for the transmit FIFO is enabled.
    #[inline(always)]
    pub const fn with_txdmae(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sspdmacr::TXDMAE_MASK
            } else {
                self.bits & !Sspdmacr::TXDMAE_MASK
            }
        }
    }

    /// Set **RXDMAE** `[0:0]`
    /// Receive DMA Enable. If this bit is set to 1, DMA for the receive FIFO is enabled.
    #[inline(always)]
    pub const fn with_rxdmae(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sspdmacr::RXDMAE_MASK
            } else {
                self.bits & !Sspdmacr::RXDMAE_MASK
            }
        }
    }

}

impl Default for SspdmacrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Sspdmacr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SspdmacrVal {
        SspdmacrVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SspdmacrVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SspdmacrVal) -> SspdmacrVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// SSPPERIPHID0 Register
///
/// Peripheral identification registers, SSPPeriphID0-3 on page 3-13
///
/// Address offset: 0x0fe0
/// Reset value: 0x00000022
///
/// ## Fields
///
/// - **PARTNUMBER0** `[7:0]` (RO)
///   These bits read back as 0x22
#[repr(transparent)]
pub struct Sspperiphid0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Sspperiphid0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Sspperiphid0 {
    // ======== Field Constants ========

    /// **PARTNUMBER0**
    /// These bits read back as 0x22
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const PARTNUMBER0_OFFSET: u32 = 0;
    pub const PARTNUMBER0_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// **PARTNUMBER0** `[7:0]` (RO)
    /// These bits read back as 0x22
    #[inline(always)]
    pub fn partnumber0(&self) -> u32 {
        (self.value.read_bits() & Self::PARTNUMBER0_MASK) >> Self::PARTNUMBER0_OFFSET
    }

}

/// SSPPERIPHID0 value type for batch read/modify/write operations
///
/// Use `sspperiphid0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `sspperiphid0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Sspperiphid0Val {
    bits: u32,
}

impl Sspperiphid0Val {
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
        Self { bits: 0x00000022 }
    }

    /// Read **PARTNUMBER0** `[7:0]`
    /// These bits read back as 0x22
    #[inline(always)]
    pub const fn partnumber0(&self) -> u32 {
        (self.bits & Sspperiphid0::PARTNUMBER0_MASK) >> Sspperiphid0::PARTNUMBER0_OFFSET
    }

}

impl Default for Sspperiphid0Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Sspperiphid0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Sspperiphid0Val {
        Sspperiphid0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Sspperiphid0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Sspperiphid0Val) -> Sspperiphid0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// SSPPERIPHID1 Register
///
/// Peripheral identification registers, SSPPeriphID0-3 on page 3-13
///
/// Address offset: 0x0fe4
/// Reset value: 0x00000010
///
/// ## Fields
///
/// - **DESIGNER0** `[7:4]` (RO)
///   These bits read back as 0x1
/// - **PARTNUMBER1** `[3:0]` (RO)
///   These bits read back as 0x0
#[repr(transparent)]
pub struct Sspperiphid1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Sspperiphid1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Sspperiphid1 {
    // ======== Field Constants ========

    /// **DESIGNER0**
    /// These bits read back as 0x1
    /// Bits `[7:4]`, mask `0x000000f0`
    pub const DESIGNER0_OFFSET: u32 = 4;
    pub const DESIGNER0_MASK: u32 = 0x000000f0;

    /// **PARTNUMBER1**
    /// These bits read back as 0x0
    /// Bits `[3:0]`, mask `0x0000000f`
    pub const PARTNUMBER1_OFFSET: u32 = 0;
    pub const PARTNUMBER1_MASK: u32 = 0x0000000f;

    // ======== Field Accessors ========

    /// **DESIGNER0** `[7:4]` (RO)
    /// These bits read back as 0x1
    #[inline(always)]
    pub fn designer0(&self) -> u32 {
        (self.value.read_bits() & Self::DESIGNER0_MASK) >> Self::DESIGNER0_OFFSET
    }

    /// **PARTNUMBER1** `[3:0]` (RO)
    /// These bits read back as 0x0
    #[inline(always)]
    pub fn partnumber1(&self) -> u32 {
        (self.value.read_bits() & Self::PARTNUMBER1_MASK) >> Self::PARTNUMBER1_OFFSET
    }

}

/// SSPPERIPHID1 value type for batch read/modify/write operations
///
/// Use `sspperiphid1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `sspperiphid1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Sspperiphid1Val {
    bits: u32,
}

impl Sspperiphid1Val {
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
    /// These bits read back as 0x1
    #[inline(always)]
    pub const fn designer0(&self) -> u32 {
        (self.bits & Sspperiphid1::DESIGNER0_MASK) >> Sspperiphid1::DESIGNER0_OFFSET
    }

    /// Read **PARTNUMBER1** `[3:0]`
    /// These bits read back as 0x0
    #[inline(always)]
    pub const fn partnumber1(&self) -> u32 {
        (self.bits & Sspperiphid1::PARTNUMBER1_MASK) >> Sspperiphid1::PARTNUMBER1_OFFSET
    }

}

impl Default for Sspperiphid1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Sspperiphid1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Sspperiphid1Val {
        Sspperiphid1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Sspperiphid1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Sspperiphid1Val) -> Sspperiphid1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// SSPPERIPHID2 Register
///
/// Peripheral identification registers, SSPPeriphID0-3 on page 3-13
///
/// Address offset: 0x0fe8
/// Reset value: 0x00000034
///
/// ## Fields
///
/// - **REVISION** `[7:4]` (RO)
///   These bits return the peripheral revision
/// - **DESIGNER1** `[3:0]` (RO)
///   These bits read back as 0x4
#[repr(transparent)]
pub struct Sspperiphid2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Sspperiphid2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Sspperiphid2 {
    // ======== Field Constants ========

    /// **REVISION**
    /// These bits return the peripheral revision
    /// Bits `[7:4]`, mask `0x000000f0`
    pub const REVISION_OFFSET: u32 = 4;
    pub const REVISION_MASK: u32 = 0x000000f0;

    /// **DESIGNER1**
    /// These bits read back as 0x4
    /// Bits `[3:0]`, mask `0x0000000f`
    pub const DESIGNER1_OFFSET: u32 = 0;
    pub const DESIGNER1_MASK: u32 = 0x0000000f;

    // ======== Field Accessors ========

    /// **REVISION** `[7:4]` (RO)
    /// These bits return the peripheral revision
    #[inline(always)]
    pub fn revision(&self) -> u32 {
        (self.value.read_bits() & Self::REVISION_MASK) >> Self::REVISION_OFFSET
    }

    /// **DESIGNER1** `[3:0]` (RO)
    /// These bits read back as 0x4
    #[inline(always)]
    pub fn designer1(&self) -> u32 {
        (self.value.read_bits() & Self::DESIGNER1_MASK) >> Self::DESIGNER1_OFFSET
    }

}

/// SSPPERIPHID2 value type for batch read/modify/write operations
///
/// Use `sspperiphid2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `sspperiphid2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Sspperiphid2Val {
    bits: u32,
}

impl Sspperiphid2Val {
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
    /// These bits return the peripheral revision
    #[inline(always)]
    pub const fn revision(&self) -> u32 {
        (self.bits & Sspperiphid2::REVISION_MASK) >> Sspperiphid2::REVISION_OFFSET
    }

    /// Read **DESIGNER1** `[3:0]`
    /// These bits read back as 0x4
    #[inline(always)]
    pub const fn designer1(&self) -> u32 {
        (self.bits & Sspperiphid2::DESIGNER1_MASK) >> Sspperiphid2::DESIGNER1_OFFSET
    }

}

impl Default for Sspperiphid2Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Sspperiphid2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Sspperiphid2Val {
        Sspperiphid2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Sspperiphid2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Sspperiphid2Val) -> Sspperiphid2Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// SSPPERIPHID3 Register
///
/// Peripheral identification registers, SSPPeriphID0-3 on page 3-13
///
/// Address offset: 0x0fec
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **CONFIGURATION** `[7:0]` (RO)
///   These bits read back as 0x00
#[repr(transparent)]
pub struct Sspperiphid3 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Sspperiphid3 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Sspperiphid3 {
    // ======== Field Constants ========

    /// **CONFIGURATION**
    /// These bits read back as 0x00
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const CONFIGURATION_OFFSET: u32 = 0;
    pub const CONFIGURATION_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// **CONFIGURATION** `[7:0]` (RO)
    /// These bits read back as 0x00
    #[inline(always)]
    pub fn configuration(&self) -> u32 {
        (self.value.read_bits() & Self::CONFIGURATION_MASK) >> Self::CONFIGURATION_OFFSET
    }

}

/// SSPPERIPHID3 value type for batch read/modify/write operations
///
/// Use `sspperiphid3.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `sspperiphid3.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Sspperiphid3Val {
    bits: u32,
}

impl Sspperiphid3Val {
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
    /// These bits read back as 0x00
    #[inline(always)]
    pub const fn configuration(&self) -> u32 {
        (self.bits & Sspperiphid3::CONFIGURATION_MASK) >> Sspperiphid3::CONFIGURATION_OFFSET
    }

}

impl Default for Sspperiphid3Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Sspperiphid3 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Sspperiphid3Val {
        Sspperiphid3Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Sspperiphid3Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Sspperiphid3Val) -> Sspperiphid3Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// SSPPCELLID0 Register
///
/// PrimeCell identification registers, SSPPCellID0-3 on page 3-16
///
/// Address offset: 0x0ff0
/// Reset value: 0x0000000d
///
/// ## Fields
///
/// - **SSPPCELLID0** `[7:0]` (RO)
///   These bits read back as 0x0D
#[repr(transparent)]
pub struct Ssppcellid0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ssppcellid0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ssppcellid0 {
    // ======== Field Constants ========

    /// **SSPPCELLID0**
    /// These bits read back as 0x0D
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const SSPPCELLID0_OFFSET: u32 = 0;
    pub const SSPPCELLID0_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// **SSPPCELLID0** `[7:0]` (RO)
    /// These bits read back as 0x0D
    #[inline(always)]
    pub fn ssppcellid0(&self) -> u32 {
        (self.value.read_bits() & Self::SSPPCELLID0_MASK) >> Self::SSPPCELLID0_OFFSET
    }

}

/// SSPPCELLID0 value type for batch read/modify/write operations
///
/// Use `ssppcellid0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ssppcellid0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Ssppcellid0Val {
    bits: u32,
}

impl Ssppcellid0Val {
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

    /// Read **SSPPCELLID0** `[7:0]`
    /// These bits read back as 0x0D
    #[inline(always)]
    pub const fn ssppcellid0(&self) -> u32 {
        (self.bits & Ssppcellid0::SSPPCELLID0_MASK) >> Ssppcellid0::SSPPCELLID0_OFFSET
    }

}

impl Default for Ssppcellid0Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Ssppcellid0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Ssppcellid0Val {
        Ssppcellid0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Ssppcellid0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Ssppcellid0Val) -> Ssppcellid0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// SSPPCELLID1 Register
///
/// PrimeCell identification registers, SSPPCellID0-3 on page 3-16
///
/// Address offset: 0x0ff4
/// Reset value: 0x000000f0
///
/// ## Fields
///
/// - **SSPPCELLID1** `[7:0]` (RO)
///   These bits read back as 0xF0
#[repr(transparent)]
pub struct Ssppcellid1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ssppcellid1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ssppcellid1 {
    // ======== Field Constants ========

    /// **SSPPCELLID1**
    /// These bits read back as 0xF0
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const SSPPCELLID1_OFFSET: u32 = 0;
    pub const SSPPCELLID1_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// **SSPPCELLID1** `[7:0]` (RO)
    /// These bits read back as 0xF0
    #[inline(always)]
    pub fn ssppcellid1(&self) -> u32 {
        (self.value.read_bits() & Self::SSPPCELLID1_MASK) >> Self::SSPPCELLID1_OFFSET
    }

}

/// SSPPCELLID1 value type for batch read/modify/write operations
///
/// Use `ssppcellid1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ssppcellid1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Ssppcellid1Val {
    bits: u32,
}

impl Ssppcellid1Val {
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

    /// Read **SSPPCELLID1** `[7:0]`
    /// These bits read back as 0xF0
    #[inline(always)]
    pub const fn ssppcellid1(&self) -> u32 {
        (self.bits & Ssppcellid1::SSPPCELLID1_MASK) >> Ssppcellid1::SSPPCELLID1_OFFSET
    }

}

impl Default for Ssppcellid1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Ssppcellid1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Ssppcellid1Val {
        Ssppcellid1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Ssppcellid1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Ssppcellid1Val) -> Ssppcellid1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// SSPPCELLID2 Register
///
/// PrimeCell identification registers, SSPPCellID0-3 on page 3-16
///
/// Address offset: 0x0ff8
/// Reset value: 0x00000005
///
/// ## Fields
///
/// - **SSPPCELLID2** `[7:0]` (RO)
///   These bits read back as 0x05
#[repr(transparent)]
pub struct Ssppcellid2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ssppcellid2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ssppcellid2 {
    // ======== Field Constants ========

    /// **SSPPCELLID2**
    /// These bits read back as 0x05
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const SSPPCELLID2_OFFSET: u32 = 0;
    pub const SSPPCELLID2_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// **SSPPCELLID2** `[7:0]` (RO)
    /// These bits read back as 0x05
    #[inline(always)]
    pub fn ssppcellid2(&self) -> u32 {
        (self.value.read_bits() & Self::SSPPCELLID2_MASK) >> Self::SSPPCELLID2_OFFSET
    }

}

/// SSPPCELLID2 value type for batch read/modify/write operations
///
/// Use `ssppcellid2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ssppcellid2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Ssppcellid2Val {
    bits: u32,
}

impl Ssppcellid2Val {
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

    /// Read **SSPPCELLID2** `[7:0]`
    /// These bits read back as 0x05
    #[inline(always)]
    pub const fn ssppcellid2(&self) -> u32 {
        (self.bits & Ssppcellid2::SSPPCELLID2_MASK) >> Ssppcellid2::SSPPCELLID2_OFFSET
    }

}

impl Default for Ssppcellid2Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Ssppcellid2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Ssppcellid2Val {
        Ssppcellid2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Ssppcellid2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Ssppcellid2Val) -> Ssppcellid2Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// SSPPCELLID3 Register
///
/// PrimeCell identification registers, SSPPCellID0-3 on page 3-16
///
/// Address offset: 0x0ffc
/// Reset value: 0x000000b1
///
/// ## Fields
///
/// - **SSPPCELLID3** `[7:0]` (RO)
///   These bits read back as 0xB1
#[repr(transparent)]
pub struct Ssppcellid3 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ssppcellid3 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ssppcellid3 {
    // ======== Field Constants ========

    /// **SSPPCELLID3**
    /// These bits read back as 0xB1
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const SSPPCELLID3_OFFSET: u32 = 0;
    pub const SSPPCELLID3_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// **SSPPCELLID3** `[7:0]` (RO)
    /// These bits read back as 0xB1
    #[inline(always)]
    pub fn ssppcellid3(&self) -> u32 {
        (self.value.read_bits() & Self::SSPPCELLID3_MASK) >> Self::SSPPCELLID3_OFFSET
    }

}

/// SSPPCELLID3 value type for batch read/modify/write operations
///
/// Use `ssppcellid3.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ssppcellid3.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Ssppcellid3Val {
    bits: u32,
}

impl Ssppcellid3Val {
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

    /// Read **SSPPCELLID3** `[7:0]`
    /// These bits read back as 0xB1
    #[inline(always)]
    pub const fn ssppcellid3(&self) -> u32 {
        (self.bits & Ssppcellid3::SSPPCELLID3_MASK) >> Ssppcellid3::SSPPCELLID3_OFFSET
    }

}

impl Default for Ssppcellid3Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Ssppcellid3 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Ssppcellid3Val {
        Ssppcellid3Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Ssppcellid3Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Ssppcellid3Val) -> Ssppcellid3Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// Register block
#[repr(C)]
pub struct RegisterBlock {
    /// 0x0000
    /// Control register 0, SSPCR0 on page 3-4
    pub sspcr0: Sspcr0,
    /// 0x0004
    /// Control register 1, SSPCR1 on page 3-5
    pub sspcr1: Sspcr1,
    /// 0x0008
    /// Data register, SSPDR on page 3-6
    pub sspdr: Sspdr,
    /// 0x000c
    /// Status register, SSPSR on page 3-7
    pub sspsr: Sspsr,
    /// 0x0010
    /// Clock prescale register, SSPCPSR on page 3-8
    pub sspcpsr: Sspcpsr,
    /// 0x0014
    /// Interrupt mask set or clear register, SSPIMSC on page 3-9
    pub sspimsc: Sspimsc,
    /// 0x0018
    /// Raw interrupt status register, SSPRIS on page 3-10
    pub sspris: Sspris,
    /// 0x001c
    /// Masked interrupt status register, SSPMIS on page 3-11
    pub sspmis: Sspmis,
    /// 0x0020
    /// Interrupt clear register, SSPICR on page 3-11
    pub sspicr: Sspicr,
    /// 0x0024
    /// DMA control register, SSPDMACR on page 3-12
    pub sspdmacr: Sspdmacr,
    _reserved0: [u8; 4024],
    /// 0x0fe0
    /// Peripheral identification registers, SSPPeriphID0-3 on page 3-13
    pub sspperiphid0: Sspperiphid0,
    /// 0x0fe4
    /// Peripheral identification registers, SSPPeriphID0-3 on page 3-13
    pub sspperiphid1: Sspperiphid1,
    /// 0x0fe8
    /// Peripheral identification registers, SSPPeriphID0-3 on page 3-13
    pub sspperiphid2: Sspperiphid2,
    /// 0x0fec
    /// Peripheral identification registers, SSPPeriphID0-3 on page 3-13
    pub sspperiphid3: Sspperiphid3,
    /// 0x0ff0
    /// PrimeCell identification registers, SSPPCellID0-3 on page 3-16
    pub ssppcellid0: Ssppcellid0,
    /// 0x0ff4
    /// PrimeCell identification registers, SSPPCellID0-3 on page 3-16
    pub ssppcellid1: Ssppcellid1,
    /// 0x0ff8
    /// PrimeCell identification registers, SSPPCellID0-3 on page 3-16
    pub ssppcellid2: Ssppcellid2,
    /// 0x0ffc
    /// PrimeCell identification registers, SSPPCellID0-3 on page 3-16
    pub ssppcellid3: Ssppcellid3,
}

