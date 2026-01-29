//! I2C0 - DW_apb_i2c address block List of configuration constants for the Synopsys I...
//!
//! DW_apb_i2c address block List of configuration constants for the Synopsys
//! I2C hardware (you may see references to these in I2C register header; these
//! are *fixed* values, set at hardware design time): IC_ULTRA_FAST_MODE
//! ................ 0x0 IC_UFM_TBUF_CNT_DEFAULT ........... 0x8
//! IC_UFM_SCL_LOW_COUNT .............. 0x0008 IC_UFM_SCL_HIGH_COUNT
//! ............. 0x0006 IC_TX_TL .......................... 0x0 IC_TX_CMD_BLOCK
//! ................... 0x1 IC_HAS_DMA ........................ 0x1
//! IC_HAS_ASYNC_FIFO ................. 0x0 IC_SMBUS_ARP ......................
//! 0x0 IC_FIRST_DATA_BYTE_STATUS ......... 0x1 IC_INTR_IO
//! ........................ 0x1 IC_MASTER_MODE .................... 0x1
//! IC_DEFAULT_ACK_GENERAL_CALL ....... 0x1 IC_INTR_POL .......................
//! 0x1 IC_OPTIONAL_SAR ................... 0x0 IC_DEFAULT_TAR_SLAVE_ADDR
//! ......... 0x055 IC_DEFAULT_SLAVE_ADDR ............. 0x055
//! IC_DEFAULT_HS_SPKLEN .............. 0x1 IC_FS_SCL_HIGH_COUNT ..............
//! 0x0006 IC_HS_SCL_LOW_COUNT ............... 0x0008 IC_DEVICE_ID_VALUE
//! ................ 0x0 IC_10BITADDR_MASTER ............... 0x0
//! IC_CLK_FREQ_OPTIMIZATION .......... 0x0 IC_DEFAULT_FS_SPKLEN ..............
//! 0x7 IC_ADD_ENCODED_PARAMS ............. 0x0 IC_DEFAULT_SDA_HOLD
//! ............... 0x000001 IC_DEFAULT_SDA_SETUP .............. 0x64
//! IC_AVOID_RX_FIFO_FLUSH_ON_TX_ABRT . 0x0 IC_CLOCK_PERIOD ...................
//! 100 IC_EMPTYFIFO_HOLD_MASTER_EN ....... 1 IC_RESTART_EN
//! ..................... 0x1 IC_TX_CMD_BLOCK_DEFAULT ........... 0x0
//! IC_BUS_CLEAR_FEATURE .............. 0x0 IC_CAP_LOADING ....................
//! 100 IC_FS_SCL_LOW_COUNT ............... 0x000d APB_DATA_WIDTH
//! .................... 32 IC_SDA_STUCK_TIMEOUT_DEFAULT ...... 0xffffffff
//! IC_SLV_DATA_NACK_ONLY ............. 0x1 IC_10BITADDR_SLAVE ................
//! 0x0 IC_CLK_TYPE ....................... 0x0 IC_SMBUS_UDID_MSB
//! ................. 0x0 IC_SMBUS_SUSPEND_ALERT ............ 0x0
//! IC_HS_SCL_HIGH_COUNT .............. 0x0006 IC_SLV_RESTART_DET_EN
//! ............. 0x1 IC_SMBUS .......................... 0x0
//! IC_OPTIONAL_SAR_DEFAULT ........... 0x0 IC_PERSISTANT_SLV_ADDR_DEFAULT ....
//! 0x0 IC_USE_COUNTS ..................... 0x0 IC_RX_BUFFER_DEPTH
//! ................ 16 IC_SCL_STUCK_TIMEOUT_DEFAULT ...... 0xffffffff
//! IC_RX_FULL_HLD_BUS_EN ............. 0x1 IC_SLAVE_DISABLE ..................
//! 0x1 IC_RX_TL .......................... 0x0 IC_DEVICE_ID
//! ...................... 0x0 IC_HC_COUNT_VALUES ................ 0x0
//! I2C_DYNAMIC_TAR_UPDATE ............ 0 IC_SMBUS_CLK_LOW_MEXT_DEFAULT .....
//! 0xffffffff IC_SMBUS_CLK_LOW_SEXT_DEFAULT ..... 0xffffffff IC_HS_MASTER_CODE
//! ................. 0x1 IC_SMBUS_RST_IDLE_CNT_DEFAULT ..... 0xffff
//! IC_SMBUS_UDID_LSB_DEFAULT ......... 0xffffffff IC_SS_SCL_HIGH_COUNT
//! .............. 0x0028 IC_SS_SCL_LOW_COUNT ............... 0x002f
//! IC_MAX_SPEED_MODE ................. 0x2 IC_STAT_FOR_CLK_STRETCH ...........
//! 0x0 IC_STOP_DET_IF_MASTER_ACTIVE ...... 0x0 IC_DEFAULT_UFM_SPKLEN
//! ............. 0x1 IC_TX_BUFFER_DEPTH ................ 16

#![allow(dead_code)]

pub const BASE: u32 = 0x40090000;

/// IC_CON Register
///
/// I2C Control Register. This register can be written only when the DW_apb_i2c
/// is disabled, which corresponds to the IC_ENABLE[0] register being set to 0.
/// Writes at other times have no effect. Read/Write Access: - bit 10 is read
/// only. - bit 11 is read only - bit 16 is read only - bit 17 is read only -
/// bits 18 and 19 are read only.
///
/// Address offset: 0x0000
/// Reset value: 0x00000065
///
/// ## Fields
///
/// - **STOP_DET_IF_MASTER_ACTIVE** [10] (RO): Master issues the STOP_DET interrupt irrespective of whether master is acti...
/// - **RX_FIFO_FULL_HLD_CTRL** [9] (RW): This bit controls whether DW_apb_i2c should hold the bus when the Rx FIFO i...
/// - **TX_EMPTY_CTRL** [8] (RW): This bit controls the generation of the TX_EMPTY interrupt, as described in...
/// - **STOP_DET_IFADDRESSED** [7] (RW): In slave mode: - 1'b1: issues the STOP_DET interrupt only when it is addres...
/// - **IC_SLAVE_DISABLE** [6] (RW): This bit controls whether I2C has its slave disabled, which means once the ...
/// - **IC_RESTART_EN** [5] (RW): Determines whether RESTART conditions may be sent when acting as a master. ...
/// - **IC_10BITADDR_MASTER** [4] (RW): Controls whether the DW_apb_i2c starts its transfers in 7- or 10-bit addres...
/// - **IC_10BITADDR_SLAVE** [3] (RW): When acting as a slave, this bit controls whether the DW_apb_i2c responds t...
/// - **SPEED** [2:1] (RW): These bits control at which speed the DW_apb_i2c operates; its setting is r...
/// - **MASTER_MODE** [0] (RW): This bit controls whether the DW_apb_i2c master is enabled. NOTE: Software ...
#[repr(transparent)]
pub struct IcCon {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for IcCon {}
unsafe impl Sync for IcCon {}

impl IcCon {
    // ======== Field Constants ========

    /// STOP_DET_IF_MASTER_ACTIVE - Master issues the STOP_DET interrupt irrespective of whether master is acti...
    /// Bits [10:10], mask 0x00000400
    pub const STOP_DET_IF_MASTER_ACTIVE_OFFSET: u32 = 10;
    pub const STOP_DET_IF_MASTER_ACTIVE_MASK: u32 = 0x00000400;
    pub const STOP_DET_IF_MASTER_ACTIVE: u32 = 1 << 10;

    /// RX_FIFO_FULL_HLD_CTRL - This bit controls whether DW_apb_i2c should hold the bus when the Rx FIFO i...
    /// Bits [9:9], mask 0x00000200
    pub const RX_FIFO_FULL_HLD_CTRL_OFFSET: u32 = 9;
    pub const RX_FIFO_FULL_HLD_CTRL_MASK: u32 = 0x00000200;
    pub const RX_FIFO_FULL_HLD_CTRL: u32 = 1 << 9;
    /// DISABLED = 0: Overflow when RX_FIFO is full
    pub const RX_FIFO_FULL_HLD_CTRL_DISABLED: u32 = 0 << 9;
    /// ENABLED = 1: Hold bus when RX_FIFO is full
    pub const RX_FIFO_FULL_HLD_CTRL_ENABLED: u32 = 1 << 9;

    /// TX_EMPTY_CTRL - This bit controls the generation of the TX_EMPTY interrupt, as described in...
    /// Bits [8:8], mask 0x00000100
    pub const TX_EMPTY_CTRL_OFFSET: u32 = 8;
    pub const TX_EMPTY_CTRL_MASK: u32 = 0x00000100;
    pub const TX_EMPTY_CTRL: u32 = 1 << 8;
    /// DISABLED = 0: Default behaviour of TX_EMPTY interrupt
    pub const TX_EMPTY_CTRL_DISABLED: u32 = 0 << 8;
    /// ENABLED = 1: Controlled generation of TX_EMPTY interrupt
    pub const TX_EMPTY_CTRL_ENABLED: u32 = 1 << 8;

    /// STOP_DET_IFADDRESSED - In slave mode: - 1'b1: issues the STOP_DET interrupt only when it is addres...
    /// Bits [7:7], mask 0x00000080
    pub const STOP_DET_IFADDRESSED_OFFSET: u32 = 7;
    pub const STOP_DET_IFADDRESSED_MASK: u32 = 0x00000080;
    pub const STOP_DET_IFADDRESSED: u32 = 1 << 7;
    /// DISABLED = 0: slave issues STOP_DET intr always
    pub const STOP_DET_IFADDRESSED_DISABLED: u32 = 0 << 7;
    /// ENABLED = 1: slave issues STOP_DET intr only if addressed
    pub const STOP_DET_IFADDRESSED_ENABLED: u32 = 1 << 7;

    /// IC_SLAVE_DISABLE - This bit controls whether I2C has its slave disabled, which means once the ...
    /// Bits [6:6], mask 0x00000040
    pub const IC_SLAVE_DISABLE_OFFSET: u32 = 6;
    pub const IC_SLAVE_DISABLE_MASK: u32 = 0x00000040;
    pub const IC_SLAVE_DISABLE: u32 = 1 << 6;
    /// SLAVE_ENABLED = 0: Slave mode is enabled
    pub const IC_SLAVE_DISABLE_SLAVE_ENABLED: u32 = 0 << 6;
    /// SLAVE_DISABLED = 1: Slave mode is disabled
    pub const IC_SLAVE_DISABLE_SLAVE_DISABLED: u32 = 1 << 6;

    /// IC_RESTART_EN - Determines whether RESTART conditions may be sent when acting as a master. ...
    /// Bits [5:5], mask 0x00000020
    pub const IC_RESTART_EN_OFFSET: u32 = 5;
    pub const IC_RESTART_EN_MASK: u32 = 0x00000020;
    pub const IC_RESTART_EN: u32 = 1 << 5;
    /// DISABLED = 0: Master restart disabled
    pub const IC_RESTART_EN_DISABLED: u32 = 0 << 5;
    /// ENABLED = 1: Master restart enabled
    pub const IC_RESTART_EN_ENABLED: u32 = 1 << 5;

    /// IC_10BITADDR_MASTER - Controls whether the DW_apb_i2c starts its transfers in 7- or 10-bit addres...
    /// Bits [4:4], mask 0x00000010
    pub const IC_10BITADDR_MASTER_OFFSET: u32 = 4;
    pub const IC_10BITADDR_MASTER_MASK: u32 = 0x00000010;
    pub const IC_10BITADDR_MASTER: u32 = 1 << 4;
    /// ADDR_7BITS = 0: Master 7Bit addressing mode
    pub const IC_10BITADDR_MASTER_ADDR_7BITS: u32 = 0 << 4;
    /// ADDR_10BITS = 1: Master 10Bit addressing mode
    pub const IC_10BITADDR_MASTER_ADDR_10BITS: u32 = 1 << 4;

    /// IC_10BITADDR_SLAVE - When acting as a slave, this bit controls whether the DW_apb_i2c responds t...
    /// Bits [3:3], mask 0x00000008
    pub const IC_10BITADDR_SLAVE_OFFSET: u32 = 3;
    pub const IC_10BITADDR_SLAVE_MASK: u32 = 0x00000008;
    pub const IC_10BITADDR_SLAVE: u32 = 1 << 3;
    /// ADDR_7BITS = 0: Slave 7Bit addressing
    pub const IC_10BITADDR_SLAVE_ADDR_7BITS: u32 = 0 << 3;
    /// ADDR_10BITS = 1: Slave 10Bit addressing
    pub const IC_10BITADDR_SLAVE_ADDR_10BITS: u32 = 1 << 3;

    /// SPEED - These bits control at which speed the DW_apb_i2c operates; its setting is r...
    /// Bits [2:1], mask 0x00000006
    pub const SPEED_OFFSET: u32 = 1;
    pub const SPEED_MASK: u32 = 0x00000006;
    /// STANDARD = 1: Standard Speed mode of operation
    pub const SPEED_STANDARD: u32 = 1 << 1;
    /// FAST = 2: Fast or Fast Plus mode of operation
    pub const SPEED_FAST: u32 = 2 << 1;
    /// HIGH = 3: High Speed mode of operation
    pub const SPEED_HIGH: u32 = 3 << 1;

    /// MASTER_MODE - This bit controls whether the DW_apb_i2c master is enabled. NOTE: Software ...
    /// Bits [0:0], mask 0x00000001
    pub const MASTER_MODE_OFFSET: u32 = 0;
    pub const MASTER_MODE_MASK: u32 = 0x00000001;
    pub const MASTER_MODE: u32 = 1 << 0;
    /// DISABLED = 0: Master mode is disabled
    pub const MASTER_MODE_DISABLED: u32 = 0 << 0;
    /// ENABLED = 1: Master mode is enabled
    pub const MASTER_MODE_ENABLED: u32 = 1 << 0;

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

    /// **STOP_DET_IF_MASTER_ACTIVE** [10:10]: Master issues the STOP_DET interrupt irrespective of whether master is acti...
    #[inline(always)]
    pub fn stop_det_if_master_active(&self) -> bool {
        (self.read_bits() & Self::STOP_DET_IF_MASTER_ACTIVE_MASK) != 0
    }

    /// **RX_FIFO_FULL_HLD_CTRL** [9:9]: This bit controls whether DW_apb_i2c should hold the bus when the Rx FIFO i...
    #[inline(always)]
    pub fn rx_fifo_full_hld_ctrl(&self) -> bool {
        (self.read_bits() & Self::RX_FIFO_FULL_HLD_CTRL_MASK) != 0
    }

    /// Set RX_FIFO_FULL_HLD_CTRL bit
    #[inline(always)]
    pub fn set_rx_fifo_full_hld_ctrl(&self) {
        self.set_bits(Self::RX_FIFO_FULL_HLD_CTRL_MASK);
    }

    /// Clear RX_FIFO_FULL_HLD_CTRL bit
    #[inline(always)]
    pub fn clr_rx_fifo_full_hld_ctrl(&self) {
        self.clr_bits(Self::RX_FIFO_FULL_HLD_CTRL_MASK);
    }

    /// **TX_EMPTY_CTRL** [8:8]: This bit controls the generation of the TX_EMPTY interrupt, as described in...
    #[inline(always)]
    pub fn tx_empty_ctrl(&self) -> bool {
        (self.read_bits() & Self::TX_EMPTY_CTRL_MASK) != 0
    }

    /// Set TX_EMPTY_CTRL bit
    #[inline(always)]
    pub fn set_tx_empty_ctrl(&self) {
        self.set_bits(Self::TX_EMPTY_CTRL_MASK);
    }

    /// Clear TX_EMPTY_CTRL bit
    #[inline(always)]
    pub fn clr_tx_empty_ctrl(&self) {
        self.clr_bits(Self::TX_EMPTY_CTRL_MASK);
    }

    /// **STOP_DET_IFADDRESSED** [7:7]: In slave mode: - 1'b1: issues the STOP_DET interrupt only when it is addres...
    #[inline(always)]
    pub fn stop_det_ifaddressed(&self) -> bool {
        (self.read_bits() & Self::STOP_DET_IFADDRESSED_MASK) != 0
    }

    /// Set STOP_DET_IFADDRESSED bit
    #[inline(always)]
    pub fn set_stop_det_ifaddressed(&self) {
        self.set_bits(Self::STOP_DET_IFADDRESSED_MASK);
    }

    /// Clear STOP_DET_IFADDRESSED bit
    #[inline(always)]
    pub fn clr_stop_det_ifaddressed(&self) {
        self.clr_bits(Self::STOP_DET_IFADDRESSED_MASK);
    }

    /// **IC_SLAVE_DISABLE** [6:6]: This bit controls whether I2C has its slave disabled, which means once the ...
    #[inline(always)]
    pub fn ic_slave_disable(&self) -> bool {
        (self.read_bits() & Self::IC_SLAVE_DISABLE_MASK) != 0
    }

    /// Set IC_SLAVE_DISABLE bit
    #[inline(always)]
    pub fn set_ic_slave_disable(&self) {
        self.set_bits(Self::IC_SLAVE_DISABLE_MASK);
    }

    /// Clear IC_SLAVE_DISABLE bit
    #[inline(always)]
    pub fn clr_ic_slave_disable(&self) {
        self.clr_bits(Self::IC_SLAVE_DISABLE_MASK);
    }

    /// **IC_RESTART_EN** [5:5]: Determines whether RESTART conditions may be sent when acting as a master. ...
    #[inline(always)]
    pub fn ic_restart_en(&self) -> bool {
        (self.read_bits() & Self::IC_RESTART_EN_MASK) != 0
    }

    /// Set IC_RESTART_EN bit
    #[inline(always)]
    pub fn set_ic_restart_en(&self) {
        self.set_bits(Self::IC_RESTART_EN_MASK);
    }

    /// Clear IC_RESTART_EN bit
    #[inline(always)]
    pub fn clr_ic_restart_en(&self) {
        self.clr_bits(Self::IC_RESTART_EN_MASK);
    }

    /// **IC_10BITADDR_MASTER** [4:4]: Controls whether the DW_apb_i2c starts its transfers in 7- or 10-bit addres...
    #[inline(always)]
    pub fn ic_10bitaddr_master(&self) -> bool {
        (self.read_bits() & Self::IC_10BITADDR_MASTER_MASK) != 0
    }

    /// Set IC_10BITADDR_MASTER bit
    #[inline(always)]
    pub fn set_ic_10bitaddr_master(&self) {
        self.set_bits(Self::IC_10BITADDR_MASTER_MASK);
    }

    /// Clear IC_10BITADDR_MASTER bit
    #[inline(always)]
    pub fn clr_ic_10bitaddr_master(&self) {
        self.clr_bits(Self::IC_10BITADDR_MASTER_MASK);
    }

    /// **IC_10BITADDR_SLAVE** [3:3]: When acting as a slave, this bit controls whether the DW_apb_i2c responds t...
    #[inline(always)]
    pub fn ic_10bitaddr_slave(&self) -> bool {
        (self.read_bits() & Self::IC_10BITADDR_SLAVE_MASK) != 0
    }

    /// Set IC_10BITADDR_SLAVE bit
    #[inline(always)]
    pub fn set_ic_10bitaddr_slave(&self) {
        self.set_bits(Self::IC_10BITADDR_SLAVE_MASK);
    }

    /// Clear IC_10BITADDR_SLAVE bit
    #[inline(always)]
    pub fn clr_ic_10bitaddr_slave(&self) {
        self.clr_bits(Self::IC_10BITADDR_SLAVE_MASK);
    }

    /// **SPEED** [2:1]: These bits control at which speed the DW_apb_i2c operates; its setting is r...
    #[inline(always)]
    pub fn speed(&self) -> u32 {
        (self.read_bits() & Self::SPEED_MASK) >> Self::SPEED_OFFSET
    }

    /// Write SPEED field
    #[inline(always)]
    pub fn write_speed(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::SPEED_MASK) | ((value << Self::SPEED_OFFSET) & Self::SPEED_MASK));
    }

    /// **MASTER_MODE** [0:0]: This bit controls whether the DW_apb_i2c master is enabled. NOTE: Software ...
    #[inline(always)]
    pub fn master_mode(&self) -> bool {
        (self.read_bits() & Self::MASTER_MODE_MASK) != 0
    }

    /// Set MASTER_MODE bit
    #[inline(always)]
    pub fn set_master_mode(&self) {
        self.set_bits(Self::MASTER_MODE_MASK);
    }

    /// Clear MASTER_MODE bit
    #[inline(always)]
    pub fn clr_master_mode(&self) {
        self.clr_bits(Self::MASTER_MODE_MASK);
    }

}

/// IC_CON value type for batch read/modify/write operations
///
/// Use `iccon.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `iccon.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IcConVal {
    bits: u32,
}

impl IcConVal {
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
        Self { bits: 0x00000065 }
    }

    /// Read STOP_DET_IF_MASTER_ACTIVE [10:10]: Master issues the STOP_DET interrupt irrespective of whether master is acti...
    #[inline(always)]
    pub const fn stop_det_if_master_active(&self) -> bool {
        (self.bits & IcCon::STOP_DET_IF_MASTER_ACTIVE_MASK) != 0
    }

    /// Read RX_FIFO_FULL_HLD_CTRL [9:9]: This bit controls whether DW_apb_i2c should hold the bus when the Rx FIFO i...
    #[inline(always)]
    pub const fn rx_fifo_full_hld_ctrl(&self) -> bool {
        (self.bits & IcCon::RX_FIFO_FULL_HLD_CTRL_MASK) != 0
    }

    /// Read TX_EMPTY_CTRL [8:8]: This bit controls the generation of the TX_EMPTY interrupt, as described in...
    #[inline(always)]
    pub const fn tx_empty_ctrl(&self) -> bool {
        (self.bits & IcCon::TX_EMPTY_CTRL_MASK) != 0
    }

    /// Read STOP_DET_IFADDRESSED [7:7]: In slave mode: - 1'b1: issues the STOP_DET interrupt only when it is addres...
    #[inline(always)]
    pub const fn stop_det_ifaddressed(&self) -> bool {
        (self.bits & IcCon::STOP_DET_IFADDRESSED_MASK) != 0
    }

    /// Read IC_SLAVE_DISABLE [6:6]: This bit controls whether I2C has its slave disabled, which means once the ...
    #[inline(always)]
    pub const fn ic_slave_disable(&self) -> bool {
        (self.bits & IcCon::IC_SLAVE_DISABLE_MASK) != 0
    }

    /// Read IC_RESTART_EN [5:5]: Determines whether RESTART conditions may be sent when acting as a master. ...
    #[inline(always)]
    pub const fn ic_restart_en(&self) -> bool {
        (self.bits & IcCon::IC_RESTART_EN_MASK) != 0
    }

    /// Read IC_10BITADDR_MASTER [4:4]: Controls whether the DW_apb_i2c starts its transfers in 7- or 10-bit addres...
    #[inline(always)]
    pub const fn ic_10bitaddr_master(&self) -> bool {
        (self.bits & IcCon::IC_10BITADDR_MASTER_MASK) != 0
    }

    /// Read IC_10BITADDR_SLAVE [3:3]: When acting as a slave, this bit controls whether the DW_apb_i2c responds t...
    #[inline(always)]
    pub const fn ic_10bitaddr_slave(&self) -> bool {
        (self.bits & IcCon::IC_10BITADDR_SLAVE_MASK) != 0
    }

    /// Read SPEED [2:1]: These bits control at which speed the DW_apb_i2c operates; its setting is r...
    #[inline(always)]
    pub const fn speed(&self) -> u32 {
        (self.bits & IcCon::SPEED_MASK) >> IcCon::SPEED_OFFSET
    }

    /// Read MASTER_MODE [0:0]: This bit controls whether the DW_apb_i2c master is enabled. NOTE: Software ...
    #[inline(always)]
    pub const fn master_mode(&self) -> bool {
        (self.bits & IcCon::MASTER_MODE_MASK) != 0
    }

    /// Set RX_FIFO_FULL_HLD_CTRL [9:9]: This bit controls whether DW_apb_i2c should hold the bus when the Rx FIFO i...
    #[inline(always)]
    pub const fn with_rx_fifo_full_hld_ctrl(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IcCon::RX_FIFO_FULL_HLD_CTRL_MASK
            } else {
                self.bits & !IcCon::RX_FIFO_FULL_HLD_CTRL_MASK
            }
        }
    }

    /// Set TX_EMPTY_CTRL [8:8]: This bit controls the generation of the TX_EMPTY interrupt, as described in...
    #[inline(always)]
    pub const fn with_tx_empty_ctrl(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IcCon::TX_EMPTY_CTRL_MASK
            } else {
                self.bits & !IcCon::TX_EMPTY_CTRL_MASK
            }
        }
    }

    /// Set STOP_DET_IFADDRESSED [7:7]: In slave mode: - 1'b1: issues the STOP_DET interrupt only when it is addres...
    #[inline(always)]
    pub const fn with_stop_det_ifaddressed(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IcCon::STOP_DET_IFADDRESSED_MASK
            } else {
                self.bits & !IcCon::STOP_DET_IFADDRESSED_MASK
            }
        }
    }

    /// Set IC_SLAVE_DISABLE [6:6]: This bit controls whether I2C has its slave disabled, which means once the ...
    #[inline(always)]
    pub const fn with_ic_slave_disable(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IcCon::IC_SLAVE_DISABLE_MASK
            } else {
                self.bits & !IcCon::IC_SLAVE_DISABLE_MASK
            }
        }
    }

    /// Set IC_RESTART_EN [5:5]: Determines whether RESTART conditions may be sent when acting as a master. ...
    #[inline(always)]
    pub const fn with_ic_restart_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IcCon::IC_RESTART_EN_MASK
            } else {
                self.bits & !IcCon::IC_RESTART_EN_MASK
            }
        }
    }

    /// Set IC_10BITADDR_MASTER [4:4]: Controls whether the DW_apb_i2c starts its transfers in 7- or 10-bit addres...
    #[inline(always)]
    pub const fn with_ic_10bitaddr_master(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IcCon::IC_10BITADDR_MASTER_MASK
            } else {
                self.bits & !IcCon::IC_10BITADDR_MASTER_MASK
            }
        }
    }

    /// Set IC_10BITADDR_SLAVE [3:3]: When acting as a slave, this bit controls whether the DW_apb_i2c responds t...
    #[inline(always)]
    pub const fn with_ic_10bitaddr_slave(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IcCon::IC_10BITADDR_SLAVE_MASK
            } else {
                self.bits & !IcCon::IC_10BITADDR_SLAVE_MASK
            }
        }
    }

    /// Set SPEED [2:1]: These bits control at which speed the DW_apb_i2c operates; its setting is r...
    #[inline(always)]
    pub const fn with_speed(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !IcCon::SPEED_MASK) | ((value << IcCon::SPEED_OFFSET) & IcCon::SPEED_MASK)
        }
    }

    /// Set MASTER_MODE [0:0]: This bit controls whether the DW_apb_i2c master is enabled. NOTE: Software ...
    #[inline(always)]
    pub const fn with_master_mode(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IcCon::MASTER_MODE_MASK
            } else {
                self.bits & !IcCon::MASTER_MODE_MASK
            }
        }
    }

}

impl Default for IcConVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl IcCon {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> IcConVal {
        IcConVal::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: IcConVal) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(IcConVal) -> IcConVal>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// IC_TAR Register
///
/// I2C Target Address Register This register is 12 bits wide, and bits 31:12
/// are reserved. This register can be written to only when IC_ENABLE[0] is set
/// to 0. Note: If the software or application is aware that the DW_apb_i2c is
/// not using the TAR address for the pending commands in the Tx FIFO, then it
/// is possible to update the TAR address even while the Tx FIFO has entries
/// (IC_STATUS[2]= 0). - It is not necessary to perform any write to this
/// register if DW_apb_i2c is enabled as an I2C slave only.
///
/// Address offset: 0x0004
/// Reset value: 0x00000055
///
/// ## Fields
///
/// - **SPECIAL** [11] (RW): This bit indicates whether software performs a Device-ID or General Call or...
/// - **GC_OR_START** [10] (RW): If bit 11 (SPECIAL) is set to 1 and bit 13(Device-ID) is set to 0, then thi...
/// - **IC_TAR** [9:0] (RW): This is the target address for any master transaction. When transmitting a ...
#[repr(transparent)]
pub struct IcTar {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for IcTar {}
unsafe impl Sync for IcTar {}

impl IcTar {
    // ======== Field Constants ========

    /// SPECIAL - This bit indicates whether software performs a Device-ID or General Call or...
    /// Bits [11:11], mask 0x00000800
    pub const SPECIAL_OFFSET: u32 = 11;
    pub const SPECIAL_MASK: u32 = 0x00000800;
    pub const SPECIAL: u32 = 1 << 11;
    /// DISABLED = 0: Disables programming of GENERAL_CALL or START_BYTE transmission
    pub const SPECIAL_DISABLED: u32 = 0 << 11;
    /// ENABLED = 1: Enables programming of GENERAL_CALL or START_BYTE transmission
    pub const SPECIAL_ENABLED: u32 = 1 << 11;

    /// GC_OR_START - If bit 11 (SPECIAL) is set to 1 and bit 13(Device-ID) is set to 0, then thi...
    /// Bits [10:10], mask 0x00000400
    pub const GC_OR_START_OFFSET: u32 = 10;
    pub const GC_OR_START_MASK: u32 = 0x00000400;
    pub const GC_OR_START: u32 = 1 << 10;
    /// GENERAL_CALL = 0: GENERAL_CALL byte transmission
    pub const GC_OR_START_GENERAL_CALL: u32 = 0 << 10;
    /// START_BYTE = 1: START byte transmission
    pub const GC_OR_START_START_BYTE: u32 = 1 << 10;

    /// IC_TAR - This is the target address for any master transaction. When transmitting a ...
    /// Bits [9:0], mask 0x000003ff
    pub const IC_TAR_OFFSET: u32 = 0;
    pub const IC_TAR_MASK: u32 = 0x000003ff;

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

    /// **SPECIAL** [11:11]: This bit indicates whether software performs a Device-ID or General Call or...
    #[inline(always)]
    pub fn special(&self) -> bool {
        (self.read_bits() & Self::SPECIAL_MASK) != 0
    }

    /// Set SPECIAL bit
    #[inline(always)]
    pub fn set_special(&self) {
        self.set_bits(Self::SPECIAL_MASK);
    }

    /// Clear SPECIAL bit
    #[inline(always)]
    pub fn clr_special(&self) {
        self.clr_bits(Self::SPECIAL_MASK);
    }

    /// **GC_OR_START** [10:10]: If bit 11 (SPECIAL) is set to 1 and bit 13(Device-ID) is set to 0, then thi...
    #[inline(always)]
    pub fn gc_or_start(&self) -> bool {
        (self.read_bits() & Self::GC_OR_START_MASK) != 0
    }

    /// Set GC_OR_START bit
    #[inline(always)]
    pub fn set_gc_or_start(&self) {
        self.set_bits(Self::GC_OR_START_MASK);
    }

    /// Clear GC_OR_START bit
    #[inline(always)]
    pub fn clr_gc_or_start(&self) {
        self.clr_bits(Self::GC_OR_START_MASK);
    }

    /// **IC_TAR** [9:0]: This is the target address for any master transaction. When transmitting a ...
    #[inline(always)]
    pub fn ic_tar(&self) -> u32 {
        (self.read_bits() & Self::IC_TAR_MASK) >> Self::IC_TAR_OFFSET
    }

