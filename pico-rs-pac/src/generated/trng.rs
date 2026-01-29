//! TRNG - ARM TrustZone RNG register block

#![allow(dead_code)]

pub const BASE: u32 = 0x400f0000;

/// RNG_IMR Register
///
/// Interrupt masking.
///
/// Address offset: 0x0100
/// Reset value: 0x0000000f
///
/// ## Fields
///
/// - **RESERVED** [31:4] (RO): RESERVED
/// - **VN_ERR_INT_MASK** [3] (RW): 1'b1-mask interrupt, no interrupt will be generated. See RNG_ISR for an exp...
/// - **CRNGT_ERR_INT_MASK** [2] (RW): 1'b1-mask interrupt, no interrupt will be generated. See RNG_ISR for an exp...
/// - **AUTOCORR_ERR_INT_MASK** [1] (RW): 1'b1-mask interrupt, no interrupt will be generated. See RNG_ISR for an exp...
/// - **EHR_VALID_INT_MASK** [0] (RW): 1'b1-mask interrupt, no interrupt will be generated. See RNG_ISR for an exp...
#[repr(transparent)]
pub struct RngImr {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for RngImr {}
unsafe impl Sync for RngImr {}

impl RngImr {
    // ======== Field Constants ========

    /// RESERVED - RESERVED
    /// Bits [31:4], mask 0xfffffff0
    pub const RESERVED_OFFSET: u32 = 4;
    pub const RESERVED_MASK: u32 = 0xfffffff0;

    /// VN_ERR_INT_MASK - 1'b1-mask interrupt, no interrupt will be generated. See RNG_ISR for an exp...
    /// Bits [3:3], mask 0x00000008
    pub const VN_ERR_INT_MASK_OFFSET: u32 = 3;
    pub const VN_ERR_INT_MASK_MASK: u32 = 0x00000008;
    pub const VN_ERR_INT_MASK: u32 = 1 << 3;

    /// CRNGT_ERR_INT_MASK - 1'b1-mask interrupt, no interrupt will be generated. See RNG_ISR for an exp...
    /// Bits [2:2], mask 0x00000004
    pub const CRNGT_ERR_INT_MASK_OFFSET: u32 = 2;
    pub const CRNGT_ERR_INT_MASK_MASK: u32 = 0x00000004;
    pub const CRNGT_ERR_INT_MASK: u32 = 1 << 2;

    /// AUTOCORR_ERR_INT_MASK - 1'b1-mask interrupt, no interrupt will be generated. See RNG_ISR for an exp...
    /// Bits [1:1], mask 0x00000002
    pub const AUTOCORR_ERR_INT_MASK_OFFSET: u32 = 1;
    pub const AUTOCORR_ERR_INT_MASK_MASK: u32 = 0x00000002;
    pub const AUTOCORR_ERR_INT_MASK: u32 = 1 << 1;

    /// EHR_VALID_INT_MASK - 1'b1-mask interrupt, no interrupt will be generated. See RNG_ISR for an exp...
    /// Bits [0:0], mask 0x00000001
    pub const EHR_VALID_INT_MASK_OFFSET: u32 = 0;
    pub const EHR_VALID_INT_MASK_MASK: u32 = 0x00000001;
    pub const EHR_VALID_INT_MASK: u32 = 1 << 0;

    // ======== Basic Access Methods ========

    /// Read the raw register value as u32
    #[inline(always)]
    pub fn read_bits(&self) -> u32 {
        unsafe { core::ptr::read_volatile(self.value.get()) }
    }

    /// Write a raw u32 value to the register
    #[inline(always)]
    pub fn write_bits(&self, value: u32) {
        unsafe { core::ptr::write_volatile(self.value.get(), value) }
    }

    /// Modify the raw register value
    #[inline(always)]
    pub fn modify_bits<F: FnOnce(u32) -> u32>(&self, f: F) {
        self.write_bits(f(self.read_bits()));
    }

    /// Atomic SET - sets bits that are 1 (hardware alias at +0x2000)
    #[inline(always)]
    pub fn set_bits(&self, bits: u32) {
        unsafe {
            let addr = (self.value.get() as usize + 0x2000) as *mut u32;
            core::ptr::write_volatile(addr, bits);
        }
    }

    /// Atomic CLR - clears bits that are 1 (hardware alias at +0x3000)
    #[inline(always)]
    pub fn clr_bits(&self, bits: u32) {
        unsafe {
            let addr = (self.value.get() as usize + 0x3000) as *mut u32;
            core::ptr::write_volatile(addr, bits);
        }
    }

    /// Atomic XOR - toggles bits that are 1 (hardware alias at +0x1000)
    #[inline(always)]
    pub fn xor_bits(&self, bits: u32) {
        unsafe {
            let addr = (self.value.get() as usize + 0x1000) as *mut u32;
            core::ptr::write_volatile(addr, bits);
        }
    }

    // ======== Field Accessors ========

    /// **RESERVED** [31:4]: RESERVED
    #[inline(always)]
    pub fn reserved(&self) -> u32 {
        (self.read_bits() & Self::RESERVED_MASK) >> Self::RESERVED_OFFSET
    }

    /// **VN_ERR_INT_MASK** [3:3]: 1'b1-mask interrupt, no interrupt will be generated. See RNG_ISR for an exp...
    #[inline(always)]
    pub fn vn_err_int_mask(&self) -> bool {
        (self.read_bits() & Self::VN_ERR_INT_MASK_MASK) != 0
    }

    /// Set VN_ERR_INT_MASK bit
    #[inline(always)]
    pub fn set_vn_err_int_mask(&self) {
        self.set_bits(Self::VN_ERR_INT_MASK_MASK);
    }

    /// Clear VN_ERR_INT_MASK bit
    #[inline(always)]
    pub fn clr_vn_err_int_mask(&self) {
        self.clr_bits(Self::VN_ERR_INT_MASK_MASK);
    }

    /// **CRNGT_ERR_INT_MASK** [2:2]: 1'b1-mask interrupt, no interrupt will be generated. See RNG_ISR for an exp...
    #[inline(always)]
    pub fn crngt_err_int_mask(&self) -> bool {
        (self.read_bits() & Self::CRNGT_ERR_INT_MASK_MASK) != 0
    }

    /// Set CRNGT_ERR_INT_MASK bit
    #[inline(always)]
    pub fn set_crngt_err_int_mask(&self) {
        self.set_bits(Self::CRNGT_ERR_INT_MASK_MASK);
    }

    /// Clear CRNGT_ERR_INT_MASK bit
    #[inline(always)]
    pub fn clr_crngt_err_int_mask(&self) {
        self.clr_bits(Self::CRNGT_ERR_INT_MASK_MASK);
    }

    /// **AUTOCORR_ERR_INT_MASK** [1:1]: 1'b1-mask interrupt, no interrupt will be generated. See RNG_ISR for an exp...
    #[inline(always)]
    pub fn autocorr_err_int_mask(&self) -> bool {
        (self.read_bits() & Self::AUTOCORR_ERR_INT_MASK_MASK) != 0
    }

    /// Set AUTOCORR_ERR_INT_MASK bit
    #[inline(always)]
    pub fn set_autocorr_err_int_mask(&self) {
        self.set_bits(Self::AUTOCORR_ERR_INT_MASK_MASK);
    }

    /// Clear AUTOCORR_ERR_INT_MASK bit
    #[inline(always)]
    pub fn clr_autocorr_err_int_mask(&self) {
        self.clr_bits(Self::AUTOCORR_ERR_INT_MASK_MASK);
    }

    /// **EHR_VALID_INT_MASK** [0:0]: 1'b1-mask interrupt, no interrupt will be generated. See RNG_ISR for an exp...
    #[inline(always)]
    pub fn ehr_valid_int_mask(&self) -> bool {
        (self.read_bits() & Self::EHR_VALID_INT_MASK_MASK) != 0
    }

    /// Set EHR_VALID_INT_MASK bit
    #[inline(always)]
    pub fn set_ehr_valid_int_mask(&self) {
        self.set_bits(Self::EHR_VALID_INT_MASK_MASK);
    }

    /// Clear EHR_VALID_INT_MASK bit
    #[inline(always)]
    pub fn clr_ehr_valid_int_mask(&self) {
        self.clr_bits(Self::EHR_VALID_INT_MASK_MASK);
    }

}

/// RNG_IMR value type for batch read/modify/write operations
///
/// Use `rngimr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rngimr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RngImrVal {
    bits: u32,
}

impl RngImrVal {
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
        Self { bits: 0x0000000f }
    }

    /// Read RESERVED [31:4]: RESERVED
    #[inline(always)]
    pub const fn reserved(&self) -> u32 {
        (self.bits & RngImr::RESERVED_MASK) >> RngImr::RESERVED_OFFSET
    }

    /// Read VN_ERR_INT_MASK [3:3]: 1'b1-mask interrupt, no interrupt will be generated. See RNG_ISR for an exp...
    #[inline(always)]
    pub const fn vn_err_int_mask(&self) -> bool {
        (self.bits & RngImr::VN_ERR_INT_MASK_MASK) != 0
    }

    /// Read CRNGT_ERR_INT_MASK [2:2]: 1'b1-mask interrupt, no interrupt will be generated. See RNG_ISR for an exp...
    #[inline(always)]
    pub const fn crngt_err_int_mask(&self) -> bool {
        (self.bits & RngImr::CRNGT_ERR_INT_MASK_MASK) != 0
    }

    /// Read AUTOCORR_ERR_INT_MASK [1:1]: 1'b1-mask interrupt, no interrupt will be generated. See RNG_ISR for an exp...
    #[inline(always)]
    pub const fn autocorr_err_int_mask(&self) -> bool {
        (self.bits & RngImr::AUTOCORR_ERR_INT_MASK_MASK) != 0
    }

    /// Read EHR_VALID_INT_MASK [0:0]: 1'b1-mask interrupt, no interrupt will be generated. See RNG_ISR for an exp...
    #[inline(always)]
    pub const fn ehr_valid_int_mask(&self) -> bool {
        (self.bits & RngImr::EHR_VALID_INT_MASK_MASK) != 0
    }

    /// Set VN_ERR_INT_MASK [3:3]: 1'b1-mask interrupt, no interrupt will be generated. See RNG_ISR for an exp...
    #[inline(always)]
    pub const fn with_vn_err_int_mask(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RngImr::VN_ERR_INT_MASK_MASK
            } else {
                self.bits & !RngImr::VN_ERR_INT_MASK_MASK
            }
        }
    }

    /// Set CRNGT_ERR_INT_MASK [2:2]: 1'b1-mask interrupt, no interrupt will be generated. See RNG_ISR for an exp...
    #[inline(always)]
    pub const fn with_crngt_err_int_mask(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RngImr::CRNGT_ERR_INT_MASK_MASK
            } else {
                self.bits & !RngImr::CRNGT_ERR_INT_MASK_MASK
            }
        }
    }

    /// Set AUTOCORR_ERR_INT_MASK [1:1]: 1'b1-mask interrupt, no interrupt will be generated. See RNG_ISR for an exp...
    #[inline(always)]
    pub const fn with_autocorr_err_int_mask(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RngImr::AUTOCORR_ERR_INT_MASK_MASK
            } else {
                self.bits & !RngImr::AUTOCORR_ERR_INT_MASK_MASK
            }
        }
    }

    /// Set EHR_VALID_INT_MASK [0:0]: 1'b1-mask interrupt, no interrupt will be generated. See RNG_ISR for an exp...
    #[inline(always)]
    pub const fn with_ehr_valid_int_mask(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RngImr::EHR_VALID_INT_MASK_MASK
            } else {
                self.bits & !RngImr::EHR_VALID_INT_MASK_MASK
            }
        }
    }

}

impl Default for RngImrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl RngImr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> RngImrVal {
        RngImrVal::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: RngImrVal) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(RngImrVal) -> RngImrVal>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// RNG_ISR Register
///
/// RNG status register. If corresponding RNG_IMR bit is unmasked, an interrupt
/// will be generated.
///
/// Address offset: 0x0104
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **RESERVED** [31:4] (RO): RESERVED
/// - **VN_ERR** [3] (RO): 1'b1 indicates Von Neuman error. Error in von Neuman occurs if 32 consecuti...
/// - **CRNGT_ERR** [2] (RO): 1'b1 indicates CRNGT in the RNG test failed. Failure occurs when two consec...
/// - **AUTOCORR_ERR** [1] (RO): 1'b1 indicates Autocorrelation test failed four times in a row. When set, R...
/// - **EHR_VALID** [0] (RO): 1'b1 indicates that 192 bits have been collected in the RNG, and are ready ...
#[repr(transparent)]
pub struct RngIsr {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for RngIsr {}
unsafe impl Sync for RngIsr {}

impl RngIsr {
    // ======== Field Constants ========

    /// RESERVED - RESERVED
    /// Bits [31:4], mask 0xfffffff0
    pub const RESERVED_OFFSET: u32 = 4;
    pub const RESERVED_MASK: u32 = 0xfffffff0;

    /// VN_ERR - 1'b1 indicates Von Neuman error. Error in von Neuman occurs if 32 consecuti...
    /// Bits [3:3], mask 0x00000008
    pub const VN_ERR_OFFSET: u32 = 3;
    pub const VN_ERR_MASK: u32 = 0x00000008;
    pub const VN_ERR: u32 = 1 << 3;

    /// CRNGT_ERR - 1'b1 indicates CRNGT in the RNG test failed. Failure occurs when two consec...
    /// Bits [2:2], mask 0x00000004
    pub const CRNGT_ERR_OFFSET: u32 = 2;
    pub const CRNGT_ERR_MASK: u32 = 0x00000004;
    pub const CRNGT_ERR: u32 = 1 << 2;

    /// AUTOCORR_ERR - 1'b1 indicates Autocorrelation test failed four times in a row. When set, R...
    /// Bits [1:1], mask 0x00000002
    pub const AUTOCORR_ERR_OFFSET: u32 = 1;
    pub const AUTOCORR_ERR_MASK: u32 = 0x00000002;
    pub const AUTOCORR_ERR: u32 = 1 << 1;

    /// EHR_VALID - 1'b1 indicates that 192 bits have been collected in the RNG, and are ready ...
    /// Bits [0:0], mask 0x00000001
    pub const EHR_VALID_OFFSET: u32 = 0;
    pub const EHR_VALID_MASK: u32 = 0x00000001;
    pub const EHR_VALID: u32 = 1 << 0;

    // ======== Basic Access Methods ========

    /// Read the raw register value as u32
    #[inline(always)]
    pub fn read_bits(&self) -> u32 {
        unsafe { core::ptr::read_volatile(self.value.get()) }
    }

    /// Write a raw u32 value to the register
    #[inline(always)]
    pub fn write_bits(&self, value: u32) {
        unsafe { core::ptr::write_volatile(self.value.get(), value) }
    }

    /// Modify the raw register value
    #[inline(always)]
    pub fn modify_bits<F: FnOnce(u32) -> u32>(&self, f: F) {
        self.write_bits(f(self.read_bits()));
    }

    /// Atomic SET - sets bits that are 1 (hardware alias at +0x2000)
    #[inline(always)]
    pub fn set_bits(&self, bits: u32) {
        unsafe {
            let addr = (self.value.get() as usize + 0x2000) as *mut u32;
            core::ptr::write_volatile(addr, bits);
        }
    }

    /// Atomic CLR - clears bits that are 1 (hardware alias at +0x3000)
    #[inline(always)]
    pub fn clr_bits(&self, bits: u32) {
        unsafe {
            let addr = (self.value.get() as usize + 0x3000) as *mut u32;
            core::ptr::write_volatile(addr, bits);
        }
    }

    /// Atomic XOR - toggles bits that are 1 (hardware alias at +0x1000)
    #[inline(always)]
    pub fn xor_bits(&self, bits: u32) {
        unsafe {
            let addr = (self.value.get() as usize + 0x1000) as *mut u32;
            core::ptr::write_volatile(addr, bits);
        }
    }

    // ======== Field Accessors ========

    /// **RESERVED** [31:4]: RESERVED
    #[inline(always)]
    pub fn reserved(&self) -> u32 {
        (self.read_bits() & Self::RESERVED_MASK) >> Self::RESERVED_OFFSET
    }

