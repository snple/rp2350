//! Rust code generator v3 - Proper memory-mapped registers with field accessors
//!
//! Generates PAC code where:
//! - Each register is a `#[repr(transparent)]` struct with `UnsafeCell<u32>`
//! - RegisterBlock is a `#[repr(C)]` struct that directly maps to hardware memory
//! - Each register has field accessor methods with proper documentation
//! - Supports atomic SET/CLR/XOR operations via RP2350 hardware aliases

use crate::types::*;
use anyhow::Result;
use heck::{ToShoutySnakeCase, ToSnakeCase, ToUpperCamelCase};
use std::collections::HashMap;
use std::fmt::Write;
use std::fs;
use std::path::Path;

/// Generate all modules
pub fn generate_all(device: &SvdDevice, output_dir: &Path) -> Result<()> {
    fs::create_dir_all(output_dir)?;

    let periph_map: HashMap<_, _> = device
        .peripherals
        .iter()
        .map(|p| (p.name.as_str(), p))
        .collect();

    // Generate common module
    fs::write(output_dir.join("common.rs"), generate_common())?;
    println!("  Generated common.rs");

    // Collect peripherals to generate
    let peripherals: Vec<_> = device
        .peripherals
        .iter()
        .filter(|p| p.base_address.is_some())
        .collect();

    // Generate each peripheral
    for periph in &peripherals {
        let content = generate_peripheral(periph, &periph_map)?;
        let filename = format!("{}.rs", periph.module_name());
        fs::write(output_dir.join(&filename), content)?;
        println!("  Generated {}", filename);
    }

    // Generate mod.rs
    let mod_content = generate_mod_rs(&peripherals);
    fs::write(output_dir.join("mod.rs"), mod_content)?;
    println!("  Generated mod.rs");

    Ok(())
}