    /// Write IC_TAR field
    #[inline(always)]
    pub fn write_ic_tar(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::IC_TAR_MASK) | ((value << Self::IC_TAR_OFFSET) & Self::IC_TAR_MASK));
    }

}

/// IC_TAR value type for batch read/modify/write operations
///
/// Use `ictar.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ictar.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IcTarVal {
    bits: u32,
}

impl IcTarVal {
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
        Self { bits: 0x00000055 }
    }

    /// Read SPECIAL [11:11]: This bit indicates whether software performs a Device-ID or General Call or...
    #[inline(always)]
    pub const fn special(&self) -> bool {
        (self.bits & IcTar::SPECIAL_MASK) != 0
    }

    /// Read GC_OR_START [10:10]: If bit 11 (SPECIAL) is set to 1 and bit 13(Device-ID) is set to 0, then thi...
    #[inline(always)]
    pub const fn gc_or_start(&self) -> bool {
        (self.bits & IcTar::GC_OR_START_MASK) != 0
    }

    /// Read IC_TAR [9:0]: This is the target address for any master transaction. When transmitting a ...
    #[inline(always)]
    pub const fn ic_tar(&self) -> u32 {
        (self.bits & IcTar::IC_TAR_MASK) >> IcTar::IC_TAR_OFFSET
    }

    /// Set SPECIAL [11:11]: This bit indicates whether software performs a Device-ID or General Call or...
    #[inline(always)]
    pub const fn with_special(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IcTar::SPECIAL_MASK
            } else {
                self.bits & !IcTar::SPECIAL_MASK
            }
        }
    }

    /// Set GC_OR_START [10:10]: If bit 11 (SPECIAL) is set to 1 and bit 13(Device-ID) is set to 0, then thi...
    #[inline(always)]
    pub const fn with_gc_or_start(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IcTar::GC_OR_START_MASK
            } else {
                self.bits & !IcTar::GC_OR_START_MASK
            }
        }
    }

    /// Set IC_TAR [9:0]: This is the target address for any master transaction. When transmitting a ...
    #[inline(always)]
    pub const fn with_ic_tar(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !IcTar::IC_TAR_MASK) | ((value << IcTar::IC_TAR_OFFSET) & IcTar::IC_TAR_MASK)
        }
    }

}

impl Default for IcTarVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl IcTar {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> IcTarVal {
        IcTarVal::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: IcTarVal) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(IcTarVal) -> IcTarVal>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// IC_SAR Register
///
/// I2C Slave Address Register
///
/// Address offset: 0x0008
/// Reset value: 0x00000055
///
/// ## Fields
///
/// - **IC_SAR** [9:0] (RW): The IC_SAR holds the slave address when the I2C is operating as a slave. Fo...
#[repr(transparent)]
pub struct IcSar {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for IcSar {}
unsafe impl Sync for IcSar {}

impl IcSar {
    // ======== Field Constants ========

    /// IC_SAR - The IC_SAR holds the slave address when the I2C is operating as a slave. Fo...
    /// Bits [9:0], mask 0x000003ff
    pub const IC_SAR_OFFSET: u32 = 0;
    pub const IC_SAR_MASK: u32 = 0x000003ff;

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

    /// **IC_SAR** [9:0]: The IC_SAR holds the slave address when the I2C is operating as a slave. Fo...
    #[inline(always)]
    pub fn ic_sar(&self) -> u32 {
        (self.read_bits() & Self::IC_SAR_MASK) >> Self::IC_SAR_OFFSET
    }

    /// Write IC_SAR field
    #[inline(always)]
    pub fn write_ic_sar(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::IC_SAR_MASK) | ((value << Self::IC_SAR_OFFSET) & Self::IC_SAR_MASK));
    }

}

/// IC_SAR value type for batch read/modify/write operations
///
/// Use `icsar.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `icsar.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IcSarVal {
    bits: u32,
}

impl IcSarVal {
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
        Self { bits: 0x00000055 }
    }

    /// Read IC_SAR [9:0]: The IC_SAR holds the slave address when the I2C is operating as a slave. Fo...
    #[inline(always)]
    pub const fn ic_sar(&self) -> u32 {
        (self.bits & IcSar::IC_SAR_MASK) >> IcSar::IC_SAR_OFFSET
    }

    /// Set IC_SAR [9:0]: The IC_SAR holds the slave address when the I2C is operating as a slave. Fo...
    #[inline(always)]
    pub const fn with_ic_sar(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !IcSar::IC_SAR_MASK) | ((value << IcSar::IC_SAR_OFFSET) & IcSar::IC_SAR_MASK)
        }
    }

}

impl Default for IcSarVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl IcSar {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> IcSarVal {
        IcSarVal::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: IcSarVal) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(IcSarVal) -> IcSarVal>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// IC_DATA_CMD Register
///
/// I2C Rx/Tx Data Buffer and Command Register; this is the register the CPU
/// writes to when filling the TX FIFO and the CPU reads from when retrieving
/// bytes from RX FIFO. The size of the register changes as follows: Write: - 11
/// bits when IC_EMPTYFIFO_HOLD_MASTER_EN=1 - 9 bits when
/// IC_EMPTYFIFO_HOLD_MASTER_EN=0 Read: - 12 bits when IC_FIRST_DATA_BYTE_STATUS
/// = 1 - 8 bits when IC_FIRST_DATA_BYTE_STATUS = 0 Note: In order for the
/// DW_apb_i2c to continue acknowledging reads, a read command should be written
/// for every byte that is to be received; otherwise the DW_apb_i2c will stop
/// acknowledging.
///
/// Address offset: 0x0010
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **FIRST_DATA_BYTE** [11] (RO): Indicates the first data byte received after the address phase for receive ...
/// - **RESTART** [10] (WO): This bit controls whether a RESTART is issued before the byte is sent or re...
/// - **STOP** [9] (WO): This bit controls whether a STOP is issued after the byte is sent or receiv...
/// - **CMD** [8] (WO): This bit controls whether a read or a write is performed. This bit does not...
/// - **DAT** [7:0] (RW): This register contains the data to be transmitted or received on the I2C bu...
#[repr(transparent)]
pub struct IcDataCmd {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for IcDataCmd {}
unsafe impl Sync for IcDataCmd {}

impl IcDataCmd {
    // ======== Field Constants ========

    /// FIRST_DATA_BYTE - Indicates the first data byte received after the address phase for receive ...
    /// Bits [11:11], mask 0x00000800
    pub const FIRST_DATA_BYTE_OFFSET: u32 = 11;
    pub const FIRST_DATA_BYTE_MASK: u32 = 0x00000800;
    pub const FIRST_DATA_BYTE: u32 = 1 << 11;
    /// INACTIVE = 0: Sequential data byte received
    pub const FIRST_DATA_BYTE_INACTIVE: u32 = 0 << 11;
    /// ACTIVE = 1: Non sequential data byte received
    pub const FIRST_DATA_BYTE_ACTIVE: u32 = 1 << 11;

    /// RESTART - This bit controls whether a RESTART is issued before the byte is sent or re...
    /// Bits [10:10], mask 0x00000400
    pub const RESTART_OFFSET: u32 = 10;
    pub const RESTART_MASK: u32 = 0x00000400;
    pub const RESTART: u32 = 1 << 10;
    /// DISABLE = 0: Don't Issue RESTART before this command
    pub const RESTART_DISABLE: u32 = 0 << 10;
    /// ENABLE = 1: Issue RESTART before this command
    pub const RESTART_ENABLE: u32 = 1 << 10;

    /// STOP - This bit controls whether a STOP is issued after the byte is sent or receiv...
    /// Bits [9:9], mask 0x00000200
    pub const STOP_OFFSET: u32 = 9;
    pub const STOP_MASK: u32 = 0x00000200;
    pub const STOP: u32 = 1 << 9;
    /// DISABLE = 0: Don't Issue STOP after this command
    pub const STOP_DISABLE: u32 = 0 << 9;
    /// ENABLE = 1: Issue STOP after this command
    pub const STOP_ENABLE: u32 = 1 << 9;

    /// CMD - This bit controls whether a read or a write is performed. This bit does not...
    /// Bits [8:8], mask 0x00000100
    pub const CMD_OFFSET: u32 = 8;
    pub const CMD_MASK: u32 = 0x00000100;
    pub const CMD: u32 = 1 << 8;
    /// WRITE = 0: Master Write Command
    pub const CMD_WRITE: u32 = 0 << 8;
    /// READ = 1: Master Read Command
    pub const CMD_READ: u32 = 1 << 8;

    /// DAT - This register contains the data to be transmitted or received on the I2C bu...
    /// Bits [7:0], mask 0x000000ff
    pub const DAT_OFFSET: u32 = 0;
    pub const DAT_MASK: u32 = 0x000000ff;

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

    /// **FIRST_DATA_BYTE** [11:11]: Indicates the first data byte received after the address phase for receive ...
    #[inline(always)]
    pub fn first_data_byte(&self) -> bool {
        (self.read_bits() & Self::FIRST_DATA_BYTE_MASK) != 0
    }

    /// **RESTART** [10:10]: This bit controls whether a RESTART is issued before the byte is sent or re...
    /// Trigger RESTART (write-only)
    #[inline(always)]
    pub fn trigger_restart(&self) {
        self.write_bits(Self::RESTART_MASK);
    }

    /// **STOP** [9:9]: This bit controls whether a STOP is issued after the byte is sent or receiv...
    /// Trigger STOP (write-only)
    #[inline(always)]
    pub fn trigger_stop(&self) {
        self.write_bits(Self::STOP_MASK);
    }

    /// **CMD** [8:8]: This bit controls whether a read or a write is performed. This bit does not...
    /// Trigger CMD (write-only)
    #[inline(always)]
    pub fn trigger_cmd(&self) {
        self.write_bits(Self::CMD_MASK);
    }

    /// **DAT** [7:0]: This register contains the data to be transmitted or received on the I2C bu...
    #[inline(always)]
    pub fn dat(&self) -> u32 {
        (self.read_bits() & Self::DAT_MASK) >> Self::DAT_OFFSET
    }

    /// Write DAT field
    #[inline(always)]
    pub fn write_dat(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::DAT_MASK) | ((value << Self::DAT_OFFSET) & Self::DAT_MASK));
    }

}

/// IC_DATA_CMD value type for batch read/modify/write operations
///
/// Use `icdatacmd.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `icdatacmd.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IcDataCmdVal {
    bits: u32,
}

impl IcDataCmdVal {
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

    /// Read FIRST_DATA_BYTE [11:11]: Indicates the first data byte received after the address phase for receive ...
    #[inline(always)]
    pub const fn first_data_byte(&self) -> bool {
        (self.bits & IcDataCmd::FIRST_DATA_BYTE_MASK) != 0
    }

    /// Read DAT [7:0]: This register contains the data to be transmitted or received on the I2C bu...
    #[inline(always)]
    pub const fn dat(&self) -> u32 {
        (self.bits & IcDataCmd::DAT_MASK) >> IcDataCmd::DAT_OFFSET
    }

    /// Set RESTART [10:10]: This bit controls whether a RESTART is issued before the byte is sent or re...
    #[inline(always)]
    pub const fn with_restart(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IcDataCmd::RESTART_MASK
            } else {
                self.bits & !IcDataCmd::RESTART_MASK
            }
        }
    }

    /// Set STOP [9:9]: This bit controls whether a STOP is issued after the byte is sent or receiv...
    #[inline(always)]
    pub const fn with_stop(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IcDataCmd::STOP_MASK
            } else {
                self.bits & !IcDataCmd::STOP_MASK
            }
        }
    }

    /// Set CMD [8:8]: This bit controls whether a read or a write is performed. This bit does not...
    #[inline(always)]
    pub const fn with_cmd(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IcDataCmd::CMD_MASK
            } else {
                self.bits & !IcDataCmd::CMD_MASK
            }
        }
    }

    /// Set DAT [7:0]: This register contains the data to be transmitted or received on the I2C bu...
    #[inline(always)]
    pub const fn with_dat(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !IcDataCmd::DAT_MASK) | ((value << IcDataCmd::DAT_OFFSET) & IcDataCmd::DAT_MASK)
        }
    }

}

impl Default for IcDataCmdVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl IcDataCmd {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> IcDataCmdVal {
        IcDataCmdVal::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: IcDataCmdVal) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(IcDataCmdVal) -> IcDataCmdVal>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// IC_SS_SCL_HCNT Register
///
/// Standard Speed I2C Clock SCL High Count Register
///
/// Address offset: 0x0014
/// Reset value: 0x00000028
///
/// ## Fields
///
/// - **IC_SS_SCL_HCNT** [15:0] (RW): This register must be set before any I2C bus transaction can take place to ...
#[repr(transparent)]
pub struct IcSsSclHcnt {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for IcSsSclHcnt {}
unsafe impl Sync for IcSsSclHcnt {}

impl IcSsSclHcnt {
    // ======== Field Constants ========

    /// IC_SS_SCL_HCNT - This register must be set before any I2C bus transaction can take place to ...
    /// Bits [15:0], mask 0x0000ffff
    pub const IC_SS_SCL_HCNT_OFFSET: u32 = 0;
    pub const IC_SS_SCL_HCNT_MASK: u32 = 0x0000ffff;

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

    /// **IC_SS_SCL_HCNT** [15:0]: This register must be set before any I2C bus transaction can take place to ...
    #[inline(always)]
    pub fn ic_ss_scl_hcnt(&self) -> u32 {
        (self.read_bits() & Self::IC_SS_SCL_HCNT_MASK) >> Self::IC_SS_SCL_HCNT_OFFSET
    }

    /// Write IC_SS_SCL_HCNT field
    #[inline(always)]
    pub fn write_ic_ss_scl_hcnt(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::IC_SS_SCL_HCNT_MASK) | ((value << Self::IC_SS_SCL_HCNT_OFFSET) & Self::IC_SS_SCL_HCNT_MASK));
    }

}

/// IC_SS_SCL_HCNT value type for batch read/modify/write operations
///
/// Use `icsssclhcnt.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `icsssclhcnt.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IcSsSclHcntVal {
    bits: u32,
}

impl IcSsSclHcntVal {
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
        Self { bits: 0x00000028 }
    }

    /// Read IC_SS_SCL_HCNT [15:0]: This register must be set before any I2C bus transaction can take place to ...
    #[inline(always)]
    pub const fn ic_ss_scl_hcnt(&self) -> u32 {
        (self.bits & IcSsSclHcnt::IC_SS_SCL_HCNT_MASK) >> IcSsSclHcnt::IC_SS_SCL_HCNT_OFFSET
    }

    /// Set IC_SS_SCL_HCNT [15:0]: This register must be set before any I2C bus transaction can take place to ...
    #[inline(always)]
    pub const fn with_ic_ss_scl_hcnt(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !IcSsSclHcnt::IC_SS_SCL_HCNT_MASK) | ((value << IcSsSclHcnt::IC_SS_SCL_HCNT_OFFSET) & IcSsSclHcnt::IC_SS_SCL_HCNT_MASK)
        }
    }

}

impl Default for IcSsSclHcntVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl IcSsSclHcnt {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> IcSsSclHcntVal {
        IcSsSclHcntVal::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: IcSsSclHcntVal) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(IcSsSclHcntVal) -> IcSsSclHcntVal>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// IC_SS_SCL_LCNT Register
///
/// Standard Speed I2C Clock SCL Low Count Register
///
/// Address offset: 0x0018
/// Reset value: 0x0000002f
///
/// ## Fields
///
/// - **IC_SS_SCL_LCNT** [15:0] (RW): This register must be set before any I2C bus transaction can take place to ...
#[repr(transparent)]
pub struct IcSsSclLcnt {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for IcSsSclLcnt {}
unsafe impl Sync for IcSsSclLcnt {}

impl IcSsSclLcnt {
    // ======== Field Constants ========

    /// IC_SS_SCL_LCNT - This register must be set before any I2C bus transaction can take place to ...
    /// Bits [15:0], mask 0x0000ffff
    pub const IC_SS_SCL_LCNT_OFFSET: u32 = 0;
    pub const IC_SS_SCL_LCNT_MASK: u32 = 0x0000ffff;

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

    /// **IC_SS_SCL_LCNT** [15:0]: This register must be set before any I2C bus transaction can take place to ...
    #[inline(always)]
    pub fn ic_ss_scl_lcnt(&self) -> u32 {
        (self.read_bits() & Self::IC_SS_SCL_LCNT_MASK) >> Self::IC_SS_SCL_LCNT_OFFSET
    }

    /// Write IC_SS_SCL_LCNT field
    #[inline(always)]
    pub fn write_ic_ss_scl_lcnt(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::IC_SS_SCL_LCNT_MASK) | ((value << Self::IC_SS_SCL_LCNT_OFFSET) & Self::IC_SS_SCL_LCNT_MASK));
    }

}

/// IC_SS_SCL_LCNT value type for batch read/modify/write operations
///
/// Use `icssscllcnt.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `icssscllcnt.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IcSsSclLcntVal {
    bits: u32,
}

impl IcSsSclLcntVal {
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
        Self { bits: 0x0000002f }
    }

    /// Read IC_SS_SCL_LCNT [15:0]: This register must be set before any I2C bus transaction can take place to ...
    #[inline(always)]
    pub const fn ic_ss_scl_lcnt(&self) -> u32 {
        (self.bits & IcSsSclLcnt::IC_SS_SCL_LCNT_MASK) >> IcSsSclLcnt::IC_SS_SCL_LCNT_OFFSET
    }

    /// Set IC_SS_SCL_LCNT [15:0]: This register must be set before any I2C bus transaction can take place to ...
    #[inline(always)]
    pub const fn with_ic_ss_scl_lcnt(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !IcSsSclLcnt::IC_SS_SCL_LCNT_MASK) | ((value << IcSsSclLcnt::IC_SS_SCL_LCNT_OFFSET) & IcSsSclLcnt::IC_SS_SCL_LCNT_MASK)
        }
    }

}

impl Default for IcSsSclLcntVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl IcSsSclLcnt {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> IcSsSclLcntVal {
        IcSsSclLcntVal::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: IcSsSclLcntVal) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(IcSsSclLcntVal) -> IcSsSclLcntVal>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// IC_FS_SCL_HCNT Register
///
/// Fast Mode or Fast Mode Plus I2C Clock SCL High Count Register
///
/// Address offset: 0x001c
/// Reset value: 0x00000006
///
/// ## Fields
///
/// - **IC_FS_SCL_HCNT** [15:0] (RW): This register must be set before any I2C bus transaction can take place to ...
#[repr(transparent)]
pub struct IcFsSclHcnt {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for IcFsSclHcnt {}
unsafe impl Sync for IcFsSclHcnt {}

impl IcFsSclHcnt {
    // ======== Field Constants ========

    /// IC_FS_SCL_HCNT - This register must be set before any I2C bus transaction can take place to ...
    /// Bits [15:0], mask 0x0000ffff
    pub const IC_FS_SCL_HCNT_OFFSET: u32 = 0;
    pub const IC_FS_SCL_HCNT_MASK: u32 = 0x0000ffff;

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

    /// **IC_FS_SCL_HCNT** [15:0]: This register must be set before any I2C bus transaction can take place to ...
    #[inline(always)]
    pub fn ic_fs_scl_hcnt(&self) -> u32 {
        (self.read_bits() & Self::IC_FS_SCL_HCNT_MASK) >> Self::IC_FS_SCL_HCNT_OFFSET
    }

    /// Write IC_FS_SCL_HCNT field
    #[inline(always)]
    pub fn write_ic_fs_scl_hcnt(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::IC_FS_SCL_HCNT_MASK) | ((value << Self::IC_FS_SCL_HCNT_OFFSET) & Self::IC_FS_SCL_HCNT_MASK));
    }

}

/// IC_FS_SCL_HCNT value type for batch read/modify/write operations
///
/// Use `icfssclhcnt.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `icfssclhcnt.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IcFsSclHcntVal {
    bits: u32,
}

impl IcFsSclHcntVal {
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
        Self { bits: 0x00000006 }
    }

    /// Read IC_FS_SCL_HCNT [15:0]: This register must be set before any I2C bus transaction can take place to ...
    #[inline(always)]
    pub const fn ic_fs_scl_hcnt(&self) -> u32 {
        (self.bits & IcFsSclHcnt::IC_FS_SCL_HCNT_MASK) >> IcFsSclHcnt::IC_FS_SCL_HCNT_OFFSET
    }

    /// Set IC_FS_SCL_HCNT [15:0]: This register must be set before any I2C bus transaction can take place to ...
    #[inline(always)]
    pub const fn with_ic_fs_scl_hcnt(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !IcFsSclHcnt::IC_FS_SCL_HCNT_MASK) | ((value << IcFsSclHcnt::IC_FS_SCL_HCNT_OFFSET) & IcFsSclHcnt::IC_FS_SCL_HCNT_MASK)
        }
    }

}

impl Default for IcFsSclHcntVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl IcFsSclHcnt {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> IcFsSclHcntVal {
        IcFsSclHcntVal::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: IcFsSclHcntVal) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(IcFsSclHcntVal) -> IcFsSclHcntVal>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// IC_FS_SCL_LCNT Register
///
/// Fast Mode or Fast Mode Plus I2C Clock SCL Low Count Register
///
/// Address offset: 0x0020
/// Reset value: 0x0000000d
///
/// ## Fields
///
/// - **IC_FS_SCL_LCNT** [15:0] (RW): This register must be set before any I2C bus transaction can take place to ...
#[repr(transparent)]
pub struct IcFsSclLcnt {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for IcFsSclLcnt {}
unsafe impl Sync for IcFsSclLcnt {}

impl IcFsSclLcnt {
    // ======== Field Constants ========

    /// IC_FS_SCL_LCNT - This register must be set before any I2C bus transaction can take place to ...
    /// Bits [15:0], mask 0x0000ffff
    pub const IC_FS_SCL_LCNT_OFFSET: u32 = 0;
    pub const IC_FS_SCL_LCNT_MASK: u32 = 0x0000ffff;

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

    /// **IC_FS_SCL_LCNT** [15:0]: This register must be set before any I2C bus transaction can take place to ...
    #[inline(always)]
    pub fn ic_fs_scl_lcnt(&self) -> u32 {
        (self.read_bits() & Self::IC_FS_SCL_LCNT_MASK) >> Self::IC_FS_SCL_LCNT_OFFSET
    }

    /// Write IC_FS_SCL_LCNT field
    #[inline(always)]
    pub fn write_ic_fs_scl_lcnt(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::IC_FS_SCL_LCNT_MASK) | ((value << Self::IC_FS_SCL_LCNT_OFFSET) & Self::IC_FS_SCL_LCNT_MASK));
    }

}

/// IC_FS_SCL_LCNT value type for batch read/modify/write operations
///
/// Use `icfsscllcnt.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `icfsscllcnt.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IcFsSclLcntVal {
    bits: u32,
}

impl IcFsSclLcntVal {
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

    /// Read IC_FS_SCL_LCNT [15:0]: This register must be set before any I2C bus transaction can take place to ...
    #[inline(always)]
    pub const fn ic_fs_scl_lcnt(&self) -> u32 {
        (self.bits & IcFsSclLcnt::IC_FS_SCL_LCNT_MASK) >> IcFsSclLcnt::IC_FS_SCL_LCNT_OFFSET
    }

    /// Set IC_FS_SCL_LCNT [15:0]: This register must be set before any I2C bus transaction can take place to ...
    #[inline(always)]
    pub const fn with_ic_fs_scl_lcnt(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !IcFsSclLcnt::IC_FS_SCL_LCNT_MASK) | ((value << IcFsSclLcnt::IC_FS_SCL_LCNT_OFFSET) & IcFsSclLcnt::IC_FS_SCL_LCNT_MASK)
        }
    }

}

impl Default for IcFsSclLcntVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl IcFsSclLcnt {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> IcFsSclLcntVal {
        IcFsSclLcntVal::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: IcFsSclLcntVal) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(IcFsSclLcntVal) -> IcFsSclLcntVal>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// IC_INTR_STAT Register
///
/// I2C Interrupt Status Register Each bit in this register has a corresponding
/// mask bit in the IC_INTR_MASK register. These bits are cleared by reading the
/// matching interrupt clear register. The unmasked raw versions of these bits
/// are available in the IC_RAW_INTR_STAT register.
///
/// Address offset: 0x002c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **R_RESTART_DET** [12] (RO): See IC_RAW_INTR_STAT for a detailed description of R_RESTART_DET bit. Reset...
/// - **R_GEN_CALL** [11] (RO): See IC_RAW_INTR_STAT for a detailed description of R_GEN_CALL bit. Reset va...
/// - **R_START_DET** [10] (RO): See IC_RAW_INTR_STAT for a detailed description of R_START_DET bit. Reset v...
/// - **R_STOP_DET** [9] (RO): See IC_RAW_INTR_STAT for a detailed description of R_STOP_DET bit. Reset va...
/// - **R_ACTIVITY** [8] (RO): See IC_RAW_INTR_STAT for a detailed description of R_ACTIVITY bit. Reset va...
/// - **R_RX_DONE** [7] (RO): See IC_RAW_INTR_STAT for a detailed description of R_RX_DONE bit. Reset val...
/// - **R_TX_ABRT** [6] (RO): See IC_RAW_INTR_STAT for a detailed description of R_TX_ABRT bit. Reset val...
/// - **R_RD_REQ** [5] (RO): See IC_RAW_INTR_STAT for a detailed description of R_RD_REQ bit. Reset valu...
/// - **R_TX_EMPTY** [4] (RO): See IC_RAW_INTR_STAT for a detailed description of R_TX_EMPTY bit. Reset va...
/// - **R_TX_OVER** [3] (RO): See IC_RAW_INTR_STAT for a detailed description of R_TX_OVER bit. Reset val...
/// - **R_RX_FULL** [2] (RO): See IC_RAW_INTR_STAT for a detailed description of R_RX_FULL bit. Reset val...
/// - **R_RX_OVER** [1] (RO): See IC_RAW_INTR_STAT for a detailed description of R_RX_OVER bit. Reset val...
/// - **R_RX_UNDER** [0] (RO): See IC_RAW_INTR_STAT for a detailed description of R_RX_UNDER bit. Reset va...
#[repr(transparent)]
pub struct IcIntrStat {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for IcIntrStat {}
unsafe impl Sync for IcIntrStat {}

impl IcIntrStat {
    // ======== Field Constants ========

    /// R_RESTART_DET - See IC_RAW_INTR_STAT for a detailed description of R_RESTART_DET bit. Reset...
    /// Bits [12:12], mask 0x00001000
    pub const R_RESTART_DET_OFFSET: u32 = 12;
    pub const R_RESTART_DET_MASK: u32 = 0x00001000;
    pub const R_RESTART_DET: u32 = 1 << 12;
    /// INACTIVE = 0: R_RESTART_DET interrupt is inactive
    pub const R_RESTART_DET_INACTIVE: u32 = 0 << 12;
    /// ACTIVE = 1: R_RESTART_DET interrupt is active
    pub const R_RESTART_DET_ACTIVE: u32 = 1 << 12;

    /// R_GEN_CALL - See IC_RAW_INTR_STAT for a detailed description of R_GEN_CALL bit. Reset va...
    /// Bits [11:11], mask 0x00000800
    pub const R_GEN_CALL_OFFSET: u32 = 11;
    pub const R_GEN_CALL_MASK: u32 = 0x00000800;
    pub const R_GEN_CALL: u32 = 1 << 11;
    /// INACTIVE = 0: R_GEN_CALL interrupt is inactive
    pub const R_GEN_CALL_INACTIVE: u32 = 0 << 11;
    /// ACTIVE = 1: R_GEN_CALL interrupt is active
    pub const R_GEN_CALL_ACTIVE: u32 = 1 << 11;

    /// R_START_DET - See IC_RAW_INTR_STAT for a detailed description of R_START_DET bit. Reset v...
    /// Bits [10:10], mask 0x00000400
    pub const R_START_DET_OFFSET: u32 = 10;
    pub const R_START_DET_MASK: u32 = 0x00000400;
    pub const R_START_DET: u32 = 1 << 10;
    /// INACTIVE = 0: R_START_DET interrupt is inactive
    pub const R_START_DET_INACTIVE: u32 = 0 << 10;
    /// ACTIVE = 1: R_START_DET interrupt is active
    pub const R_START_DET_ACTIVE: u32 = 1 << 10;

    /// R_STOP_DET - See IC_RAW_INTR_STAT for a detailed description of R_STOP_DET bit. Reset va...
    /// Bits [9:9], mask 0x00000200
    pub const R_STOP_DET_OFFSET: u32 = 9;
    pub const R_STOP_DET_MASK: u32 = 0x00000200;
    pub const R_STOP_DET: u32 = 1 << 9;
    /// INACTIVE = 0: R_STOP_DET interrupt is inactive
    pub const R_STOP_DET_INACTIVE: u32 = 0 << 9;
    /// ACTIVE = 1: R_STOP_DET interrupt is active
    pub const R_STOP_DET_ACTIVE: u32 = 1 << 9;

    /// R_ACTIVITY - See IC_RAW_INTR_STAT for a detailed description of R_ACTIVITY bit. Reset va...
    /// Bits [8:8], mask 0x00000100
    pub const R_ACTIVITY_OFFSET: u32 = 8;
    pub const R_ACTIVITY_MASK: u32 = 0x00000100;
    pub const R_ACTIVITY: u32 = 1 << 8;
    /// INACTIVE = 0: R_ACTIVITY interrupt is inactive
    pub const R_ACTIVITY_INACTIVE: u32 = 0 << 8;
    /// ACTIVE = 1: R_ACTIVITY interrupt is active
    pub const R_ACTIVITY_ACTIVE: u32 = 1 << 8;

    /// R_RX_DONE - See IC_RAW_INTR_STAT for a detailed description of R_RX_DONE bit. Reset val...
    /// Bits [7:7], mask 0x00000080
    pub const R_RX_DONE_OFFSET: u32 = 7;
    pub const R_RX_DONE_MASK: u32 = 0x00000080;
    pub const R_RX_DONE: u32 = 1 << 7;
    /// INACTIVE = 0: R_RX_DONE interrupt is inactive
    pub const R_RX_DONE_INACTIVE: u32 = 0 << 7;
    /// ACTIVE = 1: R_RX_DONE interrupt is active
    pub const R_RX_DONE_ACTIVE: u32 = 1 << 7;

    /// R_TX_ABRT - See IC_RAW_INTR_STAT for a detailed description of R_TX_ABRT bit. Reset val...
    /// Bits [6:6], mask 0x00000040
    pub const R_TX_ABRT_OFFSET: u32 = 6;
    pub const R_TX_ABRT_MASK: u32 = 0x00000040;
    pub const R_TX_ABRT: u32 = 1 << 6;
    /// INACTIVE = 0: R_TX_ABRT interrupt is inactive
    pub const R_TX_ABRT_INACTIVE: u32 = 0 << 6;
    /// ACTIVE = 1: R_TX_ABRT interrupt is active
    pub const R_TX_ABRT_ACTIVE: u32 = 1 << 6;

