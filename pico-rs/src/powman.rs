//! Power Management (POWMAN) HAL for RP2350
//!
//! This module provides control over the RP2350's power management subsystem,
//! including the voltage regulator, brown-out detection, low-power oscillator,
//! and Always-On (AON) timer.
//!
//! ## Features
//!
//! - Voltage regulator control (VREG)
//! - Brown-out detection (BOD)
//! - Low-power oscillator (LPOSC) configuration
//! - Always-On timer with alarm support
//! - GPIO wakeup from low-power states
//! - Power domain control (SRAM, XIP, switched core)
//!
//! ## Example: AON Timer
//!
//! ```ignore
//! use pico_rs::powman::Powman;
//! use pico_rs_pac::POWMAN;
//!
//! // Get POWMAN peripheral
//! let powman_pac = unsafe { POWMAN::steal() };
//! let powman = Powman::init(powman_pac);
//!
//! // Use XOSC as timer source
//! powman.timer_set_1khz_tick_source_xosc(12_000_000);
//!
//! // Set current time (milliseconds since epoch)
//! powman.timer_set_ms(0);
//!
//! // Start timer
//! powman.timer_start();
//!
//! // Read current time
//! let now_ms = powman.timer_get_ms();
//! ```

use pico_rs_pac::POWMAN;

use crate::Steal;

/// Password required for POWMAN writes (upper 16 bits)
pub const POWMAN_PASSWORD: u32 = 0x5AFE_0000;

/// Power domains that can be controlled
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PowerDomain {
    /// SRAM Bank 1 (top 256K + scratch X/Y)
    SramBank1 = 0,
    /// SRAM Bank 0 (bottom 256K)
    SramBank0 = 1,
    /// XIP Cache (2x 8K)
    XipCache = 2,
    /// Switched core (processors, bus fabric, peripherals)
    SwitchedCore = 3,
}

/// GPIO wakeup trigger mode
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum WakeupMode {
    /// Wake on level
    Level,
    /// Wake on edge
    Edge,
}

/// GPIO wakeup direction
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum WakeupDirection {
    /// Active low / falling edge
    Low,
    /// Active high / rising edge
    High,
}

/// Voltage regulator high-temperature threshold
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum VregTempThreshold {
    /// 100°C threshold
    Temp100C = 0,
    /// 105°C threshold
    Temp105C = 1,
    /// 110°C threshold
    Temp110C = 2,
    /// 115°C threshold
    Temp115C = 3,
    /// 120°C threshold
    Temp120C = 4,
    /// 125°C threshold
    Temp125C = 5,
    /// 135°C threshold
    Temp135C = 6,
    /// 150°C threshold
    Temp150C = 7,
}

/// Core voltage selection.
///
/// The reset default is [`V1_10`] (1.10 V).
///
/// Voltages above [`V1_30`] require calling
/// [`Powman::set_voltage_limit_disabled(true)`](Powman::set_voltage_limit_disabled)
/// first.
///
/// | Voltage | Typical Max Freq |
/// |---------|-----------------|
/// | V0_85   | ~50 MHz |
/// | V1_10   | 150 MHz (default) |
/// | V1_30   | 300 MHz |
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Voltage {
    V0_55 = 0b00000,
    V0_60 = 0b00001,
    V0_65 = 0b00010,
    V0_70 = 0b00011,
    V0_75 = 0b00100,
    V0_80 = 0b00101,
    V0_85 = 0b00110,
    V0_90 = 0b00111,
    V0_95 = 0b01000,
    V1_00 = 0b01001,
    V1_05 = 0b01010,
    /// Reset default
    V1_10 = 0b01011,
    V1_15 = 0b01100,
    V1_20 = 0b01101,
    V1_25 = 0b01110,
    /// Maximum without limit disabled
    V1_30 = 0b01111,
    /// Requires [`Powman::set_voltage_limit_disabled(true)`](Powman::set_voltage_limit_disabled)
    V1_35 = 0b10000,
    V1_40 = 0b10001,
    V1_50 = 0b10010,
    V1_60 = 0b10011,
    V1_65 = 0b10100,
    V1_70 = 0b10101,
    V1_80 = 0b10110,
    V1_90 = 0b10111,
    V2_00 = 0b11000,
    V2_35 = 0b11001,
    V2_50 = 0b11010,
    V2_65 = 0b11011,
    V2_80 = 0b11100,
    V3_00 = 0b11101,
    V3_15 = 0b11110,
    V3_30 = 0b11111,
}

impl Voltage {
    /// Minimum available voltage.
    pub const MIN: Self = Voltage::V0_85;
    /// Reset default.
    pub const DEFAULT: Self = Voltage::V1_10;
    /// Maximum without calling `set_voltage_limit_disabled`.
    pub const MAX: Self = Voltage::V1_30;
}

/// Power state (bit mask of enabled domains)
pub type PowerState = u32;