fn generate_mod_rs(peripherals: &[&Peripheral]) -> String {
    let mut out = String::new();

    writeln!(out, "//! RP2350 Hardware Register Definitions").unwrap();
    writeln!(out, "//!").unwrap();
    writeln!(
        out,
        "//! Auto-generated from RP2350.svd - DO NOT EDIT MANUALLY"
    )
    .unwrap();
    writeln!(out).unwrap();

    writeln!(out, "#![allow(dead_code)]").unwrap();
    writeln!(out, "#![allow(non_camel_case_types)]").unwrap();
    writeln!(out, "#![allow(clippy::identity_op)]").unwrap();
    writeln!(out).unwrap();

    writeln!(out, "use core::marker::PhantomData;").unwrap();
    writeln!(out, "use core::ops::Deref;").unwrap();
    writeln!(out).unwrap();

    writeln!(out, "pub mod common;").unwrap();
    writeln!(out).unwrap();

    // Collect all unique interrupts
    let mut all_interrupts: Vec<(&Interrupt, &str)> = Vec::new();
    for periph in peripherals {
        for int in &periph.interrupts {
            all_interrupts.push((int, &periph.name));
        }
    }
    all_interrupts.sort_by_key(|(int, _)| int.value);
    all_interrupts.dedup_by_key(|(int, _)| int.value);

    // Generate interrupt handlers
    if !all_interrupts.is_empty() {
        writeln!(
            out,
            "// ============================================================================"
        )
        .unwrap();
        writeln!(out, "// Interrupt Handlers").unwrap();
        writeln!(
            out,
            "// ============================================================================"
        )
        .unwrap();
        writeln!(out).unwrap();

        writeln!(out, "#[cfg(feature = \"rt\")]").unwrap();
        writeln!(out, "unsafe extern \"C\" {{").unwrap();
        for (int, _) in &all_interrupts {
            writeln!(out, "    fn {}();", int.name.to_shouty_snake_case()).unwrap();
        }
        writeln!(out, "}}").unwrap();
        writeln!(out).unwrap();

        writeln!(out, "/// Union type for interrupt vectors").unwrap();
        writeln!(out, "#[doc(hidden)]").unwrap();
        writeln!(out, "#[repr(C)]").unwrap();
        writeln!(out, "pub union Vector {{").unwrap();
        writeln!(out, "    _handler: unsafe extern \"C\" fn(),").unwrap();
        writeln!(out, "    _reserved: u32,").unwrap();
        writeln!(out, "}}").unwrap();
        writeln!(out).unwrap();

        let max_irq = all_interrupts
            .iter()
            .map(|(int, _)| int.value)
            .max()
            .unwrap_or(0);
        writeln!(
            out,
            "/// RP2350 interrupt vector table ({} interrupts)",
            max_irq + 1
        )
        .unwrap();
        writeln!(out, "#[cfg(feature = \"rt\")]").unwrap();
        writeln!(out, "#[doc(hidden)]").unwrap();
        writeln!(
            out,
            "#[unsafe(link_section = \".vector_table.interrupts\")]"
        )
        .unwrap();
        writeln!(out, "#[unsafe(no_mangle)]").unwrap();
        writeln!(
            out,
            "pub static __INTERRUPTS: [Vector; {}] = [",
            max_irq + 1
        )
        .unwrap();

        let int_map: std::collections::HashMap<_, _> = all_interrupts
            .iter()
            .map(|(int, _)| (int.value, int))
            .collect();
        for i in 0..=max_irq {
            if let Some(int) = int_map.get(&i) {
                writeln!(
                    out,
                    "    Vector {{ _handler: {} }},",
                    int.name.to_shouty_snake_case()
                )
                .unwrap();
            } else {
                writeln!(out, "    Vector {{ _reserved: 0 }}, // Reserved").unwrap();
            }
        }
        writeln!(out, "];").unwrap();
        writeln!(out).unwrap();

        // Generate Interrupt enum
        writeln!(out, "/// RP2350 Interrupt enumeration").unwrap();
        writeln!(out, "#[derive(Copy, Clone, Debug, PartialEq, Eq)]").unwrap();
        writeln!(out, "#[repr(u16)]").unwrap();
        writeln!(out, "pub enum Interrupt {{").unwrap();
        for (int, periph_name) in &all_interrupts {
            let desc = if int.description.is_empty() {
                format!("{} interrupt", periph_name)
            } else {
                short_desc(&int.description)
            };
            writeln!(out, "    /// {} - {}", int.value, desc).unwrap();
            writeln!(
                out,
                "    {} = {},",
                int.name.to_shouty_snake_case(),
                int.value
            )
            .unwrap();
        }
        writeln!(out, "}}").unwrap();
        writeln!(out).unwrap();

        writeln!(out, "#[cfg(feature = \"cortex-m\")]").unwrap();
        writeln!(
            out,
            "unsafe impl cortex_m::interrupt::InterruptNumber for Interrupt {{"
        )
        .unwrap();
        writeln!(out, "    #[inline(always)]").unwrap();
        writeln!(out, "    fn number(self) -> u16 {{").unwrap();
        writeln!(out, "        self as u16").unwrap();
        writeln!(out, "    }}").unwrap();
        writeln!(out, "}}").unwrap();
        writeln!(out).unwrap();
    }

    // Generate peripheral structs
    writeln!(
        out,
        "// ============================================================================"
    )
    .unwrap();
    writeln!(out, "// Peripheral Definitions").unwrap();
    writeln!(
        out,
        "// ============================================================================"
    )
    .unwrap();
    writeln!(out).unwrap();

    for periph in peripherals {
        let struct_name = periph.name.to_shouty_snake_case();
        let mod_name = periph.module_name();
        let base_addr = periph.base_address.unwrap();

        if !periph.description.is_empty() {
            writeln!(out, "/// {}", short_desc(&periph.description)).unwrap();
        } else {
            writeln!(out, "/// {} peripheral", periph.name).unwrap();
        }
        writeln!(out, "pub struct {} {{", struct_name).unwrap();
        writeln!(out, "    _marker: PhantomData<*const ()>,").unwrap();
        writeln!(out, "}}").unwrap();
        writeln!(out).unwrap();
        writeln!(out, "unsafe impl Send for {} {{}}", struct_name).unwrap();
        writeln!(out).unwrap();
        writeln!(out, "impl {} {{", struct_name).unwrap();
        writeln!(out, "    /// Pointer to the register block").unwrap();
        writeln!(
            out,
            "    pub const PTR: *const {}::RegisterBlock = {:#010x} as *const _;",
            mod_name, base_addr
        )
        .unwrap();
        writeln!(out).unwrap();
        writeln!(out, "    /// Return the pointer to the register block").unwrap();
        writeln!(out, "    #[inline(always)]").unwrap();
        writeln!(
            out,
            "    pub const fn ptr() -> *const {}::RegisterBlock {{",
            mod_name
        )
        .unwrap();
        writeln!(out, "        Self::PTR").unwrap();
        writeln!(out, "    }}").unwrap();
        writeln!(out).unwrap();
        writeln!(out, "    /// Steal an instance of this peripheral").unwrap();
        writeln!(out, "    ///").unwrap();
        writeln!(out, "    /// # Safety").unwrap();
        writeln!(out, "    ///").unwrap();
        writeln!(
            out,
            "    /// Ensure that the new instance of the peripheral cannot be used in a way"
        )
        .unwrap();
        writeln!(out, "    /// that may race with any existing instances.").unwrap();
        writeln!(out, "    #[inline]").unwrap();
        writeln!(out, "    pub unsafe fn steal() -> Self {{").unwrap();
        writeln!(out, "        Self {{ _marker: PhantomData }}").unwrap();
        writeln!(out, "    }}").unwrap();
        writeln!(out, "}}").unwrap();
        writeln!(out).unwrap();
        writeln!(out, "impl Deref for {} {{", struct_name).unwrap();
        writeln!(out, "    type Target = {}::RegisterBlock;", mod_name).unwrap();
        writeln!(out).unwrap();
        writeln!(out, "    #[inline(always)]").unwrap();
        writeln!(out, "    fn deref(&self) -> &Self::Target {{").unwrap();
        writeln!(out, "        unsafe {{ &*Self::PTR }}").unwrap();
        writeln!(out, "    }}").unwrap();
        writeln!(out, "}}").unwrap();
        writeln!(out).unwrap();
        writeln!(out, "impl core::fmt::Debug for {} {{", struct_name).unwrap();
        writeln!(
            out,
            "    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {{"
        )
        .unwrap();
        writeln!(out, "        f.debug_struct(\"{}\").finish()", struct_name).unwrap();
        writeln!(out, "    }}").unwrap();
        writeln!(out, "}}").unwrap();
        writeln!(out).unwrap();
        if !periph.description.is_empty() {
            writeln!(out, "/// {}", short_desc(&periph.description)).unwrap();
        } else {
            writeln!(out, "/// {} register module", periph.name).unwrap();
        }
        writeln!(out, "pub mod {};", mod_name).unwrap();
        writeln!(out).unwrap();
    }

    // Generate Peripherals struct
    writeln!(
        out,
        "// ============================================================================"
    )
    .unwrap();
    writeln!(out, "// Peripherals - All device peripherals").unwrap();
    writeln!(
        out,
        "// ============================================================================"
    )
    .unwrap();
    writeln!(out).unwrap();

    writeln!(out, "#[unsafe(no_mangle)]").unwrap();
    writeln!(out, "static mut DEVICE_PERIPHERALS: bool = false;").unwrap();
    writeln!(out).unwrap();

    writeln!(out, "/// All the peripherals.").unwrap();
    writeln!(out, "#[allow(non_snake_case)]").unwrap();
    writeln!(out, "pub struct Peripherals {{").unwrap();
    for periph in peripherals {
        let struct_name = periph.name.to_shouty_snake_case();
        if !periph.description.is_empty() {
            writeln!(out, "    /// {}", short_desc(&periph.description)).unwrap();
        } else {
            writeln!(out, "    /// {} peripheral", periph.name).unwrap();
        }
        writeln!(out, "    pub {}: {},", struct_name, struct_name).unwrap();
    }
    writeln!(out, "}}").unwrap();
    writeln!(out).unwrap();

    writeln!(out, "impl Peripherals {{").unwrap();
    writeln!(out, "    /// Returns all the peripherals *once*.").unwrap();
    writeln!(out, "    #[cfg(feature = \"critical-section\")]").unwrap();
    writeln!(out, "    #[inline]").unwrap();
    writeln!(out, "    pub fn take() -> Option<Self> {{").unwrap();
    writeln!(out, "        critical_section::with(|_| {{").unwrap();
    writeln!(out, "            if unsafe {{ DEVICE_PERIPHERALS }} {{").unwrap();
    writeln!(out, "                return None;").unwrap();
    writeln!(out, "            }}").unwrap();
    writeln!(out, "            Some(unsafe {{ Peripherals::steal() }})").unwrap();
    writeln!(out, "        }})").unwrap();
    writeln!(out, "    }}").unwrap();
    writeln!(out).unwrap();

    writeln!(out, "    /// Unchecked version of `Peripherals::take`.").unwrap();
    writeln!(out, "    ///").unwrap();
    writeln!(out, "    /// # Safety").unwrap();
    writeln!(out, "    ///").unwrap();
    writeln!(
        out,
        "    /// Each of the returned peripherals must be used at most once."
    )
    .unwrap();
    writeln!(out, "    #[inline]").unwrap();
    writeln!(out, "    pub unsafe fn steal() -> Self {{").unwrap();
    writeln!(out, "        unsafe {{").unwrap();
    writeln!(out, "            DEVICE_PERIPHERALS = true;").unwrap();
    writeln!(out, "            Peripherals {{").unwrap();
    for periph in peripherals {
        let struct_name = periph.name.to_shouty_snake_case();
        writeln!(
            out,
            "                {}: {}::steal(),",
            struct_name, struct_name
        )
        .unwrap();
    }
    writeln!(out, "            }}").unwrap();
    writeln!(out, "        }}").unwrap();
    writeln!(out, "    }}").unwrap();
    writeln!(out, "}}").unwrap();

    out
}

