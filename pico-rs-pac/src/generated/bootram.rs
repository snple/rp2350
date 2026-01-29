//! BOOTRAM - Additional registers mapped adjacent to the bootram, for use by the bootrom.
//!
//! Additional registers mapped adjacent to the bootram, for use by the bootrom.

#![allow(dead_code)]

pub const BASE: u32 = 0x400e0000;

/// WRITE_ONCE0 Register
///
/// This registers always ORs writes into its current contents. Once a bit is
/// set, it can only be cleared by a reset.
///
/// Address offset: 0x0800
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **WRITE_ONCE0** [31:0] (RW): 
#[repr(transparent)]
pub struct WriteOnce0 {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for WriteOnce0 {}
unsafe impl Sync for WriteOnce0 {}

impl WriteOnce0 {
    // ======== Field Constants ========

    /// Bits [31:0], mask 0xffffffff
    pub const WRITE_ONCE0_OFFSET: u32 = 0;
    pub const WRITE_ONCE0_MASK: u32 = 0xffffffff;

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

    /// **WRITE_ONCE0** [31:0]
    #[inline(always)]
    pub fn write_once0(&self) -> u32 {
        (self.read_bits() & Self::WRITE_ONCE0_MASK) >> Self::WRITE_ONCE0_OFFSET
    }

    /// Write WRITE_ONCE0 field
    #[inline(always)]
    pub fn write_write_once0(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::WRITE_ONCE0_MASK) | ((value << Self::WRITE_ONCE0_OFFSET) & Self::WRITE_ONCE0_MASK));
    }

}

/// WRITE_ONCE0 value type for batch read/modify/write operations
///
/// Use `writeonce0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `writeonce0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct WriteOnce0Val {
    bits: u32,
}

impl WriteOnce0Val {
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

    /// Read WRITE_ONCE0 [31:0]
    #[inline(always)]
    pub const fn write_once0(&self) -> u32 {
        (self.bits & WriteOnce0::WRITE_ONCE0_MASK) >> WriteOnce0::WRITE_ONCE0_OFFSET
    }

    /// Set WRITE_ONCE0 [31:0]
    #[inline(always)]
    pub const fn with_write_once0(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !WriteOnce0::WRITE_ONCE0_MASK) | ((value << WriteOnce0::WRITE_ONCE0_OFFSET) & WriteOnce0::WRITE_ONCE0_MASK)
        }
    }

}

impl Default for WriteOnce0Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl WriteOnce0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> WriteOnce0Val {
        WriteOnce0Val::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: WriteOnce0Val) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(WriteOnce0Val) -> WriteOnce0Val>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// WRITE_ONCE1 Register
///
/// This registers always ORs writes into its current contents. Once a bit is
/// set, it can only be cleared by a reset.
///
/// Address offset: 0x0804
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **WRITE_ONCE1** [31:0] (RW): 
#[repr(transparent)]
pub struct WriteOnce1 {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for WriteOnce1 {}
unsafe impl Sync for WriteOnce1 {}

impl WriteOnce1 {
    // ======== Field Constants ========

    /// Bits [31:0], mask 0xffffffff
    pub const WRITE_ONCE1_OFFSET: u32 = 0;
    pub const WRITE_ONCE1_MASK: u32 = 0xffffffff;

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

    /// **WRITE_ONCE1** [31:0]
    #[inline(always)]
    pub fn write_once1(&self) -> u32 {
        (self.read_bits() & Self::WRITE_ONCE1_MASK) >> Self::WRITE_ONCE1_OFFSET
    }

    /// Write WRITE_ONCE1 field
    #[inline(always)]
    pub fn write_write_once1(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::WRITE_ONCE1_MASK) | ((value << Self::WRITE_ONCE1_OFFSET) & Self::WRITE_ONCE1_MASK));
    }

}

/// WRITE_ONCE1 value type for batch read/modify/write operations
///
/// Use `writeonce1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `writeonce1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct WriteOnce1Val {
    bits: u32,
}

impl WriteOnce1Val {
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

    /// Read WRITE_ONCE1 [31:0]
    #[inline(always)]
    pub const fn write_once1(&self) -> u32 {
        (self.bits & WriteOnce1::WRITE_ONCE1_MASK) >> WriteOnce1::WRITE_ONCE1_OFFSET
    }