    /// **VN_ERR** [3:3]: 1'b1 indicates Von Neuman error. Error in von Neuman occurs if 32 consecuti...
    #[inline(always)]
    pub fn vn_err(&self) -> bool {
        (self.read_bits() & Self::VN_ERR_MASK) != 0
    }

    /// **CRNGT_ERR** [2:2]: 1'b1 indicates CRNGT in the RNG test failed. Failure occurs when two consec...
    #[inline(always)]
    pub fn crngt_err(&self) -> bool {
        (self.read_bits() & Self::CRNGT_ERR_MASK) != 0
    }

    /// **AUTOCORR_ERR** [1:1]: 1'b1 indicates Autocorrelation test failed four times in a row. When set, R...
    #[inline(always)]
    pub fn autocorr_err(&self) -> bool {
        (self.read_bits() & Self::AUTOCORR_ERR_MASK) != 0
    }

    /// **EHR_VALID** [0:0]: 1'b1 indicates that 192 bits have been collected in the RNG, and are ready ...
    #[inline(always)]
    pub fn ehr_valid(&self) -> bool {
        (self.read_bits() & Self::EHR_VALID_MASK) != 0
    }

}

/// RNG_ISR value type for batch read/modify/write operations
///
/// Use `rngisr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rngisr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RngIsrVal {
    bits: u32,
}

impl RngIsrVal {
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

    /// Read RESERVED [31:4]: RESERVED
    #[inline(always)]
    pub const fn reserved(&self) -> u32 {
        (self.bits & RngIsr::RESERVED_MASK) >> RngIsr::RESERVED_OFFSET
    }

    /// Read VN_ERR [3:3]: 1'b1 indicates Von Neuman error. Error in von Neuman occurs if 32 consecuti...
    #[inline(always)]
    pub const fn vn_err(&self) -> bool {
        (self.bits & RngIsr::VN_ERR_MASK) != 0
    }

    /// Read CRNGT_ERR [2:2]: 1'b1 indicates CRNGT in the RNG test failed. Failure occurs when two consec...
    #[inline(always)]
    pub const fn crngt_err(&self) -> bool {
        (self.bits & RngIsr::CRNGT_ERR_MASK) != 0
    }

    /// Read AUTOCORR_ERR [1:1]: 1'b1 indicates Autocorrelation test failed four times in a row. When set, R...
    #[inline(always)]
    pub const fn autocorr_err(&self) -> bool {
        (self.bits & RngIsr::AUTOCORR_ERR_MASK) != 0
    }

    /// Read EHR_VALID [0:0]: 1'b1 indicates that 192 bits have been collected in the RNG, and are ready ...
    #[inline(always)]
    pub const fn ehr_valid(&self) -> bool {
        (self.bits & RngIsr::EHR_VALID_MASK) != 0
    }

}

impl Default for RngIsrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl RngIsr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> RngIsrVal {
        RngIsrVal::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: RngIsrVal) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(RngIsrVal) -> RngIsrVal>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// RNG_ICR Register
///
/// Interrupt/status bit clear Register.
///
/// Address offset: 0x0108
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **RESERVED** [31:4] (RO): RESERVED
/// - **VN_ERR** [3] (RW): Write 1'b1 - clear corresponding bit in RNG_ISR.
/// - **CRNGT_ERR** [2] (RW): Write 1'b1 - clear corresponding bit in RNG_ISR.
/// - **AUTOCORR_ERR** [1] (RW): Cannot be cleared by SW! Only RNG reset clears this bit.
/// - **EHR_VALID** [0] (RW): Write 1'b1 - clear corresponding bit in RNG_ISR.
#[repr(transparent)]
pub struct RngIcr {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for RngIcr {}
unsafe impl Sync for RngIcr {}

impl RngIcr {
    // ======== Field Constants ========

    /// RESERVED - RESERVED
    /// Bits [31:4], mask 0xfffffff0
    pub const RESERVED_OFFSET: u32 = 4;
    pub const RESERVED_MASK: u32 = 0xfffffff0;

    /// VN_ERR - Write 1'b1 - clear corresponding bit in RNG_ISR.
    /// Bits [3:3], mask 0x00000008
    pub const VN_ERR_OFFSET: u32 = 3;
    pub const VN_ERR_MASK: u32 = 0x00000008;
    pub const VN_ERR: u32 = 1 << 3;

    /// CRNGT_ERR - Write 1'b1 - clear corresponding bit in RNG_ISR.
    /// Bits [2:2], mask 0x00000004
    pub const CRNGT_ERR_OFFSET: u32 = 2;
    pub const CRNGT_ERR_MASK: u32 = 0x00000004;
    pub const CRNGT_ERR: u32 = 1 << 2;

    /// AUTOCORR_ERR - Cannot be cleared by SW! Only RNG reset clears this bit.
    /// Bits [1:1], mask 0x00000002
    pub const AUTOCORR_ERR_OFFSET: u32 = 1;
    pub const AUTOCORR_ERR_MASK: u32 = 0x00000002;
    pub const AUTOCORR_ERR: u32 = 1 << 1;

    /// EHR_VALID - Write 1'b1 - clear corresponding bit in RNG_ISR.
    /// Bits [0:0], mask 0x00000001
    pub const EHR_VALID_OFFSET: u32 = 0;
    pub const EHR_VALID_MASK: u32 = 0x00000001;
    pub const EHR_VALID: u32 = 1 << 0;

    // ======== Basic Access Methods ========

    /// Read the raw register value as u32
    #[inline(always)]
    pub fn read_bits(&self) -> u32 {
        unsafe { core::ptr::read_volatile(self.value.get()) }
    }

    /// Write a raw u32 value to the register
    #[inline(always)]
    pub fn write_bits(&self, value: u32) {
        unsafe { core::ptr::write_volatile(self.value.get(), value) }
    }

    /// Modify the raw register value
    #[inline(always)]
    pub fn modify_bits<F: FnOnce(u32) -> u32>(&self, f: F) {
        self.write_bits(f(self.read_bits()));
    }

    /// Atomic SET - sets bits that are 1 (hardware alias at +0x2000)
    #[inline(always)]
    pub fn set_bits(&self, bits: u32) {
        unsafe {
            let addr = (self.value.get() as usize + 0x2000) as *mut u32;
            core::ptr::write_volatile(addr, bits);
        }
    }

    /// Atomic CLR - clears bits that are 1 (hardware alias at +0x3000)
    #[inline(always)]
    pub fn clr_bits(&self, bits: u32) {
        unsafe {
            let addr = (self.value.get() as usize + 0x3000) as *mut u32;
            core::ptr::write_volatile(addr, bits);
        }
    }

    /// Atomic XOR - toggles bits that are 1 (hardware alias at +0x1000)
    #[inline(always)]
    pub fn xor_bits(&self, bits: u32) {
        unsafe {
            let addr = (self.value.get() as usize + 0x1000) as *mut u32;
            core::ptr::write_volatile(addr, bits);
        }
    }

    // ======== Field Accessors ========

    /// **RESERVED** [31:4]: RESERVED
    #[inline(always)]
    pub fn reserved(&self) -> u32 {
        (self.read_bits() & Self::RESERVED_MASK) >> Self::RESERVED_OFFSET
    }

    /// **VN_ERR** [3:3]: Write 1'b1 - clear corresponding bit in RNG_ISR.
    #[inline(always)]
    pub fn vn_err(&self) -> bool {
        (self.read_bits() & Self::VN_ERR_MASK) != 0
    }

    /// Set VN_ERR bit
    #[inline(always)]
    pub fn set_vn_err(&self) {
        self.set_bits(Self::VN_ERR_MASK);
    }

    /// Clear VN_ERR bit
    #[inline(always)]
    pub fn clr_vn_err(&self) {
        self.clr_bits(Self::VN_ERR_MASK);
    }

    /// **CRNGT_ERR** [2:2]: Write 1'b1 - clear corresponding bit in RNG_ISR.
    #[inline(always)]
    pub fn crngt_err(&self) -> bool {
        (self.read_bits() & Self::CRNGT_ERR_MASK) != 0
    }

    /// Set CRNGT_ERR bit
    #[inline(always)]
    pub fn set_crngt_err(&self) {
        self.set_bits(Self::CRNGT_ERR_MASK);
    }

    /// Clear CRNGT_ERR bit
    #[inline(always)]
    pub fn clr_crngt_err(&self) {
        self.clr_bits(Self::CRNGT_ERR_MASK);
    }

    /// **AUTOCORR_ERR** [1:1]: Cannot be cleared by SW! Only RNG reset clears this bit.
    #[inline(always)]
    pub fn autocorr_err(&self) -> bool {
        (self.read_bits() & Self::AUTOCORR_ERR_MASK) != 0
    }

    /// Set AUTOCORR_ERR bit
    #[inline(always)]
    pub fn set_autocorr_err(&self) {
        self.set_bits(Self::AUTOCORR_ERR_MASK);
    }

    /// Clear AUTOCORR_ERR bit
    #[inline(always)]
    pub fn clr_autocorr_err(&self) {
        self.clr_bits(Self::AUTOCORR_ERR_MASK);
    }

    /// **EHR_VALID** [0:0]: Write 1'b1 - clear corresponding bit in RNG_ISR.
    #[inline(always)]
    pub fn ehr_valid(&self) -> bool {
        (self.read_bits() & Self::EHR_VALID_MASK) != 0
    }

    /// Set EHR_VALID bit
    #[inline(always)]
    pub fn set_ehr_valid(&self) {
        self.set_bits(Self::EHR_VALID_MASK);
    }

    /// Clear EHR_VALID bit
    #[inline(always)]
    pub fn clr_ehr_valid(&self) {
        self.clr_bits(Self::EHR_VALID_MASK);
    }

}

/// RNG_ICR value type for batch read/modify/write operations
///
/// Use `rngicr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rngicr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RngIcrVal {
    bits: u32,
}

impl RngIcrVal {
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

    /// Read RESERVED [31:4]: RESERVED
    #[inline(always)]
    pub const fn reserved(&self) -> u32 {
        (self.bits & RngIcr::RESERVED_MASK) >> RngIcr::RESERVED_OFFSET
    }

    /// Read VN_ERR [3:3]: Write 1'b1 - clear corresponding bit in RNG_ISR.
    #[inline(always)]
    pub const fn vn_err(&self) -> bool {
        (self.bits & RngIcr::VN_ERR_MASK) != 0
    }

    /// Read CRNGT_ERR [2:2]: Write 1'b1 - clear corresponding bit in RNG_ISR.
    #[inline(always)]
    pub const fn crngt_err(&self) -> bool {
        (self.bits & RngIcr::CRNGT_ERR_MASK) != 0
    }

    /// Read AUTOCORR_ERR [1:1]: Cannot be cleared by SW! Only RNG reset clears this bit.
    #[inline(always)]
    pub const fn autocorr_err(&self) -> bool {
        (self.bits & RngIcr::AUTOCORR_ERR_MASK) != 0
    }

    /// Read EHR_VALID [0:0]: Write 1'b1 - clear corresponding bit in RNG_ISR.
    #[inline(always)]
    pub const fn ehr_valid(&self) -> bool {
        (self.bits & RngIcr::EHR_VALID_MASK) != 0
    }

    /// Set VN_ERR [3:3]: Write 1'b1 - clear corresponding bit in RNG_ISR.
    #[inline(always)]
    pub const fn with_vn_err(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RngIcr::VN_ERR_MASK
            } else {
                self.bits & !RngIcr::VN_ERR_MASK
            }
        }
    }

    /// Set CRNGT_ERR [2:2]: Write 1'b1 - clear corresponding bit in RNG_ISR.
    #[inline(always)]
    pub const fn with_crngt_err(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RngIcr::CRNGT_ERR_MASK
            } else {
                self.bits & !RngIcr::CRNGT_ERR_MASK
            }
        }
    }

    /// Set AUTOCORR_ERR [1:1]: Cannot be cleared by SW! Only RNG reset clears this bit.
    #[inline(always)]
    pub const fn with_autocorr_err(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RngIcr::AUTOCORR_ERR_MASK
            } else {
                self.bits & !RngIcr::AUTOCORR_ERR_MASK
            }
        }
    }

    /// Set EHR_VALID [0:0]: Write 1'b1 - clear corresponding bit in RNG_ISR.
    #[inline(always)]
    pub const fn with_ehr_valid(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RngIcr::EHR_VALID_MASK
            } else {
                self.bits & !RngIcr::EHR_VALID_MASK
            }
        }
    }

}

impl Default for RngIcrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl RngIcr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> RngIcrVal {
        RngIcrVal::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: RngIcrVal) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(RngIcrVal) -> RngIcrVal>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// TRNG_CONFIG Register
///
/// Selecting the inverter-chain length.
///
/// Address offset: 0x010c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **RESERVED** [31:2] (RO): RESERVED
/// - **RND_SRC_SEL** [1:0] (RW): Selects the number of inverters (out of four possible selections) in the ri...
#[repr(transparent)]
pub struct TrngConfig {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for TrngConfig {}
unsafe impl Sync for TrngConfig {}

impl TrngConfig {
    // ======== Field Constants ========

    /// RESERVED - RESERVED
    /// Bits [31:2], mask 0xfffffffc
    pub const RESERVED_OFFSET: u32 = 2;
    pub const RESERVED_MASK: u32 = 0xfffffffc;

    /// RND_SRC_SEL - Selects the number of inverters (out of four possible selections) in the ri...
    /// Bits [1:0], mask 0x00000003
    pub const RND_SRC_SEL_OFFSET: u32 = 0;
    pub const RND_SRC_SEL_MASK: u32 = 0x00000003;

    // ======== Basic Access Methods ========

    /// Read the raw register value as u32
    #[inline(always)]
    pub fn read_bits(&self) -> u32 {
        unsafe { core::ptr::read_volatile(self.value.get()) }
    }

    /// Write a raw u32 value to the register
    #[inline(always)]
    pub fn write_bits(&self, value: u32) {
        unsafe { core::ptr::write_volatile(self.value.get(), value) }
    }

    /// Modify the raw register value
    #[inline(always)]
    pub fn modify_bits<F: FnOnce(u32) -> u32>(&self, f: F) {
        self.write_bits(f(self.read_bits()));
    }

    /// Atomic SET - sets bits that are 1 (hardware alias at +0x2000)
    #[inline(always)]
    pub fn set_bits(&self, bits: u32) {
        unsafe {
            let addr = (self.value.get() as usize + 0x2000) as *mut u32;
            core::ptr::write_volatile(addr, bits);
        }
    }

    /// Atomic CLR - clears bits that are 1 (hardware alias at +0x3000)
    #[inline(always)]
    pub fn clr_bits(&self, bits: u32) {
        unsafe {
            let addr = (self.value.get() as usize + 0x3000) as *mut u32;
            core::ptr::write_volatile(addr, bits);
        }
    }

    /// Atomic XOR - toggles bits that are 1 (hardware alias at +0x1000)
    #[inline(always)]
    pub fn xor_bits(&self, bits: u32) {
        unsafe {
            let addr = (self.value.get() as usize + 0x1000) as *mut u32;
            core::ptr::write_volatile(addr, bits);
        }
    }

    // ======== Field Accessors ========

    /// **RESERVED** [31:2]: RESERVED
    #[inline(always)]
    pub fn reserved(&self) -> u32 {
        (self.read_bits() & Self::RESERVED_MASK) >> Self::RESERVED_OFFSET
    }

    /// **RND_SRC_SEL** [1:0]: Selects the number of inverters (out of four possible selections) in the ri...
    #[inline(always)]
    pub fn rnd_src_sel(&self) -> u32 {
        (self.read_bits() & Self::RND_SRC_SEL_MASK) >> Self::RND_SRC_SEL_OFFSET
    }

    /// Write RND_SRC_SEL field
    #[inline(always)]
    pub fn write_rnd_src_sel(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::RND_SRC_SEL_MASK) | ((value << Self::RND_SRC_SEL_OFFSET) & Self::RND_SRC_SEL_MASK));
    }

}