fn generate_common() -> String {
    r##"//! Common types and utilities for register access

#![allow(dead_code)]

use core::ptr::{read_volatile, write_volatile};

/// Spin until condition is true
#[inline(always)]
pub fn wait_until<F: Fn() -> bool>(cond: F) {
    while !cond() {
        core::hint::spin_loop();
    }
}

/// Spin while condition is true
#[inline(always)]
pub fn wait_while<F: Fn() -> bool>(cond: F) {
    while cond() {
        core::hint::spin_loop();
    }
}

/// Hardware spinlock (SIO has 32 spinlocks)
#[derive(Clone, Copy)]
pub struct Spinlock(u8);

impl Spinlock {
    pub const fn new(index: u8) -> Self {
        Self(index)
    }

    #[inline(always)]
    pub fn try_lock(&self) -> bool {
        let addr = 0xd0000100 + (self.0 as u32) * 4;
        unsafe { read_volatile(addr as *const u32) != 0 }
    }

    #[inline(always)]
    pub fn unlock(&self) {
        let addr = 0xd0000100 + (self.0 as u32) * 4;
        unsafe { write_volatile(addr as *mut u32, 1) }
    }

    #[inline(always)]
    pub fn lock(&self) {
        while !self.try_lock() {
            core::hint::spin_loop();
        }
    }
}
"##
    .to_string()
}

fn generate_peripheral(
    periph: &Peripheral,
    periph_map: &HashMap<&str, &Peripheral>,
) -> Result<String> {
    let registers = if let Some(ref derived) = periph.derived_from {
        if let Some(base) = periph_map.get(derived.as_str()) {
            &base.registers
        } else {
            &periph.registers
        }
    } else {
        &periph.registers
    };

    generate_generic(periph, registers)
}

fn generate_generic(periph: &Peripheral, registers: &[Register]) -> Result<String> {
    let mut out = String::new();

    write_header(&mut out, periph)?;

    // Generate each register struct with field accessors
    for reg in registers {
        write_register_struct(&mut out, reg)?;
        // Generate the value struct for batch read/write operations
        write_register_value_struct(&mut out, reg)?;
    }

    // Generate RegisterBlock
    write_register_block(&mut out, &periph.name, registers)?;

    Ok(out)
}

