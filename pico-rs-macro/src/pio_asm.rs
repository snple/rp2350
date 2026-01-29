//! PIO Assembly compiler
//!
//! Compiles PIO assembly source code into 16-bit machine code instructions.

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use std::collections::HashMap;
use syn::{
    LitStr, Token, parse::Parse, parse::ParseStream, parse_macro_input, punctuated::Punctuated,
};

/// Input to the pio_asm macro - a list of string literals
struct PioAsmInput {
    lines: Vec<String>,
}

impl Parse for PioAsmInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lines_punct: Punctuated<LitStr, Token![,]> = Punctuated::parse_terminated(input)?;
        let lines = lines_punct.iter().map(|lit| lit.value()).collect();
        Ok(PioAsmInput { lines })
    }
}

/// Compiled PIO program
#[derive(Debug)]
struct CompiledProgram {
    instructions: Vec<u16>,
    wrap_target: Option<usize>,
    wrap: Option<usize>,
    origin: Option<usize>,
    side_set_count: u32,
    side_set_opt: bool,
    side_set_pindirs: bool,
}

/// PIO instruction opcodes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Opcode {
    Jmp = 0b000,
    Wait = 0b001,
    In = 0b010,
    Out = 0b011,
    PushPull = 0b100,
    Mov = 0b101,
    Irq = 0b110,
    Set = 0b111,
}

/// JMP conditions
#[derive(Debug, Clone, Copy)]
enum JmpCondition {
    Always = 0b000,
    NotX = 0b001,
    XDecNonzero = 0b010,
    NotY = 0b011,
    YDecNonzero = 0b100,
    XNotEqualY = 0b101,
    Pin = 0b110,
    NotOsre = 0b111,
}

/// WAIT sources
#[derive(Debug, Clone, Copy)]
enum WaitSource {
    Gpio = 0b00,
    Pin = 0b01,
    Irq = 0b10,
}

/// IN/OUT sources and destinations
#[derive(Debug, Clone, Copy)]
enum InOutDest {
    Pins = 0b000,
    X = 0b001,
    Y = 0b010,
    Null = 0b011,
    Pindirs = 0b100,
    Exec = 0b101,
    Pc = 0b110,
    IsrOsr = 0b111, // ISR for IN, OSR for OUT
}

/// MOV sources
#[derive(Debug, Clone, Copy)]
enum MovSource {
    Pins = 0b000,
    X = 0b001,
    Y = 0b010,
    Null = 0b011,
    Status = 0b101,
    Isr = 0b110,
    Osr = 0b111,
}

/// MOV destinations
#[derive(Debug, Clone, Copy)]
enum MovDest {
    Pins = 0b000,
    X = 0b001,
    Y = 0b010,
    Exec = 0b100,
    Pc = 0b101,
    Isr = 0b110,
    Osr = 0b111,
}

/// MOV operations
#[derive(Debug, Clone, Copy)]
enum MovOp {
    None = 0b00,
    Invert = 0b01,
    BitReverse = 0b10,
}

/// SET destinations
#[derive(Debug, Clone, Copy)]
enum SetDest {
    Pins = 0b000,
    X = 0b001,
    Y = 0b010,
    Pindirs = 0b100,
}

