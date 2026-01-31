//! BUSCTRL
//! Register block for busfabric control signals and performance counters

#![allow(dead_code)]

pub const BASE: u32 = 0x40068000;

/// BUS_PRIORITY Register
///
/// Set the priority of each master for bus arbitration.
///
/// Address offset: 0x0000
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **DMA_W** `[12]` (RW)
///
///   0 - low priority, 1 - high priority
///
/// - **DMA_R** `[8]` (RW)
///
///   0 - low priority, 1 - high priority
///
/// - **PROC1** `[4]` (RW)
///
///   0 - low priority, 1 - high priority
///
/// - **PROC0** `[0]` (RW)
///
///   0 - low priority, 1 - high priority
#[repr(transparent)]
pub struct BusPriority {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for BusPriority {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl BusPriority {
    // ======== Field Constants ========

    /// **DMA_W**
    ///
    /// 0 - low priority, 1 - high priority
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const DMA_W_OFFSET: u32 = 12;
    pub const DMA_W_MASK: u32 = 0x00001000;
    pub const DMA_W: u32 = 1 << 12;

    /// **DMA_R**
    ///
    /// 0 - low priority, 1 - high priority
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const DMA_R_OFFSET: u32 = 8;
    pub const DMA_R_MASK: u32 = 0x00000100;
    pub const DMA_R: u32 = 1 << 8;

    /// **PROC1**
    ///
    /// 0 - low priority, 1 - high priority
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const PROC1_OFFSET: u32 = 4;
    pub const PROC1_MASK: u32 = 0x00000010;
    pub const PROC1: u32 = 1 << 4;

    /// **PROC0**
    ///
    /// 0 - low priority, 1 - high priority
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const PROC0_OFFSET: u32 = 0;
    pub const PROC0_MASK: u32 = 0x00000001;
    pub const PROC0: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **DMA_W** `[12]` (RW)
    ///
    /// 0 - low priority, 1 - high priority
    #[inline(always)]
    pub fn dma_w(&self) -> bool {
        (self.value.read_bits() & Self::DMA_W_MASK) != 0
    }

    /// Set DMA_W bit atomically (RW)
    #[inline(always)]
    pub fn set_dma_w(&self) {
        self.value.set_bits(Self::DMA_W_MASK);
    }

    /// Clear DMA_W bit atomically (RW)
    #[inline(always)]
    pub fn clr_dma_w(&self) {
        self.value.clr_bits(Self::DMA_W_MASK);
    }

    /// **DMA_R** `[8]` (RW)
    ///
    /// 0 - low priority, 1 - high priority
    #[inline(always)]
    pub fn dma_r(&self) -> bool {
        (self.value.read_bits() & Self::DMA_R_MASK) != 0
    }

    /// Set DMA_R bit atomically (RW)
    #[inline(always)]
    pub fn set_dma_r(&self) {
        self.value.set_bits(Self::DMA_R_MASK);
    }

    /// Clear DMA_R bit atomically (RW)
    #[inline(always)]
    pub fn clr_dma_r(&self) {
        self.value.clr_bits(Self::DMA_R_MASK);
    }

    /// **PROC1** `[4]` (RW)
    ///
    /// 0 - low priority, 1 - high priority
    #[inline(always)]
    pub fn proc1(&self) -> bool {
        (self.value.read_bits() & Self::PROC1_MASK) != 0
    }

    /// Set PROC1 bit atomically (RW)
    #[inline(always)]
    pub fn set_proc1(&self) {
        self.value.set_bits(Self::PROC1_MASK);
    }

    /// Clear PROC1 bit atomically (RW)
    #[inline(always)]
    pub fn clr_proc1(&self) {
        self.value.clr_bits(Self::PROC1_MASK);
    }

    /// **PROC0** `[0]` (RW)
    ///
    /// 0 - low priority, 1 - high priority
    #[inline(always)]
    pub fn proc0(&self) -> bool {
        (self.value.read_bits() & Self::PROC0_MASK) != 0
    }

    /// Set PROC0 bit atomically (RW)
    #[inline(always)]
    pub fn set_proc0(&self) {
        self.value.set_bits(Self::PROC0_MASK);
    }

    /// Clear PROC0 bit atomically (RW)
    #[inline(always)]
    pub fn clr_proc0(&self) {
        self.value.clr_bits(Self::PROC0_MASK);
    }

}

/// BUS_PRIORITY value type for batch read/modify/write operations
///
/// Use `buspriority.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `buspriority.write_val(val)`.
#[derive(Clone, Copy)]
pub struct BusPriorityVal {
    bits: u32,
}

impl BusPriorityVal {
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

    /// Read **DMA_W** `[12:12]`
    ///
    /// 0 - low priority, 1 - high priority
    #[inline(always)]
    pub const fn dma_w(&self) -> bool {
        (self.bits & BusPriority::DMA_W_MASK) != 0
    }

    /// Read **DMA_R** `[8:8]`
    ///
    /// 0 - low priority, 1 - high priority
    #[inline(always)]
    pub const fn dma_r(&self) -> bool {
        (self.bits & BusPriority::DMA_R_MASK) != 0
    }

    /// Read **PROC1** `[4:4]`
    ///
    /// 0 - low priority, 1 - high priority
    #[inline(always)]
    pub const fn proc1(&self) -> bool {
        (self.bits & BusPriority::PROC1_MASK) != 0
    }

    /// Read **PROC0** `[0:0]`
    ///
    /// 0 - low priority, 1 - high priority
    #[inline(always)]
    pub const fn proc0(&self) -> bool {
        (self.bits & BusPriority::PROC0_MASK) != 0
    }

    /// Set **DMA_W** `[12:12]`
    ///
    /// 0 - low priority, 1 - high priority
    #[inline(always)]
    pub const fn with_dma_w(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | BusPriority::DMA_W_MASK
            } else {
                self.bits & !BusPriority::DMA_W_MASK
            }
        }
    }

    /// Set **DMA_R** `[8:8]`
    ///
    /// 0 - low priority, 1 - high priority
    #[inline(always)]
    pub const fn with_dma_r(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | BusPriority::DMA_R_MASK
            } else {
                self.bits & !BusPriority::DMA_R_MASK
            }
        }
    }

    /// Set **PROC1** `[4:4]`
    ///
    /// 0 - low priority, 1 - high priority
    #[inline(always)]
    pub const fn with_proc1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | BusPriority::PROC1_MASK
            } else {
                self.bits & !BusPriority::PROC1_MASK
            }
        }
    }

    /// Set **PROC0** `[0:0]`
    ///
    /// 0 - low priority, 1 - high priority
    #[inline(always)]
    pub const fn with_proc0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | BusPriority::PROC0_MASK
            } else {
                self.bits & !BusPriority::PROC0_MASK
            }
        }
    }

}

impl Default for BusPriorityVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl BusPriority {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> BusPriorityVal {
        BusPriorityVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: BusPriorityVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(BusPriorityVal) -> BusPriorityVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// BUS_PRIORITY_ACK Register
///
/// Bus priority acknowledge
///
/// Address offset: 0x0004
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **BUS_PRIORITY_ACK** `[0]` (RO)
///
///   Goes to 1 once all arbiters have registered the new global priority levels.
///   Arbiters update their local priority when servicing a new nonsequential access.
///   In normal circumstances this will happen almost immediately.
#[repr(transparent)]
pub struct BusPriorityAck {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for BusPriorityAck {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl BusPriorityAck {
    // ======== Field Constants ========

    /// **BUS_PRIORITY_ACK**
    ///
    /// Goes to 1 once all arbiters have registered the new global priority levels.
    /// Arbiters update their local priority when servicing a new nonsequential access.
    /// In normal circumstances this will happen almost immediately.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const BUS_PRIORITY_ACK_OFFSET: u32 = 0;
    pub const BUS_PRIORITY_ACK_MASK: u32 = 0x00000001;
    pub const BUS_PRIORITY_ACK: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **BUS_PRIORITY_ACK** `[0]` (RO)
    ///
    /// Goes to 1 once all arbiters have registered the new global priority levels.
    /// Arbiters update their local priority when servicing a new nonsequential access.
    /// In normal circumstances this will happen almost immediately.
    #[inline(always)]
    pub fn bus_priority_ack(&self) -> bool {
        (self.value.read_bits() & Self::BUS_PRIORITY_ACK_MASK) != 0
    }

}

/// BUS_PRIORITY_ACK value type for batch read/modify/write operations
///
/// Use `buspriorityack.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `buspriorityack.write_val(val)`.
#[derive(Clone, Copy)]
pub struct BusPriorityAckVal {
    bits: u32,
}

impl BusPriorityAckVal {
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

    /// Read **BUS_PRIORITY_ACK** `[0:0]`
    ///
    /// Goes to 1 once all arbiters have registered the new global priority levels.
    /// Arbiters update their local priority when servicing a new nonsequential access.
    /// In normal circumstances this will happen almost immediately.
    #[inline(always)]
    pub const fn bus_priority_ack(&self) -> bool {
        (self.bits & BusPriorityAck::BUS_PRIORITY_ACK_MASK) != 0
    }

}

impl Default for BusPriorityAckVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl BusPriorityAck {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> BusPriorityAckVal {
        BusPriorityAckVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: BusPriorityAckVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(BusPriorityAckVal) -> BusPriorityAckVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// PERFCTR_EN Register
///
/// Enable the performance counters. If 0, the performance counters do not increment. This can be used to precisely start/stop event sampling around the profiled section of code.
/// The performance counters are initially disabled, to save energy.
///
/// Address offset: 0x0008
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **PERFCTR_EN** `[0]` (RW)
#[repr(transparent)]
pub struct PerfctrEn {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for PerfctrEn {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl PerfctrEn {
    // ======== Field Constants ========

    /// **PERFCTR_EN**
    /// Bits `[0:0]`, mask `0x00000001`
    pub const PERFCTR_EN_OFFSET: u32 = 0;
    pub const PERFCTR_EN_MASK: u32 = 0x00000001;
    pub const PERFCTR_EN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **PERFCTR_EN** `[0]` (RW)
    #[inline(always)]
    pub fn perfctr_en(&self) -> bool {
        (self.value.read_bits() & Self::PERFCTR_EN_MASK) != 0
    }

    /// Set PERFCTR_EN bit atomically (RW)
    #[inline(always)]
    pub fn set_perfctr_en(&self) {
        self.value.set_bits(Self::PERFCTR_EN_MASK);
    }

    /// Clear PERFCTR_EN bit atomically (RW)
    #[inline(always)]
    pub fn clr_perfctr_en(&self) {
        self.value.clr_bits(Self::PERFCTR_EN_MASK);
    }

}

/// PERFCTR_EN value type for batch read/modify/write operations
///
/// Use `perfctren.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `perfctren.write_val(val)`.
#[derive(Clone, Copy)]
pub struct PerfctrEnVal {
    bits: u32,
}

impl PerfctrEnVal {
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

    /// Read **PERFCTR_EN** `[0:0]`
    #[inline(always)]
    pub const fn perfctr_en(&self) -> bool {
        (self.bits & PerfctrEn::PERFCTR_EN_MASK) != 0
    }

    /// Set **PERFCTR_EN** `[0:0]`
    #[inline(always)]
    pub const fn with_perfctr_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | PerfctrEn::PERFCTR_EN_MASK
            } else {
                self.bits & !PerfctrEn::PERFCTR_EN_MASK
            }
        }
    }

}

impl Default for PerfctrEnVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl PerfctrEn {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> PerfctrEnVal {
        PerfctrEnVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: PerfctrEnVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(PerfctrEnVal) -> PerfctrEnVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// PERFCTR0 Register
///
/// Bus fabric performance counter 0
///
/// Address offset: 0x000c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **PERFCTR0** `[23:0]` (RW)
///
///   Busfabric saturating performance counter 0
///   Count some event signal from the busfabric arbiters, if PERFCTR_EN is set.
///   Write any value to clear. Select an event to count using PERFSEL0
#[repr(transparent)]
pub struct Perfctr0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Perfctr0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Perfctr0 {
    // ======== Field Constants ========

    /// **PERFCTR0**
    ///
    /// Busfabric saturating performance counter 0
    /// Count some event signal from the busfabric arbiters, if PERFCTR_EN is set.
    /// Write any value to clear. Select an event to count using PERFSEL0
    ///
    /// Bits `[23:0]`, mask `0x00ffffff`
    pub const PERFCTR0_OFFSET: u32 = 0;
    pub const PERFCTR0_MASK: u32 = 0x00ffffff;

