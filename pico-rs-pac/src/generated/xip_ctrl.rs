//! XIP_CTRL
//! QSPI flash execute-in-place block
//!
//! This peripheral supports atomic SET/CLR/XOR register aliases.

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
/// - **WRITABLE_M1** `[11]` (RW)
///
///   If 1, enable writes to XIP memory window 1 (addresses 0x11000000 through 0x11ffffff, and their uncached mirrors). If 0, this region is read-only.
///   XIP memory is *read-only by default*. This bit must be set to enable writes if a RAM device is attached on QSPI chip select 1.
///   The default read-only behaviour avoids two issues with writing to a read-only QSPI device (e.g. flash). First, a write will initially appear to succeed due to caching, but the data will eventually be lost when the written line is evicted, causing unpredictable behaviour.
///   Second, when a written line is evicted, it will cause a write command to be issued to the flash, which can break the flash out of its continuous read mode. After this point, flash reads will return garbage. This is a security concern, as it allows Non-secure software to break Secure flash reads if it has permission to write to any flash address.
///   Note the read-only behaviour is implemented by downgrading writes to reads, so writes will still cause allocation of an address, but have no other effect.
///
/// - **WRITABLE_M0** `[10]` (RW)
///
///   If 1, enable writes to XIP memory window 0 (addresses 0x10000000 through 0x10ffffff, and their uncached mirrors). If 0, this region is read-only.
///   XIP memory is *read-only by default*. This bit must be set to enable writes if a RAM device is attached on QSPI chip select 0.
///   The default read-only behaviour avoids two issues with writing to a read-only QSPI device (e.g. flash). First, a write will initially appear to succeed due to caching, but the data will eventually be lost when the written line is evicted, causing unpredictable behaviour.
///   Second, when a written line is evicted, it will cause a write command to be issued to the flash, which can break the flash out of its continuous read mode. After this point, flash reads will return garbage. This is a security concern, as it allows Non-secure software to break Secure flash reads if it has permission to write to any flash address.
///   Note the read-only behaviour is implemented by downgrading writes to reads, so writes will still cause allocation of an address, but have no other effect.
///
/// - **SPLIT_WAYS** `[9]` (RW)
///
///   When 1, route all cached+Secure accesses to way 0 of the cache, and route all cached+Non-secure accesses to way 1 of the cache.
///   This partitions the cache into two half-sized direct-mapped regions, such that Non-secure code can not observe cache line state changes caused by Secure execution.
///   A full cache flush is required when changing the value of SPLIT_WAYS. The flush should be performed whilst SPLIT_WAYS is 0, so that both cache ways are accessible for invalidation.
///
/// - **MAINT_NONSEC** `[8]` (RW)
///
///   When 0, Non-secure accesses to the cache maintenance address window (addr[27] == 1, addr[26] == 0) will generate a bus error. When 1, Non-secure accesses can perform cache maintenance operations by writing to the cache maintenance address window.
///   Cache maintenance operations may be used to corrupt Secure data by invalidating cache lines inappropriately, or map Secure content into a Non-secure region by pinning cache lines. Therefore this bit should generally be set to 0, unless Secure code is not using the cache.
///   Care should also be taken to clear the cache data memory and tag memory before granting maintenance operations to Non-secure code.
///
/// - **NO_UNTRANSLATED_NONSEC** `[7]` (RW)
///
///   When 1, Non-secure accesses to the uncached, untranslated window (addr[27:26] == 3) will generate a bus error.
///
/// - **NO_UNTRANSLATED_SEC** `[6]` (RW)
///
///   When 1, Secure accesses to the uncached, untranslated window (addr[27:26] == 3) will generate a bus error.
///
/// - **NO_UNCACHED_NONSEC** `[5]` (RW)
///
///   When 1, Non-secure accesses to the uncached window (addr[27:26] == 1) will generate a bus error. This may reduce the number of SAU/MPU/PMP regions required to protect flash contents.
///   Note this does not disable access to the uncached, untranslated window -- see NO_UNTRANSLATED_SEC.
///
/// - **NO_UNCACHED_SEC** `[4]` (RW)
///
///   When 1, Secure accesses to the uncached window (addr[27:26] == 1) will generate a bus error. This may reduce the number of SAU/MPU/PMP regions required to protect flash contents.
///   Note this does not disable access to the uncached, untranslated window -- see NO_UNTRANSLATED_SEC.
///
/// - **POWER_DOWN** `[3]` (RW)
///
///   When 1, the cache memories are powered down. They retain state, but can not be accessed. This reduces static power dissipation. Writing 1 to this bit forces CTRL_EN_SECURE and CTRL_EN_NONSECURE to 0, i.e. the cache cannot be enabled when powered down.
///
/// - **EN_NONSECURE** `[1]` (RW)
///
///   When 1, enable the cache for Non-secure accesses. When enabled, Non-secure XIP accesses to the cached (addr[26] == 0) window will query the cache, and QSPI accesses are performed only if the requested data is not present. When disabled, Secure access ignore the cache contents, and always access the QSPI interface.
///   Accesses to the uncached (addr[26] == 1) window will never query the cache, irrespective of this bit.
///
/// - **EN_SECURE** `[0]` (RW)
///
///   When 1, enable the cache for Secure accesses. When enabled, Secure XIP accesses to the cached (addr[26] == 0) window will query the cache, and QSPI accesses are performed only if the requested data is not present. When disabled, Secure access ignore the cache contents, and always access the QSPI interface.
///   Accesses to the uncached (addr[26] == 1) window will never query the cache, irrespective of this bit.
///   There is no cache-as-SRAM address window. Cache lines are allocated for SRAM-like use by individually pinning them, and keeping the cache enabled.
#[repr(transparent)]
pub struct Ctrl {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Ctrl {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ctrl {
    // ======== Field Constants ========

    /// **WRITABLE_M1**
    ///
    /// If 1, enable writes to XIP memory window 1 (addresses 0x11000000 through 0x11ffffff, and their uncached mirrors). If 0, this region is read-only.
    /// XIP memory is *read-only by default*. This bit must be set to enable writes if a RAM device is attached on QSPI chip select 1.
    /// The default read-only behaviour avoids two issues with writing to a read-only QSPI device (e.g. flash). First, a write will initially appear to succeed due to caching, but the data will eventually be lost when the written line is evicted, causing unpredictable behaviour.
    /// Second, when a written line is evicted, it will cause a write command to be issued to the flash, which can break the flash out of its continuous read mode. After this point, flash reads will return garbage. This is a security concern, as it allows Non-secure software to break Secure flash reads if it has permission to write to any flash address.
    /// Note the read-only behaviour is implemented by downgrading writes to reads, so writes will still cause allocation of an address, but have no other effect.
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const WRITABLE_M1_OFFSET: u32 = 11;
    pub const WRITABLE_M1_MASK: u32 = 0x00000800;
    pub const WRITABLE_M1: u32 = 1 << 11;

