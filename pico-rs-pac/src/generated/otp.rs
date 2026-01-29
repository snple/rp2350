//! OTP - SNPS OTP control IF (SBPI and RPi wrapper control)

#![allow(dead_code)]

pub const BASE: u32 = 0x40120000;

/// SW_LOCK0 Register
///
/// Software lock register for page 0. Locks are initialised from the OTP lock
/// pages at reset. This register can be written to further advance the lock
/// state of each page (until next reset), and read to check the current lock
/// state of a page.
///
/// Address offset: 0x0000
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **NSEC** [3:2] (RW): Non-secure lock status. Writes are OR'd with the current value.
/// - **SEC** [1:0] (RW): Secure lock status. Writes are OR'd with the current value. This field is r...
#[repr(transparent)]
pub struct SwLock0 {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for SwLock0 {}
unsafe impl Sync for SwLock0 {}

impl SwLock0 {
    // ======== Field Constants ========

    /// NSEC - Non-secure lock status. Writes are OR'd with the current value.
    /// Bits [3:2], mask 0x0000000c
    pub const NSEC_OFFSET: u32 = 2;
    pub const NSEC_MASK: u32 = 0x0000000c;
    pub const NSEC_READ_WRITE: u32 = 0 << 2;
    pub const NSEC_READ_ONLY: u32 = 1 << 2;
    pub const NSEC_INACCESSIBLE: u32 = 3 << 2;

    /// SEC - Secure lock status. Writes are OR'd with the current value. This field is r...
    /// Bits [1:0], mask 0x00000003
    pub const SEC_OFFSET: u32 = 0;
    pub const SEC_MASK: u32 = 0x00000003;
    pub const SEC_READ_WRITE: u32 = 0 << 0;
    pub const SEC_READ_ONLY: u32 = 1 << 0;
    pub const SEC_INACCESSIBLE: u32 = 3 << 0;

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

    /// **NSEC** [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub fn nsec(&self) -> u32 {
        (self.read_bits() & Self::NSEC_MASK) >> Self::NSEC_OFFSET
    }

    /// Write NSEC field
    #[inline(always)]
    pub fn write_nsec(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::NSEC_MASK) | ((value << Self::NSEC_OFFSET) & Self::NSEC_MASK));
    }

    /// **SEC** [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub fn sec(&self) -> u32 {
        (self.read_bits() & Self::SEC_MASK) >> Self::SEC_OFFSET
    }

    /// Write SEC field
    #[inline(always)]
    pub fn write_sec(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::SEC_MASK) | ((value << Self::SEC_OFFSET) & Self::SEC_MASK));
    }

}

/// SW_LOCK0 value type for batch read/modify/write operations
///
/// Use `swlock0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `swlock0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SwLock0Val {
    bits: u32,
}

impl SwLock0Val {
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

    /// Read NSEC [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub const fn nsec(&self) -> u32 {
        (self.bits & SwLock0::NSEC_MASK) >> SwLock0::NSEC_OFFSET
    }

    /// Read SEC [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub const fn sec(&self) -> u32 {
        (self.bits & SwLock0::SEC_MASK) >> SwLock0::SEC_OFFSET
    }

    /// Set NSEC [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub const fn with_nsec(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SwLock0::NSEC_MASK) | ((value << SwLock0::NSEC_OFFSET) & SwLock0::NSEC_MASK)
        }
    }

    /// Set SEC [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub const fn with_sec(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SwLock0::SEC_MASK) | ((value << SwLock0::SEC_OFFSET) & SwLock0::SEC_MASK)
        }
    }

}

impl Default for SwLock0Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SwLock0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> SwLock0Val {
        SwLock0Val::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: SwLock0Val) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(SwLock0Val) -> SwLock0Val>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// SW_LOCK1 Register
///
/// Software lock register for page 1. Locks are initialised from the OTP lock
/// pages at reset. This register can be written to further advance the lock
/// state of each page (until next reset), and read to check the current lock
/// state of a page.
///
/// Address offset: 0x0004
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **NSEC** [3:2] (RW): Non-secure lock status. Writes are OR'd with the current value.
/// - **SEC** [1:0] (RW): Secure lock status. Writes are OR'd with the current value. This field is r...
#[repr(transparent)]
pub struct SwLock1 {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for SwLock1 {}
unsafe impl Sync for SwLock1 {}

impl SwLock1 {
    // ======== Field Constants ========

    /// NSEC - Non-secure lock status. Writes are OR'd with the current value.
    /// Bits [3:2], mask 0x0000000c
    pub const NSEC_OFFSET: u32 = 2;
    pub const NSEC_MASK: u32 = 0x0000000c;
    pub const NSEC_READ_WRITE: u32 = 0 << 2;
    pub const NSEC_READ_ONLY: u32 = 1 << 2;
    pub const NSEC_INACCESSIBLE: u32 = 3 << 2;

    /// SEC - Secure lock status. Writes are OR'd with the current value. This field is r...
    /// Bits [1:0], mask 0x00000003
    pub const SEC_OFFSET: u32 = 0;
    pub const SEC_MASK: u32 = 0x00000003;
    pub const SEC_READ_WRITE: u32 = 0 << 0;
    pub const SEC_READ_ONLY: u32 = 1 << 0;
    pub const SEC_INACCESSIBLE: u32 = 3 << 0;

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

    /// **NSEC** [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub fn nsec(&self) -> u32 {
        (self.read_bits() & Self::NSEC_MASK) >> Self::NSEC_OFFSET
    }

    /// Write NSEC field
    #[inline(always)]
    pub fn write_nsec(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::NSEC_MASK) | ((value << Self::NSEC_OFFSET) & Self::NSEC_MASK));
    }

    /// **SEC** [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub fn sec(&self) -> u32 {
        (self.read_bits() & Self::SEC_MASK) >> Self::SEC_OFFSET
    }

    /// Write SEC field
    #[inline(always)]
    pub fn write_sec(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::SEC_MASK) | ((value << Self::SEC_OFFSET) & Self::SEC_MASK));
    }

}

/// SW_LOCK1 value type for batch read/modify/write operations
///
/// Use `swlock1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `swlock1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SwLock1Val {
    bits: u32,
}

impl SwLock1Val {
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

    /// Read NSEC [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub const fn nsec(&self) -> u32 {
        (self.bits & SwLock1::NSEC_MASK) >> SwLock1::NSEC_OFFSET
    }

    /// Read SEC [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub const fn sec(&self) -> u32 {
        (self.bits & SwLock1::SEC_MASK) >> SwLock1::SEC_OFFSET
    }

    /// Set NSEC [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub const fn with_nsec(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SwLock1::NSEC_MASK) | ((value << SwLock1::NSEC_OFFSET) & SwLock1::NSEC_MASK)
        }
    }

    /// Set SEC [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub const fn with_sec(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SwLock1::SEC_MASK) | ((value << SwLock1::SEC_OFFSET) & SwLock1::SEC_MASK)
        }
    }

}

impl Default for SwLock1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SwLock1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> SwLock1Val {
        SwLock1Val::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: SwLock1Val) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(SwLock1Val) -> SwLock1Val>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// SW_LOCK2 Register
///
/// Software lock register for page 2. Locks are initialised from the OTP lock
/// pages at reset. This register can be written to further advance the lock
/// state of each page (until next reset), and read to check the current lock
/// state of a page.
///
/// Address offset: 0x0008
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **NSEC** [3:2] (RW): Non-secure lock status. Writes are OR'd with the current value.
/// - **SEC** [1:0] (RW): Secure lock status. Writes are OR'd with the current value. This field is r...
#[repr(transparent)]
pub struct SwLock2 {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for SwLock2 {}
unsafe impl Sync for SwLock2 {}

impl SwLock2 {
    // ======== Field Constants ========

    /// NSEC - Non-secure lock status. Writes are OR'd with the current value.
    /// Bits [3:2], mask 0x0000000c
    pub const NSEC_OFFSET: u32 = 2;
    pub const NSEC_MASK: u32 = 0x0000000c;
    pub const NSEC_READ_WRITE: u32 = 0 << 2;
    pub const NSEC_READ_ONLY: u32 = 1 << 2;
    pub const NSEC_INACCESSIBLE: u32 = 3 << 2;

    /// SEC - Secure lock status. Writes are OR'd with the current value. This field is r...
    /// Bits [1:0], mask 0x00000003
    pub const SEC_OFFSET: u32 = 0;
    pub const SEC_MASK: u32 = 0x00000003;
    pub const SEC_READ_WRITE: u32 = 0 << 0;
    pub const SEC_READ_ONLY: u32 = 1 << 0;
    pub const SEC_INACCESSIBLE: u32 = 3 << 0;

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

    /// **NSEC** [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub fn nsec(&self) -> u32 {
        (self.read_bits() & Self::NSEC_MASK) >> Self::NSEC_OFFSET
    }

    /// Write NSEC field
    #[inline(always)]
    pub fn write_nsec(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::NSEC_MASK) | ((value << Self::NSEC_OFFSET) & Self::NSEC_MASK));
    }

    /// **SEC** [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub fn sec(&self) -> u32 {
        (self.read_bits() & Self::SEC_MASK) >> Self::SEC_OFFSET
    }

    /// Write SEC field
    #[inline(always)]
    pub fn write_sec(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::SEC_MASK) | ((value << Self::SEC_OFFSET) & Self::SEC_MASK));
    }

}

/// SW_LOCK2 value type for batch read/modify/write operations
///
/// Use `swlock2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `swlock2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SwLock2Val {
    bits: u32,
}

impl SwLock2Val {
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

    /// Read NSEC [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub const fn nsec(&self) -> u32 {
        (self.bits & SwLock2::NSEC_MASK) >> SwLock2::NSEC_OFFSET
    }

    /// Read SEC [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub const fn sec(&self) -> u32 {
        (self.bits & SwLock2::SEC_MASK) >> SwLock2::SEC_OFFSET
    }

    /// Set NSEC [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub const fn with_nsec(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SwLock2::NSEC_MASK) | ((value << SwLock2::NSEC_OFFSET) & SwLock2::NSEC_MASK)
        }
    }

    /// Set SEC [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub const fn with_sec(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SwLock2::SEC_MASK) | ((value << SwLock2::SEC_OFFSET) & SwLock2::SEC_MASK)
        }
    }

}

impl Default for SwLock2Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SwLock2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> SwLock2Val {
        SwLock2Val::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: SwLock2Val) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(SwLock2Val) -> SwLock2Val>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// SW_LOCK3 Register
///
/// Software lock register for page 3. Locks are initialised from the OTP lock
/// pages at reset. This register can be written to further advance the lock
/// state of each page (until next reset), and read to check the current lock
/// state of a page.
///
/// Address offset: 0x000c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **NSEC** [3:2] (RW): Non-secure lock status. Writes are OR'd with the current value.
/// - **SEC** [1:0] (RW): Secure lock status. Writes are OR'd with the current value. This field is r...
#[repr(transparent)]
pub struct SwLock3 {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for SwLock3 {}
unsafe impl Sync for SwLock3 {}

impl SwLock3 {
    // ======== Field Constants ========

    /// NSEC - Non-secure lock status. Writes are OR'd with the current value.
    /// Bits [3:2], mask 0x0000000c
    pub const NSEC_OFFSET: u32 = 2;
    pub const NSEC_MASK: u32 = 0x0000000c;
    pub const NSEC_READ_WRITE: u32 = 0 << 2;
    pub const NSEC_READ_ONLY: u32 = 1 << 2;
    pub const NSEC_INACCESSIBLE: u32 = 3 << 2;

    /// SEC - Secure lock status. Writes are OR'd with the current value. This field is r...
    /// Bits [1:0], mask 0x00000003
    pub const SEC_OFFSET: u32 = 0;
    pub const SEC_MASK: u32 = 0x00000003;
    pub const SEC_READ_WRITE: u32 = 0 << 0;
    pub const SEC_READ_ONLY: u32 = 1 << 0;
    pub const SEC_INACCESSIBLE: u32 = 3 << 0;

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

    /// **NSEC** [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub fn nsec(&self) -> u32 {
        (self.read_bits() & Self::NSEC_MASK) >> Self::NSEC_OFFSET
    }

    /// Write NSEC field
    #[inline(always)]
    pub fn write_nsec(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::NSEC_MASK) | ((value << Self::NSEC_OFFSET) & Self::NSEC_MASK));
    }

    /// **SEC** [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub fn sec(&self) -> u32 {
        (self.read_bits() & Self::SEC_MASK) >> Self::SEC_OFFSET
    }

    /// Write SEC field
    #[inline(always)]
    pub fn write_sec(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::SEC_MASK) | ((value << Self::SEC_OFFSET) & Self::SEC_MASK));
    }

}

/// SW_LOCK3 value type for batch read/modify/write operations
///
/// Use `swlock3.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `swlock3.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SwLock3Val {
    bits: u32,
}

impl SwLock3Val {
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

    /// Read NSEC [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub const fn nsec(&self) -> u32 {
        (self.bits & SwLock3::NSEC_MASK) >> SwLock3::NSEC_OFFSET
    }

    /// Read SEC [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub const fn sec(&self) -> u32 {
        (self.bits & SwLock3::SEC_MASK) >> SwLock3::SEC_OFFSET
    }

    /// Set NSEC [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub const fn with_nsec(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SwLock3::NSEC_MASK) | ((value << SwLock3::NSEC_OFFSET) & SwLock3::NSEC_MASK)
        }
    }

    /// Set SEC [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub const fn with_sec(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SwLock3::SEC_MASK) | ((value << SwLock3::SEC_OFFSET) & SwLock3::SEC_MASK)
        }
    }

}

impl Default for SwLock3Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SwLock3 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> SwLock3Val {
        SwLock3Val::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: SwLock3Val) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(SwLock3Val) -> SwLock3Val>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// SW_LOCK4 Register
///
/// Software lock register for page 4. Locks are initialised from the OTP lock
/// pages at reset. This register can be written to further advance the lock
/// state of each page (until next reset), and read to check the current lock
/// state of a page.
///
/// Address offset: 0x0010
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **NSEC** [3:2] (RW): Non-secure lock status. Writes are OR'd with the current value.
/// - **SEC** [1:0] (RW): Secure lock status. Writes are OR'd with the current value. This field is r...
#[repr(transparent)]
pub struct SwLock4 {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for SwLock4 {}
unsafe impl Sync for SwLock4 {}

impl SwLock4 {
    // ======== Field Constants ========

    /// NSEC - Non-secure lock status. Writes are OR'd with the current value.
    /// Bits [3:2], mask 0x0000000c
    pub const NSEC_OFFSET: u32 = 2;
    pub const NSEC_MASK: u32 = 0x0000000c;
    pub const NSEC_READ_WRITE: u32 = 0 << 2;
    pub const NSEC_READ_ONLY: u32 = 1 << 2;
    pub const NSEC_INACCESSIBLE: u32 = 3 << 2;

    /// SEC - Secure lock status. Writes are OR'd with the current value. This field is r...
    /// Bits [1:0], mask 0x00000003
    pub const SEC_OFFSET: u32 = 0;
    pub const SEC_MASK: u32 = 0x00000003;
    pub const SEC_READ_WRITE: u32 = 0 << 0;
    pub const SEC_READ_ONLY: u32 = 1 << 0;
    pub const SEC_INACCESSIBLE: u32 = 3 << 0;

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

    /// **NSEC** [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub fn nsec(&self) -> u32 {
        (self.read_bits() & Self::NSEC_MASK) >> Self::NSEC_OFFSET
    }

    /// Write NSEC field
    #[inline(always)]
    pub fn write_nsec(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::NSEC_MASK) | ((value << Self::NSEC_OFFSET) & Self::NSEC_MASK));
    }

    /// **SEC** [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub fn sec(&self) -> u32 {
        (self.read_bits() & Self::SEC_MASK) >> Self::SEC_OFFSET
    }

    /// Write SEC field
    #[inline(always)]
    pub fn write_sec(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::SEC_MASK) | ((value << Self::SEC_OFFSET) & Self::SEC_MASK));
    }

}

/// SW_LOCK4 value type for batch read/modify/write operations
///
/// Use `swlock4.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `swlock4.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SwLock4Val {
    bits: u32,
}

impl SwLock4Val {
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

    /// Read NSEC [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub const fn nsec(&self) -> u32 {
        (self.bits & SwLock4::NSEC_MASK) >> SwLock4::NSEC_OFFSET
    }

    /// Read SEC [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub const fn sec(&self) -> u32 {
        (self.bits & SwLock4::SEC_MASK) >> SwLock4::SEC_OFFSET
    }

    /// Set NSEC [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub const fn with_nsec(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SwLock4::NSEC_MASK) | ((value << SwLock4::NSEC_OFFSET) & SwLock4::NSEC_MASK)
        }
    }

    /// Set SEC [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub const fn with_sec(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SwLock4::SEC_MASK) | ((value << SwLock4::SEC_OFFSET) & SwLock4::SEC_MASK)
        }
    }

}

impl Default for SwLock4Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SwLock4 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> SwLock4Val {
        SwLock4Val::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: SwLock4Val) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(SwLock4Val) -> SwLock4Val>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// SW_LOCK5 Register
///
/// Software lock register for page 5. Locks are initialised from the OTP lock
/// pages at reset. This register can be written to further advance the lock
/// state of each page (until next reset), and read to check the current lock
/// state of a page.
///
/// Address offset: 0x0014
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **NSEC** [3:2] (RW): Non-secure lock status. Writes are OR'd with the current value.
/// - **SEC** [1:0] (RW): Secure lock status. Writes are OR'd with the current value. This field is r...
#[repr(transparent)]
pub struct SwLock5 {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for SwLock5 {}
unsafe impl Sync for SwLock5 {}

impl SwLock5 {
    // ======== Field Constants ========

    /// NSEC - Non-secure lock status. Writes are OR'd with the current value.
    /// Bits [3:2], mask 0x0000000c
    pub const NSEC_OFFSET: u32 = 2;
    pub const NSEC_MASK: u32 = 0x0000000c;
    pub const NSEC_READ_WRITE: u32 = 0 << 2;
    pub const NSEC_READ_ONLY: u32 = 1 << 2;
    pub const NSEC_INACCESSIBLE: u32 = 3 << 2;

    /// SEC - Secure lock status. Writes are OR'd with the current value. This field is r...
    /// Bits [1:0], mask 0x00000003
    pub const SEC_OFFSET: u32 = 0;
    pub const SEC_MASK: u32 = 0x00000003;
    pub const SEC_READ_WRITE: u32 = 0 << 0;
    pub const SEC_READ_ONLY: u32 = 1 << 0;
    pub const SEC_INACCESSIBLE: u32 = 3 << 0;

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

    /// **NSEC** [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub fn nsec(&self) -> u32 {
        (self.read_bits() & Self::NSEC_MASK) >> Self::NSEC_OFFSET
    }

    /// Write NSEC field
    #[inline(always)]
    pub fn write_nsec(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::NSEC_MASK) | ((value << Self::NSEC_OFFSET) & Self::NSEC_MASK));
    }

    /// **SEC** [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub fn sec(&self) -> u32 {
        (self.read_bits() & Self::SEC_MASK) >> Self::SEC_OFFSET
    }

    /// Write SEC field
    #[inline(always)]
    pub fn write_sec(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::SEC_MASK) | ((value << Self::SEC_OFFSET) & Self::SEC_MASK));
    }

}

/// SW_LOCK5 value type for batch read/modify/write operations
///
/// Use `swlock5.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `swlock5.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SwLock5Val {
    bits: u32,
}

impl SwLock5Val {
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

    /// Read NSEC [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub const fn nsec(&self) -> u32 {
        (self.bits & SwLock5::NSEC_MASK) >> SwLock5::NSEC_OFFSET
    }

    /// Read SEC [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub const fn sec(&self) -> u32 {
        (self.bits & SwLock5::SEC_MASK) >> SwLock5::SEC_OFFSET
    }

    /// Set NSEC [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub const fn with_nsec(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SwLock5::NSEC_MASK) | ((value << SwLock5::NSEC_OFFSET) & SwLock5::NSEC_MASK)
        }
    }

    /// Set SEC [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub const fn with_sec(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SwLock5::SEC_MASK) | ((value << SwLock5::SEC_OFFSET) & SwLock5::SEC_MASK)
        }
    }

}

impl Default for SwLock5Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SwLock5 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> SwLock5Val {
        SwLock5Val::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: SwLock5Val) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(SwLock5Val) -> SwLock5Val>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// SW_LOCK6 Register
///
/// Software lock register for page 6. Locks are initialised from the OTP lock
/// pages at reset. This register can be written to further advance the lock
/// state of each page (until next reset), and read to check the current lock
/// state of a page.
///
/// Address offset: 0x0018
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **NSEC** [3:2] (RW): Non-secure lock status. Writes are OR'd with the current value.
/// - **SEC** [1:0] (RW): Secure lock status. Writes are OR'd with the current value. This field is r...
#[repr(transparent)]
pub struct SwLock6 {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for SwLock6 {}
unsafe impl Sync for SwLock6 {}

impl SwLock6 {
    // ======== Field Constants ========

    /// NSEC - Non-secure lock status. Writes are OR'd with the current value.
    /// Bits [3:2], mask 0x0000000c
    pub const NSEC_OFFSET: u32 = 2;
    pub const NSEC_MASK: u32 = 0x0000000c;
    pub const NSEC_READ_WRITE: u32 = 0 << 2;
    pub const NSEC_READ_ONLY: u32 = 1 << 2;
    pub const NSEC_INACCESSIBLE: u32 = 3 << 2;

    /// SEC - Secure lock status. Writes are OR'd with the current value. This field is r...
    /// Bits [1:0], mask 0x00000003
    pub const SEC_OFFSET: u32 = 0;
    pub const SEC_MASK: u32 = 0x00000003;
    pub const SEC_READ_WRITE: u32 = 0 << 0;
    pub const SEC_READ_ONLY: u32 = 1 << 0;
    pub const SEC_INACCESSIBLE: u32 = 3 << 0;

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

    /// **NSEC** [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub fn nsec(&self) -> u32 {
        (self.read_bits() & Self::NSEC_MASK) >> Self::NSEC_OFFSET
    }

    /// Write NSEC field
    #[inline(always)]
    pub fn write_nsec(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::NSEC_MASK) | ((value << Self::NSEC_OFFSET) & Self::NSEC_MASK));
    }

    /// **SEC** [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub fn sec(&self) -> u32 {
        (self.read_bits() & Self::SEC_MASK) >> Self::SEC_OFFSET
    }

    /// Write SEC field
    #[inline(always)]
    pub fn write_sec(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::SEC_MASK) | ((value << Self::SEC_OFFSET) & Self::SEC_MASK));
    }

}

/// SW_LOCK6 value type for batch read/modify/write operations
///
/// Use `swlock6.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `swlock6.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SwLock6Val {
    bits: u32,
}

impl SwLock6Val {
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

    /// Read NSEC [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub const fn nsec(&self) -> u32 {
        (self.bits & SwLock6::NSEC_MASK) >> SwLock6::NSEC_OFFSET
    }

    /// Read SEC [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub const fn sec(&self) -> u32 {
        (self.bits & SwLock6::SEC_MASK) >> SwLock6::SEC_OFFSET
    }

    /// Set NSEC [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub const fn with_nsec(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SwLock6::NSEC_MASK) | ((value << SwLock6::NSEC_OFFSET) & SwLock6::NSEC_MASK)
        }
    }

    /// Set SEC [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub const fn with_sec(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SwLock6::SEC_MASK) | ((value << SwLock6::SEC_OFFSET) & SwLock6::SEC_MASK)
        }
    }

}

impl Default for SwLock6Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SwLock6 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> SwLock6Val {
        SwLock6Val::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: SwLock6Val) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(SwLock6Val) -> SwLock6Val>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// SW_LOCK7 Register
///
/// Software lock register for page 7. Locks are initialised from the OTP lock
/// pages at reset. This register can be written to further advance the lock
/// state of each page (until next reset), and read to check the current lock
/// state of a page.
///
/// Address offset: 0x001c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **NSEC** [3:2] (RW): Non-secure lock status. Writes are OR'd with the current value.
/// - **SEC** [1:0] (RW): Secure lock status. Writes are OR'd with the current value. This field is r...
#[repr(transparent)]
pub struct SwLock7 {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for SwLock7 {}
unsafe impl Sync for SwLock7 {}

impl SwLock7 {
    // ======== Field Constants ========

    /// NSEC - Non-secure lock status. Writes are OR'd with the current value.
    /// Bits [3:2], mask 0x0000000c
    pub const NSEC_OFFSET: u32 = 2;
    pub const NSEC_MASK: u32 = 0x0000000c;
    pub const NSEC_READ_WRITE: u32 = 0 << 2;
    pub const NSEC_READ_ONLY: u32 = 1 << 2;
    pub const NSEC_INACCESSIBLE: u32 = 3 << 2;

    /// SEC - Secure lock status. Writes are OR'd with the current value. This field is r...
    /// Bits [1:0], mask 0x00000003
    pub const SEC_OFFSET: u32 = 0;
    pub const SEC_MASK: u32 = 0x00000003;
    pub const SEC_READ_WRITE: u32 = 0 << 0;
    pub const SEC_READ_ONLY: u32 = 1 << 0;
    pub const SEC_INACCESSIBLE: u32 = 3 << 0;

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

    /// **NSEC** [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub fn nsec(&self) -> u32 {
        (self.read_bits() & Self::NSEC_MASK) >> Self::NSEC_OFFSET
    }

    /// Write NSEC field
    #[inline(always)]
    pub fn write_nsec(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::NSEC_MASK) | ((value << Self::NSEC_OFFSET) & Self::NSEC_MASK));
    }

    /// **SEC** [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub fn sec(&self) -> u32 {
        (self.read_bits() & Self::SEC_MASK) >> Self::SEC_OFFSET
    }

    /// Write SEC field
    #[inline(always)]
    pub fn write_sec(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::SEC_MASK) | ((value << Self::SEC_OFFSET) & Self::SEC_MASK));
    }

}

/// SW_LOCK7 value type for batch read/modify/write operations
///
/// Use `swlock7.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `swlock7.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SwLock7Val {
    bits: u32,
}

impl SwLock7Val {
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

    /// Read NSEC [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub const fn nsec(&self) -> u32 {
        (self.bits & SwLock7::NSEC_MASK) >> SwLock7::NSEC_OFFSET
    }

    /// Read SEC [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub const fn sec(&self) -> u32 {
        (self.bits & SwLock7::SEC_MASK) >> SwLock7::SEC_OFFSET
    }

    /// Set NSEC [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub const fn with_nsec(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SwLock7::NSEC_MASK) | ((value << SwLock7::NSEC_OFFSET) & SwLock7::NSEC_MASK)
        }
    }

    /// Set SEC [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub const fn with_sec(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SwLock7::SEC_MASK) | ((value << SwLock7::SEC_OFFSET) & SwLock7::SEC_MASK)
        }
    }

}

impl Default for SwLock7Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SwLock7 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> SwLock7Val {
        SwLock7Val::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: SwLock7Val) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(SwLock7Val) -> SwLock7Val>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// SW_LOCK8 Register
///
/// Software lock register for page 8. Locks are initialised from the OTP lock
/// pages at reset. This register can be written to further advance the lock
/// state of each page (until next reset), and read to check the current lock
/// state of a page.
///
/// Address offset: 0x0020
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **NSEC** [3:2] (RW): Non-secure lock status. Writes are OR'd with the current value.
/// - **SEC** [1:0] (RW): Secure lock status. Writes are OR'd with the current value. This field is r...
#[repr(transparent)]
pub struct SwLock8 {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for SwLock8 {}
unsafe impl Sync for SwLock8 {}

impl SwLock8 {
    // ======== Field Constants ========

    /// NSEC - Non-secure lock status. Writes are OR'd with the current value.
    /// Bits [3:2], mask 0x0000000c
    pub const NSEC_OFFSET: u32 = 2;
    pub const NSEC_MASK: u32 = 0x0000000c;
    pub const NSEC_READ_WRITE: u32 = 0 << 2;
    pub const NSEC_READ_ONLY: u32 = 1 << 2;
    pub const NSEC_INACCESSIBLE: u32 = 3 << 2;

    /// SEC - Secure lock status. Writes are OR'd with the current value. This field is r...
    /// Bits [1:0], mask 0x00000003
    pub const SEC_OFFSET: u32 = 0;
    pub const SEC_MASK: u32 = 0x00000003;
    pub const SEC_READ_WRITE: u32 = 0 << 0;
    pub const SEC_READ_ONLY: u32 = 1 << 0;
    pub const SEC_INACCESSIBLE: u32 = 3 << 0;

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

    /// **NSEC** [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub fn nsec(&self) -> u32 {
        (self.read_bits() & Self::NSEC_MASK) >> Self::NSEC_OFFSET
    }

    /// Write NSEC field
    #[inline(always)]
    pub fn write_nsec(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::NSEC_MASK) | ((value << Self::NSEC_OFFSET) & Self::NSEC_MASK));
    }

    /// **SEC** [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub fn sec(&self) -> u32 {
        (self.read_bits() & Self::SEC_MASK) >> Self::SEC_OFFSET
    }

    /// Write SEC field
    #[inline(always)]
    pub fn write_sec(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::SEC_MASK) | ((value << Self::SEC_OFFSET) & Self::SEC_MASK));
    }

}

/// SW_LOCK8 value type for batch read/modify/write operations
///
/// Use `swlock8.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `swlock8.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SwLock8Val {
    bits: u32,
}

impl SwLock8Val {
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

    /// Read NSEC [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub const fn nsec(&self) -> u32 {
        (self.bits & SwLock8::NSEC_MASK) >> SwLock8::NSEC_OFFSET
    }

    /// Read SEC [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub const fn sec(&self) -> u32 {
        (self.bits & SwLock8::SEC_MASK) >> SwLock8::SEC_OFFSET
    }

    /// Set NSEC [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub const fn with_nsec(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SwLock8::NSEC_MASK) | ((value << SwLock8::NSEC_OFFSET) & SwLock8::NSEC_MASK)
        }
    }

    /// Set SEC [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub const fn with_sec(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SwLock8::SEC_MASK) | ((value << SwLock8::SEC_OFFSET) & SwLock8::SEC_MASK)
        }
    }

}

impl Default for SwLock8Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SwLock8 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> SwLock8Val {
        SwLock8Val::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: SwLock8Val) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(SwLock8Val) -> SwLock8Val>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// SW_LOCK9 Register
///
/// Software lock register for page 9. Locks are initialised from the OTP lock
/// pages at reset. This register can be written to further advance the lock
/// state of each page (until next reset), and read to check the current lock
/// state of a page.
///
/// Address offset: 0x0024
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **NSEC** [3:2] (RW): Non-secure lock status. Writes are OR'd with the current value.
/// - **SEC** [1:0] (RW): Secure lock status. Writes are OR'd with the current value. This field is r...
#[repr(transparent)]
pub struct SwLock9 {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for SwLock9 {}
unsafe impl Sync for SwLock9 {}

impl SwLock9 {
    // ======== Field Constants ========

    /// NSEC - Non-secure lock status. Writes are OR'd with the current value.
    /// Bits [3:2], mask 0x0000000c
    pub const NSEC_OFFSET: u32 = 2;
    pub const NSEC_MASK: u32 = 0x0000000c;
    pub const NSEC_READ_WRITE: u32 = 0 << 2;
    pub const NSEC_READ_ONLY: u32 = 1 << 2;
    pub const NSEC_INACCESSIBLE: u32 = 3 << 2;

    /// SEC - Secure lock status. Writes are OR'd with the current value. This field is r...
    /// Bits [1:0], mask 0x00000003
    pub const SEC_OFFSET: u32 = 0;
    pub const SEC_MASK: u32 = 0x00000003;
    pub const SEC_READ_WRITE: u32 = 0 << 0;
    pub const SEC_READ_ONLY: u32 = 1 << 0;
    pub const SEC_INACCESSIBLE: u32 = 3 << 0;

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

    /// **NSEC** [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub fn nsec(&self) -> u32 {
        (self.read_bits() & Self::NSEC_MASK) >> Self::NSEC_OFFSET
    }

    /// Write NSEC field
    #[inline(always)]
    pub fn write_nsec(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::NSEC_MASK) | ((value << Self::NSEC_OFFSET) & Self::NSEC_MASK));
    }

    /// **SEC** [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub fn sec(&self) -> u32 {
        (self.read_bits() & Self::SEC_MASK) >> Self::SEC_OFFSET
    }

    /// Write SEC field
    #[inline(always)]
    pub fn write_sec(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::SEC_MASK) | ((value << Self::SEC_OFFSET) & Self::SEC_MASK));
    }

}

/// SW_LOCK9 value type for batch read/modify/write operations
///
/// Use `swlock9.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `swlock9.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SwLock9Val {
    bits: u32,
}

impl SwLock9Val {
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

    /// Read NSEC [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub const fn nsec(&self) -> u32 {
        (self.bits & SwLock9::NSEC_MASK) >> SwLock9::NSEC_OFFSET
    }

    /// Read SEC [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub const fn sec(&self) -> u32 {
        (self.bits & SwLock9::SEC_MASK) >> SwLock9::SEC_OFFSET
    }

    /// Set NSEC [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub const fn with_nsec(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SwLock9::NSEC_MASK) | ((value << SwLock9::NSEC_OFFSET) & SwLock9::NSEC_MASK)
        }
    }

    /// Set SEC [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub const fn with_sec(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SwLock9::SEC_MASK) | ((value << SwLock9::SEC_OFFSET) & SwLock9::SEC_MASK)
        }
    }

}

impl Default for SwLock9Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SwLock9 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> SwLock9Val {
        SwLock9Val::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: SwLock9Val) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(SwLock9Val) -> SwLock9Val>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// SW_LOCK10 Register
///
/// Software lock register for page 10. Locks are initialised from the OTP lock
/// pages at reset. This register can be written to further advance the lock
/// state of each page (until next reset), and read to check the current lock
/// state of a page.
///
/// Address offset: 0x0028
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **NSEC** [3:2] (RW): Non-secure lock status. Writes are OR'd with the current value.
/// - **SEC** [1:0] (RW): Secure lock status. Writes are OR'd with the current value. This field is r...
#[repr(transparent)]
pub struct SwLock10 {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for SwLock10 {}
unsafe impl Sync for SwLock10 {}

impl SwLock10 {
    // ======== Field Constants ========

    /// NSEC - Non-secure lock status. Writes are OR'd with the current value.
    /// Bits [3:2], mask 0x0000000c
    pub const NSEC_OFFSET: u32 = 2;
    pub const NSEC_MASK: u32 = 0x0000000c;
    pub const NSEC_READ_WRITE: u32 = 0 << 2;
    pub const NSEC_READ_ONLY: u32 = 1 << 2;
    pub const NSEC_INACCESSIBLE: u32 = 3 << 2;

    /// SEC - Secure lock status. Writes are OR'd with the current value. This field is r...
    /// Bits [1:0], mask 0x00000003
    pub const SEC_OFFSET: u32 = 0;
    pub const SEC_MASK: u32 = 0x00000003;
    pub const SEC_READ_WRITE: u32 = 0 << 0;
    pub const SEC_READ_ONLY: u32 = 1 << 0;
    pub const SEC_INACCESSIBLE: u32 = 3 << 0;

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

    /// **NSEC** [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub fn nsec(&self) -> u32 {
        (self.read_bits() & Self::NSEC_MASK) >> Self::NSEC_OFFSET
    }

    /// Write NSEC field
    #[inline(always)]
    pub fn write_nsec(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::NSEC_MASK) | ((value << Self::NSEC_OFFSET) & Self::NSEC_MASK));
    }

    /// **SEC** [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub fn sec(&self) -> u32 {
        (self.read_bits() & Self::SEC_MASK) >> Self::SEC_OFFSET
    }

    /// Write SEC field
    #[inline(always)]
    pub fn write_sec(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::SEC_MASK) | ((value << Self::SEC_OFFSET) & Self::SEC_MASK));
    }

}

/// SW_LOCK10 value type for batch read/modify/write operations
///
/// Use `swlock10.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `swlock10.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SwLock10Val {
    bits: u32,
}

impl SwLock10Val {
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

    /// Read NSEC [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub const fn nsec(&self) -> u32 {
        (self.bits & SwLock10::NSEC_MASK) >> SwLock10::NSEC_OFFSET
    }

    /// Read SEC [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub const fn sec(&self) -> u32 {
        (self.bits & SwLock10::SEC_MASK) >> SwLock10::SEC_OFFSET
    }

    /// Set NSEC [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub const fn with_nsec(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SwLock10::NSEC_MASK) | ((value << SwLock10::NSEC_OFFSET) & SwLock10::NSEC_MASK)
        }
    }

    /// Set SEC [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub const fn with_sec(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SwLock10::SEC_MASK) | ((value << SwLock10::SEC_OFFSET) & SwLock10::SEC_MASK)
        }
    }

}

impl Default for SwLock10Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SwLock10 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> SwLock10Val {
        SwLock10Val::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: SwLock10Val) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(SwLock10Val) -> SwLock10Val>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// SW_LOCK11 Register
///
/// Software lock register for page 11. Locks are initialised from the OTP lock
/// pages at reset. This register can be written to further advance the lock
/// state of each page (until next reset), and read to check the current lock
/// state of a page.
///
/// Address offset: 0x002c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **NSEC** [3:2] (RW): Non-secure lock status. Writes are OR'd with the current value.
/// - **SEC** [1:0] (RW): Secure lock status. Writes are OR'd with the current value. This field is r...
#[repr(transparent)]
pub struct SwLock11 {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for SwLock11 {}
unsafe impl Sync for SwLock11 {}

impl SwLock11 {
    // ======== Field Constants ========

    /// NSEC - Non-secure lock status. Writes are OR'd with the current value.
    /// Bits [3:2], mask 0x0000000c
    pub const NSEC_OFFSET: u32 = 2;
    pub const NSEC_MASK: u32 = 0x0000000c;
    pub const NSEC_READ_WRITE: u32 = 0 << 2;
    pub const NSEC_READ_ONLY: u32 = 1 << 2;
    pub const NSEC_INACCESSIBLE: u32 = 3 << 2;

    /// SEC - Secure lock status. Writes are OR'd with the current value. This field is r...
    /// Bits [1:0], mask 0x00000003
    pub const SEC_OFFSET: u32 = 0;
    pub const SEC_MASK: u32 = 0x00000003;
    pub const SEC_READ_WRITE: u32 = 0 << 0;
    pub const SEC_READ_ONLY: u32 = 1 << 0;
    pub const SEC_INACCESSIBLE: u32 = 3 << 0;

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

    /// **NSEC** [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub fn nsec(&self) -> u32 {
        (self.read_bits() & Self::NSEC_MASK) >> Self::NSEC_OFFSET
    }

    /// Write NSEC field
    #[inline(always)]
    pub fn write_nsec(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::NSEC_MASK) | ((value << Self::NSEC_OFFSET) & Self::NSEC_MASK));
    }

    /// **SEC** [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub fn sec(&self) -> u32 {
        (self.read_bits() & Self::SEC_MASK) >> Self::SEC_OFFSET
    }

    /// Write SEC field
    #[inline(always)]
    pub fn write_sec(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::SEC_MASK) | ((value << Self::SEC_OFFSET) & Self::SEC_MASK));
    }

}

/// SW_LOCK11 value type for batch read/modify/write operations
///
/// Use `swlock11.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `swlock11.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SwLock11Val {
    bits: u32,
}

impl SwLock11Val {
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

    /// Read NSEC [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub const fn nsec(&self) -> u32 {
        (self.bits & SwLock11::NSEC_MASK) >> SwLock11::NSEC_OFFSET
    }

    /// Read SEC [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub const fn sec(&self) -> u32 {
        (self.bits & SwLock11::SEC_MASK) >> SwLock11::SEC_OFFSET
    }

    /// Set NSEC [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub const fn with_nsec(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SwLock11::NSEC_MASK) | ((value << SwLock11::NSEC_OFFSET) & SwLock11::NSEC_MASK)
        }
    }

    /// Set SEC [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub const fn with_sec(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SwLock11::SEC_MASK) | ((value << SwLock11::SEC_OFFSET) & SwLock11::SEC_MASK)
        }
    }

}

impl Default for SwLock11Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SwLock11 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> SwLock11Val {
        SwLock11Val::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: SwLock11Val) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(SwLock11Val) -> SwLock11Val>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// SW_LOCK12 Register
///
/// Software lock register for page 12. Locks are initialised from the OTP lock
/// pages at reset. This register can be written to further advance the lock
/// state of each page (until next reset), and read to check the current lock
/// state of a page.
///
/// Address offset: 0x0030
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **NSEC** [3:2] (RW): Non-secure lock status. Writes are OR'd with the current value.
/// - **SEC** [1:0] (RW): Secure lock status. Writes are OR'd with the current value. This field is r...
#[repr(transparent)]
pub struct SwLock12 {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for SwLock12 {}
unsafe impl Sync for SwLock12 {}

impl SwLock12 {
    // ======== Field Constants ========

    /// NSEC - Non-secure lock status. Writes are OR'd with the current value.
    /// Bits [3:2], mask 0x0000000c
    pub const NSEC_OFFSET: u32 = 2;
    pub const NSEC_MASK: u32 = 0x0000000c;
    pub const NSEC_READ_WRITE: u32 = 0 << 2;
    pub const NSEC_READ_ONLY: u32 = 1 << 2;
    pub const NSEC_INACCESSIBLE: u32 = 3 << 2;

    /// SEC - Secure lock status. Writes are OR'd with the current value. This field is r...
    /// Bits [1:0], mask 0x00000003
    pub const SEC_OFFSET: u32 = 0;
    pub const SEC_MASK: u32 = 0x00000003;
    pub const SEC_READ_WRITE: u32 = 0 << 0;
    pub const SEC_READ_ONLY: u32 = 1 << 0;
    pub const SEC_INACCESSIBLE: u32 = 3 << 0;

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

    /// **NSEC** [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub fn nsec(&self) -> u32 {
        (self.read_bits() & Self::NSEC_MASK) >> Self::NSEC_OFFSET
    }

    /// Write NSEC field
    #[inline(always)]
    pub fn write_nsec(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::NSEC_MASK) | ((value << Self::NSEC_OFFSET) & Self::NSEC_MASK));
    }

    /// **SEC** [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub fn sec(&self) -> u32 {
        (self.read_bits() & Self::SEC_MASK) >> Self::SEC_OFFSET
    }

    /// Write SEC field
    #[inline(always)]
    pub fn write_sec(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::SEC_MASK) | ((value << Self::SEC_OFFSET) & Self::SEC_MASK));
    }

}

/// SW_LOCK12 value type for batch read/modify/write operations
///
/// Use `swlock12.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `swlock12.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SwLock12Val {
    bits: u32,
}

impl SwLock12Val {
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

    /// Read NSEC [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub const fn nsec(&self) -> u32 {
        (self.bits & SwLock12::NSEC_MASK) >> SwLock12::NSEC_OFFSET
    }

    /// Read SEC [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub const fn sec(&self) -> u32 {
        (self.bits & SwLock12::SEC_MASK) >> SwLock12::SEC_OFFSET
    }

    /// Set NSEC [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub const fn with_nsec(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SwLock12::NSEC_MASK) | ((value << SwLock12::NSEC_OFFSET) & SwLock12::NSEC_MASK)
        }
    }

    /// Set SEC [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub const fn with_sec(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SwLock12::SEC_MASK) | ((value << SwLock12::SEC_OFFSET) & SwLock12::SEC_MASK)
        }
    }

}

impl Default for SwLock12Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SwLock12 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> SwLock12Val {
        SwLock12Val::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: SwLock12Val) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(SwLock12Val) -> SwLock12Val>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// SW_LOCK13 Register
///
/// Software lock register for page 13. Locks are initialised from the OTP lock
/// pages at reset. This register can be written to further advance the lock
/// state of each page (until next reset), and read to check the current lock
/// state of a page.
///
/// Address offset: 0x0034
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **NSEC** [3:2] (RW): Non-secure lock status. Writes are OR'd with the current value.
/// - **SEC** [1:0] (RW): Secure lock status. Writes are OR'd with the current value. This field is r...
#[repr(transparent)]
pub struct SwLock13 {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for SwLock13 {}
unsafe impl Sync for SwLock13 {}

impl SwLock13 {
    // ======== Field Constants ========

    /// NSEC - Non-secure lock status. Writes are OR'd with the current value.
    /// Bits [3:2], mask 0x0000000c
    pub const NSEC_OFFSET: u32 = 2;
    pub const NSEC_MASK: u32 = 0x0000000c;
    pub const NSEC_READ_WRITE: u32 = 0 << 2;
    pub const NSEC_READ_ONLY: u32 = 1 << 2;
    pub const NSEC_INACCESSIBLE: u32 = 3 << 2;

    /// SEC - Secure lock status. Writes are OR'd with the current value. This field is r...
    /// Bits [1:0], mask 0x00000003
    pub const SEC_OFFSET: u32 = 0;
    pub const SEC_MASK: u32 = 0x00000003;
    pub const SEC_READ_WRITE: u32 = 0 << 0;
    pub const SEC_READ_ONLY: u32 = 1 << 0;
    pub const SEC_INACCESSIBLE: u32 = 3 << 0;

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

    /// **NSEC** [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub fn nsec(&self) -> u32 {
        (self.read_bits() & Self::NSEC_MASK) >> Self::NSEC_OFFSET
    }

    /// Write NSEC field
    #[inline(always)]
    pub fn write_nsec(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::NSEC_MASK) | ((value << Self::NSEC_OFFSET) & Self::NSEC_MASK));
    }

    /// **SEC** [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub fn sec(&self) -> u32 {
        (self.read_bits() & Self::SEC_MASK) >> Self::SEC_OFFSET
    }

    /// Write SEC field
    #[inline(always)]
    pub fn write_sec(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::SEC_MASK) | ((value << Self::SEC_OFFSET) & Self::SEC_MASK));
    }

}

/// SW_LOCK13 value type for batch read/modify/write operations
///
/// Use `swlock13.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `swlock13.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SwLock13Val {
    bits: u32,
}

impl SwLock13Val {
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

    /// Read NSEC [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub const fn nsec(&self) -> u32 {
        (self.bits & SwLock13::NSEC_MASK) >> SwLock13::NSEC_OFFSET
    }

    /// Read SEC [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub const fn sec(&self) -> u32 {
        (self.bits & SwLock13::SEC_MASK) >> SwLock13::SEC_OFFSET
    }

    /// Set NSEC [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub const fn with_nsec(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SwLock13::NSEC_MASK) | ((value << SwLock13::NSEC_OFFSET) & SwLock13::NSEC_MASK)
        }
    }

    /// Set SEC [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub const fn with_sec(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SwLock13::SEC_MASK) | ((value << SwLock13::SEC_OFFSET) & SwLock13::SEC_MASK)
        }
    }

}

impl Default for SwLock13Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SwLock13 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> SwLock13Val {
        SwLock13Val::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: SwLock13Val) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(SwLock13Val) -> SwLock13Val>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// SW_LOCK14 Register
///
/// Software lock register for page 14. Locks are initialised from the OTP lock
/// pages at reset. This register can be written to further advance the lock
/// state of each page (until next reset), and read to check the current lock
/// state of a page.
///
/// Address offset: 0x0038
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **NSEC** [3:2] (RW): Non-secure lock status. Writes are OR'd with the current value.
/// - **SEC** [1:0] (RW): Secure lock status. Writes are OR'd with the current value. This field is r...
#[repr(transparent)]
pub struct SwLock14 {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for SwLock14 {}
unsafe impl Sync for SwLock14 {}

impl SwLock14 {
    // ======== Field Constants ========

    /// NSEC - Non-secure lock status. Writes are OR'd with the current value.
    /// Bits [3:2], mask 0x0000000c
    pub const NSEC_OFFSET: u32 = 2;
    pub const NSEC_MASK: u32 = 0x0000000c;
    pub const NSEC_READ_WRITE: u32 = 0 << 2;
    pub const NSEC_READ_ONLY: u32 = 1 << 2;
    pub const NSEC_INACCESSIBLE: u32 = 3 << 2;

    /// SEC - Secure lock status. Writes are OR'd with the current value. This field is r...
    /// Bits [1:0], mask 0x00000003
    pub const SEC_OFFSET: u32 = 0;
    pub const SEC_MASK: u32 = 0x00000003;
    pub const SEC_READ_WRITE: u32 = 0 << 0;
    pub const SEC_READ_ONLY: u32 = 1 << 0;
    pub const SEC_INACCESSIBLE: u32 = 3 << 0;

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

    /// **NSEC** [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub fn nsec(&self) -> u32 {
        (self.read_bits() & Self::NSEC_MASK) >> Self::NSEC_OFFSET
    }

    /// Write NSEC field
    #[inline(always)]
    pub fn write_nsec(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::NSEC_MASK) | ((value << Self::NSEC_OFFSET) & Self::NSEC_MASK));
    }

    /// **SEC** [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub fn sec(&self) -> u32 {
        (self.read_bits() & Self::SEC_MASK) >> Self::SEC_OFFSET
    }

    /// Write SEC field
    #[inline(always)]
    pub fn write_sec(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::SEC_MASK) | ((value << Self::SEC_OFFSET) & Self::SEC_MASK));
    }

}

/// SW_LOCK14 value type for batch read/modify/write operations
///
/// Use `swlock14.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `swlock14.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SwLock14Val {
    bits: u32,
}

impl SwLock14Val {
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

    /// Read NSEC [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub const fn nsec(&self) -> u32 {
        (self.bits & SwLock14::NSEC_MASK) >> SwLock14::NSEC_OFFSET
    }

    /// Read SEC [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub const fn sec(&self) -> u32 {
        (self.bits & SwLock14::SEC_MASK) >> SwLock14::SEC_OFFSET
    }

    /// Set NSEC [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub const fn with_nsec(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SwLock14::NSEC_MASK) | ((value << SwLock14::NSEC_OFFSET) & SwLock14::NSEC_MASK)
        }
    }

    /// Set SEC [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub const fn with_sec(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SwLock14::SEC_MASK) | ((value << SwLock14::SEC_OFFSET) & SwLock14::SEC_MASK)
        }
    }

}

impl Default for SwLock14Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SwLock14 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> SwLock14Val {
        SwLock14Val::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: SwLock14Val) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(SwLock14Val) -> SwLock14Val>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// SW_LOCK15 Register
///
/// Software lock register for page 15. Locks are initialised from the OTP lock
/// pages at reset. This register can be written to further advance the lock
/// state of each page (until next reset), and read to check the current lock
/// state of a page.
///
/// Address offset: 0x003c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **NSEC** [3:2] (RW): Non-secure lock status. Writes are OR'd with the current value.
/// - **SEC** [1:0] (RW): Secure lock status. Writes are OR'd with the current value. This field is r...
#[repr(transparent)]
pub struct SwLock15 {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for SwLock15 {}
unsafe impl Sync for SwLock15 {}

impl SwLock15 {
    // ======== Field Constants ========

    /// NSEC - Non-secure lock status. Writes are OR'd with the current value.
    /// Bits [3:2], mask 0x0000000c
    pub const NSEC_OFFSET: u32 = 2;
    pub const NSEC_MASK: u32 = 0x0000000c;
    pub const NSEC_READ_WRITE: u32 = 0 << 2;
    pub const NSEC_READ_ONLY: u32 = 1 << 2;
    pub const NSEC_INACCESSIBLE: u32 = 3 << 2;

    /// SEC - Secure lock status. Writes are OR'd with the current value. This field is r...
    /// Bits [1:0], mask 0x00000003
    pub const SEC_OFFSET: u32 = 0;
    pub const SEC_MASK: u32 = 0x00000003;
    pub const SEC_READ_WRITE: u32 = 0 << 0;
    pub const SEC_READ_ONLY: u32 = 1 << 0;
    pub const SEC_INACCESSIBLE: u32 = 3 << 0;

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

    /// **NSEC** [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub fn nsec(&self) -> u32 {
        (self.read_bits() & Self::NSEC_MASK) >> Self::NSEC_OFFSET
    }

    /// Write NSEC field
    #[inline(always)]
    pub fn write_nsec(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::NSEC_MASK) | ((value << Self::NSEC_OFFSET) & Self::NSEC_MASK));
    }

    /// **SEC** [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub fn sec(&self) -> u32 {
        (self.read_bits() & Self::SEC_MASK) >> Self::SEC_OFFSET
    }

    /// Write SEC field
    #[inline(always)]
    pub fn write_sec(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::SEC_MASK) | ((value << Self::SEC_OFFSET) & Self::SEC_MASK));
    }

}

/// SW_LOCK15 value type for batch read/modify/write operations
///
/// Use `swlock15.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `swlock15.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SwLock15Val {
    bits: u32,
}

impl SwLock15Val {
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

    /// Read NSEC [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub const fn nsec(&self) -> u32 {
        (self.bits & SwLock15::NSEC_MASK) >> SwLock15::NSEC_OFFSET
    }

    /// Read SEC [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub const fn sec(&self) -> u32 {
        (self.bits & SwLock15::SEC_MASK) >> SwLock15::SEC_OFFSET
    }

    /// Set NSEC [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub const fn with_nsec(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SwLock15::NSEC_MASK) | ((value << SwLock15::NSEC_OFFSET) & SwLock15::NSEC_MASK)
        }
    }

    /// Set SEC [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub const fn with_sec(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SwLock15::SEC_MASK) | ((value << SwLock15::SEC_OFFSET) & SwLock15::SEC_MASK)
        }
    }

}

impl Default for SwLock15Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SwLock15 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> SwLock15Val {
        SwLock15Val::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: SwLock15Val) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(SwLock15Val) -> SwLock15Val>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// SW_LOCK16 Register
///
/// Software lock register for page 16. Locks are initialised from the OTP lock
/// pages at reset. This register can be written to further advance the lock
/// state of each page (until next reset), and read to check the current lock
/// state of a page.
///
/// Address offset: 0x0040
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **NSEC** [3:2] (RW): Non-secure lock status. Writes are OR'd with the current value.
/// - **SEC** [1:0] (RW): Secure lock status. Writes are OR'd with the current value. This field is r...
#[repr(transparent)]
pub struct SwLock16 {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for SwLock16 {}
unsafe impl Sync for SwLock16 {}

impl SwLock16 {
    // ======== Field Constants ========

    /// NSEC - Non-secure lock status. Writes are OR'd with the current value.
    /// Bits [3:2], mask 0x0000000c
    pub const NSEC_OFFSET: u32 = 2;
    pub const NSEC_MASK: u32 = 0x0000000c;
    pub const NSEC_READ_WRITE: u32 = 0 << 2;
    pub const NSEC_READ_ONLY: u32 = 1 << 2;
    pub const NSEC_INACCESSIBLE: u32 = 3 << 2;

    /// SEC - Secure lock status. Writes are OR'd with the current value. This field is r...
    /// Bits [1:0], mask 0x00000003
    pub const SEC_OFFSET: u32 = 0;
    pub const SEC_MASK: u32 = 0x00000003;
    pub const SEC_READ_WRITE: u32 = 0 << 0;
    pub const SEC_READ_ONLY: u32 = 1 << 0;
    pub const SEC_INACCESSIBLE: u32 = 3 << 0;

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

    /// **NSEC** [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub fn nsec(&self) -> u32 {
        (self.read_bits() & Self::NSEC_MASK) >> Self::NSEC_OFFSET
    }

    /// Write NSEC field
    #[inline(always)]
    pub fn write_nsec(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::NSEC_MASK) | ((value << Self::NSEC_OFFSET) & Self::NSEC_MASK));
    }

    /// **SEC** [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub fn sec(&self) -> u32 {
        (self.read_bits() & Self::SEC_MASK) >> Self::SEC_OFFSET
    }

    /// Write SEC field
    #[inline(always)]
    pub fn write_sec(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::SEC_MASK) | ((value << Self::SEC_OFFSET) & Self::SEC_MASK));
    }

}

/// SW_LOCK16 value type for batch read/modify/write operations
///
/// Use `swlock16.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `swlock16.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SwLock16Val {
    bits: u32,
}

impl SwLock16Val {
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

    /// Read NSEC [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub const fn nsec(&self) -> u32 {
        (self.bits & SwLock16::NSEC_MASK) >> SwLock16::NSEC_OFFSET
    }

    /// Read SEC [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub const fn sec(&self) -> u32 {
        (self.bits & SwLock16::SEC_MASK) >> SwLock16::SEC_OFFSET
    }

    /// Set NSEC [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub const fn with_nsec(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SwLock16::NSEC_MASK) | ((value << SwLock16::NSEC_OFFSET) & SwLock16::NSEC_MASK)
        }
    }

    /// Set SEC [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub const fn with_sec(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SwLock16::SEC_MASK) | ((value << SwLock16::SEC_OFFSET) & SwLock16::SEC_MASK)
        }
    }

}

impl Default for SwLock16Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SwLock16 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> SwLock16Val {
        SwLock16Val::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: SwLock16Val) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(SwLock16Val) -> SwLock16Val>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// SW_LOCK17 Register
///
/// Software lock register for page 17. Locks are initialised from the OTP lock
/// pages at reset. This register can be written to further advance the lock
/// state of each page (until next reset), and read to check the current lock
/// state of a page.
///
/// Address offset: 0x0044
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **NSEC** [3:2] (RW): Non-secure lock status. Writes are OR'd with the current value.
/// - **SEC** [1:0] (RW): Secure lock status. Writes are OR'd with the current value. This field is r...
#[repr(transparent)]
pub struct SwLock17 {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for SwLock17 {}
unsafe impl Sync for SwLock17 {}

impl SwLock17 {
    // ======== Field Constants ========

    /// NSEC - Non-secure lock status. Writes are OR'd with the current value.
    /// Bits [3:2], mask 0x0000000c
    pub const NSEC_OFFSET: u32 = 2;
    pub const NSEC_MASK: u32 = 0x0000000c;
    pub const NSEC_READ_WRITE: u32 = 0 << 2;
    pub const NSEC_READ_ONLY: u32 = 1 << 2;
    pub const NSEC_INACCESSIBLE: u32 = 3 << 2;

    /// SEC - Secure lock status. Writes are OR'd with the current value. This field is r...
    /// Bits [1:0], mask 0x00000003
    pub const SEC_OFFSET: u32 = 0;
    pub const SEC_MASK: u32 = 0x00000003;
    pub const SEC_READ_WRITE: u32 = 0 << 0;
    pub const SEC_READ_ONLY: u32 = 1 << 0;
    pub const SEC_INACCESSIBLE: u32 = 3 << 0;

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

    /// **NSEC** [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub fn nsec(&self) -> u32 {
        (self.read_bits() & Self::NSEC_MASK) >> Self::NSEC_OFFSET
    }

    /// Write NSEC field
    #[inline(always)]
    pub fn write_nsec(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::NSEC_MASK) | ((value << Self::NSEC_OFFSET) & Self::NSEC_MASK));
    }

    /// **SEC** [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub fn sec(&self) -> u32 {
        (self.read_bits() & Self::SEC_MASK) >> Self::SEC_OFFSET
    }

    /// Write SEC field
    #[inline(always)]
    pub fn write_sec(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::SEC_MASK) | ((value << Self::SEC_OFFSET) & Self::SEC_MASK));
    }

}

/// SW_LOCK17 value type for batch read/modify/write operations
///
/// Use `swlock17.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `swlock17.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SwLock17Val {
    bits: u32,
}

impl SwLock17Val {
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

    /// Read NSEC [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub const fn nsec(&self) -> u32 {
        (self.bits & SwLock17::NSEC_MASK) >> SwLock17::NSEC_OFFSET
    }

    /// Read SEC [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub const fn sec(&self) -> u32 {
        (self.bits & SwLock17::SEC_MASK) >> SwLock17::SEC_OFFSET
    }

    /// Set NSEC [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub const fn with_nsec(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SwLock17::NSEC_MASK) | ((value << SwLock17::NSEC_OFFSET) & SwLock17::NSEC_MASK)
        }
    }

    /// Set SEC [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub const fn with_sec(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SwLock17::SEC_MASK) | ((value << SwLock17::SEC_OFFSET) & SwLock17::SEC_MASK)
        }
    }

}

impl Default for SwLock17Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SwLock17 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> SwLock17Val {
        SwLock17Val::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: SwLock17Val) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(SwLock17Val) -> SwLock17Val>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// SW_LOCK18 Register
///
/// Software lock register for page 18. Locks are initialised from the OTP lock
/// pages at reset. This register can be written to further advance the lock
/// state of each page (until next reset), and read to check the current lock
/// state of a page.
///
/// Address offset: 0x0048
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **NSEC** [3:2] (RW): Non-secure lock status. Writes are OR'd with the current value.
/// - **SEC** [1:0] (RW): Secure lock status. Writes are OR'd with the current value. This field is r...
#[repr(transparent)]
pub struct SwLock18 {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for SwLock18 {}
unsafe impl Sync for SwLock18 {}

impl SwLock18 {
    // ======== Field Constants ========

    /// NSEC - Non-secure lock status. Writes are OR'd with the current value.
    /// Bits [3:2], mask 0x0000000c
    pub const NSEC_OFFSET: u32 = 2;
    pub const NSEC_MASK: u32 = 0x0000000c;
    pub const NSEC_READ_WRITE: u32 = 0 << 2;
    pub const NSEC_READ_ONLY: u32 = 1 << 2;
    pub const NSEC_INACCESSIBLE: u32 = 3 << 2;

    /// SEC - Secure lock status. Writes are OR'd with the current value. This field is r...
    /// Bits [1:0], mask 0x00000003
    pub const SEC_OFFSET: u32 = 0;
    pub const SEC_MASK: u32 = 0x00000003;
    pub const SEC_READ_WRITE: u32 = 0 << 0;
    pub const SEC_READ_ONLY: u32 = 1 << 0;
    pub const SEC_INACCESSIBLE: u32 = 3 << 0;

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

    /// **NSEC** [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub fn nsec(&self) -> u32 {
        (self.read_bits() & Self::NSEC_MASK) >> Self::NSEC_OFFSET
    }

    /// Write NSEC field
    #[inline(always)]
    pub fn write_nsec(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::NSEC_MASK) | ((value << Self::NSEC_OFFSET) & Self::NSEC_MASK));
    }

    /// **SEC** [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub fn sec(&self) -> u32 {
        (self.read_bits() & Self::SEC_MASK) >> Self::SEC_OFFSET
    }

    /// Write SEC field
    #[inline(always)]
    pub fn write_sec(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::SEC_MASK) | ((value << Self::SEC_OFFSET) & Self::SEC_MASK));
    }

}

/// SW_LOCK18 value type for batch read/modify/write operations
///
/// Use `swlock18.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `swlock18.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SwLock18Val {
    bits: u32,
}

impl SwLock18Val {
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

    /// Read NSEC [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub const fn nsec(&self) -> u32 {
        (self.bits & SwLock18::NSEC_MASK) >> SwLock18::NSEC_OFFSET
    }

    /// Read SEC [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub const fn sec(&self) -> u32 {
        (self.bits & SwLock18::SEC_MASK) >> SwLock18::SEC_OFFSET
    }

    /// Set NSEC [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub const fn with_nsec(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SwLock18::NSEC_MASK) | ((value << SwLock18::NSEC_OFFSET) & SwLock18::NSEC_MASK)
        }
    }

    /// Set SEC [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub const fn with_sec(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SwLock18::SEC_MASK) | ((value << SwLock18::SEC_OFFSET) & SwLock18::SEC_MASK)
        }
    }

}

impl Default for SwLock18Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SwLock18 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> SwLock18Val {
        SwLock18Val::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: SwLock18Val) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(SwLock18Val) -> SwLock18Val>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// SW_LOCK19 Register
///
/// Software lock register for page 19. Locks are initialised from the OTP lock
/// pages at reset. This register can be written to further advance the lock
/// state of each page (until next reset), and read to check the current lock
/// state of a page.
///
/// Address offset: 0x004c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **NSEC** [3:2] (RW): Non-secure lock status. Writes are OR'd with the current value.
/// - **SEC** [1:0] (RW): Secure lock status. Writes are OR'd with the current value. This field is r...
#[repr(transparent)]
pub struct SwLock19 {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for SwLock19 {}
unsafe impl Sync for SwLock19 {}

impl SwLock19 {
    // ======== Field Constants ========

    /// NSEC - Non-secure lock status. Writes are OR'd with the current value.
    /// Bits [3:2], mask 0x0000000c
    pub const NSEC_OFFSET: u32 = 2;
    pub const NSEC_MASK: u32 = 0x0000000c;
    pub const NSEC_READ_WRITE: u32 = 0 << 2;
    pub const NSEC_READ_ONLY: u32 = 1 << 2;
    pub const NSEC_INACCESSIBLE: u32 = 3 << 2;

    /// SEC - Secure lock status. Writes are OR'd with the current value. This field is r...
    /// Bits [1:0], mask 0x00000003
    pub const SEC_OFFSET: u32 = 0;
    pub const SEC_MASK: u32 = 0x00000003;
    pub const SEC_READ_WRITE: u32 = 0 << 0;
    pub const SEC_READ_ONLY: u32 = 1 << 0;
    pub const SEC_INACCESSIBLE: u32 = 3 << 0;

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

    /// **NSEC** [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub fn nsec(&self) -> u32 {
        (self.read_bits() & Self::NSEC_MASK) >> Self::NSEC_OFFSET
    }

    /// Write NSEC field
    #[inline(always)]
    pub fn write_nsec(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::NSEC_MASK) | ((value << Self::NSEC_OFFSET) & Self::NSEC_MASK));
    }

    /// **SEC** [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub fn sec(&self) -> u32 {
        (self.read_bits() & Self::SEC_MASK) >> Self::SEC_OFFSET
    }

    /// Write SEC field
    #[inline(always)]
    pub fn write_sec(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::SEC_MASK) | ((value << Self::SEC_OFFSET) & Self::SEC_MASK));
    }

}

/// SW_LOCK19 value type for batch read/modify/write operations
///
/// Use `swlock19.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `swlock19.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SwLock19Val {
    bits: u32,
}

impl SwLock19Val {
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

    /// Read NSEC [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub const fn nsec(&self) -> u32 {
        (self.bits & SwLock19::NSEC_MASK) >> SwLock19::NSEC_OFFSET
    }

    /// Read SEC [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub const fn sec(&self) -> u32 {
        (self.bits & SwLock19::SEC_MASK) >> SwLock19::SEC_OFFSET
    }

    /// Set NSEC [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub const fn with_nsec(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SwLock19::NSEC_MASK) | ((value << SwLock19::NSEC_OFFSET) & SwLock19::NSEC_MASK)
        }
    }

    /// Set SEC [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub const fn with_sec(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SwLock19::SEC_MASK) | ((value << SwLock19::SEC_OFFSET) & SwLock19::SEC_MASK)
        }
    }

}

impl Default for SwLock19Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SwLock19 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> SwLock19Val {
        SwLock19Val::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: SwLock19Val) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(SwLock19Val) -> SwLock19Val>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// SW_LOCK20 Register
///
/// Software lock register for page 20. Locks are initialised from the OTP lock
/// pages at reset. This register can be written to further advance the lock
/// state of each page (until next reset), and read to check the current lock
/// state of a page.
///
/// Address offset: 0x0050
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **NSEC** [3:2] (RW): Non-secure lock status. Writes are OR'd with the current value.
/// - **SEC** [1:0] (RW): Secure lock status. Writes are OR'd with the current value. This field is r...
#[repr(transparent)]
pub struct SwLock20 {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for SwLock20 {}
unsafe impl Sync for SwLock20 {}

impl SwLock20 {
    // ======== Field Constants ========

    /// NSEC - Non-secure lock status. Writes are OR'd with the current value.
    /// Bits [3:2], mask 0x0000000c
    pub const NSEC_OFFSET: u32 = 2;
    pub const NSEC_MASK: u32 = 0x0000000c;
    pub const NSEC_READ_WRITE: u32 = 0 << 2;
    pub const NSEC_READ_ONLY: u32 = 1 << 2;
    pub const NSEC_INACCESSIBLE: u32 = 3 << 2;

    /// SEC - Secure lock status. Writes are OR'd with the current value. This field is r...
    /// Bits [1:0], mask 0x00000003
    pub const SEC_OFFSET: u32 = 0;
    pub const SEC_MASK: u32 = 0x00000003;
    pub const SEC_READ_WRITE: u32 = 0 << 0;
    pub const SEC_READ_ONLY: u32 = 1 << 0;
    pub const SEC_INACCESSIBLE: u32 = 3 << 0;

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

    /// **NSEC** [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub fn nsec(&self) -> u32 {
        (self.read_bits() & Self::NSEC_MASK) >> Self::NSEC_OFFSET
    }

    /// Write NSEC field
    #[inline(always)]
    pub fn write_nsec(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::NSEC_MASK) | ((value << Self::NSEC_OFFSET) & Self::NSEC_MASK));
    }

    /// **SEC** [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub fn sec(&self) -> u32 {
        (self.read_bits() & Self::SEC_MASK) >> Self::SEC_OFFSET
    }

    /// Write SEC field
    #[inline(always)]
    pub fn write_sec(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::SEC_MASK) | ((value << Self::SEC_OFFSET) & Self::SEC_MASK));
    }

}

/// SW_LOCK20 value type for batch read/modify/write operations
///
/// Use `swlock20.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `swlock20.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SwLock20Val {
    bits: u32,
}

impl SwLock20Val {
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

    /// Read NSEC [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub const fn nsec(&self) -> u32 {
        (self.bits & SwLock20::NSEC_MASK) >> SwLock20::NSEC_OFFSET
    }

    /// Read SEC [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub const fn sec(&self) -> u32 {
        (self.bits & SwLock20::SEC_MASK) >> SwLock20::SEC_OFFSET
    }

    /// Set NSEC [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub const fn with_nsec(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SwLock20::NSEC_MASK) | ((value << SwLock20::NSEC_OFFSET) & SwLock20::NSEC_MASK)
        }
    }

    /// Set SEC [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub const fn with_sec(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SwLock20::SEC_MASK) | ((value << SwLock20::SEC_OFFSET) & SwLock20::SEC_MASK)
        }
    }

}

impl Default for SwLock20Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SwLock20 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> SwLock20Val {
        SwLock20Val::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: SwLock20Val) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(SwLock20Val) -> SwLock20Val>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// SW_LOCK21 Register
///
/// Software lock register for page 21. Locks are initialised from the OTP lock
/// pages at reset. This register can be written to further advance the lock
/// state of each page (until next reset), and read to check the current lock
/// state of a page.
///
/// Address offset: 0x0054
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **NSEC** [3:2] (RW): Non-secure lock status. Writes are OR'd with the current value.
/// - **SEC** [1:0] (RW): Secure lock status. Writes are OR'd with the current value. This field is r...
#[repr(transparent)]
pub struct SwLock21 {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for SwLock21 {}
unsafe impl Sync for SwLock21 {}

impl SwLock21 {
    // ======== Field Constants ========

    /// NSEC - Non-secure lock status. Writes are OR'd with the current value.
    /// Bits [3:2], mask 0x0000000c
    pub const NSEC_OFFSET: u32 = 2;
    pub const NSEC_MASK: u32 = 0x0000000c;
    pub const NSEC_READ_WRITE: u32 = 0 << 2;
    pub const NSEC_READ_ONLY: u32 = 1 << 2;
    pub const NSEC_INACCESSIBLE: u32 = 3 << 2;

    /// SEC - Secure lock status. Writes are OR'd with the current value. This field is r...
    /// Bits [1:0], mask 0x00000003
    pub const SEC_OFFSET: u32 = 0;
    pub const SEC_MASK: u32 = 0x00000003;
    pub const SEC_READ_WRITE: u32 = 0 << 0;
    pub const SEC_READ_ONLY: u32 = 1 << 0;
    pub const SEC_INACCESSIBLE: u32 = 3 << 0;

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

    /// **NSEC** [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub fn nsec(&self) -> u32 {
        (self.read_bits() & Self::NSEC_MASK) >> Self::NSEC_OFFSET
    }

    /// Write NSEC field
    #[inline(always)]
    pub fn write_nsec(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::NSEC_MASK) | ((value << Self::NSEC_OFFSET) & Self::NSEC_MASK));
    }

    /// **SEC** [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub fn sec(&self) -> u32 {
        (self.read_bits() & Self::SEC_MASK) >> Self::SEC_OFFSET
    }

    /// Write SEC field
    #[inline(always)]
    pub fn write_sec(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::SEC_MASK) | ((value << Self::SEC_OFFSET) & Self::SEC_MASK));
    }

}

/// SW_LOCK21 value type for batch read/modify/write operations
///
/// Use `swlock21.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `swlock21.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SwLock21Val {
    bits: u32,
}

impl SwLock21Val {
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

    /// Read NSEC [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub const fn nsec(&self) -> u32 {
        (self.bits & SwLock21::NSEC_MASK) >> SwLock21::NSEC_OFFSET
    }

    /// Read SEC [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub const fn sec(&self) -> u32 {
        (self.bits & SwLock21::SEC_MASK) >> SwLock21::SEC_OFFSET
    }

    /// Set NSEC [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub const fn with_nsec(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SwLock21::NSEC_MASK) | ((value << SwLock21::NSEC_OFFSET) & SwLock21::NSEC_MASK)
        }
    }

    /// Set SEC [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub const fn with_sec(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SwLock21::SEC_MASK) | ((value << SwLock21::SEC_OFFSET) & SwLock21::SEC_MASK)
        }
    }

}

impl Default for SwLock21Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SwLock21 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> SwLock21Val {
        SwLock21Val::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: SwLock21Val) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(SwLock21Val) -> SwLock21Val>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// SW_LOCK22 Register
///
/// Software lock register for page 22. Locks are initialised from the OTP lock
/// pages at reset. This register can be written to further advance the lock
/// state of each page (until next reset), and read to check the current lock
/// state of a page.
///
/// Address offset: 0x0058
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **NSEC** [3:2] (RW): Non-secure lock status. Writes are OR'd with the current value.
/// - **SEC** [1:0] (RW): Secure lock status. Writes are OR'd with the current value. This field is r...
#[repr(transparent)]
pub struct SwLock22 {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for SwLock22 {}
unsafe impl Sync for SwLock22 {}

impl SwLock22 {
    // ======== Field Constants ========

    /// NSEC - Non-secure lock status. Writes are OR'd with the current value.
    /// Bits [3:2], mask 0x0000000c
    pub const NSEC_OFFSET: u32 = 2;
    pub const NSEC_MASK: u32 = 0x0000000c;
    pub const NSEC_READ_WRITE: u32 = 0 << 2;
    pub const NSEC_READ_ONLY: u32 = 1 << 2;
    pub const NSEC_INACCESSIBLE: u32 = 3 << 2;

    /// SEC - Secure lock status. Writes are OR'd with the current value. This field is r...
    /// Bits [1:0], mask 0x00000003
    pub const SEC_OFFSET: u32 = 0;
    pub const SEC_MASK: u32 = 0x00000003;
    pub const SEC_READ_WRITE: u32 = 0 << 0;
    pub const SEC_READ_ONLY: u32 = 1 << 0;
    pub const SEC_INACCESSIBLE: u32 = 3 << 0;

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

    /// **NSEC** [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub fn nsec(&self) -> u32 {
        (self.read_bits() & Self::NSEC_MASK) >> Self::NSEC_OFFSET
    }

    /// Write NSEC field
    #[inline(always)]
    pub fn write_nsec(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::NSEC_MASK) | ((value << Self::NSEC_OFFSET) & Self::NSEC_MASK));
    }

    /// **SEC** [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub fn sec(&self) -> u32 {
        (self.read_bits() & Self::SEC_MASK) >> Self::SEC_OFFSET
    }

    /// Write SEC field
    #[inline(always)]
    pub fn write_sec(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::SEC_MASK) | ((value << Self::SEC_OFFSET) & Self::SEC_MASK));
    }

}

/// SW_LOCK22 value type for batch read/modify/write operations
///
/// Use `swlock22.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `swlock22.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SwLock22Val {
    bits: u32,
}

impl SwLock22Val {
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

    /// Read NSEC [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub const fn nsec(&self) -> u32 {
        (self.bits & SwLock22::NSEC_MASK) >> SwLock22::NSEC_OFFSET
    }

    /// Read SEC [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub const fn sec(&self) -> u32 {
        (self.bits & SwLock22::SEC_MASK) >> SwLock22::SEC_OFFSET
    }

    /// Set NSEC [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub const fn with_nsec(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SwLock22::NSEC_MASK) | ((value << SwLock22::NSEC_OFFSET) & SwLock22::NSEC_MASK)
        }
    }

    /// Set SEC [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub const fn with_sec(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SwLock22::SEC_MASK) | ((value << SwLock22::SEC_OFFSET) & SwLock22::SEC_MASK)
        }
    }

}

impl Default for SwLock22Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SwLock22 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> SwLock22Val {
        SwLock22Val::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: SwLock22Val) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(SwLock22Val) -> SwLock22Val>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// SW_LOCK23 Register
///
/// Software lock register for page 23. Locks are initialised from the OTP lock
/// pages at reset. This register can be written to further advance the lock
/// state of each page (until next reset), and read to check the current lock
/// state of a page.
///
/// Address offset: 0x005c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **NSEC** [3:2] (RW): Non-secure lock status. Writes are OR'd with the current value.
/// - **SEC** [1:0] (RW): Secure lock status. Writes are OR'd with the current value. This field is r...
#[repr(transparent)]
pub struct SwLock23 {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for SwLock23 {}
unsafe impl Sync for SwLock23 {}

impl SwLock23 {
    // ======== Field Constants ========

    /// NSEC - Non-secure lock status. Writes are OR'd with the current value.
    /// Bits [3:2], mask 0x0000000c
    pub const NSEC_OFFSET: u32 = 2;
    pub const NSEC_MASK: u32 = 0x0000000c;
    pub const NSEC_READ_WRITE: u32 = 0 << 2;
    pub const NSEC_READ_ONLY: u32 = 1 << 2;
    pub const NSEC_INACCESSIBLE: u32 = 3 << 2;

    /// SEC - Secure lock status. Writes are OR'd with the current value. This field is r...
    /// Bits [1:0], mask 0x00000003
    pub const SEC_OFFSET: u32 = 0;
    pub const SEC_MASK: u32 = 0x00000003;
    pub const SEC_READ_WRITE: u32 = 0 << 0;
    pub const SEC_READ_ONLY: u32 = 1 << 0;
    pub const SEC_INACCESSIBLE: u32 = 3 << 0;

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

    /// **NSEC** [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub fn nsec(&self) -> u32 {
        (self.read_bits() & Self::NSEC_MASK) >> Self::NSEC_OFFSET
    }

    /// Write NSEC field
    #[inline(always)]
    pub fn write_nsec(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::NSEC_MASK) | ((value << Self::NSEC_OFFSET) & Self::NSEC_MASK));
    }

    /// **SEC** [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub fn sec(&self) -> u32 {
        (self.read_bits() & Self::SEC_MASK) >> Self::SEC_OFFSET
    }

    /// Write SEC field
    #[inline(always)]
    pub fn write_sec(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::SEC_MASK) | ((value << Self::SEC_OFFSET) & Self::SEC_MASK));
    }

}

/// SW_LOCK23 value type for batch read/modify/write operations
///
/// Use `swlock23.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `swlock23.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SwLock23Val {
    bits: u32,
}

impl SwLock23Val {
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

    /// Read NSEC [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub const fn nsec(&self) -> u32 {
        (self.bits & SwLock23::NSEC_MASK) >> SwLock23::NSEC_OFFSET
    }

    /// Read SEC [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub const fn sec(&self) -> u32 {
        (self.bits & SwLock23::SEC_MASK) >> SwLock23::SEC_OFFSET
    }

    /// Set NSEC [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub const fn with_nsec(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SwLock23::NSEC_MASK) | ((value << SwLock23::NSEC_OFFSET) & SwLock23::NSEC_MASK)
        }
    }

    /// Set SEC [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub const fn with_sec(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SwLock23::SEC_MASK) | ((value << SwLock23::SEC_OFFSET) & SwLock23::SEC_MASK)
        }
    }

}

impl Default for SwLock23Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SwLock23 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> SwLock23Val {
        SwLock23Val::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: SwLock23Val) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(SwLock23Val) -> SwLock23Val>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// SW_LOCK24 Register
///
/// Software lock register for page 24. Locks are initialised from the OTP lock
/// pages at reset. This register can be written to further advance the lock
/// state of each page (until next reset), and read to check the current lock
/// state of a page.
///
/// Address offset: 0x0060
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **NSEC** [3:2] (RW): Non-secure lock status. Writes are OR'd with the current value.
/// - **SEC** [1:0] (RW): Secure lock status. Writes are OR'd with the current value. This field is r...
#[repr(transparent)]
pub struct SwLock24 {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for SwLock24 {}
unsafe impl Sync for SwLock24 {}

impl SwLock24 {
    // ======== Field Constants ========

    /// NSEC - Non-secure lock status. Writes are OR'd with the current value.
    /// Bits [3:2], mask 0x0000000c
    pub const NSEC_OFFSET: u32 = 2;
    pub const NSEC_MASK: u32 = 0x0000000c;
    pub const NSEC_READ_WRITE: u32 = 0 << 2;
    pub const NSEC_READ_ONLY: u32 = 1 << 2;
    pub const NSEC_INACCESSIBLE: u32 = 3 << 2;

    /// SEC - Secure lock status. Writes are OR'd with the current value. This field is r...
    /// Bits [1:0], mask 0x00000003
    pub const SEC_OFFSET: u32 = 0;
    pub const SEC_MASK: u32 = 0x00000003;
    pub const SEC_READ_WRITE: u32 = 0 << 0;
    pub const SEC_READ_ONLY: u32 = 1 << 0;
    pub const SEC_INACCESSIBLE: u32 = 3 << 0;

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

    /// **NSEC** [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub fn nsec(&self) -> u32 {
        (self.read_bits() & Self::NSEC_MASK) >> Self::NSEC_OFFSET
    }

    /// Write NSEC field
    #[inline(always)]
    pub fn write_nsec(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::NSEC_MASK) | ((value << Self::NSEC_OFFSET) & Self::NSEC_MASK));
    }

    /// **SEC** [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub fn sec(&self) -> u32 {
        (self.read_bits() & Self::SEC_MASK) >> Self::SEC_OFFSET
    }

    /// Write SEC field
    #[inline(always)]
    pub fn write_sec(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::SEC_MASK) | ((value << Self::SEC_OFFSET) & Self::SEC_MASK));
    }

}

/// SW_LOCK24 value type for batch read/modify/write operations
///
/// Use `swlock24.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `swlock24.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SwLock24Val {
    bits: u32,
}

impl SwLock24Val {
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

    /// Read NSEC [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub const fn nsec(&self) -> u32 {
        (self.bits & SwLock24::NSEC_MASK) >> SwLock24::NSEC_OFFSET
    }

    /// Read SEC [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub const fn sec(&self) -> u32 {
        (self.bits & SwLock24::SEC_MASK) >> SwLock24::SEC_OFFSET
    }

    /// Set NSEC [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub const fn with_nsec(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SwLock24::NSEC_MASK) | ((value << SwLock24::NSEC_OFFSET) & SwLock24::NSEC_MASK)
        }
    }

    /// Set SEC [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub const fn with_sec(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SwLock24::SEC_MASK) | ((value << SwLock24::SEC_OFFSET) & SwLock24::SEC_MASK)
        }
    }

}

impl Default for SwLock24Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SwLock24 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> SwLock24Val {
        SwLock24Val::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: SwLock24Val) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(SwLock24Val) -> SwLock24Val>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// SW_LOCK25 Register
///
/// Software lock register for page 25. Locks are initialised from the OTP lock
/// pages at reset. This register can be written to further advance the lock
/// state of each page (until next reset), and read to check the current lock
/// state of a page.
///
/// Address offset: 0x0064
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **NSEC** [3:2] (RW): Non-secure lock status. Writes are OR'd with the current value.
/// - **SEC** [1:0] (RW): Secure lock status. Writes are OR'd with the current value. This field is r...
#[repr(transparent)]
pub struct SwLock25 {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for SwLock25 {}
unsafe impl Sync for SwLock25 {}

impl SwLock25 {
    // ======== Field Constants ========

    /// NSEC - Non-secure lock status. Writes are OR'd with the current value.
    /// Bits [3:2], mask 0x0000000c
    pub const NSEC_OFFSET: u32 = 2;
    pub const NSEC_MASK: u32 = 0x0000000c;
    pub const NSEC_READ_WRITE: u32 = 0 << 2;
    pub const NSEC_READ_ONLY: u32 = 1 << 2;
    pub const NSEC_INACCESSIBLE: u32 = 3 << 2;

    /// SEC - Secure lock status. Writes are OR'd with the current value. This field is r...
    /// Bits [1:0], mask 0x00000003
    pub const SEC_OFFSET: u32 = 0;
    pub const SEC_MASK: u32 = 0x00000003;
    pub const SEC_READ_WRITE: u32 = 0 << 0;
    pub const SEC_READ_ONLY: u32 = 1 << 0;
    pub const SEC_INACCESSIBLE: u32 = 3 << 0;

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

    /// **NSEC** [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub fn nsec(&self) -> u32 {
        (self.read_bits() & Self::NSEC_MASK) >> Self::NSEC_OFFSET
    }

    /// Write NSEC field
    #[inline(always)]
    pub fn write_nsec(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::NSEC_MASK) | ((value << Self::NSEC_OFFSET) & Self::NSEC_MASK));
    }

    /// **SEC** [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub fn sec(&self) -> u32 {
        (self.read_bits() & Self::SEC_MASK) >> Self::SEC_OFFSET
    }

    /// Write SEC field
    #[inline(always)]
    pub fn write_sec(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::SEC_MASK) | ((value << Self::SEC_OFFSET) & Self::SEC_MASK));
    }

}

/// SW_LOCK25 value type for batch read/modify/write operations
///
/// Use `swlock25.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `swlock25.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SwLock25Val {
    bits: u32,
}

impl SwLock25Val {
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

    /// Read NSEC [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub const fn nsec(&self) -> u32 {
        (self.bits & SwLock25::NSEC_MASK) >> SwLock25::NSEC_OFFSET
    }

    /// Read SEC [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub const fn sec(&self) -> u32 {
        (self.bits & SwLock25::SEC_MASK) >> SwLock25::SEC_OFFSET
    }

    /// Set NSEC [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub const fn with_nsec(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SwLock25::NSEC_MASK) | ((value << SwLock25::NSEC_OFFSET) & SwLock25::NSEC_MASK)
        }
    }

    /// Set SEC [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub const fn with_sec(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SwLock25::SEC_MASK) | ((value << SwLock25::SEC_OFFSET) & SwLock25::SEC_MASK)
        }
    }

}

impl Default for SwLock25Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SwLock25 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> SwLock25Val {
        SwLock25Val::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: SwLock25Val) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(SwLock25Val) -> SwLock25Val>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// SW_LOCK26 Register
///
/// Software lock register for page 26. Locks are initialised from the OTP lock
/// pages at reset. This register can be written to further advance the lock
/// state of each page (until next reset), and read to check the current lock
/// state of a page.
///
/// Address offset: 0x0068
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **NSEC** [3:2] (RW): Non-secure lock status. Writes are OR'd with the current value.
/// - **SEC** [1:0] (RW): Secure lock status. Writes are OR'd with the current value. This field is r...
#[repr(transparent)]
pub struct SwLock26 {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for SwLock26 {}
unsafe impl Sync for SwLock26 {}

impl SwLock26 {
    // ======== Field Constants ========

    /// NSEC - Non-secure lock status. Writes are OR'd with the current value.
    /// Bits [3:2], mask 0x0000000c
    pub const NSEC_OFFSET: u32 = 2;
    pub const NSEC_MASK: u32 = 0x0000000c;
    pub const NSEC_READ_WRITE: u32 = 0 << 2;
    pub const NSEC_READ_ONLY: u32 = 1 << 2;
    pub const NSEC_INACCESSIBLE: u32 = 3 << 2;

    /// SEC - Secure lock status. Writes are OR'd with the current value. This field is r...
    /// Bits [1:0], mask 0x00000003
    pub const SEC_OFFSET: u32 = 0;
    pub const SEC_MASK: u32 = 0x00000003;
    pub const SEC_READ_WRITE: u32 = 0 << 0;
    pub const SEC_READ_ONLY: u32 = 1 << 0;
    pub const SEC_INACCESSIBLE: u32 = 3 << 0;

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

    /// **NSEC** [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub fn nsec(&self) -> u32 {
        (self.read_bits() & Self::NSEC_MASK) >> Self::NSEC_OFFSET
    }

    /// Write NSEC field
    #[inline(always)]
    pub fn write_nsec(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::NSEC_MASK) | ((value << Self::NSEC_OFFSET) & Self::NSEC_MASK));
    }

    /// **SEC** [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub fn sec(&self) -> u32 {
        (self.read_bits() & Self::SEC_MASK) >> Self::SEC_OFFSET
    }

    /// Write SEC field
    #[inline(always)]
    pub fn write_sec(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::SEC_MASK) | ((value << Self::SEC_OFFSET) & Self::SEC_MASK));
    }

}

/// SW_LOCK26 value type for batch read/modify/write operations
///
/// Use `swlock26.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `swlock26.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SwLock26Val {
    bits: u32,
}

impl SwLock26Val {
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

    /// Read NSEC [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub const fn nsec(&self) -> u32 {
        (self.bits & SwLock26::NSEC_MASK) >> SwLock26::NSEC_OFFSET
    }

    /// Read SEC [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub const fn sec(&self) -> u32 {
        (self.bits & SwLock26::SEC_MASK) >> SwLock26::SEC_OFFSET
    }

    /// Set NSEC [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub const fn with_nsec(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SwLock26::NSEC_MASK) | ((value << SwLock26::NSEC_OFFSET) & SwLock26::NSEC_MASK)
        }
    }

    /// Set SEC [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub const fn with_sec(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SwLock26::SEC_MASK) | ((value << SwLock26::SEC_OFFSET) & SwLock26::SEC_MASK)
        }
    }

}

impl Default for SwLock26Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SwLock26 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> SwLock26Val {
        SwLock26Val::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: SwLock26Val) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(SwLock26Val) -> SwLock26Val>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// SW_LOCK27 Register
///
/// Software lock register for page 27. Locks are initialised from the OTP lock
/// pages at reset. This register can be written to further advance the lock
/// state of each page (until next reset), and read to check the current lock
/// state of a page.
///
/// Address offset: 0x006c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **NSEC** [3:2] (RW): Non-secure lock status. Writes are OR'd with the current value.
/// - **SEC** [1:0] (RW): Secure lock status. Writes are OR'd with the current value. This field is r...
#[repr(transparent)]
pub struct SwLock27 {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for SwLock27 {}
unsafe impl Sync for SwLock27 {}

impl SwLock27 {
    // ======== Field Constants ========

    /// NSEC - Non-secure lock status. Writes are OR'd with the current value.
    /// Bits [3:2], mask 0x0000000c
    pub const NSEC_OFFSET: u32 = 2;
    pub const NSEC_MASK: u32 = 0x0000000c;
    pub const NSEC_READ_WRITE: u32 = 0 << 2;
    pub const NSEC_READ_ONLY: u32 = 1 << 2;
    pub const NSEC_INACCESSIBLE: u32 = 3 << 2;

    /// SEC - Secure lock status. Writes are OR'd with the current value. This field is r...
    /// Bits [1:0], mask 0x00000003
    pub const SEC_OFFSET: u32 = 0;
    pub const SEC_MASK: u32 = 0x00000003;
    pub const SEC_READ_WRITE: u32 = 0 << 0;
    pub const SEC_READ_ONLY: u32 = 1 << 0;
    pub const SEC_INACCESSIBLE: u32 = 3 << 0;

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

    /// **NSEC** [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub fn nsec(&self) -> u32 {
        (self.read_bits() & Self::NSEC_MASK) >> Self::NSEC_OFFSET
    }

    /// Write NSEC field
    #[inline(always)]
    pub fn write_nsec(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::NSEC_MASK) | ((value << Self::NSEC_OFFSET) & Self::NSEC_MASK));
    }

    /// **SEC** [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub fn sec(&self) -> u32 {
        (self.read_bits() & Self::SEC_MASK) >> Self::SEC_OFFSET
    }

    /// Write SEC field
    #[inline(always)]
    pub fn write_sec(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::SEC_MASK) | ((value << Self::SEC_OFFSET) & Self::SEC_MASK));
    }

}

/// SW_LOCK27 value type for batch read/modify/write operations
///
/// Use `swlock27.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `swlock27.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SwLock27Val {
    bits: u32,
}

impl SwLock27Val {
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

    /// Read NSEC [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub const fn nsec(&self) -> u32 {
        (self.bits & SwLock27::NSEC_MASK) >> SwLock27::NSEC_OFFSET
    }

    /// Read SEC [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub const fn sec(&self) -> u32 {
        (self.bits & SwLock27::SEC_MASK) >> SwLock27::SEC_OFFSET
    }

    /// Set NSEC [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub const fn with_nsec(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SwLock27::NSEC_MASK) | ((value << SwLock27::NSEC_OFFSET) & SwLock27::NSEC_MASK)
        }
    }

    /// Set SEC [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub const fn with_sec(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SwLock27::SEC_MASK) | ((value << SwLock27::SEC_OFFSET) & SwLock27::SEC_MASK)
        }
    }

}

impl Default for SwLock27Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SwLock27 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> SwLock27Val {
        SwLock27Val::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: SwLock27Val) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(SwLock27Val) -> SwLock27Val>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// SW_LOCK28 Register
///
/// Software lock register for page 28. Locks are initialised from the OTP lock
/// pages at reset. This register can be written to further advance the lock
/// state of each page (until next reset), and read to check the current lock
/// state of a page.
///
/// Address offset: 0x0070
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **NSEC** [3:2] (RW): Non-secure lock status. Writes are OR'd with the current value.
/// - **SEC** [1:0] (RW): Secure lock status. Writes are OR'd with the current value. This field is r...
#[repr(transparent)]
pub struct SwLock28 {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for SwLock28 {}
unsafe impl Sync for SwLock28 {}

impl SwLock28 {
    // ======== Field Constants ========

    /// NSEC - Non-secure lock status. Writes are OR'd with the current value.
    /// Bits [3:2], mask 0x0000000c
    pub const NSEC_OFFSET: u32 = 2;
    pub const NSEC_MASK: u32 = 0x0000000c;
    pub const NSEC_READ_WRITE: u32 = 0 << 2;
    pub const NSEC_READ_ONLY: u32 = 1 << 2;
    pub const NSEC_INACCESSIBLE: u32 = 3 << 2;

    /// SEC - Secure lock status. Writes are OR'd with the current value. This field is r...
    /// Bits [1:0], mask 0x00000003
    pub const SEC_OFFSET: u32 = 0;
    pub const SEC_MASK: u32 = 0x00000003;
    pub const SEC_READ_WRITE: u32 = 0 << 0;
    pub const SEC_READ_ONLY: u32 = 1 << 0;
    pub const SEC_INACCESSIBLE: u32 = 3 << 0;

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

    /// **NSEC** [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub fn nsec(&self) -> u32 {
        (self.read_bits() & Self::NSEC_MASK) >> Self::NSEC_OFFSET
    }

    /// Write NSEC field
    #[inline(always)]
    pub fn write_nsec(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::NSEC_MASK) | ((value << Self::NSEC_OFFSET) & Self::NSEC_MASK));
    }

    /// **SEC** [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub fn sec(&self) -> u32 {
        (self.read_bits() & Self::SEC_MASK) >> Self::SEC_OFFSET
    }

    /// Write SEC field
    #[inline(always)]
    pub fn write_sec(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::SEC_MASK) | ((value << Self::SEC_OFFSET) & Self::SEC_MASK));
    }

}

/// SW_LOCK28 value type for batch read/modify/write operations
///
/// Use `swlock28.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `swlock28.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SwLock28Val {
    bits: u32,
}

impl SwLock28Val {
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

    /// Read NSEC [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub const fn nsec(&self) -> u32 {
        (self.bits & SwLock28::NSEC_MASK) >> SwLock28::NSEC_OFFSET
    }

    /// Read SEC [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub const fn sec(&self) -> u32 {
        (self.bits & SwLock28::SEC_MASK) >> SwLock28::SEC_OFFSET
    }

    /// Set NSEC [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub const fn with_nsec(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SwLock28::NSEC_MASK) | ((value << SwLock28::NSEC_OFFSET) & SwLock28::NSEC_MASK)
        }
    }

    /// Set SEC [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub const fn with_sec(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SwLock28::SEC_MASK) | ((value << SwLock28::SEC_OFFSET) & SwLock28::SEC_MASK)
        }
    }

}

impl Default for SwLock28Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SwLock28 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> SwLock28Val {
        SwLock28Val::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: SwLock28Val) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(SwLock28Val) -> SwLock28Val>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// SW_LOCK29 Register
///
/// Software lock register for page 29. Locks are initialised from the OTP lock
/// pages at reset. This register can be written to further advance the lock
/// state of each page (until next reset), and read to check the current lock
/// state of a page.
///
/// Address offset: 0x0074
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **NSEC** [3:2] (RW): Non-secure lock status. Writes are OR'd with the current value.
/// - **SEC** [1:0] (RW): Secure lock status. Writes are OR'd with the current value. This field is r...
#[repr(transparent)]
pub struct SwLock29 {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for SwLock29 {}
unsafe impl Sync for SwLock29 {}

impl SwLock29 {
    // ======== Field Constants ========

    /// NSEC - Non-secure lock status. Writes are OR'd with the current value.
    /// Bits [3:2], mask 0x0000000c
    pub const NSEC_OFFSET: u32 = 2;
    pub const NSEC_MASK: u32 = 0x0000000c;
    pub const NSEC_READ_WRITE: u32 = 0 << 2;
    pub const NSEC_READ_ONLY: u32 = 1 << 2;
    pub const NSEC_INACCESSIBLE: u32 = 3 << 2;

    /// SEC - Secure lock status. Writes are OR'd with the current value. This field is r...
    /// Bits [1:0], mask 0x00000003
    pub const SEC_OFFSET: u32 = 0;
    pub const SEC_MASK: u32 = 0x00000003;
    pub const SEC_READ_WRITE: u32 = 0 << 0;
    pub const SEC_READ_ONLY: u32 = 1 << 0;
    pub const SEC_INACCESSIBLE: u32 = 3 << 0;

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

    /// **NSEC** [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub fn nsec(&self) -> u32 {
        (self.read_bits() & Self::NSEC_MASK) >> Self::NSEC_OFFSET
    }

    /// Write NSEC field
    #[inline(always)]
    pub fn write_nsec(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::NSEC_MASK) | ((value << Self::NSEC_OFFSET) & Self::NSEC_MASK));
    }

    /// **SEC** [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub fn sec(&self) -> u32 {
        (self.read_bits() & Self::SEC_MASK) >> Self::SEC_OFFSET
    }

    /// Write SEC field
    #[inline(always)]
    pub fn write_sec(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::SEC_MASK) | ((value << Self::SEC_OFFSET) & Self::SEC_MASK));
    }

}

/// SW_LOCK29 value type for batch read/modify/write operations
///
/// Use `swlock29.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `swlock29.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SwLock29Val {
    bits: u32,
}

impl SwLock29Val {
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

    /// Read NSEC [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub const fn nsec(&self) -> u32 {
        (self.bits & SwLock29::NSEC_MASK) >> SwLock29::NSEC_OFFSET
    }

    /// Read SEC [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub const fn sec(&self) -> u32 {
        (self.bits & SwLock29::SEC_MASK) >> SwLock29::SEC_OFFSET
    }

    /// Set NSEC [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub const fn with_nsec(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SwLock29::NSEC_MASK) | ((value << SwLock29::NSEC_OFFSET) & SwLock29::NSEC_MASK)
        }
    }

    /// Set SEC [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub const fn with_sec(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SwLock29::SEC_MASK) | ((value << SwLock29::SEC_OFFSET) & SwLock29::SEC_MASK)
        }
    }

}

impl Default for SwLock29Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SwLock29 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> SwLock29Val {
        SwLock29Val::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: SwLock29Val) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(SwLock29Val) -> SwLock29Val>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// SW_LOCK30 Register
///
/// Software lock register for page 30. Locks are initialised from the OTP lock
/// pages at reset. This register can be written to further advance the lock
/// state of each page (until next reset), and read to check the current lock
/// state of a page.
///
/// Address offset: 0x0078
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **NSEC** [3:2] (RW): Non-secure lock status. Writes are OR'd with the current value.
/// - **SEC** [1:0] (RW): Secure lock status. Writes are OR'd with the current value. This field is r...
#[repr(transparent)]
pub struct SwLock30 {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for SwLock30 {}
unsafe impl Sync for SwLock30 {}

impl SwLock30 {
    // ======== Field Constants ========

    /// NSEC - Non-secure lock status. Writes are OR'd with the current value.
    /// Bits [3:2], mask 0x0000000c
    pub const NSEC_OFFSET: u32 = 2;
    pub const NSEC_MASK: u32 = 0x0000000c;
    pub const NSEC_READ_WRITE: u32 = 0 << 2;
    pub const NSEC_READ_ONLY: u32 = 1 << 2;
    pub const NSEC_INACCESSIBLE: u32 = 3 << 2;

    /// SEC - Secure lock status. Writes are OR'd with the current value. This field is r...
    /// Bits [1:0], mask 0x00000003
    pub const SEC_OFFSET: u32 = 0;
    pub const SEC_MASK: u32 = 0x00000003;
    pub const SEC_READ_WRITE: u32 = 0 << 0;
    pub const SEC_READ_ONLY: u32 = 1 << 0;
    pub const SEC_INACCESSIBLE: u32 = 3 << 0;

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

    /// **NSEC** [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub fn nsec(&self) -> u32 {
        (self.read_bits() & Self::NSEC_MASK) >> Self::NSEC_OFFSET
    }

    /// Write NSEC field
    #[inline(always)]
    pub fn write_nsec(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::NSEC_MASK) | ((value << Self::NSEC_OFFSET) & Self::NSEC_MASK));
    }

    /// **SEC** [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub fn sec(&self) -> u32 {
        (self.read_bits() & Self::SEC_MASK) >> Self::SEC_OFFSET
    }

    /// Write SEC field
    #[inline(always)]
    pub fn write_sec(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::SEC_MASK) | ((value << Self::SEC_OFFSET) & Self::SEC_MASK));
    }

}

/// SW_LOCK30 value type for batch read/modify/write operations
///
/// Use `swlock30.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `swlock30.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SwLock30Val {
    bits: u32,
}

impl SwLock30Val {
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

    /// Read NSEC [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub const fn nsec(&self) -> u32 {
        (self.bits & SwLock30::NSEC_MASK) >> SwLock30::NSEC_OFFSET
    }

    /// Read SEC [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub const fn sec(&self) -> u32 {
        (self.bits & SwLock30::SEC_MASK) >> SwLock30::SEC_OFFSET
    }

    /// Set NSEC [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub const fn with_nsec(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SwLock30::NSEC_MASK) | ((value << SwLock30::NSEC_OFFSET) & SwLock30::NSEC_MASK)
        }
    }

    /// Set SEC [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub const fn with_sec(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SwLock30::SEC_MASK) | ((value << SwLock30::SEC_OFFSET) & SwLock30::SEC_MASK)
        }
    }

}

impl Default for SwLock30Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SwLock30 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> SwLock30Val {
        SwLock30Val::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: SwLock30Val) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(SwLock30Val) -> SwLock30Val>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// SW_LOCK31 Register
///
/// Software lock register for page 31. Locks are initialised from the OTP lock
/// pages at reset. This register can be written to further advance the lock
/// state of each page (until next reset), and read to check the current lock
/// state of a page.
///
/// Address offset: 0x007c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **NSEC** [3:2] (RW): Non-secure lock status. Writes are OR'd with the current value.
/// - **SEC** [1:0] (RW): Secure lock status. Writes are OR'd with the current value. This field is r...
#[repr(transparent)]
pub struct SwLock31 {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for SwLock31 {}
unsafe impl Sync for SwLock31 {}

impl SwLock31 {
    // ======== Field Constants ========

    /// NSEC - Non-secure lock status. Writes are OR'd with the current value.
    /// Bits [3:2], mask 0x0000000c
    pub const NSEC_OFFSET: u32 = 2;
    pub const NSEC_MASK: u32 = 0x0000000c;
    pub const NSEC_READ_WRITE: u32 = 0 << 2;
    pub const NSEC_READ_ONLY: u32 = 1 << 2;
    pub const NSEC_INACCESSIBLE: u32 = 3 << 2;

    /// SEC - Secure lock status. Writes are OR'd with the current value. This field is r...
    /// Bits [1:0], mask 0x00000003
    pub const SEC_OFFSET: u32 = 0;
    pub const SEC_MASK: u32 = 0x00000003;
    pub const SEC_READ_WRITE: u32 = 0 << 0;
    pub const SEC_READ_ONLY: u32 = 1 << 0;
    pub const SEC_INACCESSIBLE: u32 = 3 << 0;

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

    /// **NSEC** [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub fn nsec(&self) -> u32 {
        (self.read_bits() & Self::NSEC_MASK) >> Self::NSEC_OFFSET
    }

    /// Write NSEC field
    #[inline(always)]
    pub fn write_nsec(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::NSEC_MASK) | ((value << Self::NSEC_OFFSET) & Self::NSEC_MASK));
    }

    /// **SEC** [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub fn sec(&self) -> u32 {
        (self.read_bits() & Self::SEC_MASK) >> Self::SEC_OFFSET
    }

    /// Write SEC field
    #[inline(always)]
    pub fn write_sec(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::SEC_MASK) | ((value << Self::SEC_OFFSET) & Self::SEC_MASK));
    }

}

/// SW_LOCK31 value type for batch read/modify/write operations
///
/// Use `swlock31.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `swlock31.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SwLock31Val {
    bits: u32,
}

impl SwLock31Val {
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

    /// Read NSEC [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub const fn nsec(&self) -> u32 {
        (self.bits & SwLock31::NSEC_MASK) >> SwLock31::NSEC_OFFSET
    }

    /// Read SEC [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub const fn sec(&self) -> u32 {
        (self.bits & SwLock31::SEC_MASK) >> SwLock31::SEC_OFFSET
    }

    /// Set NSEC [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub const fn with_nsec(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SwLock31::NSEC_MASK) | ((value << SwLock31::NSEC_OFFSET) & SwLock31::NSEC_MASK)
        }
    }

    /// Set SEC [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub const fn with_sec(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SwLock31::SEC_MASK) | ((value << SwLock31::SEC_OFFSET) & SwLock31::SEC_MASK)
        }
    }

}

impl Default for SwLock31Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SwLock31 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> SwLock31Val {
        SwLock31Val::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: SwLock31Val) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(SwLock31Val) -> SwLock31Val>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// SW_LOCK32 Register
///
/// Software lock register for page 32. Locks are initialised from the OTP lock
/// pages at reset. This register can be written to further advance the lock
/// state of each page (until next reset), and read to check the current lock
/// state of a page.
///
/// Address offset: 0x0080
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **NSEC** [3:2] (RW): Non-secure lock status. Writes are OR'd with the current value.
/// - **SEC** [1:0] (RW): Secure lock status. Writes are OR'd with the current value. This field is r...
#[repr(transparent)]
pub struct SwLock32 {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for SwLock32 {}
unsafe impl Sync for SwLock32 {}

impl SwLock32 {
    // ======== Field Constants ========

    /// NSEC - Non-secure lock status. Writes are OR'd with the current value.
    /// Bits [3:2], mask 0x0000000c
    pub const NSEC_OFFSET: u32 = 2;
    pub const NSEC_MASK: u32 = 0x0000000c;
    pub const NSEC_READ_WRITE: u32 = 0 << 2;
    pub const NSEC_READ_ONLY: u32 = 1 << 2;
    pub const NSEC_INACCESSIBLE: u32 = 3 << 2;

    /// SEC - Secure lock status. Writes are OR'd with the current value. This field is r...
    /// Bits [1:0], mask 0x00000003
    pub const SEC_OFFSET: u32 = 0;
    pub const SEC_MASK: u32 = 0x00000003;
    pub const SEC_READ_WRITE: u32 = 0 << 0;
    pub const SEC_READ_ONLY: u32 = 1 << 0;
    pub const SEC_INACCESSIBLE: u32 = 3 << 0;

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

    /// **NSEC** [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub fn nsec(&self) -> u32 {
        (self.read_bits() & Self::NSEC_MASK) >> Self::NSEC_OFFSET
    }

    /// Write NSEC field
    #[inline(always)]
    pub fn write_nsec(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::NSEC_MASK) | ((value << Self::NSEC_OFFSET) & Self::NSEC_MASK));
    }

    /// **SEC** [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub fn sec(&self) -> u32 {
        (self.read_bits() & Self::SEC_MASK) >> Self::SEC_OFFSET
    }

    /// Write SEC field
    #[inline(always)]
    pub fn write_sec(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::SEC_MASK) | ((value << Self::SEC_OFFSET) & Self::SEC_MASK));
    }

}

/// SW_LOCK32 value type for batch read/modify/write operations
///
/// Use `swlock32.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `swlock32.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SwLock32Val {
    bits: u32,
}

impl SwLock32Val {
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

    /// Read NSEC [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub const fn nsec(&self) -> u32 {
        (self.bits & SwLock32::NSEC_MASK) >> SwLock32::NSEC_OFFSET
    }

    /// Read SEC [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub const fn sec(&self) -> u32 {
        (self.bits & SwLock32::SEC_MASK) >> SwLock32::SEC_OFFSET
    }

    /// Set NSEC [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub const fn with_nsec(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SwLock32::NSEC_MASK) | ((value << SwLock32::NSEC_OFFSET) & SwLock32::NSEC_MASK)
        }
    }

    /// Set SEC [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub const fn with_sec(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SwLock32::SEC_MASK) | ((value << SwLock32::SEC_OFFSET) & SwLock32::SEC_MASK)
        }
    }

}

impl Default for SwLock32Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SwLock32 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> SwLock32Val {
        SwLock32Val::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: SwLock32Val) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(SwLock32Val) -> SwLock32Val>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// SW_LOCK33 Register
///
/// Software lock register for page 33. Locks are initialised from the OTP lock
/// pages at reset. This register can be written to further advance the lock
/// state of each page (until next reset), and read to check the current lock
/// state of a page.
///
/// Address offset: 0x0084
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **NSEC** [3:2] (RW): Non-secure lock status. Writes are OR'd with the current value.
/// - **SEC** [1:0] (RW): Secure lock status. Writes are OR'd with the current value. This field is r...
#[repr(transparent)]
pub struct SwLock33 {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for SwLock33 {}
unsafe impl Sync for SwLock33 {}

impl SwLock33 {
    // ======== Field Constants ========

    /// NSEC - Non-secure lock status. Writes are OR'd with the current value.
    /// Bits [3:2], mask 0x0000000c
    pub const NSEC_OFFSET: u32 = 2;
    pub const NSEC_MASK: u32 = 0x0000000c;
    pub const NSEC_READ_WRITE: u32 = 0 << 2;
    pub const NSEC_READ_ONLY: u32 = 1 << 2;
    pub const NSEC_INACCESSIBLE: u32 = 3 << 2;

    /// SEC - Secure lock status. Writes are OR'd with the current value. This field is r...
    /// Bits [1:0], mask 0x00000003
    pub const SEC_OFFSET: u32 = 0;
    pub const SEC_MASK: u32 = 0x00000003;
    pub const SEC_READ_WRITE: u32 = 0 << 0;
    pub const SEC_READ_ONLY: u32 = 1 << 0;
    pub const SEC_INACCESSIBLE: u32 = 3 << 0;

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

    /// **NSEC** [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub fn nsec(&self) -> u32 {
        (self.read_bits() & Self::NSEC_MASK) >> Self::NSEC_OFFSET
    }

    /// Write NSEC field
    #[inline(always)]
    pub fn write_nsec(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::NSEC_MASK) | ((value << Self::NSEC_OFFSET) & Self::NSEC_MASK));
    }

    /// **SEC** [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub fn sec(&self) -> u32 {
        (self.read_bits() & Self::SEC_MASK) >> Self::SEC_OFFSET
    }

    /// Write SEC field
    #[inline(always)]
    pub fn write_sec(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::SEC_MASK) | ((value << Self::SEC_OFFSET) & Self::SEC_MASK));
    }

}

/// SW_LOCK33 value type for batch read/modify/write operations
///
/// Use `swlock33.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `swlock33.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SwLock33Val {
    bits: u32,
}

impl SwLock33Val {
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

    /// Read NSEC [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub const fn nsec(&self) -> u32 {
        (self.bits & SwLock33::NSEC_MASK) >> SwLock33::NSEC_OFFSET
    }

    /// Read SEC [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub const fn sec(&self) -> u32 {
        (self.bits & SwLock33::SEC_MASK) >> SwLock33::SEC_OFFSET
    }

    /// Set NSEC [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub const fn with_nsec(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SwLock33::NSEC_MASK) | ((value << SwLock33::NSEC_OFFSET) & SwLock33::NSEC_MASK)
        }
    }

    /// Set SEC [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub const fn with_sec(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SwLock33::SEC_MASK) | ((value << SwLock33::SEC_OFFSET) & SwLock33::SEC_MASK)
        }
    }

}

impl Default for SwLock33Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SwLock33 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> SwLock33Val {
        SwLock33Val::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: SwLock33Val) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(SwLock33Val) -> SwLock33Val>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// SW_LOCK34 Register
///
/// Software lock register for page 34. Locks are initialised from the OTP lock
/// pages at reset. This register can be written to further advance the lock
/// state of each page (until next reset), and read to check the current lock
/// state of a page.
///
/// Address offset: 0x0088
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **NSEC** [3:2] (RW): Non-secure lock status. Writes are OR'd with the current value.
/// - **SEC** [1:0] (RW): Secure lock status. Writes are OR'd with the current value. This field is r...
#[repr(transparent)]
pub struct SwLock34 {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for SwLock34 {}
unsafe impl Sync for SwLock34 {}

impl SwLock34 {
    // ======== Field Constants ========

    /// NSEC - Non-secure lock status. Writes are OR'd with the current value.
    /// Bits [3:2], mask 0x0000000c
    pub const NSEC_OFFSET: u32 = 2;
    pub const NSEC_MASK: u32 = 0x0000000c;
    pub const NSEC_READ_WRITE: u32 = 0 << 2;
    pub const NSEC_READ_ONLY: u32 = 1 << 2;
    pub const NSEC_INACCESSIBLE: u32 = 3 << 2;

    /// SEC - Secure lock status. Writes are OR'd with the current value. This field is r...
    /// Bits [1:0], mask 0x00000003
    pub const SEC_OFFSET: u32 = 0;
    pub const SEC_MASK: u32 = 0x00000003;
    pub const SEC_READ_WRITE: u32 = 0 << 0;
    pub const SEC_READ_ONLY: u32 = 1 << 0;
    pub const SEC_INACCESSIBLE: u32 = 3 << 0;

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

    /// **NSEC** [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub fn nsec(&self) -> u32 {
        (self.read_bits() & Self::NSEC_MASK) >> Self::NSEC_OFFSET
    }

    /// Write NSEC field
    #[inline(always)]
    pub fn write_nsec(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::NSEC_MASK) | ((value << Self::NSEC_OFFSET) & Self::NSEC_MASK));
    }

    /// **SEC** [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub fn sec(&self) -> u32 {
        (self.read_bits() & Self::SEC_MASK) >> Self::SEC_OFFSET
    }

    /// Write SEC field
    #[inline(always)]
    pub fn write_sec(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::SEC_MASK) | ((value << Self::SEC_OFFSET) & Self::SEC_MASK));
    }

}

/// SW_LOCK34 value type for batch read/modify/write operations
///
/// Use `swlock34.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `swlock34.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SwLock34Val {
    bits: u32,
}

impl SwLock34Val {
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

    /// Read NSEC [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub const fn nsec(&self) -> u32 {
        (self.bits & SwLock34::NSEC_MASK) >> SwLock34::NSEC_OFFSET
    }

    /// Read SEC [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub const fn sec(&self) -> u32 {
        (self.bits & SwLock34::SEC_MASK) >> SwLock34::SEC_OFFSET
    }

    /// Set NSEC [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub const fn with_nsec(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SwLock34::NSEC_MASK) | ((value << SwLock34::NSEC_OFFSET) & SwLock34::NSEC_MASK)
        }
    }

    /// Set SEC [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub const fn with_sec(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SwLock34::SEC_MASK) | ((value << SwLock34::SEC_OFFSET) & SwLock34::SEC_MASK)
        }
    }

}

impl Default for SwLock34Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SwLock34 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> SwLock34Val {
        SwLock34Val::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: SwLock34Val) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(SwLock34Val) -> SwLock34Val>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// SW_LOCK35 Register
///
/// Software lock register for page 35. Locks are initialised from the OTP lock
/// pages at reset. This register can be written to further advance the lock
/// state of each page (until next reset), and read to check the current lock
/// state of a page.
///
/// Address offset: 0x008c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **NSEC** [3:2] (RW): Non-secure lock status. Writes are OR'd with the current value.
/// - **SEC** [1:0] (RW): Secure lock status. Writes are OR'd with the current value. This field is r...
#[repr(transparent)]
pub struct SwLock35 {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for SwLock35 {}
unsafe impl Sync for SwLock35 {}

impl SwLock35 {
    // ======== Field Constants ========

    /// NSEC - Non-secure lock status. Writes are OR'd with the current value.
    /// Bits [3:2], mask 0x0000000c
    pub const NSEC_OFFSET: u32 = 2;
    pub const NSEC_MASK: u32 = 0x0000000c;
    pub const NSEC_READ_WRITE: u32 = 0 << 2;
    pub const NSEC_READ_ONLY: u32 = 1 << 2;
    pub const NSEC_INACCESSIBLE: u32 = 3 << 2;

    /// SEC - Secure lock status. Writes are OR'd with the current value. This field is r...
    /// Bits [1:0], mask 0x00000003
    pub const SEC_OFFSET: u32 = 0;
    pub const SEC_MASK: u32 = 0x00000003;
    pub const SEC_READ_WRITE: u32 = 0 << 0;
    pub const SEC_READ_ONLY: u32 = 1 << 0;
    pub const SEC_INACCESSIBLE: u32 = 3 << 0;

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

    /// **NSEC** [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub fn nsec(&self) -> u32 {
        (self.read_bits() & Self::NSEC_MASK) >> Self::NSEC_OFFSET
    }

    /// Write NSEC field
    #[inline(always)]
    pub fn write_nsec(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::NSEC_MASK) | ((value << Self::NSEC_OFFSET) & Self::NSEC_MASK));
    }

    /// **SEC** [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub fn sec(&self) -> u32 {
        (self.read_bits() & Self::SEC_MASK) >> Self::SEC_OFFSET
    }

    /// Write SEC field
    #[inline(always)]
    pub fn write_sec(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::SEC_MASK) | ((value << Self::SEC_OFFSET) & Self::SEC_MASK));
    }

}

/// SW_LOCK35 value type for batch read/modify/write operations
///
/// Use `swlock35.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `swlock35.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SwLock35Val {
    bits: u32,
}

impl SwLock35Val {
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

    /// Read NSEC [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub const fn nsec(&self) -> u32 {
        (self.bits & SwLock35::NSEC_MASK) >> SwLock35::NSEC_OFFSET
    }

    /// Read SEC [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub const fn sec(&self) -> u32 {
        (self.bits & SwLock35::SEC_MASK) >> SwLock35::SEC_OFFSET
    }

    /// Set NSEC [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub const fn with_nsec(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SwLock35::NSEC_MASK) | ((value << SwLock35::NSEC_OFFSET) & SwLock35::NSEC_MASK)
        }
    }

    /// Set SEC [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub const fn with_sec(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SwLock35::SEC_MASK) | ((value << SwLock35::SEC_OFFSET) & SwLock35::SEC_MASK)
        }
    }

}

impl Default for SwLock35Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SwLock35 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> SwLock35Val {
        SwLock35Val::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: SwLock35Val) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(SwLock35Val) -> SwLock35Val>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// SW_LOCK36 Register
///
/// Software lock register for page 36. Locks are initialised from the OTP lock
/// pages at reset. This register can be written to further advance the lock
/// state of each page (until next reset), and read to check the current lock
/// state of a page.
///
/// Address offset: 0x0090
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **NSEC** [3:2] (RW): Non-secure lock status. Writes are OR'd with the current value.
/// - **SEC** [1:0] (RW): Secure lock status. Writes are OR'd with the current value. This field is r...
#[repr(transparent)]
pub struct SwLock36 {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for SwLock36 {}
unsafe impl Sync for SwLock36 {}

impl SwLock36 {
    // ======== Field Constants ========

    /// NSEC - Non-secure lock status. Writes are OR'd with the current value.
    /// Bits [3:2], mask 0x0000000c
    pub const NSEC_OFFSET: u32 = 2;
    pub const NSEC_MASK: u32 = 0x0000000c;
    pub const NSEC_READ_WRITE: u32 = 0 << 2;
    pub const NSEC_READ_ONLY: u32 = 1 << 2;
    pub const NSEC_INACCESSIBLE: u32 = 3 << 2;

    /// SEC - Secure lock status. Writes are OR'd with the current value. This field is r...
    /// Bits [1:0], mask 0x00000003
    pub const SEC_OFFSET: u32 = 0;
    pub const SEC_MASK: u32 = 0x00000003;
    pub const SEC_READ_WRITE: u32 = 0 << 0;
    pub const SEC_READ_ONLY: u32 = 1 << 0;
    pub const SEC_INACCESSIBLE: u32 = 3 << 0;

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

    /// **NSEC** [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub fn nsec(&self) -> u32 {
        (self.read_bits() & Self::NSEC_MASK) >> Self::NSEC_OFFSET
    }

    /// Write NSEC field
    #[inline(always)]
    pub fn write_nsec(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::NSEC_MASK) | ((value << Self::NSEC_OFFSET) & Self::NSEC_MASK));
    }

    /// **SEC** [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub fn sec(&self) -> u32 {
        (self.read_bits() & Self::SEC_MASK) >> Self::SEC_OFFSET
    }

    /// Write SEC field
    #[inline(always)]
    pub fn write_sec(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::SEC_MASK) | ((value << Self::SEC_OFFSET) & Self::SEC_MASK));
    }

}

/// SW_LOCK36 value type for batch read/modify/write operations
///
/// Use `swlock36.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `swlock36.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SwLock36Val {
    bits: u32,
}

impl SwLock36Val {
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

    /// Read NSEC [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub const fn nsec(&self) -> u32 {
        (self.bits & SwLock36::NSEC_MASK) >> SwLock36::NSEC_OFFSET
    }

    /// Read SEC [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub const fn sec(&self) -> u32 {
        (self.bits & SwLock36::SEC_MASK) >> SwLock36::SEC_OFFSET
    }

    /// Set NSEC [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub const fn with_nsec(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SwLock36::NSEC_MASK) | ((value << SwLock36::NSEC_OFFSET) & SwLock36::NSEC_MASK)
        }
    }

    /// Set SEC [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub const fn with_sec(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SwLock36::SEC_MASK) | ((value << SwLock36::SEC_OFFSET) & SwLock36::SEC_MASK)
        }
    }

}

impl Default for SwLock36Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SwLock36 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> SwLock36Val {
        SwLock36Val::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: SwLock36Val) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(SwLock36Val) -> SwLock36Val>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// SW_LOCK37 Register
///
/// Software lock register for page 37. Locks are initialised from the OTP lock
/// pages at reset. This register can be written to further advance the lock
/// state of each page (until next reset), and read to check the current lock
/// state of a page.
///
/// Address offset: 0x0094
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **NSEC** [3:2] (RW): Non-secure lock status. Writes are OR'd with the current value.
/// - **SEC** [1:0] (RW): Secure lock status. Writes are OR'd with the current value. This field is r...
#[repr(transparent)]
pub struct SwLock37 {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for SwLock37 {}
unsafe impl Sync for SwLock37 {}

impl SwLock37 {
    // ======== Field Constants ========

    /// NSEC - Non-secure lock status. Writes are OR'd with the current value.
    /// Bits [3:2], mask 0x0000000c
    pub const NSEC_OFFSET: u32 = 2;
    pub const NSEC_MASK: u32 = 0x0000000c;
    pub const NSEC_READ_WRITE: u32 = 0 << 2;
    pub const NSEC_READ_ONLY: u32 = 1 << 2;
    pub const NSEC_INACCESSIBLE: u32 = 3 << 2;

    /// SEC - Secure lock status. Writes are OR'd with the current value. This field is r...
    /// Bits [1:0], mask 0x00000003
    pub const SEC_OFFSET: u32 = 0;
    pub const SEC_MASK: u32 = 0x00000003;
    pub const SEC_READ_WRITE: u32 = 0 << 0;
    pub const SEC_READ_ONLY: u32 = 1 << 0;
    pub const SEC_INACCESSIBLE: u32 = 3 << 0;

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

    /// **NSEC** [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub fn nsec(&self) -> u32 {
        (self.read_bits() & Self::NSEC_MASK) >> Self::NSEC_OFFSET
    }

    /// Write NSEC field
    #[inline(always)]
    pub fn write_nsec(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::NSEC_MASK) | ((value << Self::NSEC_OFFSET) & Self::NSEC_MASK));
    }

    /// **SEC** [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub fn sec(&self) -> u32 {
        (self.read_bits() & Self::SEC_MASK) >> Self::SEC_OFFSET
    }

    /// Write SEC field
    #[inline(always)]
    pub fn write_sec(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::SEC_MASK) | ((value << Self::SEC_OFFSET) & Self::SEC_MASK));
    }

}

/// SW_LOCK37 value type for batch read/modify/write operations
///
/// Use `swlock37.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `swlock37.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SwLock37Val {
    bits: u32,
}

impl SwLock37Val {
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

    /// Read NSEC [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub const fn nsec(&self) -> u32 {
        (self.bits & SwLock37::NSEC_MASK) >> SwLock37::NSEC_OFFSET
    }

    /// Read SEC [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub const fn sec(&self) -> u32 {
        (self.bits & SwLock37::SEC_MASK) >> SwLock37::SEC_OFFSET
    }

    /// Set NSEC [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub const fn with_nsec(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SwLock37::NSEC_MASK) | ((value << SwLock37::NSEC_OFFSET) & SwLock37::NSEC_MASK)
        }
    }

    /// Set SEC [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub const fn with_sec(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SwLock37::SEC_MASK) | ((value << SwLock37::SEC_OFFSET) & SwLock37::SEC_MASK)
        }
    }

}

impl Default for SwLock37Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SwLock37 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> SwLock37Val {
        SwLock37Val::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: SwLock37Val) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(SwLock37Val) -> SwLock37Val>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// SW_LOCK38 Register
///
/// Software lock register for page 38. Locks are initialised from the OTP lock
/// pages at reset. This register can be written to further advance the lock
/// state of each page (until next reset), and read to check the current lock
/// state of a page.
///
/// Address offset: 0x0098
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **NSEC** [3:2] (RW): Non-secure lock status. Writes are OR'd with the current value.
/// - **SEC** [1:0] (RW): Secure lock status. Writes are OR'd with the current value. This field is r...
#[repr(transparent)]
pub struct SwLock38 {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for SwLock38 {}
unsafe impl Sync for SwLock38 {}

impl SwLock38 {
    // ======== Field Constants ========

    /// NSEC - Non-secure lock status. Writes are OR'd with the current value.
    /// Bits [3:2], mask 0x0000000c
    pub const NSEC_OFFSET: u32 = 2;
    pub const NSEC_MASK: u32 = 0x0000000c;
    pub const NSEC_READ_WRITE: u32 = 0 << 2;
    pub const NSEC_READ_ONLY: u32 = 1 << 2;
    pub const NSEC_INACCESSIBLE: u32 = 3 << 2;

    /// SEC - Secure lock status. Writes are OR'd with the current value. This field is r...
    /// Bits [1:0], mask 0x00000003
    pub const SEC_OFFSET: u32 = 0;
    pub const SEC_MASK: u32 = 0x00000003;
    pub const SEC_READ_WRITE: u32 = 0 << 0;
    pub const SEC_READ_ONLY: u32 = 1 << 0;
    pub const SEC_INACCESSIBLE: u32 = 3 << 0;

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

    /// **NSEC** [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub fn nsec(&self) -> u32 {
        (self.read_bits() & Self::NSEC_MASK) >> Self::NSEC_OFFSET
    }

    /// Write NSEC field
    #[inline(always)]
    pub fn write_nsec(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::NSEC_MASK) | ((value << Self::NSEC_OFFSET) & Self::NSEC_MASK));
    }

    /// **SEC** [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub fn sec(&self) -> u32 {
        (self.read_bits() & Self::SEC_MASK) >> Self::SEC_OFFSET
    }

    /// Write SEC field
    #[inline(always)]
    pub fn write_sec(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::SEC_MASK) | ((value << Self::SEC_OFFSET) & Self::SEC_MASK));
    }

}

/// SW_LOCK38 value type for batch read/modify/write operations
///
/// Use `swlock38.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `swlock38.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SwLock38Val {
    bits: u32,
}

impl SwLock38Val {
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

    /// Read NSEC [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub const fn nsec(&self) -> u32 {
        (self.bits & SwLock38::NSEC_MASK) >> SwLock38::NSEC_OFFSET
    }

    /// Read SEC [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub const fn sec(&self) -> u32 {
        (self.bits & SwLock38::SEC_MASK) >> SwLock38::SEC_OFFSET
    }

    /// Set NSEC [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub const fn with_nsec(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SwLock38::NSEC_MASK) | ((value << SwLock38::NSEC_OFFSET) & SwLock38::NSEC_MASK)
        }
    }

    /// Set SEC [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub const fn with_sec(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SwLock38::SEC_MASK) | ((value << SwLock38::SEC_OFFSET) & SwLock38::SEC_MASK)
        }
    }

}

impl Default for SwLock38Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SwLock38 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> SwLock38Val {
        SwLock38Val::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: SwLock38Val) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(SwLock38Val) -> SwLock38Val>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// SW_LOCK39 Register
///
/// Software lock register for page 39. Locks are initialised from the OTP lock
/// pages at reset. This register can be written to further advance the lock
/// state of each page (until next reset), and read to check the current lock
/// state of a page.
///
/// Address offset: 0x009c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **NSEC** [3:2] (RW): Non-secure lock status. Writes are OR'd with the current value.
/// - **SEC** [1:0] (RW): Secure lock status. Writes are OR'd with the current value. This field is r...
#[repr(transparent)]
pub struct SwLock39 {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for SwLock39 {}
unsafe impl Sync for SwLock39 {}

impl SwLock39 {
    // ======== Field Constants ========

    /// NSEC - Non-secure lock status. Writes are OR'd with the current value.
    /// Bits [3:2], mask 0x0000000c
    pub const NSEC_OFFSET: u32 = 2;
    pub const NSEC_MASK: u32 = 0x0000000c;
    pub const NSEC_READ_WRITE: u32 = 0 << 2;
    pub const NSEC_READ_ONLY: u32 = 1 << 2;
    pub const NSEC_INACCESSIBLE: u32 = 3 << 2;

    /// SEC - Secure lock status. Writes are OR'd with the current value. This field is r...
    /// Bits [1:0], mask 0x00000003
    pub const SEC_OFFSET: u32 = 0;
    pub const SEC_MASK: u32 = 0x00000003;
    pub const SEC_READ_WRITE: u32 = 0 << 0;
    pub const SEC_READ_ONLY: u32 = 1 << 0;
    pub const SEC_INACCESSIBLE: u32 = 3 << 0;

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

    /// **NSEC** [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub fn nsec(&self) -> u32 {
        (self.read_bits() & Self::NSEC_MASK) >> Self::NSEC_OFFSET
    }

    /// Write NSEC field
    #[inline(always)]
    pub fn write_nsec(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::NSEC_MASK) | ((value << Self::NSEC_OFFSET) & Self::NSEC_MASK));
    }

    /// **SEC** [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub fn sec(&self) -> u32 {
        (self.read_bits() & Self::SEC_MASK) >> Self::SEC_OFFSET
    }

    /// Write SEC field
    #[inline(always)]
    pub fn write_sec(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::SEC_MASK) | ((value << Self::SEC_OFFSET) & Self::SEC_MASK));
    }

}

/// SW_LOCK39 value type for batch read/modify/write operations
///
/// Use `swlock39.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `swlock39.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SwLock39Val {
    bits: u32,
}

impl SwLock39Val {
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

    /// Read NSEC [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub const fn nsec(&self) -> u32 {
        (self.bits & SwLock39::NSEC_MASK) >> SwLock39::NSEC_OFFSET
    }

    /// Read SEC [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub const fn sec(&self) -> u32 {
        (self.bits & SwLock39::SEC_MASK) >> SwLock39::SEC_OFFSET
    }

    /// Set NSEC [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub const fn with_nsec(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SwLock39::NSEC_MASK) | ((value << SwLock39::NSEC_OFFSET) & SwLock39::NSEC_MASK)
        }
    }

    /// Set SEC [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub const fn with_sec(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SwLock39::SEC_MASK) | ((value << SwLock39::SEC_OFFSET) & SwLock39::SEC_MASK)
        }
    }

}

impl Default for SwLock39Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SwLock39 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> SwLock39Val {
        SwLock39Val::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: SwLock39Val) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(SwLock39Val) -> SwLock39Val>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// SW_LOCK40 Register
///
/// Software lock register for page 40. Locks are initialised from the OTP lock
/// pages at reset. This register can be written to further advance the lock
/// state of each page (until next reset), and read to check the current lock
/// state of a page.
///
/// Address offset: 0x00a0
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **NSEC** [3:2] (RW): Non-secure lock status. Writes are OR'd with the current value.
/// - **SEC** [1:0] (RW): Secure lock status. Writes are OR'd with the current value. This field is r...
#[repr(transparent)]
pub struct SwLock40 {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for SwLock40 {}
unsafe impl Sync for SwLock40 {}

impl SwLock40 {
    // ======== Field Constants ========

    /// NSEC - Non-secure lock status. Writes are OR'd with the current value.
    /// Bits [3:2], mask 0x0000000c
    pub const NSEC_OFFSET: u32 = 2;
    pub const NSEC_MASK: u32 = 0x0000000c;
    pub const NSEC_READ_WRITE: u32 = 0 << 2;
    pub const NSEC_READ_ONLY: u32 = 1 << 2;
    pub const NSEC_INACCESSIBLE: u32 = 3 << 2;

    /// SEC - Secure lock status. Writes are OR'd with the current value. This field is r...
    /// Bits [1:0], mask 0x00000003
    pub const SEC_OFFSET: u32 = 0;
    pub const SEC_MASK: u32 = 0x00000003;
    pub const SEC_READ_WRITE: u32 = 0 << 0;
    pub const SEC_READ_ONLY: u32 = 1 << 0;
    pub const SEC_INACCESSIBLE: u32 = 3 << 0;

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

    /// **NSEC** [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub fn nsec(&self) -> u32 {
        (self.read_bits() & Self::NSEC_MASK) >> Self::NSEC_OFFSET
    }

    /// Write NSEC field
    #[inline(always)]
    pub fn write_nsec(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::NSEC_MASK) | ((value << Self::NSEC_OFFSET) & Self::NSEC_MASK));
    }

    /// **SEC** [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub fn sec(&self) -> u32 {
        (self.read_bits() & Self::SEC_MASK) >> Self::SEC_OFFSET
    }

    /// Write SEC field
    #[inline(always)]
    pub fn write_sec(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::SEC_MASK) | ((value << Self::SEC_OFFSET) & Self::SEC_MASK));
    }

}

/// SW_LOCK40 value type for batch read/modify/write operations
///
/// Use `swlock40.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `swlock40.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SwLock40Val {
    bits: u32,
}

impl SwLock40Val {
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

    /// Read NSEC [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub const fn nsec(&self) -> u32 {
        (self.bits & SwLock40::NSEC_MASK) >> SwLock40::NSEC_OFFSET
    }

    /// Read SEC [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub const fn sec(&self) -> u32 {
        (self.bits & SwLock40::SEC_MASK) >> SwLock40::SEC_OFFSET
    }

    /// Set NSEC [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub const fn with_nsec(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SwLock40::NSEC_MASK) | ((value << SwLock40::NSEC_OFFSET) & SwLock40::NSEC_MASK)
        }
    }

    /// Set SEC [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub const fn with_sec(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SwLock40::SEC_MASK) | ((value << SwLock40::SEC_OFFSET) & SwLock40::SEC_MASK)
        }
    }

}

impl Default for SwLock40Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SwLock40 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> SwLock40Val {
        SwLock40Val::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: SwLock40Val) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(SwLock40Val) -> SwLock40Val>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// SW_LOCK41 Register
///
/// Software lock register for page 41. Locks are initialised from the OTP lock
/// pages at reset. This register can be written to further advance the lock
/// state of each page (until next reset), and read to check the current lock
/// state of a page.
///
/// Address offset: 0x00a4
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **NSEC** [3:2] (RW): Non-secure lock status. Writes are OR'd with the current value.
/// - **SEC** [1:0] (RW): Secure lock status. Writes are OR'd with the current value. This field is r...
#[repr(transparent)]
pub struct SwLock41 {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for SwLock41 {}
unsafe impl Sync for SwLock41 {}

impl SwLock41 {
    // ======== Field Constants ========

    /// NSEC - Non-secure lock status. Writes are OR'd with the current value.
    /// Bits [3:2], mask 0x0000000c
    pub const NSEC_OFFSET: u32 = 2;
    pub const NSEC_MASK: u32 = 0x0000000c;
    pub const NSEC_READ_WRITE: u32 = 0 << 2;
    pub const NSEC_READ_ONLY: u32 = 1 << 2;
    pub const NSEC_INACCESSIBLE: u32 = 3 << 2;

    /// SEC - Secure lock status. Writes are OR'd with the current value. This field is r...
    /// Bits [1:0], mask 0x00000003
    pub const SEC_OFFSET: u32 = 0;
    pub const SEC_MASK: u32 = 0x00000003;
    pub const SEC_READ_WRITE: u32 = 0 << 0;
    pub const SEC_READ_ONLY: u32 = 1 << 0;
    pub const SEC_INACCESSIBLE: u32 = 3 << 0;

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

    /// **NSEC** [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub fn nsec(&self) -> u32 {
        (self.read_bits() & Self::NSEC_MASK) >> Self::NSEC_OFFSET
    }

    /// Write NSEC field
    #[inline(always)]
    pub fn write_nsec(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::NSEC_MASK) | ((value << Self::NSEC_OFFSET) & Self::NSEC_MASK));
    }

    /// **SEC** [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub fn sec(&self) -> u32 {
        (self.read_bits() & Self::SEC_MASK) >> Self::SEC_OFFSET
    }

    /// Write SEC field
    #[inline(always)]
    pub fn write_sec(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::SEC_MASK) | ((value << Self::SEC_OFFSET) & Self::SEC_MASK));
    }

}

/// SW_LOCK41 value type for batch read/modify/write operations
///
/// Use `swlock41.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `swlock41.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SwLock41Val {
    bits: u32,
}

impl SwLock41Val {
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

    /// Read NSEC [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub const fn nsec(&self) -> u32 {
        (self.bits & SwLock41::NSEC_MASK) >> SwLock41::NSEC_OFFSET
    }

    /// Read SEC [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub const fn sec(&self) -> u32 {
        (self.bits & SwLock41::SEC_MASK) >> SwLock41::SEC_OFFSET
    }

    /// Set NSEC [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub const fn with_nsec(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SwLock41::NSEC_MASK) | ((value << SwLock41::NSEC_OFFSET) & SwLock41::NSEC_MASK)
        }
    }

    /// Set SEC [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub const fn with_sec(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SwLock41::SEC_MASK) | ((value << SwLock41::SEC_OFFSET) & SwLock41::SEC_MASK)
        }
    }

}

impl Default for SwLock41Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SwLock41 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> SwLock41Val {
        SwLock41Val::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: SwLock41Val) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(SwLock41Val) -> SwLock41Val>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// SW_LOCK42 Register
///
/// Software lock register for page 42. Locks are initialised from the OTP lock
/// pages at reset. This register can be written to further advance the lock
/// state of each page (until next reset), and read to check the current lock
/// state of a page.
///
/// Address offset: 0x00a8
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **NSEC** [3:2] (RW): Non-secure lock status. Writes are OR'd with the current value.
/// - **SEC** [1:0] (RW): Secure lock status. Writes are OR'd with the current value. This field is r...
#[repr(transparent)]
pub struct SwLock42 {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for SwLock42 {}
unsafe impl Sync for SwLock42 {}

impl SwLock42 {
    // ======== Field Constants ========

    /// NSEC - Non-secure lock status. Writes are OR'd with the current value.
    /// Bits [3:2], mask 0x0000000c
    pub const NSEC_OFFSET: u32 = 2;
    pub const NSEC_MASK: u32 = 0x0000000c;
    pub const NSEC_READ_WRITE: u32 = 0 << 2;
    pub const NSEC_READ_ONLY: u32 = 1 << 2;
    pub const NSEC_INACCESSIBLE: u32 = 3 << 2;

    /// SEC - Secure lock status. Writes are OR'd with the current value. This field is r...
    /// Bits [1:0], mask 0x00000003
    pub const SEC_OFFSET: u32 = 0;
    pub const SEC_MASK: u32 = 0x00000003;
    pub const SEC_READ_WRITE: u32 = 0 << 0;
    pub const SEC_READ_ONLY: u32 = 1 << 0;
    pub const SEC_INACCESSIBLE: u32 = 3 << 0;

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

    /// **NSEC** [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub fn nsec(&self) -> u32 {
        (self.read_bits() & Self::NSEC_MASK) >> Self::NSEC_OFFSET
    }

    /// Write NSEC field
    #[inline(always)]
    pub fn write_nsec(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::NSEC_MASK) | ((value << Self::NSEC_OFFSET) & Self::NSEC_MASK));
    }

    /// **SEC** [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub fn sec(&self) -> u32 {
        (self.read_bits() & Self::SEC_MASK) >> Self::SEC_OFFSET
    }

    /// Write SEC field
    #[inline(always)]
    pub fn write_sec(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::SEC_MASK) | ((value << Self::SEC_OFFSET) & Self::SEC_MASK));
    }

}

/// SW_LOCK42 value type for batch read/modify/write operations
///
/// Use `swlock42.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `swlock42.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SwLock42Val {
    bits: u32,
}

impl SwLock42Val {
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

    /// Read NSEC [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub const fn nsec(&self) -> u32 {
        (self.bits & SwLock42::NSEC_MASK) >> SwLock42::NSEC_OFFSET
    }

    /// Read SEC [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub const fn sec(&self) -> u32 {
        (self.bits & SwLock42::SEC_MASK) >> SwLock42::SEC_OFFSET
    }

    /// Set NSEC [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub const fn with_nsec(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SwLock42::NSEC_MASK) | ((value << SwLock42::NSEC_OFFSET) & SwLock42::NSEC_MASK)
        }
    }

    /// Set SEC [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub const fn with_sec(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SwLock42::SEC_MASK) | ((value << SwLock42::SEC_OFFSET) & SwLock42::SEC_MASK)
        }
    }

}

impl Default for SwLock42Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SwLock42 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> SwLock42Val {
        SwLock42Val::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: SwLock42Val) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(SwLock42Val) -> SwLock42Val>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// SW_LOCK43 Register
///
/// Software lock register for page 43. Locks are initialised from the OTP lock
/// pages at reset. This register can be written to further advance the lock
/// state of each page (until next reset), and read to check the current lock
/// state of a page.
///
/// Address offset: 0x00ac
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **NSEC** [3:2] (RW): Non-secure lock status. Writes are OR'd with the current value.
/// - **SEC** [1:0] (RW): Secure lock status. Writes are OR'd with the current value. This field is r...
#[repr(transparent)]
pub struct SwLock43 {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for SwLock43 {}
unsafe impl Sync for SwLock43 {}

impl SwLock43 {
    // ======== Field Constants ========

    /// NSEC - Non-secure lock status. Writes are OR'd with the current value.
    /// Bits [3:2], mask 0x0000000c
    pub const NSEC_OFFSET: u32 = 2;
    pub const NSEC_MASK: u32 = 0x0000000c;
    pub const NSEC_READ_WRITE: u32 = 0 << 2;
    pub const NSEC_READ_ONLY: u32 = 1 << 2;
    pub const NSEC_INACCESSIBLE: u32 = 3 << 2;

    /// SEC - Secure lock status. Writes are OR'd with the current value. This field is r...
    /// Bits [1:0], mask 0x00000003
    pub const SEC_OFFSET: u32 = 0;
    pub const SEC_MASK: u32 = 0x00000003;
    pub const SEC_READ_WRITE: u32 = 0 << 0;
    pub const SEC_READ_ONLY: u32 = 1 << 0;
    pub const SEC_INACCESSIBLE: u32 = 3 << 0;

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

    /// **NSEC** [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub fn nsec(&self) -> u32 {
        (self.read_bits() & Self::NSEC_MASK) >> Self::NSEC_OFFSET
    }

    /// Write NSEC field
    #[inline(always)]
    pub fn write_nsec(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::NSEC_MASK) | ((value << Self::NSEC_OFFSET) & Self::NSEC_MASK));
    }

    /// **SEC** [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub fn sec(&self) -> u32 {
        (self.read_bits() & Self::SEC_MASK) >> Self::SEC_OFFSET
    }

    /// Write SEC field
    #[inline(always)]
    pub fn write_sec(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::SEC_MASK) | ((value << Self::SEC_OFFSET) & Self::SEC_MASK));
    }

}

/// SW_LOCK43 value type for batch read/modify/write operations
///
/// Use `swlock43.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `swlock43.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SwLock43Val {
    bits: u32,
}

impl SwLock43Val {
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

    /// Read NSEC [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub const fn nsec(&self) -> u32 {
        (self.bits & SwLock43::NSEC_MASK) >> SwLock43::NSEC_OFFSET
    }

    /// Read SEC [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub const fn sec(&self) -> u32 {
        (self.bits & SwLock43::SEC_MASK) >> SwLock43::SEC_OFFSET
    }

    /// Set NSEC [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub const fn with_nsec(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SwLock43::NSEC_MASK) | ((value << SwLock43::NSEC_OFFSET) & SwLock43::NSEC_MASK)
        }
    }

    /// Set SEC [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub const fn with_sec(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SwLock43::SEC_MASK) | ((value << SwLock43::SEC_OFFSET) & SwLock43::SEC_MASK)
        }
    }

}

impl Default for SwLock43Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SwLock43 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> SwLock43Val {
        SwLock43Val::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: SwLock43Val) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(SwLock43Val) -> SwLock43Val>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// SW_LOCK44 Register
///
/// Software lock register for page 44. Locks are initialised from the OTP lock
/// pages at reset. This register can be written to further advance the lock
/// state of each page (until next reset), and read to check the current lock
/// state of a page.
///
/// Address offset: 0x00b0
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **NSEC** [3:2] (RW): Non-secure lock status. Writes are OR'd with the current value.
/// - **SEC** [1:0] (RW): Secure lock status. Writes are OR'd with the current value. This field is r...
#[repr(transparent)]
pub struct SwLock44 {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for SwLock44 {}
unsafe impl Sync for SwLock44 {}

impl SwLock44 {
    // ======== Field Constants ========

    /// NSEC - Non-secure lock status. Writes are OR'd with the current value.
    /// Bits [3:2], mask 0x0000000c
    pub const NSEC_OFFSET: u32 = 2;
    pub const NSEC_MASK: u32 = 0x0000000c;
    pub const NSEC_READ_WRITE: u32 = 0 << 2;
    pub const NSEC_READ_ONLY: u32 = 1 << 2;
    pub const NSEC_INACCESSIBLE: u32 = 3 << 2;

    /// SEC - Secure lock status. Writes are OR'd with the current value. This field is r...
    /// Bits [1:0], mask 0x00000003
    pub const SEC_OFFSET: u32 = 0;
    pub const SEC_MASK: u32 = 0x00000003;
    pub const SEC_READ_WRITE: u32 = 0 << 0;
    pub const SEC_READ_ONLY: u32 = 1 << 0;
    pub const SEC_INACCESSIBLE: u32 = 3 << 0;

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

    /// **NSEC** [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub fn nsec(&self) -> u32 {
        (self.read_bits() & Self::NSEC_MASK) >> Self::NSEC_OFFSET
    }

    /// Write NSEC field
    #[inline(always)]
    pub fn write_nsec(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::NSEC_MASK) | ((value << Self::NSEC_OFFSET) & Self::NSEC_MASK));
    }

    /// **SEC** [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub fn sec(&self) -> u32 {
        (self.read_bits() & Self::SEC_MASK) >> Self::SEC_OFFSET
    }

    /// Write SEC field
    #[inline(always)]
    pub fn write_sec(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::SEC_MASK) | ((value << Self::SEC_OFFSET) & Self::SEC_MASK));
    }

}

/// SW_LOCK44 value type for batch read/modify/write operations
///
/// Use `swlock44.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `swlock44.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SwLock44Val {
    bits: u32,
}

impl SwLock44Val {
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

    /// Read NSEC [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub const fn nsec(&self) -> u32 {
        (self.bits & SwLock44::NSEC_MASK) >> SwLock44::NSEC_OFFSET
    }

    /// Read SEC [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub const fn sec(&self) -> u32 {
        (self.bits & SwLock44::SEC_MASK) >> SwLock44::SEC_OFFSET
    }

    /// Set NSEC [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub const fn with_nsec(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SwLock44::NSEC_MASK) | ((value << SwLock44::NSEC_OFFSET) & SwLock44::NSEC_MASK)
        }
    }

    /// Set SEC [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub const fn with_sec(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SwLock44::SEC_MASK) | ((value << SwLock44::SEC_OFFSET) & SwLock44::SEC_MASK)
        }
    }

}

impl Default for SwLock44Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SwLock44 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> SwLock44Val {
        SwLock44Val::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: SwLock44Val) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(SwLock44Val) -> SwLock44Val>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// SW_LOCK45 Register
///
/// Software lock register for page 45. Locks are initialised from the OTP lock
/// pages at reset. This register can be written to further advance the lock
/// state of each page (until next reset), and read to check the current lock
/// state of a page.
///
/// Address offset: 0x00b4
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **NSEC** [3:2] (RW): Non-secure lock status. Writes are OR'd with the current value.
/// - **SEC** [1:0] (RW): Secure lock status. Writes are OR'd with the current value. This field is r...
#[repr(transparent)]
pub struct SwLock45 {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for SwLock45 {}
unsafe impl Sync for SwLock45 {}

impl SwLock45 {
    // ======== Field Constants ========

    /// NSEC - Non-secure lock status. Writes are OR'd with the current value.
    /// Bits [3:2], mask 0x0000000c
    pub const NSEC_OFFSET: u32 = 2;
    pub const NSEC_MASK: u32 = 0x0000000c;
    pub const NSEC_READ_WRITE: u32 = 0 << 2;
    pub const NSEC_READ_ONLY: u32 = 1 << 2;
    pub const NSEC_INACCESSIBLE: u32 = 3 << 2;

    /// SEC - Secure lock status. Writes are OR'd with the current value. This field is r...
    /// Bits [1:0], mask 0x00000003
    pub const SEC_OFFSET: u32 = 0;
    pub const SEC_MASK: u32 = 0x00000003;
    pub const SEC_READ_WRITE: u32 = 0 << 0;
    pub const SEC_READ_ONLY: u32 = 1 << 0;
    pub const SEC_INACCESSIBLE: u32 = 3 << 0;

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

    /// **NSEC** [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub fn nsec(&self) -> u32 {
        (self.read_bits() & Self::NSEC_MASK) >> Self::NSEC_OFFSET
    }

    /// Write NSEC field
    #[inline(always)]
    pub fn write_nsec(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::NSEC_MASK) | ((value << Self::NSEC_OFFSET) & Self::NSEC_MASK));
    }

    /// **SEC** [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub fn sec(&self) -> u32 {
        (self.read_bits() & Self::SEC_MASK) >> Self::SEC_OFFSET
    }

    /// Write SEC field
    #[inline(always)]
    pub fn write_sec(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::SEC_MASK) | ((value << Self::SEC_OFFSET) & Self::SEC_MASK));
    }

}

/// SW_LOCK45 value type for batch read/modify/write operations
///
/// Use `swlock45.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `swlock45.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SwLock45Val {
    bits: u32,
}

impl SwLock45Val {
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

    /// Read NSEC [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub const fn nsec(&self) -> u32 {
        (self.bits & SwLock45::NSEC_MASK) >> SwLock45::NSEC_OFFSET
    }

    /// Read SEC [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub const fn sec(&self) -> u32 {
        (self.bits & SwLock45::SEC_MASK) >> SwLock45::SEC_OFFSET
    }

    /// Set NSEC [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub const fn with_nsec(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SwLock45::NSEC_MASK) | ((value << SwLock45::NSEC_OFFSET) & SwLock45::NSEC_MASK)
        }
    }

    /// Set SEC [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub const fn with_sec(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SwLock45::SEC_MASK) | ((value << SwLock45::SEC_OFFSET) & SwLock45::SEC_MASK)
        }
    }

}

impl Default for SwLock45Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SwLock45 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> SwLock45Val {
        SwLock45Val::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: SwLock45Val) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(SwLock45Val) -> SwLock45Val>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// SW_LOCK46 Register
///
/// Software lock register for page 46. Locks are initialised from the OTP lock
/// pages at reset. This register can be written to further advance the lock
/// state of each page (until next reset), and read to check the current lock
/// state of a page.
///
/// Address offset: 0x00b8
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **NSEC** [3:2] (RW): Non-secure lock status. Writes are OR'd with the current value.
/// - **SEC** [1:0] (RW): Secure lock status. Writes are OR'd with the current value. This field is r...
#[repr(transparent)]
pub struct SwLock46 {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for SwLock46 {}
unsafe impl Sync for SwLock46 {}

impl SwLock46 {
    // ======== Field Constants ========

    /// NSEC - Non-secure lock status. Writes are OR'd with the current value.
    /// Bits [3:2], mask 0x0000000c
    pub const NSEC_OFFSET: u32 = 2;
    pub const NSEC_MASK: u32 = 0x0000000c;
    pub const NSEC_READ_WRITE: u32 = 0 << 2;
    pub const NSEC_READ_ONLY: u32 = 1 << 2;
    pub const NSEC_INACCESSIBLE: u32 = 3 << 2;

    /// SEC - Secure lock status. Writes are OR'd with the current value. This field is r...
    /// Bits [1:0], mask 0x00000003
    pub const SEC_OFFSET: u32 = 0;
    pub const SEC_MASK: u32 = 0x00000003;
    pub const SEC_READ_WRITE: u32 = 0 << 0;
    pub const SEC_READ_ONLY: u32 = 1 << 0;
    pub const SEC_INACCESSIBLE: u32 = 3 << 0;

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

    /// **NSEC** [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub fn nsec(&self) -> u32 {
        (self.read_bits() & Self::NSEC_MASK) >> Self::NSEC_OFFSET
    }

    /// Write NSEC field
    #[inline(always)]
    pub fn write_nsec(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::NSEC_MASK) | ((value << Self::NSEC_OFFSET) & Self::NSEC_MASK));
    }

    /// **SEC** [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub fn sec(&self) -> u32 {
        (self.read_bits() & Self::SEC_MASK) >> Self::SEC_OFFSET
    }

    /// Write SEC field
    #[inline(always)]
    pub fn write_sec(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::SEC_MASK) | ((value << Self::SEC_OFFSET) & Self::SEC_MASK));
    }

}

/// SW_LOCK46 value type for batch read/modify/write operations
///
/// Use `swlock46.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `swlock46.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SwLock46Val {
    bits: u32,
}

impl SwLock46Val {
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

    /// Read NSEC [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub const fn nsec(&self) -> u32 {
        (self.bits & SwLock46::NSEC_MASK) >> SwLock46::NSEC_OFFSET
    }

    /// Read SEC [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub const fn sec(&self) -> u32 {
        (self.bits & SwLock46::SEC_MASK) >> SwLock46::SEC_OFFSET
    }

    /// Set NSEC [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub const fn with_nsec(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SwLock46::NSEC_MASK) | ((value << SwLock46::NSEC_OFFSET) & SwLock46::NSEC_MASK)
        }
    }

    /// Set SEC [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub const fn with_sec(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SwLock46::SEC_MASK) | ((value << SwLock46::SEC_OFFSET) & SwLock46::SEC_MASK)
        }
    }

}

impl Default for SwLock46Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SwLock46 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> SwLock46Val {
        SwLock46Val::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: SwLock46Val) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(SwLock46Val) -> SwLock46Val>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// SW_LOCK47 Register
///
/// Software lock register for page 47. Locks are initialised from the OTP lock
/// pages at reset. This register can be written to further advance the lock
/// state of each page (until next reset), and read to check the current lock
/// state of a page.
///
/// Address offset: 0x00bc
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **NSEC** [3:2] (RW): Non-secure lock status. Writes are OR'd with the current value.
/// - **SEC** [1:0] (RW): Secure lock status. Writes are OR'd with the current value. This field is r...
#[repr(transparent)]
pub struct SwLock47 {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for SwLock47 {}
unsafe impl Sync for SwLock47 {}

impl SwLock47 {
    // ======== Field Constants ========

    /// NSEC - Non-secure lock status. Writes are OR'd with the current value.
    /// Bits [3:2], mask 0x0000000c
    pub const NSEC_OFFSET: u32 = 2;
    pub const NSEC_MASK: u32 = 0x0000000c;
    pub const NSEC_READ_WRITE: u32 = 0 << 2;
    pub const NSEC_READ_ONLY: u32 = 1 << 2;
    pub const NSEC_INACCESSIBLE: u32 = 3 << 2;

    /// SEC - Secure lock status. Writes are OR'd with the current value. This field is r...
    /// Bits [1:0], mask 0x00000003
    pub const SEC_OFFSET: u32 = 0;
    pub const SEC_MASK: u32 = 0x00000003;
    pub const SEC_READ_WRITE: u32 = 0 << 0;
    pub const SEC_READ_ONLY: u32 = 1 << 0;
    pub const SEC_INACCESSIBLE: u32 = 3 << 0;

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

    /// **NSEC** [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub fn nsec(&self) -> u32 {
        (self.read_bits() & Self::NSEC_MASK) >> Self::NSEC_OFFSET
    }

    /// Write NSEC field
    #[inline(always)]
    pub fn write_nsec(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::NSEC_MASK) | ((value << Self::NSEC_OFFSET) & Self::NSEC_MASK));
    }

    /// **SEC** [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub fn sec(&self) -> u32 {
        (self.read_bits() & Self::SEC_MASK) >> Self::SEC_OFFSET
    }

    /// Write SEC field
    #[inline(always)]
    pub fn write_sec(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::SEC_MASK) | ((value << Self::SEC_OFFSET) & Self::SEC_MASK));
    }

}

/// SW_LOCK47 value type for batch read/modify/write operations
///
/// Use `swlock47.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `swlock47.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SwLock47Val {
    bits: u32,
}

impl SwLock47Val {
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

    /// Read NSEC [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub const fn nsec(&self) -> u32 {
        (self.bits & SwLock47::NSEC_MASK) >> SwLock47::NSEC_OFFSET
    }

    /// Read SEC [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub const fn sec(&self) -> u32 {
        (self.bits & SwLock47::SEC_MASK) >> SwLock47::SEC_OFFSET
    }

    /// Set NSEC [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub const fn with_nsec(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SwLock47::NSEC_MASK) | ((value << SwLock47::NSEC_OFFSET) & SwLock47::NSEC_MASK)
        }
    }

    /// Set SEC [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub const fn with_sec(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SwLock47::SEC_MASK) | ((value << SwLock47::SEC_OFFSET) & SwLock47::SEC_MASK)
        }
    }

}

impl Default for SwLock47Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SwLock47 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> SwLock47Val {
        SwLock47Val::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: SwLock47Val) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(SwLock47Val) -> SwLock47Val>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// SW_LOCK48 Register
///
/// Software lock register for page 48. Locks are initialised from the OTP lock
/// pages at reset. This register can be written to further advance the lock
/// state of each page (until next reset), and read to check the current lock
/// state of a page.
///
/// Address offset: 0x00c0
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **NSEC** [3:2] (RW): Non-secure lock status. Writes are OR'd with the current value.
/// - **SEC** [1:0] (RW): Secure lock status. Writes are OR'd with the current value. This field is r...
#[repr(transparent)]
pub struct SwLock48 {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for SwLock48 {}
unsafe impl Sync for SwLock48 {}

impl SwLock48 {
    // ======== Field Constants ========

    /// NSEC - Non-secure lock status. Writes are OR'd with the current value.
    /// Bits [3:2], mask 0x0000000c
    pub const NSEC_OFFSET: u32 = 2;
    pub const NSEC_MASK: u32 = 0x0000000c;
    pub const NSEC_READ_WRITE: u32 = 0 << 2;
    pub const NSEC_READ_ONLY: u32 = 1 << 2;
    pub const NSEC_INACCESSIBLE: u32 = 3 << 2;

    /// SEC - Secure lock status. Writes are OR'd with the current value. This field is r...
    /// Bits [1:0], mask 0x00000003
    pub const SEC_OFFSET: u32 = 0;
    pub const SEC_MASK: u32 = 0x00000003;
    pub const SEC_READ_WRITE: u32 = 0 << 0;
    pub const SEC_READ_ONLY: u32 = 1 << 0;
    pub const SEC_INACCESSIBLE: u32 = 3 << 0;

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

    /// **NSEC** [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub fn nsec(&self) -> u32 {
        (self.read_bits() & Self::NSEC_MASK) >> Self::NSEC_OFFSET
    }

    /// Write NSEC field
    #[inline(always)]
    pub fn write_nsec(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::NSEC_MASK) | ((value << Self::NSEC_OFFSET) & Self::NSEC_MASK));
    }

    /// **SEC** [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub fn sec(&self) -> u32 {
        (self.read_bits() & Self::SEC_MASK) >> Self::SEC_OFFSET
    }

    /// Write SEC field
    #[inline(always)]
    pub fn write_sec(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::SEC_MASK) | ((value << Self::SEC_OFFSET) & Self::SEC_MASK));
    }

}

/// SW_LOCK48 value type for batch read/modify/write operations
///
/// Use `swlock48.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `swlock48.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SwLock48Val {
    bits: u32,
}

impl SwLock48Val {
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

    /// Read NSEC [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub const fn nsec(&self) -> u32 {
        (self.bits & SwLock48::NSEC_MASK) >> SwLock48::NSEC_OFFSET
    }

    /// Read SEC [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub const fn sec(&self) -> u32 {
        (self.bits & SwLock48::SEC_MASK) >> SwLock48::SEC_OFFSET
    }

    /// Set NSEC [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub const fn with_nsec(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SwLock48::NSEC_MASK) | ((value << SwLock48::NSEC_OFFSET) & SwLock48::NSEC_MASK)
        }
    }

    /// Set SEC [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub const fn with_sec(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SwLock48::SEC_MASK) | ((value << SwLock48::SEC_OFFSET) & SwLock48::SEC_MASK)
        }
    }

}

impl Default for SwLock48Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SwLock48 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> SwLock48Val {
        SwLock48Val::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: SwLock48Val) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(SwLock48Val) -> SwLock48Val>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// SW_LOCK49 Register
///
/// Software lock register for page 49. Locks are initialised from the OTP lock
/// pages at reset. This register can be written to further advance the lock
/// state of each page (until next reset), and read to check the current lock
/// state of a page.
///
/// Address offset: 0x00c4
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **NSEC** [3:2] (RW): Non-secure lock status. Writes are OR'd with the current value.
/// - **SEC** [1:0] (RW): Secure lock status. Writes are OR'd with the current value. This field is r...
#[repr(transparent)]
pub struct SwLock49 {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for SwLock49 {}
unsafe impl Sync for SwLock49 {}

impl SwLock49 {
    // ======== Field Constants ========

    /// NSEC - Non-secure lock status. Writes are OR'd with the current value.
    /// Bits [3:2], mask 0x0000000c
    pub const NSEC_OFFSET: u32 = 2;
    pub const NSEC_MASK: u32 = 0x0000000c;
    pub const NSEC_READ_WRITE: u32 = 0 << 2;
    pub const NSEC_READ_ONLY: u32 = 1 << 2;
    pub const NSEC_INACCESSIBLE: u32 = 3 << 2;

    /// SEC - Secure lock status. Writes are OR'd with the current value. This field is r...
    /// Bits [1:0], mask 0x00000003
    pub const SEC_OFFSET: u32 = 0;
    pub const SEC_MASK: u32 = 0x00000003;
    pub const SEC_READ_WRITE: u32 = 0 << 0;
    pub const SEC_READ_ONLY: u32 = 1 << 0;
    pub const SEC_INACCESSIBLE: u32 = 3 << 0;

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

    /// **NSEC** [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub fn nsec(&self) -> u32 {
        (self.read_bits() & Self::NSEC_MASK) >> Self::NSEC_OFFSET
    }

    /// Write NSEC field
    #[inline(always)]
    pub fn write_nsec(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::NSEC_MASK) | ((value << Self::NSEC_OFFSET) & Self::NSEC_MASK));
    }

    /// **SEC** [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub fn sec(&self) -> u32 {
        (self.read_bits() & Self::SEC_MASK) >> Self::SEC_OFFSET
    }

    /// Write SEC field
    #[inline(always)]
    pub fn write_sec(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::SEC_MASK) | ((value << Self::SEC_OFFSET) & Self::SEC_MASK));
    }

}

/// SW_LOCK49 value type for batch read/modify/write operations
///
/// Use `swlock49.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `swlock49.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SwLock49Val {
    bits: u32,
}

impl SwLock49Val {
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

    /// Read NSEC [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub const fn nsec(&self) -> u32 {
        (self.bits & SwLock49::NSEC_MASK) >> SwLock49::NSEC_OFFSET
    }

    /// Read SEC [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub const fn sec(&self) -> u32 {
        (self.bits & SwLock49::SEC_MASK) >> SwLock49::SEC_OFFSET
    }

    /// Set NSEC [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub const fn with_nsec(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SwLock49::NSEC_MASK) | ((value << SwLock49::NSEC_OFFSET) & SwLock49::NSEC_MASK)
        }
    }

    /// Set SEC [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub const fn with_sec(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SwLock49::SEC_MASK) | ((value << SwLock49::SEC_OFFSET) & SwLock49::SEC_MASK)
        }
    }

}

impl Default for SwLock49Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SwLock49 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> SwLock49Val {
        SwLock49Val::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: SwLock49Val) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(SwLock49Val) -> SwLock49Val>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// SW_LOCK50 Register
///
/// Software lock register for page 50. Locks are initialised from the OTP lock
/// pages at reset. This register can be written to further advance the lock
/// state of each page (until next reset), and read to check the current lock
/// state of a page.
///
/// Address offset: 0x00c8
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **NSEC** [3:2] (RW): Non-secure lock status. Writes are OR'd with the current value.
/// - **SEC** [1:0] (RW): Secure lock status. Writes are OR'd with the current value. This field is r...
#[repr(transparent)]
pub struct SwLock50 {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for SwLock50 {}
unsafe impl Sync for SwLock50 {}

impl SwLock50 {
    // ======== Field Constants ========

    /// NSEC - Non-secure lock status. Writes are OR'd with the current value.
    /// Bits [3:2], mask 0x0000000c
    pub const NSEC_OFFSET: u32 = 2;
    pub const NSEC_MASK: u32 = 0x0000000c;
    pub const NSEC_READ_WRITE: u32 = 0 << 2;
    pub const NSEC_READ_ONLY: u32 = 1 << 2;
    pub const NSEC_INACCESSIBLE: u32 = 3 << 2;

    /// SEC - Secure lock status. Writes are OR'd with the current value. This field is r...
    /// Bits [1:0], mask 0x00000003
    pub const SEC_OFFSET: u32 = 0;
    pub const SEC_MASK: u32 = 0x00000003;
    pub const SEC_READ_WRITE: u32 = 0 << 0;
    pub const SEC_READ_ONLY: u32 = 1 << 0;
    pub const SEC_INACCESSIBLE: u32 = 3 << 0;

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

    /// **NSEC** [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub fn nsec(&self) -> u32 {
        (self.read_bits() & Self::NSEC_MASK) >> Self::NSEC_OFFSET
    }

    /// Write NSEC field
    #[inline(always)]
    pub fn write_nsec(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::NSEC_MASK) | ((value << Self::NSEC_OFFSET) & Self::NSEC_MASK));
    }

    /// **SEC** [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub fn sec(&self) -> u32 {
        (self.read_bits() & Self::SEC_MASK) >> Self::SEC_OFFSET
    }

    /// Write SEC field
    #[inline(always)]
    pub fn write_sec(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::SEC_MASK) | ((value << Self::SEC_OFFSET) & Self::SEC_MASK));
    }

}

/// SW_LOCK50 value type for batch read/modify/write operations
///
/// Use `swlock50.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `swlock50.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SwLock50Val {
    bits: u32,
}

impl SwLock50Val {
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

    /// Read NSEC [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub const fn nsec(&self) -> u32 {
        (self.bits & SwLock50::NSEC_MASK) >> SwLock50::NSEC_OFFSET
    }

    /// Read SEC [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub const fn sec(&self) -> u32 {
        (self.bits & SwLock50::SEC_MASK) >> SwLock50::SEC_OFFSET
    }

    /// Set NSEC [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub const fn with_nsec(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SwLock50::NSEC_MASK) | ((value << SwLock50::NSEC_OFFSET) & SwLock50::NSEC_MASK)
        }
    }

    /// Set SEC [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub const fn with_sec(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SwLock50::SEC_MASK) | ((value << SwLock50::SEC_OFFSET) & SwLock50::SEC_MASK)
        }
    }

}

impl Default for SwLock50Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SwLock50 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> SwLock50Val {
        SwLock50Val::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: SwLock50Val) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(SwLock50Val) -> SwLock50Val>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// SW_LOCK51 Register
///
/// Software lock register for page 51. Locks are initialised from the OTP lock
/// pages at reset. This register can be written to further advance the lock
/// state of each page (until next reset), and read to check the current lock
/// state of a page.
///
/// Address offset: 0x00cc
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **NSEC** [3:2] (RW): Non-secure lock status. Writes are OR'd with the current value.
/// - **SEC** [1:0] (RW): Secure lock status. Writes are OR'd with the current value. This field is r...
#[repr(transparent)]
pub struct SwLock51 {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for SwLock51 {}
unsafe impl Sync for SwLock51 {}

impl SwLock51 {
    // ======== Field Constants ========

    /// NSEC - Non-secure lock status. Writes are OR'd with the current value.
    /// Bits [3:2], mask 0x0000000c
    pub const NSEC_OFFSET: u32 = 2;
    pub const NSEC_MASK: u32 = 0x0000000c;
    pub const NSEC_READ_WRITE: u32 = 0 << 2;
    pub const NSEC_READ_ONLY: u32 = 1 << 2;
    pub const NSEC_INACCESSIBLE: u32 = 3 << 2;

    /// SEC - Secure lock status. Writes are OR'd with the current value. This field is r...
    /// Bits [1:0], mask 0x00000003
    pub const SEC_OFFSET: u32 = 0;
    pub const SEC_MASK: u32 = 0x00000003;
    pub const SEC_READ_WRITE: u32 = 0 << 0;
    pub const SEC_READ_ONLY: u32 = 1 << 0;
    pub const SEC_INACCESSIBLE: u32 = 3 << 0;

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

    /// **NSEC** [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub fn nsec(&self) -> u32 {
        (self.read_bits() & Self::NSEC_MASK) >> Self::NSEC_OFFSET
    }

    /// Write NSEC field
    #[inline(always)]
    pub fn write_nsec(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::NSEC_MASK) | ((value << Self::NSEC_OFFSET) & Self::NSEC_MASK));
    }

    /// **SEC** [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub fn sec(&self) -> u32 {
        (self.read_bits() & Self::SEC_MASK) >> Self::SEC_OFFSET
    }

    /// Write SEC field
    #[inline(always)]
    pub fn write_sec(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::SEC_MASK) | ((value << Self::SEC_OFFSET) & Self::SEC_MASK));
    }

}

/// SW_LOCK51 value type for batch read/modify/write operations
///
/// Use `swlock51.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `swlock51.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SwLock51Val {
    bits: u32,
}

impl SwLock51Val {
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

    /// Read NSEC [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub const fn nsec(&self) -> u32 {
        (self.bits & SwLock51::NSEC_MASK) >> SwLock51::NSEC_OFFSET
    }

    /// Read SEC [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub const fn sec(&self) -> u32 {
        (self.bits & SwLock51::SEC_MASK) >> SwLock51::SEC_OFFSET
    }

    /// Set NSEC [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub const fn with_nsec(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SwLock51::NSEC_MASK) | ((value << SwLock51::NSEC_OFFSET) & SwLock51::NSEC_MASK)
        }
    }

    /// Set SEC [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub const fn with_sec(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SwLock51::SEC_MASK) | ((value << SwLock51::SEC_OFFSET) & SwLock51::SEC_MASK)
        }
    }

}

impl Default for SwLock51Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SwLock51 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> SwLock51Val {
        SwLock51Val::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: SwLock51Val) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(SwLock51Val) -> SwLock51Val>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// SW_LOCK52 Register
///
/// Software lock register for page 52. Locks are initialised from the OTP lock
/// pages at reset. This register can be written to further advance the lock
/// state of each page (until next reset), and read to check the current lock
/// state of a page.
///
/// Address offset: 0x00d0
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **NSEC** [3:2] (RW): Non-secure lock status. Writes are OR'd with the current value.
/// - **SEC** [1:0] (RW): Secure lock status. Writes are OR'd with the current value. This field is r...
#[repr(transparent)]
pub struct SwLock52 {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for SwLock52 {}
unsafe impl Sync for SwLock52 {}

impl SwLock52 {
    // ======== Field Constants ========

    /// NSEC - Non-secure lock status. Writes are OR'd with the current value.
    /// Bits [3:2], mask 0x0000000c
    pub const NSEC_OFFSET: u32 = 2;
    pub const NSEC_MASK: u32 = 0x0000000c;
    pub const NSEC_READ_WRITE: u32 = 0 << 2;
    pub const NSEC_READ_ONLY: u32 = 1 << 2;
    pub const NSEC_INACCESSIBLE: u32 = 3 << 2;

    /// SEC - Secure lock status. Writes are OR'd with the current value. This field is r...
    /// Bits [1:0], mask 0x00000003
    pub const SEC_OFFSET: u32 = 0;
    pub const SEC_MASK: u32 = 0x00000003;
    pub const SEC_READ_WRITE: u32 = 0 << 0;
    pub const SEC_READ_ONLY: u32 = 1 << 0;
    pub const SEC_INACCESSIBLE: u32 = 3 << 0;

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

    /// **NSEC** [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub fn nsec(&self) -> u32 {
        (self.read_bits() & Self::NSEC_MASK) >> Self::NSEC_OFFSET
    }

    /// Write NSEC field
    #[inline(always)]
    pub fn write_nsec(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::NSEC_MASK) | ((value << Self::NSEC_OFFSET) & Self::NSEC_MASK));
    }

    /// **SEC** [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub fn sec(&self) -> u32 {
        (self.read_bits() & Self::SEC_MASK) >> Self::SEC_OFFSET
    }

    /// Write SEC field
    #[inline(always)]
    pub fn write_sec(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::SEC_MASK) | ((value << Self::SEC_OFFSET) & Self::SEC_MASK));
    }

}

/// SW_LOCK52 value type for batch read/modify/write operations
///
/// Use `swlock52.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `swlock52.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SwLock52Val {
    bits: u32,
}

impl SwLock52Val {
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

    /// Read NSEC [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub const fn nsec(&self) -> u32 {
        (self.bits & SwLock52::NSEC_MASK) >> SwLock52::NSEC_OFFSET
    }

    /// Read SEC [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub const fn sec(&self) -> u32 {
        (self.bits & SwLock52::SEC_MASK) >> SwLock52::SEC_OFFSET
    }

    /// Set NSEC [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub const fn with_nsec(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SwLock52::NSEC_MASK) | ((value << SwLock52::NSEC_OFFSET) & SwLock52::NSEC_MASK)
        }
    }

    /// Set SEC [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub const fn with_sec(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SwLock52::SEC_MASK) | ((value << SwLock52::SEC_OFFSET) & SwLock52::SEC_MASK)
        }
    }

}

impl Default for SwLock52Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SwLock52 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> SwLock52Val {
        SwLock52Val::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: SwLock52Val) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(SwLock52Val) -> SwLock52Val>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// SW_LOCK53 Register
///
/// Software lock register for page 53. Locks are initialised from the OTP lock
/// pages at reset. This register can be written to further advance the lock
/// state of each page (until next reset), and read to check the current lock
/// state of a page.
///
/// Address offset: 0x00d4
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **NSEC** [3:2] (RW): Non-secure lock status. Writes are OR'd with the current value.
/// - **SEC** [1:0] (RW): Secure lock status. Writes are OR'd with the current value. This field is r...
#[repr(transparent)]
pub struct SwLock53 {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for SwLock53 {}
unsafe impl Sync for SwLock53 {}

impl SwLock53 {
    // ======== Field Constants ========

    /// NSEC - Non-secure lock status. Writes are OR'd with the current value.
    /// Bits [3:2], mask 0x0000000c
    pub const NSEC_OFFSET: u32 = 2;
    pub const NSEC_MASK: u32 = 0x0000000c;
    pub const NSEC_READ_WRITE: u32 = 0 << 2;
    pub const NSEC_READ_ONLY: u32 = 1 << 2;
    pub const NSEC_INACCESSIBLE: u32 = 3 << 2;

    /// SEC - Secure lock status. Writes are OR'd with the current value. This field is r...
    /// Bits [1:0], mask 0x00000003
    pub const SEC_OFFSET: u32 = 0;
    pub const SEC_MASK: u32 = 0x00000003;
    pub const SEC_READ_WRITE: u32 = 0 << 0;
    pub const SEC_READ_ONLY: u32 = 1 << 0;
    pub const SEC_INACCESSIBLE: u32 = 3 << 0;

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

    /// **NSEC** [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub fn nsec(&self) -> u32 {
        (self.read_bits() & Self::NSEC_MASK) >> Self::NSEC_OFFSET
    }

    /// Write NSEC field
    #[inline(always)]
    pub fn write_nsec(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::NSEC_MASK) | ((value << Self::NSEC_OFFSET) & Self::NSEC_MASK));
    }

    /// **SEC** [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub fn sec(&self) -> u32 {
        (self.read_bits() & Self::SEC_MASK) >> Self::SEC_OFFSET
    }

    /// Write SEC field
    #[inline(always)]
    pub fn write_sec(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::SEC_MASK) | ((value << Self::SEC_OFFSET) & Self::SEC_MASK));
    }

}

/// SW_LOCK53 value type for batch read/modify/write operations
///
/// Use `swlock53.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `swlock53.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SwLock53Val {
    bits: u32,
}

impl SwLock53Val {
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

    /// Read NSEC [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub const fn nsec(&self) -> u32 {
        (self.bits & SwLock53::NSEC_MASK) >> SwLock53::NSEC_OFFSET
    }

    /// Read SEC [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub const fn sec(&self) -> u32 {
        (self.bits & SwLock53::SEC_MASK) >> SwLock53::SEC_OFFSET
    }

    /// Set NSEC [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub const fn with_nsec(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SwLock53::NSEC_MASK) | ((value << SwLock53::NSEC_OFFSET) & SwLock53::NSEC_MASK)
        }
    }

    /// Set SEC [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub const fn with_sec(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SwLock53::SEC_MASK) | ((value << SwLock53::SEC_OFFSET) & SwLock53::SEC_MASK)
        }
    }

}

impl Default for SwLock53Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SwLock53 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> SwLock53Val {
        SwLock53Val::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: SwLock53Val) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(SwLock53Val) -> SwLock53Val>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// SW_LOCK54 Register
///
/// Software lock register for page 54. Locks are initialised from the OTP lock
/// pages at reset. This register can be written to further advance the lock
/// state of each page (until next reset), and read to check the current lock
/// state of a page.
///
/// Address offset: 0x00d8
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **NSEC** [3:2] (RW): Non-secure lock status. Writes are OR'd with the current value.
/// - **SEC** [1:0] (RW): Secure lock status. Writes are OR'd with the current value. This field is r...
#[repr(transparent)]
pub struct SwLock54 {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for SwLock54 {}
unsafe impl Sync for SwLock54 {}

impl SwLock54 {
    // ======== Field Constants ========

    /// NSEC - Non-secure lock status. Writes are OR'd with the current value.
    /// Bits [3:2], mask 0x0000000c
    pub const NSEC_OFFSET: u32 = 2;
    pub const NSEC_MASK: u32 = 0x0000000c;
    pub const NSEC_READ_WRITE: u32 = 0 << 2;
    pub const NSEC_READ_ONLY: u32 = 1 << 2;
    pub const NSEC_INACCESSIBLE: u32 = 3 << 2;

    /// SEC - Secure lock status. Writes are OR'd with the current value. This field is r...
    /// Bits [1:0], mask 0x00000003
    pub const SEC_OFFSET: u32 = 0;
    pub const SEC_MASK: u32 = 0x00000003;
    pub const SEC_READ_WRITE: u32 = 0 << 0;
    pub const SEC_READ_ONLY: u32 = 1 << 0;
    pub const SEC_INACCESSIBLE: u32 = 3 << 0;

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

    /// **NSEC** [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub fn nsec(&self) -> u32 {
        (self.read_bits() & Self::NSEC_MASK) >> Self::NSEC_OFFSET
    }

    /// Write NSEC field
    #[inline(always)]
    pub fn write_nsec(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::NSEC_MASK) | ((value << Self::NSEC_OFFSET) & Self::NSEC_MASK));
    }

    /// **SEC** [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub fn sec(&self) -> u32 {
        (self.read_bits() & Self::SEC_MASK) >> Self::SEC_OFFSET
    }

    /// Write SEC field
    #[inline(always)]
    pub fn write_sec(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::SEC_MASK) | ((value << Self::SEC_OFFSET) & Self::SEC_MASK));
    }

}

/// SW_LOCK54 value type for batch read/modify/write operations
///
/// Use `swlock54.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `swlock54.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SwLock54Val {
    bits: u32,
}

impl SwLock54Val {
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

    /// Read NSEC [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub const fn nsec(&self) -> u32 {
        (self.bits & SwLock54::NSEC_MASK) >> SwLock54::NSEC_OFFSET
    }

    /// Read SEC [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub const fn sec(&self) -> u32 {
        (self.bits & SwLock54::SEC_MASK) >> SwLock54::SEC_OFFSET
    }

    /// Set NSEC [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub const fn with_nsec(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SwLock54::NSEC_MASK) | ((value << SwLock54::NSEC_OFFSET) & SwLock54::NSEC_MASK)
        }
    }

    /// Set SEC [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub const fn with_sec(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SwLock54::SEC_MASK) | ((value << SwLock54::SEC_OFFSET) & SwLock54::SEC_MASK)
        }
    }

}

impl Default for SwLock54Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SwLock54 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> SwLock54Val {
        SwLock54Val::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: SwLock54Val) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(SwLock54Val) -> SwLock54Val>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// SW_LOCK55 Register
///
/// Software lock register for page 55. Locks are initialised from the OTP lock
/// pages at reset. This register can be written to further advance the lock
/// state of each page (until next reset), and read to check the current lock
/// state of a page.
///
/// Address offset: 0x00dc
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **NSEC** [3:2] (RW): Non-secure lock status. Writes are OR'd with the current value.
/// - **SEC** [1:0] (RW): Secure lock status. Writes are OR'd with the current value. This field is r...
#[repr(transparent)]
pub struct SwLock55 {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for SwLock55 {}
unsafe impl Sync for SwLock55 {}

impl SwLock55 {
    // ======== Field Constants ========

    /// NSEC - Non-secure lock status. Writes are OR'd with the current value.
    /// Bits [3:2], mask 0x0000000c
    pub const NSEC_OFFSET: u32 = 2;
    pub const NSEC_MASK: u32 = 0x0000000c;
    pub const NSEC_READ_WRITE: u32 = 0 << 2;
    pub const NSEC_READ_ONLY: u32 = 1 << 2;
    pub const NSEC_INACCESSIBLE: u32 = 3 << 2;

    /// SEC - Secure lock status. Writes are OR'd with the current value. This field is r...
    /// Bits [1:0], mask 0x00000003
    pub const SEC_OFFSET: u32 = 0;
    pub const SEC_MASK: u32 = 0x00000003;
    pub const SEC_READ_WRITE: u32 = 0 << 0;
    pub const SEC_READ_ONLY: u32 = 1 << 0;
    pub const SEC_INACCESSIBLE: u32 = 3 << 0;

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

    /// **NSEC** [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub fn nsec(&self) -> u32 {
        (self.read_bits() & Self::NSEC_MASK) >> Self::NSEC_OFFSET
    }

    /// Write NSEC field
    #[inline(always)]
    pub fn write_nsec(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::NSEC_MASK) | ((value << Self::NSEC_OFFSET) & Self::NSEC_MASK));
    }

    /// **SEC** [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub fn sec(&self) -> u32 {
        (self.read_bits() & Self::SEC_MASK) >> Self::SEC_OFFSET
    }

    /// Write SEC field
    #[inline(always)]
    pub fn write_sec(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::SEC_MASK) | ((value << Self::SEC_OFFSET) & Self::SEC_MASK));
    }

}

/// SW_LOCK55 value type for batch read/modify/write operations
///
/// Use `swlock55.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `swlock55.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SwLock55Val {
    bits: u32,
}

impl SwLock55Val {
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

    /// Read NSEC [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub const fn nsec(&self) -> u32 {
        (self.bits & SwLock55::NSEC_MASK) >> SwLock55::NSEC_OFFSET
    }

    /// Read SEC [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub const fn sec(&self) -> u32 {
        (self.bits & SwLock55::SEC_MASK) >> SwLock55::SEC_OFFSET
    }

    /// Set NSEC [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub const fn with_nsec(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SwLock55::NSEC_MASK) | ((value << SwLock55::NSEC_OFFSET) & SwLock55::NSEC_MASK)
        }
    }

    /// Set SEC [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub const fn with_sec(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SwLock55::SEC_MASK) | ((value << SwLock55::SEC_OFFSET) & SwLock55::SEC_MASK)
        }
    }

}

impl Default for SwLock55Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SwLock55 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> SwLock55Val {
        SwLock55Val::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: SwLock55Val) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(SwLock55Val) -> SwLock55Val>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// SW_LOCK56 Register
///
/// Software lock register for page 56. Locks are initialised from the OTP lock
/// pages at reset. This register can be written to further advance the lock
/// state of each page (until next reset), and read to check the current lock
/// state of a page.
///
/// Address offset: 0x00e0
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **NSEC** [3:2] (RW): Non-secure lock status. Writes are OR'd with the current value.
/// - **SEC** [1:0] (RW): Secure lock status. Writes are OR'd with the current value. This field is r...
#[repr(transparent)]
pub struct SwLock56 {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for SwLock56 {}
unsafe impl Sync for SwLock56 {}

impl SwLock56 {
    // ======== Field Constants ========

    /// NSEC - Non-secure lock status. Writes are OR'd with the current value.
    /// Bits [3:2], mask 0x0000000c
    pub const NSEC_OFFSET: u32 = 2;
    pub const NSEC_MASK: u32 = 0x0000000c;
    pub const NSEC_READ_WRITE: u32 = 0 << 2;
    pub const NSEC_READ_ONLY: u32 = 1 << 2;
    pub const NSEC_INACCESSIBLE: u32 = 3 << 2;

    /// SEC - Secure lock status. Writes are OR'd with the current value. This field is r...
    /// Bits [1:0], mask 0x00000003
    pub const SEC_OFFSET: u32 = 0;
    pub const SEC_MASK: u32 = 0x00000003;
    pub const SEC_READ_WRITE: u32 = 0 << 0;
    pub const SEC_READ_ONLY: u32 = 1 << 0;
    pub const SEC_INACCESSIBLE: u32 = 3 << 0;

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

    /// **NSEC** [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub fn nsec(&self) -> u32 {
        (self.read_bits() & Self::NSEC_MASK) >> Self::NSEC_OFFSET
    }

    /// Write NSEC field
    #[inline(always)]
    pub fn write_nsec(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::NSEC_MASK) | ((value << Self::NSEC_OFFSET) & Self::NSEC_MASK));
    }

    /// **SEC** [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub fn sec(&self) -> u32 {
        (self.read_bits() & Self::SEC_MASK) >> Self::SEC_OFFSET
    }

    /// Write SEC field
    #[inline(always)]
    pub fn write_sec(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::SEC_MASK) | ((value << Self::SEC_OFFSET) & Self::SEC_MASK));
    }

}

/// SW_LOCK56 value type for batch read/modify/write operations
///
/// Use `swlock56.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `swlock56.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SwLock56Val {
    bits: u32,
}

impl SwLock56Val {
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

    /// Read NSEC [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub const fn nsec(&self) -> u32 {
        (self.bits & SwLock56::NSEC_MASK) >> SwLock56::NSEC_OFFSET
    }

    /// Read SEC [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub const fn sec(&self) -> u32 {
        (self.bits & SwLock56::SEC_MASK) >> SwLock56::SEC_OFFSET
    }

    /// Set NSEC [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub const fn with_nsec(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SwLock56::NSEC_MASK) | ((value << SwLock56::NSEC_OFFSET) & SwLock56::NSEC_MASK)
        }
    }

    /// Set SEC [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub const fn with_sec(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SwLock56::SEC_MASK) | ((value << SwLock56::SEC_OFFSET) & SwLock56::SEC_MASK)
        }
    }

}

impl Default for SwLock56Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SwLock56 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> SwLock56Val {
        SwLock56Val::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: SwLock56Val) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(SwLock56Val) -> SwLock56Val>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// SW_LOCK57 Register
///
/// Software lock register for page 57. Locks are initialised from the OTP lock
/// pages at reset. This register can be written to further advance the lock
/// state of each page (until next reset), and read to check the current lock
/// state of a page.
///
/// Address offset: 0x00e4
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **NSEC** [3:2] (RW): Non-secure lock status. Writes are OR'd with the current value.
/// - **SEC** [1:0] (RW): Secure lock status. Writes are OR'd with the current value. This field is r...
#[repr(transparent)]
pub struct SwLock57 {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for SwLock57 {}
unsafe impl Sync for SwLock57 {}

impl SwLock57 {
    // ======== Field Constants ========

    /// NSEC - Non-secure lock status. Writes are OR'd with the current value.
    /// Bits [3:2], mask 0x0000000c
    pub const NSEC_OFFSET: u32 = 2;
    pub const NSEC_MASK: u32 = 0x0000000c;
    pub const NSEC_READ_WRITE: u32 = 0 << 2;
    pub const NSEC_READ_ONLY: u32 = 1 << 2;
    pub const NSEC_INACCESSIBLE: u32 = 3 << 2;

    /// SEC - Secure lock status. Writes are OR'd with the current value. This field is r...
    /// Bits [1:0], mask 0x00000003
    pub const SEC_OFFSET: u32 = 0;
    pub const SEC_MASK: u32 = 0x00000003;
    pub const SEC_READ_WRITE: u32 = 0 << 0;
    pub const SEC_READ_ONLY: u32 = 1 << 0;
    pub const SEC_INACCESSIBLE: u32 = 3 << 0;

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

    /// **NSEC** [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub fn nsec(&self) -> u32 {
        (self.read_bits() & Self::NSEC_MASK) >> Self::NSEC_OFFSET
    }

    /// Write NSEC field
    #[inline(always)]
    pub fn write_nsec(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::NSEC_MASK) | ((value << Self::NSEC_OFFSET) & Self::NSEC_MASK));
    }

    /// **SEC** [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub fn sec(&self) -> u32 {
        (self.read_bits() & Self::SEC_MASK) >> Self::SEC_OFFSET
    }

    /// Write SEC field
    #[inline(always)]
    pub fn write_sec(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::SEC_MASK) | ((value << Self::SEC_OFFSET) & Self::SEC_MASK));
    }

}

/// SW_LOCK57 value type for batch read/modify/write operations
///
/// Use `swlock57.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `swlock57.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SwLock57Val {
    bits: u32,
}

impl SwLock57Val {
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

    /// Read NSEC [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub const fn nsec(&self) -> u32 {
        (self.bits & SwLock57::NSEC_MASK) >> SwLock57::NSEC_OFFSET
    }

    /// Read SEC [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub const fn sec(&self) -> u32 {
        (self.bits & SwLock57::SEC_MASK) >> SwLock57::SEC_OFFSET
    }

    /// Set NSEC [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub const fn with_nsec(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SwLock57::NSEC_MASK) | ((value << SwLock57::NSEC_OFFSET) & SwLock57::NSEC_MASK)
        }
    }

    /// Set SEC [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub const fn with_sec(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SwLock57::SEC_MASK) | ((value << SwLock57::SEC_OFFSET) & SwLock57::SEC_MASK)
        }
    }

}

impl Default for SwLock57Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SwLock57 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> SwLock57Val {
        SwLock57Val::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: SwLock57Val) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(SwLock57Val) -> SwLock57Val>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// SW_LOCK58 Register
///
/// Software lock register for page 58. Locks are initialised from the OTP lock
/// pages at reset. This register can be written to further advance the lock
/// state of each page (until next reset), and read to check the current lock
/// state of a page.
///
/// Address offset: 0x00e8
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **NSEC** [3:2] (RW): Non-secure lock status. Writes are OR'd with the current value.
/// - **SEC** [1:0] (RW): Secure lock status. Writes are OR'd with the current value. This field is r...
#[repr(transparent)]
pub struct SwLock58 {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for SwLock58 {}
unsafe impl Sync for SwLock58 {}

impl SwLock58 {
    // ======== Field Constants ========

    /// NSEC - Non-secure lock status. Writes are OR'd with the current value.
    /// Bits [3:2], mask 0x0000000c
    pub const NSEC_OFFSET: u32 = 2;
    pub const NSEC_MASK: u32 = 0x0000000c;
    pub const NSEC_READ_WRITE: u32 = 0 << 2;
    pub const NSEC_READ_ONLY: u32 = 1 << 2;
    pub const NSEC_INACCESSIBLE: u32 = 3 << 2;

    /// SEC - Secure lock status. Writes are OR'd with the current value. This field is r...
    /// Bits [1:0], mask 0x00000003
    pub const SEC_OFFSET: u32 = 0;
    pub const SEC_MASK: u32 = 0x00000003;
    pub const SEC_READ_WRITE: u32 = 0 << 0;
    pub const SEC_READ_ONLY: u32 = 1 << 0;
    pub const SEC_INACCESSIBLE: u32 = 3 << 0;

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

    /// **NSEC** [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub fn nsec(&self) -> u32 {
        (self.read_bits() & Self::NSEC_MASK) >> Self::NSEC_OFFSET
    }

    /// Write NSEC field
    #[inline(always)]
    pub fn write_nsec(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::NSEC_MASK) | ((value << Self::NSEC_OFFSET) & Self::NSEC_MASK));
    }

    /// **SEC** [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub fn sec(&self) -> u32 {
        (self.read_bits() & Self::SEC_MASK) >> Self::SEC_OFFSET
    }

    /// Write SEC field
    #[inline(always)]
    pub fn write_sec(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::SEC_MASK) | ((value << Self::SEC_OFFSET) & Self::SEC_MASK));
    }

}

/// SW_LOCK58 value type for batch read/modify/write operations
///
/// Use `swlock58.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `swlock58.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SwLock58Val {
    bits: u32,
}

impl SwLock58Val {
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

    /// Read NSEC [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub const fn nsec(&self) -> u32 {
        (self.bits & SwLock58::NSEC_MASK) >> SwLock58::NSEC_OFFSET
    }

    /// Read SEC [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub const fn sec(&self) -> u32 {
        (self.bits & SwLock58::SEC_MASK) >> SwLock58::SEC_OFFSET
    }

    /// Set NSEC [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub const fn with_nsec(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SwLock58::NSEC_MASK) | ((value << SwLock58::NSEC_OFFSET) & SwLock58::NSEC_MASK)
        }
    }

    /// Set SEC [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub const fn with_sec(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SwLock58::SEC_MASK) | ((value << SwLock58::SEC_OFFSET) & SwLock58::SEC_MASK)
        }
    }

}

impl Default for SwLock58Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SwLock58 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> SwLock58Val {
        SwLock58Val::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: SwLock58Val) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(SwLock58Val) -> SwLock58Val>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// SW_LOCK59 Register
///
/// Software lock register for page 59. Locks are initialised from the OTP lock
/// pages at reset. This register can be written to further advance the lock
/// state of each page (until next reset), and read to check the current lock
/// state of a page.
///
/// Address offset: 0x00ec
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **NSEC** [3:2] (RW): Non-secure lock status. Writes are OR'd with the current value.
/// - **SEC** [1:0] (RW): Secure lock status. Writes are OR'd with the current value. This field is r...
#[repr(transparent)]
pub struct SwLock59 {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for SwLock59 {}
unsafe impl Sync for SwLock59 {}

impl SwLock59 {
    // ======== Field Constants ========

    /// NSEC - Non-secure lock status. Writes are OR'd with the current value.
    /// Bits [3:2], mask 0x0000000c
    pub const NSEC_OFFSET: u32 = 2;
    pub const NSEC_MASK: u32 = 0x0000000c;
    pub const NSEC_READ_WRITE: u32 = 0 << 2;
    pub const NSEC_READ_ONLY: u32 = 1 << 2;
    pub const NSEC_INACCESSIBLE: u32 = 3 << 2;

    /// SEC - Secure lock status. Writes are OR'd with the current value. This field is r...
    /// Bits [1:0], mask 0x00000003
    pub const SEC_OFFSET: u32 = 0;
    pub const SEC_MASK: u32 = 0x00000003;
    pub const SEC_READ_WRITE: u32 = 0 << 0;
    pub const SEC_READ_ONLY: u32 = 1 << 0;
    pub const SEC_INACCESSIBLE: u32 = 3 << 0;

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

    /// **NSEC** [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub fn nsec(&self) -> u32 {
        (self.read_bits() & Self::NSEC_MASK) >> Self::NSEC_OFFSET
    }

    /// Write NSEC field
    #[inline(always)]
    pub fn write_nsec(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::NSEC_MASK) | ((value << Self::NSEC_OFFSET) & Self::NSEC_MASK));
    }

    /// **SEC** [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub fn sec(&self) -> u32 {
        (self.read_bits() & Self::SEC_MASK) >> Self::SEC_OFFSET
    }

    /// Write SEC field
    #[inline(always)]
    pub fn write_sec(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::SEC_MASK) | ((value << Self::SEC_OFFSET) & Self::SEC_MASK));
    }

}

/// SW_LOCK59 value type for batch read/modify/write operations
///
/// Use `swlock59.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `swlock59.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SwLock59Val {
    bits: u32,
}

impl SwLock59Val {
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

    /// Read NSEC [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub const fn nsec(&self) -> u32 {
        (self.bits & SwLock59::NSEC_MASK) >> SwLock59::NSEC_OFFSET
    }

    /// Read SEC [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub const fn sec(&self) -> u32 {
        (self.bits & SwLock59::SEC_MASK) >> SwLock59::SEC_OFFSET
    }

    /// Set NSEC [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub const fn with_nsec(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SwLock59::NSEC_MASK) | ((value << SwLock59::NSEC_OFFSET) & SwLock59::NSEC_MASK)
        }
    }

    /// Set SEC [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub const fn with_sec(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SwLock59::SEC_MASK) | ((value << SwLock59::SEC_OFFSET) & SwLock59::SEC_MASK)
        }
    }

}

impl Default for SwLock59Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SwLock59 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> SwLock59Val {
        SwLock59Val::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: SwLock59Val) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(SwLock59Val) -> SwLock59Val>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// SW_LOCK60 Register
///
/// Software lock register for page 60. Locks are initialised from the OTP lock
/// pages at reset. This register can be written to further advance the lock
/// state of each page (until next reset), and read to check the current lock
/// state of a page.
///
/// Address offset: 0x00f0
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **NSEC** [3:2] (RW): Non-secure lock status. Writes are OR'd with the current value.
/// - **SEC** [1:0] (RW): Secure lock status. Writes are OR'd with the current value. This field is r...
#[repr(transparent)]
pub struct SwLock60 {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for SwLock60 {}
unsafe impl Sync for SwLock60 {}

impl SwLock60 {
    // ======== Field Constants ========

    /// NSEC - Non-secure lock status. Writes are OR'd with the current value.
    /// Bits [3:2], mask 0x0000000c
    pub const NSEC_OFFSET: u32 = 2;
    pub const NSEC_MASK: u32 = 0x0000000c;
    pub const NSEC_READ_WRITE: u32 = 0 << 2;
    pub const NSEC_READ_ONLY: u32 = 1 << 2;
    pub const NSEC_INACCESSIBLE: u32 = 3 << 2;

    /// SEC - Secure lock status. Writes are OR'd with the current value. This field is r...
    /// Bits [1:0], mask 0x00000003
    pub const SEC_OFFSET: u32 = 0;
    pub const SEC_MASK: u32 = 0x00000003;
    pub const SEC_READ_WRITE: u32 = 0 << 0;
    pub const SEC_READ_ONLY: u32 = 1 << 0;
    pub const SEC_INACCESSIBLE: u32 = 3 << 0;

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

    /// **NSEC** [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub fn nsec(&self) -> u32 {
        (self.read_bits() & Self::NSEC_MASK) >> Self::NSEC_OFFSET
    }

    /// Write NSEC field
    #[inline(always)]
    pub fn write_nsec(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::NSEC_MASK) | ((value << Self::NSEC_OFFSET) & Self::NSEC_MASK));
    }

    /// **SEC** [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub fn sec(&self) -> u32 {
        (self.read_bits() & Self::SEC_MASK) >> Self::SEC_OFFSET
    }

    /// Write SEC field
    #[inline(always)]
    pub fn write_sec(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::SEC_MASK) | ((value << Self::SEC_OFFSET) & Self::SEC_MASK));
    }

}

/// SW_LOCK60 value type for batch read/modify/write operations
///
/// Use `swlock60.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `swlock60.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SwLock60Val {
    bits: u32,
}

impl SwLock60Val {
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

    /// Read NSEC [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub const fn nsec(&self) -> u32 {
        (self.bits & SwLock60::NSEC_MASK) >> SwLock60::NSEC_OFFSET
    }

    /// Read SEC [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub const fn sec(&self) -> u32 {
        (self.bits & SwLock60::SEC_MASK) >> SwLock60::SEC_OFFSET
    }

    /// Set NSEC [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub const fn with_nsec(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SwLock60::NSEC_MASK) | ((value << SwLock60::NSEC_OFFSET) & SwLock60::NSEC_MASK)
        }
    }

    /// Set SEC [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub const fn with_sec(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SwLock60::SEC_MASK) | ((value << SwLock60::SEC_OFFSET) & SwLock60::SEC_MASK)
        }
    }

}

impl Default for SwLock60Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SwLock60 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> SwLock60Val {
        SwLock60Val::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: SwLock60Val) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(SwLock60Val) -> SwLock60Val>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// SW_LOCK61 Register
///
/// Software lock register for page 61. Locks are initialised from the OTP lock
/// pages at reset. This register can be written to further advance the lock
/// state of each page (until next reset), and read to check the current lock
/// state of a page.
///
/// Address offset: 0x00f4
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **NSEC** [3:2] (RW): Non-secure lock status. Writes are OR'd with the current value.
/// - **SEC** [1:0] (RW): Secure lock status. Writes are OR'd with the current value. This field is r...
#[repr(transparent)]
pub struct SwLock61 {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for SwLock61 {}
unsafe impl Sync for SwLock61 {}

impl SwLock61 {
    // ======== Field Constants ========

    /// NSEC - Non-secure lock status. Writes are OR'd with the current value.
    /// Bits [3:2], mask 0x0000000c
    pub const NSEC_OFFSET: u32 = 2;
    pub const NSEC_MASK: u32 = 0x0000000c;
    pub const NSEC_READ_WRITE: u32 = 0 << 2;
    pub const NSEC_READ_ONLY: u32 = 1 << 2;
    pub const NSEC_INACCESSIBLE: u32 = 3 << 2;

    /// SEC - Secure lock status. Writes are OR'd with the current value. This field is r...
    /// Bits [1:0], mask 0x00000003
    pub const SEC_OFFSET: u32 = 0;
    pub const SEC_MASK: u32 = 0x00000003;
    pub const SEC_READ_WRITE: u32 = 0 << 0;
    pub const SEC_READ_ONLY: u32 = 1 << 0;
    pub const SEC_INACCESSIBLE: u32 = 3 << 0;

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

    /// **NSEC** [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub fn nsec(&self) -> u32 {
        (self.read_bits() & Self::NSEC_MASK) >> Self::NSEC_OFFSET
    }

    /// Write NSEC field
    #[inline(always)]
    pub fn write_nsec(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::NSEC_MASK) | ((value << Self::NSEC_OFFSET) & Self::NSEC_MASK));
    }

    /// **SEC** [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub fn sec(&self) -> u32 {
        (self.read_bits() & Self::SEC_MASK) >> Self::SEC_OFFSET
    }

    /// Write SEC field
    #[inline(always)]
    pub fn write_sec(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::SEC_MASK) | ((value << Self::SEC_OFFSET) & Self::SEC_MASK));
    }

}

/// SW_LOCK61 value type for batch read/modify/write operations
///
/// Use `swlock61.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `swlock61.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SwLock61Val {
    bits: u32,
}

impl SwLock61Val {
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

    /// Read NSEC [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub const fn nsec(&self) -> u32 {
        (self.bits & SwLock61::NSEC_MASK) >> SwLock61::NSEC_OFFSET
    }

    /// Read SEC [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub const fn sec(&self) -> u32 {
        (self.bits & SwLock61::SEC_MASK) >> SwLock61::SEC_OFFSET
    }

    /// Set NSEC [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub const fn with_nsec(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SwLock61::NSEC_MASK) | ((value << SwLock61::NSEC_OFFSET) & SwLock61::NSEC_MASK)
        }
    }

    /// Set SEC [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub const fn with_sec(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SwLock61::SEC_MASK) | ((value << SwLock61::SEC_OFFSET) & SwLock61::SEC_MASK)
        }
    }

}

impl Default for SwLock61Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SwLock61 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> SwLock61Val {
        SwLock61Val::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: SwLock61Val) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(SwLock61Val) -> SwLock61Val>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// SW_LOCK62 Register
///
/// Software lock register for page 62. Locks are initialised from the OTP lock
/// pages at reset. This register can be written to further advance the lock
/// state of each page (until next reset), and read to check the current lock
/// state of a page.
///
/// Address offset: 0x00f8
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **NSEC** [3:2] (RW): Non-secure lock status. Writes are OR'd with the current value.
/// - **SEC** [1:0] (RW): Secure lock status. Writes are OR'd with the current value. This field is r...
#[repr(transparent)]
pub struct SwLock62 {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for SwLock62 {}
unsafe impl Sync for SwLock62 {}

impl SwLock62 {
    // ======== Field Constants ========

    /// NSEC - Non-secure lock status. Writes are OR'd with the current value.
    /// Bits [3:2], mask 0x0000000c
    pub const NSEC_OFFSET: u32 = 2;
    pub const NSEC_MASK: u32 = 0x0000000c;
    pub const NSEC_READ_WRITE: u32 = 0 << 2;
    pub const NSEC_READ_ONLY: u32 = 1 << 2;
    pub const NSEC_INACCESSIBLE: u32 = 3 << 2;

    /// SEC - Secure lock status. Writes are OR'd with the current value. This field is r...
    /// Bits [1:0], mask 0x00000003
    pub const SEC_OFFSET: u32 = 0;
    pub const SEC_MASK: u32 = 0x00000003;
    pub const SEC_READ_WRITE: u32 = 0 << 0;
    pub const SEC_READ_ONLY: u32 = 1 << 0;
    pub const SEC_INACCESSIBLE: u32 = 3 << 0;

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

    /// **NSEC** [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub fn nsec(&self) -> u32 {
        (self.read_bits() & Self::NSEC_MASK) >> Self::NSEC_OFFSET
    }

    /// Write NSEC field
    #[inline(always)]
    pub fn write_nsec(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::NSEC_MASK) | ((value << Self::NSEC_OFFSET) & Self::NSEC_MASK));
    }

    /// **SEC** [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub fn sec(&self) -> u32 {
        (self.read_bits() & Self::SEC_MASK) >> Self::SEC_OFFSET
    }

    /// Write SEC field
    #[inline(always)]
    pub fn write_sec(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::SEC_MASK) | ((value << Self::SEC_OFFSET) & Self::SEC_MASK));
    }

}

/// SW_LOCK62 value type for batch read/modify/write operations
///
/// Use `swlock62.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `swlock62.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SwLock62Val {
    bits: u32,
}

impl SwLock62Val {
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

    /// Read NSEC [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub const fn nsec(&self) -> u32 {
        (self.bits & SwLock62::NSEC_MASK) >> SwLock62::NSEC_OFFSET
    }

    /// Read SEC [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub const fn sec(&self) -> u32 {
        (self.bits & SwLock62::SEC_MASK) >> SwLock62::SEC_OFFSET
    }

    /// Set NSEC [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub const fn with_nsec(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SwLock62::NSEC_MASK) | ((value << SwLock62::NSEC_OFFSET) & SwLock62::NSEC_MASK)
        }
    }

    /// Set SEC [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub const fn with_sec(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SwLock62::SEC_MASK) | ((value << SwLock62::SEC_OFFSET) & SwLock62::SEC_MASK)
        }
    }

}

impl Default for SwLock62Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SwLock62 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> SwLock62Val {
        SwLock62Val::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: SwLock62Val) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(SwLock62Val) -> SwLock62Val>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// SW_LOCK63 Register
///
/// Software lock register for page 63. Locks are initialised from the OTP lock
/// pages at reset. This register can be written to further advance the lock
/// state of each page (until next reset), and read to check the current lock
/// state of a page.
///
/// Address offset: 0x00fc
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **NSEC** [3:2] (RW): Non-secure lock status. Writes are OR'd with the current value.
/// - **SEC** [1:0] (RW): Secure lock status. Writes are OR'd with the current value. This field is r...
#[repr(transparent)]
pub struct SwLock63 {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for SwLock63 {}
unsafe impl Sync for SwLock63 {}

impl SwLock63 {
    // ======== Field Constants ========

    /// NSEC - Non-secure lock status. Writes are OR'd with the current value.
    /// Bits [3:2], mask 0x0000000c
    pub const NSEC_OFFSET: u32 = 2;
    pub const NSEC_MASK: u32 = 0x0000000c;
    pub const NSEC_READ_WRITE: u32 = 0 << 2;
    pub const NSEC_READ_ONLY: u32 = 1 << 2;
    pub const NSEC_INACCESSIBLE: u32 = 3 << 2;

    /// SEC - Secure lock status. Writes are OR'd with the current value. This field is r...
    /// Bits [1:0], mask 0x00000003
    pub const SEC_OFFSET: u32 = 0;
    pub const SEC_MASK: u32 = 0x00000003;
    pub const SEC_READ_WRITE: u32 = 0 << 0;
    pub const SEC_READ_ONLY: u32 = 1 << 0;
    pub const SEC_INACCESSIBLE: u32 = 3 << 0;

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

    /// **NSEC** [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub fn nsec(&self) -> u32 {
        (self.read_bits() & Self::NSEC_MASK) >> Self::NSEC_OFFSET
    }

    /// Write NSEC field
    #[inline(always)]
    pub fn write_nsec(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::NSEC_MASK) | ((value << Self::NSEC_OFFSET) & Self::NSEC_MASK));
    }

    /// **SEC** [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub fn sec(&self) -> u32 {
        (self.read_bits() & Self::SEC_MASK) >> Self::SEC_OFFSET
    }

    /// Write SEC field
    #[inline(always)]
    pub fn write_sec(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::SEC_MASK) | ((value << Self::SEC_OFFSET) & Self::SEC_MASK));
    }

}

/// SW_LOCK63 value type for batch read/modify/write operations
///
/// Use `swlock63.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `swlock63.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SwLock63Val {
    bits: u32,
}

impl SwLock63Val {
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

    /// Read NSEC [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub const fn nsec(&self) -> u32 {
        (self.bits & SwLock63::NSEC_MASK) >> SwLock63::NSEC_OFFSET
    }

    /// Read SEC [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub const fn sec(&self) -> u32 {
        (self.bits & SwLock63::SEC_MASK) >> SwLock63::SEC_OFFSET
    }

    /// Set NSEC [3:2]: Non-secure lock status. Writes are OR'd with the current value.
    #[inline(always)]
    pub const fn with_nsec(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SwLock63::NSEC_MASK) | ((value << SwLock63::NSEC_OFFSET) & SwLock63::NSEC_MASK)
        }
    }

    /// Set SEC [1:0]: Secure lock status. Writes are OR'd with the current value. This field is r...
    #[inline(always)]
    pub const fn with_sec(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SwLock63::SEC_MASK) | ((value << SwLock63::SEC_OFFSET) & SwLock63::SEC_MASK)
        }
    }

}

impl Default for SwLock63Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SwLock63 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> SwLock63Val {
        SwLock63Val::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: SwLock63Val) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(SwLock63Val) -> SwLock63Val>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// SBPI_INSTR Register
///
/// Dispatch instructions to the SBPI interface, used for programming the OTP
/// fuses.
///
/// Address offset: 0x0100
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **EXEC** [30] (WO): Execute instruction
/// - **IS_WR** [29] (RW): Payload type is write
/// - **HAS_PAYLOAD** [28] (RW): Instruction has payload (data to be written or to be read)
/// - **PAYLOAD_SIZE_M1** [27:24] (RW): Instruction payload size in bytes minus 1
/// - **TARGET** [23:16] (RW): Instruction target, it can be PMC (0x3a) or DAP (0x02)
/// - **CMD** [15:8] (RW): 
/// - **SHORT_WDATA** [7:0] (RW): wdata to be used only when payload_size_m1=0
#[repr(transparent)]
pub struct SbpiInstr {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for SbpiInstr {}
unsafe impl Sync for SbpiInstr {}

impl SbpiInstr {
    // ======== Field Constants ========

    /// EXEC - Execute instruction
    /// Bits [30:30], mask 0x40000000
    pub const EXEC_OFFSET: u32 = 30;
    pub const EXEC_MASK: u32 = 0x40000000;
    pub const EXEC: u32 = 1 << 30;

    /// IS_WR - Payload type is write
    /// Bits [29:29], mask 0x20000000
    pub const IS_WR_OFFSET: u32 = 29;
    pub const IS_WR_MASK: u32 = 0x20000000;
    pub const IS_WR: u32 = 1 << 29;

    /// HAS_PAYLOAD - Instruction has payload (data to be written or to be read)
    /// Bits [28:28], mask 0x10000000
    pub const HAS_PAYLOAD_OFFSET: u32 = 28;
    pub const HAS_PAYLOAD_MASK: u32 = 0x10000000;
    pub const HAS_PAYLOAD: u32 = 1 << 28;

    /// PAYLOAD_SIZE_M1 - Instruction payload size in bytes minus 1
    /// Bits [27:24], mask 0x0f000000
    pub const PAYLOAD_SIZE_M1_OFFSET: u32 = 24;
    pub const PAYLOAD_SIZE_M1_MASK: u32 = 0x0f000000;

    /// TARGET - Instruction target, it can be PMC (0x3a) or DAP (0x02)
    /// Bits [23:16], mask 0x00ff0000
    pub const TARGET_OFFSET: u32 = 16;
    pub const TARGET_MASK: u32 = 0x00ff0000;

    /// Bits [15:8], mask 0x0000ff00
    pub const CMD_OFFSET: u32 = 8;
    pub const CMD_MASK: u32 = 0x0000ff00;

    /// SHORT_WDATA - wdata to be used only when payload_size_m1=0
    /// Bits [7:0], mask 0x000000ff
    pub const SHORT_WDATA_OFFSET: u32 = 0;
    pub const SHORT_WDATA_MASK: u32 = 0x000000ff;

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

    /// **EXEC** [30:30]: Execute instruction
    /// Trigger EXEC (write-only)
    #[inline(always)]
    pub fn trigger_exec(&self) {
        self.write_bits(Self::EXEC_MASK);
    }

    /// **IS_WR** [29:29]: Payload type is write
    #[inline(always)]
    pub fn is_wr(&self) -> bool {
        (self.read_bits() & Self::IS_WR_MASK) != 0
    }

    /// Set IS_WR bit
    #[inline(always)]
    pub fn set_is_wr(&self) {
        self.set_bits(Self::IS_WR_MASK);
    }

    /// Clear IS_WR bit
    #[inline(always)]
    pub fn clr_is_wr(&self) {
        self.clr_bits(Self::IS_WR_MASK);
    }

    /// **HAS_PAYLOAD** [28:28]: Instruction has payload (data to be written or to be read)
    #[inline(always)]
    pub fn has_payload(&self) -> bool {
        (self.read_bits() & Self::HAS_PAYLOAD_MASK) != 0
    }

    /// Set HAS_PAYLOAD bit
    #[inline(always)]
    pub fn set_has_payload(&self) {
        self.set_bits(Self::HAS_PAYLOAD_MASK);
    }

    /// Clear HAS_PAYLOAD bit
    #[inline(always)]
    pub fn clr_has_payload(&self) {
        self.clr_bits(Self::HAS_PAYLOAD_MASK);
    }

    /// **PAYLOAD_SIZE_M1** [27:24]: Instruction payload size in bytes minus 1
    #[inline(always)]
    pub fn payload_size_m1(&self) -> u32 {
        (self.read_bits() & Self::PAYLOAD_SIZE_M1_MASK) >> Self::PAYLOAD_SIZE_M1_OFFSET
    }

    /// Write PAYLOAD_SIZE_M1 field
    #[inline(always)]
    pub fn write_payload_size_m1(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::PAYLOAD_SIZE_M1_MASK) | ((value << Self::PAYLOAD_SIZE_M1_OFFSET) & Self::PAYLOAD_SIZE_M1_MASK));
    }

    /// **TARGET** [23:16]: Instruction target, it can be PMC (0x3a) or DAP (0x02)
    #[inline(always)]
    pub fn target(&self) -> u32 {
        (self.read_bits() & Self::TARGET_MASK) >> Self::TARGET_OFFSET
    }

    /// Write TARGET field
    #[inline(always)]
    pub fn write_target(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::TARGET_MASK) | ((value << Self::TARGET_OFFSET) & Self::TARGET_MASK));
    }

    /// **CMD** [15:8]
    #[inline(always)]
    pub fn cmd(&self) -> u32 {
        (self.read_bits() & Self::CMD_MASK) >> Self::CMD_OFFSET
    }

    /// Write CMD field
    #[inline(always)]
    pub fn write_cmd(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::CMD_MASK) | ((value << Self::CMD_OFFSET) & Self::CMD_MASK));
    }

    /// **SHORT_WDATA** [7:0]: wdata to be used only when payload_size_m1=0
    #[inline(always)]
    pub fn short_wdata(&self) -> u32 {
        (self.read_bits() & Self::SHORT_WDATA_MASK) >> Self::SHORT_WDATA_OFFSET
    }

    /// Write SHORT_WDATA field
    #[inline(always)]
    pub fn write_short_wdata(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::SHORT_WDATA_MASK) | ((value << Self::SHORT_WDATA_OFFSET) & Self::SHORT_WDATA_MASK));
    }

}

/// SBPI_INSTR value type for batch read/modify/write operations
///
/// Use `sbpiinstr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `sbpiinstr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SbpiInstrVal {
    bits: u32,
}

impl SbpiInstrVal {
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

    /// Read IS_WR [29:29]: Payload type is write
    #[inline(always)]
    pub const fn is_wr(&self) -> bool {
        (self.bits & SbpiInstr::IS_WR_MASK) != 0
    }

    /// Read HAS_PAYLOAD [28:28]: Instruction has payload (data to be written or to be read)
    #[inline(always)]
    pub const fn has_payload(&self) -> bool {
        (self.bits & SbpiInstr::HAS_PAYLOAD_MASK) != 0
    }

    /// Read PAYLOAD_SIZE_M1 [27:24]: Instruction payload size in bytes minus 1
    #[inline(always)]
    pub const fn payload_size_m1(&self) -> u32 {
        (self.bits & SbpiInstr::PAYLOAD_SIZE_M1_MASK) >> SbpiInstr::PAYLOAD_SIZE_M1_OFFSET
    }

    /// Read TARGET [23:16]: Instruction target, it can be PMC (0x3a) or DAP (0x02)
    #[inline(always)]
    pub const fn target(&self) -> u32 {
        (self.bits & SbpiInstr::TARGET_MASK) >> SbpiInstr::TARGET_OFFSET
    }

    /// Read CMD [15:8]
    #[inline(always)]
    pub const fn cmd(&self) -> u32 {
        (self.bits & SbpiInstr::CMD_MASK) >> SbpiInstr::CMD_OFFSET
    }

    /// Read SHORT_WDATA [7:0]: wdata to be used only when payload_size_m1=0
    #[inline(always)]
    pub const fn short_wdata(&self) -> u32 {
        (self.bits & SbpiInstr::SHORT_WDATA_MASK) >> SbpiInstr::SHORT_WDATA_OFFSET
    }

    /// Set EXEC [30:30]: Execute instruction
    #[inline(always)]
    pub const fn with_exec(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SbpiInstr::EXEC_MASK
            } else {
                self.bits & !SbpiInstr::EXEC_MASK
            }
        }
    }

    /// Set IS_WR [29:29]: Payload type is write
    #[inline(always)]
    pub const fn with_is_wr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SbpiInstr::IS_WR_MASK
            } else {
                self.bits & !SbpiInstr::IS_WR_MASK
            }
        }
    }

    /// Set HAS_PAYLOAD [28:28]: Instruction has payload (data to be written or to be read)
    #[inline(always)]
    pub const fn with_has_payload(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SbpiInstr::HAS_PAYLOAD_MASK
            } else {
                self.bits & !SbpiInstr::HAS_PAYLOAD_MASK
            }
        }
    }

    /// Set PAYLOAD_SIZE_M1 [27:24]: Instruction payload size in bytes minus 1
    #[inline(always)]
    pub const fn with_payload_size_m1(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SbpiInstr::PAYLOAD_SIZE_M1_MASK) | ((value << SbpiInstr::PAYLOAD_SIZE_M1_OFFSET) & SbpiInstr::PAYLOAD_SIZE_M1_MASK)
        }
    }

    /// Set TARGET [23:16]: Instruction target, it can be PMC (0x3a) or DAP (0x02)
    #[inline(always)]
    pub const fn with_target(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SbpiInstr::TARGET_MASK) | ((value << SbpiInstr::TARGET_OFFSET) & SbpiInstr::TARGET_MASK)
        }
    }

    /// Set CMD [15:8]
    #[inline(always)]
    pub const fn with_cmd(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SbpiInstr::CMD_MASK) | ((value << SbpiInstr::CMD_OFFSET) & SbpiInstr::CMD_MASK)
        }
    }

    /// Set SHORT_WDATA [7:0]: wdata to be used only when payload_size_m1=0
    #[inline(always)]
    pub const fn with_short_wdata(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SbpiInstr::SHORT_WDATA_MASK) | ((value << SbpiInstr::SHORT_WDATA_OFFSET) & SbpiInstr::SHORT_WDATA_MASK)
        }
    }

}

impl Default for SbpiInstrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SbpiInstr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> SbpiInstrVal {
        SbpiInstrVal::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: SbpiInstrVal) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(SbpiInstrVal) -> SbpiInstrVal>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// SBPI_WDATA_0 Register
///
/// SBPI write payload bytes 3..0
///
/// Address offset: 0x0104
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **SBPI_WDATA_0** [31:0] (RW): 
#[repr(transparent)]
pub struct SbpiWdata0 {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for SbpiWdata0 {}
unsafe impl Sync for SbpiWdata0 {}

impl SbpiWdata0 {
    // ======== Field Constants ========

    /// Bits [31:0], mask 0xffffffff
    pub const SBPI_WDATA_0_OFFSET: u32 = 0;
    pub const SBPI_WDATA_0_MASK: u32 = 0xffffffff;

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

    /// **SBPI_WDATA_0** [31:0]
    #[inline(always)]
    pub fn sbpi_wdata_0(&self) -> u32 {
        (self.read_bits() & Self::SBPI_WDATA_0_MASK) >> Self::SBPI_WDATA_0_OFFSET
    }

    /// Write SBPI_WDATA_0 field
    #[inline(always)]
    pub fn write_sbpi_wdata_0(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::SBPI_WDATA_0_MASK) | ((value << Self::SBPI_WDATA_0_OFFSET) & Self::SBPI_WDATA_0_MASK));
    }

}

/// SBPI_WDATA_0 value type for batch read/modify/write operations
///
/// Use `sbpiwdata0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `sbpiwdata0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SbpiWdata0Val {
    bits: u32,
}

impl SbpiWdata0Val {
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

    /// Read SBPI_WDATA_0 [31:0]
    #[inline(always)]
    pub const fn sbpi_wdata_0(&self) -> u32 {
        (self.bits & SbpiWdata0::SBPI_WDATA_0_MASK) >> SbpiWdata0::SBPI_WDATA_0_OFFSET
    }

    /// Set SBPI_WDATA_0 [31:0]
    #[inline(always)]
    pub const fn with_sbpi_wdata_0(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SbpiWdata0::SBPI_WDATA_0_MASK) | ((value << SbpiWdata0::SBPI_WDATA_0_OFFSET) & SbpiWdata0::SBPI_WDATA_0_MASK)
        }
    }

}

impl Default for SbpiWdata0Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SbpiWdata0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> SbpiWdata0Val {
        SbpiWdata0Val::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: SbpiWdata0Val) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(SbpiWdata0Val) -> SbpiWdata0Val>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// SBPI_WDATA_1 Register
///
/// SBPI write payload bytes 7..4
///
/// Address offset: 0x0108
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **SBPI_WDATA_1** [31:0] (RW): 
#[repr(transparent)]
pub struct SbpiWdata1 {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for SbpiWdata1 {}
unsafe impl Sync for SbpiWdata1 {}

impl SbpiWdata1 {
    // ======== Field Constants ========

    /// Bits [31:0], mask 0xffffffff
    pub const SBPI_WDATA_1_OFFSET: u32 = 0;
    pub const SBPI_WDATA_1_MASK: u32 = 0xffffffff;

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

    /// **SBPI_WDATA_1** [31:0]
    #[inline(always)]
    pub fn sbpi_wdata_1(&self) -> u32 {
        (self.read_bits() & Self::SBPI_WDATA_1_MASK) >> Self::SBPI_WDATA_1_OFFSET
    }

    /// Write SBPI_WDATA_1 field
    #[inline(always)]
    pub fn write_sbpi_wdata_1(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::SBPI_WDATA_1_MASK) | ((value << Self::SBPI_WDATA_1_OFFSET) & Self::SBPI_WDATA_1_MASK));
    }

}

/// SBPI_WDATA_1 value type for batch read/modify/write operations
///
/// Use `sbpiwdata1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `sbpiwdata1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SbpiWdata1Val {
    bits: u32,
}

impl SbpiWdata1Val {
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

    /// Read SBPI_WDATA_1 [31:0]
    #[inline(always)]
    pub const fn sbpi_wdata_1(&self) -> u32 {
        (self.bits & SbpiWdata1::SBPI_WDATA_1_MASK) >> SbpiWdata1::SBPI_WDATA_1_OFFSET
    }

    /// Set SBPI_WDATA_1 [31:0]
    #[inline(always)]
    pub const fn with_sbpi_wdata_1(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SbpiWdata1::SBPI_WDATA_1_MASK) | ((value << SbpiWdata1::SBPI_WDATA_1_OFFSET) & SbpiWdata1::SBPI_WDATA_1_MASK)
        }
    }

}

impl Default for SbpiWdata1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SbpiWdata1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> SbpiWdata1Val {
        SbpiWdata1Val::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: SbpiWdata1Val) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(SbpiWdata1Val) -> SbpiWdata1Val>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// SBPI_WDATA_2 Register
///
/// SBPI write payload bytes 11..8
///
/// Address offset: 0x010c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **SBPI_WDATA_2** [31:0] (RW): 
#[repr(transparent)]
pub struct SbpiWdata2 {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for SbpiWdata2 {}
unsafe impl Sync for SbpiWdata2 {}

impl SbpiWdata2 {
    // ======== Field Constants ========

    /// Bits [31:0], mask 0xffffffff
    pub const SBPI_WDATA_2_OFFSET: u32 = 0;
    pub const SBPI_WDATA_2_MASK: u32 = 0xffffffff;

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

    /// **SBPI_WDATA_2** [31:0]
    #[inline(always)]
    pub fn sbpi_wdata_2(&self) -> u32 {
        (self.read_bits() & Self::SBPI_WDATA_2_MASK) >> Self::SBPI_WDATA_2_OFFSET
    }

    /// Write SBPI_WDATA_2 field
    #[inline(always)]
    pub fn write_sbpi_wdata_2(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::SBPI_WDATA_2_MASK) | ((value << Self::SBPI_WDATA_2_OFFSET) & Self::SBPI_WDATA_2_MASK));
    }

}

/// SBPI_WDATA_2 value type for batch read/modify/write operations
///
/// Use `sbpiwdata2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `sbpiwdata2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SbpiWdata2Val {
    bits: u32,
}

impl SbpiWdata2Val {
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

    /// Read SBPI_WDATA_2 [31:0]
    #[inline(always)]
    pub const fn sbpi_wdata_2(&self) -> u32 {
        (self.bits & SbpiWdata2::SBPI_WDATA_2_MASK) >> SbpiWdata2::SBPI_WDATA_2_OFFSET
    }

    /// Set SBPI_WDATA_2 [31:0]
    #[inline(always)]
    pub const fn with_sbpi_wdata_2(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SbpiWdata2::SBPI_WDATA_2_MASK) | ((value << SbpiWdata2::SBPI_WDATA_2_OFFSET) & SbpiWdata2::SBPI_WDATA_2_MASK)
        }
    }

}

impl Default for SbpiWdata2Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SbpiWdata2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> SbpiWdata2Val {
        SbpiWdata2Val::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: SbpiWdata2Val) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(SbpiWdata2Val) -> SbpiWdata2Val>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// SBPI_WDATA_3 Register
///
/// SBPI write payload bytes 15..12
///
/// Address offset: 0x0110
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **SBPI_WDATA_3** [31:0] (RW): 
#[repr(transparent)]
pub struct SbpiWdata3 {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for SbpiWdata3 {}
unsafe impl Sync for SbpiWdata3 {}

impl SbpiWdata3 {
    // ======== Field Constants ========

    /// Bits [31:0], mask 0xffffffff
    pub const SBPI_WDATA_3_OFFSET: u32 = 0;
    pub const SBPI_WDATA_3_MASK: u32 = 0xffffffff;

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

    /// **SBPI_WDATA_3** [31:0]
    #[inline(always)]
    pub fn sbpi_wdata_3(&self) -> u32 {
        (self.read_bits() & Self::SBPI_WDATA_3_MASK) >> Self::SBPI_WDATA_3_OFFSET
    }

    /// Write SBPI_WDATA_3 field
    #[inline(always)]
    pub fn write_sbpi_wdata_3(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::SBPI_WDATA_3_MASK) | ((value << Self::SBPI_WDATA_3_OFFSET) & Self::SBPI_WDATA_3_MASK));
    }

}

/// SBPI_WDATA_3 value type for batch read/modify/write operations
///
/// Use `sbpiwdata3.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `sbpiwdata3.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SbpiWdata3Val {
    bits: u32,
}

impl SbpiWdata3Val {
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

    /// Read SBPI_WDATA_3 [31:0]
    #[inline(always)]
    pub const fn sbpi_wdata_3(&self) -> u32 {
        (self.bits & SbpiWdata3::SBPI_WDATA_3_MASK) >> SbpiWdata3::SBPI_WDATA_3_OFFSET
    }

    /// Set SBPI_WDATA_3 [31:0]
    #[inline(always)]
    pub const fn with_sbpi_wdata_3(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SbpiWdata3::SBPI_WDATA_3_MASK) | ((value << SbpiWdata3::SBPI_WDATA_3_OFFSET) & SbpiWdata3::SBPI_WDATA_3_MASK)
        }
    }

}

impl Default for SbpiWdata3Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SbpiWdata3 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> SbpiWdata3Val {
        SbpiWdata3Val::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: SbpiWdata3Val) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(SbpiWdata3Val) -> SbpiWdata3Val>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// SBPI_RDATA_0 Register
///
/// Read payload bytes 3..0. Once read, the data in the register will
/// automatically clear to 0.
///
/// Address offset: 0x0114
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **SBPI_RDATA_0** [31:0] (RO): 
#[repr(transparent)]
pub struct SbpiRdata0 {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for SbpiRdata0 {}
unsafe impl Sync for SbpiRdata0 {}

impl SbpiRdata0 {
    // ======== Field Constants ========

    /// Bits [31:0], mask 0xffffffff
    pub const SBPI_RDATA_0_OFFSET: u32 = 0;
    pub const SBPI_RDATA_0_MASK: u32 = 0xffffffff;

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

    /// **SBPI_RDATA_0** [31:0]
    #[inline(always)]
    pub fn sbpi_rdata_0(&self) -> u32 {
        (self.read_bits() & Self::SBPI_RDATA_0_MASK) >> Self::SBPI_RDATA_0_OFFSET
    }

}

/// SBPI_RDATA_0 value type for batch read/modify/write operations
///
/// Use `sbpirdata0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `sbpirdata0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SbpiRdata0Val {
    bits: u32,
}

impl SbpiRdata0Val {
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

    /// Read SBPI_RDATA_0 [31:0]
    #[inline(always)]
    pub const fn sbpi_rdata_0(&self) -> u32 {
        (self.bits & SbpiRdata0::SBPI_RDATA_0_MASK) >> SbpiRdata0::SBPI_RDATA_0_OFFSET
    }

}

impl Default for SbpiRdata0Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SbpiRdata0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> SbpiRdata0Val {
        SbpiRdata0Val::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: SbpiRdata0Val) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(SbpiRdata0Val) -> SbpiRdata0Val>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// SBPI_RDATA_1 Register
///
/// Read payload bytes 7..4. Once read, the data in the register will
/// automatically clear to 0.
///
/// Address offset: 0x0118
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **SBPI_RDATA_1** [31:0] (RO): 
#[repr(transparent)]
pub struct SbpiRdata1 {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for SbpiRdata1 {}
unsafe impl Sync for SbpiRdata1 {}

impl SbpiRdata1 {
    // ======== Field Constants ========

    /// Bits [31:0], mask 0xffffffff
    pub const SBPI_RDATA_1_OFFSET: u32 = 0;
    pub const SBPI_RDATA_1_MASK: u32 = 0xffffffff;

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

    /// **SBPI_RDATA_1** [31:0]
    #[inline(always)]
    pub fn sbpi_rdata_1(&self) -> u32 {
        (self.read_bits() & Self::SBPI_RDATA_1_MASK) >> Self::SBPI_RDATA_1_OFFSET
    }

}

/// SBPI_RDATA_1 value type for batch read/modify/write operations
///
/// Use `sbpirdata1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `sbpirdata1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SbpiRdata1Val {
    bits: u32,
}

impl SbpiRdata1Val {
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

    /// Read SBPI_RDATA_1 [31:0]
    #[inline(always)]
    pub const fn sbpi_rdata_1(&self) -> u32 {
        (self.bits & SbpiRdata1::SBPI_RDATA_1_MASK) >> SbpiRdata1::SBPI_RDATA_1_OFFSET
    }

}

impl Default for SbpiRdata1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SbpiRdata1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> SbpiRdata1Val {
        SbpiRdata1Val::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: SbpiRdata1Val) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(SbpiRdata1Val) -> SbpiRdata1Val>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// SBPI_RDATA_2 Register
///
/// Read payload bytes 11..8. Once read, the data in the register will
/// automatically clear to 0.
///
/// Address offset: 0x011c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **SBPI_RDATA_2** [31:0] (RO): 
#[repr(transparent)]
pub struct SbpiRdata2 {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for SbpiRdata2 {}
unsafe impl Sync for SbpiRdata2 {}

impl SbpiRdata2 {
    // ======== Field Constants ========

    /// Bits [31:0], mask 0xffffffff
    pub const SBPI_RDATA_2_OFFSET: u32 = 0;
    pub const SBPI_RDATA_2_MASK: u32 = 0xffffffff;

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

    /// **SBPI_RDATA_2** [31:0]
    #[inline(always)]
    pub fn sbpi_rdata_2(&self) -> u32 {
        (self.read_bits() & Self::SBPI_RDATA_2_MASK) >> Self::SBPI_RDATA_2_OFFSET
    }

}

/// SBPI_RDATA_2 value type for batch read/modify/write operations
///
/// Use `sbpirdata2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `sbpirdata2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SbpiRdata2Val {
    bits: u32,
}

impl SbpiRdata2Val {
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

    /// Read SBPI_RDATA_2 [31:0]
    #[inline(always)]
    pub const fn sbpi_rdata_2(&self) -> u32 {
        (self.bits & SbpiRdata2::SBPI_RDATA_2_MASK) >> SbpiRdata2::SBPI_RDATA_2_OFFSET
    }

}

impl Default for SbpiRdata2Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SbpiRdata2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> SbpiRdata2Val {
        SbpiRdata2Val::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: SbpiRdata2Val) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(SbpiRdata2Val) -> SbpiRdata2Val>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// SBPI_RDATA_3 Register
///
/// Read payload bytes 15..12. Once read, the data in the register will
/// automatically clear to 0.
///
/// Address offset: 0x0120
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **SBPI_RDATA_3** [31:0] (RO): 
#[repr(transparent)]
pub struct SbpiRdata3 {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for SbpiRdata3 {}
unsafe impl Sync for SbpiRdata3 {}

impl SbpiRdata3 {
    // ======== Field Constants ========

    /// Bits [31:0], mask 0xffffffff
    pub const SBPI_RDATA_3_OFFSET: u32 = 0;
    pub const SBPI_RDATA_3_MASK: u32 = 0xffffffff;

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

    /// **SBPI_RDATA_3** [31:0]
    #[inline(always)]
    pub fn sbpi_rdata_3(&self) -> u32 {
        (self.read_bits() & Self::SBPI_RDATA_3_MASK) >> Self::SBPI_RDATA_3_OFFSET
    }

}

/// SBPI_RDATA_3 value type for batch read/modify/write operations
///
/// Use `sbpirdata3.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `sbpirdata3.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SbpiRdata3Val {
    bits: u32,
}

impl SbpiRdata3Val {
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

    /// Read SBPI_RDATA_3 [31:0]
    #[inline(always)]
    pub const fn sbpi_rdata_3(&self) -> u32 {
        (self.bits & SbpiRdata3::SBPI_RDATA_3_MASK) >> SbpiRdata3::SBPI_RDATA_3_OFFSET
    }

}

impl Default for SbpiRdata3Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SbpiRdata3 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> SbpiRdata3Val {
        SbpiRdata3Val::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: SbpiRdata3Val) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(SbpiRdata3Val) -> SbpiRdata3Val>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// SBPI_STATUS Register
///
/// Address offset: 0x0124
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **MISO** [23:16] (RO): SBPI MISO (master in - slave out): response from SBPI
/// - **FLAG** [12] (RO): SBPI flag
/// - **INSTR_MISS** [8] (RW): Last instruction missed (dropped), as the previous has not finished running
/// - **INSTR_DONE** [4] (RW): Last instruction done
/// - **RDATA_VLD** [0] (RW): Read command has returned data
#[repr(transparent)]
pub struct SbpiStatus {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for SbpiStatus {}
unsafe impl Sync for SbpiStatus {}

impl SbpiStatus {
    // ======== Field Constants ========

    /// MISO - SBPI MISO (master in - slave out): response from SBPI
    /// Bits [23:16], mask 0x00ff0000
    pub const MISO_OFFSET: u32 = 16;
    pub const MISO_MASK: u32 = 0x00ff0000;

    /// FLAG - SBPI flag
    /// Bits [12:12], mask 0x00001000
    pub const FLAG_OFFSET: u32 = 12;
    pub const FLAG_MASK: u32 = 0x00001000;
    pub const FLAG: u32 = 1 << 12;

    /// INSTR_MISS - Last instruction missed (dropped), as the previous has not finished running
    /// Bits [8:8], mask 0x00000100
    pub const INSTR_MISS_OFFSET: u32 = 8;
    pub const INSTR_MISS_MASK: u32 = 0x00000100;
    pub const INSTR_MISS: u32 = 1 << 8;

    /// INSTR_DONE - Last instruction done
    /// Bits [4:4], mask 0x00000010
    pub const INSTR_DONE_OFFSET: u32 = 4;
    pub const INSTR_DONE_MASK: u32 = 0x00000010;
    pub const INSTR_DONE: u32 = 1 << 4;

    /// RDATA_VLD - Read command has returned data
    /// Bits [0:0], mask 0x00000001
    pub const RDATA_VLD_OFFSET: u32 = 0;
    pub const RDATA_VLD_MASK: u32 = 0x00000001;
    pub const RDATA_VLD: u32 = 1 << 0;

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

    /// **MISO** [23:16]: SBPI MISO (master in - slave out): response from SBPI
    #[inline(always)]
    pub fn miso(&self) -> u32 {
        (self.read_bits() & Self::MISO_MASK) >> Self::MISO_OFFSET
    }

    /// **FLAG** [12:12]: SBPI flag
    #[inline(always)]
    pub fn flag(&self) -> bool {
        (self.read_bits() & Self::FLAG_MASK) != 0
    }

    /// **INSTR_MISS** [8:8]: Last instruction missed (dropped), as the previous has not finished running
    #[inline(always)]
    pub fn instr_miss(&self) -> bool {
        (self.read_bits() & Self::INSTR_MISS_MASK) != 0
    }

    /// Set INSTR_MISS bit
    #[inline(always)]
    pub fn set_instr_miss(&self) {
        self.set_bits(Self::INSTR_MISS_MASK);
    }

    /// Clear INSTR_MISS bit
    #[inline(always)]
    pub fn clr_instr_miss(&self) {
        self.clr_bits(Self::INSTR_MISS_MASK);
    }

    /// **INSTR_DONE** [4:4]: Last instruction done
    #[inline(always)]
    pub fn instr_done(&self) -> bool {
        (self.read_bits() & Self::INSTR_DONE_MASK) != 0
    }

    /// Set INSTR_DONE bit
    #[inline(always)]
    pub fn set_instr_done(&self) {
        self.set_bits(Self::INSTR_DONE_MASK);
    }

    /// Clear INSTR_DONE bit
    #[inline(always)]
    pub fn clr_instr_done(&self) {
        self.clr_bits(Self::INSTR_DONE_MASK);
    }

    /// **RDATA_VLD** [0:0]: Read command has returned data
    #[inline(always)]
    pub fn rdata_vld(&self) -> bool {
        (self.read_bits() & Self::RDATA_VLD_MASK) != 0
    }

    /// Set RDATA_VLD bit
    #[inline(always)]
    pub fn set_rdata_vld(&self) {
        self.set_bits(Self::RDATA_VLD_MASK);
    }

    /// Clear RDATA_VLD bit
    #[inline(always)]
    pub fn clr_rdata_vld(&self) {
        self.clr_bits(Self::RDATA_VLD_MASK);
    }

}

/// SBPI_STATUS value type for batch read/modify/write operations
///
/// Use `sbpistatus.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `sbpistatus.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SbpiStatusVal {
    bits: u32,
}

impl SbpiStatusVal {
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

    /// Read MISO [23:16]: SBPI MISO (master in - slave out): response from SBPI
    #[inline(always)]
    pub const fn miso(&self) -> u32 {
        (self.bits & SbpiStatus::MISO_MASK) >> SbpiStatus::MISO_OFFSET
    }

    /// Read FLAG [12:12]: SBPI flag
    #[inline(always)]
    pub const fn flag(&self) -> bool {
        (self.bits & SbpiStatus::FLAG_MASK) != 0
    }

    /// Read INSTR_MISS [8:8]: Last instruction missed (dropped), as the previous has not finished running
    #[inline(always)]
    pub const fn instr_miss(&self) -> bool {
        (self.bits & SbpiStatus::INSTR_MISS_MASK) != 0
    }

    /// Read INSTR_DONE [4:4]: Last instruction done
    #[inline(always)]
    pub const fn instr_done(&self) -> bool {
        (self.bits & SbpiStatus::INSTR_DONE_MASK) != 0
    }

    /// Read RDATA_VLD [0:0]: Read command has returned data
    #[inline(always)]
    pub const fn rdata_vld(&self) -> bool {
        (self.bits & SbpiStatus::RDATA_VLD_MASK) != 0
    }

    /// Set INSTR_MISS [8:8]: Last instruction missed (dropped), as the previous has not finished running
    #[inline(always)]
    pub const fn with_instr_miss(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SbpiStatus::INSTR_MISS_MASK
            } else {
                self.bits & !SbpiStatus::INSTR_MISS_MASK
            }
        }
    }

    /// Set INSTR_DONE [4:4]: Last instruction done
    #[inline(always)]
    pub const fn with_instr_done(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SbpiStatus::INSTR_DONE_MASK
            } else {
                self.bits & !SbpiStatus::INSTR_DONE_MASK
            }
        }
    }

    /// Set RDATA_VLD [0:0]: Read command has returned data
    #[inline(always)]
    pub const fn with_rdata_vld(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SbpiStatus::RDATA_VLD_MASK
            } else {
                self.bits & !SbpiStatus::RDATA_VLD_MASK
            }
        }
    }

}

impl Default for SbpiStatusVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SbpiStatus {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> SbpiStatusVal {
        SbpiStatusVal::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: SbpiStatusVal) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(SbpiStatusVal) -> SbpiStatusVal>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// USR Register
///
/// Controls for APB data read interface (USER interface)
///
/// Address offset: 0x0128
/// Reset value: 0x00000001
///
/// ## Fields
///
/// - **PD** [4] (RW): Power-down; 1 disables current reference. Must be 0 to read data from the OTP.
/// - **DCTRL** [0] (RW): 1 enables USER interface; 0 disables USER interface (enables SBPI). This bi...
#[repr(transparent)]
pub struct Usr {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for Usr {}
unsafe impl Sync for Usr {}

impl Usr {
    // ======== Field Constants ========

    /// PD - Power-down; 1 disables current reference. Must be 0 to read data from the OTP.
    /// Bits [4:4], mask 0x00000010
    pub const PD_OFFSET: u32 = 4;
    pub const PD_MASK: u32 = 0x00000010;
    pub const PD: u32 = 1 << 4;

    /// DCTRL - 1 enables USER interface; 0 disables USER interface (enables SBPI). This bi...
    /// Bits [0:0], mask 0x00000001
    pub const DCTRL_OFFSET: u32 = 0;
    pub const DCTRL_MASK: u32 = 0x00000001;
    pub const DCTRL: u32 = 1 << 0;

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

    /// **PD** [4:4]: Power-down; 1 disables current reference. Must be 0 to read data from the OTP.
    #[inline(always)]
    pub fn pd(&self) -> bool {
        (self.read_bits() & Self::PD_MASK) != 0
    }

    /// Set PD bit
    #[inline(always)]
    pub fn set_pd(&self) {
        self.set_bits(Self::PD_MASK);
    }

    /// Clear PD bit
    #[inline(always)]
    pub fn clr_pd(&self) {
        self.clr_bits(Self::PD_MASK);
    }

    /// **DCTRL** [0:0]: 1 enables USER interface; 0 disables USER interface (enables SBPI). This bi...
    #[inline(always)]
    pub fn dctrl(&self) -> bool {
        (self.read_bits() & Self::DCTRL_MASK) != 0
    }

    /// Set DCTRL bit
    #[inline(always)]
    pub fn set_dctrl(&self) {
        self.set_bits(Self::DCTRL_MASK);
    }

    /// Clear DCTRL bit
    #[inline(always)]
    pub fn clr_dctrl(&self) {
        self.clr_bits(Self::DCTRL_MASK);
    }

}

/// USR value type for batch read/modify/write operations
///
/// Use `usr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `usr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct UsrVal {
    bits: u32,
}

impl UsrVal {
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
        Self { bits: 0x00000001 }
    }

    /// Read PD [4:4]: Power-down; 1 disables current reference. Must be 0 to read data from the OTP.
    #[inline(always)]
    pub const fn pd(&self) -> bool {
        (self.bits & Usr::PD_MASK) != 0
    }

    /// Read DCTRL [0:0]: 1 enables USER interface; 0 disables USER interface (enables SBPI). This bi...
    #[inline(always)]
    pub const fn dctrl(&self) -> bool {
        (self.bits & Usr::DCTRL_MASK) != 0
    }

    /// Set PD [4:4]: Power-down; 1 disables current reference. Must be 0 to read data from the OTP.
    #[inline(always)]
    pub const fn with_pd(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Usr::PD_MASK
            } else {
                self.bits & !Usr::PD_MASK
            }
        }
    }

    /// Set DCTRL [0:0]: 1 enables USER interface; 0 disables USER interface (enables SBPI). This bi...
    #[inline(always)]
    pub const fn with_dctrl(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Usr::DCTRL_MASK
            } else {
                self.bits & !Usr::DCTRL_MASK
            }
        }
    }

}

impl Default for UsrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Usr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> UsrVal {
        UsrVal::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: UsrVal) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(UsrVal) -> UsrVal>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// DBG Register
///
/// Debug for OTP power-on state machine
///
/// Address offset: 0x012c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **CUSTOMER_RMA_FLAG** [12] (RO): The chip is in RMA mode
/// - **PSM_STATE** [7:4] (RO): Monitor the PSM FSM's state
/// - **ROSC_UP** [3] (RO): Ring oscillator is up and running
/// - **ROSC_UP_SEEN** [2] (RW): Ring oscillator was seen up and running
/// - **BOOT_DONE** [1] (RO): PSM boot done status flag
/// - **PSM_DONE** [0] (RO): PSM done status flag
#[repr(transparent)]
pub struct Dbg {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for Dbg {}
unsafe impl Sync for Dbg {}

impl Dbg {
    // ======== Field Constants ========

    /// CUSTOMER_RMA_FLAG - The chip is in RMA mode
    /// Bits [12:12], mask 0x00001000
    pub const CUSTOMER_RMA_FLAG_OFFSET: u32 = 12;
    pub const CUSTOMER_RMA_FLAG_MASK: u32 = 0x00001000;
    pub const CUSTOMER_RMA_FLAG: u32 = 1 << 12;

    /// PSM_STATE - Monitor the PSM FSM's state
    /// Bits [7:4], mask 0x000000f0
    pub const PSM_STATE_OFFSET: u32 = 4;
    pub const PSM_STATE_MASK: u32 = 0x000000f0;

    /// ROSC_UP - Ring oscillator is up and running
    /// Bits [3:3], mask 0x00000008
    pub const ROSC_UP_OFFSET: u32 = 3;
    pub const ROSC_UP_MASK: u32 = 0x00000008;
    pub const ROSC_UP: u32 = 1 << 3;

    /// ROSC_UP_SEEN - Ring oscillator was seen up and running
    /// Bits [2:2], mask 0x00000004
    pub const ROSC_UP_SEEN_OFFSET: u32 = 2;
    pub const ROSC_UP_SEEN_MASK: u32 = 0x00000004;
    pub const ROSC_UP_SEEN: u32 = 1 << 2;

    /// BOOT_DONE - PSM boot done status flag
    /// Bits [1:1], mask 0x00000002
    pub const BOOT_DONE_OFFSET: u32 = 1;
    pub const BOOT_DONE_MASK: u32 = 0x00000002;
    pub const BOOT_DONE: u32 = 1 << 1;

    /// PSM_DONE - PSM done status flag
    /// Bits [0:0], mask 0x00000001
    pub const PSM_DONE_OFFSET: u32 = 0;
    pub const PSM_DONE_MASK: u32 = 0x00000001;
    pub const PSM_DONE: u32 = 1 << 0;

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

    /// **CUSTOMER_RMA_FLAG** [12:12]: The chip is in RMA mode
    #[inline(always)]
    pub fn customer_rma_flag(&self) -> bool {
        (self.read_bits() & Self::CUSTOMER_RMA_FLAG_MASK) != 0
    }

    /// **PSM_STATE** [7:4]: Monitor the PSM FSM's state
    #[inline(always)]
    pub fn psm_state(&self) -> u32 {
        (self.read_bits() & Self::PSM_STATE_MASK) >> Self::PSM_STATE_OFFSET
    }

    /// **ROSC_UP** [3:3]: Ring oscillator is up and running
    #[inline(always)]
    pub fn rosc_up(&self) -> bool {
        (self.read_bits() & Self::ROSC_UP_MASK) != 0
    }

    /// **ROSC_UP_SEEN** [2:2]: Ring oscillator was seen up and running
    #[inline(always)]
    pub fn rosc_up_seen(&self) -> bool {
        (self.read_bits() & Self::ROSC_UP_SEEN_MASK) != 0
    }

    /// Set ROSC_UP_SEEN bit
    #[inline(always)]
    pub fn set_rosc_up_seen(&self) {
        self.set_bits(Self::ROSC_UP_SEEN_MASK);
    }

    /// Clear ROSC_UP_SEEN bit
    #[inline(always)]
    pub fn clr_rosc_up_seen(&self) {
        self.clr_bits(Self::ROSC_UP_SEEN_MASK);
    }

    /// **BOOT_DONE** [1:1]: PSM boot done status flag
    #[inline(always)]
    pub fn boot_done(&self) -> bool {
        (self.read_bits() & Self::BOOT_DONE_MASK) != 0
    }

    /// **PSM_DONE** [0:0]: PSM done status flag
    #[inline(always)]
    pub fn psm_done(&self) -> bool {
        (self.read_bits() & Self::PSM_DONE_MASK) != 0
    }

}

/// DBG value type for batch read/modify/write operations
///
/// Use `dbg.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `dbg.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DbgVal {
    bits: u32,
}

impl DbgVal {
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

    /// Read CUSTOMER_RMA_FLAG [12:12]: The chip is in RMA mode
    #[inline(always)]
    pub const fn customer_rma_flag(&self) -> bool {
        (self.bits & Dbg::CUSTOMER_RMA_FLAG_MASK) != 0
    }

    /// Read PSM_STATE [7:4]: Monitor the PSM FSM's state
    #[inline(always)]
    pub const fn psm_state(&self) -> u32 {
        (self.bits & Dbg::PSM_STATE_MASK) >> Dbg::PSM_STATE_OFFSET
    }

    /// Read ROSC_UP [3:3]: Ring oscillator is up and running
    #[inline(always)]
    pub const fn rosc_up(&self) -> bool {
        (self.bits & Dbg::ROSC_UP_MASK) != 0
    }

    /// Read ROSC_UP_SEEN [2:2]: Ring oscillator was seen up and running
    #[inline(always)]
    pub const fn rosc_up_seen(&self) -> bool {
        (self.bits & Dbg::ROSC_UP_SEEN_MASK) != 0
    }

    /// Read BOOT_DONE [1:1]: PSM boot done status flag
    #[inline(always)]
    pub const fn boot_done(&self) -> bool {
        (self.bits & Dbg::BOOT_DONE_MASK) != 0
    }

    /// Read PSM_DONE [0:0]: PSM done status flag
    #[inline(always)]
    pub const fn psm_done(&self) -> bool {
        (self.bits & Dbg::PSM_DONE_MASK) != 0
    }

    /// Set ROSC_UP_SEEN [2:2]: Ring oscillator was seen up and running
    #[inline(always)]
    pub const fn with_rosc_up_seen(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Dbg::ROSC_UP_SEEN_MASK
            } else {
                self.bits & !Dbg::ROSC_UP_SEEN_MASK
            }
        }
    }

}

impl Default for DbgVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Dbg {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> DbgVal {
        DbgVal::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: DbgVal) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(DbgVal) -> DbgVal>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// BIST Register
///
/// During BIST, count address locations that have at least one leaky bit
///
/// Address offset: 0x0134
/// Reset value: 0x0fff0000
///
/// ## Fields
///
/// - **CNT_FAIL** [30] (RO): Flag if the count of address locations with at least one leaky bit exceeds ...
/// - **CNT_CLR** [29] (WO): Clear counter before use
/// - **CNT_ENA** [28] (RW): Enable the counter before the BIST function is initiated
/// - **CNT_MAX** [27:16] (RW): The cnt_fail flag will be set if the number of leaky locations exceeds this...
/// - **CNT** [12:0] (RO): Number of locations that have at least one leaky bit. Note: This count is t...
#[repr(transparent)]
pub struct Bist {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for Bist {}
unsafe impl Sync for Bist {}

impl Bist {
    // ======== Field Constants ========

    /// CNT_FAIL - Flag if the count of address locations with at least one leaky bit exceeds ...
    /// Bits [30:30], mask 0x40000000
    pub const CNT_FAIL_OFFSET: u32 = 30;
    pub const CNT_FAIL_MASK: u32 = 0x40000000;
    pub const CNT_FAIL: u32 = 1 << 30;

    /// CNT_CLR - Clear counter before use
    /// Bits [29:29], mask 0x20000000
    pub const CNT_CLR_OFFSET: u32 = 29;
    pub const CNT_CLR_MASK: u32 = 0x20000000;
    pub const CNT_CLR: u32 = 1 << 29;

    /// CNT_ENA - Enable the counter before the BIST function is initiated
    /// Bits [28:28], mask 0x10000000
    pub const CNT_ENA_OFFSET: u32 = 28;
    pub const CNT_ENA_MASK: u32 = 0x10000000;
    pub const CNT_ENA: u32 = 1 << 28;

    /// CNT_MAX - The cnt_fail flag will be set if the number of leaky locations exceeds this...
    /// Bits [27:16], mask 0x0fff0000
    pub const CNT_MAX_OFFSET: u32 = 16;
    pub const CNT_MAX_MASK: u32 = 0x0fff0000;

    /// CNT - Number of locations that have at least one leaky bit. Note: This count is t...
    /// Bits [12:0], mask 0x00001fff
    pub const CNT_OFFSET: u32 = 0;
    pub const CNT_MASK: u32 = 0x00001fff;

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

    /// **CNT_FAIL** [30:30]: Flag if the count of address locations with at least one leaky bit exceeds ...
    #[inline(always)]
    pub fn cnt_fail(&self) -> bool {
        (self.read_bits() & Self::CNT_FAIL_MASK) != 0
    }

    /// **CNT_CLR** [29:29]: Clear counter before use
    /// Trigger CNT_CLR (write-only)
    #[inline(always)]
    pub fn trigger_cnt_clr(&self) {
        self.write_bits(Self::CNT_CLR_MASK);
    }

    /// **CNT_ENA** [28:28]: Enable the counter before the BIST function is initiated
    #[inline(always)]
    pub fn cnt_ena(&self) -> bool {
        (self.read_bits() & Self::CNT_ENA_MASK) != 0
    }

    /// Set CNT_ENA bit
    #[inline(always)]
    pub fn set_cnt_ena(&self) {
        self.set_bits(Self::CNT_ENA_MASK);
    }

    /// Clear CNT_ENA bit
    #[inline(always)]
    pub fn clr_cnt_ena(&self) {
        self.clr_bits(Self::CNT_ENA_MASK);
    }

    /// **CNT_MAX** [27:16]: The cnt_fail flag will be set if the number of leaky locations exceeds this...
    #[inline(always)]
    pub fn cnt_max(&self) -> u32 {
        (self.read_bits() & Self::CNT_MAX_MASK) >> Self::CNT_MAX_OFFSET
    }

    /// Write CNT_MAX field
    #[inline(always)]
    pub fn write_cnt_max(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::CNT_MAX_MASK) | ((value << Self::CNT_MAX_OFFSET) & Self::CNT_MAX_MASK));
    }

    /// **CNT** [12:0]: Number of locations that have at least one leaky bit. Note: This count is t...
    #[inline(always)]
    pub fn cnt(&self) -> u32 {
        (self.read_bits() & Self::CNT_MASK) >> Self::CNT_OFFSET
    }

}

/// BIST value type for batch read/modify/write operations
///
/// Use `bist.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `bist.write_val(val)`.
#[derive(Clone, Copy)]
pub struct BistVal {
    bits: u32,
}

impl BistVal {
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
        Self { bits: 0x0fff0000 }
    }

    /// Read CNT_FAIL [30:30]: Flag if the count of address locations with at least one leaky bit exceeds ...
    #[inline(always)]
    pub const fn cnt_fail(&self) -> bool {
        (self.bits & Bist::CNT_FAIL_MASK) != 0
    }

    /// Read CNT_ENA [28:28]: Enable the counter before the BIST function is initiated
    #[inline(always)]
    pub const fn cnt_ena(&self) -> bool {
        (self.bits & Bist::CNT_ENA_MASK) != 0
    }

    /// Read CNT_MAX [27:16]: The cnt_fail flag will be set if the number of leaky locations exceeds this...
    #[inline(always)]
    pub const fn cnt_max(&self) -> u32 {
        (self.bits & Bist::CNT_MAX_MASK) >> Bist::CNT_MAX_OFFSET
    }

    /// Read CNT [12:0]: Number of locations that have at least one leaky bit. Note: This count is t...
    #[inline(always)]
    pub const fn cnt(&self) -> u32 {
        (self.bits & Bist::CNT_MASK) >> Bist::CNT_OFFSET
    }

    /// Set CNT_CLR [29:29]: Clear counter before use
    #[inline(always)]
    pub const fn with_cnt_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Bist::CNT_CLR_MASK
            } else {
                self.bits & !Bist::CNT_CLR_MASK
            }
        }
    }

    /// Set CNT_ENA [28:28]: Enable the counter before the BIST function is initiated
    #[inline(always)]
    pub const fn with_cnt_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Bist::CNT_ENA_MASK
            } else {
                self.bits & !Bist::CNT_ENA_MASK
            }
        }
    }

    /// Set CNT_MAX [27:16]: The cnt_fail flag will be set if the number of leaky locations exceeds this...
    #[inline(always)]
    pub const fn with_cnt_max(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Bist::CNT_MAX_MASK) | ((value << Bist::CNT_MAX_OFFSET) & Bist::CNT_MAX_MASK)
        }
    }

}

impl Default for BistVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Bist {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> BistVal {
        BistVal::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: BistVal) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(BistVal) -> BistVal>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// CRT_KEY_W0 Register
///
/// Word 0 (bits 31..0) of the key. Write only, read returns 0x0
///
/// Address offset: 0x0138
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **CRT_KEY_W0** [31:0] (WO): 
#[repr(transparent)]
pub struct CrtKeyW0 {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for CrtKeyW0 {}
unsafe impl Sync for CrtKeyW0 {}

impl CrtKeyW0 {
    // ======== Field Constants ========

    /// Bits [31:0], mask 0xffffffff
    pub const CRT_KEY_W0_OFFSET: u32 = 0;
    pub const CRT_KEY_W0_MASK: u32 = 0xffffffff;

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

    /// **CRT_KEY_W0** [31:0]
    /// Write CRT_KEY_W0 field (write-only)
    #[inline(always)]
    pub fn write_crt_key_w0(&self, value: u32) {
        self.write_bits((value << Self::CRT_KEY_W0_OFFSET) & Self::CRT_KEY_W0_MASK);
    }

}

/// CRT_KEY_W0 value type for batch read/modify/write operations
///
/// Use `crtkeyw0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `crtkeyw0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CrtKeyW0Val {
    bits: u32,
}

impl CrtKeyW0Val {
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

    /// Set CRT_KEY_W0 [31:0]
    #[inline(always)]
    pub const fn with_crt_key_w0(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !CrtKeyW0::CRT_KEY_W0_MASK) | ((value << CrtKeyW0::CRT_KEY_W0_OFFSET) & CrtKeyW0::CRT_KEY_W0_MASK)
        }
    }

}

impl Default for CrtKeyW0Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl CrtKeyW0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> CrtKeyW0Val {
        CrtKeyW0Val::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: CrtKeyW0Val) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(CrtKeyW0Val) -> CrtKeyW0Val>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// CRT_KEY_W1 Register
///
/// Word 1 (bits 63..32) of the key. Write only, read returns 0x0
///
/// Address offset: 0x013c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **CRT_KEY_W1** [31:0] (WO): 
#[repr(transparent)]
pub struct CrtKeyW1 {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for CrtKeyW1 {}
unsafe impl Sync for CrtKeyW1 {}

impl CrtKeyW1 {
    // ======== Field Constants ========

    /// Bits [31:0], mask 0xffffffff
    pub const CRT_KEY_W1_OFFSET: u32 = 0;
    pub const CRT_KEY_W1_MASK: u32 = 0xffffffff;

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

    /// **CRT_KEY_W1** [31:0]
    /// Write CRT_KEY_W1 field (write-only)
    #[inline(always)]
    pub fn write_crt_key_w1(&self, value: u32) {
        self.write_bits((value << Self::CRT_KEY_W1_OFFSET) & Self::CRT_KEY_W1_MASK);
    }

}

/// CRT_KEY_W1 value type for batch read/modify/write operations
///
/// Use `crtkeyw1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `crtkeyw1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CrtKeyW1Val {
    bits: u32,
}

impl CrtKeyW1Val {
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

    /// Set CRT_KEY_W1 [31:0]
    #[inline(always)]
    pub const fn with_crt_key_w1(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !CrtKeyW1::CRT_KEY_W1_MASK) | ((value << CrtKeyW1::CRT_KEY_W1_OFFSET) & CrtKeyW1::CRT_KEY_W1_MASK)
        }
    }

}

impl Default for CrtKeyW1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl CrtKeyW1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> CrtKeyW1Val {
        CrtKeyW1Val::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: CrtKeyW1Val) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(CrtKeyW1Val) -> CrtKeyW1Val>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// CRT_KEY_W2 Register
///
/// Word 2 (bits 95..64) of the key. Write only, read returns 0x0
///
/// Address offset: 0x0140
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **CRT_KEY_W2** [31:0] (WO): 
#[repr(transparent)]
pub struct CrtKeyW2 {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for CrtKeyW2 {}
unsafe impl Sync for CrtKeyW2 {}

impl CrtKeyW2 {
    // ======== Field Constants ========

    /// Bits [31:0], mask 0xffffffff
    pub const CRT_KEY_W2_OFFSET: u32 = 0;
    pub const CRT_KEY_W2_MASK: u32 = 0xffffffff;

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

    /// **CRT_KEY_W2** [31:0]
    /// Write CRT_KEY_W2 field (write-only)
    #[inline(always)]
    pub fn write_crt_key_w2(&self, value: u32) {
        self.write_bits((value << Self::CRT_KEY_W2_OFFSET) & Self::CRT_KEY_W2_MASK);
    }

}

/// CRT_KEY_W2 value type for batch read/modify/write operations
///
/// Use `crtkeyw2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `crtkeyw2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CrtKeyW2Val {
    bits: u32,
}

impl CrtKeyW2Val {
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

    /// Set CRT_KEY_W2 [31:0]
    #[inline(always)]
    pub const fn with_crt_key_w2(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !CrtKeyW2::CRT_KEY_W2_MASK) | ((value << CrtKeyW2::CRT_KEY_W2_OFFSET) & CrtKeyW2::CRT_KEY_W2_MASK)
        }
    }

}

impl Default for CrtKeyW2Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl CrtKeyW2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> CrtKeyW2Val {
        CrtKeyW2Val::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: CrtKeyW2Val) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(CrtKeyW2Val) -> CrtKeyW2Val>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// CRT_KEY_W3 Register
///
/// Word 3 (bits 127..96) of the key. Write only, read returns 0x0
///
/// Address offset: 0x0144
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **CRT_KEY_W3** [31:0] (WO): 
#[repr(transparent)]
pub struct CrtKeyW3 {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for CrtKeyW3 {}
unsafe impl Sync for CrtKeyW3 {}

impl CrtKeyW3 {
    // ======== Field Constants ========

    /// Bits [31:0], mask 0xffffffff
    pub const CRT_KEY_W3_OFFSET: u32 = 0;
    pub const CRT_KEY_W3_MASK: u32 = 0xffffffff;

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

    /// **CRT_KEY_W3** [31:0]
    /// Write CRT_KEY_W3 field (write-only)
    #[inline(always)]
    pub fn write_crt_key_w3(&self, value: u32) {
        self.write_bits((value << Self::CRT_KEY_W3_OFFSET) & Self::CRT_KEY_W3_MASK);
    }

}

/// CRT_KEY_W3 value type for batch read/modify/write operations
///
/// Use `crtkeyw3.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `crtkeyw3.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CrtKeyW3Val {
    bits: u32,
}

impl CrtKeyW3Val {
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

    /// Set CRT_KEY_W3 [31:0]
    #[inline(always)]
    pub const fn with_crt_key_w3(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !CrtKeyW3::CRT_KEY_W3_MASK) | ((value << CrtKeyW3::CRT_KEY_W3_OFFSET) & CrtKeyW3::CRT_KEY_W3_MASK)
        }
    }

}

impl Default for CrtKeyW3Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl CrtKeyW3 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> CrtKeyW3Val {
        CrtKeyW3Val::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: CrtKeyW3Val) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(CrtKeyW3Val) -> CrtKeyW3Val>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// CRITICAL Register
///
/// Quickly check values of critical flags read during boot up
///
/// Address offset: 0x0148
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **RISCV_DISABLE** [17] (RO): 
/// - **ARM_DISABLE** [16] (RO): 
/// - **GLITCH_DETECTOR_SENS** [6:5] (RO): 
/// - **GLITCH_DETECTOR_ENABLE** [4] (RO): 
/// - **DEFAULT_ARCHSEL** [3] (RO): 
/// - **DEBUG_DISABLE** [2] (RO): 
/// - **SECURE_DEBUG_DISABLE** [1] (RO): 
/// - **SECURE_BOOT_ENABLE** [0] (RO): 
#[repr(transparent)]
pub struct Critical {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for Critical {}
unsafe impl Sync for Critical {}

impl Critical {
    // ======== Field Constants ========

    /// Bits [17:17], mask 0x00020000
    pub const RISCV_DISABLE_OFFSET: u32 = 17;
    pub const RISCV_DISABLE_MASK: u32 = 0x00020000;
    pub const RISCV_DISABLE: u32 = 1 << 17;

    /// Bits [16:16], mask 0x00010000
    pub const ARM_DISABLE_OFFSET: u32 = 16;
    pub const ARM_DISABLE_MASK: u32 = 0x00010000;
    pub const ARM_DISABLE: u32 = 1 << 16;

    /// Bits [6:5], mask 0x00000060
    pub const GLITCH_DETECTOR_SENS_OFFSET: u32 = 5;
    pub const GLITCH_DETECTOR_SENS_MASK: u32 = 0x00000060;

    /// Bits [4:4], mask 0x00000010
    pub const GLITCH_DETECTOR_ENABLE_OFFSET: u32 = 4;
    pub const GLITCH_DETECTOR_ENABLE_MASK: u32 = 0x00000010;
    pub const GLITCH_DETECTOR_ENABLE: u32 = 1 << 4;

    /// Bits [3:3], mask 0x00000008
    pub const DEFAULT_ARCHSEL_OFFSET: u32 = 3;
    pub const DEFAULT_ARCHSEL_MASK: u32 = 0x00000008;
    pub const DEFAULT_ARCHSEL: u32 = 1 << 3;

    /// Bits [2:2], mask 0x00000004
    pub const DEBUG_DISABLE_OFFSET: u32 = 2;
    pub const DEBUG_DISABLE_MASK: u32 = 0x00000004;
    pub const DEBUG_DISABLE: u32 = 1 << 2;

    /// Bits [1:1], mask 0x00000002
    pub const SECURE_DEBUG_DISABLE_OFFSET: u32 = 1;
    pub const SECURE_DEBUG_DISABLE_MASK: u32 = 0x00000002;
    pub const SECURE_DEBUG_DISABLE: u32 = 1 << 1;

    /// Bits [0:0], mask 0x00000001
    pub const SECURE_BOOT_ENABLE_OFFSET: u32 = 0;
    pub const SECURE_BOOT_ENABLE_MASK: u32 = 0x00000001;
    pub const SECURE_BOOT_ENABLE: u32 = 1 << 0;

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

    /// **RISCV_DISABLE** [17:17]
    #[inline(always)]
    pub fn riscv_disable(&self) -> bool {
        (self.read_bits() & Self::RISCV_DISABLE_MASK) != 0
    }

    /// **ARM_DISABLE** [16:16]
    #[inline(always)]
    pub fn arm_disable(&self) -> bool {
        (self.read_bits() & Self::ARM_DISABLE_MASK) != 0
    }

    /// **GLITCH_DETECTOR_SENS** [6:5]
    #[inline(always)]
    pub fn glitch_detector_sens(&self) -> u32 {
        (self.read_bits() & Self::GLITCH_DETECTOR_SENS_MASK) >> Self::GLITCH_DETECTOR_SENS_OFFSET
    }

    /// **GLITCH_DETECTOR_ENABLE** [4:4]
    #[inline(always)]
    pub fn glitch_detector_enable(&self) -> bool {
        (self.read_bits() & Self::GLITCH_DETECTOR_ENABLE_MASK) != 0
    }

    /// **DEFAULT_ARCHSEL** [3:3]
    #[inline(always)]
    pub fn default_archsel(&self) -> bool {
        (self.read_bits() & Self::DEFAULT_ARCHSEL_MASK) != 0
    }

    /// **DEBUG_DISABLE** [2:2]
    #[inline(always)]
    pub fn debug_disable(&self) -> bool {
        (self.read_bits() & Self::DEBUG_DISABLE_MASK) != 0
    }

    /// **SECURE_DEBUG_DISABLE** [1:1]
    #[inline(always)]
    pub fn secure_debug_disable(&self) -> bool {
        (self.read_bits() & Self::SECURE_DEBUG_DISABLE_MASK) != 0
    }

    /// **SECURE_BOOT_ENABLE** [0:0]
    #[inline(always)]
    pub fn secure_boot_enable(&self) -> bool {
        (self.read_bits() & Self::SECURE_BOOT_ENABLE_MASK) != 0
    }

}

/// CRITICAL value type for batch read/modify/write operations
///
/// Use `critical.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `critical.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CriticalVal {
    bits: u32,
}

impl CriticalVal {
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

    /// Read RISCV_DISABLE [17:17]
    #[inline(always)]
    pub const fn riscv_disable(&self) -> bool {
        (self.bits & Critical::RISCV_DISABLE_MASK) != 0
    }

    /// Read ARM_DISABLE [16:16]
    #[inline(always)]
    pub const fn arm_disable(&self) -> bool {
        (self.bits & Critical::ARM_DISABLE_MASK) != 0
    }

    /// Read GLITCH_DETECTOR_SENS [6:5]
    #[inline(always)]
    pub const fn glitch_detector_sens(&self) -> u32 {
        (self.bits & Critical::GLITCH_DETECTOR_SENS_MASK) >> Critical::GLITCH_DETECTOR_SENS_OFFSET
    }

    /// Read GLITCH_DETECTOR_ENABLE [4:4]
    #[inline(always)]
    pub const fn glitch_detector_enable(&self) -> bool {
        (self.bits & Critical::GLITCH_DETECTOR_ENABLE_MASK) != 0
    }

    /// Read DEFAULT_ARCHSEL [3:3]
    #[inline(always)]
    pub const fn default_archsel(&self) -> bool {
        (self.bits & Critical::DEFAULT_ARCHSEL_MASK) != 0
    }

    /// Read DEBUG_DISABLE [2:2]
    #[inline(always)]
    pub const fn debug_disable(&self) -> bool {
        (self.bits & Critical::DEBUG_DISABLE_MASK) != 0
    }

    /// Read SECURE_DEBUG_DISABLE [1:1]
    #[inline(always)]
    pub const fn secure_debug_disable(&self) -> bool {
        (self.bits & Critical::SECURE_DEBUG_DISABLE_MASK) != 0
    }

    /// Read SECURE_BOOT_ENABLE [0:0]
    #[inline(always)]
    pub const fn secure_boot_enable(&self) -> bool {
        (self.bits & Critical::SECURE_BOOT_ENABLE_MASK) != 0
    }

}

impl Default for CriticalVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Critical {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> CriticalVal {
        CriticalVal::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: CriticalVal) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(CriticalVal) -> CriticalVal>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// KEY_VALID Register
///
/// Which keys were valid (enrolled) at boot time
///
/// Address offset: 0x014c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **KEY_VALID** [7:0] (RO): 
#[repr(transparent)]
pub struct KeyValid {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for KeyValid {}
unsafe impl Sync for KeyValid {}

impl KeyValid {
    // ======== Field Constants ========

    /// Bits [7:0], mask 0x000000ff
    pub const KEY_VALID_OFFSET: u32 = 0;
    pub const KEY_VALID_MASK: u32 = 0x000000ff;

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

    /// **KEY_VALID** [7:0]
    #[inline(always)]
    pub fn key_valid(&self) -> u32 {
        (self.read_bits() & Self::KEY_VALID_MASK) >> Self::KEY_VALID_OFFSET
    }

}

/// KEY_VALID value type for batch read/modify/write operations
///
/// Use `keyvalid.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `keyvalid.write_val(val)`.
#[derive(Clone, Copy)]
pub struct KeyValidVal {
    bits: u32,
}

impl KeyValidVal {
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

    /// Read KEY_VALID [7:0]
    #[inline(always)]
    pub const fn key_valid(&self) -> u32 {
        (self.bits & KeyValid::KEY_VALID_MASK) >> KeyValid::KEY_VALID_OFFSET
    }

}

impl Default for KeyValidVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl KeyValid {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> KeyValidVal {
        KeyValidVal::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: KeyValidVal) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(KeyValidVal) -> KeyValidVal>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// DEBUGEN Register
///
/// Enable a debug feature that has been disabled. Debug features are disabled
/// if one of the relevant critical boot flags is set in OTP (DEBUG_DISABLE or
/// SECURE_DEBUG_DISABLE), OR if a debug key is marked valid in OTP, and the
/// matching key value has not been supplied over SWD. Specifically: - The
/// DEBUG_DISABLE flag disables all debug features. This can be fully overridden
/// by setting all bits of this register. - The SECURE_DEBUG_DISABLE flag
/// disables secure processor debug. This can be fully overridden by setting the
/// PROC0_SECURE and PROC1_SECURE bits of this register. - If a single debug key
/// has been registered, and no matching key value has been supplied over SWD,
/// then all debug features are disabled. This can be fully overridden by
/// setting all bits of this register. - If both debug keys have been
/// registered, and the Non-secure key's value (key 6) has been supplied over
/// SWD, secure processor debug is disabled. This can be fully overridden by
/// setting the PROC0_SECURE and PROC1_SECURE bits of this register. - If both
/// debug keys have been registered, and the Secure key's value (key 5) has been
/// supplied over SWD, then no debug features are disabled by the key mechanism.
/// However, note that in this case debug features may still be disabled by the
/// critical boot flags.
///
/// Address offset: 0x0150
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **MISC** [8] (RW): Enable other debug components. Specifically, the CTI, and the APB-AP used t...
/// - **PROC1_SECURE** [3] (RW): Permit core 1's Mem-AP to generate Secure accesses, assuming it is enabled ...
/// - **PROC1** [2] (RW): Enable core 1's Mem-AP if it is currently disabled. The Mem-AP is disabled ...
/// - **PROC0_SECURE** [1] (RW): Permit core 0's Mem-AP to generate Secure accesses, assuming it is enabled ...
/// - **PROC0** [0] (RW): Enable core 0's Mem-AP if it is currently disabled. The Mem-AP is disabled ...
#[repr(transparent)]
pub struct Debugen {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for Debugen {}
unsafe impl Sync for Debugen {}

impl Debugen {
    // ======== Field Constants ========

    /// MISC - Enable other debug components. Specifically, the CTI, and the APB-AP used t...
    /// Bits [8:8], mask 0x00000100
    pub const MISC_OFFSET: u32 = 8;
    pub const MISC_MASK: u32 = 0x00000100;
    pub const MISC: u32 = 1 << 8;

    /// PROC1_SECURE - Permit core 1's Mem-AP to generate Secure accesses, assuming it is enabled ...
    /// Bits [3:3], mask 0x00000008
    pub const PROC1_SECURE_OFFSET: u32 = 3;
    pub const PROC1_SECURE_MASK: u32 = 0x00000008;
    pub const PROC1_SECURE: u32 = 1 << 3;

    /// PROC1 - Enable core 1's Mem-AP if it is currently disabled. The Mem-AP is disabled ...
    /// Bits [2:2], mask 0x00000004
    pub const PROC1_OFFSET: u32 = 2;
    pub const PROC1_MASK: u32 = 0x00000004;
    pub const PROC1: u32 = 1 << 2;

    /// PROC0_SECURE - Permit core 0's Mem-AP to generate Secure accesses, assuming it is enabled ...
    /// Bits [1:1], mask 0x00000002
    pub const PROC0_SECURE_OFFSET: u32 = 1;
    pub const PROC0_SECURE_MASK: u32 = 0x00000002;
    pub const PROC0_SECURE: u32 = 1 << 1;

    /// PROC0 - Enable core 0's Mem-AP if it is currently disabled. The Mem-AP is disabled ...
    /// Bits [0:0], mask 0x00000001
    pub const PROC0_OFFSET: u32 = 0;
    pub const PROC0_MASK: u32 = 0x00000001;
    pub const PROC0: u32 = 1 << 0;

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

    /// **MISC** [8:8]: Enable other debug components. Specifically, the CTI, and the APB-AP used t...
    #[inline(always)]
    pub fn misc(&self) -> bool {
        (self.read_bits() & Self::MISC_MASK) != 0
    }

    /// Set MISC bit
    #[inline(always)]
    pub fn set_misc(&self) {
        self.set_bits(Self::MISC_MASK);
    }

    /// Clear MISC bit
    #[inline(always)]
    pub fn clr_misc(&self) {
        self.clr_bits(Self::MISC_MASK);
    }

    /// **PROC1_SECURE** [3:3]: Permit core 1's Mem-AP to generate Secure accesses, assuming it is enabled ...
    #[inline(always)]
    pub fn proc1_secure(&self) -> bool {
        (self.read_bits() & Self::PROC1_SECURE_MASK) != 0
    }

    /// Set PROC1_SECURE bit
    #[inline(always)]
    pub fn set_proc1_secure(&self) {
        self.set_bits(Self::PROC1_SECURE_MASK);
    }

    /// Clear PROC1_SECURE bit
    #[inline(always)]
    pub fn clr_proc1_secure(&self) {
        self.clr_bits(Self::PROC1_SECURE_MASK);
    }

    /// **PROC1** [2:2]: Enable core 1's Mem-AP if it is currently disabled. The Mem-AP is disabled ...
    #[inline(always)]
    pub fn proc1(&self) -> bool {
        (self.read_bits() & Self::PROC1_MASK) != 0
    }

    /// Set PROC1 bit
    #[inline(always)]
    pub fn set_proc1(&self) {
        self.set_bits(Self::PROC1_MASK);
    }

    /// Clear PROC1 bit
    #[inline(always)]
    pub fn clr_proc1(&self) {
        self.clr_bits(Self::PROC1_MASK);
    }

    /// **PROC0_SECURE** [1:1]: Permit core 0's Mem-AP to generate Secure accesses, assuming it is enabled ...
    #[inline(always)]
    pub fn proc0_secure(&self) -> bool {
        (self.read_bits() & Self::PROC0_SECURE_MASK) != 0
    }

    /// Set PROC0_SECURE bit
    #[inline(always)]
    pub fn set_proc0_secure(&self) {
        self.set_bits(Self::PROC0_SECURE_MASK);
    }

    /// Clear PROC0_SECURE bit
    #[inline(always)]
    pub fn clr_proc0_secure(&self) {
        self.clr_bits(Self::PROC0_SECURE_MASK);
    }

    /// **PROC0** [0:0]: Enable core 0's Mem-AP if it is currently disabled. The Mem-AP is disabled ...
    #[inline(always)]
    pub fn proc0(&self) -> bool {
        (self.read_bits() & Self::PROC0_MASK) != 0
    }

    /// Set PROC0 bit
    #[inline(always)]
    pub fn set_proc0(&self) {
        self.set_bits(Self::PROC0_MASK);
    }

    /// Clear PROC0 bit
    #[inline(always)]
    pub fn clr_proc0(&self) {
        self.clr_bits(Self::PROC0_MASK);
    }

}

/// DEBUGEN value type for batch read/modify/write operations
///
/// Use `debugen.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `debugen.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DebugenVal {
    bits: u32,
}

impl DebugenVal {
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

    /// Read MISC [8:8]: Enable other debug components. Specifically, the CTI, and the APB-AP used t...
    #[inline(always)]
    pub const fn misc(&self) -> bool {
        (self.bits & Debugen::MISC_MASK) != 0
    }

    /// Read PROC1_SECURE [3:3]: Permit core 1's Mem-AP to generate Secure accesses, assuming it is enabled ...
    #[inline(always)]
    pub const fn proc1_secure(&self) -> bool {
        (self.bits & Debugen::PROC1_SECURE_MASK) != 0
    }

    /// Read PROC1 [2:2]: Enable core 1's Mem-AP if it is currently disabled. The Mem-AP is disabled ...
    #[inline(always)]
    pub const fn proc1(&self) -> bool {
        (self.bits & Debugen::PROC1_MASK) != 0
    }

    /// Read PROC0_SECURE [1:1]: Permit core 0's Mem-AP to generate Secure accesses, assuming it is enabled ...
    #[inline(always)]
    pub const fn proc0_secure(&self) -> bool {
        (self.bits & Debugen::PROC0_SECURE_MASK) != 0
    }

    /// Read PROC0 [0:0]: Enable core 0's Mem-AP if it is currently disabled. The Mem-AP is disabled ...
    #[inline(always)]
    pub const fn proc0(&self) -> bool {
        (self.bits & Debugen::PROC0_MASK) != 0
    }

    /// Set MISC [8:8]: Enable other debug components. Specifically, the CTI, and the APB-AP used t...
    #[inline(always)]
    pub const fn with_misc(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Debugen::MISC_MASK
            } else {
                self.bits & !Debugen::MISC_MASK
            }
        }
    }

    /// Set PROC1_SECURE [3:3]: Permit core 1's Mem-AP to generate Secure accesses, assuming it is enabled ...
    #[inline(always)]
    pub const fn with_proc1_secure(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Debugen::PROC1_SECURE_MASK
            } else {
                self.bits & !Debugen::PROC1_SECURE_MASK
            }
        }
    }

    /// Set PROC1 [2:2]: Enable core 1's Mem-AP if it is currently disabled. The Mem-AP is disabled ...
    #[inline(always)]
    pub const fn with_proc1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Debugen::PROC1_MASK
            } else {
                self.bits & !Debugen::PROC1_MASK
            }
        }
    }

    /// Set PROC0_SECURE [1:1]: Permit core 0's Mem-AP to generate Secure accesses, assuming it is enabled ...
    #[inline(always)]
    pub const fn with_proc0_secure(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Debugen::PROC0_SECURE_MASK
            } else {
                self.bits & !Debugen::PROC0_SECURE_MASK
            }
        }
    }

    /// Set PROC0 [0:0]: Enable core 0's Mem-AP if it is currently disabled. The Mem-AP is disabled ...
    #[inline(always)]
    pub const fn with_proc0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Debugen::PROC0_MASK
            } else {
                self.bits & !Debugen::PROC0_MASK
            }
        }
    }

}

impl Default for DebugenVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Debugen {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> DebugenVal {
        DebugenVal::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: DebugenVal) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(DebugenVal) -> DebugenVal>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// DEBUGEN_LOCK Register
///
/// Write 1s to lock corresponding bits in DEBUGEN. This register is reset by
/// the processor cold reset.
///
/// Address offset: 0x0154
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **MISC** [8] (RW): Write 1 to lock the MISC bit of DEBUGEN. Can't be cleared once set.
/// - **PROC1_SECURE** [3] (RW): Write 1 to lock the PROC1_SECURE bit of DEBUGEN. Can't be cleared once set.
/// - **PROC1** [2] (RW): Write 1 to lock the PROC1 bit of DEBUGEN. Can't be cleared once set.
/// - **PROC0_SECURE** [1] (RW): Write 1 to lock the PROC0_SECURE bit of DEBUGEN. Can't be cleared once set.
/// - **PROC0** [0] (RW): Write 1 to lock the PROC0 bit of DEBUGEN. Can't be cleared once set.
#[repr(transparent)]
pub struct DebugenLock {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for DebugenLock {}
unsafe impl Sync for DebugenLock {}

impl DebugenLock {
    // ======== Field Constants ========

    /// MISC - Write 1 to lock the MISC bit of DEBUGEN. Can't be cleared once set.
    /// Bits [8:8], mask 0x00000100
    pub const MISC_OFFSET: u32 = 8;
    pub const MISC_MASK: u32 = 0x00000100;
    pub const MISC: u32 = 1 << 8;

    /// PROC1_SECURE - Write 1 to lock the PROC1_SECURE bit of DEBUGEN. Can't be cleared once set.
    /// Bits [3:3], mask 0x00000008
    pub const PROC1_SECURE_OFFSET: u32 = 3;
    pub const PROC1_SECURE_MASK: u32 = 0x00000008;
    pub const PROC1_SECURE: u32 = 1 << 3;

    /// PROC1 - Write 1 to lock the PROC1 bit of DEBUGEN. Can't be cleared once set.
    /// Bits [2:2], mask 0x00000004
    pub const PROC1_OFFSET: u32 = 2;
    pub const PROC1_MASK: u32 = 0x00000004;
    pub const PROC1: u32 = 1 << 2;

    /// PROC0_SECURE - Write 1 to lock the PROC0_SECURE bit of DEBUGEN. Can't be cleared once set.
    /// Bits [1:1], mask 0x00000002
    pub const PROC0_SECURE_OFFSET: u32 = 1;
    pub const PROC0_SECURE_MASK: u32 = 0x00000002;
    pub const PROC0_SECURE: u32 = 1 << 1;

    /// PROC0 - Write 1 to lock the PROC0 bit of DEBUGEN. Can't be cleared once set.
    /// Bits [0:0], mask 0x00000001
    pub const PROC0_OFFSET: u32 = 0;
    pub const PROC0_MASK: u32 = 0x00000001;
    pub const PROC0: u32 = 1 << 0;

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

    /// **MISC** [8:8]: Write 1 to lock the MISC bit of DEBUGEN. Can't be cleared once set.
    #[inline(always)]
    pub fn misc(&self) -> bool {
        (self.read_bits() & Self::MISC_MASK) != 0
    }

    /// Set MISC bit
    #[inline(always)]
    pub fn set_misc(&self) {
        self.set_bits(Self::MISC_MASK);
    }

    /// Clear MISC bit
    #[inline(always)]
    pub fn clr_misc(&self) {
        self.clr_bits(Self::MISC_MASK);
    }

    /// **PROC1_SECURE** [3:3]: Write 1 to lock the PROC1_SECURE bit of DEBUGEN. Can't be cleared once set.
    #[inline(always)]
    pub fn proc1_secure(&self) -> bool {
        (self.read_bits() & Self::PROC1_SECURE_MASK) != 0
    }

    /// Set PROC1_SECURE bit
    #[inline(always)]
    pub fn set_proc1_secure(&self) {
        self.set_bits(Self::PROC1_SECURE_MASK);
    }

    /// Clear PROC1_SECURE bit
    #[inline(always)]
    pub fn clr_proc1_secure(&self) {
        self.clr_bits(Self::PROC1_SECURE_MASK);
    }

    /// **PROC1** [2:2]: Write 1 to lock the PROC1 bit of DEBUGEN. Can't be cleared once set.
    #[inline(always)]
    pub fn proc1(&self) -> bool {
        (self.read_bits() & Self::PROC1_MASK) != 0
    }

    /// Set PROC1 bit
    #[inline(always)]
    pub fn set_proc1(&self) {
        self.set_bits(Self::PROC1_MASK);
    }

    /// Clear PROC1 bit
    #[inline(always)]
    pub fn clr_proc1(&self) {
        self.clr_bits(Self::PROC1_MASK);
    }

    /// **PROC0_SECURE** [1:1]: Write 1 to lock the PROC0_SECURE bit of DEBUGEN. Can't be cleared once set.
    #[inline(always)]
    pub fn proc0_secure(&self) -> bool {
        (self.read_bits() & Self::PROC0_SECURE_MASK) != 0
    }

    /// Set PROC0_SECURE bit
    #[inline(always)]
    pub fn set_proc0_secure(&self) {
        self.set_bits(Self::PROC0_SECURE_MASK);
    }

    /// Clear PROC0_SECURE bit
    #[inline(always)]
    pub fn clr_proc0_secure(&self) {
        self.clr_bits(Self::PROC0_SECURE_MASK);
    }

    /// **PROC0** [0:0]: Write 1 to lock the PROC0 bit of DEBUGEN. Can't be cleared once set.
    #[inline(always)]
    pub fn proc0(&self) -> bool {
        (self.read_bits() & Self::PROC0_MASK) != 0
    }

    /// Set PROC0 bit
    #[inline(always)]
    pub fn set_proc0(&self) {
        self.set_bits(Self::PROC0_MASK);
    }

    /// Clear PROC0 bit
    #[inline(always)]
    pub fn clr_proc0(&self) {
        self.clr_bits(Self::PROC0_MASK);
    }

}

/// DEBUGEN_LOCK value type for batch read/modify/write operations
///
/// Use `debugenlock.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `debugenlock.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DebugenLockVal {
    bits: u32,
}

impl DebugenLockVal {
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

    /// Read MISC [8:8]: Write 1 to lock the MISC bit of DEBUGEN. Can't be cleared once set.
    #[inline(always)]
    pub const fn misc(&self) -> bool {
        (self.bits & DebugenLock::MISC_MASK) != 0
    }

    /// Read PROC1_SECURE [3:3]: Write 1 to lock the PROC1_SECURE bit of DEBUGEN. Can't be cleared once set.
    #[inline(always)]
    pub const fn proc1_secure(&self) -> bool {
        (self.bits & DebugenLock::PROC1_SECURE_MASK) != 0
    }

    /// Read PROC1 [2:2]: Write 1 to lock the PROC1 bit of DEBUGEN. Can't be cleared once set.
    #[inline(always)]
    pub const fn proc1(&self) -> bool {
        (self.bits & DebugenLock::PROC1_MASK) != 0
    }

    /// Read PROC0_SECURE [1:1]: Write 1 to lock the PROC0_SECURE bit of DEBUGEN. Can't be cleared once set.
    #[inline(always)]
    pub const fn proc0_secure(&self) -> bool {
        (self.bits & DebugenLock::PROC0_SECURE_MASK) != 0
    }

    /// Read PROC0 [0:0]: Write 1 to lock the PROC0 bit of DEBUGEN. Can't be cleared once set.
    #[inline(always)]
    pub const fn proc0(&self) -> bool {
        (self.bits & DebugenLock::PROC0_MASK) != 0
    }

    /// Set MISC [8:8]: Write 1 to lock the MISC bit of DEBUGEN. Can't be cleared once set.
    #[inline(always)]
    pub const fn with_misc(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DebugenLock::MISC_MASK
            } else {
                self.bits & !DebugenLock::MISC_MASK
            }
        }
    }

    /// Set PROC1_SECURE [3:3]: Write 1 to lock the PROC1_SECURE bit of DEBUGEN. Can't be cleared once set.
    #[inline(always)]
    pub const fn with_proc1_secure(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DebugenLock::PROC1_SECURE_MASK
            } else {
                self.bits & !DebugenLock::PROC1_SECURE_MASK
            }
        }
    }

    /// Set PROC1 [2:2]: Write 1 to lock the PROC1 bit of DEBUGEN. Can't be cleared once set.
    #[inline(always)]
    pub const fn with_proc1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DebugenLock::PROC1_MASK
            } else {
                self.bits & !DebugenLock::PROC1_MASK
            }
        }
    }

    /// Set PROC0_SECURE [1:1]: Write 1 to lock the PROC0_SECURE bit of DEBUGEN. Can't be cleared once set.
    #[inline(always)]
    pub const fn with_proc0_secure(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DebugenLock::PROC0_SECURE_MASK
            } else {
                self.bits & !DebugenLock::PROC0_SECURE_MASK
            }
        }
    }

    /// Set PROC0 [0:0]: Write 1 to lock the PROC0 bit of DEBUGEN. Can't be cleared once set.
    #[inline(always)]
    pub const fn with_proc0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DebugenLock::PROC0_MASK
            } else {
                self.bits & !DebugenLock::PROC0_MASK
            }
        }
    }

}

impl Default for DebugenLockVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl DebugenLock {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> DebugenLockVal {
        DebugenLockVal::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: DebugenLockVal) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(DebugenLockVal) -> DebugenLockVal>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// ARCHSEL Register
///
/// Architecture select (Arm/RISC-V). The default and allowable values of this
/// register are constrained by the critical boot flags. This register is reset
/// by the earliest reset in the switched core power domain (before a processor
/// cold reset). Cores sample their architecture select signal on a warm reset.
/// The source of the warm reset could be the system power-up state machine, the
/// watchdog timer, Arm SYSRESETREQ or from RISC-V hartresetreq. Note that when
/// an Arm core is deselected, its cold reset domain is also held in reset,
/// since in particular the SYSRESETREQ bit becomes inaccessible once the core
/// is deselected. Note also the RISC-V cores do not have a cold reset domain,
/// since their corresponding controls are located in the Debug Module.
///
/// Address offset: 0x0158
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **CORE1** [1] (RW): Select architecture for core 1.
/// - **CORE0** [0] (RW): Select architecture for core 0.
#[repr(transparent)]
pub struct Archsel {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for Archsel {}
unsafe impl Sync for Archsel {}

impl Archsel {
    // ======== Field Constants ========

    /// CORE1 - Select architecture for core 1.
    /// Bits [1:1], mask 0x00000002
    pub const CORE1_OFFSET: u32 = 1;
    pub const CORE1_MASK: u32 = 0x00000002;
    pub const CORE1: u32 = 1 << 1;
    /// arm = 0: Switch core 1 to Arm (Cortex-M33)
    pub const CORE1_ARM: u32 = 0 << 1;
    /// riscv = 1: Switch core 1 to RISC-V (Hazard3)
    pub const CORE1_RISCV: u32 = 1 << 1;

    /// CORE0 - Select architecture for core 0.
    /// Bits [0:0], mask 0x00000001
    pub const CORE0_OFFSET: u32 = 0;
    pub const CORE0_MASK: u32 = 0x00000001;
    pub const CORE0: u32 = 1 << 0;
    /// arm = 0: Switch core 0 to Arm (Cortex-M33)
    pub const CORE0_ARM: u32 = 0 << 0;
    /// riscv = 1: Switch core 0 to RISC-V (Hazard3)
    pub const CORE0_RISCV: u32 = 1 << 0;

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

    /// **CORE1** [1:1]: Select architecture for core 1.
    #[inline(always)]
    pub fn core1(&self) -> bool {
        (self.read_bits() & Self::CORE1_MASK) != 0
    }

    /// Set CORE1 bit
    #[inline(always)]
    pub fn set_core1(&self) {
        self.set_bits(Self::CORE1_MASK);
    }

    /// Clear CORE1 bit
    #[inline(always)]
    pub fn clr_core1(&self) {
        self.clr_bits(Self::CORE1_MASK);
    }

    /// **CORE0** [0:0]: Select architecture for core 0.
    #[inline(always)]
    pub fn core0(&self) -> bool {
        (self.read_bits() & Self::CORE0_MASK) != 0
    }

    /// Set CORE0 bit
    #[inline(always)]
    pub fn set_core0(&self) {
        self.set_bits(Self::CORE0_MASK);
    }

    /// Clear CORE0 bit
    #[inline(always)]
    pub fn clr_core0(&self) {
        self.clr_bits(Self::CORE0_MASK);
    }

}

/// ARCHSEL value type for batch read/modify/write operations
///
/// Use `archsel.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `archsel.write_val(val)`.
#[derive(Clone, Copy)]
pub struct ArchselVal {
    bits: u32,
}

impl ArchselVal {
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

    /// Read CORE1 [1:1]: Select architecture for core 1.
    #[inline(always)]
    pub const fn core1(&self) -> bool {
        (self.bits & Archsel::CORE1_MASK) != 0
    }

    /// Read CORE0 [0:0]: Select architecture for core 0.
    #[inline(always)]
    pub const fn core0(&self) -> bool {
        (self.bits & Archsel::CORE0_MASK) != 0
    }

    /// Set CORE1 [1:1]: Select architecture for core 1.
    #[inline(always)]
    pub const fn with_core1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Archsel::CORE1_MASK
            } else {
                self.bits & !Archsel::CORE1_MASK
            }
        }
    }

    /// Set CORE0 [0:0]: Select architecture for core 0.
    #[inline(always)]
    pub const fn with_core0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Archsel::CORE0_MASK
            } else {
                self.bits & !Archsel::CORE0_MASK
            }
        }
    }

}

impl Default for ArchselVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Archsel {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> ArchselVal {
        ArchselVal::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: ArchselVal) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(ArchselVal) -> ArchselVal>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// ARCHSEL_STATUS Register
///
/// Get the current architecture select state of each core. Cores sample the
/// current value of the ARCHSEL register when their warm reset is released, at
/// which point the corresponding bit in this register will also update.
///
/// Address offset: 0x015c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **CORE1** [1] (RO): Current architecture for core 0. Updated on processor warm reset.
/// - **CORE0** [0] (RO): Current architecture for core 0. Updated on processor warm reset.
#[repr(transparent)]
pub struct ArchselStatus {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for ArchselStatus {}
unsafe impl Sync for ArchselStatus {}

impl ArchselStatus {
    // ======== Field Constants ========

    /// CORE1 - Current architecture for core 0. Updated on processor warm reset.
    /// Bits [1:1], mask 0x00000002
    pub const CORE1_OFFSET: u32 = 1;
    pub const CORE1_MASK: u32 = 0x00000002;
    pub const CORE1: u32 = 1 << 1;
    /// arm = 0: Core 1 is currently Arm (Cortex-M33)
    pub const CORE1_ARM: u32 = 0 << 1;
    /// riscv = 1: Core 1 is currently RISC-V (Hazard3)
    pub const CORE1_RISCV: u32 = 1 << 1;

    /// CORE0 - Current architecture for core 0. Updated on processor warm reset.
    /// Bits [0:0], mask 0x00000001
    pub const CORE0_OFFSET: u32 = 0;
    pub const CORE0_MASK: u32 = 0x00000001;
    pub const CORE0: u32 = 1 << 0;
    /// arm = 0: Core 0 is currently Arm (Cortex-M33)
    pub const CORE0_ARM: u32 = 0 << 0;
    /// riscv = 1: Core 0 is currently RISC-V (Hazard3)
    pub const CORE0_RISCV: u32 = 1 << 0;

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

    /// **CORE1** [1:1]: Current architecture for core 0. Updated on processor warm reset.
    #[inline(always)]
    pub fn core1(&self) -> bool {
        (self.read_bits() & Self::CORE1_MASK) != 0
    }

    /// **CORE0** [0:0]: Current architecture for core 0. Updated on processor warm reset.
    #[inline(always)]
    pub fn core0(&self) -> bool {
        (self.read_bits() & Self::CORE0_MASK) != 0
    }

}

/// ARCHSEL_STATUS value type for batch read/modify/write operations
///
/// Use `archselstatus.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `archselstatus.write_val(val)`.
#[derive(Clone, Copy)]
pub struct ArchselStatusVal {
    bits: u32,
}

impl ArchselStatusVal {
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

    /// Read CORE1 [1:1]: Current architecture for core 0. Updated on processor warm reset.
    #[inline(always)]
    pub const fn core1(&self) -> bool {
        (self.bits & ArchselStatus::CORE1_MASK) != 0
    }

    /// Read CORE0 [0:0]: Current architecture for core 0. Updated on processor warm reset.
    #[inline(always)]
    pub const fn core0(&self) -> bool {
        (self.bits & ArchselStatus::CORE0_MASK) != 0
    }

}

impl Default for ArchselStatusVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl ArchselStatus {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> ArchselStatusVal {
        ArchselStatusVal::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: ArchselStatusVal) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(ArchselStatusVal) -> ArchselStatusVal>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// BOOTDIS Register
///
/// Tell the bootrom to ignore scratch register boot vectors (both power manager
/// and watchdog) on the next power up. If an early boot stage has soft-locked
/// some OTP pages in order to protect their contents from later stages, there
/// is a risk that Secure code running at a later stage can unlock the pages by
/// performing a watchdog reset that resets the OTP. This register can be used
/// to ensure that the bootloader runs as normal on the next power up,
/// preventing Secure code at a later stage from accessing OTP in its unlocked
/// state. Should be used in conjunction with the power manager BOOTDIS
/// register.
///
/// Address offset: 0x0160
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **NEXT** [1] (RW): This flag always ORs writes into its current contents. It can be set but no...
/// - **NOW** [0] (RW): When the core is powered down, the current value of BOOTDIS_NEXT is OR'd in...
#[repr(transparent)]
pub struct Bootdis {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for Bootdis {}
unsafe impl Sync for Bootdis {}

impl Bootdis {
    // ======== Field Constants ========

    /// NEXT - This flag always ORs writes into its current contents. It can be set but no...
    /// Bits [1:1], mask 0x00000002
    pub const NEXT_OFFSET: u32 = 1;
    pub const NEXT_MASK: u32 = 0x00000002;
    pub const NEXT: u32 = 1 << 1;

    /// NOW - When the core is powered down, the current value of BOOTDIS_NEXT is OR'd in...
    /// Bits [0:0], mask 0x00000001
    pub const NOW_OFFSET: u32 = 0;
    pub const NOW_MASK: u32 = 0x00000001;
    pub const NOW: u32 = 1 << 0;

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

    /// **NEXT** [1:1]: This flag always ORs writes into its current contents. It can be set but no...
    #[inline(always)]
    pub fn next(&self) -> bool {
        (self.read_bits() & Self::NEXT_MASK) != 0
    }

    /// Set NEXT bit
    #[inline(always)]
    pub fn set_next(&self) {
        self.set_bits(Self::NEXT_MASK);
    }

    /// Clear NEXT bit
    #[inline(always)]
    pub fn clr_next(&self) {
        self.clr_bits(Self::NEXT_MASK);
    }

    /// **NOW** [0:0]: When the core is powered down, the current value of BOOTDIS_NEXT is OR'd in...
    #[inline(always)]
    pub fn now(&self) -> bool {
        (self.read_bits() & Self::NOW_MASK) != 0
    }

    /// Set NOW bit
    #[inline(always)]
    pub fn set_now(&self) {
        self.set_bits(Self::NOW_MASK);
    }

    /// Clear NOW bit
    #[inline(always)]
    pub fn clr_now(&self) {
        self.clr_bits(Self::NOW_MASK);
    }

}

/// BOOTDIS value type for batch read/modify/write operations
///
/// Use `bootdis.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `bootdis.write_val(val)`.
#[derive(Clone, Copy)]
pub struct BootdisVal {
    bits: u32,
}

impl BootdisVal {
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

    /// Read NEXT [1:1]: This flag always ORs writes into its current contents. It can be set but no...
    #[inline(always)]
    pub const fn next(&self) -> bool {
        (self.bits & Bootdis::NEXT_MASK) != 0
    }

    /// Read NOW [0:0]: When the core is powered down, the current value of BOOTDIS_NEXT is OR'd in...
    #[inline(always)]
    pub const fn now(&self) -> bool {
        (self.bits & Bootdis::NOW_MASK) != 0
    }

    /// Set NEXT [1:1]: This flag always ORs writes into its current contents. It can be set but no...
    #[inline(always)]
    pub const fn with_next(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Bootdis::NEXT_MASK
            } else {
                self.bits & !Bootdis::NEXT_MASK
            }
        }
    }

    /// Set NOW [0:0]: When the core is powered down, the current value of BOOTDIS_NEXT is OR'd in...
    #[inline(always)]
    pub const fn with_now(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Bootdis::NOW_MASK
            } else {
                self.bits & !Bootdis::NOW_MASK
            }
        }
    }

}

impl Default for BootdisVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Bootdis {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> BootdisVal {
        BootdisVal::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: BootdisVal) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(BootdisVal) -> BootdisVal>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// INTR Register
///
/// Raw Interrupts
///
/// Address offset: 0x0164
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **APB_RD_NSEC_FAIL** [4] (RW): 
/// - **APB_RD_SEC_FAIL** [3] (RW): 
/// - **APB_DCTRL_FAIL** [2] (RW): 
/// - **SBPI_WR_FAIL** [1] (RW): 
/// - **SBPI_FLAG_N** [0] (RO): 
#[repr(transparent)]
pub struct Intr {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for Intr {}
unsafe impl Sync for Intr {}

impl Intr {
    // ======== Field Constants ========

    /// Bits [4:4], mask 0x00000010
    pub const APB_RD_NSEC_FAIL_OFFSET: u32 = 4;
    pub const APB_RD_NSEC_FAIL_MASK: u32 = 0x00000010;
    pub const APB_RD_NSEC_FAIL: u32 = 1 << 4;

    /// Bits [3:3], mask 0x00000008
    pub const APB_RD_SEC_FAIL_OFFSET: u32 = 3;
    pub const APB_RD_SEC_FAIL_MASK: u32 = 0x00000008;
    pub const APB_RD_SEC_FAIL: u32 = 1 << 3;

    /// Bits [2:2], mask 0x00000004
    pub const APB_DCTRL_FAIL_OFFSET: u32 = 2;
    pub const APB_DCTRL_FAIL_MASK: u32 = 0x00000004;
    pub const APB_DCTRL_FAIL: u32 = 1 << 2;

    /// Bits [1:1], mask 0x00000002
    pub const SBPI_WR_FAIL_OFFSET: u32 = 1;
    pub const SBPI_WR_FAIL_MASK: u32 = 0x00000002;
    pub const SBPI_WR_FAIL: u32 = 1 << 1;

    /// Bits [0:0], mask 0x00000001
    pub const SBPI_FLAG_N_OFFSET: u32 = 0;
    pub const SBPI_FLAG_N_MASK: u32 = 0x00000001;
    pub const SBPI_FLAG_N: u32 = 1 << 0;

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

    /// **APB_RD_NSEC_FAIL** [4:4]
    #[inline(always)]
    pub fn apb_rd_nsec_fail(&self) -> bool {
        (self.read_bits() & Self::APB_RD_NSEC_FAIL_MASK) != 0
    }

    /// Set APB_RD_NSEC_FAIL bit
    #[inline(always)]
    pub fn set_apb_rd_nsec_fail(&self) {
        self.set_bits(Self::APB_RD_NSEC_FAIL_MASK);
    }

    /// Clear APB_RD_NSEC_FAIL bit
    #[inline(always)]
    pub fn clr_apb_rd_nsec_fail(&self) {
        self.clr_bits(Self::APB_RD_NSEC_FAIL_MASK);
    }

    /// **APB_RD_SEC_FAIL** [3:3]
    #[inline(always)]
    pub fn apb_rd_sec_fail(&self) -> bool {
        (self.read_bits() & Self::APB_RD_SEC_FAIL_MASK) != 0
    }

    /// Set APB_RD_SEC_FAIL bit
    #[inline(always)]
    pub fn set_apb_rd_sec_fail(&self) {
        self.set_bits(Self::APB_RD_SEC_FAIL_MASK);
    }

    /// Clear APB_RD_SEC_FAIL bit
    #[inline(always)]
    pub fn clr_apb_rd_sec_fail(&self) {
        self.clr_bits(Self::APB_RD_SEC_FAIL_MASK);
    }

    /// **APB_DCTRL_FAIL** [2:2]
    #[inline(always)]
    pub fn apb_dctrl_fail(&self) -> bool {
        (self.read_bits() & Self::APB_DCTRL_FAIL_MASK) != 0
    }

    /// Set APB_DCTRL_FAIL bit
    #[inline(always)]
    pub fn set_apb_dctrl_fail(&self) {
        self.set_bits(Self::APB_DCTRL_FAIL_MASK);
    }

    /// Clear APB_DCTRL_FAIL bit
    #[inline(always)]
    pub fn clr_apb_dctrl_fail(&self) {
        self.clr_bits(Self::APB_DCTRL_FAIL_MASK);
    }

    /// **SBPI_WR_FAIL** [1:1]
    #[inline(always)]
    pub fn sbpi_wr_fail(&self) -> bool {
        (self.read_bits() & Self::SBPI_WR_FAIL_MASK) != 0
    }

    /// Set SBPI_WR_FAIL bit
    #[inline(always)]
    pub fn set_sbpi_wr_fail(&self) {
        self.set_bits(Self::SBPI_WR_FAIL_MASK);
    }

    /// Clear SBPI_WR_FAIL bit
    #[inline(always)]
    pub fn clr_sbpi_wr_fail(&self) {
        self.clr_bits(Self::SBPI_WR_FAIL_MASK);
    }

    /// **SBPI_FLAG_N** [0:0]
    #[inline(always)]
    pub fn sbpi_flag_n(&self) -> bool {
        (self.read_bits() & Self::SBPI_FLAG_N_MASK) != 0
    }

}

/// INTR value type for batch read/modify/write operations
///
/// Use `intr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `intr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IntrVal {
    bits: u32,
}

impl IntrVal {
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

    /// Read APB_RD_NSEC_FAIL [4:4]
    #[inline(always)]
    pub const fn apb_rd_nsec_fail(&self) -> bool {
        (self.bits & Intr::APB_RD_NSEC_FAIL_MASK) != 0
    }

    /// Read APB_RD_SEC_FAIL [3:3]
    #[inline(always)]
    pub const fn apb_rd_sec_fail(&self) -> bool {
        (self.bits & Intr::APB_RD_SEC_FAIL_MASK) != 0
    }

    /// Read APB_DCTRL_FAIL [2:2]
    #[inline(always)]
    pub const fn apb_dctrl_fail(&self) -> bool {
        (self.bits & Intr::APB_DCTRL_FAIL_MASK) != 0
    }

    /// Read SBPI_WR_FAIL [1:1]
    #[inline(always)]
    pub const fn sbpi_wr_fail(&self) -> bool {
        (self.bits & Intr::SBPI_WR_FAIL_MASK) != 0
    }

    /// Read SBPI_FLAG_N [0:0]
    #[inline(always)]
    pub const fn sbpi_flag_n(&self) -> bool {
        (self.bits & Intr::SBPI_FLAG_N_MASK) != 0
    }

    /// Set APB_RD_NSEC_FAIL [4:4]
    #[inline(always)]
    pub const fn with_apb_rd_nsec_fail(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intr::APB_RD_NSEC_FAIL_MASK
            } else {
                self.bits & !Intr::APB_RD_NSEC_FAIL_MASK
            }
        }
    }

    /// Set APB_RD_SEC_FAIL [3:3]
    #[inline(always)]
    pub const fn with_apb_rd_sec_fail(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intr::APB_RD_SEC_FAIL_MASK
            } else {
                self.bits & !Intr::APB_RD_SEC_FAIL_MASK
            }
        }
    }

    /// Set APB_DCTRL_FAIL [2:2]
    #[inline(always)]
    pub const fn with_apb_dctrl_fail(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intr::APB_DCTRL_FAIL_MASK
            } else {
                self.bits & !Intr::APB_DCTRL_FAIL_MASK
            }
        }
    }

    /// Set SBPI_WR_FAIL [1:1]
    #[inline(always)]
    pub const fn with_sbpi_wr_fail(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intr::SBPI_WR_FAIL_MASK
            } else {
                self.bits & !Intr::SBPI_WR_FAIL_MASK
            }
        }
    }

}

impl Default for IntrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Intr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> IntrVal {
        IntrVal::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: IntrVal) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(IntrVal) -> IntrVal>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// INTE Register
///
/// Interrupt Enable
///
/// Address offset: 0x0168
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **APB_RD_NSEC_FAIL** [4] (RW): 
/// - **APB_RD_SEC_FAIL** [3] (RW): 
/// - **APB_DCTRL_FAIL** [2] (RW): 
/// - **SBPI_WR_FAIL** [1] (RW): 
/// - **SBPI_FLAG_N** [0] (RW): 
#[repr(transparent)]
pub struct Inte {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for Inte {}
unsafe impl Sync for Inte {}

impl Inte {
    // ======== Field Constants ========

    /// Bits [4:4], mask 0x00000010
    pub const APB_RD_NSEC_FAIL_OFFSET: u32 = 4;
    pub const APB_RD_NSEC_FAIL_MASK: u32 = 0x00000010;
    pub const APB_RD_NSEC_FAIL: u32 = 1 << 4;

    /// Bits [3:3], mask 0x00000008
    pub const APB_RD_SEC_FAIL_OFFSET: u32 = 3;
    pub const APB_RD_SEC_FAIL_MASK: u32 = 0x00000008;
    pub const APB_RD_SEC_FAIL: u32 = 1 << 3;

    /// Bits [2:2], mask 0x00000004
    pub const APB_DCTRL_FAIL_OFFSET: u32 = 2;
    pub const APB_DCTRL_FAIL_MASK: u32 = 0x00000004;
    pub const APB_DCTRL_FAIL: u32 = 1 << 2;

    /// Bits [1:1], mask 0x00000002
    pub const SBPI_WR_FAIL_OFFSET: u32 = 1;
    pub const SBPI_WR_FAIL_MASK: u32 = 0x00000002;
    pub const SBPI_WR_FAIL: u32 = 1 << 1;

    /// Bits [0:0], mask 0x00000001
    pub const SBPI_FLAG_N_OFFSET: u32 = 0;
    pub const SBPI_FLAG_N_MASK: u32 = 0x00000001;
    pub const SBPI_FLAG_N: u32 = 1 << 0;

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

    /// **APB_RD_NSEC_FAIL** [4:4]
    #[inline(always)]
    pub fn apb_rd_nsec_fail(&self) -> bool {
        (self.read_bits() & Self::APB_RD_NSEC_FAIL_MASK) != 0
    }

    /// Set APB_RD_NSEC_FAIL bit
    #[inline(always)]
    pub fn set_apb_rd_nsec_fail(&self) {
        self.set_bits(Self::APB_RD_NSEC_FAIL_MASK);
    }

    /// Clear APB_RD_NSEC_FAIL bit
    #[inline(always)]
    pub fn clr_apb_rd_nsec_fail(&self) {
        self.clr_bits(Self::APB_RD_NSEC_FAIL_MASK);
    }

    /// **APB_RD_SEC_FAIL** [3:3]
    #[inline(always)]
    pub fn apb_rd_sec_fail(&self) -> bool {
        (self.read_bits() & Self::APB_RD_SEC_FAIL_MASK) != 0
    }

    /// Set APB_RD_SEC_FAIL bit
    #[inline(always)]
    pub fn set_apb_rd_sec_fail(&self) {
        self.set_bits(Self::APB_RD_SEC_FAIL_MASK);
    }

    /// Clear APB_RD_SEC_FAIL bit
    #[inline(always)]
    pub fn clr_apb_rd_sec_fail(&self) {
        self.clr_bits(Self::APB_RD_SEC_FAIL_MASK);
    }

    /// **APB_DCTRL_FAIL** [2:2]
    #[inline(always)]
    pub fn apb_dctrl_fail(&self) -> bool {
        (self.read_bits() & Self::APB_DCTRL_FAIL_MASK) != 0
    }

    /// Set APB_DCTRL_FAIL bit
    #[inline(always)]
    pub fn set_apb_dctrl_fail(&self) {
        self.set_bits(Self::APB_DCTRL_FAIL_MASK);
    }

    /// Clear APB_DCTRL_FAIL bit
    #[inline(always)]
    pub fn clr_apb_dctrl_fail(&self) {
        self.clr_bits(Self::APB_DCTRL_FAIL_MASK);
    }

    /// **SBPI_WR_FAIL** [1:1]
    #[inline(always)]
    pub fn sbpi_wr_fail(&self) -> bool {
        (self.read_bits() & Self::SBPI_WR_FAIL_MASK) != 0
    }

    /// Set SBPI_WR_FAIL bit
    #[inline(always)]
    pub fn set_sbpi_wr_fail(&self) {
        self.set_bits(Self::SBPI_WR_FAIL_MASK);
    }

    /// Clear SBPI_WR_FAIL bit
    #[inline(always)]
    pub fn clr_sbpi_wr_fail(&self) {
        self.clr_bits(Self::SBPI_WR_FAIL_MASK);
    }

    /// **SBPI_FLAG_N** [0:0]
    #[inline(always)]
    pub fn sbpi_flag_n(&self) -> bool {
        (self.read_bits() & Self::SBPI_FLAG_N_MASK) != 0
    }

    /// Set SBPI_FLAG_N bit
    #[inline(always)]
    pub fn set_sbpi_flag_n(&self) {
        self.set_bits(Self::SBPI_FLAG_N_MASK);
    }

    /// Clear SBPI_FLAG_N bit
    #[inline(always)]
    pub fn clr_sbpi_flag_n(&self) {
        self.clr_bits(Self::SBPI_FLAG_N_MASK);
    }

}

/// INTE value type for batch read/modify/write operations
///
/// Use `inte.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `inte.write_val(val)`.
#[derive(Clone, Copy)]
pub struct InteVal {
    bits: u32,
}

impl InteVal {
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

    /// Read APB_RD_NSEC_FAIL [4:4]
    #[inline(always)]
    pub const fn apb_rd_nsec_fail(&self) -> bool {
        (self.bits & Inte::APB_RD_NSEC_FAIL_MASK) != 0
    }

    /// Read APB_RD_SEC_FAIL [3:3]
    #[inline(always)]
    pub const fn apb_rd_sec_fail(&self) -> bool {
        (self.bits & Inte::APB_RD_SEC_FAIL_MASK) != 0
    }

    /// Read APB_DCTRL_FAIL [2:2]
    #[inline(always)]
    pub const fn apb_dctrl_fail(&self) -> bool {
        (self.bits & Inte::APB_DCTRL_FAIL_MASK) != 0
    }

    /// Read SBPI_WR_FAIL [1:1]
    #[inline(always)]
    pub const fn sbpi_wr_fail(&self) -> bool {
        (self.bits & Inte::SBPI_WR_FAIL_MASK) != 0
    }

    /// Read SBPI_FLAG_N [0:0]
    #[inline(always)]
    pub const fn sbpi_flag_n(&self) -> bool {
        (self.bits & Inte::SBPI_FLAG_N_MASK) != 0
    }

    /// Set APB_RD_NSEC_FAIL [4:4]
    #[inline(always)]
    pub const fn with_apb_rd_nsec_fail(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Inte::APB_RD_NSEC_FAIL_MASK
            } else {
                self.bits & !Inte::APB_RD_NSEC_FAIL_MASK
            }
        }
    }

    /// Set APB_RD_SEC_FAIL [3:3]
    #[inline(always)]
    pub const fn with_apb_rd_sec_fail(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Inte::APB_RD_SEC_FAIL_MASK
            } else {
                self.bits & !Inte::APB_RD_SEC_FAIL_MASK
            }
        }
    }

    /// Set APB_DCTRL_FAIL [2:2]
    #[inline(always)]
    pub const fn with_apb_dctrl_fail(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Inte::APB_DCTRL_FAIL_MASK
            } else {
                self.bits & !Inte::APB_DCTRL_FAIL_MASK
            }
        }
    }

    /// Set SBPI_WR_FAIL [1:1]
    #[inline(always)]
    pub const fn with_sbpi_wr_fail(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Inte::SBPI_WR_FAIL_MASK
            } else {
                self.bits & !Inte::SBPI_WR_FAIL_MASK
            }
        }
    }

    /// Set SBPI_FLAG_N [0:0]
    #[inline(always)]
    pub const fn with_sbpi_flag_n(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Inte::SBPI_FLAG_N_MASK
            } else {
                self.bits & !Inte::SBPI_FLAG_N_MASK
            }
        }
    }

}

impl Default for InteVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Inte {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> InteVal {
        InteVal::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: InteVal) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(InteVal) -> InteVal>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// INTF Register
///
/// Interrupt Force
///
/// Address offset: 0x016c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **APB_RD_NSEC_FAIL** [4] (RW): 
/// - **APB_RD_SEC_FAIL** [3] (RW): 
/// - **APB_DCTRL_FAIL** [2] (RW): 
/// - **SBPI_WR_FAIL** [1] (RW): 
/// - **SBPI_FLAG_N** [0] (RW): 
#[repr(transparent)]
pub struct Intf {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for Intf {}
unsafe impl Sync for Intf {}

impl Intf {
    // ======== Field Constants ========

    /// Bits [4:4], mask 0x00000010
    pub const APB_RD_NSEC_FAIL_OFFSET: u32 = 4;
    pub const APB_RD_NSEC_FAIL_MASK: u32 = 0x00000010;
    pub const APB_RD_NSEC_FAIL: u32 = 1 << 4;

    /// Bits [3:3], mask 0x00000008
    pub const APB_RD_SEC_FAIL_OFFSET: u32 = 3;
    pub const APB_RD_SEC_FAIL_MASK: u32 = 0x00000008;
    pub const APB_RD_SEC_FAIL: u32 = 1 << 3;

    /// Bits [2:2], mask 0x00000004
    pub const APB_DCTRL_FAIL_OFFSET: u32 = 2;
    pub const APB_DCTRL_FAIL_MASK: u32 = 0x00000004;
    pub const APB_DCTRL_FAIL: u32 = 1 << 2;

    /// Bits [1:1], mask 0x00000002
    pub const SBPI_WR_FAIL_OFFSET: u32 = 1;
    pub const SBPI_WR_FAIL_MASK: u32 = 0x00000002;
    pub const SBPI_WR_FAIL: u32 = 1 << 1;

    /// Bits [0:0], mask 0x00000001
    pub const SBPI_FLAG_N_OFFSET: u32 = 0;
    pub const SBPI_FLAG_N_MASK: u32 = 0x00000001;
    pub const SBPI_FLAG_N: u32 = 1 << 0;

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

    /// **APB_RD_NSEC_FAIL** [4:4]
    #[inline(always)]
    pub fn apb_rd_nsec_fail(&self) -> bool {
        (self.read_bits() & Self::APB_RD_NSEC_FAIL_MASK) != 0
    }

    /// Set APB_RD_NSEC_FAIL bit
    #[inline(always)]
    pub fn set_apb_rd_nsec_fail(&self) {
        self.set_bits(Self::APB_RD_NSEC_FAIL_MASK);
    }

    /// Clear APB_RD_NSEC_FAIL bit
    #[inline(always)]
    pub fn clr_apb_rd_nsec_fail(&self) {
        self.clr_bits(Self::APB_RD_NSEC_FAIL_MASK);
    }

    /// **APB_RD_SEC_FAIL** [3:3]
    #[inline(always)]
    pub fn apb_rd_sec_fail(&self) -> bool {
        (self.read_bits() & Self::APB_RD_SEC_FAIL_MASK) != 0
    }

    /// Set APB_RD_SEC_FAIL bit
    #[inline(always)]
    pub fn set_apb_rd_sec_fail(&self) {
        self.set_bits(Self::APB_RD_SEC_FAIL_MASK);
    }

    /// Clear APB_RD_SEC_FAIL bit
    #[inline(always)]
    pub fn clr_apb_rd_sec_fail(&self) {
        self.clr_bits(Self::APB_RD_SEC_FAIL_MASK);
    }

    /// **APB_DCTRL_FAIL** [2:2]
    #[inline(always)]
    pub fn apb_dctrl_fail(&self) -> bool {
        (self.read_bits() & Self::APB_DCTRL_FAIL_MASK) != 0
    }

    /// Set APB_DCTRL_FAIL bit
    #[inline(always)]
    pub fn set_apb_dctrl_fail(&self) {
        self.set_bits(Self::APB_DCTRL_FAIL_MASK);
    }

    /// Clear APB_DCTRL_FAIL bit
    #[inline(always)]
    pub fn clr_apb_dctrl_fail(&self) {
        self.clr_bits(Self::APB_DCTRL_FAIL_MASK);
    }

    /// **SBPI_WR_FAIL** [1:1]
    #[inline(always)]
    pub fn sbpi_wr_fail(&self) -> bool {
        (self.read_bits() & Self::SBPI_WR_FAIL_MASK) != 0
    }

    /// Set SBPI_WR_FAIL bit
    #[inline(always)]
    pub fn set_sbpi_wr_fail(&self) {
        self.set_bits(Self::SBPI_WR_FAIL_MASK);
    }

    /// Clear SBPI_WR_FAIL bit
    #[inline(always)]
    pub fn clr_sbpi_wr_fail(&self) {
        self.clr_bits(Self::SBPI_WR_FAIL_MASK);
    }

    /// **SBPI_FLAG_N** [0:0]
    #[inline(always)]
    pub fn sbpi_flag_n(&self) -> bool {
        (self.read_bits() & Self::SBPI_FLAG_N_MASK) != 0
    }

    /// Set SBPI_FLAG_N bit
    #[inline(always)]
    pub fn set_sbpi_flag_n(&self) {
        self.set_bits(Self::SBPI_FLAG_N_MASK);
    }

    /// Clear SBPI_FLAG_N bit
    #[inline(always)]
    pub fn clr_sbpi_flag_n(&self) {
        self.clr_bits(Self::SBPI_FLAG_N_MASK);
    }

}

/// INTF value type for batch read/modify/write operations
///
/// Use `intf.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `intf.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IntfVal {
    bits: u32,
}

impl IntfVal {
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

    /// Read APB_RD_NSEC_FAIL [4:4]
    #[inline(always)]
    pub const fn apb_rd_nsec_fail(&self) -> bool {
        (self.bits & Intf::APB_RD_NSEC_FAIL_MASK) != 0
    }

    /// Read APB_RD_SEC_FAIL [3:3]
    #[inline(always)]
    pub const fn apb_rd_sec_fail(&self) -> bool {
        (self.bits & Intf::APB_RD_SEC_FAIL_MASK) != 0
    }

    /// Read APB_DCTRL_FAIL [2:2]
    #[inline(always)]
    pub const fn apb_dctrl_fail(&self) -> bool {
        (self.bits & Intf::APB_DCTRL_FAIL_MASK) != 0
    }

    /// Read SBPI_WR_FAIL [1:1]
    #[inline(always)]
    pub const fn sbpi_wr_fail(&self) -> bool {
        (self.bits & Intf::SBPI_WR_FAIL_MASK) != 0
    }

    /// Read SBPI_FLAG_N [0:0]
    #[inline(always)]
    pub const fn sbpi_flag_n(&self) -> bool {
        (self.bits & Intf::SBPI_FLAG_N_MASK) != 0
    }

    /// Set APB_RD_NSEC_FAIL [4:4]
    #[inline(always)]
    pub const fn with_apb_rd_nsec_fail(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intf::APB_RD_NSEC_FAIL_MASK
            } else {
                self.bits & !Intf::APB_RD_NSEC_FAIL_MASK
            }
        }
    }

    /// Set APB_RD_SEC_FAIL [3:3]
    #[inline(always)]
    pub const fn with_apb_rd_sec_fail(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intf::APB_RD_SEC_FAIL_MASK
            } else {
                self.bits & !Intf::APB_RD_SEC_FAIL_MASK
            }
        }
    }

    /// Set APB_DCTRL_FAIL [2:2]
    #[inline(always)]
    pub const fn with_apb_dctrl_fail(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intf::APB_DCTRL_FAIL_MASK
            } else {
                self.bits & !Intf::APB_DCTRL_FAIL_MASK
            }
        }
    }

    /// Set SBPI_WR_FAIL [1:1]
    #[inline(always)]
    pub const fn with_sbpi_wr_fail(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intf::SBPI_WR_FAIL_MASK
            } else {
                self.bits & !Intf::SBPI_WR_FAIL_MASK
            }
        }
    }

    /// Set SBPI_FLAG_N [0:0]
    #[inline(always)]
    pub const fn with_sbpi_flag_n(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intf::SBPI_FLAG_N_MASK
            } else {
                self.bits & !Intf::SBPI_FLAG_N_MASK
            }
        }
    }

}

impl Default for IntfVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Intf {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> IntfVal {
        IntfVal::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: IntfVal) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(IntfVal) -> IntfVal>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// INTS Register
///
/// Interrupt status after masking & forcing
///
/// Address offset: 0x0170
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **APB_RD_NSEC_FAIL** [4] (RO): 
/// - **APB_RD_SEC_FAIL** [3] (RO): 
/// - **APB_DCTRL_FAIL** [2] (RO): 
/// - **SBPI_WR_FAIL** [1] (RO): 
/// - **SBPI_FLAG_N** [0] (RO): 
#[repr(transparent)]
pub struct Ints {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for Ints {}
unsafe impl Sync for Ints {}

impl Ints {
    // ======== Field Constants ========

    /// Bits [4:4], mask 0x00000010
    pub const APB_RD_NSEC_FAIL_OFFSET: u32 = 4;
    pub const APB_RD_NSEC_FAIL_MASK: u32 = 0x00000010;
    pub const APB_RD_NSEC_FAIL: u32 = 1 << 4;

    /// Bits [3:3], mask 0x00000008
    pub const APB_RD_SEC_FAIL_OFFSET: u32 = 3;
    pub const APB_RD_SEC_FAIL_MASK: u32 = 0x00000008;
    pub const APB_RD_SEC_FAIL: u32 = 1 << 3;

    /// Bits [2:2], mask 0x00000004
    pub const APB_DCTRL_FAIL_OFFSET: u32 = 2;
    pub const APB_DCTRL_FAIL_MASK: u32 = 0x00000004;
    pub const APB_DCTRL_FAIL: u32 = 1 << 2;

    /// Bits [1:1], mask 0x00000002
    pub const SBPI_WR_FAIL_OFFSET: u32 = 1;
    pub const SBPI_WR_FAIL_MASK: u32 = 0x00000002;
    pub const SBPI_WR_FAIL: u32 = 1 << 1;

    /// Bits [0:0], mask 0x00000001
    pub const SBPI_FLAG_N_OFFSET: u32 = 0;
    pub const SBPI_FLAG_N_MASK: u32 = 0x00000001;
    pub const SBPI_FLAG_N: u32 = 1 << 0;

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

    /// **APB_RD_NSEC_FAIL** [4:4]
    #[inline(always)]
    pub fn apb_rd_nsec_fail(&self) -> bool {
        (self.read_bits() & Self::APB_RD_NSEC_FAIL_MASK) != 0
    }

    /// **APB_RD_SEC_FAIL** [3:3]
    #[inline(always)]
    pub fn apb_rd_sec_fail(&self) -> bool {
        (self.read_bits() & Self::APB_RD_SEC_FAIL_MASK) != 0
    }

    /// **APB_DCTRL_FAIL** [2:2]
    #[inline(always)]
    pub fn apb_dctrl_fail(&self) -> bool {
        (self.read_bits() & Self::APB_DCTRL_FAIL_MASK) != 0
    }

    /// **SBPI_WR_FAIL** [1:1]
    #[inline(always)]
    pub fn sbpi_wr_fail(&self) -> bool {
        (self.read_bits() & Self::SBPI_WR_FAIL_MASK) != 0
    }

    /// **SBPI_FLAG_N** [0:0]
    #[inline(always)]
    pub fn sbpi_flag_n(&self) -> bool {
        (self.read_bits() & Self::SBPI_FLAG_N_MASK) != 0
    }

}

/// INTS value type for batch read/modify/write operations
///
/// Use `ints.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ints.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IntsVal {
    bits: u32,
}

impl IntsVal {
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

    /// Read APB_RD_NSEC_FAIL [4:4]
    #[inline(always)]
    pub const fn apb_rd_nsec_fail(&self) -> bool {
        (self.bits & Ints::APB_RD_NSEC_FAIL_MASK) != 0
    }

    /// Read APB_RD_SEC_FAIL [3:3]
    #[inline(always)]
    pub const fn apb_rd_sec_fail(&self) -> bool {
        (self.bits & Ints::APB_RD_SEC_FAIL_MASK) != 0
    }

    /// Read APB_DCTRL_FAIL [2:2]
    #[inline(always)]
    pub const fn apb_dctrl_fail(&self) -> bool {
        (self.bits & Ints::APB_DCTRL_FAIL_MASK) != 0
    }

    /// Read SBPI_WR_FAIL [1:1]
    #[inline(always)]
    pub const fn sbpi_wr_fail(&self) -> bool {
        (self.bits & Ints::SBPI_WR_FAIL_MASK) != 0
    }

    /// Read SBPI_FLAG_N [0:0]
    #[inline(always)]
    pub const fn sbpi_flag_n(&self) -> bool {
        (self.bits & Ints::SBPI_FLAG_N_MASK) != 0
    }

}

impl Default for IntsVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Ints {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> IntsVal {
        IntsVal::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: IntsVal) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(IntsVal) -> IntsVal>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// Register block
#[repr(C)]
pub struct RegisterBlock {
    /// 0x0000 - Software lock register for page 0. Locks are initialised from the OTP lock ...
    pub sw_lock0: SwLock0,
    /// 0x0004 - Software lock register for page 1. Locks are initialised from the OTP lock ...
    pub sw_lock1: SwLock1,
    /// 0x0008 - Software lock register for page 2. Locks are initialised from the OTP lock ...
    pub sw_lock2: SwLock2,
    /// 0x000c - Software lock register for page 3. Locks are initialised from the OTP lock ...
    pub sw_lock3: SwLock3,
    /// 0x0010 - Software lock register for page 4. Locks are initialised from the OTP lock ...
    pub sw_lock4: SwLock4,
    /// 0x0014 - Software lock register for page 5. Locks are initialised from the OTP lock ...
    pub sw_lock5: SwLock5,
    /// 0x0018 - Software lock register for page 6. Locks are initialised from the OTP lock ...
    pub sw_lock6: SwLock6,
    /// 0x001c - Software lock register for page 7. Locks are initialised from the OTP lock ...
    pub sw_lock7: SwLock7,
    /// 0x0020 - Software lock register for page 8. Locks are initialised from the OTP lock ...
    pub sw_lock8: SwLock8,
    /// 0x0024 - Software lock register for page 9. Locks are initialised from the OTP lock ...
    pub sw_lock9: SwLock9,
    /// 0x0028 - Software lock register for page 10. Locks are initialised from the OTP lock...
    pub sw_lock10: SwLock10,
    /// 0x002c - Software lock register for page 11. Locks are initialised from the OTP lock...
    pub sw_lock11: SwLock11,
    /// 0x0030 - Software lock register for page 12. Locks are initialised from the OTP lock...
    pub sw_lock12: SwLock12,
    /// 0x0034 - Software lock register for page 13. Locks are initialised from the OTP lock...
    pub sw_lock13: SwLock13,
    /// 0x0038 - Software lock register for page 14. Locks are initialised from the OTP lock...
    pub sw_lock14: SwLock14,
    /// 0x003c - Software lock register for page 15. Locks are initialised from the OTP lock...
    pub sw_lock15: SwLock15,
    /// 0x0040 - Software lock register for page 16. Locks are initialised from the OTP lock...
    pub sw_lock16: SwLock16,
    /// 0x0044 - Software lock register for page 17. Locks are initialised from the OTP lock...
    pub sw_lock17: SwLock17,
    /// 0x0048 - Software lock register for page 18. Locks are initialised from the OTP lock...
    pub sw_lock18: SwLock18,
    /// 0x004c - Software lock register for page 19. Locks are initialised from the OTP lock...
    pub sw_lock19: SwLock19,
    /// 0x0050 - Software lock register for page 20. Locks are initialised from the OTP lock...
    pub sw_lock20: SwLock20,
    /// 0x0054 - Software lock register for page 21. Locks are initialised from the OTP lock...
    pub sw_lock21: SwLock21,
    /// 0x0058 - Software lock register for page 22. Locks are initialised from the OTP lock...
    pub sw_lock22: SwLock22,
    /// 0x005c - Software lock register for page 23. Locks are initialised from the OTP lock...
    pub sw_lock23: SwLock23,
    /// 0x0060 - Software lock register for page 24. Locks are initialised from the OTP lock...
    pub sw_lock24: SwLock24,
    /// 0x0064 - Software lock register for page 25. Locks are initialised from the OTP lock...
    pub sw_lock25: SwLock25,
    /// 0x0068 - Software lock register for page 26. Locks are initialised from the OTP lock...
    pub sw_lock26: SwLock26,
    /// 0x006c - Software lock register for page 27. Locks are initialised from the OTP lock...
    pub sw_lock27: SwLock27,
    /// 0x0070 - Software lock register for page 28. Locks are initialised from the OTP lock...
    pub sw_lock28: SwLock28,
    /// 0x0074 - Software lock register for page 29. Locks are initialised from the OTP lock...
    pub sw_lock29: SwLock29,
    /// 0x0078 - Software lock register for page 30. Locks are initialised from the OTP lock...
    pub sw_lock30: SwLock30,
    /// 0x007c - Software lock register for page 31. Locks are initialised from the OTP lock...
    pub sw_lock31: SwLock31,
    /// 0x0080 - Software lock register for page 32. Locks are initialised from the OTP lock...
    pub sw_lock32: SwLock32,
    /// 0x0084 - Software lock register for page 33. Locks are initialised from the OTP lock...
    pub sw_lock33: SwLock33,
    /// 0x0088 - Software lock register for page 34. Locks are initialised from the OTP lock...
    pub sw_lock34: SwLock34,
    /// 0x008c - Software lock register for page 35. Locks are initialised from the OTP lock...
    pub sw_lock35: SwLock35,
    /// 0x0090 - Software lock register for page 36. Locks are initialised from the OTP lock...
    pub sw_lock36: SwLock36,
    /// 0x0094 - Software lock register for page 37. Locks are initialised from the OTP lock...
    pub sw_lock37: SwLock37,
    /// 0x0098 - Software lock register for page 38. Locks are initialised from the OTP lock...
    pub sw_lock38: SwLock38,
    /// 0x009c - Software lock register for page 39. Locks are initialised from the OTP lock...
    pub sw_lock39: SwLock39,
    /// 0x00a0 - Software lock register for page 40. Locks are initialised from the OTP lock...
    pub sw_lock40: SwLock40,
    /// 0x00a4 - Software lock register for page 41. Locks are initialised from the OTP lock...
    pub sw_lock41: SwLock41,
    /// 0x00a8 - Software lock register for page 42. Locks are initialised from the OTP lock...
    pub sw_lock42: SwLock42,
    /// 0x00ac - Software lock register for page 43. Locks are initialised from the OTP lock...
    pub sw_lock43: SwLock43,
    /// 0x00b0 - Software lock register for page 44. Locks are initialised from the OTP lock...
    pub sw_lock44: SwLock44,
    /// 0x00b4 - Software lock register for page 45. Locks are initialised from the OTP lock...
    pub sw_lock45: SwLock45,
    /// 0x00b8 - Software lock register for page 46. Locks are initialised from the OTP lock...
    pub sw_lock46: SwLock46,
    /// 0x00bc - Software lock register for page 47. Locks are initialised from the OTP lock...
    pub sw_lock47: SwLock47,
    /// 0x00c0 - Software lock register for page 48. Locks are initialised from the OTP lock...
    pub sw_lock48: SwLock48,
    /// 0x00c4 - Software lock register for page 49. Locks are initialised from the OTP lock...
    pub sw_lock49: SwLock49,
    /// 0x00c8 - Software lock register for page 50. Locks are initialised from the OTP lock...
    pub sw_lock50: SwLock50,
    /// 0x00cc - Software lock register for page 51. Locks are initialised from the OTP lock...
    pub sw_lock51: SwLock51,
    /// 0x00d0 - Software lock register for page 52. Locks are initialised from the OTP lock...
    pub sw_lock52: SwLock52,
    /// 0x00d4 - Software lock register for page 53. Locks are initialised from the OTP lock...
    pub sw_lock53: SwLock53,
    /// 0x00d8 - Software lock register for page 54. Locks are initialised from the OTP lock...
    pub sw_lock54: SwLock54,
    /// 0x00dc - Software lock register for page 55. Locks are initialised from the OTP lock...
    pub sw_lock55: SwLock55,
    /// 0x00e0 - Software lock register for page 56. Locks are initialised from the OTP lock...
    pub sw_lock56: SwLock56,
    /// 0x00e4 - Software lock register for page 57. Locks are initialised from the OTP lock...
    pub sw_lock57: SwLock57,
    /// 0x00e8 - Software lock register for page 58. Locks are initialised from the OTP lock...
    pub sw_lock58: SwLock58,
    /// 0x00ec - Software lock register for page 59. Locks are initialised from the OTP lock...
    pub sw_lock59: SwLock59,
    /// 0x00f0 - Software lock register for page 60. Locks are initialised from the OTP lock...
    pub sw_lock60: SwLock60,
    /// 0x00f4 - Software lock register for page 61. Locks are initialised from the OTP lock...
    pub sw_lock61: SwLock61,
    /// 0x00f8 - Software lock register for page 62. Locks are initialised from the OTP lock...
    pub sw_lock62: SwLock62,
    /// 0x00fc - Software lock register for page 63. Locks are initialised from the OTP lock...
    pub sw_lock63: SwLock63,
    /// 0x0100 - Dispatch instructions to the SBPI interface, used for programming the OTP f...
    pub sbpi_instr: SbpiInstr,
    /// 0x0104 - SBPI write payload bytes 3..0
    pub sbpi_wdata_0: SbpiWdata0,
    /// 0x0108 - SBPI write payload bytes 7..4
    pub sbpi_wdata_1: SbpiWdata1,
    /// 0x010c - SBPI write payload bytes 11..8
    pub sbpi_wdata_2: SbpiWdata2,
    /// 0x0110 - SBPI write payload bytes 15..12
    pub sbpi_wdata_3: SbpiWdata3,
    /// 0x0114 - Read payload bytes 3..0. Once read, the data in the register will automatic...
    pub sbpi_rdata_0: SbpiRdata0,
    /// 0x0118 - Read payload bytes 7..4. Once read, the data in the register will automatic...
    pub sbpi_rdata_1: SbpiRdata1,
    /// 0x011c - Read payload bytes 11..8. Once read, the data in the register will automati...
    pub sbpi_rdata_2: SbpiRdata2,
    /// 0x0120 - Read payload bytes 15..12. Once read, the data in the register will automat...
    pub sbpi_rdata_3: SbpiRdata3,
    /// 0x0124
    pub sbpi_status: SbpiStatus,
    /// 0x0128 - Controls for APB data read interface (USER interface)
    pub usr: Usr,
    /// 0x012c - Debug for OTP power-on state machine
    pub dbg: Dbg,
    _reserved0: [u8; 4],
    /// 0x0134 - During BIST, count address locations that have at least one leaky bit
    pub bist: Bist,
    /// 0x0138 - Word 0 (bits 31..0) of the key. Write only, read returns 0x0
    pub crt_key_w0: CrtKeyW0,
    /// 0x013c - Word 1 (bits 63..32) of the key. Write only, read returns 0x0
    pub crt_key_w1: CrtKeyW1,
    /// 0x0140 - Word 2 (bits 95..64) of the key. Write only, read returns 0x0
    pub crt_key_w2: CrtKeyW2,
    /// 0x0144 - Word 3 (bits 127..96) of the key. Write only, read returns 0x0
    pub crt_key_w3: CrtKeyW3,
    /// 0x0148 - Quickly check values of critical flags read during boot up
    pub critical: Critical,
    /// 0x014c - Which keys were valid (enrolled) at boot time
    pub key_valid: KeyValid,
    /// 0x0150 - Enable a debug feature that has been disabled. Debug features are disabled ...
    pub debugen: Debugen,
    /// 0x0154 - Write 1s to lock corresponding bits in DEBUGEN. This register is reset by t...
    pub debugen_lock: DebugenLock,
    /// 0x0158 - Architecture select (Arm/RISC-V). The default and allowable values of this ...
    pub archsel: Archsel,
    /// 0x015c - Get the current architecture select state of each core. Cores sample the cu...
    pub archsel_status: ArchselStatus,
    /// 0x0160 - Tell the bootrom to ignore scratch register boot vectors (both power manage...
    pub bootdis: Bootdis,
    /// 0x0164 - Raw Interrupts
    pub intr: Intr,
    /// 0x0168 - Interrupt Enable
    pub inte: Inte,
    /// 0x016c - Interrupt Force
    pub intf: Intf,
    /// 0x0170 - Interrupt status after masking & forcing
    pub ints: Ints,
}