    // ======== Field Accessors ========

    /// **PERFCTR0** `[23:0]` (RW)
    ///
    /// Busfabric saturating performance counter 0
    /// Count some event signal from the busfabric arbiters, if PERFCTR_EN is set.
    /// Write any value to clear. Select an event to count using PERFSEL0
    #[inline(always)]
    pub fn perfctr0(&self) -> u32 {
        (self.value.read_bits() & Self::PERFCTR0_MASK) >> Self::PERFCTR0_OFFSET
    }

    /// Write **PERFCTR0** `[23:0]` (RW)
    ///
    /// Busfabric saturating performance counter 0
    /// Count some event signal from the busfabric arbiters, if PERFCTR_EN is set.
    /// Write any value to clear. Select an event to count using PERFSEL0
    #[inline(always)]
    pub fn write_perfctr0(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::PERFCTR0_MASK) | ((value << Self::PERFCTR0_OFFSET) & Self::PERFCTR0_MASK));
    }

}

/// PERFCTR0 value type for batch read/modify/write operations
///
/// Use `perfctr0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `perfctr0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Perfctr0Val {
    bits: u32,
}

impl Perfctr0Val {
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

    /// Read **PERFCTR0** `[23:0]`
    ///
    /// Busfabric saturating performance counter 0
    /// Count some event signal from the busfabric arbiters, if PERFCTR_EN is set.
    /// Write any value to clear. Select an event to count using PERFSEL0
    #[inline(always)]
    pub const fn perfctr0(&self) -> u32 {
        (self.bits & Perfctr0::PERFCTR0_MASK) >> Perfctr0::PERFCTR0_OFFSET
    }

    /// Set **PERFCTR0** `[23:0]`
    ///
    /// Busfabric saturating performance counter 0
    /// Count some event signal from the busfabric arbiters, if PERFCTR_EN is set.
    /// Write any value to clear. Select an event to count using PERFSEL0
    #[inline(always)]
    pub const fn with_perfctr0(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Perfctr0::PERFCTR0_MASK) | ((value << Perfctr0::PERFCTR0_OFFSET) & Perfctr0::PERFCTR0_MASK)
        }
    }

}

impl Default for Perfctr0Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Perfctr0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Perfctr0Val {
        Perfctr0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Perfctr0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Perfctr0Val) -> Perfctr0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// PERFSEL0 Register
///
/// Bus fabric performance event select for PERFCTR0
///
/// Address offset: 0x0010
/// Reset value: 0x0000001f
///
/// ## Fields
///
/// - **PERFSEL0** `[6:0]` (RW)
///
///   Select an event for PERFCTR0. For each downstream port of the main crossbar, four events are available: ACCESS, an access took place; ACCESS_CONTESTED, an access took place that previously stalled due to contention from other masters; STALL_DOWNSTREAM, count cycles where any master stalled due to a stall on the downstream bus; STALL_UPSTREAM, count cycles where any master stalled for any reason, including contention from other masters.
#[repr(transparent)]
pub struct Perfsel0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Perfsel0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Perfsel0 {
    // ======== Field Constants ========

    /// **PERFSEL0**
    ///
    /// Select an event for PERFCTR0. For each downstream port of the main crossbar, four events are available: ACCESS, an access took place; ACCESS_CONTESTED, an access took place that previously stalled due to contention from other masters; STALL_DOWNSTREAM, count cycles where any master stalled due to a stall on the downstream bus; STALL_UPSTREAM, count cycles where any master stalled for any reason, including contention from other masters.
    ///
    /// Bits `[6:0]`, mask `0x0000007f`
    pub const PERFSEL0_OFFSET: u32 = 0;
    pub const PERFSEL0_MASK: u32 = 0x0000007f;
    pub const PERFSEL0_SIOB_PROC1_STALL_UPSTREAM: u32 = 0 << 0;
    pub const PERFSEL0_SIOB_PROC1_STALL_UPSTREAM_VAL: u32 = 0;
    pub const PERFSEL0_SIOB_PROC1_STALL_DOWNSTREAM: u32 = 1 << 0;
    pub const PERFSEL0_SIOB_PROC1_STALL_DOWNSTREAM_VAL: u32 = 1;
    pub const PERFSEL0_SIOB_PROC1_ACCESS_CONTESTED: u32 = 2 << 0;
    pub const PERFSEL0_SIOB_PROC1_ACCESS_CONTESTED_VAL: u32 = 2;
    pub const PERFSEL0_SIOB_PROC1_ACCESS: u32 = 3 << 0;
    pub const PERFSEL0_SIOB_PROC1_ACCESS_VAL: u32 = 3;
    pub const PERFSEL0_SIOB_PROC0_STALL_UPSTREAM: u32 = 4 << 0;
    pub const PERFSEL0_SIOB_PROC0_STALL_UPSTREAM_VAL: u32 = 4;
    pub const PERFSEL0_SIOB_PROC0_STALL_DOWNSTREAM: u32 = 5 << 0;
    pub const PERFSEL0_SIOB_PROC0_STALL_DOWNSTREAM_VAL: u32 = 5;
    pub const PERFSEL0_SIOB_PROC0_ACCESS_CONTESTED: u32 = 6 << 0;
    pub const PERFSEL0_SIOB_PROC0_ACCESS_CONTESTED_VAL: u32 = 6;
    pub const PERFSEL0_SIOB_PROC0_ACCESS: u32 = 7 << 0;
    pub const PERFSEL0_SIOB_PROC0_ACCESS_VAL: u32 = 7;
    pub const PERFSEL0_APB_STALL_UPSTREAM: u32 = 8 << 0;
    pub const PERFSEL0_APB_STALL_UPSTREAM_VAL: u32 = 8;
    pub const PERFSEL0_APB_STALL_DOWNSTREAM: u32 = 9 << 0;
    pub const PERFSEL0_APB_STALL_DOWNSTREAM_VAL: u32 = 9;
    pub const PERFSEL0_APB_ACCESS_CONTESTED: u32 = 10 << 0;
    pub const PERFSEL0_APB_ACCESS_CONTESTED_VAL: u32 = 10;
    pub const PERFSEL0_APB_ACCESS: u32 = 11 << 0;
    pub const PERFSEL0_APB_ACCESS_VAL: u32 = 11;
    pub const PERFSEL0_FASTPERI_STALL_UPSTREAM: u32 = 12 << 0;
    pub const PERFSEL0_FASTPERI_STALL_UPSTREAM_VAL: u32 = 12;
    pub const PERFSEL0_FASTPERI_STALL_DOWNSTREAM: u32 = 13 << 0;
    pub const PERFSEL0_FASTPERI_STALL_DOWNSTREAM_VAL: u32 = 13;
    pub const PERFSEL0_FASTPERI_ACCESS_CONTESTED: u32 = 14 << 0;
    pub const PERFSEL0_FASTPERI_ACCESS_CONTESTED_VAL: u32 = 14;
    pub const PERFSEL0_FASTPERI_ACCESS: u32 = 15 << 0;
    pub const PERFSEL0_FASTPERI_ACCESS_VAL: u32 = 15;
    pub const PERFSEL0_SRAM9_STALL_UPSTREAM: u32 = 16 << 0;
    pub const PERFSEL0_SRAM9_STALL_UPSTREAM_VAL: u32 = 16;
    pub const PERFSEL0_SRAM9_STALL_DOWNSTREAM: u32 = 17 << 0;
    pub const PERFSEL0_SRAM9_STALL_DOWNSTREAM_VAL: u32 = 17;
    pub const PERFSEL0_SRAM9_ACCESS_CONTESTED: u32 = 18 << 0;
    pub const PERFSEL0_SRAM9_ACCESS_CONTESTED_VAL: u32 = 18;
    pub const PERFSEL0_SRAM9_ACCESS: u32 = 19 << 0;
    pub const PERFSEL0_SRAM9_ACCESS_VAL: u32 = 19;
    pub const PERFSEL0_SRAM8_STALL_UPSTREAM: u32 = 20 << 0;
    pub const PERFSEL0_SRAM8_STALL_UPSTREAM_VAL: u32 = 20;
    pub const PERFSEL0_SRAM8_STALL_DOWNSTREAM: u32 = 21 << 0;
    pub const PERFSEL0_SRAM8_STALL_DOWNSTREAM_VAL: u32 = 21;
    pub const PERFSEL0_SRAM8_ACCESS_CONTESTED: u32 = 22 << 0;
    pub const PERFSEL0_SRAM8_ACCESS_CONTESTED_VAL: u32 = 22;
    pub const PERFSEL0_SRAM8_ACCESS: u32 = 23 << 0;
    pub const PERFSEL0_SRAM8_ACCESS_VAL: u32 = 23;
    pub const PERFSEL0_SRAM7_STALL_UPSTREAM: u32 = 24 << 0;
    pub const PERFSEL0_SRAM7_STALL_UPSTREAM_VAL: u32 = 24;
    pub const PERFSEL0_SRAM7_STALL_DOWNSTREAM: u32 = 25 << 0;
    pub const PERFSEL0_SRAM7_STALL_DOWNSTREAM_VAL: u32 = 25;
    pub const PERFSEL0_SRAM7_ACCESS_CONTESTED: u32 = 26 << 0;
    pub const PERFSEL0_SRAM7_ACCESS_CONTESTED_VAL: u32 = 26;
    pub const PERFSEL0_SRAM7_ACCESS: u32 = 27 << 0;
    pub const PERFSEL0_SRAM7_ACCESS_VAL: u32 = 27;
    pub const PERFSEL0_SRAM6_STALL_UPSTREAM: u32 = 28 << 0;
    pub const PERFSEL0_SRAM6_STALL_UPSTREAM_VAL: u32 = 28;
    pub const PERFSEL0_SRAM6_STALL_DOWNSTREAM: u32 = 29 << 0;
    pub const PERFSEL0_SRAM6_STALL_DOWNSTREAM_VAL: u32 = 29;
    pub const PERFSEL0_SRAM6_ACCESS_CONTESTED: u32 = 30 << 0;
    pub const PERFSEL0_SRAM6_ACCESS_CONTESTED_VAL: u32 = 30;
    pub const PERFSEL0_SRAM6_ACCESS: u32 = 31 << 0;
    pub const PERFSEL0_SRAM6_ACCESS_VAL: u32 = 31;
    pub const PERFSEL0_SRAM5_STALL_UPSTREAM: u32 = 32 << 0;
    pub const PERFSEL0_SRAM5_STALL_UPSTREAM_VAL: u32 = 32;
    pub const PERFSEL0_SRAM5_STALL_DOWNSTREAM: u32 = 33 << 0;
    pub const PERFSEL0_SRAM5_STALL_DOWNSTREAM_VAL: u32 = 33;
    pub const PERFSEL0_SRAM5_ACCESS_CONTESTED: u32 = 34 << 0;
    pub const PERFSEL0_SRAM5_ACCESS_CONTESTED_VAL: u32 = 34;
    pub const PERFSEL0_SRAM5_ACCESS: u32 = 35 << 0;
    pub const PERFSEL0_SRAM5_ACCESS_VAL: u32 = 35;
    pub const PERFSEL0_SRAM4_STALL_UPSTREAM: u32 = 36 << 0;
    pub const PERFSEL0_SRAM4_STALL_UPSTREAM_VAL: u32 = 36;
    pub const PERFSEL0_SRAM4_STALL_DOWNSTREAM: u32 = 37 << 0;
    pub const PERFSEL0_SRAM4_STALL_DOWNSTREAM_VAL: u32 = 37;
    pub const PERFSEL0_SRAM4_ACCESS_CONTESTED: u32 = 38 << 0;
    pub const PERFSEL0_SRAM4_ACCESS_CONTESTED_VAL: u32 = 38;
    pub const PERFSEL0_SRAM4_ACCESS: u32 = 39 << 0;
    pub const PERFSEL0_SRAM4_ACCESS_VAL: u32 = 39;
    pub const PERFSEL0_SRAM3_STALL_UPSTREAM: u32 = 40 << 0;
    pub const PERFSEL0_SRAM3_STALL_UPSTREAM_VAL: u32 = 40;
    pub const PERFSEL0_SRAM3_STALL_DOWNSTREAM: u32 = 41 << 0;
    pub const PERFSEL0_SRAM3_STALL_DOWNSTREAM_VAL: u32 = 41;
    pub const PERFSEL0_SRAM3_ACCESS_CONTESTED: u32 = 42 << 0;
    pub const PERFSEL0_SRAM3_ACCESS_CONTESTED_VAL: u32 = 42;
    pub const PERFSEL0_SRAM3_ACCESS: u32 = 43 << 0;
    pub const PERFSEL0_SRAM3_ACCESS_VAL: u32 = 43;
    pub const PERFSEL0_SRAM2_STALL_UPSTREAM: u32 = 44 << 0;
    pub const PERFSEL0_SRAM2_STALL_UPSTREAM_VAL: u32 = 44;
    pub const PERFSEL0_SRAM2_STALL_DOWNSTREAM: u32 = 45 << 0;
    pub const PERFSEL0_SRAM2_STALL_DOWNSTREAM_VAL: u32 = 45;
    pub const PERFSEL0_SRAM2_ACCESS_CONTESTED: u32 = 46 << 0;
    pub const PERFSEL0_SRAM2_ACCESS_CONTESTED_VAL: u32 = 46;
    pub const PERFSEL0_SRAM2_ACCESS: u32 = 47 << 0;
    pub const PERFSEL0_SRAM2_ACCESS_VAL: u32 = 47;
    pub const PERFSEL0_SRAM1_STALL_UPSTREAM: u32 = 48 << 0;
    pub const PERFSEL0_SRAM1_STALL_UPSTREAM_VAL: u32 = 48;
    pub const PERFSEL0_SRAM1_STALL_DOWNSTREAM: u32 = 49 << 0;
    pub const PERFSEL0_SRAM1_STALL_DOWNSTREAM_VAL: u32 = 49;
    pub const PERFSEL0_SRAM1_ACCESS_CONTESTED: u32 = 50 << 0;
    pub const PERFSEL0_SRAM1_ACCESS_CONTESTED_VAL: u32 = 50;
    pub const PERFSEL0_SRAM1_ACCESS: u32 = 51 << 0;
    pub const PERFSEL0_SRAM1_ACCESS_VAL: u32 = 51;
    pub const PERFSEL0_SRAM0_STALL_UPSTREAM: u32 = 52 << 0;
    pub const PERFSEL0_SRAM0_STALL_UPSTREAM_VAL: u32 = 52;
    pub const PERFSEL0_SRAM0_STALL_DOWNSTREAM: u32 = 53 << 0;
    pub const PERFSEL0_SRAM0_STALL_DOWNSTREAM_VAL: u32 = 53;
    pub const PERFSEL0_SRAM0_ACCESS_CONTESTED: u32 = 54 << 0;
    pub const PERFSEL0_SRAM0_ACCESS_CONTESTED_VAL: u32 = 54;
    pub const PERFSEL0_SRAM0_ACCESS: u32 = 55 << 0;
    pub const PERFSEL0_SRAM0_ACCESS_VAL: u32 = 55;
    pub const PERFSEL0_XIP_MAIN1_STALL_UPSTREAM: u32 = 56 << 0;
    pub const PERFSEL0_XIP_MAIN1_STALL_UPSTREAM_VAL: u32 = 56;
    pub const PERFSEL0_XIP_MAIN1_STALL_DOWNSTREAM: u32 = 57 << 0;
    pub const PERFSEL0_XIP_MAIN1_STALL_DOWNSTREAM_VAL: u32 = 57;
    pub const PERFSEL0_XIP_MAIN1_ACCESS_CONTESTED: u32 = 58 << 0;
    pub const PERFSEL0_XIP_MAIN1_ACCESS_CONTESTED_VAL: u32 = 58;
    pub const PERFSEL0_XIP_MAIN1_ACCESS: u32 = 59 << 0;
    pub const PERFSEL0_XIP_MAIN1_ACCESS_VAL: u32 = 59;
    pub const PERFSEL0_XIP_MAIN0_STALL_UPSTREAM: u32 = 60 << 0;
    pub const PERFSEL0_XIP_MAIN0_STALL_UPSTREAM_VAL: u32 = 60;
    pub const PERFSEL0_XIP_MAIN0_STALL_DOWNSTREAM: u32 = 61 << 0;
    pub const PERFSEL0_XIP_MAIN0_STALL_DOWNSTREAM_VAL: u32 = 61;
    pub const PERFSEL0_XIP_MAIN0_ACCESS_CONTESTED: u32 = 62 << 0;
    pub const PERFSEL0_XIP_MAIN0_ACCESS_CONTESTED_VAL: u32 = 62;
    pub const PERFSEL0_XIP_MAIN0_ACCESS: u32 = 63 << 0;
    pub const PERFSEL0_XIP_MAIN0_ACCESS_VAL: u32 = 63;
    pub const PERFSEL0_ROM_STALL_UPSTREAM: u32 = 64 << 0;
    pub const PERFSEL0_ROM_STALL_UPSTREAM_VAL: u32 = 64;
    pub const PERFSEL0_ROM_STALL_DOWNSTREAM: u32 = 65 << 0;
    pub const PERFSEL0_ROM_STALL_DOWNSTREAM_VAL: u32 = 65;
    pub const PERFSEL0_ROM_ACCESS_CONTESTED: u32 = 66 << 0;
    pub const PERFSEL0_ROM_ACCESS_CONTESTED_VAL: u32 = 66;
    pub const PERFSEL0_ROM_ACCESS: u32 = 67 << 0;
    pub const PERFSEL0_ROM_ACCESS_VAL: u32 = 67;

