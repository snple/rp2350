//! PIO Assembly compiler
//!
//! Compiles PIO assembly source code into 16-bit machine code instructions.
//! Supports pico-sdk compatible syntax including:
//! - Labels (inline and standalone): `label:`, `public label:`
//! - Directives: `.program`, `.wrap_target`, `.wrap`, `.side_set`, `.origin`, `.define`
//! - All PIO instructions: JMP, WAIT, IN, OUT, PUSH, PULL, MOV, IRQ, SET, NOP
//! - Delay with expressions: `[31]`, `[T1 - 1]`
//! - Side-set: `side 0`, `side 1`
//! - Comments: `;` and `//`
//!
//! ## Usage
//!
//! ```ignore
//! let program = pio_asm!(r#"
//!     .program blink
//!     .wrap_target
//!     set pins, 1 [31]
//!     set pins, 0 [31]
//!     .wrap
//! "#);
//! ```

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use std::collections::HashMap;
use syn::{LitStr, parse::Parse, parse::ParseStream, parse_macro_input};

/// Input to the pio_asm macro - a single multiline string
struct PioAsmInput {
    lines: Vec<String>,
}

impl Parse for PioAsmInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lit: LitStr = input.parse()?;
        let lines: Vec<String> = lit.value().lines().map(|s| s.to_string()).collect();
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
    #[allow(dead_code)]
    public_labels: HashMap<String, usize>,
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
    Jmppin = 0b11, // PIO version 1+
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
    Pindirs = 0b011, // PIO version 1+
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

/// Context for compilation including constants/defines
struct CompileContext {
    labels: HashMap<String, usize>,
    defines: HashMap<String, i32>,
    side_set_count: u32,
    side_set_opt: bool,
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

/// Remove comments from a line (both ; and // style)
fn strip_comments(line: &str) -> &str {
    // Find the first comment marker
    let semicolon_pos = line.find(';');
    let slash_pos = line.find("//");

    match (semicolon_pos, slash_pos) {
        (Some(s), Some(d)) => &line[..s.min(d)],
        (Some(s), None) => &line[..s],
        (None, Some(d)) => &line[..d],
        (None, None) => line,
    }
}

/// Parse a line, extracting any label prefix and the remaining instruction
/// Returns (Option<label_name>, Option<is_public>, remaining_line)
fn parse_label_and_instruction(line: &str) -> (Option<&str>, bool, &str) {
    let line = line.trim();

    // Check for public modifier
    let (is_public, line) = if line.starts_with("public ") || line.starts_with("public\t") {
        (true, line[7..].trim())
    } else {
        (false, line)
    };

    // Look for label (word followed by colon)
    if let Some(colon_pos) = line.find(':') {
        let potential_label = line[..colon_pos].trim();
        // Check if it's a valid label (alphanumeric + underscore, not starting with digit)
        if !potential_label.is_empty()
            && potential_label
                .chars()
                .all(|c| c.is_alphanumeric() || c == '_')
            && !potential_label.chars().next().unwrap().is_numeric()
        {
            let rest = line[colon_pos + 1..].trim();
            return (Some(potential_label), is_public, rest);
        }
    }

    (None, is_public, line)
}

fn compile_program(lines: &[String]) -> Result<CompiledProgram, String> {
    let mut labels: HashMap<String, usize> = HashMap::new();
    let mut public_labels: HashMap<String, usize> = HashMap::new();
    let mut defines: HashMap<String, i32> = HashMap::new();
    let mut wrap_target = None;
    let mut wrap = None;
    let mut origin = None;
    let mut side_set_count = 0u32;
    let mut side_set_opt = false;
    let mut side_set_pindirs = false;
    let mut _program_name = String::new();

    // First pass: collect labels, directives, and count instructions
    let mut instruction_count = 0usize;

    for line in lines {
        let line = strip_comments(line).trim();

        if line.is_empty() {
            continue;
        }

        // Handle directives first
        if line.starts_with('.') {
            parse_directive(
                line,
                instruction_count,
                &mut wrap_target,
                &mut wrap,
                &mut origin,
                &mut side_set_count,
                &mut side_set_opt,
                &mut side_set_pindirs,
                &mut defines,
                &mut _program_name,
            );
            continue;
        }

        // Parse potential label and instruction
        let (label, is_public, remaining) = parse_label_and_instruction(line);

        if let Some(label_name) = label {
            labels.insert(label_name.to_string(), instruction_count);
            if is_public {
                public_labels.insert(label_name.to_string(), instruction_count);
            }
        }

        // Count instruction if there's any remaining content
        if !remaining.is_empty() {
            instruction_count += 1;
        }
    }

    // Second pass: compile instructions
    let mut instructions = Vec::new();

    let ctx = CompileContext {
        labels,
        defines,
        side_set_count,
        side_set_opt,
    };

    for line in lines {
        let line = strip_comments(line).trim();

        if line.is_empty() || line.starts_with('.') {
            continue;
        }

        // Get the instruction part (after any label)
        let (_, _, instr_line) = parse_label_and_instruction(line);

        if instr_line.is_empty() {
            continue;
        }

        let instr = compile_instruction(instr_line, &ctx, instructions.len())?;
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
        public_labels,
    })
}

