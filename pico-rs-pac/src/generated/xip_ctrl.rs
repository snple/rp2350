//! XIP_CTRL - QSPI flash execute-in-place block

#![allow(dead_code)]

pub const BASE: u32 = 0x400c8000;

/// CTRL Register
///
/// Cache control register. Read-only from a Non-secure context.
///
/// Address offset: 0x0000
/// Reset value: 0x00000083
///
/// ## Fields
///
/// - **WRITABLE_M1** [11] (RW): If 1, enable writes to XIP memory window 1 (addresses 0x11000000 through 0x...
/// - **WRITABLE_M0** [10] (RW): If 1, enable writes to XIP memory window 0 (addresses 0x10000000 through 0x...
/// - **SPLIT_WAYS** [9] (RW): When 1, route all cached+Secure accesses to way 0 of the cache, and route a...
/// - **MAINT_NONSEC** [8] (RW): When 0, Non-secure accesses to the cache maintenance address window (addr[2...
/// - **NO_UNTRANSLATED_NONSEC** [7] (RW): When 1, Non-secure accesses to the uncached, untranslated window (addr[27:2...
/// - **NO_UNTRANSLATED_SEC** [6] (RW): When 1, Secure accesses to the uncached, untranslated window (addr[27:26] =...
/// - **NO_UNCACHED_NONSEC** [5] (RW): When 1, Non-secure accesses to the uncached window (addr[27:26] == 1) will ...
/// - **NO_UNCACHED_SEC** [4] (RW): When 1, Secure accesses to the uncached window (addr[27:26] == 1) will gene...
/// - **POWER_DOWN** [3] (RW): When 1, the cache memories are powered down. They retain state, but can not...
/// - **EN_NONSECURE** [1] (RW): When 1, enable the cache for Non-secure accesses. When enabled, Non-secure ...
/// - **EN_SECURE** [0] (RW): When 1, enable the cache for Secure accesses. When enabled, Secure XIP acce...
#[repr(transparent)]
pub struct Ctrl {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for Ctrl {}
unsafe impl Sync for Ctrl {}

impl Ctrl {
    // ======== Field Constants ========

    /// WRITABLE_M1 - If 1, enable writes to XIP memory window 1 (addresses 0x11000000 through 0x...
    /// Bits [11:11], mask 0x00000800
    pub const WRITABLE_M1_OFFSET: u32 = 11;
    pub const WRITABLE_M1_MASK: u32 = 0x00000800;
    pub const WRITABLE_M1: u32 = 1 << 11;

    /// WRITABLE_M0 - If 1, enable writes to XIP memory window 0 (addresses 0x10000000 through 0x...
    /// Bits [10:10], mask 0x00000400
    pub const WRITABLE_M0_OFFSET: u32 = 10;
    pub const WRITABLE_M0_MASK: u32 = 0x00000400;
    pub const WRITABLE_M0: u32 = 1 << 10;

    /// SPLIT_WAYS - When 1, route all cached+Secure accesses to way 0 of the cache, and route a...
    /// Bits [9:9], mask 0x00000200
    pub const SPLIT_WAYS_OFFSET: u32 = 9;
    pub const SPLIT_WAYS_MASK: u32 = 0x00000200;
    pub const SPLIT_WAYS: u32 = 1 << 9;

    /// MAINT_NONSEC - When 0, Non-secure accesses to the cache maintenance address window (addr[2...
    /// Bits [8:8], mask 0x00000100
    pub const MAINT_NONSEC_OFFSET: u32 = 8;
    pub const MAINT_NONSEC_MASK: u32 = 0x00000100;
    pub const MAINT_NONSEC: u32 = 1 << 8;

    /// NO_UNTRANSLATED_NONSEC - When 1, Non-secure accesses to the uncached, untranslated window (addr[27:2...
    /// Bits [7:7], mask 0x00000080
    pub const NO_UNTRANSLATED_NONSEC_OFFSET: u32 = 7;
    pub const NO_UNTRANSLATED_NONSEC_MASK: u32 = 0x00000080;
    pub const NO_UNTRANSLATED_NONSEC: u32 = 1 << 7;

    /// NO_UNTRANSLATED_SEC - When 1, Secure accesses to the uncached, untranslated window (addr[27:26] =...
    /// Bits [6:6], mask 0x00000040
    pub const NO_UNTRANSLATED_SEC_OFFSET: u32 = 6;
    pub const NO_UNTRANSLATED_SEC_MASK: u32 = 0x00000040;
    pub const NO_UNTRANSLATED_SEC: u32 = 1 << 6;

