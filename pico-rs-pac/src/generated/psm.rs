//! PSM
//!
//! This peripheral supports atomic SET/CLR/XOR register aliases.

#![allow(dead_code)]

pub const BASE: u32 = 0x40018000;

/// FRCE_ON Register
///
/// Force block out of reset (i.e. power it on)
///
/// Address offset: 0x0000
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **PROC1** `[24]` (RW)
///
/// - **PROC0** `[23]` (RW)
///
/// - **ACCESSCTRL** `[22]` (RW)
///
/// - **SIO** `[21]` (RW)
///
/// - **XIP** `[20]` (RW)
///
/// - **SRAM9** `[19]` (RW)
///
/// - **SRAM8** `[18]` (RW)
///
/// - **SRAM7** `[17]` (RW)
///
/// - **SRAM6** `[16]` (RW)
///
/// - **SRAM5** `[15]` (RW)
///
/// - **SRAM4** `[14]` (RW)
///
/// - **SRAM3** `[13]` (RW)
///
/// - **SRAM2** `[12]` (RW)
///
/// - **SRAM1** `[11]` (RW)
///
/// - **SRAM0** `[10]` (RW)
///
/// - **BOOTRAM** `[9]` (RW)
///
/// - **ROM** `[8]` (RW)
///
/// - **BUSFABRIC** `[7]` (RW)
///
/// - **PSM_READY** `[6]` (RW)
///
/// - **CLOCKS** `[5]` (RW)
///
/// - **RESETS** `[4]` (RW)
///
/// - **XOSC** `[3]` (RW)
///
/// - **ROSC** `[2]` (RW)
///
/// - **OTP** `[1]` (RW)
///
/// - **PROC_COLD** `[0]` (RW)
#[repr(transparent)]
pub struct FrceOn {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for FrceOn {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl FrceOn {
    // ======== Field Constants ========

    /// **PROC1**
    /// Bits `[24:24]`, mask `0x01000000`
    pub const PROC1_OFFSET: u32 = 24;
    pub const PROC1_MASK: u32 = 0x01000000;
    pub const PROC1: u32 = 1 << 24;

    /// **PROC0**
    /// Bits `[23:23]`, mask `0x00800000`
    pub const PROC0_OFFSET: u32 = 23;
    pub const PROC0_MASK: u32 = 0x00800000;
    pub const PROC0: u32 = 1 << 23;

    /// **ACCESSCTRL**
    /// Bits `[22:22]`, mask `0x00400000`
    pub const ACCESSCTRL_OFFSET: u32 = 22;
    pub const ACCESSCTRL_MASK: u32 = 0x00400000;
    pub const ACCESSCTRL: u32 = 1 << 22;

    /// **SIO**
    /// Bits `[21:21]`, mask `0x00200000`
    pub const SIO_OFFSET: u32 = 21;
    pub const SIO_MASK: u32 = 0x00200000;
    pub const SIO: u32 = 1 << 21;

    /// **XIP**
    /// Bits `[20:20]`, mask `0x00100000`
    pub const XIP_OFFSET: u32 = 20;
    pub const XIP_MASK: u32 = 0x00100000;
    pub const XIP: u32 = 1 << 20;

    /// **SRAM9**
    /// Bits `[19:19]`, mask `0x00080000`
    pub const SRAM9_OFFSET: u32 = 19;
    pub const SRAM9_MASK: u32 = 0x00080000;
    pub const SRAM9: u32 = 1 << 19;

    /// **SRAM8**
    /// Bits `[18:18]`, mask `0x00040000`
    pub const SRAM8_OFFSET: u32 = 18;
    pub const SRAM8_MASK: u32 = 0x00040000;
    pub const SRAM8: u32 = 1 << 18;

    /// **SRAM7**
    /// Bits `[17:17]`, mask `0x00020000`
    pub const SRAM7_OFFSET: u32 = 17;
    pub const SRAM7_MASK: u32 = 0x00020000;
    pub const SRAM7: u32 = 1 << 17;

    /// **SRAM6**
    /// Bits `[16:16]`, mask `0x00010000`
    pub const SRAM6_OFFSET: u32 = 16;
    pub const SRAM6_MASK: u32 = 0x00010000;
    pub const SRAM6: u32 = 1 << 16;

    /// **SRAM5**
    /// Bits `[15:15]`, mask `0x00008000`
    pub const SRAM5_OFFSET: u32 = 15;
    pub const SRAM5_MASK: u32 = 0x00008000;
    pub const SRAM5: u32 = 1 << 15;

    /// **SRAM4**
    /// Bits `[14:14]`, mask `0x00004000`
    pub const SRAM4_OFFSET: u32 = 14;
    pub const SRAM4_MASK: u32 = 0x00004000;
    pub const SRAM4: u32 = 1 << 14;

    /// **SRAM3**
    /// Bits `[13:13]`, mask `0x00002000`
    pub const SRAM3_OFFSET: u32 = 13;
    pub const SRAM3_MASK: u32 = 0x00002000;
    pub const SRAM3: u32 = 1 << 13;

    /// **SRAM2**
    /// Bits `[12:12]`, mask `0x00001000`
    pub const SRAM2_OFFSET: u32 = 12;
    pub const SRAM2_MASK: u32 = 0x00001000;
    pub const SRAM2: u32 = 1 << 12;

    /// **SRAM1**
    /// Bits `[11:11]`, mask `0x00000800`
    pub const SRAM1_OFFSET: u32 = 11;
    pub const SRAM1_MASK: u32 = 0x00000800;
    pub const SRAM1: u32 = 1 << 11;

    /// **SRAM0**
    /// Bits `[10:10]`, mask `0x00000400`
    pub const SRAM0_OFFSET: u32 = 10;
    pub const SRAM0_MASK: u32 = 0x00000400;
    pub const SRAM0: u32 = 1 << 10;

    /// **BOOTRAM**
    /// Bits `[9:9]`, mask `0x00000200`
    pub const BOOTRAM_OFFSET: u32 = 9;
    pub const BOOTRAM_MASK: u32 = 0x00000200;
    pub const BOOTRAM: u32 = 1 << 9;

    /// **ROM**
    /// Bits `[8:8]`, mask `0x00000100`
    pub const ROM_OFFSET: u32 = 8;
    pub const ROM_MASK: u32 = 0x00000100;
    pub const ROM: u32 = 1 << 8;

    /// **BUSFABRIC**
    /// Bits `[7:7]`, mask `0x00000080`
    pub const BUSFABRIC_OFFSET: u32 = 7;
    pub const BUSFABRIC_MASK: u32 = 0x00000080;
    pub const BUSFABRIC: u32 = 1 << 7;

    /// **PSM_READY**
    /// Bits `[6:6]`, mask `0x00000040`
    pub const PSM_READY_OFFSET: u32 = 6;
    pub const PSM_READY_MASK: u32 = 0x00000040;
    pub const PSM_READY: u32 = 1 << 6;

    /// **CLOCKS**
    /// Bits `[5:5]`, mask `0x00000020`
    pub const CLOCKS_OFFSET: u32 = 5;
    pub const CLOCKS_MASK: u32 = 0x00000020;
    pub const CLOCKS: u32 = 1 << 5;

    /// **RESETS**
    /// Bits `[4:4]`, mask `0x00000010`
    pub const RESETS_OFFSET: u32 = 4;
    pub const RESETS_MASK: u32 = 0x00000010;
    pub const RESETS: u32 = 1 << 4;

    /// **XOSC**
    /// Bits `[3:3]`, mask `0x00000008`
    pub const XOSC_OFFSET: u32 = 3;
    pub const XOSC_MASK: u32 = 0x00000008;
    pub const XOSC: u32 = 1 << 3;

    /// **ROSC**
    /// Bits `[2:2]`, mask `0x00000004`
    pub const ROSC_OFFSET: u32 = 2;
    pub const ROSC_MASK: u32 = 0x00000004;
    pub const ROSC: u32 = 1 << 2;

    /// **OTP**
    /// Bits `[1:1]`, mask `0x00000002`
    pub const OTP_OFFSET: u32 = 1;
    pub const OTP_MASK: u32 = 0x00000002;
    pub const OTP: u32 = 1 << 1;

    /// **PROC_COLD**
    /// Bits `[0:0]`, mask `0x00000001`
    pub const PROC_COLD_OFFSET: u32 = 0;
    pub const PROC_COLD_MASK: u32 = 0x00000001;
    pub const PROC_COLD: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **PROC1** `[24]` (RW)
    #[inline(always)]
    pub fn proc1(&self) -> bool {
        (self.value.read_bits() & Self::PROC1_MASK) != 0
    }

    /// Set PROC1 bit atomically (RW)
    #[inline(always)]
    pub fn set_proc1(&self) {
        self.set_bits(Self::PROC1_MASK);
    }

    /// Clear PROC1 bit atomically (RW)
    #[inline(always)]
    pub fn clr_proc1(&self) {
        self.clr_bits(Self::PROC1_MASK);
    }

    /// **PROC0** `[23]` (RW)
    #[inline(always)]
    pub fn proc0(&self) -> bool {
        (self.value.read_bits() & Self::PROC0_MASK) != 0
    }

    /// Set PROC0 bit atomically (RW)
    #[inline(always)]
    pub fn set_proc0(&self) {
        self.set_bits(Self::PROC0_MASK);
    }

    /// Clear PROC0 bit atomically (RW)
    #[inline(always)]
    pub fn clr_proc0(&self) {
        self.clr_bits(Self::PROC0_MASK);
    }

    /// **ACCESSCTRL** `[22]` (RW)
    #[inline(always)]
    pub fn accessctrl(&self) -> bool {
        (self.value.read_bits() & Self::ACCESSCTRL_MASK) != 0
    }

    /// Set ACCESSCTRL bit atomically (RW)
    #[inline(always)]
    pub fn set_accessctrl(&self) {
        self.set_bits(Self::ACCESSCTRL_MASK);
    }

    /// Clear ACCESSCTRL bit atomically (RW)
    #[inline(always)]
    pub fn clr_accessctrl(&self) {
        self.clr_bits(Self::ACCESSCTRL_MASK);
    }

    /// **SIO** `[21]` (RW)
    #[inline(always)]
    pub fn sio(&self) -> bool {
        (self.value.read_bits() & Self::SIO_MASK) != 0
    }

    /// Set SIO bit atomically (RW)
    #[inline(always)]
    pub fn set_sio(&self) {
        self.set_bits(Self::SIO_MASK);
    }

    /// Clear SIO bit atomically (RW)
    #[inline(always)]
    pub fn clr_sio(&self) {
        self.clr_bits(Self::SIO_MASK);
    }

    /// **XIP** `[20]` (RW)
    #[inline(always)]
    pub fn xip(&self) -> bool {
        (self.value.read_bits() & Self::XIP_MASK) != 0
    }

    /// Set XIP bit atomically (RW)
    #[inline(always)]
    pub fn set_xip(&self) {
        self.set_bits(Self::XIP_MASK);
    }

    /// Clear XIP bit atomically (RW)
    #[inline(always)]
    pub fn clr_xip(&self) {
        self.clr_bits(Self::XIP_MASK);
    }

    /// **SRAM9** `[19]` (RW)
    #[inline(always)]
    pub fn sram9(&self) -> bool {
        (self.value.read_bits() & Self::SRAM9_MASK) != 0
    }

    /// Set SRAM9 bit atomically (RW)
    #[inline(always)]
    pub fn set_sram9(&self) {
        self.set_bits(Self::SRAM9_MASK);
    }

    /// Clear SRAM9 bit atomically (RW)
    #[inline(always)]
    pub fn clr_sram9(&self) {
        self.clr_bits(Self::SRAM9_MASK);
    }

    /// **SRAM8** `[18]` (RW)
    #[inline(always)]
    pub fn sram8(&self) -> bool {
        (self.value.read_bits() & Self::SRAM8_MASK) != 0
    }

    /// Set SRAM8 bit atomically (RW)
    #[inline(always)]
    pub fn set_sram8(&self) {
        self.set_bits(Self::SRAM8_MASK);
    }

    /// Clear SRAM8 bit atomically (RW)
    #[inline(always)]
    pub fn clr_sram8(&self) {
        self.clr_bits(Self::SRAM8_MASK);
    }

    /// **SRAM7** `[17]` (RW)
    #[inline(always)]
    pub fn sram7(&self) -> bool {
        (self.value.read_bits() & Self::SRAM7_MASK) != 0
    }

    /// Set SRAM7 bit atomically (RW)
    #[inline(always)]
    pub fn set_sram7(&self) {
        self.set_bits(Self::SRAM7_MASK);
    }

    /// Clear SRAM7 bit atomically (RW)
    #[inline(always)]
    pub fn clr_sram7(&self) {
        self.clr_bits(Self::SRAM7_MASK);
    }

    /// **SRAM6** `[16]` (RW)
    #[inline(always)]
    pub fn sram6(&self) -> bool {
        (self.value.read_bits() & Self::SRAM6_MASK) != 0
    }

    /// Set SRAM6 bit atomically (RW)
    #[inline(always)]
    pub fn set_sram6(&self) {
        self.set_bits(Self::SRAM6_MASK);
    }

    /// Clear SRAM6 bit atomically (RW)
    #[inline(always)]
    pub fn clr_sram6(&self) {
        self.clr_bits(Self::SRAM6_MASK);
    }

    /// **SRAM5** `[15]` (RW)
    #[inline(always)]
    pub fn sram5(&self) -> bool {
        (self.value.read_bits() & Self::SRAM5_MASK) != 0
    }

    /// Set SRAM5 bit atomically (RW)
    #[inline(always)]
    pub fn set_sram5(&self) {
        self.set_bits(Self::SRAM5_MASK);
    }

    /// Clear SRAM5 bit atomically (RW)
    #[inline(always)]
    pub fn clr_sram5(&self) {
        self.clr_bits(Self::SRAM5_MASK);
    }

    /// **SRAM4** `[14]` (RW)
    #[inline(always)]
    pub fn sram4(&self) -> bool {
        (self.value.read_bits() & Self::SRAM4_MASK) != 0
    }

    /// Set SRAM4 bit atomically (RW)
    #[inline(always)]
    pub fn set_sram4(&self) {
        self.set_bits(Self::SRAM4_MASK);
    }

    /// Clear SRAM4 bit atomically (RW)
    #[inline(always)]
    pub fn clr_sram4(&self) {
        self.clr_bits(Self::SRAM4_MASK);
    }

    /// **SRAM3** `[13]` (RW)
    #[inline(always)]
    pub fn sram3(&self) -> bool {
        (self.value.read_bits() & Self::SRAM3_MASK) != 0
    }

    /// Set SRAM3 bit atomically (RW)
    #[inline(always)]
    pub fn set_sram3(&self) {
        self.set_bits(Self::SRAM3_MASK);
    }