#[allow(clippy::too_many_arguments)]
fn parse_directive(
    line: &str,
    instruction_count: usize,
    wrap_target: &mut Option<usize>,
    wrap: &mut Option<usize>,
    origin: &mut Option<usize>,
    side_set_count: &mut u32,
    side_set_opt: &mut bool,
    side_set_pindirs: &mut bool,
    defines: &mut HashMap<String, i32>,
    program_name: &mut String,
) {
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.is_empty() {
        return;
    }

    let directive = parts[0].to_lowercase();

    match directive.as_str() {
        ".wrap_target" => {
            *wrap_target = Some(instruction_count);
        }
        ".wrap" => {
            *wrap = Some(instruction_count.saturating_sub(1));
        }
        ".side_set" => {
            // Parse .side_set directive: .side_set <count> [opt] [pindirs]
            if parts.len() >= 2
                && let Ok(count) = parts[1].parse::<u32>()
            {
                *side_set_count = count;
            }
            for part in parts.iter().skip(2) {
                match *part {
                    "opt" => *side_set_opt = true,
                    "pindirs" => *side_set_pindirs = true,
                    _ => {}
                }
            }
        }
        ".program" => {
            if parts.len() >= 2 {
                *program_name = parts[1].to_string();
            }
        }
        ".origin" => {
            if parts.len() >= 2
                && let Ok(addr) = parts[1].parse::<usize>()
            {
                *origin = Some(addr);
            }
        }
        ".define" => {
            // Parse .define [public] <name> <value>
            if parts.len() >= 3 {
                let (name_idx, _is_public) = if parts[1] == "public" {
                    (2, true)
                } else {
                    (1, false)
                };

                if parts.len() > name_idx + 1 {
                    let name = parts[name_idx];
                    // Join remaining parts to handle expressions
                    let value_str: String = parts[name_idx + 1..].join(" ");
                    if let Ok(value) = eval_simple_expr(&value_str, defines) {
                        defines.insert(name.to_string(), value);
                    }
                }
            }
        }
        ".pio_version" | ".lang_opt" | ".word" | ".fifo" | ".out" | ".in" | ".mov_status"
        | ".clock_div" | ".set" => {
            // Ignore these directives for now
        }
        _ => {
            // Unknown directive, ignore
        }
    }
}