    /// NO_UNCACHED_NONSEC - When 1, Non-secure accesses to the uncached window (addr[27:26] == 1) will ...
    /// Bits [5:5], mask 0x00000020
    pub const NO_UNCACHED_NONSEC_OFFSET: u32 = 5;
    pub const NO_UNCACHED_NONSEC_MASK: u32 = 0x00000020;
    pub const NO_UNCACHED_NONSEC: u32 = 1 << 5;

    /// NO_UNCACHED_SEC - When 1, Secure accesses to the uncached window (addr[27:26] == 1) will gene...
    /// Bits [4:4], mask 0x00000010
    pub const NO_UNCACHED_SEC_OFFSET: u32 = 4;
    pub const NO_UNCACHED_SEC_MASK: u32 = 0x00000010;
    pub const NO_UNCACHED_SEC: u32 = 1 << 4;

    /// POWER_DOWN - When 1, the cache memories are powered down. They retain state, but can not...
    /// Bits [3:3], mask 0x00000008
    pub const POWER_DOWN_OFFSET: u32 = 3;
    pub const POWER_DOWN_MASK: u32 = 0x00000008;
    pub const POWER_DOWN: u32 = 1 << 3;

    /// EN_NONSECURE - When 1, enable the cache for Non-secure accesses. When enabled, Non-secure ...
    /// Bits [1:1], mask 0x00000002
    pub const EN_NONSECURE_OFFSET: u32 = 1;
    pub const EN_NONSECURE_MASK: u32 = 0x00000002;
    pub const EN_NONSECURE: u32 = 1 << 1;

    /// EN_SECURE - When 1, enable the cache for Secure accesses. When enabled, Secure XIP acce...
    /// Bits [0:0], mask 0x00000001
    pub const EN_SECURE_OFFSET: u32 = 0;
    pub const EN_SECURE_MASK: u32 = 0x00000001;
    pub const EN_SECURE: u32 = 1 << 0;

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

    /// **WRITABLE_M1** [11:11]: If 1, enable writes to XIP memory window 1 (addresses 0x11000000 through 0x...
    #[inline(always)]
    pub fn writable_m1(&self) -> bool {
        (self.read_bits() & Self::WRITABLE_M1_MASK) != 0
    }

    /// Set WRITABLE_M1 bit
    #[inline(always)]
    pub fn set_writable_m1(&self) {
        self.set_bits(Self::WRITABLE_M1_MASK);
    }

    /// Clear WRITABLE_M1 bit
    #[inline(always)]
    pub fn clr_writable_m1(&self) {
        self.clr_bits(Self::WRITABLE_M1_MASK);
    }

    /// **WRITABLE_M0** [10:10]: If 1, enable writes to XIP memory window 0 (addresses 0x10000000 through 0x...
    #[inline(always)]
    pub fn writable_m0(&self) -> bool {
        (self.read_bits() & Self::WRITABLE_M0_MASK) != 0
    }

    /// Set WRITABLE_M0 bit
    #[inline(always)]
    pub fn set_writable_m0(&self) {
        self.set_bits(Self::WRITABLE_M0_MASK);
    }

    /// Clear WRITABLE_M0 bit
    #[inline(always)]
    pub fn clr_writable_m0(&self) {
        self.clr_bits(Self::WRITABLE_M0_MASK);
    }

    /// **SPLIT_WAYS** [9:9]: When 1, route all cached+Secure accesses to way 0 of the cache, and route a...
    #[inline(always)]
    pub fn split_ways(&self) -> bool {
        (self.read_bits() & Self::SPLIT_WAYS_MASK) != 0
    }

    /// Set SPLIT_WAYS bit
    #[inline(always)]
    pub fn set_split_ways(&self) {
        self.set_bits(Self::SPLIT_WAYS_MASK);
    }

    /// Clear SPLIT_WAYS bit
    #[inline(always)]
    pub fn clr_split_ways(&self) {
        self.clr_bits(Self::SPLIT_WAYS_MASK);
    }

    /// **MAINT_NONSEC** [8:8]: When 0, Non-secure accesses to the cache maintenance address window (addr[2...
    #[inline(always)]
    pub fn maint_nonsec(&self) -> bool {
        (self.read_bits() & Self::MAINT_NONSEC_MASK) != 0
    }

    /// Set MAINT_NONSEC bit
    #[inline(always)]
    pub fn set_maint_nonsec(&self) {
        self.set_bits(Self::MAINT_NONSEC_MASK);
    }

    /// Clear MAINT_NONSEC bit
    #[inline(always)]
    pub fn clr_maint_nonsec(&self) {
        self.clr_bits(Self::MAINT_NONSEC_MASK);
    }