    /// Clear SRAM3 bit atomically (RW)
    #[inline(always)]
    pub fn clr_sram3(&self) {
        self.clr_bits(Self::SRAM3_MASK);
    }

    /// **SRAM2** `[12]` (RW)
    #[inline(always)]
    pub fn sram2(&self) -> bool {
        (self.value.read_bits() & Self::SRAM2_MASK) != 0
    }

    /// Set SRAM2 bit atomically (RW)
    #[inline(always)]
    pub fn set_sram2(&self) {
        self.set_bits(Self::SRAM2_MASK);
    }

    /// Clear SRAM2 bit atomically (RW)
    #[inline(always)]
    pub fn clr_sram2(&self) {
        self.clr_bits(Self::SRAM2_MASK);
    }

    /// **SRAM1** `[11]` (RW)
    #[inline(always)]
    pub fn sram1(&self) -> bool {
        (self.value.read_bits() & Self::SRAM1_MASK) != 0
    }

    /// Set SRAM1 bit atomically (RW)
    #[inline(always)]
    pub fn set_sram1(&self) {
        self.set_bits(Self::SRAM1_MASK);
    }

    /// Clear SRAM1 bit atomically (RW)
    #[inline(always)]
    pub fn clr_sram1(&self) {
        self.clr_bits(Self::SRAM1_MASK);
    }

    /// **SRAM0** `[10]` (RW)
    #[inline(always)]
    pub fn sram0(&self) -> bool {
        (self.value.read_bits() & Self::SRAM0_MASK) != 0
    }

    /// Set SRAM0 bit atomically (RW)
    #[inline(always)]
    pub fn set_sram0(&self) {
        self.set_bits(Self::SRAM0_MASK);
    }

    /// Clear SRAM0 bit atomically (RW)
    #[inline(always)]
    pub fn clr_sram0(&self) {
        self.clr_bits(Self::SRAM0_MASK);
    }

    /// **BOOTRAM** `[9]` (RW)
    #[inline(always)]
    pub fn bootram(&self) -> bool {
        (self.value.read_bits() & Self::BOOTRAM_MASK) != 0
    }

    /// Set BOOTRAM bit atomically (RW)
    #[inline(always)]
    pub fn set_bootram(&self) {
        self.set_bits(Self::BOOTRAM_MASK);
    }

    /// Clear BOOTRAM bit atomically (RW)
    #[inline(always)]
    pub fn clr_bootram(&self) {
        self.clr_bits(Self::BOOTRAM_MASK);
    }

    /// **ROM** `[8]` (RW)
    #[inline(always)]
    pub fn rom(&self) -> bool {
        (self.value.read_bits() & Self::ROM_MASK) != 0
    }

    /// Set ROM bit atomically (RW)
    #[inline(always)]
    pub fn set_rom(&self) {
        self.set_bits(Self::ROM_MASK);
    }

    /// Clear ROM bit atomically (RW)
    #[inline(always)]
    pub fn clr_rom(&self) {
        self.clr_bits(Self::ROM_MASK);
    }

    /// **BUSFABRIC** `[7]` (RW)
    #[inline(always)]
    pub fn busfabric(&self) -> bool {
        (self.value.read_bits() & Self::BUSFABRIC_MASK) != 0
    }

    /// Set BUSFABRIC bit atomically (RW)
    #[inline(always)]
    pub fn set_busfabric(&self) {
        self.set_bits(Self::BUSFABRIC_MASK);
    }

    /// Clear BUSFABRIC bit atomically (RW)
    #[inline(always)]
    pub fn clr_busfabric(&self) {
        self.clr_bits(Self::BUSFABRIC_MASK);
    }

    /// **PSM_READY** `[6]` (RW)
    #[inline(always)]
    pub fn psm_ready(&self) -> bool {
        (self.value.read_bits() & Self::PSM_READY_MASK) != 0
    }

    /// Set PSM_READY bit atomically (RW)
    #[inline(always)]
    pub fn set_psm_ready(&self) {
        self.set_bits(Self::PSM_READY_MASK);
    }

    /// Clear PSM_READY bit atomically (RW)
    #[inline(always)]
    pub fn clr_psm_ready(&self) {
        self.clr_bits(Self::PSM_READY_MASK);
    }

    /// **CLOCKS** `[5]` (RW)
    #[inline(always)]
    pub fn clocks(&self) -> bool {
        (self.value.read_bits() & Self::CLOCKS_MASK) != 0
    }

    /// Set CLOCKS bit atomically (RW)
    #[inline(always)]
    pub fn set_clocks(&self) {
        self.set_bits(Self::CLOCKS_MASK);
    }

    /// Clear CLOCKS bit atomically (RW)
    #[inline(always)]
    pub fn clr_clocks(&self) {
        self.clr_bits(Self::CLOCKS_MASK);
    }

    /// **RESETS** `[4]` (RW)
    #[inline(always)]
    pub fn resets(&self) -> bool {
        (self.value.read_bits() & Self::RESETS_MASK) != 0
    }

    /// Set RESETS bit atomically (RW)
    #[inline(always)]
    pub fn set_resets(&self) {
        self.set_bits(Self::RESETS_MASK);
    }

    /// Clear RESETS bit atomically (RW)
    #[inline(always)]
    pub fn clr_resets(&self) {
        self.clr_bits(Self::RESETS_MASK);
    }

    /// **XOSC** `[3]` (RW)
    #[inline(always)]
    pub fn xosc(&self) -> bool {
        (self.value.read_bits() & Self::XOSC_MASK) != 0
    }

    /// Set XOSC bit atomically (RW)
    #[inline(always)]
    pub fn set_xosc(&self) {
        self.set_bits(Self::XOSC_MASK);
    }

    /// Clear XOSC bit atomically (RW)
    #[inline(always)]
    pub fn clr_xosc(&self) {
        self.clr_bits(Self::XOSC_MASK);
    }

    /// **ROSC** `[2]` (RW)
    #[inline(always)]
    pub fn rosc(&self) -> bool {
        (self.value.read_bits() & Self::ROSC_MASK) != 0
    }

    /// Set ROSC bit atomically (RW)
    #[inline(always)]
    pub fn set_rosc(&self) {
        self.set_bits(Self::ROSC_MASK);
    }

    /// Clear ROSC bit atomically (RW)
    #[inline(always)]
    pub fn clr_rosc(&self) {
        self.clr_bits(Self::ROSC_MASK);
    }

    /// **OTP** `[1]` (RW)
    #[inline(always)]
    pub fn otp(&self) -> bool {
        (self.value.read_bits() & Self::OTP_MASK) != 0
    }

    /// Set OTP bit atomically (RW)
    #[inline(always)]
    pub fn set_otp(&self) {
        self.set_bits(Self::OTP_MASK);
    }

    /// Clear OTP bit atomically (RW)
    #[inline(always)]
    pub fn clr_otp(&self) {
        self.clr_bits(Self::OTP_MASK);
    }

    /// **PROC_COLD** `[0]` (RW)
    #[inline(always)]
    pub fn proc_cold(&self) -> bool {
        (self.value.read_bits() & Self::PROC_COLD_MASK) != 0
    }

    /// Set PROC_COLD bit atomically (RW)
    #[inline(always)]
    pub fn set_proc_cold(&self) {
        self.set_bits(Self::PROC_COLD_MASK);
    }

    /// Clear PROC_COLD bit atomically (RW)
    #[inline(always)]
    pub fn clr_proc_cold(&self) {
        self.clr_bits(Self::PROC_COLD_MASK);
    }

}

/// FRCE_ON value type for batch read/modify/write operations
///
/// Use `frceon.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `frceon.write_val(val)`.
#[derive(Clone, Copy)]
pub struct FrceOnVal {
    bits: u32,
}

impl FrceOnVal {
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

    /// Read **PROC1** `[24:24]`
    #[inline(always)]
    pub const fn proc1(&self) -> bool {
        (self.bits & FrceOn::PROC1_MASK) != 0
    }

    /// Read **PROC0** `[23:23]`
    #[inline(always)]
    pub const fn proc0(&self) -> bool {
        (self.bits & FrceOn::PROC0_MASK) != 0
    }

    /// Read **ACCESSCTRL** `[22:22]`
    #[inline(always)]
    pub const fn accessctrl(&self) -> bool {
        (self.bits & FrceOn::ACCESSCTRL_MASK) != 0
    }

    /// Read **SIO** `[21:21]`
    #[inline(always)]
    pub const fn sio(&self) -> bool {
        (self.bits & FrceOn::SIO_MASK) != 0
    }

    /// Read **XIP** `[20:20]`
    #[inline(always)]
    pub const fn xip(&self) -> bool {
        (self.bits & FrceOn::XIP_MASK) != 0
    }

    /// Read **SRAM9** `[19:19]`
    #[inline(always)]
    pub const fn sram9(&self) -> bool {
        (self.bits & FrceOn::SRAM9_MASK) != 0
    }

    /// Read **SRAM8** `[18:18]`
    #[inline(always)]
    pub const fn sram8(&self) -> bool {
        (self.bits & FrceOn::SRAM8_MASK) != 0
    }

    /// Read **SRAM7** `[17:17]`
    #[inline(always)]
    pub const fn sram7(&self) -> bool {
        (self.bits & FrceOn::SRAM7_MASK) != 0
    }

    /// Read **SRAM6** `[16:16]`
    #[inline(always)]
    pub const fn sram6(&self) -> bool {
        (self.bits & FrceOn::SRAM6_MASK) != 0
    }

    /// Read **SRAM5** `[15:15]`
    #[inline(always)]
    pub const fn sram5(&self) -> bool {
        (self.bits & FrceOn::SRAM5_MASK) != 0
    }

    /// Read **SRAM4** `[14:14]`
    #[inline(always)]
    pub const fn sram4(&self) -> bool {
        (self.bits & FrceOn::SRAM4_MASK) != 0
    }

    /// Read **SRAM3** `[13:13]`
    #[inline(always)]
    pub const fn sram3(&self) -> bool {
        (self.bits & FrceOn::SRAM3_MASK) != 0
    }

    /// Read **SRAM2** `[12:12]`
    #[inline(always)]
    pub const fn sram2(&self) -> bool {
        (self.bits & FrceOn::SRAM2_MASK) != 0
    }

    /// Read **SRAM1** `[11:11]`
    #[inline(always)]
    pub const fn sram1(&self) -> bool {
        (self.bits & FrceOn::SRAM1_MASK) != 0
    }

    /// Read **SRAM0** `[10:10]`
    #[inline(always)]
    pub const fn sram0(&self) -> bool {
        (self.bits & FrceOn::SRAM0_MASK) != 0
    }

    /// Read **BOOTRAM** `[9:9]`
    #[inline(always)]
    pub const fn bootram(&self) -> bool {
        (self.bits & FrceOn::BOOTRAM_MASK) != 0
    }

    /// Read **ROM** `[8:8]`
    #[inline(always)]
    pub const fn rom(&self) -> bool {
        (self.bits & FrceOn::ROM_MASK) != 0
    }

    /// Read **BUSFABRIC** `[7:7]`
    #[inline(always)]
    pub const fn busfabric(&self) -> bool {
        (self.bits & FrceOn::BUSFABRIC_MASK) != 0
    }

    /// Read **PSM_READY** `[6:6]`
    #[inline(always)]
    pub const fn psm_ready(&self) -> bool {
        (self.bits & FrceOn::PSM_READY_MASK) != 0
    }

    /// Read **CLOCKS** `[5:5]`
    #[inline(always)]
    pub const fn clocks(&self) -> bool {
        (self.bits & FrceOn::CLOCKS_MASK) != 0
    }

    /// Read **RESETS** `[4:4]`
    #[inline(always)]
    pub const fn resets(&self) -> bool {
        (self.bits & FrceOn::RESETS_MASK) != 0
    }

    /// Read **XOSC** `[3:3]`
    #[inline(always)]
    pub const fn xosc(&self) -> bool {
        (self.bits & FrceOn::XOSC_MASK) != 0
    }

    /// Read **ROSC** `[2:2]`
    #[inline(always)]
    pub const fn rosc(&self) -> bool {
        (self.bits & FrceOn::ROSC_MASK) != 0
    }

    /// Read **OTP** `[1:1]`
    #[inline(always)]
    pub const fn otp(&self) -> bool {
        (self.bits & FrceOn::OTP_MASK) != 0
    }

    /// Read **PROC_COLD** `[0:0]`
    #[inline(always)]
    pub const fn proc_cold(&self) -> bool {
        (self.bits & FrceOn::PROC_COLD_MASK) != 0
    }

