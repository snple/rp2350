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
//!
//! let powman = Powman::new();
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

/// Power state (bit mask of enabled domains)
pub type PowerState = u32;

/// Power Management peripheral wrapper
pub struct Powman {
    regs: &'static pico_rs_pac::powman::RegisterBlock,
}

impl Powman {
    /// Create a new Powman instance
    #[inline]
    pub fn new() -> Self {
        Self {
            regs: unsafe { &*POWMAN::PTR },
        }
    }

    // ========================================================================
    // Voltage Regulator (VREG) Control
    // ========================================================================

    /// Set the voltage regulator output voltage
    ///
    /// # Arguments
    /// * `vsel` - Voltage select (0-31). Typical values:
    ///   - 0x0B (11): 1.10V (default)
    ///   - 0x0F (15): 1.30V (for higher clock speeds)
    pub fn set_voltage(&self, vsel: u8) {
        let vsel = vsel.min(31) as u32;
        self.regs.vreg.write_vsel(vsel);
    }

    /// Get the current voltage setting
    #[inline]
    pub fn get_voltage(&self) -> u8 {
        self.regs.vreg.vsel() as u8
    }

    /// Check if voltage regulator output is stable
    #[inline]
    pub fn is_vreg_ok(&self) -> bool {
        self.regs.vreg_sts.vout_ok()
    }

    /// Check if voltage regulator is still starting up
    #[inline]
    pub fn is_vreg_starting(&self) -> bool {
        self.regs.vreg_sts.startup()
    }

    /// Wait for voltage regulator to stabilize
    pub fn wait_vreg_stable(&self) {
        while !self.is_vreg_ok() || self.is_vreg_starting() {
            core::hint::spin_loop();
        }
    }

    /// Set high-temperature protection threshold
    pub fn set_vreg_temp_threshold(&self, threshold: VregTempThreshold) {
        self.regs.vreg_ctrl.write_ht_th(threshold as u32);
    }

    /// Enable or disable voltage limit bypass
    ///
    /// When disabled, voltage is limited to 1.3V max.
    /// When enabled, can go higher but may damage chip if not careful!
    pub fn set_voltage_limit_disabled(&self, disabled: bool) {
        if disabled {
            self.regs.vreg_ctrl.set_disable_voltage_limit();
        } else {
            self.regs.vreg_ctrl.clr_disable_voltage_limit();
        }
    }

    // ========================================================================
    // Brown-Out Detection (BOD)
    // ========================================================================

    /// Enable brown-out detection
    #[inline]
    pub fn enable_bod(&self) {
        self.regs.bod.set_en();
    }

    /// Disable brown-out detection
    #[inline]
    pub fn disable_bod(&self) {
        self.regs.bod.clr_en();
    }

    /// Set brown-out detection threshold
    ///
    /// # Arguments
    /// * `vsel` - Voltage threshold (0-31). Higher = higher threshold.
    pub fn set_bod_threshold(&self, vsel: u8) {
        let vsel = vsel.min(31) as u32;
        self.regs.bod.write_vsel(vsel);
    }

    // ========================================================================
    // Low-Power Oscillator (LPOSC)
    // ========================================================================