    // ======== Field Accessors ========

    /// **PERFSEL0** `[6:0]` (RW)
    ///
    /// Select an event for PERFCTR0. For each downstream port of the main crossbar, four events are available: ACCESS, an access took place; ACCESS_CONTESTED, an access took place that previously stalled due to contention from other masters; STALL_DOWNSTREAM, count cycles where any master stalled due to a stall on the downstream bus; STALL_UPSTREAM, count cycles where any master stalled for any reason, including contention from other masters.
    #[inline(always)]
    pub fn perfsel0(&self) -> u32 {
        (self.value.read_bits() & Self::PERFSEL0_MASK) >> Self::PERFSEL0_OFFSET
    }

    /// Write **PERFSEL0** `[6:0]` (RW)
    ///
    /// Select an event for PERFCTR0. For each downstream port of the main crossbar, four events are available: ACCESS, an access took place; ACCESS_CONTESTED, an access took place that previously stalled due to contention from other masters; STALL_DOWNSTREAM, count cycles where any master stalled due to a stall on the downstream bus; STALL_UPSTREAM, count cycles where any master stalled for any reason, including contention from other masters.
    #[inline(always)]
    pub fn write_perfsel0(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::PERFSEL0_MASK) | ((value << Self::PERFSEL0_OFFSET) & Self::PERFSEL0_MASK));
    }

}

/// PERFSEL0 value type for batch read/modify/write operations
///
/// Use `perfsel0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `perfsel0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Perfsel0Val {
    bits: u32,
}

impl Perfsel0Val {
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
        Self { bits: 0x0000001f }
    }

    /// Read **PERFSEL0** `[6:0]`
    ///
    /// Select an event for PERFCTR0. For each downstream port of the main crossbar, four events are available: ACCESS, an access took place; ACCESS_CONTESTED, an access took place that previously stalled due to contention from other masters; STALL_DOWNSTREAM, count cycles where any master stalled due to a stall on the downstream bus; STALL_UPSTREAM, count cycles where any master stalled for any reason, including contention from other masters.
    #[inline(always)]
    pub const fn perfsel0(&self) -> u32 {
        (self.bits & Perfsel0::PERFSEL0_MASK) >> Perfsel0::PERFSEL0_OFFSET
    }

    /// Set **PERFSEL0** `[6:0]`
    ///
    /// Select an event for PERFCTR0. For each downstream port of the main crossbar, four events are available: ACCESS, an access took place; ACCESS_CONTESTED, an access took place that previously stalled due to contention from other masters; STALL_DOWNSTREAM, count cycles where any master stalled due to a stall on the downstream bus; STALL_UPSTREAM, count cycles where any master stalled for any reason, including contention from other masters.
    #[inline(always)]
    pub const fn with_perfsel0(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Perfsel0::PERFSEL0_MASK) | ((value << Perfsel0::PERFSEL0_OFFSET) & Perfsel0::PERFSEL0_MASK)
        }
    }

}

impl Default for Perfsel0Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Perfsel0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Perfsel0Val {
        Perfsel0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Perfsel0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Perfsel0Val) -> Perfsel0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// PERFCTR1 Register
///
/// Bus fabric performance counter 1
///
/// Address offset: 0x0014
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **PERFCTR1** `[23:0]` (RW)
///
///   Busfabric saturating performance counter 1
///   Count some event signal from the busfabric arbiters, if PERFCTR_EN is set.
///   Write any value to clear. Select an event to count using PERFSEL1
#[repr(transparent)]
pub struct Perfctr1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Perfctr1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Perfctr1 {
    // ======== Field Constants ========

    /// **PERFCTR1**
    ///
    /// Busfabric saturating performance counter 1
    /// Count some event signal from the busfabric arbiters, if PERFCTR_EN is set.
    /// Write any value to clear. Select an event to count using PERFSEL1
    ///
    /// Bits `[23:0]`, mask `0x00ffffff`
    pub const PERFCTR1_OFFSET: u32 = 0;
    pub const PERFCTR1_MASK: u32 = 0x00ffffff;

    // ======== Field Accessors ========

    /// **PERFCTR1** `[23:0]` (RW)
    ///
    /// Busfabric saturating performance counter 1
    /// Count some event signal from the busfabric arbiters, if PERFCTR_EN is set.
    /// Write any value to clear. Select an event to count using PERFSEL1
    #[inline(always)]
    pub fn perfctr1(&self) -> u32 {
        (self.value.read_bits() & Self::PERFCTR1_MASK) >> Self::PERFCTR1_OFFSET
    }

    /// Write **PERFCTR1** `[23:0]` (RW)
    ///
    /// Busfabric saturating performance counter 1
    /// Count some event signal from the busfabric arbiters, if PERFCTR_EN is set.
    /// Write any value to clear. Select an event to count using PERFSEL1
    #[inline(always)]
    pub fn write_perfctr1(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::PERFCTR1_MASK) | ((value << Self::PERFCTR1_OFFSET) & Self::PERFCTR1_MASK));
    }

}

/// PERFCTR1 value type for batch read/modify/write operations
///
/// Use `perfctr1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `perfctr1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Perfctr1Val {
    bits: u32,
}

impl Perfctr1Val {
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

    /// Read **PERFCTR1** `[23:0]`
    ///
    /// Busfabric saturating performance counter 1
    /// Count some event signal from the busfabric arbiters, if PERFCTR_EN is set.
    /// Write any value to clear. Select an event to count using PERFSEL1
    #[inline(always)]
    pub const fn perfctr1(&self) -> u32 {
        (self.bits & Perfctr1::PERFCTR1_MASK) >> Perfctr1::PERFCTR1_OFFSET
    }

    /// Set **PERFCTR1** `[23:0]`
    ///
    /// Busfabric saturating performance counter 1
    /// Count some event signal from the busfabric arbiters, if PERFCTR_EN is set.
    /// Write any value to clear. Select an event to count using PERFSEL1
    #[inline(always)]
    pub const fn with_perfctr1(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Perfctr1::PERFCTR1_MASK) | ((value << Perfctr1::PERFCTR1_OFFSET) & Perfctr1::PERFCTR1_MASK)
        }
    }

}

impl Default for Perfctr1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Perfctr1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Perfctr1Val {
        Perfctr1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Perfctr1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Perfctr1Val) -> Perfctr1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// PERFSEL1 Register
///
/// Bus fabric performance event select for PERFCTR1
///
/// Address offset: 0x0018
/// Reset value: 0x0000001f
///
/// ## Fields
///
/// - **PERFSEL1** `[6:0]` (RW)
///
///   Select an event for PERFCTR1. For each downstream port of the main crossbar, four events are available: ACCESS, an access took place; ACCESS_CONTESTED, an access took place that previously stalled due to contention from other masters; STALL_DOWNSTREAM, count cycles where any master stalled due to a stall on the downstream bus; STALL_UPSTREAM, count cycles where any master stalled for any reason, including contention from other masters.
#[repr(transparent)]
pub struct Perfsel1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Perfsel1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Perfsel1 {
    // ======== Field Constants ========