/// TRNG_CONFIG value type for batch read/modify/write operations
///
/// Use `trngconfig.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `trngconfig.write_val(val)`.
#[derive(Clone, Copy)]
pub struct TrngConfigVal {
    bits: u32,
}

impl TrngConfigVal {
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

    /// Read RESERVED [31:2]: RESERVED
    #[inline(always)]
    pub const fn reserved(&self) -> u32 {
        (self.bits & TrngConfig::RESERVED_MASK) >> TrngConfig::RESERVED_OFFSET
    }

    /// Read RND_SRC_SEL [1:0]: Selects the number of inverters (out of four possible selections) in the ri...
    #[inline(always)]
    pub const fn rnd_src_sel(&self) -> u32 {
        (self.bits & TrngConfig::RND_SRC_SEL_MASK) >> TrngConfig::RND_SRC_SEL_OFFSET
    }

    /// Set RND_SRC_SEL [1:0]: Selects the number of inverters (out of four possible selections) in the ri...
    #[inline(always)]
    pub const fn with_rnd_src_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !TrngConfig::RND_SRC_SEL_MASK) | ((value << TrngConfig::RND_SRC_SEL_OFFSET) & TrngConfig::RND_SRC_SEL_MASK)
        }
    }

}

impl Default for TrngConfigVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl TrngConfig {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> TrngConfigVal {
        TrngConfigVal::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: TrngConfigVal) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(TrngConfigVal) -> TrngConfigVal>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// TRNG_VALID Register
///
/// 192 bit collection indication.
///
/// Address offset: 0x0110
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **RESERVED** [31:1] (RO): RESERVED
/// - **EHR_VALID** [0] (RO): 1'b1 indicates that collection of bits in the RNG is completed, and data ca...
#[repr(transparent)]
pub struct TrngValid {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for TrngValid {}
unsafe impl Sync for TrngValid {}

impl TrngValid {
    // ======== Field Constants ========

    /// RESERVED - RESERVED
    /// Bits [31:1], mask 0xfffffffe
    pub const RESERVED_OFFSET: u32 = 1;
    pub const RESERVED_MASK: u32 = 0xfffffffe;

    /// EHR_VALID - 1'b1 indicates that collection of bits in the RNG is completed, and data ca...
    /// Bits [0:0], mask 0x00000001
    pub const EHR_VALID_OFFSET: u32 = 0;
    pub const EHR_VALID_MASK: u32 = 0x00000001;
    pub const EHR_VALID: u32 = 1 << 0;

    // ======== Basic Access Methods ========

    /// Read the raw register value as u32
    #[inline(always)]
    pub fn read_bits(&self) -> u32 {
        unsafe { core::ptr::read_volatile(self.value.get()) }
    }

    /// Write a raw u32 value to the register
    #[inline(always)]
    pub fn write_bits(&self, value: u32) {
        unsafe { core::ptr::write_volatile(self.value.get(), value) }
    }

    /// Modify the raw register value
    #[inline(always)]
    pub fn modify_bits<F: FnOnce(u32) -> u32>(&self, f: F) {
        self.write_bits(f(self.read_bits()));
    }

    /// Atomic SET - sets bits that are 1 (hardware alias at +0x2000)
    #[inline(always)]
    pub fn set_bits(&self, bits: u32) {
        unsafe {
            let addr = (self.value.get() as usize + 0x2000) as *mut u32;
            core::ptr::write_volatile(addr, bits);
        }
    }

    /// Atomic CLR - clears bits that are 1 (hardware alias at +0x3000)
    #[inline(always)]
    pub fn clr_bits(&self, bits: u32) {
        unsafe {
            let addr = (self.value.get() as usize + 0x3000) as *mut u32;
            core::ptr::write_volatile(addr, bits);
        }
    }

    /// Atomic XOR - toggles bits that are 1 (hardware alias at +0x1000)
    #[inline(always)]
    pub fn xor_bits(&self, bits: u32) {
        unsafe {
            let addr = (self.value.get() as usize + 0x1000) as *mut u32;
            core::ptr::write_volatile(addr, bits);
        }
    }

    // ======== Field Accessors ========

    /// **RESERVED** [31:1]: RESERVED
    #[inline(always)]
    pub fn reserved(&self) -> u32 {
        (self.read_bits() & Self::RESERVED_MASK) >> Self::RESERVED_OFFSET
    }

    /// **EHR_VALID** [0:0]: 1'b1 indicates that collection of bits in the RNG is completed, and data ca...
    #[inline(always)]
    pub fn ehr_valid(&self) -> bool {
        (self.read_bits() & Self::EHR_VALID_MASK) != 0
    }

}

/// TRNG_VALID value type for batch read/modify/write operations
///
/// Use `trngvalid.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `trngvalid.write_val(val)`.
#[derive(Clone, Copy)]
pub struct TrngValidVal {
    bits: u32,
}

impl TrngValidVal {
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

    /// Read RESERVED [31:1]: RESERVED
    #[inline(always)]
    pub const fn reserved(&self) -> u32 {
        (self.bits & TrngValid::RESERVED_MASK) >> TrngValid::RESERVED_OFFSET
    }

    /// Read EHR_VALID [0:0]: 1'b1 indicates that collection of bits in the RNG is completed, and data ca...
    #[inline(always)]
    pub const fn ehr_valid(&self) -> bool {
        (self.bits & TrngValid::EHR_VALID_MASK) != 0
    }

}

impl Default for TrngValidVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl TrngValid {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> TrngValidVal {
        TrngValidVal::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: TrngValidVal) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(TrngValidVal) -> TrngValidVal>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// EHR_DATA0 Register
///
/// RNG collected bits.
///
/// Address offset: 0x0114
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **EHR_DATA0** [31:0] (RO): Bits [31:0] of Entropy Holding Register (EHR) - RNG output register
#[repr(transparent)]
pub struct EhrData0 {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for EhrData0 {}
unsafe impl Sync for EhrData0 {}

impl EhrData0 {
    // ======== Field Constants ========

    /// EHR_DATA0 - Bits [31:0] of Entropy Holding Register (EHR) - RNG output register
    /// Bits [31:0], mask 0xffffffff
    pub const EHR_DATA0_OFFSET: u32 = 0;
    pub const EHR_DATA0_MASK: u32 = 0xffffffff;

    // ======== Basic Access Methods ========

    /// Read the raw register value as u32
    #[inline(always)]
    pub fn read_bits(&self) -> u32 {
        unsafe { core::ptr::read_volatile(self.value.get()) }
    }

    /// Write a raw u32 value to the register
    #[inline(always)]
    pub fn write_bits(&self, value: u32) {
        unsafe { core::ptr::write_volatile(self.value.get(), value) }
    }

    /// Modify the raw register value
    #[inline(always)]
    pub fn modify_bits<F: FnOnce(u32) -> u32>(&self, f: F) {
        self.write_bits(f(self.read_bits()));
    }

    /// Atomic SET - sets bits that are 1 (hardware alias at +0x2000)
    #[inline(always)]
    pub fn set_bits(&self, bits: u32) {
        unsafe {
            let addr = (self.value.get() as usize + 0x2000) as *mut u32;
            core::ptr::write_volatile(addr, bits);
        }
    }

    /// Atomic CLR - clears bits that are 1 (hardware alias at +0x3000)
    #[inline(always)]
    pub fn clr_bits(&self, bits: u32) {
        unsafe {
            let addr = (self.value.get() as usize + 0x3000) as *mut u32;
            core::ptr::write_volatile(addr, bits);
        }
    }

    /// Atomic XOR - toggles bits that are 1 (hardware alias at +0x1000)
    #[inline(always)]
    pub fn xor_bits(&self, bits: u32) {
        unsafe {
            let addr = (self.value.get() as usize + 0x1000) as *mut u32;
            core::ptr::write_volatile(addr, bits);
        }
    }

    // ======== Field Accessors ========

    /// **EHR_DATA0** [31:0]: Bits [31:0] of Entropy Holding Register (EHR) - RNG output register
    #[inline(always)]
    pub fn ehr_data0(&self) -> u32 {
        (self.read_bits() & Self::EHR_DATA0_MASK) >> Self::EHR_DATA0_OFFSET
    }

}

/// EHR_DATA0 value type for batch read/modify/write operations
///
/// Use `ehrdata0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ehrdata0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct EhrData0Val {
    bits: u32,
}

impl EhrData0Val {
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

    /// Read EHR_DATA0 [31:0]: Bits [31:0] of Entropy Holding Register (EHR) - RNG output register
    #[inline(always)]
    pub const fn ehr_data0(&self) -> u32 {
        (self.bits & EhrData0::EHR_DATA0_MASK) >> EhrData0::EHR_DATA0_OFFSET
    }

}

impl Default for EhrData0Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl EhrData0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> EhrData0Val {
        EhrData0Val::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: EhrData0Val) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(EhrData0Val) -> EhrData0Val>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// EHR_DATA1 Register
///
/// RNG collected bits.
///
/// Address offset: 0x0118
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **EHR_DATA1** [31:0] (RO): Bits [63:32] of Entropy Holding Register (EHR) - RNG output register
#[repr(transparent)]
pub struct EhrData1 {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for EhrData1 {}
unsafe impl Sync for EhrData1 {}

impl EhrData1 {
    // ======== Field Constants ========

    /// EHR_DATA1 - Bits [63:32] of Entropy Holding Register (EHR) - RNG output register
    /// Bits [31:0], mask 0xffffffff
    pub const EHR_DATA1_OFFSET: u32 = 0;
    pub const EHR_DATA1_MASK: u32 = 0xffffffff;

    // ======== Basic Access Methods ========

    /// Read the raw register value as u32
    #[inline(always)]
    pub fn read_bits(&self) -> u32 {
        unsafe { core::ptr::read_volatile(self.value.get()) }
    }

    /// Write a raw u32 value to the register
    #[inline(always)]
    pub fn write_bits(&self, value: u32) {
        unsafe { core::ptr::write_volatile(self.value.get(), value) }
    }

    /// Modify the raw register value
    #[inline(always)]
    pub fn modify_bits<F: FnOnce(u32) -> u32>(&self, f: F) {
        self.write_bits(f(self.read_bits()));
    }

    /// Atomic SET - sets bits that are 1 (hardware alias at +0x2000)
    #[inline(always)]
    pub fn set_bits(&self, bits: u32) {
        unsafe {
            let addr = (self.value.get() as usize + 0x2000) as *mut u32;
            core::ptr::write_volatile(addr, bits);
        }
    }

    /// Atomic CLR - clears bits that are 1 (hardware alias at +0x3000)
    #[inline(always)]
    pub fn clr_bits(&self, bits: u32) {
        unsafe {
            let addr = (self.value.get() as usize + 0x3000) as *mut u32;
            core::ptr::write_volatile(addr, bits);
        }
    }

    /// Atomic XOR - toggles bits that are 1 (hardware alias at +0x1000)
    #[inline(always)]
    pub fn xor_bits(&self, bits: u32) {
        unsafe {
            let addr = (self.value.get() as usize + 0x1000) as *mut u32;
            core::ptr::write_volatile(addr, bits);
        }
    }

    // ======== Field Accessors ========

    /// **EHR_DATA1** [31:0]: Bits [63:32] of Entropy Holding Register (EHR) - RNG output register
    #[inline(always)]
    pub fn ehr_data1(&self) -> u32 {
        (self.read_bits() & Self::EHR_DATA1_MASK) >> Self::EHR_DATA1_OFFSET
    }

}

/// EHR_DATA1 value type for batch read/modify/write operations
///
/// Use `ehrdata1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ehrdata1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct EhrData1Val {
    bits: u32,
}

impl EhrData1Val {
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

    /// Read EHR_DATA1 [31:0]: Bits [63:32] of Entropy Holding Register (EHR) - RNG output register
    #[inline(always)]
    pub const fn ehr_data1(&self) -> u32 {
        (self.bits & EhrData1::EHR_DATA1_MASK) >> EhrData1::EHR_DATA1_OFFSET
    }

}

impl Default for EhrData1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl EhrData1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> EhrData1Val {
        EhrData1Val::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: EhrData1Val) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(EhrData1Val) -> EhrData1Val>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// EHR_DATA2 Register
///
/// RNG collected bits.
///
/// Address offset: 0x011c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **EHR_DATA2** [31:0] (RO): Bits [95:64] of Entropy Holding Register (EHR) - RNG output register
#[repr(transparent)]
pub struct EhrData2 {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for EhrData2 {}
unsafe impl Sync for EhrData2 {}

impl EhrData2 {
    // ======== Field Constants ========

    /// EHR_DATA2 - Bits [95:64] of Entropy Holding Register (EHR) - RNG output register
    /// Bits [31:0], mask 0xffffffff
    pub const EHR_DATA2_OFFSET: u32 = 0;
    pub const EHR_DATA2_MASK: u32 = 0xffffffff;

    // ======== Basic Access Methods ========

    /// Read the raw register value as u32
    #[inline(always)]
    pub fn read_bits(&self) -> u32 {
        unsafe { core::ptr::read_volatile(self.value.get()) }
    }

    /// Write a raw u32 value to the register
    #[inline(always)]
    pub fn write_bits(&self, value: u32) {
        unsafe { core::ptr::write_volatile(self.value.get(), value) }
    }

    /// Modify the raw register value
    #[inline(always)]
    pub fn modify_bits<F: FnOnce(u32) -> u32>(&self, f: F) {
        self.write_bits(f(self.read_bits()));
    }

    /// Atomic SET - sets bits that are 1 (hardware alias at +0x2000)
    #[inline(always)]
    pub fn set_bits(&self, bits: u32) {
        unsafe {
            let addr = (self.value.get() as usize + 0x2000) as *mut u32;
            core::ptr::write_volatile(addr, bits);
        }
    }

    /// Atomic CLR - clears bits that are 1 (hardware alias at +0x3000)
    #[inline(always)]
    pub fn clr_bits(&self, bits: u32) {
        unsafe {
            let addr = (self.value.get() as usize + 0x3000) as *mut u32;
            core::ptr::write_volatile(addr, bits);
        }
    }

    /// Atomic XOR - toggles bits that are 1 (hardware alias at +0x1000)
    #[inline(always)]
    pub fn xor_bits(&self, bits: u32) {
        unsafe {
            let addr = (self.value.get() as usize + 0x1000) as *mut u32;
            core::ptr::write_volatile(addr, bits);
        }
    }

    // ======== Field Accessors ========

    /// **EHR_DATA2** [31:0]: Bits [95:64] of Entropy Holding Register (EHR) - RNG output register
    #[inline(always)]
    pub fn ehr_data2(&self) -> u32 {
        (self.read_bits() & Self::EHR_DATA2_MASK) >> Self::EHR_DATA2_OFFSET
    }

}

/// EHR_DATA2 value type for batch read/modify/write operations
///
/// Use `ehrdata2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ehrdata2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct EhrData2Val {
    bits: u32,
}

impl EhrData2Val {
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

    /// Read EHR_DATA2 [31:0]: Bits [95:64] of Entropy Holding Register (EHR) - RNG output register
    #[inline(always)]
    pub const fn ehr_data2(&self) -> u32 {
        (self.bits & EhrData2::EHR_DATA2_MASK) >> EhrData2::EHR_DATA2_OFFSET
    }

}

impl Default for EhrData2Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl EhrData2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> EhrData2Val {
        EhrData2Val::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: EhrData2Val) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(EhrData2Val) -> EhrData2Val>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// EHR_DATA3 Register
///
/// RNG collected bits.
///
/// Address offset: 0x0120
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **EHR_DATA3** [31:0] (RO): Bits [127:96] of Entropy Holding Register (EHR) - RNG output register
#[repr(transparent)]
pub struct EhrData3 {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for EhrData3 {}
unsafe impl Sync for EhrData3 {}

impl EhrData3 {
    // ======== Field Constants ========

    /// EHR_DATA3 - Bits [127:96] of Entropy Holding Register (EHR) - RNG output register
    /// Bits [31:0], mask 0xffffffff
    pub const EHR_DATA3_OFFSET: u32 = 0;
    pub const EHR_DATA3_MASK: u32 = 0xffffffff;

    // ======== Basic Access Methods ========