    /// Set WRITE_ONCE1 [31:0]
    #[inline(always)]
    pub const fn with_write_once1(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !WriteOnce1::WRITE_ONCE1_MASK) | ((value << WriteOnce1::WRITE_ONCE1_OFFSET) & WriteOnce1::WRITE_ONCE1_MASK)
        }
    }

}

impl Default for WriteOnce1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl WriteOnce1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> WriteOnce1Val {
        WriteOnce1Val::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: WriteOnce1Val) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(WriteOnce1Val) -> WriteOnce1Val>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// BOOTLOCK_STAT Register
///
/// Bootlock status register. 1=unclaimed, 0=claimed. These locks function
/// identically to the SIO spinlocks, but are reserved for bootrom use.
///
/// Address offset: 0x0808
/// Reset value: 0x000000ff
///
/// ## Fields
///
/// - **BOOTLOCK_STAT** [7:0] (RW): 
#[repr(transparent)]
pub struct BootlockStat {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for BootlockStat {}
unsafe impl Sync for BootlockStat {}

impl BootlockStat {
    // ======== Field Constants ========

    /// Bits [7:0], mask 0x000000ff
    pub const BOOTLOCK_STAT_OFFSET: u32 = 0;
    pub const BOOTLOCK_STAT_MASK: u32 = 0x000000ff;

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

    /// **BOOTLOCK_STAT** [7:0]
    #[inline(always)]
    pub fn bootlock_stat(&self) -> u32 {
        (self.read_bits() & Self::BOOTLOCK_STAT_MASK) >> Self::BOOTLOCK_STAT_OFFSET
    }

    /// Write BOOTLOCK_STAT field
    #[inline(always)]
    pub fn write_bootlock_stat(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::BOOTLOCK_STAT_MASK) | ((value << Self::BOOTLOCK_STAT_OFFSET) & Self::BOOTLOCK_STAT_MASK));
    }

}

/// BOOTLOCK_STAT value type for batch read/modify/write operations
///
/// Use `bootlockstat.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `bootlockstat.write_val(val)`.
#[derive(Clone, Copy)]
pub struct BootlockStatVal {
    bits: u32,
}

impl BootlockStatVal {
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
        Self { bits: 0x000000ff }
    }

    /// Read BOOTLOCK_STAT [7:0]
    #[inline(always)]
    pub const fn bootlock_stat(&self) -> u32 {
        (self.bits & BootlockStat::BOOTLOCK_STAT_MASK) >> BootlockStat::BOOTLOCK_STAT_OFFSET
    }

    /// Set BOOTLOCK_STAT [7:0]
    #[inline(always)]
    pub const fn with_bootlock_stat(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !BootlockStat::BOOTLOCK_STAT_MASK) | ((value << BootlockStat::BOOTLOCK_STAT_OFFSET) & BootlockStat::BOOTLOCK_STAT_MASK)
        }
    }

}

impl Default for BootlockStatVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl BootlockStat {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> BootlockStatVal {
        BootlockStatVal::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: BootlockStatVal) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(BootlockStatVal) -> BootlockStatVal>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// BOOTLOCK0 Register
///
/// Read to claim and check. Write to unclaim. The value returned on successful
/// claim is 1 << n, and on failed claim is zero.
///
/// Address offset: 0x080c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **BOOTLOCK0** [31:0] (RW): 
#[repr(transparent)]
pub struct Bootlock0 {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for Bootlock0 {}
unsafe impl Sync for Bootlock0 {}

impl Bootlock0 {
    // ======== Field Constants ========

    /// Bits [31:0], mask 0xffffffff
    pub const BOOTLOCK0_OFFSET: u32 = 0;
    pub const BOOTLOCK0_MASK: u32 = 0xffffffff;

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

    /// **BOOTLOCK0** [31:0]
    #[inline(always)]
    pub fn bootlock0(&self) -> u32 {
        (self.read_bits() & Self::BOOTLOCK0_MASK) >> Self::BOOTLOCK0_OFFSET
    }

    /// Write BOOTLOCK0 field
    #[inline(always)]
    pub fn write_bootlock0(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::BOOTLOCK0_MASK) | ((value << Self::BOOTLOCK0_OFFSET) & Self::BOOTLOCK0_MASK));
    }

}

/// BOOTLOCK0 value type for batch read/modify/write operations
///
/// Use `bootlock0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `bootlock0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Bootlock0Val {
    bits: u32,
}