    /// **NO_UNTRANSLATED_NONSEC** [7:7]: When 1, Non-secure accesses to the uncached, untranslated window (addr[27:2...
    #[inline(always)]
    pub fn no_untranslated_nonsec(&self) -> bool {
        (self.read_bits() & Self::NO_UNTRANSLATED_NONSEC_MASK) != 0
    }

    /// Set NO_UNTRANSLATED_NONSEC bit
    #[inline(always)]
    pub fn set_no_untranslated_nonsec(&self) {
        self.set_bits(Self::NO_UNTRANSLATED_NONSEC_MASK);
    }

    /// Clear NO_UNTRANSLATED_NONSEC bit
    #[inline(always)]
    pub fn clr_no_untranslated_nonsec(&self) {
        self.clr_bits(Self::NO_UNTRANSLATED_NONSEC_MASK);
    }

    /// **NO_UNTRANSLATED_SEC** [6:6]: When 1, Secure accesses to the uncached, untranslated window (addr[27:26] =...
    #[inline(always)]
    pub fn no_untranslated_sec(&self) -> bool {
        (self.read_bits() & Self::NO_UNTRANSLATED_SEC_MASK) != 0
    }

    /// Set NO_UNTRANSLATED_SEC bit
    #[inline(always)]
    pub fn set_no_untranslated_sec(&self) {
        self.set_bits(Self::NO_UNTRANSLATED_SEC_MASK);
    }

    /// Clear NO_UNTRANSLATED_SEC bit
    #[inline(always)]
    pub fn clr_no_untranslated_sec(&self) {
        self.clr_bits(Self::NO_UNTRANSLATED_SEC_MASK);
    }

    /// **NO_UNCACHED_NONSEC** [5:5]: When 1, Non-secure accesses to the uncached window (addr[27:26] == 1) will ...
    #[inline(always)]
    pub fn no_uncached_nonsec(&self) -> bool {
        (self.read_bits() & Self::NO_UNCACHED_NONSEC_MASK) != 0
    }

    /// Set NO_UNCACHED_NONSEC bit
    #[inline(always)]
    pub fn set_no_uncached_nonsec(&self) {
        self.set_bits(Self::NO_UNCACHED_NONSEC_MASK);
    }

    /// Clear NO_UNCACHED_NONSEC bit
    #[inline(always)]
    pub fn clr_no_uncached_nonsec(&self) {
        self.clr_bits(Self::NO_UNCACHED_NONSEC_MASK);
    }

    /// **NO_UNCACHED_SEC** [4:4]: When 1, Secure accesses to the uncached window (addr[27:26] == 1) will gene...
    #[inline(always)]
    pub fn no_uncached_sec(&self) -> bool {
        (self.read_bits() & Self::NO_UNCACHED_SEC_MASK) != 0
    }

    /// Set NO_UNCACHED_SEC bit
    #[inline(always)]
    pub fn set_no_uncached_sec(&self) {
        self.set_bits(Self::NO_UNCACHED_SEC_MASK);
    }

    /// Clear NO_UNCACHED_SEC bit
    #[inline(always)]
    pub fn clr_no_uncached_sec(&self) {
        self.clr_bits(Self::NO_UNCACHED_SEC_MASK);
    }

    /// **POWER_DOWN** [3:3]: When 1, the cache memories are powered down. They retain state, but can not...
    #[inline(always)]
    pub fn power_down(&self) -> bool {
        (self.read_bits() & Self::POWER_DOWN_MASK) != 0
    }

    /// Set POWER_DOWN bit
    #[inline(always)]
    pub fn set_power_down(&self) {
        self.set_bits(Self::POWER_DOWN_MASK);
    }

    /// Clear POWER_DOWN bit
    #[inline(always)]
    pub fn clr_power_down(&self) {
        self.clr_bits(Self::POWER_DOWN_MASK);
    }

    /// **EN_NONSECURE** [1:1]: When 1, enable the cache for Non-secure accesses. When enabled, Non-secure ...
    #[inline(always)]
    pub fn en_nonsecure(&self) -> bool {
        (self.read_bits() & Self::EN_NONSECURE_MASK) != 0
    }

    /// Set EN_NONSECURE bit
    #[inline(always)]
    pub fn set_en_nonsecure(&self) {
        self.set_bits(Self::EN_NONSECURE_MASK);
    }

    /// Clear EN_NONSECURE bit
    #[inline(always)]
    pub fn clr_en_nonsecure(&self) {
        self.clr_bits(Self::EN_NONSECURE_MASK);
    }