    /// **PERFSEL1**
    ///
    /// Select an event for PERFCTR1. For each downstream port of the main crossbar, four events are available: ACCESS, an access took place; ACCESS_CONTESTED, an access took place that previously stalled due to contention from other masters; STALL_DOWNSTREAM, count cycles where any master stalled due to a stall on the downstream bus; STALL_UPSTREAM, count cycles where any master stalled for any reason, including contention from other masters.
    ///
    /// Bits `[6:0]`, mask `0x0000007f`
    pub const PERFSEL1_OFFSET: u32 = 0;
    pub const PERFSEL1_MASK: u32 = 0x0000007f;
    pub const PERFSEL1_SIOB_PROC1_STALL_UPSTREAM: u32 = 0 << 0;
    pub const PERFSEL1_SIOB_PROC1_STALL_UPSTREAM_VAL: u32 = 0;
    pub const PERFSEL1_SIOB_PROC1_STALL_DOWNSTREAM: u32 = 1 << 0;
    pub const PERFSEL1_SIOB_PROC1_STALL_DOWNSTREAM_VAL: u32 = 1;
    pub const PERFSEL1_SIOB_PROC1_ACCESS_CONTESTED: u32 = 2 << 0;
    pub const PERFSEL1_SIOB_PROC1_ACCESS_CONTESTED_VAL: u32 = 2;
    pub const PERFSEL1_SIOB_PROC1_ACCESS: u32 = 3 << 0;
    pub const PERFSEL1_SIOB_PROC1_ACCESS_VAL: u32 = 3;
    pub const PERFSEL1_SIOB_PROC0_STALL_UPSTREAM: u32 = 4 << 0;
    pub const PERFSEL1_SIOB_PROC0_STALL_UPSTREAM_VAL: u32 = 4;
    pub const PERFSEL1_SIOB_PROC0_STALL_DOWNSTREAM: u32 = 5 << 0;
    pub const PERFSEL1_SIOB_PROC0_STALL_DOWNSTREAM_VAL: u32 = 5;
    pub const PERFSEL1_SIOB_PROC0_ACCESS_CONTESTED: u32 = 6 << 0;
    pub const PERFSEL1_SIOB_PROC0_ACCESS_CONTESTED_VAL: u32 = 6;
    pub const PERFSEL1_SIOB_PROC0_ACCESS: u32 = 7 << 0;
    pub const PERFSEL1_SIOB_PROC0_ACCESS_VAL: u32 = 7;
    pub const PERFSEL1_APB_STALL_UPSTREAM: u32 = 8 << 0;
    pub const PERFSEL1_APB_STALL_UPSTREAM_VAL: u32 = 8;
    pub const PERFSEL1_APB_STALL_DOWNSTREAM: u32 = 9 << 0;
    pub const PERFSEL1_APB_STALL_DOWNSTREAM_VAL: u32 = 9;
    pub const PERFSEL1_APB_ACCESS_CONTESTED: u32 = 10 << 0;
    pub const PERFSEL1_APB_ACCESS_CONTESTED_VAL: u32 = 10;
    pub const PERFSEL1_APB_ACCESS: u32 = 11 << 0;
    pub const PERFSEL1_APB_ACCESS_VAL: u32 = 11;
    pub const PERFSEL1_FASTPERI_STALL_UPSTREAM: u32 = 12 << 0;
    pub const PERFSEL1_FASTPERI_STALL_UPSTREAM_VAL: u32 = 12;
    pub const PERFSEL1_FASTPERI_STALL_DOWNSTREAM: u32 = 13 << 0;
    pub const PERFSEL1_FASTPERI_STALL_DOWNSTREAM_VAL: u32 = 13;
    pub const PERFSEL1_FASTPERI_ACCESS_CONTESTED: u32 = 14 << 0;
    pub const PERFSEL1_FASTPERI_ACCESS_CONTESTED_VAL: u32 = 14;
    pub const PERFSEL1_FASTPERI_ACCESS: u32 = 15 << 0;
    pub const PERFSEL1_FASTPERI_ACCESS_VAL: u32 = 15;
    pub const PERFSEL1_SRAM9_STALL_UPSTREAM: u32 = 16 << 0;
    pub const PERFSEL1_SRAM9_STALL_UPSTREAM_VAL: u32 = 16;
    pub const PERFSEL1_SRAM9_STALL_DOWNSTREAM: u32 = 17 << 0;
    pub const PERFSEL1_SRAM9_STALL_DOWNSTREAM_VAL: u32 = 17;
    pub const PERFSEL1_SRAM9_ACCESS_CONTESTED: u32 = 18 << 0;
    pub const PERFSEL1_SRAM9_ACCESS_CONTESTED_VAL: u32 = 18;
    pub const PERFSEL1_SRAM9_ACCESS: u32 = 19 << 0;
    pub const PERFSEL1_SRAM9_ACCESS_VAL: u32 = 19;
    pub const PERFSEL1_SRAM8_STALL_UPSTREAM: u32 = 20 << 0;
    pub const PERFSEL1_SRAM8_STALL_UPSTREAM_VAL: u32 = 20;
    pub const PERFSEL1_SRAM8_STALL_DOWNSTREAM: u32 = 21 << 0;
    pub const PERFSEL1_SRAM8_STALL_DOWNSTREAM_VAL: u32 = 21;
    pub const PERFSEL1_SRAM8_ACCESS_CONTESTED: u32 = 22 << 0;
    pub const PERFSEL1_SRAM8_ACCESS_CONTESTED_VAL: u32 = 22;
    pub const PERFSEL1_SRAM8_ACCESS: u32 = 23 << 0;
    pub const PERFSEL1_SRAM8_ACCESS_VAL: u32 = 23;
    pub const PERFSEL1_SRAM7_STALL_UPSTREAM: u32 = 24 << 0;
    pub const PERFSEL1_SRAM7_STALL_UPSTREAM_VAL: u32 = 24;
    pub const PERFSEL1_SRAM7_STALL_DOWNSTREAM: u32 = 25 << 0;
    pub const PERFSEL1_SRAM7_STALL_DOWNSTREAM_VAL: u32 = 25;
    pub const PERFSEL1_SRAM7_ACCESS_CONTESTED: u32 = 26 << 0;
    pub const PERFSEL1_SRAM7_ACCESS_CONTESTED_VAL: u32 = 26;
    pub const PERFSEL1_SRAM7_ACCESS: u32 = 27 << 0;
    pub const PERFSEL1_SRAM7_ACCESS_VAL: u32 = 27;
    pub const PERFSEL1_SRAM6_STALL_UPSTREAM: u32 = 28 << 0;
    pub const PERFSEL1_SRAM6_STALL_UPSTREAM_VAL: u32 = 28;
    pub const PERFSEL1_SRAM6_STALL_DOWNSTREAM: u32 = 29 << 0;
    pub const PERFSEL1_SRAM6_STALL_DOWNSTREAM_VAL: u32 = 29;
    pub const PERFSEL1_SRAM6_ACCESS_CONTESTED: u32 = 30 << 0;
    pub const PERFSEL1_SRAM6_ACCESS_CONTESTED_VAL: u32 = 30;
    pub const PERFSEL1_SRAM6_ACCESS: u32 = 31 << 0;
    pub const PERFSEL1_SRAM6_ACCESS_VAL: u32 = 31;
    pub const PERFSEL1_SRAM5_STALL_UPSTREAM: u32 = 32 << 0;
    pub const PERFSEL1_SRAM5_STALL_UPSTREAM_VAL: u32 = 32;
    pub const PERFSEL1_SRAM5_STALL_DOWNSTREAM: u32 = 33 << 0;
    pub const PERFSEL1_SRAM5_STALL_DOWNSTREAM_VAL: u32 = 33;
    pub const PERFSEL1_SRAM5_ACCESS_CONTESTED: u32 = 34 << 0;
    pub const PERFSEL1_SRAM5_ACCESS_CONTESTED_VAL: u32 = 34;
    pub const PERFSEL1_SRAM5_ACCESS: u32 = 35 << 0;
    pub const PERFSEL1_SRAM5_ACCESS_VAL: u32 = 35;
    pub const PERFSEL1_SRAM4_STALL_UPSTREAM: u32 = 36 << 0;
    pub const PERFSEL1_SRAM4_STALL_UPSTREAM_VAL: u32 = 36;
    pub const PERFSEL1_SRAM4_STALL_DOWNSTREAM: u32 = 37 << 0;
    pub const PERFSEL1_SRAM4_STALL_DOWNSTREAM_VAL: u32 = 37;
    pub const PERFSEL1_SRAM4_ACCESS_CONTESTED: u32 = 38 << 0;
    pub const PERFSEL1_SRAM4_ACCESS_CONTESTED_VAL: u32 = 38;
    pub const PERFSEL1_SRAM4_ACCESS: u32 = 39 << 0;
    pub const PERFSEL1_SRAM4_ACCESS_VAL: u32 = 39;
    pub const PERFSEL1_SRAM3_STALL_UPSTREAM: u32 = 40 << 0;
    pub const PERFSEL1_SRAM3_STALL_UPSTREAM_VAL: u32 = 40;
    pub const PERFSEL1_SRAM3_STALL_DOWNSTREAM: u32 = 41 << 0;
    pub const PERFSEL1_SRAM3_STALL_DOWNSTREAM_VAL: u32 = 41;
    pub const PERFSEL1_SRAM3_ACCESS_CONTESTED: u32 = 42 << 0;
    pub const PERFSEL1_SRAM3_ACCESS_CONTESTED_VAL: u32 = 42;
    pub const PERFSEL1_SRAM3_ACCESS: u32 = 43 << 0;
    pub const PERFSEL1_SRAM3_ACCESS_VAL: u32 = 43;
    pub const PERFSEL1_SRAM2_STALL_UPSTREAM: u32 = 44 << 0;
    pub const PERFSEL1_SRAM2_STALL_UPSTREAM_VAL: u32 = 44;
    pub const PERFSEL1_SRAM2_STALL_DOWNSTREAM: u32 = 45 << 0;
    pub const PERFSEL1_SRAM2_STALL_DOWNSTREAM_VAL: u32 = 45;
    pub const PERFSEL1_SRAM2_ACCESS_CONTESTED: u32 = 46 << 0;
    pub const PERFSEL1_SRAM2_ACCESS_CONTESTED_VAL: u32 = 46;
    pub const PERFSEL1_SRAM2_ACCESS: u32 = 47 << 0;
    pub const PERFSEL1_SRAM2_ACCESS_VAL: u32 = 47;
    pub const PERFSEL1_SRAM1_STALL_UPSTREAM: u32 = 48 << 0;
    pub const PERFSEL1_SRAM1_STALL_UPSTREAM_VAL: u32 = 48;
    pub const PERFSEL1_SRAM1_STALL_DOWNSTREAM: u32 = 49 << 0;
    pub const PERFSEL1_SRAM1_STALL_DOWNSTREAM_VAL: u32 = 49;
    pub const PERFSEL1_SRAM1_ACCESS_CONTESTED: u32 = 50 << 0;
    pub const PERFSEL1_SRAM1_ACCESS_CONTESTED_VAL: u32 = 50;
    pub const PERFSEL1_SRAM1_ACCESS: u32 = 51 << 0;
    pub const PERFSEL1_SRAM1_ACCESS_VAL: u32 = 51;
    pub const PERFSEL1_SRAM0_STALL_UPSTREAM: u32 = 52 << 0;
    pub const PERFSEL1_SRAM0_STALL_UPSTREAM_VAL: u32 = 52;
    pub const PERFSEL1_SRAM0_STALL_DOWNSTREAM: u32 = 53 << 0;
    pub const PERFSEL1_SRAM0_STALL_DOWNSTREAM_VAL: u32 = 53;
    pub const PERFSEL1_SRAM0_ACCESS_CONTESTED: u32 = 54 << 0;
    pub const PERFSEL1_SRAM0_ACCESS_CONTESTED_VAL: u32 = 54;
    pub const PERFSEL1_SRAM0_ACCESS: u32 = 55 << 0;
    pub const PERFSEL1_SRAM0_ACCESS_VAL: u32 = 55;
    pub const PERFSEL1_XIP_MAIN1_STALL_UPSTREAM: u32 = 56 << 0;
    pub const PERFSEL1_XIP_MAIN1_STALL_UPSTREAM_VAL: u32 = 56;
    pub const PERFSEL1_XIP_MAIN1_STALL_DOWNSTREAM: u32 = 57 << 0;
    pub const PERFSEL1_XIP_MAIN1_STALL_DOWNSTREAM_VAL: u32 = 57;
    pub const PERFSEL1_XIP_MAIN1_ACCESS_CONTESTED: u32 = 58 << 0;
    pub const PERFSEL1_XIP_MAIN1_ACCESS_CONTESTED_VAL: u32 = 58;
    pub const PERFSEL1_XIP_MAIN1_ACCESS: u32 = 59 << 0;
    pub const PERFSEL1_XIP_MAIN1_ACCESS_VAL: u32 = 59;
    pub const PERFSEL1_XIP_MAIN0_STALL_UPSTREAM: u32 = 60 << 0;
    pub const PERFSEL1_XIP_MAIN0_STALL_UPSTREAM_VAL: u32 = 60;
    pub const PERFSEL1_XIP_MAIN0_STALL_DOWNSTREAM: u32 = 61 << 0;
    pub const PERFSEL1_XIP_MAIN0_STALL_DOWNSTREAM_VAL: u32 = 61;
    pub const PERFSEL1_XIP_MAIN0_ACCESS_CONTESTED: u32 = 62 << 0;
    pub const PERFSEL1_XIP_MAIN0_ACCESS_CONTESTED_VAL: u32 = 62;
    pub const PERFSEL1_XIP_MAIN0_ACCESS: u32 = 63 << 0;
    pub const PERFSEL1_XIP_MAIN0_ACCESS_VAL: u32 = 63;
    pub const PERFSEL1_ROM_STALL_UPSTREAM: u32 = 64 << 0;
    pub const PERFSEL1_ROM_STALL_UPSTREAM_VAL: u32 = 64;
    pub const PERFSEL1_ROM_STALL_DOWNSTREAM: u32 = 65 << 0;
    pub const PERFSEL1_ROM_STALL_DOWNSTREAM_VAL: u32 = 65;
    pub const PERFSEL1_ROM_ACCESS_CONTESTED: u32 = 66 << 0;
    pub const PERFSEL1_ROM_ACCESS_CONTESTED_VAL: u32 = 66;
    pub const PERFSEL1_ROM_ACCESS: u32 = 67 << 0;
    pub const PERFSEL1_ROM_ACCESS_VAL: u32 = 67;