    /// Set **PROC1** `[24:24]`
    #[inline(always)]
    pub const fn with_proc1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | FrceOn::PROC1_MASK
            } else {
                self.bits & !FrceOn::PROC1_MASK
            }
        }
    }

    /// Set **PROC0** `[23:23]`
    #[inline(always)]
    pub const fn with_proc0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | FrceOn::PROC0_MASK
            } else {
                self.bits & !FrceOn::PROC0_MASK
            }
        }
    }

    /// Set **ACCESSCTRL** `[22:22]`
    #[inline(always)]
    pub const fn with_accessctrl(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | FrceOn::ACCESSCTRL_MASK
            } else {
                self.bits & !FrceOn::ACCESSCTRL_MASK
            }
        }
    }

    /// Set **SIO** `[21:21]`
    #[inline(always)]
    pub const fn with_sio(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | FrceOn::SIO_MASK
            } else {
                self.bits & !FrceOn::SIO_MASK
            }
        }
    }

    /// Set **XIP** `[20:20]`
    #[inline(always)]
    pub const fn with_xip(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | FrceOn::XIP_MASK
            } else {
                self.bits & !FrceOn::XIP_MASK
            }
        }
    }

    /// Set **SRAM9** `[19:19]`
    #[inline(always)]
    pub const fn with_sram9(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | FrceOn::SRAM9_MASK
            } else {
                self.bits & !FrceOn::SRAM9_MASK
            }
        }
    }

    /// Set **SRAM8** `[18:18]`
    #[inline(always)]
    pub const fn with_sram8(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | FrceOn::SRAM8_MASK
            } else {
                self.bits & !FrceOn::SRAM8_MASK
            }
        }
    }

    /// Set **SRAM7** `[17:17]`
    #[inline(always)]
    pub const fn with_sram7(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | FrceOn::SRAM7_MASK
            } else {
                self.bits & !FrceOn::SRAM7_MASK
            }
        }
    }

    /// Set **SRAM6** `[16:16]`
    #[inline(always)]
    pub const fn with_sram6(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | FrceOn::SRAM6_MASK
            } else {
                self.bits & !FrceOn::SRAM6_MASK
            }
        }
    }

    /// Set **SRAM5** `[15:15]`
    #[inline(always)]
    pub const fn with_sram5(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | FrceOn::SRAM5_MASK
            } else {
                self.bits & !FrceOn::SRAM5_MASK
            }
        }
    }

    /// Set **SRAM4** `[14:14]`
    #[inline(always)]
    pub const fn with_sram4(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | FrceOn::SRAM4_MASK
            } else {
                self.bits & !FrceOn::SRAM4_MASK
            }
        }
    }

    /// Set **SRAM3** `[13:13]`
    #[inline(always)]
    pub const fn with_sram3(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | FrceOn::SRAM3_MASK
            } else {
                self.bits & !FrceOn::SRAM3_MASK
            }
        }
    }

    /// Set **SRAM2** `[12:12]`
    #[inline(always)]
    pub const fn with_sram2(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | FrceOn::SRAM2_MASK
            } else {
                self.bits & !FrceOn::SRAM2_MASK
            }
        }
    }

    /// Set **SRAM1** `[11:11]`
    #[inline(always)]
    pub const fn with_sram1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | FrceOn::SRAM1_MASK
            } else {
                self.bits & !FrceOn::SRAM1_MASK
            }
        }
    }

    /// Set **SRAM0** `[10:10]`
    #[inline(always)]
    pub const fn with_sram0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | FrceOn::SRAM0_MASK
            } else {
                self.bits & !FrceOn::SRAM0_MASK
            }
        }
    }

    /// Set **BOOTRAM** `[9:9]`
    #[inline(always)]
    pub const fn with_bootram(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | FrceOn::BOOTRAM_MASK
            } else {
                self.bits & !FrceOn::BOOTRAM_MASK
            }
        }
    }

    /// Set **ROM** `[8:8]`
    #[inline(always)]
    pub const fn with_rom(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | FrceOn::ROM_MASK
            } else {
                self.bits & !FrceOn::ROM_MASK
            }
        }
    }

    /// Set **BUSFABRIC** `[7:7]`
    #[inline(always)]
    pub const fn with_busfabric(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | FrceOn::BUSFABRIC_MASK
            } else {
                self.bits & !FrceOn::BUSFABRIC_MASK
            }
        }
    }

    /// Set **PSM_READY** `[6:6]`
    #[inline(always)]
    pub const fn with_psm_ready(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | FrceOn::PSM_READY_MASK
            } else {
                self.bits & !FrceOn::PSM_READY_MASK
            }
        }
    }

    /// Set **CLOCKS** `[5:5]`
    #[inline(always)]
    pub const fn with_clocks(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | FrceOn::CLOCKS_MASK
            } else {
                self.bits & !FrceOn::CLOCKS_MASK
            }
        }
    }

    /// Set **RESETS** `[4:4]`
    #[inline(always)]
    pub const fn with_resets(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | FrceOn::RESETS_MASK
            } else {
                self.bits & !FrceOn::RESETS_MASK
            }
        }
    }

    /// Set **XOSC** `[3:3]`
    #[inline(always)]
    pub const fn with_xosc(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | FrceOn::XOSC_MASK
            } else {
                self.bits & !FrceOn::XOSC_MASK
            }
        }
    }

    /// Set **ROSC** `[2:2]`
    #[inline(always)]
    pub const fn with_rosc(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | FrceOn::ROSC_MASK
            } else {
                self.bits & !FrceOn::ROSC_MASK
            }
        }
    }

    /// Set **OTP** `[1:1]`
    #[inline(always)]
    pub const fn with_otp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | FrceOn::OTP_MASK
            } else {
                self.bits & !FrceOn::OTP_MASK
            }
        }
    }

    /// Set **PROC_COLD** `[0:0]`
    #[inline(always)]
    pub const fn with_proc_cold(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | FrceOn::PROC_COLD_MASK
            } else {
                self.bits & !FrceOn::PROC_COLD_MASK
            }
        }
    }

}

impl Default for FrceOnVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl FrceOn {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> FrceOnVal {
        FrceOnVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: FrceOnVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(FrceOnVal) -> FrceOnVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// FRCE_OFF Register
///
/// Force into reset (i.e. power it off)
///
/// Address offset: 0x0004
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **PROC1** `[24]` (RW)
///
/// - **PROC0** `[23]` (RW)
///
/// - **ACCESSCTRL** `[22]` (RW)
///
/// - **SIO** `[21]` (RW)
///
/// - **XIP** `[20]` (RW)
///
/// - **SRAM9** `[19]` (RW)
///
/// - **SRAM8** `[18]` (RW)
///
/// - **SRAM7** `[17]` (RW)
///
/// - **SRAM6** `[16]` (RW)
///
/// - **SRAM5** `[15]` (RW)
///
/// - **SRAM4** `[14]` (RW)
///
/// - **SRAM3** `[13]` (RW)
///
/// - **SRAM2** `[12]` (RW)
///
/// - **SRAM1** `[11]` (RW)
///
/// - **SRAM0** `[10]` (RW)
///
/// - **BOOTRAM** `[9]` (RW)
///
/// - **ROM** `[8]` (RW)
///
/// - **BUSFABRIC** `[7]` (RW)
///
/// - **PSM_READY** `[6]` (RW)
///
/// - **CLOCKS** `[5]` (RW)
///
/// - **RESETS** `[4]` (RW)
///
/// - **XOSC** `[3]` (RW)
///
/// - **ROSC** `[2]` (RW)
///
/// - **OTP** `[1]` (RW)
///
/// - **PROC_COLD** `[0]` (RW)
#[repr(transparent)]
pub struct FrceOff {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for FrceOff {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl FrceOff {
    // ======== Field Constants ========

    /// **PROC1**
    /// Bits `[24:24]`, mask `0x01000000`
    pub const PROC1_OFFSET: u32 = 24;
    pub const PROC1_MASK: u32 = 0x01000000;
    pub const PROC1: u32 = 1 << 24;

    /// **PROC0**
    /// Bits `[23:23]`, mask `0x00800000`
    pub const PROC0_OFFSET: u32 = 23;
    pub const PROC0_MASK: u32 = 0x00800000;
    pub const PROC0: u32 = 1 << 23;

    /// **ACCESSCTRL**
    /// Bits `[22:22]`, mask `0x00400000`
    pub const ACCESSCTRL_OFFSET: u32 = 22;
    pub const ACCESSCTRL_MASK: u32 = 0x00400000;
    pub const ACCESSCTRL: u32 = 1 << 22;

    /// **SIO**
    /// Bits `[21:21]`, mask `0x00200000`
    pub const SIO_OFFSET: u32 = 21;
    pub const SIO_MASK: u32 = 0x00200000;
    pub const SIO: u32 = 1 << 21;

    /// **XIP**
    /// Bits `[20:20]`, mask `0x00100000`
    pub const XIP_OFFSET: u32 = 20;
    pub const XIP_MASK: u32 = 0x00100000;
    pub const XIP: u32 = 1 << 20;

    /// **SRAM9**
    /// Bits `[19:19]`, mask `0x00080000`
    pub const SRAM9_OFFSET: u32 = 19;
    pub const SRAM9_MASK: u32 = 0x00080000;
    pub const SRAM9: u32 = 1 << 19;

    /// **SRAM8**
    /// Bits `[18:18]`, mask `0x00040000`
    pub const SRAM8_OFFSET: u32 = 18;
    pub const SRAM8_MASK: u32 = 0x00040000;
    pub const SRAM8: u32 = 1 << 18;

    /// **SRAM7**
    /// Bits `[17:17]`, mask `0x00020000`
    pub const SRAM7_OFFSET: u32 = 17;
    pub const SRAM7_MASK: u32 = 0x00020000;
    pub const SRAM7: u32 = 1 << 17;

    /// **SRAM6**
    /// Bits `[16:16]`, mask `0x00010000`
    pub const SRAM6_OFFSET: u32 = 16;
    pub const SRAM6_MASK: u32 = 0x00010000;
    pub const SRAM6: u32 = 1 << 16;

    /// **SRAM5**
    /// Bits `[15:15]`, mask `0x00008000`
    pub const SRAM5_OFFSET: u32 = 15;
    pub const SRAM5_MASK: u32 = 0x00008000;
    pub const SRAM5: u32 = 1 << 15;

    /// **SRAM4**
    /// Bits `[14:14]`, mask `0x00004000`
    pub const SRAM4_OFFSET: u32 = 14;
    pub const SRAM4_MASK: u32 = 0x00004000;
    pub const SRAM4: u32 = 1 << 14;

    /// **SRAM3**
    /// Bits `[13:13]`, mask `0x00002000`
    pub const SRAM3_OFFSET: u32 = 13;
    pub const SRAM3_MASK: u32 = 0x00002000;
    pub const SRAM3: u32 = 1 << 13;

    /// **SRAM2**
    /// Bits `[12:12]`, mask `0x00001000`
    pub const SRAM2_OFFSET: u32 = 12;
    pub const SRAM2_MASK: u32 = 0x00001000;
    pub const SRAM2: u32 = 1 << 12;

    /// **SRAM1**
    /// Bits `[11:11]`, mask `0x00000800`
    pub const SRAM1_OFFSET: u32 = 11;
    pub const SRAM1_MASK: u32 = 0x00000800;
    pub const SRAM1: u32 = 1 << 11;

    /// **SRAM0**
    /// Bits `[10:10]`, mask `0x00000400`
    pub const SRAM0_OFFSET: u32 = 10;
    pub const SRAM0_MASK: u32 = 0x00000400;
    pub const SRAM0: u32 = 1 << 10;

    /// **BOOTRAM**
    /// Bits `[9:9]`, mask `0x00000200`
    pub const BOOTRAM_OFFSET: u32 = 9;
    pub const BOOTRAM_MASK: u32 = 0x00000200;
    pub const BOOTRAM: u32 = 1 << 9;

    /// **ROM**
    /// Bits `[8:8]`, mask `0x00000100`
    pub const ROM_OFFSET: u32 = 8;
    pub const ROM_MASK: u32 = 0x00000100;
    pub const ROM: u32 = 1 << 8;

    /// **BUSFABRIC**
    /// Bits `[7:7]`, mask `0x00000080`
    pub const BUSFABRIC_OFFSET: u32 = 7;
    pub const BUSFABRIC_MASK: u32 = 0x00000080;
    pub const BUSFABRIC: u32 = 1 << 7;

    /// **PSM_READY**
    /// Bits `[6:6]`, mask `0x00000040`
    pub const PSM_READY_OFFSET: u32 = 6;
    pub const PSM_READY_MASK: u32 = 0x00000040;
    pub const PSM_READY: u32 = 1 << 6;

    /// **CLOCKS**
    /// Bits `[5:5]`, mask `0x00000020`
    pub const CLOCKS_OFFSET: u32 = 5;
    pub const CLOCKS_MASK: u32 = 0x00000020;
    pub const CLOCKS: u32 = 1 << 5;

    /// **RESETS**
    /// Bits `[4:4]`, mask `0x00000010`
    pub const RESETS_OFFSET: u32 = 4;
    pub const RESETS_MASK: u32 = 0x00000010;
    pub const RESETS: u32 = 1 << 4;

    /// **XOSC**
    /// Bits `[3:3]`, mask `0x00000008`
    pub const XOSC_OFFSET: u32 = 3;
    pub const XOSC_MASK: u32 = 0x00000008;
    pub const XOSC: u32 = 1 << 3;

    /// **ROSC**
    /// Bits `[2:2]`, mask `0x00000004`
    pub const ROSC_OFFSET: u32 = 2;
    pub const ROSC_MASK: u32 = 0x00000004;
    pub const ROSC: u32 = 1 << 2;

    /// **OTP**
    /// Bits `[1:1]`, mask `0x00000002`
    pub const OTP_OFFSET: u32 = 1;
    pub const OTP_MASK: u32 = 0x00000002;
    pub const OTP: u32 = 1 << 1;

    /// **PROC_COLD**
    /// Bits `[0:0]`, mask `0x00000001`
    pub const PROC_COLD_OFFSET: u32 = 0;
    pub const PROC_COLD_MASK: u32 = 0x00000001;
    pub const PROC_COLD: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **PROC1** `[24]` (RW)
    #[inline(always)]
    pub fn proc1(&self) -> bool {
        (self.value.read_bits() & Self::PROC1_MASK) != 0
    }

    /// Set PROC1 bit atomically (RW)
    #[inline(always)]
    pub fn set_proc1(&self) {
        self.set_bits(Self::PROC1_MASK);
    }

    /// Clear PROC1 bit atomically (RW)
    #[inline(always)]
    pub fn clr_proc1(&self) {
        self.clr_bits(Self::PROC1_MASK);
    }

    /// **PROC0** `[23]` (RW)
    #[inline(always)]
    pub fn proc0(&self) -> bool {
        (self.value.read_bits() & Self::PROC0_MASK) != 0
    }

    /// Set PROC0 bit atomically (RW)
    #[inline(always)]
    pub fn set_proc0(&self) {
        self.set_bits(Self::PROC0_MASK);
    }

    /// Clear PROC0 bit atomically (RW)
    #[inline(always)]
    pub fn clr_proc0(&self) {
        self.clr_bits(Self::PROC0_MASK);
    }

    /// **ACCESSCTRL** `[22]` (RW)
    #[inline(always)]
    pub fn accessctrl(&self) -> bool {
        (self.value.read_bits() & Self::ACCESSCTRL_MASK) != 0
    }

    /// Set ACCESSCTRL bit atomically (RW)
    #[inline(always)]
    pub fn set_accessctrl(&self) {
        self.set_bits(Self::ACCESSCTRL_MASK);
    }

    /// Clear ACCESSCTRL bit atomically (RW)
    #[inline(always)]
    pub fn clr_accessctrl(&self) {
        self.clr_bits(Self::ACCESSCTRL_MASK);
    }

    /// **SIO** `[21]` (RW)
    #[inline(always)]
    pub fn sio(&self) -> bool {
        (self.value.read_bits() & Self::SIO_MASK) != 0
    }

    /// Set SIO bit atomically (RW)
    #[inline(always)]
    pub fn set_sio(&self) {
        self.set_bits(Self::SIO_MASK);
    }

    /// Clear SIO bit atomically (RW)
    #[inline(always)]
    pub fn clr_sio(&self) {
        self.clr_bits(Self::SIO_MASK);
    }

    /// **XIP** `[20]` (RW)
    #[inline(always)]
    pub fn xip(&self) -> bool {
        (self.value.read_bits() & Self::XIP_MASK) != 0
    }

    /// Set XIP bit atomically (RW)
    #[inline(always)]
    pub fn set_xip(&self) {
        self.set_bits(Self::XIP_MASK);
    }

    /// Clear XIP bit atomically (RW)
    #[inline(always)]
    pub fn clr_xip(&self) {
        self.clr_bits(Self::XIP_MASK);
    }

