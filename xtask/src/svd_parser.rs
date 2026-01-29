//! SVD file parser for RP2350
//!
//! Parses the SVD XML file and extracts peripheral, register, and field definitions.

use crate::types::*;
use anyhow::{Context, Result};
use roxmltree::{Document, Node};

/// Parse an SVD file and return all peripherals
pub fn parse_svd(path: &std::path::Path) -> Result<SvdDevice> {
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read SVD file: {:?}", path))?;

    let doc = Document::parse(&content).with_context(|| "Failed to parse SVD XML")?;

    let device_node = doc.root_element();

    let name = get_child_text(&device_node, "name").unwrap_or_default();
    let description = get_child_text(&device_node, "description")
        .map(|s| clean_description(&s))
        .unwrap_or_default();

    let mut peripherals = Vec::new();

    if let Some(peripherals_node) = device_node
        .children()
        .find(|n| n.has_tag_name("peripherals"))
    {
        for periph_node in peripherals_node
            .children()
            .filter(|n| n.has_tag_name("peripheral"))
        {
            match parse_peripheral(&periph_node) {
                Ok(periph) => peripherals.push(periph),
                Err(e) => eprintln!("Warning: Failed to parse peripheral: {}", e),
            }
        }
    }

    Ok(SvdDevice {
        name,
        description,
        peripherals,
    })
}

fn parse_peripheral(node: &Node) -> Result<Peripheral> {
    let name =
        get_child_text(node, "name").ok_or_else(|| anyhow::anyhow!("Peripheral missing name"))?;

    let description = get_child_text(node, "description")
        .map(|s| clean_description(&s))
        .unwrap_or_default();

    let base_address = get_child_text(node, "baseAddress").and_then(|s| parse_number(&s));

    // Check if this peripheral derives from another
    let derived_from = node.attribute("derivedFrom").map(String::from);

    let mut registers = Vec::new();
    let mut interrupts = Vec::new();

    if let Some(registers_node) = node.children().find(|n| n.has_tag_name("registers")) {
        for reg_node in registers_node
            .children()
            .filter(|n| n.has_tag_name("register"))
        {
            match parse_register(&reg_node) {
                Ok(reg) => registers.push(reg),
                Err(e) => eprintln!("Warning: Failed to parse register in {}: {}", name, e),
            }
        }
    }

    // Parse interrupts
    for int_node in node.children().filter(|n| n.has_tag_name("interrupt")) {
        if let Some(interrupt) = parse_interrupt(&int_node) {
            interrupts.push(interrupt);
        }
    }

    Ok(Peripheral {
        name,
        description,
        base_address,
        derived_from,
        registers,
        interrupts,
    })
}

fn parse_register(node: &Node) -> Result<Register> {
    let name =
        get_child_text(node, "name").ok_or_else(|| anyhow::anyhow!("Register missing name"))?;

    let description = get_child_text(node, "description")
        .map(|s| clean_description(&s))
        .unwrap_or_default();

    let offset = get_child_text(node, "addressOffset")
        .and_then(|s| parse_number(&s))
        .unwrap_or(0);

    let reset_value = get_child_text(node, "resetValue").and_then(|s| parse_number(&s));

    let access = get_child_text(node, "access")
        .map(|s| Access::from_str(&s))
        .unwrap_or(Access::ReadWrite);

    let mut fields = Vec::new();

    if let Some(fields_node) = node.children().find(|n| n.has_tag_name("fields")) {
        for field_node in fields_node.children().filter(|n| n.has_tag_name("field")) {
            match parse_field(&field_node) {
                Ok(field) => fields.push(field),
                Err(e) => eprintln!("Warning: Failed to parse field in {}: {}", name, e),
            }
        }
    }

    // Sort fields by LSB position (highest bit first for better readability)
    fields.sort_by(|a, b| b.lsb.cmp(&a.lsb));

    Ok(Register {
        name,
        description,
        offset,
        reset_value,
        access,
        fields,
    })
}

fn parse_field(node: &Node) -> Result<Field> {
    let name = get_child_text(node, "name").ok_or_else(|| anyhow::anyhow!("Field missing name"))?;

    let description = get_child_text(node, "description")
        .map(|s| clean_description(&s))
        .unwrap_or_default();

    let (msb, lsb) = parse_bit_range(node)?;

    let access = get_child_text(node, "access")
        .map(|s| Access::from_str(&s))
        .unwrap_or(Access::ReadWrite);

    let mut enum_values = Vec::new();

    if let Some(enum_node) = node.children().find(|n| n.has_tag_name("enumeratedValues")) {
        for ev_node in enum_node
            .children()
            .filter(|n| n.has_tag_name("enumeratedValue"))
        {
            if let Some(ev) = parse_enum_value(&ev_node) {
                enum_values.push(ev);
            }
        }
    }

    Ok(Field {
        name,
        description,
        msb,
        lsb,
        access,
        enum_values,
    })
}

fn parse_bit_range(node: &Node) -> Result<(u32, u32)> {
    // Try bitRange format: [MSB:LSB]
    if let Some(range) = get_child_text(node, "bitRange") {
        let range = range.trim_matches(|c| c == '[' || c == ']');
        let parts: Vec<&str> = range.split(':').collect();
        if parts.len() == 2 {
            let msb = parts[0].parse().unwrap_or(31);
            let lsb = parts[1].parse().unwrap_or(0);
            return Ok((msb, lsb));
        }
    }

    // Try bitOffset and bitWidth
    let offset = get_child_text(node, "bitOffset")
        .and_then(|s| s.parse().ok())
        .unwrap_or(0);
    let width = get_child_text(node, "bitWidth")
        .and_then(|s| s.parse().ok())
        .unwrap_or(1);

    Ok((offset + width - 1, offset))
}

fn parse_enum_value(node: &Node) -> Option<EnumValue> {
    let name = get_child_text(node, "name")?;
    let value = get_child_text(node, "value").and_then(|s| parse_number(&s))?;
    let description = get_child_text(node, "description")
        .map(|s| clean_description(&s))
        .unwrap_or_default();

    Some(EnumValue {
        name,
        value,
        description,
    })
}

fn parse_interrupt(node: &Node) -> Option<Interrupt> {
    let name = get_child_text(node, "name")?;
    let value = get_child_text(node, "value").and_then(|s| s.parse().ok())?;
    let description = get_child_text(node, "description")
        .map(|s| clean_description(&s))
        .unwrap_or_default();

    Some(Interrupt {
        name,
        value,
        description,
    })
}

fn get_child_text(node: &Node, tag: &str) -> Option<String> {
    node.children()
        .find(|n| n.has_tag_name(tag))
        .and_then(|n| n.text())
        .map(|s| s.trim().to_string())
}

fn parse_number(s: &str) -> Option<u32> {
    let s = s.trim();
    if s.starts_with("0x") || s.starts_with("0X") {
        u32::from_str_radix(&s[2..], 16).ok()
    } else {
        s.parse().ok()
    }
}

fn clean_description(s: &str) -> String {
    // Clean up multi-line descriptions, remove excessive whitespace
    s.lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
        .replace("  ", " ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_number() {
        assert_eq!(parse_number("0x40020000"), Some(0x40020000));
        assert_eq!(parse_number("0X1FFFFFFF"), Some(0x1FFFFFFF));
        assert_eq!(parse_number("123"), Some(123));
    }

    #[test]
    fn test_clean_description() {
        let input = "  This is a\n    multi-line\n  description  ";
        assert_eq!(clean_description(input), "This is a multi-line description");
    }
}