/// Generate a single register struct with all field accessors
fn write_register_struct(out: &mut String, reg: &Register) -> Result<()> {
    let struct_name = reg.name.to_upper_camel_case();

    // Document the register
    writeln!(out, "/// {} Register", reg.name)?;
    if !reg.description.is_empty() {
        writeln!(out, "///")?;
        for line in wrap_text(&reg.description, 76) {
            writeln!(out, "/// {}", line)?;
        }
    }
    writeln!(out, "///")?;
    writeln!(out, "/// Address offset: {:#06x}", reg.offset)?;
    if let Some(reset_val) = reg.reset_value {
        writeln!(out, "/// Reset value: {:#010x}", reset_val)?;
    }

    // Document fields
    if !reg.fields.is_empty() {
        writeln!(out, "///")?;
        writeln!(out, "/// ## Fields")?;
        writeln!(out, "///")?;
        for field in &reg.fields {
            let access_str = match field.access {
                Access::ReadOnly => "RO",
                Access::WriteOnly => "WO",
                Access::ReadWrite => "RW",
            };
            let bit_range = if field.lsb == field.msb {
                format!("[{}]", field.lsb)
            } else {
                format!("[{}:{}]", field.msb, field.lsb)
            };
            writeln!(
                out,
                "/// - **{}** {} ({}): {}",
                field.name,
                bit_range,
                access_str,
                short_desc(&field.description)
            )?;
        }
    }

    writeln!(out, "#[repr(transparent)]")?;
    writeln!(out, "pub struct {} {{", struct_name)?;
    writeln!(out, "    value: core::cell::UnsafeCell<u32>,")?;
    writeln!(out, "}}")?;
    writeln!(out)?;

    // Implement Send/Sync
    writeln!(out, "unsafe impl Send for {} {{}}", struct_name)?;
    writeln!(out, "unsafe impl Sync for {} {{}}", struct_name)?;
    writeln!(out)?;

    writeln!(out, "impl {} {{", struct_name)?;

    // Generate field constants
    if !reg.fields.is_empty() {
        writeln!(out, "    // ======== Field Constants ========")?;
        writeln!(out)?;
        for field in &reg.fields {
            let field_shouty = field.name.to_shouty_snake_case();

            if !field.description.is_empty() {
                writeln!(
                    out,
                    "    /// {} - {}",
                    field.name,
                    short_desc(&field.description)
                )?;
            }
            writeln!(
                out,
                "    /// Bits [{}:{}], mask {:#010x}",
                field.msb,
                field.lsb,
                field.mask()
            )?;
            writeln!(
                out,
                "    pub const {}_OFFSET: u32 = {};",
                field_shouty, field.lsb
            )?;
            writeln!(
                out,
                "    pub const {}_MASK: u32 = {:#010x};",
                field_shouty,
                field.mask()
            )?;

            // For single-bit fields, add a simple bit constant
            if field.width() == 1 {
                writeln!(
                    out,
                    "    pub const {}: u32 = 1 << {};",
                    field_shouty, field.lsb
                )?;
            }

            // Enum values
            for ev in &field.enum_values {
                let ev_name = ev.name.to_shouty_snake_case();
                if !ev.description.is_empty() {
                    writeln!(
                        out,
                        "    /// {} = {}: {}",
                        ev.name,
                        ev.value,
                        short_desc(&ev.description)
                    )?;
                }
                writeln!(
                    out,
                    "    pub const {}_{}: u32 = {} << {};",
                    field_shouty, ev_name, ev.value, field.lsb
                )?;
            }
            writeln!(out)?;
        }
    }

    // Basic read/write methods
    writeln!(out, "    // ======== Basic Access Methods ========")?;
    writeln!(out)?;

    // read_bits() - raw u32 access for readable registers
    if matches!(reg.access, Access::ReadOnly | Access::ReadWrite) {
        writeln!(out, "    /// Read the raw register value as u32")?;
        writeln!(out, "    #[inline(always)]")?;
        writeln!(out, "    pub fn read_bits(&self) -> u32 {{")?;
        writeln!(
            out,
            "        unsafe {{ core::ptr::read_volatile(self.value.get()) }}"
        )?;
        writeln!(out, "    }}")?;
        writeln!(out)?;
    }

    // write_bits() - raw u32 access for writable registers
    if matches!(reg.access, Access::WriteOnly | Access::ReadWrite) {
        writeln!(out, "    /// Write a raw u32 value to the register")?;
        writeln!(out, "    #[inline(always)]")?;
        writeln!(out, "    pub fn write_bits(&self, value: u32) {{")?;
        writeln!(
            out,
            "        unsafe {{ core::ptr::write_volatile(self.value.get(), value) }}"
        )?;
        writeln!(out, "    }}")?;
        writeln!(out)?;
    }

    // modify_bits() - only for read-write registers
    if matches!(reg.access, Access::ReadWrite) {
        writeln!(out, "    /// Modify the raw register value")?;
        writeln!(out, "    #[inline(always)]")?;
        writeln!(
            out,
            "    pub fn modify_bits<F: FnOnce(u32) -> u32>(&self, f: F) {{"
        )?;
        writeln!(out, "        self.write_bits(f(self.read_bits()));")?;
        writeln!(out, "    }}")?;
        writeln!(out)?;

        // Atomic operations (RP2350 hardware aliases)
        writeln!(
            out,
            "    /// Atomic SET - sets bits that are 1 (hardware alias at +0x2000)"
        )?;
        writeln!(out, "    #[inline(always)]")?;
        writeln!(out, "    pub fn set_bits(&self, bits: u32) {{")?;
        writeln!(out, "        unsafe {{")?;
        writeln!(
            out,
            "            let addr = (self.value.get() as usize + 0x2000) as *mut u32;"
        )?;
        writeln!(out, "            core::ptr::write_volatile(addr, bits);")?;
        writeln!(out, "        }}")?;
        writeln!(out, "    }}")?;
        writeln!(out)?;

        writeln!(
            out,
            "    /// Atomic CLR - clears bits that are 1 (hardware alias at +0x3000)"
        )?;
        writeln!(out, "    #[inline(always)]")?;
        writeln!(out, "    pub fn clr_bits(&self, bits: u32) {{")?;
        writeln!(out, "        unsafe {{")?;
        writeln!(
            out,
            "            let addr = (self.value.get() as usize + 0x3000) as *mut u32;"
        )?;
        writeln!(out, "            core::ptr::write_volatile(addr, bits);")?;
        writeln!(out, "        }}")?;
        writeln!(out, "    }}")?;
        writeln!(out)?;

        writeln!(
            out,
            "    /// Atomic XOR - toggles bits that are 1 (hardware alias at +0x1000)"
        )?;
        writeln!(out, "    #[inline(always)]")?;
        writeln!(out, "    pub fn xor_bits(&self, bits: u32) {{")?;
        writeln!(out, "        unsafe {{")?;
        writeln!(
            out,
            "            let addr = (self.value.get() as usize + 0x1000) as *mut u32;"
        )?;
        writeln!(out, "            core::ptr::write_volatile(addr, bits);")?;
        writeln!(out, "        }}")?;
        writeln!(out, "    }}")?;
        writeln!(out)?;
    }

    // Generate field accessor methods
    if !reg.fields.is_empty() {
        writeln!(out, "    // ======== Field Accessors ========")?;
        writeln!(out)?;

        for field in &reg.fields {
            let field_snake = safe_snake_case(&field.name);
            let field_shouty = field.name.to_shouty_snake_case();
            let is_readable = matches!(field.access, Access::ReadOnly | Access::ReadWrite);
            let width = field.width();

            // Document the field
            if !field.description.is_empty() {
                writeln!(
                    out,
                    "    /// **{}** [{}:{}]: {}",
                    field.name,
                    field.msb,
                    field.lsb,
                    short_desc(&field.description)
                )?;
            } else {
                writeln!(
                    out,
                    "    /// **{}** [{}:{}]",
                    field.name, field.msb, field.lsb
                )?;
            }

            // Read accessor (for readable fields)
            if is_readable {
                if width == 1 {
                    // Single bit - return bool
                    writeln!(out, "    #[inline(always)]")?;
                    writeln!(out, "    pub fn {}(&self) -> bool {{", field_snake)?;
                    writeln!(
                        out,
                        "        (self.read_bits() & Self::{}_MASK) != 0",
                        field_shouty
                    )?;
                    writeln!(out, "    }}")?;
                } else {
                    // Multi-bit - return u32
                    writeln!(out, "    #[inline(always)]")?;
                    writeln!(out, "    pub fn {}(&self) -> u32 {{", field_snake)?;
                    writeln!(
                        out,
                        "        (self.read_bits() & Self::{}_MASK) >> Self::{}_OFFSET",
                        field_shouty, field_shouty
                    )?;
                    writeln!(out, "    }}")?;
                }
                writeln!(out)?;
            }

            // Write accessor (for RW fields in RW register, but NOT for WriteOnly fields)
            // WriteOnly fields are handled separately below
            if matches!(field.access, Access::ReadWrite) && matches!(reg.access, Access::ReadWrite)
            {
                if width == 1 {
                    // Single bit - set/clear methods
                    writeln!(out, "    /// Set {} bit", field.name)?;
                    writeln!(out, "    #[inline(always)]")?;
                    writeln!(
                        out,
                        "    pub fn {}(&self) {{",
                        prefixed_method_name("set", &field.name)
                    )?;
                    writeln!(out, "        self.set_bits(Self::{}_MASK);", field_shouty)?;
                    writeln!(out, "    }}")?;
                    writeln!(out)?;

                    writeln!(out, "    /// Clear {} bit", field.name)?;
                    writeln!(out, "    #[inline(always)]")?;
                    writeln!(
                        out,
                        "    pub fn {}(&self) {{",
                        prefixed_method_name("clr", &field.name)
                    )?;
                    writeln!(out, "        self.clr_bits(Self::{}_MASK);", field_shouty)?;
                    writeln!(out, "    }}")?;
                    writeln!(out)?;
                } else {
                    // Multi-bit - write method
                    writeln!(out, "    /// Write {} field", field.name)?;
                    writeln!(out, "    #[inline(always)]")?;
                    writeln!(
                        out,
                        "    pub fn {}(&self, value: u32) {{",
                        prefixed_method_name("write", &field.name)
                    )?;
                    writeln!(
                        out,
                        "        self.modify_bits(|r| (r & !Self::{}_MASK) | ((value << Self::{}_OFFSET) & Self::{}_MASK));",
                        field_shouty, field_shouty, field_shouty
                    )?;
                    writeln!(out, "    }}")?;
                    writeln!(out)?;
                }
            }

            // WriteOnly fields - direct write (exclusive, not in RW section)
            if matches!(field.access, Access::WriteOnly) {
                if width == 1 {
                    writeln!(out, "    /// Trigger {} (write-only)", field.name)?;
                    writeln!(out, "    #[inline(always)]")?;
                    writeln!(
                        out,
                        "    pub fn {}(&self) {{",
                        prefixed_method_name("trigger", &field.name)
                    )?;
                    writeln!(out, "        self.write_bits(Self::{}_MASK);", field_shouty)?;
                    writeln!(out, "    }}")?;
                    writeln!(out)?;
                } else {
                    writeln!(out, "    /// Write {} field (write-only)", field.name)?;
                    writeln!(out, "    #[inline(always)]")?;
                    writeln!(
                        out,
                        "    pub fn {}(&self, value: u32) {{",
                        prefixed_method_name("write", &field.name)
                    )?;
                    writeln!(
                        out,
                        "        self.write_bits((value << Self::{}_OFFSET) & Self::{}_MASK);",
                        field_shouty, field_shouty
                    )?;
                    writeln!(out, "    }}")?;
                    writeln!(out)?;
                }
            }
        }
    }

    writeln!(out, "}}")?;
    writeln!(out)?;

    Ok(())
}