/// Evaluate a simple arithmetic expression with defined constants
fn eval_simple_expr(expr: &str, defines: &HashMap<String, i32>) -> Result<i32, String> {
    let expr = expr.trim();

    // Handle parentheses: (expr)
    if expr.starts_with('(') && expr.ends_with(')') {
        return eval_simple_expr(&expr[1..expr.len() - 1], defines);
    }

    // Handle binary operations: a + b, a - b, a * b, a / b, a << b, a >> b
    // Find the lowest precedence operator from right to left
    let mut paren_depth = 0;
    let mut op_pos = None;
    let mut op_char = ' ';

    let chars: Vec<char> = expr.chars().collect();
    let len = chars.len();

    // Scan for + and - (lowest precedence)
    for i in (0..len).rev() {
        match chars[i] {
            '(' => paren_depth += 1,
            ')' => paren_depth -= 1,
            '+' | '-' if paren_depth == 0 && i > 0 => {
                // Make sure it's not part of a number sign
                if i > 0 {
                    op_pos = Some(i);
                    op_char = chars[i];
                    break;
                }
            }
            _ => {}
        }
    }

    // If no + or -, scan for * and /
    if op_pos.is_none() {
        paren_depth = 0;
        for i in (0..len).rev() {
            match chars[i] {
                '(' => paren_depth += 1,
                ')' => paren_depth -= 1,
                '*' | '/' if paren_depth == 0 => {
                    op_pos = Some(i);
                    op_char = chars[i];
                    break;
                }
                _ => {}
            }
        }
    }

    // Handle shift operators
    if op_pos.is_none() {
        if let Some(pos) = expr.find("<<") {
            let left = eval_simple_expr(&expr[..pos], defines)?;
            let right = eval_simple_expr(&expr[pos + 2..], defines)?;
            return Ok(left << right);
        }
        if let Some(pos) = expr.find(">>") {
            let left = eval_simple_expr(&expr[..pos], defines)?;
            let right = eval_simple_expr(&expr[pos + 2..], defines)?;
            return Ok(left >> right);
        }
    }

    if let Some(pos) = op_pos {
        let left = eval_simple_expr(&expr[..pos], defines)?;
        let right = eval_simple_expr(&expr[pos + 1..], defines)?;
        return match op_char {
            '+' => Ok(left + right),
            '-' => Ok(left - right),
            '*' => Ok(left * right),
            '/' => {
                if right == 0 {
                    Err("Division by zero".to_string())
                } else {
                    Ok(left / right)
                }
            }
            _ => Err(format!("Unknown operator: {}", op_char)),
        };
    }

    // Try to parse as number
    if let Ok(n) = expr.parse::<i32>() {
        return Ok(n);
    }

    // Handle hex numbers
    if (expr.starts_with("0x") || expr.starts_with("0X"))
        && let Ok(n) = i32::from_str_radix(&expr[2..], 16)
    {
        return Ok(n);
    }

    // Handle binary numbers
    if (expr.starts_with("0b") || expr.starts_with("0B"))
        && let Ok(n) = i32::from_str_radix(&expr[2..], 2)
    {
        return Ok(n);
    }
    // Try to look up as define
    if let Some(&value) = defines.get(expr) {
        return Ok(value);
    }

    Err(format!("Cannot evaluate expression: {}", expr))
}

fn compile_instruction(
    line: &str,
    ctx: &CompileContext,
    current_addr: usize,
) -> Result<u16, String> {
    // Parse delay and side-set first
    let (line, delay, side_set) = parse_delay_and_sideset(line, &ctx.defines)?;

    // Split instruction into parts, handling commas properly
    let normalized = line.replace(',', " , ");
    let parts: Vec<&str> = normalized.split_whitespace().collect();

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
            instruction = encode_jmp(&parts[1..], &ctx.labels, current_addr)?;
        }
        "wait" => {
            instruction = encode_wait(&parts[1..], &ctx.defines)?;
        }
        "in" => {
            instruction = encode_in(&parts[1..], &ctx.defines)?;
        }
        "out" => {
            instruction = encode_out(&parts[1..], &ctx.defines)?;
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
            instruction = encode_irq(&parts[1..], &ctx.defines)?;
        }
        "set" => {
            instruction = encode_set(&parts[1..], &ctx.defines)?;
        }
        _ => {
            return Err(format!("Unknown instruction: {}", mnemonic));
        }
    }

    // Calculate delay bits available
    let delay_bits = 5 - ctx.side_set_count;
    let max_delay = (1 << delay_bits) - 1;

    if delay as u32 > max_delay {
        return Err(format!(
            "Delay {} exceeds maximum {} (with {} side-set bits)",
            delay, max_delay, ctx.side_set_count
        ));
    }

    // Encode delay and side-set into bits 12:8
    if ctx.side_set_count > 0 {
        if ctx.side_set_opt {
            // Optional side-set: bit 12 is enable, bits 11:8 are side-set + delay
            if let Some(ss) = side_set {
                // Side-set is present
                instruction |= 1 << 12; // Enable bit
                instruction |= (ss as u16) << (12 - ctx.side_set_count);
                instruction |= (delay as u16) << 8;
            } else {
                // No side-set, just delay
                instruction |= (delay as u16) << 8;
            }
        } else {
            // Required side-set
            let ss = side_set.unwrap_or(0);
            instruction |= (ss as u16) << (13 - ctx.side_set_count);
            instruction |= (delay as u16) << 8;
        }
    } else {
        // No side-set configured, all 5 bits for delay
        instruction |= (delay as u16) << 8;
    }

    Ok(instruction)
}