/// Power Management peripheral wrapper
pub struct Powman {
    powman: POWMAN,
}

impl Powman {
    /// Initialize the Powman driver.
    ///
    /// # Arguments
    /// * `powman` - The POWMAN peripheral from PAC
    #[inline]
    pub fn init(powman: POWMAN) -> Self {
        Self { powman }
    }

    /// Obtain a Powman handle without ownership checks.
    ///
    /// # Safety
    ///
    /// The caller must ensure the peripheral has been initialised
    /// and that no data races occur.
    #[inline]
    pub unsafe fn steal() -> Steal<Self> {
        Steal(Self::init(unsafe { POWMAN::steal() }))
    }

    /// Get reference to the underlying register block
    ///
    /// # Safety
    /// Direct register access can bypass the HAL's safety guarantees.
    /// The caller must ensure operations do not conflict with the HAL state.
    #[inline]
    pub unsafe fn regs(&self) -> &pico_rs_pac::powman::RegisterBlock {
        &self.powman
    }

    /// Release the POWMAN peripheral
    #[inline]
    pub fn release(self) -> POWMAN {
        self.powman
    }

    // ========================================================================
    // Voltage Regulator (VREG) Control
    // ========================================================================

    /// Set the core voltage.
    ///
    /// Unlocks the VREG interface, waits for any in-progress change to finish,
    /// writes the new voltage (with POWMAN password), and waits for the update
    /// to complete.
    ///
    /// # Panics
    ///
    /// Panics if `voltage > `[`Voltage::MAX`] and
    /// [`set_voltage_limit_disabled(true)`](Self::set_voltage_limit_disabled)
    /// has not been called first.
    pub fn set_voltage(&self, voltage: Voltage) {
        // Hardware silently clamps to 1.30 V if limit not disabled
        if voltage as u8 > Voltage::MAX as u8 {
            assert!(
                self.powman.vreg_ctrl.read_bits()
                    & pico_rs_pac::powman::VregCtrl::DISABLE_VOLTAGE_LIMIT
                    != 0,
                "Voltage above 1.30 V requires calling set_voltage_limit_disabled(true) first"
            );
        }

        // Unlock VREG control interface
        self.powman
            .vreg_ctrl
            .write_bits(POWMAN_PASSWORD | pico_rs_pac::powman::VregCtrl::UNLOCK);

        // Wait for any prior update to finish (bounded)
        crate::spin::spin_wait("prior VREG update", || {
            self.powman.vreg.read_bits() & pico_rs_pac::powman::Vreg::UPDATE_IN_PROGRESS == 0
        });

        // Write new voltage with password
        let vsel = (voltage as u32) << pico_rs_pac::powman::Vreg::VSEL_OFFSET;
        self.powman.vreg.write_bits(POWMAN_PASSWORD | vsel);

        // Wait for update to complete (bounded)
        crate::spin::spin_wait("VREG update", || {
            self.powman.vreg.read_bits() & pico_rs_pac::powman::Vreg::UPDATE_IN_PROGRESS == 0
        });
    }

    /// Get the current core voltage setting.
    #[inline]
    pub fn voltage(&self) -> Voltage {
        let raw = self.powman.vreg.vsel() as u8;
        // Safety: VSEL is 5 bits (0–31), all values are valid Voltage variants
        unsafe { core::mem::transmute(raw) }
    }

    /// Check if voltage regulator output is stable
    #[inline]
    pub fn is_vreg_ok(&self) -> bool {
        self.powman.vreg_sts.vout_ok()
    }

    /// Check if voltage regulator is still starting up
    #[inline]
    pub fn is_vreg_starting(&self) -> bool {
        self.powman.vreg_sts.startup()
    }

    /// Wait for voltage regulator to stabilize
    pub fn wait_vreg_stable(&self) {
        crate::spin::spin_wait("VREG stable", || {
            self.is_vreg_ok() && !self.is_vreg_starting()
        });
    }

    /// Set high-temperature protection threshold
    pub fn set_vreg_temp_threshold(&self, threshold: VregTempThreshold) {
        self.powman.vreg_ctrl.write_ht_th(threshold as u32);
    }

    /// Enable or disable voltage limit bypass
    ///
    /// When disabled, voltage is limited to 1.3V max.
    /// When enabled, can go higher but may damage chip if not careful!
    pub fn set_voltage_limit_disabled(&self, disabled: bool) {
        if disabled {
            self.powman.vreg_ctrl.set_disable_voltage_limit();
        } else {
            self.powman.vreg_ctrl.clr_disable_voltage_limit();
        }
    }

    // ========================================================================
    // Brown-Out Detection (BOD)
    // ========================================================================

    /// Enable brown-out detection
    #[inline]
    pub fn enable_bod(&self) {
        self.powman.bod.set_en();
    }

    /// Disable brown-out detection
    #[inline]
    pub fn disable_bod(&self) {
        self.powman.bod.clr_en();
    }