    /// R_RD_REQ - See IC_RAW_INTR_STAT for a detailed description of R_RD_REQ bit. Reset valu...
    /// Bits [5:5], mask 0x00000020
    pub const R_RD_REQ_OFFSET: u32 = 5;
    pub const R_RD_REQ_MASK: u32 = 0x00000020;
    pub const R_RD_REQ: u32 = 1 << 5;
    /// INACTIVE = 0: R_RD_REQ interrupt is inactive
    pub const R_RD_REQ_INACTIVE: u32 = 0 << 5;
    /// ACTIVE = 1: R_RD_REQ interrupt is active
    pub const R_RD_REQ_ACTIVE: u32 = 1 << 5;

    /// R_TX_EMPTY - See IC_RAW_INTR_STAT for a detailed description of R_TX_EMPTY bit. Reset va...
    /// Bits [4:4], mask 0x00000010
    pub const R_TX_EMPTY_OFFSET: u32 = 4;
    pub const R_TX_EMPTY_MASK: u32 = 0x00000010;
    pub const R_TX_EMPTY: u32 = 1 << 4;
    /// INACTIVE = 0: R_TX_EMPTY interrupt is inactive
    pub const R_TX_EMPTY_INACTIVE: u32 = 0 << 4;
    /// ACTIVE = 1: R_TX_EMPTY interrupt is active
    pub const R_TX_EMPTY_ACTIVE: u32 = 1 << 4;

    /// R_TX_OVER - See IC_RAW_INTR_STAT for a detailed description of R_TX_OVER bit. Reset val...
    /// Bits [3:3], mask 0x00000008
    pub const R_TX_OVER_OFFSET: u32 = 3;
    pub const R_TX_OVER_MASK: u32 = 0x00000008;
    pub const R_TX_OVER: u32 = 1 << 3;
    /// INACTIVE = 0: R_TX_OVER interrupt is inactive
    pub const R_TX_OVER_INACTIVE: u32 = 0 << 3;
    /// ACTIVE = 1: R_TX_OVER interrupt is active
    pub const R_TX_OVER_ACTIVE: u32 = 1 << 3;

    /// R_RX_FULL - See IC_RAW_INTR_STAT for a detailed description of R_RX_FULL bit. Reset val...
    /// Bits [2:2], mask 0x00000004
    pub const R_RX_FULL_OFFSET: u32 = 2;
    pub const R_RX_FULL_MASK: u32 = 0x00000004;
    pub const R_RX_FULL: u32 = 1 << 2;
    /// INACTIVE = 0: R_RX_FULL interrupt is inactive
    pub const R_RX_FULL_INACTIVE: u32 = 0 << 2;
    /// ACTIVE = 1: R_RX_FULL interrupt is active
    pub const R_RX_FULL_ACTIVE: u32 = 1 << 2;

    /// R_RX_OVER - See IC_RAW_INTR_STAT for a detailed description of R_RX_OVER bit. Reset val...
    /// Bits [1:1], mask 0x00000002
    pub const R_RX_OVER_OFFSET: u32 = 1;
    pub const R_RX_OVER_MASK: u32 = 0x00000002;
    pub const R_RX_OVER: u32 = 1 << 1;
    /// INACTIVE = 0: R_RX_OVER interrupt is inactive
    pub const R_RX_OVER_INACTIVE: u32 = 0 << 1;
    /// ACTIVE = 1: R_RX_OVER interrupt is active
    pub const R_RX_OVER_ACTIVE: u32 = 1 << 1;

    /// R_RX_UNDER - See IC_RAW_INTR_STAT for a detailed description of R_RX_UNDER bit. Reset va...
    /// Bits [0:0], mask 0x00000001
    pub const R_RX_UNDER_OFFSET: u32 = 0;
    pub const R_RX_UNDER_MASK: u32 = 0x00000001;
    pub const R_RX_UNDER: u32 = 1 << 0;
    /// INACTIVE = 0: RX_UNDER interrupt is inactive
    pub const R_RX_UNDER_INACTIVE: u32 = 0 << 0;
    /// ACTIVE = 1: RX_UNDER interrupt is active
    pub const R_RX_UNDER_ACTIVE: u32 = 1 << 0;

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

    /// **R_RESTART_DET** [12:12]: See IC_RAW_INTR_STAT for a detailed description of R_RESTART_DET bit. Reset...
    #[inline(always)]
    pub fn r_restart_det(&self) -> bool {
        (self.read_bits() & Self::R_RESTART_DET_MASK) != 0
    }

    /// **R_GEN_CALL** [11:11]: See IC_RAW_INTR_STAT for a detailed description of R_GEN_CALL bit. Reset va...
    #[inline(always)]
    pub fn r_gen_call(&self) -> bool {
        (self.read_bits() & Self::R_GEN_CALL_MASK) != 0
    }

    /// **R_START_DET** [10:10]: See IC_RAW_INTR_STAT for a detailed description of R_START_DET bit. Reset v...
    #[inline(always)]
    pub fn r_start_det(&self) -> bool {
        (self.read_bits() & Self::R_START_DET_MASK) != 0
    }

    /// **R_STOP_DET** [9:9]: See IC_RAW_INTR_STAT for a detailed description of R_STOP_DET bit. Reset va...
    #[inline(always)]
    pub fn r_stop_det(&self) -> bool {
        (self.read_bits() & Self::R_STOP_DET_MASK) != 0
    }

    /// **R_ACTIVITY** [8:8]: See IC_RAW_INTR_STAT for a detailed description of R_ACTIVITY bit. Reset va...
    #[inline(always)]
    pub fn r_activity(&self) -> bool {
        (self.read_bits() & Self::R_ACTIVITY_MASK) != 0
    }

    /// **R_RX_DONE** [7:7]: See IC_RAW_INTR_STAT for a detailed description of R_RX_DONE bit. Reset val...
    #[inline(always)]
    pub fn r_rx_done(&self) -> bool {
        (self.read_bits() & Self::R_RX_DONE_MASK) != 0
    }

    /// **R_TX_ABRT** [6:6]: See IC_RAW_INTR_STAT for a detailed description of R_TX_ABRT bit. Reset val...
    #[inline(always)]
    pub fn r_tx_abrt(&self) -> bool {
        (self.read_bits() & Self::R_TX_ABRT_MASK) != 0
    }

    /// **R_RD_REQ** [5:5]: See IC_RAW_INTR_STAT for a detailed description of R_RD_REQ bit. Reset valu...
    #[inline(always)]
    pub fn r_rd_req(&self) -> bool {
        (self.read_bits() & Self::R_RD_REQ_MASK) != 0
    }

    /// **R_TX_EMPTY** [4:4]: See IC_RAW_INTR_STAT for a detailed description of R_TX_EMPTY bit. Reset va...
    #[inline(always)]
    pub fn r_tx_empty(&self) -> bool {
        (self.read_bits() & Self::R_TX_EMPTY_MASK) != 0
    }

    /// **R_TX_OVER** [3:3]: See IC_RAW_INTR_STAT for a detailed description of R_TX_OVER bit. Reset val...
    #[inline(always)]
    pub fn r_tx_over(&self) -> bool {
        (self.read_bits() & Self::R_TX_OVER_MASK) != 0
    }

    /// **R_RX_FULL** [2:2]: See IC_RAW_INTR_STAT for a detailed description of R_RX_FULL bit. Reset val...
    #[inline(always)]
    pub fn r_rx_full(&self) -> bool {
        (self.read_bits() & Self::R_RX_FULL_MASK) != 0
    }

    /// **R_RX_OVER** [1:1]: See IC_RAW_INTR_STAT for a detailed description of R_RX_OVER bit. Reset val...
    #[inline(always)]
    pub fn r_rx_over(&self) -> bool {
        (self.read_bits() & Self::R_RX_OVER_MASK) != 0
    }

    /// **R_RX_UNDER** [0:0]: See IC_RAW_INTR_STAT for a detailed description of R_RX_UNDER bit. Reset va...
    #[inline(always)]
    pub fn r_rx_under(&self) -> bool {
        (self.read_bits() & Self::R_RX_UNDER_MASK) != 0
    }

}

/// IC_INTR_STAT value type for batch read/modify/write operations
///
/// Use `icintrstat.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `icintrstat.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IcIntrStatVal {
    bits: u32,
}

impl IcIntrStatVal {
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

    /// Read R_RESTART_DET [12:12]: See IC_RAW_INTR_STAT for a detailed description of R_RESTART_DET bit. Reset...
    #[inline(always)]
    pub const fn r_restart_det(&self) -> bool {
        (self.bits & IcIntrStat::R_RESTART_DET_MASK) != 0
    }

    /// Read R_GEN_CALL [11:11]: See IC_RAW_INTR_STAT for a detailed description of R_GEN_CALL bit. Reset va...
    #[inline(always)]
    pub const fn r_gen_call(&self) -> bool {
        (self.bits & IcIntrStat::R_GEN_CALL_MASK) != 0
    }

    /// Read R_START_DET [10:10]: See IC_RAW_INTR_STAT for a detailed description of R_START_DET bit. Reset v...
    #[inline(always)]
    pub const fn r_start_det(&self) -> bool {
        (self.bits & IcIntrStat::R_START_DET_MASK) != 0
    }

    /// Read R_STOP_DET [9:9]: See IC_RAW_INTR_STAT for a detailed description of R_STOP_DET bit. Reset va...
    #[inline(always)]
    pub const fn r_stop_det(&self) -> bool {
        (self.bits & IcIntrStat::R_STOP_DET_MASK) != 0
    }

    /// Read R_ACTIVITY [8:8]: See IC_RAW_INTR_STAT for a detailed description of R_ACTIVITY bit. Reset va...
    #[inline(always)]
    pub const fn r_activity(&self) -> bool {
        (self.bits & IcIntrStat::R_ACTIVITY_MASK) != 0
    }

    /// Read R_RX_DONE [7:7]: See IC_RAW_INTR_STAT for a detailed description of R_RX_DONE bit. Reset val...
    #[inline(always)]
    pub const fn r_rx_done(&self) -> bool {
        (self.bits & IcIntrStat::R_RX_DONE_MASK) != 0
    }

    /// Read R_TX_ABRT [6:6]: See IC_RAW_INTR_STAT for a detailed description of R_TX_ABRT bit. Reset val...
    #[inline(always)]
    pub const fn r_tx_abrt(&self) -> bool {
        (self.bits & IcIntrStat::R_TX_ABRT_MASK) != 0
    }

    /// Read R_RD_REQ [5:5]: See IC_RAW_INTR_STAT for a detailed description of R_RD_REQ bit. Reset valu...
    #[inline(always)]
    pub const fn r_rd_req(&self) -> bool {
        (self.bits & IcIntrStat::R_RD_REQ_MASK) != 0
    }

    /// Read R_TX_EMPTY [4:4]: See IC_RAW_INTR_STAT for a detailed description of R_TX_EMPTY bit. Reset va...
    #[inline(always)]
    pub const fn r_tx_empty(&self) -> bool {
        (self.bits & IcIntrStat::R_TX_EMPTY_MASK) != 0
    }

    /// Read R_TX_OVER [3:3]: See IC_RAW_INTR_STAT for a detailed description of R_TX_OVER bit. Reset val...
    #[inline(always)]
    pub const fn r_tx_over(&self) -> bool {
        (self.bits & IcIntrStat::R_TX_OVER_MASK) != 0
    }

    /// Read R_RX_FULL [2:2]: See IC_RAW_INTR_STAT for a detailed description of R_RX_FULL bit. Reset val...
    #[inline(always)]
    pub const fn r_rx_full(&self) -> bool {
        (self.bits & IcIntrStat::R_RX_FULL_MASK) != 0
    }

    /// Read R_RX_OVER [1:1]: See IC_RAW_INTR_STAT for a detailed description of R_RX_OVER bit. Reset val...
    #[inline(always)]
    pub const fn r_rx_over(&self) -> bool {
        (self.bits & IcIntrStat::R_RX_OVER_MASK) != 0
    }

    /// Read R_RX_UNDER [0:0]: See IC_RAW_INTR_STAT for a detailed description of R_RX_UNDER bit. Reset va...
    #[inline(always)]
    pub const fn r_rx_under(&self) -> bool {
        (self.bits & IcIntrStat::R_RX_UNDER_MASK) != 0
    }

}

impl Default for IcIntrStatVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl IcIntrStat {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> IcIntrStatVal {
        IcIntrStatVal::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: IcIntrStatVal) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(IcIntrStatVal) -> IcIntrStatVal>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// IC_INTR_MASK Register
///
/// I2C Interrupt Mask Register. These bits mask their corresponding interrupt
/// status bits. This register is active low; a value of 0 masks the interrupt,
/// whereas a value of 1 unmasks the interrupt.
///
/// Address offset: 0x0030
/// Reset value: 0x000008ff
///
/// ## Fields
///
/// - **M_RESTART_DET** [12] (RW): This bit masks the R_RESTART_DET interrupt in IC_INTR_STAT register. Reset ...
/// - **M_GEN_CALL** [11] (RW): This bit masks the R_GEN_CALL interrupt in IC_INTR_STAT register. Reset val...
/// - **M_START_DET** [10] (RW): This bit masks the R_START_DET interrupt in IC_INTR_STAT register. Reset va...
/// - **M_STOP_DET** [9] (RW): This bit masks the R_STOP_DET interrupt in IC_INTR_STAT register. Reset val...
/// - **M_ACTIVITY** [8] (RW): This bit masks the R_ACTIVITY interrupt in IC_INTR_STAT register. Reset val...
/// - **M_RX_DONE** [7] (RW): This bit masks the R_RX_DONE interrupt in IC_INTR_STAT register. Reset valu...
/// - **M_TX_ABRT** [6] (RW): This bit masks the R_TX_ABRT interrupt in IC_INTR_STAT register. Reset valu...
/// - **M_RD_REQ** [5] (RW): This bit masks the R_RD_REQ interrupt in IC_INTR_STAT register. Reset value...
/// - **M_TX_EMPTY** [4] (RW): This bit masks the R_TX_EMPTY interrupt in IC_INTR_STAT register. Reset val...
/// - **M_TX_OVER** [3] (RW): This bit masks the R_TX_OVER interrupt in IC_INTR_STAT register. Reset valu...
/// - **M_RX_FULL** [2] (RW): This bit masks the R_RX_FULL interrupt in IC_INTR_STAT register. Reset valu...
/// - **M_RX_OVER** [1] (RW): This bit masks the R_RX_OVER interrupt in IC_INTR_STAT register. Reset valu...
/// - **M_RX_UNDER** [0] (RW): This bit masks the R_RX_UNDER interrupt in IC_INTR_STAT register. Reset val...
#[repr(transparent)]
pub struct IcIntrMask {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for IcIntrMask {}
unsafe impl Sync for IcIntrMask {}

impl IcIntrMask {
    // ======== Field Constants ========

    /// M_RESTART_DET - This bit masks the R_RESTART_DET interrupt in IC_INTR_STAT register. Reset ...
    /// Bits [12:12], mask 0x00001000
    pub const M_RESTART_DET_OFFSET: u32 = 12;
    pub const M_RESTART_DET_MASK: u32 = 0x00001000;
    pub const M_RESTART_DET: u32 = 1 << 12;
    /// ENABLED = 0: RESTART_DET interrupt is masked
    pub const M_RESTART_DET_ENABLED: u32 = 0 << 12;
    /// DISABLED = 1: RESTART_DET interrupt is unmasked
    pub const M_RESTART_DET_DISABLED: u32 = 1 << 12;

    /// M_GEN_CALL - This bit masks the R_GEN_CALL interrupt in IC_INTR_STAT register. Reset val...
    /// Bits [11:11], mask 0x00000800
    pub const M_GEN_CALL_OFFSET: u32 = 11;
    pub const M_GEN_CALL_MASK: u32 = 0x00000800;
    pub const M_GEN_CALL: u32 = 1 << 11;
    /// ENABLED = 0: GEN_CALL interrupt is masked
    pub const M_GEN_CALL_ENABLED: u32 = 0 << 11;
    /// DISABLED = 1: GEN_CALL interrupt is unmasked
    pub const M_GEN_CALL_DISABLED: u32 = 1 << 11;

    /// M_START_DET - This bit masks the R_START_DET interrupt in IC_INTR_STAT register. Reset va...
    /// Bits [10:10], mask 0x00000400
    pub const M_START_DET_OFFSET: u32 = 10;
    pub const M_START_DET_MASK: u32 = 0x00000400;
    pub const M_START_DET: u32 = 1 << 10;
    /// ENABLED = 0: START_DET interrupt is masked
    pub const M_START_DET_ENABLED: u32 = 0 << 10;
    /// DISABLED = 1: START_DET interrupt is unmasked
    pub const M_START_DET_DISABLED: u32 = 1 << 10;

    /// M_STOP_DET - This bit masks the R_STOP_DET interrupt in IC_INTR_STAT register. Reset val...
    /// Bits [9:9], mask 0x00000200
    pub const M_STOP_DET_OFFSET: u32 = 9;
    pub const M_STOP_DET_MASK: u32 = 0x00000200;
    pub const M_STOP_DET: u32 = 1 << 9;
    /// ENABLED = 0: STOP_DET interrupt is masked
    pub const M_STOP_DET_ENABLED: u32 = 0 << 9;
    /// DISABLED = 1: STOP_DET interrupt is unmasked
    pub const M_STOP_DET_DISABLED: u32 = 1 << 9;

    /// M_ACTIVITY - This bit masks the R_ACTIVITY interrupt in IC_INTR_STAT register. Reset val...
    /// Bits [8:8], mask 0x00000100
    pub const M_ACTIVITY_OFFSET: u32 = 8;
    pub const M_ACTIVITY_MASK: u32 = 0x00000100;
    pub const M_ACTIVITY: u32 = 1 << 8;
    /// ENABLED = 0: ACTIVITY interrupt is masked
    pub const M_ACTIVITY_ENABLED: u32 = 0 << 8;
    /// DISABLED = 1: ACTIVITY interrupt is unmasked
    pub const M_ACTIVITY_DISABLED: u32 = 1 << 8;

    /// M_RX_DONE - This bit masks the R_RX_DONE interrupt in IC_INTR_STAT register. Reset valu...
    /// Bits [7:7], mask 0x00000080
    pub const M_RX_DONE_OFFSET: u32 = 7;
    pub const M_RX_DONE_MASK: u32 = 0x00000080;
    pub const M_RX_DONE: u32 = 1 << 7;
    /// ENABLED = 0: RX_DONE interrupt is masked
    pub const M_RX_DONE_ENABLED: u32 = 0 << 7;
    /// DISABLED = 1: RX_DONE interrupt is unmasked
    pub const M_RX_DONE_DISABLED: u32 = 1 << 7;

    /// M_TX_ABRT - This bit masks the R_TX_ABRT interrupt in IC_INTR_STAT register. Reset valu...
    /// Bits [6:6], mask 0x00000040
    pub const M_TX_ABRT_OFFSET: u32 = 6;
    pub const M_TX_ABRT_MASK: u32 = 0x00000040;
    pub const M_TX_ABRT: u32 = 1 << 6;
    /// ENABLED = 0: TX_ABORT interrupt is masked
    pub const M_TX_ABRT_ENABLED: u32 = 0 << 6;
    /// DISABLED = 1: TX_ABORT interrupt is unmasked
    pub const M_TX_ABRT_DISABLED: u32 = 1 << 6;

    /// M_RD_REQ - This bit masks the R_RD_REQ interrupt in IC_INTR_STAT register. Reset value...
    /// Bits [5:5], mask 0x00000020
    pub const M_RD_REQ_OFFSET: u32 = 5;
    pub const M_RD_REQ_MASK: u32 = 0x00000020;
    pub const M_RD_REQ: u32 = 1 << 5;
    /// ENABLED = 0: RD_REQ interrupt is masked
    pub const M_RD_REQ_ENABLED: u32 = 0 << 5;
    /// DISABLED = 1: RD_REQ interrupt is unmasked
    pub const M_RD_REQ_DISABLED: u32 = 1 << 5;

    /// M_TX_EMPTY - This bit masks the R_TX_EMPTY interrupt in IC_INTR_STAT register. Reset val...
    /// Bits [4:4], mask 0x00000010
    pub const M_TX_EMPTY_OFFSET: u32 = 4;
    pub const M_TX_EMPTY_MASK: u32 = 0x00000010;
    pub const M_TX_EMPTY: u32 = 1 << 4;
    /// ENABLED = 0: TX_EMPTY interrupt is masked
    pub const M_TX_EMPTY_ENABLED: u32 = 0 << 4;
    /// DISABLED = 1: TX_EMPTY interrupt is unmasked
    pub const M_TX_EMPTY_DISABLED: u32 = 1 << 4;

    /// M_TX_OVER - This bit masks the R_TX_OVER interrupt in IC_INTR_STAT register. Reset valu...
    /// Bits [3:3], mask 0x00000008
    pub const M_TX_OVER_OFFSET: u32 = 3;
    pub const M_TX_OVER_MASK: u32 = 0x00000008;
    pub const M_TX_OVER: u32 = 1 << 3;
    /// ENABLED = 0: TX_OVER interrupt is masked
    pub const M_TX_OVER_ENABLED: u32 = 0 << 3;
    /// DISABLED = 1: TX_OVER interrupt is unmasked
    pub const M_TX_OVER_DISABLED: u32 = 1 << 3;

    /// M_RX_FULL - This bit masks the R_RX_FULL interrupt in IC_INTR_STAT register. Reset valu...
    /// Bits [2:2], mask 0x00000004
    pub const M_RX_FULL_OFFSET: u32 = 2;
    pub const M_RX_FULL_MASK: u32 = 0x00000004;
    pub const M_RX_FULL: u32 = 1 << 2;
    /// ENABLED = 0: RX_FULL interrupt is masked
    pub const M_RX_FULL_ENABLED: u32 = 0 << 2;
    /// DISABLED = 1: RX_FULL interrupt is unmasked
    pub const M_RX_FULL_DISABLED: u32 = 1 << 2;

    /// M_RX_OVER - This bit masks the R_RX_OVER interrupt in IC_INTR_STAT register. Reset valu...
    /// Bits [1:1], mask 0x00000002
    pub const M_RX_OVER_OFFSET: u32 = 1;
    pub const M_RX_OVER_MASK: u32 = 0x00000002;
    pub const M_RX_OVER: u32 = 1 << 1;
    /// ENABLED = 0: RX_OVER interrupt is masked
    pub const M_RX_OVER_ENABLED: u32 = 0 << 1;
    /// DISABLED = 1: RX_OVER interrupt is unmasked
    pub const M_RX_OVER_DISABLED: u32 = 1 << 1;

    /// M_RX_UNDER - This bit masks the R_RX_UNDER interrupt in IC_INTR_STAT register. Reset val...
    /// Bits [0:0], mask 0x00000001
    pub const M_RX_UNDER_OFFSET: u32 = 0;
    pub const M_RX_UNDER_MASK: u32 = 0x00000001;
    pub const M_RX_UNDER: u32 = 1 << 0;
    /// ENABLED = 0: RX_UNDER interrupt is masked
    pub const M_RX_UNDER_ENABLED: u32 = 0 << 0;
    /// DISABLED = 1: RX_UNDER interrupt is unmasked
    pub const M_RX_UNDER_DISABLED: u32 = 1 << 0;

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

    /// **M_RESTART_DET** [12:12]: This bit masks the R_RESTART_DET interrupt in IC_INTR_STAT register. Reset ...
    #[inline(always)]
    pub fn m_restart_det(&self) -> bool {
        (self.read_bits() & Self::M_RESTART_DET_MASK) != 0
    }

    /// Set M_RESTART_DET bit
    #[inline(always)]
    pub fn set_m_restart_det(&self) {
        self.set_bits(Self::M_RESTART_DET_MASK);
    }

    /// Clear M_RESTART_DET bit
    #[inline(always)]
    pub fn clr_m_restart_det(&self) {
        self.clr_bits(Self::M_RESTART_DET_MASK);
    }

    /// **M_GEN_CALL** [11:11]: This bit masks the R_GEN_CALL interrupt in IC_INTR_STAT register. Reset val...
    #[inline(always)]
    pub fn m_gen_call(&self) -> bool {
        (self.read_bits() & Self::M_GEN_CALL_MASK) != 0
    }

    /// Set M_GEN_CALL bit
    #[inline(always)]
    pub fn set_m_gen_call(&self) {
        self.set_bits(Self::M_GEN_CALL_MASK);
    }

    /// Clear M_GEN_CALL bit
    #[inline(always)]
    pub fn clr_m_gen_call(&self) {
        self.clr_bits(Self::M_GEN_CALL_MASK);
    }

    /// **M_START_DET** [10:10]: This bit masks the R_START_DET interrupt in IC_INTR_STAT register. Reset va...
    #[inline(always)]
    pub fn m_start_det(&self) -> bool {
        (self.read_bits() & Self::M_START_DET_MASK) != 0
    }

    /// Set M_START_DET bit
    #[inline(always)]
    pub fn set_m_start_det(&self) {
        self.set_bits(Self::M_START_DET_MASK);
    }

    /// Clear M_START_DET bit
    #[inline(always)]
    pub fn clr_m_start_det(&self) {
        self.clr_bits(Self::M_START_DET_MASK);
    }

    /// **M_STOP_DET** [9:9]: This bit masks the R_STOP_DET interrupt in IC_INTR_STAT register. Reset val...
    #[inline(always)]
    pub fn m_stop_det(&self) -> bool {
        (self.read_bits() & Self::M_STOP_DET_MASK) != 0
    }

    /// Set M_STOP_DET bit
    #[inline(always)]
    pub fn set_m_stop_det(&self) {
        self.set_bits(Self::M_STOP_DET_MASK);
    }

    /// Clear M_STOP_DET bit
    #[inline(always)]
    pub fn clr_m_stop_det(&self) {
        self.clr_bits(Self::M_STOP_DET_MASK);
    }

    /// **M_ACTIVITY** [8:8]: This bit masks the R_ACTIVITY interrupt in IC_INTR_STAT register. Reset val...
    #[inline(always)]
    pub fn m_activity(&self) -> bool {
        (self.read_bits() & Self::M_ACTIVITY_MASK) != 0
    }

    /// Set M_ACTIVITY bit
    #[inline(always)]
    pub fn set_m_activity(&self) {
        self.set_bits(Self::M_ACTIVITY_MASK);
    }

    /// Clear M_ACTIVITY bit
    #[inline(always)]
    pub fn clr_m_activity(&self) {
        self.clr_bits(Self::M_ACTIVITY_MASK);
    }

    /// **M_RX_DONE** [7:7]: This bit masks the R_RX_DONE interrupt in IC_INTR_STAT register. Reset valu...
    #[inline(always)]
    pub fn m_rx_done(&self) -> bool {
        (self.read_bits() & Self::M_RX_DONE_MASK) != 0
    }

    /// Set M_RX_DONE bit
    #[inline(always)]
    pub fn set_m_rx_done(&self) {
        self.set_bits(Self::M_RX_DONE_MASK);
    }

    /// Clear M_RX_DONE bit
    #[inline(always)]
    pub fn clr_m_rx_done(&self) {
        self.clr_bits(Self::M_RX_DONE_MASK);
    }

    /// **M_TX_ABRT** [6:6]: This bit masks the R_TX_ABRT interrupt in IC_INTR_STAT register. Reset valu...
    #[inline(always)]
    pub fn m_tx_abrt(&self) -> bool {
        (self.read_bits() & Self::M_TX_ABRT_MASK) != 0
    }

    /// Set M_TX_ABRT bit
    #[inline(always)]
    pub fn set_m_tx_abrt(&self) {
        self.set_bits(Self::M_TX_ABRT_MASK);
    }

    /// Clear M_TX_ABRT bit
    #[inline(always)]
    pub fn clr_m_tx_abrt(&self) {
        self.clr_bits(Self::M_TX_ABRT_MASK);
    }

    /// **M_RD_REQ** [5:5]: This bit masks the R_RD_REQ interrupt in IC_INTR_STAT register. Reset value...
    #[inline(always)]
    pub fn m_rd_req(&self) -> bool {
        (self.read_bits() & Self::M_RD_REQ_MASK) != 0
    }

    /// Set M_RD_REQ bit
    #[inline(always)]
    pub fn set_m_rd_req(&self) {
        self.set_bits(Self::M_RD_REQ_MASK);
    }

    /// Clear M_RD_REQ bit
    #[inline(always)]
    pub fn clr_m_rd_req(&self) {
        self.clr_bits(Self::M_RD_REQ_MASK);
    }

    /// **M_TX_EMPTY** [4:4]: This bit masks the R_TX_EMPTY interrupt in IC_INTR_STAT register. Reset val...
    #[inline(always)]
    pub fn m_tx_empty(&self) -> bool {
        (self.read_bits() & Self::M_TX_EMPTY_MASK) != 0
    }

    /// Set M_TX_EMPTY bit
    #[inline(always)]
    pub fn set_m_tx_empty(&self) {
        self.set_bits(Self::M_TX_EMPTY_MASK);
    }

    /// Clear M_TX_EMPTY bit
    #[inline(always)]
    pub fn clr_m_tx_empty(&self) {
        self.clr_bits(Self::M_TX_EMPTY_MASK);
    }

    /// **M_TX_OVER** [3:3]: This bit masks the R_TX_OVER interrupt in IC_INTR_STAT register. Reset valu...
    #[inline(always)]
    pub fn m_tx_over(&self) -> bool {
        (self.read_bits() & Self::M_TX_OVER_MASK) != 0
    }

    /// Set M_TX_OVER bit
    #[inline(always)]
    pub fn set_m_tx_over(&self) {
        self.set_bits(Self::M_TX_OVER_MASK);
    }

    /// Clear M_TX_OVER bit
    #[inline(always)]
    pub fn clr_m_tx_over(&self) {
        self.clr_bits(Self::M_TX_OVER_MASK);
    }

    /// **M_RX_FULL** [2:2]: This bit masks the R_RX_FULL interrupt in IC_INTR_STAT register. Reset valu...
    #[inline(always)]
    pub fn m_rx_full(&self) -> bool {
        (self.read_bits() & Self::M_RX_FULL_MASK) != 0
    }

    /// Set M_RX_FULL bit
    #[inline(always)]
    pub fn set_m_rx_full(&self) {
        self.set_bits(Self::M_RX_FULL_MASK);
    }

    /// Clear M_RX_FULL bit
    #[inline(always)]
    pub fn clr_m_rx_full(&self) {
        self.clr_bits(Self::M_RX_FULL_MASK);
    }

    /// **M_RX_OVER** [1:1]: This bit masks the R_RX_OVER interrupt in IC_INTR_STAT register. Reset valu...
    #[inline(always)]
    pub fn m_rx_over(&self) -> bool {
        (self.read_bits() & Self::M_RX_OVER_MASK) != 0
    }

    /// Set M_RX_OVER bit
    #[inline(always)]
    pub fn set_m_rx_over(&self) {
        self.set_bits(Self::M_RX_OVER_MASK);
    }

    /// Clear M_RX_OVER bit
    #[inline(always)]
    pub fn clr_m_rx_over(&self) {
        self.clr_bits(Self::M_RX_OVER_MASK);
    }

    /// **M_RX_UNDER** [0:0]: This bit masks the R_RX_UNDER interrupt in IC_INTR_STAT register. Reset val...
    #[inline(always)]
    pub fn m_rx_under(&self) -> bool {
        (self.read_bits() & Self::M_RX_UNDER_MASK) != 0
    }