    /// **SRAM9** `[19]` (RW)
    #[inline(always)]
    pub fn sram9(&self) -> bool {
        (self.value.read_bits() & Self::SRAM9_MASK) != 0
    }

    /// Set SRAM9 bit atomically (RW)
    #[inline(always)]
    pub fn set_sram9(&self) {
        self.set_bits(Self::SRAM9_MASK);
    }

    /// Clear SRAM9 bit atomically (RW)
    #[inline(always)]
    pub fn clr_sram9(&self) {
        self.clr_bits(Self::SRAM9_MASK);
    }

    /// **SRAM8** `[18]` (RW)
    #[inline(always)]
    pub fn sram8(&self) -> bool {
        (self.value.read_bits() & Self::SRAM8_MASK) != 0
    }

    /// Set SRAM8 bit atomically (RW)
    #[inline(always)]
    pub fn set_sram8(&self) {
        self.set_bits(Self::SRAM8_MASK);
    }

    /// Clear SRAM8 bit atomically (RW)
    #[inline(always)]
    pub fn clr_sram8(&self) {
        self.clr_bits(Self::SRAM8_MASK);
    }

    /// **SRAM7** `[17]` (RW)
    #[inline(always)]
    pub fn sram7(&self) -> bool {
        (self.value.read_bits() & Self::SRAM7_MASK) != 0
    }

    /// Set SRAM7 bit atomically (RW)
    #[inline(always)]
    pub fn set_sram7(&self) {
        self.set_bits(Self::SRAM7_MASK);
    }

    /// Clear SRAM7 bit atomically (RW)
    #[inline(always)]
    pub fn clr_sram7(&self) {
        self.clr_bits(Self::SRAM7_MASK);
    }

    /// **SRAM6** `[16]` (RW)
    #[inline(always)]
    pub fn sram6(&self) -> bool {
        (self.value.read_bits() & Self::SRAM6_MASK) != 0
    }

    /// Set SRAM6 bit atomically (RW)
    #[inline(always)]
    pub fn set_sram6(&self) {
        self.set_bits(Self::SRAM6_MASK);
    }

    /// Clear SRAM6 bit atomically (RW)
    #[inline(always)]
    pub fn clr_sram6(&self) {
        self.clr_bits(Self::SRAM6_MASK);
    }

    /// **SRAM5** `[15]` (RW)
    #[inline(always)]
    pub fn sram5(&self) -> bool {
        (self.value.read_bits() & Self::SRAM5_MASK) != 0
    }

    /// Set SRAM5 bit atomically (RW)
    #[inline(always)]
    pub fn set_sram5(&self) {
        self.set_bits(Self::SRAM5_MASK);
    }

    /// Clear SRAM5 bit atomically (RW)
    #[inline(always)]
    pub fn clr_sram5(&self) {
        self.clr_bits(Self::SRAM5_MASK);
    }

    /// **SRAM4** `[14]` (RW)
    #[inline(always)]
    pub fn sram4(&self) -> bool {
        (self.value.read_bits() & Self::SRAM4_MASK) != 0
    }

    /// Set SRAM4 bit atomically (RW)
    #[inline(always)]
    pub fn set_sram4(&self) {
        self.set_bits(Self::SRAM4_MASK);
    }

    /// Clear SRAM4 bit atomically (RW)
    #[inline(always)]
    pub fn clr_sram4(&self) {
        self.clr_bits(Self::SRAM4_MASK);
    }

    /// **SRAM3** `[13]` (RW)
    #[inline(always)]
    pub fn sram3(&self) -> bool {
        (self.value.read_bits() & Self::SRAM3_MASK) != 0
    }

    /// Set SRAM3 bit atomically (RW)
    #[inline(always)]
    pub fn set_sram3(&self) {
        self.set_bits(Self::SRAM3_MASK);
    }

    /// Clear SRAM3 bit atomically (RW)
    #[inline(always)]
    pub fn clr_sram3(&self) {
        self.clr_bits(Self::SRAM3_MASK);
    }

    /// **SRAM2** `[12]` (RW)
    #[inline(always)]
    pub fn sram2(&self) -> bool {
        (self.value.read_bits() & Self::SRAM2_MASK) != 0
    }

    /// Set SRAM2 bit atomically (RW)
    #[inline(always)]
    pub fn set_sram2(&self) {
        self.set_bits(Self::SRAM2_MASK);
    }

    /// Clear SRAM2 bit atomically (RW)
    #[inline(always)]
    pub fn clr_sram2(&self) {
        self.clr_bits(Self::SRAM2_MASK);
    }

    /// **SRAM1** `[11]` (RW)
    #[inline(always)]
    pub fn sram1(&self) -> bool {
        (self.value.read_bits() & Self::SRAM1_MASK) != 0
    }

    /// Set SRAM1 bit atomically (RW)
    #[inline(always)]
    pub fn set_sram1(&self) {
        self.set_bits(Self::SRAM1_MASK);
    }

    /// Clear SRAM1 bit atomically (RW)
    #[inline(always)]
    pub fn clr_sram1(&self) {
        self.clr_bits(Self::SRAM1_MASK);
    }

    /// **SRAM0** `[10]` (RW)
    #[inline(always)]
    pub fn sram0(&self) -> bool {
        (self.value.read_bits() & Self::SRAM0_MASK) != 0
    }

    /// Set SRAM0 bit atomically (RW)
    #[inline(always)]
    pub fn set_sram0(&self) {
        self.set_bits(Self::SRAM0_MASK);
    }

    /// Clear SRAM0 bit atomically (RW)
    #[inline(always)]
    pub fn clr_sram0(&self) {
        self.clr_bits(Self::SRAM0_MASK);
    }

    /// **BOOTRAM** `[9]` (RW)
    #[inline(always)]
    pub fn bootram(&self) -> bool {
        (self.value.read_bits() & Self::BOOTRAM_MASK) != 0
    }

    /// Set BOOTRAM bit atomically (RW)
    #[inline(always)]
    pub fn set_bootram(&self) {
        self.set_bits(Self::BOOTRAM_MASK);
    }

    /// Clear BOOTRAM bit atomically (RW)
    #[inline(always)]
    pub fn clr_bootram(&self) {
        self.clr_bits(Self::BOOTRAM_MASK);
    }

    /// **ROM** `[8]` (RW)
    #[inline(always)]
    pub fn rom(&self) -> bool {
        (self.value.read_bits() & Self::ROM_MASK) != 0
    }

    /// Set ROM bit atomically (RW)
    #[inline(always)]
    pub fn set_rom(&self) {
        self.set_bits(Self::ROM_MASK);
    }

    /// Clear ROM bit atomically (RW)
    #[inline(always)]
    pub fn clr_rom(&self) {
        self.clr_bits(Self::ROM_MASK);
    }

    /// **BUSFABRIC** `[7]` (RW)
    #[inline(always)]
    pub fn busfabric(&self) -> bool {
        (self.value.read_bits() & Self::BUSFABRIC_MASK) != 0
    }

    /// Set BUSFABRIC bit atomically (RW)
    #[inline(always)]
    pub fn set_busfabric(&self) {
        self.set_bits(Self::BUSFABRIC_MASK);
    }

    /// Clear BUSFABRIC bit atomically (RW)
    #[inline(always)]
    pub fn clr_busfabric(&self) {
        self.clr_bits(Self::BUSFABRIC_MASK);
    }

    /// **PSM_READY** `[6]` (RW)
    #[inline(always)]
    pub fn psm_ready(&self) -> bool {
        (self.value.read_bits() & Self::PSM_READY_MASK) != 0
    }

    /// Set PSM_READY bit atomically (RW)
    #[inline(always)]
    pub fn set_psm_ready(&self) {
        self.set_bits(Self::PSM_READY_MASK);
    }

    /// Clear PSM_READY bit atomically (RW)
    #[inline(always)]
    pub fn clr_psm_ready(&self) {
        self.clr_bits(Self::PSM_READY_MASK);
    }

    /// **CLOCKS** `[5]` (RW)
    #[inline(always)]
    pub fn clocks(&self) -> bool {
        (self.value.read_bits() & Self::CLOCKS_MASK) != 0
    }

    /// Set CLOCKS bit atomically (RW)
    #[inline(always)]
    pub fn set_clocks(&self) {
        self.set_bits(Self::CLOCKS_MASK);
    }

    /// Clear CLOCKS bit atomically (RW)
    #[inline(always)]
    pub fn clr_clocks(&self) {
        self.clr_bits(Self::CLOCKS_MASK);
    }

    /// **RESETS** `[4]` (RW)
    #[inline(always)]
    pub fn resets(&self) -> bool {
        (self.value.read_bits() & Self::RESETS_MASK) != 0
    }

    /// Set RESETS bit atomically (RW)
    #[inline(always)]
    pub fn set_resets(&self) {
        self.set_bits(Self::RESETS_MASK);
    }

    /// Clear RESETS bit atomically (RW)
    #[inline(always)]
    pub fn clr_resets(&self) {
        self.clr_bits(Self::RESETS_MASK);
    }

    /// **XOSC** `[3]` (RW)
    #[inline(always)]
    pub fn xosc(&self) -> bool {
        (self.value.read_bits() & Self::XOSC_MASK) != 0
    }

    /// Set XOSC bit atomically (RW)
    #[inline(always)]
    pub fn set_xosc(&self) {
        self.set_bits(Self::XOSC_MASK);
    }

    /// Clear XOSC bit atomically (RW)
    #[inline(always)]
    pub fn clr_xosc(&self) {
        self.clr_bits(Self::XOSC_MASK);
    }

    /// **ROSC** `[2]` (RW)
    #[inline(always)]
    pub fn rosc(&self) -> bool {
        (self.value.read_bits() & Self::ROSC_MASK) != 0
    }

    /// Set ROSC bit atomically (RW)
    #[inline(always)]
    pub fn set_rosc(&self) {
        self.set_bits(Self::ROSC_MASK);
    }

    /// Clear ROSC bit atomically (RW)
    #[inline(always)]
    pub fn clr_rosc(&self) {
        self.clr_bits(Self::ROSC_MASK);
    }

    /// **OTP** `[1]` (RW)
    #[inline(always)]
    pub fn otp(&self) -> bool {
        (self.value.read_bits() & Self::OTP_MASK) != 0
    }

    /// Set OTP bit atomically (RW)
    #[inline(always)]
    pub fn set_otp(&self) {
        self.set_bits(Self::OTP_MASK);
    }

    /// Clear OTP bit atomically (RW)
    #[inline(always)]
    pub fn clr_otp(&self) {
        self.clr_bits(Self::OTP_MASK);
    }

    /// **PROC_COLD** `[0]` (RW)
    #[inline(always)]
    pub fn proc_cold(&self) -> bool {
        (self.value.read_bits() & Self::PROC_COLD_MASK) != 0
    }

    /// Set PROC_COLD bit atomically (RW)
    #[inline(always)]
    pub fn set_proc_cold(&self) {
        self.set_bits(Self::PROC_COLD_MASK);
    }

    /// Clear PROC_COLD bit atomically (RW)
    #[inline(always)]
    pub fn clr_proc_cold(&self) {
        self.clr_bits(Self::PROC_COLD_MASK);
    }

}

/// FRCE_OFF value type for batch read/modify/write operations
///
/// Use `frceoff.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `frceoff.write_val(val)`.
#[derive(Clone, Copy)]
pub struct FrceOffVal {
    bits: u32,
}

impl FrceOffVal {
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

    /// Read **PROC1** `[24:24]`
    #[inline(always)]
    pub const fn proc1(&self) -> bool {
        (self.bits & FrceOff::PROC1_MASK) != 0
    }

    /// Read **PROC0** `[23:23]`
    #[inline(always)]
    pub const fn proc0(&self) -> bool {
        (self.bits & FrceOff::PROC0_MASK) != 0
    }

    /// Read **ACCESSCTRL** `[22:22]`
    #[inline(always)]
    pub const fn accessctrl(&self) -> bool {
        (self.bits & FrceOff::ACCESSCTRL_MASK) != 0
    }

    /// Read **SIO** `[21:21]`
    #[inline(always)]
    pub const fn sio(&self) -> bool {
        (self.bits & FrceOff::SIO_MASK) != 0
    }

    /// Read **XIP** `[20:20]`
    #[inline(always)]
    pub const fn xip(&self) -> bool {
        (self.bits & FrceOff::XIP_MASK) != 0
    }

    /// Read **SRAM9** `[19:19]`
    #[inline(always)]
    pub const fn sram9(&self) -> bool {
        (self.bits & FrceOff::SRAM9_MASK) != 0
    }

    /// Read **SRAM8** `[18:18]`
    #[inline(always)]
    pub const fn sram8(&self) -> bool {
        (self.bits & FrceOff::SRAM8_MASK) != 0
    }

    /// Read **SRAM7** `[17:17]`
    #[inline(always)]
    pub const fn sram7(&self) -> bool {
        (self.bits & FrceOff::SRAM7_MASK) != 0
    }

    /// Read **SRAM6** `[16:16]`
    #[inline(always)]
    pub const fn sram6(&self) -> bool {
        (self.bits & FrceOff::SRAM6_MASK) != 0
    }

    /// Read **SRAM5** `[15:15]`
    #[inline(always)]
    pub const fn sram5(&self) -> bool {
        (self.bits & FrceOff::SRAM5_MASK) != 0
    }

    /// Read **SRAM4** `[14:14]`
    #[inline(always)]
    pub const fn sram4(&self) -> bool {
        (self.bits & FrceOff::SRAM4_MASK) != 0
    }

    /// Read **SRAM3** `[13:13]`
    #[inline(always)]
    pub const fn sram3(&self) -> bool {
        (self.bits & FrceOff::SRAM3_MASK) != 0
    }

    /// Read **SRAM2** `[12:12]`
    #[inline(always)]
    pub const fn sram2(&self) -> bool {
        (self.bits & FrceOff::SRAM2_MASK) != 0
    }

    /// Read **SRAM1** `[11:11]`
    #[inline(always)]
    pub const fn sram1(&self) -> bool {
        (self.bits & FrceOff::SRAM1_MASK) != 0
    }

    /// Read **SRAM0** `[10:10]`
    #[inline(always)]
    pub const fn sram0(&self) -> bool {
        (self.bits & FrceOff::SRAM0_MASK) != 0
    }

    /// Read **BOOTRAM** `[9:9]`
    #[inline(always)]
    pub const fn bootram(&self) -> bool {
        (self.bits & FrceOff::BOOTRAM_MASK) != 0
    }

    /// Read **ROM** `[8:8]`
    #[inline(always)]
    pub const fn rom(&self) -> bool {
        (self.bits & FrceOff::ROM_MASK) != 0
    }