    /// **WRITABLE_M0**
    ///
    /// If 1, enable writes to XIP memory window 0 (addresses 0x10000000 through 0x10ffffff, and their uncached mirrors). If 0, this region is read-only.
    /// XIP memory is *read-only by default*. This bit must be set to enable writes if a RAM device is attached on QSPI chip select 0.
    /// The default read-only behaviour avoids two issues with writing to a read-only QSPI device (e.g. flash). First, a write will initially appear to succeed due to caching, but the data will eventually be lost when the written line is evicted, causing unpredictable behaviour.
    /// Second, when a written line is evicted, it will cause a write command to be issued to the flash, which can break the flash out of its continuous read mode. After this point, flash reads will return garbage. This is a security concern, as it allows Non-secure software to break Secure flash reads if it has permission to write to any flash address.
    /// Note the read-only behaviour is implemented by downgrading writes to reads, so writes will still cause allocation of an address, but have no other effect.
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const WRITABLE_M0_OFFSET: u32 = 10;
    pub const WRITABLE_M0_MASK: u32 = 0x00000400;
    pub const WRITABLE_M0: u32 = 1 << 10;

    /// **SPLIT_WAYS**
    ///
    /// When 1, route all cached+Secure accesses to way 0 of the cache, and route all cached+Non-secure accesses to way 1 of the cache.
    /// This partitions the cache into two half-sized direct-mapped regions, such that Non-secure code can not observe cache line state changes caused by Secure execution.
    /// A full cache flush is required when changing the value of SPLIT_WAYS. The flush should be performed whilst SPLIT_WAYS is 0, so that both cache ways are accessible for invalidation.
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const SPLIT_WAYS_OFFSET: u32 = 9;
    pub const SPLIT_WAYS_MASK: u32 = 0x00000200;
    pub const SPLIT_WAYS: u32 = 1 << 9;

    /// **MAINT_NONSEC**
    ///
    /// When 0, Non-secure accesses to the cache maintenance address window (addr[27] == 1, addr[26] == 0) will generate a bus error. When 1, Non-secure accesses can perform cache maintenance operations by writing to the cache maintenance address window.
    /// Cache maintenance operations may be used to corrupt Secure data by invalidating cache lines inappropriately, or map Secure content into a Non-secure region by pinning cache lines. Therefore this bit should generally be set to 0, unless Secure code is not using the cache.
    /// Care should also be taken to clear the cache data memory and tag memory before granting maintenance operations to Non-secure code.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const MAINT_NONSEC_OFFSET: u32 = 8;
    pub const MAINT_NONSEC_MASK: u32 = 0x00000100;
    pub const MAINT_NONSEC: u32 = 1 << 8;

    /// **NO_UNTRANSLATED_NONSEC**
    ///
    /// When 1, Non-secure accesses to the uncached, untranslated window (addr[27:26] == 3) will generate a bus error.
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const NO_UNTRANSLATED_NONSEC_OFFSET: u32 = 7;
    pub const NO_UNTRANSLATED_NONSEC_MASK: u32 = 0x00000080;
    pub const NO_UNTRANSLATED_NONSEC: u32 = 1 << 7;

    /// **NO_UNTRANSLATED_SEC**
    ///
    /// When 1, Secure accesses to the uncached, untranslated window (addr[27:26] == 3) will generate a bus error.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const NO_UNTRANSLATED_SEC_OFFSET: u32 = 6;
    pub const NO_UNTRANSLATED_SEC_MASK: u32 = 0x00000040;
    pub const NO_UNTRANSLATED_SEC: u32 = 1 << 6;

    /// **NO_UNCACHED_NONSEC**
    ///
    /// When 1, Non-secure accesses to the uncached window (addr[27:26] == 1) will generate a bus error. This may reduce the number of SAU/MPU/PMP regions required to protect flash contents.
    /// Note this does not disable access to the uncached, untranslated window -- see NO_UNTRANSLATED_SEC.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const NO_UNCACHED_NONSEC_OFFSET: u32 = 5;
    pub const NO_UNCACHED_NONSEC_MASK: u32 = 0x00000020;
    pub const NO_UNCACHED_NONSEC: u32 = 1 << 5;

    /// **NO_UNCACHED_SEC**
    ///
    /// When 1, Secure accesses to the uncached window (addr[27:26] == 1) will generate a bus error. This may reduce the number of SAU/MPU/PMP regions required to protect flash contents.
    /// Note this does not disable access to the uncached, untranslated window -- see NO_UNTRANSLATED_SEC.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const NO_UNCACHED_SEC_OFFSET: u32 = 4;
    pub const NO_UNCACHED_SEC_MASK: u32 = 0x00000010;
    pub const NO_UNCACHED_SEC: u32 = 1 << 4;

    /// **POWER_DOWN**
    ///
    /// When 1, the cache memories are powered down. They retain state, but can not be accessed. This reduces static power dissipation. Writing 1 to this bit forces CTRL_EN_SECURE and CTRL_EN_NONSECURE to 0, i.e. the cache cannot be enabled when powered down.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const POWER_DOWN_OFFSET: u32 = 3;
    pub const POWER_DOWN_MASK: u32 = 0x00000008;
    pub const POWER_DOWN: u32 = 1 << 3;

    /// **EN_NONSECURE**
    ///
    /// When 1, enable the cache for Non-secure accesses. When enabled, Non-secure XIP accesses to the cached (addr[26] == 0) window will query the cache, and QSPI accesses are performed only if the requested data is not present. When disabled, Secure access ignore the cache contents, and always access the QSPI interface.
    /// Accesses to the uncached (addr[26] == 1) window will never query the cache, irrespective of this bit.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const EN_NONSECURE_OFFSET: u32 = 1;
    pub const EN_NONSECURE_MASK: u32 = 0x00000002;
    pub const EN_NONSECURE: u32 = 1 << 1;

    /// **EN_SECURE**
    ///
    /// When 1, enable the cache for Secure accesses. When enabled, Secure XIP accesses to the cached (addr[26] == 0) window will query the cache, and QSPI accesses are performed only if the requested data is not present. When disabled, Secure access ignore the cache contents, and always access the QSPI interface.
    /// Accesses to the uncached (addr[26] == 1) window will never query the cache, irrespective of this bit.
    /// There is no cache-as-SRAM address window. Cache lines are allocated for SRAM-like use by individually pinning them, and keeping the cache enabled.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const EN_SECURE_OFFSET: u32 = 0;
    pub const EN_SECURE_MASK: u32 = 0x00000001;
    pub const EN_SECURE: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **WRITABLE_M1** `[11]` (RW)
    ///
    /// If 1, enable writes to XIP memory window 1 (addresses 0x11000000 through 0x11ffffff, and their uncached mirrors). If 0, this region is read-only.
    /// XIP memory is *read-only by default*. This bit must be set to enable writes if a RAM device is attached on QSPI chip select 1.
    /// The default read-only behaviour avoids two issues with writing to a read-only QSPI device (e.g. flash). First, a write will initially appear to succeed due to caching, but the data will eventually be lost when the written line is evicted, causing unpredictable behaviour.
    /// Second, when a written line is evicted, it will cause a write command to be issued to the flash, which can break the flash out of its continuous read mode. After this point, flash reads will return garbage. This is a security concern, as it allows Non-secure software to break Secure flash reads if it has permission to write to any flash address.
    /// Note the read-only behaviour is implemented by downgrading writes to reads, so writes will still cause allocation of an address, but have no other effect.
    #[inline(always)]
    pub fn writable_m1(&self) -> bool {
        (self.value.read_bits() & Self::WRITABLE_M1_MASK) != 0
    }