    /// Set M_RX_UNDER bit
    #[inline(always)]
    pub fn set_m_rx_under(&self) {
        self.set_bits(Self::M_RX_UNDER_MASK);
    }

    /// Clear M_RX_UNDER bit
    #[inline(always)]
    pub fn clr_m_rx_under(&self) {
        self.clr_bits(Self::M_RX_UNDER_MASK);
    }

}

/// IC_INTR_MASK value type for batch read/modify/write operations
///
/// Use `icintrmask.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `icintrmask.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IcIntrMaskVal {
    bits: u32,
}

impl IcIntrMaskVal {
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
        Self { bits: 0x000008ff }
    }

    /// Read M_RESTART_DET [12:12]: This bit masks the R_RESTART_DET interrupt in IC_INTR_STAT register. Reset ...
    #[inline(always)]
    pub const fn m_restart_det(&self) -> bool {
        (self.bits & IcIntrMask::M_RESTART_DET_MASK) != 0
    }

    /// Read M_GEN_CALL [11:11]: This bit masks the R_GEN_CALL interrupt in IC_INTR_STAT register. Reset val...
    #[inline(always)]
    pub const fn m_gen_call(&self) -> bool {
        (self.bits & IcIntrMask::M_GEN_CALL_MASK) != 0
    }

    /// Read M_START_DET [10:10]: This bit masks the R_START_DET interrupt in IC_INTR_STAT register. Reset va...
    #[inline(always)]
    pub const fn m_start_det(&self) -> bool {
        (self.bits & IcIntrMask::M_START_DET_MASK) != 0
    }

    /// Read M_STOP_DET [9:9]: This bit masks the R_STOP_DET interrupt in IC_INTR_STAT register. Reset val...
    #[inline(always)]
    pub const fn m_stop_det(&self) -> bool {
        (self.bits & IcIntrMask::M_STOP_DET_MASK) != 0
    }

    /// Read M_ACTIVITY [8:8]: This bit masks the R_ACTIVITY interrupt in IC_INTR_STAT register. Reset val...
    #[inline(always)]
    pub const fn m_activity(&self) -> bool {
        (self.bits & IcIntrMask::M_ACTIVITY_MASK) != 0
    }

    /// Read M_RX_DONE [7:7]: This bit masks the R_RX_DONE interrupt in IC_INTR_STAT register. Reset valu...
    #[inline(always)]
    pub const fn m_rx_done(&self) -> bool {
        (self.bits & IcIntrMask::M_RX_DONE_MASK) != 0
    }

    /// Read M_TX_ABRT [6:6]: This bit masks the R_TX_ABRT interrupt in IC_INTR_STAT register. Reset valu...
    #[inline(always)]
    pub const fn m_tx_abrt(&self) -> bool {
        (self.bits & IcIntrMask::M_TX_ABRT_MASK) != 0
    }

    /// Read M_RD_REQ [5:5]: This bit masks the R_RD_REQ interrupt in IC_INTR_STAT register. Reset value...
    #[inline(always)]
    pub const fn m_rd_req(&self) -> bool {
        (self.bits & IcIntrMask::M_RD_REQ_MASK) != 0
    }

    /// Read M_TX_EMPTY [4:4]: This bit masks the R_TX_EMPTY interrupt in IC_INTR_STAT register. Reset val...
    #[inline(always)]
    pub const fn m_tx_empty(&self) -> bool {
        (self.bits & IcIntrMask::M_TX_EMPTY_MASK) != 0
    }

    /// Read M_TX_OVER [3:3]: This bit masks the R_TX_OVER interrupt in IC_INTR_STAT register. Reset valu...
    #[inline(always)]
    pub const fn m_tx_over(&self) -> bool {
        (self.bits & IcIntrMask::M_TX_OVER_MASK) != 0
    }

    /// Read M_RX_FULL [2:2]: This bit masks the R_RX_FULL interrupt in IC_INTR_STAT register. Reset valu...
    #[inline(always)]
    pub const fn m_rx_full(&self) -> bool {
        (self.bits & IcIntrMask::M_RX_FULL_MASK) != 0
    }

    /// Read M_RX_OVER [1:1]: This bit masks the R_RX_OVER interrupt in IC_INTR_STAT register. Reset valu...
    #[inline(always)]
    pub const fn m_rx_over(&self) -> bool {
        (self.bits & IcIntrMask::M_RX_OVER_MASK) != 0
    }

    /// Read M_RX_UNDER [0:0]: This bit masks the R_RX_UNDER interrupt in IC_INTR_STAT register. Reset val...
    #[inline(always)]
    pub const fn m_rx_under(&self) -> bool {
        (self.bits & IcIntrMask::M_RX_UNDER_MASK) != 0
    }

    /// Set M_RESTART_DET [12:12]: This bit masks the R_RESTART_DET interrupt in IC_INTR_STAT register. Reset ...
    #[inline(always)]
    pub const fn with_m_restart_det(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IcIntrMask::M_RESTART_DET_MASK
            } else {
                self.bits & !IcIntrMask::M_RESTART_DET_MASK
            }
        }
    }

    /// Set M_GEN_CALL [11:11]: This bit masks the R_GEN_CALL interrupt in IC_INTR_STAT register. Reset val...
    #[inline(always)]
    pub const fn with_m_gen_call(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IcIntrMask::M_GEN_CALL_MASK
            } else {
                self.bits & !IcIntrMask::M_GEN_CALL_MASK
            }
        }
    }

    /// Set M_START_DET [10:10]: This bit masks the R_START_DET interrupt in IC_INTR_STAT register. Reset va...
    #[inline(always)]
    pub const fn with_m_start_det(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IcIntrMask::M_START_DET_MASK
            } else {
                self.bits & !IcIntrMask::M_START_DET_MASK
            }
        }
    }

    /// Set M_STOP_DET [9:9]: This bit masks the R_STOP_DET interrupt in IC_INTR_STAT register. Reset val...
    #[inline(always)]
    pub const fn with_m_stop_det(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IcIntrMask::M_STOP_DET_MASK
            } else {
                self.bits & !IcIntrMask::M_STOP_DET_MASK
            }
        }
    }

    /// Set M_ACTIVITY [8:8]: This bit masks the R_ACTIVITY interrupt in IC_INTR_STAT register. Reset val...
    #[inline(always)]
    pub const fn with_m_activity(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IcIntrMask::M_ACTIVITY_MASK
            } else {
                self.bits & !IcIntrMask::M_ACTIVITY_MASK
            }
        }
    }

    /// Set M_RX_DONE [7:7]: This bit masks the R_RX_DONE interrupt in IC_INTR_STAT register. Reset valu...
    #[inline(always)]
    pub const fn with_m_rx_done(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IcIntrMask::M_RX_DONE_MASK
            } else {
                self.bits & !IcIntrMask::M_RX_DONE_MASK
            }
        }
    }

    /// Set M_TX_ABRT [6:6]: This bit masks the R_TX_ABRT interrupt in IC_INTR_STAT register. Reset valu...
    #[inline(always)]
    pub const fn with_m_tx_abrt(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IcIntrMask::M_TX_ABRT_MASK
            } else {
                self.bits & !IcIntrMask::M_TX_ABRT_MASK
            }
        }
    }

    /// Set M_RD_REQ [5:5]: This bit masks the R_RD_REQ interrupt in IC_INTR_STAT register. Reset value...
    #[inline(always)]
    pub const fn with_m_rd_req(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IcIntrMask::M_RD_REQ_MASK
            } else {
                self.bits & !IcIntrMask::M_RD_REQ_MASK
            }
        }
    }

    /// Set M_TX_EMPTY [4:4]: This bit masks the R_TX_EMPTY interrupt in IC_INTR_STAT register. Reset val...
    #[inline(always)]
    pub const fn with_m_tx_empty(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IcIntrMask::M_TX_EMPTY_MASK
            } else {
                self.bits & !IcIntrMask::M_TX_EMPTY_MASK
            }
        }
    }

    /// Set M_TX_OVER [3:3]: This bit masks the R_TX_OVER interrupt in IC_INTR_STAT register. Reset valu...
    #[inline(always)]
    pub const fn with_m_tx_over(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IcIntrMask::M_TX_OVER_MASK
            } else {
                self.bits & !IcIntrMask::M_TX_OVER_MASK
            }
        }
    }

    /// Set M_RX_FULL [2:2]: This bit masks the R_RX_FULL interrupt in IC_INTR_STAT register. Reset valu...
    #[inline(always)]
    pub const fn with_m_rx_full(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IcIntrMask::M_RX_FULL_MASK
            } else {
                self.bits & !IcIntrMask::M_RX_FULL_MASK
            }
        }
    }

    /// Set M_RX_OVER [1:1]: This bit masks the R_RX_OVER interrupt in IC_INTR_STAT register. Reset valu...
    #[inline(always)]
    pub const fn with_m_rx_over(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IcIntrMask::M_RX_OVER_MASK
            } else {
                self.bits & !IcIntrMask::M_RX_OVER_MASK
            }
        }
    }

    /// Set M_RX_UNDER [0:0]: This bit masks the R_RX_UNDER interrupt in IC_INTR_STAT register. Reset val...
    #[inline(always)]
    pub const fn with_m_rx_under(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IcIntrMask::M_RX_UNDER_MASK
            } else {
                self.bits & !IcIntrMask::M_RX_UNDER_MASK
            }
        }
    }

}

impl Default for IcIntrMaskVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl IcIntrMask {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> IcIntrMaskVal {
        IcIntrMaskVal::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: IcIntrMaskVal) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(IcIntrMaskVal) -> IcIntrMaskVal>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// IC_RAW_INTR_STAT Register
///
/// I2C Raw Interrupt Status Register Unlike the IC_INTR_STAT register, these
/// bits are not masked so they always show the true status of the DW_apb_i2c.
///
/// Address offset: 0x0034
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **RESTART_DET** [12] (RO): Indicates whether a RESTART condition has occurred on the I2C interface whe...
/// - **GEN_CALL** [11] (RO): Set only when a General Call address is received and it is acknowledged. It...
/// - **START_DET** [10] (RO): Indicates whether a START or RESTART condition has occurred on the I2C inte...
/// - **STOP_DET** [9] (RO): Indicates whether a STOP condition has occurred on the I2C interface regard...
/// - **ACTIVITY** [8] (RO): This bit captures DW_apb_i2c activity and stays set until it is cleared. Th...
/// - **RX_DONE** [7] (RO): When the DW_apb_i2c is acting as a slave-transmitter, this bit is set to 1 ...
/// - **TX_ABRT** [6] (RO): This bit indicates if DW_apb_i2c, as an I2C transmitter, is unable to compl...
/// - **RD_REQ** [5] (RO): This bit is set to 1 when DW_apb_i2c is acting as a slave and another I2C m...
/// - **TX_EMPTY** [4] (RO): The behavior of the TX_EMPTY interrupt status differs based on the TX_EMPTY...
/// - **TX_OVER** [3] (RO): Set during transmit if the transmit buffer is filled to IC_TX_BUFFER_DEPTH ...
/// - **RX_FULL** [2] (RO): Set when the receive buffer reaches or goes above the RX_TL threshold in th...
/// - **RX_OVER** [1] (RO): Set if the receive buffer is completely filled to IC_RX_BUFFER_DEPTH and an...
/// - **RX_UNDER** [0] (RO): Set if the processor attempts to read the receive buffer when it is empty b...
#[repr(transparent)]
pub struct IcRawIntrStat {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for IcRawIntrStat {}
unsafe impl Sync for IcRawIntrStat {}

impl IcRawIntrStat {
    // ======== Field Constants ========

    /// RESTART_DET - Indicates whether a RESTART condition has occurred on the I2C interface whe...
    /// Bits [12:12], mask 0x00001000
    pub const RESTART_DET_OFFSET: u32 = 12;
    pub const RESTART_DET_MASK: u32 = 0x00001000;
    pub const RESTART_DET: u32 = 1 << 12;
    /// INACTIVE = 0: RESTART_DET interrupt is inactive
    pub const RESTART_DET_INACTIVE: u32 = 0 << 12;
    /// ACTIVE = 1: RESTART_DET interrupt is active
    pub const RESTART_DET_ACTIVE: u32 = 1 << 12;

    /// GEN_CALL - Set only when a General Call address is received and it is acknowledged. It...
    /// Bits [11:11], mask 0x00000800
    pub const GEN_CALL_OFFSET: u32 = 11;
    pub const GEN_CALL_MASK: u32 = 0x00000800;
    pub const GEN_CALL: u32 = 1 << 11;
    /// INACTIVE = 0: GEN_CALL interrupt is inactive
    pub const GEN_CALL_INACTIVE: u32 = 0 << 11;
    /// ACTIVE = 1: GEN_CALL interrupt is active
    pub const GEN_CALL_ACTIVE: u32 = 1 << 11;

    /// START_DET - Indicates whether a START or RESTART condition has occurred on the I2C inte...
    /// Bits [10:10], mask 0x00000400
    pub const START_DET_OFFSET: u32 = 10;
    pub const START_DET_MASK: u32 = 0x00000400;
    pub const START_DET: u32 = 1 << 10;
    /// INACTIVE = 0: START_DET interrupt is inactive
    pub const START_DET_INACTIVE: u32 = 0 << 10;
    /// ACTIVE = 1: START_DET interrupt is active
    pub const START_DET_ACTIVE: u32 = 1 << 10;

    /// STOP_DET - Indicates whether a STOP condition has occurred on the I2C interface regard...
    /// Bits [9:9], mask 0x00000200
    pub const STOP_DET_OFFSET: u32 = 9;
    pub const STOP_DET_MASK: u32 = 0x00000200;
    pub const STOP_DET: u32 = 1 << 9;
    /// INACTIVE = 0: STOP_DET interrupt is inactive
    pub const STOP_DET_INACTIVE: u32 = 0 << 9;
    /// ACTIVE = 1: STOP_DET interrupt is active
    pub const STOP_DET_ACTIVE: u32 = 1 << 9;

    /// ACTIVITY - This bit captures DW_apb_i2c activity and stays set until it is cleared. Th...
    /// Bits [8:8], mask 0x00000100
    pub const ACTIVITY_OFFSET: u32 = 8;
    pub const ACTIVITY_MASK: u32 = 0x00000100;
    pub const ACTIVITY: u32 = 1 << 8;
    /// INACTIVE = 0: RAW_INTR_ACTIVITY interrupt is inactive
    pub const ACTIVITY_INACTIVE: u32 = 0 << 8;
    /// ACTIVE = 1: RAW_INTR_ACTIVITY interrupt is active
    pub const ACTIVITY_ACTIVE: u32 = 1 << 8;

    /// RX_DONE - When the DW_apb_i2c is acting as a slave-transmitter, this bit is set to 1 ...
    /// Bits [7:7], mask 0x00000080
    pub const RX_DONE_OFFSET: u32 = 7;
    pub const RX_DONE_MASK: u32 = 0x00000080;
    pub const RX_DONE: u32 = 1 << 7;
    /// INACTIVE = 0: RX_DONE interrupt is inactive
    pub const RX_DONE_INACTIVE: u32 = 0 << 7;
    /// ACTIVE = 1: RX_DONE interrupt is active
    pub const RX_DONE_ACTIVE: u32 = 1 << 7;

    /// TX_ABRT - This bit indicates if DW_apb_i2c, as an I2C transmitter, is unable to compl...
    /// Bits [6:6], mask 0x00000040
    pub const TX_ABRT_OFFSET: u32 = 6;
    pub const TX_ABRT_MASK: u32 = 0x00000040;
    pub const TX_ABRT: u32 = 1 << 6;
    /// INACTIVE = 0: TX_ABRT interrupt is inactive
    pub const TX_ABRT_INACTIVE: u32 = 0 << 6;
    /// ACTIVE = 1: TX_ABRT interrupt is active
    pub const TX_ABRT_ACTIVE: u32 = 1 << 6;

    /// RD_REQ - This bit is set to 1 when DW_apb_i2c is acting as a slave and another I2C m...
    /// Bits [5:5], mask 0x00000020
    pub const RD_REQ_OFFSET: u32 = 5;
    pub const RD_REQ_MASK: u32 = 0x00000020;
    pub const RD_REQ: u32 = 1 << 5;
    /// INACTIVE = 0: RD_REQ interrupt is inactive
    pub const RD_REQ_INACTIVE: u32 = 0 << 5;
    /// ACTIVE = 1: RD_REQ interrupt is active
    pub const RD_REQ_ACTIVE: u32 = 1 << 5;

    /// TX_EMPTY - The behavior of the TX_EMPTY interrupt status differs based on the TX_EMPTY...
    /// Bits [4:4], mask 0x00000010
    pub const TX_EMPTY_OFFSET: u32 = 4;
    pub const TX_EMPTY_MASK: u32 = 0x00000010;
    pub const TX_EMPTY: u32 = 1 << 4;
    /// INACTIVE = 0: TX_EMPTY interrupt is inactive
    pub const TX_EMPTY_INACTIVE: u32 = 0 << 4;
    /// ACTIVE = 1: TX_EMPTY interrupt is active
    pub const TX_EMPTY_ACTIVE: u32 = 1 << 4;

    /// TX_OVER - Set during transmit if the transmit buffer is filled to IC_TX_BUFFER_DEPTH ...
    /// Bits [3:3], mask 0x00000008
    pub const TX_OVER_OFFSET: u32 = 3;
    pub const TX_OVER_MASK: u32 = 0x00000008;
    pub const TX_OVER: u32 = 1 << 3;
    /// INACTIVE = 0: TX_OVER interrupt is inactive
    pub const TX_OVER_INACTIVE: u32 = 0 << 3;
    /// ACTIVE = 1: TX_OVER interrupt is active
    pub const TX_OVER_ACTIVE: u32 = 1 << 3;

    /// RX_FULL - Set when the receive buffer reaches or goes above the RX_TL threshold in th...
    /// Bits [2:2], mask 0x00000004
    pub const RX_FULL_OFFSET: u32 = 2;
    pub const RX_FULL_MASK: u32 = 0x00000004;
    pub const RX_FULL: u32 = 1 << 2;
    /// INACTIVE = 0: RX_FULL interrupt is inactive
    pub const RX_FULL_INACTIVE: u32 = 0 << 2;
    /// ACTIVE = 1: RX_FULL interrupt is active
    pub const RX_FULL_ACTIVE: u32 = 1 << 2;

    /// RX_OVER - Set if the receive buffer is completely filled to IC_RX_BUFFER_DEPTH and an...
    /// Bits [1:1], mask 0x00000002
    pub const RX_OVER_OFFSET: u32 = 1;
    pub const RX_OVER_MASK: u32 = 0x00000002;
    pub const RX_OVER: u32 = 1 << 1;
    /// INACTIVE = 0: RX_OVER interrupt is inactive
    pub const RX_OVER_INACTIVE: u32 = 0 << 1;
    /// ACTIVE = 1: RX_OVER interrupt is active
    pub const RX_OVER_ACTIVE: u32 = 1 << 1;

    /// RX_UNDER - Set if the processor attempts to read the receive buffer when it is empty b...
    /// Bits [0:0], mask 0x00000001
    pub const RX_UNDER_OFFSET: u32 = 0;
    pub const RX_UNDER_MASK: u32 = 0x00000001;
    pub const RX_UNDER: u32 = 1 << 0;
    /// INACTIVE = 0: RX_UNDER interrupt is inactive
    pub const RX_UNDER_INACTIVE: u32 = 0 << 0;
    /// ACTIVE = 1: RX_UNDER interrupt is active
    pub const RX_UNDER_ACTIVE: u32 = 1 << 0;

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

    /// **RESTART_DET** [12:12]: Indicates whether a RESTART condition has occurred on the I2C interface whe...
    #[inline(always)]
    pub fn restart_det(&self) -> bool {
        (self.read_bits() & Self::RESTART_DET_MASK) != 0
    }

    /// **GEN_CALL** [11:11]: Set only when a General Call address is received and it is acknowledged. It...
    #[inline(always)]
    pub fn gen_call(&self) -> bool {
        (self.read_bits() & Self::GEN_CALL_MASK) != 0
    }

    /// **START_DET** [10:10]: Indicates whether a START or RESTART condition has occurred on the I2C inte...
    #[inline(always)]
    pub fn start_det(&self) -> bool {
        (self.read_bits() & Self::START_DET_MASK) != 0
    }

    /// **STOP_DET** [9:9]: Indicates whether a STOP condition has occurred on the I2C interface regard...
    #[inline(always)]
    pub fn stop_det(&self) -> bool {
        (self.read_bits() & Self::STOP_DET_MASK) != 0
    }

    /// **ACTIVITY** [8:8]: This bit captures DW_apb_i2c activity and stays set until it is cleared. Th...
    #[inline(always)]
    pub fn activity(&self) -> bool {
        (self.read_bits() & Self::ACTIVITY_MASK) != 0
    }

    /// **RX_DONE** [7:7]: When the DW_apb_i2c is acting as a slave-transmitter, this bit is set to 1 ...
    #[inline(always)]
    pub fn rx_done(&self) -> bool {
        (self.read_bits() & Self::RX_DONE_MASK) != 0
    }

    /// **TX_ABRT** [6:6]: This bit indicates if DW_apb_i2c, as an I2C transmitter, is unable to compl...
    #[inline(always)]
    pub fn tx_abrt(&self) -> bool {
        (self.read_bits() & Self::TX_ABRT_MASK) != 0
    }

    /// **RD_REQ** [5:5]: This bit is set to 1 when DW_apb_i2c is acting as a slave and another I2C m...
    #[inline(always)]
    pub fn rd_req(&self) -> bool {
        (self.read_bits() & Self::RD_REQ_MASK) != 0
    }

    /// **TX_EMPTY** [4:4]: The behavior of the TX_EMPTY interrupt status differs based on the TX_EMPTY...
    #[inline(always)]
    pub fn tx_empty(&self) -> bool {
        (self.read_bits() & Self::TX_EMPTY_MASK) != 0
    }

    /// **TX_OVER** [3:3]: Set during transmit if the transmit buffer is filled to IC_TX_BUFFER_DEPTH ...
    #[inline(always)]
    pub fn tx_over(&self) -> bool {
        (self.read_bits() & Self::TX_OVER_MASK) != 0
    }

    /// **RX_FULL** [2:2]: Set when the receive buffer reaches or goes above the RX_TL threshold in th...
    #[inline(always)]
    pub fn rx_full(&self) -> bool {
        (self.read_bits() & Self::RX_FULL_MASK) != 0
    }

    /// **RX_OVER** [1:1]: Set if the receive buffer is completely filled to IC_RX_BUFFER_DEPTH and an...
    #[inline(always)]
    pub fn rx_over(&self) -> bool {
        (self.read_bits() & Self::RX_OVER_MASK) != 0
    }

    /// **RX_UNDER** [0:0]: Set if the processor attempts to read the receive buffer when it is empty b...
    #[inline(always)]
    pub fn rx_under(&self) -> bool {
        (self.read_bits() & Self::RX_UNDER_MASK) != 0
    }

}

/// IC_RAW_INTR_STAT value type for batch read/modify/write operations
///
/// Use `icrawintrstat.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `icrawintrstat.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IcRawIntrStatVal {
    bits: u32,
}

impl IcRawIntrStatVal {
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

    /// Read RESTART_DET [12:12]: Indicates whether a RESTART condition has occurred on the I2C interface whe...
    #[inline(always)]
    pub const fn restart_det(&self) -> bool {
        (self.bits & IcRawIntrStat::RESTART_DET_MASK) != 0
    }

    /// Read GEN_CALL [11:11]: Set only when a General Call address is received and it is acknowledged. It...
    #[inline(always)]
    pub const fn gen_call(&self) -> bool {
        (self.bits & IcRawIntrStat::GEN_CALL_MASK) != 0
    }

    /// Read START_DET [10:10]: Indicates whether a START or RESTART condition has occurred on the I2C inte...
    #[inline(always)]
    pub const fn start_det(&self) -> bool {
        (self.bits & IcRawIntrStat::START_DET_MASK) != 0
    }

    /// Read STOP_DET [9:9]: Indicates whether a STOP condition has occurred on the I2C interface regard...
    #[inline(always)]
    pub const fn stop_det(&self) -> bool {
        (self.bits & IcRawIntrStat::STOP_DET_MASK) != 0
    }

    /// Read ACTIVITY [8:8]: This bit captures DW_apb_i2c activity and stays set until it is cleared. Th...
    #[inline(always)]
    pub const fn activity(&self) -> bool {
        (self.bits & IcRawIntrStat::ACTIVITY_MASK) != 0
    }

    /// Read RX_DONE [7:7]: When the DW_apb_i2c is acting as a slave-transmitter, this bit is set to 1 ...
    #[inline(always)]
    pub const fn rx_done(&self) -> bool {
        (self.bits & IcRawIntrStat::RX_DONE_MASK) != 0
    }

    /// Read TX_ABRT [6:6]: This bit indicates if DW_apb_i2c, as an I2C transmitter, is unable to compl...
    #[inline(always)]
    pub const fn tx_abrt(&self) -> bool {
        (self.bits & IcRawIntrStat::TX_ABRT_MASK) != 0
    }

    /// Read RD_REQ [5:5]: This bit is set to 1 when DW_apb_i2c is acting as a slave and another I2C m...
    #[inline(always)]
    pub const fn rd_req(&self) -> bool {
        (self.bits & IcRawIntrStat::RD_REQ_MASK) != 0
    }

    /// Read TX_EMPTY [4:4]: The behavior of the TX_EMPTY interrupt status differs based on the TX_EMPTY...
    #[inline(always)]
    pub const fn tx_empty(&self) -> bool {
        (self.bits & IcRawIntrStat::TX_EMPTY_MASK) != 0
    }

    /// Read TX_OVER [3:3]: Set during transmit if the transmit buffer is filled to IC_TX_BUFFER_DEPTH ...
    #[inline(always)]
    pub const fn tx_over(&self) -> bool {
        (self.bits & IcRawIntrStat::TX_OVER_MASK) != 0
    }

    /// Read RX_FULL [2:2]: Set when the receive buffer reaches or goes above the RX_TL threshold in th...
    #[inline(always)]
    pub const fn rx_full(&self) -> bool {
        (self.bits & IcRawIntrStat::RX_FULL_MASK) != 0
    }

    /// Read RX_OVER [1:1]: Set if the receive buffer is completely filled to IC_RX_BUFFER_DEPTH and an...
    #[inline(always)]
    pub const fn rx_over(&self) -> bool {
        (self.bits & IcRawIntrStat::RX_OVER_MASK) != 0
    }

    /// Read RX_UNDER [0:0]: Set if the processor attempts to read the receive buffer when it is empty b...
    #[inline(always)]
    pub const fn rx_under(&self) -> bool {
        (self.bits & IcRawIntrStat::RX_UNDER_MASK) != 0
    }

}

impl Default for IcRawIntrStatVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl IcRawIntrStat {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> IcRawIntrStatVal {
        IcRawIntrStatVal::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: IcRawIntrStatVal) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(IcRawIntrStatVal) -> IcRawIntrStatVal>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// IC_RX_TL Register
///
/// I2C Receive FIFO Threshold Register
///
/// Address offset: 0x0038
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **RX_TL** [7:0] (RW): Receive FIFO Threshold Level. Controls the level of entries (or above) that...
#[repr(transparent)]
pub struct IcRxTl {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for IcRxTl {}
unsafe impl Sync for IcRxTl {}

impl IcRxTl {
    // ======== Field Constants ========

    /// RX_TL - Receive FIFO Threshold Level. Controls the level of entries (or above) that...
    /// Bits [7:0], mask 0x000000ff
    pub const RX_TL_OFFSET: u32 = 0;
    pub const RX_TL_MASK: u32 = 0x000000ff;

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

    /// **RX_TL** [7:0]: Receive FIFO Threshold Level. Controls the level of entries (or above) that...
    #[inline(always)]
    pub fn rx_tl(&self) -> u32 {
        (self.read_bits() & Self::RX_TL_MASK) >> Self::RX_TL_OFFSET
    }

    /// Write RX_TL field
    #[inline(always)]
    pub fn write_rx_tl(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::RX_TL_MASK) | ((value << Self::RX_TL_OFFSET) & Self::RX_TL_MASK));
    }

}

/// IC_RX_TL value type for batch read/modify/write operations
///
/// Use `icrxtl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `icrxtl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IcRxTlVal {
    bits: u32,
}

impl IcRxTlVal {
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

    /// Read RX_TL [7:0]: Receive FIFO Threshold Level. Controls the level of entries (or above) that...
    #[inline(always)]
    pub const fn rx_tl(&self) -> u32 {
        (self.bits & IcRxTl::RX_TL_MASK) >> IcRxTl::RX_TL_OFFSET
    }

    /// Set RX_TL [7:0]: Receive FIFO Threshold Level. Controls the level of entries (or above) that...
    #[inline(always)]
    pub const fn with_rx_tl(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !IcRxTl::RX_TL_MASK) | ((value << IcRxTl::RX_TL_OFFSET) & IcRxTl::RX_TL_MASK)
        }
    }

}

impl Default for IcRxTlVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl IcRxTl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> IcRxTlVal {
        IcRxTlVal::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: IcRxTlVal) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(IcRxTlVal) -> IcRxTlVal>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// IC_TX_TL Register
///
/// I2C Transmit FIFO Threshold Register
///
/// Address offset: 0x003c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **TX_TL** [7:0] (RW): Transmit FIFO Threshold Level. Controls the level of entries (or below) tha...
#[repr(transparent)]
pub struct IcTxTl {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for IcTxTl {}
unsafe impl Sync for IcTxTl {}

impl IcTxTl {
    // ======== Field Constants ========

    /// TX_TL - Transmit FIFO Threshold Level. Controls the level of entries (or below) tha...
    /// Bits [7:0], mask 0x000000ff
    pub const TX_TL_OFFSET: u32 = 0;
    pub const TX_TL_MASK: u32 = 0x000000ff;

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

    /// **TX_TL** [7:0]: Transmit FIFO Threshold Level. Controls the level of entries (or below) tha...
    #[inline(always)]
    pub fn tx_tl(&self) -> u32 {
        (self.read_bits() & Self::TX_TL_MASK) >> Self::TX_TL_OFFSET
    }

    /// Write TX_TL field
    #[inline(always)]
    pub fn write_tx_tl(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::TX_TL_MASK) | ((value << Self::TX_TL_OFFSET) & Self::TX_TL_MASK));
    }

}

/// IC_TX_TL value type for batch read/modify/write operations
///
/// Use `ictxtl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ictxtl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IcTxTlVal {
    bits: u32,
}

impl IcTxTlVal {
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

    /// Read TX_TL [7:0]: Transmit FIFO Threshold Level. Controls the level of entries (or below) tha...
    #[inline(always)]
    pub const fn tx_tl(&self) -> u32 {
        (self.bits & IcTxTl::TX_TL_MASK) >> IcTxTl::TX_TL_OFFSET
    }

    /// Set TX_TL [7:0]: Transmit FIFO Threshold Level. Controls the level of entries (or below) tha...
    #[inline(always)]
    pub const fn with_tx_tl(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !IcTxTl::TX_TL_MASK) | ((value << IcTxTl::TX_TL_OFFSET) & IcTxTl::TX_TL_MASK)
        }
    }

}

impl Default for IcTxTlVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl IcTxTl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> IcTxTlVal {
        IcTxTlVal::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: IcTxTlVal) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(IcTxTlVal) -> IcTxTlVal>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// IC_CLR_INTR Register