    /// Set brown-out detection threshold
    ///
    /// # Arguments
    /// * `vsel` - Voltage threshold (0-31). Higher = higher threshold.
    pub fn set_bod_threshold(&self, vsel: u8) {
        let vsel = vsel.min(31) as u32;
        self.powman.bod.write_vsel(vsel);
    }

    // ========================================================================
    // Low-Power Oscillator (LPOSC)
    // ========================================================================

    /// Get LPOSC frequency in Hz (approximately 32 kHz)
    pub fn lposc_freq(&self) -> u32 {
        // LPOSC is nominally 32.768 kHz but varies
        32768
    }

    // ========================================================================
    // AON Timer
    // ========================================================================

    /// Set timer 1kHz tick source to LPOSC
    pub fn timer_set_1khz_tick_source_lposc(&self) {
        self.timer_set_1khz_tick_source_lposc_with_hz(32768);
    }

    /// Set timer 1kHz tick source to LPOSC with known frequency
    pub fn timer_set_1khz_tick_source_lposc_with_hz(&self, lposc_freq_hz: u32) {
        let was_running = self.timer_is_running();
        if was_running {
            self.timer_stop();
        }

        let freq_khz = lposc_freq_hz / 1000;
        let freq_khz_frac = (lposc_freq_hz % 1000) * 65536 / 1000;

        // Write frequency info using PAC write_bits (includes UnsafeCell)
        self.powman
            .lposc_freq_khz_int
            .write_bits(POWMAN_PASSWORD | (freq_khz & 0xFFFF));
        self.powman
            .lposc_freq_khz_frac
            .write_bits(POWMAN_PASSWORD | (freq_khz_frac & 0xFFFF));

        // Enable LPOSC as source using atomic SET alias
        const USE_LPOSC: u32 = 1 << 9; // POWMAN_TIMER_USE_LPOSC
        self.powman.timer.set_bits(POWMAN_PASSWORD | USE_LPOSC);

        if was_running {
            self.timer_start();
            // Wait for source to be active (bounded)
            const USING_LPOSC: u32 = 1 << 5; // POWMAN_TIMER_USING_LPOSC
            crate::spin::spin_wait("LPOSC tick source active", || {
                self.powman.timer.read_bits() & USING_LPOSC != 0
            });
        }
    }

    /// Set timer 1kHz tick source to XOSC
    pub fn timer_set_1khz_tick_source_xosc(&self) {
        self.timer_set_1khz_tick_source_xosc_with_hz(12_000_000);
    }

    /// Set timer 1kHz tick source to XOSC with known frequency
    pub fn timer_set_1khz_tick_source_xosc_with_hz(&self, xosc_freq_hz: u32) {
        let was_running = self.timer_is_running();
        if was_running {
            self.timer_stop();
        }

        let freq_khz = xosc_freq_hz / 1000;
        let freq_khz_frac = (xosc_freq_hz % 1000) * 65536 / 1000;

        // Write frequency info using PAC write_bits (includes UnsafeCell)
        self.powman
            .xosc_freq_khz_int
            .write_bits(POWMAN_PASSWORD | (freq_khz & 0xFFFF));
        self.powman
            .xosc_freq_khz_frac
            .write_bits(POWMAN_PASSWORD | (freq_khz_frac & 0xFFFF));

        // Enable XOSC as source using atomic SET alias
        const USE_XOSC: u32 = 1 << 8; // POWMAN_TIMER_USE_XOSC
        self.powman.timer.set_bits(POWMAN_PASSWORD | USE_XOSC);

        if was_running {
            self.timer_start();
            // Wait for source to be active (bounded)
            const USING_XOSC: u32 = 1 << 4; // POWMAN_TIMER_USING_XOSC
            crate::spin::spin_wait("XOSC tick source active", || {
                self.powman.timer.read_bits() & USING_XOSC != 0
            });
        }
    }

    /// Set timer 1kHz tick source from GPIO
    ///
    /// # Arguments
    /// * `gpio` - GPIO number (must be 12, 14, 20, or 22)
    pub fn timer_set_1khz_tick_source_gpio(&self, gpio: u8) {
        let source = match gpio {
            12 => 0,
            20 => 1,
            14 => 2,
            22 => 3,
            _ => return, // Invalid GPIO
        };

        let was_running = self.timer_is_running();
        if was_running {
            self.timer_stop();
        }

        // Set external time reference source
        self.powman
            .ext_time_ref
            .write_bits(POWMAN_PASSWORD | source);

        // Enable GPIO 1kHz as source using atomic SET alias
        const USE_GPIO_1KHZ: u32 = 1 << 10; // POWMAN_TIMER_USE_GPIO_1KHZ
        self.powman.timer.set_bits(POWMAN_PASSWORD | USE_GPIO_1KHZ);

        if was_running {
            self.timer_start();
        }
    }