impl Bootlock0Val {
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

    /// Read BOOTLOCK0 [31:0]
    #[inline(always)]
    pub const fn bootlock0(&self) -> u32 {
        (self.bits & Bootlock0::BOOTLOCK0_MASK) >> Bootlock0::BOOTLOCK0_OFFSET
    }

    /// Set BOOTLOCK0 [31:0]
    #[inline(always)]
    pub const fn with_bootlock0(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Bootlock0::BOOTLOCK0_MASK) | ((value << Bootlock0::BOOTLOCK0_OFFSET) & Bootlock0::BOOTLOCK0_MASK)
        }
    }

}

impl Default for Bootlock0Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Bootlock0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> Bootlock0Val {
        Bootlock0Val::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: Bootlock0Val) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(Bootlock0Val) -> Bootlock0Val>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// BOOTLOCK1 Register
///
/// Read to claim and check. Write to unclaim. The value returned on successful
/// claim is 1 << n, and on failed claim is zero.
///
/// Address offset: 0x0810
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **BOOTLOCK1** [31:0] (RW): 
#[repr(transparent)]
pub struct Bootlock1 {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for Bootlock1 {}
unsafe impl Sync for Bootlock1 {}

impl Bootlock1 {
    // ======== Field Constants ========

    /// Bits [31:0], mask 0xffffffff
    pub const BOOTLOCK1_OFFSET: u32 = 0;
    pub const BOOTLOCK1_MASK: u32 = 0xffffffff;

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

    /// **BOOTLOCK1** [31:0]
    #[inline(always)]
    pub fn bootlock1(&self) -> u32 {
        (self.read_bits() & Self::BOOTLOCK1_MASK) >> Self::BOOTLOCK1_OFFSET
    }

    /// Write BOOTLOCK1 field
    #[inline(always)]
    pub fn write_bootlock1(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::BOOTLOCK1_MASK) | ((value << Self::BOOTLOCK1_OFFSET) & Self::BOOTLOCK1_MASK));
    }

}

/// BOOTLOCK1 value type for batch read/modify/write operations
///
/// Use `bootlock1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `bootlock1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Bootlock1Val {
    bits: u32,
}

impl Bootlock1Val {
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

    /// Read BOOTLOCK1 [31:0]
    #[inline(always)]
    pub const fn bootlock1(&self) -> u32 {
        (self.bits & Bootlock1::BOOTLOCK1_MASK) >> Bootlock1::BOOTLOCK1_OFFSET
    }

    /// Set BOOTLOCK1 [31:0]
    #[inline(always)]
    pub const fn with_bootlock1(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Bootlock1::BOOTLOCK1_MASK) | ((value << Bootlock1::BOOTLOCK1_OFFSET) & Bootlock1::BOOTLOCK1_MASK)
        }
    }

}

impl Default for Bootlock1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Bootlock1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> Bootlock1Val {
        Bootlock1Val::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: Bootlock1Val) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(Bootlock1Val) -> Bootlock1Val>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// BOOTLOCK2 Register
///
/// Read to claim and check. Write to unclaim. The value returned on successful
/// claim is 1 << n, and on failed claim is zero.
///
/// Address offset: 0x0814
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **BOOTLOCK2** [31:0] (RW): 
#[repr(transparent)]
pub struct Bootlock2 {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for Bootlock2 {}
unsafe impl Sync for Bootlock2 {}

impl Bootlock2 {
    // ======== Field Constants ========

    /// Bits [31:0], mask 0xffffffff
    pub const BOOTLOCK2_OFFSET: u32 = 0;
    pub const BOOTLOCK2_MASK: u32 = 0xffffffff;

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

    /// **BOOTLOCK2** [31:0]
    #[inline(always)]
    pub fn bootlock2(&self) -> u32 {
        (self.read_bits() & Self::BOOTLOCK2_MASK) >> Self::BOOTLOCK2_OFFSET
    }

    /// Write BOOTLOCK2 field
    #[inline(always)]
    pub fn write_bootlock2(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::BOOTLOCK2_MASK) | ((value << Self::BOOTLOCK2_OFFSET) & Self::BOOTLOCK2_MASK));
    }

}

/// BOOTLOCK2 value type for batch read/modify/write operations
///
/// Use `bootlock2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `bootlock2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Bootlock2Val {
    bits: u32,
}

impl Bootlock2Val {
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