    /// Read the raw register value as u32
    #[inline(always)]
    pub fn read_bits(&self) -> u32 {
        unsafe { core::ptr::read_volatile(self.value.get()) }
    }

    /// Write a raw u32 value to the register
    #[inline(always)]
    pub fn write_bits(&self, value: u32) {
        unsafe { core::ptr::write_volatile(self.value.get(), value) }
    }

    /// Modify the raw register value
    #[inline(always)]
    pub fn modify_bits<F: FnOnce(u32) -> u32>(&self, f: F) {
        self.write_bits(f(self.read_bits()));
    }

    /// Atomic SET - sets bits that are 1 (hardware alias at +0x2000)
    #[inline(always)]
    pub fn set_bits(&self, bits: u32) {
        unsafe {
            let addr = (self.value.get() as usize + 0x2000) as *mut u32;
            core::ptr::write_volatile(addr, bits);
        }
    }

    /// Atomic CLR - clears bits that are 1 (hardware alias at +0x3000)
    #[inline(always)]
    pub fn clr_bits(&self, bits: u32) {
        unsafe {
            let addr = (self.value.get() as usize + 0x3000) as *mut u32;
            core::ptr::write_volatile(addr, bits);
        }
    }

    /// Atomic XOR - toggles bits that are 1 (hardware alias at +0x1000)
    #[inline(always)]
    pub fn xor_bits(&self, bits: u32) {
        unsafe {
            let addr = (self.value.get() as usize + 0x1000) as *mut u32;
            core::ptr::write_volatile(addr, bits);
        }
    }

    // ======== Field Accessors ========

    /// **EHR_DATA3** [31:0]: Bits [127:96] of Entropy Holding Register (EHR) - RNG output register
    #[inline(always)]
    pub fn ehr_data3(&self) -> u32 {
        (self.read_bits() & Self::EHR_DATA3_MASK) >> Self::EHR_DATA3_OFFSET
    }

}

/// EHR_DATA3 value type for batch read/modify/write operations
///
/// Use `ehrdata3.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ehrdata3.write_val(val)`.
#[derive(Clone, Copy)]
pub struct EhrData3Val {
    bits: u32,
}

impl EhrData3Val {
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

    /// Read EHR_DATA3 [31:0]: Bits [127:96] of Entropy Holding Register (EHR) - RNG output register
    #[inline(always)]
    pub const fn ehr_data3(&self) -> u32 {
        (self.bits & EhrData3::EHR_DATA3_MASK) >> EhrData3::EHR_DATA3_OFFSET
    }

}

impl Default for EhrData3Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl EhrData3 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> EhrData3Val {
        EhrData3Val::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: EhrData3Val) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(EhrData3Val) -> EhrData3Val>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// EHR_DATA4 Register
///
/// RNG collected bits.
///
/// Address offset: 0x0124
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **EHR_DATA4** [31:0] (RO): Bits [159:128] of Entropy Holding Register (EHR) - RNG output register
#[repr(transparent)]
pub struct EhrData4 {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for EhrData4 {}
unsafe impl Sync for EhrData4 {}

impl EhrData4 {
    // ======== Field Constants ========

    /// EHR_DATA4 - Bits [159:128] of Entropy Holding Register (EHR) - RNG output register
    /// Bits [31:0], mask 0xffffffff
    pub const EHR_DATA4_OFFSET: u32 = 0;
    pub const EHR_DATA4_MASK: u32 = 0xffffffff;

    // ======== Basic Access Methods ========

    /// Read the raw register value as u32
    #[inline(always)]
    pub fn read_bits(&self) -> u32 {
        unsafe { core::ptr::read_volatile(self.value.get()) }
    }

    /// Write a raw u32 value to the register
    #[inline(always)]
    pub fn write_bits(&self, value: u32) {
        unsafe { core::ptr::write_volatile(self.value.get(), value) }
    }

    /// Modify the raw register value
    #[inline(always)]
    pub fn modify_bits<F: FnOnce(u32) -> u32>(&self, f: F) {
        self.write_bits(f(self.read_bits()));
    }

    /// Atomic SET - sets bits that are 1 (hardware alias at +0x2000)
    #[inline(always)]
    pub fn set_bits(&self, bits: u32) {
        unsafe {
            let addr = (self.value.get() as usize + 0x2000) as *mut u32;
            core::ptr::write_volatile(addr, bits);
        }
    }

    /// Atomic CLR - clears bits that are 1 (hardware alias at +0x3000)
    #[inline(always)]
    pub fn clr_bits(&self, bits: u32) {
        unsafe {
            let addr = (self.value.get() as usize + 0x3000) as *mut u32;
            core::ptr::write_volatile(addr, bits);
        }
    }

    /// Atomic XOR - toggles bits that are 1 (hardware alias at +0x1000)
    #[inline(always)]
    pub fn xor_bits(&self, bits: u32) {
        unsafe {
            let addr = (self.value.get() as usize + 0x1000) as *mut u32;
            core::ptr::write_volatile(addr, bits);
        }
    }

    // ======== Field Accessors ========

    /// **EHR_DATA4** [31:0]: Bits [159:128] of Entropy Holding Register (EHR) - RNG output register
    #[inline(always)]
    pub fn ehr_data4(&self) -> u32 {
        (self.read_bits() & Self::EHR_DATA4_MASK) >> Self::EHR_DATA4_OFFSET
    }

}

/// EHR_DATA4 value type for batch read/modify/write operations
///
/// Use `ehrdata4.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ehrdata4.write_val(val)`.
#[derive(Clone, Copy)]
pub struct EhrData4Val {
    bits: u32,
}

impl EhrData4Val {
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

    /// Read EHR_DATA4 [31:0]: Bits [159:128] of Entropy Holding Register (EHR) - RNG output register
    #[inline(always)]
    pub const fn ehr_data4(&self) -> u32 {
        (self.bits & EhrData4::EHR_DATA4_MASK) >> EhrData4::EHR_DATA4_OFFSET
    }

}

impl Default for EhrData4Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl EhrData4 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> EhrData4Val {
        EhrData4Val::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: EhrData4Val) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(EhrData4Val) -> EhrData4Val>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// EHR_DATA5 Register
///
/// RNG collected bits.
///
/// Address offset: 0x0128
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **EHR_DATA5** [31:0] (RO): Bits [191:160] of Entropy Holding Register (EHR) - RNG output register
#[repr(transparent)]
pub struct EhrData5 {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for EhrData5 {}
unsafe impl Sync for EhrData5 {}

impl EhrData5 {
    // ======== Field Constants ========

    /// EHR_DATA5 - Bits [191:160] of Entropy Holding Register (EHR) - RNG output register
    /// Bits [31:0], mask 0xffffffff
    pub const EHR_DATA5_OFFSET: u32 = 0;
    pub const EHR_DATA5_MASK: u32 = 0xffffffff;

    // ======== Basic Access Methods ========

    /// Read the raw register value as u32
    #[inline(always)]
    pub fn read_bits(&self) -> u32 {
        unsafe { core::ptr::read_volatile(self.value.get()) }
    }

    /// Write a raw u32 value to the register
    #[inline(always)]
    pub fn write_bits(&self, value: u32) {
        unsafe { core::ptr::write_volatile(self.value.get(), value) }
    }

    /// Modify the raw register value
    #[inline(always)]
    pub fn modify_bits<F: FnOnce(u32) -> u32>(&self, f: F) {
        self.write_bits(f(self.read_bits()));
    }

    /// Atomic SET - sets bits that are 1 (hardware alias at +0x2000)
    #[inline(always)]
    pub fn set_bits(&self, bits: u32) {
        unsafe {
            let addr = (self.value.get() as usize + 0x2000) as *mut u32;
            core::ptr::write_volatile(addr, bits);
        }
    }

    /// Atomic CLR - clears bits that are 1 (hardware alias at +0x3000)
    #[inline(always)]
    pub fn clr_bits(&self, bits: u32) {
        unsafe {
            let addr = (self.value.get() as usize + 0x3000) as *mut u32;
            core::ptr::write_volatile(addr, bits);
        }
    }

    /// Atomic XOR - toggles bits that are 1 (hardware alias at +0x1000)
    #[inline(always)]
    pub fn xor_bits(&self, bits: u32) {
        unsafe {
            let addr = (self.value.get() as usize + 0x1000) as *mut u32;
            core::ptr::write_volatile(addr, bits);
        }
    }

    // ======== Field Accessors ========

    /// **EHR_DATA5** [31:0]: Bits [191:160] of Entropy Holding Register (EHR) - RNG output register
    #[inline(always)]
    pub fn ehr_data5(&self) -> u32 {
        (self.read_bits() & Self::EHR_DATA5_MASK) >> Self::EHR_DATA5_OFFSET
    }

}

/// EHR_DATA5 value type for batch read/modify/write operations
///
/// Use `ehrdata5.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ehrdata5.write_val(val)`.
#[derive(Clone, Copy)]
pub struct EhrData5Val {
    bits: u32,
}

impl EhrData5Val {
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

    /// Read EHR_DATA5 [31:0]: Bits [191:160] of Entropy Holding Register (EHR) - RNG output register
    #[inline(always)]
    pub const fn ehr_data5(&self) -> u32 {
        (self.bits & EhrData5::EHR_DATA5_MASK) >> EhrData5::EHR_DATA5_OFFSET
    }

}

impl Default for EhrData5Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl EhrData5 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> EhrData5Val {
        EhrData5Val::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: EhrData5Val) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(EhrData5Val) -> EhrData5Val>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// RND_SOURCE_ENABLE Register
///
/// Enable signal for the random source.
///
/// Address offset: 0x012c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **RESERVED** [31:1] (RO): RESERVED
/// - **RND_SRC_EN** [0] (RW): * 1'b1 - entropy source is enabled. *1'b0 - entropy source is disabled
#[repr(transparent)]
pub struct RndSourceEnable {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for RndSourceEnable {}
unsafe impl Sync for RndSourceEnable {}

impl RndSourceEnable {
    // ======== Field Constants ========

    /// RESERVED - RESERVED
    /// Bits [31:1], mask 0xfffffffe
    pub const RESERVED_OFFSET: u32 = 1;
    pub const RESERVED_MASK: u32 = 0xfffffffe;

    /// RND_SRC_EN - * 1'b1 - entropy source is enabled. *1'b0 - entropy source is disabled
    /// Bits [0:0], mask 0x00000001
    pub const RND_SRC_EN_OFFSET: u32 = 0;
    pub const RND_SRC_EN_MASK: u32 = 0x00000001;
    pub const RND_SRC_EN: u32 = 1 << 0;

    // ======== Basic Access Methods ========

    /// Read the raw register value as u32
    #[inline(always)]
    pub fn read_bits(&self) -> u32 {
        unsafe { core::ptr::read_volatile(self.value.get()) }
    }

    /// Write a raw u32 value to the register
    #[inline(always)]
    pub fn write_bits(&self, value: u32) {
        unsafe { core::ptr::write_volatile(self.value.get(), value) }
    }

    /// Modify the raw register value
    #[inline(always)]
    pub fn modify_bits<F: FnOnce(u32) -> u32>(&self, f: F) {
        self.write_bits(f(self.read_bits()));
    }

    /// Atomic SET - sets bits that are 1 (hardware alias at +0x2000)
    #[inline(always)]
    pub fn set_bits(&self, bits: u32) {
        unsafe {
            let addr = (self.value.get() as usize + 0x2000) as *mut u32;
            core::ptr::write_volatile(addr, bits);
        }
    }

    /// Atomic CLR - clears bits that are 1 (hardware alias at +0x3000)
    #[inline(always)]
    pub fn clr_bits(&self, bits: u32) {
        unsafe {
            let addr = (self.value.get() as usize + 0x3000) as *mut u32;
            core::ptr::write_volatile(addr, bits);
        }
    }

    /// Atomic XOR - toggles bits that are 1 (hardware alias at +0x1000)
    #[inline(always)]
    pub fn xor_bits(&self, bits: u32) {
        unsafe {
            let addr = (self.value.get() as usize + 0x1000) as *mut u32;
            core::ptr::write_volatile(addr, bits);
        }
    }

    // ======== Field Accessors ========

    /// **RESERVED** [31:1]: RESERVED
    #[inline(always)]
    pub fn reserved(&self) -> u32 {
        (self.read_bits() & Self::RESERVED_MASK) >> Self::RESERVED_OFFSET
    }

    /// **RND_SRC_EN** [0:0]: * 1'b1 - entropy source is enabled. *1'b0 - entropy source is disabled
    #[inline(always)]
    pub fn rnd_src_en(&self) -> bool {
        (self.read_bits() & Self::RND_SRC_EN_MASK) != 0
    }

    /// Set RND_SRC_EN bit
    #[inline(always)]
    pub fn set_rnd_src_en(&self) {
        self.set_bits(Self::RND_SRC_EN_MASK);
    }

    /// Clear RND_SRC_EN bit
    #[inline(always)]
    pub fn clr_rnd_src_en(&self) {
        self.clr_bits(Self::RND_SRC_EN_MASK);
    }

}

/// RND_SOURCE_ENABLE value type for batch read/modify/write operations
///
/// Use `rndsourceenable.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rndsourceenable.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RndSourceEnableVal {
    bits: u32,
}

impl RndSourceEnableVal {
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

    /// Read RESERVED [31:1]: RESERVED
    #[inline(always)]
    pub const fn reserved(&self) -> u32 {
        (self.bits & RndSourceEnable::RESERVED_MASK) >> RndSourceEnable::RESERVED_OFFSET
    }

    /// Read RND_SRC_EN [0:0]: * 1'b1 - entropy source is enabled. *1'b0 - entropy source is disabled
    #[inline(always)]
    pub const fn rnd_src_en(&self) -> bool {
        (self.bits & RndSourceEnable::RND_SRC_EN_MASK) != 0
    }

    /// Set RND_SRC_EN [0:0]: * 1'b1 - entropy source is enabled. *1'b0 - entropy source is disabled
    #[inline(always)]
    pub const fn with_rnd_src_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RndSourceEnable::RND_SRC_EN_MASK
            } else {
                self.bits & !RndSourceEnable::RND_SRC_EN_MASK
            }
        }
    }

}

impl Default for RndSourceEnableVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl RndSourceEnable {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> RndSourceEnableVal {
        RndSourceEnableVal::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: RndSourceEnableVal) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(RndSourceEnableVal) -> RndSourceEnableVal>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// SAMPLE_CNT1 Register
///
/// Counts clocks between sampling of random bit.
///
/// Address offset: 0x0130
/// Reset value: 0x0000ffff
///
/// ## Fields
///
/// - **SAMPLE_CNTR1** [31:0] (RW): Sets the number of rng_clk cycles between two consecutive ring oscillator s...
#[repr(transparent)]
pub struct SampleCnt1 {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for SampleCnt1 {}
unsafe impl Sync for SampleCnt1 {}

impl SampleCnt1 {
    // ======== Field Constants ========

    /// SAMPLE_CNTR1 - Sets the number of rng_clk cycles between two consecutive ring oscillator s...
    /// Bits [31:0], mask 0xffffffff
    pub const SAMPLE_CNTR1_OFFSET: u32 = 0;
    pub const SAMPLE_CNTR1_MASK: u32 = 0xffffffff;

    // ======== Basic Access Methods ========

    /// Read the raw register value as u32
    #[inline(always)]
    pub fn read_bits(&self) -> u32 {
        unsafe { core::ptr::read_volatile(self.value.get()) }
    }

    /// Write a raw u32 value to the register
    #[inline(always)]
    pub fn write_bits(&self, value: u32) {
        unsafe { core::ptr::write_volatile(self.value.get(), value) }
    }

    /// Modify the raw register value
    #[inline(always)]
    pub fn modify_bits<F: FnOnce(u32) -> u32>(&self, f: F) {
        self.write_bits(f(self.read_bits()));
    }

    /// Atomic SET - sets bits that are 1 (hardware alias at +0x2000)
    #[inline(always)]
    pub fn set_bits(&self, bits: u32) {
        unsafe {
            let addr = (self.value.get() as usize + 0x2000) as *mut u32;
            core::ptr::write_volatile(addr, bits);
        }
    }

