//! System information read through the bootrom
//!
//! The bootrom's `get_sys_info` call (tag `GS`) fills a buffer with a block of
//! related values chosen by a flag. This module wraps that call with typed
//! accessors for the blocks the HAL exposes: the chip identifier, the OTP
//! critical register, the running CPU architecture, the QSPI device
//! information, the per-boot random value and the boot diagnostics.
//!
//! The raw call is [`crate::bootrom::get_sys_info`]; this module only decodes
//! it. For the other half of the picture, the `SYSINFO` peripheral registers,
//! see [`crate::sysinfo`].
//!
//! ## Example
//!
//! ```ignore
//! use rp2350::sysinfo_rom;
//!
//! // Size of the flash device holding the boot image.
//! let flash_bytes = sysinfo_rom::flash_device_info().unwrap().cs0_size_bytes();
//!
//! // Which path did the chip take on this boot?
//! let boot = sysinfo_rom::boot_info().unwrap();
//! let _ = boot.boot_type();
//! ```
//!
//! ## Notes
//!
//! Every accessor resolves the Secure ARM entry first and falls back to the
//! Non-secure one, so it works from either security state. A block the resident
//! bootrom does not implement is [`Error::Unsupported`]; a call it rejects is
//! [`Error::Rom`].

use crate::bootrom::{self, BootRomError};

/// Error from a `get_sys_info` call.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Error {
    /// The resident bootrom does not provide this information.
    Unsupported,
    /// The bootrom rejected the call.
    Rom(BootRomError),
}

impl From<BootRomError> for Error {
    fn from(value: BootRomError) -> Self {
        Self::Rom(value)
    }
}

// Flag values from RP2350 datasheet §5.5.11.1. The bootrom also has a `NONCE`
// flag (0x0020), which application code has no use for.
const FLAG_CHIP_INFO: u32 = 0x0001;
const FLAG_CRITICAL: u32 = 0x0002;
const FLAG_CPU_INFO: u32 = 0x0004;
const FLAG_FLASH_DEVICE_INFO: u32 = 0x0008;
const FLAG_BOOT_RANDOM: u32 = 0x0010;
const FLAG_BOOT_INFO: u32 = 0x0040;

/// Read the block selected by `flag` into `out` and return its payload.
///
/// The first word the bootrom writes reports which flags it satisfied, so the
/// payload starts at index 1.
fn block(flag: u32, out: &mut [u32]) -> Result<&[u32], Error> {
    let words = bootrom::get_sys_info(out, flag);
    if words < 0 {
        return Err(Error::Rom(BootRomError::from(words)));
    }
    if words == 0 || out[0] & flag == 0 {
        return Err(Error::Unsupported);
    }

    // The bootrom will not write more words than the buffer it was given, but
    // clamp anyway so a stray count cannot panic.
    let end = (words as usize).min(out.len());
    Ok(&out[1..end])
}

/// Unique identifier for the chip.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ChipInfo {
    package_sel: u32,
    device_id: u32,
    wafer_id: u32,
}

impl ChipInfo {
    /// Value of the `CHIP_INFO_PACKAGE_SEL` register.
    ///
    /// [`crate::sysinfo::Sysinfo`] reads the `PACKAGE_SEL` bit straight from
    /// the peripheral, which is the authoritative source for the chip variant.
    pub fn package_sel(&self) -> u32 {
        self.package_sel
    }

    /// Wafer-level device identifier.
    pub fn device_id(&self) -> u32 {
        self.device_id
    }

    /// Wafer identifier.
    pub fn wafer_id(&self) -> u32 {
        self.wafer_id
    }
}

/// Unique identifiers for this chip.
pub fn chip_info() -> Result<ChipInfo, Error> {
    let mut words = [0u32; 4];
    let info = block(FLAG_CHIP_INFO, &mut words)?;
    if info.len() < 3 {
        return Err(Error::Unsupported);
    }

    Ok(ChipInfo {
        package_sel: info[0],
        device_id: info[1],
        wafer_id: info[2],
    })
}