    /// Read **BUSFABRIC** `[7:7]`
    #[inline(always)]
    pub const fn busfabric(&self) -> bool {
        (self.bits & FrceOff::BUSFABRIC_MASK) != 0
    }

    /// Read **PSM_READY** `[6:6]`
    #[inline(always)]
    pub const fn psm_ready(&self) -> bool {
        (self.bits & FrceOff::PSM_READY_MASK) != 0
    }

    /// Read **CLOCKS** `[5:5]`
    #[inline(always)]
    pub const fn clocks(&self) -> bool {
        (self.bits & FrceOff::CLOCKS_MASK) != 0
    }

    /// Read **RESETS** `[4:4]`
    #[inline(always)]
    pub const fn resets(&self) -> bool {
        (self.bits & FrceOff::RESETS_MASK) != 0
    }

    /// Read **XOSC** `[3:3]`
    #[inline(always)]
    pub const fn xosc(&self) -> bool {
        (self.bits & FrceOff::XOSC_MASK) != 0
    }

    /// Read **ROSC** `[2:2]`
    #[inline(always)]
    pub const fn rosc(&self) -> bool {
        (self.bits & FrceOff::ROSC_MASK) != 0
    }

    /// Read **OTP** `[1:1]`
    #[inline(always)]
    pub const fn otp(&self) -> bool {
        (self.bits & FrceOff::OTP_MASK) != 0
    }

    /// Read **PROC_COLD** `[0:0]`
    #[inline(always)]
    pub const fn proc_cold(&self) -> bool {
        (self.bits & FrceOff::PROC_COLD_MASK) != 0
    }

    /// Set **PROC1** `[24:24]`
    #[inline(always)]
    pub const fn with_proc1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | FrceOff::PROC1_MASK
            } else {
                self.bits & !FrceOff::PROC1_MASK
            }
        }
    }

    /// Set **PROC0** `[23:23]`
    #[inline(always)]
    pub const fn with_proc0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | FrceOff::PROC0_MASK
            } else {
                self.bits & !FrceOff::PROC0_MASK
            }
        }
    }

    /// Set **ACCESSCTRL** `[22:22]`
    #[inline(always)]
    pub const fn with_accessctrl(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | FrceOff::ACCESSCTRL_MASK
            } else {
                self.bits & !FrceOff::ACCESSCTRL_MASK
            }
        }
    }

    /// Set **SIO** `[21:21]`
    #[inline(always)]
    pub const fn with_sio(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | FrceOff::SIO_MASK
            } else {
                self.bits & !FrceOff::SIO_MASK
            }
        }
    }

    /// Set **XIP** `[20:20]`
    #[inline(always)]
    pub const fn with_xip(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | FrceOff::XIP_MASK
            } else {
                self.bits & !FrceOff::XIP_MASK
            }
        }
    }

    /// Set **SRAM9** `[19:19]`
    #[inline(always)]
    pub const fn with_sram9(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | FrceOff::SRAM9_MASK
            } else {
                self.bits & !FrceOff::SRAM9_MASK
            }
        }
    }

    /// Set **SRAM8** `[18:18]`
    #[inline(always)]
    pub const fn with_sram8(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | FrceOff::SRAM8_MASK
            } else {
                self.bits & !FrceOff::SRAM8_MASK
            }
        }
    }

    /// Set **SRAM7** `[17:17]`
    #[inline(always)]
    pub const fn with_sram7(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | FrceOff::SRAM7_MASK
            } else {
                self.bits & !FrceOff::SRAM7_MASK
            }
        }
    }

    /// Set **SRAM6** `[16:16]`
    #[inline(always)]
    pub const fn with_sram6(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | FrceOff::SRAM6_MASK
            } else {
                self.bits & !FrceOff::SRAM6_MASK
            }
        }
    }

    /// Set **SRAM5** `[15:15]`
    #[inline(always)]
    pub const fn with_sram5(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | FrceOff::SRAM5_MASK
            } else {
                self.bits & !FrceOff::SRAM5_MASK
            }
        }
    }

    /// Set **SRAM4** `[14:14]`
    #[inline(always)]
    pub const fn with_sram4(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | FrceOff::SRAM4_MASK
            } else {
                self.bits & !FrceOff::SRAM4_MASK
            }
        }
    }

    /// Set **SRAM3** `[13:13]`
    #[inline(always)]
    pub const fn with_sram3(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | FrceOff::SRAM3_MASK
            } else {
                self.bits & !FrceOff::SRAM3_MASK
            }
        }
    }

    /// Set **SRAM2** `[12:12]`
    #[inline(always)]
    pub const fn with_sram2(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | FrceOff::SRAM2_MASK
            } else {
                self.bits & !FrceOff::SRAM2_MASK
            }
        }
    }

    /// Set **SRAM1** `[11:11]`
    #[inline(always)]
    pub const fn with_sram1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | FrceOff::SRAM1_MASK
            } else {
                self.bits & !FrceOff::SRAM1_MASK
            }
        }
    }

    /// Set **SRAM0** `[10:10]`
    #[inline(always)]
    pub const fn with_sram0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | FrceOff::SRAM0_MASK
            } else {
                self.bits & !FrceOff::SRAM0_MASK
            }
        }
    }

    /// Set **BOOTRAM** `[9:9]`
    #[inline(always)]
    pub const fn with_bootram(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | FrceOff::BOOTRAM_MASK
            } else {
                self.bits & !FrceOff::BOOTRAM_MASK
            }
        }
    }

    /// Set **ROM** `[8:8]`
    #[inline(always)]
    pub const fn with_rom(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | FrceOff::ROM_MASK
            } else {
                self.bits & !FrceOff::ROM_MASK
            }
        }
    }

    /// Set **BUSFABRIC** `[7:7]`
    #[inline(always)]
    pub const fn with_busfabric(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | FrceOff::BUSFABRIC_MASK
            } else {
                self.bits & !FrceOff::BUSFABRIC_MASK
            }
        }
    }

    /// Set **PSM_READY** `[6:6]`
    #[inline(always)]
    pub const fn with_psm_ready(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | FrceOff::PSM_READY_MASK
            } else {
                self.bits & !FrceOff::PSM_READY_MASK
            }
        }
    }

    /// Set **CLOCKS** `[5:5]`
    #[inline(always)]
    pub const fn with_clocks(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | FrceOff::CLOCKS_MASK
            } else {
                self.bits & !FrceOff::CLOCKS_MASK
            }
        }
    }

    /// Set **RESETS** `[4:4]`
    #[inline(always)]
    pub const fn with_resets(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | FrceOff::RESETS_MASK
            } else {
                self.bits & !FrceOff::RESETS_MASK
            }
        }
    }

    /// Set **XOSC** `[3:3]`
    #[inline(always)]
    pub const fn with_xosc(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | FrceOff::XOSC_MASK
            } else {
                self.bits & !FrceOff::XOSC_MASK
            }
        }
    }

    /// Set **ROSC** `[2:2]`
    #[inline(always)]
    pub const fn with_rosc(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | FrceOff::ROSC_MASK
            } else {
                self.bits & !FrceOff::ROSC_MASK
            }
        }
    }

    /// Set **OTP** `[1:1]`
    #[inline(always)]
    pub const fn with_otp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | FrceOff::OTP_MASK
            } else {
                self.bits & !FrceOff::OTP_MASK
            }
        }
    }

    /// Set **PROC_COLD** `[0:0]`
    #[inline(always)]
    pub const fn with_proc_cold(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | FrceOff::PROC_COLD_MASK
            } else {
                self.bits & !FrceOff::PROC_COLD_MASK
            }
        }
    }

}

impl Default for FrceOffVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl FrceOff {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> FrceOffVal {
        FrceOffVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: FrceOffVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(FrceOffVal) -> FrceOffVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// WDSEL Register
///
/// Set to 1 if the watchdog should reset this
///
/// Address offset: 0x0008
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **PROC1** `[24]` (RW)
///
/// - **PROC0** `[23]` (RW)
///
/// - **ACCESSCTRL** `[22]` (RW)
///
/// - **SIO** `[21]` (RW)
///
/// - **XIP** `[20]` (RW)
///
/// - **SRAM9** `[19]` (RW)
///
/// - **SRAM8** `[18]` (RW)
///
/// - **SRAM7** `[17]` (RW)
///
/// - **SRAM6** `[16]` (RW)
///
/// - **SRAM5** `[15]` (RW)
///
/// - **SRAM4** `[14]` (RW)
///
/// - **SRAM3** `[13]` (RW)
///
/// - **SRAM2** `[12]` (RW)
///
/// - **SRAM1** `[11]` (RW)
///
/// - **SRAM0** `[10]` (RW)
///
/// - **BOOTRAM** `[9]` (RW)
///
/// - **ROM** `[8]` (RW)
///
/// - **BUSFABRIC** `[7]` (RW)
///
/// - **PSM_READY** `[6]` (RW)
///
/// - **CLOCKS** `[5]` (RW)
///
/// - **RESETS** `[4]` (RW)
///
/// - **XOSC** `[3]` (RW)
///
/// - **ROSC** `[2]` (RW)
///
/// - **OTP** `[1]` (RW)
///
/// - **PROC_COLD** `[0]` (RW)
#[repr(transparent)]
pub struct Wdsel {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Wdsel {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Wdsel {
    // ======== Field Constants ========

    /// **PROC1**
    /// Bits `[24:24]`, mask `0x01000000`
    pub const PROC1_OFFSET: u32 = 24;
    pub const PROC1_MASK: u32 = 0x01000000;
    pub const PROC1: u32 = 1 << 24;

    /// **PROC0**
    /// Bits `[23:23]`, mask `0x00800000`
    pub const PROC0_OFFSET: u32 = 23;
    pub const PROC0_MASK: u32 = 0x00800000;
    pub const PROC0: u32 = 1 << 23;

    /// **ACCESSCTRL**
    /// Bits `[22:22]`, mask `0x00400000`
    pub const ACCESSCTRL_OFFSET: u32 = 22;
    pub const ACCESSCTRL_MASK: u32 = 0x00400000;
    pub const ACCESSCTRL: u32 = 1 << 22;

    /// **SIO**
    /// Bits `[21:21]`, mask `0x00200000`
    pub const SIO_OFFSET: u32 = 21;
    pub const SIO_MASK: u32 = 0x00200000;
    pub const SIO: u32 = 1 << 21;

    /// **XIP**
    /// Bits `[20:20]`, mask `0x00100000`
    pub const XIP_OFFSET: u32 = 20;
    pub const XIP_MASK: u32 = 0x00100000;
    pub const XIP: u32 = 1 << 20;

    /// **SRAM9**
    /// Bits `[19:19]`, mask `0x00080000`
    pub const SRAM9_OFFSET: u32 = 19;
    pub const SRAM9_MASK: u32 = 0x00080000;
    pub const SRAM9: u32 = 1 << 19;

    /// **SRAM8**
    /// Bits `[18:18]`, mask `0x00040000`
    pub const SRAM8_OFFSET: u32 = 18;
    pub const SRAM8_MASK: u32 = 0x00040000;
    pub const SRAM8: u32 = 1 << 18;

    /// **SRAM7**
    /// Bits `[17:17]`, mask `0x00020000`
    pub const SRAM7_OFFSET: u32 = 17;
    pub const SRAM7_MASK: u32 = 0x00020000;
    pub const SRAM7: u32 = 1 << 17;

    /// **SRAM6**
    /// Bits `[16:16]`, mask `0x00010000`
    pub const SRAM6_OFFSET: u32 = 16;
    pub const SRAM6_MASK: u32 = 0x00010000;
    pub const SRAM6: u32 = 1 << 16;

    /// **SRAM5**
    /// Bits `[15:15]`, mask `0x00008000`
    pub const SRAM5_OFFSET: u32 = 15;
    pub const SRAM5_MASK: u32 = 0x00008000;
    pub const SRAM5: u32 = 1 << 15;

    /// **SRAM4**
    /// Bits `[14:14]`, mask `0x00004000`
    pub const SRAM4_OFFSET: u32 = 14;
    pub const SRAM4_MASK: u32 = 0x00004000;
    pub const SRAM4: u32 = 1 << 14;

    /// **SRAM3**
    /// Bits `[13:13]`, mask `0x00002000`
    pub const SRAM3_OFFSET: u32 = 13;
    pub const SRAM3_MASK: u32 = 0x00002000;
    pub const SRAM3: u32 = 1 << 13;

    /// **SRAM2**
    /// Bits `[12:12]`, mask `0x00001000`
    pub const SRAM2_OFFSET: u32 = 12;
    pub const SRAM2_MASK: u32 = 0x00001000;
    pub const SRAM2: u32 = 1 << 12;

    /// **SRAM1**
    /// Bits `[11:11]`, mask `0x00000800`
    pub const SRAM1_OFFSET: u32 = 11;
    pub const SRAM1_MASK: u32 = 0x00000800;
    pub const SRAM1: u32 = 1 << 11;

    /// **SRAM0**
    /// Bits `[10:10]`, mask `0x00000400`
    pub const SRAM0_OFFSET: u32 = 10;
    pub const SRAM0_MASK: u32 = 0x00000400;
    pub const SRAM0: u32 = 1 << 10;

    /// **BOOTRAM**
    /// Bits `[9:9]`, mask `0x00000200`
    pub const BOOTRAM_OFFSET: u32 = 9;
    pub const BOOTRAM_MASK: u32 = 0x00000200;
    pub const BOOTRAM: u32 = 1 << 9;

    /// **ROM**
    /// Bits `[8:8]`, mask `0x00000100`
    pub const ROM_OFFSET: u32 = 8;
    pub const ROM_MASK: u32 = 0x00000100;
    pub const ROM: u32 = 1 << 8;

    /// **BUSFABRIC**
    /// Bits `[7:7]`, mask `0x00000080`
    pub const BUSFABRIC_OFFSET: u32 = 7;
    pub const BUSFABRIC_MASK: u32 = 0x00000080;
    pub const BUSFABRIC: u32 = 1 << 7;

    /// **PSM_READY**
    /// Bits `[6:6]`, mask `0x00000040`
    pub const PSM_READY_OFFSET: u32 = 6;
    pub const PSM_READY_MASK: u32 = 0x00000040;
    pub const PSM_READY: u32 = 1 << 6;

    /// **CLOCKS**
    /// Bits `[5:5]`, mask `0x00000020`
    pub const CLOCKS_OFFSET: u32 = 5;
    pub const CLOCKS_MASK: u32 = 0x00000020;
    pub const CLOCKS: u32 = 1 << 5;

    /// **RESETS**
    /// Bits `[4:4]`, mask `0x00000010`
    pub const RESETS_OFFSET: u32 = 4;
    pub const RESETS_MASK: u32 = 0x00000010;
    pub const RESETS: u32 = 1 << 4;