    // ======== Field Accessors ========

    /// **PERFSEL1** `[6:0]` (RW)
    ///
    /// Select an event for PERFCTR1. For each downstream port of the main crossbar, four events are available: ACCESS, an access took place; ACCESS_CONTESTED, an access took place that previously stalled due to contention from other masters; STALL_DOWNSTREAM, count cycles where any master stalled due to a stall on the downstream bus; STALL_UPSTREAM, count cycles where any master stalled for any reason, including contention from other masters.
    #[inline(always)]
    pub fn perfsel1(&self) -> u32 {
        (self.value.read_bits() & Self::PERFSEL1_MASK) >> Self::PERFSEL1_OFFSET
    }

    /// Write **PERFSEL1** `[6:0]` (RW)
    ///
    /// Select an event for PERFCTR1. For each downstream port of the main crossbar, four events are available: ACCESS, an access took place; ACCESS_CONTESTED, an access took place that previously stalled due to contention from other masters; STALL_DOWNSTREAM, count cycles where any master stalled due to a stall on the downstream bus; STALL_UPSTREAM, count cycles where any master stalled for any reason, including contention from other masters.
    #[inline(always)]
    pub fn write_perfsel1(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::PERFSEL1_MASK) | ((value << Self::PERFSEL1_OFFSET) & Self::PERFSEL1_MASK));
    }

}

/// PERFSEL1 value type for batch read/modify/write operations
///
/// Use `perfsel1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `perfsel1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Perfsel1Val {
    bits: u32,
}

impl Perfsel1Val {
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
        Self { bits: 0x0000001f }
    }

    /// Read **PERFSEL1** `[6:0]`
    ///
    /// Select an event for PERFCTR1. For each downstream port of the main crossbar, four events are available: ACCESS, an access took place; ACCESS_CONTESTED, an access took place that previously stalled due to contention from other masters; STALL_DOWNSTREAM, count cycles where any master stalled due to a stall on the downstream bus; STALL_UPSTREAM, count cycles where any master stalled for any reason, including contention from other masters.
    #[inline(always)]
    pub const fn perfsel1(&self) -> u32 {
        (self.bits & Perfsel1::PERFSEL1_MASK) >> Perfsel1::PERFSEL1_OFFSET
    }

    /// Set **PERFSEL1** `[6:0]`
    ///
    /// Select an event for PERFCTR1. For each downstream port of the main crossbar, four events are available: ACCESS, an access took place; ACCESS_CONTESTED, an access took place that previously stalled due to contention from other masters; STALL_DOWNSTREAM, count cycles where any master stalled due to a stall on the downstream bus; STALL_UPSTREAM, count cycles where any master stalled for any reason, including contention from other masters.
    #[inline(always)]
    pub const fn with_perfsel1(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Perfsel1::PERFSEL1_MASK) | ((value << Perfsel1::PERFSEL1_OFFSET) & Perfsel1::PERFSEL1_MASK)
        }
    }

}

impl Default for Perfsel1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Perfsel1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Perfsel1Val {
        Perfsel1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Perfsel1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Perfsel1Val) -> Perfsel1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// PERFCTR2 Register
///
/// Bus fabric performance counter 2
///
/// Address offset: 0x001c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **PERFCTR2** `[23:0]` (RW)
///
///   Busfabric saturating performance counter 2
///   Count some event signal from the busfabric arbiters, if PERFCTR_EN is set.
///   Write any value to clear. Select an event to count using PERFSEL2
#[repr(transparent)]
pub struct Perfctr2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Perfctr2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Perfctr2 {
    // ======== Field Constants ========

    /// **PERFCTR2**
    ///
    /// Busfabric saturating performance counter 2
    /// Count some event signal from the busfabric arbiters, if PERFCTR_EN is set.
    /// Write any value to clear. Select an event to count using PERFSEL2
    ///
    /// Bits `[23:0]`, mask `0x00ffffff`
    pub const PERFCTR2_OFFSET: u32 = 0;
    pub const PERFCTR2_MASK: u32 = 0x00ffffff;

    // ======== Field Accessors ========

    /// **PERFCTR2** `[23:0]` (RW)
    ///
    /// Busfabric saturating performance counter 2
    /// Count some event signal from the busfabric arbiters, if PERFCTR_EN is set.
    /// Write any value to clear. Select an event to count using PERFSEL2
    #[inline(always)]
    pub fn perfctr2(&self) -> u32 {
        (self.value.read_bits() & Self::PERFCTR2_MASK) >> Self::PERFCTR2_OFFSET
    }

    /// Write **PERFCTR2** `[23:0]` (RW)
    ///
    /// Busfabric saturating performance counter 2
    /// Count some event signal from the busfabric arbiters, if PERFCTR_EN is set.
    /// Write any value to clear. Select an event to count using PERFSEL2
    #[inline(always)]
    pub fn write_perfctr2(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::PERFCTR2_MASK) | ((value << Self::PERFCTR2_OFFSET) & Self::PERFCTR2_MASK));
    }

}

/// PERFCTR2 value type for batch read/modify/write operations
///
/// Use `perfctr2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `perfctr2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Perfctr2Val {
    bits: u32,
}

impl Perfctr2Val {
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

    /// Read **PERFCTR2** `[23:0]`
    ///
    /// Busfabric saturating performance counter 2
    /// Count some event signal from the busfabric arbiters, if PERFCTR_EN is set.
    /// Write any value to clear. Select an event to count using PERFSEL2
    #[inline(always)]
    pub const fn perfctr2(&self) -> u32 {
        (self.bits & Perfctr2::PERFCTR2_MASK) >> Perfctr2::PERFCTR2_OFFSET
    }

    /// Set **PERFCTR2** `[23:0]`
    ///
    /// Busfabric saturating performance counter 2
    /// Count some event signal from the busfabric arbiters, if PERFCTR_EN is set.
    /// Write any value to clear. Select an event to count using PERFSEL2
    #[inline(always)]
    pub const fn with_perfctr2(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Perfctr2::PERFCTR2_MASK) | ((value << Perfctr2::PERFCTR2_OFFSET) & Perfctr2::PERFCTR2_MASK)
        }
    }

}

impl Default for Perfctr2Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Perfctr2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Perfctr2Val {
        Perfctr2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Perfctr2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Perfctr2Val) -> Perfctr2Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// PERFSEL2 Register
///
/// Bus fabric performance event select for PERFCTR2
///
/// Address offset: 0x0020
/// Reset value: 0x0000001f
///
/// ## Fields
///
/// - **PERFSEL2** `[6:0]` (RW)
///
///   Select an event for PERFCTR2. For each downstream port of the main crossbar, four events are available: ACCESS, an access took place; ACCESS_CONTESTED, an access took place that previously stalled due to contention from other masters; STALL_DOWNSTREAM, count cycles where any master stalled due to a stall on the downstream bus; STALL_UPSTREAM, count cycles where any master stalled for any reason, including contention from other masters.
#[repr(transparent)]
pub struct Perfsel2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Perfsel2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Perfsel2 {
    // ======== Field Constants ========