    /// Set WRITABLE_M1 bit atomically (RW)
    #[inline(always)]
    pub fn set_writable_m1(&self) {
        self.set_bits(Self::WRITABLE_M1_MASK);
    }

    /// Clear WRITABLE_M1 bit atomically (RW)
    #[inline(always)]
    pub fn clr_writable_m1(&self) {
        self.clr_bits(Self::WRITABLE_M1_MASK);
    }

    /// **WRITABLE_M0** `[10]` (RW)
    ///
    /// If 1, enable writes to XIP memory window 0 (addresses 0x10000000 through 0x10ffffff, and their uncached mirrors). If 0, this region is read-only.
    /// XIP memory is *read-only by default*. This bit must be set to enable writes if a RAM device is attached on QSPI chip select 0.
    /// The default read-only behaviour avoids two issues with writing to a read-only QSPI device (e.g. flash). First, a write will initially appear to succeed due to caching, but the data will eventually be lost when the written line is evicted, causing unpredictable behaviour.
    /// Second, when a written line is evicted, it will cause a write command to be issued to the flash, which can break the flash out of its continuous read mode. After this point, flash reads will return garbage. This is a security concern, as it allows Non-secure software to break Secure flash reads if it has permission to write to any flash address.
    /// Note the read-only behaviour is implemented by downgrading writes to reads, so writes will still cause allocation of an address, but have no other effect.
    #[inline(always)]
    pub fn writable_m0(&self) -> bool {
        (self.value.read_bits() & Self::WRITABLE_M0_MASK) != 0
    }

    /// Set WRITABLE_M0 bit atomically (RW)
    #[inline(always)]
    pub fn set_writable_m0(&self) {
        self.set_bits(Self::WRITABLE_M0_MASK);
    }

    /// Clear WRITABLE_M0 bit atomically (RW)
    #[inline(always)]
    pub fn clr_writable_m0(&self) {
        self.clr_bits(Self::WRITABLE_M0_MASK);
    }

    /// **SPLIT_WAYS** `[9]` (RW)
    ///
    /// When 1, route all cached+Secure accesses to way 0 of the cache, and route all cached+Non-secure accesses to way 1 of the cache.
    /// This partitions the cache into two half-sized direct-mapped regions, such that Non-secure code can not observe cache line state changes caused by Secure execution.
    /// A full cache flush is required when changing the value of SPLIT_WAYS. The flush should be performed whilst SPLIT_WAYS is 0, so that both cache ways are accessible for invalidation.
    #[inline(always)]
    pub fn split_ways(&self) -> bool {
        (self.value.read_bits() & Self::SPLIT_WAYS_MASK) != 0
    }

    /// Set SPLIT_WAYS bit atomically (RW)
    #[inline(always)]
    pub fn set_split_ways(&self) {
        self.set_bits(Self::SPLIT_WAYS_MASK);
    }

    /// Clear SPLIT_WAYS bit atomically (RW)
    #[inline(always)]
    pub fn clr_split_ways(&self) {
        self.clr_bits(Self::SPLIT_WAYS_MASK);
    }

    /// **MAINT_NONSEC** `[8]` (RW)
    ///
    /// When 0, Non-secure accesses to the cache maintenance address window (addr[27] == 1, addr[26] == 0) will generate a bus error. When 1, Non-secure accesses can perform cache maintenance operations by writing to the cache maintenance address window.
    /// Cache maintenance operations may be used to corrupt Secure data by invalidating cache lines inappropriately, or map Secure content into a Non-secure region by pinning cache lines. Therefore this bit should generally be set to 0, unless Secure code is not using the cache.
    /// Care should also be taken to clear the cache data memory and tag memory before granting maintenance operations to Non-secure code.
    #[inline(always)]
    pub fn maint_nonsec(&self) -> bool {
        (self.value.read_bits() & Self::MAINT_NONSEC_MASK) != 0
    }

    /// Set MAINT_NONSEC bit atomically (RW)
    #[inline(always)]
    pub fn set_maint_nonsec(&self) {
        self.set_bits(Self::MAINT_NONSEC_MASK);
    }

    /// Clear MAINT_NONSEC bit atomically (RW)
    #[inline(always)]
    pub fn clr_maint_nonsec(&self) {
        self.clr_bits(Self::MAINT_NONSEC_MASK);
    }

    /// **NO_UNTRANSLATED_NONSEC** `[7]` (RW)
    ///
    /// When 1, Non-secure accesses to the uncached, untranslated window (addr[27:26] == 3) will generate a bus error.
    #[inline(always)]
    pub fn no_untranslated_nonsec(&self) -> bool {
        (self.value.read_bits() & Self::NO_UNTRANSLATED_NONSEC_MASK) != 0
    }

    /// Set NO_UNTRANSLATED_NONSEC bit atomically (RW)
    #[inline(always)]
    pub fn set_no_untranslated_nonsec(&self) {
        self.set_bits(Self::NO_UNTRANSLATED_NONSEC_MASK);
    }

    /// Clear NO_UNTRANSLATED_NONSEC bit atomically (RW)
    #[inline(always)]
    pub fn clr_no_untranslated_nonsec(&self) {
        self.clr_bits(Self::NO_UNTRANSLATED_NONSEC_MASK);
    }

    /// **NO_UNTRANSLATED_SEC** `[6]` (RW)
    ///
    /// When 1, Secure accesses to the uncached, untranslated window (addr[27:26] == 3) will generate a bus error.
    #[inline(always)]
    pub fn no_untranslated_sec(&self) -> bool {
        (self.value.read_bits() & Self::NO_UNTRANSLATED_SEC_MASK) != 0
    }

    /// Set NO_UNTRANSLATED_SEC bit atomically (RW)
    #[inline(always)]
    pub fn set_no_untranslated_sec(&self) {
        self.set_bits(Self::NO_UNTRANSLATED_SEC_MASK);
    }

    /// Clear NO_UNTRANSLATED_SEC bit atomically (RW)
    #[inline(always)]
    pub fn clr_no_untranslated_sec(&self) {
        self.clr_bits(Self::NO_UNTRANSLATED_SEC_MASK);
    }