    /// Atomic CLR - clears bits that are 1 (hardware alias at +0x3000)
    #[inline(always)]
    pub fn clr_bits(&self, bits: u32) {
        unsafe {
            let addr = (self.value.get() as usize + 0x3000) as *mut u32;
            core::ptr::write_volatile(addr, bits);
        }
    }

    /// Atomic XOR - toggles bits that are 1 (hardware alias at +0x1000)
    #[inline(always)]
    pub fn xor_bits(&self, bits: u32) {
        unsafe {
            let addr = (self.value.get() as usize + 0x1000) as *mut u32;
            core::ptr::write_volatile(addr, bits);
        }
    }

    // ======== Field Accessors ========

    /// **SAMPLE_CNTR1** [31:0]: Sets the number of rng_clk cycles between two consecutive ring oscillator s...
    #[inline(always)]
    pub fn sample_cntr1(&self) -> u32 {
        (self.read_bits() & Self::SAMPLE_CNTR1_MASK) >> Self::SAMPLE_CNTR1_OFFSET
    }

    /// Write SAMPLE_CNTR1 field
    #[inline(always)]
    pub fn write_sample_cntr1(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::SAMPLE_CNTR1_MASK) | ((value << Self::SAMPLE_CNTR1_OFFSET) & Self::SAMPLE_CNTR1_MASK));
    }

}

/// SAMPLE_CNT1 value type for batch read/modify/write operations
///
/// Use `samplecnt1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `samplecnt1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SampleCnt1Val {
    bits: u32,
}

impl SampleCnt1Val {
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
        Self { bits: 0x0000ffff }
    }

    /// Read SAMPLE_CNTR1 [31:0]: Sets the number of rng_clk cycles between two consecutive ring oscillator s...
    #[inline(always)]
    pub const fn sample_cntr1(&self) -> u32 {
        (self.bits & SampleCnt1::SAMPLE_CNTR1_MASK) >> SampleCnt1::SAMPLE_CNTR1_OFFSET
    }

    /// Set SAMPLE_CNTR1 [31:0]: Sets the number of rng_clk cycles between two consecutive ring oscillator s...
    #[inline(always)]
    pub const fn with_sample_cntr1(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SampleCnt1::SAMPLE_CNTR1_MASK) | ((value << SampleCnt1::SAMPLE_CNTR1_OFFSET) & SampleCnt1::SAMPLE_CNTR1_MASK)
        }
    }

}

impl Default for SampleCnt1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SampleCnt1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> SampleCnt1Val {
        SampleCnt1Val::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: SampleCnt1Val) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(SampleCnt1Val) -> SampleCnt1Val>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// AUTOCORR_STATISTIC Register
///
/// Statistic about Autocorrelation test activations.
///
/// Address offset: 0x0134
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **RESERVED** [31:22] (RO): RESERVED
/// - **AUTOCORR_FAILS** [21:14] (RW): Count each time an autocorrelation test fails. Any write to the register re...
/// - **AUTOCORR_TRYS** [13:0] (RW): Count each time an autocorrelation test starts. Any write to the register r...
#[repr(transparent)]
pub struct AutocorrStatistic {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for AutocorrStatistic {}
unsafe impl Sync for AutocorrStatistic {}

impl AutocorrStatistic {
    // ======== Field Constants ========

    /// RESERVED - RESERVED
    /// Bits [31:22], mask 0xffc00000
    pub const RESERVED_OFFSET: u32 = 22;
    pub const RESERVED_MASK: u32 = 0xffc00000;

    /// AUTOCORR_FAILS - Count each time an autocorrelation test fails. Any write to the register re...
    /// Bits [21:14], mask 0x003fc000
    pub const AUTOCORR_FAILS_OFFSET: u32 = 14;
    pub const AUTOCORR_FAILS_MASK: u32 = 0x003fc000;

    /// AUTOCORR_TRYS - Count each time an autocorrelation test starts. Any write to the register r...
    /// Bits [13:0], mask 0x00003fff
    pub const AUTOCORR_TRYS_OFFSET: u32 = 0;
    pub const AUTOCORR_TRYS_MASK: u32 = 0x00003fff;

    // ======== Basic Access Methods ========

    /// Read the raw register value as u32
    #[inline(always)]
    pub fn read_bits(&self) -> u32 {
        unsafe { core::ptr::read_volatile(self.value.get()) }
    }

    /// Write a raw u32 value to the register
    #[inline(always)]
    pub fn write_bits(&self, value: u32) {
        unsafe { core::ptr::write_volatile(self.value.get(), value) }
    }

    /// Modify the raw register value
    #[inline(always)]
    pub fn modify_bits<F: FnOnce(u32) -> u32>(&self, f: F) {
        self.write_bits(f(self.read_bits()));
    }

    /// Atomic SET - sets bits that are 1 (hardware alias at +0x2000)
    #[inline(always)]
    pub fn set_bits(&self, bits: u32) {
        unsafe {
            let addr = (self.value.get() as usize + 0x2000) as *mut u32;
            core::ptr::write_volatile(addr, bits);
        }
    }

    /// Atomic CLR - clears bits that are 1 (hardware alias at +0x3000)
    #[inline(always)]
    pub fn clr_bits(&self, bits: u32) {
        unsafe {
            let addr = (self.value.get() as usize + 0x3000) as *mut u32;
            core::ptr::write_volatile(addr, bits);
        }
    }

    /// Atomic XOR - toggles bits that are 1 (hardware alias at +0x1000)
    #[inline(always)]
    pub fn xor_bits(&self, bits: u32) {
        unsafe {
            let addr = (self.value.get() as usize + 0x1000) as *mut u32;
            core::ptr::write_volatile(addr, bits);
        }
    }

    // ======== Field Accessors ========

    /// **RESERVED** [31:22]: RESERVED
    #[inline(always)]
    pub fn reserved(&self) -> u32 {
        (self.read_bits() & Self::RESERVED_MASK) >> Self::RESERVED_OFFSET
    }

    /// **AUTOCORR_FAILS** [21:14]: Count each time an autocorrelation test fails. Any write to the register re...
    #[inline(always)]
    pub fn autocorr_fails(&self) -> u32 {
        (self.read_bits() & Self::AUTOCORR_FAILS_MASK) >> Self::AUTOCORR_FAILS_OFFSET
    }

    /// Write AUTOCORR_FAILS field
    #[inline(always)]
    pub fn write_autocorr_fails(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::AUTOCORR_FAILS_MASK) | ((value << Self::AUTOCORR_FAILS_OFFSET) & Self::AUTOCORR_FAILS_MASK));
    }

    /// **AUTOCORR_TRYS** [13:0]: Count each time an autocorrelation test starts. Any write to the register r...
    #[inline(always)]
    pub fn autocorr_trys(&self) -> u32 {
        (self.read_bits() & Self::AUTOCORR_TRYS_MASK) >> Self::AUTOCORR_TRYS_OFFSET
    }

    /// Write AUTOCORR_TRYS field
    #[inline(always)]
    pub fn write_autocorr_trys(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::AUTOCORR_TRYS_MASK) | ((value << Self::AUTOCORR_TRYS_OFFSET) & Self::AUTOCORR_TRYS_MASK));
    }

}

/// AUTOCORR_STATISTIC value type for batch read/modify/write operations
///
/// Use `autocorrstatistic.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `autocorrstatistic.write_val(val)`.
#[derive(Clone, Copy)]
pub struct AutocorrStatisticVal {
    bits: u32,
}

impl AutocorrStatisticVal {
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

    /// Read RESERVED [31:22]: RESERVED
    #[inline(always)]
    pub const fn reserved(&self) -> u32 {
        (self.bits & AutocorrStatistic::RESERVED_MASK) >> AutocorrStatistic::RESERVED_OFFSET
    }

    /// Read AUTOCORR_FAILS [21:14]: Count each time an autocorrelation test fails. Any write to the register re...
    #[inline(always)]
    pub const fn autocorr_fails(&self) -> u32 {
        (self.bits & AutocorrStatistic::AUTOCORR_FAILS_MASK) >> AutocorrStatistic::AUTOCORR_FAILS_OFFSET
    }

    /// Read AUTOCORR_TRYS [13:0]: Count each time an autocorrelation test starts. Any write to the register r...
    #[inline(always)]
    pub const fn autocorr_trys(&self) -> u32 {
        (self.bits & AutocorrStatistic::AUTOCORR_TRYS_MASK) >> AutocorrStatistic::AUTOCORR_TRYS_OFFSET
    }

    /// Set AUTOCORR_FAILS [21:14]: Count each time an autocorrelation test fails. Any write to the register re...
    #[inline(always)]
    pub const fn with_autocorr_fails(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !AutocorrStatistic::AUTOCORR_FAILS_MASK) | ((value << AutocorrStatistic::AUTOCORR_FAILS_OFFSET) & AutocorrStatistic::AUTOCORR_FAILS_MASK)
        }
    }

    /// Set AUTOCORR_TRYS [13:0]: Count each time an autocorrelation test starts. Any write to the register r...
    #[inline(always)]
    pub const fn with_autocorr_trys(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !AutocorrStatistic::AUTOCORR_TRYS_MASK) | ((value << AutocorrStatistic::AUTOCORR_TRYS_OFFSET) & AutocorrStatistic::AUTOCORR_TRYS_MASK)
        }
    }

}

impl Default for AutocorrStatisticVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl AutocorrStatistic {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> AutocorrStatisticVal {
        AutocorrStatisticVal::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: AutocorrStatisticVal) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(AutocorrStatisticVal) -> AutocorrStatisticVal>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// TRNG_DEBUG_CONTROL Register
///
/// Debug register.
///
/// Address offset: 0x0138
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **AUTO_CORRELATE_BYPASS** [3] (RW): When set, the autocorrelation test in the TRNG module is bypassed.
/// - **TRNG_CRNGT_BYPASS** [2] (RW): When set, the CRNGT test in the RNG is bypassed.
/// - **VNC_BYPASS** [1] (RW): When set, the Von-Neuman balancer is bypassed (including the 32 consecutive...
/// - **RESERVED** [0] (RO): N/A
#[repr(transparent)]
pub struct TrngDebugControl {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for TrngDebugControl {}
unsafe impl Sync for TrngDebugControl {}

impl TrngDebugControl {
    // ======== Field Constants ========

    /// AUTO_CORRELATE_BYPASS - When set, the autocorrelation test in the TRNG module is bypassed.
    /// Bits [3:3], mask 0x00000008
    pub const AUTO_CORRELATE_BYPASS_OFFSET: u32 = 3;
    pub const AUTO_CORRELATE_BYPASS_MASK: u32 = 0x00000008;
    pub const AUTO_CORRELATE_BYPASS: u32 = 1 << 3;

    /// TRNG_CRNGT_BYPASS - When set, the CRNGT test in the RNG is bypassed.
    /// Bits [2:2], mask 0x00000004
    pub const TRNG_CRNGT_BYPASS_OFFSET: u32 = 2;
    pub const TRNG_CRNGT_BYPASS_MASK: u32 = 0x00000004;
    pub const TRNG_CRNGT_BYPASS: u32 = 1 << 2;

    /// VNC_BYPASS - When set, the Von-Neuman balancer is bypassed (including the 32 consecutive...
    /// Bits [1:1], mask 0x00000002
    pub const VNC_BYPASS_OFFSET: u32 = 1;
    pub const VNC_BYPASS_MASK: u32 = 0x00000002;
    pub const VNC_BYPASS: u32 = 1 << 1;

    /// RESERVED - N/A
    /// Bits [0:0], mask 0x00000001
    pub const RESERVED_OFFSET: u32 = 0;
    pub const RESERVED_MASK: u32 = 0x00000001;
    pub const RESERVED: u32 = 1 << 0;

    // ======== Basic Access Methods ========

    /// Read the raw register value as u32
    #[inline(always)]
    pub fn read_bits(&self) -> u32 {
        unsafe { core::ptr::read_volatile(self.value.get()) }
    }

    /// Write a raw u32 value to the register
    #[inline(always)]
    pub fn write_bits(&self, value: u32) {
        unsafe { core::ptr::write_volatile(self.value.get(), value) }
    }

    /// Modify the raw register value
    #[inline(always)]
    pub fn modify_bits<F: FnOnce(u32) -> u32>(&self, f: F) {
        self.write_bits(f(self.read_bits()));
    }

    /// Atomic SET - sets bits that are 1 (hardware alias at +0x2000)
    #[inline(always)]
    pub fn set_bits(&self, bits: u32) {
        unsafe {
            let addr = (self.value.get() as usize + 0x2000) as *mut u32;
            core::ptr::write_volatile(addr, bits);
        }
    }

    /// Atomic CLR - clears bits that are 1 (hardware alias at +0x3000)
    #[inline(always)]
    pub fn clr_bits(&self, bits: u32) {
        unsafe {
            let addr = (self.value.get() as usize + 0x3000) as *mut u32;
            core::ptr::write_volatile(addr, bits);
        }
    }

    /// Atomic XOR - toggles bits that are 1 (hardware alias at +0x1000)
    #[inline(always)]
    pub fn xor_bits(&self, bits: u32) {
        unsafe {
            let addr = (self.value.get() as usize + 0x1000) as *mut u32;
            core::ptr::write_volatile(addr, bits);
        }
    }

    // ======== Field Accessors ========

    /// **AUTO_CORRELATE_BYPASS** [3:3]: When set, the autocorrelation test in the TRNG module is bypassed.
    #[inline(always)]
    pub fn auto_correlate_bypass(&self) -> bool {
        (self.read_bits() & Self::AUTO_CORRELATE_BYPASS_MASK) != 0
    }

    /// Set AUTO_CORRELATE_BYPASS bit
    #[inline(always)]
    pub fn set_auto_correlate_bypass(&self) {
        self.set_bits(Self::AUTO_CORRELATE_BYPASS_MASK);
    }

    /// Clear AUTO_CORRELATE_BYPASS bit
    #[inline(always)]
    pub fn clr_auto_correlate_bypass(&self) {
        self.clr_bits(Self::AUTO_CORRELATE_BYPASS_MASK);
    }

    /// **TRNG_CRNGT_BYPASS** [2:2]: When set, the CRNGT test in the RNG is bypassed.
    #[inline(always)]
    pub fn trng_crngt_bypass(&self) -> bool {
        (self.read_bits() & Self::TRNG_CRNGT_BYPASS_MASK) != 0
    }

    /// Set TRNG_CRNGT_BYPASS bit
    #[inline(always)]
    pub fn set_trng_crngt_bypass(&self) {
        self.set_bits(Self::TRNG_CRNGT_BYPASS_MASK);
    }

    /// Clear TRNG_CRNGT_BYPASS bit
    #[inline(always)]
    pub fn clr_trng_crngt_bypass(&self) {
        self.clr_bits(Self::TRNG_CRNGT_BYPASS_MASK);
    }

    /// **VNC_BYPASS** [1:1]: When set, the Von-Neuman balancer is bypassed (including the 32 consecutive...
    #[inline(always)]
    pub fn vnc_bypass(&self) -> bool {
        (self.read_bits() & Self::VNC_BYPASS_MASK) != 0
    }

    /// Set VNC_BYPASS bit
    #[inline(always)]
    pub fn set_vnc_bypass(&self) {
        self.set_bits(Self::VNC_BYPASS_MASK);
    }

    /// Clear VNC_BYPASS bit
    #[inline(always)]
    pub fn clr_vnc_bypass(&self) {
        self.clr_bits(Self::VNC_BYPASS_MASK);
    }

    /// **RESERVED** [0:0]: N/A
    #[inline(always)]
    pub fn reserved(&self) -> bool {
        (self.read_bits() & Self::RESERVED_MASK) != 0
    }

}

/// TRNG_DEBUG_CONTROL value type for batch read/modify/write operations
///
/// Use `trngdebugcontrol.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `trngdebugcontrol.write_val(val)`.
#[derive(Clone, Copy)]
pub struct TrngDebugControlVal {
    bits: u32,
}

impl TrngDebugControlVal {
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