/// Generate the value struct for batch read/write operations
/// This allows reading/writing the entire register once and manipulating fields in memory
fn write_register_value_struct(out: &mut String, reg: &Register) -> Result<()> {
    let struct_name = reg.name.to_upper_camel_case();
    let val_name = format!("{}Val", struct_name);

    // Only generate for registers with fields
    if reg.fields.is_empty() {
        return Ok(());
    }

    writeln!(
        out,
        "/// {} value type for batch read/modify/write operations",
        reg.name
    )?;
    writeln!(out, "///")?;
    writeln!(
        out,
        "/// Use `{}.read_val()` to read the register into this struct,",
        struct_name.to_lowercase()
    )?;
    writeln!(
        out,
        "/// modify fields with builder methods, then write back with `{}.write_val(val)`.",
        struct_name.to_lowercase()
    )?;
    writeln!(out, "#[derive(Clone, Copy)]")?;
    writeln!(out, "pub struct {} {{", val_name)?;
    writeln!(out, "    bits: u32,")?;
    writeln!(out, "}}")?;
    writeln!(out)?;

    writeln!(out, "impl {} {{", val_name)?;

    // Constructor from raw bits
    writeln!(out, "    /// Create from raw bits")?;
    writeln!(out, "    #[inline(always)]")?;
    writeln!(out, "    pub const fn from_bits(bits: u32) -> Self {{")?;
    writeln!(out, "        Self {{ bits }}")?;
    writeln!(out, "    }}")?;
    writeln!(out)?;

    // Get raw bits
    writeln!(out, "    /// Get the raw bits")?;
    writeln!(out, "    #[inline(always)]")?;
    writeln!(out, "    pub const fn bits(&self) -> u32 {{")?;
    writeln!(out, "        self.bits")?;
    writeln!(out, "    }}")?;
    writeln!(out)?;

    // Default (reset value or zero)
    // Check if any field is named "reset" to avoid method name collision
    let has_reset_field = reg.fields.iter().any(|f| f.name.to_lowercase() == "reset");

    if let Some(reset_val) = reg.reset_value
        && !has_reset_field
    {
        writeln!(out, "    /// Create with reset value")?;
        writeln!(out, "    #[inline(always)]")?;
        writeln!(out, "    pub const fn reset_val() -> Self {{")?;
        writeln!(out, "        Self {{ bits: {:#010x} }}", reset_val)?;
        writeln!(out, "    }}")?;
        writeln!(out)?;
    }

    // Field reader methods
    for field in &reg.fields {
        let field_snake = safe_snake_case(&field.name);
        let field_shouty = field.name.to_shouty_snake_case();
        let is_readable = matches!(field.access, Access::ReadOnly | Access::ReadWrite);
        let width = field.width();

        if is_readable {
            if !field.description.is_empty() {
                writeln!(
                    out,
                    "    /// Read {} [{}:{}]: {}",
                    field.name,
                    field.msb,
                    field.lsb,
                    short_desc(&field.description)
                )?;
            } else {
                writeln!(
                    out,
                    "    /// Read {} [{}:{}]",
                    field.name, field.msb, field.lsb
                )?;
            }
            writeln!(out, "    #[inline(always)]")?;
            if width == 1 {
                writeln!(out, "    pub const fn {}(&self) -> bool {{", field_snake)?;
                writeln!(
                    out,
                    "        (self.bits & {}::{}_MASK) != 0",
                    struct_name, field_shouty
                )?;
                writeln!(out, "    }}")?;
            } else {
                writeln!(out, "    pub const fn {}(&self) -> u32 {{", field_snake)?;
                writeln!(
                    out,
                    "        (self.bits & {}::{}_MASK) >> {}::{}_OFFSET",
                    struct_name, field_shouty, struct_name, field_shouty
                )?;
                writeln!(out, "    }}")?;
            }
            writeln!(out)?;
        }
    }

    // Field builder methods (for writable fields)
    for field in &reg.fields {
        let field_shouty = field.name.to_shouty_snake_case();
        let is_writable = matches!(field.access, Access::WriteOnly | Access::ReadWrite);
        let width = field.width();
        let with_method = prefixed_method_name("with", &field.name);

        if is_writable {
            if !field.description.is_empty() {
                writeln!(
                    out,
                    "    /// Set {} [{}:{}]: {}",
                    field.name,
                    field.msb,
                    field.lsb,
                    short_desc(&field.description)
                )?;
            } else {
                writeln!(
                    out,
                    "    /// Set {} [{}:{}]",
                    field.name, field.msb, field.lsb
                )?;
            }
            writeln!(out, "    #[inline(always)]")?;
            if width == 1 {
                writeln!(
                    out,
                    "    pub const fn {}(self, value: bool) -> Self {{",
                    with_method
                )?;
                writeln!(out, "        Self {{")?;
                writeln!(out, "            bits: if value {{")?;
                writeln!(
                    out,
                    "                self.bits | {}::{}_MASK",
                    struct_name, field_shouty
                )?;
                writeln!(out, "            }} else {{")?;
                writeln!(
                    out,
                    "                self.bits & !{}::{}_MASK",
                    struct_name, field_shouty
                )?;
                writeln!(out, "            }}")?;
                writeln!(out, "        }}")?;
                writeln!(out, "    }}")?;
            } else {
                writeln!(
                    out,
                    "    pub const fn {}(self, value: u32) -> Self {{",
                    with_method
                )?;
                writeln!(out, "        Self {{")?;
                writeln!(
                    out,
                    "            bits: (self.bits & !{}::{}_MASK) | ((value << {}::{}_OFFSET) & {}::{}_MASK)",
                    struct_name, field_shouty, struct_name, field_shouty, struct_name, field_shouty
                )?;
                writeln!(out, "        }}")?;
                writeln!(out, "    }}")?;
            }
            writeln!(out)?;
        }
    }

    writeln!(out, "}}")?;
    writeln!(out)?;

    // Default impl
    writeln!(out, "impl Default for {} {{", val_name)?;
    writeln!(out, "    fn default() -> Self {{")?;
    if reg.reset_value.is_some() && !has_reset_field {
        writeln!(out, "        Self::reset_val()")?;
    } else if let Some(reset_val) = reg.reset_value {
        writeln!(out, "        Self {{ bits: {:#010x} }}", reset_val)?;
    } else {
        writeln!(out, "        Self {{ bits: 0 }}")?;
    }
    writeln!(out, "    }}")?;
    writeln!(out, "}}")?;
    writeln!(out)?;

    // Add read/write/modify methods that use XxxVal to the register struct
    writeln!(out, "impl {} {{", struct_name)?;

    if matches!(reg.access, Access::ReadOnly | Access::ReadWrite) {
        writeln!(out, "    /// Read the register into a value struct")?;
        writeln!(out, "    #[inline(always)]")?;
        writeln!(out, "    pub fn read(&self) -> {} {{", val_name)?;
        writeln!(out, "        {}::from_bits(self.read_bits())", val_name)?;
        writeln!(out, "    }}")?;
        writeln!(out)?;
    }

    if matches!(reg.access, Access::WriteOnly | Access::ReadWrite) {
        writeln!(out, "    /// Write a value struct to the register")?;
        writeln!(out, "    #[inline(always)]")?;
        writeln!(out, "    pub fn write(&self, val: {}) {{", val_name)?;
        writeln!(out, "        self.write_bits(val.bits())")?;
        writeln!(out, "    }}")?;
        writeln!(out)?;
    }

    if matches!(reg.access, Access::ReadWrite) {
        writeln!(out, "    /// Modify the register using a value struct")?;
        writeln!(out, "    #[inline(always)]")?;
        writeln!(
            out,
            "    pub fn modify<F: FnOnce({}) -> {}>(&self, f: F) {{",
            val_name, val_name
        )?;
        writeln!(out, "        self.write(f(self.read()))")?;
        writeln!(out, "    }}")?;
        writeln!(out)?;
    }

    writeln!(out, "}}")?;
    writeln!(out)?;

    Ok(())
}