    /// **EN_SECURE** [0:0]: When 1, enable the cache for Secure accesses. When enabled, Secure XIP acce...
    #[inline(always)]
    pub fn en_secure(&self) -> bool {
        (self.read_bits() & Self::EN_SECURE_MASK) != 0
    }

    /// Set EN_SECURE bit
    #[inline(always)]
    pub fn set_en_secure(&self) {
        self.set_bits(Self::EN_SECURE_MASK);
    }

    /// Clear EN_SECURE bit
    #[inline(always)]
    pub fn clr_en_secure(&self) {
        self.clr_bits(Self::EN_SECURE_MASK);
    }

}

/// CTRL value type for batch read/modify/write operations
///
/// Use `ctrl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ctrl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CtrlVal {
    bits: u32,
}

impl CtrlVal {
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
        Self { bits: 0x00000083 }
    }

    /// Read WRITABLE_M1 [11:11]: If 1, enable writes to XIP memory window 1 (addresses 0x11000000 through 0x...
    #[inline(always)]
    pub const fn writable_m1(&self) -> bool {
        (self.bits & Ctrl::WRITABLE_M1_MASK) != 0
    }

    /// Read WRITABLE_M0 [10:10]: If 1, enable writes to XIP memory window 0 (addresses 0x10000000 through 0x...
    #[inline(always)]
    pub const fn writable_m0(&self) -> bool {
        (self.bits & Ctrl::WRITABLE_M0_MASK) != 0
    }

    /// Read SPLIT_WAYS [9:9]: When 1, route all cached+Secure accesses to way 0 of the cache, and route a...
    #[inline(always)]
    pub const fn split_ways(&self) -> bool {
        (self.bits & Ctrl::SPLIT_WAYS_MASK) != 0
    }

    /// Read MAINT_NONSEC [8:8]: When 0, Non-secure accesses to the cache maintenance address window (addr[2...
    #[inline(always)]
    pub const fn maint_nonsec(&self) -> bool {
        (self.bits & Ctrl::MAINT_NONSEC_MASK) != 0
    }

    /// Read NO_UNTRANSLATED_NONSEC [7:7]: When 1, Non-secure accesses to the uncached, untranslated window (addr[27:2...
    #[inline(always)]
    pub const fn no_untranslated_nonsec(&self) -> bool {
        (self.bits & Ctrl::NO_UNTRANSLATED_NONSEC_MASK) != 0
    }

    /// Read NO_UNTRANSLATED_SEC [6:6]: When 1, Secure accesses to the uncached, untranslated window (addr[27:26] =...
    #[inline(always)]
    pub const fn no_untranslated_sec(&self) -> bool {
        (self.bits & Ctrl::NO_UNTRANSLATED_SEC_MASK) != 0
    }

    /// Read NO_UNCACHED_NONSEC [5:5]: When 1, Non-secure accesses to the uncached window (addr[27:26] == 1) will ...
    #[inline(always)]
    pub const fn no_uncached_nonsec(&self) -> bool {
        (self.bits & Ctrl::NO_UNCACHED_NONSEC_MASK) != 0
    }

    /// Read NO_UNCACHED_SEC [4:4]: When 1, Secure accesses to the uncached window (addr[27:26] == 1) will gene...
    #[inline(always)]
    pub const fn no_uncached_sec(&self) -> bool {
        (self.bits & Ctrl::NO_UNCACHED_SEC_MASK) != 0
    }

    /// Read POWER_DOWN [3:3]: When 1, the cache memories are powered down. They retain state, but can not...
    #[inline(always)]
    pub const fn power_down(&self) -> bool {
        (self.bits & Ctrl::POWER_DOWN_MASK) != 0
    }

    /// Read EN_NONSECURE [1:1]: When 1, enable the cache for Non-secure accesses. When enabled, Non-secure ...
    #[inline(always)]
    pub const fn en_nonsecure(&self) -> bool {
        (self.bits & Ctrl::EN_NONSECURE_MASK) != 0
    }

    /// Read EN_SECURE [0:0]: When 1, enable the cache for Secure accesses. When enabled, Secure XIP acce...
    #[inline(always)]
    pub const fn en_secure(&self) -> bool {
        (self.bits & Ctrl::EN_SECURE_MASK) != 0
    }

    /// Set WRITABLE_M1 [11:11]: If 1, enable writes to XIP memory window 1 (addresses 0x11000000 through 0x...
    #[inline(always)]
    pub const fn with_writable_m1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctrl::WRITABLE_M1_MASK
            } else {
                self.bits & !Ctrl::WRITABLE_M1_MASK
            }
        }
    }

    /// Set WRITABLE_M0 [10:10]: If 1, enable writes to XIP memory window 0 (addresses 0x10000000 through 0x...
    #[inline(always)]
    pub const fn with_writable_m0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctrl::WRITABLE_M0_MASK
            } else {
                self.bits & !Ctrl::WRITABLE_M0_MASK
            }
        }
    }

    /// Set SPLIT_WAYS [9:9]: When 1, route all cached+Secure accesses to way 0 of the cache, and route a...
    #[inline(always)]
    pub const fn with_split_ways(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctrl::SPLIT_WAYS_MASK
            } else {
                self.bits & !Ctrl::SPLIT_WAYS_MASK
            }
        }
    }

    /// Set MAINT_NONSEC [8:8]: When 0, Non-secure accesses to the cache maintenance address window (addr[2...
    #[inline(always)]
    pub const fn with_maint_nonsec(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctrl::MAINT_NONSEC_MASK
            } else {
                self.bits & !Ctrl::MAINT_NONSEC_MASK
            }
        }
    }

    /// Set NO_UNTRANSLATED_NONSEC [7:7]: When 1, Non-secure accesses to the uncached, untranslated window (addr[27:2...
    #[inline(always)]
    pub const fn with_no_untranslated_nonsec(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctrl::NO_UNTRANSLATED_NONSEC_MASK
            } else {
                self.bits & !Ctrl::NO_UNTRANSLATED_NONSEC_MASK
            }
        }
    }

    /// Set NO_UNTRANSLATED_SEC [6:6]: When 1, Secure accesses to the uncached, untranslated window (addr[27:26] =...
    #[inline(always)]
    pub const fn with_no_untranslated_sec(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctrl::NO_UNTRANSLATED_SEC_MASK
            } else {
                self.bits & !Ctrl::NO_UNTRANSLATED_SEC_MASK
            }
        }
    }

    /// Set NO_UNCACHED_NONSEC [5:5]: When 1, Non-secure accesses to the uncached window (addr[27:26] == 1) will ...
    #[inline(always)]
    pub const fn with_no_uncached_nonsec(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctrl::NO_UNCACHED_NONSEC_MASK
            } else {
                self.bits & !Ctrl::NO_UNCACHED_NONSEC_MASK
            }
        }
    }

    /// Set NO_UNCACHED_SEC [4:4]: When 1, Secure accesses to the uncached window (addr[27:26] == 1) will gene...
    #[inline(always)]
    pub const fn with_no_uncached_sec(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctrl::NO_UNCACHED_SEC_MASK
            } else {
                self.bits & !Ctrl::NO_UNCACHED_SEC_MASK
            }
        }
    }

    /// Set POWER_DOWN [3:3]: When 1, the cache memories are powered down. They retain state, but can not...
    #[inline(always)]
    pub const fn with_power_down(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctrl::POWER_DOWN_MASK
            } else {
                self.bits & !Ctrl::POWER_DOWN_MASK
            }
        }
    }

    /// Set EN_NONSECURE [1:1]: When 1, enable the cache for Non-secure accesses. When enabled, Non-secure ...
    #[inline(always)]
    pub const fn with_en_nonsecure(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctrl::EN_NONSECURE_MASK
            } else {
                self.bits & !Ctrl::EN_NONSECURE_MASK
            }
        }
    }

    /// Set EN_SECURE [0:0]: When 1, enable the cache for Secure accesses. When enabled, Secure XIP acce...
    #[inline(always)]
    pub const fn with_en_secure(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctrl::EN_SECURE_MASK
            } else {
                self.bits & !Ctrl::EN_SECURE_MASK
            }
        }
    }

}