///
/// Clear Combined and Individual Interrupt Register
///
/// Address offset: 0x0040
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **CLR_INTR** [0] (RO): Read this register to clear the combined interrupt, all individual interrup...
#[repr(transparent)]
pub struct IcClrIntr {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for IcClrIntr {}
unsafe impl Sync for IcClrIntr {}

impl IcClrIntr {
    // ======== Field Constants ========

    /// CLR_INTR - Read this register to clear the combined interrupt, all individual interrup...
    /// Bits [0:0], mask 0x00000001
    pub const CLR_INTR_OFFSET: u32 = 0;
    pub const CLR_INTR_MASK: u32 = 0x00000001;
    pub const CLR_INTR: u32 = 1 << 0;

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

    /// **CLR_INTR** [0:0]: Read this register to clear the combined interrupt, all individual interrup...
    #[inline(always)]
    pub fn clr_intr(&self) -> bool {
        (self.read_bits() & Self::CLR_INTR_MASK) != 0
    }

}

/// IC_CLR_INTR value type for batch read/modify/write operations
///
/// Use `icclrintr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `icclrintr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IcClrIntrVal {
    bits: u32,
}

impl IcClrIntrVal {
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

    /// Read CLR_INTR [0:0]: Read this register to clear the combined interrupt, all individual interrup...
    #[inline(always)]
    pub const fn clr_intr(&self) -> bool {
        (self.bits & IcClrIntr::CLR_INTR_MASK) != 0
    }

}

impl Default for IcClrIntrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl IcClrIntr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> IcClrIntrVal {
        IcClrIntrVal::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: IcClrIntrVal) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(IcClrIntrVal) -> IcClrIntrVal>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// IC_CLR_RX_UNDER Register
///
/// Clear RX_UNDER Interrupt Register
///
/// Address offset: 0x0044
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **CLR_RX_UNDER** [0] (RO): Read this register to clear the RX_UNDER interrupt (bit 0) of the IC_RAW_IN...
#[repr(transparent)]
pub struct IcClrRxUnder {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for IcClrRxUnder {}
unsafe impl Sync for IcClrRxUnder {}

impl IcClrRxUnder {
    // ======== Field Constants ========

    /// CLR_RX_UNDER - Read this register to clear the RX_UNDER interrupt (bit 0) of the IC_RAW_IN...
    /// Bits [0:0], mask 0x00000001
    pub const CLR_RX_UNDER_OFFSET: u32 = 0;
    pub const CLR_RX_UNDER_MASK: u32 = 0x00000001;
    pub const CLR_RX_UNDER: u32 = 1 << 0;

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

    /// **CLR_RX_UNDER** [0:0]: Read this register to clear the RX_UNDER interrupt (bit 0) of the IC_RAW_IN...
    #[inline(always)]
    pub fn clr_rx_under(&self) -> bool {
        (self.read_bits() & Self::CLR_RX_UNDER_MASK) != 0
    }

}

/// IC_CLR_RX_UNDER value type for batch read/modify/write operations
///
/// Use `icclrrxunder.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `icclrrxunder.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IcClrRxUnderVal {
    bits: u32,
}

impl IcClrRxUnderVal {
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

    /// Read CLR_RX_UNDER [0:0]: Read this register to clear the RX_UNDER interrupt (bit 0) of the IC_RAW_IN...
    #[inline(always)]
    pub const fn clr_rx_under(&self) -> bool {
        (self.bits & IcClrRxUnder::CLR_RX_UNDER_MASK) != 0
    }

}

impl Default for IcClrRxUnderVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl IcClrRxUnder {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> IcClrRxUnderVal {
        IcClrRxUnderVal::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: IcClrRxUnderVal) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(IcClrRxUnderVal) -> IcClrRxUnderVal>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// IC_CLR_RX_OVER Register
///
/// Clear RX_OVER Interrupt Register
///
/// Address offset: 0x0048
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **CLR_RX_OVER** [0] (RO): Read this register to clear the RX_OVER interrupt (bit 1) of the IC_RAW_INT...
#[repr(transparent)]
pub struct IcClrRxOver {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for IcClrRxOver {}
unsafe impl Sync for IcClrRxOver {}

impl IcClrRxOver {
    // ======== Field Constants ========

    /// CLR_RX_OVER - Read this register to clear the RX_OVER interrupt (bit 1) of the IC_RAW_INT...
    /// Bits [0:0], mask 0x00000001
    pub const CLR_RX_OVER_OFFSET: u32 = 0;
    pub const CLR_RX_OVER_MASK: u32 = 0x00000001;
    pub const CLR_RX_OVER: u32 = 1 << 0;

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

    /// **CLR_RX_OVER** [0:0]: Read this register to clear the RX_OVER interrupt (bit 1) of the IC_RAW_INT...
    #[inline(always)]
    pub fn clr_rx_over(&self) -> bool {
        (self.read_bits() & Self::CLR_RX_OVER_MASK) != 0
    }

}

/// IC_CLR_RX_OVER value type for batch read/modify/write operations
///
/// Use `icclrrxover.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `icclrrxover.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IcClrRxOverVal {
    bits: u32,
}

impl IcClrRxOverVal {
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

    /// Read CLR_RX_OVER [0:0]: Read this register to clear the RX_OVER interrupt (bit 1) of the IC_RAW_INT...
    #[inline(always)]
    pub const fn clr_rx_over(&self) -> bool {
        (self.bits & IcClrRxOver::CLR_RX_OVER_MASK) != 0
    }

}

impl Default for IcClrRxOverVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl IcClrRxOver {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> IcClrRxOverVal {
        IcClrRxOverVal::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: IcClrRxOverVal) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(IcClrRxOverVal) -> IcClrRxOverVal>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// IC_CLR_TX_OVER Register
///
/// Clear TX_OVER Interrupt Register
///
/// Address offset: 0x004c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **CLR_TX_OVER** [0] (RO): Read this register to clear the TX_OVER interrupt (bit 3) of the IC_RAW_INT...
#[repr(transparent)]
pub struct IcClrTxOver {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for IcClrTxOver {}
unsafe impl Sync for IcClrTxOver {}

impl IcClrTxOver {
    // ======== Field Constants ========

    /// CLR_TX_OVER - Read this register to clear the TX_OVER interrupt (bit 3) of the IC_RAW_INT...
    /// Bits [0:0], mask 0x00000001
    pub const CLR_TX_OVER_OFFSET: u32 = 0;
    pub const CLR_TX_OVER_MASK: u32 = 0x00000001;
    pub const CLR_TX_OVER: u32 = 1 << 0;

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

    /// **CLR_TX_OVER** [0:0]: Read this register to clear the TX_OVER interrupt (bit 3) of the IC_RAW_INT...
    #[inline(always)]
    pub fn clr_tx_over(&self) -> bool {
        (self.read_bits() & Self::CLR_TX_OVER_MASK) != 0
    }

}

/// IC_CLR_TX_OVER value type for batch read/modify/write operations
///
/// Use `icclrtxover.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `icclrtxover.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IcClrTxOverVal {
    bits: u32,
}

impl IcClrTxOverVal {
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

    /// Read CLR_TX_OVER [0:0]: Read this register to clear the TX_OVER interrupt (bit 3) of the IC_RAW_INT...
    #[inline(always)]
    pub const fn clr_tx_over(&self) -> bool {
        (self.bits & IcClrTxOver::CLR_TX_OVER_MASK) != 0
    }

}

impl Default for IcClrTxOverVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl IcClrTxOver {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> IcClrTxOverVal {
        IcClrTxOverVal::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: IcClrTxOverVal) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(IcClrTxOverVal) -> IcClrTxOverVal>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// IC_CLR_RD_REQ Register
///
/// Clear RD_REQ Interrupt Register
///
/// Address offset: 0x0050
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **CLR_RD_REQ** [0] (RO): Read this register to clear the RD_REQ interrupt (bit 5) of the IC_RAW_INTR...
#[repr(transparent)]
pub struct IcClrRdReq {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for IcClrRdReq {}
unsafe impl Sync for IcClrRdReq {}

impl IcClrRdReq {
    // ======== Field Constants ========

    /// CLR_RD_REQ - Read this register to clear the RD_REQ interrupt (bit 5) of the IC_RAW_INTR...
    /// Bits [0:0], mask 0x00000001
    pub const CLR_RD_REQ_OFFSET: u32 = 0;
    pub const CLR_RD_REQ_MASK: u32 = 0x00000001;
    pub const CLR_RD_REQ: u32 = 1 << 0;

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

    /// **CLR_RD_REQ** [0:0]: Read this register to clear the RD_REQ interrupt (bit 5) of the IC_RAW_INTR...
    #[inline(always)]
    pub fn clr_rd_req(&self) -> bool {
        (self.read_bits() & Self::CLR_RD_REQ_MASK) != 0
    }

}

/// IC_CLR_RD_REQ value type for batch read/modify/write operations
///
/// Use `icclrrdreq.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `icclrrdreq.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IcClrRdReqVal {
    bits: u32,
}

impl IcClrRdReqVal {
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

    /// Read CLR_RD_REQ [0:0]: Read this register to clear the RD_REQ interrupt (bit 5) of the IC_RAW_INTR...
    #[inline(always)]
    pub const fn clr_rd_req(&self) -> bool {
        (self.bits & IcClrRdReq::CLR_RD_REQ_MASK) != 0
    }

}

impl Default for IcClrRdReqVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl IcClrRdReq {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> IcClrRdReqVal {
        IcClrRdReqVal::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: IcClrRdReqVal) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(IcClrRdReqVal) -> IcClrRdReqVal>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// IC_CLR_TX_ABRT Register
///
/// Clear TX_ABRT Interrupt Register
///
/// Address offset: 0x0054
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **CLR_TX_ABRT** [0] (RO): Read this register to clear the TX_ABRT interrupt (bit 6) of the IC_RAW_INT...
#[repr(transparent)]
pub struct IcClrTxAbrt {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for IcClrTxAbrt {}
unsafe impl Sync for IcClrTxAbrt {}

impl IcClrTxAbrt {
    // ======== Field Constants ========

    /// CLR_TX_ABRT - Read this register to clear the TX_ABRT interrupt (bit 6) of the IC_RAW_INT...
    /// Bits [0:0], mask 0x00000001
    pub const CLR_TX_ABRT_OFFSET: u32 = 0;
    pub const CLR_TX_ABRT_MASK: u32 = 0x00000001;
    pub const CLR_TX_ABRT: u32 = 1 << 0;

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

    /// **CLR_TX_ABRT** [0:0]: Read this register to clear the TX_ABRT interrupt (bit 6) of the IC_RAW_INT...
    #[inline(always)]
    pub fn clr_tx_abrt(&self) -> bool {
        (self.read_bits() & Self::CLR_TX_ABRT_MASK) != 0
    }

}

/// IC_CLR_TX_ABRT value type for batch read/modify/write operations
///
/// Use `icclrtxabrt.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `icclrtxabrt.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IcClrTxAbrtVal {
    bits: u32,
}

impl IcClrTxAbrtVal {
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

    /// Read CLR_TX_ABRT [0:0]: Read this register to clear the TX_ABRT interrupt (bit 6) of the IC_RAW_INT...
    #[inline(always)]
    pub const fn clr_tx_abrt(&self) -> bool {
        (self.bits & IcClrTxAbrt::CLR_TX_ABRT_MASK) != 0
    }

}

impl Default for IcClrTxAbrtVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl IcClrTxAbrt {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> IcClrTxAbrtVal {
        IcClrTxAbrtVal::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: IcClrTxAbrtVal) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(IcClrTxAbrtVal) -> IcClrTxAbrtVal>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// IC_CLR_RX_DONE Register
///
/// Clear RX_DONE Interrupt Register
///
/// Address offset: 0x0058
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **CLR_RX_DONE** [0] (RO): Read this register to clear the RX_DONE interrupt (bit 7) of the IC_RAW_INT...
#[repr(transparent)]
pub struct IcClrRxDone {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for IcClrRxDone {}
unsafe impl Sync for IcClrRxDone {}

impl IcClrRxDone {
    // ======== Field Constants ========

    /// CLR_RX_DONE - Read this register to clear the RX_DONE interrupt (bit 7) of the IC_RAW_INT...
    /// Bits [0:0], mask 0x00000001
    pub const CLR_RX_DONE_OFFSET: u32 = 0;
    pub const CLR_RX_DONE_MASK: u32 = 0x00000001;
    pub const CLR_RX_DONE: u32 = 1 << 0;

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

    /// **CLR_RX_DONE** [0:0]: Read this register to clear the RX_DONE interrupt (bit 7) of the IC_RAW_INT...
    #[inline(always)]
    pub fn clr_rx_done(&self) -> bool {
        (self.read_bits() & Self::CLR_RX_DONE_MASK) != 0
    }

}

/// IC_CLR_RX_DONE value type for batch read/modify/write operations
///
/// Use `icclrrxdone.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `icclrrxdone.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IcClrRxDoneVal {
    bits: u32,
}

impl IcClrRxDoneVal {
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

    /// Read CLR_RX_DONE [0:0]: Read this register to clear the RX_DONE interrupt (bit 7) of the IC_RAW_INT...
    #[inline(always)]
    pub const fn clr_rx_done(&self) -> bool {
        (self.bits & IcClrRxDone::CLR_RX_DONE_MASK) != 0
    }

}

impl Default for IcClrRxDoneVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl IcClrRxDone {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> IcClrRxDoneVal {
        IcClrRxDoneVal::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: IcClrRxDoneVal) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(IcClrRxDoneVal) -> IcClrRxDoneVal>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// IC_CLR_ACTIVITY Register
///
/// Clear ACTIVITY Interrupt Register
///
/// Address offset: 0x005c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **CLR_ACTIVITY** [0] (RO): Reading this register clears the ACTIVITY interrupt if the I2C is not activ...
#[repr(transparent)]
pub struct IcClrActivity {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for IcClrActivity {}
unsafe impl Sync for IcClrActivity {}

impl IcClrActivity {
    // ======== Field Constants ========

    /// CLR_ACTIVITY - Reading this register clears the ACTIVITY interrupt if the I2C is not activ...
    /// Bits [0:0], mask 0x00000001
    pub const CLR_ACTIVITY_OFFSET: u32 = 0;
    pub const CLR_ACTIVITY_MASK: u32 = 0x00000001;
    pub const CLR_ACTIVITY: u32 = 1 << 0;

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

    /// **CLR_ACTIVITY** [0:0]: Reading this register clears the ACTIVITY interrupt if the I2C is not activ...
    #[inline(always)]
    pub fn clr_activity(&self) -> bool {
        (self.read_bits() & Self::CLR_ACTIVITY_MASK) != 0
    }

}

/// IC_CLR_ACTIVITY value type for batch read/modify/write operations
///
/// Use `icclractivity.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `icclractivity.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IcClrActivityVal {
    bits: u32,
}

impl IcClrActivityVal {
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

    /// Read CLR_ACTIVITY [0:0]: Reading this register clears the ACTIVITY interrupt if the I2C is not activ...
    #[inline(always)]
    pub const fn clr_activity(&self) -> bool {
        (self.bits & IcClrActivity::CLR_ACTIVITY_MASK) != 0
    }

}

impl Default for IcClrActivityVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl IcClrActivity {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> IcClrActivityVal {
        IcClrActivityVal::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: IcClrActivityVal) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(IcClrActivityVal) -> IcClrActivityVal>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// IC_CLR_STOP_DET Register
///
/// Clear STOP_DET Interrupt Register
///
/// Address offset: 0x0060
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **CLR_STOP_DET** [0] (RO): Read this register to clear the STOP_DET interrupt (bit 9) of the IC_RAW_IN...
#[repr(transparent)]
pub struct IcClrStopDet {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for IcClrStopDet {}
unsafe impl Sync for IcClrStopDet {}

impl IcClrStopDet {
    // ======== Field Constants ========

    /// CLR_STOP_DET - Read this register to clear the STOP_DET interrupt (bit 9) of the IC_RAW_IN...
    /// Bits [0:0], mask 0x00000001
    pub const CLR_STOP_DET_OFFSET: u32 = 0;
    pub const CLR_STOP_DET_MASK: u32 = 0x00000001;
    pub const CLR_STOP_DET: u32 = 1 << 0;

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

    /// **CLR_STOP_DET** [0:0]: Read this register to clear the STOP_DET interrupt (bit 9) of the IC_RAW_IN...
    #[inline(always)]
    pub fn clr_stop_det(&self) -> bool {
        (self.read_bits() & Self::CLR_STOP_DET_MASK) != 0
    }

}

/// IC_CLR_STOP_DET value type for batch read/modify/write operations
///
/// Use `icclrstopdet.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `icclrstopdet.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IcClrStopDetVal {
    bits: u32,
}

impl IcClrStopDetVal {
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

    /// Read CLR_STOP_DET [0:0]: Read this register to clear the STOP_DET interrupt (bit 9) of the IC_RAW_IN...
    #[inline(always)]
    pub const fn clr_stop_det(&self) -> bool {
        (self.bits & IcClrStopDet::CLR_STOP_DET_MASK) != 0
    }

}

impl Default for IcClrStopDetVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl IcClrStopDet {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> IcClrStopDetVal {
        IcClrStopDetVal::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: IcClrStopDetVal) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(IcClrStopDetVal) -> IcClrStopDetVal>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// IC_CLR_START_DET Register
///
/// Clear START_DET Interrupt Register
///
/// Address offset: 0x0064
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **CLR_START_DET** [0] (RO): Read this register to clear the START_DET interrupt (bit 10) of the IC_RAW_...
#[repr(transparent)]
pub struct IcClrStartDet {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for IcClrStartDet {}
unsafe impl Sync for IcClrStartDet {}

impl IcClrStartDet {
    // ======== Field Constants ========

    /// CLR_START_DET - Read this register to clear the START_DET interrupt (bit 10) of the IC_RAW_...
    /// Bits [0:0], mask 0x00000001
    pub const CLR_START_DET_OFFSET: u32 = 0;
    pub const CLR_START_DET_MASK: u32 = 0x00000001;
    pub const CLR_START_DET: u32 = 1 << 0;

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

    /// **CLR_START_DET** [0:0]: Read this register to clear the START_DET interrupt (bit 10) of the IC_RAW_...
    #[inline(always)]
    pub fn clr_start_det(&self) -> bool {
        (self.read_bits() & Self::CLR_START_DET_MASK) != 0
    }

}

/// IC_CLR_START_DET value type for batch read/modify/write operations
///
/// Use `icclrstartdet.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `icclrstartdet.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IcClrStartDetVal {
    bits: u32,
}

impl IcClrStartDetVal {
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

    /// Read CLR_START_DET [0:0]: Read this register to clear the START_DET interrupt (bit 10) of the IC_RAW_...
    #[inline(always)]
    pub const fn clr_start_det(&self) -> bool {
        (self.bits & IcClrStartDet::CLR_START_DET_MASK) != 0
    }

}

impl Default for IcClrStartDetVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl IcClrStartDet {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> IcClrStartDetVal {
        IcClrStartDetVal::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: IcClrStartDetVal) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(IcClrStartDetVal) -> IcClrStartDetVal>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// IC_CLR_GEN_CALL Register
///
/// Clear GEN_CALL Interrupt Register
///
/// Address offset: 0x0068
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **CLR_GEN_CALL** [0] (RO): Read this register to clear the GEN_CALL interrupt (bit 11) of IC_RAW_INTR_...
#[repr(transparent)]
pub struct IcClrGenCall {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for IcClrGenCall {}
unsafe impl Sync for IcClrGenCall {}

impl IcClrGenCall {
    // ======== Field Constants ========

    /// CLR_GEN_CALL - Read this register to clear the GEN_CALL interrupt (bit 11) of IC_RAW_INTR_...
    /// Bits [0:0], mask 0x00000001
    pub const CLR_GEN_CALL_OFFSET: u32 = 0;
    pub const CLR_GEN_CALL_MASK: u32 = 0x00000001;
    pub const CLR_GEN_CALL: u32 = 1 << 0;

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

    /// **CLR_GEN_CALL** [0:0]: Read this register to clear the GEN_CALL interrupt (bit 11) of IC_RAW_INTR_...
    #[inline(always)]
    pub fn clr_gen_call(&self) -> bool {
        (self.read_bits() & Self::CLR_GEN_CALL_MASK) != 0
    }

}

/// IC_CLR_GEN_CALL value type for batch read/modify/write operations
///
/// Use `icclrgencall.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `icclrgencall.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IcClrGenCallVal {
    bits: u32,
}

impl IcClrGenCallVal {
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

    /// Read CLR_GEN_CALL [0:0]: Read this register to clear the GEN_CALL interrupt (bit 11) of IC_RAW_INTR_...
    #[inline(always)]
    pub const fn clr_gen_call(&self) -> bool {
        (self.bits & IcClrGenCall::CLR_GEN_CALL_MASK) != 0
    }

}

impl Default for IcClrGenCallVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl IcClrGenCall {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> IcClrGenCallVal {
        IcClrGenCallVal::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: IcClrGenCallVal) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(IcClrGenCallVal) -> IcClrGenCallVal>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// IC_ENABLE Register
///
/// I2C Enable Register
///
/// Address offset: 0x006c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **TX_CMD_BLOCK** [2] (RW): In Master mode: - 1'b1: Blocks the transmission of data on I2C bus even if ...
/// - **ABORT** [1] (RW): When set, the controller initiates the transfer abort. - 0: ABORT not initi...
/// - **ENABLE** [0] (RW): Controls whether the DW_apb_i2c is enabled. - 0: Disables DW_apb_i2c (TX an...
#[repr(transparent)]
pub struct IcEnable {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for IcEnable {}
unsafe impl Sync for IcEnable {}

impl IcEnable {
    // ======== Field Constants ========

    /// TX_CMD_BLOCK - In Master mode: - 1'b1: Blocks the transmission of data on I2C bus even if ...
    /// Bits [2:2], mask 0x00000004
    pub const TX_CMD_BLOCK_OFFSET: u32 = 2;
    pub const TX_CMD_BLOCK_MASK: u32 = 0x00000004;
    pub const TX_CMD_BLOCK: u32 = 1 << 2;
    /// NOT_BLOCKED = 0: Tx Command execution not blocked
    pub const TX_CMD_BLOCK_NOT_BLOCKED: u32 = 0 << 2;
    /// BLOCKED = 1: Tx Command execution blocked
    pub const TX_CMD_BLOCK_BLOCKED: u32 = 1 << 2;

    /// ABORT - When set, the controller initiates the transfer abort. - 0: ABORT not initi...
    /// Bits [1:1], mask 0x00000002
    pub const ABORT_OFFSET: u32 = 1;
    pub const ABORT_MASK: u32 = 0x00000002;
    pub const ABORT: u32 = 1 << 1;
    /// DISABLE = 0: ABORT operation not in progress
    pub const ABORT_DISABLE: u32 = 0 << 1;
    /// ENABLED = 1: ABORT operation in progress
    pub const ABORT_ENABLED: u32 = 1 << 1;

    /// ENABLE - Controls whether the DW_apb_i2c is enabled. - 0: Disables DW_apb_i2c (TX an...
    /// Bits [0:0], mask 0x00000001
    pub const ENABLE_OFFSET: u32 = 0;
    pub const ENABLE_MASK: u32 = 0x00000001;
    pub const ENABLE: u32 = 1 << 0;
    /// DISABLED = 0: I2C is disabled
    pub const ENABLE_DISABLED: u32 = 0 << 0;
    /// ENABLED = 1: I2C is enabled
    pub const ENABLE_ENABLED: u32 = 1 << 0;

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

    /// **TX_CMD_BLOCK** [2:2]: In Master mode: - 1'b1: Blocks the transmission of data on I2C bus even if ...
    #[inline(always)]
    pub fn tx_cmd_block(&self) -> bool {
        (self.read_bits() & Self::TX_CMD_BLOCK_MASK) != 0
    }

    /// Set TX_CMD_BLOCK bit
    #[inline(always)]
    pub fn set_tx_cmd_block(&self) {
        self.set_bits(Self::TX_CMD_BLOCK_MASK);
    }

    /// Clear TX_CMD_BLOCK bit
    #[inline(always)]
    pub fn clr_tx_cmd_block(&self) {
        self.clr_bits(Self::TX_CMD_BLOCK_MASK);
    }

    /// **ABORT** [1:1]: When set, the controller initiates the transfer abort. - 0: ABORT not initi...
    #[inline(always)]
    pub fn abort(&self) -> bool {
        (self.read_bits() & Self::ABORT_MASK) != 0
    }

    /// Set ABORT bit
    #[inline(always)]
    pub fn set_abort(&self) {
        self.set_bits(Self::ABORT_MASK);
    }

    /// Clear ABORT bit
    #[inline(always)]
    pub fn clr_abort(&self) {
        self.clr_bits(Self::ABORT_MASK);
    }

    /// **ENABLE** [0:0]: Controls whether the DW_apb_i2c is enabled. - 0: Disables DW_apb_i2c (TX an...
    #[inline(always)]
    pub fn enable(&self) -> bool {
        (self.read_bits() & Self::ENABLE_MASK) != 0
    }

    /// Set ENABLE bit
    #[inline(always)]
    pub fn set_enable(&self) {
        self.set_bits(Self::ENABLE_MASK);
    }

    /// Clear ENABLE bit
    #[inline(always)]
    pub fn clr_enable(&self) {
        self.clr_bits(Self::ENABLE_MASK);
    }

}

/// IC_ENABLE value type for batch read/modify/write operations
///
/// Use `icenable.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `icenable.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IcEnableVal {
    bits: u32,
}

impl IcEnableVal {
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

    /// Read TX_CMD_BLOCK [2:2]: In Master mode: - 1'b1: Blocks the transmission of data on I2C bus even if ...
    #[inline(always)]
    pub const fn tx_cmd_block(&self) -> bool {
        (self.bits & IcEnable::TX_CMD_BLOCK_MASK) != 0
    }

    /// Read ABORT [1:1]: When set, the controller initiates the transfer abort. - 0: ABORT not initi...
    #[inline(always)]
    pub const fn abort(&self) -> bool {
        (self.bits & IcEnable::ABORT_MASK) != 0
    }

    /// Read ENABLE [0:0]: Controls whether the DW_apb_i2c is enabled. - 0: Disables DW_apb_i2c (TX an...
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        (self.bits & IcEnable::ENABLE_MASK) != 0
    }

    /// Set TX_CMD_BLOCK [2:2]: In Master mode: - 1'b1: Blocks the transmission of data on I2C bus even if ...
    #[inline(always)]
    pub const fn with_tx_cmd_block(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IcEnable::TX_CMD_BLOCK_MASK
            } else {
                self.bits & !IcEnable::TX_CMD_BLOCK_MASK
            }
        }
    }

    /// Set ABORT [1:1]: When set, the controller initiates the transfer abort. - 0: ABORT not initi...
    #[inline(always)]
    pub const fn with_abort(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IcEnable::ABORT_MASK
            } else {
                self.bits & !IcEnable::ABORT_MASK
            }
        }
    }

    /// Set ENABLE [0:0]: Controls whether the DW_apb_i2c is enabled. - 0: Disables DW_apb_i2c (TX an...
    #[inline(always)]
    pub const fn with_enable(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IcEnable::ENABLE_MASK
            } else {
                self.bits & !IcEnable::ENABLE_MASK
            }
        }
    }

}

impl Default for IcEnableVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl IcEnable {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> IcEnableVal {
        IcEnableVal::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: IcEnableVal) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(IcEnableVal) -> IcEnableVal>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// IC_STATUS Register
///
/// I2C Status Register This is a read-only register used to indicate the
/// current transfer status and FIFO status. The status register may be read at
/// any time. None of the bits in this register request an interrupt. When the
/// I2C is disabled by writing 0 in bit 0 of the IC_ENABLE register: - Bits 1
/// and 2 are set to 1 - Bits 3 and 10 are set to 0 When the master or slave
/// state machines goes to idle and ic_en=0: - Bits 5 and 6 are set to 0
///
/// Address offset: 0x0070
/// Reset value: 0x00000006
///
/// ## Fields
///
/// - **SLV_ACTIVITY** [6] (RO): Slave FSM Activity Status. When the Slave Finite State Machine (FSM) is not...
/// - **MST_ACTIVITY** [5] (RO): Master FSM Activity Status. When the Master Finite State Machine (FSM) is n...
/// - **RFF** [4] (RO): Receive FIFO Completely Full. When the receive FIFO is completely full, thi...
/// - **RFNE** [3] (RO): Receive FIFO Not Empty. This bit is set when the receive FIFO contains one ...
/// - **TFE** [2] (RO): Transmit FIFO Completely Empty. When the transmit FIFO is completely empty,...
/// - **TFNF** [1] (RO): Transmit FIFO Not Full. Set when the transmit FIFO contains one or more emp...
/// - **ACTIVITY** [0] (RO): I2C Activity Status. Reset value: 0x0
#[repr(transparent)]
pub struct IcStatus {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for IcStatus {}
unsafe impl Sync for IcStatus {}

impl IcStatus {
    // ======== Field Constants ========

    /// SLV_ACTIVITY - Slave FSM Activity Status. When the Slave Finite State Machine (FSM) is not...
    /// Bits [6:6], mask 0x00000040
    pub const SLV_ACTIVITY_OFFSET: u32 = 6;
    pub const SLV_ACTIVITY_MASK: u32 = 0x00000040;
    pub const SLV_ACTIVITY: u32 = 1 << 6;
    /// IDLE = 0: Slave is idle
    pub const SLV_ACTIVITY_IDLE: u32 = 0 << 6;
    /// ACTIVE = 1: Slave not idle
    pub const SLV_ACTIVITY_ACTIVE: u32 = 1 << 6;

    /// MST_ACTIVITY - Master FSM Activity Status. When the Master Finite State Machine (FSM) is n...
    /// Bits [5:5], mask 0x00000020
    pub const MST_ACTIVITY_OFFSET: u32 = 5;
    pub const MST_ACTIVITY_MASK: u32 = 0x00000020;
    pub const MST_ACTIVITY: u32 = 1 << 5;
    /// IDLE = 0: Master is idle
    pub const MST_ACTIVITY_IDLE: u32 = 0 << 5;
    /// ACTIVE = 1: Master not idle
    pub const MST_ACTIVITY_ACTIVE: u32 = 1 << 5;

    /// RFF - Receive FIFO Completely Full. When the receive FIFO is completely full, thi...
    /// Bits [4:4], mask 0x00000010
    pub const RFF_OFFSET: u32 = 4;
    pub const RFF_MASK: u32 = 0x00000010;
    pub const RFF: u32 = 1 << 4;
    /// NOT_FULL = 0: Rx FIFO not full
    pub const RFF_NOT_FULL: u32 = 0 << 4;
    /// FULL = 1: Rx FIFO is full
    pub const RFF_FULL: u32 = 1 << 4;

    /// RFNE - Receive FIFO Not Empty. This bit is set when the receive FIFO contains one ...
    /// Bits [3:3], mask 0x00000008
    pub const RFNE_OFFSET: u32 = 3;
    pub const RFNE_MASK: u32 = 0x00000008;
    pub const RFNE: u32 = 1 << 3;
    /// EMPTY = 0: Rx FIFO is empty
    pub const RFNE_EMPTY: u32 = 0 << 3;
    /// NOT_EMPTY = 1: Rx FIFO not empty
    pub const RFNE_NOT_EMPTY: u32 = 1 << 3;