    /// Check if timer is running
    #[inline]
    pub fn timer_is_running(&self) -> bool {
        (self.powman.timer.read_bits() & 1) != 0 // POWMAN_TIMER_RUN
    }

    /// Start the AON timer
    pub fn timer_start(&self) {
        const RUN: u32 = 1; // POWMAN_TIMER_RUN
        self.powman.timer.set_bits(POWMAN_PASSWORD | RUN);
    }

    /// Stop the AON timer
    pub fn timer_stop(&self) {
        const RUN: u32 = 1; // POWMAN_TIMER_RUN
        self.powman.timer.clr_bits(POWMAN_PASSWORD | RUN);
    }

    /// Get current timer value in milliseconds
    pub fn timer_get_ms(&self) -> u64 {
        // Read upper 32 bits first
        let mut hi = self.powman.read_time_upper.read_bits();
        loop {
            let lo = self.powman.read_time_lower.read_bits();
            let next_hi = self.powman.read_time_upper.read_bits();
            if hi == next_hi {
                return ((hi as u64) << 32) | (lo as u64);
            }
            hi = next_hi;
        }
    }

    /// Set current timer value in milliseconds
    pub fn timer_set_ms(&self, time_ms: u64) {
        let was_running = self.timer_is_running();
        if was_running {
            self.timer_stop();
        }

        // Write time values using PAC write_bits
        self.powman
            .set_time_15to0
            .write_bits(POWMAN_PASSWORD | ((time_ms & 0xFFFF) as u32));
        self.powman
            .set_time_31to16
            .write_bits(POWMAN_PASSWORD | (((time_ms >> 16) & 0xFFFF) as u32));
        self.powman
            .set_time_47to32
            .write_bits(POWMAN_PASSWORD | (((time_ms >> 32) & 0xFFFF) as u32));
        self.powman
            .set_time_63to48
            .write_bits(POWMAN_PASSWORD | (((time_ms >> 48) & 0xFFFF) as u32));

        if was_running {
            self.timer_start();
        }
    }

    /// Enable alarm at a specific time
    pub fn timer_enable_alarm_at_ms(&self, alarm_time_ms: u64) {
        // Enable timer interrupt
        const TIMER_INT: u32 = 1 << 0; // POWMAN_INTE_TIMER
        self.powman.inte.set_bits(POWMAN_PASSWORD | TIMER_INT);

        // Disable alarm first
        const ALARM_ENAB: u32 = 1 << 2; // POWMAN_TIMER_ALARM_ENAB
        self.powman.timer.clr_bits(POWMAN_PASSWORD | ALARM_ENAB);

        // Set alarm time using PAC write_bits
        self.powman
            .alarm_time_15to0
            .write_bits(POWMAN_PASSWORD | ((alarm_time_ms & 0xFFFF) as u32));
        self.powman
            .alarm_time_31to16
            .write_bits(POWMAN_PASSWORD | (((alarm_time_ms >> 16) & 0xFFFF) as u32));
        self.powman
            .alarm_time_47to32
            .write_bits(POWMAN_PASSWORD | (((alarm_time_ms >> 32) & 0xFFFF) as u32));
        self.powman
            .alarm_time_63to48
            .write_bits(POWMAN_PASSWORD | (((alarm_time_ms >> 48) & 0xFFFF) as u32));

        // Clear any pending alarm
        self.timer_clear_alarm();

        // Enable alarm
        self.powman.timer.set_bits(POWMAN_PASSWORD | ALARM_ENAB);
    }

    /// Disable timer alarm
    pub fn timer_disable_alarm(&self) {
        const TIMER_INT: u32 = 1 << 0;
        const ALARM_ENAB: u32 = 1 << 2;
        // Disable interrupt
        self.powman.inte.clr_bits(POWMAN_PASSWORD | TIMER_INT);
        // Disable alarm
        self.powman.timer.clr_bits(POWMAN_PASSWORD | ALARM_ENAB);
    }

    /// Clear the alarm flag
    pub fn timer_clear_alarm(&self) {
        const ALARM: u32 = 1 << 1; // POWMAN_TIMER_ALARM
        self.powman.timer.clr_bits(POWMAN_PASSWORD | ALARM);
    }

    /// Check if alarm has fired
    #[inline]
    pub fn timer_alarm_fired(&self) -> bool {
        (self.powman.timer.read_bits() & (1 << 1)) != 0
    }

    /// Enable alarm wakeup from low power mode
    pub fn enable_alarm_wakeup_at_ms(&self, alarm_time_ms: u64) {
        self.timer_enable_alarm_at_ms(alarm_time_ms);

        // Enable powerup on alarm
        const PWRUP_ON_ALARM: u32 = 1 << 3; // POWMAN_TIMER_PWRUP_ON_ALARM
        self.powman.timer.set_bits(POWMAN_PASSWORD | PWRUP_ON_ALARM);
    }