impl Default for CtrlVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Ctrl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> CtrlVal {
        CtrlVal::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: CtrlVal) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(CtrlVal) -> CtrlVal>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// STAT Register
///
/// Address offset: 0x0008
/// Reset value: 0x00000002
///
/// ## Fields
///
/// - **FIFO_FULL** [2] (RO): When 1, indicates the XIP streaming FIFO is completely full. The streaming ...
/// - **FIFO_EMPTY** [1] (RO): When 1, indicates the XIP streaming FIFO is completely empty.
#[repr(transparent)]
pub struct Stat {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for Stat {}
unsafe impl Sync for Stat {}

impl Stat {
    // ======== Field Constants ========

    /// FIFO_FULL - When 1, indicates the XIP streaming FIFO is completely full. The streaming ...
    /// Bits [2:2], mask 0x00000004
    pub const FIFO_FULL_OFFSET: u32 = 2;
    pub const FIFO_FULL_MASK: u32 = 0x00000004;
    pub const FIFO_FULL: u32 = 1 << 2;

    /// FIFO_EMPTY - When 1, indicates the XIP streaming FIFO is completely empty.
    /// Bits [1:1], mask 0x00000002
    pub const FIFO_EMPTY_OFFSET: u32 = 1;
    pub const FIFO_EMPTY_MASK: u32 = 0x00000002;
    pub const FIFO_EMPTY: u32 = 1 << 1;

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