    /// Read AUTO_CORRELATE_BYPASS [3:3]: When set, the autocorrelation test in the TRNG module is bypassed.
    #[inline(always)]
    pub const fn auto_correlate_bypass(&self) -> bool {
        (self.bits & TrngDebugControl::AUTO_CORRELATE_BYPASS_MASK) != 0
    }

    /// Read TRNG_CRNGT_BYPASS [2:2]: When set, the CRNGT test in the RNG is bypassed.
    #[inline(always)]
    pub const fn trng_crngt_bypass(&self) -> bool {
        (self.bits & TrngDebugControl::TRNG_CRNGT_BYPASS_MASK) != 0
    }

    /// Read VNC_BYPASS [1:1]: When set, the Von-Neuman balancer is bypassed (including the 32 consecutive...
    #[inline(always)]
    pub const fn vnc_bypass(&self) -> bool {
        (self.bits & TrngDebugControl::VNC_BYPASS_MASK) != 0
    }

    /// Read RESERVED [0:0]: N/A
    #[inline(always)]
    pub const fn reserved(&self) -> bool {
        (self.bits & TrngDebugControl::RESERVED_MASK) != 0
    }

    /// Set AUTO_CORRELATE_BYPASS [3:3]: When set, the autocorrelation test in the TRNG module is bypassed.
    #[inline(always)]
    pub const fn with_auto_correlate_bypass(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TrngDebugControl::AUTO_CORRELATE_BYPASS_MASK
            } else {
                self.bits & !TrngDebugControl::AUTO_CORRELATE_BYPASS_MASK
            }
        }
    }

    /// Set TRNG_CRNGT_BYPASS [2:2]: When set, the CRNGT test in the RNG is bypassed.
    #[inline(always)]
    pub const fn with_trng_crngt_bypass(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TrngDebugControl::TRNG_CRNGT_BYPASS_MASK
            } else {
                self.bits & !TrngDebugControl::TRNG_CRNGT_BYPASS_MASK
            }
        }
    }

    /// Set VNC_BYPASS [1:1]: When set, the Von-Neuman balancer is bypassed (including the 32 consecutive...
    #[inline(always)]
    pub const fn with_vnc_bypass(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TrngDebugControl::VNC_BYPASS_MASK
            } else {
                self.bits & !TrngDebugControl::VNC_BYPASS_MASK
            }
        }
    }

}

impl Default for TrngDebugControlVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl TrngDebugControl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> TrngDebugControlVal {
        TrngDebugControlVal::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: TrngDebugControlVal) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(TrngDebugControlVal) -> TrngDebugControlVal>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// TRNG_SW_RESET Register
///
/// Generate internal SW reset within the RNG block.
///
/// Address offset: 0x0140
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **RESERVED** [31:1] (RO): RESERVED
/// - **TRNG_SW_RESET** [0] (RW): Writing 1'b1 to this register causes an internal RNG reset.
#[repr(transparent)]
pub struct TrngSwReset {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for TrngSwReset {}
unsafe impl Sync for TrngSwReset {}

impl TrngSwReset {
    // ======== Field Constants ========

    /// RESERVED - RESERVED
    /// Bits [31:1], mask 0xfffffffe
    pub const RESERVED_OFFSET: u32 = 1;
    pub const RESERVED_MASK: u32 = 0xfffffffe;

    /// TRNG_SW_RESET - Writing 1'b1 to this register causes an internal RNG reset.
    /// Bits [0:0], mask 0x00000001
    pub const TRNG_SW_RESET_OFFSET: u32 = 0;
    pub const TRNG_SW_RESET_MASK: u32 = 0x00000001;
    pub const TRNG_SW_RESET: u32 = 1 << 0;

    // ======== Basic Access Methods ========

    /// Read the raw register value as u32
    #[inline(always)]
    pub fn read_bits(&self) -> u32 {
        unsafe { core::ptr::read_volatile(self.value.get()) }
    }

    /// Write a raw u32 value to the register
    #[inline(always)]
    pub fn write_bits(&self, value: u32) {
        unsafe { core::ptr::write_volatile(self.value.get(), value) }
    }

    /// Modify the raw register value
    #[inline(always)]
    pub fn modify_bits<F: FnOnce(u32) -> u32>(&self, f: F) {
        self.write_bits(f(self.read_bits()));
    }

    /// Atomic SET - sets bits that are 1 (hardware alias at +0x2000)
    #[inline(always)]
    pub fn set_bits(&self, bits: u32) {
        unsafe {
            let addr = (self.value.get() as usize + 0x2000) as *mut u32;
            core::ptr::write_volatile(addr, bits);
        }
    }

    /// Atomic CLR - clears bits that are 1 (hardware alias at +0x3000)
    #[inline(always)]
    pub fn clr_bits(&self, bits: u32) {
        unsafe {
            let addr = (self.value.get() as usize + 0x3000) as *mut u32;
            core::ptr::write_volatile(addr, bits);
        }
    }

    /// Atomic XOR - toggles bits that are 1 (hardware alias at +0x1000)
    #[inline(always)]
    pub fn xor_bits(&self, bits: u32) {
        unsafe {
            let addr = (self.value.get() as usize + 0x1000) as *mut u32;
            core::ptr::write_volatile(addr, bits);
        }
    }

    // ======== Field Accessors ========

    /// **RESERVED** [31:1]: RESERVED
    #[inline(always)]
    pub fn reserved(&self) -> u32 {
        (self.read_bits() & Self::RESERVED_MASK) >> Self::RESERVED_OFFSET
    }

    /// **TRNG_SW_RESET** [0:0]: Writing 1'b1 to this register causes an internal RNG reset.
    #[inline(always)]
    pub fn trng_sw_reset(&self) -> bool {
        (self.read_bits() & Self::TRNG_SW_RESET_MASK) != 0
    }

    /// Set TRNG_SW_RESET bit
    #[inline(always)]
    pub fn set_trng_sw_reset(&self) {
        self.set_bits(Self::TRNG_SW_RESET_MASK);
    }

    /// Clear TRNG_SW_RESET bit
    #[inline(always)]
    pub fn clr_trng_sw_reset(&self) {
        self.clr_bits(Self::TRNG_SW_RESET_MASK);
    }

}

/// TRNG_SW_RESET value type for batch read/modify/write operations
///
/// Use `trngswreset.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `trngswreset.write_val(val)`.
#[derive(Clone, Copy)]
pub struct TrngSwResetVal {
    bits: u32,
}

impl TrngSwResetVal {
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

    /// Read RESERVED [31:1]: RESERVED
    #[inline(always)]
    pub const fn reserved(&self) -> u32 {
        (self.bits & TrngSwReset::RESERVED_MASK) >> TrngSwReset::RESERVED_OFFSET
    }

    /// Read TRNG_SW_RESET [0:0]: Writing 1'b1 to this register causes an internal RNG reset.
    #[inline(always)]
    pub const fn trng_sw_reset(&self) -> bool {
        (self.bits & TrngSwReset::TRNG_SW_RESET_MASK) != 0
    }

    /// Set TRNG_SW_RESET [0:0]: Writing 1'b1 to this register causes an internal RNG reset.
    #[inline(always)]
    pub const fn with_trng_sw_reset(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TrngSwReset::TRNG_SW_RESET_MASK
            } else {
                self.bits & !TrngSwReset::TRNG_SW_RESET_MASK
            }
        }
    }

}

impl Default for TrngSwResetVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl TrngSwReset {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> TrngSwResetVal {
        TrngSwResetVal::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: TrngSwResetVal) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(TrngSwResetVal) -> TrngSwResetVal>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// RNG_DEBUG_EN_INPUT Register
///
/// Enable the RNG debug mode
///
/// Address offset: 0x01b4
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **RESERVED** [31:1] (RO): RESERVED
/// - **RNG_DEBUG_EN** [0] (RW): * 1'b1 - debug mode is enabled. *1'b0 - debug mode is disabled
#[repr(transparent)]
pub struct RngDebugEnInput {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for RngDebugEnInput {}
unsafe impl Sync for RngDebugEnInput {}

impl RngDebugEnInput {
    // ======== Field Constants ========

    /// RESERVED - RESERVED
    /// Bits [31:1], mask 0xfffffffe
    pub const RESERVED_OFFSET: u32 = 1;
    pub const RESERVED_MASK: u32 = 0xfffffffe;

    /// RNG_DEBUG_EN - * 1'b1 - debug mode is enabled. *1'b0 - debug mode is disabled
    /// Bits [0:0], mask 0x00000001
    pub const RNG_DEBUG_EN_OFFSET: u32 = 0;
    pub const RNG_DEBUG_EN_MASK: u32 = 0x00000001;
    pub const RNG_DEBUG_EN: u32 = 1 << 0;

    // ======== Basic Access Methods ========

    /// Read the raw register value as u32
    #[inline(always)]
    pub fn read_bits(&self) -> u32 {
        unsafe { core::ptr::read_volatile(self.value.get()) }
    }

    /// Write a raw u32 value to the register
    #[inline(always)]
    pub fn write_bits(&self, value: u32) {
        unsafe { core::ptr::write_volatile(self.value.get(), value) }
    }

    /// Modify the raw register value
    #[inline(always)]
    pub fn modify_bits<F: FnOnce(u32) -> u32>(&self, f: F) {
        self.write_bits(f(self.read_bits()));
    }

    /// Atomic SET - sets bits that are 1 (hardware alias at +0x2000)
    #[inline(always)]
    pub fn set_bits(&self, bits: u32) {
        unsafe {
            let addr = (self.value.get() as usize + 0x2000) as *mut u32;
            core::ptr::write_volatile(addr, bits);
        }
    }

    /// Atomic CLR - clears bits that are 1 (hardware alias at +0x3000)
    #[inline(always)]
    pub fn clr_bits(&self, bits: u32) {
        unsafe {
            let addr = (self.value.get() as usize + 0x3000) as *mut u32;
            core::ptr::write_volatile(addr, bits);
        }
    }

    /// Atomic XOR - toggles bits that are 1 (hardware alias at +0x1000)
    #[inline(always)]
    pub fn xor_bits(&self, bits: u32) {
        unsafe {
            let addr = (self.value.get() as usize + 0x1000) as *mut u32;
            core::ptr::write_volatile(addr, bits);
        }
    }

    // ======== Field Accessors ========

    /// **RESERVED** [31:1]: RESERVED
    #[inline(always)]
    pub fn reserved(&self) -> u32 {
        (self.read_bits() & Self::RESERVED_MASK) >> Self::RESERVED_OFFSET
    }

    /// **RNG_DEBUG_EN** [0:0]: * 1'b1 - debug mode is enabled. *1'b0 - debug mode is disabled
    #[inline(always)]
    pub fn rng_debug_en(&self) -> bool {
        (self.read_bits() & Self::RNG_DEBUG_EN_MASK) != 0
    }

    /// Set RNG_DEBUG_EN bit
    #[inline(always)]
    pub fn set_rng_debug_en(&self) {
        self.set_bits(Self::RNG_DEBUG_EN_MASK);
    }

    /// Clear RNG_DEBUG_EN bit
    #[inline(always)]
    pub fn clr_rng_debug_en(&self) {
        self.clr_bits(Self::RNG_DEBUG_EN_MASK);
    }

}

/// RNG_DEBUG_EN_INPUT value type for batch read/modify/write operations
///
/// Use `rngdebugeninput.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rngdebugeninput.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RngDebugEnInputVal {
    bits: u32,
}

impl RngDebugEnInputVal {
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

    /// Read RESERVED [31:1]: RESERVED
    #[inline(always)]
    pub const fn reserved(&self) -> u32 {
        (self.bits & RngDebugEnInput::RESERVED_MASK) >> RngDebugEnInput::RESERVED_OFFSET
    }

    /// Read RNG_DEBUG_EN [0:0]: * 1'b1 - debug mode is enabled. *1'b0 - debug mode is disabled
    #[inline(always)]
    pub const fn rng_debug_en(&self) -> bool {
        (self.bits & RngDebugEnInput::RNG_DEBUG_EN_MASK) != 0
    }

    /// Set RNG_DEBUG_EN [0:0]: * 1'b1 - debug mode is enabled. *1'b0 - debug mode is disabled
    #[inline(always)]
    pub const fn with_rng_debug_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RngDebugEnInput::RNG_DEBUG_EN_MASK
            } else {
                self.bits & !RngDebugEnInput::RNG_DEBUG_EN_MASK
            }
        }
    }

}

impl Default for RngDebugEnInputVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl RngDebugEnInput {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> RngDebugEnInputVal {
        RngDebugEnInputVal::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: RngDebugEnInputVal) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(RngDebugEnInputVal) -> RngDebugEnInputVal>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// TRNG_BUSY Register
///
/// RNG Busy indication.
///
/// Address offset: 0x01b8
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **RESERVED** [31:1] (RO): RESERVED
/// - **TRNG_BUSY** [0] (RO): Reflects rng_busy status.
#[repr(transparent)]
pub struct TrngBusy {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for TrngBusy {}
unsafe impl Sync for TrngBusy {}

impl TrngBusy {
    // ======== Field Constants ========

    /// RESERVED - RESERVED
    /// Bits [31:1], mask 0xfffffffe
    pub const RESERVED_OFFSET: u32 = 1;
    pub const RESERVED_MASK: u32 = 0xfffffffe;

    /// TRNG_BUSY - Reflects rng_busy status.
    /// Bits [0:0], mask 0x00000001
    pub const TRNG_BUSY_OFFSET: u32 = 0;
    pub const TRNG_BUSY_MASK: u32 = 0x00000001;
    pub const TRNG_BUSY: u32 = 1 << 0;

    // ======== Basic Access Methods ========

    /// Read the raw register value as u32
    #[inline(always)]
    pub fn read_bits(&self) -> u32 {
        unsafe { core::ptr::read_volatile(self.value.get()) }
    }

    /// Write a raw u32 value to the register
    #[inline(always)]
    pub fn write_bits(&self, value: u32) {
        unsafe { core::ptr::write_volatile(self.value.get(), value) }
    }

    /// Modify the raw register value
    #[inline(always)]
    pub fn modify_bits<F: FnOnce(u32) -> u32>(&self, f: F) {
        self.write_bits(f(self.read_bits()));
    }

    /// Atomic SET - sets bits that are 1 (hardware alias at +0x2000)
    #[inline(always)]
    pub fn set_bits(&self, bits: u32) {
        unsafe {
            let addr = (self.value.get() as usize + 0x2000) as *mut u32;
            core::ptr::write_volatile(addr, bits);
        }
    }

    /// Atomic CLR - clears bits that are 1 (hardware alias at +0x3000)
    #[inline(always)]
    pub fn clr_bits(&self, bits: u32) {
        unsafe {
            let addr = (self.value.get() as usize + 0x3000) as *mut u32;
            core::ptr::write_volatile(addr, bits);
        }
    }

    /// Atomic XOR - toggles bits that are 1 (hardware alias at +0x1000)
    #[inline(always)]
    pub fn xor_bits(&self, bits: u32) {
        unsafe {
            let addr = (self.value.get() as usize + 0x1000) as *mut u32;
            core::ptr::write_volatile(addr, bits);
        }
    }

    // ======== Field Accessors ========

    /// **RESERVED** [31:1]: RESERVED
    #[inline(always)]
    pub fn reserved(&self) -> u32 {
        (self.read_bits() & Self::RESERVED_MASK) >> Self::RESERVED_OFFSET
    }

    /// **TRNG_BUSY** [0:0]: Reflects rng_busy status.
    #[inline(always)]
    pub fn trng_busy(&self) -> bool {
        (self.read_bits() & Self::TRNG_BUSY_MASK) != 0
    }

}

/// TRNG_BUSY value type for batch read/modify/write operations
///
/// Use `trngbusy.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `trngbusy.write_val(val)`.
#[derive(Clone, Copy)]
pub struct TrngBusyVal {
    bits: u32,
}

impl TrngBusyVal {
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

    /// Read RESERVED [31:1]: RESERVED
    #[inline(always)]
    pub const fn reserved(&self) -> u32 {
        (self.bits & TrngBusy::RESERVED_MASK) >> TrngBusy::RESERVED_OFFSET
    }