pub fn pio_asm_impl(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as PioAsmInput);

    match compile_program(&input.lines) {
        Ok(program) => {
            let instructions = &program.instructions;
            let len = instructions.len();

            let wrap_target = program.wrap_target.unwrap_or(0);
            let wrap = program.wrap.unwrap_or(len.saturating_sub(1));
            let origin = program.origin;
            let side_set = program.side_set_count;
            let side_set_opt = program.side_set_opt;
            let side_set_pindirs = program.side_set_pindirs;

            let origin_field = if let Some(orig) = origin {
                quote! { Some(#orig) }
            } else {
                quote! { None }
            };

            quote! {
                {
                    const PROGRAM: &[u16] = &[#(#instructions),*];
                    const WRAP_TARGET: usize = #wrap_target;
                    const WRAP: usize = #wrap;

                    struct PioProgram {
                        pub instructions: &'static [u16],
                        pub wrap_target: usize,
                        pub wrap: usize,
                        pub origin: Option<usize>,
                        pub side_set_count: u32,
                        pub side_set_opt: bool,
                        pub side_set_pindirs: bool,
                    }

                    PioProgram {
                        instructions: PROGRAM,
                        wrap_target: WRAP_TARGET,
                        wrap: WRAP,
                        origin: #origin_field,
                        side_set_count: #side_set,
                        side_set_opt: #side_set_opt,
                        side_set_pindirs: #side_set_pindirs,
                    }
                }
            }
            .into()
        }
        Err(e) => syn::Error::new(Span::call_site(), e)
            .to_compile_error()
            .into(),
    }
}

fn compile_program(lines: &[String]) -> Result<CompiledProgram, String> {
    let mut instructions = Vec::new();
    let mut labels: HashMap<String, usize> = HashMap::new();
    let mut wrap_target = None;
    let mut wrap = None;
    let mut origin = None;
    let mut side_set_count = 0;
    let mut side_set_opt = false;
    let mut side_set_pindirs = false;
    let mut _program_name = String::new();

    // First pass: collect labels and directives
    for line in lines {
        let line = line.trim();

        // Skip empty lines and comments
        if line.is_empty() || line.starts_with(';') || line.starts_with("//") {
            continue;
        }

        // Handle directives
        if line.starts_with('.') {
            if line == ".wrap_target" {
                wrap_target = Some(instructions.len());
            } else if line == ".wrap" {
                wrap = Some(instructions.len().saturating_sub(1));
            } else if line.starts_with(".side_set") {
                // Parse .side_set directive: .side_set <count> [opt] [pindirs]
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    if let Ok(count) = parts[1].parse::<u32>() {
                        side_set_count = count;
                    }
                }
                for part in parts.iter().skip(2) {
                    match *part {
                        "opt" => side_set_opt = true,
                        "pindirs" => side_set_pindirs = true,
                        _ => {}
                    }
                }
            } else if line.starts_with(".program") {
                // Parse .program <name>
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    _program_name = parts[1].to_string();
                }
            } else if line.starts_with(".origin") {
                // Parse .origin <address>
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    if let Ok(addr) = parts[1].parse::<usize>() {
                        origin = Some(addr);
                    }
                }
            } else if line.starts_with(".pio_version") {
                // Ignore version directive
                continue;
            } else if line.starts_with(".define") {
                // TODO: Handle .define directives for constants
                continue;
            }
            continue;
        }

        // Handle labels
        if line.ends_with(':') {
            let label = line.trim_end_matches(':').trim();
            // Remove any inline comments after the label
            let label = if let Some(idx) = label.find(';') {
                label[..idx].trim()
            } else if let Some(idx) = label.find("//") {
                label[..idx].trim()
            } else {
                label
            };
            labels.insert(label.to_string(), instructions.len());
            continue;
        }

        // This is an instruction
        instructions.push(0); // Placeholder
    }

    // Second pass: compile instructions
    let mut instructions = Vec::new();
    for line in lines {
        let line = line.trim();

        // Skip empty lines, comments, directives, and labels
        if line.is_empty()
            || line.starts_with(';')
            || line.starts_with("//")
            || line.starts_with('.')
            || line.ends_with(':')
        {
            continue;
        }

        // Remove inline comments
        let line = if let Some(idx) = line.find(';') {
            line[..idx].trim()
        } else if let Some(idx) = line.find("//") {
            line[..idx].trim()
        } else {
            line
        };

        if line.is_empty() {
            continue;
        }

        let instr = compile_instruction(
            line,
            &labels,
            instructions.len(),
            side_set_count,
            side_set_opt,
        )?;
        instructions.push(instr);
    }

    Ok(CompiledProgram {
        instructions,
        wrap_target,
        wrap,
        origin,
        side_set_count,
        side_set_opt,
        side_set_pindirs,
    })
}