    /// Disable alarm wakeup
    pub fn disable_alarm_wakeup(&self) {
        self.timer_disable_alarm();

        const PWRUP_ON_ALARM: u32 = 1 << 3;
        self.powman.timer.clr_bits(POWMAN_PASSWORD | PWRUP_ON_ALARM);
    }

    // ========================================================================
    // GPIO Wakeup
    // ========================================================================

    /// Enable GPIO wakeup
    ///
    /// # Arguments
    /// * `wakeup_num` - Wakeup instance (0-3)
    /// * `gpio` - GPIO number (0-47)
    /// * `mode` - Level or edge triggered
    /// * `direction` - Active high/low
    pub fn enable_gpio_wakeup(
        &self,
        wakeup_num: u8,
        gpio: u8,
        mode: WakeupMode,
        direction: WakeupDirection,
    ) {
        if wakeup_num > 3 {
            return;
        }

        // Calculate pwrup register value
        let mode_bits = match mode {
            WakeupMode::Level => 0 << 4,
            WakeupMode::Edge => 1 << 4,
        };
        let direction_bits = match direction {
            WakeupDirection::Low => 0,
            WakeupDirection::High => 1 << 6,
        };
        let source_bits = (gpio as u32) << 8;
        let value = mode_bits | direction_bits | source_bits;

        // Access the pwrup register based on wakeup_num
        match wakeup_num {
            0 => {
                self.powman.pwrup0.write_bits(POWMAN_PASSWORD | value);
                self.powman.pwrup0.clr_bits(POWMAN_PASSWORD | (1 << 7)); // Clear STATUS
                self.powman.pwrup0.set_bits(POWMAN_PASSWORD | 1); // Set ENABLE
            }
            1 => {
                self.powman.pwrup1.write_bits(POWMAN_PASSWORD | value);
                self.powman.pwrup1.clr_bits(POWMAN_PASSWORD | (1 << 7));
                self.powman.pwrup1.set_bits(POWMAN_PASSWORD | 1);
            }
            2 => {
                self.powman.pwrup2.write_bits(POWMAN_PASSWORD | value);
                self.powman.pwrup2.clr_bits(POWMAN_PASSWORD | (1 << 7));
                self.powman.pwrup2.set_bits(POWMAN_PASSWORD | 1);
            }
            3 => {
                self.powman.pwrup3.write_bits(POWMAN_PASSWORD | value);
                self.powman.pwrup3.clr_bits(POWMAN_PASSWORD | (1 << 7));
                self.powman.pwrup3.set_bits(POWMAN_PASSWORD | 1);
            }
            _ => {}
        }
    }

    /// Disable GPIO wakeup
    pub fn disable_gpio_wakeup(&self, wakeup_num: u8) {
        if wakeup_num > 3 {
            return;
        }

        // Access the pwrup register based on wakeup_num
        match wakeup_num {
            0 => self.powman.pwrup0.clr_bits(POWMAN_PASSWORD | 1), // Clear ENABLE
            1 => self.powman.pwrup1.clr_bits(POWMAN_PASSWORD | 1),
            2 => self.powman.pwrup2.clr_bits(POWMAN_PASSWORD | 1),
            3 => self.powman.pwrup3.clr_bits(POWMAN_PASSWORD | 1),
            _ => {}
        }
    }

    /// Disable all wakeup sources
    pub fn disable_all_wakeups(&self) {
        for i in 0..4 {
            self.disable_gpio_wakeup(i);
        }
        self.disable_alarm_wakeup();
    }

    // ========================================================================
    // Power State Control
    // ========================================================================

    /// Get current power state
    pub fn power_state(&self) -> PowerState {
        // State register shows which domains are OFF (inverted)

        !self.powman.state.read_bits() & 0xF
    }

    /// Check if a power domain is on
    pub fn is_domain_on(&self, domain: PowerDomain) -> bool {
        (self.power_state() & (1 << domain as u32)) != 0
    }

    /// Create a power state with domain on
    pub fn state_with_domain_on(state: PowerState, domain: PowerDomain) -> PowerState {
        state | (1 << domain as u32)
    }

    /// Create a power state with domain off
    pub fn state_with_domain_off(state: PowerState, domain: PowerDomain) -> PowerState {
        state & !(1 << domain as u32)
    }

    // ========================================================================
    // Scratch Registers
    // ========================================================================

    /// Write to scratch register (persists in low power mode)
    ///
    /// Note: Scratch registers 0-7 are available
    pub fn write_scratch(&self, index: u8, value: u32) {
        match index {
            0 => self.powman.scratch0.write_bits(value),
            1 => self.powman.scratch1.write_bits(value),
            2 => self.powman.scratch2.write_bits(value),
            3 => self.powman.scratch3.write_bits(value),
            4 => self.powman.scratch4.write_bits(value),
            5 => self.powman.scratch5.write_bits(value),
            6 => self.powman.scratch6.write_bits(value),
            7 => self.powman.scratch7.write_bits(value),
            _ => {}
        }
    }