    /// **XOSC**
    /// Bits `[3:3]`, mask `0x00000008`
    pub const XOSC_OFFSET: u32 = 3;
    pub const XOSC_MASK: u32 = 0x00000008;
    pub const XOSC: u32 = 1 << 3;

    /// **ROSC**
    /// Bits `[2:2]`, mask `0x00000004`
    pub const ROSC_OFFSET: u32 = 2;
    pub const ROSC_MASK: u32 = 0x00000004;
    pub const ROSC: u32 = 1 << 2;

    /// **OTP**
    /// Bits `[1:1]`, mask `0x00000002`
    pub const OTP_OFFSET: u32 = 1;
    pub const OTP_MASK: u32 = 0x00000002;
    pub const OTP: u32 = 1 << 1;

    /// **PROC_COLD**
    /// Bits `[0:0]`, mask `0x00000001`
    pub const PROC_COLD_OFFSET: u32 = 0;
    pub const PROC_COLD_MASK: u32 = 0x00000001;
    pub const PROC_COLD: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **PROC1** `[24]` (RW)
    #[inline(always)]
    pub fn proc1(&self) -> bool {
        (self.value.read_bits() & Self::PROC1_MASK) != 0
    }

    /// Set PROC1 bit atomically (RW)
    #[inline(always)]
    pub fn set_proc1(&self) {
        self.set_bits(Self::PROC1_MASK);
    }

    /// Clear PROC1 bit atomically (RW)
    #[inline(always)]
    pub fn clr_proc1(&self) {
        self.clr_bits(Self::PROC1_MASK);
    }

    /// **PROC0** `[23]` (RW)
    #[inline(always)]
    pub fn proc0(&self) -> bool {
        (self.value.read_bits() & Self::PROC0_MASK) != 0
    }

    /// Set PROC0 bit atomically (RW)
    #[inline(always)]
    pub fn set_proc0(&self) {
        self.set_bits(Self::PROC0_MASK);
    }

    /// Clear PROC0 bit atomically (RW)
    #[inline(always)]
    pub fn clr_proc0(&self) {
        self.clr_bits(Self::PROC0_MASK);
    }

    /// **ACCESSCTRL** `[22]` (RW)
    #[inline(always)]
    pub fn accessctrl(&self) -> bool {
        (self.value.read_bits() & Self::ACCESSCTRL_MASK) != 0
    }

    /// Set ACCESSCTRL bit atomically (RW)
    #[inline(always)]
    pub fn set_accessctrl(&self) {
        self.set_bits(Self::ACCESSCTRL_MASK);
    }

    /// Clear ACCESSCTRL bit atomically (RW)
    #[inline(always)]
    pub fn clr_accessctrl(&self) {
        self.clr_bits(Self::ACCESSCTRL_MASK);
    }

    /// **SIO** `[21]` (RW)
    #[inline(always)]
    pub fn sio(&self) -> bool {
        (self.value.read_bits() & Self::SIO_MASK) != 0
    }

    /// Set SIO bit atomically (RW)
    #[inline(always)]
    pub fn set_sio(&self) {
        self.set_bits(Self::SIO_MASK);
    }

    /// Clear SIO bit atomically (RW)
    #[inline(always)]
    pub fn clr_sio(&self) {
        self.clr_bits(Self::SIO_MASK);
    }

    /// **XIP** `[20]` (RW)
    #[inline(always)]
    pub fn xip(&self) -> bool {
        (self.value.read_bits() & Self::XIP_MASK) != 0
    }

    /// Set XIP bit atomically (RW)
    #[inline(always)]
    pub fn set_xip(&self) {
        self.set_bits(Self::XIP_MASK);
    }

    /// Clear XIP bit atomically (RW)
    #[inline(always)]
    pub fn clr_xip(&self) {
        self.clr_bits(Self::XIP_MASK);
    }

    /// **SRAM9** `[19]` (RW)
    #[inline(always)]
    pub fn sram9(&self) -> bool {
        (self.value.read_bits() & Self::SRAM9_MASK) != 0
    }

    /// Set SRAM9 bit atomically (RW)
    #[inline(always)]
    pub fn set_sram9(&self) {
        self.set_bits(Self::SRAM9_MASK);
    }

    /// Clear SRAM9 bit atomically (RW)
    #[inline(always)]
    pub fn clr_sram9(&self) {
        self.clr_bits(Self::SRAM9_MASK);
    }

    /// **SRAM8** `[18]` (RW)
    #[inline(always)]
    pub fn sram8(&self) -> bool {
        (self.value.read_bits() & Self::SRAM8_MASK) != 0
    }

    /// Set SRAM8 bit atomically (RW)
    #[inline(always)]
    pub fn set_sram8(&self) {
        self.set_bits(Self::SRAM8_MASK);
    }

    /// Clear SRAM8 bit atomically (RW)
    #[inline(always)]
    pub fn clr_sram8(&self) {
        self.clr_bits(Self::SRAM8_MASK);
    }

    /// **SRAM7** `[17]` (RW)
    #[inline(always)]
    pub fn sram7(&self) -> bool {
        (self.value.read_bits() & Self::SRAM7_MASK) != 0
    }

    /// Set SRAM7 bit atomically (RW)
    #[inline(always)]
    pub fn set_sram7(&self) {
        self.set_bits(Self::SRAM7_MASK);
    }

    /// Clear SRAM7 bit atomically (RW)
    #[inline(always)]
    pub fn clr_sram7(&self) {
        self.clr_bits(Self::SRAM7_MASK);
    }

    /// **SRAM6** `[16]` (RW)
    #[inline(always)]
    pub fn sram6(&self) -> bool {
        (self.value.read_bits() & Self::SRAM6_MASK) != 0
    }

    /// Set SRAM6 bit atomically (RW)
    #[inline(always)]
    pub fn set_sram6(&self) {
        self.set_bits(Self::SRAM6_MASK);
    }

    /// Clear SRAM6 bit atomically (RW)
    #[inline(always)]
    pub fn clr_sram6(&self) {
        self.clr_bits(Self::SRAM6_MASK);
    }

    /// **SRAM5** `[15]` (RW)
    #[inline(always)]
    pub fn sram5(&self) -> bool {
        (self.value.read_bits() & Self::SRAM5_MASK) != 0
    }

    /// Set SRAM5 bit atomically (RW)
    #[inline(always)]
    pub fn set_sram5(&self) {
        self.set_bits(Self::SRAM5_MASK);
    }

    /// Clear SRAM5 bit atomically (RW)
    #[inline(always)]
    pub fn clr_sram5(&self) {
        self.clr_bits(Self::SRAM5_MASK);
    }

    /// **SRAM4** `[14]` (RW)
    #[inline(always)]
    pub fn sram4(&self) -> bool {
        (self.value.read_bits() & Self::SRAM4_MASK) != 0
    }

    /// Set SRAM4 bit atomically (RW)
    #[inline(always)]
    pub fn set_sram4(&self) {
        self.set_bits(Self::SRAM4_MASK);
    }

    /// Clear SRAM4 bit atomically (RW)
    #[inline(always)]
    pub fn clr_sram4(&self) {
        self.clr_bits(Self::SRAM4_MASK);
    }

    /// **SRAM3** `[13]` (RW)
    #[inline(always)]
    pub fn sram3(&self) -> bool {
        (self.value.read_bits() & Self::SRAM3_MASK) != 0
    }

    /// Set SRAM3 bit atomically (RW)
    #[inline(always)]
    pub fn set_sram3(&self) {
        self.set_bits(Self::SRAM3_MASK);
    }

    /// Clear SRAM3 bit atomically (RW)
    #[inline(always)]
    pub fn clr_sram3(&self) {
        self.clr_bits(Self::SRAM3_MASK);
    }

    /// **SRAM2** `[12]` (RW)
    #[inline(always)]
    pub fn sram2(&self) -> bool {
        (self.value.read_bits() & Self::SRAM2_MASK) != 0
    }

    /// Set SRAM2 bit atomically (RW)
    #[inline(always)]
    pub fn set_sram2(&self) {
        self.set_bits(Self::SRAM2_MASK);
    }

    /// Clear SRAM2 bit atomically (RW)
    #[inline(always)]
    pub fn clr_sram2(&self) {
        self.clr_bits(Self::SRAM2_MASK);
    }

    /// **SRAM1** `[11]` (RW)
    #[inline(always)]
    pub fn sram1(&self) -> bool {
        (self.value.read_bits() & Self::SRAM1_MASK) != 0
    }

    /// Set SRAM1 bit atomically (RW)
    #[inline(always)]
    pub fn set_sram1(&self) {
        self.set_bits(Self::SRAM1_MASK);
    }

    /// Clear SRAM1 bit atomically (RW)
    #[inline(always)]
    pub fn clr_sram1(&self) {
        self.clr_bits(Self::SRAM1_MASK);
    }

    /// **SRAM0** `[10]` (RW)
    #[inline(always)]
    pub fn sram0(&self) -> bool {
        (self.value.read_bits() & Self::SRAM0_MASK) != 0
    }

    /// Set SRAM0 bit atomically (RW)
    #[inline(always)]
    pub fn set_sram0(&self) {
        self.set_bits(Self::SRAM0_MASK);
    }

    /// Clear SRAM0 bit atomically (RW)
    #[inline(always)]
    pub fn clr_sram0(&self) {
        self.clr_bits(Self::SRAM0_MASK);
    }

    /// **BOOTRAM** `[9]` (RW)
    #[inline(always)]
    pub fn bootram(&self) -> bool {
        (self.value.read_bits() & Self::BOOTRAM_MASK) != 0
    }

    /// Set BOOTRAM bit atomically (RW)
    #[inline(always)]
    pub fn set_bootram(&self) {
        self.set_bits(Self::BOOTRAM_MASK);
    }

    /// Clear BOOTRAM bit atomically (RW)
    #[inline(always)]
    pub fn clr_bootram(&self) {
        self.clr_bits(Self::BOOTRAM_MASK);
    }

    /// **ROM** `[8]` (RW)
    #[inline(always)]
    pub fn rom(&self) -> bool {
        (self.value.read_bits() & Self::ROM_MASK) != 0
    }

    /// Set ROM bit atomically (RW)
    #[inline(always)]
    pub fn set_rom(&self) {
        self.set_bits(Self::ROM_MASK);
    }

    /// Clear ROM bit atomically (RW)
    #[inline(always)]
    pub fn clr_rom(&self) {
        self.clr_bits(Self::ROM_MASK);
    }

    /// **BUSFABRIC** `[7]` (RW)
    #[inline(always)]
    pub fn busfabric(&self) -> bool {
        (self.value.read_bits() & Self::BUSFABRIC_MASK) != 0
    }

    /// Set BUSFABRIC bit atomically (RW)
    #[inline(always)]
    pub fn set_busfabric(&self) {
        self.set_bits(Self::BUSFABRIC_MASK);
    }

    /// Clear BUSFABRIC bit atomically (RW)
    #[inline(always)]
    pub fn clr_busfabric(&self) {
        self.clr_bits(Self::BUSFABRIC_MASK);
    }

    /// **PSM_READY** `[6]` (RW)
    #[inline(always)]
    pub fn psm_ready(&self) -> bool {
        (self.value.read_bits() & Self::PSM_READY_MASK) != 0
    }

    /// Set PSM_READY bit atomically (RW)
    #[inline(always)]
    pub fn set_psm_ready(&self) {
        self.set_bits(Self::PSM_READY_MASK);
    }

    /// Clear PSM_READY bit atomically (RW)
    #[inline(always)]
    pub fn clr_psm_ready(&self) {
        self.clr_bits(Self::PSM_READY_MASK);
    }

    /// **CLOCKS** `[5]` (RW)
    #[inline(always)]
    pub fn clocks(&self) -> bool {
        (self.value.read_bits() & Self::CLOCKS_MASK) != 0
    }

    /// Set CLOCKS bit atomically (RW)
    #[inline(always)]
    pub fn set_clocks(&self) {
        self.set_bits(Self::CLOCKS_MASK);
    }

    /// Clear CLOCKS bit atomically (RW)
    #[inline(always)]
    pub fn clr_clocks(&self) {
        self.clr_bits(Self::CLOCKS_MASK);
    }

    /// **RESETS** `[4]` (RW)
    #[inline(always)]
    pub fn resets(&self) -> bool {
        (self.value.read_bits() & Self::RESETS_MASK) != 0
    }

    /// Set RESETS bit atomically (RW)
    #[inline(always)]
    pub fn set_resets(&self) {
        self.set_bits(Self::RESETS_MASK);
    }

    /// Clear RESETS bit atomically (RW)
    #[inline(always)]
    pub fn clr_resets(&self) {
        self.clr_bits(Self::RESETS_MASK);
    }

    /// **XOSC** `[3]` (RW)
    #[inline(always)]
    pub fn xosc(&self) -> bool {
        (self.value.read_bits() & Self::XOSC_MASK) != 0
    }

    /// Set XOSC bit atomically (RW)
    #[inline(always)]
    pub fn set_xosc(&self) {
        self.set_bits(Self::XOSC_MASK);
    }

    /// Clear XOSC bit atomically (RW)
    #[inline(always)]
    pub fn clr_xosc(&self) {
        self.clr_bits(Self::XOSC_MASK);
    }

    /// **ROSC** `[2]` (RW)
    #[inline(always)]
    pub fn rosc(&self) -> bool {
        (self.value.read_bits() & Self::ROSC_MASK) != 0
    }

    /// Set ROSC bit atomically (RW)
    #[inline(always)]
    pub fn set_rosc(&self) {
        self.set_bits(Self::ROSC_MASK);
    }

    /// Clear ROSC bit atomically (RW)
    #[inline(always)]
    pub fn clr_rosc(&self) {
        self.clr_bits(Self::ROSC_MASK);
    }

    /// **OTP** `[1]` (RW)
    #[inline(always)]
    pub fn otp(&self) -> bool {
        (self.value.read_bits() & Self::OTP_MASK) != 0
    }

    /// Set OTP bit atomically (RW)
    #[inline(always)]
    pub fn set_otp(&self) {
        self.set_bits(Self::OTP_MASK);
    }

    /// Clear OTP bit atomically (RW)
    #[inline(always)]
    pub fn clr_otp(&self) {
        self.clr_bits(Self::OTP_MASK);
    }

    /// **PROC_COLD** `[0]` (RW)
    #[inline(always)]
    pub fn proc_cold(&self) -> bool {
        (self.value.read_bits() & Self::PROC_COLD_MASK) != 0
    }

    /// Set PROC_COLD bit atomically (RW)
    #[inline(always)]
    pub fn set_proc_cold(&self) {
        self.set_bits(Self::PROC_COLD_MASK);
    }