fn parse_delay_and_sideset(
    line: &str,
    defines: &HashMap<String, i32>,
) -> Result<(String, u8, Option<u8>), String> {
    let mut delay = 0u8;
    let mut side_set = None;
    let mut result = line.to_string();

    // Parse delay [n] or [expr]
    while let Some(delay_start) = result.find('[') {
        if let Some(delay_end) = result.find(']') {
            let delay_expr = &result[delay_start + 1..delay_end].trim();
            let delay_value =
                eval_simple_expr(delay_expr, defines).map_err(|e| format!("Delay error: {}", e))?;
            if !(0..=31).contains(&delay_value) {
                return Err(format!("Delay value {} out of range (0-31)", delay_value));
            }
            delay = delay_value as u8;
            result = format!(
                "{}{}",
                &result[..delay_start].trim(),
                &result[delay_end + 1..].trim()
            );
        } else {
            break;
        }
    }

    // Parse side-set (side n)
    // Case insensitive search for "side"
    let lower = result.to_lowercase();
    if let Some(side_pos) = lower.find("side") {
        let before_side = &result[..side_pos].trim();
        let after_side = &result[side_pos + 4..].trim();

        // Get the side-set value
        let parts: Vec<&str> = after_side.split_whitespace().collect();
        if !parts.is_empty() {
            let ss_value = eval_simple_expr(parts[0], defines)
                .map_err(|e| format!("Side-set error: {}", e))?;
            if !(0..=31).contains(&ss_value) {
                return Err(format!("Side-set value {} out of range (0-31)", ss_value));
            }
            side_set = Some(ss_value as u8);
        }
        result = before_side.to_string();
    }

    Ok((result, delay, side_set))
}