    /// Read BOOTLOCK2 [31:0]
    #[inline(always)]
    pub const fn bootlock2(&self) -> u32 {
        (self.bits & Bootlock2::BOOTLOCK2_MASK) >> Bootlock2::BOOTLOCK2_OFFSET
    }

    /// Set BOOTLOCK2 [31:0]
    #[inline(always)]
    pub const fn with_bootlock2(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Bootlock2::BOOTLOCK2_MASK) | ((value << Bootlock2::BOOTLOCK2_OFFSET) & Bootlock2::BOOTLOCK2_MASK)
        }
    }

}

impl Default for Bootlock2Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Bootlock2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> Bootlock2Val {
        Bootlock2Val::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: Bootlock2Val) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(Bootlock2Val) -> Bootlock2Val>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// BOOTLOCK3 Register
///
/// Read to claim and check. Write to unclaim. The value returned on successful
/// claim is 1 << n, and on failed claim is zero.
///
/// Address offset: 0x0818
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **BOOTLOCK3** [31:0] (RW): 
#[repr(transparent)]
pub struct Bootlock3 {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for Bootlock3 {}
unsafe impl Sync for Bootlock3 {}

impl Bootlock3 {
    // ======== Field Constants ========

    /// Bits [31:0], mask 0xffffffff
    pub const BOOTLOCK3_OFFSET: u32 = 0;
    pub const BOOTLOCK3_MASK: u32 = 0xffffffff;

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

    /// **BOOTLOCK3** [31:0]
    #[inline(always)]
    pub fn bootlock3(&self) -> u32 {
        (self.read_bits() & Self::BOOTLOCK3_MASK) >> Self::BOOTLOCK3_OFFSET
    }

    /// Write BOOTLOCK3 field
    #[inline(always)]
    pub fn write_bootlock3(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::BOOTLOCK3_MASK) | ((value << Self::BOOTLOCK3_OFFSET) & Self::BOOTLOCK3_MASK));
    }

}

/// BOOTLOCK3 value type for batch read/modify/write operations
///
/// Use `bootlock3.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `bootlock3.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Bootlock3Val {
    bits: u32,
}

impl Bootlock3Val {
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

    /// Read BOOTLOCK3 [31:0]
    #[inline(always)]
    pub const fn bootlock3(&self) -> u32 {
        (self.bits & Bootlock3::BOOTLOCK3_MASK) >> Bootlock3::BOOTLOCK3_OFFSET
    }

    /// Set BOOTLOCK3 [31:0]
    #[inline(always)]
    pub const fn with_bootlock3(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Bootlock3::BOOTLOCK3_MASK) | ((value << Bootlock3::BOOTLOCK3_OFFSET) & Bootlock3::BOOTLOCK3_MASK)
        }
    }

}

impl Default for Bootlock3Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Bootlock3 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> Bootlock3Val {
        Bootlock3Val::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: Bootlock3Val) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(Bootlock3Val) -> Bootlock3Val>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// BOOTLOCK4 Register
///
/// Read to claim and check. Write to unclaim. The value returned on successful
/// claim is 1 << n, and on failed claim is zero.
///
/// Address offset: 0x081c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **BOOTLOCK4** [31:0] (RW): 
#[repr(transparent)]
pub struct Bootlock4 {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for Bootlock4 {}
unsafe impl Sync for Bootlock4 {}

impl Bootlock4 {
    // ======== Field Constants ========

    /// Bits [31:0], mask 0xffffffff
    pub const BOOTLOCK4_OFFSET: u32 = 0;
    pub const BOOTLOCK4_MASK: u32 = 0xffffffff;

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

    /// **BOOTLOCK4** [31:0]
    #[inline(always)]
    pub fn bootlock4(&self) -> u32 {
        (self.read_bits() & Self::BOOTLOCK4_MASK) >> Self::BOOTLOCK4_OFFSET
    }

    /// Write BOOTLOCK4 field
    #[inline(always)]
    pub fn write_bootlock4(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::BOOTLOCK4_MASK) | ((value << Self::BOOTLOCK4_OFFSET) & Self::BOOTLOCK4_MASK));
    }

}

/// BOOTLOCK4 value type for batch read/modify/write operations
///
/// Use `bootlock4.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `bootlock4.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Bootlock4Val {
    bits: u32,
}

impl Bootlock4Val {
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