    /// Get LPOSC frequency in Hz (approximately 32 kHz)
    pub fn get_lposc_freq(&self) -> u32 {
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
        self.regs
            .lposc_freq_khz_int
            .write_bits(POWMAN_PASSWORD | (freq_khz & 0xFFFF));
        self.regs
            .lposc_freq_khz_frac
            .write_bits(POWMAN_PASSWORD | (freq_khz_frac & 0xFFFF));

        // Enable LPOSC as source using atomic SET alias
        const USE_LPOSC: u32 = 1 << 9; // POWMAN_TIMER_USE_LPOSC
        self.regs.timer.set_bits(POWMAN_PASSWORD | USE_LPOSC);

        if was_running {
            self.timer_start();
            // Wait for source to be active
            const USING_LPOSC: u32 = 1 << 5; // POWMAN_TIMER_USING_LPOSC
            while (self.regs.timer.read_bits() & USING_LPOSC) == 0 {
                core::hint::spin_loop();
            }
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
        self.regs
            .xosc_freq_khz_int
            .write_bits(POWMAN_PASSWORD | (freq_khz & 0xFFFF));
        self.regs
            .xosc_freq_khz_frac
            .write_bits(POWMAN_PASSWORD | (freq_khz_frac & 0xFFFF));

        // Enable XOSC as source using atomic SET alias
        const USE_XOSC: u32 = 1 << 8; // POWMAN_TIMER_USE_XOSC
        self.regs.timer.set_bits(POWMAN_PASSWORD | USE_XOSC);

        if was_running {
            self.timer_start();
            // Wait for source to be active
            const USING_XOSC: u32 = 1 << 4; // POWMAN_TIMER_USING_XOSC
            while (self.regs.timer.read_bits() & USING_XOSC) == 0 {
                core::hint::spin_loop();
            }
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
        self.regs.ext_time_ref.write_bits(POWMAN_PASSWORD | source);

        // Enable GPIO 1kHz as source using atomic SET alias
        const USE_GPIO_1KHZ: u32 = 1 << 10; // POWMAN_TIMER_USE_GPIO_1KHZ
        self.regs.timer.set_bits(POWMAN_PASSWORD | USE_GPIO_1KHZ);

        if was_running {
            self.timer_start();
        }
    }

    /// Check if timer is running
    #[inline]
    pub fn timer_is_running(&self) -> bool {
        (self.regs.timer.read_bits() & 1) != 0 // POWMAN_TIMER_RUN
    }

    /// Start the AON timer
    pub fn timer_start(&self) {
        const RUN: u32 = 1; // POWMAN_TIMER_RUN
        self.regs.timer.set_bits(POWMAN_PASSWORD | RUN);
    }

    /// Stop the AON timer
    pub fn timer_stop(&self) {
        const RUN: u32 = 1; // POWMAN_TIMER_RUN
        self.regs.timer.clr_bits(POWMAN_PASSWORD | RUN);
    }

    /// Get current timer value in milliseconds
    pub fn timer_get_ms(&self) -> u64 {
        // Read upper 32 bits first
        let mut hi = self.regs.read_time_upper.read_bits();
        loop {
            let lo = self.regs.read_time_lower.read_bits();
            let next_hi = self.regs.read_time_upper.read_bits();
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
        self.regs
            .set_time_15to0
            .write_bits(POWMAN_PASSWORD | ((time_ms & 0xFFFF) as u32));
        self.regs
            .set_time_31to16
            .write_bits(POWMAN_PASSWORD | (((time_ms >> 16) & 0xFFFF) as u32));
        self.regs
            .set_time_47to32
            .write_bits(POWMAN_PASSWORD | (((time_ms >> 32) & 0xFFFF) as u32));
        self.regs
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
        self.regs.inte.set_bits(POWMAN_PASSWORD | TIMER_INT);

        // Disable alarm first
        const ALARM_ENAB: u32 = 1 << 2; // POWMAN_TIMER_ALARM_ENAB
        self.regs.timer.clr_bits(POWMAN_PASSWORD | ALARM_ENAB);

        // Set alarm time using PAC write_bits
        self.regs
            .alarm_time_15to0
            .write_bits(POWMAN_PASSWORD | ((alarm_time_ms & 0xFFFF) as u32));
        self.regs
            .alarm_time_31to16
            .write_bits(POWMAN_PASSWORD | (((alarm_time_ms >> 16) & 0xFFFF) as u32));
        self.regs
            .alarm_time_47to32
            .write_bits(POWMAN_PASSWORD | (((alarm_time_ms >> 32) & 0xFFFF) as u32));
        self.regs
            .alarm_time_63to48
            .write_bits(POWMAN_PASSWORD | (((alarm_time_ms >> 48) & 0xFFFF) as u32));

        // Clear any pending alarm
        self.timer_clear_alarm();

        // Enable alarm
        self.regs.timer.set_bits(POWMAN_PASSWORD | ALARM_ENAB);
    }

    /// Disable timer alarm
    pub fn timer_disable_alarm(&self) {
        const TIMER_INT: u32 = 1 << 0;
        const ALARM_ENAB: u32 = 1 << 2;
        // Disable interrupt
        self.regs.inte.clr_bits(POWMAN_PASSWORD | TIMER_INT);
        // Disable alarm
        self.regs.timer.clr_bits(POWMAN_PASSWORD | ALARM_ENAB);
    }

    /// Clear the alarm flag
    pub fn timer_clear_alarm(&self) {
        const ALARM: u32 = 1 << 1; // POWMAN_TIMER_ALARM
        self.regs.timer.clr_bits(POWMAN_PASSWORD | ALARM);
    }

    /// Check if alarm has fired
    #[inline]
    pub fn timer_alarm_fired(&self) -> bool {
        (self.regs.timer.read_bits() & (1 << 1)) != 0
    }

    /// Enable alarm wakeup from low power mode
    pub fn enable_alarm_wakeup_at_ms(&self, alarm_time_ms: u64) {
        self.timer_enable_alarm_at_ms(alarm_time_ms);

        // Enable powerup on alarm
        const PWRUP_ON_ALARM: u32 = 1 << 3; // POWMAN_TIMER_PWRUP_ON_ALARM
        self.regs.timer.set_bits(POWMAN_PASSWORD | PWRUP_ON_ALARM);
    }

    /// Disable alarm wakeup
    pub fn disable_alarm_wakeup(&self) {
        self.timer_disable_alarm();

        const PWRUP_ON_ALARM: u32 = 1 << 3;
        self.regs.timer.clr_bits(POWMAN_PASSWORD | PWRUP_ON_ALARM);
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
                self.regs.pwrup0.write_bits(POWMAN_PASSWORD | value);
                self.regs.pwrup0.clr_bits(POWMAN_PASSWORD | (1 << 7)); // Clear STATUS
                self.regs.pwrup0.set_bits(POWMAN_PASSWORD | 1); // Set ENABLE
            }
            1 => {
                self.regs.pwrup1.write_bits(POWMAN_PASSWORD | value);
                self.regs.pwrup1.clr_bits(POWMAN_PASSWORD | (1 << 7));
                self.regs.pwrup1.set_bits(POWMAN_PASSWORD | 1);
            }
            2 => {
                self.regs.pwrup2.write_bits(POWMAN_PASSWORD | value);
                self.regs.pwrup2.clr_bits(POWMAN_PASSWORD | (1 << 7));
                self.regs.pwrup2.set_bits(POWMAN_PASSWORD | 1);
            }
            3 => {
                self.regs.pwrup3.write_bits(POWMAN_PASSWORD | value);
                self.regs.pwrup3.clr_bits(POWMAN_PASSWORD | (1 << 7));
                self.regs.pwrup3.set_bits(POWMAN_PASSWORD | 1);
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
            0 => self.regs.pwrup0.clr_bits(POWMAN_PASSWORD | 1), // Clear ENABLE
            1 => self.regs.pwrup1.clr_bits(POWMAN_PASSWORD | 1),
            2 => self.regs.pwrup2.clr_bits(POWMAN_PASSWORD | 1),
            3 => self.regs.pwrup3.clr_bits(POWMAN_PASSWORD | 1),
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
    pub fn get_power_state(&self) -> PowerState {
        // State register shows which domains are OFF (inverted)

        !self.regs.state.read_bits() & 0xF
    }

    /// Check if a power domain is on
    pub fn is_domain_on(&self, domain: PowerDomain) -> bool {
        (self.get_power_state() & (1 << domain as u32)) != 0
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
            0 => self.regs.scratch0.write_bits(value),
            1 => self.regs.scratch1.write_bits(value),
            2 => self.regs.scratch2.write_bits(value),
            3 => self.regs.scratch3.write_bits(value),
            4 => self.regs.scratch4.write_bits(value),
            5 => self.regs.scratch5.write_bits(value),
            6 => self.regs.scratch6.write_bits(value),
            7 => self.regs.scratch7.write_bits(value),
            _ => {}
        }
    }

    /// Read from scratch register
    pub fn read_scratch(&self, index: u8) -> u32 {
        match index {
            0 => self.regs.scratch0.read_bits(),
            1 => self.regs.scratch1.read_bits(),
            2 => self.regs.scratch2.read_bits(),
            3 => self.regs.scratch3.read_bits(),
            4 => self.regs.scratch4.read_bits(),
            5 => self.regs.scratch5.read_bits(),
            6 => self.regs.scratch6.read_bits(),
            7 => self.regs.scratch7.read_bits(),
            _ => 0,
        }
    }

    // ========================================================================
    // Interrupts
    // ========================================================================

    /// Check raw timer interrupt
    #[inline]
    pub fn timer_irq_raw(&self) -> bool {
        self.regs.intr.timer()
    }

    /// Check raw VREG output low interrupt
    #[inline]
    pub fn vreg_output_low_irq_raw(&self) -> bool {
        self.regs.intr.vreg_output_low()
    }

    /// Check raw state request ignored interrupt
    #[inline]
    pub fn state_req_ignored_irq_raw(&self) -> bool {
        self.regs.intr.state_req_ignored()
    }

    /// Check raw pwrup while waiting interrupt
    #[inline]
    pub fn pwrup_while_waiting_irq_raw(&self) -> bool {
        self.regs.intr.pwrup_while_waiting()
    }

    /// Check pwrup0 status
    #[inline]
    pub fn pwrup0_status(&self) -> bool {
        self.regs.pwrup0.status()
    }

    /// Check pwrup1 status
    #[inline]
    pub fn pwrup1_status(&self) -> bool {
        self.regs.pwrup1.status()
    }

    /// Check pwrup2 status
    #[inline]
    pub fn pwrup2_status(&self) -> bool {
        self.regs.pwrup2.status()
    }

    /// Check pwrup3 status
    #[inline]
    pub fn pwrup3_status(&self) -> bool {
        self.regs.pwrup3.status()
    }
}

impl Default for Powman {
    fn default() -> Self {
        Self::new()
    }
}