    /// TFE - Transmit FIFO Completely Empty. When the transmit FIFO is completely empty,...
    /// Bits [2:2], mask 0x00000004
    pub const TFE_OFFSET: u32 = 2;
    pub const TFE_MASK: u32 = 0x00000004;
    pub const TFE: u32 = 1 << 2;
    /// NON_EMPTY = 0: Tx FIFO not empty
    pub const TFE_NON_EMPTY: u32 = 0 << 2;
    /// EMPTY = 1: Tx FIFO is empty
    pub const TFE_EMPTY: u32 = 1 << 2;

    /// TFNF - Transmit FIFO Not Full. Set when the transmit FIFO contains one or more emp...
    /// Bits [1:1], mask 0x00000002
    pub const TFNF_OFFSET: u32 = 1;
    pub const TFNF_MASK: u32 = 0x00000002;
    pub const TFNF: u32 = 1 << 1;
    /// FULL = 0: Tx FIFO is full
    pub const TFNF_FULL: u32 = 0 << 1;
    /// NOT_FULL = 1: Tx FIFO not full
    pub const TFNF_NOT_FULL: u32 = 1 << 1;

    /// ACTIVITY - I2C Activity Status. Reset value: 0x0
    /// Bits [0:0], mask 0x00000001
    pub const ACTIVITY_OFFSET: u32 = 0;
    pub const ACTIVITY_MASK: u32 = 0x00000001;
    pub const ACTIVITY: u32 = 1 << 0;
    /// INACTIVE = 0: I2C is idle
    pub const ACTIVITY_INACTIVE: u32 = 0 << 0;
    /// ACTIVE = 1: I2C is active
    pub const ACTIVITY_ACTIVE: u32 = 1 << 0;

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

    /// **SLV_ACTIVITY** [6:6]: Slave FSM Activity Status. When the Slave Finite State Machine (FSM) is not...
    #[inline(always)]
    pub fn slv_activity(&self) -> bool {
        (self.read_bits() & Self::SLV_ACTIVITY_MASK) != 0
    }

    /// **MST_ACTIVITY** [5:5]: Master FSM Activity Status. When the Master Finite State Machine (FSM) is n...
    #[inline(always)]
    pub fn mst_activity(&self) -> bool {
        (self.read_bits() & Self::MST_ACTIVITY_MASK) != 0
    }

    /// **RFF** [4:4]: Receive FIFO Completely Full. When the receive FIFO is completely full, thi...
    #[inline(always)]
    pub fn rff(&self) -> bool {
        (self.read_bits() & Self::RFF_MASK) != 0
    }

    /// **RFNE** [3:3]: Receive FIFO Not Empty. This bit is set when the receive FIFO contains one ...
    #[inline(always)]
    pub fn rfne(&self) -> bool {
        (self.read_bits() & Self::RFNE_MASK) != 0
    }

    /// **TFE** [2:2]: Transmit FIFO Completely Empty. When the transmit FIFO is completely empty,...
    #[inline(always)]
    pub fn tfe(&self) -> bool {
        (self.read_bits() & Self::TFE_MASK) != 0
    }

    /// **TFNF** [1:1]: Transmit FIFO Not Full. Set when the transmit FIFO contains one or more emp...
    #[inline(always)]
    pub fn tfnf(&self) -> bool {
        (self.read_bits() & Self::TFNF_MASK) != 0
    }

    /// **ACTIVITY** [0:0]: I2C Activity Status. Reset value: 0x0
    #[inline(always)]
    pub fn activity(&self) -> bool {
        (self.read_bits() & Self::ACTIVITY_MASK) != 0
    }

}

/// IC_STATUS value type for batch read/modify/write operations
///
/// Use `icstatus.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `icstatus.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IcStatusVal {
    bits: u32,
}

impl IcStatusVal {
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
        Self { bits: 0x00000006 }
    }

    /// Read SLV_ACTIVITY [6:6]: Slave FSM Activity Status. When the Slave Finite State Machine (FSM) is not...
    #[inline(always)]
    pub const fn slv_activity(&self) -> bool {
        (self.bits & IcStatus::SLV_ACTIVITY_MASK) != 0
    }

    /// Read MST_ACTIVITY [5:5]: Master FSM Activity Status. When the Master Finite State Machine (FSM) is n...
    #[inline(always)]
    pub const fn mst_activity(&self) -> bool {
        (self.bits & IcStatus::MST_ACTIVITY_MASK) != 0
    }

    /// Read RFF [4:4]: Receive FIFO Completely Full. When the receive FIFO is completely full, thi...
    #[inline(always)]
    pub const fn rff(&self) -> bool {
        (self.bits & IcStatus::RFF_MASK) != 0
    }

    /// Read RFNE [3:3]: Receive FIFO Not Empty. This bit is set when the receive FIFO contains one ...
    #[inline(always)]
    pub const fn rfne(&self) -> bool {
        (self.bits & IcStatus::RFNE_MASK) != 0
    }

    /// Read TFE [2:2]: Transmit FIFO Completely Empty. When the transmit FIFO is completely empty,...
    #[inline(always)]
    pub const fn tfe(&self) -> bool {
        (self.bits & IcStatus::TFE_MASK) != 0
    }

    /// Read TFNF [1:1]: Transmit FIFO Not Full. Set when the transmit FIFO contains one or more emp...
    #[inline(always)]
    pub const fn tfnf(&self) -> bool {
        (self.bits & IcStatus::TFNF_MASK) != 0
    }

    /// Read ACTIVITY [0:0]: I2C Activity Status. Reset value: 0x0
    #[inline(always)]
    pub const fn activity(&self) -> bool {
        (self.bits & IcStatus::ACTIVITY_MASK) != 0
    }

}

impl Default for IcStatusVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl IcStatus {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> IcStatusVal {
        IcStatusVal::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: IcStatusVal) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(IcStatusVal) -> IcStatusVal>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// IC_TXFLR Register
///
/// I2C Transmit FIFO Level Register This register contains the number of valid
/// data entries in the transmit FIFO buffer. It is cleared whenever: - The I2C
/// is disabled - There is a transmit abort - that is, TX_ABRT bit is set in the
/// IC_RAW_INTR_STAT register - The slave bulk transmit mode is aborted The
/// register increments whenever data is placed into the transmit FIFO and
/// decrements when data is taken from the transmit FIFO.
///
/// Address offset: 0x0074
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **TXFLR** [4:0] (RO): Transmit FIFO Level. Contains the number of valid data entries in the trans...
#[repr(transparent)]
pub struct IcTxflr {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for IcTxflr {}
unsafe impl Sync for IcTxflr {}

impl IcTxflr {
    // ======== Field Constants ========

    /// TXFLR - Transmit FIFO Level. Contains the number of valid data entries in the trans...
    /// Bits [4:0], mask 0x0000001f
    pub const TXFLR_OFFSET: u32 = 0;
    pub const TXFLR_MASK: u32 = 0x0000001f;

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

    /// **TXFLR** [4:0]: Transmit FIFO Level. Contains the number of valid data entries in the trans...
    #[inline(always)]
    pub fn txflr(&self) -> u32 {
        (self.read_bits() & Self::TXFLR_MASK) >> Self::TXFLR_OFFSET
    }

}

/// IC_TXFLR value type for batch read/modify/write operations
///
/// Use `ictxflr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ictxflr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IcTxflrVal {
    bits: u32,
}

impl IcTxflrVal {
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

    /// Read TXFLR [4:0]: Transmit FIFO Level. Contains the number of valid data entries in the trans...
    #[inline(always)]
    pub const fn txflr(&self) -> u32 {
        (self.bits & IcTxflr::TXFLR_MASK) >> IcTxflr::TXFLR_OFFSET
    }

}

impl Default for IcTxflrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl IcTxflr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> IcTxflrVal {
        IcTxflrVal::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: IcTxflrVal) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(IcTxflrVal) -> IcTxflrVal>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// IC_RXFLR Register
///
/// I2C Receive FIFO Level Register This register contains the number of valid
/// data entries in the receive FIFO buffer. It is cleared whenever: - The I2C
/// is disabled - Whenever there is a transmit abort caused by any of the events
/// tracked in IC_TX_ABRT_SOURCE The register increments whenever data is placed
/// into the receive FIFO and decrements when data is taken from the receive
/// FIFO.
///
/// Address offset: 0x0078
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **RXFLR** [4:0] (RO): Receive FIFO Level. Contains the number of valid data entries in the receiv...
#[repr(transparent)]
pub struct IcRxflr {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for IcRxflr {}
unsafe impl Sync for IcRxflr {}

impl IcRxflr {
    // ======== Field Constants ========

    /// RXFLR - Receive FIFO Level. Contains the number of valid data entries in the receiv...
    /// Bits [4:0], mask 0x0000001f
    pub const RXFLR_OFFSET: u32 = 0;
    pub const RXFLR_MASK: u32 = 0x0000001f;

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

    /// **RXFLR** [4:0]: Receive FIFO Level. Contains the number of valid data entries in the receiv...
    #[inline(always)]
    pub fn rxflr(&self) -> u32 {
        (self.read_bits() & Self::RXFLR_MASK) >> Self::RXFLR_OFFSET
    }

}

/// IC_RXFLR value type for batch read/modify/write operations
///
/// Use `icrxflr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `icrxflr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IcRxflrVal {
    bits: u32,
}

impl IcRxflrVal {
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

    /// Read RXFLR [4:0]: Receive FIFO Level. Contains the number of valid data entries in the receiv...
    #[inline(always)]
    pub const fn rxflr(&self) -> u32 {
        (self.bits & IcRxflr::RXFLR_MASK) >> IcRxflr::RXFLR_OFFSET
    }

}

impl Default for IcRxflrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl IcRxflr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> IcRxflrVal {
        IcRxflrVal::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: IcRxflrVal) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(IcRxflrVal) -> IcRxflrVal>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// IC_SDA_HOLD Register
///
/// I2C SDA Hold Time Length Register The bits [15:0] of this register are used
/// to control the hold time of SDA during transmit in both slave and master
/// mode (after SCL goes from HIGH to LOW). The bits [23:16] of this register
/// are used to extend the SDA transition (if any) whenever SCL is HIGH in the
/// receiver in either master or slave mode. Writes to this register succeed
/// only when IC_ENABLE[0]=0. The values in this register are in units of ic_clk
/// period. The value programmed in IC_SDA_TX_HOLD must be greater than the
/// minimum hold time in each mode (one cycle in master mode, seven cycles in
/// slave mode) for the value to be implemented. The programmed SDA hold time
/// during transmit (IC_SDA_TX_HOLD) cannot exceed at any time the duration of
/// the low part of scl. Therefore the programmed value cannot be larger than
/// N_SCL_LOW-2, where N_SCL_LOW is the duration of the low part of the scl
/// period measured in ic_clk cycles.
///
/// Address offset: 0x007c
/// Reset value: 0x00000001
///
/// ## Fields
///
/// - **IC_SDA_RX_HOLD** [23:16] (RW): Sets the required SDA hold time in units of ic_clk period, when DW_apb_i2c ...
/// - **IC_SDA_TX_HOLD** [15:0] (RW): Sets the required SDA hold time in units of ic_clk period, when DW_apb_i2c ...
#[repr(transparent)]
pub struct IcSdaHold {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for IcSdaHold {}
unsafe impl Sync for IcSdaHold {}

impl IcSdaHold {
    // ======== Field Constants ========

    /// IC_SDA_RX_HOLD - Sets the required SDA hold time in units of ic_clk period, when DW_apb_i2c ...
    /// Bits [23:16], mask 0x00ff0000
    pub const IC_SDA_RX_HOLD_OFFSET: u32 = 16;
    pub const IC_SDA_RX_HOLD_MASK: u32 = 0x00ff0000;

    /// IC_SDA_TX_HOLD - Sets the required SDA hold time in units of ic_clk period, when DW_apb_i2c ...
    /// Bits [15:0], mask 0x0000ffff
    pub const IC_SDA_TX_HOLD_OFFSET: u32 = 0;
    pub const IC_SDA_TX_HOLD_MASK: u32 = 0x0000ffff;

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

    /// **IC_SDA_RX_HOLD** [23:16]: Sets the required SDA hold time in units of ic_clk period, when DW_apb_i2c ...
    #[inline(always)]
    pub fn ic_sda_rx_hold(&self) -> u32 {
        (self.read_bits() & Self::IC_SDA_RX_HOLD_MASK) >> Self::IC_SDA_RX_HOLD_OFFSET
    }

    /// Write IC_SDA_RX_HOLD field
    #[inline(always)]
    pub fn write_ic_sda_rx_hold(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::IC_SDA_RX_HOLD_MASK) | ((value << Self::IC_SDA_RX_HOLD_OFFSET) & Self::IC_SDA_RX_HOLD_MASK));
    }

    /// **IC_SDA_TX_HOLD** [15:0]: Sets the required SDA hold time in units of ic_clk period, when DW_apb_i2c ...
    #[inline(always)]
    pub fn ic_sda_tx_hold(&self) -> u32 {
        (self.read_bits() & Self::IC_SDA_TX_HOLD_MASK) >> Self::IC_SDA_TX_HOLD_OFFSET
    }

    /// Write IC_SDA_TX_HOLD field
    #[inline(always)]
    pub fn write_ic_sda_tx_hold(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::IC_SDA_TX_HOLD_MASK) | ((value << Self::IC_SDA_TX_HOLD_OFFSET) & Self::IC_SDA_TX_HOLD_MASK));
    }

}

/// IC_SDA_HOLD value type for batch read/modify/write operations
///
/// Use `icsdahold.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `icsdahold.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IcSdaHoldVal {
    bits: u32,
}

impl IcSdaHoldVal {
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

    /// Read IC_SDA_RX_HOLD [23:16]: Sets the required SDA hold time in units of ic_clk period, when DW_apb_i2c ...
    #[inline(always)]
    pub const fn ic_sda_rx_hold(&self) -> u32 {
        (self.bits & IcSdaHold::IC_SDA_RX_HOLD_MASK) >> IcSdaHold::IC_SDA_RX_HOLD_OFFSET
    }

    /// Read IC_SDA_TX_HOLD [15:0]: Sets the required SDA hold time in units of ic_clk period, when DW_apb_i2c ...
    #[inline(always)]
    pub const fn ic_sda_tx_hold(&self) -> u32 {
        (self.bits & IcSdaHold::IC_SDA_TX_HOLD_MASK) >> IcSdaHold::IC_SDA_TX_HOLD_OFFSET
    }

    /// Set IC_SDA_RX_HOLD [23:16]: Sets the required SDA hold time in units of ic_clk period, when DW_apb_i2c ...
    #[inline(always)]
    pub const fn with_ic_sda_rx_hold(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !IcSdaHold::IC_SDA_RX_HOLD_MASK) | ((value << IcSdaHold::IC_SDA_RX_HOLD_OFFSET) & IcSdaHold::IC_SDA_RX_HOLD_MASK)
        }
    }

    /// Set IC_SDA_TX_HOLD [15:0]: Sets the required SDA hold time in units of ic_clk period, when DW_apb_i2c ...
    #[inline(always)]
    pub const fn with_ic_sda_tx_hold(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !IcSdaHold::IC_SDA_TX_HOLD_MASK) | ((value << IcSdaHold::IC_SDA_TX_HOLD_OFFSET) & IcSdaHold::IC_SDA_TX_HOLD_MASK)
        }
    }

}

impl Default for IcSdaHoldVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl IcSdaHold {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> IcSdaHoldVal {
        IcSdaHoldVal::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: IcSdaHoldVal) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(IcSdaHoldVal) -> IcSdaHoldVal>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// IC_TX_ABRT_SOURCE Register
///
/// I2C Transmit Abort Source Register This register has 32 bits that indicate
/// the source of the TX_ABRT bit. Except for Bit 9, this register is cleared
/// whenever the IC_CLR_TX_ABRT register or the IC_CLR_INTR register is read. To
/// clear Bit 9, the source of the ABRT_SBYTE_NORSTRT must be fixed first;
/// RESTART must be enabled (IC_CON[5]=1), the SPECIAL bit must be cleared
/// (IC_TAR[11]), or the GC_OR_START bit must be cleared (IC_TAR[10]). Once the
/// source of the ABRT_SBYTE_NORSTRT is fixed, then this bit can be cleared in
/// the same manner as other bits in this register. If the source of the
/// ABRT_SBYTE_NORSTRT is not fixed before attempting to clear this bit, Bit 9
/// clears for one cycle and is then re-asserted.
///
/// Address offset: 0x0080
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **TX_FLUSH_CNT** [31:23] (RO): This field indicates the number of Tx FIFO Data Commands which are flushed ...
/// - **ABRT_USER_ABRT** [16] (RO): This is a master-mode-only bit. Master has detected the transfer abort (IC_...
/// - **ABRT_SLVRD_INTX** [15] (RO): 1: When the processor side responds to a slave mode request for data to be ...
/// - **ABRT_SLV_ARBLOST** [14] (RO): This field indicates that a Slave has lost the bus while transmitting data ...
/// - **ABRT_SLVFLUSH_TXFIFO** [13] (RO): This field specifies that the Slave has received a read command and some da...
/// - **ARB_LOST** [12] (RO): This field specifies that the Master has lost arbitration, or if IC_TX_ABRT...
/// - **ABRT_MASTER_DIS** [11] (RO): This field indicates that the User tries to initiate a Master operation wit...
/// - **ABRT_10B_RD_NORSTRT** [10] (RO): This field indicates that the restart is disabled (IC_RESTART_EN bit (IC_CO...
/// - **ABRT_SBYTE_NORSTRT** [9] (RO): To clear Bit 9, the source of the ABRT_SBYTE_NORSTRT must be fixed first; r...
/// - **ABRT_HS_NORSTRT** [8] (RO): This field indicates that the restart is disabled (IC_RESTART_EN bit (IC_CO...
/// - **ABRT_SBYTE_ACKDET** [7] (RO): This field indicates that the Master has sent a START Byte and the START By...
/// - **ABRT_HS_ACKDET** [6] (RO): This field indicates that the Master is in High Speed mode and the High Spe...
/// - **ABRT_GCALL_READ** [5] (RO): This field indicates that DW_apb_i2c in the master mode has sent a General ...
/// - **ABRT_GCALL_NOACK** [4] (RO): This field indicates that DW_apb_i2c in master mode has sent a General Call...
/// - **ABRT_TXDATA_NOACK** [3] (RO): This field indicates the master-mode only bit. When the master receives an ...
/// - **ABRT_10ADDR2_NOACK** [2] (RO): This field indicates that the Master is in 10-bit address mode and that the...
/// - **ABRT_10ADDR1_NOACK** [1] (RO): This field indicates that the Master is in 10-bit address mode and the firs...
/// - **ABRT_7B_ADDR_NOACK** [0] (RO): This field indicates that the Master is in 7-bit addressing mode and the ad...
#[repr(transparent)]
pub struct IcTxAbrtSource {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for IcTxAbrtSource {}
unsafe impl Sync for IcTxAbrtSource {}

impl IcTxAbrtSource {
    // ======== Field Constants ========

    /// TX_FLUSH_CNT - This field indicates the number of Tx FIFO Data Commands which are flushed ...
    /// Bits [31:23], mask 0xff800000
    pub const TX_FLUSH_CNT_OFFSET: u32 = 23;
    pub const TX_FLUSH_CNT_MASK: u32 = 0xff800000;

    /// ABRT_USER_ABRT - This is a master-mode-only bit. Master has detected the transfer abort (IC_...
    /// Bits [16:16], mask 0x00010000
    pub const ABRT_USER_ABRT_OFFSET: u32 = 16;
    pub const ABRT_USER_ABRT_MASK: u32 = 0x00010000;
    pub const ABRT_USER_ABRT: u32 = 1 << 16;
    /// ABRT_USER_ABRT_VOID = 0: Transfer abort detected by master- scenario not present
    pub const ABRT_USER_ABRT_ABRT_USER_ABRT_VOID: u32 = 0 << 16;
    /// ABRT_USER_ABRT_GENERATED = 1: Transfer abort detected by master
    pub const ABRT_USER_ABRT_ABRT_USER_ABRT_GENERATED: u32 = 1 << 16;

    /// ABRT_SLVRD_INTX - 1: When the processor side responds to a slave mode request for data to be ...
    /// Bits [15:15], mask 0x00008000
    pub const ABRT_SLVRD_INTX_OFFSET: u32 = 15;
    pub const ABRT_SLVRD_INTX_MASK: u32 = 0x00008000;
    pub const ABRT_SLVRD_INTX: u32 = 1 << 15;
    /// ABRT_SLVRD_INTX_VOID = 0: Slave trying to transmit to remote master in read mode- scenario not present
    pub const ABRT_SLVRD_INTX_ABRT_SLVRD_INTX_VOID: u32 = 0 << 15;
    /// ABRT_SLVRD_INTX_GENERATED = 1: Slave trying to transmit to remote master in read mode
    pub const ABRT_SLVRD_INTX_ABRT_SLVRD_INTX_GENERATED: u32 = 1 << 15;

    /// ABRT_SLV_ARBLOST - This field indicates that a Slave has lost the bus while transmitting data ...
    /// Bits [14:14], mask 0x00004000
    pub const ABRT_SLV_ARBLOST_OFFSET: u32 = 14;
    pub const ABRT_SLV_ARBLOST_MASK: u32 = 0x00004000;
    pub const ABRT_SLV_ARBLOST: u32 = 1 << 14;
    /// ABRT_SLV_ARBLOST_VOID = 0: Slave lost arbitration to remote master- scenario not present
    pub const ABRT_SLV_ARBLOST_ABRT_SLV_ARBLOST_VOID: u32 = 0 << 14;
    /// ABRT_SLV_ARBLOST_GENERATED = 1: Slave lost arbitration to remote master
    pub const ABRT_SLV_ARBLOST_ABRT_SLV_ARBLOST_GENERATED: u32 = 1 << 14;

    /// ABRT_SLVFLUSH_TXFIFO - This field specifies that the Slave has received a read command and some da...
    /// Bits [13:13], mask 0x00002000
    pub const ABRT_SLVFLUSH_TXFIFO_OFFSET: u32 = 13;
    pub const ABRT_SLVFLUSH_TXFIFO_MASK: u32 = 0x00002000;
    pub const ABRT_SLVFLUSH_TXFIFO: u32 = 1 << 13;
    /// ABRT_SLVFLUSH_TXFIFO_VOID = 0: Slave flushes existing data in TX-FIFO upon getting read command- scenario ...
    pub const ABRT_SLVFLUSH_TXFIFO_ABRT_SLVFLUSH_TXFIFO_VOID: u32 = 0 << 13;
    /// ABRT_SLVFLUSH_TXFIFO_GENERATED = 1: Slave flushes existing data in TX-FIFO upon getting read command
    pub const ABRT_SLVFLUSH_TXFIFO_ABRT_SLVFLUSH_TXFIFO_GENERATED: u32 = 1 << 13;

    /// ARB_LOST - This field specifies that the Master has lost arbitration, or if IC_TX_ABRT...
    /// Bits [12:12], mask 0x00001000
    pub const ARB_LOST_OFFSET: u32 = 12;
    pub const ARB_LOST_MASK: u32 = 0x00001000;
    pub const ARB_LOST: u32 = 1 << 12;
    /// ABRT_LOST_VOID = 0: Master or Slave-Transmitter lost arbitration- scenario not present
    pub const ARB_LOST_ABRT_LOST_VOID: u32 = 0 << 12;
    /// ABRT_LOST_GENERATED = 1: Master or Slave-Transmitter lost arbitration
    pub const ARB_LOST_ABRT_LOST_GENERATED: u32 = 1 << 12;

    /// ABRT_MASTER_DIS - This field indicates that the User tries to initiate a Master operation wit...
    /// Bits [11:11], mask 0x00000800
    pub const ABRT_MASTER_DIS_OFFSET: u32 = 11;
    pub const ABRT_MASTER_DIS_MASK: u32 = 0x00000800;
    pub const ABRT_MASTER_DIS: u32 = 1 << 11;
    /// ABRT_MASTER_DIS_VOID = 0: User initiating master operation when MASTER disabled- scenario not present
    pub const ABRT_MASTER_DIS_ABRT_MASTER_DIS_VOID: u32 = 0 << 11;
    /// ABRT_MASTER_DIS_GENERATED = 1: User initiating master operation when MASTER disabled
    pub const ABRT_MASTER_DIS_ABRT_MASTER_DIS_GENERATED: u32 = 1 << 11;

    /// ABRT_10B_RD_NORSTRT - This field indicates that the restart is disabled (IC_RESTART_EN bit (IC_CO...
    /// Bits [10:10], mask 0x00000400
    pub const ABRT_10B_RD_NORSTRT_OFFSET: u32 = 10;
    pub const ABRT_10B_RD_NORSTRT_MASK: u32 = 0x00000400;
    pub const ABRT_10B_RD_NORSTRT: u32 = 1 << 10;
    /// ABRT_10B_RD_VOID = 0: Master not trying to read in 10Bit addressing mode when RESTART disabled
    pub const ABRT_10B_RD_NORSTRT_ABRT_10B_RD_VOID: u32 = 0 << 10;
    /// ABRT_10B_RD_GENERATED = 1: Master trying to read in 10Bit addressing mode when RESTART disabled
    pub const ABRT_10B_RD_NORSTRT_ABRT_10B_RD_GENERATED: u32 = 1 << 10;

    /// ABRT_SBYTE_NORSTRT - To clear Bit 9, the source of the ABRT_SBYTE_NORSTRT must be fixed first; r...
    /// Bits [9:9], mask 0x00000200
    pub const ABRT_SBYTE_NORSTRT_OFFSET: u32 = 9;
    pub const ABRT_SBYTE_NORSTRT_MASK: u32 = 0x00000200;
    pub const ABRT_SBYTE_NORSTRT: u32 = 1 << 9;
    /// ABRT_SBYTE_NORSTRT_VOID = 0: User trying to send START byte when RESTART disabled- scenario not present
    pub const ABRT_SBYTE_NORSTRT_ABRT_SBYTE_NORSTRT_VOID: u32 = 0 << 9;
    /// ABRT_SBYTE_NORSTRT_GENERATED = 1: User trying to send START byte when RESTART disabled
    pub const ABRT_SBYTE_NORSTRT_ABRT_SBYTE_NORSTRT_GENERATED: u32 = 1 << 9;

    /// ABRT_HS_NORSTRT - This field indicates that the restart is disabled (IC_RESTART_EN bit (IC_CO...
    /// Bits [8:8], mask 0x00000100
    pub const ABRT_HS_NORSTRT_OFFSET: u32 = 8;
    pub const ABRT_HS_NORSTRT_MASK: u32 = 0x00000100;
    pub const ABRT_HS_NORSTRT: u32 = 1 << 8;
    /// ABRT_HS_NORSTRT_VOID = 0: User trying to switch Master to HS mode when RESTART disabled- scenario not...
    pub const ABRT_HS_NORSTRT_ABRT_HS_NORSTRT_VOID: u32 = 0 << 8;
    /// ABRT_HS_NORSTRT_GENERATED = 1: User trying to switch Master to HS mode when RESTART disabled
    pub const ABRT_HS_NORSTRT_ABRT_HS_NORSTRT_GENERATED: u32 = 1 << 8;

    /// ABRT_SBYTE_ACKDET - This field indicates that the Master has sent a START Byte and the START By...
    /// Bits [7:7], mask 0x00000080
    pub const ABRT_SBYTE_ACKDET_OFFSET: u32 = 7;
    pub const ABRT_SBYTE_ACKDET_MASK: u32 = 0x00000080;
    pub const ABRT_SBYTE_ACKDET: u32 = 1 << 7;
    /// ABRT_SBYTE_ACKDET_VOID = 0: ACK detected for START byte- scenario not present
    pub const ABRT_SBYTE_ACKDET_ABRT_SBYTE_ACKDET_VOID: u32 = 0 << 7;
    /// ABRT_SBYTE_ACKDET_GENERATED = 1: ACK detected for START byte
    pub const ABRT_SBYTE_ACKDET_ABRT_SBYTE_ACKDET_GENERATED: u32 = 1 << 7;

    /// ABRT_HS_ACKDET - This field indicates that the Master is in High Speed mode and the High Spe...
    /// Bits [6:6], mask 0x00000040
    pub const ABRT_HS_ACKDET_OFFSET: u32 = 6;
    pub const ABRT_HS_ACKDET_MASK: u32 = 0x00000040;
    pub const ABRT_HS_ACKDET: u32 = 1 << 6;
    /// ABRT_HS_ACK_VOID = 0: HS Master code ACKed in HS Mode- scenario not present
    pub const ABRT_HS_ACKDET_ABRT_HS_ACK_VOID: u32 = 0 << 6;
    /// ABRT_HS_ACK_GENERATED = 1: HS Master code ACKed in HS Mode
    pub const ABRT_HS_ACKDET_ABRT_HS_ACK_GENERATED: u32 = 1 << 6;

    /// ABRT_GCALL_READ - This field indicates that DW_apb_i2c in the master mode has sent a General ...
    /// Bits [5:5], mask 0x00000020
    pub const ABRT_GCALL_READ_OFFSET: u32 = 5;
    pub const ABRT_GCALL_READ_MASK: u32 = 0x00000020;
    pub const ABRT_GCALL_READ: u32 = 1 << 5;
    /// ABRT_GCALL_READ_VOID = 0: GCALL is followed by read from bus-scenario not present
    pub const ABRT_GCALL_READ_ABRT_GCALL_READ_VOID: u32 = 0 << 5;
    /// ABRT_GCALL_READ_GENERATED = 1: GCALL is followed by read from bus
    pub const ABRT_GCALL_READ_ABRT_GCALL_READ_GENERATED: u32 = 1 << 5;

    /// ABRT_GCALL_NOACK - This field indicates that DW_apb_i2c in master mode has sent a General Call...
    /// Bits [4:4], mask 0x00000010
    pub const ABRT_GCALL_NOACK_OFFSET: u32 = 4;
    pub const ABRT_GCALL_NOACK_MASK: u32 = 0x00000010;
    pub const ABRT_GCALL_NOACK: u32 = 1 << 4;
    /// ABRT_GCALL_NOACK_VOID = 0: GCALL not ACKed by any slave-scenario not present
    pub const ABRT_GCALL_NOACK_ABRT_GCALL_NOACK_VOID: u32 = 0 << 4;
    /// ABRT_GCALL_NOACK_GENERATED = 1: GCALL not ACKed by any slave
    pub const ABRT_GCALL_NOACK_ABRT_GCALL_NOACK_GENERATED: u32 = 1 << 4;

    /// ABRT_TXDATA_NOACK - This field indicates the master-mode only bit. When the master receives an ...
    /// Bits [3:3], mask 0x00000008
    pub const ABRT_TXDATA_NOACK_OFFSET: u32 = 3;
    pub const ABRT_TXDATA_NOACK_MASK: u32 = 0x00000008;
    pub const ABRT_TXDATA_NOACK: u32 = 1 << 3;
    /// ABRT_TXDATA_NOACK_VOID = 0: Transmitted data non-ACKed by addressed slave-scenario not present
    pub const ABRT_TXDATA_NOACK_ABRT_TXDATA_NOACK_VOID: u32 = 0 << 3;
    /// ABRT_TXDATA_NOACK_GENERATED = 1: Transmitted data not ACKed by addressed slave
    pub const ABRT_TXDATA_NOACK_ABRT_TXDATA_NOACK_GENERATED: u32 = 1 << 3;