/// Generate the RegisterBlock struct
fn write_register_block(out: &mut String, periph_name: &str, registers: &[Register]) -> Result<()> {
    writeln!(out, "/// Register block")?;
    writeln!(out, "#[repr(C)]")?;
    writeln!(out, "pub struct RegisterBlock {{")?;

    let mut sorted_regs: Vec<_> = registers.iter().collect();
    sorted_regs.sort_by_key(|r| r.offset);

    let mut prev_end: u32 = 0;
    let mut padding_idx = 0;

    for reg in &sorted_regs {
        if reg.offset > prev_end {
            let gap = reg.offset - prev_end;
            writeln!(out, "    _reserved{}: [u8; {}],", padding_idx, gap)?;
            padding_idx += 1;
        }

        let field_name = reg.name.to_snake_case();
        let type_name = reg.name.to_upper_camel_case();

        if !reg.description.is_empty() {
            writeln!(
                out,
                "    /// {:#06x} - {}",
                reg.offset,
                short_desc(&reg.description)
            )?;
        } else {
            writeln!(out, "    /// {:#06x}", reg.offset)?;
        }
        writeln!(out, "    pub {}: {},", field_name, type_name)?;

        prev_end = reg.offset + 4;
    }

    writeln!(out, "}}")?;
    writeln!(out)?;

    // Generate helper methods for specific peripherals
    write_peripheral_helpers(out, periph_name)?;

    Ok(())
}