    /// **PERFSEL2**
    ///
    /// Select an event for PERFCTR2. For each downstream port of the main crossbar, four events are available: ACCESS, an access took place; ACCESS_CONTESTED, an access took place that previously stalled due to contention from other masters; STALL_DOWNSTREAM, count cycles where any master stalled due to a stall on the downstream bus; STALL_UPSTREAM, count cycles where any master stalled for any reason, including contention from other masters.
    ///
    /// Bits `[6:0]`, mask `0x0000007f`
    pub const PERFSEL2_OFFSET: u32 = 0;
    pub const PERFSEL2_MASK: u32 = 0x0000007f;
    pub const PERFSEL2_SIOB_PROC1_STALL_UPSTREAM: u32 = 0 << 0;
    pub const PERFSEL2_SIOB_PROC1_STALL_UPSTREAM_VAL: u32 = 0;
    pub const PERFSEL2_SIOB_PROC1_STALL_DOWNSTREAM: u32 = 1 << 0;
    pub const PERFSEL2_SIOB_PROC1_STALL_DOWNSTREAM_VAL: u32 = 1;
    pub const PERFSEL2_SIOB_PROC1_ACCESS_CONTESTED: u32 = 2 << 0;
    pub const PERFSEL2_SIOB_PROC1_ACCESS_CONTESTED_VAL: u32 = 2;
    pub const PERFSEL2_SIOB_PROC1_ACCESS: u32 = 3 << 0;
    pub const PERFSEL2_SIOB_PROC1_ACCESS_VAL: u32 = 3;
    pub const PERFSEL2_SIOB_PROC0_STALL_UPSTREAM: u32 = 4 << 0;
    pub const PERFSEL2_SIOB_PROC0_STALL_UPSTREAM_VAL: u32 = 4;
    pub const PERFSEL2_SIOB_PROC0_STALL_DOWNSTREAM: u32 = 5 << 0;
    pub const PERFSEL2_SIOB_PROC0_STALL_DOWNSTREAM_VAL: u32 = 5;
    pub const PERFSEL2_SIOB_PROC0_ACCESS_CONTESTED: u32 = 6 << 0;
    pub const PERFSEL2_SIOB_PROC0_ACCESS_CONTESTED_VAL: u32 = 6;
    pub const PERFSEL2_SIOB_PROC0_ACCESS: u32 = 7 << 0;
    pub const PERFSEL2_SIOB_PROC0_ACCESS_VAL: u32 = 7;
    pub const PERFSEL2_APB_STALL_UPSTREAM: u32 = 8 << 0;
    pub const PERFSEL2_APB_STALL_UPSTREAM_VAL: u32 = 8;
    pub const PERFSEL2_APB_STALL_DOWNSTREAM: u32 = 9 << 0;
    pub const PERFSEL2_APB_STALL_DOWNSTREAM_VAL: u32 = 9;
    pub const PERFSEL2_APB_ACCESS_CONTESTED: u32 = 10 << 0;
    pub const PERFSEL2_APB_ACCESS_CONTESTED_VAL: u32 = 10;
    pub const PERFSEL2_APB_ACCESS: u32 = 11 << 0;
    pub const PERFSEL2_APB_ACCESS_VAL: u32 = 11;
    pub const PERFSEL2_FASTPERI_STALL_UPSTREAM: u32 = 12 << 0;
    pub const PERFSEL2_FASTPERI_STALL_UPSTREAM_VAL: u32 = 12;
    pub const PERFSEL2_FASTPERI_STALL_DOWNSTREAM: u32 = 13 << 0;
    pub const PERFSEL2_FASTPERI_STALL_DOWNSTREAM_VAL: u32 = 13;
    pub const PERFSEL2_FASTPERI_ACCESS_CONTESTED: u32 = 14 << 0;
    pub const PERFSEL2_FASTPERI_ACCESS_CONTESTED_VAL: u32 = 14;
    pub const PERFSEL2_FASTPERI_ACCESS: u32 = 15 << 0;
    pub const PERFSEL2_FASTPERI_ACCESS_VAL: u32 = 15;
    pub const PERFSEL2_SRAM9_STALL_UPSTREAM: u32 = 16 << 0;
    pub const PERFSEL2_SRAM9_STALL_UPSTREAM_VAL: u32 = 16;
    pub const PERFSEL2_SRAM9_STALL_DOWNSTREAM: u32 = 17 << 0;
    pub const PERFSEL2_SRAM9_STALL_DOWNSTREAM_VAL: u32 = 17;
    pub const PERFSEL2_SRAM9_ACCESS_CONTESTED: u32 = 18 << 0;
    pub const PERFSEL2_SRAM9_ACCESS_CONTESTED_VAL: u32 = 18;
    pub const PERFSEL2_SRAM9_ACCESS: u32 = 19 << 0;
    pub const PERFSEL2_SRAM9_ACCESS_VAL: u32 = 19;
    pub const PERFSEL2_SRAM8_STALL_UPSTREAM: u32 = 20 << 0;
    pub const PERFSEL2_SRAM8_STALL_UPSTREAM_VAL: u32 = 20;
    pub const PERFSEL2_SRAM8_STALL_DOWNSTREAM: u32 = 21 << 0;
    pub const PERFSEL2_SRAM8_STALL_DOWNSTREAM_VAL: u32 = 21;
    pub const PERFSEL2_SRAM8_ACCESS_CONTESTED: u32 = 22 << 0;
    pub const PERFSEL2_SRAM8_ACCESS_CONTESTED_VAL: u32 = 22;
    pub const PERFSEL2_SRAM8_ACCESS: u32 = 23 << 0;
    pub const PERFSEL2_SRAM8_ACCESS_VAL: u32 = 23;
    pub const PERFSEL2_SRAM7_STALL_UPSTREAM: u32 = 24 << 0;
    pub const PERFSEL2_SRAM7_STALL_UPSTREAM_VAL: u32 = 24;
    pub const PERFSEL2_SRAM7_STALL_DOWNSTREAM: u32 = 25 << 0;
    pub const PERFSEL2_SRAM7_STALL_DOWNSTREAM_VAL: u32 = 25;
    pub const PERFSEL2_SRAM7_ACCESS_CONTESTED: u32 = 26 << 0;
    pub const PERFSEL2_SRAM7_ACCESS_CONTESTED_VAL: u32 = 26;
    pub const PERFSEL2_SRAM7_ACCESS: u32 = 27 << 0;
    pub const PERFSEL2_SRAM7_ACCESS_VAL: u32 = 27;
    pub const PERFSEL2_SRAM6_STALL_UPSTREAM: u32 = 28 << 0;
    pub const PERFSEL2_SRAM6_STALL_UPSTREAM_VAL: u32 = 28;
    pub const PERFSEL2_SRAM6_STALL_DOWNSTREAM: u32 = 29 << 0;
    pub const PERFSEL2_SRAM6_STALL_DOWNSTREAM_VAL: u32 = 29;
    pub const PERFSEL2_SRAM6_ACCESS_CONTESTED: u32 = 30 << 0;
    pub const PERFSEL2_SRAM6_ACCESS_CONTESTED_VAL: u32 = 30;
    pub const PERFSEL2_SRAM6_ACCESS: u32 = 31 << 0;
    pub const PERFSEL2_SRAM6_ACCESS_VAL: u32 = 31;
    pub const PERFSEL2_SRAM5_STALL_UPSTREAM: u32 = 32 << 0;
    pub const PERFSEL2_SRAM5_STALL_UPSTREAM_VAL: u32 = 32;
    pub const PERFSEL2_SRAM5_STALL_DOWNSTREAM: u32 = 33 << 0;
    pub const PERFSEL2_SRAM5_STALL_DOWNSTREAM_VAL: u32 = 33;
    pub const PERFSEL2_SRAM5_ACCESS_CONTESTED: u32 = 34 << 0;
    pub const PERFSEL2_SRAM5_ACCESS_CONTESTED_VAL: u32 = 34;
    pub const PERFSEL2_SRAM5_ACCESS: u32 = 35 << 0;
    pub const PERFSEL2_SRAM5_ACCESS_VAL: u32 = 35;
    pub const PERFSEL2_SRAM4_STALL_UPSTREAM: u32 = 36 << 0;
    pub const PERFSEL2_SRAM4_STALL_UPSTREAM_VAL: u32 = 36;
    pub const PERFSEL2_SRAM4_STALL_DOWNSTREAM: u32 = 37 << 0;
    pub const PERFSEL2_SRAM4_STALL_DOWNSTREAM_VAL: u32 = 37;
    pub const PERFSEL2_SRAM4_ACCESS_CONTESTED: u32 = 38 << 0;
    pub const PERFSEL2_SRAM4_ACCESS_CONTESTED_VAL: u32 = 38;
    pub const PERFSEL2_SRAM4_ACCESS: u32 = 39 << 0;
    pub const PERFSEL2_SRAM4_ACCESS_VAL: u32 = 39;
    pub const PERFSEL2_SRAM3_STALL_UPSTREAM: u32 = 40 << 0;
    pub const PERFSEL2_SRAM3_STALL_UPSTREAM_VAL: u32 = 40;
    pub const PERFSEL2_SRAM3_STALL_DOWNSTREAM: u32 = 41 << 0;
    pub const PERFSEL2_SRAM3_STALL_DOWNSTREAM_VAL: u32 = 41;
    pub const PERFSEL2_SRAM3_ACCESS_CONTESTED: u32 = 42 << 0;
    pub const PERFSEL2_SRAM3_ACCESS_CONTESTED_VAL: u32 = 42;
    pub const PERFSEL2_SRAM3_ACCESS: u32 = 43 << 0;
    pub const PERFSEL2_SRAM3_ACCESS_VAL: u32 = 43;
    pub const PERFSEL2_SRAM2_STALL_UPSTREAM: u32 = 44 << 0;
    pub const PERFSEL2_SRAM2_STALL_UPSTREAM_VAL: u32 = 44;
    pub const PERFSEL2_SRAM2_STALL_DOWNSTREAM: u32 = 45 << 0;
    pub const PERFSEL2_SRAM2_STALL_DOWNSTREAM_VAL: u32 = 45;
    pub const PERFSEL2_SRAM2_ACCESS_CONTESTED: u32 = 46 << 0;
    pub const PERFSEL2_SRAM2_ACCESS_CONTESTED_VAL: u32 = 46;
    pub const PERFSEL2_SRAM2_ACCESS: u32 = 47 << 0;
    pub const PERFSEL2_SRAM2_ACCESS_VAL: u32 = 47;
    pub const PERFSEL2_SRAM1_STALL_UPSTREAM: u32 = 48 << 0;
    pub const PERFSEL2_SRAM1_STALL_UPSTREAM_VAL: u32 = 48;
    pub const PERFSEL2_SRAM1_STALL_DOWNSTREAM: u32 = 49 << 0;
    pub const PERFSEL2_SRAM1_STALL_DOWNSTREAM_VAL: u32 = 49;
    pub const PERFSEL2_SRAM1_ACCESS_CONTESTED: u32 = 50 << 0;
    pub const PERFSEL2_SRAM1_ACCESS_CONTESTED_VAL: u32 = 50;
    pub const PERFSEL2_SRAM1_ACCESS: u32 = 51 << 0;
    pub const PERFSEL2_SRAM1_ACCESS_VAL: u32 = 51;
    pub const PERFSEL2_SRAM0_STALL_UPSTREAM: u32 = 52 << 0;
    pub const PERFSEL2_SRAM0_STALL_UPSTREAM_VAL: u32 = 52;
    pub const PERFSEL2_SRAM0_STALL_DOWNSTREAM: u32 = 53 << 0;
    pub const PERFSEL2_SRAM0_STALL_DOWNSTREAM_VAL: u32 = 53;
    pub const PERFSEL2_SRAM0_ACCESS_CONTESTED: u32 = 54 << 0;
    pub const PERFSEL2_SRAM0_ACCESS_CONTESTED_VAL: u32 = 54;
    pub const PERFSEL2_SRAM0_ACCESS: u32 = 55 << 0;
    pub const PERFSEL2_SRAM0_ACCESS_VAL: u32 = 55;
    pub const PERFSEL2_XIP_MAIN1_STALL_UPSTREAM: u32 = 56 << 0;
    pub const PERFSEL2_XIP_MAIN1_STALL_UPSTREAM_VAL: u32 = 56;
    pub const PERFSEL2_XIP_MAIN1_STALL_DOWNSTREAM: u32 = 57 << 0;
    pub const PERFSEL2_XIP_MAIN1_STALL_DOWNSTREAM_VAL: u32 = 57;
    pub const PERFSEL2_XIP_MAIN1_ACCESS_CONTESTED: u32 = 58 << 0;
    pub const PERFSEL2_XIP_MAIN1_ACCESS_CONTESTED_VAL: u32 = 58;
    pub const PERFSEL2_XIP_MAIN1_ACCESS: u32 = 59 << 0;
    pub const PERFSEL2_XIP_MAIN1_ACCESS_VAL: u32 = 59;
    pub const PERFSEL2_XIP_MAIN0_STALL_UPSTREAM: u32 = 60 << 0;
    pub const PERFSEL2_XIP_MAIN0_STALL_UPSTREAM_VAL: u32 = 60;
    pub const PERFSEL2_XIP_MAIN0_STALL_DOWNSTREAM: u32 = 61 << 0;
    pub const PERFSEL2_XIP_MAIN0_STALL_DOWNSTREAM_VAL: u32 = 61;
    pub const PERFSEL2_XIP_MAIN0_ACCESS_CONTESTED: u32 = 62 << 0;
    pub const PERFSEL2_XIP_MAIN0_ACCESS_CONTESTED_VAL: u32 = 62;
    pub const PERFSEL2_XIP_MAIN0_ACCESS: u32 = 63 << 0;
    pub const PERFSEL2_XIP_MAIN0_ACCESS_VAL: u32 = 63;
    pub const PERFSEL2_ROM_STALL_UPSTREAM: u32 = 64 << 0;
    pub const PERFSEL2_ROM_STALL_UPSTREAM_VAL: u32 = 64;
    pub const PERFSEL2_ROM_STALL_DOWNSTREAM: u32 = 65 << 0;
    pub const PERFSEL2_ROM_STALL_DOWNSTREAM_VAL: u32 = 65;
    pub const PERFSEL2_ROM_ACCESS_CONTESTED: u32 = 66 << 0;
    pub const PERFSEL2_ROM_ACCESS_CONTESTED_VAL: u32 = 66;
    pub const PERFSEL2_ROM_ACCESS: u32 = 67 << 0;
    pub const PERFSEL2_ROM_ACCESS_VAL: u32 = 67;

    // ======== Field Accessors ========

    /// **PERFSEL2** `[6:0]` (RW)
    ///
    /// Select an event for PERFCTR2. For each downstream port of the main crossbar, four events are available: ACCESS, an access took place; ACCESS_CONTESTED, an access took place that previously stalled due to contention from other masters; STALL_DOWNSTREAM, count cycles where any master stalled due to a stall on the downstream bus; STALL_UPSTREAM, count cycles where any master stalled for any reason, including contention from other masters.
    #[inline(always)]
    pub fn perfsel2(&self) -> u32 {
        (self.value.read_bits() & Self::PERFSEL2_MASK) >> Self::PERFSEL2_OFFSET
    }

    /// Write **PERFSEL2** `[6:0]` (RW)
    ///
    /// Select an event for PERFCTR2. For each downstream port of the main crossbar, four events are available: ACCESS, an access took place; ACCESS_CONTESTED, an access took place that previously stalled due to contention from other masters; STALL_DOWNSTREAM, count cycles where any master stalled due to a stall on the downstream bus; STALL_UPSTREAM, count cycles where any master stalled for any reason, including contention from other masters.
    #[inline(always)]
    pub fn write_perfsel2(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::PERFSEL2_MASK) | ((value << Self::PERFSEL2_OFFSET) & Self::PERFSEL2_MASK));
    }

}