    /// Read BOOTLOCK4 [31:0]
    #[inline(always)]
    pub const fn bootlock4(&self) -> u32 {
        (self.bits & Bootlock4::BOOTLOCK4_MASK) >> Bootlock4::BOOTLOCK4_OFFSET
    }

    /// Set BOOTLOCK4 [31:0]
    #[inline(always)]
    pub const fn with_bootlock4(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Bootlock4::BOOTLOCK4_MASK) | ((value << Bootlock4::BOOTLOCK4_OFFSET) & Bootlock4::BOOTLOCK4_MASK)
        }
    }

}

impl Default for Bootlock4Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Bootlock4 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> Bootlock4Val {
        Bootlock4Val::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: Bootlock4Val) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(Bootlock4Val) -> Bootlock4Val>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// BOOTLOCK5 Register
///
/// Read to claim and check. Write to unclaim. The value returned on successful
/// claim is 1 << n, and on failed claim is zero.
///
/// Address offset: 0x0820
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **BOOTLOCK5** [31:0] (RW): 
#[repr(transparent)]
pub struct Bootlock5 {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for Bootlock5 {}
unsafe impl Sync for Bootlock5 {}

impl Bootlock5 {
    // ======== Field Constants ========

    /// Bits [31:0], mask 0xffffffff
    pub const BOOTLOCK5_OFFSET: u32 = 0;
    pub const BOOTLOCK5_MASK: u32 = 0xffffffff;

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

    /// **BOOTLOCK5** [31:0]
    #[inline(always)]
    pub fn bootlock5(&self) -> u32 {
        (self.read_bits() & Self::BOOTLOCK5_MASK) >> Self::BOOTLOCK5_OFFSET
    }

    /// Write BOOTLOCK5 field
    #[inline(always)]
    pub fn write_bootlock5(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::BOOTLOCK5_MASK) | ((value << Self::BOOTLOCK5_OFFSET) & Self::BOOTLOCK5_MASK));
    }

}

/// BOOTLOCK5 value type for batch read/modify/write operations
///
/// Use `bootlock5.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `bootlock5.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Bootlock5Val {
    bits: u32,
}

impl Bootlock5Val {
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

    /// Read BOOTLOCK5 [31:0]
    #[inline(always)]
    pub const fn bootlock5(&self) -> u32 {
        (self.bits & Bootlock5::BOOTLOCK5_MASK) >> Bootlock5::BOOTLOCK5_OFFSET
    }

    /// Set BOOTLOCK5 [31:0]
    #[inline(always)]
    pub const fn with_bootlock5(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Bootlock5::BOOTLOCK5_MASK) | ((value << Bootlock5::BOOTLOCK5_OFFSET) & Bootlock5::BOOTLOCK5_MASK)
        }
    }

}

impl Default for Bootlock5Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Bootlock5 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> Bootlock5Val {
        Bootlock5Val::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: Bootlock5Val) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(Bootlock5Val) -> Bootlock5Val>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// BOOTLOCK6 Register
///
/// Read to claim and check. Write to unclaim. The value returned on successful
/// claim is 1 << n, and on failed claim is zero.
///
/// Address offset: 0x0824
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **BOOTLOCK6** [31:0] (RW): 
#[repr(transparent)]
pub struct Bootlock6 {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for Bootlock6 {}
unsafe impl Sync for Bootlock6 {}

impl Bootlock6 {
    // ======== Field Constants ========

    /// Bits [31:0], mask 0xffffffff
    pub const BOOTLOCK6_OFFSET: u32 = 0;
    pub const BOOTLOCK6_MASK: u32 = 0xffffffff;

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

    /// **BOOTLOCK6** [31:0]
    #[inline(always)]
    pub fn bootlock6(&self) -> u32 {
        (self.read_bits() & Self::BOOTLOCK6_MASK) >> Self::BOOTLOCK6_OFFSET
    }

    /// Write BOOTLOCK6 field
    #[inline(always)]
    pub fn write_bootlock6(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::BOOTLOCK6_MASK) | ((value << Self::BOOTLOCK6_OFFSET) & Self::BOOTLOCK6_MASK));
    }

}

/// BOOTLOCK6 value type for batch read/modify/write operations
///
/// Use `bootlock6.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `bootlock6.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Bootlock6Val {
    bits: u32,
}

impl Bootlock6Val {
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