/// Generate peripheral-specific helper methods
fn write_peripheral_helpers(out: &mut String, periph_name: &str) -> Result<()> {
    match periph_name {
        "IO_BANK0" => {
            writeln!(out, "impl RegisterBlock {{")?;
            writeln!(
                out,
                "    /// Get GPIO control register by pin number (0-47)"
            )?;
            writeln!(out, "    ///")?;
            writeln!(out, "    /// # Safety")?;
            writeln!(out, "    /// Pin number must be < 48")?;
            writeln!(out, "    #[inline(always)]")?;
            writeln!(
                out,
                "    pub unsafe fn gpio_ctrl(&self, pin: u8) -> &Gpio0Ctrl {{"
            )?;
            writeln!(out, "        debug_assert!(pin < 48);")?;
            writeln!(
                out,
                "        // GPIO ctrl registers are at offset 0x04 + (pin * 8)"
            )?;
            writeln!(out, "        unsafe {{")?;
            writeln!(
                out,
                "            let ptr = (self as *const Self as *const u8).add(0x04 + (pin as usize) * 8);"
            )?;
            writeln!(out, "            &*(ptr as *const Gpio0Ctrl)")?;
            writeln!(out, "        }}")?;
            writeln!(out, "    }}")?;
            writeln!(out)?;
            writeln!(out, "    /// Get GPIO status register by pin number (0-47)")?;
            writeln!(out, "    ///")?;
            writeln!(out, "    /// # Safety")?;
            writeln!(out, "    /// Pin number must be < 48")?;
            writeln!(out, "    #[inline(always)]")?;
            writeln!(
                out,
                "    pub unsafe fn gpio_status(&self, pin: u8) -> &Gpio0Status {{"
            )?;
            writeln!(out, "        debug_assert!(pin < 48);")?;
            writeln!(
                out,
                "        // GPIO status registers are at offset 0x00 + (pin * 8)"
            )?;
            writeln!(out, "        unsafe {{")?;
            writeln!(
                out,
                "            let ptr = (self as *const Self as *const u8).add((pin as usize) * 8);"
            )?;
            writeln!(out, "            &*(ptr as *const Gpio0Status)")?;
            writeln!(out, "        }}")?;
            writeln!(out, "    }}")?;
            writeln!(out, "}}")?;
            writeln!(out)?;
        }
        "PADS_BANK0" => {
            writeln!(out, "impl RegisterBlock {{")?;
            writeln!(
                out,
                "    /// Get pad control register by GPIO pin number (0-47)"
            )?;
            writeln!(out, "    ///")?;
            writeln!(out, "    /// # Safety")?;
            writeln!(out, "    /// Pin number must be < 48")?;
            writeln!(out, "    #[inline(always)]")?;
            writeln!(out, "    pub unsafe fn gpio(&self, pin: u8) -> &Gpio0 {{")?;
            writeln!(out, "        debug_assert!(pin < 48);")?;
            writeln!(
                out,
                "        // Pad registers are at offset 0x04 + (pin * 4)"
            )?;
            writeln!(out, "        unsafe {{")?;
            writeln!(
                out,
                "            let ptr = (self as *const Self as *const u8).add(0x04 + (pin as usize) * 4);"
            )?;
            writeln!(out, "            &*(ptr as *const Gpio0)")?;
            writeln!(out, "        }}")?;
            writeln!(out, "    }}")?;
            writeln!(out, "}}")?;
            writeln!(out)?;
        }
        "PADS_QSPI" => {
            writeln!(out, "impl RegisterBlock {{")?;
            writeln!(
                out,
                "    /// Get QSPI pad control register by index (0=SCLK, 1=SD0, 2=SD1, 3=SD2, 4=SD3, 5=SS)"
            )?;
            writeln!(out, "    ///")?;
            writeln!(out, "    /// # Safety")?;
            writeln!(out, "    /// Index must be < 6")?;
            writeln!(out, "    #[inline(always)]")?;
            writeln!(
                out,
                "    pub unsafe fn pad(&self, index: u8) -> &GpioQspiSclk {{"
            )?;
            writeln!(out, "        debug_assert!(index < 6);")?;
            writeln!(out, "        unsafe {{")?;
            writeln!(
                out,
                "            let ptr = (self as *const Self as *const u8).add(0x04 + (index as usize) * 4);"
            )?;
            writeln!(out, "            &*(ptr as *const GpioQspiSclk)")?;
            writeln!(out, "        }}")?;
            writeln!(out, "    }}")?;
            writeln!(out, "}}")?;
            writeln!(out)?;
        }
        _ => {}
    }
    Ok(())
}