fn compile_instruction(
    line: &str,
    labels: &HashMap<String, usize>,
    current_addr: usize,
    side_set_count: u32,
    side_set_opt: bool,
) -> Result<u16, String> {
    // Parse delay and side-set
    let (line, delay, side_set) = parse_delay_and_sideset(line)?;

    // Split instruction into parts
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.is_empty() {
        return Err("Empty instruction".to_string());
    }

    let mnemonic = parts[0].to_lowercase();

    let mut instruction: u16;

    // Encode based on mnemonic
    match mnemonic.as_str() {
        "nop" => {
            // NOP is encoded as MOV y, y
            instruction = encode_mov(MovDest::Y, MovOp::None, MovSource::Y);
        }
        "jmp" => {
            instruction = encode_jmp(&parts[1..], labels, current_addr)?;
        }
        "wait" => {
            instruction = encode_wait(&parts[1..])?;
        }
        "in" => {
            instruction = encode_in(&parts[1..])?;
        }
        "out" => {
            instruction = encode_out(&parts[1..])?;
        }
        "push" => {
            instruction = encode_push(&parts[1..])?;
        }
        "pull" => {
            instruction = encode_pull(&parts[1..])?;
        }
        "mov" => {
            instruction = encode_mov_instr(&parts[1..])?;
        }
        "irq" => {
            instruction = encode_irq(&parts[1..])?;
        }
        "set" => {
            instruction = encode_set(&parts[1..])?;
        }
        _ => {
            return Err(format!("Unknown instruction: {}", mnemonic));
        }
    }

    // Add delay
    instruction |= (delay as u16) << 8;

    // Add side-set if present
    if let Some(ss) = side_set {
        let shift = if side_set_opt {
            12
        } else {
            13 - side_set_count
        };
        instruction |= (ss as u16) << shift;
        if side_set_opt {
            instruction |= 1 << (12 + side_set_count); // Set side-set enable bit
        }
    }

    Ok(instruction)
}

fn parse_delay_and_sideset(line: &str) -> Result<(&str, u8, Option<u8>), String> {
    let mut delay = 0u8;
    let mut side_set = None;
    let mut cleaned_line = line;

    // Parse delay [n]
    if let Some(delay_start) = line.find('[') {
        if let Some(delay_end) = line.find(']') {
            let delay_str = &line[delay_start + 1..delay_end].trim();
            delay = delay_str
                .parse()
                .map_err(|_| format!("Invalid delay: {}", delay_str))?;
            cleaned_line = &line[..delay_start].trim();
        }
    }

    // Parse side-set (side n)
    if let Some(side_pos) = cleaned_line.to_lowercase().find("side") {
        let before_side = &cleaned_line[..side_pos].trim();
        let after_side = &cleaned_line[side_pos + 4..].trim();

        let parts: Vec<&str> = after_side.split_whitespace().collect();
        if !parts.is_empty() {
            side_set = Some(
                parts[0]
                    .parse()
                    .map_err(|_| format!("Invalid side-set value: {}", parts[0]))?,
            );
        }
        cleaned_line = before_side;
    }

    Ok((cleaned_line, delay, side_set))
}

fn encode_jmp(
    args: &[&str],
    labels: &HashMap<String, usize>,
    current_addr: usize,
) -> Result<u16, String> {
    if args.is_empty() {
        return Err("JMP requires arguments".to_string());
    }

    let (condition, target) = if args.len() == 1 {
        (JmpCondition::Always, args[0])
    } else {
        let cond = match args[0].to_lowercase().trim_end_matches(',') {
            s if s == "!" || s == "not_x" => JmpCondition::NotX,
            s if s == "x--" || s == "x_dec" => JmpCondition::XDecNonzero,
            s if s == "!y" || s == "not_y" => JmpCondition::NotY,
            s if s == "y--" || s == "y_dec" => JmpCondition::YDecNonzero,
            s if s == "x!=y" || s == "x_ne_y" => JmpCondition::XNotEqualY,
            s if s == "pin" => JmpCondition::Pin,
            s if s == "!osre" || s == "not_osre" => JmpCondition::NotOsre,
            s => return Err(format!("Unknown JMP condition: {}", s)),
        };
        (cond, args[1])
    };

    let addr = if let Ok(addr) = target.parse::<usize>() {
        addr
    } else if let Some(&addr) = labels.get(target) {
        addr
    } else {
        return Err(format!("Unknown label: {}", target));
    };

    if addr >= 32 {
        return Err(format!("JMP address out of range: {}", addr));
    }

    Ok(((Opcode::Jmp as u16) << 13) | ((condition as u16) << 5) | (addr as u16))
}