    /// **FIFO_FULL** [2:2]: When 1, indicates the XIP streaming FIFO is completely full. The streaming ...
    #[inline(always)]
    pub fn fifo_full(&self) -> bool {
        (self.read_bits() & Self::FIFO_FULL_MASK) != 0
    }

    /// **FIFO_EMPTY** [1:1]: When 1, indicates the XIP streaming FIFO is completely empty.
    #[inline(always)]
    pub fn fifo_empty(&self) -> bool {
        (self.read_bits() & Self::FIFO_EMPTY_MASK) != 0
    }

}

/// STAT value type for batch read/modify/write operations
///
/// Use `stat.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `stat.write_val(val)`.
#[derive(Clone, Copy)]
pub struct StatVal {
    bits: u32,
}

impl StatVal {
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
        Self { bits: 0x00000002 }
    }

    /// Read FIFO_FULL [2:2]: When 1, indicates the XIP streaming FIFO is completely full. The streaming ...
    #[inline(always)]
    pub const fn fifo_full(&self) -> bool {
        (self.bits & Stat::FIFO_FULL_MASK) != 0
    }

    /// Read FIFO_EMPTY [1:1]: When 1, indicates the XIP streaming FIFO is completely empty.
    #[inline(always)]
    pub const fn fifo_empty(&self) -> bool {
        (self.bits & Stat::FIFO_EMPTY_MASK) != 0
    }

}

impl Default for StatVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Stat {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> StatVal {
        StatVal::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: StatVal) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(StatVal) -> StatVal>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// CTR_HIT Register
///
/// Cache Hit counter
///
/// Address offset: 0x000c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **CTR_HIT** [31:0] (RW): A 32 bit saturating counter that increments upon each cache hit, i.e. when ...
#[repr(transparent)]
pub struct CtrHit {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for CtrHit {}
unsafe impl Sync for CtrHit {}

impl CtrHit {
    // ======== Field Constants ========

    /// CTR_HIT - A 32 bit saturating counter that increments upon each cache hit, i.e. when ...
    /// Bits [31:0], mask 0xffffffff
    pub const CTR_HIT_OFFSET: u32 = 0;
    pub const CTR_HIT_MASK: u32 = 0xffffffff;

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

    /// **CTR_HIT** [31:0]: A 32 bit saturating counter that increments upon each cache hit, i.e. when ...
    #[inline(always)]
    pub fn ctr_hit(&self) -> u32 {
        (self.read_bits() & Self::CTR_HIT_MASK) >> Self::CTR_HIT_OFFSET
    }

    /// Write CTR_HIT field
    #[inline(always)]
    pub fn write_ctr_hit(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::CTR_HIT_MASK) | ((value << Self::CTR_HIT_OFFSET) & Self::CTR_HIT_MASK));
    }

}

/// CTR_HIT value type for batch read/modify/write operations
///
/// Use `ctrhit.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ctrhit.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CtrHitVal {
    bits: u32,
}

impl CtrHitVal {
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

    /// Read CTR_HIT [31:0]: A 32 bit saturating counter that increments upon each cache hit, i.e. when ...
    #[inline(always)]
    pub const fn ctr_hit(&self) -> u32 {
        (self.bits & CtrHit::CTR_HIT_MASK) >> CtrHit::CTR_HIT_OFFSET
    }

    /// Set CTR_HIT [31:0]: A 32 bit saturating counter that increments upon each cache hit, i.e. when ...
    #[inline(always)]
    pub const fn with_ctr_hit(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !CtrHit::CTR_HIT_MASK) | ((value << CtrHit::CTR_HIT_OFFSET) & CtrHit::CTR_HIT_MASK)
        }
    }

}

impl Default for CtrHitVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl CtrHit {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> CtrHitVal {
        CtrHitVal::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: CtrHitVal) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(CtrHitVal) -> CtrHitVal>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// CTR_ACC Register
///
/// Cache Access counter
///
/// Address offset: 0x0010
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **CTR_ACC** [31:0] (RW): A 32 bit saturating counter that increments upon each XIP access, whether t...
#[repr(transparent)]
pub struct CtrAcc {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for CtrAcc {}
unsafe impl Sync for CtrAcc {}

impl CtrAcc {
    // ======== Field Constants ========