    /// Read TRNG_BUSY [0:0]: Reflects rng_busy status.
    #[inline(always)]
    pub const fn trng_busy(&self) -> bool {
        (self.bits & TrngBusy::TRNG_BUSY_MASK) != 0
    }

}

impl Default for TrngBusyVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl TrngBusy {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> TrngBusyVal {
        TrngBusyVal::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: TrngBusyVal) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(TrngBusyVal) -> TrngBusyVal>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// RST_BITS_COUNTER Register
///
/// Reset the counter of collected bits in the RNG.
///
/// Address offset: 0x01bc
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **RESERVED** [31:1] (RO): RESERVED
/// - **RST_BITS_COUNTER** [0] (RW): Writing any value to this address will reset the bits counter and RNG valid...
#[repr(transparent)]
pub struct RstBitsCounter {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for RstBitsCounter {}
unsafe impl Sync for RstBitsCounter {}

impl RstBitsCounter {
    // ======== Field Constants ========

    /// RESERVED - RESERVED
    /// Bits [31:1], mask 0xfffffffe
    pub const RESERVED_OFFSET: u32 = 1;
    pub const RESERVED_MASK: u32 = 0xfffffffe;

    /// RST_BITS_COUNTER - Writing any value to this address will reset the bits counter and RNG valid...
    /// Bits [0:0], mask 0x00000001
    pub const RST_BITS_COUNTER_OFFSET: u32 = 0;
    pub const RST_BITS_COUNTER_MASK: u32 = 0x00000001;
    pub const RST_BITS_COUNTER: u32 = 1 << 0;

    // ======== Basic Access Methods ========

    /// Read the raw register value as u32
    #[inline(always)]
    pub fn read_bits(&self) -> u32 {
        unsafe { core::ptr::read_volatile(self.value.get()) }
    }

    /// Write a raw u32 value to the register
    #[inline(always)]
    pub fn write_bits(&self, value: u32) {
        unsafe { core::ptr::write_volatile(self.value.get(), value) }
    }

    /// Modify the raw register value
    #[inline(always)]
    pub fn modify_bits<F: FnOnce(u32) -> u32>(&self, f: F) {
        self.write_bits(f(self.read_bits()));
    }

    /// Atomic SET - sets bits that are 1 (hardware alias at +0x2000)
    #[inline(always)]
    pub fn set_bits(&self, bits: u32) {
        unsafe {
            let addr = (self.value.get() as usize + 0x2000) as *mut u32;
            core::ptr::write_volatile(addr, bits);
        }
    }

    /// Atomic CLR - clears bits that are 1 (hardware alias at +0x3000)
    #[inline(always)]
    pub fn clr_bits(&self, bits: u32) {
        unsafe {
            let addr = (self.value.get() as usize + 0x3000) as *mut u32;
            core::ptr::write_volatile(addr, bits);
        }
    }

    /// Atomic XOR - toggles bits that are 1 (hardware alias at +0x1000)
    #[inline(always)]
    pub fn xor_bits(&self, bits: u32) {
        unsafe {
            let addr = (self.value.get() as usize + 0x1000) as *mut u32;
            core::ptr::write_volatile(addr, bits);
        }
    }

    // ======== Field Accessors ========

    /// **RESERVED** [31:1]: RESERVED
    #[inline(always)]
    pub fn reserved(&self) -> u32 {
        (self.read_bits() & Self::RESERVED_MASK) >> Self::RESERVED_OFFSET
    }

    /// **RST_BITS_COUNTER** [0:0]: Writing any value to this address will reset the bits counter and RNG valid...
    #[inline(always)]
    pub fn rst_bits_counter(&self) -> bool {
        (self.read_bits() & Self::RST_BITS_COUNTER_MASK) != 0
    }

    /// Set RST_BITS_COUNTER bit
    #[inline(always)]
    pub fn set_rst_bits_counter(&self) {
        self.set_bits(Self::RST_BITS_COUNTER_MASK);
    }

    /// Clear RST_BITS_COUNTER bit
    #[inline(always)]
    pub fn clr_rst_bits_counter(&self) {
        self.clr_bits(Self::RST_BITS_COUNTER_MASK);
    }

}

/// RST_BITS_COUNTER value type for batch read/modify/write operations
///
/// Use `rstbitscounter.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rstbitscounter.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RstBitsCounterVal {
    bits: u32,
}

impl RstBitsCounterVal {
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

    /// Read RESERVED [31:1]: RESERVED
    #[inline(always)]
    pub const fn reserved(&self) -> u32 {
        (self.bits & RstBitsCounter::RESERVED_MASK) >> RstBitsCounter::RESERVED_OFFSET
    }

    /// Read RST_BITS_COUNTER [0:0]: Writing any value to this address will reset the bits counter and RNG valid...
    #[inline(always)]
    pub const fn rst_bits_counter(&self) -> bool {
        (self.bits & RstBitsCounter::RST_BITS_COUNTER_MASK) != 0
    }

    /// Set RST_BITS_COUNTER [0:0]: Writing any value to this address will reset the bits counter and RNG valid...
    #[inline(always)]
    pub const fn with_rst_bits_counter(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RstBitsCounter::RST_BITS_COUNTER_MASK
            } else {
                self.bits & !RstBitsCounter::RST_BITS_COUNTER_MASK
            }
        }
    }

}

impl Default for RstBitsCounterVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl RstBitsCounter {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> RstBitsCounterVal {
        RstBitsCounterVal::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: RstBitsCounterVal) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(RstBitsCounterVal) -> RstBitsCounterVal>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// RNG_VERSION Register
///
/// Displays the version settings of the TRNG.
///
/// Address offset: 0x01c0
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **RESERVED** [31:8] (RO): RESERVED
/// - **RNG_USE_5_SBOXES** [7] (RO): * 1'b1 - 5 SBOX AES. *1'b0 - 20 SBOX AES
/// - **RESEEDING_EXISTS** [6] (RO): * 1'b1 - Exists. *1'b0 - Does not exist
/// - **KAT_EXISTS** [5] (RO): * 1'b1 - Exists. *1'b0 - Does not exist
/// - **PRNG_EXISTS** [4] (RO): * 1'b1 - Exists. *1'b0 - Does not exist
/// - **TRNG_TESTS_BYPASS_EN** [3] (RO): * 1'b1 - Exists. *1'b0 - Does not exist
/// - **AUTOCORR_EXISTS** [2] (RO): * 1'b1 - Exists. *1'b0 - Does not exist
/// - **CRNGT_EXISTS** [1] (RO): * 1'b1 - Exists. *1'b0 - Does not exist
/// - **EHR_WIDTH_192** [0] (RO): * 1'b1 - 192-bit EHR. *1'b0 - 128-bit EHR
#[repr(transparent)]
pub struct RngVersion {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for RngVersion {}
unsafe impl Sync for RngVersion {}

impl RngVersion {
    // ======== Field Constants ========

    /// RESERVED - RESERVED
    /// Bits [31:8], mask 0xffffff00
    pub const RESERVED_OFFSET: u32 = 8;
    pub const RESERVED_MASK: u32 = 0xffffff00;

    /// RNG_USE_5_SBOXES - * 1'b1 - 5 SBOX AES. *1'b0 - 20 SBOX AES
    /// Bits [7:7], mask 0x00000080
    pub const RNG_USE_5_SBOXES_OFFSET: u32 = 7;
    pub const RNG_USE_5_SBOXES_MASK: u32 = 0x00000080;
    pub const RNG_USE_5_SBOXES: u32 = 1 << 7;

    /// RESEEDING_EXISTS - * 1'b1 - Exists. *1'b0 - Does not exist
    /// Bits [6:6], mask 0x00000040
    pub const RESEEDING_EXISTS_OFFSET: u32 = 6;
    pub const RESEEDING_EXISTS_MASK: u32 = 0x00000040;
    pub const RESEEDING_EXISTS: u32 = 1 << 6;

    /// KAT_EXISTS - * 1'b1 - Exists. *1'b0 - Does not exist
    /// Bits [5:5], mask 0x00000020
    pub const KAT_EXISTS_OFFSET: u32 = 5;
    pub const KAT_EXISTS_MASK: u32 = 0x00000020;
    pub const KAT_EXISTS: u32 = 1 << 5;

    /// PRNG_EXISTS - * 1'b1 - Exists. *1'b0 - Does not exist
    /// Bits [4:4], mask 0x00000010
    pub const PRNG_EXISTS_OFFSET: u32 = 4;
    pub const PRNG_EXISTS_MASK: u32 = 0x00000010;
    pub const PRNG_EXISTS: u32 = 1 << 4;

    /// TRNG_TESTS_BYPASS_EN - * 1'b1 - Exists. *1'b0 - Does not exist
    /// Bits [3:3], mask 0x00000008
    pub const TRNG_TESTS_BYPASS_EN_OFFSET: u32 = 3;
    pub const TRNG_TESTS_BYPASS_EN_MASK: u32 = 0x00000008;
    pub const TRNG_TESTS_BYPASS_EN: u32 = 1 << 3;

    /// AUTOCORR_EXISTS - * 1'b1 - Exists. *1'b0 - Does not exist
    /// Bits [2:2], mask 0x00000004
    pub const AUTOCORR_EXISTS_OFFSET: u32 = 2;
    pub const AUTOCORR_EXISTS_MASK: u32 = 0x00000004;
    pub const AUTOCORR_EXISTS: u32 = 1 << 2;

    /// CRNGT_EXISTS - * 1'b1 - Exists. *1'b0 - Does not exist
    /// Bits [1:1], mask 0x00000002
    pub const CRNGT_EXISTS_OFFSET: u32 = 1;
    pub const CRNGT_EXISTS_MASK: u32 = 0x00000002;
    pub const CRNGT_EXISTS: u32 = 1 << 1;

    /// EHR_WIDTH_192 - * 1'b1 - 192-bit EHR. *1'b0 - 128-bit EHR
    /// Bits [0:0], mask 0x00000001
    pub const EHR_WIDTH_192_OFFSET: u32 = 0;
    pub const EHR_WIDTH_192_MASK: u32 = 0x00000001;
    pub const EHR_WIDTH_192: u32 = 1 << 0;

    // ======== Basic Access Methods ========

    /// Read the raw register value as u32
    #[inline(always)]
    pub fn read_bits(&self) -> u32 {
        unsafe { core::ptr::read_volatile(self.value.get()) }
    }

    /// Write a raw u32 value to the register
    #[inline(always)]
    pub fn write_bits(&self, value: u32) {
        unsafe { core::ptr::write_volatile(self.value.get(), value) }
    }

    /// Modify the raw register value
    #[inline(always)]
    pub fn modify_bits<F: FnOnce(u32) -> u32>(&self, f: F) {
        self.write_bits(f(self.read_bits()));
    }

    /// Atomic SET - sets bits that are 1 (hardware alias at +0x2000)
    #[inline(always)]
    pub fn set_bits(&self, bits: u32) {
        unsafe {
            let addr = (self.value.get() as usize + 0x2000) as *mut u32;
            core::ptr::write_volatile(addr, bits);
        }
    }

    /// Atomic CLR - clears bits that are 1 (hardware alias at +0x3000)
    #[inline(always)]
    pub fn clr_bits(&self, bits: u32) {
        unsafe {
            let addr = (self.value.get() as usize + 0x3000) as *mut u32;
            core::ptr::write_volatile(addr, bits);
        }
    }

    /// Atomic XOR - toggles bits that are 1 (hardware alias at +0x1000)
    #[inline(always)]
    pub fn xor_bits(&self, bits: u32) {
        unsafe {
            let addr = (self.value.get() as usize + 0x1000) as *mut u32;
            core::ptr::write_volatile(addr, bits);
        }
    }

    // ======== Field Accessors ========

    /// **RESERVED** [31:8]: RESERVED
    #[inline(always)]
    pub fn reserved(&self) -> u32 {
        (self.read_bits() & Self::RESERVED_MASK) >> Self::RESERVED_OFFSET
    }

    /// **RNG_USE_5_SBOXES** [7:7]: * 1'b1 - 5 SBOX AES. *1'b0 - 20 SBOX AES
    #[inline(always)]
    pub fn rng_use_5_sboxes(&self) -> bool {
        (self.read_bits() & Self::RNG_USE_5_SBOXES_MASK) != 0
    }

    /// **RESEEDING_EXISTS** [6:6]: * 1'b1 - Exists. *1'b0 - Does not exist
    #[inline(always)]
    pub fn reseeding_exists(&self) -> bool {
        (self.read_bits() & Self::RESEEDING_EXISTS_MASK) != 0
    }

    /// **KAT_EXISTS** [5:5]: * 1'b1 - Exists. *1'b0 - Does not exist
    #[inline(always)]
    pub fn kat_exists(&self) -> bool {
        (self.read_bits() & Self::KAT_EXISTS_MASK) != 0
    }

    /// **PRNG_EXISTS** [4:4]: * 1'b1 - Exists. *1'b0 - Does not exist
    #[inline(always)]
    pub fn prng_exists(&self) -> bool {
        (self.read_bits() & Self::PRNG_EXISTS_MASK) != 0
    }

    /// **TRNG_TESTS_BYPASS_EN** [3:3]: * 1'b1 - Exists. *1'b0 - Does not exist
    #[inline(always)]
    pub fn trng_tests_bypass_en(&self) -> bool {
        (self.read_bits() & Self::TRNG_TESTS_BYPASS_EN_MASK) != 0
    }

    /// **AUTOCORR_EXISTS** [2:2]: * 1'b1 - Exists. *1'b0 - Does not exist
    #[inline(always)]
    pub fn autocorr_exists(&self) -> bool {
        (self.read_bits() & Self::AUTOCORR_EXISTS_MASK) != 0
    }

    /// **CRNGT_EXISTS** [1:1]: * 1'b1 - Exists. *1'b0 - Does not exist
    #[inline(always)]
    pub fn crngt_exists(&self) -> bool {
        (self.read_bits() & Self::CRNGT_EXISTS_MASK) != 0
    }

    /// **EHR_WIDTH_192** [0:0]: * 1'b1 - 192-bit EHR. *1'b0 - 128-bit EHR
    #[inline(always)]
    pub fn ehr_width_192(&self) -> bool {
        (self.read_bits() & Self::EHR_WIDTH_192_MASK) != 0
    }

}

/// RNG_VERSION value type for batch read/modify/write operations
///
/// Use `rngversion.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rngversion.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RngVersionVal {
    bits: u32,
}

impl RngVersionVal {
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

    /// Read RESERVED [31:8]: RESERVED
    #[inline(always)]
    pub const fn reserved(&self) -> u32 {
        (self.bits & RngVersion::RESERVED_MASK) >> RngVersion::RESERVED_OFFSET
    }

    /// Read RNG_USE_5_SBOXES [7:7]: * 1'b1 - 5 SBOX AES. *1'b0 - 20 SBOX AES
    #[inline(always)]
    pub const fn rng_use_5_sboxes(&self) -> bool {
        (self.bits & RngVersion::RNG_USE_5_SBOXES_MASK) != 0
    }

    /// Read RESEEDING_EXISTS [6:6]: * 1'b1 - Exists. *1'b0 - Does not exist
    #[inline(always)]
    pub const fn reseeding_exists(&self) -> bool {
        (self.bits & RngVersion::RESEEDING_EXISTS_MASK) != 0
    }

    /// Read KAT_EXISTS [5:5]: * 1'b1 - Exists. *1'b0 - Does not exist
    #[inline(always)]
    pub const fn kat_exists(&self) -> bool {
        (self.bits & RngVersion::KAT_EXISTS_MASK) != 0
    }

    /// Read PRNG_EXISTS [4:4]: * 1'b1 - Exists. *1'b0 - Does not exist
    #[inline(always)]
    pub const fn prng_exists(&self) -> bool {
        (self.bits & RngVersion::PRNG_EXISTS_MASK) != 0
    }

    /// Read TRNG_TESTS_BYPASS_EN [3:3]: * 1'b1 - Exists. *1'b0 - Does not exist
    #[inline(always)]
    pub const fn trng_tests_bypass_en(&self) -> bool {
        (self.bits & RngVersion::TRNG_TESTS_BYPASS_EN_MASK) != 0
    }