/// Architecture a core is running.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CpuArch {
    /// Arm (Cortex-M33).
    Arm,
    /// RISC-V (Hazard3).
    RiscV,
}

impl From<u32> for CpuArch {
    fn from(value: u32) -> Self {
        if value == 0 { Self::Arm } else { Self::RiscV }
    }
}

/// Architecture the calling core is running.
pub fn cpu_arch() -> Result<CpuArch, Error> {
    let mut words = [0u32; 2];
    let payload = block(FLAG_CPU_INFO, &mut words)?;

    payload
        .first()
        .copied()
        .map(CpuArch::from)
        .ok_or(Error::Unsupported)
}

/// The OTP critical register, as the bootrom latched it at startup.
///
/// These bits decide whether secure boot is enforced and which architecture
/// starts. [`crate::otp::Otp`] covers the OTP data the HAL exposes directly.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct CriticalRegister(u32);

impl CriticalRegister {
    /// The raw register value.
    pub fn raw(&self) -> u32 {
        self.0
    }

    /// Secure boot is enforced, so only signed images run.
    pub fn secure_boot_enabled(&self) -> bool {
        self.0 & 0x1 != 0
    }

    /// Debugging the secure world is disabled.
    pub fn secure_debug_disabled(&self) -> bool {
        self.0 & 0x2 != 0
    }

    /// Debugging is disabled entirely.
    pub fn debug_disabled(&self) -> bool {
        self.0 & 0x4 != 0
    }

    /// Architecture selected in OTP. Ignored when secure boot is on or the
    /// selected architecture is disabled.
    pub fn default_arch(&self) -> CpuArch {
        CpuArch::from((self.0 >> 3) & 1)
    }

    /// The glitch detector is enabled.
    pub fn glitch_detector_enabled(&self) -> bool {
        self.0 & 0x10 != 0
    }

    /// Sensitivity of the glitch detector, 0 to 3.
    pub fn glitch_detector_sensitivity(&self) -> u8 {
        ((self.0 >> 5) & 0x3) as u8
    }

    /// The Arm architecture is disabled.
    pub fn arm_disabled(&self) -> bool {
        self.0 & 0x1_0000 != 0
    }

    /// The RISC-V architecture is disabled.
    pub fn risc_disabled(&self) -> bool {
        self.0 & 0x2_0000 != 0
    }
}

/// The OTP critical register as latched at boot.
pub fn critical_register() -> Result<CriticalRegister, Error> {
    let mut words = [0u32; 2];
    let payload = block(FLAG_CRITICAL, &mut words)?;

    payload
        .first()
        .copied()
        .map(CriticalRegister)
        .ok_or(Error::Unsupported)
}

/// The QSPI devices the bootrom saw, from the OTP `FLASH_DEVINFO` entry.
///
/// The bootrom read this once at startup. It covers both flash and PSRAM,
/// since the bootrom does not distinguish them.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct FlashDeviceInfo(u32);

impl FlashDeviceInfo {
    /// The raw 16-bit `FLASH_DEVINFO` value.
    pub fn raw(&self) -> u16 {
        self.0 as u16
    }

    /// Size in bytes of the device on chip select 0, which holds the boot
    /// image, or `None` when the field says there is none.
    pub fn cs0_size_bytes(&self) -> Option<u32> {
        device_size((self.0 >> 8) & 0xf)
    }

    /// Size in bytes of the device on chip select 1, if one is attached.
    pub fn cs1_size_bytes(&self) -> Option<u32> {
        device_size((self.0 >> 12) & 0xf)
    }

    /// GPIO used as the secondary chip select, or `None` when CS1 has no
    /// device.
    ///
    /// On RP2350 the permissible numbers are 0, 8, 19 and 47. The bootrom
    /// configures the pin itself when the flash boot path runs.
    pub fn cs1_gpio(&self) -> Option<u8> {
        self.cs1_size_bytes()?;

        Some((self.0 & 0x3f) as u8)
    }