    /// **NO_UNCACHED_NONSEC** `[5]` (RW)
    ///
    /// When 1, Non-secure accesses to the uncached window (addr[27:26] == 1) will generate a bus error. This may reduce the number of SAU/MPU/PMP regions required to protect flash contents.
    /// Note this does not disable access to the uncached, untranslated window -- see NO_UNTRANSLATED_SEC.
    #[inline(always)]
    pub fn no_uncached_nonsec(&self) -> bool {
        (self.value.read_bits() & Self::NO_UNCACHED_NONSEC_MASK) != 0
    }

    /// Set NO_UNCACHED_NONSEC bit atomically (RW)
    #[inline(always)]
    pub fn set_no_uncached_nonsec(&self) {
        self.set_bits(Self::NO_UNCACHED_NONSEC_MASK);
    }

    /// Clear NO_UNCACHED_NONSEC bit atomically (RW)
    #[inline(always)]
    pub fn clr_no_uncached_nonsec(&self) {
        self.clr_bits(Self::NO_UNCACHED_NONSEC_MASK);
    }

    /// **NO_UNCACHED_SEC** `[4]` (RW)
    ///
    /// When 1, Secure accesses to the uncached window (addr[27:26] == 1) will generate a bus error. This may reduce the number of SAU/MPU/PMP regions required to protect flash contents.
    /// Note this does not disable access to the uncached, untranslated window -- see NO_UNTRANSLATED_SEC.
    #[inline(always)]
    pub fn no_uncached_sec(&self) -> bool {
        (self.value.read_bits() & Self::NO_UNCACHED_SEC_MASK) != 0
    }

    /// Set NO_UNCACHED_SEC bit atomically (RW)
    #[inline(always)]
    pub fn set_no_uncached_sec(&self) {
        self.set_bits(Self::NO_UNCACHED_SEC_MASK);
    }

    /// Clear NO_UNCACHED_SEC bit atomically (RW)
    #[inline(always)]
    pub fn clr_no_uncached_sec(&self) {
        self.clr_bits(Self::NO_UNCACHED_SEC_MASK);
    }

    /// **POWER_DOWN** `[3]` (RW)
    ///
    /// When 1, the cache memories are powered down. They retain state, but can not be accessed. This reduces static power dissipation. Writing 1 to this bit forces CTRL_EN_SECURE and CTRL_EN_NONSECURE to 0, i.e. the cache cannot be enabled when powered down.
    #[inline(always)]
    pub fn power_down(&self) -> bool {
        (self.value.read_bits() & Self::POWER_DOWN_MASK) != 0
    }

    /// Set POWER_DOWN bit atomically (RW)
    #[inline(always)]
    pub fn set_power_down(&self) {
        self.set_bits(Self::POWER_DOWN_MASK);
    }

    /// Clear POWER_DOWN bit atomically (RW)
    #[inline(always)]
    pub fn clr_power_down(&self) {
        self.clr_bits(Self::POWER_DOWN_MASK);
    }

    /// **EN_NONSECURE** `[1]` (RW)
    ///
    /// When 1, enable the cache for Non-secure accesses. When enabled, Non-secure XIP accesses to the cached (addr[26] == 0) window will query the cache, and QSPI accesses are performed only if the requested data is not present. When disabled, Secure access ignore the cache contents, and always access the QSPI interface.
    /// Accesses to the uncached (addr[26] == 1) window will never query the cache, irrespective of this bit.
    #[inline(always)]
    pub fn en_nonsecure(&self) -> bool {
        (self.value.read_bits() & Self::EN_NONSECURE_MASK) != 0
    }

    /// Set EN_NONSECURE bit atomically (RW)
    #[inline(always)]
    pub fn set_en_nonsecure(&self) {
        self.set_bits(Self::EN_NONSECURE_MASK);
    }

    /// Clear EN_NONSECURE bit atomically (RW)
    #[inline(always)]
    pub fn clr_en_nonsecure(&self) {
        self.clr_bits(Self::EN_NONSECURE_MASK);
    }

    /// **EN_SECURE** `[0]` (RW)
    ///
    /// When 1, enable the cache for Secure accesses. When enabled, Secure XIP accesses to the cached (addr[26] == 0) window will query the cache, and QSPI accesses are performed only if the requested data is not present. When disabled, Secure access ignore the cache contents, and always access the QSPI interface.
    /// Accesses to the uncached (addr[26] == 1) window will never query the cache, irrespective of this bit.
    /// There is no cache-as-SRAM address window. Cache lines are allocated for SRAM-like use by individually pinning them, and keeping the cache enabled.
    #[inline(always)]
    pub fn en_secure(&self) -> bool {
        (self.value.read_bits() & Self::EN_SECURE_MASK) != 0
    }

    /// Set EN_SECURE bit atomically (RW)
    #[inline(always)]
    pub fn set_en_secure(&self) {
        self.set_bits(Self::EN_SECURE_MASK);
    }

    /// Clear EN_SECURE bit atomically (RW)
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

    /// Read **WRITABLE_M1** `[11:11]`
    ///
    /// If 1, enable writes to XIP memory window 1 (addresses 0x11000000 through 0x11ffffff, and their uncached mirrors). If 0, this region is read-only.
    /// XIP memory is *read-only by default*. This bit must be set to enable writes if a RAM device is attached on QSPI chip select 1.
    /// The default read-only behaviour avoids two issues with writing to a read-only QSPI device (e.g. flash). First, a write will initially appear to succeed due to caching, but the data will eventually be lost when the written line is evicted, causing unpredictable behaviour.
    /// Second, when a written line is evicted, it will cause a write command to be issued to the flash, which can break the flash out of its continuous read mode. After this point, flash reads will return garbage. This is a security concern, as it allows Non-secure software to break Secure flash reads if it has permission to write to any flash address.
    /// Note the read-only behaviour is implemented by downgrading writes to reads, so writes will still cause allocation of an address, but have no other effect.
    #[inline(always)]
    pub const fn writable_m1(&self) -> bool {
        (self.bits & Ctrl::WRITABLE_M1_MASK) != 0
    }

    /// Read **WRITABLE_M0** `[10:10]`
    ///
    /// If 1, enable writes to XIP memory window 0 (addresses 0x10000000 through 0x10ffffff, and their uncached mirrors). If 0, this region is read-only.
    /// XIP memory is *read-only by default*. This bit must be set to enable writes if a RAM device is attached on QSPI chip select 0.
    /// The default read-only behaviour avoids two issues with writing to a read-only QSPI device (e.g. flash). First, a write will initially appear to succeed due to caching, but the data will eventually be lost when the written line is evicted, causing unpredictable behaviour.
    /// Second, when a written line is evicted, it will cause a write command to be issued to the flash, which can break the flash out of its continuous read mode. After this point, flash reads will return garbage. This is a security concern, as it allows Non-secure software to break Secure flash reads if it has permission to write to any flash address.
    /// Note the read-only behaviour is implemented by downgrading writes to reads, so writes will still cause allocation of an address, but have no other effect.
    #[inline(always)]
    pub const fn writable_m0(&self) -> bool {
        (self.bits & Ctrl::WRITABLE_M0_MASK) != 0
    }