// ============================================================================
// Helpers
// ============================================================================

const RUST_KEYWORDS: &[&str] = &[
    "as", "break", "const", "continue", "crate", "else", "enum", "extern", "false", "fn", "for",
    "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref", "return",
    "self", "Self", "static", "struct", "super", "trait", "true", "type", "unsafe", "use", "where",
    "while", "async", "await", "dyn", "abstract", "become", "box", "do", "final", "macro",
    "override", "priv", "typeof", "unsized", "virtual", "yield", "try",
];

fn escape_keyword(name: &str) -> String {
    if RUST_KEYWORDS.contains(&name) {
        format!("r#{}", name)
    } else {
        name.to_string()
    }
}

/// Safe snake_case for standalone identifiers (e.g., field reads)
fn safe_snake_case(name: &str) -> String {
    let snake = name.to_snake_case();
    escape_keyword(&snake)
}

/// Build prefixed method name, handling keyword escaping properly
/// For `write_match`, we want `write_match_` to avoid the raw keyword issue
fn prefixed_method_name(prefix: &str, field_name: &str) -> String {
    let base = field_name.to_snake_case();
    let combined = format!("{}_{}", prefix, base);
    // If the base name is a keyword, append underscore rather than using r#
    if RUST_KEYWORDS.contains(&base.as_str()) {
        format!("{}_", combined)
    } else {
        combined
    }
}

fn write_header(out: &mut String, periph: &Peripheral) -> Result<()> {
    writeln!(
        out,
        "//! {} - {}",
        periph.name,
        short_desc(&periph.description)
    )?;
    if periph.description.len() > 60 {
        writeln!(out, "//!")?;
        for line in wrap_text(&periph.description, 76) {
            writeln!(out, "//! {}", line)?;
        }
    }
    writeln!(out)?;
    writeln!(out, "#![allow(dead_code)]")?;
    writeln!(out)?;
    if let Some(addr) = periph.base_address {
        writeln!(out, "pub const BASE: u32 = {:#010x};", addr)?;
        writeln!(out)?;
    }
    Ok(())
}

fn short_desc(s: &str) -> String {
    let s = s.trim().replace('\n', " ");
    let s: String = s.split_whitespace().collect::<Vec<_>>().join(" ");
    if s.len() > 78 {
        format!("{}...", &s[..75])
    } else {
        s
    }
}

fn wrap_text(s: &str, width: usize) -> Vec<String> {
    let s = s.replace('\n', " ");
    let words: Vec<_> = s.split_whitespace().collect();
    let mut lines = Vec::new();
    let mut current = String::new();

    for word in words {
        if current.is_empty() {
            current = word.to_string();
        } else if current.len() + 1 + word.len() <= width {
            current.push(' ');
            current.push_str(word);
        } else {
            lines.push(current);
            current = word.to_string();
        }
    }

    if !current.is_empty() {
        lines.push(current);
    }

    lines
}