    /// Every attached device supports, or ignores, the `D8h` 64 KiB block
    /// erase command that the bootrom uses to speed up bulk erase.
    pub fn d8h_erase_supported(&self) -> bool {
        self.0 & 0x80 != 0
    }
}

/// Decode a `FLASH_DEVINFO` size field: `4 KiB << code`, where code 0 means no
/// device. Codes above `0xc` are not defined.
fn device_size(code: u32) -> Option<u32> {
    if code == 0 || code > 0xc {
        return None;
    }

    Some((4 * 1024) << code)
}

/// Sizes and wiring of the QSPI devices, from OTP.
pub fn flash_device_info() -> Result<FlashDeviceInfo, Error> {
    let mut words = [0u32; 2];
    let payload = block(FLAG_FLASH_DEVICE_INFO, &mut words)?;

    payload
        .first()
        .copied()
        .map(FlashDeviceInfo)
        .ok_or(Error::Unsupported)
}

/// The 128-bit random value the bootrom generates on each boot.
///
/// This is fresh entropy that needs no peripheral setup, but the bootrom does
/// not promise it is uniformly random or cryptographically strong, so it is not
/// suitable as a secret. Use [`crate::trng::Trng`] when it must be.
pub fn boot_random() -> Result<u128, Error> {
    let mut words = [0u32; 5];
    let payload = block(FLAG_BOOT_RANDOM, &mut words)?;
    if payload.len() < 4 {
        return Err(Error::Unsupported);
    }

    let mut value = 0u128;
    for word in &payload[..4] {
        value = (value << 32) | *word as u128;
    }

    Ok(value)
}

/// Partition the boot diagnostics describe.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DiagnosticPartition {
    /// A numbered partition.
    Partition(u8),
    /// No partition.
    None,
    /// Slot 0 at the start of flash.
    Slot0,
    /// Slot 1 at the start of flash.
    Slot1,
    /// The RAM/XIP window.
    Window,
    /// A value this HAL does not know.
    Unknown,
}

impl From<i8> for DiagnosticPartition {
    fn from(value: i8) -> Self {
        match value {
            -1 => Self::None,
            -2 => Self::Slot0,
            -3 => Self::Slot1,
            -4 => Self::Window,
            0..=15 => Self::Partition(value as u8),
            _ => Self::Unknown,
        }
    }
}

/// Boot path the chip took.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BootType {
    /// Normal boot through the partition table rules.
    Normal,
    /// BOOTSEL, exposing the USB boot interfaces.
    BootSel,
    /// A RAM image was found and launched.
    RamImage,
    /// Flash update boot, preferring a just-updated partition.
    FlashUpdate,
    /// A direct PC/SP jump.
    PcSp,
    /// A value this HAL does not know.
    Unknown,
}

impl From<u8> for BootType {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Normal,
            2 => Self::BootSel,
            3 => Self::RamImage,
            4 => Self::FlashUpdate,
            0xd => Self::PcSp,
            _ => Self::Unknown,
        }
    }
}