    /// Read **SPLIT_WAYS** `[9:9]`
    ///
    /// When 1, route all cached+Secure accesses to way 0 of the cache, and route all cached+Non-secure accesses to way 1 of the cache.
    /// This partitions the cache into two half-sized direct-mapped regions, such that Non-secure code can not observe cache line state changes caused by Secure execution.
    /// A full cache flush is required when changing the value of SPLIT_WAYS. The flush should be performed whilst SPLIT_WAYS is 0, so that both cache ways are accessible for invalidation.
    #[inline(always)]
    pub const fn split_ways(&self) -> bool {
        (self.bits & Ctrl::SPLIT_WAYS_MASK) != 0
    }

    /// Read **MAINT_NONSEC** `[8:8]`
    ///
    /// When 0, Non-secure accesses to the cache maintenance address window (addr[27] == 1, addr[26] == 0) will generate a bus error. When 1, Non-secure accesses can perform cache maintenance operations by writing to the cache maintenance address window.
    /// Cache maintenance operations may be used to corrupt Secure data by invalidating cache lines inappropriately, or map Secure content into a Non-secure region by pinning cache lines. Therefore this bit should generally be set to 0, unless Secure code is not using the cache.
    /// Care should also be taken to clear the cache data memory and tag memory before granting maintenance operations to Non-secure code.
    #[inline(always)]
    pub const fn maint_nonsec(&self) -> bool {
        (self.bits & Ctrl::MAINT_NONSEC_MASK) != 0
    }

    /// Read **NO_UNTRANSLATED_NONSEC** `[7:7]`
    ///
    /// When 1, Non-secure accesses to the uncached, untranslated window (addr[27:26] == 3) will generate a bus error.
    #[inline(always)]
    pub const fn no_untranslated_nonsec(&self) -> bool {
        (self.bits & Ctrl::NO_UNTRANSLATED_NONSEC_MASK) != 0
    }

    /// Read **NO_UNTRANSLATED_SEC** `[6:6]`
    ///
    /// When 1, Secure accesses to the uncached, untranslated window (addr[27:26] == 3) will generate a bus error.
    #[inline(always)]
    pub const fn no_untranslated_sec(&self) -> bool {
        (self.bits & Ctrl::NO_UNTRANSLATED_SEC_MASK) != 0
    }

    /// Read **NO_UNCACHED_NONSEC** `[5:5]`
    ///
    /// When 1, Non-secure accesses to the uncached window (addr[27:26] == 1) will generate a bus error. This may reduce the number of SAU/MPU/PMP regions required to protect flash contents.
    /// Note this does not disable access to the uncached, untranslated window -- see NO_UNTRANSLATED_SEC.
    #[inline(always)]
    pub const fn no_uncached_nonsec(&self) -> bool {
        (self.bits & Ctrl::NO_UNCACHED_NONSEC_MASK) != 0
    }

    /// Read **NO_UNCACHED_SEC** `[4:4]`
    ///
    /// When 1, Secure accesses to the uncached window (addr[27:26] == 1) will generate a bus error. This may reduce the number of SAU/MPU/PMP regions required to protect flash contents.
    /// Note this does not disable access to the uncached, untranslated window -- see NO_UNTRANSLATED_SEC.
    #[inline(always)]
    pub const fn no_uncached_sec(&self) -> bool {
        (self.bits & Ctrl::NO_UNCACHED_SEC_MASK) != 0
    }

    /// Read **POWER_DOWN** `[3:3]`
    ///
    /// When 1, the cache memories are powered down. They retain state, but can not be accessed. This reduces static power dissipation. Writing 1 to this bit forces CTRL_EN_SECURE and CTRL_EN_NONSECURE to 0, i.e. the cache cannot be enabled when powered down.
    #[inline(always)]
    pub const fn power_down(&self) -> bool {
        (self.bits & Ctrl::POWER_DOWN_MASK) != 0
    }

    /// Read **EN_NONSECURE** `[1:1]`
    ///
    /// When 1, enable the cache for Non-secure accesses. When enabled, Non-secure XIP accesses to the cached (addr[26] == 0) window will query the cache, and QSPI accesses are performed only if the requested data is not present. When disabled, Secure access ignore the cache contents, and always access the QSPI interface.
    /// Accesses to the uncached (addr[26] == 1) window will never query the cache, irrespective of this bit.
    #[inline(always)]
    pub const fn en_nonsecure(&self) -> bool {
        (self.bits & Ctrl::EN_NONSECURE_MASK) != 0
    }

    /// Read **EN_SECURE** `[0:0]`
    ///
    /// When 1, enable the cache for Secure accesses. When enabled, Secure XIP accesses to the cached (addr[26] == 0) window will query the cache, and QSPI accesses are performed only if the requested data is not present. When disabled, Secure access ignore the cache contents, and always access the QSPI interface.
    /// Accesses to the uncached (addr[26] == 1) window will never query the cache, irrespective of this bit.
    /// There is no cache-as-SRAM address window. Cache lines are allocated for SRAM-like use by individually pinning them, and keeping the cache enabled.
    #[inline(always)]
    pub const fn en_secure(&self) -> bool {
        (self.bits & Ctrl::EN_SECURE_MASK) != 0
    }

    /// Set **WRITABLE_M1** `[11:11]`
    ///
    /// If 1, enable writes to XIP memory window 1 (addresses 0x11000000 through 0x11ffffff, and their uncached mirrors). If 0, this region is read-only.
    /// XIP memory is *read-only by default*. This bit must be set to enable writes if a RAM device is attached on QSPI chip select 1.
    /// The default read-only behaviour avoids two issues with writing to a read-only QSPI device (e.g. flash). First, a write will initially appear to succeed due to caching, but the data will eventually be lost when the written line is evicted, causing unpredictable behaviour.
    /// Second, when a written line is evicted, it will cause a write command to be issued to the flash, which can break the flash out of its continuous read mode. After this point, flash reads will return garbage. This is a security concern, as it allows Non-secure software to break Secure flash reads if it has permission to write to any flash address.
    /// Note the read-only behaviour is implemented by downgrading writes to reads, so writes will still cause allocation of an address, but have no other effect.
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

    /// Set **WRITABLE_M0** `[10:10]`
    ///
    /// If 1, enable writes to XIP memory window 0 (addresses 0x10000000 through 0x10ffffff, and their uncached mirrors). If 0, this region is read-only.
    /// XIP memory is *read-only by default*. This bit must be set to enable writes if a RAM device is attached on QSPI chip select 0.
    /// The default read-only behaviour avoids two issues with writing to a read-only QSPI device (e.g. flash). First, a write will initially appear to succeed due to caching, but the data will eventually be lost when the written line is evicted, causing unpredictable behaviour.
    /// Second, when a written line is evicted, it will cause a write command to be issued to the flash, which can break the flash out of its continuous read mode. After this point, flash reads will return garbage. This is a security concern, as it allows Non-secure software to break Secure flash reads if it has permission to write to any flash address.
    /// Note the read-only behaviour is implemented by downgrading writes to reads, so writes will still cause allocation of an address, but have no other effect.
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