    /// Read from scratch register
    pub fn read_scratch(&self, index: u8) -> u32 {
        match index {
            0 => self.powman.scratch0.read_bits(),
            1 => self.powman.scratch1.read_bits(),
            2 => self.powman.scratch2.read_bits(),
            3 => self.powman.scratch3.read_bits(),
            4 => self.powman.scratch4.read_bits(),
            5 => self.powman.scratch5.read_bits(),
            6 => self.powman.scratch6.read_bits(),
            7 => self.powman.scratch7.read_bits(),
            _ => 0,
        }
    }

    // ========================================================================
    // Interrupts
    // ========================================================================

    /// Check raw timer interrupt
    #[inline]
    pub fn timer_irq_raw(&self) -> bool {
        self.powman.intr.timer()
    }

    /// Check raw VREG output low interrupt
    #[inline]
    pub fn vreg_output_low_irq_raw(&self) -> bool {
        self.powman.intr.vreg_output_low()
    }

    /// Check raw state request ignored interrupt
    #[inline]
    pub fn state_req_ignored_irq_raw(&self) -> bool {
        self.powman.intr.state_req_ignored()
    }

    /// Check raw pwrup while waiting interrupt
    #[inline]
    pub fn pwrup_while_waiting_irq_raw(&self) -> bool {
        self.powman.intr.pwrup_while_waiting()
    }

    /// Check pwrup0 status
    #[inline]
    pub fn pwrup0_status(&self) -> bool {
        self.powman.pwrup0.status()
    }

    /// Check pwrup1 status
    #[inline]
    pub fn pwrup1_status(&self) -> bool {
        self.powman.pwrup1.status()
    }

    /// Check pwrup2 status
    #[inline]
    pub fn pwrup2_status(&self) -> bool {
        self.powman.pwrup2.status()
    }

    /// Check pwrup3 status
    #[inline]
    pub fn pwrup3_status(&self) -> bool {
        self.powman.pwrup3.status()
    }
}

// ============================================================================
// AON Timer (Always-On Timer)
// ============================================================================

/// Time representation (seconds and nanoseconds since epoch)
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Timespec {
    /// Seconds since epoch
    pub tv_sec: i64,
    /// Nanoseconds (0-999,999,999)
    pub tv_nsec: i64,
}

impl Timespec {
    /// Create a new Timespec
    pub const fn new(secs: i64, nsecs: i64) -> Self {
        Self {
            tv_sec: secs,
            tv_nsec: nsecs,
        }
    }

    /// Create from seconds only
    pub const fn from_secs(secs: i64) -> Self {
        Self {
            tv_sec: secs,
            tv_nsec: 0,
        }
    }

    /// Create from milliseconds
    pub const fn from_millis(millis: i64) -> Self {
        Self {
            tv_sec: millis / 1000,
            tv_nsec: (millis % 1000) * 1_000_000,
        }
    }

    /// Get total seconds
    pub const fn as_secs(&self) -> i64 {
        self.tv_sec
    }

    /// Get total milliseconds
    pub const fn as_millis(&self) -> i64 {
        self.tv_sec * 1000 + self.tv_nsec / 1_000_000
    }

    /// Convert to milliseconds (u64)
    pub fn to_ms(&self) -> u64 {
        let secs = self.tv_sec.max(0) as u64;
        let nsecs = self.tv_nsec.max(0) as u64;
        secs * 1000 + nsecs / 1_000_000
    }

    /// Create from milliseconds (u64)
    pub fn from_ms(ms: u64) -> Self {
        Self {
            tv_sec: (ms / 1000) as i64,
            tv_nsec: ((ms % 1000) * 1_000_000) as i64,
        }
    }
}

/// Clock source for AON timer
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AonClockSource {
    /// Low-power oscillator (~32 kHz)
    Lposc,
    /// Crystal oscillator (accurate)
    Xosc,
    /// External GPIO (1 kHz signal)
    Gpio(u8),
}

/// Always-On Timer
///
/// Wraps the `Powman` driver to provide a higher-level AON timer interface
/// with `Timespec`-based time operations, alarm management, and calendar support.
///
/// ## Example
///
/// ```ignore
/// use pico_rs::powman::{AonTimer, Timespec};
/// use pico_rs_pac::POWMAN;
///
/// let powman_pac = unsafe { POWMAN::steal() };
///
/// // Initialize AON timer with XOSC as source
/// let aon = AonTimer::init_with_xosc(powman_pac, 12_000_000);
///
/// // Set current time
/// aon.set_time(&Timespec::from_secs(0));
///
/// // Get current time
/// let now = aon.get_time();
///
/// // Set an alarm for 10 seconds from now
/// let alarm_time = Timespec::from_secs(now.as_secs() + 10);
/// aon.enable_alarm(&alarm_time, false);
/// ```
pub struct AonTimer {
    powman: Powman,
}