/// A bit in [`BootInfo::diagnostic_raw`] (RP2350 datasheet `BOOT_DIAGNOSTIC_*`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u16)]
pub enum DiagnosticFlag {
    /// The region was searched for a block loop.
    RegionSearched = 0x0001,
    /// A block loop was found but is structurally invalid.
    InvalidBlockLoop = 0x0002,
    /// A structurally valid block loop was found.
    ValidBlockLoop = 0x0004,
    /// A valid, executable `IMAGE_DEF` was found in the region.
    ValidImageDef = 0x0008,
    /// A partition table is present.
    HasPartitionTable = 0x0010,
    /// This slot or partition was considered.
    Considered = 0x0020,
    /// This slot or partition was chosen.
    Chosen = 0x0040,
    /// The partition table is signed with an OTP-enrolled key.
    PartitionTableMatchingKeyForVerify = 0x0080,
    /// A partition table hash check could be performed.
    PartitionTableHashForVerify = 0x0100,
    /// The partition table passed verification.
    PartitionTableVerifiedOk = 0x0200,
    /// The image definition is signed with an OTP-enrolled key.
    ImageDefMatchingKeyForVerify = 0x0400,
    /// An image definition hash check could be performed.
    ImageDefHashForVerify = 0x0800,
    /// The image definition passed verification and its load map is valid.
    ImageDefVerifiedOk = 0x1000,
    /// Code was copied into RAM through a load map.
    LoadMapEntriesLoaded = 0x2000,
    /// The image from this region was launched.
    ImageLaunched = 0x4000,
    /// The image failed its final checks and was not launched.
    ImageConditionFailure = 0x8000,
}

/// Diagnostic information about the most recent boot.
///
/// It survives a watchdog reset, which is how a failed boot is diagnosed
/// afterwards, but not a RUN pin or POWMAN reset.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct BootInfo {
    boot_type: BootType,
    chained: bool,
    partition: i8,
    diagnostic_partition: DiagnosticPartition,
    tbyb_update: u8,
    diagnostic: u32,
    reboot_params: [u32; 2],
}

impl BootInfo {
    /// Boot path the chip took.
    pub fn boot_type(&self) -> BootType {
        self.boot_type
    }

    /// A bootloader chained into this image.
    pub fn chained(&self) -> bool {
        self.chained
    }

    /// Partition the boot came from, as the bootrom encodes it.
    pub fn partition(&self) -> i8 {
        self.partition
    }

    /// Partition the diagnostics describe.
    pub fn diagnostic_partition(&self) -> DiagnosticPartition {
        self.diagnostic_partition
    }

    /// Try-Before-You-Buy and update progress bits (`BOOT_TBYB_AND_UPDATE_*`).
    pub fn tbyb_update(&self) -> u8 {
        self.tbyb_update
    }

    /// The raw diagnostic word.
    ///
    /// The low half word covers slot 0 or partition A, the high half word
    /// slot 1 or partition B.
    pub fn diagnostic_raw(&self) -> u32 {
        self.diagnostic
    }

    /// Parameters passed to a `RAM_IMAGE`, `FLASH_UPDATE` or `PC_SP` reboot.
    pub fn reboot_params(&self) -> [u32; 2] {
        self.reboot_params
    }

    /// Whether `flag` is set for slot 0 / partition A.
    pub fn slot0(&self, flag: DiagnosticFlag) -> bool {
        self.diagnostic as u16 & flag as u16 != 0
    }

    /// Whether `flag` is set for slot 1 / partition B.
    pub fn slot1(&self, flag: DiagnosticFlag) -> bool {
        (self.diagnostic >> 16) as u16 & flag as u16 != 0
    }
}

/// Diagnostics for the most recent boot.
pub fn boot_info() -> Result<BootInfo, Error> {
    let mut words = [0u32; 5];
    let payload = block(FLAG_BOOT_INFO, &mut words)?;
    if payload.len() < 4 {
        return Err(Error::Unsupported);
    }

    let boot_word = payload[0];
    let boot_byte = (boot_word >> 8) as u8;

    Ok(BootInfo {
        // The top bit of the boot type byte is the chained flag, so mask it
        // off before decoding the boot path.
        boot_type: BootType::from(boot_byte & 0x7f),
        chained: boot_byte & 0x80 != 0,
        partition: (boot_word >> 16) as u8 as i8,
        diagnostic_partition: DiagnosticPartition::from((boot_word & 0xff) as i8),
        tbyb_update: (boot_word >> 24) as u8,
        diagnostic: payload[1],
        reboot_params: [payload[2], payload[3]],
    })
}