fn encode_jmp(
    args: &[&str],
    labels: &HashMap<String, usize>,
    _current_addr: usize,
) -> Result<u16, String> {
    if args.is_empty() {
        return Err("JMP requires arguments".to_string());
    }

    // Filter out commas
    let args: Vec<&str> = args.iter().filter(|&&s| s != ",").copied().collect();

    let (condition, target) = if args.len() == 1 {
        (JmpCondition::Always, args[0])
    } else {
        // Parse condition - handle various formats
        let cond_str = args[0].to_lowercase();
        let cond_str = cond_str.trim_end_matches(',');

        let cond = match cond_str {
            // !x variants
            "!x" | "not_x" => JmpCondition::NotX,
            // x-- variants
            "x--" | "x_dec" => JmpCondition::XDecNonzero,
            // !y variants
            "!y" | "not_y" => JmpCondition::NotY,
            // y-- variants
            "y--" | "y_dec" => JmpCondition::YDecNonzero,
            // x != y variants
            "x!=y" | "x_ne_y" => JmpCondition::XNotEqualY,
            // pin
            "pin" => JmpCondition::Pin,
            // !osre variants
            "!osre" | "not_osre" => JmpCondition::NotOsre,
            _ => return Err(format!("Unknown JMP condition: {}", cond_str)),
        };

        // Target is the last argument
        (cond, *args.last().unwrap())
    };

    // Parse target address
    let target = target.trim_end_matches(',');
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

fn encode_wait(args: &[&str], defines: &HashMap<String, i32>) -> Result<u16, String> {
    if args.len() < 2 {
        return Err("WAIT requires at least 2 arguments".to_string());
    }

    // Filter out commas
    let args: Vec<&str> = args.iter().filter(|&&s| s != ",").copied().collect();

    let polarity_str = args[0].trim_end_matches(',');
    let polarity = match polarity_str {
        "0" => 0u16,
        "1" => 1u16,
        _ => return Err(format!("Invalid WAIT polarity: {}", polarity_str)),
    };

    let source_str = args[1].to_lowercase();
    let source_str = source_str.trim_end_matches(',');

    let (source, index, rel) = match source_str {
        "gpio" => {
            if args.len() < 3 {
                return Err("WAIT gpio requires index".to_string());
            }
            let idx = eval_simple_expr(args[2].trim_end_matches(','), defines)
                .map_err(|_| "Invalid GPIO index")?;
            (WaitSource::Gpio, idx as u16, false)
        }
        "pin" => {
            if args.len() < 3 {
                return Err("WAIT pin requires index".to_string());
            }
            let idx = eval_simple_expr(args[2].trim_end_matches(','), defines)
                .map_err(|_| "Invalid pin index")?;
            (WaitSource::Pin, idx as u16, false)
        }
        "irq" => {
            if args.len() < 3 {
                return Err("WAIT irq requires index".to_string());
            }
            // Check for rel modifier
            let mut idx_str = args[2].trim_end_matches(',');
            let mut is_rel = false;

            // Handle "next" and "prev" modifiers (PIO v1)
            if idx_str == "next" || idx_str == "prev" {
                if args.len() < 4 {
                    return Err("WAIT irq next/prev requires index".to_string());
                }
                idx_str = args[3].trim_end_matches(',');
            }

            // Check for "rel" at the end
            if args.len() > 3 && args.last().map(|s| s.to_lowercase()) == Some("rel".to_string()) {
                is_rel = true;
            }

            let idx = eval_simple_expr(idx_str, defines).map_err(|_| "Invalid IRQ index")?;
            (WaitSource::Irq, idx as u16, is_rel)
        }
        "jmppin" => {
            // PIO v1 feature
            (WaitSource::Jmppin, 0, false)
        }
        _ => return Err(format!("Unknown WAIT source: {}", source_str)),
    };

    let rel_bit = if rel { 1 << 4 } else { 0 };

    Ok(((Opcode::Wait as u16) << 13)
        | (polarity << 7)
        | ((source as u16) << 5)
        | rel_bit
        | (index & 0x1F))
}

fn encode_in(args: &[&str], defines: &HashMap<String, i32>) -> Result<u16, String> {
    if args.len() < 2 {
        return Err("IN requires source and bit count".to_string());
    }

    // Filter out commas
    let args: Vec<&str> = args.iter().filter(|&&s| s != ",").copied().collect();

    let source = parse_in_out_dest(args[0].trim_end_matches(','))?;
    let bit_count = eval_simple_expr(args[1].trim_end_matches(','), defines)
        .map_err(|_| "Invalid bit count")?;

    if !(0..=32).contains(&bit_count) {
        return Err("IN bit count must be 0-32".to_string());
    }

    // bit_count of 32 is encoded as 0
    let encoded_count = if bit_count == 32 { 0 } else { bit_count as u16 };

    Ok(((Opcode::In as u16) << 13) | ((source as u16) << 5) | (encoded_count & 0x1F))
}

fn encode_out(args: &[&str], defines: &HashMap<String, i32>) -> Result<u16, String> {
    if args.len() < 2 {
        return Err("OUT requires destination and bit count".to_string());
    }

    // Filter out commas
    let args: Vec<&str> = args.iter().filter(|&&s| s != ",").copied().collect();

    let dest = parse_in_out_dest(args[0].trim_end_matches(','))?;
    let bit_count = eval_simple_expr(args[1].trim_end_matches(','), defines)
        .map_err(|_| "Invalid bit count")?;

    if !(0..=32).contains(&bit_count) {
        return Err("OUT bit count must be 0-32".to_string());
    }

    // bit_count of 32 is encoded as 0
    let encoded_count = if bit_count == 32 { 0 } else { bit_count as u16 };

    Ok(((Opcode::Out as u16) << 13) | ((dest as u16) << 5) | (encoded_count & 0x1F))
}

fn encode_push(args: &[&str]) -> Result<u16, String> {
    let mut iffull = false;
    let mut block = true;

    // Filter out commas
    let args: Vec<&str> = args.iter().filter(|&&s| s != ",").copied().collect();

    for arg in args {
        match arg.trim_end_matches(',').to_lowercase().as_str() {
            "iffull" => iffull = true,
            "noblock" => block = false,
            "block" => block = true,
            _ => return Err(format!("Unknown PUSH modifier: {}", arg)),
        }
    }

    Ok(((Opcode::PushPull as u16) << 13) | ((iffull as u16) << 6) | ((block as u16) << 5))
}

fn encode_pull(args: &[&str]) -> Result<u16, String> {
    let mut ifempty = false;
    let mut block = true;

    // Filter out commas
    let args: Vec<&str> = args.iter().filter(|&&s| s != ",").copied().collect();

    for arg in args {
        match arg.trim_end_matches(',').to_lowercase().as_str() {
            "ifempty" => ifempty = true,
            "noblock" => block = false,
            "block" => block = true,
            _ => return Err(format!("Unknown PULL modifier: {}", arg)),
        }
    }

    Ok(((Opcode::PushPull as u16) << 13)
        | (1 << 7)
        | ((ifempty as u16) << 6)
        | ((block as u16) << 5))
}

fn encode_mov_instr(args: &[&str]) -> Result<u16, String> {
    if args.len() < 2 {
        return Err("MOV requires destination and source".to_string());
    }

    // Filter out commas
    let args: Vec<&str> = args.iter().filter(|&&s| s != ",").copied().collect();

    let dest = parse_mov_dest(args[0].trim_end_matches(','))?;

    // Check for operation prefix on source: ~, ,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,, ::
    let source_idx = 1;
    let source_str = args.get(source_idx).ok_or("MOV requires source")?;

    let (op, actual_source) = if source_str.starts_with('~') || source_str.starts_with('!') {
        // Invert operation: ~x, !x
        (MovOp::Invert, &source_str[1..])
    } else if let Some(stripped) = source_str.strip_prefix("::") {
        // Bit reverse: ::x
        (MovOp::BitReverse, stripped)
    } else if args.len() > 2 && (*source_str == "~" || *source_str == "!" || *source_str == "::") {
        // Operation as separate token
        let op = match *source_str {
            "~" | "!" => MovOp::Invert,
            "::" => MovOp::BitReverse,
            _ => MovOp::None,
        };
        (op, *args.get(2).unwrap_or(&""))
    } else {
        (MovOp::None, *source_str)
    };

    let source = parse_mov_source(actual_source.trim_end_matches(','))?;

    Ok(encode_mov(dest, op, source))
}

fn encode_mov(dest: MovDest, op: MovOp, source: MovSource) -> u16 {
    ((Opcode::Mov as u16) << 13) | ((dest as u16) << 5) | ((op as u16) << 3) | (source as u16)
}

fn encode_irq(args: &[&str], defines: &HashMap<String, i32>) -> Result<u16, String> {
    if args.is_empty() {
        return Err("IRQ requires arguments".to_string());
    }

    // Filter out commas
    let args: Vec<&str> = args.iter().filter(|&&s| s != ",").copied().collect();

    let mut clear = false;
    let mut wait = false;
    let mut rel = false;
    let mut index_value: Option<i32> = None;

    for arg in &args {
        let arg_lower = arg.to_lowercase();
        let arg_lower = arg_lower.trim_end_matches(',');

        match arg_lower {
            "clear" | "clr" => {
                clear = true;
            }
            "wait" => {
                wait = true;
            }
            "set" => {
                // Default behavior, nothing to do
            }
            "nowait" => {
                wait = false;
            }
            "rel" => {
                rel = true;
            }
            "next" | "prev" => {
                // PIO v1 features, handle differently
                rel = true;
            }
            _ => {
                // Try to parse as index
                if let Ok(idx) = eval_simple_expr(arg.trim_end_matches(','), defines) {
                    index_value = Some(idx);
                }
            }
        }
    }

    let index = index_value.ok_or("IRQ requires an index")? as u16;

    if index > 7 {
        return Err("IRQ index must be 0-7".to_string());
    }

    // Encoding:
    // Bit 6: clear (1) vs set (0)
    // Bit 5: wait
    // Bits 4:0: index (with rel in bit 4)
    let idx_with_rel = if rel { index | 0x10 } else { index };

    Ok(((Opcode::Irq as u16) << 13) | ((clear as u16) << 6) | ((wait as u16) << 5) | idx_with_rel)
}

fn encode_set(args: &[&str], defines: &HashMap<String, i32>) -> Result<u16, String> {
    if args.len() < 2 {
        return Err("SET requires destination and value".to_string());
    }

    // Filter out commas
    let args: Vec<&str> = args.iter().filter(|&&s| s != ",").copied().collect();

    let dest = parse_set_dest(args[0].trim_end_matches(','))?;
    let value = eval_simple_expr(args[1].trim_end_matches(','), defines)
        .map_err(|e| format!("Invalid SET value: {}", e))?;

    if !(0..=31).contains(&value) {
        return Err(format!("SET value {} must be 0-31", value));
    }

    Ok(((Opcode::Set as u16) << 13) | ((dest as u16) << 5) | (value as u16))
}

fn parse_in_out_dest(s: &str) -> Result<InOutDest, String> {
    match s.to_lowercase().as_ref() {
        "pins" => Ok(InOutDest::Pins),
        "x" => Ok(InOutDest::X),
        "y" => Ok(InOutDest::Y),
        "null" => Ok(InOutDest::Null),
        "pindirs" => Ok(InOutDest::Pindirs),
        "pc" => Ok(InOutDest::Pc),
        "isr" => Ok(InOutDest::IsrOsr),
        "osr" => Ok(InOutDest::IsrOsr),
        "exec" => Ok(InOutDest::Exec),
        _ => Err(format!("Unknown IN/OUT destination: {}", s)),
    }
}

fn parse_mov_dest(s: &str) -> Result<MovDest, String> {
    match s.to_lowercase().as_ref() {
        "pins" => Ok(MovDest::Pins),
        "x" => Ok(MovDest::X),
        "y" => Ok(MovDest::Y),
        "pindirs" => Ok(MovDest::Pindirs),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eval_simple_expr() {
        let mut defines = HashMap::new();
        defines.insert("T1".to_string(), 3);
        defines.insert("T2".to_string(), 3);
        defines.insert("T3".to_string(), 4);

        assert_eq!(eval_simple_expr("5", &defines).unwrap(), 5);
        assert_eq!(eval_simple_expr("T1", &defines).unwrap(), 3);
        assert_eq!(eval_simple_expr("T3 - 1", &defines).unwrap(), 3);
        assert_eq!(eval_simple_expr("T1 + T2", &defines).unwrap(), 6);
        assert_eq!(eval_simple_expr("1 << 3", &defines).unwrap(), 8);
        assert_eq!(eval_simple_expr("(1 << 3)", &defines).unwrap(), 8);
    }

    #[test]
    fn test_compile_blink_program() {
        let source = r#"
            .program blink
                pull block
                out y, 32
            .wrap_target
                mov x, y
                set pins, 1   ; Turn LED on
            lp1:
                jmp x-- lp1   ; Delay
                mov x, y
                set pins, 0   ; Turn LED off
            lp2:
                jmp x-- lp2   ; Delay
            .wrap
        "#;
        let lines: Vec<String> = source.lines().map(|s| s.to_string()).collect();

        let program = compile_program(&lines).unwrap();
        assert_eq!(program.instructions.len(), 8);
        assert_eq!(program.wrap_target, Some(2));
        assert_eq!(program.wrap, Some(7));
    }

    #[test]
    fn test_jmp_conditions() {
        let mut labels = HashMap::new();
        labels.insert("target".to_string(), 5);

        // Test x-- condition
        let instr = encode_jmp(&["x--", "target"], &labels, 0).unwrap();
        assert_eq!(instr & 0xE000, 0x0000); // JMP opcode
        assert_eq!((instr >> 5) & 0x7, 0b010); // x-- condition
        assert_eq!(instr & 0x1F, 5); // target address

        // Test !x condition
        let instr = encode_jmp(&["!x", "target"], &labels, 0).unwrap();
        assert_eq!((instr >> 5) & 0x7, 0b001); // !x condition
    }

    #[test]
    fn test_ws2812_sideset() {
        let source = r#"
            .program ws2812
            .side_set 1
            .define T1 3
            .define T2 3
            .define T3 4
            .wrap_target
            bitloop:
                out x, 1       side 0 [T3 - 1]
                jmp !x do_zero side 1 [T1 - 1]
            do_one:
                jmp  bitloop   side 1 [T2 - 1]
            do_zero:
                nop            side 0 [T2 - 1]
            .wrap
        "#;
        let lines: Vec<String> = source.lines().map(|s| s.to_string()).collect();

        let program = compile_program(&lines).unwrap();
        assert_eq!(program.side_set_count, 1);
        assert_eq!(program.instructions.len(), 4);
    }

    #[test]
    fn test_inline_labels() {
        let source = r#"
            start: set pins, 1
            loop: jmp loop
        "#;
        let lines: Vec<String> = source.lines().map(|s| s.to_string()).collect();

        let program = compile_program(&lines).unwrap();
        assert_eq!(program.instructions.len(), 2);
    }
}