/// PERFSEL2 value type for batch read/modify/write operations
///
/// Use `perfsel2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `perfsel2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Perfsel2Val {
    bits: u32,
}

impl Perfsel2Val {
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
        Self { bits: 0x0000001f }
    }

    /// Read **PERFSEL2** `[6:0]`
    ///
    /// Select an event for PERFCTR2. For each downstream port of the main crossbar, four events are available: ACCESS, an access took place; ACCESS_CONTESTED, an access took place that previously stalled due to contention from other masters; STALL_DOWNSTREAM, count cycles where any master stalled due to a stall on the downstream bus; STALL_UPSTREAM, count cycles where any master stalled for any reason, including contention from other masters.
    #[inline(always)]
    pub const fn perfsel2(&self) -> u32 {
        (self.bits & Perfsel2::PERFSEL2_MASK) >> Perfsel2::PERFSEL2_OFFSET
    }

    /// Set **PERFSEL2** `[6:0]`
    ///
    /// Select an event for PERFCTR2. For each downstream port of the main crossbar, four events are available: ACCESS, an access took place; ACCESS_CONTESTED, an access took place that previously stalled due to contention from other masters; STALL_DOWNSTREAM, count cycles where any master stalled due to a stall on the downstream bus; STALL_UPSTREAM, count cycles where any master stalled for any reason, including contention from other masters.
    #[inline(always)]
    pub const fn with_perfsel2(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Perfsel2::PERFSEL2_MASK) | ((value << Perfsel2::PERFSEL2_OFFSET) & Perfsel2::PERFSEL2_MASK)
        }
    }

}

impl Default for Perfsel2Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Perfsel2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Perfsel2Val {
        Perfsel2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Perfsel2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Perfsel2Val) -> Perfsel2Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// PERFCTR3 Register
///
/// Bus fabric performance counter 3
///
/// Address offset: 0x0024
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **PERFCTR3** `[23:0]` (RW)
///
///   Busfabric saturating performance counter 3
///   Count some event signal from the busfabric arbiters, if PERFCTR_EN is set.
///   Write any value to clear. Select an event to count using PERFSEL3
#[repr(transparent)]
pub struct Perfctr3 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Perfctr3 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Perfctr3 {
    // ======== Field Constants ========

    /// **PERFCTR3**
    ///
    /// Busfabric saturating performance counter 3
    /// Count some event signal from the busfabric arbiters, if PERFCTR_EN is set.
    /// Write any value to clear. Select an event to count using PERFSEL3
    ///
    /// Bits `[23:0]`, mask `0x00ffffff`
    pub const PERFCTR3_OFFSET: u32 = 0;
    pub const PERFCTR3_MASK: u32 = 0x00ffffff;

    // ======== Field Accessors ========

    /// **PERFCTR3** `[23:0]` (RW)
    ///
    /// Busfabric saturating performance counter 3
    /// Count some event signal from the busfabric arbiters, if PERFCTR_EN is set.
    /// Write any value to clear. Select an event to count using PERFSEL3
    #[inline(always)]
    pub fn perfctr3(&self) -> u32 {
        (self.value.read_bits() & Self::PERFCTR3_MASK) >> Self::PERFCTR3_OFFSET
    }

    /// Write **PERFCTR3** `[23:0]` (RW)
    ///
    /// Busfabric saturating performance counter 3
    /// Count some event signal from the busfabric arbiters, if PERFCTR_EN is set.
    /// Write any value to clear. Select an event to count using PERFSEL3
    #[inline(always)]
    pub fn write_perfctr3(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::PERFCTR3_MASK) | ((value << Self::PERFCTR3_OFFSET) & Self::PERFCTR3_MASK));
    }

}

/// PERFCTR3 value type for batch read/modify/write operations
///
/// Use `perfctr3.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `perfctr3.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Perfctr3Val {
    bits: u32,
}

impl Perfctr3Val {
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

    /// Read **PERFCTR3** `[23:0]`
    ///
    /// Busfabric saturating performance counter 3
    /// Count some event signal from the busfabric arbiters, if PERFCTR_EN is set.
    /// Write any value to clear. Select an event to count using PERFSEL3
    #[inline(always)]
    pub const fn perfctr3(&self) -> u32 {
        (self.bits & Perfctr3::PERFCTR3_MASK) >> Perfctr3::PERFCTR3_OFFSET
    }

    /// Set **PERFCTR3** `[23:0]`
    ///
    /// Busfabric saturating performance counter 3
    /// Count some event signal from the busfabric arbiters, if PERFCTR_EN is set.
    /// Write any value to clear. Select an event to count using PERFSEL3
    #[inline(always)]
    pub const fn with_perfctr3(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Perfctr3::PERFCTR3_MASK) | ((value << Perfctr3::PERFCTR3_OFFSET) & Perfctr3::PERFCTR3_MASK)
        }
    }

}

impl Default for Perfctr3Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Perfctr3 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Perfctr3Val {
        Perfctr3Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Perfctr3Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Perfctr3Val) -> Perfctr3Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// PERFSEL3 Register
///
/// Bus fabric performance event select for PERFCTR3
///
/// Address offset: 0x0028
/// Reset value: 0x0000001f
///
/// ## Fields
///
/// - **PERFSEL3** `[6:0]` (RW)
///
///   Select an event for PERFCTR3. For each downstream port of the main crossbar, four events are available: ACCESS, an access took place; ACCESS_CONTESTED, an access took place that previously stalled due to contention from other masters; STALL_DOWNSTREAM, count cycles where any master stalled due to a stall on the downstream bus; STALL_UPSTREAM, count cycles where any master stalled for any reason, including contention from other masters.
#[repr(transparent)]
pub struct Perfsel3 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Perfsel3 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Perfsel3 {
    // ======== Field Constants ========