    /// Clear PROC_COLD bit atomically (RW)
    #[inline(always)]
    pub fn clr_proc_cold(&self) {
        self.clr_bits(Self::PROC_COLD_MASK);
    }

}

/// WDSEL value type for batch read/modify/write operations
///
/// Use `wdsel.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `wdsel.write_val(val)`.
#[derive(Clone, Copy)]
pub struct WdselVal {
    bits: u32,
}

impl WdselVal {
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

    /// Read **PROC1** `[24:24]`
    #[inline(always)]
    pub const fn proc1(&self) -> bool {
        (self.bits & Wdsel::PROC1_MASK) != 0
    }

    /// Read **PROC0** `[23:23]`
    #[inline(always)]
    pub const fn proc0(&self) -> bool {
        (self.bits & Wdsel::PROC0_MASK) != 0
    }

    /// Read **ACCESSCTRL** `[22:22]`
    #[inline(always)]
    pub const fn accessctrl(&self) -> bool {
        (self.bits & Wdsel::ACCESSCTRL_MASK) != 0
    }

    /// Read **SIO** `[21:21]`
    #[inline(always)]
    pub const fn sio(&self) -> bool {
        (self.bits & Wdsel::SIO_MASK) != 0
    }

    /// Read **XIP** `[20:20]`
    #[inline(always)]
    pub const fn xip(&self) -> bool {
        (self.bits & Wdsel::XIP_MASK) != 0
    }

    /// Read **SRAM9** `[19:19]`
    #[inline(always)]
    pub const fn sram9(&self) -> bool {
        (self.bits & Wdsel::SRAM9_MASK) != 0
    }

    /// Read **SRAM8** `[18:18]`
    #[inline(always)]
    pub const fn sram8(&self) -> bool {
        (self.bits & Wdsel::SRAM8_MASK) != 0
    }

    /// Read **SRAM7** `[17:17]`
    #[inline(always)]
    pub const fn sram7(&self) -> bool {
        (self.bits & Wdsel::SRAM7_MASK) != 0
    }

    /// Read **SRAM6** `[16:16]`
    #[inline(always)]
    pub const fn sram6(&self) -> bool {
        (self.bits & Wdsel::SRAM6_MASK) != 0
    }

    /// Read **SRAM5** `[15:15]`
    #[inline(always)]
    pub const fn sram5(&self) -> bool {
        (self.bits & Wdsel::SRAM5_MASK) != 0
    }

    /// Read **SRAM4** `[14:14]`
    #[inline(always)]
    pub const fn sram4(&self) -> bool {
        (self.bits & Wdsel::SRAM4_MASK) != 0
    }

    /// Read **SRAM3** `[13:13]`
    #[inline(always)]
    pub const fn sram3(&self) -> bool {
        (self.bits & Wdsel::SRAM3_MASK) != 0
    }

    /// Read **SRAM2** `[12:12]`
    #[inline(always)]
    pub const fn sram2(&self) -> bool {
        (self.bits & Wdsel::SRAM2_MASK) != 0
    }

    /// Read **SRAM1** `[11:11]`
    #[inline(always)]
    pub const fn sram1(&self) -> bool {
        (self.bits & Wdsel::SRAM1_MASK) != 0
    }

    /// Read **SRAM0** `[10:10]`
    #[inline(always)]
    pub const fn sram0(&self) -> bool {
        (self.bits & Wdsel::SRAM0_MASK) != 0
    }

    /// Read **BOOTRAM** `[9:9]`
    #[inline(always)]
    pub const fn bootram(&self) -> bool {
        (self.bits & Wdsel::BOOTRAM_MASK) != 0
    }

    /// Read **ROM** `[8:8]`
    #[inline(always)]
    pub const fn rom(&self) -> bool {
        (self.bits & Wdsel::ROM_MASK) != 0
    }

    /// Read **BUSFABRIC** `[7:7]`
    #[inline(always)]
    pub const fn busfabric(&self) -> bool {
        (self.bits & Wdsel::BUSFABRIC_MASK) != 0
    }

    /// Read **PSM_READY** `[6:6]`
    #[inline(always)]
    pub const fn psm_ready(&self) -> bool {
        (self.bits & Wdsel::PSM_READY_MASK) != 0
    }

    /// Read **CLOCKS** `[5:5]`
    #[inline(always)]
    pub const fn clocks(&self) -> bool {
        (self.bits & Wdsel::CLOCKS_MASK) != 0
    }

    /// Read **RESETS** `[4:4]`
    #[inline(always)]
    pub const fn resets(&self) -> bool {
        (self.bits & Wdsel::RESETS_MASK) != 0
    }

    /// Read **XOSC** `[3:3]`
    #[inline(always)]
    pub const fn xosc(&self) -> bool {
        (self.bits & Wdsel::XOSC_MASK) != 0
    }

    /// Read **ROSC** `[2:2]`
    #[inline(always)]
    pub const fn rosc(&self) -> bool {
        (self.bits & Wdsel::ROSC_MASK) != 0
    }

    /// Read **OTP** `[1:1]`
    #[inline(always)]
    pub const fn otp(&self) -> bool {
        (self.bits & Wdsel::OTP_MASK) != 0
    }

    /// Read **PROC_COLD** `[0:0]`
    #[inline(always)]
    pub const fn proc_cold(&self) -> bool {
        (self.bits & Wdsel::PROC_COLD_MASK) != 0
    }

    /// Set **PROC1** `[24:24]`
    #[inline(always)]
    pub const fn with_proc1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Wdsel::PROC1_MASK
            } else {
                self.bits & !Wdsel::PROC1_MASK
            }
        }
    }

    /// Set **PROC0** `[23:23]`
    #[inline(always)]
    pub const fn with_proc0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Wdsel::PROC0_MASK
            } else {
                self.bits & !Wdsel::PROC0_MASK
            }
        }
    }

    /// Set **ACCESSCTRL** `[22:22]`
    #[inline(always)]
    pub const fn with_accessctrl(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Wdsel::ACCESSCTRL_MASK
            } else {
                self.bits & !Wdsel::ACCESSCTRL_MASK
            }
        }
    }

    /// Set **SIO** `[21:21]`
    #[inline(always)]
    pub const fn with_sio(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Wdsel::SIO_MASK
            } else {
                self.bits & !Wdsel::SIO_MASK
            }
        }
    }

    /// Set **XIP** `[20:20]`
    #[inline(always)]
    pub const fn with_xip(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Wdsel::XIP_MASK
            } else {
                self.bits & !Wdsel::XIP_MASK
            }
        }
    }

    /// Set **SRAM9** `[19:19]`
    #[inline(always)]
    pub const fn with_sram9(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Wdsel::SRAM9_MASK
            } else {
                self.bits & !Wdsel::SRAM9_MASK
            }
        }
    }

    /// Set **SRAM8** `[18:18]`
    #[inline(always)]
    pub const fn with_sram8(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Wdsel::SRAM8_MASK
            } else {
                self.bits & !Wdsel::SRAM8_MASK
            }
        }
    }

    /// Set **SRAM7** `[17:17]`
    #[inline(always)]
    pub const fn with_sram7(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Wdsel::SRAM7_MASK
            } else {
                self.bits & !Wdsel::SRAM7_MASK
            }
        }
    }

    /// Set **SRAM6** `[16:16]`
    #[inline(always)]
    pub const fn with_sram6(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Wdsel::SRAM6_MASK
            } else {
                self.bits & !Wdsel::SRAM6_MASK
            }
        }
    }

    /// Set **SRAM5** `[15:15]`
    #[inline(always)]
    pub const fn with_sram5(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Wdsel::SRAM5_MASK
            } else {
                self.bits & !Wdsel::SRAM5_MASK
            }
        }
    }

    /// Set **SRAM4** `[14:14]`
    #[inline(always)]
    pub const fn with_sram4(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Wdsel::SRAM4_MASK
            } else {
                self.bits & !Wdsel::SRAM4_MASK
            }
        }
    }

    /// Set **SRAM3** `[13:13]`
    #[inline(always)]
    pub const fn with_sram3(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Wdsel::SRAM3_MASK
            } else {
                self.bits & !Wdsel::SRAM3_MASK
            }
        }
    }

    /// Set **SRAM2** `[12:12]`
    #[inline(always)]
    pub const fn with_sram2(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Wdsel::SRAM2_MASK
            } else {
                self.bits & !Wdsel::SRAM2_MASK
            }
        }
    }

    /// Set **SRAM1** `[11:11]`
    #[inline(always)]
    pub const fn with_sram1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Wdsel::SRAM1_MASK
            } else {
                self.bits & !Wdsel::SRAM1_MASK
            }
        }
    }

    /// Set **SRAM0** `[10:10]`
    #[inline(always)]
    pub const fn with_sram0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Wdsel::SRAM0_MASK
            } else {
                self.bits & !Wdsel::SRAM0_MASK
            }
        }
    }

    /// Set **BOOTRAM** `[9:9]`
    #[inline(always)]
    pub const fn with_bootram(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Wdsel::BOOTRAM_MASK
            } else {
                self.bits & !Wdsel::BOOTRAM_MASK
            }
        }
    }

    /// Set **ROM** `[8:8]`
    #[inline(always)]
    pub const fn with_rom(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Wdsel::ROM_MASK
            } else {
                self.bits & !Wdsel::ROM_MASK
            }
        }
    }

    /// Set **BUSFABRIC** `[7:7]`
    #[inline(always)]
    pub const fn with_busfabric(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Wdsel::BUSFABRIC_MASK
            } else {
                self.bits & !Wdsel::BUSFABRIC_MASK
            }
        }
    }

    /// Set **PSM_READY** `[6:6]`
    #[inline(always)]
    pub const fn with_psm_ready(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Wdsel::PSM_READY_MASK
            } else {
                self.bits & !Wdsel::PSM_READY_MASK
            }
        }
    }

    /// Set **CLOCKS** `[5:5]`
    #[inline(always)]
    pub const fn with_clocks(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Wdsel::CLOCKS_MASK
            } else {
                self.bits & !Wdsel::CLOCKS_MASK
            }
        }
    }

    /// Set **RESETS** `[4:4]`
    #[inline(always)]
    pub const fn with_resets(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Wdsel::RESETS_MASK
            } else {
                self.bits & !Wdsel::RESETS_MASK
            }
        }
    }

    /// Set **XOSC** `[3:3]`
    #[inline(always)]
    pub const fn with_xosc(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Wdsel::XOSC_MASK
            } else {
                self.bits & !Wdsel::XOSC_MASK
            }
        }
    }

    /// Set **ROSC** `[2:2]`
    #[inline(always)]
    pub const fn with_rosc(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Wdsel::ROSC_MASK
            } else {
                self.bits & !Wdsel::ROSC_MASK
            }
        }
    }

    /// Set **OTP** `[1:1]`
    #[inline(always)]
    pub const fn with_otp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Wdsel::OTP_MASK
            } else {
                self.bits & !Wdsel::OTP_MASK
            }
        }
    }

    /// Set **PROC_COLD** `[0:0]`
    #[inline(always)]
    pub const fn with_proc_cold(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Wdsel::PROC_COLD_MASK
            } else {
                self.bits & !Wdsel::PROC_COLD_MASK
            }
        }
    }

}

impl Default for WdselVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Wdsel {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> WdselVal {
        WdselVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: WdselVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(WdselVal) -> WdselVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// DONE Register
///
/// Is the subsystem ready?
///
/// Address offset: 0x000c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **PROC1** `[24]` (RO)
///
/// - **PROC0** `[23]` (RO)
///
/// - **ACCESSCTRL** `[22]` (RO)
///
/// - **SIO** `[21]` (RO)
///
/// - **XIP** `[20]` (RO)
///
/// - **SRAM9** `[19]` (RO)
///
/// - **SRAM8** `[18]` (RO)
///
/// - **SRAM7** `[17]` (RO)
///
/// - **SRAM6** `[16]` (RO)
///
/// - **SRAM5** `[15]` (RO)
///
/// - **SRAM4** `[14]` (RO)
///
/// - **SRAM3** `[13]` (RO)
///
/// - **SRAM2** `[12]` (RO)
///
/// - **SRAM1** `[11]` (RO)
///
/// - **SRAM0** `[10]` (RO)
///
/// - **BOOTRAM** `[9]` (RO)
///
/// - **ROM** `[8]` (RO)
///
/// - **BUSFABRIC** `[7]` (RO)
///
/// - **PSM_READY** `[6]` (RO)
///
/// - **CLOCKS** `[5]` (RO)
///
/// - **RESETS** `[4]` (RO)
///
/// - **XOSC** `[3]` (RO)
///
/// - **ROSC** `[2]` (RO)
///
/// - **OTP** `[1]` (RO)
///
/// - **PROC_COLD** `[0]` (RO)
#[repr(transparent)]
pub struct Done {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Done {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Done {
    // ======== Field Constants ========

    /// **PROC1**
    /// Bits `[24:24]`, mask `0x01000000`
    pub const PROC1_OFFSET: u32 = 24;
    pub const PROC1_MASK: u32 = 0x01000000;
    pub const PROC1: u32 = 1 << 24;

    /// **PROC0**
    /// Bits `[23:23]`, mask `0x00800000`
    pub const PROC0_OFFSET: u32 = 23;
    pub const PROC0_MASK: u32 = 0x00800000;
    pub const PROC0: u32 = 1 << 23;

    /// **ACCESSCTRL**
    /// Bits `[22:22]`, mask `0x00400000`
    pub const ACCESSCTRL_OFFSET: u32 = 22;
    pub const ACCESSCTRL_MASK: u32 = 0x00400000;
    pub const ACCESSCTRL: u32 = 1 << 22;

    /// **SIO**
    /// Bits `[21:21]`, mask `0x00200000`
    pub const SIO_OFFSET: u32 = 21;
    pub const SIO_MASK: u32 = 0x00200000;
    pub const SIO: u32 = 1 << 21;

    /// **XIP**
    /// Bits `[20:20]`, mask `0x00100000`
    pub const XIP_OFFSET: u32 = 20;
    pub const XIP_MASK: u32 = 0x00100000;
    pub const XIP: u32 = 1 << 20;

    /// **SRAM9**
    /// Bits `[19:19]`, mask `0x00080000`
    pub const SRAM9_OFFSET: u32 = 19;
    pub const SRAM9_MASK: u32 = 0x00080000;
    pub const SRAM9: u32 = 1 << 19;

    /// **SRAM8**
    /// Bits `[18:18]`, mask `0x00040000`
    pub const SRAM8_OFFSET: u32 = 18;
    pub const SRAM8_MASK: u32 = 0x00040000;
    pub const SRAM8: u32 = 1 << 18;

    /// **SRAM7**
    /// Bits `[17:17]`, mask `0x00020000`
    pub const SRAM7_OFFSET: u32 = 17;
    pub const SRAM7_MASK: u32 = 0x00020000;
    pub const SRAM7: u32 = 1 << 17;