    /// CTR_ACC - A 32 bit saturating counter that increments upon each XIP access, whether t...
    /// Bits [31:0], mask 0xffffffff
    pub const CTR_ACC_OFFSET: u32 = 0;
    pub const CTR_ACC_MASK: u32 = 0xffffffff;

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

    /// **CTR_ACC** [31:0]: A 32 bit saturating counter that increments upon each XIP access, whether t...
    #[inline(always)]
    pub fn ctr_acc(&self) -> u32 {
        (self.read_bits() & Self::CTR_ACC_MASK) >> Self::CTR_ACC_OFFSET
    }

    /// Write CTR_ACC field
    #[inline(always)]
    pub fn write_ctr_acc(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::CTR_ACC_MASK) | ((value << Self::CTR_ACC_OFFSET) & Self::CTR_ACC_MASK));
    }

}

/// CTR_ACC value type for batch read/modify/write operations
///
/// Use `ctracc.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ctracc.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CtrAccVal {
    bits: u32,
}

impl CtrAccVal {
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

    /// Read CTR_ACC [31:0]: A 32 bit saturating counter that increments upon each XIP access, whether t...
    #[inline(always)]
    pub const fn ctr_acc(&self) -> u32 {
        (self.bits & CtrAcc::CTR_ACC_MASK) >> CtrAcc::CTR_ACC_OFFSET
    }

    /// Set CTR_ACC [31:0]: A 32 bit saturating counter that increments upon each XIP access, whether t...
    #[inline(always)]
    pub const fn with_ctr_acc(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !CtrAcc::CTR_ACC_MASK) | ((value << CtrAcc::CTR_ACC_OFFSET) & CtrAcc::CTR_ACC_MASK)
        }
    }

}

impl Default for CtrAccVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl CtrAcc {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> CtrAccVal {
        CtrAccVal::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: CtrAccVal) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(CtrAccVal) -> CtrAccVal>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// STREAM_ADDR Register
///
/// FIFO stream address
///
/// Address offset: 0x0014
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **STREAM_ADDR** [31:2] (RW): The address of the next word to be streamed from flash to the streaming FIF...
#[repr(transparent)]
pub struct StreamAddr {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for StreamAddr {}
unsafe impl Sync for StreamAddr {}

impl StreamAddr {
    // ======== Field Constants ========

    /// STREAM_ADDR - The address of the next word to be streamed from flash to the streaming FIF...
    /// Bits [31:2], mask 0xfffffffc
    pub const STREAM_ADDR_OFFSET: u32 = 2;
    pub const STREAM_ADDR_MASK: u32 = 0xfffffffc;

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

    /// **STREAM_ADDR** [31:2]: The address of the next word to be streamed from flash to the streaming FIF...
    #[inline(always)]
    pub fn stream_addr(&self) -> u32 {
        (self.read_bits() & Self::STREAM_ADDR_MASK) >> Self::STREAM_ADDR_OFFSET
    }

    /// Write STREAM_ADDR field
    #[inline(always)]
    pub fn write_stream_addr(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::STREAM_ADDR_MASK) | ((value << Self::STREAM_ADDR_OFFSET) & Self::STREAM_ADDR_MASK));
    }

}

/// STREAM_ADDR value type for batch read/modify/write operations
///
/// Use `streamaddr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `streamaddr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct StreamAddrVal {
    bits: u32,
}

impl StreamAddrVal {
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

    /// Read STREAM_ADDR [31:2]: The address of the next word to be streamed from flash to the streaming FIF...
    #[inline(always)]
    pub const fn stream_addr(&self) -> u32 {
        (self.bits & StreamAddr::STREAM_ADDR_MASK) >> StreamAddr::STREAM_ADDR_OFFSET
    }

    /// Set STREAM_ADDR [31:2]: The address of the next word to be streamed from flash to the streaming FIF...
    #[inline(always)]
    pub const fn with_stream_addr(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !StreamAddr::STREAM_ADDR_MASK) | ((value << StreamAddr::STREAM_ADDR_OFFSET) & StreamAddr::STREAM_ADDR_MASK)
        }
    }

}

impl Default for StreamAddrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl StreamAddr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> StreamAddrVal {
        StreamAddrVal::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: StreamAddrVal) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(StreamAddrVal) -> StreamAddrVal>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// STREAM_CTR Register
///
/// FIFO stream control
///
/// Address offset: 0x0018
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **STREAM_CTR** [21:0] (RW): Write a nonzero value to start a streaming read. This will then progress in...
#[repr(transparent)]
pub struct StreamCtr {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for StreamCtr {}
unsafe impl Sync for StreamCtr {}

impl StreamCtr {
    // ======== Field Constants ========