    /// Set **SPLIT_WAYS** `[9:9]`
    ///
    /// When 1, route all cached+Secure accesses to way 0 of the cache, and route all cached+Non-secure accesses to way 1 of the cache.
    /// This partitions the cache into two half-sized direct-mapped regions, such that Non-secure code can not observe cache line state changes caused by Secure execution.
    /// A full cache flush is required when changing the value of SPLIT_WAYS. The flush should be performed whilst SPLIT_WAYS is 0, so that both cache ways are accessible for invalidation.
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

    /// Set **MAINT_NONSEC** `[8:8]`
    ///
    /// When 0, Non-secure accesses to the cache maintenance address window (addr[27] == 1, addr[26] == 0) will generate a bus error. When 1, Non-secure accesses can perform cache maintenance operations by writing to the cache maintenance address window.
    /// Cache maintenance operations may be used to corrupt Secure data by invalidating cache lines inappropriately, or map Secure content into a Non-secure region by pinning cache lines. Therefore this bit should generally be set to 0, unless Secure code is not using the cache.
    /// Care should also be taken to clear the cache data memory and tag memory before granting maintenance operations to Non-secure code.
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

    /// Set **NO_UNTRANSLATED_NONSEC** `[7:7]`
    ///
    /// When 1, Non-secure accesses to the uncached, untranslated window (addr[27:26] == 3) will generate a bus error.
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

    /// Set **NO_UNTRANSLATED_SEC** `[6:6]`
    ///
    /// When 1, Secure accesses to the uncached, untranslated window (addr[27:26] == 3) will generate a bus error.
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

    /// Set **NO_UNCACHED_NONSEC** `[5:5]`
    ///
    /// When 1, Non-secure accesses to the uncached window (addr[27:26] == 1) will generate a bus error. This may reduce the number of SAU/MPU/PMP regions required to protect flash contents.
    /// Note this does not disable access to the uncached, untranslated window -- see NO_UNTRANSLATED_SEC.
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

    /// Set **NO_UNCACHED_SEC** `[4:4]`
    ///
    /// When 1, Secure accesses to the uncached window (addr[27:26] == 1) will generate a bus error. This may reduce the number of SAU/MPU/PMP regions required to protect flash contents.
    /// Note this does not disable access to the uncached, untranslated window -- see NO_UNTRANSLATED_SEC.
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

    /// Set **POWER_DOWN** `[3:3]`
    ///
    /// When 1, the cache memories are powered down. They retain state, but can not be accessed. This reduces static power dissipation. Writing 1 to this bit forces CTRL_EN_SECURE and CTRL_EN_NONSECURE to 0, i.e. the cache cannot be enabled when powered down.
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

    /// Set **EN_NONSECURE** `[1:1]`
    ///
    /// When 1, enable the cache for Non-secure accesses. When enabled, Non-secure XIP accesses to the cached (addr[26] == 0) window will query the cache, and QSPI accesses are performed only if the requested data is not present. When disabled, Secure access ignore the cache contents, and always access the QSPI interface.
    /// Accesses to the uncached (addr[26] == 1) window will never query the cache, irrespective of this bit.
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

    /// Set **EN_SECURE** `[0:0]`
    ///
    /// When 1, enable the cache for Secure accesses. When enabled, Secure XIP accesses to the cached (addr[26] == 0) window will query the cache, and QSPI accesses are performed only if the requested data is not present. When disabled, Secure access ignore the cache contents, and always access the QSPI interface.
    /// Accesses to the uncached (addr[26] == 1) window will never query the cache, irrespective of this bit.
    /// There is no cache-as-SRAM address window. Cache lines are allocated for SRAM-like use by individually pinning them, and keeping the cache enabled.
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
    pub fn read_val(&self) -> CtrlVal {
        CtrlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CtrlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CtrlVal) -> CtrlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// STAT Register
///
/// Address offset: 0x0008
/// Reset value: 0x00000002
///
/// ## Fields
///
/// - **FIFO_FULL** `[2]` (RO)
///
///   When 1, indicates the XIP streaming FIFO is completely full.
///   The streaming FIFO is 2 entries deep, so the full and empty
///   flag allow its level to be ascertained.
///
/// - **FIFO_EMPTY** `[1]` (RO)
///
///   When 1, indicates the XIP streaming FIFO is completely empty.
#[repr(transparent)]
pub struct Stat {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Stat {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Stat {
    // ======== Field Constants ========

    /// **FIFO_FULL**
    ///
    /// When 1, indicates the XIP streaming FIFO is completely full.
    /// The streaming FIFO is 2 entries deep, so the full and empty
    /// flag allow its level to be ascertained.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const FIFO_FULL_OFFSET: u32 = 2;
    pub const FIFO_FULL_MASK: u32 = 0x00000004;
    pub const FIFO_FULL: u32 = 1 << 2;

    /// **FIFO_EMPTY**
    ///
    /// When 1, indicates the XIP streaming FIFO is completely empty.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const FIFO_EMPTY_OFFSET: u32 = 1;
    pub const FIFO_EMPTY_MASK: u32 = 0x00000002;
    pub const FIFO_EMPTY: u32 = 1 << 1;

    // ======== Field Accessors ========

    /// **FIFO_FULL** `[2]` (RO)
    ///
    /// When 1, indicates the XIP streaming FIFO is completely full.
    /// The streaming FIFO is 2 entries deep, so the full and empty
    /// flag allow its level to be ascertained.
    #[inline(always)]
    pub fn fifo_full(&self) -> bool {
        (self.value.read_bits() & Self::FIFO_FULL_MASK) != 0
    }