fn encode_wait(args: &[&str]) -> Result<u16, String> {
    if args.len() < 2 {
        return Err("WAIT requires at least 2 arguments".to_string());
    }

    let polarity = match args[0].trim_end_matches(',') {
        "0" => 0,
        "1" => 1,
        _ => return Err(format!("Invalid WAIT polarity: {}", args[0])),
    };

    let parts: Vec<&str> = args[1].split(',').collect();
    let source_str = parts[0].trim().to_lowercase();

    let (source, index) = if source_str == "gpio" {
        if parts.len() < 2 {
            return Err("WAIT gpio requires index".to_string());
        }
        let idx: u16 = parts[1].trim().parse().map_err(|_| "Invalid GPIO index")?;
        (WaitSource::Gpio, idx)
    } else if source_str == "pin" {
        if parts.len() < 2 {
            return Err("WAIT pin requires index".to_string());
        }
        let idx: u16 = parts[1].trim().parse().map_err(|_| "Invalid pin index")?;
        (WaitSource::Pin, idx)
    } else if source_str == "irq" {
        if parts.len() < 2 {
            return Err("WAIT irq requires index".to_string());
        }
        let idx: u16 = parts[1].trim().parse().map_err(|_| "Invalid IRQ index")?;
        (WaitSource::Irq, idx)
    } else {
        return Err(format!("Unknown WAIT source: {}", source_str));
    };

    Ok(((Opcode::Wait as u16) << 13)
        | ((polarity as u16) << 7)
        | ((source as u16) << 5)
        | (index as u16))
}

fn encode_in(args: &[&str]) -> Result<u16, String> {
    if args.len() < 2 {
        return Err("IN requires source and bit count".to_string());
    }

    let source = parse_in_out_dest(args[0].trim_end_matches(','))?;
    let bit_count: u16 = args[1].parse().map_err(|_| "Invalid bit count")?;

    if bit_count > 32 {
        return Err("IN bit count must be 0-32".to_string());
    }

    Ok(((Opcode::In as u16) << 13) | ((source as u16) << 5) | (bit_count & 0x1F))
}

fn encode_out(args: &[&str]) -> Result<u16, String> {
    if args.len() < 2 {
        return Err("OUT requires destination and bit count".to_string());
    }

    let dest = parse_in_out_dest(args[0].trim_end_matches(','))?;
    let bit_count: u16 = args[1].parse().map_err(|_| "Invalid bit count")?;

    if bit_count > 32 {
        return Err("OUT bit count must be 0-32".to_string());
    }

    Ok(((Opcode::Out as u16) << 13) | ((dest as u16) << 5) | (bit_count & 0x1F))
}

fn encode_push(args: &[&str]) -> Result<u16, String> {
    let mut iffull = false;
    let mut block = true;

    for arg in args {
        match arg.trim_end_matches(',').to_lowercase().as_str() {
            "iffull" => iffull = true,
            "noblock" => block = false,
            "block" => block = true,
            _ => return Err(format!("Unknown PUSH modifier: {}", arg)),
        }
    }

    Ok(((Opcode::PushPull as u16) << 13)
        | (1 << 7)
        | ((iffull as u16) << 6)
        | ((block as u16) << 5))
}

fn encode_pull(args: &[&str]) -> Result<u16, String> {
    let mut ifempty = false;
    let mut block = true;

    for arg in args {
        match arg.trim_end_matches(',').to_lowercase().as_str() {
            "ifempty" => ifempty = true,
            "noblock" => block = false,
            "block" => block = true,
            _ => return Err(format!("Unknown PULL modifier: {}", arg)),
        }
    }

    Ok(((Opcode::PushPull as u16) << 13)
        | (0 << 7)
        | ((ifempty as u16) << 6)
        | ((block as u16) << 5))
}