    /// ABRT_10ADDR2_NOACK - This field indicates that the Master is in 10-bit address mode and that the...
    /// Bits [2:2], mask 0x00000004
    pub const ABRT_10ADDR2_NOACK_OFFSET: u32 = 2;
    pub const ABRT_10ADDR2_NOACK_MASK: u32 = 0x00000004;
    pub const ABRT_10ADDR2_NOACK: u32 = 1 << 2;
    /// INACTIVE = 0: This abort is not generated
    pub const ABRT_10ADDR2_NOACK_INACTIVE: u32 = 0 << 2;
    /// ACTIVE = 1: Byte 2 of 10Bit Address not ACKed by any slave
    pub const ABRT_10ADDR2_NOACK_ACTIVE: u32 = 1 << 2;

    /// ABRT_10ADDR1_NOACK - This field indicates that the Master is in 10-bit address mode and the firs...
    /// Bits [1:1], mask 0x00000002
    pub const ABRT_10ADDR1_NOACK_OFFSET: u32 = 1;
    pub const ABRT_10ADDR1_NOACK_MASK: u32 = 0x00000002;
    pub const ABRT_10ADDR1_NOACK: u32 = 1 << 1;
    /// INACTIVE = 0: This abort is not generated
    pub const ABRT_10ADDR1_NOACK_INACTIVE: u32 = 0 << 1;
    /// ACTIVE = 1: Byte 1 of 10Bit Address not ACKed by any slave
    pub const ABRT_10ADDR1_NOACK_ACTIVE: u32 = 1 << 1;

    /// ABRT_7B_ADDR_NOACK - This field indicates that the Master is in 7-bit addressing mode and the ad...
    /// Bits [0:0], mask 0x00000001
    pub const ABRT_7B_ADDR_NOACK_OFFSET: u32 = 0;
    pub const ABRT_7B_ADDR_NOACK_MASK: u32 = 0x00000001;
    pub const ABRT_7B_ADDR_NOACK: u32 = 1 << 0;
    /// INACTIVE = 0: This abort is not generated
    pub const ABRT_7B_ADDR_NOACK_INACTIVE: u32 = 0 << 0;
    /// ACTIVE = 1: This abort is generated because of NOACK for 7-bit address
    pub const ABRT_7B_ADDR_NOACK_ACTIVE: u32 = 1 << 0;

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

    /// **TX_FLUSH_CNT** [31:23]: This field indicates the number of Tx FIFO Data Commands which are flushed ...
    #[inline(always)]
    pub fn tx_flush_cnt(&self) -> u32 {
        (self.read_bits() & Self::TX_FLUSH_CNT_MASK) >> Self::TX_FLUSH_CNT_OFFSET
    }

    /// **ABRT_USER_ABRT** [16:16]: This is a master-mode-only bit. Master has detected the transfer abort (IC_...
    #[inline(always)]
    pub fn abrt_user_abrt(&self) -> bool {
        (self.read_bits() & Self::ABRT_USER_ABRT_MASK) != 0
    }

    /// **ABRT_SLVRD_INTX** [15:15]: 1: When the processor side responds to a slave mode request for data to be ...
    #[inline(always)]
    pub fn abrt_slvrd_intx(&self) -> bool {
        (self.read_bits() & Self::ABRT_SLVRD_INTX_MASK) != 0
    }

    /// **ABRT_SLV_ARBLOST** [14:14]: This field indicates that a Slave has lost the bus while transmitting data ...
    #[inline(always)]
    pub fn abrt_slv_arblost(&self) -> bool {
        (self.read_bits() & Self::ABRT_SLV_ARBLOST_MASK) != 0
    }

    /// **ABRT_SLVFLUSH_TXFIFO** [13:13]: This field specifies that the Slave has received a read command and some da...
    #[inline(always)]
    pub fn abrt_slvflush_txfifo(&self) -> bool {
        (self.read_bits() & Self::ABRT_SLVFLUSH_TXFIFO_MASK) != 0
    }

    /// **ARB_LOST** [12:12]: This field specifies that the Master has lost arbitration, or if IC_TX_ABRT...
    #[inline(always)]
    pub fn arb_lost(&self) -> bool {
        (self.read_bits() & Self::ARB_LOST_MASK) != 0
    }

    /// **ABRT_MASTER_DIS** [11:11]: This field indicates that the User tries to initiate a Master operation wit...
    #[inline(always)]
    pub fn abrt_master_dis(&self) -> bool {
        (self.read_bits() & Self::ABRT_MASTER_DIS_MASK) != 0
    }

    /// **ABRT_10B_RD_NORSTRT** [10:10]: This field indicates that the restart is disabled (IC_RESTART_EN bit (IC_CO...
    #[inline(always)]
    pub fn abrt_10b_rd_norstrt(&self) -> bool {
        (self.read_bits() & Self::ABRT_10B_RD_NORSTRT_MASK) != 0
    }

    /// **ABRT_SBYTE_NORSTRT** [9:9]: To clear Bit 9, the source of the ABRT_SBYTE_NORSTRT must be fixed first; r...
    #[inline(always)]
    pub fn abrt_sbyte_norstrt(&self) -> bool {
        (self.read_bits() & Self::ABRT_SBYTE_NORSTRT_MASK) != 0
    }

    /// **ABRT_HS_NORSTRT** [8:8]: This field indicates that the restart is disabled (IC_RESTART_EN bit (IC_CO...
    #[inline(always)]
    pub fn abrt_hs_norstrt(&self) -> bool {
        (self.read_bits() & Self::ABRT_HS_NORSTRT_MASK) != 0
    }

    /// **ABRT_SBYTE_ACKDET** [7:7]: This field indicates that the Master has sent a START Byte and the START By...
    #[inline(always)]
    pub fn abrt_sbyte_ackdet(&self) -> bool {
        (self.read_bits() & Self::ABRT_SBYTE_ACKDET_MASK) != 0
    }

    /// **ABRT_HS_ACKDET** [6:6]: This field indicates that the Master is in High Speed mode and the High Spe...
    #[inline(always)]
    pub fn abrt_hs_ackdet(&self) -> bool {
        (self.read_bits() & Self::ABRT_HS_ACKDET_MASK) != 0
    }

    /// **ABRT_GCALL_READ** [5:5]: This field indicates that DW_apb_i2c in the master mode has sent a General ...
    #[inline(always)]
    pub fn abrt_gcall_read(&self) -> bool {
        (self.read_bits() & Self::ABRT_GCALL_READ_MASK) != 0
    }

    /// **ABRT_GCALL_NOACK** [4:4]: This field indicates that DW_apb_i2c in master mode has sent a General Call...
    #[inline(always)]
    pub fn abrt_gcall_noack(&self) -> bool {
        (self.read_bits() & Self::ABRT_GCALL_NOACK_MASK) != 0
    }

    /// **ABRT_TXDATA_NOACK** [3:3]: This field indicates the master-mode only bit. When the master receives an ...
    #[inline(always)]
    pub fn abrt_txdata_noack(&self) -> bool {
        (self.read_bits() & Self::ABRT_TXDATA_NOACK_MASK) != 0
    }

    /// **ABRT_10ADDR2_NOACK** [2:2]: This field indicates that the Master is in 10-bit address mode and that the...
    #[inline(always)]
    pub fn abrt_10addr2_noack(&self) -> bool {
        (self.read_bits() & Self::ABRT_10ADDR2_NOACK_MASK) != 0
    }

    /// **ABRT_10ADDR1_NOACK** [1:1]: This field indicates that the Master is in 10-bit address mode and the firs...
    #[inline(always)]
    pub fn abrt_10addr1_noack(&self) -> bool {
        (self.read_bits() & Self::ABRT_10ADDR1_NOACK_MASK) != 0
    }

    /// **ABRT_7B_ADDR_NOACK** [0:0]: This field indicates that the Master is in 7-bit addressing mode and the ad...
    #[inline(always)]
    pub fn abrt_7b_addr_noack(&self) -> bool {
        (self.read_bits() & Self::ABRT_7B_ADDR_NOACK_MASK) != 0
    }

}

/// IC_TX_ABRT_SOURCE value type for batch read/modify/write operations
///
/// Use `ictxabrtsource.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ictxabrtsource.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IcTxAbrtSourceVal {
    bits: u32,
}

impl IcTxAbrtSourceVal {
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

    /// Read TX_FLUSH_CNT [31:23]: This field indicates the number of Tx FIFO Data Commands which are flushed ...
    #[inline(always)]
    pub const fn tx_flush_cnt(&self) -> u32 {
        (self.bits & IcTxAbrtSource::TX_FLUSH_CNT_MASK) >> IcTxAbrtSource::TX_FLUSH_CNT_OFFSET
    }

    /// Read ABRT_USER_ABRT [16:16]: This is a master-mode-only bit. Master has detected the transfer abort (IC_...
    #[inline(always)]
    pub const fn abrt_user_abrt(&self) -> bool {
        (self.bits & IcTxAbrtSource::ABRT_USER_ABRT_MASK) != 0
    }

    /// Read ABRT_SLVRD_INTX [15:15]: 1: When the processor side responds to a slave mode request for data to be ...
    #[inline(always)]
    pub const fn abrt_slvrd_intx(&self) -> bool {
        (self.bits & IcTxAbrtSource::ABRT_SLVRD_INTX_MASK) != 0
    }

    /// Read ABRT_SLV_ARBLOST [14:14]: This field indicates that a Slave has lost the bus while transmitting data ...
    #[inline(always)]
    pub const fn abrt_slv_arblost(&self) -> bool {
        (self.bits & IcTxAbrtSource::ABRT_SLV_ARBLOST_MASK) != 0
    }

    /// Read ABRT_SLVFLUSH_TXFIFO [13:13]: This field specifies that the Slave has received a read command and some da...
    #[inline(always)]
    pub const fn abrt_slvflush_txfifo(&self) -> bool {
        (self.bits & IcTxAbrtSource::ABRT_SLVFLUSH_TXFIFO_MASK) != 0
    }

    /// Read ARB_LOST [12:12]: This field specifies that the Master has lost arbitration, or if IC_TX_ABRT...
    #[inline(always)]
    pub const fn arb_lost(&self) -> bool {
        (self.bits & IcTxAbrtSource::ARB_LOST_MASK) != 0
    }

    /// Read ABRT_MASTER_DIS [11:11]: This field indicates that the User tries to initiate a Master operation wit...
    #[inline(always)]
    pub const fn abrt_master_dis(&self) -> bool {
        (self.bits & IcTxAbrtSource::ABRT_MASTER_DIS_MASK) != 0
    }

    /// Read ABRT_10B_RD_NORSTRT [10:10]: This field indicates that the restart is disabled (IC_RESTART_EN bit (IC_CO...
    #[inline(always)]
    pub const fn abrt_10b_rd_norstrt(&self) -> bool {
        (self.bits & IcTxAbrtSource::ABRT_10B_RD_NORSTRT_MASK) != 0
    }

    /// Read ABRT_SBYTE_NORSTRT [9:9]: To clear Bit 9, the source of the ABRT_SBYTE_NORSTRT must be fixed first; r...
    #[inline(always)]
    pub const fn abrt_sbyte_norstrt(&self) -> bool {
        (self.bits & IcTxAbrtSource::ABRT_SBYTE_NORSTRT_MASK) != 0
    }

    /// Read ABRT_HS_NORSTRT [8:8]: This field indicates that the restart is disabled (IC_RESTART_EN bit (IC_CO...
    #[inline(always)]
    pub const fn abrt_hs_norstrt(&self) -> bool {
        (self.bits & IcTxAbrtSource::ABRT_HS_NORSTRT_MASK) != 0
    }

    /// Read ABRT_SBYTE_ACKDET [7:7]: This field indicates that the Master has sent a START Byte and the START By...
    #[inline(always)]
    pub const fn abrt_sbyte_ackdet(&self) -> bool {
        (self.bits & IcTxAbrtSource::ABRT_SBYTE_ACKDET_MASK) != 0
    }

    /// Read ABRT_HS_ACKDET [6:6]: This field indicates that the Master is in High Speed mode and the High Spe...
    #[inline(always)]
    pub const fn abrt_hs_ackdet(&self) -> bool {
        (self.bits & IcTxAbrtSource::ABRT_HS_ACKDET_MASK) != 0
    }

    /// Read ABRT_GCALL_READ [5:5]: This field indicates that DW_apb_i2c in the master mode has sent a General ...
    #[inline(always)]
    pub const fn abrt_gcall_read(&self) -> bool {
        (self.bits & IcTxAbrtSource::ABRT_GCALL_READ_MASK) != 0
    }

    /// Read ABRT_GCALL_NOACK [4:4]: This field indicates that DW_apb_i2c in master mode has sent a General Call...
    #[inline(always)]
    pub const fn abrt_gcall_noack(&self) -> bool {
        (self.bits & IcTxAbrtSource::ABRT_GCALL_NOACK_MASK) != 0
    }

    /// Read ABRT_TXDATA_NOACK [3:3]: This field indicates the master-mode only bit. When the master receives an ...
    #[inline(always)]
    pub const fn abrt_txdata_noack(&self) -> bool {
        (self.bits & IcTxAbrtSource::ABRT_TXDATA_NOACK_MASK) != 0
    }

    /// Read ABRT_10ADDR2_NOACK [2:2]: This field indicates that the Master is in 10-bit address mode and that the...
    #[inline(always)]
    pub const fn abrt_10addr2_noack(&self) -> bool {
        (self.bits & IcTxAbrtSource::ABRT_10ADDR2_NOACK_MASK) != 0
    }

    /// Read ABRT_10ADDR1_NOACK [1:1]: This field indicates that the Master is in 10-bit address mode and the firs...
    #[inline(always)]
    pub const fn abrt_10addr1_noack(&self) -> bool {
        (self.bits & IcTxAbrtSource::ABRT_10ADDR1_NOACK_MASK) != 0
    }

    /// Read ABRT_7B_ADDR_NOACK [0:0]: This field indicates that the Master is in 7-bit addressing mode and the ad...
    #[inline(always)]
    pub const fn abrt_7b_addr_noack(&self) -> bool {
        (self.bits & IcTxAbrtSource::ABRT_7B_ADDR_NOACK_MASK) != 0
    }

}

impl Default for IcTxAbrtSourceVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl IcTxAbrtSource {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> IcTxAbrtSourceVal {
        IcTxAbrtSourceVal::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: IcTxAbrtSourceVal) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(IcTxAbrtSourceVal) -> IcTxAbrtSourceVal>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// IC_SLV_DATA_NACK_ONLY Register
///
/// Generate Slave Data NACK Register The register is used to generate a NACK
/// for the data part of a transfer when DW_apb_i2c is acting as a
/// slave-receiver. This register only exists when the IC_SLV_DATA_NACK_ONLY
/// parameter is set to 1. When this parameter disabled, this register does not
/// exist and writing to the register's address has no effect. A write can occur
/// on this register if both of the following conditions are met: - DW_apb_i2c
/// is disabled (IC_ENABLE[0] = 0) - Slave part is inactive (IC_STATUS[6] = 0)
/// Note: The IC_STATUS[6] is a register read-back location for the internal
/// slv_activity signal; the user should poll this before writing the
/// ic_slv_data_nack_only bit.
///
/// Address offset: 0x0084
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **NACK** [0] (RW): Generate NACK. This NACK generation only occurs when DW_apb_i2c is a slave-...
#[repr(transparent)]
pub struct IcSlvDataNackOnly {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for IcSlvDataNackOnly {}
unsafe impl Sync for IcSlvDataNackOnly {}

impl IcSlvDataNackOnly {
    // ======== Field Constants ========

    /// NACK - Generate NACK. This NACK generation only occurs when DW_apb_i2c is a slave-...
    /// Bits [0:0], mask 0x00000001
    pub const NACK_OFFSET: u32 = 0;
    pub const NACK_MASK: u32 = 0x00000001;
    pub const NACK: u32 = 1 << 0;
    /// DISABLED = 0: Slave receiver generates NACK normally
    pub const NACK_DISABLED: u32 = 0 << 0;
    /// ENABLED = 1: Slave receiver generates NACK upon data reception only
    pub const NACK_ENABLED: u32 = 1 << 0;

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

    /// **NACK** [0:0]: Generate NACK. This NACK generation only occurs when DW_apb_i2c is a slave-...
    #[inline(always)]
    pub fn nack(&self) -> bool {
        (self.read_bits() & Self::NACK_MASK) != 0
    }

    /// Set NACK bit
    #[inline(always)]
    pub fn set_nack(&self) {
        self.set_bits(Self::NACK_MASK);
    }

    /// Clear NACK bit
    #[inline(always)]
    pub fn clr_nack(&self) {
        self.clr_bits(Self::NACK_MASK);
    }

}

/// IC_SLV_DATA_NACK_ONLY value type for batch read/modify/write operations
///
/// Use `icslvdatanackonly.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `icslvdatanackonly.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IcSlvDataNackOnlyVal {
    bits: u32,
}

impl IcSlvDataNackOnlyVal {
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

    /// Read NACK [0:0]: Generate NACK. This NACK generation only occurs when DW_apb_i2c is a slave-...
    #[inline(always)]
    pub const fn nack(&self) -> bool {
        (self.bits & IcSlvDataNackOnly::NACK_MASK) != 0
    }

    /// Set NACK [0:0]: Generate NACK. This NACK generation only occurs when DW_apb_i2c is a slave-...
    #[inline(always)]
    pub const fn with_nack(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IcSlvDataNackOnly::NACK_MASK
            } else {
                self.bits & !IcSlvDataNackOnly::NACK_MASK
            }
        }
    }

}

impl Default for IcSlvDataNackOnlyVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl IcSlvDataNackOnly {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> IcSlvDataNackOnlyVal {
        IcSlvDataNackOnlyVal::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: IcSlvDataNackOnlyVal) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(IcSlvDataNackOnlyVal) -> IcSlvDataNackOnlyVal>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// IC_DMA_CR Register
///
/// DMA Control Register The register is used to enable the DMA Controller
/// interface operation. There is a separate bit for transmit and receive. This
/// can be programmed regardless of the state of IC_ENABLE.
///
/// Address offset: 0x0088
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **TDMAE** [1] (RW): Transmit DMA Enable. This bit enables/disables the transmit FIFO DMA channe...
/// - **RDMAE** [0] (RW): Receive DMA Enable. This bit enables/disables the receive FIFO DMA channel....
#[repr(transparent)]
pub struct IcDmaCr {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for IcDmaCr {}
unsafe impl Sync for IcDmaCr {}

impl IcDmaCr {
    // ======== Field Constants ========

    /// TDMAE - Transmit DMA Enable. This bit enables/disables the transmit FIFO DMA channe...
    /// Bits [1:1], mask 0x00000002
    pub const TDMAE_OFFSET: u32 = 1;
    pub const TDMAE_MASK: u32 = 0x00000002;
    pub const TDMAE: u32 = 1 << 1;
    /// DISABLED = 0: transmit FIFO DMA channel disabled
    pub const TDMAE_DISABLED: u32 = 0 << 1;
    /// ENABLED = 1: Transmit FIFO DMA channel enabled
    pub const TDMAE_ENABLED: u32 = 1 << 1;

    /// RDMAE - Receive DMA Enable. This bit enables/disables the receive FIFO DMA channel....
    /// Bits [0:0], mask 0x00000001
    pub const RDMAE_OFFSET: u32 = 0;
    pub const RDMAE_MASK: u32 = 0x00000001;
    pub const RDMAE: u32 = 1 << 0;
    /// DISABLED = 0: Receive FIFO DMA channel disabled
    pub const RDMAE_DISABLED: u32 = 0 << 0;
    /// ENABLED = 1: Receive FIFO DMA channel enabled
    pub const RDMAE_ENABLED: u32 = 1 << 0;

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

    /// **TDMAE** [1:1]: Transmit DMA Enable. This bit enables/disables the transmit FIFO DMA channe...
    #[inline(always)]
    pub fn tdmae(&self) -> bool {
        (self.read_bits() & Self::TDMAE_MASK) != 0
    }

    /// Set TDMAE bit
    #[inline(always)]
    pub fn set_tdmae(&self) {
        self.set_bits(Self::TDMAE_MASK);
    }

    /// Clear TDMAE bit
    #[inline(always)]
    pub fn clr_tdmae(&self) {
        self.clr_bits(Self::TDMAE_MASK);
    }

    /// **RDMAE** [0:0]: Receive DMA Enable. This bit enables/disables the receive FIFO DMA channel....
    #[inline(always)]
    pub fn rdmae(&self) -> bool {
        (self.read_bits() & Self::RDMAE_MASK) != 0
    }

    /// Set RDMAE bit
    #[inline(always)]
    pub fn set_rdmae(&self) {
        self.set_bits(Self::RDMAE_MASK);
    }

    /// Clear RDMAE bit
    #[inline(always)]
    pub fn clr_rdmae(&self) {
        self.clr_bits(Self::RDMAE_MASK);
    }

}

/// IC_DMA_CR value type for batch read/modify/write operations
///
/// Use `icdmacr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `icdmacr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IcDmaCrVal {
    bits: u32,
}

impl IcDmaCrVal {
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

    /// Read TDMAE [1:1]: Transmit DMA Enable. This bit enables/disables the transmit FIFO DMA channe...
    #[inline(always)]
    pub const fn tdmae(&self) -> bool {
        (self.bits & IcDmaCr::TDMAE_MASK) != 0
    }

    /// Read RDMAE [0:0]: Receive DMA Enable. This bit enables/disables the receive FIFO DMA channel....
    #[inline(always)]
    pub const fn rdmae(&self) -> bool {
        (self.bits & IcDmaCr::RDMAE_MASK) != 0
    }

    /// Set TDMAE [1:1]: Transmit DMA Enable. This bit enables/disables the transmit FIFO DMA channe...
    #[inline(always)]
    pub const fn with_tdmae(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IcDmaCr::TDMAE_MASK
            } else {
                self.bits & !IcDmaCr::TDMAE_MASK
            }
        }
    }

    /// Set RDMAE [0:0]: Receive DMA Enable. This bit enables/disables the receive FIFO DMA channel....
    #[inline(always)]
    pub const fn with_rdmae(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IcDmaCr::RDMAE_MASK
            } else {
                self.bits & !IcDmaCr::RDMAE_MASK
            }
        }
    }

}

impl Default for IcDmaCrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl IcDmaCr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> IcDmaCrVal {
        IcDmaCrVal::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: IcDmaCrVal) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(IcDmaCrVal) -> IcDmaCrVal>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// IC_DMA_TDLR Register
///
/// DMA Transmit Data Level Register
///
/// Address offset: 0x008c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **DMATDL** [3:0] (RW): Transmit Data Level. This bit field controls the level at which a DMA reque...
#[repr(transparent)]
pub struct IcDmaTdlr {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for IcDmaTdlr {}
unsafe impl Sync for IcDmaTdlr {}

impl IcDmaTdlr {
    // ======== Field Constants ========

    /// DMATDL - Transmit Data Level. This bit field controls the level at which a DMA reque...
    /// Bits [3:0], mask 0x0000000f
    pub const DMATDL_OFFSET: u32 = 0;
    pub const DMATDL_MASK: u32 = 0x0000000f;

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

    /// **DMATDL** [3:0]: Transmit Data Level. This bit field controls the level at which a DMA reque...
    #[inline(always)]
    pub fn dmatdl(&self) -> u32 {
        (self.read_bits() & Self::DMATDL_MASK) >> Self::DMATDL_OFFSET
    }

    /// Write DMATDL field
    #[inline(always)]
    pub fn write_dmatdl(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::DMATDL_MASK) | ((value << Self::DMATDL_OFFSET) & Self::DMATDL_MASK));
    }

}

/// IC_DMA_TDLR value type for batch read/modify/write operations
///
/// Use `icdmatdlr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `icdmatdlr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IcDmaTdlrVal {
    bits: u32,
}

impl IcDmaTdlrVal {
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

    /// Read DMATDL [3:0]: Transmit Data Level. This bit field controls the level at which a DMA reque...
    #[inline(always)]
    pub const fn dmatdl(&self) -> u32 {
        (self.bits & IcDmaTdlr::DMATDL_MASK) >> IcDmaTdlr::DMATDL_OFFSET
    }

    /// Set DMATDL [3:0]: Transmit Data Level. This bit field controls the level at which a DMA reque...
    #[inline(always)]
    pub const fn with_dmatdl(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !IcDmaTdlr::DMATDL_MASK) | ((value << IcDmaTdlr::DMATDL_OFFSET) & IcDmaTdlr::DMATDL_MASK)
        }
    }

}

impl Default for IcDmaTdlrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl IcDmaTdlr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> IcDmaTdlrVal {
        IcDmaTdlrVal::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: IcDmaTdlrVal) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(IcDmaTdlrVal) -> IcDmaTdlrVal>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// IC_DMA_RDLR Register
///
/// I2C Receive Data Level Register
///
/// Address offset: 0x0090
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **DMARDL** [3:0] (RW): Receive Data Level. This bit field controls the level at which a DMA reques...
#[repr(transparent)]
pub struct IcDmaRdlr {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for IcDmaRdlr {}
unsafe impl Sync for IcDmaRdlr {}

impl IcDmaRdlr {
    // ======== Field Constants ========

    /// DMARDL - Receive Data Level. This bit field controls the level at which a DMA reques...
    /// Bits [3:0], mask 0x0000000f
    pub const DMARDL_OFFSET: u32 = 0;
    pub const DMARDL_MASK: u32 = 0x0000000f;

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

    /// **DMARDL** [3:0]: Receive Data Level. This bit field controls the level at which a DMA reques...
    #[inline(always)]
    pub fn dmardl(&self) -> u32 {
        (self.read_bits() & Self::DMARDL_MASK) >> Self::DMARDL_OFFSET
    }

    /// Write DMARDL field
    #[inline(always)]
    pub fn write_dmardl(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::DMARDL_MASK) | ((value << Self::DMARDL_OFFSET) & Self::DMARDL_MASK));
    }

}

/// IC_DMA_RDLR value type for batch read/modify/write operations
///
/// Use `icdmardlr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `icdmardlr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IcDmaRdlrVal {
    bits: u32,
}

impl IcDmaRdlrVal {
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

    /// Read DMARDL [3:0]: Receive Data Level. This bit field controls the level at which a DMA reques...
    #[inline(always)]
    pub const fn dmardl(&self) -> u32 {
        (self.bits & IcDmaRdlr::DMARDL_MASK) >> IcDmaRdlr::DMARDL_OFFSET
    }

    /// Set DMARDL [3:0]: Receive Data Level. This bit field controls the level at which a DMA reques...
    #[inline(always)]
    pub const fn with_dmardl(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !IcDmaRdlr::DMARDL_MASK) | ((value << IcDmaRdlr::DMARDL_OFFSET) & IcDmaRdlr::DMARDL_MASK)
        }
    }

}

impl Default for IcDmaRdlrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl IcDmaRdlr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> IcDmaRdlrVal {
        IcDmaRdlrVal::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: IcDmaRdlrVal) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(IcDmaRdlrVal) -> IcDmaRdlrVal>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// IC_SDA_SETUP Register
///
/// I2C SDA Setup Register This register controls the amount of time delay (in
/// terms of number of ic_clk clock periods) introduced in the rising edge of
/// SCL - relative to SDA changing - when DW_apb_i2c services a read request in
/// a slave-transmitter operation. The relevant I2C requirement is tSU:DAT (note
/// 4) as detailed in the I2C Bus Specification. This register must be
/// programmed with a value equal to or greater than 2. Writes to this register
/// succeed only when IC_ENABLE[0] = 0. Note: The length of setup time is
/// calculated using [(IC_SDA_SETUP - 1) * (ic_clk_period)], so if the user
/// requires 10 ic_clk periods of setup time, they should program a value of 11.
/// The IC_SDA_SETUP register is only used by the DW_apb_i2c when operating as a
/// slave transmitter.
///
/// Address offset: 0x0094
/// Reset value: 0x00000064
///
/// ## Fields
///
/// - **SDA_SETUP** [7:0] (RW): SDA Setup. It is recommended that if the required delay is 1000ns, then for...
#[repr(transparent)]
pub struct IcSdaSetup {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for IcSdaSetup {}
unsafe impl Sync for IcSdaSetup {}

impl IcSdaSetup {
    // ======== Field Constants ========

    /// SDA_SETUP - SDA Setup. It is recommended that if the required delay is 1000ns, then for...
    /// Bits [7:0], mask 0x000000ff
    pub const SDA_SETUP_OFFSET: u32 = 0;
    pub const SDA_SETUP_MASK: u32 = 0x000000ff;

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

    /// **SDA_SETUP** [7:0]: SDA Setup. It is recommended that if the required delay is 1000ns, then for...
    #[inline(always)]
    pub fn sda_setup(&self) -> u32 {
        (self.read_bits() & Self::SDA_SETUP_MASK) >> Self::SDA_SETUP_OFFSET
    }

    /// Write SDA_SETUP field
    #[inline(always)]
    pub fn write_sda_setup(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::SDA_SETUP_MASK) | ((value << Self::SDA_SETUP_OFFSET) & Self::SDA_SETUP_MASK));
    }

}

/// IC_SDA_SETUP value type for batch read/modify/write operations
///
/// Use `icsdasetup.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `icsdasetup.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IcSdaSetupVal {
    bits: u32,
}

impl IcSdaSetupVal {
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
        Self { bits: 0x00000064 }
    }

    /// Read SDA_SETUP [7:0]: SDA Setup. It is recommended that if the required delay is 1000ns, then for...
    #[inline(always)]
    pub const fn sda_setup(&self) -> u32 {
        (self.bits & IcSdaSetup::SDA_SETUP_MASK) >> IcSdaSetup::SDA_SETUP_OFFSET
    }

    /// Set SDA_SETUP [7:0]: SDA Setup. It is recommended that if the required delay is 1000ns, then for...
    #[inline(always)]
    pub const fn with_sda_setup(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !IcSdaSetup::SDA_SETUP_MASK) | ((value << IcSdaSetup::SDA_SETUP_OFFSET) & IcSdaSetup::SDA_SETUP_MASK)
        }
    }

}

impl Default for IcSdaSetupVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl IcSdaSetup {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> IcSdaSetupVal {
        IcSdaSetupVal::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: IcSdaSetupVal) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(IcSdaSetupVal) -> IcSdaSetupVal>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// IC_ACK_GENERAL_CALL Register
///
/// I2C ACK General Call Register The register controls whether DW_apb_i2c
/// responds with a ACK or NACK when it receives an I2C General Call address.
/// This register is applicable only when the DW_apb_i2c is in slave mode.
///
/// Address offset: 0x0098
/// Reset value: 0x00000001
///
/// ## Fields
///
/// - **ACK_GEN_CALL** [0] (RW): ACK General Call. When set to 1, DW_apb_i2c responds with a ACK (by asserti...
#[repr(transparent)]
pub struct IcAckGeneralCall {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for IcAckGeneralCall {}
unsafe impl Sync for IcAckGeneralCall {}

impl IcAckGeneralCall {
    // ======== Field Constants ========