    /// **FIFO_EMPTY** `[1]` (RO)
    ///
    /// When 1, indicates the XIP streaming FIFO is completely empty.
    #[inline(always)]
    pub fn fifo_empty(&self) -> bool {
        (self.value.read_bits() & Self::FIFO_EMPTY_MASK) != 0
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

    /// Read **FIFO_FULL** `[2:2]`
    ///
    /// When 1, indicates the XIP streaming FIFO is completely full.
    /// The streaming FIFO is 2 entries deep, so the full and empty
    /// flag allow its level to be ascertained.
    #[inline(always)]
    pub const fn fifo_full(&self) -> bool {
        (self.bits & Stat::FIFO_FULL_MASK) != 0
    }

    /// Read **FIFO_EMPTY** `[1:1]`
    ///
    /// When 1, indicates the XIP streaming FIFO is completely empty.
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
    pub fn read_val(&self) -> StatVal {
        StatVal::from_bits(self.value.read_bits())
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
/// - **CTR_HIT** `[31:0]` (RW)
///
///   A 32 bit saturating counter that increments upon each cache hit,
///   i.e. when an XIP access is serviced directly from cached data.
///   Write any value to clear.
#[repr(transparent)]
pub struct CtrHit {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for CtrHit {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl CtrHit {
    // ======== Field Constants ========

    /// **CTR_HIT**
    ///
    /// A 32 bit saturating counter that increments upon each cache hit,
    /// i.e. when an XIP access is serviced directly from cached data.
    /// Write any value to clear.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const CTR_HIT_OFFSET: u32 = 0;
    pub const CTR_HIT_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **CTR_HIT** `[31:0]` (RW)
    ///
    /// A 32 bit saturating counter that increments upon each cache hit,
    /// i.e. when an XIP access is serviced directly from cached data.
    /// Write any value to clear.
    #[inline(always)]
    pub fn ctr_hit(&self) -> u32 {
        (self.value.read_bits() & Self::CTR_HIT_MASK) >> Self::CTR_HIT_OFFSET
    }

    /// Write **CTR_HIT** `[31:0]` (RW)
    ///
    /// A 32 bit saturating counter that increments upon each cache hit,
    /// i.e. when an XIP access is serviced directly from cached data.
    /// Write any value to clear.
    #[inline(always)]
    pub fn write_ctr_hit(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::CTR_HIT_MASK) | ((value << Self::CTR_HIT_OFFSET) & Self::CTR_HIT_MASK));
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

    /// Read **CTR_HIT** `[31:0]`
    ///
    /// A 32 bit saturating counter that increments upon each cache hit,
    /// i.e. when an XIP access is serviced directly from cached data.
    /// Write any value to clear.
    #[inline(always)]
    pub const fn ctr_hit(&self) -> u32 {
        (self.bits & CtrHit::CTR_HIT_MASK) >> CtrHit::CTR_HIT_OFFSET
    }

    /// Set **CTR_HIT** `[31:0]`
    ///
    /// A 32 bit saturating counter that increments upon each cache hit,
    /// i.e. when an XIP access is serviced directly from cached data.
    /// Write any value to clear.
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
    pub fn read_val(&self) -> CtrHitVal {
        CtrHitVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CtrHitVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CtrHitVal) -> CtrHitVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
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
/// - **CTR_ACC** `[31:0]` (RW)
///
///   A 32 bit saturating counter that increments upon each XIP access,
///   whether the cache is hit or not. This includes noncacheable accesses.
///   Write any value to clear.
#[repr(transparent)]
pub struct CtrAcc {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for CtrAcc {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl CtrAcc {
    // ======== Field Constants ========

    /// **CTR_ACC**
    ///
    /// A 32 bit saturating counter that increments upon each XIP access,
    /// whether the cache is hit or not. This includes noncacheable accesses.
    /// Write any value to clear.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const CTR_ACC_OFFSET: u32 = 0;
    pub const CTR_ACC_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **CTR_ACC** `[31:0]` (RW)
    ///
    /// A 32 bit saturating counter that increments upon each XIP access,
    /// whether the cache is hit or not. This includes noncacheable accesses.
    /// Write any value to clear.
    #[inline(always)]
    pub fn ctr_acc(&self) -> u32 {
        (self.value.read_bits() & Self::CTR_ACC_MASK) >> Self::CTR_ACC_OFFSET
    }

    /// Write **CTR_ACC** `[31:0]` (RW)
    ///
    /// A 32 bit saturating counter that increments upon each XIP access,
    /// whether the cache is hit or not. This includes noncacheable accesses.
    /// Write any value to clear.
    #[inline(always)]
    pub fn write_ctr_acc(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::CTR_ACC_MASK) | ((value << Self::CTR_ACC_OFFSET) & Self::CTR_ACC_MASK));
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

    /// Read **CTR_ACC** `[31:0]`
    ///
    /// A 32 bit saturating counter that increments upon each XIP access,
    /// whether the cache is hit or not. This includes noncacheable accesses.
    /// Write any value to clear.
    #[inline(always)]
    pub const fn ctr_acc(&self) -> u32 {
        (self.bits & CtrAcc::CTR_ACC_MASK) >> CtrAcc::CTR_ACC_OFFSET
    }

    /// Set **CTR_ACC** `[31:0]`
    ///
    /// A 32 bit saturating counter that increments upon each XIP access,
    /// whether the cache is hit or not. This includes noncacheable accesses.
    /// Write any value to clear.
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
    pub fn read_val(&self) -> CtrAccVal {
        CtrAccVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CtrAccVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CtrAccVal) -> CtrAccVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
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
/// - **STREAM_ADDR** `[31:2]` (RW)
///
///   The address of the next word to be streamed from flash to the streaming FIFO.
///   Increments automatically after each flash access.
///   Write the initial access address here before starting a streaming read.
#[repr(transparent)]
pub struct StreamAddr {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for StreamAddr {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl StreamAddr {
    // ======== Field Constants ========

    /// **STREAM_ADDR**
    ///
    /// The address of the next word to be streamed from flash to the streaming FIFO.
    /// Increments automatically after each flash access.
    /// Write the initial access address here before starting a streaming read.
    ///
    /// Bits `[31:2]`, mask `0xfffffffc`
    pub const STREAM_ADDR_OFFSET: u32 = 2;
    pub const STREAM_ADDR_MASK: u32 = 0xfffffffc;

    // ======== Field Accessors ========

    /// **STREAM_ADDR** `[31:2]` (RW)
    ///
    /// The address of the next word to be streamed from flash to the streaming FIFO.
    /// Increments automatically after each flash access.
    /// Write the initial access address here before starting a streaming read.
    #[inline(always)]
    pub fn stream_addr(&self) -> u32 {
        (self.value.read_bits() & Self::STREAM_ADDR_MASK) >> Self::STREAM_ADDR_OFFSET
    }

    /// Write **STREAM_ADDR** `[31:2]` (RW)
    ///
    /// The address of the next word to be streamed from flash to the streaming FIFO.
    /// Increments automatically after each flash access.
    /// Write the initial access address here before starting a streaming read.
    #[inline(always)]
    pub fn write_stream_addr(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::STREAM_ADDR_MASK) | ((value << Self::STREAM_ADDR_OFFSET) & Self::STREAM_ADDR_MASK));
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

    /// Read **STREAM_ADDR** `[31:2]`
    ///
    /// The address of the next word to be streamed from flash to the streaming FIFO.
    /// Increments automatically after each flash access.
    /// Write the initial access address here before starting a streaming read.
    #[inline(always)]
    pub const fn stream_addr(&self) -> u32 {
        (self.bits & StreamAddr::STREAM_ADDR_MASK) >> StreamAddr::STREAM_ADDR_OFFSET
    }

    /// Set **STREAM_ADDR** `[31:2]`
    ///
    /// The address of the next word to be streamed from flash to the streaming FIFO.
    /// Increments automatically after each flash access.
    /// Write the initial access address here before starting a streaming read.
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
    pub fn read_val(&self) -> StreamAddrVal {
        StreamAddrVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: StreamAddrVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(StreamAddrVal) -> StreamAddrVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
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
/// - **STREAM_CTR** `[21:0]` (RW)
///
///   Write a nonzero value to start a streaming read. This will then
///   progress in the background, using flash idle cycles to transfer
///   a linear data block from flash to the streaming FIFO.
///   Decrements automatically (1 at a time) as the stream
///   progresses, and halts on reaching 0.
///   Write 0 to halt an in-progress stream, and discard any in-flight
///   read, so that a new stream can immediately be started (after
///   draining the FIFO and reinitialising STREAM_ADDR)
#[repr(transparent)]
pub struct StreamCtr {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for StreamCtr {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl StreamCtr {
    // ======== Field Constants ========

    /// **STREAM_CTR**
    ///
    /// Write a nonzero value to start a streaming read. This will then
    /// progress in the background, using flash idle cycles to transfer
    /// a linear data block from flash to the streaming FIFO.
    /// Decrements automatically (1 at a time) as the stream
    /// progresses, and halts on reaching 0.
    /// Write 0 to halt an in-progress stream, and discard any in-flight
    /// read, so that a new stream can immediately be started (after
    /// draining the FIFO and reinitialising STREAM_ADDR)
    ///
    /// Bits `[21:0]`, mask `0x003fffff`
    pub const STREAM_CTR_OFFSET: u32 = 0;
    pub const STREAM_CTR_MASK: u32 = 0x003fffff;

    // ======== Field Accessors ========

    /// **STREAM_CTR** `[21:0]` (RW)
    ///
    /// Write a nonzero value to start a streaming read. This will then
    /// progress in the background, using flash idle cycles to transfer
    /// a linear data block from flash to the streaming FIFO.
    /// Decrements automatically (1 at a time) as the stream
    /// progresses, and halts on reaching 0.
    /// Write 0 to halt an in-progress stream, and discard any in-flight
    /// read, so that a new stream can immediately be started (after
    /// draining the FIFO and reinitialising STREAM_ADDR)
    #[inline(always)]
    pub fn stream_ctr(&self) -> u32 {
        (self.value.read_bits() & Self::STREAM_CTR_MASK) >> Self::STREAM_CTR_OFFSET
    }

    /// Write **STREAM_CTR** `[21:0]` (RW)
    ///
    /// Write a nonzero value to start a streaming read. This will then
    /// progress in the background, using flash idle cycles to transfer
    /// a linear data block from flash to the streaming FIFO.
    /// Decrements automatically (1 at a time) as the stream
    /// progresses, and halts on reaching 0.
    /// Write 0 to halt an in-progress stream, and discard any in-flight
    /// read, so that a new stream can immediately be started (after
    /// draining the FIFO and reinitialising STREAM_ADDR)
    #[inline(always)]
    pub fn write_stream_ctr(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::STREAM_CTR_MASK) | ((value << Self::STREAM_CTR_OFFSET) & Self::STREAM_CTR_MASK));
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

    /// Read **STREAM_CTR** `[21:0]`
    ///
    /// Write a nonzero value to start a streaming read. This will then
    /// progress in the background, using flash idle cycles to transfer
    /// a linear data block from flash to the streaming FIFO.
    /// Decrements automatically (1 at a time) as the stream
    /// progresses, and halts on reaching 0.
    /// Write 0 to halt an in-progress stream, and discard any in-flight
    /// read, so that a new stream can immediately be started (after
    /// draining the FIFO and reinitialising STREAM_ADDR)
    #[inline(always)]
    pub const fn stream_ctr(&self) -> u32 {
        (self.bits & StreamCtr::STREAM_CTR_MASK) >> StreamCtr::STREAM_CTR_OFFSET
    }

    /// Set **STREAM_CTR** `[21:0]`
    ///
    /// Write a nonzero value to start a streaming read. This will then
    /// progress in the background, using flash idle cycles to transfer
    /// a linear data block from flash to the streaming FIFO.
    /// Decrements automatically (1 at a time) as the stream
    /// progresses, and halts on reaching 0.
    /// Write 0 to halt an in-progress stream, and discard any in-flight
    /// read, so that a new stream can immediately be started (after
    /// draining the FIFO and reinitialising STREAM_ADDR)
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
    pub fn read_val(&self) -> StreamCtrVal {
        StreamCtrVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: StreamCtrVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(StreamCtrVal) -> StreamCtrVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
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
/// - **STREAM_FIFO** `[31:0]` (RO)
///
///   Streamed data is buffered here, for retrieval by the system DMA.
///   This FIFO can also be accessed via the XIP_AUX slave, to avoid exposing
///   the DMA to bus stalls caused by other XIP traffic.
#[repr(transparent)]
pub struct StreamFifo {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for StreamFifo {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl StreamFifo {
    // ======== Field Constants ========

    /// **STREAM_FIFO**
    ///
    /// Streamed data is buffered here, for retrieval by the system DMA.
    /// This FIFO can also be accessed via the XIP_AUX slave, to avoid exposing
    /// the DMA to bus stalls caused by other XIP traffic.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const STREAM_FIFO_OFFSET: u32 = 0;
    pub const STREAM_FIFO_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **STREAM_FIFO** `[31:0]` (RO)
    ///
    /// Streamed data is buffered here, for retrieval by the system DMA.
    /// This FIFO can also be accessed via the XIP_AUX slave, to avoid exposing
    /// the DMA to bus stalls caused by other XIP traffic.
    #[inline(always)]
    pub fn stream_fifo(&self) -> u32 {
        (self.value.read_bits() & Self::STREAM_FIFO_MASK) >> Self::STREAM_FIFO_OFFSET
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

    /// Read **STREAM_FIFO** `[31:0]`
    ///
    /// Streamed data is buffered here, for retrieval by the system DMA.
    /// This FIFO can also be accessed via the XIP_AUX slave, to avoid exposing
    /// the DMA to bus stalls caused by other XIP traffic.
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
    pub fn read_val(&self) -> StreamFifoVal {
        StreamFifoVal::from_bits(self.value.read_bits())
    }
}

/// Register block
#[repr(C)]
pub struct RegisterBlock {
    /// 0x0000
    /// Cache control register. Read-only from a Non-secure context.
    pub ctrl: Ctrl,
    _reserved0: [u8; 4],
    /// 0x0008
    pub stat: Stat,
    /// 0x000c
    /// Cache Hit counter
    pub ctr_hit: CtrHit,
    /// 0x0010
    /// Cache Access counter
    pub ctr_acc: CtrAcc,
    /// 0x0014
    /// FIFO stream address
    pub stream_addr: StreamAddr,
    /// 0x0018
    /// FIFO stream control
    pub stream_ctr: StreamCtr,
    /// 0x001c
    /// FIFO stream data
    pub stream_fifo: StreamFifo,
}

