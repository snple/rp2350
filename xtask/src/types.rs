//! Data types for representing SVD register definitions
#![allow(dead_code)]

/// Access type for a register or field
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Access {
    ReadOnly,
    WriteOnly,
    ReadWrite,
}

impl Access {
    pub fn from_str(s: &str) -> Self {
        match s.trim().to_lowercase().as_str() {
            "read-only" | "ro" => Access::ReadOnly,
            "write-only" | "wo" => Access::WriteOnly,
            "read-write" | "read-writeonce" | "rw" => Access::ReadWrite,
            _ => Access::ReadWrite,
        }
    }

    pub fn rust_type(&self) -> &'static str {
        match self {
            Access::ReadOnly => "RO",
            Access::WriteOnly => "WO",
            Access::ReadWrite => "RW",
        }
    }
}

/// A bit field within a register
#[derive(Debug, Clone)]
pub struct Field {
    pub name: String,
    pub description: String,
    pub msb: u32,
    pub lsb: u32,
    pub access: Access,
    pub enum_values: Vec<EnumValue>,
}

impl Field {
    /// Get the bit mask for this field
    pub fn mask(&self) -> u32 {
        let width = self.msb - self.lsb + 1;
        (((1u64 << width) - 1) as u32) << self.lsb
    }

    /// Get the width of this field in bits
    pub fn width(&self) -> u32 {
        self.msb - self.lsb + 1
    }
}

/// An enumerated value for a field
#[derive(Debug, Clone)]
pub struct EnumValue {
    pub name: String,
    pub value: u32,
    pub description: String,
}

/// A hardware register
#[derive(Debug, Clone)]
pub struct Register {
    pub name: String,
    pub description: String,
    pub offset: u32,
    pub reset_value: Option<u32>,
    pub access: Access,
    pub fields: Vec<Field>,
}

/// An interrupt definition
#[derive(Debug, Clone)]
pub struct Interrupt {
    pub name: String,
    pub value: u32,
    pub description: String,
}

/// A complete peripheral (register block)
#[derive(Debug, Clone)]
pub struct Peripheral {
    pub name: String,
    pub description: String,
    pub base_address: Option<u32>,
    pub derived_from: Option<String>,
    pub registers: Vec<Register>,
    pub interrupts: Vec<Interrupt>,
}

impl Peripheral {
    pub fn module_name(&self) -> String {
        self.name.to_lowercase()
    }
}

/// The complete SVD device
#[derive(Debug)]
pub struct SvdDevice {
    pub name: String,
    pub description: String,
    pub peripherals: Vec<Peripheral>,
}