    /// ACK_GEN_CALL - ACK General Call. When set to 1, DW_apb_i2c responds with a ACK (by asserti...
    /// Bits [0:0], mask 0x00000001
    pub const ACK_GEN_CALL_OFFSET: u32 = 0;
    pub const ACK_GEN_CALL_MASK: u32 = 0x00000001;
    pub const ACK_GEN_CALL: u32 = 1 << 0;
    /// DISABLED = 0: Generate NACK for a General Call
    pub const ACK_GEN_CALL_DISABLED: u32 = 0 << 0;
    /// ENABLED = 1: Generate ACK for a General Call
    pub const ACK_GEN_CALL_ENABLED: u32 = 1 << 0;

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

    /// **ACK_GEN_CALL** [0:0]: ACK General Call. When set to 1, DW_apb_i2c responds with a ACK (by asserti...
    #[inline(always)]
    pub fn ack_gen_call(&self) -> bool {
        (self.read_bits() & Self::ACK_GEN_CALL_MASK) != 0
    }

    /// Set ACK_GEN_CALL bit
    #[inline(always)]
    pub fn set_ack_gen_call(&self) {
        self.set_bits(Self::ACK_GEN_CALL_MASK);
    }

    /// Clear ACK_GEN_CALL bit
    #[inline(always)]
    pub fn clr_ack_gen_call(&self) {
        self.clr_bits(Self::ACK_GEN_CALL_MASK);
    }

}

/// IC_ACK_GENERAL_CALL value type for batch read/modify/write operations
///
/// Use `icackgeneralcall.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `icackgeneralcall.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IcAckGeneralCallVal {
    bits: u32,
}

impl IcAckGeneralCallVal {
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

    /// Read ACK_GEN_CALL [0:0]: ACK General Call. When set to 1, DW_apb_i2c responds with a ACK (by asserti...
    #[inline(always)]
    pub const fn ack_gen_call(&self) -> bool {
        (self.bits & IcAckGeneralCall::ACK_GEN_CALL_MASK) != 0
    }

    /// Set ACK_GEN_CALL [0:0]: ACK General Call. When set to 1, DW_apb_i2c responds with a ACK (by asserti...
    #[inline(always)]
    pub const fn with_ack_gen_call(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IcAckGeneralCall::ACK_GEN_CALL_MASK
            } else {
                self.bits & !IcAckGeneralCall::ACK_GEN_CALL_MASK
            }
        }
    }

}

impl Default for IcAckGeneralCallVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl IcAckGeneralCall {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> IcAckGeneralCallVal {
        IcAckGeneralCallVal::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: IcAckGeneralCallVal) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(IcAckGeneralCallVal) -> IcAckGeneralCallVal>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// IC_ENABLE_STATUS Register
///
/// I2C Enable Status Register The register is used to report the DW_apb_i2c
/// hardware status when the IC_ENABLE[0] register is set from 1 to 0; that is,
/// when DW_apb_i2c is disabled. If IC_ENABLE[0] has been set to 1, bits 2:1 are
/// forced to 0, and bit 0 is forced to 1. If IC_ENABLE[0] has been set to 0,
/// bits 2:1 is only be valid as soon as bit 0 is read as '0'. Note: When
/// IC_ENABLE[0] has been set to 0, a delay occurs for bit 0 to be read as 0
/// because disabling the DW_apb_i2c depends on I2C bus activities.
///
/// Address offset: 0x009c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **SLV_RX_DATA_LOST** [2] (RO): Slave Received Data Lost. This bit indicates if a Slave-Receiver operation ...
/// - **SLV_DISABLED_WHILE_BUSY** [1] (RO): Slave Disabled While Busy (Transmit, Receive). This bit indicates if a pote...
/// - **IC_EN** [0] (RO): ic_en Status. This bit always reflects the value driven on the output port ...
#[repr(transparent)]
pub struct IcEnableStatus {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for IcEnableStatus {}
unsafe impl Sync for IcEnableStatus {}

impl IcEnableStatus {
    // ======== Field Constants ========

    /// SLV_RX_DATA_LOST - Slave Received Data Lost. This bit indicates if a Slave-Receiver operation ...
    /// Bits [2:2], mask 0x00000004
    pub const SLV_RX_DATA_LOST_OFFSET: u32 = 2;
    pub const SLV_RX_DATA_LOST_MASK: u32 = 0x00000004;
    pub const SLV_RX_DATA_LOST: u32 = 1 << 2;
    /// INACTIVE = 0: Slave RX Data is not lost
    pub const SLV_RX_DATA_LOST_INACTIVE: u32 = 0 << 2;
    /// ACTIVE = 1: Slave RX Data is lost
    pub const SLV_RX_DATA_LOST_ACTIVE: u32 = 1 << 2;

    /// SLV_DISABLED_WHILE_BUSY - Slave Disabled While Busy (Transmit, Receive). This bit indicates if a pote...
    /// Bits [1:1], mask 0x00000002
    pub const SLV_DISABLED_WHILE_BUSY_OFFSET: u32 = 1;
    pub const SLV_DISABLED_WHILE_BUSY_MASK: u32 = 0x00000002;
    pub const SLV_DISABLED_WHILE_BUSY: u32 = 1 << 1;
    /// INACTIVE = 0: Slave is disabled when it is idle
    pub const SLV_DISABLED_WHILE_BUSY_INACTIVE: u32 = 0 << 1;
    /// ACTIVE = 1: Slave is disabled when it is active
    pub const SLV_DISABLED_WHILE_BUSY_ACTIVE: u32 = 1 << 1;

    /// IC_EN - ic_en Status. This bit always reflects the value driven on the output port ...
    /// Bits [0:0], mask 0x00000001
    pub const IC_EN_OFFSET: u32 = 0;
    pub const IC_EN_MASK: u32 = 0x00000001;
    pub const IC_EN: u32 = 1 << 0;
    /// DISABLED = 0: I2C disabled
    pub const IC_EN_DISABLED: u32 = 0 << 0;
    /// ENABLED = 1: I2C enabled
    pub const IC_EN_ENABLED: u32 = 1 << 0;

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

    /// **SLV_RX_DATA_LOST** [2:2]: Slave Received Data Lost. This bit indicates if a Slave-Receiver operation ...
    #[inline(always)]
    pub fn slv_rx_data_lost(&self) -> bool {
        (self.read_bits() & Self::SLV_RX_DATA_LOST_MASK) != 0
    }

    /// **SLV_DISABLED_WHILE_BUSY** [1:1]: Slave Disabled While Busy (Transmit, Receive). This bit indicates if a pote...
    #[inline(always)]
    pub fn slv_disabled_while_busy(&self) -> bool {
        (self.read_bits() & Self::SLV_DISABLED_WHILE_BUSY_MASK) != 0
    }

    /// **IC_EN** [0:0]: ic_en Status. This bit always reflects the value driven on the output port ...
    #[inline(always)]
    pub fn ic_en(&self) -> bool {
        (self.read_bits() & Self::IC_EN_MASK) != 0
    }

}

/// IC_ENABLE_STATUS value type for batch read/modify/write operations
///
/// Use `icenablestatus.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `icenablestatus.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IcEnableStatusVal {
    bits: u32,
}

impl IcEnableStatusVal {
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

    /// Read SLV_RX_DATA_LOST [2:2]: Slave Received Data Lost. This bit indicates if a Slave-Receiver operation ...
    #[inline(always)]
    pub const fn slv_rx_data_lost(&self) -> bool {
        (self.bits & IcEnableStatus::SLV_RX_DATA_LOST_MASK) != 0
    }

    /// Read SLV_DISABLED_WHILE_BUSY [1:1]: Slave Disabled While Busy (Transmit, Receive). This bit indicates if a pote...
    #[inline(always)]
    pub const fn slv_disabled_while_busy(&self) -> bool {
        (self.bits & IcEnableStatus::SLV_DISABLED_WHILE_BUSY_MASK) != 0
    }

    /// Read IC_EN [0:0]: ic_en Status. This bit always reflects the value driven on the output port ...
    #[inline(always)]
    pub const fn ic_en(&self) -> bool {
        (self.bits & IcEnableStatus::IC_EN_MASK) != 0
    }

}

impl Default for IcEnableStatusVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl IcEnableStatus {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> IcEnableStatusVal {
        IcEnableStatusVal::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: IcEnableStatusVal) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(IcEnableStatusVal) -> IcEnableStatusVal>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// IC_FS_SPKLEN Register
///
/// I2C SS, FS or FM+ spike suppression limit This register is used to store the
/// duration, measured in ic_clk cycles, of the longest spike that is filtered
/// out by the spike suppression logic when the component is operating in SS, FS
/// or FM+ modes. The relevant I2C requirement is tSP (table 4) as detailed in
/// the I2C Bus Specification. This register must be programmed with a minimum
/// value of 1.
///
/// Address offset: 0x00a0
/// Reset value: 0x00000007
///
/// ## Fields
///
/// - **IC_FS_SPKLEN** [7:0] (RW): This register must be set before any I2C bus transaction can take place to ...
#[repr(transparent)]
pub struct IcFsSpklen {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for IcFsSpklen {}
unsafe impl Sync for IcFsSpklen {}

impl IcFsSpklen {
    // ======== Field Constants ========

    /// IC_FS_SPKLEN - This register must be set before any I2C bus transaction can take place to ...
    /// Bits [7:0], mask 0x000000ff
    pub const IC_FS_SPKLEN_OFFSET: u32 = 0;
    pub const IC_FS_SPKLEN_MASK: u32 = 0x000000ff;

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

    /// **IC_FS_SPKLEN** [7:0]: This register must be set before any I2C bus transaction can take place to ...
    #[inline(always)]
    pub fn ic_fs_spklen(&self) -> u32 {
        (self.read_bits() & Self::IC_FS_SPKLEN_MASK) >> Self::IC_FS_SPKLEN_OFFSET
    }

    /// Write IC_FS_SPKLEN field
    #[inline(always)]
    pub fn write_ic_fs_spklen(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::IC_FS_SPKLEN_MASK) | ((value << Self::IC_FS_SPKLEN_OFFSET) & Self::IC_FS_SPKLEN_MASK));
    }

}

/// IC_FS_SPKLEN value type for batch read/modify/write operations
///
/// Use `icfsspklen.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `icfsspklen.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IcFsSpklenVal {
    bits: u32,
}

impl IcFsSpklenVal {
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
        Self { bits: 0x00000007 }
    }

    /// Read IC_FS_SPKLEN [7:0]: This register must be set before any I2C bus transaction can take place to ...
    #[inline(always)]
    pub const fn ic_fs_spklen(&self) -> u32 {
        (self.bits & IcFsSpklen::IC_FS_SPKLEN_MASK) >> IcFsSpklen::IC_FS_SPKLEN_OFFSET
    }

    /// Set IC_FS_SPKLEN [7:0]: This register must be set before any I2C bus transaction can take place to ...
    #[inline(always)]
    pub const fn with_ic_fs_spklen(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !IcFsSpklen::IC_FS_SPKLEN_MASK) | ((value << IcFsSpklen::IC_FS_SPKLEN_OFFSET) & IcFsSpklen::IC_FS_SPKLEN_MASK)
        }
    }

}

impl Default for IcFsSpklenVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl IcFsSpklen {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> IcFsSpklenVal {
        IcFsSpklenVal::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: IcFsSpklenVal) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(IcFsSpklenVal) -> IcFsSpklenVal>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// IC_CLR_RESTART_DET Register
///
/// Clear RESTART_DET Interrupt Register
///
/// Address offset: 0x00a8
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **CLR_RESTART_DET** [0] (RO): Read this register to clear the RESTART_DET interrupt (bit 12) of IC_RAW_IN...
#[repr(transparent)]
pub struct IcClrRestartDet {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for IcClrRestartDet {}
unsafe impl Sync for IcClrRestartDet {}

impl IcClrRestartDet {
    // ======== Field Constants ========

    /// CLR_RESTART_DET - Read this register to clear the RESTART_DET interrupt (bit 12) of IC_RAW_IN...
    /// Bits [0:0], mask 0x00000001
    pub const CLR_RESTART_DET_OFFSET: u32 = 0;
    pub const CLR_RESTART_DET_MASK: u32 = 0x00000001;
    pub const CLR_RESTART_DET: u32 = 1 << 0;

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

    /// **CLR_RESTART_DET** [0:0]: Read this register to clear the RESTART_DET interrupt (bit 12) of IC_RAW_IN...
    #[inline(always)]
    pub fn clr_restart_det(&self) -> bool {
        (self.read_bits() & Self::CLR_RESTART_DET_MASK) != 0
    }

}

/// IC_CLR_RESTART_DET value type for batch read/modify/write operations
///
/// Use `icclrrestartdet.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `icclrrestartdet.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IcClrRestartDetVal {
    bits: u32,
}

impl IcClrRestartDetVal {
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

    /// Read CLR_RESTART_DET [0:0]: Read this register to clear the RESTART_DET interrupt (bit 12) of IC_RAW_IN...
    #[inline(always)]
    pub const fn clr_restart_det(&self) -> bool {
        (self.bits & IcClrRestartDet::CLR_RESTART_DET_MASK) != 0
    }

}

impl Default for IcClrRestartDetVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl IcClrRestartDet {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> IcClrRestartDetVal {
        IcClrRestartDetVal::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: IcClrRestartDetVal) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(IcClrRestartDetVal) -> IcClrRestartDetVal>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// IC_COMP_PARAM_1 Register
///
/// Component Parameter Register 1 Note This register is not implemented and
/// therefore reads as 0. If it was implemented it would be a constant read-only
/// register that contains encoded information about the component's parameter
/// settings. Fields shown below are the settings for those parameters
///
/// Address offset: 0x00f4
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **TX_BUFFER_DEPTH** [23:16] (RO): TX Buffer Depth = 16
/// - **RX_BUFFER_DEPTH** [15:8] (RO): RX Buffer Depth = 16
/// - **ADD_ENCODED_PARAMS** [7] (RO): Encoded parameters not visible
/// - **HAS_DMA** [6] (RO): DMA handshaking signals are enabled
/// - **INTR_IO** [5] (RO): COMBINED Interrupt outputs
/// - **HC_COUNT_VALUES** [4] (RO): Programmable count values for each mode.
/// - **MAX_SPEED_MODE** [3:2] (RO): MAX SPEED MODE = FAST MODE
/// - **APB_DATA_WIDTH** [1:0] (RO): APB data bus width is 32 bits
#[repr(transparent)]
pub struct IcCompParam1 {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for IcCompParam1 {}
unsafe impl Sync for IcCompParam1 {}

impl IcCompParam1 {
    // ======== Field Constants ========

    /// TX_BUFFER_DEPTH - TX Buffer Depth = 16
    /// Bits [23:16], mask 0x00ff0000
    pub const TX_BUFFER_DEPTH_OFFSET: u32 = 16;
    pub const TX_BUFFER_DEPTH_MASK: u32 = 0x00ff0000;

    /// RX_BUFFER_DEPTH - RX Buffer Depth = 16
    /// Bits [15:8], mask 0x0000ff00
    pub const RX_BUFFER_DEPTH_OFFSET: u32 = 8;
    pub const RX_BUFFER_DEPTH_MASK: u32 = 0x0000ff00;

    /// ADD_ENCODED_PARAMS - Encoded parameters not visible
    /// Bits [7:7], mask 0x00000080
    pub const ADD_ENCODED_PARAMS_OFFSET: u32 = 7;
    pub const ADD_ENCODED_PARAMS_MASK: u32 = 0x00000080;
    pub const ADD_ENCODED_PARAMS: u32 = 1 << 7;

    /// HAS_DMA - DMA handshaking signals are enabled
    /// Bits [6:6], mask 0x00000040
    pub const HAS_DMA_OFFSET: u32 = 6;
    pub const HAS_DMA_MASK: u32 = 0x00000040;
    pub const HAS_DMA: u32 = 1 << 6;

    /// INTR_IO - COMBINED Interrupt outputs
    /// Bits [5:5], mask 0x00000020
    pub const INTR_IO_OFFSET: u32 = 5;
    pub const INTR_IO_MASK: u32 = 0x00000020;
    pub const INTR_IO: u32 = 1 << 5;

    /// HC_COUNT_VALUES - Programmable count values for each mode.
    /// Bits [4:4], mask 0x00000010
    pub const HC_COUNT_VALUES_OFFSET: u32 = 4;
    pub const HC_COUNT_VALUES_MASK: u32 = 0x00000010;
    pub const HC_COUNT_VALUES: u32 = 1 << 4;

    /// MAX_SPEED_MODE - MAX SPEED MODE = FAST MODE
    /// Bits [3:2], mask 0x0000000c
    pub const MAX_SPEED_MODE_OFFSET: u32 = 2;
    pub const MAX_SPEED_MODE_MASK: u32 = 0x0000000c;

    /// APB_DATA_WIDTH - APB data bus width is 32 bits
    /// Bits [1:0], mask 0x00000003
    pub const APB_DATA_WIDTH_OFFSET: u32 = 0;
    pub const APB_DATA_WIDTH_MASK: u32 = 0x00000003;

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

    /// **TX_BUFFER_DEPTH** [23:16]: TX Buffer Depth = 16
    #[inline(always)]
    pub fn tx_buffer_depth(&self) -> u32 {
        (self.read_bits() & Self::TX_BUFFER_DEPTH_MASK) >> Self::TX_BUFFER_DEPTH_OFFSET
    }

    /// **RX_BUFFER_DEPTH** [15:8]: RX Buffer Depth = 16
    #[inline(always)]
    pub fn rx_buffer_depth(&self) -> u32 {
        (self.read_bits() & Self::RX_BUFFER_DEPTH_MASK) >> Self::RX_BUFFER_DEPTH_OFFSET
    }

    /// **ADD_ENCODED_PARAMS** [7:7]: Encoded parameters not visible
    #[inline(always)]
    pub fn add_encoded_params(&self) -> bool {
        (self.read_bits() & Self::ADD_ENCODED_PARAMS_MASK) != 0
    }

    /// **HAS_DMA** [6:6]: DMA handshaking signals are enabled
    #[inline(always)]
    pub fn has_dma(&self) -> bool {
        (self.read_bits() & Self::HAS_DMA_MASK) != 0
    }

    /// **INTR_IO** [5:5]: COMBINED Interrupt outputs
    #[inline(always)]
    pub fn intr_io(&self) -> bool {
        (self.read_bits() & Self::INTR_IO_MASK) != 0
    }

    /// **HC_COUNT_VALUES** [4:4]: Programmable count values for each mode.
    #[inline(always)]
    pub fn hc_count_values(&self) -> bool {
        (self.read_bits() & Self::HC_COUNT_VALUES_MASK) != 0
    }

    /// **MAX_SPEED_MODE** [3:2]: MAX SPEED MODE = FAST MODE
    #[inline(always)]
    pub fn max_speed_mode(&self) -> u32 {
        (self.read_bits() & Self::MAX_SPEED_MODE_MASK) >> Self::MAX_SPEED_MODE_OFFSET
    }

    /// **APB_DATA_WIDTH** [1:0]: APB data bus width is 32 bits
    #[inline(always)]
    pub fn apb_data_width(&self) -> u32 {
        (self.read_bits() & Self::APB_DATA_WIDTH_MASK) >> Self::APB_DATA_WIDTH_OFFSET
    }

}

/// IC_COMP_PARAM_1 value type for batch read/modify/write operations
///
/// Use `iccompparam1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `iccompparam1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IcCompParam1Val {
    bits: u32,
}

impl IcCompParam1Val {
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

    /// Read TX_BUFFER_DEPTH [23:16]: TX Buffer Depth = 16
    #[inline(always)]
    pub const fn tx_buffer_depth(&self) -> u32 {
        (self.bits & IcCompParam1::TX_BUFFER_DEPTH_MASK) >> IcCompParam1::TX_BUFFER_DEPTH_OFFSET
    }

    /// Read RX_BUFFER_DEPTH [15:8]: RX Buffer Depth = 16
    #[inline(always)]
    pub const fn rx_buffer_depth(&self) -> u32 {
        (self.bits & IcCompParam1::RX_BUFFER_DEPTH_MASK) >> IcCompParam1::RX_BUFFER_DEPTH_OFFSET
    }

    /// Read ADD_ENCODED_PARAMS [7:7]: Encoded parameters not visible
    #[inline(always)]
    pub const fn add_encoded_params(&self) -> bool {
        (self.bits & IcCompParam1::ADD_ENCODED_PARAMS_MASK) != 0
    }

    /// Read HAS_DMA [6:6]: DMA handshaking signals are enabled
    #[inline(always)]
    pub const fn has_dma(&self) -> bool {
        (self.bits & IcCompParam1::HAS_DMA_MASK) != 0
    }

    /// Read INTR_IO [5:5]: COMBINED Interrupt outputs
    #[inline(always)]
    pub const fn intr_io(&self) -> bool {
        (self.bits & IcCompParam1::INTR_IO_MASK) != 0
    }

    /// Read HC_COUNT_VALUES [4:4]: Programmable count values for each mode.
    #[inline(always)]
    pub const fn hc_count_values(&self) -> bool {
        (self.bits & IcCompParam1::HC_COUNT_VALUES_MASK) != 0
    }

    /// Read MAX_SPEED_MODE [3:2]: MAX SPEED MODE = FAST MODE
    #[inline(always)]
    pub const fn max_speed_mode(&self) -> u32 {
        (self.bits & IcCompParam1::MAX_SPEED_MODE_MASK) >> IcCompParam1::MAX_SPEED_MODE_OFFSET
    }

    /// Read APB_DATA_WIDTH [1:0]: APB data bus width is 32 bits
    #[inline(always)]
    pub const fn apb_data_width(&self) -> u32 {
        (self.bits & IcCompParam1::APB_DATA_WIDTH_MASK) >> IcCompParam1::APB_DATA_WIDTH_OFFSET
    }

}

impl Default for IcCompParam1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl IcCompParam1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> IcCompParam1Val {
        IcCompParam1Val::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: IcCompParam1Val) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(IcCompParam1Val) -> IcCompParam1Val>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// IC_COMP_VERSION Register
///
/// I2C Component Version Register
///
/// Address offset: 0x00f8
/// Reset value: 0x3230312a
///
/// ## Fields
///
/// - **IC_COMP_VERSION** [31:0] (RO): 
#[repr(transparent)]
pub struct IcCompVersion {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for IcCompVersion {}
unsafe impl Sync for IcCompVersion {}

impl IcCompVersion {
    // ======== Field Constants ========

    /// Bits [31:0], mask 0xffffffff
    pub const IC_COMP_VERSION_OFFSET: u32 = 0;
    pub const IC_COMP_VERSION_MASK: u32 = 0xffffffff;

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

    /// **IC_COMP_VERSION** [31:0]
    #[inline(always)]
    pub fn ic_comp_version(&self) -> u32 {
        (self.read_bits() & Self::IC_COMP_VERSION_MASK) >> Self::IC_COMP_VERSION_OFFSET
    }

}

/// IC_COMP_VERSION value type for batch read/modify/write operations
///
/// Use `iccompversion.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `iccompversion.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IcCompVersionVal {
    bits: u32,
}

impl IcCompVersionVal {
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
        Self { bits: 0x3230312a }
    }

    /// Read IC_COMP_VERSION [31:0]
    #[inline(always)]
    pub const fn ic_comp_version(&self) -> u32 {
        (self.bits & IcCompVersion::IC_COMP_VERSION_MASK) >> IcCompVersion::IC_COMP_VERSION_OFFSET
    }

}

impl Default for IcCompVersionVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl IcCompVersion {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> IcCompVersionVal {
        IcCompVersionVal::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: IcCompVersionVal) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(IcCompVersionVal) -> IcCompVersionVal>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// IC_COMP_TYPE Register
///
/// I2C Component Type Register
///
/// Address offset: 0x00fc
/// Reset value: 0x44570140
///
/// ## Fields
///
/// - **IC_COMP_TYPE** [31:0] (RO): Designware Component Type number = 0x44_57_01_40. This assigned unique hex ...
#[repr(transparent)]
pub struct IcCompType {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for IcCompType {}
unsafe impl Sync for IcCompType {}

impl IcCompType {
    // ======== Field Constants ========

    /// IC_COMP_TYPE - Designware Component Type number = 0x44_57_01_40. This assigned unique hex ...
    /// Bits [31:0], mask 0xffffffff
    pub const IC_COMP_TYPE_OFFSET: u32 = 0;
    pub const IC_COMP_TYPE_MASK: u32 = 0xffffffff;

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

    /// **IC_COMP_TYPE** [31:0]: Designware Component Type number = 0x44_57_01_40. This assigned unique hex ...
    #[inline(always)]
    pub fn ic_comp_type(&self) -> u32 {
        (self.read_bits() & Self::IC_COMP_TYPE_MASK) >> Self::IC_COMP_TYPE_OFFSET
    }

}

/// IC_COMP_TYPE value type for batch read/modify/write operations
///
/// Use `iccomptype.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `iccomptype.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IcCompTypeVal {
    bits: u32,
}

impl IcCompTypeVal {
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
        Self { bits: 0x44570140 }
    }

    /// Read IC_COMP_TYPE [31:0]: Designware Component Type number = 0x44_57_01_40. This assigned unique hex ...
    #[inline(always)]
    pub const fn ic_comp_type(&self) -> u32 {
        (self.bits & IcCompType::IC_COMP_TYPE_MASK) >> IcCompType::IC_COMP_TYPE_OFFSET
    }

}

impl Default for IcCompTypeVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl IcCompType {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> IcCompTypeVal {
        IcCompTypeVal::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: IcCompTypeVal) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(IcCompTypeVal) -> IcCompTypeVal>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// Register block
#[repr(C)]
pub struct RegisterBlock {
    /// 0x0000 - I2C Control Register. This register can be written only when the DW_apb_i2c...
    pub ic_con: IcCon,
    /// 0x0004 - I2C Target Address Register This register is 12 bits wide, and bits 31:12 a...
    pub ic_tar: IcTar,
    /// 0x0008 - I2C Slave Address Register
    pub ic_sar: IcSar,
    _reserved0: [u8; 4],
    /// 0x0010 - I2C Rx/Tx Data Buffer and Command Register; this is the register the CPU wr...
    pub ic_data_cmd: IcDataCmd,
    /// 0x0014 - Standard Speed I2C Clock SCL High Count Register
    pub ic_ss_scl_hcnt: IcSsSclHcnt,
    /// 0x0018 - Standard Speed I2C Clock SCL Low Count Register
    pub ic_ss_scl_lcnt: IcSsSclLcnt,
    /// 0x001c - Fast Mode or Fast Mode Plus I2C Clock SCL High Count Register
    pub ic_fs_scl_hcnt: IcFsSclHcnt,
    /// 0x0020 - Fast Mode or Fast Mode Plus I2C Clock SCL Low Count Register
    pub ic_fs_scl_lcnt: IcFsSclLcnt,
    _reserved1: [u8; 8],
    /// 0x002c - I2C Interrupt Status Register Each bit in this register has a corresponding...
    pub ic_intr_stat: IcIntrStat,
    /// 0x0030 - I2C Interrupt Mask Register. These bits mask their corresponding interrupt ...
    pub ic_intr_mask: IcIntrMask,
    /// 0x0034 - I2C Raw Interrupt Status Register Unlike the IC_INTR_STAT register, these b...
    pub ic_raw_intr_stat: IcRawIntrStat,
    /// 0x0038 - I2C Receive FIFO Threshold Register
    pub ic_rx_tl: IcRxTl,
    /// 0x003c - I2C Transmit FIFO Threshold Register
    pub ic_tx_tl: IcTxTl,
    /// 0x0040 - Clear Combined and Individual Interrupt Register
    pub ic_clr_intr: IcClrIntr,
    /// 0x0044 - Clear RX_UNDER Interrupt Register
    pub ic_clr_rx_under: IcClrRxUnder,
    /// 0x0048 - Clear RX_OVER Interrupt Register
    pub ic_clr_rx_over: IcClrRxOver,
    /// 0x004c - Clear TX_OVER Interrupt Register
    pub ic_clr_tx_over: IcClrTxOver,
    /// 0x0050 - Clear RD_REQ Interrupt Register
    pub ic_clr_rd_req: IcClrRdReq,
    /// 0x0054 - Clear TX_ABRT Interrupt Register
    pub ic_clr_tx_abrt: IcClrTxAbrt,
    /// 0x0058 - Clear RX_DONE Interrupt Register
    pub ic_clr_rx_done: IcClrRxDone,
    /// 0x005c - Clear ACTIVITY Interrupt Register
    pub ic_clr_activity: IcClrActivity,
    /// 0x0060 - Clear STOP_DET Interrupt Register
    pub ic_clr_stop_det: IcClrStopDet,
    /// 0x0064 - Clear START_DET Interrupt Register
    pub ic_clr_start_det: IcClrStartDet,
    /// 0x0068 - Clear GEN_CALL Interrupt Register
    pub ic_clr_gen_call: IcClrGenCall,
    /// 0x006c - I2C Enable Register
    pub ic_enable: IcEnable,
    /// 0x0070 - I2C Status Register This is a read-only register used to indicate the curre...
    pub ic_status: IcStatus,
    /// 0x0074 - I2C Transmit FIFO Level Register This register contains the number of valid...
    pub ic_txflr: IcTxflr,
    /// 0x0078 - I2C Receive FIFO Level Register This register contains the number of valid ...
    pub ic_rxflr: IcRxflr,
    /// 0x007c - I2C SDA Hold Time Length Register The bits [15:0] of this register are used...
    pub ic_sda_hold: IcSdaHold,
    /// 0x0080 - I2C Transmit Abort Source Register This register has 32 bits that indicate ...
    pub ic_tx_abrt_source: IcTxAbrtSource,
    /// 0x0084 - Generate Slave Data NACK Register The register is used to generate a NACK f...
    pub ic_slv_data_nack_only: IcSlvDataNackOnly,
    /// 0x0088 - DMA Control Register The register is used to enable the DMA Controller inte...
    pub ic_dma_cr: IcDmaCr,
    /// 0x008c - DMA Transmit Data Level Register
    pub ic_dma_tdlr: IcDmaTdlr,
    /// 0x0090 - I2C Receive Data Level Register
    pub ic_dma_rdlr: IcDmaRdlr,
    /// 0x0094 - I2C SDA Setup Register This register controls the amount of time delay (in ...
    pub ic_sda_setup: IcSdaSetup,
    /// 0x0098 - I2C ACK General Call Register The register controls whether DW_apb_i2c resp...
    pub ic_ack_general_call: IcAckGeneralCall,
    /// 0x009c - I2C Enable Status Register The register is used to report the DW_apb_i2c ha...
    pub ic_enable_status: IcEnableStatus,
    /// 0x00a0 - I2C SS, FS or FM+ spike suppression limit This register is used to store th...
    pub ic_fs_spklen: IcFsSpklen,
    _reserved2: [u8; 4],
    /// 0x00a8 - Clear RESTART_DET Interrupt Register
    pub ic_clr_restart_det: IcClrRestartDet,
    _reserved3: [u8; 72],
    /// 0x00f4 - Component Parameter Register 1 Note This register is not implemented and th...
    pub ic_comp_param_1: IcCompParam1,
    /// 0x00f8 - I2C Component Version Register
    pub ic_comp_version: IcCompVersion,
    /// 0x00fc - I2C Component Type Register
    pub ic_comp_type: IcCompType,
}

