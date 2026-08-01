//! I2C0
//! DW_apb_i2c address block
//!
//! This peripheral supports atomic SET/CLR/XOR register aliases.

#![allow(dead_code)]

pub const BASE: u32 = 0x40090000;

/// IC_CON Register
///
/// I2C Control Register. This register can be written only when the DW_apb_i2c is disabled, which corresponds to the IC_ENABLE[0] register being set to 0. Writes at other times have no effect.
/// Read/Write Access: - bit 10 is read only. - bit 11 is read only - bit 16 is read only - bit 17 is read only - bits 18 and 19 are read only.
///
/// Address offset: 0x0000
/// Reset value: 0x00000065
///
/// ## Fields
///
/// - **STOP_DET_IF_MASTER_ACTIVE** `[10]` (RO)
///
///   Master issues the STOP_DET interrupt irrespective of whether master is active or not
///
/// - **RX_FIFO_FULL_HLD_CTRL** `[9]` (RW)
///
///   This bit controls whether DW_apb_i2c should hold the bus when the Rx FIFO is physically full to its RX_BUFFER_DEPTH, as described in the IC_RX_FULL_HLD_BUS_EN parameter.
///   Reset value: 0x0.
///
/// - **TX_EMPTY_CTRL** `[8]` (RW)
///
///   This bit controls the generation of the TX_EMPTY interrupt, as described in the IC_RAW_INTR_STAT register.
///   Reset value: 0x0.
///
/// - **STOP_DET_IFADDRESSED** `[7]` (RW)
///
///   In slave mode: - 1'b1: issues the STOP_DET interrupt only when it is addressed. - 1'b0: issues the STOP_DET irrespective of whether it's addressed or not. Reset value: 0x0
///   NOTE: During a general call address, this slave does not issue the STOP_DET interrupt if STOP_DET_IF_ADDRESSED = 1'b1, even if the slave responds to the general call address by generating ACK. The STOP_DET interrupt is generated only when the transmitted address matches the slave address (SAR).
///
/// - **IC_SLAVE_DISABLE** `[6]` (RW)
///
///   This bit controls whether I2C has its slave disabled, which means once the presetn signal is applied, then this bit is set and the slave is disabled.
///   If this bit is set (slave is disabled), DW_apb_i2c functions only as a master and does not perform any action that requires a slave.
///   NOTE: Software should ensure that if this bit is written with 0, then bit 0 should also be written with a 0.
///
/// - **IC_RESTART_EN** `[5]` (RW)
///
///   Determines whether RESTART conditions may be sent when acting as a master. Some older slaves do not support handling RESTART conditions; however, RESTART conditions are used in several DW_apb_i2c operations. When RESTART is disabled, the master is prohibited from performing the following functions: - Sending a START BYTE - Performing any high-speed mode operation - High-speed mode operation - Performing direction changes in combined format mode - Performing a read operation with a 10-bit address By replacing RESTART condition followed by a STOP and a subsequent START condition, split operations are broken down into multiple DW_apb_i2c transfers. If the above operations are performed, it will result in setting bit 6 (TX_ABRT) of the IC_RAW_INTR_STAT register.
///   Reset value: ENABLED
///
/// - **IC_10BITADDR_MASTER** `[4]` (RW)
///
///   Controls whether the DW_apb_i2c starts its transfers in 7- or 10-bit addressing mode when acting as a master. - 0: 7-bit addressing - 1: 10-bit addressing
///
/// - **IC_10BITADDR_SLAVE** `[3]` (RW)
///
///   When acting as a slave, this bit controls whether the DW_apb_i2c responds to 7- or 10-bit addresses. - 0: 7-bit addressing. The DW_apb_i2c ignores transactions that involve 10-bit addressing; for 7-bit addressing, only the lower 7 bits of the IC_SAR register are compared. - 1: 10-bit addressing. The DW_apb_i2c responds to only 10-bit addressing transfers that match the full 10 bits of the IC_SAR register.
///
/// - **SPEED** `[2:1]` (RW)
///
///   These bits control at which speed the DW_apb_i2c operates; its setting is relevant only if one is operating the DW_apb_i2c in master mode. Hardware protects against illegal values being programmed by software. These bits must be programmed appropriately for slave mode also, as it is used to capture correct value of spike filter as per the speed mode.
///   This register should be programmed only with a value in the range of 1 to IC_MAX_SPEED_MODE; otherwise, hardware updates this register with the value of IC_MAX_SPEED_MODE.
///   1: standard mode (100 kbit/s)
///   2: fast mode (<=400 kbit/s) or fast mode plus (<=1000Kbit/s)
///   3: high speed mode (3.4 Mbit/s)
///   Note: This field is not applicable when IC_ULTRA_FAST_MODE=1
///
/// - **MASTER_MODE** `[0]` (RW)
///
///   This bit controls whether the DW_apb_i2c master is enabled.
///   NOTE: Software should ensure that if this bit is written with '1' then bit 6 should also be written with a '1'.
#[repr(transparent)]
pub struct IcCon {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for IcCon {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl IcCon {
    // ======== Field Constants ========

    /// **STOP_DET_IF_MASTER_ACTIVE**
    ///
    /// Master issues the STOP_DET interrupt irrespective of whether master is active or not
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const STOP_DET_IF_MASTER_ACTIVE_OFFSET: u32 = 10;
    pub const STOP_DET_IF_MASTER_ACTIVE_MASK: u32 = 0x00000400;
    pub const STOP_DET_IF_MASTER_ACTIVE: u32 = 1 << 10;

    /// **RX_FIFO_FULL_HLD_CTRL**
    ///
    /// This bit controls whether DW_apb_i2c should hold the bus when the Rx FIFO is physically full to its RX_BUFFER_DEPTH, as described in the IC_RX_FULL_HLD_BUS_EN parameter.
    /// Reset value: 0x0.
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const RX_FIFO_FULL_HLD_CTRL_OFFSET: u32 = 9;
    pub const RX_FIFO_FULL_HLD_CTRL_MASK: u32 = 0x00000200;
    pub const RX_FIFO_FULL_HLD_CTRL: u32 = 1 << 9;
    /// DISABLED = 0
    /// Overflow when RX_FIFO is full
    pub const RX_FIFO_FULL_HLD_CTRL_DISABLED: u32 = 0 << 9;
    pub const RX_FIFO_FULL_HLD_CTRL_DISABLED_VAL: u32 = 0;
    /// ENABLED = 1
    /// Hold bus when RX_FIFO is full
    pub const RX_FIFO_FULL_HLD_CTRL_ENABLED: u32 = 1 << 9;
    pub const RX_FIFO_FULL_HLD_CTRL_ENABLED_VAL: u32 = 1;

    /// **TX_EMPTY_CTRL**
    ///
    /// This bit controls the generation of the TX_EMPTY interrupt, as described in the IC_RAW_INTR_STAT register.
    /// Reset value: 0x0.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const TX_EMPTY_CTRL_OFFSET: u32 = 8;
    pub const TX_EMPTY_CTRL_MASK: u32 = 0x00000100;
    pub const TX_EMPTY_CTRL: u32 = 1 << 8;
    /// DISABLED = 0
    /// Default behaviour of TX_EMPTY interrupt
    pub const TX_EMPTY_CTRL_DISABLED: u32 = 0 << 8;
    pub const TX_EMPTY_CTRL_DISABLED_VAL: u32 = 0;
    /// ENABLED = 1
    /// Controlled generation of TX_EMPTY interrupt
    pub const TX_EMPTY_CTRL_ENABLED: u32 = 1 << 8;
    pub const TX_EMPTY_CTRL_ENABLED_VAL: u32 = 1;

    /// **STOP_DET_IFADDRESSED**
    ///
    /// In slave mode: - 1'b1: issues the STOP_DET interrupt only when it is addressed. - 1'b0: issues the STOP_DET irrespective of whether it's addressed or not. Reset value: 0x0
    /// NOTE: During a general call address, this slave does not issue the STOP_DET interrupt if STOP_DET_IF_ADDRESSED = 1'b1, even if the slave responds to the general call address by generating ACK. The STOP_DET interrupt is generated only when the transmitted address matches the slave address (SAR).
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const STOP_DET_IFADDRESSED_OFFSET: u32 = 7;
    pub const STOP_DET_IFADDRESSED_MASK: u32 = 0x00000080;
    pub const STOP_DET_IFADDRESSED: u32 = 1 << 7;
    /// DISABLED = 0
    /// slave issues STOP_DET intr always
    pub const STOP_DET_IFADDRESSED_DISABLED: u32 = 0 << 7;
    pub const STOP_DET_IFADDRESSED_DISABLED_VAL: u32 = 0;
    /// ENABLED = 1
    /// slave issues STOP_DET intr only if addressed
    pub const STOP_DET_IFADDRESSED_ENABLED: u32 = 1 << 7;
    pub const STOP_DET_IFADDRESSED_ENABLED_VAL: u32 = 1;

    /// **IC_SLAVE_DISABLE**
    ///
    /// This bit controls whether I2C has its slave disabled, which means once the presetn signal is applied, then this bit is set and the slave is disabled.
    /// If this bit is set (slave is disabled), DW_apb_i2c functions only as a master and does not perform any action that requires a slave.
    /// NOTE: Software should ensure that if this bit is written with 0, then bit 0 should also be written with a 0.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const IC_SLAVE_DISABLE_OFFSET: u32 = 6;
    pub const IC_SLAVE_DISABLE_MASK: u32 = 0x00000040;
    pub const IC_SLAVE_DISABLE: u32 = 1 << 6;
    /// SLAVE_ENABLED = 0
    /// Slave mode is enabled
    pub const IC_SLAVE_DISABLE_SLAVE_ENABLED: u32 = 0 << 6;
    pub const IC_SLAVE_DISABLE_SLAVE_ENABLED_VAL: u32 = 0;
    /// SLAVE_DISABLED = 1
    /// Slave mode is disabled
    pub const IC_SLAVE_DISABLE_SLAVE_DISABLED: u32 = 1 << 6;
    pub const IC_SLAVE_DISABLE_SLAVE_DISABLED_VAL: u32 = 1;

    /// **IC_RESTART_EN**
    ///
    /// Determines whether RESTART conditions may be sent when acting as a master. Some older slaves do not support handling RESTART conditions; however, RESTART conditions are used in several DW_apb_i2c operations. When RESTART is disabled, the master is prohibited from performing the following functions: - Sending a START BYTE - Performing any high-speed mode operation - High-speed mode operation - Performing direction changes in combined format mode - Performing a read operation with a 10-bit address By replacing RESTART condition followed by a STOP and a subsequent START condition, split operations are broken down into multiple DW_apb_i2c transfers. If the above operations are performed, it will result in setting bit 6 (TX_ABRT) of the IC_RAW_INTR_STAT register.
    /// Reset value: ENABLED
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const IC_RESTART_EN_OFFSET: u32 = 5;
    pub const IC_RESTART_EN_MASK: u32 = 0x00000020;
    pub const IC_RESTART_EN: u32 = 1 << 5;
    /// DISABLED = 0
    /// Master restart disabled
    pub const IC_RESTART_EN_DISABLED: u32 = 0 << 5;
    pub const IC_RESTART_EN_DISABLED_VAL: u32 = 0;
    /// ENABLED = 1
    /// Master restart enabled
    pub const IC_RESTART_EN_ENABLED: u32 = 1 << 5;
    pub const IC_RESTART_EN_ENABLED_VAL: u32 = 1;

    /// **IC_10BITADDR_MASTER**
    ///
    /// Controls whether the DW_apb_i2c starts its transfers in 7- or 10-bit addressing mode when acting as a master. - 0: 7-bit addressing - 1: 10-bit addressing
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const IC_10BITADDR_MASTER_OFFSET: u32 = 4;
    pub const IC_10BITADDR_MASTER_MASK: u32 = 0x00000010;
    pub const IC_10BITADDR_MASTER: u32 = 1 << 4;
    /// ADDR_7BITS = 0
    /// Master 7Bit addressing mode
    pub const IC_10BITADDR_MASTER_ADDR_7BITS: u32 = 0 << 4;
    pub const IC_10BITADDR_MASTER_ADDR_7BITS_VAL: u32 = 0;
    /// ADDR_10BITS = 1
    /// Master 10Bit addressing mode
    pub const IC_10BITADDR_MASTER_ADDR_10BITS: u32 = 1 << 4;
    pub const IC_10BITADDR_MASTER_ADDR_10BITS_VAL: u32 = 1;

    /// **IC_10BITADDR_SLAVE**
    ///
    /// When acting as a slave, this bit controls whether the DW_apb_i2c responds to 7- or 10-bit addresses. - 0: 7-bit addressing. The DW_apb_i2c ignores transactions that involve 10-bit addressing; for 7-bit addressing, only the lower 7 bits of the IC_SAR register are compared. - 1: 10-bit addressing. The DW_apb_i2c responds to only 10-bit addressing transfers that match the full 10 bits of the IC_SAR register.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const IC_10BITADDR_SLAVE_OFFSET: u32 = 3;
    pub const IC_10BITADDR_SLAVE_MASK: u32 = 0x00000008;
    pub const IC_10BITADDR_SLAVE: u32 = 1 << 3;
    /// ADDR_7BITS = 0
    /// Slave 7Bit addressing
    pub const IC_10BITADDR_SLAVE_ADDR_7BITS: u32 = 0 << 3;
    pub const IC_10BITADDR_SLAVE_ADDR_7BITS_VAL: u32 = 0;
    /// ADDR_10BITS = 1
    /// Slave 10Bit addressing
    pub const IC_10BITADDR_SLAVE_ADDR_10BITS: u32 = 1 << 3;
    pub const IC_10BITADDR_SLAVE_ADDR_10BITS_VAL: u32 = 1;

    /// **SPEED**
    ///
    /// These bits control at which speed the DW_apb_i2c operates; its setting is relevant only if one is operating the DW_apb_i2c in master mode. Hardware protects against illegal values being programmed by software. These bits must be programmed appropriately for slave mode also, as it is used to capture correct value of spike filter as per the speed mode.
    /// This register should be programmed only with a value in the range of 1 to IC_MAX_SPEED_MODE; otherwise, hardware updates this register with the value of IC_MAX_SPEED_MODE.
    /// 1: standard mode (100 kbit/s)
    /// 2: fast mode (<=400 kbit/s) or fast mode plus (<=1000Kbit/s)
    /// 3: high speed mode (3.4 Mbit/s)
    /// Note: This field is not applicable when IC_ULTRA_FAST_MODE=1
    ///
    /// Bits `[2:1]`, mask `0x00000006`
    pub const SPEED_OFFSET: u32 = 1;
    pub const SPEED_MASK: u32 = 0x00000006;
    /// STANDARD = 1
    /// Standard Speed mode of operation
    pub const SPEED_STANDARD: u32 = 1 << 1;
    pub const SPEED_STANDARD_VAL: u32 = 1;
    /// FAST = 2
    /// Fast or Fast Plus mode of operation
    pub const SPEED_FAST: u32 = 2 << 1;
    pub const SPEED_FAST_VAL: u32 = 2;
    /// HIGH = 3
    /// High Speed mode of operation
    pub const SPEED_HIGH: u32 = 3 << 1;
    pub const SPEED_HIGH_VAL: u32 = 3;

    /// **MASTER_MODE**
    ///
    /// This bit controls whether the DW_apb_i2c master is enabled.
    /// NOTE: Software should ensure that if this bit is written with '1' then bit 6 should also be written with a '1'.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const MASTER_MODE_OFFSET: u32 = 0;
    pub const MASTER_MODE_MASK: u32 = 0x00000001;
    pub const MASTER_MODE: u32 = 1 << 0;
    /// DISABLED = 0
    /// Master mode is disabled
    pub const MASTER_MODE_DISABLED: u32 = 0 << 0;
    pub const MASTER_MODE_DISABLED_VAL: u32 = 0;
    /// ENABLED = 1
    /// Master mode is enabled
    pub const MASTER_MODE_ENABLED: u32 = 1 << 0;
    pub const MASTER_MODE_ENABLED_VAL: u32 = 1;

    // ======== Field Accessors ========

    /// **STOP_DET_IF_MASTER_ACTIVE** `[10]` (RO)
    ///
    /// Master issues the STOP_DET interrupt irrespective of whether master is active or not
    #[inline(always)]
    pub fn stop_det_if_master_active(&self) -> bool {
        (self.value.read_bits() & Self::STOP_DET_IF_MASTER_ACTIVE_MASK) != 0
    }

    /// **RX_FIFO_FULL_HLD_CTRL** `[9]` (RW)
    ///
    /// This bit controls whether DW_apb_i2c should hold the bus when the Rx FIFO is physically full to its RX_BUFFER_DEPTH, as described in the IC_RX_FULL_HLD_BUS_EN parameter.
    /// Reset value: 0x0.
    #[inline(always)]
    pub fn rx_fifo_full_hld_ctrl(&self) -> bool {
        (self.value.read_bits() & Self::RX_FIFO_FULL_HLD_CTRL_MASK) != 0
    }

    /// Set RX_FIFO_FULL_HLD_CTRL bit atomically (RW)
    #[inline(always)]
    pub fn set_rx_fifo_full_hld_ctrl(&self) {
        self.set_bits(Self::RX_FIFO_FULL_HLD_CTRL_MASK);
    }

    /// Clear RX_FIFO_FULL_HLD_CTRL bit atomically (RW)
    #[inline(always)]
    pub fn clr_rx_fifo_full_hld_ctrl(&self) {
        self.clr_bits(Self::RX_FIFO_FULL_HLD_CTRL_MASK);
    }

    /// **TX_EMPTY_CTRL** `[8]` (RW)
    ///
    /// This bit controls the generation of the TX_EMPTY interrupt, as described in the IC_RAW_INTR_STAT register.
    /// Reset value: 0x0.
    #[inline(always)]
    pub fn tx_empty_ctrl(&self) -> bool {
        (self.value.read_bits() & Self::TX_EMPTY_CTRL_MASK) != 0
    }

    /// Set TX_EMPTY_CTRL bit atomically (RW)
    #[inline(always)]
    pub fn set_tx_empty_ctrl(&self) {
        self.set_bits(Self::TX_EMPTY_CTRL_MASK);
    }

    /// Clear TX_EMPTY_CTRL bit atomically (RW)
    #[inline(always)]
    pub fn clr_tx_empty_ctrl(&self) {
        self.clr_bits(Self::TX_EMPTY_CTRL_MASK);
    }

    /// **STOP_DET_IFADDRESSED** `[7]` (RW)
    ///
    /// In slave mode: - 1'b1: issues the STOP_DET interrupt only when it is addressed. - 1'b0: issues the STOP_DET irrespective of whether it's addressed or not. Reset value: 0x0
    /// NOTE: During a general call address, this slave does not issue the STOP_DET interrupt if STOP_DET_IF_ADDRESSED = 1'b1, even if the slave responds to the general call address by generating ACK. The STOP_DET interrupt is generated only when the transmitted address matches the slave address (SAR).
    #[inline(always)]
    pub fn stop_det_ifaddressed(&self) -> bool {
        (self.value.read_bits() & Self::STOP_DET_IFADDRESSED_MASK) != 0
    }

    /// Set STOP_DET_IFADDRESSED bit atomically (RW)
    #[inline(always)]
    pub fn set_stop_det_ifaddressed(&self) {
        self.set_bits(Self::STOP_DET_IFADDRESSED_MASK);
    }

    /// Clear STOP_DET_IFADDRESSED bit atomically (RW)
    #[inline(always)]
    pub fn clr_stop_det_ifaddressed(&self) {
        self.clr_bits(Self::STOP_DET_IFADDRESSED_MASK);
    }

    /// **IC_SLAVE_DISABLE** `[6]` (RW)
    ///
    /// This bit controls whether I2C has its slave disabled, which means once the presetn signal is applied, then this bit is set and the slave is disabled.
    /// If this bit is set (slave is disabled), DW_apb_i2c functions only as a master and does not perform any action that requires a slave.
    /// NOTE: Software should ensure that if this bit is written with 0, then bit 0 should also be written with a 0.
    #[inline(always)]
    pub fn ic_slave_disable(&self) -> bool {
        (self.value.read_bits() & Self::IC_SLAVE_DISABLE_MASK) != 0
    }

    /// Set IC_SLAVE_DISABLE bit atomically (RW)
    #[inline(always)]
    pub fn set_ic_slave_disable(&self) {
        self.set_bits(Self::IC_SLAVE_DISABLE_MASK);
    }

    /// Clear IC_SLAVE_DISABLE bit atomically (RW)
    #[inline(always)]
    pub fn clr_ic_slave_disable(&self) {
        self.clr_bits(Self::IC_SLAVE_DISABLE_MASK);
    }

    /// **IC_RESTART_EN** `[5]` (RW)
    ///
    /// Determines whether RESTART conditions may be sent when acting as a master. Some older slaves do not support handling RESTART conditions; however, RESTART conditions are used in several DW_apb_i2c operations. When RESTART is disabled, the master is prohibited from performing the following functions: - Sending a START BYTE - Performing any high-speed mode operation - High-speed mode operation - Performing direction changes in combined format mode - Performing a read operation with a 10-bit address By replacing RESTART condition followed by a STOP and a subsequent START condition, split operations are broken down into multiple DW_apb_i2c transfers. If the above operations are performed, it will result in setting bit 6 (TX_ABRT) of the IC_RAW_INTR_STAT register.
    /// Reset value: ENABLED
    #[inline(always)]
    pub fn ic_restart_en(&self) -> bool {
        (self.value.read_bits() & Self::IC_RESTART_EN_MASK) != 0
    }

    /// Set IC_RESTART_EN bit atomically (RW)
    #[inline(always)]
    pub fn set_ic_restart_en(&self) {
        self.set_bits(Self::IC_RESTART_EN_MASK);
    }

    /// Clear IC_RESTART_EN bit atomically (RW)
    #[inline(always)]
    pub fn clr_ic_restart_en(&self) {
        self.clr_bits(Self::IC_RESTART_EN_MASK);
    }

    /// **IC_10BITADDR_MASTER** `[4]` (RW)
    ///
    /// Controls whether the DW_apb_i2c starts its transfers in 7- or 10-bit addressing mode when acting as a master. - 0: 7-bit addressing - 1: 10-bit addressing
    #[inline(always)]
    pub fn ic_10bitaddr_master(&self) -> bool {
        (self.value.read_bits() & Self::IC_10BITADDR_MASTER_MASK) != 0
    }

    /// Set IC_10BITADDR_MASTER bit atomically (RW)
    #[inline(always)]
    pub fn set_ic_10bitaddr_master(&self) {
        self.set_bits(Self::IC_10BITADDR_MASTER_MASK);
    }

    /// Clear IC_10BITADDR_MASTER bit atomically (RW)
    #[inline(always)]
    pub fn clr_ic_10bitaddr_master(&self) {
        self.clr_bits(Self::IC_10BITADDR_MASTER_MASK);
    }

    /// **IC_10BITADDR_SLAVE** `[3]` (RW)
    ///
    /// When acting as a slave, this bit controls whether the DW_apb_i2c responds to 7- or 10-bit addresses. - 0: 7-bit addressing. The DW_apb_i2c ignores transactions that involve 10-bit addressing; for 7-bit addressing, only the lower 7 bits of the IC_SAR register are compared. - 1: 10-bit addressing. The DW_apb_i2c responds to only 10-bit addressing transfers that match the full 10 bits of the IC_SAR register.
    #[inline(always)]
    pub fn ic_10bitaddr_slave(&self) -> bool {
        (self.value.read_bits() & Self::IC_10BITADDR_SLAVE_MASK) != 0
    }

    /// Set IC_10BITADDR_SLAVE bit atomically (RW)
    #[inline(always)]
    pub fn set_ic_10bitaddr_slave(&self) {
        self.set_bits(Self::IC_10BITADDR_SLAVE_MASK);
    }

    /// Clear IC_10BITADDR_SLAVE bit atomically (RW)
    #[inline(always)]
    pub fn clr_ic_10bitaddr_slave(&self) {
        self.clr_bits(Self::IC_10BITADDR_SLAVE_MASK);
    }

    /// **SPEED** `[2:1]` (RW)
    ///
    /// These bits control at which speed the DW_apb_i2c operates; its setting is relevant only if one is operating the DW_apb_i2c in master mode. Hardware protects against illegal values being programmed by software. These bits must be programmed appropriately for slave mode also, as it is used to capture correct value of spike filter as per the speed mode.
    /// This register should be programmed only with a value in the range of 1 to IC_MAX_SPEED_MODE; otherwise, hardware updates this register with the value of IC_MAX_SPEED_MODE.
    /// 1: standard mode (100 kbit/s)
    /// 2: fast mode (<=400 kbit/s) or fast mode plus (<=1000Kbit/s)
    /// 3: high speed mode (3.4 Mbit/s)
    /// Note: This field is not applicable when IC_ULTRA_FAST_MODE=1
    #[inline(always)]
    pub fn speed(&self) -> u32 {
        (self.value.read_bits() & Self::SPEED_MASK) >> Self::SPEED_OFFSET
    }

    /// Write **SPEED** `[2:1]` (RW)
    ///
    /// These bits control at which speed the DW_apb_i2c operates; its setting is relevant only if one is operating the DW_apb_i2c in master mode. Hardware protects against illegal values being programmed by software. These bits must be programmed appropriately for slave mode also, as it is used to capture correct value of spike filter as per the speed mode.
    /// This register should be programmed only with a value in the range of 1 to IC_MAX_SPEED_MODE; otherwise, hardware updates this register with the value of IC_MAX_SPEED_MODE.
    /// 1: standard mode (100 kbit/s)
    /// 2: fast mode (<=400 kbit/s) or fast mode plus (<=1000Kbit/s)
    /// 3: high speed mode (3.4 Mbit/s)
    /// Note: This field is not applicable when IC_ULTRA_FAST_MODE=1
    #[inline(always)]
    pub fn write_speed(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPEED_MASK) | ((value << Self::SPEED_OFFSET) & Self::SPEED_MASK)
        });
    }

    /// **MASTER_MODE** `[0]` (RW)
    ///
    /// This bit controls whether the DW_apb_i2c master is enabled.
    /// NOTE: Software should ensure that if this bit is written with '1' then bit 6 should also be written with a '1'.
    #[inline(always)]
    pub fn master_mode(&self) -> bool {
        (self.value.read_bits() & Self::MASTER_MODE_MASK) != 0
    }

    /// Set MASTER_MODE bit atomically (RW)
    #[inline(always)]
    pub fn set_master_mode(&self) {
        self.set_bits(Self::MASTER_MODE_MASK);
    }

    /// Clear MASTER_MODE bit atomically (RW)
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

    /// Read **STOP_DET_IF_MASTER_ACTIVE** `[10:10]`
    ///
    /// Master issues the STOP_DET interrupt irrespective of whether master is active or not
    #[inline(always)]
    pub const fn stop_det_if_master_active(&self) -> bool {
        (self.bits & IcCon::STOP_DET_IF_MASTER_ACTIVE_MASK) != 0
    }

    /// Read **RX_FIFO_FULL_HLD_CTRL** `[9:9]`
    ///
    /// This bit controls whether DW_apb_i2c should hold the bus when the Rx FIFO is physically full to its RX_BUFFER_DEPTH, as described in the IC_RX_FULL_HLD_BUS_EN parameter.
    /// Reset value: 0x0.
    #[inline(always)]
    pub const fn rx_fifo_full_hld_ctrl(&self) -> bool {
        (self.bits & IcCon::RX_FIFO_FULL_HLD_CTRL_MASK) != 0
    }

    /// Read **TX_EMPTY_CTRL** `[8:8]`
    ///
    /// This bit controls the generation of the TX_EMPTY interrupt, as described in the IC_RAW_INTR_STAT register.
    /// Reset value: 0x0.
    #[inline(always)]
    pub const fn tx_empty_ctrl(&self) -> bool {
        (self.bits & IcCon::TX_EMPTY_CTRL_MASK) != 0
    }

    /// Read **STOP_DET_IFADDRESSED** `[7:7]`
    ///
    /// In slave mode: - 1'b1: issues the STOP_DET interrupt only when it is addressed. - 1'b0: issues the STOP_DET irrespective of whether it's addressed or not. Reset value: 0x0
    /// NOTE: During a general call address, this slave does not issue the STOP_DET interrupt if STOP_DET_IF_ADDRESSED = 1'b1, even if the slave responds to the general call address by generating ACK. The STOP_DET interrupt is generated only when the transmitted address matches the slave address (SAR).
    #[inline(always)]
    pub const fn stop_det_ifaddressed(&self) -> bool {
        (self.bits & IcCon::STOP_DET_IFADDRESSED_MASK) != 0
    }

    /// Read **IC_SLAVE_DISABLE** `[6:6]`
    ///
    /// This bit controls whether I2C has its slave disabled, which means once the presetn signal is applied, then this bit is set and the slave is disabled.
    /// If this bit is set (slave is disabled), DW_apb_i2c functions only as a master and does not perform any action that requires a slave.
    /// NOTE: Software should ensure that if this bit is written with 0, then bit 0 should also be written with a 0.
    #[inline(always)]
    pub const fn ic_slave_disable(&self) -> bool {
        (self.bits & IcCon::IC_SLAVE_DISABLE_MASK) != 0
    }

    /// Read **IC_RESTART_EN** `[5:5]`
    ///
    /// Determines whether RESTART conditions may be sent when acting as a master. Some older slaves do not support handling RESTART conditions; however, RESTART conditions are used in several DW_apb_i2c operations. When RESTART is disabled, the master is prohibited from performing the following functions: - Sending a START BYTE - Performing any high-speed mode operation - High-speed mode operation - Performing direction changes in combined format mode - Performing a read operation with a 10-bit address By replacing RESTART condition followed by a STOP and a subsequent START condition, split operations are broken down into multiple DW_apb_i2c transfers. If the above operations are performed, it will result in setting bit 6 (TX_ABRT) of the IC_RAW_INTR_STAT register.
    /// Reset value: ENABLED
    #[inline(always)]
    pub const fn ic_restart_en(&self) -> bool {
        (self.bits & IcCon::IC_RESTART_EN_MASK) != 0
    }

    /// Read **IC_10BITADDR_MASTER** `[4:4]`
    ///
    /// Controls whether the DW_apb_i2c starts its transfers in 7- or 10-bit addressing mode when acting as a master. - 0: 7-bit addressing - 1: 10-bit addressing
    #[inline(always)]
    pub const fn ic_10bitaddr_master(&self) -> bool {
        (self.bits & IcCon::IC_10BITADDR_MASTER_MASK) != 0
    }

    /// Read **IC_10BITADDR_SLAVE** `[3:3]`
    ///
    /// When acting as a slave, this bit controls whether the DW_apb_i2c responds to 7- or 10-bit addresses. - 0: 7-bit addressing. The DW_apb_i2c ignores transactions that involve 10-bit addressing; for 7-bit addressing, only the lower 7 bits of the IC_SAR register are compared. - 1: 10-bit addressing. The DW_apb_i2c responds to only 10-bit addressing transfers that match the full 10 bits of the IC_SAR register.
    #[inline(always)]
    pub const fn ic_10bitaddr_slave(&self) -> bool {
        (self.bits & IcCon::IC_10BITADDR_SLAVE_MASK) != 0
    }

    /// Read **SPEED** `[2:1]`
    ///
    /// These bits control at which speed the DW_apb_i2c operates; its setting is relevant only if one is operating the DW_apb_i2c in master mode. Hardware protects against illegal values being programmed by software. These bits must be programmed appropriately for slave mode also, as it is used to capture correct value of spike filter as per the speed mode.
    /// This register should be programmed only with a value in the range of 1 to IC_MAX_SPEED_MODE; otherwise, hardware updates this register with the value of IC_MAX_SPEED_MODE.
    /// 1: standard mode (100 kbit/s)
    /// 2: fast mode (<=400 kbit/s) or fast mode plus (<=1000Kbit/s)
    /// 3: high speed mode (3.4 Mbit/s)
    /// Note: This field is not applicable when IC_ULTRA_FAST_MODE=1
    #[inline(always)]
    pub const fn speed(&self) -> u32 {
        (self.bits & IcCon::SPEED_MASK) >> IcCon::SPEED_OFFSET
    }

    /// Read **MASTER_MODE** `[0:0]`
    ///
    /// This bit controls whether the DW_apb_i2c master is enabled.
    /// NOTE: Software should ensure that if this bit is written with '1' then bit 6 should also be written with a '1'.
    #[inline(always)]
    pub const fn master_mode(&self) -> bool {
        (self.bits & IcCon::MASTER_MODE_MASK) != 0
    }

    /// Set **RX_FIFO_FULL_HLD_CTRL** `[9:9]`
    ///
    /// This bit controls whether DW_apb_i2c should hold the bus when the Rx FIFO is physically full to its RX_BUFFER_DEPTH, as described in the IC_RX_FULL_HLD_BUS_EN parameter.
    /// Reset value: 0x0.
    #[inline(always)]
    pub const fn with_rx_fifo_full_hld_ctrl(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IcCon::RX_FIFO_FULL_HLD_CTRL_MASK
            } else {
                self.bits & !IcCon::RX_FIFO_FULL_HLD_CTRL_MASK
            },
        }
    }

    /// Set **TX_EMPTY_CTRL** `[8:8]`
    ///
    /// This bit controls the generation of the TX_EMPTY interrupt, as described in the IC_RAW_INTR_STAT register.
    /// Reset value: 0x0.
    #[inline(always)]
    pub const fn with_tx_empty_ctrl(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IcCon::TX_EMPTY_CTRL_MASK
            } else {
                self.bits & !IcCon::TX_EMPTY_CTRL_MASK
            },
        }
    }

    /// Set **STOP_DET_IFADDRESSED** `[7:7]`
    ///
    /// In slave mode: - 1'b1: issues the STOP_DET interrupt only when it is addressed. - 1'b0: issues the STOP_DET irrespective of whether it's addressed or not. Reset value: 0x0
    /// NOTE: During a general call address, this slave does not issue the STOP_DET interrupt if STOP_DET_IF_ADDRESSED = 1'b1, even if the slave responds to the general call address by generating ACK. The STOP_DET interrupt is generated only when the transmitted address matches the slave address (SAR).
    #[inline(always)]
    pub const fn with_stop_det_ifaddressed(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IcCon::STOP_DET_IFADDRESSED_MASK
            } else {
                self.bits & !IcCon::STOP_DET_IFADDRESSED_MASK
            },
        }
    }

    /// Set **IC_SLAVE_DISABLE** `[6:6]`
    ///
    /// This bit controls whether I2C has its slave disabled, which means once the presetn signal is applied, then this bit is set and the slave is disabled.
    /// If this bit is set (slave is disabled), DW_apb_i2c functions only as a master and does not perform any action that requires a slave.
    /// NOTE: Software should ensure that if this bit is written with 0, then bit 0 should also be written with a 0.
    #[inline(always)]
    pub const fn with_ic_slave_disable(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IcCon::IC_SLAVE_DISABLE_MASK
            } else {
                self.bits & !IcCon::IC_SLAVE_DISABLE_MASK
            },
        }
    }

    /// Set **IC_RESTART_EN** `[5:5]`
    ///
    /// Determines whether RESTART conditions may be sent when acting as a master. Some older slaves do not support handling RESTART conditions; however, RESTART conditions are used in several DW_apb_i2c operations. When RESTART is disabled, the master is prohibited from performing the following functions: - Sending a START BYTE - Performing any high-speed mode operation - High-speed mode operation - Performing direction changes in combined format mode - Performing a read operation with a 10-bit address By replacing RESTART condition followed by a STOP and a subsequent START condition, split operations are broken down into multiple DW_apb_i2c transfers. If the above operations are performed, it will result in setting bit 6 (TX_ABRT) of the IC_RAW_INTR_STAT register.
    /// Reset value: ENABLED
    #[inline(always)]
    pub const fn with_ic_restart_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IcCon::IC_RESTART_EN_MASK
            } else {
                self.bits & !IcCon::IC_RESTART_EN_MASK
            },
        }
    }

    /// Set **IC_10BITADDR_MASTER** `[4:4]`
    ///
    /// Controls whether the DW_apb_i2c starts its transfers in 7- or 10-bit addressing mode when acting as a master. - 0: 7-bit addressing - 1: 10-bit addressing
    #[inline(always)]
    pub const fn with_ic_10bitaddr_master(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IcCon::IC_10BITADDR_MASTER_MASK
            } else {
                self.bits & !IcCon::IC_10BITADDR_MASTER_MASK
            },
        }
    }

    /// Set **IC_10BITADDR_SLAVE** `[3:3]`
    ///
    /// When acting as a slave, this bit controls whether the DW_apb_i2c responds to 7- or 10-bit addresses. - 0: 7-bit addressing. The DW_apb_i2c ignores transactions that involve 10-bit addressing; for 7-bit addressing, only the lower 7 bits of the IC_SAR register are compared. - 1: 10-bit addressing. The DW_apb_i2c responds to only 10-bit addressing transfers that match the full 10 bits of the IC_SAR register.
    #[inline(always)]
    pub const fn with_ic_10bitaddr_slave(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IcCon::IC_10BITADDR_SLAVE_MASK
            } else {
                self.bits & !IcCon::IC_10BITADDR_SLAVE_MASK
            },
        }
    }

    /// Set **SPEED** `[2:1]`
    ///
    /// These bits control at which speed the DW_apb_i2c operates; its setting is relevant only if one is operating the DW_apb_i2c in master mode. Hardware protects against illegal values being programmed by software. These bits must be programmed appropriately for slave mode also, as it is used to capture correct value of spike filter as per the speed mode.
    /// This register should be programmed only with a value in the range of 1 to IC_MAX_SPEED_MODE; otherwise, hardware updates this register with the value of IC_MAX_SPEED_MODE.
    /// 1: standard mode (100 kbit/s)
    /// 2: fast mode (<=400 kbit/s) or fast mode plus (<=1000Kbit/s)
    /// 3: high speed mode (3.4 Mbit/s)
    /// Note: This field is not applicable when IC_ULTRA_FAST_MODE=1
    #[inline(always)]
    pub const fn with_speed(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !IcCon::SPEED_MASK)
                | ((value << IcCon::SPEED_OFFSET) & IcCon::SPEED_MASK),
        }
    }

    /// Set **MASTER_MODE** `[0:0]`
    ///
    /// This bit controls whether the DW_apb_i2c master is enabled.
    /// NOTE: Software should ensure that if this bit is written with '1' then bit 6 should also be written with a '1'.
    #[inline(always)]
    pub const fn with_master_mode(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IcCon::MASTER_MODE_MASK
            } else {
                self.bits & !IcCon::MASTER_MODE_MASK
            },
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
    pub fn read_val(&self) -> IcConVal {
        IcConVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: IcConVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(IcConVal) -> IcConVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// IC_TAR Register
///
/// I2C Target Address Register
/// This register is 12 bits wide, and bits 31:12 are reserved. This register can be written to only when IC_ENABLE[0] is set to 0.
/// Note: If the software or application is aware that the DW_apb_i2c is not using the TAR address for the pending commands in the Tx FIFO, then it is possible to update the TAR address even while the Tx FIFO has entries (IC_STATUS[2]= 0). - It is not necessary to perform any write to this register if DW_apb_i2c is enabled as an I2C slave only.
///
/// Address offset: 0x0004
/// Reset value: 0x00000055
///
/// ## Fields
///
/// - **SPECIAL** `[11]` (RW)
///
///   This bit indicates whether software performs a Device-ID or General Call or START BYTE command. - 0: ignore bit 10 GC_OR_START and use IC_TAR normally - 1: perform special I2C command as specified in Device_ID or GC_OR_START bit Reset value: 0x0
///
/// - **GC_OR_START** `[10]` (RW)
///
///   If bit 11 (SPECIAL) is set to 1 and bit 13(Device-ID) is set to 0, then this bit indicates whether a General Call or START byte command is to be performed by the DW_apb_i2c. - 0: General Call Address - after issuing a General Call, only writes may be performed. Attempting to issue a read command results in setting bit 6 (TX_ABRT) of the IC_RAW_INTR_STAT register. The DW_apb_i2c remains in General Call mode until the SPECIAL bit value (bit 11) is cleared. - 1: START BYTE Reset value: 0x0
///
/// - **IC_TAR** `[9:0]` (RW)
///
///   This is the target address for any master transaction. When transmitting a General Call, these bits are ignored. To generate a START BYTE, the CPU needs to write only once into these bits.
///   If the IC_TAR and IC_SAR are the same, loopback exists but the FIFOs are shared between master and slave, so full loopback is not feasible. Only one direction loopback mode is supported (simplex), not duplex. A master cannot transmit to itself; it can transmit to only a slave.
#[repr(transparent)]
pub struct IcTar {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for IcTar {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl IcTar {
    // ======== Field Constants ========

    /// **SPECIAL**
    ///
    /// This bit indicates whether software performs a Device-ID or General Call or START BYTE command. - 0: ignore bit 10 GC_OR_START and use IC_TAR normally - 1: perform special I2C command as specified in Device_ID or GC_OR_START bit Reset value: 0x0
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const SPECIAL_OFFSET: u32 = 11;
    pub const SPECIAL_MASK: u32 = 0x00000800;
    pub const SPECIAL: u32 = 1 << 11;
    /// DISABLED = 0
    /// Disables programming of GENERAL_CALL or START_BYTE transmission
    pub const SPECIAL_DISABLED: u32 = 0 << 11;
    pub const SPECIAL_DISABLED_VAL: u32 = 0;
    /// ENABLED = 1
    /// Enables programming of GENERAL_CALL or START_BYTE transmission
    pub const SPECIAL_ENABLED: u32 = 1 << 11;
    pub const SPECIAL_ENABLED_VAL: u32 = 1;

    /// **GC_OR_START**
    ///
    /// If bit 11 (SPECIAL) is set to 1 and bit 13(Device-ID) is set to 0, then this bit indicates whether a General Call or START byte command is to be performed by the DW_apb_i2c. - 0: General Call Address - after issuing a General Call, only writes may be performed. Attempting to issue a read command results in setting bit 6 (TX_ABRT) of the IC_RAW_INTR_STAT register. The DW_apb_i2c remains in General Call mode until the SPECIAL bit value (bit 11) is cleared. - 1: START BYTE Reset value: 0x0
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const GC_OR_START_OFFSET: u32 = 10;
    pub const GC_OR_START_MASK: u32 = 0x00000400;
    pub const GC_OR_START: u32 = 1 << 10;
    /// GENERAL_CALL = 0
    /// GENERAL_CALL byte transmission
    pub const GC_OR_START_GENERAL_CALL: u32 = 0 << 10;
    pub const GC_OR_START_GENERAL_CALL_VAL: u32 = 0;
    /// START_BYTE = 1
    /// START byte transmission
    pub const GC_OR_START_START_BYTE: u32 = 1 << 10;
    pub const GC_OR_START_START_BYTE_VAL: u32 = 1;

    /// **IC_TAR**
    ///
    /// This is the target address for any master transaction. When transmitting a General Call, these bits are ignored. To generate a START BYTE, the CPU needs to write only once into these bits.
    /// If the IC_TAR and IC_SAR are the same, loopback exists but the FIFOs are shared between master and slave, so full loopback is not feasible. Only one direction loopback mode is supported (simplex), not duplex. A master cannot transmit to itself; it can transmit to only a slave.
    ///
    /// Bits `[9:0]`, mask `0x000003ff`
    pub const IC_TAR_OFFSET: u32 = 0;
    pub const IC_TAR_MASK: u32 = 0x000003ff;

    // ======== Field Accessors ========

    /// **SPECIAL** `[11]` (RW)
    ///
    /// This bit indicates whether software performs a Device-ID or General Call or START BYTE command. - 0: ignore bit 10 GC_OR_START and use IC_TAR normally - 1: perform special I2C command as specified in Device_ID or GC_OR_START bit Reset value: 0x0
    #[inline(always)]
    pub fn special(&self) -> bool {
        (self.value.read_bits() & Self::SPECIAL_MASK) != 0
    }

    /// Set SPECIAL bit atomically (RW)
    #[inline(always)]
    pub fn set_special(&self) {
        self.set_bits(Self::SPECIAL_MASK);
    }

    /// Clear SPECIAL bit atomically (RW)
    #[inline(always)]
    pub fn clr_special(&self) {
        self.clr_bits(Self::SPECIAL_MASK);
    }

    /// **GC_OR_START** `[10]` (RW)
    ///
    /// If bit 11 (SPECIAL) is set to 1 and bit 13(Device-ID) is set to 0, then this bit indicates whether a General Call or START byte command is to be performed by the DW_apb_i2c. - 0: General Call Address - after issuing a General Call, only writes may be performed. Attempting to issue a read command results in setting bit 6 (TX_ABRT) of the IC_RAW_INTR_STAT register. The DW_apb_i2c remains in General Call mode until the SPECIAL bit value (bit 11) is cleared. - 1: START BYTE Reset value: 0x0
    #[inline(always)]
    pub fn gc_or_start(&self) -> bool {
        (self.value.read_bits() & Self::GC_OR_START_MASK) != 0
    }

    /// Set GC_OR_START bit atomically (RW)
    #[inline(always)]
    pub fn set_gc_or_start(&self) {
        self.set_bits(Self::GC_OR_START_MASK);
    }

    /// Clear GC_OR_START bit atomically (RW)
    #[inline(always)]
    pub fn clr_gc_or_start(&self) {
        self.clr_bits(Self::GC_OR_START_MASK);
    }

    /// **IC_TAR** `[9:0]` (RW)
    ///
    /// This is the target address for any master transaction. When transmitting a General Call, these bits are ignored. To generate a START BYTE, the CPU needs to write only once into these bits.
    /// If the IC_TAR and IC_SAR are the same, loopback exists but the FIFOs are shared between master and slave, so full loopback is not feasible. Only one direction loopback mode is supported (simplex), not duplex. A master cannot transmit to itself; it can transmit to only a slave.
    #[inline(always)]
    pub fn ic_tar(&self) -> u32 {
        (self.value.read_bits() & Self::IC_TAR_MASK) >> Self::IC_TAR_OFFSET
    }

    /// Write **IC_TAR** `[9:0]` (RW)
    ///
    /// This is the target address for any master transaction. When transmitting a General Call, these bits are ignored. To generate a START BYTE, the CPU needs to write only once into these bits.
    /// If the IC_TAR and IC_SAR are the same, loopback exists but the FIFOs are shared between master and slave, so full loopback is not feasible. Only one direction loopback mode is supported (simplex), not duplex. A master cannot transmit to itself; it can transmit to only a slave.
    #[inline(always)]
    pub fn write_ic_tar(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::IC_TAR_MASK) | ((value << Self::IC_TAR_OFFSET) & Self::IC_TAR_MASK)
        });
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

    /// Read **SPECIAL** `[11:11]`
    ///
    /// This bit indicates whether software performs a Device-ID or General Call or START BYTE command. - 0: ignore bit 10 GC_OR_START and use IC_TAR normally - 1: perform special I2C command as specified in Device_ID or GC_OR_START bit Reset value: 0x0
    #[inline(always)]
    pub const fn special(&self) -> bool {
        (self.bits & IcTar::SPECIAL_MASK) != 0
    }

    /// Read **GC_OR_START** `[10:10]`
    ///
    /// If bit 11 (SPECIAL) is set to 1 and bit 13(Device-ID) is set to 0, then this bit indicates whether a General Call or START byte command is to be performed by the DW_apb_i2c. - 0: General Call Address - after issuing a General Call, only writes may be performed. Attempting to issue a read command results in setting bit 6 (TX_ABRT) of the IC_RAW_INTR_STAT register. The DW_apb_i2c remains in General Call mode until the SPECIAL bit value (bit 11) is cleared. - 1: START BYTE Reset value: 0x0
    #[inline(always)]
    pub const fn gc_or_start(&self) -> bool {
        (self.bits & IcTar::GC_OR_START_MASK) != 0
    }

    /// Read **IC_TAR** `[9:0]`
    ///
    /// This is the target address for any master transaction. When transmitting a General Call, these bits are ignored. To generate a START BYTE, the CPU needs to write only once into these bits.
    /// If the IC_TAR and IC_SAR are the same, loopback exists but the FIFOs are shared between master and slave, so full loopback is not feasible. Only one direction loopback mode is supported (simplex), not duplex. A master cannot transmit to itself; it can transmit to only a slave.
    #[inline(always)]
    pub const fn ic_tar(&self) -> u32 {
        (self.bits & IcTar::IC_TAR_MASK) >> IcTar::IC_TAR_OFFSET
    }

    /// Set **SPECIAL** `[11:11]`
    ///
    /// This bit indicates whether software performs a Device-ID or General Call or START BYTE command. - 0: ignore bit 10 GC_OR_START and use IC_TAR normally - 1: perform special I2C command as specified in Device_ID or GC_OR_START bit Reset value: 0x0
    #[inline(always)]
    pub const fn with_special(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IcTar::SPECIAL_MASK
            } else {
                self.bits & !IcTar::SPECIAL_MASK
            },
        }
    }

    /// Set **GC_OR_START** `[10:10]`
    ///
    /// If bit 11 (SPECIAL) is set to 1 and bit 13(Device-ID) is set to 0, then this bit indicates whether a General Call or START byte command is to be performed by the DW_apb_i2c. - 0: General Call Address - after issuing a General Call, only writes may be performed. Attempting to issue a read command results in setting bit 6 (TX_ABRT) of the IC_RAW_INTR_STAT register. The DW_apb_i2c remains in General Call mode until the SPECIAL bit value (bit 11) is cleared. - 1: START BYTE Reset value: 0x0
    #[inline(always)]
    pub const fn with_gc_or_start(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IcTar::GC_OR_START_MASK
            } else {
                self.bits & !IcTar::GC_OR_START_MASK
            },
        }
    }

    /// Set **IC_TAR** `[9:0]`
    ///
    /// This is the target address for any master transaction. When transmitting a General Call, these bits are ignored. To generate a START BYTE, the CPU needs to write only once into these bits.
    /// If the IC_TAR and IC_SAR are the same, loopback exists but the FIFOs are shared between master and slave, so full loopback is not feasible. Only one direction loopback mode is supported (simplex), not duplex. A master cannot transmit to itself; it can transmit to only a slave.
    #[inline(always)]
    pub const fn with_ic_tar(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !IcTar::IC_TAR_MASK)
                | ((value << IcTar::IC_TAR_OFFSET) & IcTar::IC_TAR_MASK),
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
    pub fn read_val(&self) -> IcTarVal {
        IcTarVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: IcTarVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(IcTarVal) -> IcTarVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
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
/// - **IC_SAR** `[9:0]` (RW)
///
///   The IC_SAR holds the slave address when the I2C is operating as a slave. For 7-bit addressing, only IC_SAR[6:0] is used.
///   This register can be written only when the I2C interface is disabled, which corresponds to the IC_ENABLE[0] register being set to 0. Writes at other times have no effect.
///   Note: The default values cannot be any of the reserved address locations: that is, 0x00 to 0x07, or 0x78 to 0x7f. The correct operation of the device is not guaranteed if you program the IC_SAR or IC_TAR to a reserved value. Refer to <<table_I2C_firstbyte_bit_defs>> for a complete list of these reserved values.
#[repr(transparent)]
pub struct IcSar {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for IcSar {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl IcSar {
    // ======== Field Constants ========

    /// **IC_SAR**
    ///
    /// The IC_SAR holds the slave address when the I2C is operating as a slave. For 7-bit addressing, only IC_SAR[6:0] is used.
    /// This register can be written only when the I2C interface is disabled, which corresponds to the IC_ENABLE[0] register being set to 0. Writes at other times have no effect.
    /// Note: The default values cannot be any of the reserved address locations: that is, 0x00 to 0x07, or 0x78 to 0x7f. The correct operation of the device is not guaranteed if you program the IC_SAR or IC_TAR to a reserved value. Refer to <<table_I2C_firstbyte_bit_defs>> for a complete list of these reserved values.
    ///
    /// Bits `[9:0]`, mask `0x000003ff`
    pub const IC_SAR_OFFSET: u32 = 0;
    pub const IC_SAR_MASK: u32 = 0x000003ff;

    // ======== Field Accessors ========

    /// **IC_SAR** `[9:0]` (RW)
    ///
    /// The IC_SAR holds the slave address when the I2C is operating as a slave. For 7-bit addressing, only IC_SAR[6:0] is used.
    /// This register can be written only when the I2C interface is disabled, which corresponds to the IC_ENABLE[0] register being set to 0. Writes at other times have no effect.
    /// Note: The default values cannot be any of the reserved address locations: that is, 0x00 to 0x07, or 0x78 to 0x7f. The correct operation of the device is not guaranteed if you program the IC_SAR or IC_TAR to a reserved value. Refer to <<table_I2C_firstbyte_bit_defs>> for a complete list of these reserved values.
    #[inline(always)]
    pub fn ic_sar(&self) -> u32 {
        (self.value.read_bits() & Self::IC_SAR_MASK) >> Self::IC_SAR_OFFSET
    }

    /// Write **IC_SAR** `[9:0]` (RW)
    ///
    /// The IC_SAR holds the slave address when the I2C is operating as a slave. For 7-bit addressing, only IC_SAR[6:0] is used.
    /// This register can be written only when the I2C interface is disabled, which corresponds to the IC_ENABLE[0] register being set to 0. Writes at other times have no effect.
    /// Note: The default values cannot be any of the reserved address locations: that is, 0x00 to 0x07, or 0x78 to 0x7f. The correct operation of the device is not guaranteed if you program the IC_SAR or IC_TAR to a reserved value. Refer to <<table_I2C_firstbyte_bit_defs>> for a complete list of these reserved values.
    #[inline(always)]
    pub fn write_ic_sar(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::IC_SAR_MASK) | ((value << Self::IC_SAR_OFFSET) & Self::IC_SAR_MASK)
        });
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

    /// Read **IC_SAR** `[9:0]`
    ///
    /// The IC_SAR holds the slave address when the I2C is operating as a slave. For 7-bit addressing, only IC_SAR[6:0] is used.
    /// This register can be written only when the I2C interface is disabled, which corresponds to the IC_ENABLE[0] register being set to 0. Writes at other times have no effect.
    /// Note: The default values cannot be any of the reserved address locations: that is, 0x00 to 0x07, or 0x78 to 0x7f. The correct operation of the device is not guaranteed if you program the IC_SAR or IC_TAR to a reserved value. Refer to <<table_I2C_firstbyte_bit_defs>> for a complete list of these reserved values.
    #[inline(always)]
    pub const fn ic_sar(&self) -> u32 {
        (self.bits & IcSar::IC_SAR_MASK) >> IcSar::IC_SAR_OFFSET
    }

    /// Set **IC_SAR** `[9:0]`
    ///
    /// The IC_SAR holds the slave address when the I2C is operating as a slave. For 7-bit addressing, only IC_SAR[6:0] is used.
    /// This register can be written only when the I2C interface is disabled, which corresponds to the IC_ENABLE[0] register being set to 0. Writes at other times have no effect.
    /// Note: The default values cannot be any of the reserved address locations: that is, 0x00 to 0x07, or 0x78 to 0x7f. The correct operation of the device is not guaranteed if you program the IC_SAR or IC_TAR to a reserved value. Refer to <<table_I2C_firstbyte_bit_defs>> for a complete list of these reserved values.
    #[inline(always)]
    pub const fn with_ic_sar(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !IcSar::IC_SAR_MASK)
                | ((value << IcSar::IC_SAR_OFFSET) & IcSar::IC_SAR_MASK),
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
    pub fn read_val(&self) -> IcSarVal {
        IcSarVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: IcSarVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(IcSarVal) -> IcSarVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// IC_DATA_CMD Register
///
/// I2C Rx/Tx Data Buffer and Command Register; this is the register the CPU writes to when filling the TX FIFO and the CPU reads from when retrieving bytes from RX FIFO.
/// The size of the register changes as follows:
/// Write: - 11 bits when IC_EMPTYFIFO_HOLD_MASTER_EN=1 - 9 bits when IC_EMPTYFIFO_HOLD_MASTER_EN=0 Read: - 12 bits when IC_FIRST_DATA_BYTE_STATUS = 1 - 8 bits when IC_FIRST_DATA_BYTE_STATUS = 0 Note: In order for the DW_apb_i2c to continue acknowledging reads, a read command should be written for every byte that is to be received; otherwise the DW_apb_i2c will stop acknowledging.
///
/// Address offset: 0x0010
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **FIRST_DATA_BYTE** `[11]` (RO)
///
///   Indicates the first data byte received after the address phase for receive transfer in Master receiver or Slave receiver mode.
///   Reset value : 0x0
///   NOTE: In case of APB_DATA_WIDTH=8,
///   1. The user has to perform two APB Reads to IC_DATA_CMD in order to get status on 11 bit.
///   2. In order to read the 11 bit, the user has to perform the first data byte read [7:0] (offset 0x10) and then perform the second read [15:8] (offset 0x11) in order to know the status of 11 bit (whether the data received in previous read is a first data byte or not).
///   3. The 11th bit is an optional read field, user can ignore 2nd byte read [15:8] (offset 0x11) if not interested in FIRST_DATA_BYTE status.
///
/// - **RESTART** `[10]` (WO)
///
///   This bit controls whether a RESTART is issued before the byte is sent or received.
///   1 - If IC_RESTART_EN is 1, a RESTART is issued before the data is sent/received (according to the value of CMD), regardless of whether or not the transfer direction is changing from the previous command; if IC_RESTART_EN is 0, a STOP followed by a START is issued instead.
///   0 - If IC_RESTART_EN is 1, a RESTART is issued only if the transfer direction is changing from the previous command; if IC_RESTART_EN is 0, a STOP followed by a START is issued instead.
///   Reset value: 0x0
///
/// - **STOP** `[9]` (WO)
///
///   This bit controls whether a STOP is issued after the byte is sent or received.
///   - 1 - STOP is issued after this byte, regardless of whether or not the Tx FIFO is empty. If the Tx FIFO is not empty, the master immediately tries to start a new transfer by issuing a START and arbitrating for the bus. - 0 - STOP is not issued after this byte, regardless of whether or not the Tx FIFO is empty. If the Tx FIFO is not empty, the master continues the current transfer by sending/receiving data bytes according to the value of the CMD bit. If the Tx FIFO is empty, the master holds the SCL line low and stalls the bus until a new command is available in the Tx FIFO. Reset value: 0x0
///
/// - **CMD** `[8]` (WO)
///
///   This bit controls whether a read or a write is performed. This bit does not control the direction when the DW_apb_i2con acts as a slave. It controls only the direction when it acts as a master.
///   When a command is entered in the TX FIFO, this bit distinguishes the write and read commands. In slave-receiver mode, this bit is a 'don't care' because writes to this register are not required. In slave-transmitter mode, a '0' indicates that the data in IC_DATA_CMD is to be transmitted.
///   When programming this bit, you should remember the following: attempting to perform a read operation after a General Call command has been sent results in a TX_ABRT interrupt (bit 6 of the IC_RAW_INTR_STAT register), unless bit 11 (SPECIAL) in the IC_TAR register has been cleared. If a '1' is written to this bit after receiving a RD_REQ interrupt, then a TX_ABRT interrupt occurs.
///   Reset value: 0x0
///
/// - **DAT** `[7:0]` (RW)
///
///   This register contains the data to be transmitted or received on the I2C bus. If you are writing to this register and want to perform a read, bits 7:0 (DAT) are ignored by the DW_apb_i2c. However, when you read this register, these bits return the value of data received on the DW_apb_i2c interface.
///   Reset value: 0x0
#[repr(transparent)]
pub struct IcDataCmd {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for IcDataCmd {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl IcDataCmd {
    // ======== Field Constants ========

    /// **FIRST_DATA_BYTE**
    ///
    /// Indicates the first data byte received after the address phase for receive transfer in Master receiver or Slave receiver mode.
    /// Reset value : 0x0
    /// NOTE: In case of APB_DATA_WIDTH=8,
    /// 1. The user has to perform two APB Reads to IC_DATA_CMD in order to get status on 11 bit.
    /// 2. In order to read the 11 bit, the user has to perform the first data byte read [7:0] (offset 0x10) and then perform the second read [15:8] (offset 0x11) in order to know the status of 11 bit (whether the data received in previous read is a first data byte or not).
    /// 3. The 11th bit is an optional read field, user can ignore 2nd byte read [15:8] (offset 0x11) if not interested in FIRST_DATA_BYTE status.
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const FIRST_DATA_BYTE_OFFSET: u32 = 11;
    pub const FIRST_DATA_BYTE_MASK: u32 = 0x00000800;
    pub const FIRST_DATA_BYTE: u32 = 1 << 11;
    /// INACTIVE = 0
    /// Sequential data byte received
    pub const FIRST_DATA_BYTE_INACTIVE: u32 = 0 << 11;
    pub const FIRST_DATA_BYTE_INACTIVE_VAL: u32 = 0;
    /// ACTIVE = 1
    /// Non sequential data byte received
    pub const FIRST_DATA_BYTE_ACTIVE: u32 = 1 << 11;
    pub const FIRST_DATA_BYTE_ACTIVE_VAL: u32 = 1;

    /// **RESTART**
    ///
    /// This bit controls whether a RESTART is issued before the byte is sent or received.
    /// 1 - If IC_RESTART_EN is 1, a RESTART is issued before the data is sent/received (according to the value of CMD), regardless of whether or not the transfer direction is changing from the previous command; if IC_RESTART_EN is 0, a STOP followed by a START is issued instead.
    /// 0 - If IC_RESTART_EN is 1, a RESTART is issued only if the transfer direction is changing from the previous command; if IC_RESTART_EN is 0, a STOP followed by a START is issued instead.
    /// Reset value: 0x0
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const RESTART_OFFSET: u32 = 10;
    pub const RESTART_MASK: u32 = 0x00000400;
    pub const RESTART: u32 = 1 << 10;
    /// DISABLE = 0
    /// Don't Issue RESTART before this command
    pub const RESTART_DISABLE: u32 = 0 << 10;
    pub const RESTART_DISABLE_VAL: u32 = 0;
    /// ENABLE = 1
    /// Issue RESTART before this command
    pub const RESTART_ENABLE: u32 = 1 << 10;
    pub const RESTART_ENABLE_VAL: u32 = 1;

    /// **STOP**
    ///
    /// This bit controls whether a STOP is issued after the byte is sent or received.
    /// - 1 - STOP is issued after this byte, regardless of whether or not the Tx FIFO is empty. If the Tx FIFO is not empty, the master immediately tries to start a new transfer by issuing a START and arbitrating for the bus. - 0 - STOP is not issued after this byte, regardless of whether or not the Tx FIFO is empty. If the Tx FIFO is not empty, the master continues the current transfer by sending/receiving data bytes according to the value of the CMD bit. If the Tx FIFO is empty, the master holds the SCL line low and stalls the bus until a new command is available in the Tx FIFO. Reset value: 0x0
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const STOP_OFFSET: u32 = 9;
    pub const STOP_MASK: u32 = 0x00000200;
    pub const STOP: u32 = 1 << 9;
    /// DISABLE = 0
    /// Don't Issue STOP after this command
    pub const STOP_DISABLE: u32 = 0 << 9;
    pub const STOP_DISABLE_VAL: u32 = 0;
    /// ENABLE = 1
    /// Issue STOP after this command
    pub const STOP_ENABLE: u32 = 1 << 9;
    pub const STOP_ENABLE_VAL: u32 = 1;

    /// **CMD**
    ///
    /// This bit controls whether a read or a write is performed. This bit does not control the direction when the DW_apb_i2con acts as a slave. It controls only the direction when it acts as a master.
    /// When a command is entered in the TX FIFO, this bit distinguishes the write and read commands. In slave-receiver mode, this bit is a 'don't care' because writes to this register are not required. In slave-transmitter mode, a '0' indicates that the data in IC_DATA_CMD is to be transmitted.
    /// When programming this bit, you should remember the following: attempting to perform a read operation after a General Call command has been sent results in a TX_ABRT interrupt (bit 6 of the IC_RAW_INTR_STAT register), unless bit 11 (SPECIAL) in the IC_TAR register has been cleared. If a '1' is written to this bit after receiving a RD_REQ interrupt, then a TX_ABRT interrupt occurs.
    /// Reset value: 0x0
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const CMD_OFFSET: u32 = 8;
    pub const CMD_MASK: u32 = 0x00000100;
    pub const CMD: u32 = 1 << 8;
    /// WRITE = 0
    /// Master Write Command
    pub const CMD_WRITE: u32 = 0 << 8;
    pub const CMD_WRITE_VAL: u32 = 0;
    /// READ = 1
    /// Master Read Command
    pub const CMD_READ: u32 = 1 << 8;
    pub const CMD_READ_VAL: u32 = 1;

    /// **DAT**
    ///
    /// This register contains the data to be transmitted or received on the I2C bus. If you are writing to this register and want to perform a read, bits 7:0 (DAT) are ignored by the DW_apb_i2c. However, when you read this register, these bits return the value of data received on the DW_apb_i2c interface.
    /// Reset value: 0x0
    ///
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const DAT_OFFSET: u32 = 0;
    pub const DAT_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// **FIRST_DATA_BYTE** `[11]` (RO)
    ///
    /// Indicates the first data byte received after the address phase for receive transfer in Master receiver or Slave receiver mode.
    /// Reset value : 0x0
    /// NOTE: In case of APB_DATA_WIDTH=8,
    /// 1. The user has to perform two APB Reads to IC_DATA_CMD in order to get status on 11 bit.
    /// 2. In order to read the 11 bit, the user has to perform the first data byte read [7:0] (offset 0x10) and then perform the second read [15:8] (offset 0x11) in order to know the status of 11 bit (whether the data received in previous read is a first data byte or not).
    /// 3. The 11th bit is an optional read field, user can ignore 2nd byte read [15:8] (offset 0x11) if not interested in FIRST_DATA_BYTE status.
    #[inline(always)]
    pub fn first_data_byte(&self) -> bool {
        (self.value.read_bits() & Self::FIRST_DATA_BYTE_MASK) != 0
    }

    /// Trigger **RESTART** (WO)
    ///
    /// This bit controls whether a RESTART is issued before the byte is sent or received.
    /// 1 - If IC_RESTART_EN is 1, a RESTART is issued before the data is sent/received (according to the value of CMD), regardless of whether or not the transfer direction is changing from the previous command; if IC_RESTART_EN is 0, a STOP followed by a START is issued instead.
    /// 0 - If IC_RESTART_EN is 1, a RESTART is issued only if the transfer direction is changing from the previous command; if IC_RESTART_EN is 0, a STOP followed by a START is issued instead.
    /// Reset value: 0x0
    #[inline(always)]
    pub fn trigger_restart(&self) {
        self.set_bits(Self::RESTART_MASK);
    }

    /// Trigger **STOP** (WO)
    ///
    /// This bit controls whether a STOP is issued after the byte is sent or received.
    /// - 1 - STOP is issued after this byte, regardless of whether or not the Tx FIFO is empty. If the Tx FIFO is not empty, the master immediately tries to start a new transfer by issuing a START and arbitrating for the bus. - 0 - STOP is not issued after this byte, regardless of whether or not the Tx FIFO is empty. If the Tx FIFO is not empty, the master continues the current transfer by sending/receiving data bytes according to the value of the CMD bit. If the Tx FIFO is empty, the master holds the SCL line low and stalls the bus until a new command is available in the Tx FIFO. Reset value: 0x0
    #[inline(always)]
    pub fn trigger_stop(&self) {
        self.set_bits(Self::STOP_MASK);
    }

    /// Trigger **CMD** (WO)
    ///
    /// This bit controls whether a read or a write is performed. This bit does not control the direction when the DW_apb_i2con acts as a slave. It controls only the direction when it acts as a master.
    /// When a command is entered in the TX FIFO, this bit distinguishes the write and read commands. In slave-receiver mode, this bit is a 'don't care' because writes to this register are not required. In slave-transmitter mode, a '0' indicates that the data in IC_DATA_CMD is to be transmitted.
    /// When programming this bit, you should remember the following: attempting to perform a read operation after a General Call command has been sent results in a TX_ABRT interrupt (bit 6 of the IC_RAW_INTR_STAT register), unless bit 11 (SPECIAL) in the IC_TAR register has been cleared. If a '1' is written to this bit after receiving a RD_REQ interrupt, then a TX_ABRT interrupt occurs.
    /// Reset value: 0x0
    #[inline(always)]
    pub fn trigger_cmd(&self) {
        self.set_bits(Self::CMD_MASK);
    }

    /// **DAT** `[7:0]` (RW)
    ///
    /// This register contains the data to be transmitted or received on the I2C bus. If you are writing to this register and want to perform a read, bits 7:0 (DAT) are ignored by the DW_apb_i2c. However, when you read this register, these bits return the value of data received on the DW_apb_i2c interface.
    /// Reset value: 0x0
    #[inline(always)]
    pub fn dat(&self) -> u32 {
        (self.value.read_bits() & Self::DAT_MASK) >> Self::DAT_OFFSET
    }

    /// Write **DAT** `[7:0]` (RW)
    ///
    /// This register contains the data to be transmitted or received on the I2C bus. If you are writing to this register and want to perform a read, bits 7:0 (DAT) are ignored by the DW_apb_i2c. However, when you read this register, these bits return the value of data received on the DW_apb_i2c interface.
    /// Reset value: 0x0
    #[inline(always)]
    pub fn write_dat(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DAT_MASK) | ((value << Self::DAT_OFFSET) & Self::DAT_MASK)
        });
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

    /// Read **FIRST_DATA_BYTE** `[11:11]`
    ///
    /// Indicates the first data byte received after the address phase for receive transfer in Master receiver or Slave receiver mode.
    /// Reset value : 0x0
    /// NOTE: In case of APB_DATA_WIDTH=8,
    /// 1. The user has to perform two APB Reads to IC_DATA_CMD in order to get status on 11 bit.
    /// 2. In order to read the 11 bit, the user has to perform the first data byte read [7:0] (offset 0x10) and then perform the second read [15:8] (offset 0x11) in order to know the status of 11 bit (whether the data received in previous read is a first data byte or not).
    /// 3. The 11th bit is an optional read field, user can ignore 2nd byte read [15:8] (offset 0x11) if not interested in FIRST_DATA_BYTE status.
    #[inline(always)]
    pub const fn first_data_byte(&self) -> bool {
        (self.bits & IcDataCmd::FIRST_DATA_BYTE_MASK) != 0
    }

    /// Read **DAT** `[7:0]`
    ///
    /// This register contains the data to be transmitted or received on the I2C bus. If you are writing to this register and want to perform a read, bits 7:0 (DAT) are ignored by the DW_apb_i2c. However, when you read this register, these bits return the value of data received on the DW_apb_i2c interface.
    /// Reset value: 0x0
    #[inline(always)]
    pub const fn dat(&self) -> u32 {
        (self.bits & IcDataCmd::DAT_MASK) >> IcDataCmd::DAT_OFFSET
    }

    /// Set **RESTART** `[10:10]`
    ///
    /// This bit controls whether a RESTART is issued before the byte is sent or received.
    /// 1 - If IC_RESTART_EN is 1, a RESTART is issued before the data is sent/received (according to the value of CMD), regardless of whether or not the transfer direction is changing from the previous command; if IC_RESTART_EN is 0, a STOP followed by a START is issued instead.
    /// 0 - If IC_RESTART_EN is 1, a RESTART is issued only if the transfer direction is changing from the previous command; if IC_RESTART_EN is 0, a STOP followed by a START is issued instead.
    /// Reset value: 0x0
    #[inline(always)]
    pub const fn with_restart(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IcDataCmd::RESTART_MASK
            } else {
                self.bits & !IcDataCmd::RESTART_MASK
            },
        }
    }

    /// Set **STOP** `[9:9]`
    ///
    /// This bit controls whether a STOP is issued after the byte is sent or received.
    /// - 1 - STOP is issued after this byte, regardless of whether or not the Tx FIFO is empty. If the Tx FIFO is not empty, the master immediately tries to start a new transfer by issuing a START and arbitrating for the bus. - 0 - STOP is not issued after this byte, regardless of whether or not the Tx FIFO is empty. If the Tx FIFO is not empty, the master continues the current transfer by sending/receiving data bytes according to the value of the CMD bit. If the Tx FIFO is empty, the master holds the SCL line low and stalls the bus until a new command is available in the Tx FIFO. Reset value: 0x0
    #[inline(always)]
    pub const fn with_stop(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IcDataCmd::STOP_MASK
            } else {
                self.bits & !IcDataCmd::STOP_MASK
            },
        }
    }

    /// Set **CMD** `[8:8]`
    ///
    /// This bit controls whether a read or a write is performed. This bit does not control the direction when the DW_apb_i2con acts as a slave. It controls only the direction when it acts as a master.
    /// When a command is entered in the TX FIFO, this bit distinguishes the write and read commands. In slave-receiver mode, this bit is a 'don't care' because writes to this register are not required. In slave-transmitter mode, a '0' indicates that the data in IC_DATA_CMD is to be transmitted.
    /// When programming this bit, you should remember the following: attempting to perform a read operation after a General Call command has been sent results in a TX_ABRT interrupt (bit 6 of the IC_RAW_INTR_STAT register), unless bit 11 (SPECIAL) in the IC_TAR register has been cleared. If a '1' is written to this bit after receiving a RD_REQ interrupt, then a TX_ABRT interrupt occurs.
    /// Reset value: 0x0
    #[inline(always)]
    pub const fn with_cmd(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IcDataCmd::CMD_MASK
            } else {
                self.bits & !IcDataCmd::CMD_MASK
            },
        }
    }

    /// Set **DAT** `[7:0]`
    ///
    /// This register contains the data to be transmitted or received on the I2C bus. If you are writing to this register and want to perform a read, bits 7:0 (DAT) are ignored by the DW_apb_i2c. However, when you read this register, these bits return the value of data received on the DW_apb_i2c interface.
    /// Reset value: 0x0
    #[inline(always)]
    pub const fn with_dat(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !IcDataCmd::DAT_MASK)
                | ((value << IcDataCmd::DAT_OFFSET) & IcDataCmd::DAT_MASK),
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
    pub fn read_val(&self) -> IcDataCmdVal {
        IcDataCmdVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: IcDataCmdVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(IcDataCmdVal) -> IcDataCmdVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
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
/// - **IC_SS_SCL_HCNT** `[15:0]` (RW)
///
///   This register must be set before any I2C bus transaction can take place to ensure proper I/O timing. This register sets the SCL clock high-period count for standard speed. For more information, refer to 'IC_CLK Frequency Configuration'.
///   This register can be written only when the I2C interface is disabled which corresponds to the IC_ENABLE[0] register being set to 0. Writes at other times have no effect.
///   The minimum valid value is 6; hardware prevents values less than this being written, and if attempted results in 6 being set. For designs with APB_DATA_WIDTH = 8, the order of programming is important to ensure the correct operation of the DW_apb_i2c. The lower byte must be programmed first. Then the upper byte is programmed.
///   NOTE: This register must not be programmed to a value higher than 65525, because DW_apb_i2c uses a 16-bit counter to flag an I2C bus idle condition when this counter reaches a value of IC_SS_SCL_HCNT + 10.
#[repr(transparent)]
pub struct IcSsSclHcnt {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for IcSsSclHcnt {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl IcSsSclHcnt {
    // ======== Field Constants ========

    /// **IC_SS_SCL_HCNT**
    ///
    /// This register must be set before any I2C bus transaction can take place to ensure proper I/O timing. This register sets the SCL clock high-period count for standard speed. For more information, refer to 'IC_CLK Frequency Configuration'.
    /// This register can be written only when the I2C interface is disabled which corresponds to the IC_ENABLE[0] register being set to 0. Writes at other times have no effect.
    /// The minimum valid value is 6; hardware prevents values less than this being written, and if attempted results in 6 being set. For designs with APB_DATA_WIDTH = 8, the order of programming is important to ensure the correct operation of the DW_apb_i2c. The lower byte must be programmed first. Then the upper byte is programmed.
    /// NOTE: This register must not be programmed to a value higher than 65525, because DW_apb_i2c uses a 16-bit counter to flag an I2C bus idle condition when this counter reaches a value of IC_SS_SCL_HCNT + 10.
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const IC_SS_SCL_HCNT_OFFSET: u32 = 0;
    pub const IC_SS_SCL_HCNT_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **IC_SS_SCL_HCNT** `[15:0]` (RW)
    ///
    /// This register must be set before any I2C bus transaction can take place to ensure proper I/O timing. This register sets the SCL clock high-period count for standard speed. For more information, refer to 'IC_CLK Frequency Configuration'.
    /// This register can be written only when the I2C interface is disabled which corresponds to the IC_ENABLE[0] register being set to 0. Writes at other times have no effect.
    /// The minimum valid value is 6; hardware prevents values less than this being written, and if attempted results in 6 being set. For designs with APB_DATA_WIDTH = 8, the order of programming is important to ensure the correct operation of the DW_apb_i2c. The lower byte must be programmed first. Then the upper byte is programmed.
    /// NOTE: This register must not be programmed to a value higher than 65525, because DW_apb_i2c uses a 16-bit counter to flag an I2C bus idle condition when this counter reaches a value of IC_SS_SCL_HCNT + 10.
    #[inline(always)]
    pub fn ic_ss_scl_hcnt(&self) -> u32 {
        (self.value.read_bits() & Self::IC_SS_SCL_HCNT_MASK) >> Self::IC_SS_SCL_HCNT_OFFSET
    }

    /// Write **IC_SS_SCL_HCNT** `[15:0]` (RW)
    ///
    /// This register must be set before any I2C bus transaction can take place to ensure proper I/O timing. This register sets the SCL clock high-period count for standard speed. For more information, refer to 'IC_CLK Frequency Configuration'.
    /// This register can be written only when the I2C interface is disabled which corresponds to the IC_ENABLE[0] register being set to 0. Writes at other times have no effect.
    /// The minimum valid value is 6; hardware prevents values less than this being written, and if attempted results in 6 being set. For designs with APB_DATA_WIDTH = 8, the order of programming is important to ensure the correct operation of the DW_apb_i2c. The lower byte must be programmed first. Then the upper byte is programmed.
    /// NOTE: This register must not be programmed to a value higher than 65525, because DW_apb_i2c uses a 16-bit counter to flag an I2C bus idle condition when this counter reaches a value of IC_SS_SCL_HCNT + 10.
    #[inline(always)]
    pub fn write_ic_ss_scl_hcnt(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::IC_SS_SCL_HCNT_MASK)
                | ((value << Self::IC_SS_SCL_HCNT_OFFSET) & Self::IC_SS_SCL_HCNT_MASK)
        });
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

    /// Read **IC_SS_SCL_HCNT** `[15:0]`
    ///
    /// This register must be set before any I2C bus transaction can take place to ensure proper I/O timing. This register sets the SCL clock high-period count for standard speed. For more information, refer to 'IC_CLK Frequency Configuration'.
    /// This register can be written only when the I2C interface is disabled which corresponds to the IC_ENABLE[0] register being set to 0. Writes at other times have no effect.
    /// The minimum valid value is 6; hardware prevents values less than this being written, and if attempted results in 6 being set. For designs with APB_DATA_WIDTH = 8, the order of programming is important to ensure the correct operation of the DW_apb_i2c. The lower byte must be programmed first. Then the upper byte is programmed.
    /// NOTE: This register must not be programmed to a value higher than 65525, because DW_apb_i2c uses a 16-bit counter to flag an I2C bus idle condition when this counter reaches a value of IC_SS_SCL_HCNT + 10.
    #[inline(always)]
    pub const fn ic_ss_scl_hcnt(&self) -> u32 {
        (self.bits & IcSsSclHcnt::IC_SS_SCL_HCNT_MASK) >> IcSsSclHcnt::IC_SS_SCL_HCNT_OFFSET
    }

    /// Set **IC_SS_SCL_HCNT** `[15:0]`
    ///
    /// This register must be set before any I2C bus transaction can take place to ensure proper I/O timing. This register sets the SCL clock high-period count for standard speed. For more information, refer to 'IC_CLK Frequency Configuration'.
    /// This register can be written only when the I2C interface is disabled which corresponds to the IC_ENABLE[0] register being set to 0. Writes at other times have no effect.
    /// The minimum valid value is 6; hardware prevents values less than this being written, and if attempted results in 6 being set. For designs with APB_DATA_WIDTH = 8, the order of programming is important to ensure the correct operation of the DW_apb_i2c. The lower byte must be programmed first. Then the upper byte is programmed.
    /// NOTE: This register must not be programmed to a value higher than 65525, because DW_apb_i2c uses a 16-bit counter to flag an I2C bus idle condition when this counter reaches a value of IC_SS_SCL_HCNT + 10.
    #[inline(always)]
    pub const fn with_ic_ss_scl_hcnt(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !IcSsSclHcnt::IC_SS_SCL_HCNT_MASK)
                | ((value << IcSsSclHcnt::IC_SS_SCL_HCNT_OFFSET)
                    & IcSsSclHcnt::IC_SS_SCL_HCNT_MASK),
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
    pub fn read_val(&self) -> IcSsSclHcntVal {
        IcSsSclHcntVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: IcSsSclHcntVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(IcSsSclHcntVal) -> IcSsSclHcntVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
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
/// - **IC_SS_SCL_LCNT** `[15:0]` (RW)
///
///   This register must be set before any I2C bus transaction can take place to ensure proper I/O timing. This register sets the SCL clock low period count for standard speed. For more information, refer to 'IC_CLK Frequency Configuration'
///   This register can be written only when the I2C interface is disabled which corresponds to the IC_ENABLE[0] register being set to 0. Writes at other times have no effect.
///   The minimum valid value is 8; hardware prevents values less than this being written, and if attempted, results in 8 being set. For designs with APB_DATA_WIDTH = 8, the order of programming is important to ensure the correct operation of DW_apb_i2c. The lower byte must be programmed first, and then the upper byte is programmed.
#[repr(transparent)]
pub struct IcSsSclLcnt {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for IcSsSclLcnt {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl IcSsSclLcnt {
    // ======== Field Constants ========

    /// **IC_SS_SCL_LCNT**
    ///
    /// This register must be set before any I2C bus transaction can take place to ensure proper I/O timing. This register sets the SCL clock low period count for standard speed. For more information, refer to 'IC_CLK Frequency Configuration'
    /// This register can be written only when the I2C interface is disabled which corresponds to the IC_ENABLE[0] register being set to 0. Writes at other times have no effect.
    /// The minimum valid value is 8; hardware prevents values less than this being written, and if attempted, results in 8 being set. For designs with APB_DATA_WIDTH = 8, the order of programming is important to ensure the correct operation of DW_apb_i2c. The lower byte must be programmed first, and then the upper byte is programmed.
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const IC_SS_SCL_LCNT_OFFSET: u32 = 0;
    pub const IC_SS_SCL_LCNT_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **IC_SS_SCL_LCNT** `[15:0]` (RW)
    ///
    /// This register must be set before any I2C bus transaction can take place to ensure proper I/O timing. This register sets the SCL clock low period count for standard speed. For more information, refer to 'IC_CLK Frequency Configuration'
    /// This register can be written only when the I2C interface is disabled which corresponds to the IC_ENABLE[0] register being set to 0. Writes at other times have no effect.
    /// The minimum valid value is 8; hardware prevents values less than this being written, and if attempted, results in 8 being set. For designs with APB_DATA_WIDTH = 8, the order of programming is important to ensure the correct operation of DW_apb_i2c. The lower byte must be programmed first, and then the upper byte is programmed.
    #[inline(always)]
    pub fn ic_ss_scl_lcnt(&self) -> u32 {
        (self.value.read_bits() & Self::IC_SS_SCL_LCNT_MASK) >> Self::IC_SS_SCL_LCNT_OFFSET
    }

    /// Write **IC_SS_SCL_LCNT** `[15:0]` (RW)
    ///
    /// This register must be set before any I2C bus transaction can take place to ensure proper I/O timing. This register sets the SCL clock low period count for standard speed. For more information, refer to 'IC_CLK Frequency Configuration'
    /// This register can be written only when the I2C interface is disabled which corresponds to the IC_ENABLE[0] register being set to 0. Writes at other times have no effect.
    /// The minimum valid value is 8; hardware prevents values less than this being written, and if attempted, results in 8 being set. For designs with APB_DATA_WIDTH = 8, the order of programming is important to ensure the correct operation of DW_apb_i2c. The lower byte must be programmed first, and then the upper byte is programmed.
    #[inline(always)]
    pub fn write_ic_ss_scl_lcnt(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::IC_SS_SCL_LCNT_MASK)
                | ((value << Self::IC_SS_SCL_LCNT_OFFSET) & Self::IC_SS_SCL_LCNT_MASK)
        });
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

    /// Read **IC_SS_SCL_LCNT** `[15:0]`
    ///
    /// This register must be set before any I2C bus transaction can take place to ensure proper I/O timing. This register sets the SCL clock low period count for standard speed. For more information, refer to 'IC_CLK Frequency Configuration'
    /// This register can be written only when the I2C interface is disabled which corresponds to the IC_ENABLE[0] register being set to 0. Writes at other times have no effect.
    /// The minimum valid value is 8; hardware prevents values less than this being written, and if attempted, results in 8 being set. For designs with APB_DATA_WIDTH = 8, the order of programming is important to ensure the correct operation of DW_apb_i2c. The lower byte must be programmed first, and then the upper byte is programmed.
    #[inline(always)]
    pub const fn ic_ss_scl_lcnt(&self) -> u32 {
        (self.bits & IcSsSclLcnt::IC_SS_SCL_LCNT_MASK) >> IcSsSclLcnt::IC_SS_SCL_LCNT_OFFSET
    }

    /// Set **IC_SS_SCL_LCNT** `[15:0]`
    ///
    /// This register must be set before any I2C bus transaction can take place to ensure proper I/O timing. This register sets the SCL clock low period count for standard speed. For more information, refer to 'IC_CLK Frequency Configuration'
    /// This register can be written only when the I2C interface is disabled which corresponds to the IC_ENABLE[0] register being set to 0. Writes at other times have no effect.
    /// The minimum valid value is 8; hardware prevents values less than this being written, and if attempted, results in 8 being set. For designs with APB_DATA_WIDTH = 8, the order of programming is important to ensure the correct operation of DW_apb_i2c. The lower byte must be programmed first, and then the upper byte is programmed.
    #[inline(always)]
    pub const fn with_ic_ss_scl_lcnt(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !IcSsSclLcnt::IC_SS_SCL_LCNT_MASK)
                | ((value << IcSsSclLcnt::IC_SS_SCL_LCNT_OFFSET)
                    & IcSsSclLcnt::IC_SS_SCL_LCNT_MASK),
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
    pub fn read_val(&self) -> IcSsSclLcntVal {
        IcSsSclLcntVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: IcSsSclLcntVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(IcSsSclLcntVal) -> IcSsSclLcntVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
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
/// - **IC_FS_SCL_HCNT** `[15:0]` (RW)
///
///   This register must be set before any I2C bus transaction can take place to ensure proper I/O timing. This register sets the SCL clock high-period count for fast mode or fast mode plus. It is used in high-speed mode to send the Master Code and START BYTE or General CALL. For more information, refer to 'IC_CLK Frequency Configuration'.
///   This register goes away and becomes read-only returning 0s if IC_MAX_SPEED_MODE = standard. This register can be written only when the I2C interface is disabled, which corresponds to the IC_ENABLE[0] register being set to 0. Writes at other times have no effect.
///   The minimum valid value is 6; hardware prevents values less than this being written, and if attempted results in 6 being set. For designs with APB_DATA_WIDTH == 8 the order of programming is important to ensure the correct operation of the DW_apb_i2c. The lower byte must be programmed first. Then the upper byte is programmed.
#[repr(transparent)]
pub struct IcFsSclHcnt {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for IcFsSclHcnt {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl IcFsSclHcnt {
    // ======== Field Constants ========

    /// **IC_FS_SCL_HCNT**
    ///
    /// This register must be set before any I2C bus transaction can take place to ensure proper I/O timing. This register sets the SCL clock high-period count for fast mode or fast mode plus. It is used in high-speed mode to send the Master Code and START BYTE or General CALL. For more information, refer to 'IC_CLK Frequency Configuration'.
    /// This register goes away and becomes read-only returning 0s if IC_MAX_SPEED_MODE = standard. This register can be written only when the I2C interface is disabled, which corresponds to the IC_ENABLE[0] register being set to 0. Writes at other times have no effect.
    /// The minimum valid value is 6; hardware prevents values less than this being written, and if attempted results in 6 being set. For designs with APB_DATA_WIDTH == 8 the order of programming is important to ensure the correct operation of the DW_apb_i2c. The lower byte must be programmed first. Then the upper byte is programmed.
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const IC_FS_SCL_HCNT_OFFSET: u32 = 0;
    pub const IC_FS_SCL_HCNT_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **IC_FS_SCL_HCNT** `[15:0]` (RW)
    ///
    /// This register must be set before any I2C bus transaction can take place to ensure proper I/O timing. This register sets the SCL clock high-period count for fast mode or fast mode plus. It is used in high-speed mode to send the Master Code and START BYTE or General CALL. For more information, refer to 'IC_CLK Frequency Configuration'.
    /// This register goes away and becomes read-only returning 0s if IC_MAX_SPEED_MODE = standard. This register can be written only when the I2C interface is disabled, which corresponds to the IC_ENABLE[0] register being set to 0. Writes at other times have no effect.
    /// The minimum valid value is 6; hardware prevents values less than this being written, and if attempted results in 6 being set. For designs with APB_DATA_WIDTH == 8 the order of programming is important to ensure the correct operation of the DW_apb_i2c. The lower byte must be programmed first. Then the upper byte is programmed.
    #[inline(always)]
    pub fn ic_fs_scl_hcnt(&self) -> u32 {
        (self.value.read_bits() & Self::IC_FS_SCL_HCNT_MASK) >> Self::IC_FS_SCL_HCNT_OFFSET
    }

    /// Write **IC_FS_SCL_HCNT** `[15:0]` (RW)
    ///
    /// This register must be set before any I2C bus transaction can take place to ensure proper I/O timing. This register sets the SCL clock high-period count for fast mode or fast mode plus. It is used in high-speed mode to send the Master Code and START BYTE or General CALL. For more information, refer to 'IC_CLK Frequency Configuration'.
    /// This register goes away and becomes read-only returning 0s if IC_MAX_SPEED_MODE = standard. This register can be written only when the I2C interface is disabled, which corresponds to the IC_ENABLE[0] register being set to 0. Writes at other times have no effect.
    /// The minimum valid value is 6; hardware prevents values less than this being written, and if attempted results in 6 being set. For designs with APB_DATA_WIDTH == 8 the order of programming is important to ensure the correct operation of the DW_apb_i2c. The lower byte must be programmed first. Then the upper byte is programmed.
    #[inline(always)]
    pub fn write_ic_fs_scl_hcnt(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::IC_FS_SCL_HCNT_MASK)
                | ((value << Self::IC_FS_SCL_HCNT_OFFSET) & Self::IC_FS_SCL_HCNT_MASK)
        });
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

    /// Read **IC_FS_SCL_HCNT** `[15:0]`
    ///
    /// This register must be set before any I2C bus transaction can take place to ensure proper I/O timing. This register sets the SCL clock high-period count for fast mode or fast mode plus. It is used in high-speed mode to send the Master Code and START BYTE or General CALL. For more information, refer to 'IC_CLK Frequency Configuration'.
    /// This register goes away and becomes read-only returning 0s if IC_MAX_SPEED_MODE = standard. This register can be written only when the I2C interface is disabled, which corresponds to the IC_ENABLE[0] register being set to 0. Writes at other times have no effect.
    /// The minimum valid value is 6; hardware prevents values less than this being written, and if attempted results in 6 being set. For designs with APB_DATA_WIDTH == 8 the order of programming is important to ensure the correct operation of the DW_apb_i2c. The lower byte must be programmed first. Then the upper byte is programmed.
    #[inline(always)]
    pub const fn ic_fs_scl_hcnt(&self) -> u32 {
        (self.bits & IcFsSclHcnt::IC_FS_SCL_HCNT_MASK) >> IcFsSclHcnt::IC_FS_SCL_HCNT_OFFSET
    }

    /// Set **IC_FS_SCL_HCNT** `[15:0]`
    ///
    /// This register must be set before any I2C bus transaction can take place to ensure proper I/O timing. This register sets the SCL clock high-period count for fast mode or fast mode plus. It is used in high-speed mode to send the Master Code and START BYTE or General CALL. For more information, refer to 'IC_CLK Frequency Configuration'.
    /// This register goes away and becomes read-only returning 0s if IC_MAX_SPEED_MODE = standard. This register can be written only when the I2C interface is disabled, which corresponds to the IC_ENABLE[0] register being set to 0. Writes at other times have no effect.
    /// The minimum valid value is 6; hardware prevents values less than this being written, and if attempted results in 6 being set. For designs with APB_DATA_WIDTH == 8 the order of programming is important to ensure the correct operation of the DW_apb_i2c. The lower byte must be programmed first. Then the upper byte is programmed.
    #[inline(always)]
    pub const fn with_ic_fs_scl_hcnt(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !IcFsSclHcnt::IC_FS_SCL_HCNT_MASK)
                | ((value << IcFsSclHcnt::IC_FS_SCL_HCNT_OFFSET)
                    & IcFsSclHcnt::IC_FS_SCL_HCNT_MASK),
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
    pub fn read_val(&self) -> IcFsSclHcntVal {
        IcFsSclHcntVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: IcFsSclHcntVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(IcFsSclHcntVal) -> IcFsSclHcntVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
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
/// - **IC_FS_SCL_LCNT** `[15:0]` (RW)
///
///   This register must be set before any I2C bus transaction can take place to ensure proper I/O timing. This register sets the SCL clock low period count for fast speed. It is used in high-speed mode to send the Master Code and START BYTE or General CALL. For more information, refer to 'IC_CLK Frequency Configuration'.
///   This register goes away and becomes read-only returning 0s if IC_MAX_SPEED_MODE = standard.
///   This register can be written only when the I2C interface is disabled, which corresponds to the IC_ENABLE[0] register being set to 0. Writes at other times have no effect.
///   The minimum valid value is 8; hardware prevents values less than this being written, and if attempted results in 8 being set. For designs with APB_DATA_WIDTH = 8 the order of programming is important to ensure the correct operation of the DW_apb_i2c. The lower byte must be programmed first. Then the upper byte is programmed. If the value is less than 8 then the count value gets changed to 8.
#[repr(transparent)]
pub struct IcFsSclLcnt {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for IcFsSclLcnt {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl IcFsSclLcnt {
    // ======== Field Constants ========

    /// **IC_FS_SCL_LCNT**
    ///
    /// This register must be set before any I2C bus transaction can take place to ensure proper I/O timing. This register sets the SCL clock low period count for fast speed. It is used in high-speed mode to send the Master Code and START BYTE or General CALL. For more information, refer to 'IC_CLK Frequency Configuration'.
    /// This register goes away and becomes read-only returning 0s if IC_MAX_SPEED_MODE = standard.
    /// This register can be written only when the I2C interface is disabled, which corresponds to the IC_ENABLE[0] register being set to 0. Writes at other times have no effect.
    /// The minimum valid value is 8; hardware prevents values less than this being written, and if attempted results in 8 being set. For designs with APB_DATA_WIDTH = 8 the order of programming is important to ensure the correct operation of the DW_apb_i2c. The lower byte must be programmed first. Then the upper byte is programmed. If the value is less than 8 then the count value gets changed to 8.
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const IC_FS_SCL_LCNT_OFFSET: u32 = 0;
    pub const IC_FS_SCL_LCNT_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **IC_FS_SCL_LCNT** `[15:0]` (RW)
    ///
    /// This register must be set before any I2C bus transaction can take place to ensure proper I/O timing. This register sets the SCL clock low period count for fast speed. It is used in high-speed mode to send the Master Code and START BYTE or General CALL. For more information, refer to 'IC_CLK Frequency Configuration'.
    /// This register goes away and becomes read-only returning 0s if IC_MAX_SPEED_MODE = standard.
    /// This register can be written only when the I2C interface is disabled, which corresponds to the IC_ENABLE[0] register being set to 0. Writes at other times have no effect.
    /// The minimum valid value is 8; hardware prevents values less than this being written, and if attempted results in 8 being set. For designs with APB_DATA_WIDTH = 8 the order of programming is important to ensure the correct operation of the DW_apb_i2c. The lower byte must be programmed first. Then the upper byte is programmed. If the value is less than 8 then the count value gets changed to 8.
    #[inline(always)]
    pub fn ic_fs_scl_lcnt(&self) -> u32 {
        (self.value.read_bits() & Self::IC_FS_SCL_LCNT_MASK) >> Self::IC_FS_SCL_LCNT_OFFSET
    }

    /// Write **IC_FS_SCL_LCNT** `[15:0]` (RW)
    ///
    /// This register must be set before any I2C bus transaction can take place to ensure proper I/O timing. This register sets the SCL clock low period count for fast speed. It is used in high-speed mode to send the Master Code and START BYTE or General CALL. For more information, refer to 'IC_CLK Frequency Configuration'.
    /// This register goes away and becomes read-only returning 0s if IC_MAX_SPEED_MODE = standard.
    /// This register can be written only when the I2C interface is disabled, which corresponds to the IC_ENABLE[0] register being set to 0. Writes at other times have no effect.
    /// The minimum valid value is 8; hardware prevents values less than this being written, and if attempted results in 8 being set. For designs with APB_DATA_WIDTH = 8 the order of programming is important to ensure the correct operation of the DW_apb_i2c. The lower byte must be programmed first. Then the upper byte is programmed. If the value is less than 8 then the count value gets changed to 8.
    #[inline(always)]
    pub fn write_ic_fs_scl_lcnt(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::IC_FS_SCL_LCNT_MASK)
                | ((value << Self::IC_FS_SCL_LCNT_OFFSET) & Self::IC_FS_SCL_LCNT_MASK)
        });
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

    /// Read **IC_FS_SCL_LCNT** `[15:0]`
    ///
    /// This register must be set before any I2C bus transaction can take place to ensure proper I/O timing. This register sets the SCL clock low period count for fast speed. It is used in high-speed mode to send the Master Code and START BYTE or General CALL. For more information, refer to 'IC_CLK Frequency Configuration'.
    /// This register goes away and becomes read-only returning 0s if IC_MAX_SPEED_MODE = standard.
    /// This register can be written only when the I2C interface is disabled, which corresponds to the IC_ENABLE[0] register being set to 0. Writes at other times have no effect.
    /// The minimum valid value is 8; hardware prevents values less than this being written, and if attempted results in 8 being set. For designs with APB_DATA_WIDTH = 8 the order of programming is important to ensure the correct operation of the DW_apb_i2c. The lower byte must be programmed first. Then the upper byte is programmed. If the value is less than 8 then the count value gets changed to 8.
    #[inline(always)]
    pub const fn ic_fs_scl_lcnt(&self) -> u32 {
        (self.bits & IcFsSclLcnt::IC_FS_SCL_LCNT_MASK) >> IcFsSclLcnt::IC_FS_SCL_LCNT_OFFSET
    }

    /// Set **IC_FS_SCL_LCNT** `[15:0]`
    ///
    /// This register must be set before any I2C bus transaction can take place to ensure proper I/O timing. This register sets the SCL clock low period count for fast speed. It is used in high-speed mode to send the Master Code and START BYTE or General CALL. For more information, refer to 'IC_CLK Frequency Configuration'.
    /// This register goes away and becomes read-only returning 0s if IC_MAX_SPEED_MODE = standard.
    /// This register can be written only when the I2C interface is disabled, which corresponds to the IC_ENABLE[0] register being set to 0. Writes at other times have no effect.
    /// The minimum valid value is 8; hardware prevents values less than this being written, and if attempted results in 8 being set. For designs with APB_DATA_WIDTH = 8 the order of programming is important to ensure the correct operation of the DW_apb_i2c. The lower byte must be programmed first. Then the upper byte is programmed. If the value is less than 8 then the count value gets changed to 8.
    #[inline(always)]
    pub const fn with_ic_fs_scl_lcnt(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !IcFsSclLcnt::IC_FS_SCL_LCNT_MASK)
                | ((value << IcFsSclLcnt::IC_FS_SCL_LCNT_OFFSET)
                    & IcFsSclLcnt::IC_FS_SCL_LCNT_MASK),
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
    pub fn read_val(&self) -> IcFsSclLcntVal {
        IcFsSclLcntVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: IcFsSclLcntVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(IcFsSclLcntVal) -> IcFsSclLcntVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// IC_INTR_STAT Register
///
/// I2C Interrupt Status Register
/// Each bit in this register has a corresponding mask bit in the IC_INTR_MASK register. These bits are cleared by reading the matching interrupt clear register. The unmasked raw versions of these bits are available in the IC_RAW_INTR_STAT register.
///
/// Address offset: 0x002c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **R_RESTART_DET** `[12]` (RO)
///
///   See IC_RAW_INTR_STAT for a detailed description of R_RESTART_DET bit.
///   Reset value: 0x0
///
/// - **R_GEN_CALL** `[11]` (RO)
///
///   See IC_RAW_INTR_STAT for a detailed description of R_GEN_CALL bit.
///   Reset value: 0x0
///
/// - **R_START_DET** `[10]` (RO)
///
///   See IC_RAW_INTR_STAT for a detailed description of R_START_DET bit.
///   Reset value: 0x0
///
/// - **R_STOP_DET** `[9]` (RO)
///
///   See IC_RAW_INTR_STAT for a detailed description of R_STOP_DET bit.
///   Reset value: 0x0
///
/// - **R_ACTIVITY** `[8]` (RO)
///
///   See IC_RAW_INTR_STAT for a detailed description of R_ACTIVITY bit.
///   Reset value: 0x0
///
/// - **R_RX_DONE** `[7]` (RO)
///
///   See IC_RAW_INTR_STAT for a detailed description of R_RX_DONE bit.
///   Reset value: 0x0
///
/// - **R_TX_ABRT** `[6]` (RO)
///
///   See IC_RAW_INTR_STAT for a detailed description of R_TX_ABRT bit.
///   Reset value: 0x0
///
/// - **R_RD_REQ** `[5]` (RO)
///
///   See IC_RAW_INTR_STAT for a detailed description of R_RD_REQ bit.
///   Reset value: 0x0
///
/// - **R_TX_EMPTY** `[4]` (RO)
///
///   See IC_RAW_INTR_STAT for a detailed description of R_TX_EMPTY bit.
///   Reset value: 0x0
///
/// - **R_TX_OVER** `[3]` (RO)
///
///   See IC_RAW_INTR_STAT for a detailed description of R_TX_OVER bit.
///   Reset value: 0x0
///
/// - **R_RX_FULL** `[2]` (RO)
///
///   See IC_RAW_INTR_STAT for a detailed description of R_RX_FULL bit.
///   Reset value: 0x0
///
/// - **R_RX_OVER** `[1]` (RO)
///
///   See IC_RAW_INTR_STAT for a detailed description of R_RX_OVER bit.
///   Reset value: 0x0
///
/// - **R_RX_UNDER** `[0]` (RO)
///
///   See IC_RAW_INTR_STAT for a detailed description of R_RX_UNDER bit.
///   Reset value: 0x0
#[repr(transparent)]
pub struct IcIntrStat {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for IcIntrStat {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl IcIntrStat {
    // ======== Field Constants ========

    /// **R_RESTART_DET**
    ///
    /// See IC_RAW_INTR_STAT for a detailed description of R_RESTART_DET bit.
    /// Reset value: 0x0
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const R_RESTART_DET_OFFSET: u32 = 12;
    pub const R_RESTART_DET_MASK: u32 = 0x00001000;
    pub const R_RESTART_DET: u32 = 1 << 12;
    /// INACTIVE = 0
    /// R_RESTART_DET interrupt is inactive
    pub const R_RESTART_DET_INACTIVE: u32 = 0 << 12;
    pub const R_RESTART_DET_INACTIVE_VAL: u32 = 0;
    /// ACTIVE = 1
    /// R_RESTART_DET interrupt is active
    pub const R_RESTART_DET_ACTIVE: u32 = 1 << 12;
    pub const R_RESTART_DET_ACTIVE_VAL: u32 = 1;

    /// **R_GEN_CALL**
    ///
    /// See IC_RAW_INTR_STAT for a detailed description of R_GEN_CALL bit.
    /// Reset value: 0x0
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const R_GEN_CALL_OFFSET: u32 = 11;
    pub const R_GEN_CALL_MASK: u32 = 0x00000800;
    pub const R_GEN_CALL: u32 = 1 << 11;
    /// INACTIVE = 0
    /// R_GEN_CALL interrupt is inactive
    pub const R_GEN_CALL_INACTIVE: u32 = 0 << 11;
    pub const R_GEN_CALL_INACTIVE_VAL: u32 = 0;
    /// ACTIVE = 1
    /// R_GEN_CALL interrupt is active
    pub const R_GEN_CALL_ACTIVE: u32 = 1 << 11;
    pub const R_GEN_CALL_ACTIVE_VAL: u32 = 1;

    /// **R_START_DET**
    ///
    /// See IC_RAW_INTR_STAT for a detailed description of R_START_DET bit.
    /// Reset value: 0x0
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const R_START_DET_OFFSET: u32 = 10;
    pub const R_START_DET_MASK: u32 = 0x00000400;
    pub const R_START_DET: u32 = 1 << 10;
    /// INACTIVE = 0
    /// R_START_DET interrupt is inactive
    pub const R_START_DET_INACTIVE: u32 = 0 << 10;
    pub const R_START_DET_INACTIVE_VAL: u32 = 0;
    /// ACTIVE = 1
    /// R_START_DET interrupt is active
    pub const R_START_DET_ACTIVE: u32 = 1 << 10;
    pub const R_START_DET_ACTIVE_VAL: u32 = 1;

    /// **R_STOP_DET**
    ///
    /// See IC_RAW_INTR_STAT for a detailed description of R_STOP_DET bit.
    /// Reset value: 0x0
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const R_STOP_DET_OFFSET: u32 = 9;
    pub const R_STOP_DET_MASK: u32 = 0x00000200;
    pub const R_STOP_DET: u32 = 1 << 9;
    /// INACTIVE = 0
    /// R_STOP_DET interrupt is inactive
    pub const R_STOP_DET_INACTIVE: u32 = 0 << 9;
    pub const R_STOP_DET_INACTIVE_VAL: u32 = 0;
    /// ACTIVE = 1
    /// R_STOP_DET interrupt is active
    pub const R_STOP_DET_ACTIVE: u32 = 1 << 9;
    pub const R_STOP_DET_ACTIVE_VAL: u32 = 1;

    /// **R_ACTIVITY**
    ///
    /// See IC_RAW_INTR_STAT for a detailed description of R_ACTIVITY bit.
    /// Reset value: 0x0
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const R_ACTIVITY_OFFSET: u32 = 8;
    pub const R_ACTIVITY_MASK: u32 = 0x00000100;
    pub const R_ACTIVITY: u32 = 1 << 8;
    /// INACTIVE = 0
    /// R_ACTIVITY interrupt is inactive
    pub const R_ACTIVITY_INACTIVE: u32 = 0 << 8;
    pub const R_ACTIVITY_INACTIVE_VAL: u32 = 0;
    /// ACTIVE = 1
    /// R_ACTIVITY interrupt is active
    pub const R_ACTIVITY_ACTIVE: u32 = 1 << 8;
    pub const R_ACTIVITY_ACTIVE_VAL: u32 = 1;

    /// **R_RX_DONE**
    ///
    /// See IC_RAW_INTR_STAT for a detailed description of R_RX_DONE bit.
    /// Reset value: 0x0
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const R_RX_DONE_OFFSET: u32 = 7;
    pub const R_RX_DONE_MASK: u32 = 0x00000080;
    pub const R_RX_DONE: u32 = 1 << 7;
    /// INACTIVE = 0
    /// R_RX_DONE interrupt is inactive
    pub const R_RX_DONE_INACTIVE: u32 = 0 << 7;
    pub const R_RX_DONE_INACTIVE_VAL: u32 = 0;
    /// ACTIVE = 1
    /// R_RX_DONE interrupt is active
    pub const R_RX_DONE_ACTIVE: u32 = 1 << 7;
    pub const R_RX_DONE_ACTIVE_VAL: u32 = 1;

    /// **R_TX_ABRT**
    ///
    /// See IC_RAW_INTR_STAT for a detailed description of R_TX_ABRT bit.
    /// Reset value: 0x0
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const R_TX_ABRT_OFFSET: u32 = 6;
    pub const R_TX_ABRT_MASK: u32 = 0x00000040;
    pub const R_TX_ABRT: u32 = 1 << 6;
    /// INACTIVE = 0
    /// R_TX_ABRT interrupt is inactive
    pub const R_TX_ABRT_INACTIVE: u32 = 0 << 6;
    pub const R_TX_ABRT_INACTIVE_VAL: u32 = 0;
    /// ACTIVE = 1
    /// R_TX_ABRT interrupt is active
    pub const R_TX_ABRT_ACTIVE: u32 = 1 << 6;
    pub const R_TX_ABRT_ACTIVE_VAL: u32 = 1;

    /// **R_RD_REQ**
    ///
    /// See IC_RAW_INTR_STAT for a detailed description of R_RD_REQ bit.
    /// Reset value: 0x0
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const R_RD_REQ_OFFSET: u32 = 5;
    pub const R_RD_REQ_MASK: u32 = 0x00000020;
    pub const R_RD_REQ: u32 = 1 << 5;
    /// INACTIVE = 0
    /// R_RD_REQ interrupt is inactive
    pub const R_RD_REQ_INACTIVE: u32 = 0 << 5;
    pub const R_RD_REQ_INACTIVE_VAL: u32 = 0;
    /// ACTIVE = 1
    /// R_RD_REQ interrupt is active
    pub const R_RD_REQ_ACTIVE: u32 = 1 << 5;
    pub const R_RD_REQ_ACTIVE_VAL: u32 = 1;

    /// **R_TX_EMPTY**
    ///
    /// See IC_RAW_INTR_STAT for a detailed description of R_TX_EMPTY bit.
    /// Reset value: 0x0
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const R_TX_EMPTY_OFFSET: u32 = 4;
    pub const R_TX_EMPTY_MASK: u32 = 0x00000010;
    pub const R_TX_EMPTY: u32 = 1 << 4;
    /// INACTIVE = 0
    /// R_TX_EMPTY interrupt is inactive
    pub const R_TX_EMPTY_INACTIVE: u32 = 0 << 4;
    pub const R_TX_EMPTY_INACTIVE_VAL: u32 = 0;
    /// ACTIVE = 1
    /// R_TX_EMPTY interrupt is active
    pub const R_TX_EMPTY_ACTIVE: u32 = 1 << 4;
    pub const R_TX_EMPTY_ACTIVE_VAL: u32 = 1;

    /// **R_TX_OVER**
    ///
    /// See IC_RAW_INTR_STAT for a detailed description of R_TX_OVER bit.
    /// Reset value: 0x0
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const R_TX_OVER_OFFSET: u32 = 3;
    pub const R_TX_OVER_MASK: u32 = 0x00000008;
    pub const R_TX_OVER: u32 = 1 << 3;
    /// INACTIVE = 0
    /// R_TX_OVER interrupt is inactive
    pub const R_TX_OVER_INACTIVE: u32 = 0 << 3;
    pub const R_TX_OVER_INACTIVE_VAL: u32 = 0;
    /// ACTIVE = 1
    /// R_TX_OVER interrupt is active
    pub const R_TX_OVER_ACTIVE: u32 = 1 << 3;
    pub const R_TX_OVER_ACTIVE_VAL: u32 = 1;

    /// **R_RX_FULL**
    ///
    /// See IC_RAW_INTR_STAT for a detailed description of R_RX_FULL bit.
    /// Reset value: 0x0
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const R_RX_FULL_OFFSET: u32 = 2;
    pub const R_RX_FULL_MASK: u32 = 0x00000004;
    pub const R_RX_FULL: u32 = 1 << 2;
    /// INACTIVE = 0
    /// R_RX_FULL interrupt is inactive
    pub const R_RX_FULL_INACTIVE: u32 = 0 << 2;
    pub const R_RX_FULL_INACTIVE_VAL: u32 = 0;
    /// ACTIVE = 1
    /// R_RX_FULL interrupt is active
    pub const R_RX_FULL_ACTIVE: u32 = 1 << 2;
    pub const R_RX_FULL_ACTIVE_VAL: u32 = 1;

    /// **R_RX_OVER**
    ///
    /// See IC_RAW_INTR_STAT for a detailed description of R_RX_OVER bit.
    /// Reset value: 0x0
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const R_RX_OVER_OFFSET: u32 = 1;
    pub const R_RX_OVER_MASK: u32 = 0x00000002;
    pub const R_RX_OVER: u32 = 1 << 1;
    /// INACTIVE = 0
    /// R_RX_OVER interrupt is inactive
    pub const R_RX_OVER_INACTIVE: u32 = 0 << 1;
    pub const R_RX_OVER_INACTIVE_VAL: u32 = 0;
    /// ACTIVE = 1
    /// R_RX_OVER interrupt is active
    pub const R_RX_OVER_ACTIVE: u32 = 1 << 1;
    pub const R_RX_OVER_ACTIVE_VAL: u32 = 1;

    /// **R_RX_UNDER**
    ///
    /// See IC_RAW_INTR_STAT for a detailed description of R_RX_UNDER bit.
    /// Reset value: 0x0
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const R_RX_UNDER_OFFSET: u32 = 0;
    pub const R_RX_UNDER_MASK: u32 = 0x00000001;
    pub const R_RX_UNDER: u32 = 1 << 0;
    /// INACTIVE = 0
    /// RX_UNDER interrupt is inactive
    pub const R_RX_UNDER_INACTIVE: u32 = 0 << 0;
    pub const R_RX_UNDER_INACTIVE_VAL: u32 = 0;
    /// ACTIVE = 1
    /// RX_UNDER interrupt is active
    pub const R_RX_UNDER_ACTIVE: u32 = 1 << 0;
    pub const R_RX_UNDER_ACTIVE_VAL: u32 = 1;

    // ======== Field Accessors ========

    /// **R_RESTART_DET** `[12]` (RO)
    ///
    /// See IC_RAW_INTR_STAT for a detailed description of R_RESTART_DET bit.
    /// Reset value: 0x0
    #[inline(always)]
    pub fn r_restart_det(&self) -> bool {
        (self.value.read_bits() & Self::R_RESTART_DET_MASK) != 0
    }

    /// **R_GEN_CALL** `[11]` (RO)
    ///
    /// See IC_RAW_INTR_STAT for a detailed description of R_GEN_CALL bit.
    /// Reset value: 0x0
    #[inline(always)]
    pub fn r_gen_call(&self) -> bool {
        (self.value.read_bits() & Self::R_GEN_CALL_MASK) != 0
    }

    /// **R_START_DET** `[10]` (RO)
    ///
    /// See IC_RAW_INTR_STAT for a detailed description of R_START_DET bit.
    /// Reset value: 0x0
    #[inline(always)]
    pub fn r_start_det(&self) -> bool {
        (self.value.read_bits() & Self::R_START_DET_MASK) != 0
    }

    /// **R_STOP_DET** `[9]` (RO)
    ///
    /// See IC_RAW_INTR_STAT for a detailed description of R_STOP_DET bit.
    /// Reset value: 0x0
    #[inline(always)]
    pub fn r_stop_det(&self) -> bool {
        (self.value.read_bits() & Self::R_STOP_DET_MASK) != 0
    }

    /// **R_ACTIVITY** `[8]` (RO)
    ///
    /// See IC_RAW_INTR_STAT for a detailed description of R_ACTIVITY bit.
    /// Reset value: 0x0
    #[inline(always)]
    pub fn r_activity(&self) -> bool {
        (self.value.read_bits() & Self::R_ACTIVITY_MASK) != 0
    }

    /// **R_RX_DONE** `[7]` (RO)
    ///
    /// See IC_RAW_INTR_STAT for a detailed description of R_RX_DONE bit.
    /// Reset value: 0x0
    #[inline(always)]
    pub fn r_rx_done(&self) -> bool {
        (self.value.read_bits() & Self::R_RX_DONE_MASK) != 0
    }

    /// **R_TX_ABRT** `[6]` (RO)
    ///
    /// See IC_RAW_INTR_STAT for a detailed description of R_TX_ABRT bit.
    /// Reset value: 0x0
    #[inline(always)]
    pub fn r_tx_abrt(&self) -> bool {
        (self.value.read_bits() & Self::R_TX_ABRT_MASK) != 0
    }

    /// **R_RD_REQ** `[5]` (RO)
    ///
    /// See IC_RAW_INTR_STAT for a detailed description of R_RD_REQ bit.
    /// Reset value: 0x0
    #[inline(always)]
    pub fn r_rd_req(&self) -> bool {
        (self.value.read_bits() & Self::R_RD_REQ_MASK) != 0
    }

    /// **R_TX_EMPTY** `[4]` (RO)
    ///
    /// See IC_RAW_INTR_STAT for a detailed description of R_TX_EMPTY bit.
    /// Reset value: 0x0
    #[inline(always)]
    pub fn r_tx_empty(&self) -> bool {
        (self.value.read_bits() & Self::R_TX_EMPTY_MASK) != 0
    }

    /// **R_TX_OVER** `[3]` (RO)
    ///
    /// See IC_RAW_INTR_STAT for a detailed description of R_TX_OVER bit.
    /// Reset value: 0x0
    #[inline(always)]
    pub fn r_tx_over(&self) -> bool {
        (self.value.read_bits() & Self::R_TX_OVER_MASK) != 0
    }

    /// **R_RX_FULL** `[2]` (RO)
    ///
    /// See IC_RAW_INTR_STAT for a detailed description of R_RX_FULL bit.
    /// Reset value: 0x0
    #[inline(always)]
    pub fn r_rx_full(&self) -> bool {
        (self.value.read_bits() & Self::R_RX_FULL_MASK) != 0
    }

    /// **R_RX_OVER** `[1]` (RO)
    ///
    /// See IC_RAW_INTR_STAT for a detailed description of R_RX_OVER bit.
    /// Reset value: 0x0
    #[inline(always)]
    pub fn r_rx_over(&self) -> bool {
        (self.value.read_bits() & Self::R_RX_OVER_MASK) != 0
    }

    /// **R_RX_UNDER** `[0]` (RO)
    ///
    /// See IC_RAW_INTR_STAT for a detailed description of R_RX_UNDER bit.
    /// Reset value: 0x0
    #[inline(always)]
    pub fn r_rx_under(&self) -> bool {
        (self.value.read_bits() & Self::R_RX_UNDER_MASK) != 0
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

    /// Read **R_RESTART_DET** `[12:12]`
    ///
    /// See IC_RAW_INTR_STAT for a detailed description of R_RESTART_DET bit.
    /// Reset value: 0x0
    #[inline(always)]
    pub const fn r_restart_det(&self) -> bool {
        (self.bits & IcIntrStat::R_RESTART_DET_MASK) != 0
    }

    /// Read **R_GEN_CALL** `[11:11]`
    ///
    /// See IC_RAW_INTR_STAT for a detailed description of R_GEN_CALL bit.
    /// Reset value: 0x0
    #[inline(always)]
    pub const fn r_gen_call(&self) -> bool {
        (self.bits & IcIntrStat::R_GEN_CALL_MASK) != 0
    }

    /// Read **R_START_DET** `[10:10]`
    ///
    /// See IC_RAW_INTR_STAT for a detailed description of R_START_DET bit.
    /// Reset value: 0x0
    #[inline(always)]
    pub const fn r_start_det(&self) -> bool {
        (self.bits & IcIntrStat::R_START_DET_MASK) != 0
    }

    /// Read **R_STOP_DET** `[9:9]`
    ///
    /// See IC_RAW_INTR_STAT for a detailed description of R_STOP_DET bit.
    /// Reset value: 0x0
    #[inline(always)]
    pub const fn r_stop_det(&self) -> bool {
        (self.bits & IcIntrStat::R_STOP_DET_MASK) != 0
    }

    /// Read **R_ACTIVITY** `[8:8]`
    ///
    /// See IC_RAW_INTR_STAT for a detailed description of R_ACTIVITY bit.
    /// Reset value: 0x0
    #[inline(always)]
    pub const fn r_activity(&self) -> bool {
        (self.bits & IcIntrStat::R_ACTIVITY_MASK) != 0
    }

    /// Read **R_RX_DONE** `[7:7]`
    ///
    /// See IC_RAW_INTR_STAT for a detailed description of R_RX_DONE bit.
    /// Reset value: 0x0
    #[inline(always)]
    pub const fn r_rx_done(&self) -> bool {
        (self.bits & IcIntrStat::R_RX_DONE_MASK) != 0
    }

    /// Read **R_TX_ABRT** `[6:6]`
    ///
    /// See IC_RAW_INTR_STAT for a detailed description of R_TX_ABRT bit.
    /// Reset value: 0x0
    #[inline(always)]
    pub const fn r_tx_abrt(&self) -> bool {
        (self.bits & IcIntrStat::R_TX_ABRT_MASK) != 0
    }

    /// Read **R_RD_REQ** `[5:5]`
    ///
    /// See IC_RAW_INTR_STAT for a detailed description of R_RD_REQ bit.
    /// Reset value: 0x0
    #[inline(always)]
    pub const fn r_rd_req(&self) -> bool {
        (self.bits & IcIntrStat::R_RD_REQ_MASK) != 0
    }

    /// Read **R_TX_EMPTY** `[4:4]`
    ///
    /// See IC_RAW_INTR_STAT for a detailed description of R_TX_EMPTY bit.
    /// Reset value: 0x0
    #[inline(always)]
    pub const fn r_tx_empty(&self) -> bool {
        (self.bits & IcIntrStat::R_TX_EMPTY_MASK) != 0
    }

    /// Read **R_TX_OVER** `[3:3]`
    ///
    /// See IC_RAW_INTR_STAT for a detailed description of R_TX_OVER bit.
    /// Reset value: 0x0
    #[inline(always)]
    pub const fn r_tx_over(&self) -> bool {
        (self.bits & IcIntrStat::R_TX_OVER_MASK) != 0
    }

    /// Read **R_RX_FULL** `[2:2]`
    ///
    /// See IC_RAW_INTR_STAT for a detailed description of R_RX_FULL bit.
    /// Reset value: 0x0
    #[inline(always)]
    pub const fn r_rx_full(&self) -> bool {
        (self.bits & IcIntrStat::R_RX_FULL_MASK) != 0
    }

    /// Read **R_RX_OVER** `[1:1]`
    ///
    /// See IC_RAW_INTR_STAT for a detailed description of R_RX_OVER bit.
    /// Reset value: 0x0
    #[inline(always)]
    pub const fn r_rx_over(&self) -> bool {
        (self.bits & IcIntrStat::R_RX_OVER_MASK) != 0
    }

    /// Read **R_RX_UNDER** `[0:0]`
    ///
    /// See IC_RAW_INTR_STAT for a detailed description of R_RX_UNDER bit.
    /// Reset value: 0x0
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
    pub fn read_val(&self) -> IcIntrStatVal {
        IcIntrStatVal::from_bits(self.value.read_bits())
    }
}

/// IC_INTR_MASK Register
///
/// I2C Interrupt Mask Register.
/// These bits mask their corresponding interrupt status bits. This register is active low; a value of 0 masks the interrupt, whereas a value of 1 unmasks the interrupt.
///
/// Address offset: 0x0030
/// Reset value: 0x000008ff
///
/// ## Fields
///
/// - **M_RESTART_DET** `[12]` (RW)
///
///   This bit masks the R_RESTART_DET interrupt in IC_INTR_STAT register.
///   Reset value: 0x0
///
/// - **M_GEN_CALL** `[11]` (RW)
///
///   This bit masks the R_GEN_CALL interrupt in IC_INTR_STAT register.
///   Reset value: 0x1
///
/// - **M_START_DET** `[10]` (RW)
///
///   This bit masks the R_START_DET interrupt in IC_INTR_STAT register.
///   Reset value: 0x0
///
/// - **M_STOP_DET** `[9]` (RW)
///
///   This bit masks the R_STOP_DET interrupt in IC_INTR_STAT register.
///   Reset value: 0x0
///
/// - **M_ACTIVITY** `[8]` (RW)
///
///   This bit masks the R_ACTIVITY interrupt in IC_INTR_STAT register.
///   Reset value: 0x0
///
/// - **M_RX_DONE** `[7]` (RW)
///
///   This bit masks the R_RX_DONE interrupt in IC_INTR_STAT register.
///   Reset value: 0x1
///
/// - **M_TX_ABRT** `[6]` (RW)
///
///   This bit masks the R_TX_ABRT interrupt in IC_INTR_STAT register.
///   Reset value: 0x1
///
/// - **M_RD_REQ** `[5]` (RW)
///
///   This bit masks the R_RD_REQ interrupt in IC_INTR_STAT register.
///   Reset value: 0x1
///
/// - **M_TX_EMPTY** `[4]` (RW)
///
///   This bit masks the R_TX_EMPTY interrupt in IC_INTR_STAT register.
///   Reset value: 0x1
///
/// - **M_TX_OVER** `[3]` (RW)
///
///   This bit masks the R_TX_OVER interrupt in IC_INTR_STAT register.
///   Reset value: 0x1
///
/// - **M_RX_FULL** `[2]` (RW)
///
///   This bit masks the R_RX_FULL interrupt in IC_INTR_STAT register.
///   Reset value: 0x1
///
/// - **M_RX_OVER** `[1]` (RW)
///
///   This bit masks the R_RX_OVER interrupt in IC_INTR_STAT register.
///   Reset value: 0x1
///
/// - **M_RX_UNDER** `[0]` (RW)
///
///   This bit masks the R_RX_UNDER interrupt in IC_INTR_STAT register.
///   Reset value: 0x1
#[repr(transparent)]
pub struct IcIntrMask {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for IcIntrMask {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl IcIntrMask {
    // ======== Field Constants ========

    /// **M_RESTART_DET**
    ///
    /// This bit masks the R_RESTART_DET interrupt in IC_INTR_STAT register.
    /// Reset value: 0x0
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const M_RESTART_DET_OFFSET: u32 = 12;
    pub const M_RESTART_DET_MASK: u32 = 0x00001000;
    pub const M_RESTART_DET: u32 = 1 << 12;
    /// ENABLED = 0
    /// RESTART_DET interrupt is masked
    pub const M_RESTART_DET_ENABLED: u32 = 0 << 12;
    pub const M_RESTART_DET_ENABLED_VAL: u32 = 0;
    /// DISABLED = 1
    /// RESTART_DET interrupt is unmasked
    pub const M_RESTART_DET_DISABLED: u32 = 1 << 12;
    pub const M_RESTART_DET_DISABLED_VAL: u32 = 1;

    /// **M_GEN_CALL**
    ///
    /// This bit masks the R_GEN_CALL interrupt in IC_INTR_STAT register.
    /// Reset value: 0x1
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const M_GEN_CALL_OFFSET: u32 = 11;
    pub const M_GEN_CALL_MASK: u32 = 0x00000800;
    pub const M_GEN_CALL: u32 = 1 << 11;
    /// ENABLED = 0
    /// GEN_CALL interrupt is masked
    pub const M_GEN_CALL_ENABLED: u32 = 0 << 11;
    pub const M_GEN_CALL_ENABLED_VAL: u32 = 0;
    /// DISABLED = 1
    /// GEN_CALL interrupt is unmasked
    pub const M_GEN_CALL_DISABLED: u32 = 1 << 11;
    pub const M_GEN_CALL_DISABLED_VAL: u32 = 1;

    /// **M_START_DET**
    ///
    /// This bit masks the R_START_DET interrupt in IC_INTR_STAT register.
    /// Reset value: 0x0
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const M_START_DET_OFFSET: u32 = 10;
    pub const M_START_DET_MASK: u32 = 0x00000400;
    pub const M_START_DET: u32 = 1 << 10;
    /// ENABLED = 0
    /// START_DET interrupt is masked
    pub const M_START_DET_ENABLED: u32 = 0 << 10;
    pub const M_START_DET_ENABLED_VAL: u32 = 0;
    /// DISABLED = 1
    /// START_DET interrupt is unmasked
    pub const M_START_DET_DISABLED: u32 = 1 << 10;
    pub const M_START_DET_DISABLED_VAL: u32 = 1;

    /// **M_STOP_DET**
    ///
    /// This bit masks the R_STOP_DET interrupt in IC_INTR_STAT register.
    /// Reset value: 0x0
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const M_STOP_DET_OFFSET: u32 = 9;
    pub const M_STOP_DET_MASK: u32 = 0x00000200;
    pub const M_STOP_DET: u32 = 1 << 9;
    /// ENABLED = 0
    /// STOP_DET interrupt is masked
    pub const M_STOP_DET_ENABLED: u32 = 0 << 9;
    pub const M_STOP_DET_ENABLED_VAL: u32 = 0;
    /// DISABLED = 1
    /// STOP_DET interrupt is unmasked
    pub const M_STOP_DET_DISABLED: u32 = 1 << 9;
    pub const M_STOP_DET_DISABLED_VAL: u32 = 1;

    /// **M_ACTIVITY**
    ///
    /// This bit masks the R_ACTIVITY interrupt in IC_INTR_STAT register.
    /// Reset value: 0x0
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const M_ACTIVITY_OFFSET: u32 = 8;
    pub const M_ACTIVITY_MASK: u32 = 0x00000100;
    pub const M_ACTIVITY: u32 = 1 << 8;
    /// ENABLED = 0
    /// ACTIVITY interrupt is masked
    pub const M_ACTIVITY_ENABLED: u32 = 0 << 8;
    pub const M_ACTIVITY_ENABLED_VAL: u32 = 0;
    /// DISABLED = 1
    /// ACTIVITY interrupt is unmasked
    pub const M_ACTIVITY_DISABLED: u32 = 1 << 8;
    pub const M_ACTIVITY_DISABLED_VAL: u32 = 1;

    /// **M_RX_DONE**
    ///
    /// This bit masks the R_RX_DONE interrupt in IC_INTR_STAT register.
    /// Reset value: 0x1
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const M_RX_DONE_OFFSET: u32 = 7;
    pub const M_RX_DONE_MASK: u32 = 0x00000080;
    pub const M_RX_DONE: u32 = 1 << 7;
    /// ENABLED = 0
    /// RX_DONE interrupt is masked
    pub const M_RX_DONE_ENABLED: u32 = 0 << 7;
    pub const M_RX_DONE_ENABLED_VAL: u32 = 0;
    /// DISABLED = 1
    /// RX_DONE interrupt is unmasked
    pub const M_RX_DONE_DISABLED: u32 = 1 << 7;
    pub const M_RX_DONE_DISABLED_VAL: u32 = 1;

    /// **M_TX_ABRT**
    ///
    /// This bit masks the R_TX_ABRT interrupt in IC_INTR_STAT register.
    /// Reset value: 0x1
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const M_TX_ABRT_OFFSET: u32 = 6;
    pub const M_TX_ABRT_MASK: u32 = 0x00000040;
    pub const M_TX_ABRT: u32 = 1 << 6;
    /// ENABLED = 0
    /// TX_ABORT interrupt is masked
    pub const M_TX_ABRT_ENABLED: u32 = 0 << 6;
    pub const M_TX_ABRT_ENABLED_VAL: u32 = 0;
    /// DISABLED = 1
    /// TX_ABORT interrupt is unmasked
    pub const M_TX_ABRT_DISABLED: u32 = 1 << 6;
    pub const M_TX_ABRT_DISABLED_VAL: u32 = 1;

    /// **M_RD_REQ**
    ///
    /// This bit masks the R_RD_REQ interrupt in IC_INTR_STAT register.
    /// Reset value: 0x1
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const M_RD_REQ_OFFSET: u32 = 5;
    pub const M_RD_REQ_MASK: u32 = 0x00000020;
    pub const M_RD_REQ: u32 = 1 << 5;
    /// ENABLED = 0
    /// RD_REQ interrupt is masked
    pub const M_RD_REQ_ENABLED: u32 = 0 << 5;
    pub const M_RD_REQ_ENABLED_VAL: u32 = 0;
    /// DISABLED = 1
    /// RD_REQ interrupt is unmasked
    pub const M_RD_REQ_DISABLED: u32 = 1 << 5;
    pub const M_RD_REQ_DISABLED_VAL: u32 = 1;

    /// **M_TX_EMPTY**
    ///
    /// This bit masks the R_TX_EMPTY interrupt in IC_INTR_STAT register.
    /// Reset value: 0x1
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const M_TX_EMPTY_OFFSET: u32 = 4;
    pub const M_TX_EMPTY_MASK: u32 = 0x00000010;
    pub const M_TX_EMPTY: u32 = 1 << 4;
    /// ENABLED = 0
    /// TX_EMPTY interrupt is masked
    pub const M_TX_EMPTY_ENABLED: u32 = 0 << 4;
    pub const M_TX_EMPTY_ENABLED_VAL: u32 = 0;
    /// DISABLED = 1
    /// TX_EMPTY interrupt is unmasked
    pub const M_TX_EMPTY_DISABLED: u32 = 1 << 4;
    pub const M_TX_EMPTY_DISABLED_VAL: u32 = 1;

    /// **M_TX_OVER**
    ///
    /// This bit masks the R_TX_OVER interrupt in IC_INTR_STAT register.
    /// Reset value: 0x1
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const M_TX_OVER_OFFSET: u32 = 3;
    pub const M_TX_OVER_MASK: u32 = 0x00000008;
    pub const M_TX_OVER: u32 = 1 << 3;
    /// ENABLED = 0
    /// TX_OVER interrupt is masked
    pub const M_TX_OVER_ENABLED: u32 = 0 << 3;
    pub const M_TX_OVER_ENABLED_VAL: u32 = 0;
    /// DISABLED = 1
    /// TX_OVER interrupt is unmasked
    pub const M_TX_OVER_DISABLED: u32 = 1 << 3;
    pub const M_TX_OVER_DISABLED_VAL: u32 = 1;

    /// **M_RX_FULL**
    ///
    /// This bit masks the R_RX_FULL interrupt in IC_INTR_STAT register.
    /// Reset value: 0x1
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const M_RX_FULL_OFFSET: u32 = 2;
    pub const M_RX_FULL_MASK: u32 = 0x00000004;
    pub const M_RX_FULL: u32 = 1 << 2;
    /// ENABLED = 0
    /// RX_FULL interrupt is masked
    pub const M_RX_FULL_ENABLED: u32 = 0 << 2;
    pub const M_RX_FULL_ENABLED_VAL: u32 = 0;
    /// DISABLED = 1
    /// RX_FULL interrupt is unmasked
    pub const M_RX_FULL_DISABLED: u32 = 1 << 2;
    pub const M_RX_FULL_DISABLED_VAL: u32 = 1;

    /// **M_RX_OVER**
    ///
    /// This bit masks the R_RX_OVER interrupt in IC_INTR_STAT register.
    /// Reset value: 0x1
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const M_RX_OVER_OFFSET: u32 = 1;
    pub const M_RX_OVER_MASK: u32 = 0x00000002;
    pub const M_RX_OVER: u32 = 1 << 1;
    /// ENABLED = 0
    /// RX_OVER interrupt is masked
    pub const M_RX_OVER_ENABLED: u32 = 0 << 1;
    pub const M_RX_OVER_ENABLED_VAL: u32 = 0;
    /// DISABLED = 1
    /// RX_OVER interrupt is unmasked
    pub const M_RX_OVER_DISABLED: u32 = 1 << 1;
    pub const M_RX_OVER_DISABLED_VAL: u32 = 1;

    /// **M_RX_UNDER**
    ///
    /// This bit masks the R_RX_UNDER interrupt in IC_INTR_STAT register.
    /// Reset value: 0x1
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const M_RX_UNDER_OFFSET: u32 = 0;
    pub const M_RX_UNDER_MASK: u32 = 0x00000001;
    pub const M_RX_UNDER: u32 = 1 << 0;
    /// ENABLED = 0
    /// RX_UNDER interrupt is masked
    pub const M_RX_UNDER_ENABLED: u32 = 0 << 0;
    pub const M_RX_UNDER_ENABLED_VAL: u32 = 0;
    /// DISABLED = 1
    /// RX_UNDER interrupt is unmasked
    pub const M_RX_UNDER_DISABLED: u32 = 1 << 0;
    pub const M_RX_UNDER_DISABLED_VAL: u32 = 1;

    // ======== Field Accessors ========

    /// **M_RESTART_DET** `[12]` (RW)
    ///
    /// This bit masks the R_RESTART_DET interrupt in IC_INTR_STAT register.
    /// Reset value: 0x0
    #[inline(always)]
    pub fn m_restart_det(&self) -> bool {
        (self.value.read_bits() & Self::M_RESTART_DET_MASK) != 0
    }

    /// Set M_RESTART_DET bit atomically (RW)
    #[inline(always)]
    pub fn set_m_restart_det(&self) {
        self.set_bits(Self::M_RESTART_DET_MASK);
    }

    /// Clear M_RESTART_DET bit atomically (RW)
    #[inline(always)]
    pub fn clr_m_restart_det(&self) {
        self.clr_bits(Self::M_RESTART_DET_MASK);
    }

    /// **M_GEN_CALL** `[11]` (RW)
    ///
    /// This bit masks the R_GEN_CALL interrupt in IC_INTR_STAT register.
    /// Reset value: 0x1
    #[inline(always)]
    pub fn m_gen_call(&self) -> bool {
        (self.value.read_bits() & Self::M_GEN_CALL_MASK) != 0
    }

    /// Set M_GEN_CALL bit atomically (RW)
    #[inline(always)]
    pub fn set_m_gen_call(&self) {
        self.set_bits(Self::M_GEN_CALL_MASK);
    }

    /// Clear M_GEN_CALL bit atomically (RW)
    #[inline(always)]
    pub fn clr_m_gen_call(&self) {
        self.clr_bits(Self::M_GEN_CALL_MASK);
    }

    /// **M_START_DET** `[10]` (RW)
    ///
    /// This bit masks the R_START_DET interrupt in IC_INTR_STAT register.
    /// Reset value: 0x0
    #[inline(always)]
    pub fn m_start_det(&self) -> bool {
        (self.value.read_bits() & Self::M_START_DET_MASK) != 0
    }

    /// Set M_START_DET bit atomically (RW)
    #[inline(always)]
    pub fn set_m_start_det(&self) {
        self.set_bits(Self::M_START_DET_MASK);
    }

    /// Clear M_START_DET bit atomically (RW)
    #[inline(always)]
    pub fn clr_m_start_det(&self) {
        self.clr_bits(Self::M_START_DET_MASK);
    }

    /// **M_STOP_DET** `[9]` (RW)
    ///
    /// This bit masks the R_STOP_DET interrupt in IC_INTR_STAT register.
    /// Reset value: 0x0
    #[inline(always)]
    pub fn m_stop_det(&self) -> bool {
        (self.value.read_bits() & Self::M_STOP_DET_MASK) != 0
    }

    /// Set M_STOP_DET bit atomically (RW)
    #[inline(always)]
    pub fn set_m_stop_det(&self) {
        self.set_bits(Self::M_STOP_DET_MASK);
    }

    /// Clear M_STOP_DET bit atomically (RW)
    #[inline(always)]
    pub fn clr_m_stop_det(&self) {
        self.clr_bits(Self::M_STOP_DET_MASK);
    }

    /// **M_ACTIVITY** `[8]` (RW)
    ///
    /// This bit masks the R_ACTIVITY interrupt in IC_INTR_STAT register.
    /// Reset value: 0x0
    #[inline(always)]
    pub fn m_activity(&self) -> bool {
        (self.value.read_bits() & Self::M_ACTIVITY_MASK) != 0
    }

    /// Set M_ACTIVITY bit atomically (RW)
    #[inline(always)]
    pub fn set_m_activity(&self) {
        self.set_bits(Self::M_ACTIVITY_MASK);
    }

    /// Clear M_ACTIVITY bit atomically (RW)
    #[inline(always)]
    pub fn clr_m_activity(&self) {
        self.clr_bits(Self::M_ACTIVITY_MASK);
    }

    /// **M_RX_DONE** `[7]` (RW)
    ///
    /// This bit masks the R_RX_DONE interrupt in IC_INTR_STAT register.
    /// Reset value: 0x1
    #[inline(always)]
    pub fn m_rx_done(&self) -> bool {
        (self.value.read_bits() & Self::M_RX_DONE_MASK) != 0
    }

    /// Set M_RX_DONE bit atomically (RW)
    #[inline(always)]
    pub fn set_m_rx_done(&self) {
        self.set_bits(Self::M_RX_DONE_MASK);
    }

    /// Clear M_RX_DONE bit atomically (RW)
    #[inline(always)]
    pub fn clr_m_rx_done(&self) {
        self.clr_bits(Self::M_RX_DONE_MASK);
    }

    /// **M_TX_ABRT** `[6]` (RW)
    ///
    /// This bit masks the R_TX_ABRT interrupt in IC_INTR_STAT register.
    /// Reset value: 0x1
    #[inline(always)]
    pub fn m_tx_abrt(&self) -> bool {
        (self.value.read_bits() & Self::M_TX_ABRT_MASK) != 0
    }

    /// Set M_TX_ABRT bit atomically (RW)
    #[inline(always)]
    pub fn set_m_tx_abrt(&self) {
        self.set_bits(Self::M_TX_ABRT_MASK);
    }

    /// Clear M_TX_ABRT bit atomically (RW)
    #[inline(always)]
    pub fn clr_m_tx_abrt(&self) {
        self.clr_bits(Self::M_TX_ABRT_MASK);
    }

    /// **M_RD_REQ** `[5]` (RW)
    ///
    /// This bit masks the R_RD_REQ interrupt in IC_INTR_STAT register.
    /// Reset value: 0x1
    #[inline(always)]
    pub fn m_rd_req(&self) -> bool {
        (self.value.read_bits() & Self::M_RD_REQ_MASK) != 0
    }

    /// Set M_RD_REQ bit atomically (RW)
    #[inline(always)]
    pub fn set_m_rd_req(&self) {
        self.set_bits(Self::M_RD_REQ_MASK);
    }

    /// Clear M_RD_REQ bit atomically (RW)
    #[inline(always)]
    pub fn clr_m_rd_req(&self) {
        self.clr_bits(Self::M_RD_REQ_MASK);
    }

    /// **M_TX_EMPTY** `[4]` (RW)
    ///
    /// This bit masks the R_TX_EMPTY interrupt in IC_INTR_STAT register.
    /// Reset value: 0x1
    #[inline(always)]
    pub fn m_tx_empty(&self) -> bool {
        (self.value.read_bits() & Self::M_TX_EMPTY_MASK) != 0
    }

    /// Set M_TX_EMPTY bit atomically (RW)
    #[inline(always)]
    pub fn set_m_tx_empty(&self) {
        self.set_bits(Self::M_TX_EMPTY_MASK);
    }

    /// Clear M_TX_EMPTY bit atomically (RW)
    #[inline(always)]
    pub fn clr_m_tx_empty(&self) {
        self.clr_bits(Self::M_TX_EMPTY_MASK);
    }

    /// **M_TX_OVER** `[3]` (RW)
    ///
    /// This bit masks the R_TX_OVER interrupt in IC_INTR_STAT register.
    /// Reset value: 0x1
    #[inline(always)]
    pub fn m_tx_over(&self) -> bool {
        (self.value.read_bits() & Self::M_TX_OVER_MASK) != 0
    }

    /// Set M_TX_OVER bit atomically (RW)
    #[inline(always)]
    pub fn set_m_tx_over(&self) {
        self.set_bits(Self::M_TX_OVER_MASK);
    }

    /// Clear M_TX_OVER bit atomically (RW)
    #[inline(always)]
    pub fn clr_m_tx_over(&self) {
        self.clr_bits(Self::M_TX_OVER_MASK);
    }

    /// **M_RX_FULL** `[2]` (RW)
    ///
    /// This bit masks the R_RX_FULL interrupt in IC_INTR_STAT register.
    /// Reset value: 0x1
    #[inline(always)]
    pub fn m_rx_full(&self) -> bool {
        (self.value.read_bits() & Self::M_RX_FULL_MASK) != 0
    }

    /// Set M_RX_FULL bit atomically (RW)
    #[inline(always)]
    pub fn set_m_rx_full(&self) {
        self.set_bits(Self::M_RX_FULL_MASK);
    }

    /// Clear M_RX_FULL bit atomically (RW)
    #[inline(always)]
    pub fn clr_m_rx_full(&self) {
        self.clr_bits(Self::M_RX_FULL_MASK);
    }

    /// **M_RX_OVER** `[1]` (RW)
    ///
    /// This bit masks the R_RX_OVER interrupt in IC_INTR_STAT register.
    /// Reset value: 0x1
    #[inline(always)]
    pub fn m_rx_over(&self) -> bool {
        (self.value.read_bits() & Self::M_RX_OVER_MASK) != 0
    }

    /// Set M_RX_OVER bit atomically (RW)
    #[inline(always)]
    pub fn set_m_rx_over(&self) {
        self.set_bits(Self::M_RX_OVER_MASK);
    }

    /// Clear M_RX_OVER bit atomically (RW)
    #[inline(always)]
    pub fn clr_m_rx_over(&self) {
        self.clr_bits(Self::M_RX_OVER_MASK);
    }

    /// **M_RX_UNDER** `[0]` (RW)
    ///
    /// This bit masks the R_RX_UNDER interrupt in IC_INTR_STAT register.
    /// Reset value: 0x1
    #[inline(always)]
    pub fn m_rx_under(&self) -> bool {
        (self.value.read_bits() & Self::M_RX_UNDER_MASK) != 0
    }

    /// Set M_RX_UNDER bit atomically (RW)
    #[inline(always)]
    pub fn set_m_rx_under(&self) {
        self.set_bits(Self::M_RX_UNDER_MASK);
    }

    /// Clear M_RX_UNDER bit atomically (RW)
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

    /// Read **M_RESTART_DET** `[12:12]`
    ///
    /// This bit masks the R_RESTART_DET interrupt in IC_INTR_STAT register.
    /// Reset value: 0x0
    #[inline(always)]
    pub const fn m_restart_det(&self) -> bool {
        (self.bits & IcIntrMask::M_RESTART_DET_MASK) != 0
    }

    /// Read **M_GEN_CALL** `[11:11]`
    ///
    /// This bit masks the R_GEN_CALL interrupt in IC_INTR_STAT register.
    /// Reset value: 0x1
    #[inline(always)]
    pub const fn m_gen_call(&self) -> bool {
        (self.bits & IcIntrMask::M_GEN_CALL_MASK) != 0
    }

    /// Read **M_START_DET** `[10:10]`
    ///
    /// This bit masks the R_START_DET interrupt in IC_INTR_STAT register.
    /// Reset value: 0x0
    #[inline(always)]
    pub const fn m_start_det(&self) -> bool {
        (self.bits & IcIntrMask::M_START_DET_MASK) != 0
    }

    /// Read **M_STOP_DET** `[9:9]`
    ///
    /// This bit masks the R_STOP_DET interrupt in IC_INTR_STAT register.
    /// Reset value: 0x0
    #[inline(always)]
    pub const fn m_stop_det(&self) -> bool {
        (self.bits & IcIntrMask::M_STOP_DET_MASK) != 0
    }

    /// Read **M_ACTIVITY** `[8:8]`
    ///
    /// This bit masks the R_ACTIVITY interrupt in IC_INTR_STAT register.
    /// Reset value: 0x0
    #[inline(always)]
    pub const fn m_activity(&self) -> bool {
        (self.bits & IcIntrMask::M_ACTIVITY_MASK) != 0
    }

    /// Read **M_RX_DONE** `[7:7]`
    ///
    /// This bit masks the R_RX_DONE interrupt in IC_INTR_STAT register.
    /// Reset value: 0x1
    #[inline(always)]
    pub const fn m_rx_done(&self) -> bool {
        (self.bits & IcIntrMask::M_RX_DONE_MASK) != 0
    }

    /// Read **M_TX_ABRT** `[6:6]`
    ///
    /// This bit masks the R_TX_ABRT interrupt in IC_INTR_STAT register.
    /// Reset value: 0x1
    #[inline(always)]
    pub const fn m_tx_abrt(&self) -> bool {
        (self.bits & IcIntrMask::M_TX_ABRT_MASK) != 0
    }

    /// Read **M_RD_REQ** `[5:5]`
    ///
    /// This bit masks the R_RD_REQ interrupt in IC_INTR_STAT register.
    /// Reset value: 0x1
    #[inline(always)]
    pub const fn m_rd_req(&self) -> bool {
        (self.bits & IcIntrMask::M_RD_REQ_MASK) != 0
    }

    /// Read **M_TX_EMPTY** `[4:4]`
    ///
    /// This bit masks the R_TX_EMPTY interrupt in IC_INTR_STAT register.
    /// Reset value: 0x1
    #[inline(always)]
    pub const fn m_tx_empty(&self) -> bool {
        (self.bits & IcIntrMask::M_TX_EMPTY_MASK) != 0
    }

    /// Read **M_TX_OVER** `[3:3]`
    ///
    /// This bit masks the R_TX_OVER interrupt in IC_INTR_STAT register.
    /// Reset value: 0x1
    #[inline(always)]
    pub const fn m_tx_over(&self) -> bool {
        (self.bits & IcIntrMask::M_TX_OVER_MASK) != 0
    }

    /// Read **M_RX_FULL** `[2:2]`
    ///
    /// This bit masks the R_RX_FULL interrupt in IC_INTR_STAT register.
    /// Reset value: 0x1
    #[inline(always)]
    pub const fn m_rx_full(&self) -> bool {
        (self.bits & IcIntrMask::M_RX_FULL_MASK) != 0
    }

    /// Read **M_RX_OVER** `[1:1]`
    ///
    /// This bit masks the R_RX_OVER interrupt in IC_INTR_STAT register.
    /// Reset value: 0x1
    #[inline(always)]
    pub const fn m_rx_over(&self) -> bool {
        (self.bits & IcIntrMask::M_RX_OVER_MASK) != 0
    }

    /// Read **M_RX_UNDER** `[0:0]`
    ///
    /// This bit masks the R_RX_UNDER interrupt in IC_INTR_STAT register.
    /// Reset value: 0x1
    #[inline(always)]
    pub const fn m_rx_under(&self) -> bool {
        (self.bits & IcIntrMask::M_RX_UNDER_MASK) != 0
    }

    /// Set **M_RESTART_DET** `[12:12]`
    ///
    /// This bit masks the R_RESTART_DET interrupt in IC_INTR_STAT register.
    /// Reset value: 0x0
    #[inline(always)]
    pub const fn with_m_restart_det(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IcIntrMask::M_RESTART_DET_MASK
            } else {
                self.bits & !IcIntrMask::M_RESTART_DET_MASK
            },
        }
    }

    /// Set **M_GEN_CALL** `[11:11]`
    ///
    /// This bit masks the R_GEN_CALL interrupt in IC_INTR_STAT register.
    /// Reset value: 0x1
    #[inline(always)]
    pub const fn with_m_gen_call(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IcIntrMask::M_GEN_CALL_MASK
            } else {
                self.bits & !IcIntrMask::M_GEN_CALL_MASK
            },
        }
    }

    /// Set **M_START_DET** `[10:10]`
    ///
    /// This bit masks the R_START_DET interrupt in IC_INTR_STAT register.
    /// Reset value: 0x0
    #[inline(always)]
    pub const fn with_m_start_det(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IcIntrMask::M_START_DET_MASK
            } else {
                self.bits & !IcIntrMask::M_START_DET_MASK
            },
        }
    }

    /// Set **M_STOP_DET** `[9:9]`
    ///
    /// This bit masks the R_STOP_DET interrupt in IC_INTR_STAT register.
    /// Reset value: 0x0
    #[inline(always)]
    pub const fn with_m_stop_det(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IcIntrMask::M_STOP_DET_MASK
            } else {
                self.bits & !IcIntrMask::M_STOP_DET_MASK
            },
        }
    }

    /// Set **M_ACTIVITY** `[8:8]`
    ///
    /// This bit masks the R_ACTIVITY interrupt in IC_INTR_STAT register.
    /// Reset value: 0x0
    #[inline(always)]
    pub const fn with_m_activity(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IcIntrMask::M_ACTIVITY_MASK
            } else {
                self.bits & !IcIntrMask::M_ACTIVITY_MASK
            },
        }
    }

    /// Set **M_RX_DONE** `[7:7]`
    ///
    /// This bit masks the R_RX_DONE interrupt in IC_INTR_STAT register.
    /// Reset value: 0x1
    #[inline(always)]
    pub const fn with_m_rx_done(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IcIntrMask::M_RX_DONE_MASK
            } else {
                self.bits & !IcIntrMask::M_RX_DONE_MASK
            },
        }
    }

    /// Set **M_TX_ABRT** `[6:6]`
    ///
    /// This bit masks the R_TX_ABRT interrupt in IC_INTR_STAT register.
    /// Reset value: 0x1
    #[inline(always)]
    pub const fn with_m_tx_abrt(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IcIntrMask::M_TX_ABRT_MASK
            } else {
                self.bits & !IcIntrMask::M_TX_ABRT_MASK
            },
        }
    }

    /// Set **M_RD_REQ** `[5:5]`
    ///
    /// This bit masks the R_RD_REQ interrupt in IC_INTR_STAT register.
    /// Reset value: 0x1
    #[inline(always)]
    pub const fn with_m_rd_req(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IcIntrMask::M_RD_REQ_MASK
            } else {
                self.bits & !IcIntrMask::M_RD_REQ_MASK
            },
        }
    }

    /// Set **M_TX_EMPTY** `[4:4]`
    ///
    /// This bit masks the R_TX_EMPTY interrupt in IC_INTR_STAT register.
    /// Reset value: 0x1
    #[inline(always)]
    pub const fn with_m_tx_empty(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IcIntrMask::M_TX_EMPTY_MASK
            } else {
                self.bits & !IcIntrMask::M_TX_EMPTY_MASK
            },
        }
    }

    /// Set **M_TX_OVER** `[3:3]`
    ///
    /// This bit masks the R_TX_OVER interrupt in IC_INTR_STAT register.
    /// Reset value: 0x1
    #[inline(always)]
    pub const fn with_m_tx_over(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IcIntrMask::M_TX_OVER_MASK
            } else {
                self.bits & !IcIntrMask::M_TX_OVER_MASK
            },
        }
    }

    /// Set **M_RX_FULL** `[2:2]`
    ///
    /// This bit masks the R_RX_FULL interrupt in IC_INTR_STAT register.
    /// Reset value: 0x1
    #[inline(always)]
    pub const fn with_m_rx_full(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IcIntrMask::M_RX_FULL_MASK
            } else {
                self.bits & !IcIntrMask::M_RX_FULL_MASK
            },
        }
    }

    /// Set **M_RX_OVER** `[1:1]`
    ///
    /// This bit masks the R_RX_OVER interrupt in IC_INTR_STAT register.
    /// Reset value: 0x1
    #[inline(always)]
    pub const fn with_m_rx_over(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IcIntrMask::M_RX_OVER_MASK
            } else {
                self.bits & !IcIntrMask::M_RX_OVER_MASK
            },
        }
    }

    /// Set **M_RX_UNDER** `[0:0]`
    ///
    /// This bit masks the R_RX_UNDER interrupt in IC_INTR_STAT register.
    /// Reset value: 0x1
    #[inline(always)]
    pub const fn with_m_rx_under(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IcIntrMask::M_RX_UNDER_MASK
            } else {
                self.bits & !IcIntrMask::M_RX_UNDER_MASK
            },
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
    pub fn read_val(&self) -> IcIntrMaskVal {
        IcIntrMaskVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: IcIntrMaskVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(IcIntrMaskVal) -> IcIntrMaskVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// IC_RAW_INTR_STAT Register
///
/// I2C Raw Interrupt Status Register
/// Unlike the IC_INTR_STAT register, these bits are not masked so they always show the true status of the DW_apb_i2c.
///
/// Address offset: 0x0034
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **RESTART_DET** `[12]` (RO)
///
///   Indicates whether a RESTART condition has occurred on the I2C interface when DW_apb_i2c is operating in Slave mode and the slave is being addressed. Enabled only when IC_SLV_RESTART_DET_EN=1.
///   Note: However, in high-speed mode or during a START BYTE transfer, the RESTART comes before the address field as per the I2C protocol. In this case, the slave is not the addressed slave when the RESTART is issued, therefore DW_apb_i2c does not generate the RESTART_DET interrupt.
///   Reset value: 0x0
///
/// - **GEN_CALL** `[11]` (RO)
///
///   Set only when a General Call address is received and it is acknowledged. It stays set until it is cleared either by disabling DW_apb_i2c or when the CPU reads bit 0 of the IC_CLR_GEN_CALL register. DW_apb_i2c stores the received data in the Rx buffer.
///   Reset value: 0x0
///
/// - **START_DET** `[10]` (RO)
///
///   Indicates whether a START or RESTART condition has occurred on the I2C interface regardless of whether DW_apb_i2c is operating in slave or master mode.
///   Reset value: 0x0
///
/// - **STOP_DET** `[9]` (RO)
///
///   Indicates whether a STOP condition has occurred on the I2C interface regardless of whether DW_apb_i2c is operating in slave or master mode.
///   In Slave Mode: - If IC_CON[7]=1'b1 (STOP_DET_IFADDRESSED), the STOP_DET interrupt will be issued only if slave is addressed. Note: During a general call address, this slave does not issue a STOP_DET interrupt if STOP_DET_IF_ADDRESSED=1'b1, even if the slave responds to the general call address by generating ACK. The STOP_DET interrupt is generated only when the transmitted address matches the slave address (SAR). - If IC_CON[7]=1'b0 (STOP_DET_IFADDRESSED), the STOP_DET interrupt is issued irrespective of whether it is being addressed. In Master Mode: - If IC_CON[10]=1'b1 (STOP_DET_IF_MASTER_ACTIVE),the STOP_DET interrupt will be issued only if Master is active. - If IC_CON[10]=1'b0 (STOP_DET_IFADDRESSED),the STOP_DET interrupt will be issued irrespective of whether master is active or not. Reset value: 0x0
///
/// - **ACTIVITY** `[8]` (RO)
///
///   This bit captures DW_apb_i2c activity and stays set until it is cleared. There are four ways to clear it: - Disabling the DW_apb_i2c - Reading the IC_CLR_ACTIVITY register - Reading the IC_CLR_INTR register - System reset Once this bit is set, it stays set unless one of the four methods is used to clear it. Even if the DW_apb_i2c module is idle, this bit remains set until cleared, indicating that there was activity on the bus.
///   Reset value: 0x0
///
/// - **RX_DONE** `[7]` (RO)
///
///   When the DW_apb_i2c is acting as a slave-transmitter, this bit is set to 1 if the master does not acknowledge a transmitted byte. This occurs on the last byte of the transmission, indicating that the transmission is done.
///   Reset value: 0x0
///
/// - **TX_ABRT** `[6]` (RO)
///
///   This bit indicates if DW_apb_i2c, as an I2C transmitter, is unable to complete the intended actions on the contents of the transmit FIFO. This situation can occur both as an I2C master or an I2C slave, and is referred to as a 'transmit abort'. When this bit is set to 1, the IC_TX_ABRT_SOURCE register indicates the reason why the transmit abort takes places.
///   Note: The DW_apb_i2c flushes/resets/empties the TX_FIFO and RX_FIFO whenever there is a transmit abort caused by any of the events tracked by the IC_TX_ABRT_SOURCE register. The FIFOs remains in this flushed state until the register IC_CLR_TX_ABRT is read. Once this read is performed, the Tx FIFO is then ready to accept more data bytes from the APB interface.
///   Reset value: 0x0
///
/// - **RD_REQ** `[5]` (RO)
///
///   This bit is set to 1 when DW_apb_i2c is acting as a slave and another I2C master is attempting to read data from DW_apb_i2c. The DW_apb_i2c holds the I2C bus in a wait state (SCL=0) until this interrupt is serviced, which means that the slave has been addressed by a remote master that is asking for data to be transferred. The processor must respond to this interrupt and then write the requested data to the IC_DATA_CMD register. This bit is set to 0 just after the processor reads the IC_CLR_RD_REQ register.
///   Reset value: 0x0
///
/// - **TX_EMPTY** `[4]` (RO)
///
///   The behavior of the TX_EMPTY interrupt status differs based on the TX_EMPTY_CTRL selection in the IC_CON register. - When TX_EMPTY_CTRL = 0: This bit is set to 1 when the transmit buffer is at or below the threshold value set in the IC_TX_TL register. - When TX_EMPTY_CTRL = 1: This bit is set to 1 when the transmit buffer is at or below the threshold value set in the IC_TX_TL register and the transmission of the address/data from the internal shift register for the most recently popped command is completed. It is automatically cleared by hardware when the buffer level goes above the threshold. When IC_ENABLE[0] is set to 0, the TX FIFO is flushed and held in reset. There the TX FIFO looks like it has no data within it, so this bit is set to 1, provided there is activity in the master or slave state machines. When there is no longer any activity, then with ic_en=0, this bit is set to 0.
///   Reset value: 0x0.
///
/// - **TX_OVER** `[3]` (RO)
///
///   Set during transmit if the transmit buffer is filled to IC_TX_BUFFER_DEPTH and the processor attempts to issue another I2C command by writing to the IC_DATA_CMD register. When the module is disabled, this bit keeps its level until the master or slave state machines go into idle, and when ic_en goes to 0, this interrupt is cleared.
///   Reset value: 0x0
///
/// - **RX_FULL** `[2]` (RO)
///
///   Set when the receive buffer reaches or goes above the RX_TL threshold in the IC_RX_TL register. It is automatically cleared by hardware when buffer level goes below the threshold. If the module is disabled (IC_ENABLE[0]=0), the RX FIFO is flushed and held in reset; therefore the RX FIFO is not full. So this bit is cleared once the IC_ENABLE bit 0 is programmed with a 0, regardless of the activity that continues.
///   Reset value: 0x0
///
/// - **RX_OVER** `[1]` (RO)
///
///   Set if the receive buffer is completely filled to IC_RX_BUFFER_DEPTH and an additional byte is received from an external I2C device. The DW_apb_i2c acknowledges this, but any data bytes received after the FIFO is full are lost. If the module is disabled (IC_ENABLE[0]=0), this bit keeps its level until the master or slave state machines go into idle, and when ic_en goes to 0, this interrupt is cleared.
///   Note: If bit 9 of the IC_CON register (RX_FIFO_FULL_HLD_CTRL) is programmed to HIGH, then the RX_OVER interrupt never occurs, because the Rx FIFO never overflows.
///   Reset value: 0x0
///
/// - **RX_UNDER** `[0]` (RO)
///
///   Set if the processor attempts to read the receive buffer when it is empty by reading from the IC_DATA_CMD register. If the module is disabled (IC_ENABLE[0]=0), this bit keeps its level until the master or slave state machines go into idle, and when ic_en goes to 0, this interrupt is cleared.
///   Reset value: 0x0
#[repr(transparent)]
pub struct IcRawIntrStat {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for IcRawIntrStat {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl IcRawIntrStat {
    // ======== Field Constants ========

    /// **RESTART_DET**
    ///
    /// Indicates whether a RESTART condition has occurred on the I2C interface when DW_apb_i2c is operating in Slave mode and the slave is being addressed. Enabled only when IC_SLV_RESTART_DET_EN=1.
    /// Note: However, in high-speed mode or during a START BYTE transfer, the RESTART comes before the address field as per the I2C protocol. In this case, the slave is not the addressed slave when the RESTART is issued, therefore DW_apb_i2c does not generate the RESTART_DET interrupt.
    /// Reset value: 0x0
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const RESTART_DET_OFFSET: u32 = 12;
    pub const RESTART_DET_MASK: u32 = 0x00001000;
    pub const RESTART_DET: u32 = 1 << 12;
    /// INACTIVE = 0
    /// RESTART_DET interrupt is inactive
    pub const RESTART_DET_INACTIVE: u32 = 0 << 12;
    pub const RESTART_DET_INACTIVE_VAL: u32 = 0;
    /// ACTIVE = 1
    /// RESTART_DET interrupt is active
    pub const RESTART_DET_ACTIVE: u32 = 1 << 12;
    pub const RESTART_DET_ACTIVE_VAL: u32 = 1;

    /// **GEN_CALL**
    ///
    /// Set only when a General Call address is received and it is acknowledged. It stays set until it is cleared either by disabling DW_apb_i2c or when the CPU reads bit 0 of the IC_CLR_GEN_CALL register. DW_apb_i2c stores the received data in the Rx buffer.
    /// Reset value: 0x0
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const GEN_CALL_OFFSET: u32 = 11;
    pub const GEN_CALL_MASK: u32 = 0x00000800;
    pub const GEN_CALL: u32 = 1 << 11;
    /// INACTIVE = 0
    /// GEN_CALL interrupt is inactive
    pub const GEN_CALL_INACTIVE: u32 = 0 << 11;
    pub const GEN_CALL_INACTIVE_VAL: u32 = 0;
    /// ACTIVE = 1
    /// GEN_CALL interrupt is active
    pub const GEN_CALL_ACTIVE: u32 = 1 << 11;
    pub const GEN_CALL_ACTIVE_VAL: u32 = 1;

    /// **START_DET**
    ///
    /// Indicates whether a START or RESTART condition has occurred on the I2C interface regardless of whether DW_apb_i2c is operating in slave or master mode.
    /// Reset value: 0x0
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const START_DET_OFFSET: u32 = 10;
    pub const START_DET_MASK: u32 = 0x00000400;
    pub const START_DET: u32 = 1 << 10;
    /// INACTIVE = 0
    /// START_DET interrupt is inactive
    pub const START_DET_INACTIVE: u32 = 0 << 10;
    pub const START_DET_INACTIVE_VAL: u32 = 0;
    /// ACTIVE = 1
    /// START_DET interrupt is active
    pub const START_DET_ACTIVE: u32 = 1 << 10;
    pub const START_DET_ACTIVE_VAL: u32 = 1;

    /// **STOP_DET**
    ///
    /// Indicates whether a STOP condition has occurred on the I2C interface regardless of whether DW_apb_i2c is operating in slave or master mode.
    /// In Slave Mode: - If IC_CON[7]=1'b1 (STOP_DET_IFADDRESSED), the STOP_DET interrupt will be issued only if slave is addressed. Note: During a general call address, this slave does not issue a STOP_DET interrupt if STOP_DET_IF_ADDRESSED=1'b1, even if the slave responds to the general call address by generating ACK. The STOP_DET interrupt is generated only when the transmitted address matches the slave address (SAR). - If IC_CON[7]=1'b0 (STOP_DET_IFADDRESSED), the STOP_DET interrupt is issued irrespective of whether it is being addressed. In Master Mode: - If IC_CON[10]=1'b1 (STOP_DET_IF_MASTER_ACTIVE),the STOP_DET interrupt will be issued only if Master is active. - If IC_CON[10]=1'b0 (STOP_DET_IFADDRESSED),the STOP_DET interrupt will be issued irrespective of whether master is active or not. Reset value: 0x0
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const STOP_DET_OFFSET: u32 = 9;
    pub const STOP_DET_MASK: u32 = 0x00000200;
    pub const STOP_DET: u32 = 1 << 9;
    /// INACTIVE = 0
    /// STOP_DET interrupt is inactive
    pub const STOP_DET_INACTIVE: u32 = 0 << 9;
    pub const STOP_DET_INACTIVE_VAL: u32 = 0;
    /// ACTIVE = 1
    /// STOP_DET interrupt is active
    pub const STOP_DET_ACTIVE: u32 = 1 << 9;
    pub const STOP_DET_ACTIVE_VAL: u32 = 1;

    /// **ACTIVITY**
    ///
    /// This bit captures DW_apb_i2c activity and stays set until it is cleared. There are four ways to clear it: - Disabling the DW_apb_i2c - Reading the IC_CLR_ACTIVITY register - Reading the IC_CLR_INTR register - System reset Once this bit is set, it stays set unless one of the four methods is used to clear it. Even if the DW_apb_i2c module is idle, this bit remains set until cleared, indicating that there was activity on the bus.
    /// Reset value: 0x0
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const ACTIVITY_OFFSET: u32 = 8;
    pub const ACTIVITY_MASK: u32 = 0x00000100;
    pub const ACTIVITY: u32 = 1 << 8;
    /// INACTIVE = 0
    /// RAW_INTR_ACTIVITY interrupt is inactive
    pub const ACTIVITY_INACTIVE: u32 = 0 << 8;
    pub const ACTIVITY_INACTIVE_VAL: u32 = 0;
    /// ACTIVE = 1
    /// RAW_INTR_ACTIVITY interrupt is active
    pub const ACTIVITY_ACTIVE: u32 = 1 << 8;
    pub const ACTIVITY_ACTIVE_VAL: u32 = 1;

    /// **RX_DONE**
    ///
    /// When the DW_apb_i2c is acting as a slave-transmitter, this bit is set to 1 if the master does not acknowledge a transmitted byte. This occurs on the last byte of the transmission, indicating that the transmission is done.
    /// Reset value: 0x0
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const RX_DONE_OFFSET: u32 = 7;
    pub const RX_DONE_MASK: u32 = 0x00000080;
    pub const RX_DONE: u32 = 1 << 7;
    /// INACTIVE = 0
    /// RX_DONE interrupt is inactive
    pub const RX_DONE_INACTIVE: u32 = 0 << 7;
    pub const RX_DONE_INACTIVE_VAL: u32 = 0;
    /// ACTIVE = 1
    /// RX_DONE interrupt is active
    pub const RX_DONE_ACTIVE: u32 = 1 << 7;
    pub const RX_DONE_ACTIVE_VAL: u32 = 1;

    /// **TX_ABRT**
    ///
    /// This bit indicates if DW_apb_i2c, as an I2C transmitter, is unable to complete the intended actions on the contents of the transmit FIFO. This situation can occur both as an I2C master or an I2C slave, and is referred to as a 'transmit abort'. When this bit is set to 1, the IC_TX_ABRT_SOURCE register indicates the reason why the transmit abort takes places.
    /// Note: The DW_apb_i2c flushes/resets/empties the TX_FIFO and RX_FIFO whenever there is a transmit abort caused by any of the events tracked by the IC_TX_ABRT_SOURCE register. The FIFOs remains in this flushed state until the register IC_CLR_TX_ABRT is read. Once this read is performed, the Tx FIFO is then ready to accept more data bytes from the APB interface.
    /// Reset value: 0x0
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const TX_ABRT_OFFSET: u32 = 6;
    pub const TX_ABRT_MASK: u32 = 0x00000040;
    pub const TX_ABRT: u32 = 1 << 6;
    /// INACTIVE = 0
    /// TX_ABRT interrupt is inactive
    pub const TX_ABRT_INACTIVE: u32 = 0 << 6;
    pub const TX_ABRT_INACTIVE_VAL: u32 = 0;
    /// ACTIVE = 1
    /// TX_ABRT interrupt is active
    pub const TX_ABRT_ACTIVE: u32 = 1 << 6;
    pub const TX_ABRT_ACTIVE_VAL: u32 = 1;

    /// **RD_REQ**
    ///
    /// This bit is set to 1 when DW_apb_i2c is acting as a slave and another I2C master is attempting to read data from DW_apb_i2c. The DW_apb_i2c holds the I2C bus in a wait state (SCL=0) until this interrupt is serviced, which means that the slave has been addressed by a remote master that is asking for data to be transferred. The processor must respond to this interrupt and then write the requested data to the IC_DATA_CMD register. This bit is set to 0 just after the processor reads the IC_CLR_RD_REQ register.
    /// Reset value: 0x0
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const RD_REQ_OFFSET: u32 = 5;
    pub const RD_REQ_MASK: u32 = 0x00000020;
    pub const RD_REQ: u32 = 1 << 5;
    /// INACTIVE = 0
    /// RD_REQ interrupt is inactive
    pub const RD_REQ_INACTIVE: u32 = 0 << 5;
    pub const RD_REQ_INACTIVE_VAL: u32 = 0;
    /// ACTIVE = 1
    /// RD_REQ interrupt is active
    pub const RD_REQ_ACTIVE: u32 = 1 << 5;
    pub const RD_REQ_ACTIVE_VAL: u32 = 1;

    /// **TX_EMPTY**
    ///
    /// The behavior of the TX_EMPTY interrupt status differs based on the TX_EMPTY_CTRL selection in the IC_CON register. - When TX_EMPTY_CTRL = 0: This bit is set to 1 when the transmit buffer is at or below the threshold value set in the IC_TX_TL register. - When TX_EMPTY_CTRL = 1: This bit is set to 1 when the transmit buffer is at or below the threshold value set in the IC_TX_TL register and the transmission of the address/data from the internal shift register for the most recently popped command is completed. It is automatically cleared by hardware when the buffer level goes above the threshold. When IC_ENABLE[0] is set to 0, the TX FIFO is flushed and held in reset. There the TX FIFO looks like it has no data within it, so this bit is set to 1, provided there is activity in the master or slave state machines. When there is no longer any activity, then with ic_en=0, this bit is set to 0.
    /// Reset value: 0x0.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const TX_EMPTY_OFFSET: u32 = 4;
    pub const TX_EMPTY_MASK: u32 = 0x00000010;
    pub const TX_EMPTY: u32 = 1 << 4;
    /// INACTIVE = 0
    /// TX_EMPTY interrupt is inactive
    pub const TX_EMPTY_INACTIVE: u32 = 0 << 4;
    pub const TX_EMPTY_INACTIVE_VAL: u32 = 0;
    /// ACTIVE = 1
    /// TX_EMPTY interrupt is active
    pub const TX_EMPTY_ACTIVE: u32 = 1 << 4;
    pub const TX_EMPTY_ACTIVE_VAL: u32 = 1;

    /// **TX_OVER**
    ///
    /// Set during transmit if the transmit buffer is filled to IC_TX_BUFFER_DEPTH and the processor attempts to issue another I2C command by writing to the IC_DATA_CMD register. When the module is disabled, this bit keeps its level until the master or slave state machines go into idle, and when ic_en goes to 0, this interrupt is cleared.
    /// Reset value: 0x0
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const TX_OVER_OFFSET: u32 = 3;
    pub const TX_OVER_MASK: u32 = 0x00000008;
    pub const TX_OVER: u32 = 1 << 3;
    /// INACTIVE = 0
    /// TX_OVER interrupt is inactive
    pub const TX_OVER_INACTIVE: u32 = 0 << 3;
    pub const TX_OVER_INACTIVE_VAL: u32 = 0;
    /// ACTIVE = 1
    /// TX_OVER interrupt is active
    pub const TX_OVER_ACTIVE: u32 = 1 << 3;
    pub const TX_OVER_ACTIVE_VAL: u32 = 1;

    /// **RX_FULL**
    ///
    /// Set when the receive buffer reaches or goes above the RX_TL threshold in the IC_RX_TL register. It is automatically cleared by hardware when buffer level goes below the threshold. If the module is disabled (IC_ENABLE[0]=0), the RX FIFO is flushed and held in reset; therefore the RX FIFO is not full. So this bit is cleared once the IC_ENABLE bit 0 is programmed with a 0, regardless of the activity that continues.
    /// Reset value: 0x0
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const RX_FULL_OFFSET: u32 = 2;
    pub const RX_FULL_MASK: u32 = 0x00000004;
    pub const RX_FULL: u32 = 1 << 2;
    /// INACTIVE = 0
    /// RX_FULL interrupt is inactive
    pub const RX_FULL_INACTIVE: u32 = 0 << 2;
    pub const RX_FULL_INACTIVE_VAL: u32 = 0;
    /// ACTIVE = 1
    /// RX_FULL interrupt is active
    pub const RX_FULL_ACTIVE: u32 = 1 << 2;
    pub const RX_FULL_ACTIVE_VAL: u32 = 1;

    /// **RX_OVER**
    ///
    /// Set if the receive buffer is completely filled to IC_RX_BUFFER_DEPTH and an additional byte is received from an external I2C device. The DW_apb_i2c acknowledges this, but any data bytes received after the FIFO is full are lost. If the module is disabled (IC_ENABLE[0]=0), this bit keeps its level until the master or slave state machines go into idle, and when ic_en goes to 0, this interrupt is cleared.
    /// Note: If bit 9 of the IC_CON register (RX_FIFO_FULL_HLD_CTRL) is programmed to HIGH, then the RX_OVER interrupt never occurs, because the Rx FIFO never overflows.
    /// Reset value: 0x0
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const RX_OVER_OFFSET: u32 = 1;
    pub const RX_OVER_MASK: u32 = 0x00000002;
    pub const RX_OVER: u32 = 1 << 1;
    /// INACTIVE = 0
    /// RX_OVER interrupt is inactive
    pub const RX_OVER_INACTIVE: u32 = 0 << 1;
    pub const RX_OVER_INACTIVE_VAL: u32 = 0;
    /// ACTIVE = 1
    /// RX_OVER interrupt is active
    pub const RX_OVER_ACTIVE: u32 = 1 << 1;
    pub const RX_OVER_ACTIVE_VAL: u32 = 1;

    /// **RX_UNDER**
    ///
    /// Set if the processor attempts to read the receive buffer when it is empty by reading from the IC_DATA_CMD register. If the module is disabled (IC_ENABLE[0]=0), this bit keeps its level until the master or slave state machines go into idle, and when ic_en goes to 0, this interrupt is cleared.
    /// Reset value: 0x0
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const RX_UNDER_OFFSET: u32 = 0;
    pub const RX_UNDER_MASK: u32 = 0x00000001;
    pub const RX_UNDER: u32 = 1 << 0;
    /// INACTIVE = 0
    /// RX_UNDER interrupt is inactive
    pub const RX_UNDER_INACTIVE: u32 = 0 << 0;
    pub const RX_UNDER_INACTIVE_VAL: u32 = 0;
    /// ACTIVE = 1
    /// RX_UNDER interrupt is active
    pub const RX_UNDER_ACTIVE: u32 = 1 << 0;
    pub const RX_UNDER_ACTIVE_VAL: u32 = 1;

    // ======== Field Accessors ========

    /// **RESTART_DET** `[12]` (RO)
    ///
    /// Indicates whether a RESTART condition has occurred on the I2C interface when DW_apb_i2c is operating in Slave mode and the slave is being addressed. Enabled only when IC_SLV_RESTART_DET_EN=1.
    /// Note: However, in high-speed mode or during a START BYTE transfer, the RESTART comes before the address field as per the I2C protocol. In this case, the slave is not the addressed slave when the RESTART is issued, therefore DW_apb_i2c does not generate the RESTART_DET interrupt.
    /// Reset value: 0x0
    #[inline(always)]
    pub fn restart_det(&self) -> bool {
        (self.value.read_bits() & Self::RESTART_DET_MASK) != 0
    }

    /// **GEN_CALL** `[11]` (RO)
    ///
    /// Set only when a General Call address is received and it is acknowledged. It stays set until it is cleared either by disabling DW_apb_i2c or when the CPU reads bit 0 of the IC_CLR_GEN_CALL register. DW_apb_i2c stores the received data in the Rx buffer.
    /// Reset value: 0x0
    #[inline(always)]
    pub fn gen_call(&self) -> bool {
        (self.value.read_bits() & Self::GEN_CALL_MASK) != 0
    }

    /// **START_DET** `[10]` (RO)
    ///
    /// Indicates whether a START or RESTART condition has occurred on the I2C interface regardless of whether DW_apb_i2c is operating in slave or master mode.
    /// Reset value: 0x0
    #[inline(always)]
    pub fn start_det(&self) -> bool {
        (self.value.read_bits() & Self::START_DET_MASK) != 0
    }

    /// **STOP_DET** `[9]` (RO)
    ///
    /// Indicates whether a STOP condition has occurred on the I2C interface regardless of whether DW_apb_i2c is operating in slave or master mode.
    /// In Slave Mode: - If IC_CON[7]=1'b1 (STOP_DET_IFADDRESSED), the STOP_DET interrupt will be issued only if slave is addressed. Note: During a general call address, this slave does not issue a STOP_DET interrupt if STOP_DET_IF_ADDRESSED=1'b1, even if the slave responds to the general call address by generating ACK. The STOP_DET interrupt is generated only when the transmitted address matches the slave address (SAR). - If IC_CON[7]=1'b0 (STOP_DET_IFADDRESSED), the STOP_DET interrupt is issued irrespective of whether it is being addressed. In Master Mode: - If IC_CON[10]=1'b1 (STOP_DET_IF_MASTER_ACTIVE),the STOP_DET interrupt will be issued only if Master is active. - If IC_CON[10]=1'b0 (STOP_DET_IFADDRESSED),the STOP_DET interrupt will be issued irrespective of whether master is active or not. Reset value: 0x0
    #[inline(always)]
    pub fn stop_det(&self) -> bool {
        (self.value.read_bits() & Self::STOP_DET_MASK) != 0
    }

    /// **ACTIVITY** `[8]` (RO)
    ///
    /// This bit captures DW_apb_i2c activity and stays set until it is cleared. There are four ways to clear it: - Disabling the DW_apb_i2c - Reading the IC_CLR_ACTIVITY register - Reading the IC_CLR_INTR register - System reset Once this bit is set, it stays set unless one of the four methods is used to clear it. Even if the DW_apb_i2c module is idle, this bit remains set until cleared, indicating that there was activity on the bus.
    /// Reset value: 0x0
    #[inline(always)]
    pub fn activity(&self) -> bool {
        (self.value.read_bits() & Self::ACTIVITY_MASK) != 0
    }

    /// **RX_DONE** `[7]` (RO)
    ///
    /// When the DW_apb_i2c is acting as a slave-transmitter, this bit is set to 1 if the master does not acknowledge a transmitted byte. This occurs on the last byte of the transmission, indicating that the transmission is done.
    /// Reset value: 0x0
    #[inline(always)]
    pub fn rx_done(&self) -> bool {
        (self.value.read_bits() & Self::RX_DONE_MASK) != 0
    }

    /// **TX_ABRT** `[6]` (RO)
    ///
    /// This bit indicates if DW_apb_i2c, as an I2C transmitter, is unable to complete the intended actions on the contents of the transmit FIFO. This situation can occur both as an I2C master or an I2C slave, and is referred to as a 'transmit abort'. When this bit is set to 1, the IC_TX_ABRT_SOURCE register indicates the reason why the transmit abort takes places.
    /// Note: The DW_apb_i2c flushes/resets/empties the TX_FIFO and RX_FIFO whenever there is a transmit abort caused by any of the events tracked by the IC_TX_ABRT_SOURCE register. The FIFOs remains in this flushed state until the register IC_CLR_TX_ABRT is read. Once this read is performed, the Tx FIFO is then ready to accept more data bytes from the APB interface.
    /// Reset value: 0x0
    #[inline(always)]
    pub fn tx_abrt(&self) -> bool {
        (self.value.read_bits() & Self::TX_ABRT_MASK) != 0
    }

    /// **RD_REQ** `[5]` (RO)
    ///
    /// This bit is set to 1 when DW_apb_i2c is acting as a slave and another I2C master is attempting to read data from DW_apb_i2c. The DW_apb_i2c holds the I2C bus in a wait state (SCL=0) until this interrupt is serviced, which means that the slave has been addressed by a remote master that is asking for data to be transferred. The processor must respond to this interrupt and then write the requested data to the IC_DATA_CMD register. This bit is set to 0 just after the processor reads the IC_CLR_RD_REQ register.
    /// Reset value: 0x0
    #[inline(always)]
    pub fn rd_req(&self) -> bool {
        (self.value.read_bits() & Self::RD_REQ_MASK) != 0
    }

    /// **TX_EMPTY** `[4]` (RO)
    ///
    /// The behavior of the TX_EMPTY interrupt status differs based on the TX_EMPTY_CTRL selection in the IC_CON register. - When TX_EMPTY_CTRL = 0: This bit is set to 1 when the transmit buffer is at or below the threshold value set in the IC_TX_TL register. - When TX_EMPTY_CTRL = 1: This bit is set to 1 when the transmit buffer is at or below the threshold value set in the IC_TX_TL register and the transmission of the address/data from the internal shift register for the most recently popped command is completed. It is automatically cleared by hardware when the buffer level goes above the threshold. When IC_ENABLE[0] is set to 0, the TX FIFO is flushed and held in reset. There the TX FIFO looks like it has no data within it, so this bit is set to 1, provided there is activity in the master or slave state machines. When there is no longer any activity, then with ic_en=0, this bit is set to 0.
    /// Reset value: 0x0.
    #[inline(always)]
    pub fn tx_empty(&self) -> bool {
        (self.value.read_bits() & Self::TX_EMPTY_MASK) != 0
    }

    /// **TX_OVER** `[3]` (RO)
    ///
    /// Set during transmit if the transmit buffer is filled to IC_TX_BUFFER_DEPTH and the processor attempts to issue another I2C command by writing to the IC_DATA_CMD register. When the module is disabled, this bit keeps its level until the master or slave state machines go into idle, and when ic_en goes to 0, this interrupt is cleared.
    /// Reset value: 0x0
    #[inline(always)]
    pub fn tx_over(&self) -> bool {
        (self.value.read_bits() & Self::TX_OVER_MASK) != 0
    }

    /// **RX_FULL** `[2]` (RO)
    ///
    /// Set when the receive buffer reaches or goes above the RX_TL threshold in the IC_RX_TL register. It is automatically cleared by hardware when buffer level goes below the threshold. If the module is disabled (IC_ENABLE[0]=0), the RX FIFO is flushed and held in reset; therefore the RX FIFO is not full. So this bit is cleared once the IC_ENABLE bit 0 is programmed with a 0, regardless of the activity that continues.
    /// Reset value: 0x0
    #[inline(always)]
    pub fn rx_full(&self) -> bool {
        (self.value.read_bits() & Self::RX_FULL_MASK) != 0
    }

    /// **RX_OVER** `[1]` (RO)
    ///
    /// Set if the receive buffer is completely filled to IC_RX_BUFFER_DEPTH and an additional byte is received from an external I2C device. The DW_apb_i2c acknowledges this, but any data bytes received after the FIFO is full are lost. If the module is disabled (IC_ENABLE[0]=0), this bit keeps its level until the master or slave state machines go into idle, and when ic_en goes to 0, this interrupt is cleared.
    /// Note: If bit 9 of the IC_CON register (RX_FIFO_FULL_HLD_CTRL) is programmed to HIGH, then the RX_OVER interrupt never occurs, because the Rx FIFO never overflows.
    /// Reset value: 0x0
    #[inline(always)]
    pub fn rx_over(&self) -> bool {
        (self.value.read_bits() & Self::RX_OVER_MASK) != 0
    }

    /// **RX_UNDER** `[0]` (RO)
    ///
    /// Set if the processor attempts to read the receive buffer when it is empty by reading from the IC_DATA_CMD register. If the module is disabled (IC_ENABLE[0]=0), this bit keeps its level until the master or slave state machines go into idle, and when ic_en goes to 0, this interrupt is cleared.
    /// Reset value: 0x0
    #[inline(always)]
    pub fn rx_under(&self) -> bool {
        (self.value.read_bits() & Self::RX_UNDER_MASK) != 0
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

    /// Read **RESTART_DET** `[12:12]`
    ///
    /// Indicates whether a RESTART condition has occurred on the I2C interface when DW_apb_i2c is operating in Slave mode and the slave is being addressed. Enabled only when IC_SLV_RESTART_DET_EN=1.
    /// Note: However, in high-speed mode or during a START BYTE transfer, the RESTART comes before the address field as per the I2C protocol. In this case, the slave is not the addressed slave when the RESTART is issued, therefore DW_apb_i2c does not generate the RESTART_DET interrupt.
    /// Reset value: 0x0
    #[inline(always)]
    pub const fn restart_det(&self) -> bool {
        (self.bits & IcRawIntrStat::RESTART_DET_MASK) != 0
    }

    /// Read **GEN_CALL** `[11:11]`
    ///
    /// Set only when a General Call address is received and it is acknowledged. It stays set until it is cleared either by disabling DW_apb_i2c or when the CPU reads bit 0 of the IC_CLR_GEN_CALL register. DW_apb_i2c stores the received data in the Rx buffer.
    /// Reset value: 0x0
    #[inline(always)]
    pub const fn gen_call(&self) -> bool {
        (self.bits & IcRawIntrStat::GEN_CALL_MASK) != 0
    }

    /// Read **START_DET** `[10:10]`
    ///
    /// Indicates whether a START or RESTART condition has occurred on the I2C interface regardless of whether DW_apb_i2c is operating in slave or master mode.
    /// Reset value: 0x0
    #[inline(always)]
    pub const fn start_det(&self) -> bool {
        (self.bits & IcRawIntrStat::START_DET_MASK) != 0
    }

    /// Read **STOP_DET** `[9:9]`
    ///
    /// Indicates whether a STOP condition has occurred on the I2C interface regardless of whether DW_apb_i2c is operating in slave or master mode.
    /// In Slave Mode: - If IC_CON[7]=1'b1 (STOP_DET_IFADDRESSED), the STOP_DET interrupt will be issued only if slave is addressed. Note: During a general call address, this slave does not issue a STOP_DET interrupt if STOP_DET_IF_ADDRESSED=1'b1, even if the slave responds to the general call address by generating ACK. The STOP_DET interrupt is generated only when the transmitted address matches the slave address (SAR). - If IC_CON[7]=1'b0 (STOP_DET_IFADDRESSED), the STOP_DET interrupt is issued irrespective of whether it is being addressed. In Master Mode: - If IC_CON[10]=1'b1 (STOP_DET_IF_MASTER_ACTIVE),the STOP_DET interrupt will be issued only if Master is active. - If IC_CON[10]=1'b0 (STOP_DET_IFADDRESSED),the STOP_DET interrupt will be issued irrespective of whether master is active or not. Reset value: 0x0
    #[inline(always)]
    pub const fn stop_det(&self) -> bool {
        (self.bits & IcRawIntrStat::STOP_DET_MASK) != 0
    }

    /// Read **ACTIVITY** `[8:8]`
    ///
    /// This bit captures DW_apb_i2c activity and stays set until it is cleared. There are four ways to clear it: - Disabling the DW_apb_i2c - Reading the IC_CLR_ACTIVITY register - Reading the IC_CLR_INTR register - System reset Once this bit is set, it stays set unless one of the four methods is used to clear it. Even if the DW_apb_i2c module is idle, this bit remains set until cleared, indicating that there was activity on the bus.
    /// Reset value: 0x0
    #[inline(always)]
    pub const fn activity(&self) -> bool {
        (self.bits & IcRawIntrStat::ACTIVITY_MASK) != 0
    }

    /// Read **RX_DONE** `[7:7]`
    ///
    /// When the DW_apb_i2c is acting as a slave-transmitter, this bit is set to 1 if the master does not acknowledge a transmitted byte. This occurs on the last byte of the transmission, indicating that the transmission is done.
    /// Reset value: 0x0
    #[inline(always)]
    pub const fn rx_done(&self) -> bool {
        (self.bits & IcRawIntrStat::RX_DONE_MASK) != 0
    }

    /// Read **TX_ABRT** `[6:6]`
    ///
    /// This bit indicates if DW_apb_i2c, as an I2C transmitter, is unable to complete the intended actions on the contents of the transmit FIFO. This situation can occur both as an I2C master or an I2C slave, and is referred to as a 'transmit abort'. When this bit is set to 1, the IC_TX_ABRT_SOURCE register indicates the reason why the transmit abort takes places.
    /// Note: The DW_apb_i2c flushes/resets/empties the TX_FIFO and RX_FIFO whenever there is a transmit abort caused by any of the events tracked by the IC_TX_ABRT_SOURCE register. The FIFOs remains in this flushed state until the register IC_CLR_TX_ABRT is read. Once this read is performed, the Tx FIFO is then ready to accept more data bytes from the APB interface.
    /// Reset value: 0x0
    #[inline(always)]
    pub const fn tx_abrt(&self) -> bool {
        (self.bits & IcRawIntrStat::TX_ABRT_MASK) != 0
    }

    /// Read **RD_REQ** `[5:5]`
    ///
    /// This bit is set to 1 when DW_apb_i2c is acting as a slave and another I2C master is attempting to read data from DW_apb_i2c. The DW_apb_i2c holds the I2C bus in a wait state (SCL=0) until this interrupt is serviced, which means that the slave has been addressed by a remote master that is asking for data to be transferred. The processor must respond to this interrupt and then write the requested data to the IC_DATA_CMD register. This bit is set to 0 just after the processor reads the IC_CLR_RD_REQ register.
    /// Reset value: 0x0
    #[inline(always)]
    pub const fn rd_req(&self) -> bool {
        (self.bits & IcRawIntrStat::RD_REQ_MASK) != 0
    }

    /// Read **TX_EMPTY** `[4:4]`
    ///
    /// The behavior of the TX_EMPTY interrupt status differs based on the TX_EMPTY_CTRL selection in the IC_CON register. - When TX_EMPTY_CTRL = 0: This bit is set to 1 when the transmit buffer is at or below the threshold value set in the IC_TX_TL register. - When TX_EMPTY_CTRL = 1: This bit is set to 1 when the transmit buffer is at or below the threshold value set in the IC_TX_TL register and the transmission of the address/data from the internal shift register for the most recently popped command is completed. It is automatically cleared by hardware when the buffer level goes above the threshold. When IC_ENABLE[0] is set to 0, the TX FIFO is flushed and held in reset. There the TX FIFO looks like it has no data within it, so this bit is set to 1, provided there is activity in the master or slave state machines. When there is no longer any activity, then with ic_en=0, this bit is set to 0.
    /// Reset value: 0x0.
    #[inline(always)]
    pub const fn tx_empty(&self) -> bool {
        (self.bits & IcRawIntrStat::TX_EMPTY_MASK) != 0
    }

    /// Read **TX_OVER** `[3:3]`
    ///
    /// Set during transmit if the transmit buffer is filled to IC_TX_BUFFER_DEPTH and the processor attempts to issue another I2C command by writing to the IC_DATA_CMD register. When the module is disabled, this bit keeps its level until the master or slave state machines go into idle, and when ic_en goes to 0, this interrupt is cleared.
    /// Reset value: 0x0
    #[inline(always)]
    pub const fn tx_over(&self) -> bool {
        (self.bits & IcRawIntrStat::TX_OVER_MASK) != 0
    }

    /// Read **RX_FULL** `[2:2]`
    ///
    /// Set when the receive buffer reaches or goes above the RX_TL threshold in the IC_RX_TL register. It is automatically cleared by hardware when buffer level goes below the threshold. If the module is disabled (IC_ENABLE[0]=0), the RX FIFO is flushed and held in reset; therefore the RX FIFO is not full. So this bit is cleared once the IC_ENABLE bit 0 is programmed with a 0, regardless of the activity that continues.
    /// Reset value: 0x0
    #[inline(always)]
    pub const fn rx_full(&self) -> bool {
        (self.bits & IcRawIntrStat::RX_FULL_MASK) != 0
    }

    /// Read **RX_OVER** `[1:1]`
    ///
    /// Set if the receive buffer is completely filled to IC_RX_BUFFER_DEPTH and an additional byte is received from an external I2C device. The DW_apb_i2c acknowledges this, but any data bytes received after the FIFO is full are lost. If the module is disabled (IC_ENABLE[0]=0), this bit keeps its level until the master or slave state machines go into idle, and when ic_en goes to 0, this interrupt is cleared.
    /// Note: If bit 9 of the IC_CON register (RX_FIFO_FULL_HLD_CTRL) is programmed to HIGH, then the RX_OVER interrupt never occurs, because the Rx FIFO never overflows.
    /// Reset value: 0x0
    #[inline(always)]
    pub const fn rx_over(&self) -> bool {
        (self.bits & IcRawIntrStat::RX_OVER_MASK) != 0
    }

    /// Read **RX_UNDER** `[0:0]`
    ///
    /// Set if the processor attempts to read the receive buffer when it is empty by reading from the IC_DATA_CMD register. If the module is disabled (IC_ENABLE[0]=0), this bit keeps its level until the master or slave state machines go into idle, and when ic_en goes to 0, this interrupt is cleared.
    /// Reset value: 0x0
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
    pub fn read_val(&self) -> IcRawIntrStatVal {
        IcRawIntrStatVal::from_bits(self.value.read_bits())
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
/// - **RX_TL** `[7:0]` (RW)
///
///   Receive FIFO Threshold Level.
///   Controls the level of entries (or above) that triggers the RX_FULL interrupt (bit 2 in IC_RAW_INTR_STAT register). The valid range is 0-255, with the additional restriction that hardware does not allow this value to be set to a value larger than the depth of the buffer. If an attempt is made to do that, the actual value set will be the maximum depth of the buffer. A value of 0 sets the threshold for 1 entry, and a value of 255 sets the threshold for 256 entries.
#[repr(transparent)]
pub struct IcRxTl {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for IcRxTl {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl IcRxTl {
    // ======== Field Constants ========

    /// **RX_TL**
    ///
    /// Receive FIFO Threshold Level.
    /// Controls the level of entries (or above) that triggers the RX_FULL interrupt (bit 2 in IC_RAW_INTR_STAT register). The valid range is 0-255, with the additional restriction that hardware does not allow this value to be set to a value larger than the depth of the buffer. If an attempt is made to do that, the actual value set will be the maximum depth of the buffer. A value of 0 sets the threshold for 1 entry, and a value of 255 sets the threshold for 256 entries.
    ///
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const RX_TL_OFFSET: u32 = 0;
    pub const RX_TL_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// **RX_TL** `[7:0]` (RW)
    ///
    /// Receive FIFO Threshold Level.
    /// Controls the level of entries (or above) that triggers the RX_FULL interrupt (bit 2 in IC_RAW_INTR_STAT register). The valid range is 0-255, with the additional restriction that hardware does not allow this value to be set to a value larger than the depth of the buffer. If an attempt is made to do that, the actual value set will be the maximum depth of the buffer. A value of 0 sets the threshold for 1 entry, and a value of 255 sets the threshold for 256 entries.
    #[inline(always)]
    pub fn rx_tl(&self) -> u32 {
        (self.value.read_bits() & Self::RX_TL_MASK) >> Self::RX_TL_OFFSET
    }

    /// Write **RX_TL** `[7:0]` (RW)
    ///
    /// Receive FIFO Threshold Level.
    /// Controls the level of entries (or above) that triggers the RX_FULL interrupt (bit 2 in IC_RAW_INTR_STAT register). The valid range is 0-255, with the additional restriction that hardware does not allow this value to be set to a value larger than the depth of the buffer. If an attempt is made to do that, the actual value set will be the maximum depth of the buffer. A value of 0 sets the threshold for 1 entry, and a value of 255 sets the threshold for 256 entries.
    #[inline(always)]
    pub fn write_rx_tl(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::RX_TL_MASK) | ((value << Self::RX_TL_OFFSET) & Self::RX_TL_MASK)
        });
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

    /// Read **RX_TL** `[7:0]`
    ///
    /// Receive FIFO Threshold Level.
    /// Controls the level of entries (or above) that triggers the RX_FULL interrupt (bit 2 in IC_RAW_INTR_STAT register). The valid range is 0-255, with the additional restriction that hardware does not allow this value to be set to a value larger than the depth of the buffer. If an attempt is made to do that, the actual value set will be the maximum depth of the buffer. A value of 0 sets the threshold for 1 entry, and a value of 255 sets the threshold for 256 entries.
    #[inline(always)]
    pub const fn rx_tl(&self) -> u32 {
        (self.bits & IcRxTl::RX_TL_MASK) >> IcRxTl::RX_TL_OFFSET
    }

    /// Set **RX_TL** `[7:0]`
    ///
    /// Receive FIFO Threshold Level.
    /// Controls the level of entries (or above) that triggers the RX_FULL interrupt (bit 2 in IC_RAW_INTR_STAT register). The valid range is 0-255, with the additional restriction that hardware does not allow this value to be set to a value larger than the depth of the buffer. If an attempt is made to do that, the actual value set will be the maximum depth of the buffer. A value of 0 sets the threshold for 1 entry, and a value of 255 sets the threshold for 256 entries.
    #[inline(always)]
    pub const fn with_rx_tl(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !IcRxTl::RX_TL_MASK)
                | ((value << IcRxTl::RX_TL_OFFSET) & IcRxTl::RX_TL_MASK),
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
    pub fn read_val(&self) -> IcRxTlVal {
        IcRxTlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: IcRxTlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(IcRxTlVal) -> IcRxTlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
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
/// - **TX_TL** `[7:0]` (RW)
///
///   Transmit FIFO Threshold Level.
///   Controls the level of entries (or below) that trigger the TX_EMPTY interrupt (bit 4 in IC_RAW_INTR_STAT register). The valid range is 0-255, with the additional restriction that it may not be set to value larger than the depth of the buffer. If an attempt is made to do that, the actual value set will be the maximum depth of the buffer. A value of 0 sets the threshold for 0 entries, and a value of 255 sets the threshold for 255 entries.
#[repr(transparent)]
pub struct IcTxTl {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for IcTxTl {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl IcTxTl {
    // ======== Field Constants ========

    /// **TX_TL**
    ///
    /// Transmit FIFO Threshold Level.
    /// Controls the level of entries (or below) that trigger the TX_EMPTY interrupt (bit 4 in IC_RAW_INTR_STAT register). The valid range is 0-255, with the additional restriction that it may not be set to value larger than the depth of the buffer. If an attempt is made to do that, the actual value set will be the maximum depth of the buffer. A value of 0 sets the threshold for 0 entries, and a value of 255 sets the threshold for 255 entries.
    ///
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const TX_TL_OFFSET: u32 = 0;
    pub const TX_TL_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// **TX_TL** `[7:0]` (RW)
    ///
    /// Transmit FIFO Threshold Level.
    /// Controls the level of entries (or below) that trigger the TX_EMPTY interrupt (bit 4 in IC_RAW_INTR_STAT register). The valid range is 0-255, with the additional restriction that it may not be set to value larger than the depth of the buffer. If an attempt is made to do that, the actual value set will be the maximum depth of the buffer. A value of 0 sets the threshold for 0 entries, and a value of 255 sets the threshold for 255 entries.
    #[inline(always)]
    pub fn tx_tl(&self) -> u32 {
        (self.value.read_bits() & Self::TX_TL_MASK) >> Self::TX_TL_OFFSET
    }

    /// Write **TX_TL** `[7:0]` (RW)
    ///
    /// Transmit FIFO Threshold Level.
    /// Controls the level of entries (or below) that trigger the TX_EMPTY interrupt (bit 4 in IC_RAW_INTR_STAT register). The valid range is 0-255, with the additional restriction that it may not be set to value larger than the depth of the buffer. If an attempt is made to do that, the actual value set will be the maximum depth of the buffer. A value of 0 sets the threshold for 0 entries, and a value of 255 sets the threshold for 255 entries.
    #[inline(always)]
    pub fn write_tx_tl(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::TX_TL_MASK) | ((value << Self::TX_TL_OFFSET) & Self::TX_TL_MASK)
        });
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

    /// Read **TX_TL** `[7:0]`
    ///
    /// Transmit FIFO Threshold Level.
    /// Controls the level of entries (or below) that trigger the TX_EMPTY interrupt (bit 4 in IC_RAW_INTR_STAT register). The valid range is 0-255, with the additional restriction that it may not be set to value larger than the depth of the buffer. If an attempt is made to do that, the actual value set will be the maximum depth of the buffer. A value of 0 sets the threshold for 0 entries, and a value of 255 sets the threshold for 255 entries.
    #[inline(always)]
    pub const fn tx_tl(&self) -> u32 {
        (self.bits & IcTxTl::TX_TL_MASK) >> IcTxTl::TX_TL_OFFSET
    }

    /// Set **TX_TL** `[7:0]`
    ///
    /// Transmit FIFO Threshold Level.
    /// Controls the level of entries (or below) that trigger the TX_EMPTY interrupt (bit 4 in IC_RAW_INTR_STAT register). The valid range is 0-255, with the additional restriction that it may not be set to value larger than the depth of the buffer. If an attempt is made to do that, the actual value set will be the maximum depth of the buffer. A value of 0 sets the threshold for 0 entries, and a value of 255 sets the threshold for 255 entries.
    #[inline(always)]
    pub const fn with_tx_tl(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !IcTxTl::TX_TL_MASK)
                | ((value << IcTxTl::TX_TL_OFFSET) & IcTxTl::TX_TL_MASK),
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
    pub fn read_val(&self) -> IcTxTlVal {
        IcTxTlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: IcTxTlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(IcTxTlVal) -> IcTxTlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
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
/// - **CLR_INTR** `[0]` (RO)
///
///   Read this register to clear the combined interrupt, all individual interrupts, and the IC_TX_ABRT_SOURCE register. This bit does not clear hardware clearable interrupts but software clearable interrupts. Refer to Bit 9 of the IC_TX_ABRT_SOURCE register for an exception to clearing IC_TX_ABRT_SOURCE.
///   Reset value: 0x0
#[repr(transparent)]
pub struct IcClrIntr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for IcClrIntr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl IcClrIntr {
    // ======== Field Constants ========

    /// **CLR_INTR**
    ///
    /// Read this register to clear the combined interrupt, all individual interrupts, and the IC_TX_ABRT_SOURCE register. This bit does not clear hardware clearable interrupts but software clearable interrupts. Refer to Bit 9 of the IC_TX_ABRT_SOURCE register for an exception to clearing IC_TX_ABRT_SOURCE.
    /// Reset value: 0x0
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const CLR_INTR_OFFSET: u32 = 0;
    pub const CLR_INTR_MASK: u32 = 0x00000001;
    pub const CLR_INTR: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **CLR_INTR** `[0]` (RO)
    ///
    /// Read this register to clear the combined interrupt, all individual interrupts, and the IC_TX_ABRT_SOURCE register. This bit does not clear hardware clearable interrupts but software clearable interrupts. Refer to Bit 9 of the IC_TX_ABRT_SOURCE register for an exception to clearing IC_TX_ABRT_SOURCE.
    /// Reset value: 0x0
    #[inline(always)]
    pub fn clr_intr(&self) -> bool {
        (self.value.read_bits() & Self::CLR_INTR_MASK) != 0
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

    /// Read **CLR_INTR** `[0:0]`
    ///
    /// Read this register to clear the combined interrupt, all individual interrupts, and the IC_TX_ABRT_SOURCE register. This bit does not clear hardware clearable interrupts but software clearable interrupts. Refer to Bit 9 of the IC_TX_ABRT_SOURCE register for an exception to clearing IC_TX_ABRT_SOURCE.
    /// Reset value: 0x0
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
    pub fn read_val(&self) -> IcClrIntrVal {
        IcClrIntrVal::from_bits(self.value.read_bits())
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
/// - **CLR_RX_UNDER** `[0]` (RO)
///
///   Read this register to clear the RX_UNDER interrupt (bit 0) of the IC_RAW_INTR_STAT register.
///   Reset value: 0x0
#[repr(transparent)]
pub struct IcClrRxUnder {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for IcClrRxUnder {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl IcClrRxUnder {
    // ======== Field Constants ========

    /// **CLR_RX_UNDER**
    ///
    /// Read this register to clear the RX_UNDER interrupt (bit 0) of the IC_RAW_INTR_STAT register.
    /// Reset value: 0x0
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const CLR_RX_UNDER_OFFSET: u32 = 0;
    pub const CLR_RX_UNDER_MASK: u32 = 0x00000001;
    pub const CLR_RX_UNDER: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **CLR_RX_UNDER** `[0]` (RO)
    ///
    /// Read this register to clear the RX_UNDER interrupt (bit 0) of the IC_RAW_INTR_STAT register.
    /// Reset value: 0x0
    #[inline(always)]
    pub fn clr_rx_under(&self) -> bool {
        (self.value.read_bits() & Self::CLR_RX_UNDER_MASK) != 0
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

    /// Read **CLR_RX_UNDER** `[0:0]`
    ///
    /// Read this register to clear the RX_UNDER interrupt (bit 0) of the IC_RAW_INTR_STAT register.
    /// Reset value: 0x0
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
    pub fn read_val(&self) -> IcClrRxUnderVal {
        IcClrRxUnderVal::from_bits(self.value.read_bits())
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
/// - **CLR_RX_OVER** `[0]` (RO)
///
///   Read this register to clear the RX_OVER interrupt (bit 1) of the IC_RAW_INTR_STAT register.
///   Reset value: 0x0
#[repr(transparent)]
pub struct IcClrRxOver {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for IcClrRxOver {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl IcClrRxOver {
    // ======== Field Constants ========

    /// **CLR_RX_OVER**
    ///
    /// Read this register to clear the RX_OVER interrupt (bit 1) of the IC_RAW_INTR_STAT register.
    /// Reset value: 0x0
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const CLR_RX_OVER_OFFSET: u32 = 0;
    pub const CLR_RX_OVER_MASK: u32 = 0x00000001;
    pub const CLR_RX_OVER: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **CLR_RX_OVER** `[0]` (RO)
    ///
    /// Read this register to clear the RX_OVER interrupt (bit 1) of the IC_RAW_INTR_STAT register.
    /// Reset value: 0x0
    #[inline(always)]
    pub fn clr_rx_over(&self) -> bool {
        (self.value.read_bits() & Self::CLR_RX_OVER_MASK) != 0
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

    /// Read **CLR_RX_OVER** `[0:0]`
    ///
    /// Read this register to clear the RX_OVER interrupt (bit 1) of the IC_RAW_INTR_STAT register.
    /// Reset value: 0x0
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
    pub fn read_val(&self) -> IcClrRxOverVal {
        IcClrRxOverVal::from_bits(self.value.read_bits())
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
/// - **CLR_TX_OVER** `[0]` (RO)
///
///   Read this register to clear the TX_OVER interrupt (bit 3) of the IC_RAW_INTR_STAT register.
///   Reset value: 0x0
#[repr(transparent)]
pub struct IcClrTxOver {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for IcClrTxOver {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl IcClrTxOver {
    // ======== Field Constants ========

    /// **CLR_TX_OVER**
    ///
    /// Read this register to clear the TX_OVER interrupt (bit 3) of the IC_RAW_INTR_STAT register.
    /// Reset value: 0x0
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const CLR_TX_OVER_OFFSET: u32 = 0;
    pub const CLR_TX_OVER_MASK: u32 = 0x00000001;
    pub const CLR_TX_OVER: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **CLR_TX_OVER** `[0]` (RO)
    ///
    /// Read this register to clear the TX_OVER interrupt (bit 3) of the IC_RAW_INTR_STAT register.
    /// Reset value: 0x0
    #[inline(always)]
    pub fn clr_tx_over(&self) -> bool {
        (self.value.read_bits() & Self::CLR_TX_OVER_MASK) != 0
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

    /// Read **CLR_TX_OVER** `[0:0]`
    ///
    /// Read this register to clear the TX_OVER interrupt (bit 3) of the IC_RAW_INTR_STAT register.
    /// Reset value: 0x0
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
    pub fn read_val(&self) -> IcClrTxOverVal {
        IcClrTxOverVal::from_bits(self.value.read_bits())
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
/// - **CLR_RD_REQ** `[0]` (RO)
///
///   Read this register to clear the RD_REQ interrupt (bit 5) of the IC_RAW_INTR_STAT register.
///   Reset value: 0x0
#[repr(transparent)]
pub struct IcClrRdReq {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for IcClrRdReq {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl IcClrRdReq {
    // ======== Field Constants ========

    /// **CLR_RD_REQ**
    ///
    /// Read this register to clear the RD_REQ interrupt (bit 5) of the IC_RAW_INTR_STAT register.
    /// Reset value: 0x0
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const CLR_RD_REQ_OFFSET: u32 = 0;
    pub const CLR_RD_REQ_MASK: u32 = 0x00000001;
    pub const CLR_RD_REQ: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **CLR_RD_REQ** `[0]` (RO)
    ///
    /// Read this register to clear the RD_REQ interrupt (bit 5) of the IC_RAW_INTR_STAT register.
    /// Reset value: 0x0
    #[inline(always)]
    pub fn clr_rd_req(&self) -> bool {
        (self.value.read_bits() & Self::CLR_RD_REQ_MASK) != 0
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

    /// Read **CLR_RD_REQ** `[0:0]`
    ///
    /// Read this register to clear the RD_REQ interrupt (bit 5) of the IC_RAW_INTR_STAT register.
    /// Reset value: 0x0
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
    pub fn read_val(&self) -> IcClrRdReqVal {
        IcClrRdReqVal::from_bits(self.value.read_bits())
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
/// - **CLR_TX_ABRT** `[0]` (RO)
///
///   Read this register to clear the TX_ABRT interrupt (bit 6) of the IC_RAW_INTR_STAT register, and the IC_TX_ABRT_SOURCE register. This also releases the TX FIFO from the flushed/reset state, allowing more writes to the TX FIFO. Refer to Bit 9 of the IC_TX_ABRT_SOURCE register for an exception to clearing IC_TX_ABRT_SOURCE.
///   Reset value: 0x0
#[repr(transparent)]
pub struct IcClrTxAbrt {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for IcClrTxAbrt {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl IcClrTxAbrt {
    // ======== Field Constants ========

    /// **CLR_TX_ABRT**
    ///
    /// Read this register to clear the TX_ABRT interrupt (bit 6) of the IC_RAW_INTR_STAT register, and the IC_TX_ABRT_SOURCE register. This also releases the TX FIFO from the flushed/reset state, allowing more writes to the TX FIFO. Refer to Bit 9 of the IC_TX_ABRT_SOURCE register for an exception to clearing IC_TX_ABRT_SOURCE.
    /// Reset value: 0x0
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const CLR_TX_ABRT_OFFSET: u32 = 0;
    pub const CLR_TX_ABRT_MASK: u32 = 0x00000001;
    pub const CLR_TX_ABRT: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **CLR_TX_ABRT** `[0]` (RO)
    ///
    /// Read this register to clear the TX_ABRT interrupt (bit 6) of the IC_RAW_INTR_STAT register, and the IC_TX_ABRT_SOURCE register. This also releases the TX FIFO from the flushed/reset state, allowing more writes to the TX FIFO. Refer to Bit 9 of the IC_TX_ABRT_SOURCE register for an exception to clearing IC_TX_ABRT_SOURCE.
    /// Reset value: 0x0
    #[inline(always)]
    pub fn clr_tx_abrt(&self) -> bool {
        (self.value.read_bits() & Self::CLR_TX_ABRT_MASK) != 0
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

    /// Read **CLR_TX_ABRT** `[0:0]`
    ///
    /// Read this register to clear the TX_ABRT interrupt (bit 6) of the IC_RAW_INTR_STAT register, and the IC_TX_ABRT_SOURCE register. This also releases the TX FIFO from the flushed/reset state, allowing more writes to the TX FIFO. Refer to Bit 9 of the IC_TX_ABRT_SOURCE register for an exception to clearing IC_TX_ABRT_SOURCE.
    /// Reset value: 0x0
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
    pub fn read_val(&self) -> IcClrTxAbrtVal {
        IcClrTxAbrtVal::from_bits(self.value.read_bits())
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
/// - **CLR_RX_DONE** `[0]` (RO)
///
///   Read this register to clear the RX_DONE interrupt (bit 7) of the IC_RAW_INTR_STAT register.
///   Reset value: 0x0
#[repr(transparent)]
pub struct IcClrRxDone {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for IcClrRxDone {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl IcClrRxDone {
    // ======== Field Constants ========

    /// **CLR_RX_DONE**
    ///
    /// Read this register to clear the RX_DONE interrupt (bit 7) of the IC_RAW_INTR_STAT register.
    /// Reset value: 0x0
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const CLR_RX_DONE_OFFSET: u32 = 0;
    pub const CLR_RX_DONE_MASK: u32 = 0x00000001;
    pub const CLR_RX_DONE: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **CLR_RX_DONE** `[0]` (RO)
    ///
    /// Read this register to clear the RX_DONE interrupt (bit 7) of the IC_RAW_INTR_STAT register.
    /// Reset value: 0x0
    #[inline(always)]
    pub fn clr_rx_done(&self) -> bool {
        (self.value.read_bits() & Self::CLR_RX_DONE_MASK) != 0
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

    /// Read **CLR_RX_DONE** `[0:0]`
    ///
    /// Read this register to clear the RX_DONE interrupt (bit 7) of the IC_RAW_INTR_STAT register.
    /// Reset value: 0x0
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
    pub fn read_val(&self) -> IcClrRxDoneVal {
        IcClrRxDoneVal::from_bits(self.value.read_bits())
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
/// - **CLR_ACTIVITY** `[0]` (RO)
///
///   Reading this register clears the ACTIVITY interrupt if the I2C is not active anymore. If the I2C module is still active on the bus, the ACTIVITY interrupt bit continues to be set. It is automatically cleared by hardware if the module is disabled and if there is no further activity on the bus. The value read from this register to get status of the ACTIVITY interrupt (bit 8) of the IC_RAW_INTR_STAT register.
///   Reset value: 0x0
#[repr(transparent)]
pub struct IcClrActivity {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for IcClrActivity {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl IcClrActivity {
    // ======== Field Constants ========

    /// **CLR_ACTIVITY**
    ///
    /// Reading this register clears the ACTIVITY interrupt if the I2C is not active anymore. If the I2C module is still active on the bus, the ACTIVITY interrupt bit continues to be set. It is automatically cleared by hardware if the module is disabled and if there is no further activity on the bus. The value read from this register to get status of the ACTIVITY interrupt (bit 8) of the IC_RAW_INTR_STAT register.
    /// Reset value: 0x0
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const CLR_ACTIVITY_OFFSET: u32 = 0;
    pub const CLR_ACTIVITY_MASK: u32 = 0x00000001;
    pub const CLR_ACTIVITY: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **CLR_ACTIVITY** `[0]` (RO)
    ///
    /// Reading this register clears the ACTIVITY interrupt if the I2C is not active anymore. If the I2C module is still active on the bus, the ACTIVITY interrupt bit continues to be set. It is automatically cleared by hardware if the module is disabled and if there is no further activity on the bus. The value read from this register to get status of the ACTIVITY interrupt (bit 8) of the IC_RAW_INTR_STAT register.
    /// Reset value: 0x0
    #[inline(always)]
    pub fn clr_activity(&self) -> bool {
        (self.value.read_bits() & Self::CLR_ACTIVITY_MASK) != 0
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

    /// Read **CLR_ACTIVITY** `[0:0]`
    ///
    /// Reading this register clears the ACTIVITY interrupt if the I2C is not active anymore. If the I2C module is still active on the bus, the ACTIVITY interrupt bit continues to be set. It is automatically cleared by hardware if the module is disabled and if there is no further activity on the bus. The value read from this register to get status of the ACTIVITY interrupt (bit 8) of the IC_RAW_INTR_STAT register.
    /// Reset value: 0x0
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
    pub fn read_val(&self) -> IcClrActivityVal {
        IcClrActivityVal::from_bits(self.value.read_bits())
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
/// - **CLR_STOP_DET** `[0]` (RO)
///
///   Read this register to clear the STOP_DET interrupt (bit 9) of the IC_RAW_INTR_STAT register.
///   Reset value: 0x0
#[repr(transparent)]
pub struct IcClrStopDet {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for IcClrStopDet {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl IcClrStopDet {
    // ======== Field Constants ========

    /// **CLR_STOP_DET**
    ///
    /// Read this register to clear the STOP_DET interrupt (bit 9) of the IC_RAW_INTR_STAT register.
    /// Reset value: 0x0
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const CLR_STOP_DET_OFFSET: u32 = 0;
    pub const CLR_STOP_DET_MASK: u32 = 0x00000001;
    pub const CLR_STOP_DET: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **CLR_STOP_DET** `[0]` (RO)
    ///
    /// Read this register to clear the STOP_DET interrupt (bit 9) of the IC_RAW_INTR_STAT register.
    /// Reset value: 0x0
    #[inline(always)]
    pub fn clr_stop_det(&self) -> bool {
        (self.value.read_bits() & Self::CLR_STOP_DET_MASK) != 0
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

    /// Read **CLR_STOP_DET** `[0:0]`
    ///
    /// Read this register to clear the STOP_DET interrupt (bit 9) of the IC_RAW_INTR_STAT register.
    /// Reset value: 0x0
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
    pub fn read_val(&self) -> IcClrStopDetVal {
        IcClrStopDetVal::from_bits(self.value.read_bits())
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
/// - **CLR_START_DET** `[0]` (RO)
///
///   Read this register to clear the START_DET interrupt (bit 10) of the IC_RAW_INTR_STAT register.
///   Reset value: 0x0
#[repr(transparent)]
pub struct IcClrStartDet {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for IcClrStartDet {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl IcClrStartDet {
    // ======== Field Constants ========

    /// **CLR_START_DET**
    ///
    /// Read this register to clear the START_DET interrupt (bit 10) of the IC_RAW_INTR_STAT register.
    /// Reset value: 0x0
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const CLR_START_DET_OFFSET: u32 = 0;
    pub const CLR_START_DET_MASK: u32 = 0x00000001;
    pub const CLR_START_DET: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **CLR_START_DET** `[0]` (RO)
    ///
    /// Read this register to clear the START_DET interrupt (bit 10) of the IC_RAW_INTR_STAT register.
    /// Reset value: 0x0
    #[inline(always)]
    pub fn clr_start_det(&self) -> bool {
        (self.value.read_bits() & Self::CLR_START_DET_MASK) != 0
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

    /// Read **CLR_START_DET** `[0:0]`
    ///
    /// Read this register to clear the START_DET interrupt (bit 10) of the IC_RAW_INTR_STAT register.
    /// Reset value: 0x0
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
    pub fn read_val(&self) -> IcClrStartDetVal {
        IcClrStartDetVal::from_bits(self.value.read_bits())
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
/// - **CLR_GEN_CALL** `[0]` (RO)
///
///   Read this register to clear the GEN_CALL interrupt (bit 11) of IC_RAW_INTR_STAT register.
///   Reset value: 0x0
#[repr(transparent)]
pub struct IcClrGenCall {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for IcClrGenCall {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl IcClrGenCall {
    // ======== Field Constants ========

    /// **CLR_GEN_CALL**
    ///
    /// Read this register to clear the GEN_CALL interrupt (bit 11) of IC_RAW_INTR_STAT register.
    /// Reset value: 0x0
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const CLR_GEN_CALL_OFFSET: u32 = 0;
    pub const CLR_GEN_CALL_MASK: u32 = 0x00000001;
    pub const CLR_GEN_CALL: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **CLR_GEN_CALL** `[0]` (RO)
    ///
    /// Read this register to clear the GEN_CALL interrupt (bit 11) of IC_RAW_INTR_STAT register.
    /// Reset value: 0x0
    #[inline(always)]
    pub fn clr_gen_call(&self) -> bool {
        (self.value.read_bits() & Self::CLR_GEN_CALL_MASK) != 0
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

    /// Read **CLR_GEN_CALL** `[0:0]`
    ///
    /// Read this register to clear the GEN_CALL interrupt (bit 11) of IC_RAW_INTR_STAT register.
    /// Reset value: 0x0
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
    pub fn read_val(&self) -> IcClrGenCallVal {
        IcClrGenCallVal::from_bits(self.value.read_bits())
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
/// - **TX_CMD_BLOCK** `[2]` (RW)
///
///   In Master mode: - 1'b1: Blocks the transmission of data on I2C bus even if Tx FIFO has data to transmit. - 1'b0: The transmission of data starts on I2C bus automatically, as soon as the first data is available in the Tx FIFO. Note: To block the execution of Master commands, set the TX_CMD_BLOCK bit only when Tx FIFO is empty (IC_STATUS[2]==1) and Master is in Idle state (IC_STATUS[5] == 0). Any further commands put in the Tx FIFO are not executed until TX_CMD_BLOCK bit is unset. Reset value: IC_TX_CMD_BLOCK_DEFAULT
///
/// - **ABORT** `[1]` (RW)
///
///   When set, the controller initiates the transfer abort. - 0: ABORT not initiated or ABORT done - 1: ABORT operation in progress The software can abort the I2C transfer in master mode by setting this bit. The software can set this bit only when ENABLE is already set; otherwise, the controller ignores any write to ABORT bit. The software cannot clear the ABORT bit once set. In response to an ABORT, the controller issues a STOP and flushes the Tx FIFO after completing the current transfer, then sets the TX_ABORT interrupt after the abort operation. The ABORT bit is cleared automatically after the abort operation.
///   For a detailed description on how to abort I2C transfers, refer to 'Aborting I2C Transfers'.
///   Reset value: 0x0
///
/// - **ENABLE** `[0]` (RW)
///
///   Controls whether the DW_apb_i2c is enabled. - 0: Disables DW_apb_i2c (TX and RX FIFOs are held in an erased state) - 1: Enables DW_apb_i2c Software can disable DW_apb_i2c while it is active. However, it is important that care be taken to ensure that DW_apb_i2c is disabled properly. A recommended procedure is described in 'Disabling DW_apb_i2c'.
///   When DW_apb_i2c is disabled, the following occurs: - The TX FIFO and RX FIFO get flushed. - Status bits in the IC_INTR_STAT register are still active until DW_apb_i2c goes into IDLE state. If the module is transmitting, it stops as well as deletes the contents of the transmit buffer after the current transfer is complete. If the module is receiving, the DW_apb_i2c stops the current transfer at the end of the current byte and does not acknowledge the transfer.
///   In systems with asynchronous pclk and ic_clk when IC_CLK_TYPE parameter set to asynchronous (1), there is a two ic_clk delay when enabling or disabling the DW_apb_i2c. For a detailed description on how to disable DW_apb_i2c, refer to 'Disabling DW_apb_i2c'
///   Reset value: 0x0
#[repr(transparent)]
pub struct IcEnable {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for IcEnable {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl IcEnable {
    // ======== Field Constants ========

    /// **TX_CMD_BLOCK**
    ///
    /// In Master mode: - 1'b1: Blocks the transmission of data on I2C bus even if Tx FIFO has data to transmit. - 1'b0: The transmission of data starts on I2C bus automatically, as soon as the first data is available in the Tx FIFO. Note: To block the execution of Master commands, set the TX_CMD_BLOCK bit only when Tx FIFO is empty (IC_STATUS[2]==1) and Master is in Idle state (IC_STATUS[5] == 0). Any further commands put in the Tx FIFO are not executed until TX_CMD_BLOCK bit is unset. Reset value: IC_TX_CMD_BLOCK_DEFAULT
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const TX_CMD_BLOCK_OFFSET: u32 = 2;
    pub const TX_CMD_BLOCK_MASK: u32 = 0x00000004;
    pub const TX_CMD_BLOCK: u32 = 1 << 2;
    /// NOT_BLOCKED = 0
    /// Tx Command execution not blocked
    pub const TX_CMD_BLOCK_NOT_BLOCKED: u32 = 0 << 2;
    pub const TX_CMD_BLOCK_NOT_BLOCKED_VAL: u32 = 0;
    /// BLOCKED = 1
    /// Tx Command execution blocked
    pub const TX_CMD_BLOCK_BLOCKED: u32 = 1 << 2;
    pub const TX_CMD_BLOCK_BLOCKED_VAL: u32 = 1;

    /// **ABORT**
    ///
    /// When set, the controller initiates the transfer abort. - 0: ABORT not initiated or ABORT done - 1: ABORT operation in progress The software can abort the I2C transfer in master mode by setting this bit. The software can set this bit only when ENABLE is already set; otherwise, the controller ignores any write to ABORT bit. The software cannot clear the ABORT bit once set. In response to an ABORT, the controller issues a STOP and flushes the Tx FIFO after completing the current transfer, then sets the TX_ABORT interrupt after the abort operation. The ABORT bit is cleared automatically after the abort operation.
    /// For a detailed description on how to abort I2C transfers, refer to 'Aborting I2C Transfers'.
    /// Reset value: 0x0
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const ABORT_OFFSET: u32 = 1;
    pub const ABORT_MASK: u32 = 0x00000002;
    pub const ABORT: u32 = 1 << 1;
    /// DISABLE = 0
    /// ABORT operation not in progress
    pub const ABORT_DISABLE: u32 = 0 << 1;
    pub const ABORT_DISABLE_VAL: u32 = 0;
    /// ENABLED = 1
    /// ABORT operation in progress
    pub const ABORT_ENABLED: u32 = 1 << 1;
    pub const ABORT_ENABLED_VAL: u32 = 1;

    /// **ENABLE**
    ///
    /// Controls whether the DW_apb_i2c is enabled. - 0: Disables DW_apb_i2c (TX and RX FIFOs are held in an erased state) - 1: Enables DW_apb_i2c Software can disable DW_apb_i2c while it is active. However, it is important that care be taken to ensure that DW_apb_i2c is disabled properly. A recommended procedure is described in 'Disabling DW_apb_i2c'.
    /// When DW_apb_i2c is disabled, the following occurs: - The TX FIFO and RX FIFO get flushed. - Status bits in the IC_INTR_STAT register are still active until DW_apb_i2c goes into IDLE state. If the module is transmitting, it stops as well as deletes the contents of the transmit buffer after the current transfer is complete. If the module is receiving, the DW_apb_i2c stops the current transfer at the end of the current byte and does not acknowledge the transfer.
    /// In systems with asynchronous pclk and ic_clk when IC_CLK_TYPE parameter set to asynchronous (1), there is a two ic_clk delay when enabling or disabling the DW_apb_i2c. For a detailed description on how to disable DW_apb_i2c, refer to 'Disabling DW_apb_i2c'
    /// Reset value: 0x0
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const ENABLE_OFFSET: u32 = 0;
    pub const ENABLE_MASK: u32 = 0x00000001;
    pub const ENABLE: u32 = 1 << 0;
    /// DISABLED = 0
    /// I2C is disabled
    pub const ENABLE_DISABLED: u32 = 0 << 0;
    pub const ENABLE_DISABLED_VAL: u32 = 0;
    /// ENABLED = 1
    /// I2C is enabled
    pub const ENABLE_ENABLED: u32 = 1 << 0;
    pub const ENABLE_ENABLED_VAL: u32 = 1;

    // ======== Field Accessors ========

    /// **TX_CMD_BLOCK** `[2]` (RW)
    ///
    /// In Master mode: - 1'b1: Blocks the transmission of data on I2C bus even if Tx FIFO has data to transmit. - 1'b0: The transmission of data starts on I2C bus automatically, as soon as the first data is available in the Tx FIFO. Note: To block the execution of Master commands, set the TX_CMD_BLOCK bit only when Tx FIFO is empty (IC_STATUS[2]==1) and Master is in Idle state (IC_STATUS[5] == 0). Any further commands put in the Tx FIFO are not executed until TX_CMD_BLOCK bit is unset. Reset value: IC_TX_CMD_BLOCK_DEFAULT
    #[inline(always)]
    pub fn tx_cmd_block(&self) -> bool {
        (self.value.read_bits() & Self::TX_CMD_BLOCK_MASK) != 0
    }

    /// Set TX_CMD_BLOCK bit atomically (RW)
    #[inline(always)]
    pub fn set_tx_cmd_block(&self) {
        self.set_bits(Self::TX_CMD_BLOCK_MASK);
    }

    /// Clear TX_CMD_BLOCK bit atomically (RW)
    #[inline(always)]
    pub fn clr_tx_cmd_block(&self) {
        self.clr_bits(Self::TX_CMD_BLOCK_MASK);
    }

    /// **ABORT** `[1]` (RW)
    ///
    /// When set, the controller initiates the transfer abort. - 0: ABORT not initiated or ABORT done - 1: ABORT operation in progress The software can abort the I2C transfer in master mode by setting this bit. The software can set this bit only when ENABLE is already set; otherwise, the controller ignores any write to ABORT bit. The software cannot clear the ABORT bit once set. In response to an ABORT, the controller issues a STOP and flushes the Tx FIFO after completing the current transfer, then sets the TX_ABORT interrupt after the abort operation. The ABORT bit is cleared automatically after the abort operation.
    /// For a detailed description on how to abort I2C transfers, refer to 'Aborting I2C Transfers'.
    /// Reset value: 0x0
    #[inline(always)]
    pub fn abort(&self) -> bool {
        (self.value.read_bits() & Self::ABORT_MASK) != 0
    }

    /// Set ABORT bit atomically (RW)
    #[inline(always)]
    pub fn set_abort(&self) {
        self.set_bits(Self::ABORT_MASK);
    }

    /// Clear ABORT bit atomically (RW)
    #[inline(always)]
    pub fn clr_abort(&self) {
        self.clr_bits(Self::ABORT_MASK);
    }

    /// **ENABLE** `[0]` (RW)
    ///
    /// Controls whether the DW_apb_i2c is enabled. - 0: Disables DW_apb_i2c (TX and RX FIFOs are held in an erased state) - 1: Enables DW_apb_i2c Software can disable DW_apb_i2c while it is active. However, it is important that care be taken to ensure that DW_apb_i2c is disabled properly. A recommended procedure is described in 'Disabling DW_apb_i2c'.
    /// When DW_apb_i2c is disabled, the following occurs: - The TX FIFO and RX FIFO get flushed. - Status bits in the IC_INTR_STAT register are still active until DW_apb_i2c goes into IDLE state. If the module is transmitting, it stops as well as deletes the contents of the transmit buffer after the current transfer is complete. If the module is receiving, the DW_apb_i2c stops the current transfer at the end of the current byte and does not acknowledge the transfer.
    /// In systems with asynchronous pclk and ic_clk when IC_CLK_TYPE parameter set to asynchronous (1), there is a two ic_clk delay when enabling or disabling the DW_apb_i2c. For a detailed description on how to disable DW_apb_i2c, refer to 'Disabling DW_apb_i2c'
    /// Reset value: 0x0
    #[inline(always)]
    pub fn enable(&self) -> bool {
        (self.value.read_bits() & Self::ENABLE_MASK) != 0
    }

    /// Set ENABLE bit atomically (RW)
    #[inline(always)]
    pub fn set_enable(&self) {
        self.set_bits(Self::ENABLE_MASK);
    }

    /// Clear ENABLE bit atomically (RW)
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

    /// Read **TX_CMD_BLOCK** `[2:2]`
    ///
    /// In Master mode: - 1'b1: Blocks the transmission of data on I2C bus even if Tx FIFO has data to transmit. - 1'b0: The transmission of data starts on I2C bus automatically, as soon as the first data is available in the Tx FIFO. Note: To block the execution of Master commands, set the TX_CMD_BLOCK bit only when Tx FIFO is empty (IC_STATUS[2]==1) and Master is in Idle state (IC_STATUS[5] == 0). Any further commands put in the Tx FIFO are not executed until TX_CMD_BLOCK bit is unset. Reset value: IC_TX_CMD_BLOCK_DEFAULT
    #[inline(always)]
    pub const fn tx_cmd_block(&self) -> bool {
        (self.bits & IcEnable::TX_CMD_BLOCK_MASK) != 0
    }

    /// Read **ABORT** `[1:1]`
    ///
    /// When set, the controller initiates the transfer abort. - 0: ABORT not initiated or ABORT done - 1: ABORT operation in progress The software can abort the I2C transfer in master mode by setting this bit. The software can set this bit only when ENABLE is already set; otherwise, the controller ignores any write to ABORT bit. The software cannot clear the ABORT bit once set. In response to an ABORT, the controller issues a STOP and flushes the Tx FIFO after completing the current transfer, then sets the TX_ABORT interrupt after the abort operation. The ABORT bit is cleared automatically after the abort operation.
    /// For a detailed description on how to abort I2C transfers, refer to 'Aborting I2C Transfers'.
    /// Reset value: 0x0
    #[inline(always)]
    pub const fn abort(&self) -> bool {
        (self.bits & IcEnable::ABORT_MASK) != 0
    }

    /// Read **ENABLE** `[0:0]`
    ///
    /// Controls whether the DW_apb_i2c is enabled. - 0: Disables DW_apb_i2c (TX and RX FIFOs are held in an erased state) - 1: Enables DW_apb_i2c Software can disable DW_apb_i2c while it is active. However, it is important that care be taken to ensure that DW_apb_i2c is disabled properly. A recommended procedure is described in 'Disabling DW_apb_i2c'.
    /// When DW_apb_i2c is disabled, the following occurs: - The TX FIFO and RX FIFO get flushed. - Status bits in the IC_INTR_STAT register are still active until DW_apb_i2c goes into IDLE state. If the module is transmitting, it stops as well as deletes the contents of the transmit buffer after the current transfer is complete. If the module is receiving, the DW_apb_i2c stops the current transfer at the end of the current byte and does not acknowledge the transfer.
    /// In systems with asynchronous pclk and ic_clk when IC_CLK_TYPE parameter set to asynchronous (1), there is a two ic_clk delay when enabling or disabling the DW_apb_i2c. For a detailed description on how to disable DW_apb_i2c, refer to 'Disabling DW_apb_i2c'
    /// Reset value: 0x0
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        (self.bits & IcEnable::ENABLE_MASK) != 0
    }

    /// Set **TX_CMD_BLOCK** `[2:2]`
    ///
    /// In Master mode: - 1'b1: Blocks the transmission of data on I2C bus even if Tx FIFO has data to transmit. - 1'b0: The transmission of data starts on I2C bus automatically, as soon as the first data is available in the Tx FIFO. Note: To block the execution of Master commands, set the TX_CMD_BLOCK bit only when Tx FIFO is empty (IC_STATUS[2]==1) and Master is in Idle state (IC_STATUS[5] == 0). Any further commands put in the Tx FIFO are not executed until TX_CMD_BLOCK bit is unset. Reset value: IC_TX_CMD_BLOCK_DEFAULT
    #[inline(always)]
    pub const fn with_tx_cmd_block(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IcEnable::TX_CMD_BLOCK_MASK
            } else {
                self.bits & !IcEnable::TX_CMD_BLOCK_MASK
            },
        }
    }

    /// Set **ABORT** `[1:1]`
    ///
    /// When set, the controller initiates the transfer abort. - 0: ABORT not initiated or ABORT done - 1: ABORT operation in progress The software can abort the I2C transfer in master mode by setting this bit. The software can set this bit only when ENABLE is already set; otherwise, the controller ignores any write to ABORT bit. The software cannot clear the ABORT bit once set. In response to an ABORT, the controller issues a STOP and flushes the Tx FIFO after completing the current transfer, then sets the TX_ABORT interrupt after the abort operation. The ABORT bit is cleared automatically after the abort operation.
    /// For a detailed description on how to abort I2C transfers, refer to 'Aborting I2C Transfers'.
    /// Reset value: 0x0
    #[inline(always)]
    pub const fn with_abort(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IcEnable::ABORT_MASK
            } else {
                self.bits & !IcEnable::ABORT_MASK
            },
        }
    }

    /// Set **ENABLE** `[0:0]`
    ///
    /// Controls whether the DW_apb_i2c is enabled. - 0: Disables DW_apb_i2c (TX and RX FIFOs are held in an erased state) - 1: Enables DW_apb_i2c Software can disable DW_apb_i2c while it is active. However, it is important that care be taken to ensure that DW_apb_i2c is disabled properly. A recommended procedure is described in 'Disabling DW_apb_i2c'.
    /// When DW_apb_i2c is disabled, the following occurs: - The TX FIFO and RX FIFO get flushed. - Status bits in the IC_INTR_STAT register are still active until DW_apb_i2c goes into IDLE state. If the module is transmitting, it stops as well as deletes the contents of the transmit buffer after the current transfer is complete. If the module is receiving, the DW_apb_i2c stops the current transfer at the end of the current byte and does not acknowledge the transfer.
    /// In systems with asynchronous pclk and ic_clk when IC_CLK_TYPE parameter set to asynchronous (1), there is a two ic_clk delay when enabling or disabling the DW_apb_i2c. For a detailed description on how to disable DW_apb_i2c, refer to 'Disabling DW_apb_i2c'
    /// Reset value: 0x0
    #[inline(always)]
    pub const fn with_enable(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IcEnable::ENABLE_MASK
            } else {
                self.bits & !IcEnable::ENABLE_MASK
            },
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
    pub fn read_val(&self) -> IcEnableVal {
        IcEnableVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: IcEnableVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(IcEnableVal) -> IcEnableVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// IC_STATUS Register
///
/// I2C Status Register
/// This is a read-only register used to indicate the current transfer status and FIFO status. The status register may be read at any time. None of the bits in this register request an interrupt.
/// When the I2C is disabled by writing 0 in bit 0 of the IC_ENABLE register: - Bits 1 and 2 are set to 1 - Bits 3 and 10 are set to 0 When the master or slave state machines goes to idle and ic_en=0: - Bits 5 and 6 are set to 0
///
/// Address offset: 0x0070
/// Reset value: 0x00000006
///
/// ## Fields
///
/// - **SLV_ACTIVITY** `[6]` (RO)
///
///   Slave FSM Activity Status. When the Slave Finite State Machine (FSM) is not in the IDLE state, this bit is set. - 0: Slave FSM is in IDLE state so the Slave part of DW_apb_i2c is not Active - 1: Slave FSM is not in IDLE state so the Slave part of DW_apb_i2c is Active Reset value: 0x0
///
/// - **MST_ACTIVITY** `[5]` (RO)
///
///   Master FSM Activity Status. When the Master Finite State Machine (FSM) is not in the IDLE state, this bit is set. - 0: Master FSM is in IDLE state so the Master part of DW_apb_i2c is not Active - 1: Master FSM is not in IDLE state so the Master part of DW_apb_i2c is Active Note: IC_STATUS[0]-that is, ACTIVITY bit-is the OR of SLV_ACTIVITY and MST_ACTIVITY bits.
///   Reset value: 0x0
///
/// - **RFF** `[4]` (RO)
///
///   Receive FIFO Completely Full. When the receive FIFO is completely full, this bit is set. When the receive FIFO contains one or more empty location, this bit is cleared. - 0: Receive FIFO is not full - 1: Receive FIFO is full Reset value: 0x0
///
/// - **RFNE** `[3]` (RO)
///
///   Receive FIFO Not Empty. This bit is set when the receive FIFO contains one or more entries; it is cleared when the receive FIFO is empty. - 0: Receive FIFO is empty - 1: Receive FIFO is not empty Reset value: 0x0
///
/// - **TFE** `[2]` (RO)
///
///   Transmit FIFO Completely Empty. When the transmit FIFO is completely empty, this bit is set. When it contains one or more valid entries, this bit is cleared. This bit field does not request an interrupt. - 0: Transmit FIFO is not empty - 1: Transmit FIFO is empty Reset value: 0x1
///
/// - **TFNF** `[1]` (RO)
///
///   Transmit FIFO Not Full. Set when the transmit FIFO contains one or more empty locations, and is cleared when the FIFO is full. - 0: Transmit FIFO is full - 1: Transmit FIFO is not full Reset value: 0x1
///
/// - **ACTIVITY** `[0]` (RO)
///
///   I2C Activity Status. Reset value: 0x0
#[repr(transparent)]
pub struct IcStatus {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for IcStatus {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl IcStatus {
    // ======== Field Constants ========

    /// **SLV_ACTIVITY**
    ///
    /// Slave FSM Activity Status. When the Slave Finite State Machine (FSM) is not in the IDLE state, this bit is set. - 0: Slave FSM is in IDLE state so the Slave part of DW_apb_i2c is not Active - 1: Slave FSM is not in IDLE state so the Slave part of DW_apb_i2c is Active Reset value: 0x0
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const SLV_ACTIVITY_OFFSET: u32 = 6;
    pub const SLV_ACTIVITY_MASK: u32 = 0x00000040;
    pub const SLV_ACTIVITY: u32 = 1 << 6;
    /// IDLE = 0
    /// Slave is idle
    pub const SLV_ACTIVITY_IDLE: u32 = 0 << 6;
    pub const SLV_ACTIVITY_IDLE_VAL: u32 = 0;
    /// ACTIVE = 1
    /// Slave not idle
    pub const SLV_ACTIVITY_ACTIVE: u32 = 1 << 6;
    pub const SLV_ACTIVITY_ACTIVE_VAL: u32 = 1;

    /// **MST_ACTIVITY**
    ///
    /// Master FSM Activity Status. When the Master Finite State Machine (FSM) is not in the IDLE state, this bit is set. - 0: Master FSM is in IDLE state so the Master part of DW_apb_i2c is not Active - 1: Master FSM is not in IDLE state so the Master part of DW_apb_i2c is Active Note: IC_STATUS[0]-that is, ACTIVITY bit-is the OR of SLV_ACTIVITY and MST_ACTIVITY bits.
    /// Reset value: 0x0
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const MST_ACTIVITY_OFFSET: u32 = 5;
    pub const MST_ACTIVITY_MASK: u32 = 0x00000020;
    pub const MST_ACTIVITY: u32 = 1 << 5;
    /// IDLE = 0
    /// Master is idle
    pub const MST_ACTIVITY_IDLE: u32 = 0 << 5;
    pub const MST_ACTIVITY_IDLE_VAL: u32 = 0;
    /// ACTIVE = 1
    /// Master not idle
    pub const MST_ACTIVITY_ACTIVE: u32 = 1 << 5;
    pub const MST_ACTIVITY_ACTIVE_VAL: u32 = 1;

    /// **RFF**
    ///
    /// Receive FIFO Completely Full. When the receive FIFO is completely full, this bit is set. When the receive FIFO contains one or more empty location, this bit is cleared. - 0: Receive FIFO is not full - 1: Receive FIFO is full Reset value: 0x0
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const RFF_OFFSET: u32 = 4;
    pub const RFF_MASK: u32 = 0x00000010;
    pub const RFF: u32 = 1 << 4;
    /// NOT_FULL = 0
    /// Rx FIFO not full
    pub const RFF_NOT_FULL: u32 = 0 << 4;
    pub const RFF_NOT_FULL_VAL: u32 = 0;
    /// FULL = 1
    /// Rx FIFO is full
    pub const RFF_FULL: u32 = 1 << 4;
    pub const RFF_FULL_VAL: u32 = 1;

    /// **RFNE**
    ///
    /// Receive FIFO Not Empty. This bit is set when the receive FIFO contains one or more entries; it is cleared when the receive FIFO is empty. - 0: Receive FIFO is empty - 1: Receive FIFO is not empty Reset value: 0x0
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const RFNE_OFFSET: u32 = 3;
    pub const RFNE_MASK: u32 = 0x00000008;
    pub const RFNE: u32 = 1 << 3;
    /// EMPTY = 0
    /// Rx FIFO is empty
    pub const RFNE_EMPTY: u32 = 0 << 3;
    pub const RFNE_EMPTY_VAL: u32 = 0;
    /// NOT_EMPTY = 1
    /// Rx FIFO not empty
    pub const RFNE_NOT_EMPTY: u32 = 1 << 3;
    pub const RFNE_NOT_EMPTY_VAL: u32 = 1;

    /// **TFE**
    ///
    /// Transmit FIFO Completely Empty. When the transmit FIFO is completely empty, this bit is set. When it contains one or more valid entries, this bit is cleared. This bit field does not request an interrupt. - 0: Transmit FIFO is not empty - 1: Transmit FIFO is empty Reset value: 0x1
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const TFE_OFFSET: u32 = 2;
    pub const TFE_MASK: u32 = 0x00000004;
    pub const TFE: u32 = 1 << 2;
    /// NON_EMPTY = 0
    /// Tx FIFO not empty
    pub const TFE_NON_EMPTY: u32 = 0 << 2;
    pub const TFE_NON_EMPTY_VAL: u32 = 0;
    /// EMPTY = 1
    /// Tx FIFO is empty
    pub const TFE_EMPTY: u32 = 1 << 2;
    pub const TFE_EMPTY_VAL: u32 = 1;

    /// **TFNF**
    ///
    /// Transmit FIFO Not Full. Set when the transmit FIFO contains one or more empty locations, and is cleared when the FIFO is full. - 0: Transmit FIFO is full - 1: Transmit FIFO is not full Reset value: 0x1
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const TFNF_OFFSET: u32 = 1;
    pub const TFNF_MASK: u32 = 0x00000002;
    pub const TFNF: u32 = 1 << 1;
    /// FULL = 0
    /// Tx FIFO is full
    pub const TFNF_FULL: u32 = 0 << 1;
    pub const TFNF_FULL_VAL: u32 = 0;
    /// NOT_FULL = 1
    /// Tx FIFO not full
    pub const TFNF_NOT_FULL: u32 = 1 << 1;
    pub const TFNF_NOT_FULL_VAL: u32 = 1;

    /// **ACTIVITY**
    ///
    /// I2C Activity Status. Reset value: 0x0
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const ACTIVITY_OFFSET: u32 = 0;
    pub const ACTIVITY_MASK: u32 = 0x00000001;
    pub const ACTIVITY: u32 = 1 << 0;
    /// INACTIVE = 0
    /// I2C is idle
    pub const ACTIVITY_INACTIVE: u32 = 0 << 0;
    pub const ACTIVITY_INACTIVE_VAL: u32 = 0;
    /// ACTIVE = 1
    /// I2C is active
    pub const ACTIVITY_ACTIVE: u32 = 1 << 0;
    pub const ACTIVITY_ACTIVE_VAL: u32 = 1;

    // ======== Field Accessors ========

    /// **SLV_ACTIVITY** `[6]` (RO)
    ///
    /// Slave FSM Activity Status. When the Slave Finite State Machine (FSM) is not in the IDLE state, this bit is set. - 0: Slave FSM is in IDLE state so the Slave part of DW_apb_i2c is not Active - 1: Slave FSM is not in IDLE state so the Slave part of DW_apb_i2c is Active Reset value: 0x0
    #[inline(always)]
    pub fn slv_activity(&self) -> bool {
        (self.value.read_bits() & Self::SLV_ACTIVITY_MASK) != 0
    }

    /// **MST_ACTIVITY** `[5]` (RO)
    ///
    /// Master FSM Activity Status. When the Master Finite State Machine (FSM) is not in the IDLE state, this bit is set. - 0: Master FSM is in IDLE state so the Master part of DW_apb_i2c is not Active - 1: Master FSM is not in IDLE state so the Master part of DW_apb_i2c is Active Note: IC_STATUS[0]-that is, ACTIVITY bit-is the OR of SLV_ACTIVITY and MST_ACTIVITY bits.
    /// Reset value: 0x0
    #[inline(always)]
    pub fn mst_activity(&self) -> bool {
        (self.value.read_bits() & Self::MST_ACTIVITY_MASK) != 0
    }

    /// **RFF** `[4]` (RO)
    ///
    /// Receive FIFO Completely Full. When the receive FIFO is completely full, this bit is set. When the receive FIFO contains one or more empty location, this bit is cleared. - 0: Receive FIFO is not full - 1: Receive FIFO is full Reset value: 0x0
    #[inline(always)]
    pub fn rff(&self) -> bool {
        (self.value.read_bits() & Self::RFF_MASK) != 0
    }

    /// **RFNE** `[3]` (RO)
    ///
    /// Receive FIFO Not Empty. This bit is set when the receive FIFO contains one or more entries; it is cleared when the receive FIFO is empty. - 0: Receive FIFO is empty - 1: Receive FIFO is not empty Reset value: 0x0
    #[inline(always)]
    pub fn rfne(&self) -> bool {
        (self.value.read_bits() & Self::RFNE_MASK) != 0
    }

    /// **TFE** `[2]` (RO)
    ///
    /// Transmit FIFO Completely Empty. When the transmit FIFO is completely empty, this bit is set. When it contains one or more valid entries, this bit is cleared. This bit field does not request an interrupt. - 0: Transmit FIFO is not empty - 1: Transmit FIFO is empty Reset value: 0x1
    #[inline(always)]
    pub fn tfe(&self) -> bool {
        (self.value.read_bits() & Self::TFE_MASK) != 0
    }

    /// **TFNF** `[1]` (RO)
    ///
    /// Transmit FIFO Not Full. Set when the transmit FIFO contains one or more empty locations, and is cleared when the FIFO is full. - 0: Transmit FIFO is full - 1: Transmit FIFO is not full Reset value: 0x1
    #[inline(always)]
    pub fn tfnf(&self) -> bool {
        (self.value.read_bits() & Self::TFNF_MASK) != 0
    }

    /// **ACTIVITY** `[0]` (RO)
    ///
    /// I2C Activity Status. Reset value: 0x0
    #[inline(always)]
    pub fn activity(&self) -> bool {
        (self.value.read_bits() & Self::ACTIVITY_MASK) != 0
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

    /// Read **SLV_ACTIVITY** `[6:6]`
    ///
    /// Slave FSM Activity Status. When the Slave Finite State Machine (FSM) is not in the IDLE state, this bit is set. - 0: Slave FSM is in IDLE state so the Slave part of DW_apb_i2c is not Active - 1: Slave FSM is not in IDLE state so the Slave part of DW_apb_i2c is Active Reset value: 0x0
    #[inline(always)]
    pub const fn slv_activity(&self) -> bool {
        (self.bits & IcStatus::SLV_ACTIVITY_MASK) != 0
    }

    /// Read **MST_ACTIVITY** `[5:5]`
    ///
    /// Master FSM Activity Status. When the Master Finite State Machine (FSM) is not in the IDLE state, this bit is set. - 0: Master FSM is in IDLE state so the Master part of DW_apb_i2c is not Active - 1: Master FSM is not in IDLE state so the Master part of DW_apb_i2c is Active Note: IC_STATUS[0]-that is, ACTIVITY bit-is the OR of SLV_ACTIVITY and MST_ACTIVITY bits.
    /// Reset value: 0x0
    #[inline(always)]
    pub const fn mst_activity(&self) -> bool {
        (self.bits & IcStatus::MST_ACTIVITY_MASK) != 0
    }

    /// Read **RFF** `[4:4]`
    ///
    /// Receive FIFO Completely Full. When the receive FIFO is completely full, this bit is set. When the receive FIFO contains one or more empty location, this bit is cleared. - 0: Receive FIFO is not full - 1: Receive FIFO is full Reset value: 0x0
    #[inline(always)]
    pub const fn rff(&self) -> bool {
        (self.bits & IcStatus::RFF_MASK) != 0
    }

    /// Read **RFNE** `[3:3]`
    ///
    /// Receive FIFO Not Empty. This bit is set when the receive FIFO contains one or more entries; it is cleared when the receive FIFO is empty. - 0: Receive FIFO is empty - 1: Receive FIFO is not empty Reset value: 0x0
    #[inline(always)]
    pub const fn rfne(&self) -> bool {
        (self.bits & IcStatus::RFNE_MASK) != 0
    }

    /// Read **TFE** `[2:2]`
    ///
    /// Transmit FIFO Completely Empty. When the transmit FIFO is completely empty, this bit is set. When it contains one or more valid entries, this bit is cleared. This bit field does not request an interrupt. - 0: Transmit FIFO is not empty - 1: Transmit FIFO is empty Reset value: 0x1
    #[inline(always)]
    pub const fn tfe(&self) -> bool {
        (self.bits & IcStatus::TFE_MASK) != 0
    }

    /// Read **TFNF** `[1:1]`
    ///
    /// Transmit FIFO Not Full. Set when the transmit FIFO contains one or more empty locations, and is cleared when the FIFO is full. - 0: Transmit FIFO is full - 1: Transmit FIFO is not full Reset value: 0x1
    #[inline(always)]
    pub const fn tfnf(&self) -> bool {
        (self.bits & IcStatus::TFNF_MASK) != 0
    }

    /// Read **ACTIVITY** `[0:0]`
    ///
    /// I2C Activity Status. Reset value: 0x0
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
    pub fn read_val(&self) -> IcStatusVal {
        IcStatusVal::from_bits(self.value.read_bits())
    }
}

/// IC_TXFLR Register
///
/// I2C Transmit FIFO Level Register This register contains the number of valid data entries in the transmit FIFO buffer. It is cleared whenever: - The I2C is disabled - There is a transmit abort - that is, TX_ABRT bit is set in the IC_RAW_INTR_STAT register - The slave bulk transmit mode is aborted The register increments whenever data is placed into the transmit FIFO and decrements when data is taken from the transmit FIFO.
///
/// Address offset: 0x0074
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **TXFLR** `[4:0]` (RO)
///
///   Transmit FIFO Level. Contains the number of valid data entries in the transmit FIFO.
///   Reset value: 0x0
#[repr(transparent)]
pub struct IcTxflr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for IcTxflr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl IcTxflr {
    // ======== Field Constants ========

    /// **TXFLR**
    ///
    /// Transmit FIFO Level. Contains the number of valid data entries in the transmit FIFO.
    /// Reset value: 0x0
    ///
    /// Bits `[4:0]`, mask `0x0000001f`
    pub const TXFLR_OFFSET: u32 = 0;
    pub const TXFLR_MASK: u32 = 0x0000001f;

    // ======== Field Accessors ========

    /// **TXFLR** `[4:0]` (RO)
    ///
    /// Transmit FIFO Level. Contains the number of valid data entries in the transmit FIFO.
    /// Reset value: 0x0
    #[inline(always)]
    pub fn txflr(&self) -> u32 {
        (self.value.read_bits() & Self::TXFLR_MASK) >> Self::TXFLR_OFFSET
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

    /// Read **TXFLR** `[4:0]`
    ///
    /// Transmit FIFO Level. Contains the number of valid data entries in the transmit FIFO.
    /// Reset value: 0x0
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
    pub fn read_val(&self) -> IcTxflrVal {
        IcTxflrVal::from_bits(self.value.read_bits())
    }
}

/// IC_RXFLR Register
///
/// I2C Receive FIFO Level Register This register contains the number of valid data entries in the receive FIFO buffer. It is cleared whenever: - The I2C is disabled - Whenever there is a transmit abort caused by any of the events tracked in IC_TX_ABRT_SOURCE The register increments whenever data is placed into the receive FIFO and decrements when data is taken from the receive FIFO.
///
/// Address offset: 0x0078
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **RXFLR** `[4:0]` (RO)
///
///   Receive FIFO Level. Contains the number of valid data entries in the receive FIFO.
///   Reset value: 0x0
#[repr(transparent)]
pub struct IcRxflr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for IcRxflr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl IcRxflr {
    // ======== Field Constants ========

    /// **RXFLR**
    ///
    /// Receive FIFO Level. Contains the number of valid data entries in the receive FIFO.
    /// Reset value: 0x0
    ///
    /// Bits `[4:0]`, mask `0x0000001f`
    pub const RXFLR_OFFSET: u32 = 0;
    pub const RXFLR_MASK: u32 = 0x0000001f;

    // ======== Field Accessors ========

    /// **RXFLR** `[4:0]` (RO)
    ///
    /// Receive FIFO Level. Contains the number of valid data entries in the receive FIFO.
    /// Reset value: 0x0
    #[inline(always)]
    pub fn rxflr(&self) -> u32 {
        (self.value.read_bits() & Self::RXFLR_MASK) >> Self::RXFLR_OFFSET
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

    /// Read **RXFLR** `[4:0]`
    ///
    /// Receive FIFO Level. Contains the number of valid data entries in the receive FIFO.
    /// Reset value: 0x0
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
    pub fn read_val(&self) -> IcRxflrVal {
        IcRxflrVal::from_bits(self.value.read_bits())
    }
}

/// IC_SDA_HOLD Register
///
/// I2C SDA Hold Time Length Register
/// The bits [15:0] of this register are used to control the hold time of SDA during transmit in both slave and master mode (after SCL goes from HIGH to LOW).
/// The bits [23:16] of this register are used to extend the SDA transition (if any) whenever SCL is HIGH in the receiver in either master or slave mode.
/// Writes to this register succeed only when IC_ENABLE[0]=0.
/// The values in this register are in units of ic_clk period. The value programmed in IC_SDA_TX_HOLD must be greater than the minimum hold time in each mode (one cycle in master mode, seven cycles in slave mode) for the value to be implemented.
/// The programmed SDA hold time during transmit (IC_SDA_TX_HOLD) cannot exceed at any time the duration of the low part of scl. Therefore the programmed value cannot be larger than N_SCL_LOW-2, where N_SCL_LOW is the duration of the low part of the scl period measured in ic_clk cycles.
///
/// Address offset: 0x007c
/// Reset value: 0x00000001
///
/// ## Fields
///
/// - **IC_SDA_RX_HOLD** `[23:16]` (RW)
///
///   Sets the required SDA hold time in units of ic_clk period, when DW_apb_i2c acts as a receiver.
///   Reset value: IC_DEFAULT_SDA_HOLD[23:16].
///
/// - **IC_SDA_TX_HOLD** `[15:0]` (RW)
///
///   Sets the required SDA hold time in units of ic_clk period, when DW_apb_i2c acts as a transmitter.
///   Reset value: IC_DEFAULT_SDA_HOLD[15:0].
#[repr(transparent)]
pub struct IcSdaHold {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for IcSdaHold {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl IcSdaHold {
    // ======== Field Constants ========

    /// **IC_SDA_RX_HOLD**
    ///
    /// Sets the required SDA hold time in units of ic_clk period, when DW_apb_i2c acts as a receiver.
    /// Reset value: IC_DEFAULT_SDA_HOLD[23:16].
    ///
    /// Bits `[23:16]`, mask `0x00ff0000`
    pub const IC_SDA_RX_HOLD_OFFSET: u32 = 16;
    pub const IC_SDA_RX_HOLD_MASK: u32 = 0x00ff0000;

    /// **IC_SDA_TX_HOLD**
    ///
    /// Sets the required SDA hold time in units of ic_clk period, when DW_apb_i2c acts as a transmitter.
    /// Reset value: IC_DEFAULT_SDA_HOLD[15:0].
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const IC_SDA_TX_HOLD_OFFSET: u32 = 0;
    pub const IC_SDA_TX_HOLD_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **IC_SDA_RX_HOLD** `[23:16]` (RW)
    ///
    /// Sets the required SDA hold time in units of ic_clk period, when DW_apb_i2c acts as a receiver.
    /// Reset value: IC_DEFAULT_SDA_HOLD[23:16].
    #[inline(always)]
    pub fn ic_sda_rx_hold(&self) -> u32 {
        (self.value.read_bits() & Self::IC_SDA_RX_HOLD_MASK) >> Self::IC_SDA_RX_HOLD_OFFSET
    }

    /// Write **IC_SDA_RX_HOLD** `[23:16]` (RW)
    ///
    /// Sets the required SDA hold time in units of ic_clk period, when DW_apb_i2c acts as a receiver.
    /// Reset value: IC_DEFAULT_SDA_HOLD[23:16].
    #[inline(always)]
    pub fn write_ic_sda_rx_hold(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::IC_SDA_RX_HOLD_MASK)
                | ((value << Self::IC_SDA_RX_HOLD_OFFSET) & Self::IC_SDA_RX_HOLD_MASK)
        });
    }

    /// **IC_SDA_TX_HOLD** `[15:0]` (RW)
    ///
    /// Sets the required SDA hold time in units of ic_clk period, when DW_apb_i2c acts as a transmitter.
    /// Reset value: IC_DEFAULT_SDA_HOLD[15:0].
    #[inline(always)]
    pub fn ic_sda_tx_hold(&self) -> u32 {
        (self.value.read_bits() & Self::IC_SDA_TX_HOLD_MASK) >> Self::IC_SDA_TX_HOLD_OFFSET
    }

    /// Write **IC_SDA_TX_HOLD** `[15:0]` (RW)
    ///
    /// Sets the required SDA hold time in units of ic_clk period, when DW_apb_i2c acts as a transmitter.
    /// Reset value: IC_DEFAULT_SDA_HOLD[15:0].
    #[inline(always)]
    pub fn write_ic_sda_tx_hold(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::IC_SDA_TX_HOLD_MASK)
                | ((value << Self::IC_SDA_TX_HOLD_OFFSET) & Self::IC_SDA_TX_HOLD_MASK)
        });
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

    /// Read **IC_SDA_RX_HOLD** `[23:16]`
    ///
    /// Sets the required SDA hold time in units of ic_clk period, when DW_apb_i2c acts as a receiver.
    /// Reset value: IC_DEFAULT_SDA_HOLD[23:16].
    #[inline(always)]
    pub const fn ic_sda_rx_hold(&self) -> u32 {
        (self.bits & IcSdaHold::IC_SDA_RX_HOLD_MASK) >> IcSdaHold::IC_SDA_RX_HOLD_OFFSET
    }

    /// Read **IC_SDA_TX_HOLD** `[15:0]`
    ///
    /// Sets the required SDA hold time in units of ic_clk period, when DW_apb_i2c acts as a transmitter.
    /// Reset value: IC_DEFAULT_SDA_HOLD[15:0].
    #[inline(always)]
    pub const fn ic_sda_tx_hold(&self) -> u32 {
        (self.bits & IcSdaHold::IC_SDA_TX_HOLD_MASK) >> IcSdaHold::IC_SDA_TX_HOLD_OFFSET
    }

    /// Set **IC_SDA_RX_HOLD** `[23:16]`
    ///
    /// Sets the required SDA hold time in units of ic_clk period, when DW_apb_i2c acts as a receiver.
    /// Reset value: IC_DEFAULT_SDA_HOLD[23:16].
    #[inline(always)]
    pub const fn with_ic_sda_rx_hold(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !IcSdaHold::IC_SDA_RX_HOLD_MASK)
                | ((value << IcSdaHold::IC_SDA_RX_HOLD_OFFSET) & IcSdaHold::IC_SDA_RX_HOLD_MASK),
        }
    }

    /// Set **IC_SDA_TX_HOLD** `[15:0]`
    ///
    /// Sets the required SDA hold time in units of ic_clk period, when DW_apb_i2c acts as a transmitter.
    /// Reset value: IC_DEFAULT_SDA_HOLD[15:0].
    #[inline(always)]
    pub const fn with_ic_sda_tx_hold(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !IcSdaHold::IC_SDA_TX_HOLD_MASK)
                | ((value << IcSdaHold::IC_SDA_TX_HOLD_OFFSET) & IcSdaHold::IC_SDA_TX_HOLD_MASK),
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
    pub fn read_val(&self) -> IcSdaHoldVal {
        IcSdaHoldVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: IcSdaHoldVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(IcSdaHoldVal) -> IcSdaHoldVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// IC_TX_ABRT_SOURCE Register
///
/// I2C Transmit Abort Source Register
/// This register has 32 bits that indicate the source of the TX_ABRT bit. Except for Bit 9, this register is cleared whenever the IC_CLR_TX_ABRT register or the IC_CLR_INTR register is read. To clear Bit 9, the source of the ABRT_SBYTE_NORSTRT must be fixed first; RESTART must be enabled (IC_CON[5]=1), the SPECIAL bit must be cleared (IC_TAR[11]), or the GC_OR_START bit must be cleared (IC_TAR[10]).
/// Once the source of the ABRT_SBYTE_NORSTRT is fixed, then this bit can be cleared in the same manner as other bits in this register. If the source of the ABRT_SBYTE_NORSTRT is not fixed before attempting to clear this bit, Bit 9 clears for one cycle and is then re-asserted.
///
/// Address offset: 0x0080
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **TX_FLUSH_CNT** `[31:23]` (RO)
///
///   This field indicates the number of Tx FIFO Data Commands which are flushed due to TX_ABRT interrupt. It is cleared whenever I2C is disabled.
///   Reset value: 0x0
///   Role of DW_apb_i2c: Master-Transmitter or Slave-Transmitter
///
/// - **ABRT_USER_ABRT** `[16]` (RO)
///
///   This is a master-mode-only bit. Master has detected the transfer abort (IC_ENABLE[1])
///   Reset value: 0x0
///   Role of DW_apb_i2c: Master-Transmitter
///
/// - **ABRT_SLVRD_INTX** `[15]` (RO)
///
///   1: When the processor side responds to a slave mode request for data to be transmitted to a remote master and user writes a 1 in CMD (bit 8) of IC_DATA_CMD register.
///   Reset value: 0x0
///   Role of DW_apb_i2c: Slave-Transmitter
///
/// - **ABRT_SLV_ARBLOST** `[14]` (RO)
///
///   This field indicates that a Slave has lost the bus while transmitting data to a remote master. IC_TX_ABRT_SOURCE[12] is set at the same time. Note: Even though the slave never 'owns' the bus, something could go wrong on the bus. This is a fail safe check. For instance, during a data transmission at the low-to-high transition of SCL, if what is on the data bus is not what is supposed to be transmitted, then DW_apb_i2c no longer own the bus.
///   Reset value: 0x0
///   Role of DW_apb_i2c: Slave-Transmitter
///
/// - **ABRT_SLVFLUSH_TXFIFO** `[13]` (RO)
///
///   This field specifies that the Slave has received a read command and some data exists in the TX FIFO, so the slave issues a TX_ABRT interrupt to flush old data in TX FIFO.
///   Reset value: 0x0
///   Role of DW_apb_i2c: Slave-Transmitter
///
/// - **ARB_LOST** `[12]` (RO)
///
///   This field specifies that the Master has lost arbitration, or if IC_TX_ABRT_SOURCE[14] is also set, then the slave transmitter has lost arbitration.
///   Reset value: 0x0
///   Role of DW_apb_i2c: Master-Transmitter or Slave-Transmitter
///
/// - **ABRT_MASTER_DIS** `[11]` (RO)
///
///   This field indicates that the User tries to initiate a Master operation with the Master mode disabled.
///   Reset value: 0x0
///   Role of DW_apb_i2c: Master-Transmitter or Master-Receiver
///
/// - **ABRT_10B_RD_NORSTRT** `[10]` (RO)
///
///   This field indicates that the restart is disabled (IC_RESTART_EN bit (IC_CON[5]) =0) and the master sends a read command in 10-bit addressing mode.
///   Reset value: 0x0
///   Role of DW_apb_i2c: Master-Receiver
///
/// - **ABRT_SBYTE_NORSTRT** `[9]` (RO)
///
///   To clear Bit 9, the source of the ABRT_SBYTE_NORSTRT must be fixed first; restart must be enabled (IC_CON[5]=1), the SPECIAL bit must be cleared (IC_TAR[11]), or the GC_OR_START bit must be cleared (IC_TAR[10]). Once the source of the ABRT_SBYTE_NORSTRT is fixed, then this bit can be cleared in the same manner as other bits in this register. If the source of the ABRT_SBYTE_NORSTRT is not fixed before attempting to clear this bit, bit 9 clears for one cycle and then gets reasserted. When this field is set to 1, the restart is disabled (IC_RESTART_EN bit (IC_CON[5]) =0) and the user is trying to send a START Byte.
///   Reset value: 0x0
///   Role of DW_apb_i2c: Master
///
/// - **ABRT_HS_NORSTRT** `[8]` (RO)
///
///   This field indicates that the restart is disabled (IC_RESTART_EN bit (IC_CON[5]) =0) and the user is trying to use the master to transfer data in High Speed mode.
///   Reset value: 0x0
///   Role of DW_apb_i2c: Master-Transmitter or Master-Receiver
///
/// - **ABRT_SBYTE_ACKDET** `[7]` (RO)
///
///   This field indicates that the Master has sent a START Byte and the START Byte was acknowledged (wrong behavior).
///   Reset value: 0x0
///   Role of DW_apb_i2c: Master
///
/// - **ABRT_HS_ACKDET** `[6]` (RO)
///
///   This field indicates that the Master is in High Speed mode and the High Speed Master code was acknowledged (wrong behavior).
///   Reset value: 0x0
///   Role of DW_apb_i2c: Master
///
/// - **ABRT_GCALL_READ** `[5]` (RO)
///
///   This field indicates that DW_apb_i2c in the master mode has sent a General Call but the user programmed the byte following the General Call to be a read from the bus (IC_DATA_CMD[9] is set to 1).
///   Reset value: 0x0
///   Role of DW_apb_i2c: Master-Transmitter
///
/// - **ABRT_GCALL_NOACK** `[4]` (RO)
///
///   This field indicates that DW_apb_i2c in master mode has sent a General Call and no slave on the bus acknowledged the General Call.
///   Reset value: 0x0
///   Role of DW_apb_i2c: Master-Transmitter
///
/// - **ABRT_TXDATA_NOACK** `[3]` (RO)
///
///   This field indicates the master-mode only bit. When the master receives an acknowledgement for the address, but when it sends data byte(s) following the address, it did not receive an acknowledge from the remote slave(s).
///   Reset value: 0x0
///   Role of DW_apb_i2c: Master-Transmitter
///
/// - **ABRT_10ADDR2_NOACK** `[2]` (RO)
///
///   This field indicates that the Master is in 10-bit address mode and that the second address byte of the 10-bit address was not acknowledged by any slave.
///   Reset value: 0x0
///   Role of DW_apb_i2c: Master-Transmitter or Master-Receiver
///
/// - **ABRT_10ADDR1_NOACK** `[1]` (RO)
///
///   This field indicates that the Master is in 10-bit address mode and the first 10-bit address byte was not acknowledged by any slave.
///   Reset value: 0x0
///   Role of DW_apb_i2c: Master-Transmitter or Master-Receiver
///
/// - **ABRT_7B_ADDR_NOACK** `[0]` (RO)
///
///   This field indicates that the Master is in 7-bit addressing mode and the address sent was not acknowledged by any slave.
///   Reset value: 0x0
///   Role of DW_apb_i2c: Master-Transmitter or Master-Receiver
#[repr(transparent)]
pub struct IcTxAbrtSource {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for IcTxAbrtSource {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl IcTxAbrtSource {
    // ======== Field Constants ========

    /// **TX_FLUSH_CNT**
    ///
    /// This field indicates the number of Tx FIFO Data Commands which are flushed due to TX_ABRT interrupt. It is cleared whenever I2C is disabled.
    /// Reset value: 0x0
    /// Role of DW_apb_i2c: Master-Transmitter or Slave-Transmitter
    ///
    /// Bits `[31:23]`, mask `0xff800000`
    pub const TX_FLUSH_CNT_OFFSET: u32 = 23;
    pub const TX_FLUSH_CNT_MASK: u32 = 0xff800000;

    /// **ABRT_USER_ABRT**
    ///
    /// This is a master-mode-only bit. Master has detected the transfer abort (IC_ENABLE[1])
    /// Reset value: 0x0
    /// Role of DW_apb_i2c: Master-Transmitter
    ///
    /// Bits `[16:16]`, mask `0x00010000`
    pub const ABRT_USER_ABRT_OFFSET: u32 = 16;
    pub const ABRT_USER_ABRT_MASK: u32 = 0x00010000;
    pub const ABRT_USER_ABRT: u32 = 1 << 16;
    /// ABRT_USER_ABRT_VOID = 0
    /// Transfer abort detected by master- scenario not present
    pub const ABRT_USER_ABRT_ABRT_USER_ABRT_VOID: u32 = 0 << 16;
    pub const ABRT_USER_ABRT_ABRT_USER_ABRT_VOID_VAL: u32 = 0;
    /// ABRT_USER_ABRT_GENERATED = 1
    /// Transfer abort detected by master
    pub const ABRT_USER_ABRT_ABRT_USER_ABRT_GENERATED: u32 = 1 << 16;
    pub const ABRT_USER_ABRT_ABRT_USER_ABRT_GENERATED_VAL: u32 = 1;

    /// **ABRT_SLVRD_INTX**
    ///
    /// 1: When the processor side responds to a slave mode request for data to be transmitted to a remote master and user writes a 1 in CMD (bit 8) of IC_DATA_CMD register.
    /// Reset value: 0x0
    /// Role of DW_apb_i2c: Slave-Transmitter
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const ABRT_SLVRD_INTX_OFFSET: u32 = 15;
    pub const ABRT_SLVRD_INTX_MASK: u32 = 0x00008000;
    pub const ABRT_SLVRD_INTX: u32 = 1 << 15;
    /// ABRT_SLVRD_INTX_VOID = 0
    /// Slave trying to transmit to remote master in read mode- scenario not present
    pub const ABRT_SLVRD_INTX_ABRT_SLVRD_INTX_VOID: u32 = 0 << 15;
    pub const ABRT_SLVRD_INTX_ABRT_SLVRD_INTX_VOID_VAL: u32 = 0;
    /// ABRT_SLVRD_INTX_GENERATED = 1
    /// Slave trying to transmit to remote master in read mode
    pub const ABRT_SLVRD_INTX_ABRT_SLVRD_INTX_GENERATED: u32 = 1 << 15;
    pub const ABRT_SLVRD_INTX_ABRT_SLVRD_INTX_GENERATED_VAL: u32 = 1;

    /// **ABRT_SLV_ARBLOST**
    ///
    /// This field indicates that a Slave has lost the bus while transmitting data to a remote master. IC_TX_ABRT_SOURCE[12] is set at the same time. Note: Even though the slave never 'owns' the bus, something could go wrong on the bus. This is a fail safe check. For instance, during a data transmission at the low-to-high transition of SCL, if what is on the data bus is not what is supposed to be transmitted, then DW_apb_i2c no longer own the bus.
    /// Reset value: 0x0
    /// Role of DW_apb_i2c: Slave-Transmitter
    ///
    /// Bits `[14:14]`, mask `0x00004000`
    pub const ABRT_SLV_ARBLOST_OFFSET: u32 = 14;
    pub const ABRT_SLV_ARBLOST_MASK: u32 = 0x00004000;
    pub const ABRT_SLV_ARBLOST: u32 = 1 << 14;
    /// ABRT_SLV_ARBLOST_VOID = 0
    /// Slave lost arbitration to remote master- scenario not present
    pub const ABRT_SLV_ARBLOST_ABRT_SLV_ARBLOST_VOID: u32 = 0 << 14;
    pub const ABRT_SLV_ARBLOST_ABRT_SLV_ARBLOST_VOID_VAL: u32 = 0;
    /// ABRT_SLV_ARBLOST_GENERATED = 1
    /// Slave lost arbitration to remote master
    pub const ABRT_SLV_ARBLOST_ABRT_SLV_ARBLOST_GENERATED: u32 = 1 << 14;
    pub const ABRT_SLV_ARBLOST_ABRT_SLV_ARBLOST_GENERATED_VAL: u32 = 1;

    /// **ABRT_SLVFLUSH_TXFIFO**
    ///
    /// This field specifies that the Slave has received a read command and some data exists in the TX FIFO, so the slave issues a TX_ABRT interrupt to flush old data in TX FIFO.
    /// Reset value: 0x0
    /// Role of DW_apb_i2c: Slave-Transmitter
    ///
    /// Bits `[13:13]`, mask `0x00002000`
    pub const ABRT_SLVFLUSH_TXFIFO_OFFSET: u32 = 13;
    pub const ABRT_SLVFLUSH_TXFIFO_MASK: u32 = 0x00002000;
    pub const ABRT_SLVFLUSH_TXFIFO: u32 = 1 << 13;
    /// ABRT_SLVFLUSH_TXFIFO_VOID = 0
    /// Slave flushes existing data in TX-FIFO upon getting read command- scenario not present
    pub const ABRT_SLVFLUSH_TXFIFO_ABRT_SLVFLUSH_TXFIFO_VOID: u32 = 0 << 13;
    pub const ABRT_SLVFLUSH_TXFIFO_ABRT_SLVFLUSH_TXFIFO_VOID_VAL: u32 = 0;
    /// ABRT_SLVFLUSH_TXFIFO_GENERATED = 1
    /// Slave flushes existing data in TX-FIFO upon getting read command
    pub const ABRT_SLVFLUSH_TXFIFO_ABRT_SLVFLUSH_TXFIFO_GENERATED: u32 = 1 << 13;
    pub const ABRT_SLVFLUSH_TXFIFO_ABRT_SLVFLUSH_TXFIFO_GENERATED_VAL: u32 = 1;

    /// **ARB_LOST**
    ///
    /// This field specifies that the Master has lost arbitration, or if IC_TX_ABRT_SOURCE[14] is also set, then the slave transmitter has lost arbitration.
    /// Reset value: 0x0
    /// Role of DW_apb_i2c: Master-Transmitter or Slave-Transmitter
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const ARB_LOST_OFFSET: u32 = 12;
    pub const ARB_LOST_MASK: u32 = 0x00001000;
    pub const ARB_LOST: u32 = 1 << 12;
    /// ABRT_LOST_VOID = 0
    /// Master or Slave-Transmitter lost arbitration- scenario not present
    pub const ARB_LOST_ABRT_LOST_VOID: u32 = 0 << 12;
    pub const ARB_LOST_ABRT_LOST_VOID_VAL: u32 = 0;
    /// ABRT_LOST_GENERATED = 1
    /// Master or Slave-Transmitter lost arbitration
    pub const ARB_LOST_ABRT_LOST_GENERATED: u32 = 1 << 12;
    pub const ARB_LOST_ABRT_LOST_GENERATED_VAL: u32 = 1;

    /// **ABRT_MASTER_DIS**
    ///
    /// This field indicates that the User tries to initiate a Master operation with the Master mode disabled.
    /// Reset value: 0x0
    /// Role of DW_apb_i2c: Master-Transmitter or Master-Receiver
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const ABRT_MASTER_DIS_OFFSET: u32 = 11;
    pub const ABRT_MASTER_DIS_MASK: u32 = 0x00000800;
    pub const ABRT_MASTER_DIS: u32 = 1 << 11;
    /// ABRT_MASTER_DIS_VOID = 0
    /// User initiating master operation when MASTER disabled- scenario not present
    pub const ABRT_MASTER_DIS_ABRT_MASTER_DIS_VOID: u32 = 0 << 11;
    pub const ABRT_MASTER_DIS_ABRT_MASTER_DIS_VOID_VAL: u32 = 0;
    /// ABRT_MASTER_DIS_GENERATED = 1
    /// User initiating master operation when MASTER disabled
    pub const ABRT_MASTER_DIS_ABRT_MASTER_DIS_GENERATED: u32 = 1 << 11;
    pub const ABRT_MASTER_DIS_ABRT_MASTER_DIS_GENERATED_VAL: u32 = 1;

    /// **ABRT_10B_RD_NORSTRT**
    ///
    /// This field indicates that the restart is disabled (IC_RESTART_EN bit (IC_CON[5]) =0) and the master sends a read command in 10-bit addressing mode.
    /// Reset value: 0x0
    /// Role of DW_apb_i2c: Master-Receiver
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const ABRT_10B_RD_NORSTRT_OFFSET: u32 = 10;
    pub const ABRT_10B_RD_NORSTRT_MASK: u32 = 0x00000400;
    pub const ABRT_10B_RD_NORSTRT: u32 = 1 << 10;
    /// ABRT_10B_RD_VOID = 0
    /// Master not trying to read in 10Bit addressing mode when RESTART disabled
    pub const ABRT_10B_RD_NORSTRT_ABRT_10B_RD_VOID: u32 = 0 << 10;
    pub const ABRT_10B_RD_NORSTRT_ABRT_10B_RD_VOID_VAL: u32 = 0;
    /// ABRT_10B_RD_GENERATED = 1
    /// Master trying to read in 10Bit addressing mode when RESTART disabled
    pub const ABRT_10B_RD_NORSTRT_ABRT_10B_RD_GENERATED: u32 = 1 << 10;
    pub const ABRT_10B_RD_NORSTRT_ABRT_10B_RD_GENERATED_VAL: u32 = 1;

    /// **ABRT_SBYTE_NORSTRT**
    ///
    /// To clear Bit 9, the source of the ABRT_SBYTE_NORSTRT must be fixed first; restart must be enabled (IC_CON[5]=1), the SPECIAL bit must be cleared (IC_TAR[11]), or the GC_OR_START bit must be cleared (IC_TAR[10]). Once the source of the ABRT_SBYTE_NORSTRT is fixed, then this bit can be cleared in the same manner as other bits in this register. If the source of the ABRT_SBYTE_NORSTRT is not fixed before attempting to clear this bit, bit 9 clears for one cycle and then gets reasserted. When this field is set to 1, the restart is disabled (IC_RESTART_EN bit (IC_CON[5]) =0) and the user is trying to send a START Byte.
    /// Reset value: 0x0
    /// Role of DW_apb_i2c: Master
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const ABRT_SBYTE_NORSTRT_OFFSET: u32 = 9;
    pub const ABRT_SBYTE_NORSTRT_MASK: u32 = 0x00000200;
    pub const ABRT_SBYTE_NORSTRT: u32 = 1 << 9;
    /// ABRT_SBYTE_NORSTRT_VOID = 0
    /// User trying to send START byte when RESTART disabled- scenario not present
    pub const ABRT_SBYTE_NORSTRT_ABRT_SBYTE_NORSTRT_VOID: u32 = 0 << 9;
    pub const ABRT_SBYTE_NORSTRT_ABRT_SBYTE_NORSTRT_VOID_VAL: u32 = 0;
    /// ABRT_SBYTE_NORSTRT_GENERATED = 1
    /// User trying to send START byte when RESTART disabled
    pub const ABRT_SBYTE_NORSTRT_ABRT_SBYTE_NORSTRT_GENERATED: u32 = 1 << 9;
    pub const ABRT_SBYTE_NORSTRT_ABRT_SBYTE_NORSTRT_GENERATED_VAL: u32 = 1;

    /// **ABRT_HS_NORSTRT**
    ///
    /// This field indicates that the restart is disabled (IC_RESTART_EN bit (IC_CON[5]) =0) and the user is trying to use the master to transfer data in High Speed mode.
    /// Reset value: 0x0
    /// Role of DW_apb_i2c: Master-Transmitter or Master-Receiver
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const ABRT_HS_NORSTRT_OFFSET: u32 = 8;
    pub const ABRT_HS_NORSTRT_MASK: u32 = 0x00000100;
    pub const ABRT_HS_NORSTRT: u32 = 1 << 8;
    /// ABRT_HS_NORSTRT_VOID = 0
    /// User trying to switch Master to HS mode when RESTART disabled- scenario not present
    pub const ABRT_HS_NORSTRT_ABRT_HS_NORSTRT_VOID: u32 = 0 << 8;
    pub const ABRT_HS_NORSTRT_ABRT_HS_NORSTRT_VOID_VAL: u32 = 0;
    /// ABRT_HS_NORSTRT_GENERATED = 1
    /// User trying to switch Master to HS mode when RESTART disabled
    pub const ABRT_HS_NORSTRT_ABRT_HS_NORSTRT_GENERATED: u32 = 1 << 8;
    pub const ABRT_HS_NORSTRT_ABRT_HS_NORSTRT_GENERATED_VAL: u32 = 1;

    /// **ABRT_SBYTE_ACKDET**
    ///
    /// This field indicates that the Master has sent a START Byte and the START Byte was acknowledged (wrong behavior).
    /// Reset value: 0x0
    /// Role of DW_apb_i2c: Master
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const ABRT_SBYTE_ACKDET_OFFSET: u32 = 7;
    pub const ABRT_SBYTE_ACKDET_MASK: u32 = 0x00000080;
    pub const ABRT_SBYTE_ACKDET: u32 = 1 << 7;
    /// ABRT_SBYTE_ACKDET_VOID = 0
    /// ACK detected for START byte- scenario not present
    pub const ABRT_SBYTE_ACKDET_ABRT_SBYTE_ACKDET_VOID: u32 = 0 << 7;
    pub const ABRT_SBYTE_ACKDET_ABRT_SBYTE_ACKDET_VOID_VAL: u32 = 0;
    /// ABRT_SBYTE_ACKDET_GENERATED = 1
    /// ACK detected for START byte
    pub const ABRT_SBYTE_ACKDET_ABRT_SBYTE_ACKDET_GENERATED: u32 = 1 << 7;
    pub const ABRT_SBYTE_ACKDET_ABRT_SBYTE_ACKDET_GENERATED_VAL: u32 = 1;

    /// **ABRT_HS_ACKDET**
    ///
    /// This field indicates that the Master is in High Speed mode and the High Speed Master code was acknowledged (wrong behavior).
    /// Reset value: 0x0
    /// Role of DW_apb_i2c: Master
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const ABRT_HS_ACKDET_OFFSET: u32 = 6;
    pub const ABRT_HS_ACKDET_MASK: u32 = 0x00000040;
    pub const ABRT_HS_ACKDET: u32 = 1 << 6;
    /// ABRT_HS_ACK_VOID = 0
    /// HS Master code ACKed in HS Mode- scenario not present
    pub const ABRT_HS_ACKDET_ABRT_HS_ACK_VOID: u32 = 0 << 6;
    pub const ABRT_HS_ACKDET_ABRT_HS_ACK_VOID_VAL: u32 = 0;
    /// ABRT_HS_ACK_GENERATED = 1
    /// HS Master code ACKed in HS Mode
    pub const ABRT_HS_ACKDET_ABRT_HS_ACK_GENERATED: u32 = 1 << 6;
    pub const ABRT_HS_ACKDET_ABRT_HS_ACK_GENERATED_VAL: u32 = 1;

    /// **ABRT_GCALL_READ**
    ///
    /// This field indicates that DW_apb_i2c in the master mode has sent a General Call but the user programmed the byte following the General Call to be a read from the bus (IC_DATA_CMD[9] is set to 1).
    /// Reset value: 0x0
    /// Role of DW_apb_i2c: Master-Transmitter
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const ABRT_GCALL_READ_OFFSET: u32 = 5;
    pub const ABRT_GCALL_READ_MASK: u32 = 0x00000020;
    pub const ABRT_GCALL_READ: u32 = 1 << 5;
    /// ABRT_GCALL_READ_VOID = 0
    /// GCALL is followed by read from bus-scenario not present
    pub const ABRT_GCALL_READ_ABRT_GCALL_READ_VOID: u32 = 0 << 5;
    pub const ABRT_GCALL_READ_ABRT_GCALL_READ_VOID_VAL: u32 = 0;
    /// ABRT_GCALL_READ_GENERATED = 1
    /// GCALL is followed by read from bus
    pub const ABRT_GCALL_READ_ABRT_GCALL_READ_GENERATED: u32 = 1 << 5;
    pub const ABRT_GCALL_READ_ABRT_GCALL_READ_GENERATED_VAL: u32 = 1;

    /// **ABRT_GCALL_NOACK**
    ///
    /// This field indicates that DW_apb_i2c in master mode has sent a General Call and no slave on the bus acknowledged the General Call.
    /// Reset value: 0x0
    /// Role of DW_apb_i2c: Master-Transmitter
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const ABRT_GCALL_NOACK_OFFSET: u32 = 4;
    pub const ABRT_GCALL_NOACK_MASK: u32 = 0x00000010;
    pub const ABRT_GCALL_NOACK: u32 = 1 << 4;
    /// ABRT_GCALL_NOACK_VOID = 0
    /// GCALL not ACKed by any slave-scenario not present
    pub const ABRT_GCALL_NOACK_ABRT_GCALL_NOACK_VOID: u32 = 0 << 4;
    pub const ABRT_GCALL_NOACK_ABRT_GCALL_NOACK_VOID_VAL: u32 = 0;
    /// ABRT_GCALL_NOACK_GENERATED = 1
    /// GCALL not ACKed by any slave
    pub const ABRT_GCALL_NOACK_ABRT_GCALL_NOACK_GENERATED: u32 = 1 << 4;
    pub const ABRT_GCALL_NOACK_ABRT_GCALL_NOACK_GENERATED_VAL: u32 = 1;

    /// **ABRT_TXDATA_NOACK**
    ///
    /// This field indicates the master-mode only bit. When the master receives an acknowledgement for the address, but when it sends data byte(s) following the address, it did not receive an acknowledge from the remote slave(s).
    /// Reset value: 0x0
    /// Role of DW_apb_i2c: Master-Transmitter
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const ABRT_TXDATA_NOACK_OFFSET: u32 = 3;
    pub const ABRT_TXDATA_NOACK_MASK: u32 = 0x00000008;
    pub const ABRT_TXDATA_NOACK: u32 = 1 << 3;
    /// ABRT_TXDATA_NOACK_VOID = 0
    /// Transmitted data non-ACKed by addressed slave-scenario not present
    pub const ABRT_TXDATA_NOACK_ABRT_TXDATA_NOACK_VOID: u32 = 0 << 3;
    pub const ABRT_TXDATA_NOACK_ABRT_TXDATA_NOACK_VOID_VAL: u32 = 0;
    /// ABRT_TXDATA_NOACK_GENERATED = 1
    /// Transmitted data not ACKed by addressed slave
    pub const ABRT_TXDATA_NOACK_ABRT_TXDATA_NOACK_GENERATED: u32 = 1 << 3;
    pub const ABRT_TXDATA_NOACK_ABRT_TXDATA_NOACK_GENERATED_VAL: u32 = 1;

    /// **ABRT_10ADDR2_NOACK**
    ///
    /// This field indicates that the Master is in 10-bit address mode and that the second address byte of the 10-bit address was not acknowledged by any slave.
    /// Reset value: 0x0
    /// Role of DW_apb_i2c: Master-Transmitter or Master-Receiver
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const ABRT_10ADDR2_NOACK_OFFSET: u32 = 2;
    pub const ABRT_10ADDR2_NOACK_MASK: u32 = 0x00000004;
    pub const ABRT_10ADDR2_NOACK: u32 = 1 << 2;
    /// INACTIVE = 0
    /// This abort is not generated
    pub const ABRT_10ADDR2_NOACK_INACTIVE: u32 = 0 << 2;
    pub const ABRT_10ADDR2_NOACK_INACTIVE_VAL: u32 = 0;
    /// ACTIVE = 1
    /// Byte 2 of 10Bit Address not ACKed by any slave
    pub const ABRT_10ADDR2_NOACK_ACTIVE: u32 = 1 << 2;
    pub const ABRT_10ADDR2_NOACK_ACTIVE_VAL: u32 = 1;

    /// **ABRT_10ADDR1_NOACK**
    ///
    /// This field indicates that the Master is in 10-bit address mode and the first 10-bit address byte was not acknowledged by any slave.
    /// Reset value: 0x0
    /// Role of DW_apb_i2c: Master-Transmitter or Master-Receiver
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const ABRT_10ADDR1_NOACK_OFFSET: u32 = 1;
    pub const ABRT_10ADDR1_NOACK_MASK: u32 = 0x00000002;
    pub const ABRT_10ADDR1_NOACK: u32 = 1 << 1;
    /// INACTIVE = 0
    /// This abort is not generated
    pub const ABRT_10ADDR1_NOACK_INACTIVE: u32 = 0 << 1;
    pub const ABRT_10ADDR1_NOACK_INACTIVE_VAL: u32 = 0;
    /// ACTIVE = 1
    /// Byte 1 of 10Bit Address not ACKed by any slave
    pub const ABRT_10ADDR1_NOACK_ACTIVE: u32 = 1 << 1;
    pub const ABRT_10ADDR1_NOACK_ACTIVE_VAL: u32 = 1;

    /// **ABRT_7B_ADDR_NOACK**
    ///
    /// This field indicates that the Master is in 7-bit addressing mode and the address sent was not acknowledged by any slave.
    /// Reset value: 0x0
    /// Role of DW_apb_i2c: Master-Transmitter or Master-Receiver
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const ABRT_7B_ADDR_NOACK_OFFSET: u32 = 0;
    pub const ABRT_7B_ADDR_NOACK_MASK: u32 = 0x00000001;
    pub const ABRT_7B_ADDR_NOACK: u32 = 1 << 0;
    /// INACTIVE = 0
    /// This abort is not generated
    pub const ABRT_7B_ADDR_NOACK_INACTIVE: u32 = 0 << 0;
    pub const ABRT_7B_ADDR_NOACK_INACTIVE_VAL: u32 = 0;
    /// ACTIVE = 1
    /// This abort is generated because of NOACK for 7-bit address
    pub const ABRT_7B_ADDR_NOACK_ACTIVE: u32 = 1 << 0;
    pub const ABRT_7B_ADDR_NOACK_ACTIVE_VAL: u32 = 1;

    // ======== Field Accessors ========

    /// **TX_FLUSH_CNT** `[31:23]` (RO)
    ///
    /// This field indicates the number of Tx FIFO Data Commands which are flushed due to TX_ABRT interrupt. It is cleared whenever I2C is disabled.
    /// Reset value: 0x0
    /// Role of DW_apb_i2c: Master-Transmitter or Slave-Transmitter
    #[inline(always)]
    pub fn tx_flush_cnt(&self) -> u32 {
        (self.value.read_bits() & Self::TX_FLUSH_CNT_MASK) >> Self::TX_FLUSH_CNT_OFFSET
    }

    /// **ABRT_USER_ABRT** `[16]` (RO)
    ///
    /// This is a master-mode-only bit. Master has detected the transfer abort (IC_ENABLE[1])
    /// Reset value: 0x0
    /// Role of DW_apb_i2c: Master-Transmitter
    #[inline(always)]
    pub fn abrt_user_abrt(&self) -> bool {
        (self.value.read_bits() & Self::ABRT_USER_ABRT_MASK) != 0
    }

    /// **ABRT_SLVRD_INTX** `[15]` (RO)
    ///
    /// 1: When the processor side responds to a slave mode request for data to be transmitted to a remote master and user writes a 1 in CMD (bit 8) of IC_DATA_CMD register.
    /// Reset value: 0x0
    /// Role of DW_apb_i2c: Slave-Transmitter
    #[inline(always)]
    pub fn abrt_slvrd_intx(&self) -> bool {
        (self.value.read_bits() & Self::ABRT_SLVRD_INTX_MASK) != 0
    }

    /// **ABRT_SLV_ARBLOST** `[14]` (RO)
    ///
    /// This field indicates that a Slave has lost the bus while transmitting data to a remote master. IC_TX_ABRT_SOURCE[12] is set at the same time. Note: Even though the slave never 'owns' the bus, something could go wrong on the bus. This is a fail safe check. For instance, during a data transmission at the low-to-high transition of SCL, if what is on the data bus is not what is supposed to be transmitted, then DW_apb_i2c no longer own the bus.
    /// Reset value: 0x0
    /// Role of DW_apb_i2c: Slave-Transmitter
    #[inline(always)]
    pub fn abrt_slv_arblost(&self) -> bool {
        (self.value.read_bits() & Self::ABRT_SLV_ARBLOST_MASK) != 0
    }

    /// **ABRT_SLVFLUSH_TXFIFO** `[13]` (RO)
    ///
    /// This field specifies that the Slave has received a read command and some data exists in the TX FIFO, so the slave issues a TX_ABRT interrupt to flush old data in TX FIFO.
    /// Reset value: 0x0
    /// Role of DW_apb_i2c: Slave-Transmitter
    #[inline(always)]
    pub fn abrt_slvflush_txfifo(&self) -> bool {
        (self.value.read_bits() & Self::ABRT_SLVFLUSH_TXFIFO_MASK) != 0
    }

    /// **ARB_LOST** `[12]` (RO)
    ///
    /// This field specifies that the Master has lost arbitration, or if IC_TX_ABRT_SOURCE[14] is also set, then the slave transmitter has lost arbitration.
    /// Reset value: 0x0
    /// Role of DW_apb_i2c: Master-Transmitter or Slave-Transmitter
    #[inline(always)]
    pub fn arb_lost(&self) -> bool {
        (self.value.read_bits() & Self::ARB_LOST_MASK) != 0
    }

    /// **ABRT_MASTER_DIS** `[11]` (RO)
    ///
    /// This field indicates that the User tries to initiate a Master operation with the Master mode disabled.
    /// Reset value: 0x0
    /// Role of DW_apb_i2c: Master-Transmitter or Master-Receiver
    #[inline(always)]
    pub fn abrt_master_dis(&self) -> bool {
        (self.value.read_bits() & Self::ABRT_MASTER_DIS_MASK) != 0
    }

    /// **ABRT_10B_RD_NORSTRT** `[10]` (RO)
    ///
    /// This field indicates that the restart is disabled (IC_RESTART_EN bit (IC_CON[5]) =0) and the master sends a read command in 10-bit addressing mode.
    /// Reset value: 0x0
    /// Role of DW_apb_i2c: Master-Receiver
    #[inline(always)]
    pub fn abrt_10b_rd_norstrt(&self) -> bool {
        (self.value.read_bits() & Self::ABRT_10B_RD_NORSTRT_MASK) != 0
    }

    /// **ABRT_SBYTE_NORSTRT** `[9]` (RO)
    ///
    /// To clear Bit 9, the source of the ABRT_SBYTE_NORSTRT must be fixed first; restart must be enabled (IC_CON[5]=1), the SPECIAL bit must be cleared (IC_TAR[11]), or the GC_OR_START bit must be cleared (IC_TAR[10]). Once the source of the ABRT_SBYTE_NORSTRT is fixed, then this bit can be cleared in the same manner as other bits in this register. If the source of the ABRT_SBYTE_NORSTRT is not fixed before attempting to clear this bit, bit 9 clears for one cycle and then gets reasserted. When this field is set to 1, the restart is disabled (IC_RESTART_EN bit (IC_CON[5]) =0) and the user is trying to send a START Byte.
    /// Reset value: 0x0
    /// Role of DW_apb_i2c: Master
    #[inline(always)]
    pub fn abrt_sbyte_norstrt(&self) -> bool {
        (self.value.read_bits() & Self::ABRT_SBYTE_NORSTRT_MASK) != 0
    }

    /// **ABRT_HS_NORSTRT** `[8]` (RO)
    ///
    /// This field indicates that the restart is disabled (IC_RESTART_EN bit (IC_CON[5]) =0) and the user is trying to use the master to transfer data in High Speed mode.
    /// Reset value: 0x0
    /// Role of DW_apb_i2c: Master-Transmitter or Master-Receiver
    #[inline(always)]
    pub fn abrt_hs_norstrt(&self) -> bool {
        (self.value.read_bits() & Self::ABRT_HS_NORSTRT_MASK) != 0
    }

    /// **ABRT_SBYTE_ACKDET** `[7]` (RO)
    ///
    /// This field indicates that the Master has sent a START Byte and the START Byte was acknowledged (wrong behavior).
    /// Reset value: 0x0
    /// Role of DW_apb_i2c: Master
    #[inline(always)]
    pub fn abrt_sbyte_ackdet(&self) -> bool {
        (self.value.read_bits() & Self::ABRT_SBYTE_ACKDET_MASK) != 0
    }

    /// **ABRT_HS_ACKDET** `[6]` (RO)
    ///
    /// This field indicates that the Master is in High Speed mode and the High Speed Master code was acknowledged (wrong behavior).
    /// Reset value: 0x0
    /// Role of DW_apb_i2c: Master
    #[inline(always)]
    pub fn abrt_hs_ackdet(&self) -> bool {
        (self.value.read_bits() & Self::ABRT_HS_ACKDET_MASK) != 0
    }

    /// **ABRT_GCALL_READ** `[5]` (RO)
    ///
    /// This field indicates that DW_apb_i2c in the master mode has sent a General Call but the user programmed the byte following the General Call to be a read from the bus (IC_DATA_CMD[9] is set to 1).
    /// Reset value: 0x0
    /// Role of DW_apb_i2c: Master-Transmitter
    #[inline(always)]
    pub fn abrt_gcall_read(&self) -> bool {
        (self.value.read_bits() & Self::ABRT_GCALL_READ_MASK) != 0
    }

    /// **ABRT_GCALL_NOACK** `[4]` (RO)
    ///
    /// This field indicates that DW_apb_i2c in master mode has sent a General Call and no slave on the bus acknowledged the General Call.
    /// Reset value: 0x0
    /// Role of DW_apb_i2c: Master-Transmitter
    #[inline(always)]
    pub fn abrt_gcall_noack(&self) -> bool {
        (self.value.read_bits() & Self::ABRT_GCALL_NOACK_MASK) != 0
    }

    /// **ABRT_TXDATA_NOACK** `[3]` (RO)
    ///
    /// This field indicates the master-mode only bit. When the master receives an acknowledgement for the address, but when it sends data byte(s) following the address, it did not receive an acknowledge from the remote slave(s).
    /// Reset value: 0x0
    /// Role of DW_apb_i2c: Master-Transmitter
    #[inline(always)]
    pub fn abrt_txdata_noack(&self) -> bool {
        (self.value.read_bits() & Self::ABRT_TXDATA_NOACK_MASK) != 0
    }

    /// **ABRT_10ADDR2_NOACK** `[2]` (RO)
    ///
    /// This field indicates that the Master is in 10-bit address mode and that the second address byte of the 10-bit address was not acknowledged by any slave.
    /// Reset value: 0x0
    /// Role of DW_apb_i2c: Master-Transmitter or Master-Receiver
    #[inline(always)]
    pub fn abrt_10addr2_noack(&self) -> bool {
        (self.value.read_bits() & Self::ABRT_10ADDR2_NOACK_MASK) != 0
    }

    /// **ABRT_10ADDR1_NOACK** `[1]` (RO)
    ///
    /// This field indicates that the Master is in 10-bit address mode and the first 10-bit address byte was not acknowledged by any slave.
    /// Reset value: 0x0
    /// Role of DW_apb_i2c: Master-Transmitter or Master-Receiver
    #[inline(always)]
    pub fn abrt_10addr1_noack(&self) -> bool {
        (self.value.read_bits() & Self::ABRT_10ADDR1_NOACK_MASK) != 0
    }

    /// **ABRT_7B_ADDR_NOACK** `[0]` (RO)
    ///
    /// This field indicates that the Master is in 7-bit addressing mode and the address sent was not acknowledged by any slave.
    /// Reset value: 0x0
    /// Role of DW_apb_i2c: Master-Transmitter or Master-Receiver
    #[inline(always)]
    pub fn abrt_7b_addr_noack(&self) -> bool {
        (self.value.read_bits() & Self::ABRT_7B_ADDR_NOACK_MASK) != 0
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

    /// Read **TX_FLUSH_CNT** `[31:23]`
    ///
    /// This field indicates the number of Tx FIFO Data Commands which are flushed due to TX_ABRT interrupt. It is cleared whenever I2C is disabled.
    /// Reset value: 0x0
    /// Role of DW_apb_i2c: Master-Transmitter or Slave-Transmitter
    #[inline(always)]
    pub const fn tx_flush_cnt(&self) -> u32 {
        (self.bits & IcTxAbrtSource::TX_FLUSH_CNT_MASK) >> IcTxAbrtSource::TX_FLUSH_CNT_OFFSET
    }

    /// Read **ABRT_USER_ABRT** `[16:16]`
    ///
    /// This is a master-mode-only bit. Master has detected the transfer abort (IC_ENABLE[1])
    /// Reset value: 0x0
    /// Role of DW_apb_i2c: Master-Transmitter
    #[inline(always)]
    pub const fn abrt_user_abrt(&self) -> bool {
        (self.bits & IcTxAbrtSource::ABRT_USER_ABRT_MASK) != 0
    }

    /// Read **ABRT_SLVRD_INTX** `[15:15]`
    ///
    /// 1: When the processor side responds to a slave mode request for data to be transmitted to a remote master and user writes a 1 in CMD (bit 8) of IC_DATA_CMD register.
    /// Reset value: 0x0
    /// Role of DW_apb_i2c: Slave-Transmitter
    #[inline(always)]
    pub const fn abrt_slvrd_intx(&self) -> bool {
        (self.bits & IcTxAbrtSource::ABRT_SLVRD_INTX_MASK) != 0
    }

    /// Read **ABRT_SLV_ARBLOST** `[14:14]`
    ///
    /// This field indicates that a Slave has lost the bus while transmitting data to a remote master. IC_TX_ABRT_SOURCE[12] is set at the same time. Note: Even though the slave never 'owns' the bus, something could go wrong on the bus. This is a fail safe check. For instance, during a data transmission at the low-to-high transition of SCL, if what is on the data bus is not what is supposed to be transmitted, then DW_apb_i2c no longer own the bus.
    /// Reset value: 0x0
    /// Role of DW_apb_i2c: Slave-Transmitter
    #[inline(always)]
    pub const fn abrt_slv_arblost(&self) -> bool {
        (self.bits & IcTxAbrtSource::ABRT_SLV_ARBLOST_MASK) != 0
    }

    /// Read **ABRT_SLVFLUSH_TXFIFO** `[13:13]`
    ///
    /// This field specifies that the Slave has received a read command and some data exists in the TX FIFO, so the slave issues a TX_ABRT interrupt to flush old data in TX FIFO.
    /// Reset value: 0x0
    /// Role of DW_apb_i2c: Slave-Transmitter
    #[inline(always)]
    pub const fn abrt_slvflush_txfifo(&self) -> bool {
        (self.bits & IcTxAbrtSource::ABRT_SLVFLUSH_TXFIFO_MASK) != 0
    }

    /// Read **ARB_LOST** `[12:12]`
    ///
    /// This field specifies that the Master has lost arbitration, or if IC_TX_ABRT_SOURCE[14] is also set, then the slave transmitter has lost arbitration.
    /// Reset value: 0x0
    /// Role of DW_apb_i2c: Master-Transmitter or Slave-Transmitter
    #[inline(always)]
    pub const fn arb_lost(&self) -> bool {
        (self.bits & IcTxAbrtSource::ARB_LOST_MASK) != 0
    }

    /// Read **ABRT_MASTER_DIS** `[11:11]`
    ///
    /// This field indicates that the User tries to initiate a Master operation with the Master mode disabled.
    /// Reset value: 0x0
    /// Role of DW_apb_i2c: Master-Transmitter or Master-Receiver
    #[inline(always)]
    pub const fn abrt_master_dis(&self) -> bool {
        (self.bits & IcTxAbrtSource::ABRT_MASTER_DIS_MASK) != 0
    }

    /// Read **ABRT_10B_RD_NORSTRT** `[10:10]`
    ///
    /// This field indicates that the restart is disabled (IC_RESTART_EN bit (IC_CON[5]) =0) and the master sends a read command in 10-bit addressing mode.
    /// Reset value: 0x0
    /// Role of DW_apb_i2c: Master-Receiver
    #[inline(always)]
    pub const fn abrt_10b_rd_norstrt(&self) -> bool {
        (self.bits & IcTxAbrtSource::ABRT_10B_RD_NORSTRT_MASK) != 0
    }

    /// Read **ABRT_SBYTE_NORSTRT** `[9:9]`
    ///
    /// To clear Bit 9, the source of the ABRT_SBYTE_NORSTRT must be fixed first; restart must be enabled (IC_CON[5]=1), the SPECIAL bit must be cleared (IC_TAR[11]), or the GC_OR_START bit must be cleared (IC_TAR[10]). Once the source of the ABRT_SBYTE_NORSTRT is fixed, then this bit can be cleared in the same manner as other bits in this register. If the source of the ABRT_SBYTE_NORSTRT is not fixed before attempting to clear this bit, bit 9 clears for one cycle and then gets reasserted. When this field is set to 1, the restart is disabled (IC_RESTART_EN bit (IC_CON[5]) =0) and the user is trying to send a START Byte.
    /// Reset value: 0x0
    /// Role of DW_apb_i2c: Master
    #[inline(always)]
    pub const fn abrt_sbyte_norstrt(&self) -> bool {
        (self.bits & IcTxAbrtSource::ABRT_SBYTE_NORSTRT_MASK) != 0
    }

    /// Read **ABRT_HS_NORSTRT** `[8:8]`
    ///
    /// This field indicates that the restart is disabled (IC_RESTART_EN bit (IC_CON[5]) =0) and the user is trying to use the master to transfer data in High Speed mode.
    /// Reset value: 0x0
    /// Role of DW_apb_i2c: Master-Transmitter or Master-Receiver
    #[inline(always)]
    pub const fn abrt_hs_norstrt(&self) -> bool {
        (self.bits & IcTxAbrtSource::ABRT_HS_NORSTRT_MASK) != 0
    }

    /// Read **ABRT_SBYTE_ACKDET** `[7:7]`
    ///
    /// This field indicates that the Master has sent a START Byte and the START Byte was acknowledged (wrong behavior).
    /// Reset value: 0x0
    /// Role of DW_apb_i2c: Master
    #[inline(always)]
    pub const fn abrt_sbyte_ackdet(&self) -> bool {
        (self.bits & IcTxAbrtSource::ABRT_SBYTE_ACKDET_MASK) != 0
    }

    /// Read **ABRT_HS_ACKDET** `[6:6]`
    ///
    /// This field indicates that the Master is in High Speed mode and the High Speed Master code was acknowledged (wrong behavior).
    /// Reset value: 0x0
    /// Role of DW_apb_i2c: Master
    #[inline(always)]
    pub const fn abrt_hs_ackdet(&self) -> bool {
        (self.bits & IcTxAbrtSource::ABRT_HS_ACKDET_MASK) != 0
    }

    /// Read **ABRT_GCALL_READ** `[5:5]`
    ///
    /// This field indicates that DW_apb_i2c in the master mode has sent a General Call but the user programmed the byte following the General Call to be a read from the bus (IC_DATA_CMD[9] is set to 1).
    /// Reset value: 0x0
    /// Role of DW_apb_i2c: Master-Transmitter
    #[inline(always)]
    pub const fn abrt_gcall_read(&self) -> bool {
        (self.bits & IcTxAbrtSource::ABRT_GCALL_READ_MASK) != 0
    }

    /// Read **ABRT_GCALL_NOACK** `[4:4]`
    ///
    /// This field indicates that DW_apb_i2c in master mode has sent a General Call and no slave on the bus acknowledged the General Call.
    /// Reset value: 0x0
    /// Role of DW_apb_i2c: Master-Transmitter
    #[inline(always)]
    pub const fn abrt_gcall_noack(&self) -> bool {
        (self.bits & IcTxAbrtSource::ABRT_GCALL_NOACK_MASK) != 0
    }

    /// Read **ABRT_TXDATA_NOACK** `[3:3]`
    ///
    /// This field indicates the master-mode only bit. When the master receives an acknowledgement for the address, but when it sends data byte(s) following the address, it did not receive an acknowledge from the remote slave(s).
    /// Reset value: 0x0
    /// Role of DW_apb_i2c: Master-Transmitter
    #[inline(always)]
    pub const fn abrt_txdata_noack(&self) -> bool {
        (self.bits & IcTxAbrtSource::ABRT_TXDATA_NOACK_MASK) != 0
    }

    /// Read **ABRT_10ADDR2_NOACK** `[2:2]`
    ///
    /// This field indicates that the Master is in 10-bit address mode and that the second address byte of the 10-bit address was not acknowledged by any slave.
    /// Reset value: 0x0
    /// Role of DW_apb_i2c: Master-Transmitter or Master-Receiver
    #[inline(always)]
    pub const fn abrt_10addr2_noack(&self) -> bool {
        (self.bits & IcTxAbrtSource::ABRT_10ADDR2_NOACK_MASK) != 0
    }

    /// Read **ABRT_10ADDR1_NOACK** `[1:1]`
    ///
    /// This field indicates that the Master is in 10-bit address mode and the first 10-bit address byte was not acknowledged by any slave.
    /// Reset value: 0x0
    /// Role of DW_apb_i2c: Master-Transmitter or Master-Receiver
    #[inline(always)]
    pub const fn abrt_10addr1_noack(&self) -> bool {
        (self.bits & IcTxAbrtSource::ABRT_10ADDR1_NOACK_MASK) != 0
    }

    /// Read **ABRT_7B_ADDR_NOACK** `[0:0]`
    ///
    /// This field indicates that the Master is in 7-bit addressing mode and the address sent was not acknowledged by any slave.
    /// Reset value: 0x0
    /// Role of DW_apb_i2c: Master-Transmitter or Master-Receiver
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
    pub fn read_val(&self) -> IcTxAbrtSourceVal {
        IcTxAbrtSourceVal::from_bits(self.value.read_bits())
    }
}

/// IC_SLV_DATA_NACK_ONLY Register
///
/// Generate Slave Data NACK Register
/// The register is used to generate a NACK for the data part of a transfer when DW_apb_i2c is acting as a slave-receiver. This register only exists when the IC_SLV_DATA_NACK_ONLY parameter is set to 1. When this parameter disabled, this register does not exist and writing to the register's address has no effect.
/// A write can occur on this register if both of the following conditions are met: - DW_apb_i2c is disabled (IC_ENABLE[0] = 0) - Slave part is inactive (IC_STATUS[6] = 0) Note: The IC_STATUS[6] is a register read-back location for the internal slv_activity signal; the user should poll this before writing the ic_slv_data_nack_only bit.
///
/// Address offset: 0x0084
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **NACK** `[0]` (RW)
///
///   Generate NACK. This NACK generation only occurs when DW_apb_i2c is a slave-receiver. If this register is set to a value of 1, it can only generate a NACK after a data byte is received; hence, the data transfer is aborted and the data received is not pushed to the receive buffer.
///   When the register is set to a value of 0, it generates NACK/ACK, depending on normal criteria. - 1: generate NACK after data byte received - 0: generate NACK/ACK normally Reset value: 0x0
#[repr(transparent)]
pub struct IcSlvDataNackOnly {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for IcSlvDataNackOnly {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl IcSlvDataNackOnly {
    // ======== Field Constants ========

    /// **NACK**
    ///
    /// Generate NACK. This NACK generation only occurs when DW_apb_i2c is a slave-receiver. If this register is set to a value of 1, it can only generate a NACK after a data byte is received; hence, the data transfer is aborted and the data received is not pushed to the receive buffer.
    /// When the register is set to a value of 0, it generates NACK/ACK, depending on normal criteria. - 1: generate NACK after data byte received - 0: generate NACK/ACK normally Reset value: 0x0
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const NACK_OFFSET: u32 = 0;
    pub const NACK_MASK: u32 = 0x00000001;
    pub const NACK: u32 = 1 << 0;
    /// DISABLED = 0
    /// Slave receiver generates NACK normally
    pub const NACK_DISABLED: u32 = 0 << 0;
    pub const NACK_DISABLED_VAL: u32 = 0;
    /// ENABLED = 1
    /// Slave receiver generates NACK upon data reception only
    pub const NACK_ENABLED: u32 = 1 << 0;
    pub const NACK_ENABLED_VAL: u32 = 1;

    // ======== Field Accessors ========

    /// **NACK** `[0]` (RW)
    ///
    /// Generate NACK. This NACK generation only occurs when DW_apb_i2c is a slave-receiver. If this register is set to a value of 1, it can only generate a NACK after a data byte is received; hence, the data transfer is aborted and the data received is not pushed to the receive buffer.
    /// When the register is set to a value of 0, it generates NACK/ACK, depending on normal criteria. - 1: generate NACK after data byte received - 0: generate NACK/ACK normally Reset value: 0x0
    #[inline(always)]
    pub fn nack(&self) -> bool {
        (self.value.read_bits() & Self::NACK_MASK) != 0
    }

    /// Set NACK bit atomically (RW)
    #[inline(always)]
    pub fn set_nack(&self) {
        self.set_bits(Self::NACK_MASK);
    }

    /// Clear NACK bit atomically (RW)
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

    /// Read **NACK** `[0:0]`
    ///
    /// Generate NACK. This NACK generation only occurs when DW_apb_i2c is a slave-receiver. If this register is set to a value of 1, it can only generate a NACK after a data byte is received; hence, the data transfer is aborted and the data received is not pushed to the receive buffer.
    /// When the register is set to a value of 0, it generates NACK/ACK, depending on normal criteria. - 1: generate NACK after data byte received - 0: generate NACK/ACK normally Reset value: 0x0
    #[inline(always)]
    pub const fn nack(&self) -> bool {
        (self.bits & IcSlvDataNackOnly::NACK_MASK) != 0
    }

    /// Set **NACK** `[0:0]`
    ///
    /// Generate NACK. This NACK generation only occurs when DW_apb_i2c is a slave-receiver. If this register is set to a value of 1, it can only generate a NACK after a data byte is received; hence, the data transfer is aborted and the data received is not pushed to the receive buffer.
    /// When the register is set to a value of 0, it generates NACK/ACK, depending on normal criteria. - 1: generate NACK after data byte received - 0: generate NACK/ACK normally Reset value: 0x0
    #[inline(always)]
    pub const fn with_nack(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IcSlvDataNackOnly::NACK_MASK
            } else {
                self.bits & !IcSlvDataNackOnly::NACK_MASK
            },
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
    pub fn read_val(&self) -> IcSlvDataNackOnlyVal {
        IcSlvDataNackOnlyVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: IcSlvDataNackOnlyVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(IcSlvDataNackOnlyVal) -> IcSlvDataNackOnlyVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// IC_DMA_CR Register
///
/// DMA Control Register
/// The register is used to enable the DMA Controller interface operation. There is a separate bit for transmit and receive. This can be programmed regardless of the state of IC_ENABLE.
///
/// Address offset: 0x0088
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **TDMAE** `[1]` (RW)
///
///   Transmit DMA Enable. This bit enables/disables the transmit FIFO DMA channel. Reset value: 0x0
///
/// - **RDMAE** `[0]` (RW)
///
///   Receive DMA Enable. This bit enables/disables the receive FIFO DMA channel. Reset value: 0x0
#[repr(transparent)]
pub struct IcDmaCr {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for IcDmaCr {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl IcDmaCr {
    // ======== Field Constants ========

    /// **TDMAE**
    ///
    /// Transmit DMA Enable. This bit enables/disables the transmit FIFO DMA channel. Reset value: 0x0
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const TDMAE_OFFSET: u32 = 1;
    pub const TDMAE_MASK: u32 = 0x00000002;
    pub const TDMAE: u32 = 1 << 1;
    /// DISABLED = 0
    /// transmit FIFO DMA channel disabled
    pub const TDMAE_DISABLED: u32 = 0 << 1;
    pub const TDMAE_DISABLED_VAL: u32 = 0;
    /// ENABLED = 1
    /// Transmit FIFO DMA channel enabled
    pub const TDMAE_ENABLED: u32 = 1 << 1;
    pub const TDMAE_ENABLED_VAL: u32 = 1;

    /// **RDMAE**
    ///
    /// Receive DMA Enable. This bit enables/disables the receive FIFO DMA channel. Reset value: 0x0
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const RDMAE_OFFSET: u32 = 0;
    pub const RDMAE_MASK: u32 = 0x00000001;
    pub const RDMAE: u32 = 1 << 0;
    /// DISABLED = 0
    /// Receive FIFO DMA channel disabled
    pub const RDMAE_DISABLED: u32 = 0 << 0;
    pub const RDMAE_DISABLED_VAL: u32 = 0;
    /// ENABLED = 1
    /// Receive FIFO DMA channel enabled
    pub const RDMAE_ENABLED: u32 = 1 << 0;
    pub const RDMAE_ENABLED_VAL: u32 = 1;

    // ======== Field Accessors ========

    /// **TDMAE** `[1]` (RW)
    ///
    /// Transmit DMA Enable. This bit enables/disables the transmit FIFO DMA channel. Reset value: 0x0
    #[inline(always)]
    pub fn tdmae(&self) -> bool {
        (self.value.read_bits() & Self::TDMAE_MASK) != 0
    }

    /// Set TDMAE bit atomically (RW)
    #[inline(always)]
    pub fn set_tdmae(&self) {
        self.set_bits(Self::TDMAE_MASK);
    }

    /// Clear TDMAE bit atomically (RW)
    #[inline(always)]
    pub fn clr_tdmae(&self) {
        self.clr_bits(Self::TDMAE_MASK);
    }

    /// **RDMAE** `[0]` (RW)
    ///
    /// Receive DMA Enable. This bit enables/disables the receive FIFO DMA channel. Reset value: 0x0
    #[inline(always)]
    pub fn rdmae(&self) -> bool {
        (self.value.read_bits() & Self::RDMAE_MASK) != 0
    }

    /// Set RDMAE bit atomically (RW)
    #[inline(always)]
    pub fn set_rdmae(&self) {
        self.set_bits(Self::RDMAE_MASK);
    }

    /// Clear RDMAE bit atomically (RW)
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

    /// Read **TDMAE** `[1:1]`
    ///
    /// Transmit DMA Enable. This bit enables/disables the transmit FIFO DMA channel. Reset value: 0x0
    #[inline(always)]
    pub const fn tdmae(&self) -> bool {
        (self.bits & IcDmaCr::TDMAE_MASK) != 0
    }

    /// Read **RDMAE** `[0:0]`
    ///
    /// Receive DMA Enable. This bit enables/disables the receive FIFO DMA channel. Reset value: 0x0
    #[inline(always)]
    pub const fn rdmae(&self) -> bool {
        (self.bits & IcDmaCr::RDMAE_MASK) != 0
    }

    /// Set **TDMAE** `[1:1]`
    ///
    /// Transmit DMA Enable. This bit enables/disables the transmit FIFO DMA channel. Reset value: 0x0
    #[inline(always)]
    pub const fn with_tdmae(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IcDmaCr::TDMAE_MASK
            } else {
                self.bits & !IcDmaCr::TDMAE_MASK
            },
        }
    }

    /// Set **RDMAE** `[0:0]`
    ///
    /// Receive DMA Enable. This bit enables/disables the receive FIFO DMA channel. Reset value: 0x0
    #[inline(always)]
    pub const fn with_rdmae(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IcDmaCr::RDMAE_MASK
            } else {
                self.bits & !IcDmaCr::RDMAE_MASK
            },
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
    pub fn read_val(&self) -> IcDmaCrVal {
        IcDmaCrVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: IcDmaCrVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(IcDmaCrVal) -> IcDmaCrVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
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
/// - **DMATDL** `[3:0]` (RW)
///
///   Transmit Data Level. This bit field controls the level at which a DMA request is made by the transmit logic. It is equal to the watermark level; that is, the dma_tx_req signal is generated when the number of valid data entries in the transmit FIFO is equal to or below this field value, and TDMAE = 1.
///   Reset value: 0x0
#[repr(transparent)]
pub struct IcDmaTdlr {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for IcDmaTdlr {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl IcDmaTdlr {
    // ======== Field Constants ========

    /// **DMATDL**
    ///
    /// Transmit Data Level. This bit field controls the level at which a DMA request is made by the transmit logic. It is equal to the watermark level; that is, the dma_tx_req signal is generated when the number of valid data entries in the transmit FIFO is equal to or below this field value, and TDMAE = 1.
    /// Reset value: 0x0
    ///
    /// Bits `[3:0]`, mask `0x0000000f`
    pub const DMATDL_OFFSET: u32 = 0;
    pub const DMATDL_MASK: u32 = 0x0000000f;

    // ======== Field Accessors ========

    /// **DMATDL** `[3:0]` (RW)
    ///
    /// Transmit Data Level. This bit field controls the level at which a DMA request is made by the transmit logic. It is equal to the watermark level; that is, the dma_tx_req signal is generated when the number of valid data entries in the transmit FIFO is equal to or below this field value, and TDMAE = 1.
    /// Reset value: 0x0
    #[inline(always)]
    pub fn dmatdl(&self) -> u32 {
        (self.value.read_bits() & Self::DMATDL_MASK) >> Self::DMATDL_OFFSET
    }

    /// Write **DMATDL** `[3:0]` (RW)
    ///
    /// Transmit Data Level. This bit field controls the level at which a DMA request is made by the transmit logic. It is equal to the watermark level; that is, the dma_tx_req signal is generated when the number of valid data entries in the transmit FIFO is equal to or below this field value, and TDMAE = 1.
    /// Reset value: 0x0
    #[inline(always)]
    pub fn write_dmatdl(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DMATDL_MASK) | ((value << Self::DMATDL_OFFSET) & Self::DMATDL_MASK)
        });
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

    /// Read **DMATDL** `[3:0]`
    ///
    /// Transmit Data Level. This bit field controls the level at which a DMA request is made by the transmit logic. It is equal to the watermark level; that is, the dma_tx_req signal is generated when the number of valid data entries in the transmit FIFO is equal to or below this field value, and TDMAE = 1.
    /// Reset value: 0x0
    #[inline(always)]
    pub const fn dmatdl(&self) -> u32 {
        (self.bits & IcDmaTdlr::DMATDL_MASK) >> IcDmaTdlr::DMATDL_OFFSET
    }

    /// Set **DMATDL** `[3:0]`
    ///
    /// Transmit Data Level. This bit field controls the level at which a DMA request is made by the transmit logic. It is equal to the watermark level; that is, the dma_tx_req signal is generated when the number of valid data entries in the transmit FIFO is equal to or below this field value, and TDMAE = 1.
    /// Reset value: 0x0
    #[inline(always)]
    pub const fn with_dmatdl(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !IcDmaTdlr::DMATDL_MASK)
                | ((value << IcDmaTdlr::DMATDL_OFFSET) & IcDmaTdlr::DMATDL_MASK),
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
    pub fn read_val(&self) -> IcDmaTdlrVal {
        IcDmaTdlrVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: IcDmaTdlrVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(IcDmaTdlrVal) -> IcDmaTdlrVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
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
/// - **DMARDL** `[3:0]` (RW)
///
///   Receive Data Level. This bit field controls the level at which a DMA request is made by the receive logic. The watermark level = DMARDL+1; that is, dma_rx_req is generated when the number of valid data entries in the receive FIFO is equal to or more than this field value + 1, and RDMAE =1. For instance, when DMARDL is 0, then dma_rx_req is asserted when 1 or more data entries are present in the receive FIFO.
///   Reset value: 0x0
#[repr(transparent)]
pub struct IcDmaRdlr {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for IcDmaRdlr {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl IcDmaRdlr {
    // ======== Field Constants ========

    /// **DMARDL**
    ///
    /// Receive Data Level. This bit field controls the level at which a DMA request is made by the receive logic. The watermark level = DMARDL+1; that is, dma_rx_req is generated when the number of valid data entries in the receive FIFO is equal to or more than this field value + 1, and RDMAE =1. For instance, when DMARDL is 0, then dma_rx_req is asserted when 1 or more data entries are present in the receive FIFO.
    /// Reset value: 0x0
    ///
    /// Bits `[3:0]`, mask `0x0000000f`
    pub const DMARDL_OFFSET: u32 = 0;
    pub const DMARDL_MASK: u32 = 0x0000000f;

    // ======== Field Accessors ========

    /// **DMARDL** `[3:0]` (RW)
    ///
    /// Receive Data Level. This bit field controls the level at which a DMA request is made by the receive logic. The watermark level = DMARDL+1; that is, dma_rx_req is generated when the number of valid data entries in the receive FIFO is equal to or more than this field value + 1, and RDMAE =1. For instance, when DMARDL is 0, then dma_rx_req is asserted when 1 or more data entries are present in the receive FIFO.
    /// Reset value: 0x0
    #[inline(always)]
    pub fn dmardl(&self) -> u32 {
        (self.value.read_bits() & Self::DMARDL_MASK) >> Self::DMARDL_OFFSET
    }

    /// Write **DMARDL** `[3:0]` (RW)
    ///
    /// Receive Data Level. This bit field controls the level at which a DMA request is made by the receive logic. The watermark level = DMARDL+1; that is, dma_rx_req is generated when the number of valid data entries in the receive FIFO is equal to or more than this field value + 1, and RDMAE =1. For instance, when DMARDL is 0, then dma_rx_req is asserted when 1 or more data entries are present in the receive FIFO.
    /// Reset value: 0x0
    #[inline(always)]
    pub fn write_dmardl(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DMARDL_MASK) | ((value << Self::DMARDL_OFFSET) & Self::DMARDL_MASK)
        });
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

    /// Read **DMARDL** `[3:0]`
    ///
    /// Receive Data Level. This bit field controls the level at which a DMA request is made by the receive logic. The watermark level = DMARDL+1; that is, dma_rx_req is generated when the number of valid data entries in the receive FIFO is equal to or more than this field value + 1, and RDMAE =1. For instance, when DMARDL is 0, then dma_rx_req is asserted when 1 or more data entries are present in the receive FIFO.
    /// Reset value: 0x0
    #[inline(always)]
    pub const fn dmardl(&self) -> u32 {
        (self.bits & IcDmaRdlr::DMARDL_MASK) >> IcDmaRdlr::DMARDL_OFFSET
    }

    /// Set **DMARDL** `[3:0]`
    ///
    /// Receive Data Level. This bit field controls the level at which a DMA request is made by the receive logic. The watermark level = DMARDL+1; that is, dma_rx_req is generated when the number of valid data entries in the receive FIFO is equal to or more than this field value + 1, and RDMAE =1. For instance, when DMARDL is 0, then dma_rx_req is asserted when 1 or more data entries are present in the receive FIFO.
    /// Reset value: 0x0
    #[inline(always)]
    pub const fn with_dmardl(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !IcDmaRdlr::DMARDL_MASK)
                | ((value << IcDmaRdlr::DMARDL_OFFSET) & IcDmaRdlr::DMARDL_MASK),
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
    pub fn read_val(&self) -> IcDmaRdlrVal {
        IcDmaRdlrVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: IcDmaRdlrVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(IcDmaRdlrVal) -> IcDmaRdlrVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// IC_SDA_SETUP Register
///
/// I2C SDA Setup Register
/// This register controls the amount of time delay (in terms of number of ic_clk clock periods) introduced in the rising edge of SCL - relative to SDA changing - when DW_apb_i2c services a read request in a slave-transmitter operation. The relevant I2C requirement is tSU:DAT (note 4) as detailed in the I2C Bus Specification. This register must be programmed with a value equal to or greater than 2.
/// Writes to this register succeed only when IC_ENABLE[0] = 0.
/// Note: The length of setup time is calculated using [(IC_SDA_SETUP - 1) * (ic_clk_period)], so if the user requires 10 ic_clk periods of setup time, they should program a value of 11. The IC_SDA_SETUP register is only used by the DW_apb_i2c when operating as a slave transmitter.
///
/// Address offset: 0x0094
/// Reset value: 0x00000064
///
/// ## Fields
///
/// - **SDA_SETUP** `[7:0]` (RW)
///
///   SDA Setup. It is recommended that if the required delay is 1000ns, then for an ic_clk frequency of 10 MHz, IC_SDA_SETUP should be programmed to a value of 11. IC_SDA_SETUP must be programmed with a minimum value of 2.
#[repr(transparent)]
pub struct IcSdaSetup {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for IcSdaSetup {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl IcSdaSetup {
    // ======== Field Constants ========

    /// **SDA_SETUP**
    ///
    /// SDA Setup. It is recommended that if the required delay is 1000ns, then for an ic_clk frequency of 10 MHz, IC_SDA_SETUP should be programmed to a value of 11. IC_SDA_SETUP must be programmed with a minimum value of 2.
    ///
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const SDA_SETUP_OFFSET: u32 = 0;
    pub const SDA_SETUP_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// **SDA_SETUP** `[7:0]` (RW)
    ///
    /// SDA Setup. It is recommended that if the required delay is 1000ns, then for an ic_clk frequency of 10 MHz, IC_SDA_SETUP should be programmed to a value of 11. IC_SDA_SETUP must be programmed with a minimum value of 2.
    #[inline(always)]
    pub fn sda_setup(&self) -> u32 {
        (self.value.read_bits() & Self::SDA_SETUP_MASK) >> Self::SDA_SETUP_OFFSET
    }

    /// Write **SDA_SETUP** `[7:0]` (RW)
    ///
    /// SDA Setup. It is recommended that if the required delay is 1000ns, then for an ic_clk frequency of 10 MHz, IC_SDA_SETUP should be programmed to a value of 11. IC_SDA_SETUP must be programmed with a minimum value of 2.
    #[inline(always)]
    pub fn write_sda_setup(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SDA_SETUP_MASK) | ((value << Self::SDA_SETUP_OFFSET) & Self::SDA_SETUP_MASK)
        });
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

    /// Read **SDA_SETUP** `[7:0]`
    ///
    /// SDA Setup. It is recommended that if the required delay is 1000ns, then for an ic_clk frequency of 10 MHz, IC_SDA_SETUP should be programmed to a value of 11. IC_SDA_SETUP must be programmed with a minimum value of 2.
    #[inline(always)]
    pub const fn sda_setup(&self) -> u32 {
        (self.bits & IcSdaSetup::SDA_SETUP_MASK) >> IcSdaSetup::SDA_SETUP_OFFSET
    }

    /// Set **SDA_SETUP** `[7:0]`
    ///
    /// SDA Setup. It is recommended that if the required delay is 1000ns, then for an ic_clk frequency of 10 MHz, IC_SDA_SETUP should be programmed to a value of 11. IC_SDA_SETUP must be programmed with a minimum value of 2.
    #[inline(always)]
    pub const fn with_sda_setup(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !IcSdaSetup::SDA_SETUP_MASK)
                | ((value << IcSdaSetup::SDA_SETUP_OFFSET) & IcSdaSetup::SDA_SETUP_MASK),
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
    pub fn read_val(&self) -> IcSdaSetupVal {
        IcSdaSetupVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: IcSdaSetupVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(IcSdaSetupVal) -> IcSdaSetupVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// IC_ACK_GENERAL_CALL Register
///
/// I2C ACK General Call Register
/// The register controls whether DW_apb_i2c responds with a ACK or NACK when it receives an I2C General Call address.
/// This register is applicable only when the DW_apb_i2c is in slave mode.
///
/// Address offset: 0x0098
/// Reset value: 0x00000001
///
/// ## Fields
///
/// - **ACK_GEN_CALL** `[0]` (RW)
///
///   ACK General Call. When set to 1, DW_apb_i2c responds with a ACK (by asserting ic_data_oe) when it receives a General Call. Otherwise, DW_apb_i2c responds with a NACK (by negating ic_data_oe).
#[repr(transparent)]
pub struct IcAckGeneralCall {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for IcAckGeneralCall {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl IcAckGeneralCall {
    // ======== Field Constants ========

    /// **ACK_GEN_CALL**
    ///
    /// ACK General Call. When set to 1, DW_apb_i2c responds with a ACK (by asserting ic_data_oe) when it receives a General Call. Otherwise, DW_apb_i2c responds with a NACK (by negating ic_data_oe).
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const ACK_GEN_CALL_OFFSET: u32 = 0;
    pub const ACK_GEN_CALL_MASK: u32 = 0x00000001;
    pub const ACK_GEN_CALL: u32 = 1 << 0;
    /// DISABLED = 0
    /// Generate NACK for a General Call
    pub const ACK_GEN_CALL_DISABLED: u32 = 0 << 0;
    pub const ACK_GEN_CALL_DISABLED_VAL: u32 = 0;
    /// ENABLED = 1
    /// Generate ACK for a General Call
    pub const ACK_GEN_CALL_ENABLED: u32 = 1 << 0;
    pub const ACK_GEN_CALL_ENABLED_VAL: u32 = 1;

    // ======== Field Accessors ========

    /// **ACK_GEN_CALL** `[0]` (RW)
    ///
    /// ACK General Call. When set to 1, DW_apb_i2c responds with a ACK (by asserting ic_data_oe) when it receives a General Call. Otherwise, DW_apb_i2c responds with a NACK (by negating ic_data_oe).
    #[inline(always)]
    pub fn ack_gen_call(&self) -> bool {
        (self.value.read_bits() & Self::ACK_GEN_CALL_MASK) != 0
    }

    /// Set ACK_GEN_CALL bit atomically (RW)
    #[inline(always)]
    pub fn set_ack_gen_call(&self) {
        self.set_bits(Self::ACK_GEN_CALL_MASK);
    }

    /// Clear ACK_GEN_CALL bit atomically (RW)
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

    /// Read **ACK_GEN_CALL** `[0:0]`
    ///
    /// ACK General Call. When set to 1, DW_apb_i2c responds with a ACK (by asserting ic_data_oe) when it receives a General Call. Otherwise, DW_apb_i2c responds with a NACK (by negating ic_data_oe).
    #[inline(always)]
    pub const fn ack_gen_call(&self) -> bool {
        (self.bits & IcAckGeneralCall::ACK_GEN_CALL_MASK) != 0
    }

    /// Set **ACK_GEN_CALL** `[0:0]`
    ///
    /// ACK General Call. When set to 1, DW_apb_i2c responds with a ACK (by asserting ic_data_oe) when it receives a General Call. Otherwise, DW_apb_i2c responds with a NACK (by negating ic_data_oe).
    #[inline(always)]
    pub const fn with_ack_gen_call(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IcAckGeneralCall::ACK_GEN_CALL_MASK
            } else {
                self.bits & !IcAckGeneralCall::ACK_GEN_CALL_MASK
            },
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
    pub fn read_val(&self) -> IcAckGeneralCallVal {
        IcAckGeneralCallVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: IcAckGeneralCallVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(IcAckGeneralCallVal) -> IcAckGeneralCallVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// IC_ENABLE_STATUS Register
///
/// I2C Enable Status Register
/// The register is used to report the DW_apb_i2c hardware status when the IC_ENABLE[0] register is set from 1 to 0; that is, when DW_apb_i2c is disabled.
/// If IC_ENABLE[0] has been set to 1, bits 2:1 are forced to 0, and bit 0 is forced to 1.
/// If IC_ENABLE[0] has been set to 0, bits 2:1 is only be valid as soon as bit 0 is read as '0'.
/// Note: When IC_ENABLE[0] has been set to 0, a delay occurs for bit 0 to be read as 0 because disabling the DW_apb_i2c depends on I2C bus activities.
///
/// Address offset: 0x009c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **SLV_RX_DATA_LOST** `[2]` (RO)
///
///   Slave Received Data Lost. This bit indicates if a Slave-Receiver operation has been aborted with at least one data byte received from an I2C transfer due to the setting bit 0 of IC_ENABLE from 1 to 0. When read as 1, DW_apb_i2c is deemed to have been actively engaged in an aborted I2C transfer (with matching address) and the data phase of the I2C transfer has been entered, even though a data byte has been responded with a NACK.
///   Note: If the remote I2C master terminates the transfer with a STOP condition before the DW_apb_i2c has a chance to NACK a transfer, and IC_ENABLE[0] has been set to 0, then this bit is also set to 1.
///   When read as 0, DW_apb_i2c is deemed to have been disabled without being actively involved in the data phase of a Slave-Receiver transfer.
///   Note: The CPU can safely read this bit when IC_EN (bit 0) is read as 0.
///   Reset value: 0x0
///
/// - **SLV_DISABLED_WHILE_BUSY** `[1]` (RO)
///
///   Slave Disabled While Busy (Transmit, Receive). This bit indicates if a potential or active Slave operation has been aborted due to the setting bit 0 of the IC_ENABLE register from 1 to 0. This bit is set when the CPU writes a 0 to the IC_ENABLE register while:
///   (a) DW_apb_i2c is receiving the address byte of the Slave-Transmitter operation from a remote master;
///   OR,
///   (b) address and data bytes of the Slave-Receiver operation from a remote master.
///   When read as 1, DW_apb_i2c is deemed to have forced a NACK during any part of an I2C transfer, irrespective of whether the I2C address matches the slave address set in DW_apb_i2c (IC_SAR register) OR if the transfer is completed before IC_ENABLE is set to 0 but has not taken effect.
///   Note: If the remote I2C master terminates the transfer with a STOP condition before the DW_apb_i2c has a chance to NACK a transfer, and IC_ENABLE[0] has been set to 0, then this bit will also be set to 1.
///   When read as 0, DW_apb_i2c is deemed to have been disabled when there is master activity, or when the I2C bus is idle.
///   Note: The CPU can safely read this bit when IC_EN (bit 0) is read as 0.
///   Reset value: 0x0
///
/// - **IC_EN** `[0]` (RO)
///
///   ic_en Status. This bit always reflects the value driven on the output port ic_en. - When read as 1, DW_apb_i2c is deemed to be in an enabled state. - When read as 0, DW_apb_i2c is deemed completely inactive. Note: The CPU can safely read this bit anytime. When this bit is read as 0, the CPU can safely read SLV_RX_DATA_LOST (bit 2) and SLV_DISABLED_WHILE_BUSY (bit 1).
///   Reset value: 0x0
#[repr(transparent)]
pub struct IcEnableStatus {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for IcEnableStatus {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl IcEnableStatus {
    // ======== Field Constants ========

    /// **SLV_RX_DATA_LOST**
    ///
    /// Slave Received Data Lost. This bit indicates if a Slave-Receiver operation has been aborted with at least one data byte received from an I2C transfer due to the setting bit 0 of IC_ENABLE from 1 to 0. When read as 1, DW_apb_i2c is deemed to have been actively engaged in an aborted I2C transfer (with matching address) and the data phase of the I2C transfer has been entered, even though a data byte has been responded with a NACK.
    /// Note: If the remote I2C master terminates the transfer with a STOP condition before the DW_apb_i2c has a chance to NACK a transfer, and IC_ENABLE[0] has been set to 0, then this bit is also set to 1.
    /// When read as 0, DW_apb_i2c is deemed to have been disabled without being actively involved in the data phase of a Slave-Receiver transfer.
    /// Note: The CPU can safely read this bit when IC_EN (bit 0) is read as 0.
    /// Reset value: 0x0
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const SLV_RX_DATA_LOST_OFFSET: u32 = 2;
    pub const SLV_RX_DATA_LOST_MASK: u32 = 0x00000004;
    pub const SLV_RX_DATA_LOST: u32 = 1 << 2;
    /// INACTIVE = 0
    /// Slave RX Data is not lost
    pub const SLV_RX_DATA_LOST_INACTIVE: u32 = 0 << 2;
    pub const SLV_RX_DATA_LOST_INACTIVE_VAL: u32 = 0;
    /// ACTIVE = 1
    /// Slave RX Data is lost
    pub const SLV_RX_DATA_LOST_ACTIVE: u32 = 1 << 2;
    pub const SLV_RX_DATA_LOST_ACTIVE_VAL: u32 = 1;

    /// **SLV_DISABLED_WHILE_BUSY**
    ///
    /// Slave Disabled While Busy (Transmit, Receive). This bit indicates if a potential or active Slave operation has been aborted due to the setting bit 0 of the IC_ENABLE register from 1 to 0. This bit is set when the CPU writes a 0 to the IC_ENABLE register while:
    /// (a) DW_apb_i2c is receiving the address byte of the Slave-Transmitter operation from a remote master;
    /// OR,
    /// (b) address and data bytes of the Slave-Receiver operation from a remote master.
    /// When read as 1, DW_apb_i2c is deemed to have forced a NACK during any part of an I2C transfer, irrespective of whether the I2C address matches the slave address set in DW_apb_i2c (IC_SAR register) OR if the transfer is completed before IC_ENABLE is set to 0 but has not taken effect.
    /// Note: If the remote I2C master terminates the transfer with a STOP condition before the DW_apb_i2c has a chance to NACK a transfer, and IC_ENABLE[0] has been set to 0, then this bit will also be set to 1.
    /// When read as 0, DW_apb_i2c is deemed to have been disabled when there is master activity, or when the I2C bus is idle.
    /// Note: The CPU can safely read this bit when IC_EN (bit 0) is read as 0.
    /// Reset value: 0x0
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const SLV_DISABLED_WHILE_BUSY_OFFSET: u32 = 1;
    pub const SLV_DISABLED_WHILE_BUSY_MASK: u32 = 0x00000002;
    pub const SLV_DISABLED_WHILE_BUSY: u32 = 1 << 1;
    /// INACTIVE = 0
    /// Slave is disabled when it is idle
    pub const SLV_DISABLED_WHILE_BUSY_INACTIVE: u32 = 0 << 1;
    pub const SLV_DISABLED_WHILE_BUSY_INACTIVE_VAL: u32 = 0;
    /// ACTIVE = 1
    /// Slave is disabled when it is active
    pub const SLV_DISABLED_WHILE_BUSY_ACTIVE: u32 = 1 << 1;
    pub const SLV_DISABLED_WHILE_BUSY_ACTIVE_VAL: u32 = 1;

    /// **IC_EN**
    ///
    /// ic_en Status. This bit always reflects the value driven on the output port ic_en. - When read as 1, DW_apb_i2c is deemed to be in an enabled state. - When read as 0, DW_apb_i2c is deemed completely inactive. Note: The CPU can safely read this bit anytime. When this bit is read as 0, the CPU can safely read SLV_RX_DATA_LOST (bit 2) and SLV_DISABLED_WHILE_BUSY (bit 1).
    /// Reset value: 0x0
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const IC_EN_OFFSET: u32 = 0;
    pub const IC_EN_MASK: u32 = 0x00000001;
    pub const IC_EN: u32 = 1 << 0;
    /// DISABLED = 0
    /// I2C disabled
    pub const IC_EN_DISABLED: u32 = 0 << 0;
    pub const IC_EN_DISABLED_VAL: u32 = 0;
    /// ENABLED = 1
    /// I2C enabled
    pub const IC_EN_ENABLED: u32 = 1 << 0;
    pub const IC_EN_ENABLED_VAL: u32 = 1;

    // ======== Field Accessors ========

    /// **SLV_RX_DATA_LOST** `[2]` (RO)
    ///
    /// Slave Received Data Lost. This bit indicates if a Slave-Receiver operation has been aborted with at least one data byte received from an I2C transfer due to the setting bit 0 of IC_ENABLE from 1 to 0. When read as 1, DW_apb_i2c is deemed to have been actively engaged in an aborted I2C transfer (with matching address) and the data phase of the I2C transfer has been entered, even though a data byte has been responded with a NACK.
    /// Note: If the remote I2C master terminates the transfer with a STOP condition before the DW_apb_i2c has a chance to NACK a transfer, and IC_ENABLE[0] has been set to 0, then this bit is also set to 1.
    /// When read as 0, DW_apb_i2c is deemed to have been disabled without being actively involved in the data phase of a Slave-Receiver transfer.
    /// Note: The CPU can safely read this bit when IC_EN (bit 0) is read as 0.
    /// Reset value: 0x0
    #[inline(always)]
    pub fn slv_rx_data_lost(&self) -> bool {
        (self.value.read_bits() & Self::SLV_RX_DATA_LOST_MASK) != 0
    }

    /// **SLV_DISABLED_WHILE_BUSY** `[1]` (RO)
    ///
    /// Slave Disabled While Busy (Transmit, Receive). This bit indicates if a potential or active Slave operation has been aborted due to the setting bit 0 of the IC_ENABLE register from 1 to 0. This bit is set when the CPU writes a 0 to the IC_ENABLE register while:
    /// (a) DW_apb_i2c is receiving the address byte of the Slave-Transmitter operation from a remote master;
    /// OR,
    /// (b) address and data bytes of the Slave-Receiver operation from a remote master.
    /// When read as 1, DW_apb_i2c is deemed to have forced a NACK during any part of an I2C transfer, irrespective of whether the I2C address matches the slave address set in DW_apb_i2c (IC_SAR register) OR if the transfer is completed before IC_ENABLE is set to 0 but has not taken effect.
    /// Note: If the remote I2C master terminates the transfer with a STOP condition before the DW_apb_i2c has a chance to NACK a transfer, and IC_ENABLE[0] has been set to 0, then this bit will also be set to 1.
    /// When read as 0, DW_apb_i2c is deemed to have been disabled when there is master activity, or when the I2C bus is idle.
    /// Note: The CPU can safely read this bit when IC_EN (bit 0) is read as 0.
    /// Reset value: 0x0
    #[inline(always)]
    pub fn slv_disabled_while_busy(&self) -> bool {
        (self.value.read_bits() & Self::SLV_DISABLED_WHILE_BUSY_MASK) != 0
    }

    /// **IC_EN** `[0]` (RO)
    ///
    /// ic_en Status. This bit always reflects the value driven on the output port ic_en. - When read as 1, DW_apb_i2c is deemed to be in an enabled state. - When read as 0, DW_apb_i2c is deemed completely inactive. Note: The CPU can safely read this bit anytime. When this bit is read as 0, the CPU can safely read SLV_RX_DATA_LOST (bit 2) and SLV_DISABLED_WHILE_BUSY (bit 1).
    /// Reset value: 0x0
    #[inline(always)]
    pub fn ic_en(&self) -> bool {
        (self.value.read_bits() & Self::IC_EN_MASK) != 0
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

    /// Read **SLV_RX_DATA_LOST** `[2:2]`
    ///
    /// Slave Received Data Lost. This bit indicates if a Slave-Receiver operation has been aborted with at least one data byte received from an I2C transfer due to the setting bit 0 of IC_ENABLE from 1 to 0. When read as 1, DW_apb_i2c is deemed to have been actively engaged in an aborted I2C transfer (with matching address) and the data phase of the I2C transfer has been entered, even though a data byte has been responded with a NACK.
    /// Note: If the remote I2C master terminates the transfer with a STOP condition before the DW_apb_i2c has a chance to NACK a transfer, and IC_ENABLE[0] has been set to 0, then this bit is also set to 1.
    /// When read as 0, DW_apb_i2c is deemed to have been disabled without being actively involved in the data phase of a Slave-Receiver transfer.
    /// Note: The CPU can safely read this bit when IC_EN (bit 0) is read as 0.
    /// Reset value: 0x0
    #[inline(always)]
    pub const fn slv_rx_data_lost(&self) -> bool {
        (self.bits & IcEnableStatus::SLV_RX_DATA_LOST_MASK) != 0
    }

    /// Read **SLV_DISABLED_WHILE_BUSY** `[1:1]`
    ///
    /// Slave Disabled While Busy (Transmit, Receive). This bit indicates if a potential or active Slave operation has been aborted due to the setting bit 0 of the IC_ENABLE register from 1 to 0. This bit is set when the CPU writes a 0 to the IC_ENABLE register while:
    /// (a) DW_apb_i2c is receiving the address byte of the Slave-Transmitter operation from a remote master;
    /// OR,
    /// (b) address and data bytes of the Slave-Receiver operation from a remote master.
    /// When read as 1, DW_apb_i2c is deemed to have forced a NACK during any part of an I2C transfer, irrespective of whether the I2C address matches the slave address set in DW_apb_i2c (IC_SAR register) OR if the transfer is completed before IC_ENABLE is set to 0 but has not taken effect.
    /// Note: If the remote I2C master terminates the transfer with a STOP condition before the DW_apb_i2c has a chance to NACK a transfer, and IC_ENABLE[0] has been set to 0, then this bit will also be set to 1.
    /// When read as 0, DW_apb_i2c is deemed to have been disabled when there is master activity, or when the I2C bus is idle.
    /// Note: The CPU can safely read this bit when IC_EN (bit 0) is read as 0.
    /// Reset value: 0x0
    #[inline(always)]
    pub const fn slv_disabled_while_busy(&self) -> bool {
        (self.bits & IcEnableStatus::SLV_DISABLED_WHILE_BUSY_MASK) != 0
    }

    /// Read **IC_EN** `[0:0]`
    ///
    /// ic_en Status. This bit always reflects the value driven on the output port ic_en. - When read as 1, DW_apb_i2c is deemed to be in an enabled state. - When read as 0, DW_apb_i2c is deemed completely inactive. Note: The CPU can safely read this bit anytime. When this bit is read as 0, the CPU can safely read SLV_RX_DATA_LOST (bit 2) and SLV_DISABLED_WHILE_BUSY (bit 1).
    /// Reset value: 0x0
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
    pub fn read_val(&self) -> IcEnableStatusVal {
        IcEnableStatusVal::from_bits(self.value.read_bits())
    }
}

/// IC_FS_SPKLEN Register
///
/// I2C SS, FS or FM+ spike suppression limit
/// This register is used to store the duration, measured in ic_clk cycles, of the longest spike that is filtered out by the spike suppression logic when the component is operating in SS, FS or FM+ modes. The relevant I2C requirement is tSP (table 4) as detailed in the I2C Bus Specification. This register must be programmed with a minimum value of 1.
///
/// Address offset: 0x00a0
/// Reset value: 0x00000007
///
/// ## Fields
///
/// - **IC_FS_SPKLEN** `[7:0]` (RW)
///
///   This register must be set before any I2C bus transaction can take place to ensure stable operation. This register sets the duration, measured in ic_clk cycles, of the longest spike in the SCL or SDA lines that will be filtered out by the spike suppression logic. This register can be written only when the I2C interface is disabled which corresponds to the IC_ENABLE[0] register being set to 0. Writes at other times have no effect. The minimum valid value is 1; hardware prevents values less than this being written, and if attempted results in 1 being set. or more information, refer to 'Spike Suppression'.
#[repr(transparent)]
pub struct IcFsSpklen {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for IcFsSpklen {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl IcFsSpklen {
    // ======== Field Constants ========

    /// **IC_FS_SPKLEN**
    ///
    /// This register must be set before any I2C bus transaction can take place to ensure stable operation. This register sets the duration, measured in ic_clk cycles, of the longest spike in the SCL or SDA lines that will be filtered out by the spike suppression logic. This register can be written only when the I2C interface is disabled which corresponds to the IC_ENABLE[0] register being set to 0. Writes at other times have no effect. The minimum valid value is 1; hardware prevents values less than this being written, and if attempted results in 1 being set. or more information, refer to 'Spike Suppression'.
    ///
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const IC_FS_SPKLEN_OFFSET: u32 = 0;
    pub const IC_FS_SPKLEN_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// **IC_FS_SPKLEN** `[7:0]` (RW)
    ///
    /// This register must be set before any I2C bus transaction can take place to ensure stable operation. This register sets the duration, measured in ic_clk cycles, of the longest spike in the SCL or SDA lines that will be filtered out by the spike suppression logic. This register can be written only when the I2C interface is disabled which corresponds to the IC_ENABLE[0] register being set to 0. Writes at other times have no effect. The minimum valid value is 1; hardware prevents values less than this being written, and if attempted results in 1 being set. or more information, refer to 'Spike Suppression'.
    #[inline(always)]
    pub fn ic_fs_spklen(&self) -> u32 {
        (self.value.read_bits() & Self::IC_FS_SPKLEN_MASK) >> Self::IC_FS_SPKLEN_OFFSET
    }

    /// Write **IC_FS_SPKLEN** `[7:0]` (RW)
    ///
    /// This register must be set before any I2C bus transaction can take place to ensure stable operation. This register sets the duration, measured in ic_clk cycles, of the longest spike in the SCL or SDA lines that will be filtered out by the spike suppression logic. This register can be written only when the I2C interface is disabled which corresponds to the IC_ENABLE[0] register being set to 0. Writes at other times have no effect. The minimum valid value is 1; hardware prevents values less than this being written, and if attempted results in 1 being set. or more information, refer to 'Spike Suppression'.
    #[inline(always)]
    pub fn write_ic_fs_spklen(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::IC_FS_SPKLEN_MASK)
                | ((value << Self::IC_FS_SPKLEN_OFFSET) & Self::IC_FS_SPKLEN_MASK)
        });
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

    /// Read **IC_FS_SPKLEN** `[7:0]`
    ///
    /// This register must be set before any I2C bus transaction can take place to ensure stable operation. This register sets the duration, measured in ic_clk cycles, of the longest spike in the SCL or SDA lines that will be filtered out by the spike suppression logic. This register can be written only when the I2C interface is disabled which corresponds to the IC_ENABLE[0] register being set to 0. Writes at other times have no effect. The minimum valid value is 1; hardware prevents values less than this being written, and if attempted results in 1 being set. or more information, refer to 'Spike Suppression'.
    #[inline(always)]
    pub const fn ic_fs_spklen(&self) -> u32 {
        (self.bits & IcFsSpklen::IC_FS_SPKLEN_MASK) >> IcFsSpklen::IC_FS_SPKLEN_OFFSET
    }

    /// Set **IC_FS_SPKLEN** `[7:0]`
    ///
    /// This register must be set before any I2C bus transaction can take place to ensure stable operation. This register sets the duration, measured in ic_clk cycles, of the longest spike in the SCL or SDA lines that will be filtered out by the spike suppression logic. This register can be written only when the I2C interface is disabled which corresponds to the IC_ENABLE[0] register being set to 0. Writes at other times have no effect. The minimum valid value is 1; hardware prevents values less than this being written, and if attempted results in 1 being set. or more information, refer to 'Spike Suppression'.
    #[inline(always)]
    pub const fn with_ic_fs_spklen(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !IcFsSpklen::IC_FS_SPKLEN_MASK)
                | ((value << IcFsSpklen::IC_FS_SPKLEN_OFFSET) & IcFsSpklen::IC_FS_SPKLEN_MASK),
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
    pub fn read_val(&self) -> IcFsSpklenVal {
        IcFsSpklenVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: IcFsSpklenVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(IcFsSpklenVal) -> IcFsSpklenVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
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
/// - **CLR_RESTART_DET** `[0]` (RO)
///
///   Read this register to clear the RESTART_DET interrupt (bit 12) of IC_RAW_INTR_STAT register.
///   Reset value: 0x0
#[repr(transparent)]
pub struct IcClrRestartDet {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for IcClrRestartDet {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl IcClrRestartDet {
    // ======== Field Constants ========

    /// **CLR_RESTART_DET**
    ///
    /// Read this register to clear the RESTART_DET interrupt (bit 12) of IC_RAW_INTR_STAT register.
    /// Reset value: 0x0
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const CLR_RESTART_DET_OFFSET: u32 = 0;
    pub const CLR_RESTART_DET_MASK: u32 = 0x00000001;
    pub const CLR_RESTART_DET: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **CLR_RESTART_DET** `[0]` (RO)
    ///
    /// Read this register to clear the RESTART_DET interrupt (bit 12) of IC_RAW_INTR_STAT register.
    /// Reset value: 0x0
    #[inline(always)]
    pub fn clr_restart_det(&self) -> bool {
        (self.value.read_bits() & Self::CLR_RESTART_DET_MASK) != 0
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

    /// Read **CLR_RESTART_DET** `[0:0]`
    ///
    /// Read this register to clear the RESTART_DET interrupt (bit 12) of IC_RAW_INTR_STAT register.
    /// Reset value: 0x0
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
    pub fn read_val(&self) -> IcClrRestartDetVal {
        IcClrRestartDetVal::from_bits(self.value.read_bits())
    }
}

/// IC_COMP_PARAM_1 Register
///
/// Component Parameter Register 1
/// Note This register is not implemented and therefore reads as 0. If it was implemented it would be a constant read-only register that contains encoded information about the component's parameter settings. Fields shown below are the settings for those parameters
///
/// Address offset: 0x00f4
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **TX_BUFFER_DEPTH** `[23:16]` (RO)
///
///   TX Buffer Depth = 16
///
/// - **RX_BUFFER_DEPTH** `[15:8]` (RO)
///
///   RX Buffer Depth = 16
///
/// - **ADD_ENCODED_PARAMS** `[7]` (RO)
///
///   Encoded parameters not visible
///
/// - **HAS_DMA** `[6]` (RO)
///
///   DMA handshaking signals are enabled
///
/// - **INTR_IO** `[5]` (RO)
///
///   COMBINED Interrupt outputs
///
/// - **HC_COUNT_VALUES** `[4]` (RO)
///
///   Programmable count values for each mode.
///
/// - **MAX_SPEED_MODE** `[3:2]` (RO)
///
///   MAX SPEED MODE = FAST MODE
///
/// - **APB_DATA_WIDTH** `[1:0]` (RO)
///
///   APB data bus width is 32 bits
#[repr(transparent)]
pub struct IcCompParam1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for IcCompParam1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl IcCompParam1 {
    // ======== Field Constants ========

    /// **TX_BUFFER_DEPTH**
    ///
    /// TX Buffer Depth = 16
    ///
    /// Bits `[23:16]`, mask `0x00ff0000`
    pub const TX_BUFFER_DEPTH_OFFSET: u32 = 16;
    pub const TX_BUFFER_DEPTH_MASK: u32 = 0x00ff0000;

    /// **RX_BUFFER_DEPTH**
    ///
    /// RX Buffer Depth = 16
    ///
    /// Bits `[15:8]`, mask `0x0000ff00`
    pub const RX_BUFFER_DEPTH_OFFSET: u32 = 8;
    pub const RX_BUFFER_DEPTH_MASK: u32 = 0x0000ff00;

    /// **ADD_ENCODED_PARAMS**
    ///
    /// Encoded parameters not visible
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const ADD_ENCODED_PARAMS_OFFSET: u32 = 7;
    pub const ADD_ENCODED_PARAMS_MASK: u32 = 0x00000080;
    pub const ADD_ENCODED_PARAMS: u32 = 1 << 7;

    /// **HAS_DMA**
    ///
    /// DMA handshaking signals are enabled
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const HAS_DMA_OFFSET: u32 = 6;
    pub const HAS_DMA_MASK: u32 = 0x00000040;
    pub const HAS_DMA: u32 = 1 << 6;

    /// **INTR_IO**
    ///
    /// COMBINED Interrupt outputs
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const INTR_IO_OFFSET: u32 = 5;
    pub const INTR_IO_MASK: u32 = 0x00000020;
    pub const INTR_IO: u32 = 1 << 5;

    /// **HC_COUNT_VALUES**
    ///
    /// Programmable count values for each mode.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const HC_COUNT_VALUES_OFFSET: u32 = 4;
    pub const HC_COUNT_VALUES_MASK: u32 = 0x00000010;
    pub const HC_COUNT_VALUES: u32 = 1 << 4;

    /// **MAX_SPEED_MODE**
    ///
    /// MAX SPEED MODE = FAST MODE
    ///
    /// Bits `[3:2]`, mask `0x0000000c`
    pub const MAX_SPEED_MODE_OFFSET: u32 = 2;
    pub const MAX_SPEED_MODE_MASK: u32 = 0x0000000c;

    /// **APB_DATA_WIDTH**
    ///
    /// APB data bus width is 32 bits
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const APB_DATA_WIDTH_OFFSET: u32 = 0;
    pub const APB_DATA_WIDTH_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **TX_BUFFER_DEPTH** `[23:16]` (RO)
    ///
    /// TX Buffer Depth = 16
    #[inline(always)]
    pub fn tx_buffer_depth(&self) -> u32 {
        (self.value.read_bits() & Self::TX_BUFFER_DEPTH_MASK) >> Self::TX_BUFFER_DEPTH_OFFSET
    }

    /// **RX_BUFFER_DEPTH** `[15:8]` (RO)
    ///
    /// RX Buffer Depth = 16
    #[inline(always)]
    pub fn rx_buffer_depth(&self) -> u32 {
        (self.value.read_bits() & Self::RX_BUFFER_DEPTH_MASK) >> Self::RX_BUFFER_DEPTH_OFFSET
    }

    /// **ADD_ENCODED_PARAMS** `[7]` (RO)
    ///
    /// Encoded parameters not visible
    #[inline(always)]
    pub fn add_encoded_params(&self) -> bool {
        (self.value.read_bits() & Self::ADD_ENCODED_PARAMS_MASK) != 0
    }

    /// **HAS_DMA** `[6]` (RO)
    ///
    /// DMA handshaking signals are enabled
    #[inline(always)]
    pub fn has_dma(&self) -> bool {
        (self.value.read_bits() & Self::HAS_DMA_MASK) != 0
    }

    /// **INTR_IO** `[5]` (RO)
    ///
    /// COMBINED Interrupt outputs
    #[inline(always)]
    pub fn intr_io(&self) -> bool {
        (self.value.read_bits() & Self::INTR_IO_MASK) != 0
    }

    /// **HC_COUNT_VALUES** `[4]` (RO)
    ///
    /// Programmable count values for each mode.
    #[inline(always)]
    pub fn hc_count_values(&self) -> bool {
        (self.value.read_bits() & Self::HC_COUNT_VALUES_MASK) != 0
    }

    /// **MAX_SPEED_MODE** `[3:2]` (RO)
    ///
    /// MAX SPEED MODE = FAST MODE
    #[inline(always)]
    pub fn max_speed_mode(&self) -> u32 {
        (self.value.read_bits() & Self::MAX_SPEED_MODE_MASK) >> Self::MAX_SPEED_MODE_OFFSET
    }

    /// **APB_DATA_WIDTH** `[1:0]` (RO)
    ///
    /// APB data bus width is 32 bits
    #[inline(always)]
    pub fn apb_data_width(&self) -> u32 {
        (self.value.read_bits() & Self::APB_DATA_WIDTH_MASK) >> Self::APB_DATA_WIDTH_OFFSET
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

    /// Read **TX_BUFFER_DEPTH** `[23:16]`
    ///
    /// TX Buffer Depth = 16
    #[inline(always)]
    pub const fn tx_buffer_depth(&self) -> u32 {
        (self.bits & IcCompParam1::TX_BUFFER_DEPTH_MASK) >> IcCompParam1::TX_BUFFER_DEPTH_OFFSET
    }

    /// Read **RX_BUFFER_DEPTH** `[15:8]`
    ///
    /// RX Buffer Depth = 16
    #[inline(always)]
    pub const fn rx_buffer_depth(&self) -> u32 {
        (self.bits & IcCompParam1::RX_BUFFER_DEPTH_MASK) >> IcCompParam1::RX_BUFFER_DEPTH_OFFSET
    }

    /// Read **ADD_ENCODED_PARAMS** `[7:7]`
    ///
    /// Encoded parameters not visible
    #[inline(always)]
    pub const fn add_encoded_params(&self) -> bool {
        (self.bits & IcCompParam1::ADD_ENCODED_PARAMS_MASK) != 0
    }

    /// Read **HAS_DMA** `[6:6]`
    ///
    /// DMA handshaking signals are enabled
    #[inline(always)]
    pub const fn has_dma(&self) -> bool {
        (self.bits & IcCompParam1::HAS_DMA_MASK) != 0
    }

    /// Read **INTR_IO** `[5:5]`
    ///
    /// COMBINED Interrupt outputs
    #[inline(always)]
    pub const fn intr_io(&self) -> bool {
        (self.bits & IcCompParam1::INTR_IO_MASK) != 0
    }

    /// Read **HC_COUNT_VALUES** `[4:4]`
    ///
    /// Programmable count values for each mode.
    #[inline(always)]
    pub const fn hc_count_values(&self) -> bool {
        (self.bits & IcCompParam1::HC_COUNT_VALUES_MASK) != 0
    }

    /// Read **MAX_SPEED_MODE** `[3:2]`
    ///
    /// MAX SPEED MODE = FAST MODE
    #[inline(always)]
    pub const fn max_speed_mode(&self) -> u32 {
        (self.bits & IcCompParam1::MAX_SPEED_MODE_MASK) >> IcCompParam1::MAX_SPEED_MODE_OFFSET
    }

    /// Read **APB_DATA_WIDTH** `[1:0]`
    ///
    /// APB data bus width is 32 bits
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
    pub fn read_val(&self) -> IcCompParam1Val {
        IcCompParam1Val::from_bits(self.value.read_bits())
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
/// - **IC_COMP_VERSION** `[31:0]` (RO)
#[repr(transparent)]
pub struct IcCompVersion {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for IcCompVersion {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl IcCompVersion {
    // ======== Field Constants ========

    /// **IC_COMP_VERSION**
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const IC_COMP_VERSION_OFFSET: u32 = 0;
    pub const IC_COMP_VERSION_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **IC_COMP_VERSION** `[31:0]` (RO)
    #[inline(always)]
    pub fn ic_comp_version(&self) -> u32 {
        (self.value.read_bits() & Self::IC_COMP_VERSION_MASK) >> Self::IC_COMP_VERSION_OFFSET
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

    /// Read **IC_COMP_VERSION** `[31:0]`
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
    pub fn read_val(&self) -> IcCompVersionVal {
        IcCompVersionVal::from_bits(self.value.read_bits())
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
/// - **IC_COMP_TYPE** `[31:0]` (RO)
///
///   Designware Component Type number = 0x44_57_01_40. This assigned unique hex value is constant and is derived from the two ASCII letters 'DW' followed by a 16-bit unsigned number.
#[repr(transparent)]
pub struct IcCompType {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for IcCompType {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl IcCompType {
    // ======== Field Constants ========

    /// **IC_COMP_TYPE**
    ///
    /// Designware Component Type number = 0x44_57_01_40. This assigned unique hex value is constant and is derived from the two ASCII letters 'DW' followed by a 16-bit unsigned number.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const IC_COMP_TYPE_OFFSET: u32 = 0;
    pub const IC_COMP_TYPE_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **IC_COMP_TYPE** `[31:0]` (RO)
    ///
    /// Designware Component Type number = 0x44_57_01_40. This assigned unique hex value is constant and is derived from the two ASCII letters 'DW' followed by a 16-bit unsigned number.
    #[inline(always)]
    pub fn ic_comp_type(&self) -> u32 {
        (self.value.read_bits() & Self::IC_COMP_TYPE_MASK) >> Self::IC_COMP_TYPE_OFFSET
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

    /// Read **IC_COMP_TYPE** `[31:0]`
    ///
    /// Designware Component Type number = 0x44_57_01_40. This assigned unique hex value is constant and is derived from the two ASCII letters 'DW' followed by a 16-bit unsigned number.
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
    pub fn read_val(&self) -> IcCompTypeVal {
        IcCompTypeVal::from_bits(self.value.read_bits())
    }
}

/// Register block
#[repr(C)]
pub struct RegisterBlock {
    /// 0x0000
    /// I2C Control Register. This register can be written only when the DW_apb_i2c is disabled, which corresponds to the IC_ENABLE[0] register being set to 0. Writes at other times have no effect.
    /// Read/Write Access: - bit 10 is read only. - bit 11 is read only - bit 16 is read only - bit 17 is read only - bits 18 and 19 are read only.
    pub ic_con: IcCon,
    /// 0x0004
    /// I2C Target Address Register
    /// This register is 12 bits wide, and bits 31:12 are reserved. This register can be written to only when IC_ENABLE[0] is set to 0.
    /// Note: If the software or application is aware that the DW_apb_i2c is not using the TAR address for the pending commands in the Tx FIFO, then it is possible to update the TAR address even while the Tx FIFO has entries (IC_STATUS[2]= 0). - It is not necessary to perform any write to this register if DW_apb_i2c is enabled as an I2C slave only.
    pub ic_tar: IcTar,
    /// 0x0008
    /// I2C Slave Address Register
    pub ic_sar: IcSar,
    _reserved0: [u8; 4],
    /// 0x0010
    /// I2C Rx/Tx Data Buffer and Command Register; this is the register the CPU writes to when filling the TX FIFO and the CPU reads from when retrieving bytes from RX FIFO.
    /// The size of the register changes as follows:
    /// Write: - 11 bits when IC_EMPTYFIFO_HOLD_MASTER_EN=1 - 9 bits when IC_EMPTYFIFO_HOLD_MASTER_EN=0 Read: - 12 bits when IC_FIRST_DATA_BYTE_STATUS = 1 - 8 bits when IC_FIRST_DATA_BYTE_STATUS = 0 Note: In order for the DW_apb_i2c to continue acknowledging reads, a read command should be written for every byte that is to be received; otherwise the DW_apb_i2c will stop acknowledging.
    pub ic_data_cmd: IcDataCmd,
    /// 0x0014
    /// Standard Speed I2C Clock SCL High Count Register
    pub ic_ss_scl_hcnt: IcSsSclHcnt,
    /// 0x0018
    /// Standard Speed I2C Clock SCL Low Count Register
    pub ic_ss_scl_lcnt: IcSsSclLcnt,
    /// 0x001c
    /// Fast Mode or Fast Mode Plus I2C Clock SCL High Count Register
    pub ic_fs_scl_hcnt: IcFsSclHcnt,
    /// 0x0020
    /// Fast Mode or Fast Mode Plus I2C Clock SCL Low Count Register
    pub ic_fs_scl_lcnt: IcFsSclLcnt,
    _reserved1: [u8; 8],
    /// 0x002c
    /// I2C Interrupt Status Register
    /// Each bit in this register has a corresponding mask bit in the IC_INTR_MASK register. These bits are cleared by reading the matching interrupt clear register. The unmasked raw versions of these bits are available in the IC_RAW_INTR_STAT register.
    pub ic_intr_stat: IcIntrStat,
    /// 0x0030
    /// I2C Interrupt Mask Register.
    /// These bits mask their corresponding interrupt status bits. This register is active low; a value of 0 masks the interrupt, whereas a value of 1 unmasks the interrupt.
    pub ic_intr_mask: IcIntrMask,
    /// 0x0034
    /// I2C Raw Interrupt Status Register
    /// Unlike the IC_INTR_STAT register, these bits are not masked so they always show the true status of the DW_apb_i2c.
    pub ic_raw_intr_stat: IcRawIntrStat,
    /// 0x0038
    /// I2C Receive FIFO Threshold Register
    pub ic_rx_tl: IcRxTl,
    /// 0x003c
    /// I2C Transmit FIFO Threshold Register
    pub ic_tx_tl: IcTxTl,
    /// 0x0040
    /// Clear Combined and Individual Interrupt Register
    pub ic_clr_intr: IcClrIntr,
    /// 0x0044
    /// Clear RX_UNDER Interrupt Register
    pub ic_clr_rx_under: IcClrRxUnder,
    /// 0x0048
    /// Clear RX_OVER Interrupt Register
    pub ic_clr_rx_over: IcClrRxOver,
    /// 0x004c
    /// Clear TX_OVER Interrupt Register
    pub ic_clr_tx_over: IcClrTxOver,
    /// 0x0050
    /// Clear RD_REQ Interrupt Register
    pub ic_clr_rd_req: IcClrRdReq,
    /// 0x0054
    /// Clear TX_ABRT Interrupt Register
    pub ic_clr_tx_abrt: IcClrTxAbrt,
    /// 0x0058
    /// Clear RX_DONE Interrupt Register
    pub ic_clr_rx_done: IcClrRxDone,
    /// 0x005c
    /// Clear ACTIVITY Interrupt Register
    pub ic_clr_activity: IcClrActivity,
    /// 0x0060
    /// Clear STOP_DET Interrupt Register
    pub ic_clr_stop_det: IcClrStopDet,
    /// 0x0064
    /// Clear START_DET Interrupt Register
    pub ic_clr_start_det: IcClrStartDet,
    /// 0x0068
    /// Clear GEN_CALL Interrupt Register
    pub ic_clr_gen_call: IcClrGenCall,
    /// 0x006c
    /// I2C Enable Register
    pub ic_enable: IcEnable,
    /// 0x0070
    /// I2C Status Register
    /// This is a read-only register used to indicate the current transfer status and FIFO status. The status register may be read at any time. None of the bits in this register request an interrupt.
    /// When the I2C is disabled by writing 0 in bit 0 of the IC_ENABLE register: - Bits 1 and 2 are set to 1 - Bits 3 and 10 are set to 0 When the master or slave state machines goes to idle and ic_en=0: - Bits 5 and 6 are set to 0
    pub ic_status: IcStatus,
    /// 0x0074
    /// I2C Transmit FIFO Level Register This register contains the number of valid data entries in the transmit FIFO buffer. It is cleared whenever: - The I2C is disabled - There is a transmit abort - that is, TX_ABRT bit is set in the IC_RAW_INTR_STAT register - The slave bulk transmit mode is aborted The register increments whenever data is placed into the transmit FIFO and decrements when data is taken from the transmit FIFO.
    pub ic_txflr: IcTxflr,
    /// 0x0078
    /// I2C Receive FIFO Level Register This register contains the number of valid data entries in the receive FIFO buffer. It is cleared whenever: - The I2C is disabled - Whenever there is a transmit abort caused by any of the events tracked in IC_TX_ABRT_SOURCE The register increments whenever data is placed into the receive FIFO and decrements when data is taken from the receive FIFO.
    pub ic_rxflr: IcRxflr,
    /// 0x007c
    /// I2C SDA Hold Time Length Register
    /// The bits [15:0] of this register are used to control the hold time of SDA during transmit in both slave and master mode (after SCL goes from HIGH to LOW).
    /// The bits [23:16] of this register are used to extend the SDA transition (if any) whenever SCL is HIGH in the receiver in either master or slave mode.
    /// Writes to this register succeed only when IC_ENABLE[0]=0.
    /// The values in this register are in units of ic_clk period. The value programmed in IC_SDA_TX_HOLD must be greater than the minimum hold time in each mode (one cycle in master mode, seven cycles in slave mode) for the value to be implemented.
    /// The programmed SDA hold time during transmit (IC_SDA_TX_HOLD) cannot exceed at any time the duration of the low part of scl. Therefore the programmed value cannot be larger than N_SCL_LOW-2, where N_SCL_LOW is the duration of the low part of the scl period measured in ic_clk cycles.
    pub ic_sda_hold: IcSdaHold,
    /// 0x0080
    /// I2C Transmit Abort Source Register
    /// This register has 32 bits that indicate the source of the TX_ABRT bit. Except for Bit 9, this register is cleared whenever the IC_CLR_TX_ABRT register or the IC_CLR_INTR register is read. To clear Bit 9, the source of the ABRT_SBYTE_NORSTRT must be fixed first; RESTART must be enabled (IC_CON[5]=1), the SPECIAL bit must be cleared (IC_TAR[11]), or the GC_OR_START bit must be cleared (IC_TAR[10]).
    /// Once the source of the ABRT_SBYTE_NORSTRT is fixed, then this bit can be cleared in the same manner as other bits in this register. If the source of the ABRT_SBYTE_NORSTRT is not fixed before attempting to clear this bit, Bit 9 clears for one cycle and is then re-asserted.
    pub ic_tx_abrt_source: IcTxAbrtSource,
    /// 0x0084
    /// Generate Slave Data NACK Register
    /// The register is used to generate a NACK for the data part of a transfer when DW_apb_i2c is acting as a slave-receiver. This register only exists when the IC_SLV_DATA_NACK_ONLY parameter is set to 1. When this parameter disabled, this register does not exist and writing to the register's address has no effect.
    /// A write can occur on this register if both of the following conditions are met: - DW_apb_i2c is disabled (IC_ENABLE[0] = 0) - Slave part is inactive (IC_STATUS[6] = 0) Note: The IC_STATUS[6] is a register read-back location for the internal slv_activity signal; the user should poll this before writing the ic_slv_data_nack_only bit.
    pub ic_slv_data_nack_only: IcSlvDataNackOnly,
    /// 0x0088
    /// DMA Control Register
    /// The register is used to enable the DMA Controller interface operation. There is a separate bit for transmit and receive. This can be programmed regardless of the state of IC_ENABLE.
    pub ic_dma_cr: IcDmaCr,
    /// 0x008c
    /// DMA Transmit Data Level Register
    pub ic_dma_tdlr: IcDmaTdlr,
    /// 0x0090
    /// I2C Receive Data Level Register
    pub ic_dma_rdlr: IcDmaRdlr,
    /// 0x0094
    /// I2C SDA Setup Register
    /// This register controls the amount of time delay (in terms of number of ic_clk clock periods) introduced in the rising edge of SCL - relative to SDA changing - when DW_apb_i2c services a read request in a slave-transmitter operation. The relevant I2C requirement is tSU:DAT (note 4) as detailed in the I2C Bus Specification. This register must be programmed with a value equal to or greater than 2.
    /// Writes to this register succeed only when IC_ENABLE[0] = 0.
    /// Note: The length of setup time is calculated using [(IC_SDA_SETUP - 1) * (ic_clk_period)], so if the user requires 10 ic_clk periods of setup time, they should program a value of 11. The IC_SDA_SETUP register is only used by the DW_apb_i2c when operating as a slave transmitter.
    pub ic_sda_setup: IcSdaSetup,
    /// 0x0098
    /// I2C ACK General Call Register
    /// The register controls whether DW_apb_i2c responds with a ACK or NACK when it receives an I2C General Call address.
    /// This register is applicable only when the DW_apb_i2c is in slave mode.
    pub ic_ack_general_call: IcAckGeneralCall,
    /// 0x009c
    /// I2C Enable Status Register
    /// The register is used to report the DW_apb_i2c hardware status when the IC_ENABLE[0] register is set from 1 to 0; that is, when DW_apb_i2c is disabled.
    /// If IC_ENABLE[0] has been set to 1, bits 2:1 are forced to 0, and bit 0 is forced to 1.
    /// If IC_ENABLE[0] has been set to 0, bits 2:1 is only be valid as soon as bit 0 is read as '0'.
    /// Note: When IC_ENABLE[0] has been set to 0, a delay occurs for bit 0 to be read as 0 because disabling the DW_apb_i2c depends on I2C bus activities.
    pub ic_enable_status: IcEnableStatus,
    /// 0x00a0
    /// I2C SS, FS or FM+ spike suppression limit
    /// This register is used to store the duration, measured in ic_clk cycles, of the longest spike that is filtered out by the spike suppression logic when the component is operating in SS, FS or FM+ modes. The relevant I2C requirement is tSP (table 4) as detailed in the I2C Bus Specification. This register must be programmed with a minimum value of 1.
    pub ic_fs_spklen: IcFsSpklen,
    _reserved2: [u8; 4],
    /// 0x00a8
    /// Clear RESTART_DET Interrupt Register
    pub ic_clr_restart_det: IcClrRestartDet,
    _reserved3: [u8; 72],
    /// 0x00f4
    /// Component Parameter Register 1
    /// Note This register is not implemented and therefore reads as 0. If it was implemented it would be a constant read-only register that contains encoded information about the component's parameter settings. Fields shown below are the settings for those parameters
    pub ic_comp_param_1: IcCompParam1,
    /// 0x00f8
    /// I2C Component Version Register
    pub ic_comp_version: IcCompVersion,
    /// 0x00fc
    /// I2C Component Type Register
    pub ic_comp_type: IcCompType,
}