    /// **SRAM6**
    /// Bits `[16:16]`, mask `0x00010000`
    pub const SRAM6_OFFSET: u32 = 16;
    pub const SRAM6_MASK: u32 = 0x00010000;
    pub const SRAM6: u32 = 1 << 16;

    /// **SRAM5**
    /// Bits `[15:15]`, mask `0x00008000`
    pub const SRAM5_OFFSET: u32 = 15;
    pub const SRAM5_MASK: u32 = 0x00008000;
    pub const SRAM5: u32 = 1 << 15;

    /// **SRAM4**
    /// Bits `[14:14]`, mask `0x00004000`
    pub const SRAM4_OFFSET: u32 = 14;
    pub const SRAM4_MASK: u32 = 0x00004000;
    pub const SRAM4: u32 = 1 << 14;

    /// **SRAM3**
    /// Bits `[13:13]`, mask `0x00002000`
    pub const SRAM3_OFFSET: u32 = 13;
    pub const SRAM3_MASK: u32 = 0x00002000;
    pub const SRAM3: u32 = 1 << 13;

    /// **SRAM2**
    /// Bits `[12:12]`, mask `0x00001000`
    pub const SRAM2_OFFSET: u32 = 12;
    pub const SRAM2_MASK: u32 = 0x00001000;
    pub const SRAM2: u32 = 1 << 12;

    /// **SRAM1**
    /// Bits `[11:11]`, mask `0x00000800`
    pub const SRAM1_OFFSET: u32 = 11;
    pub const SRAM1_MASK: u32 = 0x00000800;
    pub const SRAM1: u32 = 1 << 11;

    /// **SRAM0**
    /// Bits `[10:10]`, mask `0x00000400`
    pub const SRAM0_OFFSET: u32 = 10;
    pub const SRAM0_MASK: u32 = 0x00000400;
    pub const SRAM0: u32 = 1 << 10;

    /// **BOOTRAM**
    /// Bits `[9:9]`, mask `0x00000200`
    pub const BOOTRAM_OFFSET: u32 = 9;
    pub const BOOTRAM_MASK: u32 = 0x00000200;
    pub const BOOTRAM: u32 = 1 << 9;

    /// **ROM**
    /// Bits `[8:8]`, mask `0x00000100`
    pub const ROM_OFFSET: u32 = 8;
    pub const ROM_MASK: u32 = 0x00000100;
    pub const ROM: u32 = 1 << 8;

    /// **BUSFABRIC**
    /// Bits `[7:7]`, mask `0x00000080`
    pub const BUSFABRIC_OFFSET: u32 = 7;
    pub const BUSFABRIC_MASK: u32 = 0x00000080;
    pub const BUSFABRIC: u32 = 1 << 7;

    /// **PSM_READY**
    /// Bits `[6:6]`, mask `0x00000040`
    pub const PSM_READY_OFFSET: u32 = 6;
    pub const PSM_READY_MASK: u32 = 0x00000040;
    pub const PSM_READY: u32 = 1 << 6;

    /// **CLOCKS**
    /// Bits `[5:5]`, mask `0x00000020`
    pub const CLOCKS_OFFSET: u32 = 5;
    pub const CLOCKS_MASK: u32 = 0x00000020;
    pub const CLOCKS: u32 = 1 << 5;

    /// **RESETS**
    /// Bits `[4:4]`, mask `0x00000010`
    pub const RESETS_OFFSET: u32 = 4;
    pub const RESETS_MASK: u32 = 0x00000010;
    pub const RESETS: u32 = 1 << 4;

    /// **XOSC**
    /// Bits `[3:3]`, mask `0x00000008`
    pub const XOSC_OFFSET: u32 = 3;
    pub const XOSC_MASK: u32 = 0x00000008;
    pub const XOSC: u32 = 1 << 3;

    /// **ROSC**
    /// Bits `[2:2]`, mask `0x00000004`
    pub const ROSC_OFFSET: u32 = 2;
    pub const ROSC_MASK: u32 = 0x00000004;
    pub const ROSC: u32 = 1 << 2;

    /// **OTP**
    /// Bits `[1:1]`, mask `0x00000002`
    pub const OTP_OFFSET: u32 = 1;
    pub const OTP_MASK: u32 = 0x00000002;
    pub const OTP: u32 = 1 << 1;

    /// **PROC_COLD**
    /// Bits `[0:0]`, mask `0x00000001`
    pub const PROC_COLD_OFFSET: u32 = 0;
    pub const PROC_COLD_MASK: u32 = 0x00000001;
    pub const PROC_COLD: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **PROC1** `[24]` (RO)
    #[inline(always)]
    pub fn proc1(&self) -> bool {
        (self.value.read_bits() & Self::PROC1_MASK) != 0
    }

    /// **PROC0** `[23]` (RO)
    #[inline(always)]
    pub fn proc0(&self) -> bool {
        (self.value.read_bits() & Self::PROC0_MASK) != 0
    }

    /// **ACCESSCTRL** `[22]` (RO)
    #[inline(always)]
    pub fn accessctrl(&self) -> bool {
        (self.value.read_bits() & Self::ACCESSCTRL_MASK) != 0
    }

    /// **SIO** `[21]` (RO)
    #[inline(always)]
    pub fn sio(&self) -> bool {
        (self.value.read_bits() & Self::SIO_MASK) != 0
    }

    /// **XIP** `[20]` (RO)
    #[inline(always)]
    pub fn xip(&self) -> bool {
        (self.value.read_bits() & Self::XIP_MASK) != 0
    }

    /// **SRAM9** `[19]` (RO)
    #[inline(always)]
    pub fn sram9(&self) -> bool {
        (self.value.read_bits() & Self::SRAM9_MASK) != 0
    }

    /// **SRAM8** `[18]` (RO)
    #[inline(always)]
    pub fn sram8(&self) -> bool {
        (self.value.read_bits() & Self::SRAM8_MASK) != 0
    }

    /// **SRAM7** `[17]` (RO)
    #[inline(always)]
    pub fn sram7(&self) -> bool {
        (self.value.read_bits() & Self::SRAM7_MASK) != 0
    }

    /// **SRAM6** `[16]` (RO)
    #[inline(always)]
    pub fn sram6(&self) -> bool {
        (self.value.read_bits() & Self::SRAM6_MASK) != 0
    }

    /// **SRAM5** `[15]` (RO)
    #[inline(always)]
    pub fn sram5(&self) -> bool {
        (self.value.read_bits() & Self::SRAM5_MASK) != 0
    }

    /// **SRAM4** `[14]` (RO)
    #[inline(always)]
    pub fn sram4(&self) -> bool {
        (self.value.read_bits() & Self::SRAM4_MASK) != 0
    }

    /// **SRAM3** `[13]` (RO)
    #[inline(always)]
    pub fn sram3(&self) -> bool {
        (self.value.read_bits() & Self::SRAM3_MASK) != 0
    }

    /// **SRAM2** `[12]` (RO)
    #[inline(always)]
    pub fn sram2(&self) -> bool {
        (self.value.read_bits() & Self::SRAM2_MASK) != 0
    }

    /// **SRAM1** `[11]` (RO)
    #[inline(always)]
    pub fn sram1(&self) -> bool {
        (self.value.read_bits() & Self::SRAM1_MASK) != 0
    }

    /// **SRAM0** `[10]` (RO)
    #[inline(always)]
    pub fn sram0(&self) -> bool {
        (self.value.read_bits() & Self::SRAM0_MASK) != 0
    }

    /// **BOOTRAM** `[9]` (RO)
    #[inline(always)]
    pub fn bootram(&self) -> bool {
        (self.value.read_bits() & Self::BOOTRAM_MASK) != 0
    }

    /// **ROM** `[8]` (RO)
    #[inline(always)]
    pub fn rom(&self) -> bool {
        (self.value.read_bits() & Self::ROM_MASK) != 0
    }

    /// **BUSFABRIC** `[7]` (RO)
    #[inline(always)]
    pub fn busfabric(&self) -> bool {
        (self.value.read_bits() & Self::BUSFABRIC_MASK) != 0
    }

    /// **PSM_READY** `[6]` (RO)
    #[inline(always)]
    pub fn psm_ready(&self) -> bool {
        (self.value.read_bits() & Self::PSM_READY_MASK) != 0
    }

    /// **CLOCKS** `[5]` (RO)
    #[inline(always)]
    pub fn clocks(&self) -> bool {
        (self.value.read_bits() & Self::CLOCKS_MASK) != 0
    }

    /// **RESETS** `[4]` (RO)
    #[inline(always)]
    pub fn resets(&self) -> bool {
        (self.value.read_bits() & Self::RESETS_MASK) != 0
    }

    /// **XOSC** `[3]` (RO)
    #[inline(always)]
    pub fn xosc(&self) -> bool {
        (self.value.read_bits() & Self::XOSC_MASK) != 0
    }

    /// **ROSC** `[2]` (RO)
    #[inline(always)]
    pub fn rosc(&self) -> bool {
        (self.value.read_bits() & Self::ROSC_MASK) != 0
    }

    /// **OTP** `[1]` (RO)
    #[inline(always)]
    pub fn otp(&self) -> bool {
        (self.value.read_bits() & Self::OTP_MASK) != 0
    }

    /// **PROC_COLD** `[0]` (RO)
    #[inline(always)]
    pub fn proc_cold(&self) -> bool {
        (self.value.read_bits() & Self::PROC_COLD_MASK) != 0
    }

}

/// DONE value type for batch read/modify/write operations
///
/// Use `done.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `done.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DoneVal {
    bits: u32,
}

impl DoneVal {
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

    /// Read **PROC1** `[24:24]`
    #[inline(always)]
    pub const fn proc1(&self) -> bool {
        (self.bits & Done::PROC1_MASK) != 0
    }

    /// Read **PROC0** `[23:23]`
    #[inline(always)]
    pub const fn proc0(&self) -> bool {
        (self.bits & Done::PROC0_MASK) != 0
    }

    /// Read **ACCESSCTRL** `[22:22]`
    #[inline(always)]
    pub const fn accessctrl(&self) -> bool {
        (self.bits & Done::ACCESSCTRL_MASK) != 0
    }

    /// Read **SIO** `[21:21]`
    #[inline(always)]
    pub const fn sio(&self) -> bool {
        (self.bits & Done::SIO_MASK) != 0
    }

    /// Read **XIP** `[20:20]`
    #[inline(always)]
    pub const fn xip(&self) -> bool {
        (self.bits & Done::XIP_MASK) != 0
    }

    /// Read **SRAM9** `[19:19]`
    #[inline(always)]
    pub const fn sram9(&self) -> bool {
        (self.bits & Done::SRAM9_MASK) != 0
    }

    /// Read **SRAM8** `[18:18]`
    #[inline(always)]
    pub const fn sram8(&self) -> bool {
        (self.bits & Done::SRAM8_MASK) != 0
    }

    /// Read **SRAM7** `[17:17]`
    #[inline(always)]
    pub const fn sram7(&self) -> bool {
        (self.bits & Done::SRAM7_MASK) != 0
    }

    /// Read **SRAM6** `[16:16]`
    #[inline(always)]
    pub const fn sram6(&self) -> bool {
        (self.bits & Done::SRAM6_MASK) != 0
    }

    /// Read **SRAM5** `[15:15]`
    #[inline(always)]
    pub const fn sram5(&self) -> bool {
        (self.bits & Done::SRAM5_MASK) != 0
    }

    /// Read **SRAM4** `[14:14]`
    #[inline(always)]
    pub const fn sram4(&self) -> bool {
        (self.bits & Done::SRAM4_MASK) != 0
    }

    /// Read **SRAM3** `[13:13]`
    #[inline(always)]
    pub const fn sram3(&self) -> bool {
        (self.bits & Done::SRAM3_MASK) != 0
    }

    /// Read **SRAM2** `[12:12]`
    #[inline(always)]
    pub const fn sram2(&self) -> bool {
        (self.bits & Done::SRAM2_MASK) != 0
    }

    /// Read **SRAM1** `[11:11]`
    #[inline(always)]
    pub const fn sram1(&self) -> bool {
        (self.bits & Done::SRAM1_MASK) != 0
    }

    /// Read **SRAM0** `[10:10]`
    #[inline(always)]
    pub const fn sram0(&self) -> bool {
        (self.bits & Done::SRAM0_MASK) != 0
    }

    /// Read **BOOTRAM** `[9:9]`
    #[inline(always)]
    pub const fn bootram(&self) -> bool {
        (self.bits & Done::BOOTRAM_MASK) != 0
    }

    /// Read **ROM** `[8:8]`
    #[inline(always)]
    pub const fn rom(&self) -> bool {
        (self.bits & Done::ROM_MASK) != 0
    }

    /// Read **BUSFABRIC** `[7:7]`
    #[inline(always)]
    pub const fn busfabric(&self) -> bool {
        (self.bits & Done::BUSFABRIC_MASK) != 0
    }

    /// Read **PSM_READY** `[6:6]`
    #[inline(always)]
    pub const fn psm_ready(&self) -> bool {
        (self.bits & Done::PSM_READY_MASK) != 0
    }

    /// Read **CLOCKS** `[5:5]`
    #[inline(always)]
    pub const fn clocks(&self) -> bool {
        (self.bits & Done::CLOCKS_MASK) != 0
    }

    /// Read **RESETS** `[4:4]`
    #[inline(always)]
    pub const fn resets(&self) -> bool {
        (self.bits & Done::RESETS_MASK) != 0
    }

    /// Read **XOSC** `[3:3]`
    #[inline(always)]
    pub const fn xosc(&self) -> bool {
        (self.bits & Done::XOSC_MASK) != 0
    }

    /// Read **ROSC** `[2:2]`
    #[inline(always)]
    pub const fn rosc(&self) -> bool {
        (self.bits & Done::ROSC_MASK) != 0
    }

    /// Read **OTP** `[1:1]`
    #[inline(always)]
    pub const fn otp(&self) -> bool {
        (self.bits & Done::OTP_MASK) != 0
    }

    /// Read **PROC_COLD** `[0:0]`
    #[inline(always)]
    pub const fn proc_cold(&self) -> bool {
        (self.bits & Done::PROC_COLD_MASK) != 0
    }

}

impl Default for DoneVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Done {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DoneVal {
        DoneVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: DoneVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(DoneVal) -> DoneVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// Register block
#[repr(C)]
pub struct RegisterBlock {
    /// 0x0000
    /// Force block out of reset (i.e. power it on)
    pub frce_on: FrceOn,
    /// 0x0004
    /// Force into reset (i.e. power it off)
    pub frce_off: FrceOff,
    /// 0x0008
    /// Set to 1 if the watchdog should reset this
    pub wdsel: Wdsel,
    /// 0x000c
    /// Is the subsystem ready?
    pub done: Done,
}