fn encode_mov_instr(args: &[&str]) -> Result<u16, String> {
    if args.len() < 2 {
        return Err("MOV requires destination and source".to_string());
    }

    let dest = parse_mov_dest(args[0].trim_end_matches(','))?;

    // Check for operation (::, ~, !!)
    let (op, source_str) =
        if args.len() > 2 && (args[1] == "::" || args[1] == "~" || args[1] == "!") {
            let op = match args[1] {
                "::" => MovOp::BitReverse,
                "~" | "!" => MovOp::Invert,
                _ => MovOp::None,
            };
            (op, args[2])
        } else {
            (MovOp::None, args[1])
        };

    let source = parse_mov_source(source_str)?;

    Ok(encode_mov(dest, op, source))
}

fn encode_mov(dest: MovDest, op: MovOp, source: MovSource) -> u16 {
    ((Opcode::Mov as u16) << 13) | ((dest as u16) << 5) | ((op as u16) << 3) | (source as u16)
}

fn encode_irq(args: &[&str]) -> Result<u16, String> {
    if args.is_empty() {
        return Err("IRQ requires arguments".to_string());
    }

    let mut clear = false;
    let mut wait = false;
    let mut index_str = args[0];

    for (i, arg) in args.iter().enumerate() {
        match arg.trim_end_matches(',').to_lowercase().as_str() {
            "clear" => clear = true,
            "wait" => wait = true,
            _ if i == 0 || i == args.len() - 1 => index_str = arg,
            _ => {}
        }
    }

    let index: u16 = index_str
        .trim_end_matches(',')
        .parse()
        .map_err(|_| "Invalid IRQ index")?;

    if index > 7 {
        return Err("IRQ index must be 0-7".to_string());
    }

    Ok(((Opcode::Irq as u16) << 13) | ((clear as u16) << 6) | ((wait as u16) << 5) | index)
}

fn encode_set(args: &[&str]) -> Result<u16, String> {
    if args.len() < 2 {
        return Err("SET requires destination and value".to_string());
    }

    let dest = parse_set_dest(args[0].trim_end_matches(','))?;
    let value: u16 = args[1].parse().map_err(|_| "Invalid SET value")?;

    if value > 31 {
        return Err("SET value must be 0-31".to_string());
    }

    Ok(((Opcode::Set as u16) << 13) | ((dest as u16) << 5) | value)
}

fn parse_in_out_dest(s: &str) -> Result<InOutDest, String> {
    match s.to_lowercase().as_ref() {
        "pins" => Ok(InOutDest::Pins),
        "x" => Ok(InOutDest::X),
        "y" => Ok(InOutDest::Y),
        "null" => Ok(InOutDest::Null),
        "pindirs" => Ok(InOutDest::Pindirs),
        "pc" => Ok(InOutDest::Pc),
        "isr" | "osr" => Ok(InOutDest::IsrOsr),
        "exec" => Ok(InOutDest::Exec),
        _ => Err(format!("Unknown IN/OUT destination: {}", s)),
    }
}

fn parse_mov_dest(s: &str) -> Result<MovDest, String> {
    match s.to_lowercase().as_ref() {
        "pins" => Ok(MovDest::Pins),
        "x" => Ok(MovDest::X),
        "y" => Ok(MovDest::Y),
        "exec" => Ok(MovDest::Exec),
        "pc" => Ok(MovDest::Pc),
        "isr" => Ok(MovDest::Isr),
        "osr" => Ok(MovDest::Osr),
        _ => Err(format!("Unknown MOV destination: {}", s)),
    }
}

fn parse_mov_source(s: &str) -> Result<MovSource, String> {
    match s.to_lowercase().as_ref() {
        "pins" => Ok(MovSource::Pins),
        "x" => Ok(MovSource::X),
        "y" => Ok(MovSource::Y),
        "null" => Ok(MovSource::Null),
        "status" => Ok(MovSource::Status),
        "isr" => Ok(MovSource::Isr),
        "osr" => Ok(MovSource::Osr),
        _ => Err(format!("Unknown MOV source: {}", s)),
    }
}

fn parse_set_dest(s: &str) -> Result<SetDest, String> {
    match s.to_lowercase().as_ref() {
        "pins" => Ok(SetDest::Pins),
        "x" => Ok(SetDest::X),
        "y" => Ok(SetDest::Y),
        "pindirs" => Ok(SetDest::Pindirs),
        _ => Err(format!("Unknown SET destination: {}", s)),
    }
}