impl AonTimer {
    /// Create a new AON timer instance without configuring a clock source
    ///
    /// # Arguments
    /// * `powman` - The POWMAN peripheral from PAC
    ///
    /// Note: You should call one of the `init_with_*` methods instead,
    /// or configure the clock source manually via `powman()`.
    #[inline]
    pub fn init(powman: POWMAN) -> Self {
        Self {
            powman: Powman::init(powman),
        }
    }

    /// Obtain an AonTimer handle without ownership checks.
    ///
    /// The clock source is not configured. Call one of the `init_with_*`
    /// methods or configure the clock manually before use.
    ///
    /// # Safety
    ///
    /// The caller must ensure the peripheral has been initialised
    /// and that no data races occur.
    #[inline]
    pub unsafe fn steal() -> Steal<Self> {
        Steal(Self::init(unsafe { POWMAN::steal() }))
    }

    /// Initialize AON timer with LPOSC as clock source
    ///
    /// LPOSC is less accurate but available in all power modes.
    pub fn init_with_lposc(powman: POWMAN) -> Self {
        let timer = Self::init(powman);
        timer.powman.timer_set_1khz_tick_source_lposc();
        timer
    }

    /// Initialize AON timer with LPOSC and known frequency
    pub fn init_with_lposc_hz(powman: POWMAN, lposc_freq_hz: u32) -> Self {
        let timer = Self::init(powman);
        timer
            .powman
            .timer_set_1khz_tick_source_lposc_with_hz(lposc_freq_hz);
        timer
    }

    /// Initialize AON timer with XOSC as clock source
    ///
    /// XOSC is more accurate but requires crystal to be running.
    pub fn init_with_xosc(powman: POWMAN, xosc_freq_hz: u32) -> Self {
        let timer = Self::init(powman);
        timer
            .powman
            .timer_set_1khz_tick_source_xosc_with_hz(xosc_freq_hz);
        timer
    }

    /// Initialize AON timer with GPIO as clock source
    ///
    /// # Arguments
    /// * `gpio` - GPIO number (must be 12, 14, 20, or 22)
    pub fn init_with_gpio(powman: POWMAN, gpio: u8) -> Self {
        let timer = Self::init(powman);
        timer.powman.timer_set_1khz_tick_source_gpio(gpio);
        timer
    }

    /// Release the underlying POWMAN peripheral
    #[inline]
    pub fn release(self) -> POWMAN {
        self.powman.release()
    }

    /// Get reference to underlying Powman driver
    #[inline]
    pub fn powman(&self) -> &Powman {
        &self.powman
    }

    // ========================================================================
    // Timer Control
    // ========================================================================

    /// Start the AON timer
    #[inline]
    pub fn start(&self) {
        self.powman.timer_start();
    }

    /// Stop the AON timer
    #[inline]
    pub fn stop(&self) {
        self.powman.timer_stop();
    }

    /// Check if timer is running
    #[inline]
    pub fn is_running(&self) -> bool {
        self.powman.timer_is_running()
    }

    // ========================================================================
    // Time Get/Set
    // ========================================================================

    /// Set the current time
    pub fn set_time(&self, ts: &Timespec) -> bool {
        self.powman.timer_set_ms(ts.to_ms());
        true
    }

    /// Get the current time
    pub fn time(&self) -> Timespec {
        Timespec::from_ms(self.powman.timer_get_ms())
    }

    /// Get the current time in milliseconds
    #[inline]
    pub fn time_ms(&self) -> u64 {
        self.powman.timer_get_ms()
    }

    /// Set the current time in milliseconds
    #[inline]
    pub fn set_time_ms(&self, ms: u64) {
        self.powman.timer_set_ms(ms);
    }

    // ========================================================================
    // Alarm Control
    // ========================================================================

    /// Enable alarm at a specific time
    ///
    /// # Arguments
    /// * `ts` - Time at which alarm should fire
    /// * `wakeup_from_low_power` - If true, will wake system from low-power mode
    pub fn enable_alarm(&self, ts: &Timespec, wakeup_from_low_power: bool) {
        if wakeup_from_low_power {
            self.powman.enable_alarm_wakeup_at_ms(ts.to_ms());
        } else {
            self.powman.timer_enable_alarm_at_ms(ts.to_ms());
        }
    }

    /// Enable alarm at a specific time in milliseconds
    pub fn enable_alarm_at_ms(&self, alarm_time_ms: u64, wakeup_from_low_power: bool) {
        if wakeup_from_low_power {
            self.powman.enable_alarm_wakeup_at_ms(alarm_time_ms);
        } else {
            self.powman.timer_enable_alarm_at_ms(alarm_time_ms);
        }
    }

    /// Disable the alarm
    pub fn disable_alarm(&self) {
        self.powman.timer_disable_alarm();
    }

    /// Check if alarm has fired
    #[inline]
    pub fn alarm_fired(&self) -> bool {
        self.powman.timer_alarm_fired()
    }