    /// STREAM_CTR - Write a nonzero value to start a streaming read. This will then progress in...
    /// Bits [21:0], mask 0x003fffff
    pub const STREAM_CTR_OFFSET: u32 = 0;
    pub const STREAM_CTR_MASK: u32 = 0x003fffff;

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

    /// **STREAM_CTR** [21:0]: Write a nonzero value to start a streaming read. This will then progress in...
    #[inline(always)]
    pub fn stream_ctr(&self) -> u32 {
        (self.read_bits() & Self::STREAM_CTR_MASK) >> Self::STREAM_CTR_OFFSET
    }

    /// Write STREAM_CTR field
    #[inline(always)]
    pub fn write_stream_ctr(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::STREAM_CTR_MASK) | ((value << Self::STREAM_CTR_OFFSET) & Self::STREAM_CTR_MASK));
    }

}

/// STREAM_CTR value type for batch read/modify/write operations
///
/// Use `streamctr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `streamctr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct StreamCtrVal {
    bits: u32,
}

impl StreamCtrVal {
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

    /// Read STREAM_CTR [21:0]: Write a nonzero value to start a streaming read. This will then progress in...
    #[inline(always)]
    pub const fn stream_ctr(&self) -> u32 {
        (self.bits & StreamCtr::STREAM_CTR_MASK) >> StreamCtr::STREAM_CTR_OFFSET
    }

    /// Set STREAM_CTR [21:0]: Write a nonzero value to start a streaming read. This will then progress in...
    #[inline(always)]
    pub const fn with_stream_ctr(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !StreamCtr::STREAM_CTR_MASK) | ((value << StreamCtr::STREAM_CTR_OFFSET) & StreamCtr::STREAM_CTR_MASK)
        }
    }

}

impl Default for StreamCtrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl StreamCtr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> StreamCtrVal {
        StreamCtrVal::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: StreamCtrVal) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(StreamCtrVal) -> StreamCtrVal>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// STREAM_FIFO Register
///
/// FIFO stream data
///
/// Address offset: 0x001c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **STREAM_FIFO** [31:0] (RO): Streamed data is buffered here, for retrieval by the system DMA. This FIFO ...
#[repr(transparent)]
pub struct StreamFifo {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for StreamFifo {}
unsafe impl Sync for StreamFifo {}

impl StreamFifo {
    // ======== Field Constants ========

    /// STREAM_FIFO - Streamed data is buffered here, for retrieval by the system DMA. This FIFO ...
    /// Bits [31:0], mask 0xffffffff
    pub const STREAM_FIFO_OFFSET: u32 = 0;
    pub const STREAM_FIFO_MASK: u32 = 0xffffffff;

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

    /// **STREAM_FIFO** [31:0]: Streamed data is buffered here, for retrieval by the system DMA. This FIFO ...
    #[inline(always)]
    pub fn stream_fifo(&self) -> u32 {
        (self.read_bits() & Self::STREAM_FIFO_MASK) >> Self::STREAM_FIFO_OFFSET
    }

}

/// STREAM_FIFO value type for batch read/modify/write operations
///
/// Use `streamfifo.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `streamfifo.write_val(val)`.
#[derive(Clone, Copy)]
pub struct StreamFifoVal {
    bits: u32,
}

impl StreamFifoVal {
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

    /// Read STREAM_FIFO [31:0]: Streamed data is buffered here, for retrieval by the system DMA. This FIFO ...
    #[inline(always)]
    pub const fn stream_fifo(&self) -> u32 {
        (self.bits & StreamFifo::STREAM_FIFO_MASK) >> StreamFifo::STREAM_FIFO_OFFSET
    }

}

impl Default for StreamFifoVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl StreamFifo {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> StreamFifoVal {
        StreamFifoVal::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: StreamFifoVal) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(StreamFifoVal) -> StreamFifoVal>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// Register block
#[repr(C)]
pub struct RegisterBlock {
    /// 0x0000 - Cache control register. Read-only from a Non-secure context.
    pub ctrl: Ctrl,
    _reserved0: [u8; 4],
    /// 0x0008
    pub stat: Stat,
    /// 0x000c - Cache Hit counter
    pub ctr_hit: CtrHit,
    /// 0x0010 - Cache Access counter
    pub ctr_acc: CtrAcc,
    /// 0x0014 - FIFO stream address
    pub stream_addr: StreamAddr,
    /// 0x0018 - FIFO stream control
    pub stream_ctr: StreamCtr,
    /// 0x001c - FIFO stream data
    pub stream_fifo: StreamFifo,
}