    /// Read AUTOCORR_EXISTS [2:2]: * 1'b1 - Exists. *1'b0 - Does not exist
    #[inline(always)]
    pub const fn autocorr_exists(&self) -> bool {
        (self.bits & RngVersion::AUTOCORR_EXISTS_MASK) != 0
    }

    /// Read CRNGT_EXISTS [1:1]: * 1'b1 - Exists. *1'b0 - Does not exist
    #[inline(always)]
    pub const fn crngt_exists(&self) -> bool {
        (self.bits & RngVersion::CRNGT_EXISTS_MASK) != 0
    }

    /// Read EHR_WIDTH_192 [0:0]: * 1'b1 - 192-bit EHR. *1'b0 - 128-bit EHR
    #[inline(always)]
    pub const fn ehr_width_192(&self) -> bool {
        (self.bits & RngVersion::EHR_WIDTH_192_MASK) != 0
    }

}

impl Default for RngVersionVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl RngVersion {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> RngVersionVal {
        RngVersionVal::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: RngVersionVal) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(RngVersionVal) -> RngVersionVal>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// RNG_BIST_CNTR_0 Register
///
/// Collected BIST results.
///
/// Address offset: 0x01e0
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **RESERVED** [31:22] (RO): RESERVED
/// - **ROSC_CNTR_VAL** [21:0] (RO): Reflects the results of RNG BIST counter.
#[repr(transparent)]
pub struct RngBistCntr0 {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for RngBistCntr0 {}
unsafe impl Sync for RngBistCntr0 {}

impl RngBistCntr0 {
    // ======== Field Constants ========

    /// RESERVED - RESERVED
    /// Bits [31:22], mask 0xffc00000
    pub const RESERVED_OFFSET: u32 = 22;
    pub const RESERVED_MASK: u32 = 0xffc00000;

    /// ROSC_CNTR_VAL - Reflects the results of RNG BIST counter.
    /// Bits [21:0], mask 0x003fffff
    pub const ROSC_CNTR_VAL_OFFSET: u32 = 0;
    pub const ROSC_CNTR_VAL_MASK: u32 = 0x003fffff;

    // ======== Basic Access Methods ========

    /// Read the raw register value as u32
    #[inline(always)]
    pub fn read_bits(&self) -> u32 {
        unsafe { core::ptr::read_volatile(self.value.get()) }
    }

    /// Write a raw u32 value to the register
    #[inline(always)]
    pub fn write_bits(&self, value: u32) {
        unsafe { core::ptr::write_volatile(self.value.get(), value) }
    }

    /// Modify the raw register value
    #[inline(always)]
    pub fn modify_bits<F: FnOnce(u32) -> u32>(&self, f: F) {
        self.write_bits(f(self.read_bits()));
    }

    /// Atomic SET - sets bits that are 1 (hardware alias at +0x2000)
    #[inline(always)]
    pub fn set_bits(&self, bits: u32) {
        unsafe {
            let addr = (self.value.get() as usize + 0x2000) as *mut u32;
            core::ptr::write_volatile(addr, bits);
        }
    }

    /// Atomic CLR - clears bits that are 1 (hardware alias at +0x3000)
    #[inline(always)]
    pub fn clr_bits(&self, bits: u32) {
        unsafe {
            let addr = (self.value.get() as usize + 0x3000) as *mut u32;
            core::ptr::write_volatile(addr, bits);
        }
    }

    /// Atomic XOR - toggles bits that are 1 (hardware alias at +0x1000)
    #[inline(always)]
    pub fn xor_bits(&self, bits: u32) {
        unsafe {
            let addr = (self.value.get() as usize + 0x1000) as *mut u32;
            core::ptr::write_volatile(addr, bits);
        }
    }

    // ======== Field Accessors ========

    /// **RESERVED** [31:22]: RESERVED
    #[inline(always)]
    pub fn reserved(&self) -> u32 {
        (self.read_bits() & Self::RESERVED_MASK) >> Self::RESERVED_OFFSET
    }

    /// **ROSC_CNTR_VAL** [21:0]: Reflects the results of RNG BIST counter.
    #[inline(always)]
    pub fn rosc_cntr_val(&self) -> u32 {
        (self.read_bits() & Self::ROSC_CNTR_VAL_MASK) >> Self::ROSC_CNTR_VAL_OFFSET
    }

}

/// RNG_BIST_CNTR_0 value type for batch read/modify/write operations
///
/// Use `rngbistcntr0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rngbistcntr0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RngBistCntr0Val {
    bits: u32,
}

impl RngBistCntr0Val {
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

    /// Read RESERVED [31:22]: RESERVED
    #[inline(always)]
    pub const fn reserved(&self) -> u32 {
        (self.bits & RngBistCntr0::RESERVED_MASK) >> RngBistCntr0::RESERVED_OFFSET
    }

    /// Read ROSC_CNTR_VAL [21:0]: Reflects the results of RNG BIST counter.
    #[inline(always)]
    pub const fn rosc_cntr_val(&self) -> u32 {
        (self.bits & RngBistCntr0::ROSC_CNTR_VAL_MASK) >> RngBistCntr0::ROSC_CNTR_VAL_OFFSET
    }

}

impl Default for RngBistCntr0Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl RngBistCntr0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> RngBistCntr0Val {
        RngBistCntr0Val::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: RngBistCntr0Val) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(RngBistCntr0Val) -> RngBistCntr0Val>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// RNG_BIST_CNTR_1 Register
///
/// Collected BIST results.
///
/// Address offset: 0x01e4
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **RESERVED** [31:22] (RO): RESERVED
/// - **ROSC_CNTR_VAL** [21:0] (RO): Reflects the results of RNG BIST counter.
#[repr(transparent)]
pub struct RngBistCntr1 {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for RngBistCntr1 {}
unsafe impl Sync for RngBistCntr1 {}

impl RngBistCntr1 {
    // ======== Field Constants ========

    /// RESERVED - RESERVED
    /// Bits [31:22], mask 0xffc00000
    pub const RESERVED_OFFSET: u32 = 22;
    pub const RESERVED_MASK: u32 = 0xffc00000;

    /// ROSC_CNTR_VAL - Reflects the results of RNG BIST counter.
    /// Bits [21:0], mask 0x003fffff
    pub const ROSC_CNTR_VAL_OFFSET: u32 = 0;
    pub const ROSC_CNTR_VAL_MASK: u32 = 0x003fffff;

    // ======== Basic Access Methods ========

    /// Read the raw register value as u32
    #[inline(always)]
    pub fn read_bits(&self) -> u32 {
        unsafe { core::ptr::read_volatile(self.value.get()) }
    }

    /// Write a raw u32 value to the register
    #[inline(always)]
    pub fn write_bits(&self, value: u32) {
        unsafe { core::ptr::write_volatile(self.value.get(), value) }
    }

    /// Modify the raw register value
    #[inline(always)]
    pub fn modify_bits<F: FnOnce(u32) -> u32>(&self, f: F) {
        self.write_bits(f(self.read_bits()));
    }

    /// Atomic SET - sets bits that are 1 (hardware alias at +0x2000)
    #[inline(always)]
    pub fn set_bits(&self, bits: u32) {
        unsafe {
            let addr = (self.value.get() as usize + 0x2000) as *mut u32;
            core::ptr::write_volatile(addr, bits);
        }
    }

    /// Atomic CLR - clears bits that are 1 (hardware alias at +0x3000)
    #[inline(always)]
    pub fn clr_bits(&self, bits: u32) {
        unsafe {
            let addr = (self.value.get() as usize + 0x3000) as *mut u32;
            core::ptr::write_volatile(addr, bits);
        }
    }

    /// Atomic XOR - toggles bits that are 1 (hardware alias at +0x1000)
    #[inline(always)]
    pub fn xor_bits(&self, bits: u32) {
        unsafe {
            let addr = (self.value.get() as usize + 0x1000) as *mut u32;
            core::ptr::write_volatile(addr, bits);
        }
    }

    // ======== Field Accessors ========

    /// **RESERVED** [31:22]: RESERVED
    #[inline(always)]
    pub fn reserved(&self) -> u32 {
        (self.read_bits() & Self::RESERVED_MASK) >> Self::RESERVED_OFFSET
    }

    /// **ROSC_CNTR_VAL** [21:0]: Reflects the results of RNG BIST counter.
    #[inline(always)]
    pub fn rosc_cntr_val(&self) -> u32 {
        (self.read_bits() & Self::ROSC_CNTR_VAL_MASK) >> Self::ROSC_CNTR_VAL_OFFSET
    }

}

/// RNG_BIST_CNTR_1 value type for batch read/modify/write operations
///
/// Use `rngbistcntr1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rngbistcntr1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RngBistCntr1Val {
    bits: u32,
}

impl RngBistCntr1Val {
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

    /// Read RESERVED [31:22]: RESERVED
    #[inline(always)]
    pub const fn reserved(&self) -> u32 {
        (self.bits & RngBistCntr1::RESERVED_MASK) >> RngBistCntr1::RESERVED_OFFSET
    }

    /// Read ROSC_CNTR_VAL [21:0]: Reflects the results of RNG BIST counter.
    #[inline(always)]
    pub const fn rosc_cntr_val(&self) -> u32 {
        (self.bits & RngBistCntr1::ROSC_CNTR_VAL_MASK) >> RngBistCntr1::ROSC_CNTR_VAL_OFFSET
    }

}

impl Default for RngBistCntr1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl RngBistCntr1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> RngBistCntr1Val {
        RngBistCntr1Val::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: RngBistCntr1Val) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(RngBistCntr1Val) -> RngBistCntr1Val>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// RNG_BIST_CNTR_2 Register
///
/// Collected BIST results.
///
/// Address offset: 0x01e8
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **RESERVED** [31:22] (RO): RESERVED
/// - **ROSC_CNTR_VAL** [21:0] (RO): Reflects the results of RNG BIST counter.
#[repr(transparent)]
pub struct RngBistCntr2 {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for RngBistCntr2 {}
unsafe impl Sync for RngBistCntr2 {}

impl RngBistCntr2 {
    // ======== Field Constants ========

    /// RESERVED - RESERVED
    /// Bits [31:22], mask 0xffc00000
    pub const RESERVED_OFFSET: u32 = 22;
    pub const RESERVED_MASK: u32 = 0xffc00000;

    /// ROSC_CNTR_VAL - Reflects the results of RNG BIST counter.
    /// Bits [21:0], mask 0x003fffff
    pub const ROSC_CNTR_VAL_OFFSET: u32 = 0;
    pub const ROSC_CNTR_VAL_MASK: u32 = 0x003fffff;

    // ======== Basic Access Methods ========

    /// Read the raw register value as u32
    #[inline(always)]
    pub fn read_bits(&self) -> u32 {
        unsafe { core::ptr::read_volatile(self.value.get()) }
    }

    /// Write a raw u32 value to the register
    #[inline(always)]
    pub fn write_bits(&self, value: u32) {
        unsafe { core::ptr::write_volatile(self.value.get(), value) }
    }

    /// Modify the raw register value
    #[inline(always)]
    pub fn modify_bits<F: FnOnce(u32) -> u32>(&self, f: F) {
        self.write_bits(f(self.read_bits()));
    }

    /// Atomic SET - sets bits that are 1 (hardware alias at +0x2000)
    #[inline(always)]
    pub fn set_bits(&self, bits: u32) {
        unsafe {
            let addr = (self.value.get() as usize + 0x2000) as *mut u32;
            core::ptr::write_volatile(addr, bits);
        }
    }

    /// Atomic CLR - clears bits that are 1 (hardware alias at +0x3000)
    #[inline(always)]
    pub fn clr_bits(&self, bits: u32) {
        unsafe {
            let addr = (self.value.get() as usize + 0x3000) as *mut u32;
            core::ptr::write_volatile(addr, bits);
        }
    }

    /// Atomic XOR - toggles bits that are 1 (hardware alias at +0x1000)
    #[inline(always)]
    pub fn xor_bits(&self, bits: u32) {
        unsafe {
            let addr = (self.value.get() as usize + 0x1000) as *mut u32;
            core::ptr::write_volatile(addr, bits);
        }
    }

    // ======== Field Accessors ========

    /// **RESERVED** [31:22]: RESERVED
    #[inline(always)]
    pub fn reserved(&self) -> u32 {
        (self.read_bits() & Self::RESERVED_MASK) >> Self::RESERVED_OFFSET
    }

    /// **ROSC_CNTR_VAL** [21:0]: Reflects the results of RNG BIST counter.
    #[inline(always)]
    pub fn rosc_cntr_val(&self) -> u32 {
        (self.read_bits() & Self::ROSC_CNTR_VAL_MASK) >> Self::ROSC_CNTR_VAL_OFFSET
    }

}

/// RNG_BIST_CNTR_2 value type for batch read/modify/write operations
///
/// Use `rngbistcntr2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rngbistcntr2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RngBistCntr2Val {
    bits: u32,
}

impl RngBistCntr2Val {
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

    /// Read RESERVED [31:22]: RESERVED
    #[inline(always)]
    pub const fn reserved(&self) -> u32 {
        (self.bits & RngBistCntr2::RESERVED_MASK) >> RngBistCntr2::RESERVED_OFFSET
    }

    /// Read ROSC_CNTR_VAL [21:0]: Reflects the results of RNG BIST counter.
    #[inline(always)]
    pub const fn rosc_cntr_val(&self) -> u32 {
        (self.bits & RngBistCntr2::ROSC_CNTR_VAL_MASK) >> RngBistCntr2::ROSC_CNTR_VAL_OFFSET
    }

}

impl Default for RngBistCntr2Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl RngBistCntr2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> RngBistCntr2Val {
        RngBistCntr2Val::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: RngBistCntr2Val) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(RngBistCntr2Val) -> RngBistCntr2Val>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// Register block
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 256],
    /// 0x0100 - Interrupt masking.
    pub rng_imr: RngImr,
    /// 0x0104 - RNG status register. If corresponding RNG_IMR bit is unmasked, an interrupt...
    pub rng_isr: RngIsr,
    /// 0x0108 - Interrupt/status bit clear Register.
    pub rng_icr: RngIcr,
    /// 0x010c - Selecting the inverter-chain length.
    pub trng_config: TrngConfig,
    /// 0x0110 - 192 bit collection indication.
    pub trng_valid: TrngValid,
    /// 0x0114 - RNG collected bits.
    pub ehr_data0: EhrData0,
    /// 0x0118 - RNG collected bits.
    pub ehr_data1: EhrData1,
    /// 0x011c - RNG collected bits.
    pub ehr_data2: EhrData2,
    /// 0x0120 - RNG collected bits.
    pub ehr_data3: EhrData3,
    /// 0x0124 - RNG collected bits.
    pub ehr_data4: EhrData4,
    /// 0x0128 - RNG collected bits.
    pub ehr_data5: EhrData5,
    /// 0x012c - Enable signal for the random source.
    pub rnd_source_enable: RndSourceEnable,
    /// 0x0130 - Counts clocks between sampling of random bit.
    pub sample_cnt1: SampleCnt1,
    /// 0x0134 - Statistic about Autocorrelation test activations.
    pub autocorr_statistic: AutocorrStatistic,
    /// 0x0138 - Debug register.
    pub trng_debug_control: TrngDebugControl,
    _reserved1: [u8; 4],
    /// 0x0140 - Generate internal SW reset within the RNG block.
    pub trng_sw_reset: TrngSwReset,
    _reserved2: [u8; 112],
    /// 0x01b4 - Enable the RNG debug mode
    pub rng_debug_en_input: RngDebugEnInput,
    /// 0x01b8 - RNG Busy indication.
    pub trng_busy: TrngBusy,
    /// 0x01bc - Reset the counter of collected bits in the RNG.
    pub rst_bits_counter: RstBitsCounter,
    /// 0x01c0 - Displays the version settings of the TRNG.
    pub rng_version: RngVersion,
    _reserved3: [u8; 28],
    /// 0x01e0 - Collected BIST results.
    pub rng_bist_cntr_0: RngBistCntr0,
    /// 0x01e4 - Collected BIST results.
    pub rng_bist_cntr_1: RngBistCntr1,
    /// 0x01e8 - Collected BIST results.
    pub rng_bist_cntr_2: RngBistCntr2,
}