    /// Clear the alarm flag
    #[inline]
    pub fn clear_alarm(&self) {
        self.powman.timer_clear_alarm();
    }

    // ========================================================================
    // IRQ Handling
    // ========================================================================

    /// Get the IRQ number for AON timer alarm
    pub const fn irq_num() -> crate::pac::Interrupt {
        crate::pac::Interrupt::POWMAN_IRQ_TIMER
    }

    /// Check if timer interrupt is pending
    #[inline]
    pub fn irq_pending(&self) -> bool {
        self.powman.timer_irq_raw()
    }

    // ========================================================================
    // Convenience Functions
    // ========================================================================

    /// Sleep until the specified time (busy wait)
    ///
    /// Note: This is a busy wait and does not put the system to sleep.
    /// For true low-power sleep, use the alarm wakeup feature.
    pub fn wait_until(&self, ts: &Timespec) {
        let target_ms = ts.to_ms();
        while self.time_ms() < target_ms {
            core::hint::spin_loop();
        }
    }

    /// Delay for the specified duration (busy wait)
    pub fn delay_ms(&self, ms: u64) {
        let target = self.time_ms() + ms;
        while self.time_ms() < target {
            core::hint::spin_loop();
        }
    }
}

/// Convert between calendar time and timespec
pub mod calendar {
    use super::Timespec;

    /// Simple date/time structure
    #[derive(Clone, Copy, Debug, Default)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub struct DateTime {
        /// Year (e.g., 2024)
        pub year: u16,
        /// Month (1-12)
        pub month: u8,
        /// Day of month (1-31)
        pub day: u8,
        /// Hour (0-23)
        pub hour: u8,
        /// Minute (0-59)
        pub minute: u8,
        /// Second (0-59)
        pub second: u8,
        /// Day of week (0=Sunday, 6=Saturday)
        pub day_of_week: u8,
    }

    impl DateTime {
        /// Create a new DateTime
        pub const fn new(year: u16, month: u8, day: u8, hour: u8, minute: u8, second: u8) -> Self {
            Self {
                year,
                month,
                day,
                hour,
                minute,
                second,
                day_of_week: 0,
            }
        }

        /// Convert to seconds since Unix epoch (simplified, no timezone)
        ///
        /// Note: This is a simplified calculation and does not account for
        /// leap seconds or timezone. For accurate time, use a proper RTC library.
        pub fn to_unix_secs(&self) -> i64 {
            // Days from year
            let mut days = 0i64;
            for y in 1970..self.year {
                days += if is_leap_year(y) { 366 } else { 365 };
            }

            // Days from months
            let days_in_months = if is_leap_year(self.year) {
                [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
            } else {
                [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
            };

            for &days_in_month in days_in_months
                .iter()
                .take(self.month.saturating_sub(1) as usize)
            {
                days += days_in_month as i64;
            }

            // Days from day of month
            days += (self.day.saturating_sub(1)) as i64;

            // Convert to seconds
            days * 86400
                + (self.hour as i64) * 3600
                + (self.minute as i64) * 60
                + self.second as i64
        }

        /// Convert from seconds since Unix epoch (simplified)
        pub fn from_unix_secs(secs: i64) -> Self {
            if secs < 0 {
                return Self::default();
            }

            let mut remaining = secs as u64;

            // Extract time
            let second = (remaining % 60) as u8;
            remaining /= 60;
            let minute = (remaining % 60) as u8;
            remaining /= 60;
            let hour = (remaining % 24) as u8;
            remaining /= 24; // Now in days since epoch

            // Calculate day of week (1970-01-01 was Thursday = 4)
            let day_of_week = ((remaining + 4) % 7) as u8;

            // Extract year
            let mut year = 1970u16;
            loop {
                let days_in_year = if is_leap_year(year) { 366 } else { 365 };
                if remaining < days_in_year {
                    break;
                }
                remaining -= days_in_year;
                year += 1;
            }

            // Extract month and day
            let days_in_months = if is_leap_year(year) {
                [31u64, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
            } else {
                [31u64, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
            };

            let mut month = 1u8;
            for days_in_month in days_in_months.iter() {
                if remaining < *days_in_month {
                    break;
                }
                remaining -= *days_in_month;
                month += 1;
            }

            let day = remaining as u8 + 1;

            Self {
                year,
                month,
                day,
                hour,
                minute,
                second,
                day_of_week,
            }
        }

        /// Convert to Timespec
        pub fn to_timespec(&self) -> Timespec {
            Timespec::from_secs(self.to_unix_secs())
        }

        /// Convert from Timespec
        pub fn from_timespec(ts: &Timespec) -> Self {
            Self::from_unix_secs(ts.tv_sec)
        }
    }

    /// Check if year is a leap year
    pub const fn is_leap_year(year: u16) -> bool {
        (year.is_multiple_of(4) && !year.is_multiple_of(100)) || year.is_multiple_of(400)
    }
}