    /// **PERFSEL3**
    ///
    /// Select an event for PERFCTR3. For each downstream port of the main crossbar, four events are available: ACCESS, an access took place; ACCESS_CONTESTED, an access took place that previously stalled due to contention from other masters; STALL_DOWNSTREAM, count cycles where any master stalled due to a stall on the downstream bus; STALL_UPSTREAM, count cycles where any master stalled for any reason, including contention from other masters.
    ///
    /// Bits `[6:0]`, mask `0x0000007f`
    pub const PERFSEL3_OFFSET: u32 = 0;
    pub const PERFSEL3_MASK: u32 = 0x0000007f;
    pub const PERFSEL3_SIOB_PROC1_STALL_UPSTREAM: u32 = 0 << 0;
    pub const PERFSEL3_SIOB_PROC1_STALL_UPSTREAM_VAL: u32 = 0;
    pub const PERFSEL3_SIOB_PROC1_STALL_DOWNSTREAM: u32 = 1 << 0;
    pub const PERFSEL3_SIOB_PROC1_STALL_DOWNSTREAM_VAL: u32 = 1;
    pub const PERFSEL3_SIOB_PROC1_ACCESS_CONTESTED: u32 = 2 << 0;
    pub const PERFSEL3_SIOB_PROC1_ACCESS_CONTESTED_VAL: u32 = 2;
    pub const PERFSEL3_SIOB_PROC1_ACCESS: u32 = 3 << 0;
    pub const PERFSEL3_SIOB_PROC1_ACCESS_VAL: u32 = 3;
    pub const PERFSEL3_SIOB_PROC0_STALL_UPSTREAM: u32 = 4 << 0;
    pub const PERFSEL3_SIOB_PROC0_STALL_UPSTREAM_VAL: u32 = 4;
    pub const PERFSEL3_SIOB_PROC0_STALL_DOWNSTREAM: u32 = 5 << 0;
    pub const PERFSEL3_SIOB_PROC0_STALL_DOWNSTREAM_VAL: u32 = 5;
    pub const PERFSEL3_SIOB_PROC0_ACCESS_CONTESTED: u32 = 6 << 0;
    pub const PERFSEL3_SIOB_PROC0_ACCESS_CONTESTED_VAL: u32 = 6;
    pub const PERFSEL3_SIOB_PROC0_ACCESS: u32 = 7 << 0;
    pub const PERFSEL3_SIOB_PROC0_ACCESS_VAL: u32 = 7;
    pub const PERFSEL3_APB_STALL_UPSTREAM: u32 = 8 << 0;
    pub const PERFSEL3_APB_STALL_UPSTREAM_VAL: u32 = 8;
    pub const PERFSEL3_APB_STALL_DOWNSTREAM: u32 = 9 << 0;
    pub const PERFSEL3_APB_STALL_DOWNSTREAM_VAL: u32 = 9;
    pub const PERFSEL3_APB_ACCESS_CONTESTED: u32 = 10 << 0;
    pub const PERFSEL3_APB_ACCESS_CONTESTED_VAL: u32 = 10;
    pub const PERFSEL3_APB_ACCESS: u32 = 11 << 0;
    pub const PERFSEL3_APB_ACCESS_VAL: u32 = 11;
    pub const PERFSEL3_FASTPERI_STALL_UPSTREAM: u32 = 12 << 0;
    pub const PERFSEL3_FASTPERI_STALL_UPSTREAM_VAL: u32 = 12;
    pub const PERFSEL3_FASTPERI_STALL_DOWNSTREAM: u32 = 13 << 0;
    pub const PERFSEL3_FASTPERI_STALL_DOWNSTREAM_VAL: u32 = 13;
    pub const PERFSEL3_FASTPERI_ACCESS_CONTESTED: u32 = 14 << 0;
    pub const PERFSEL3_FASTPERI_ACCESS_CONTESTED_VAL: u32 = 14;
    pub const PERFSEL3_FASTPERI_ACCESS: u32 = 15 << 0;
    pub const PERFSEL3_FASTPERI_ACCESS_VAL: u32 = 15;
    pub const PERFSEL3_SRAM9_STALL_UPSTREAM: u32 = 16 << 0;
    pub const PERFSEL3_SRAM9_STALL_UPSTREAM_VAL: u32 = 16;
    pub const PERFSEL3_SRAM9_STALL_DOWNSTREAM: u32 = 17 << 0;
    pub const PERFSEL3_SRAM9_STALL_DOWNSTREAM_VAL: u32 = 17;
    pub const PERFSEL3_SRAM9_ACCESS_CONTESTED: u32 = 18 << 0;
    pub const PERFSEL3_SRAM9_ACCESS_CONTESTED_VAL: u32 = 18;
    pub const PERFSEL3_SRAM9_ACCESS: u32 = 19 << 0;
    pub const PERFSEL3_SRAM9_ACCESS_VAL: u32 = 19;
    pub const PERFSEL3_SRAM8_STALL_UPSTREAM: u32 = 20 << 0;
    pub const PERFSEL3_SRAM8_STALL_UPSTREAM_VAL: u32 = 20;
    pub const PERFSEL3_SRAM8_STALL_DOWNSTREAM: u32 = 21 << 0;
    pub const PERFSEL3_SRAM8_STALL_DOWNSTREAM_VAL: u32 = 21;
    pub const PERFSEL3_SRAM8_ACCESS_CONTESTED: u32 = 22 << 0;
    pub const PERFSEL3_SRAM8_ACCESS_CONTESTED_VAL: u32 = 22;
    pub const PERFSEL3_SRAM8_ACCESS: u32 = 23 << 0;
    pub const PERFSEL3_SRAM8_ACCESS_VAL: u32 = 23;
    pub const PERFSEL3_SRAM7_STALL_UPSTREAM: u32 = 24 << 0;
    pub const PERFSEL3_SRAM7_STALL_UPSTREAM_VAL: u32 = 24;
    pub const PERFSEL3_SRAM7_STALL_DOWNSTREAM: u32 = 25 << 0;
    pub const PERFSEL3_SRAM7_STALL_DOWNSTREAM_VAL: u32 = 25;
    pub const PERFSEL3_SRAM7_ACCESS_CONTESTED: u32 = 26 << 0;
    pub const PERFSEL3_SRAM7_ACCESS_CONTESTED_VAL: u32 = 26;
    pub const PERFSEL3_SRAM7_ACCESS: u32 = 27 << 0;
    pub const PERFSEL3_SRAM7_ACCESS_VAL: u32 = 27;
    pub const PERFSEL3_SRAM6_STALL_UPSTREAM: u32 = 28 << 0;
    pub const PERFSEL3_SRAM6_STALL_UPSTREAM_VAL: u32 = 28;
    pub const PERFSEL3_SRAM6_STALL_DOWNSTREAM: u32 = 29 << 0;
    pub const PERFSEL3_SRAM6_STALL_DOWNSTREAM_VAL: u32 = 29;
    pub const PERFSEL3_SRAM6_ACCESS_CONTESTED: u32 = 30 << 0;
    pub const PERFSEL3_SRAM6_ACCESS_CONTESTED_VAL: u32 = 30;
    pub const PERFSEL3_SRAM6_ACCESS: u32 = 31 << 0;
    pub const PERFSEL3_SRAM6_ACCESS_VAL: u32 = 31;
    pub const PERFSEL3_SRAM5_STALL_UPSTREAM: u32 = 32 << 0;
    pub const PERFSEL3_SRAM5_STALL_UPSTREAM_VAL: u32 = 32;
    pub const PERFSEL3_SRAM5_STALL_DOWNSTREAM: u32 = 33 << 0;
    pub const PERFSEL3_SRAM5_STALL_DOWNSTREAM_VAL: u32 = 33;
    pub const PERFSEL3_SRAM5_ACCESS_CONTESTED: u32 = 34 << 0;
    pub const PERFSEL3_SRAM5_ACCESS_CONTESTED_VAL: u32 = 34;
    pub const PERFSEL3_SRAM5_ACCESS: u32 = 35 << 0;
    pub const PERFSEL3_SRAM5_ACCESS_VAL: u32 = 35;
    pub const PERFSEL3_SRAM4_STALL_UPSTREAM: u32 = 36 << 0;
    pub const PERFSEL3_SRAM4_STALL_UPSTREAM_VAL: u32 = 36;
    pub const PERFSEL3_SRAM4_STALL_DOWNSTREAM: u32 = 37 << 0;
    pub const PERFSEL3_SRAM4_STALL_DOWNSTREAM_VAL: u32 = 37;
    pub const PERFSEL3_SRAM4_ACCESS_CONTESTED: u32 = 38 << 0;
    pub const PERFSEL3_SRAM4_ACCESS_CONTESTED_VAL: u32 = 38;
    pub const PERFSEL3_SRAM4_ACCESS: u32 = 39 << 0;
    pub const PERFSEL3_SRAM4_ACCESS_VAL: u32 = 39;
    pub const PERFSEL3_SRAM3_STALL_UPSTREAM: u32 = 40 << 0;
    pub const PERFSEL3_SRAM3_STALL_UPSTREAM_VAL: u32 = 40;
    pub const PERFSEL3_SRAM3_STALL_DOWNSTREAM: u32 = 41 << 0;
    pub const PERFSEL3_SRAM3_STALL_DOWNSTREAM_VAL: u32 = 41;
    pub const PERFSEL3_SRAM3_ACCESS_CONTESTED: u32 = 42 << 0;
    pub const PERFSEL3_SRAM3_ACCESS_CONTESTED_VAL: u32 = 42;
    pub const PERFSEL3_SRAM3_ACCESS: u32 = 43 << 0;
    pub const PERFSEL3_SRAM3_ACCESS_VAL: u32 = 43;
    pub const PERFSEL3_SRAM2_STALL_UPSTREAM: u32 = 44 << 0;
    pub const PERFSEL3_SRAM2_STALL_UPSTREAM_VAL: u32 = 44;
    pub const PERFSEL3_SRAM2_STALL_DOWNSTREAM: u32 = 45 << 0;
    pub const PERFSEL3_SRAM2_STALL_DOWNSTREAM_VAL: u32 = 45;
    pub const PERFSEL3_SRAM2_ACCESS_CONTESTED: u32 = 46 << 0;
    pub const PERFSEL3_SRAM2_ACCESS_CONTESTED_VAL: u32 = 46;
    pub const PERFSEL3_SRAM2_ACCESS: u32 = 47 << 0;
    pub const PERFSEL3_SRAM2_ACCESS_VAL: u32 = 47;
    pub const PERFSEL3_SRAM1_STALL_UPSTREAM: u32 = 48 << 0;
    pub const PERFSEL3_SRAM1_STALL_UPSTREAM_VAL: u32 = 48;
    pub const PERFSEL3_SRAM1_STALL_DOWNSTREAM: u32 = 49 << 0;
    pub const PERFSEL3_SRAM1_STALL_DOWNSTREAM_VAL: u32 = 49;
    pub const PERFSEL3_SRAM1_ACCESS_CONTESTED: u32 = 50 << 0;
    pub const PERFSEL3_SRAM1_ACCESS_CONTESTED_VAL: u32 = 50;
    pub const PERFSEL3_SRAM1_ACCESS: u32 = 51 << 0;
    pub const PERFSEL3_SRAM1_ACCESS_VAL: u32 = 51;
    pub const PERFSEL3_SRAM0_STALL_UPSTREAM: u32 = 52 << 0;
    pub const PERFSEL3_SRAM0_STALL_UPSTREAM_VAL: u32 = 52;
    pub const PERFSEL3_SRAM0_STALL_DOWNSTREAM: u32 = 53 << 0;
    pub const PERFSEL3_SRAM0_STALL_DOWNSTREAM_VAL: u32 = 53;
    pub const PERFSEL3_SRAM0_ACCESS_CONTESTED: u32 = 54 << 0;
    pub const PERFSEL3_SRAM0_ACCESS_CONTESTED_VAL: u32 = 54;
    pub const PERFSEL3_SRAM0_ACCESS: u32 = 55 << 0;
    pub const PERFSEL3_SRAM0_ACCESS_VAL: u32 = 55;
    pub const PERFSEL3_XIP_MAIN1_STALL_UPSTREAM: u32 = 56 << 0;
    pub const PERFSEL3_XIP_MAIN1_STALL_UPSTREAM_VAL: u32 = 56;
    pub const PERFSEL3_XIP_MAIN1_STALL_DOWNSTREAM: u32 = 57 << 0;
    pub const PERFSEL3_XIP_MAIN1_STALL_DOWNSTREAM_VAL: u32 = 57;
    pub const PERFSEL3_XIP_MAIN1_ACCESS_CONTESTED: u32 = 58 << 0;
    pub const PERFSEL3_XIP_MAIN1_ACCESS_CONTESTED_VAL: u32 = 58;
    pub const PERFSEL3_XIP_MAIN1_ACCESS: u32 = 59 << 0;
    pub const PERFSEL3_XIP_MAIN1_ACCESS_VAL: u32 = 59;
    pub const PERFSEL3_XIP_MAIN0_STALL_UPSTREAM: u32 = 60 << 0;
    pub const PERFSEL3_XIP_MAIN0_STALL_UPSTREAM_VAL: u32 = 60;
    pub const PERFSEL3_XIP_MAIN0_STALL_DOWNSTREAM: u32 = 61 << 0;
    pub const PERFSEL3_XIP_MAIN0_STALL_DOWNSTREAM_VAL: u32 = 61;
    pub const PERFSEL3_XIP_MAIN0_ACCESS_CONTESTED: u32 = 62 << 0;
    pub const PERFSEL3_XIP_MAIN0_ACCESS_CONTESTED_VAL: u32 = 62;
    pub const PERFSEL3_XIP_MAIN0_ACCESS: u32 = 63 << 0;
    pub const PERFSEL3_XIP_MAIN0_ACCESS_VAL: u32 = 63;
    pub const PERFSEL3_ROM_STALL_UPSTREAM: u32 = 64 << 0;
    pub const PERFSEL3_ROM_STALL_UPSTREAM_VAL: u32 = 64;
    pub const PERFSEL3_ROM_STALL_DOWNSTREAM: u32 = 65 << 0;
    pub const PERFSEL3_ROM_STALL_DOWNSTREAM_VAL: u32 = 65;
    pub const PERFSEL3_ROM_ACCESS_CONTESTED: u32 = 66 << 0;
    pub const PERFSEL3_ROM_ACCESS_CONTESTED_VAL: u32 = 66;
    pub const PERFSEL3_ROM_ACCESS: u32 = 67 << 0;
    pub const PERFSEL3_ROM_ACCESS_VAL: u32 = 67;

    // ======== Field Accessors ========

    /// **PERFSEL3** `[6:0]` (RW)
    ///
    /// Select an event for PERFCTR3. For each downstream port of the main crossbar, four events are available: ACCESS, an access took place; ACCESS_CONTESTED, an access took place that previously stalled due to contention from other masters; STALL_DOWNSTREAM, count cycles where any master stalled due to a stall on the downstream bus; STALL_UPSTREAM, count cycles where any master stalled for any reason, including contention from other masters.
    #[inline(always)]
    pub fn perfsel3(&self) -> u32 {
        (self.value.read_bits() & Self::PERFSEL3_MASK) >> Self::PERFSEL3_OFFSET
    }

    /// Write **PERFSEL3** `[6:0]` (RW)
    ///
    /// Select an event for PERFCTR3. For each downstream port of the main crossbar, four events are available: ACCESS, an access took place; ACCESS_CONTESTED, an access took place that previously stalled due to contention from other masters; STALL_DOWNSTREAM, count cycles where any master stalled due to a stall on the downstream bus; STALL_UPSTREAM, count cycles where any master stalled for any reason, including contention from other masters.
    #[inline(always)]
    pub fn write_perfsel3(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::PERFSEL3_MASK) | ((value << Self::PERFSEL3_OFFSET) & Self::PERFSEL3_MASK));
    }

}

/// PERFSEL3 value type for batch read/modify/write operations
///
/// Use `perfsel3.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `perfsel3.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Perfsel3Val {
    bits: u32,
}

impl Perfsel3Val {
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
        Self { bits: 0x0000001f }
    }

    /// Read **PERFSEL3** `[6:0]`
    ///
    /// Select an event for PERFCTR3. For each downstream port of the main crossbar, four events are available: ACCESS, an access took place; ACCESS_CONTESTED, an access took place that previously stalled due to contention from other masters; STALL_DOWNSTREAM, count cycles where any master stalled due to a stall on the downstream bus; STALL_UPSTREAM, count cycles where any master stalled for any reason, including contention from other masters.
    #[inline(always)]
    pub const fn perfsel3(&self) -> u32 {
        (self.bits & Perfsel3::PERFSEL3_MASK) >> Perfsel3::PERFSEL3_OFFSET
    }

    /// Set **PERFSEL3** `[6:0]`
    ///
    /// Select an event for PERFCTR3. For each downstream port of the main crossbar, four events are available: ACCESS, an access took place; ACCESS_CONTESTED, an access took place that previously stalled due to contention from other masters; STALL_DOWNSTREAM, count cycles where any master stalled due to a stall on the downstream bus; STALL_UPSTREAM, count cycles where any master stalled for any reason, including contention from other masters.
    #[inline(always)]
    pub const fn with_perfsel3(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Perfsel3::PERFSEL3_MASK) | ((value << Perfsel3::PERFSEL3_OFFSET) & Perfsel3::PERFSEL3_MASK)
        }
    }

}

impl Default for Perfsel3Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Perfsel3 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Perfsel3Val {
        Perfsel3Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Perfsel3Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Perfsel3Val) -> Perfsel3Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// Register block
#[repr(C)]
pub struct RegisterBlock {
    /// 0x0000
    /// Set the priority of each master for bus arbitration.
    pub bus_priority: BusPriority,
    /// 0x0004
    /// Bus priority acknowledge
    pub bus_priority_ack: BusPriorityAck,
    /// 0x0008
    /// Enable the performance counters. If 0, the performance counters do not increment. This can be used to precisely start/stop event sampling around the profiled section of code.
    /// The performance counters are initially disabled, to save energy.
    pub perfctr_en: PerfctrEn,
    /// 0x000c
    /// Bus fabric performance counter 0
    pub perfctr0: Perfctr0,
    /// 0x0010
    /// Bus fabric performance event select for PERFCTR0
    pub perfsel0: Perfsel0,
    /// 0x0014
    /// Bus fabric performance counter 1
    pub perfctr1: Perfctr1,
    /// 0x0018
    /// Bus fabric performance event select for PERFCTR1
    pub perfsel1: Perfsel1,
    /// 0x001c
    /// Bus fabric performance counter 2
    pub perfctr2: Perfctr2,
    /// 0x0020
    /// Bus fabric performance event select for PERFCTR2
    pub perfsel2: Perfsel2,
    /// 0x0024
    /// Bus fabric performance counter 3
    pub perfctr3: Perfctr3,
    /// 0x0028
    /// Bus fabric performance event select for PERFCTR3
    pub perfsel3: Perfsel3,
}