    /// Read BOOTLOCK6 [31:0]
    #[inline(always)]
    pub const fn bootlock6(&self) -> u32 {
        (self.bits & Bootlock6::BOOTLOCK6_MASK) >> Bootlock6::BOOTLOCK6_OFFSET
    }

    /// Set BOOTLOCK6 [31:0]
    #[inline(always)]
    pub const fn with_bootlock6(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Bootlock6::BOOTLOCK6_MASK) | ((value << Bootlock6::BOOTLOCK6_OFFSET) & Bootlock6::BOOTLOCK6_MASK)
        }
    }

}

impl Default for Bootlock6Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Bootlock6 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> Bootlock6Val {
        Bootlock6Val::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: Bootlock6Val) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(Bootlock6Val) -> Bootlock6Val>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// BOOTLOCK7 Register
///
/// Read to claim and check. Write to unclaim. The value returned on successful
/// claim is 1 << n, and on failed claim is zero.
///
/// Address offset: 0x0828
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **BOOTLOCK7** [31:0] (RW): 
#[repr(transparent)]
pub struct Bootlock7 {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for Bootlock7 {}
unsafe impl Sync for Bootlock7 {}

impl Bootlock7 {
    // ======== Field Constants ========

    /// Bits [31:0], mask 0xffffffff
    pub const BOOTLOCK7_OFFSET: u32 = 0;
    pub const BOOTLOCK7_MASK: u32 = 0xffffffff;

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

    /// **BOOTLOCK7** [31:0]
    #[inline(always)]
    pub fn bootlock7(&self) -> u32 {
        (self.read_bits() & Self::BOOTLOCK7_MASK) >> Self::BOOTLOCK7_OFFSET
    }

    /// Write BOOTLOCK7 field
    #[inline(always)]
    pub fn write_bootlock7(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::BOOTLOCK7_MASK) | ((value << Self::BOOTLOCK7_OFFSET) & Self::BOOTLOCK7_MASK));
    }

}

/// BOOTLOCK7 value type for batch read/modify/write operations
///
/// Use `bootlock7.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `bootlock7.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Bootlock7Val {
    bits: u32,
}

impl Bootlock7Val {
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

    /// Read BOOTLOCK7 [31:0]
    #[inline(always)]
    pub const fn bootlock7(&self) -> u32 {
        (self.bits & Bootlock7::BOOTLOCK7_MASK) >> Bootlock7::BOOTLOCK7_OFFSET
    }

    /// Set BOOTLOCK7 [31:0]
    #[inline(always)]
    pub const fn with_bootlock7(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Bootlock7::BOOTLOCK7_MASK) | ((value << Bootlock7::BOOTLOCK7_OFFSET) & Bootlock7::BOOTLOCK7_MASK)
        }
    }

}

impl Default for Bootlock7Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Bootlock7 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> Bootlock7Val {
        Bootlock7Val::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: Bootlock7Val) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(Bootlock7Val) -> Bootlock7Val>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// Register block
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 2048],
    /// 0x0800 - This registers always ORs writes into its current contents. Once a bit is s...
    pub write_once0: WriteOnce0,
    /// 0x0804 - This registers always ORs writes into its current contents. Once a bit is s...
    pub write_once1: WriteOnce1,
    /// 0x0808 - Bootlock status register. 1=unclaimed, 0=claimed. These locks function iden...
    pub bootlock_stat: BootlockStat,
    /// 0x080c - Read to claim and check. Write to unclaim. The value returned on successful...
    pub bootlock0: Bootlock0,
    /// 0x0810 - Read to claim and check. Write to unclaim. The value returned on successful...
    pub bootlock1: Bootlock1,
    /// 0x0814 - Read to claim and check. Write to unclaim. The value returned on successful...
    pub bootlock2: Bootlock2,
    /// 0x0818 - Read to claim and check. Write to unclaim. The value returned on successful...
    pub bootlock3: Bootlock3,
    /// 0x081c - Read to claim and check. Write to unclaim. The value returned on successful...
    pub bootlock4: Bootlock4,
    /// 0x0820 - Read to claim and check. Write to unclaim. The value returned on successful...
    pub bootlock5: Bootlock5,
    /// 0x0824 - Read to claim and check. Write to unclaim. The value returned on successful...
    pub bootlock6: Bootlock6,
    /// 0x0828 - Read to claim and check. Write to unclaim. The value returned on successful...
    pub bootlock7: Bootlock7,
}

