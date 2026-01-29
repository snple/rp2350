# PIO Assembly Macro (`pio_asm!`)

The `pio_asm!` macro compiles PIO assembly code at compile time into machine code that can be loaded into PIO instruction memory.

## Features

- Full support for `.pio` file syntax
- Compile-time assembly with error checking
- Support for labels, directives, and comments
- Compatible with Raspberry Pi Pico SDK `.pio` files

## Basic Usage

```rust
use pico_rs::pio_asm;

let program = pio_asm!(
    "set pindirs, 1",
    "set pins, 1",
    "set pins, 0"
);
```

## Full .pio Syntax

The macro supports the complete `.pio` file syntax:

```rust
let program = pio_asm!(
    "; Comments start with semicolon",
    "",
    ".pio_version 0",
    "",
    ".program blink",
    "    pull block",
    "    out y, 32",
    ".wrap_target",
    "    mov x, y",
    "    set pins, 1   ; Turn LED on",
    "lp1:",
    "    jmp x-- lp1   ; Delay loop",
    "    mov x, y",
    "    set pins, 0   ; Turn LED off",
    "lp2:",
    "    jmp x-- lp2",
    ".wrap"
);
```

## Supported Directives

- `.program <name>` - Define program name
- `.wrap_target` - Mark start of wrap region
- `.wrap` - Mark end of wrap region (wraps to .wrap_target)
- `.origin <addr>` - Specify fixed origin address
- `.side_set <count> [opt] [pindirs]` - Configure side-set
- `.pio_version <n>` - Specify PIO version (informational)
- `.define <name> <value>` - Define constants (TODO)

## Supported Instructions

### Data Transfer

- `mov <dest>, <src>` - Move data
  - Operations: `mov x, ~y` (invert), `mov x, ::y` (bit reverse)
- `in <src>, <count>` - Shift in from source
- `out <dest>, <count>` - Shift out to destination
- `pull [block] [ifempty]` - Pull from FIFO
- `push [block] [iffull]` - Push to FIFO

### Control Flow

- `jmp <label>` - Unconditional jump
- `jmp <cond>, <label>` - Conditional jump
  - Conditions: `!x`, `x--`, `!y`, `y--`, `x!=y`, `pin`, `!osre`
- `wait <polarity>, <source>, <index>` - Wait for condition
  - Sources: `gpio`, `pin`, `irq`

### Pin Control

- `set <dest>, <value>` - Set pins or registers
  - Destinations: `pins`, `x`, `y`, `pindirs`

### Interrupts

- `irq <index> [clear] [wait]` - Set/clear IRQ

### Misc

- `nop` - No operation (encoded as `mov y, y`)

## Delay and Side-set

Instructions can include delay and side-set:

```rust
pio_asm!(
    ".side_set 1",
    "set pins, 1 side 1 [31]",  // side-set to 1, delay 31 cycles
    "set pins, 0 side 0 [31]"   // side-set to 0, delay 31 cycles
)
```

## Program Structure

The macro returns a `PioProgram` struct:

```rust
struct PioProgram {
    instructions: &'static [u16],  // Compiled instructions
    wrap_target: usize,             // Wrap target address
    wrap: usize,                    // Wrap address
    origin: Option<usize>,          // Fixed origin (if specified)
    side_set_count: u32,            // Number of side-set bits
    side_set_opt: bool,             // Side-set is optional
    side_set_pindirs: bool,         // Side-set affects pindirs
}
```

## Example: Installing and Running

```rust
use pico_rs::pio::{self, PioBlock};
use pico_rs::pio_asm;

// Compile program
let program = pio_asm!(
    ".program test",
    ".wrap_target",
    "set pins, 1 [31]",
    "set pins, 0 [31]",
    ".wrap"
);

// Initialize PIO
let mut pio = pio::Pio0::take();

// Install program
let offset = pio.install_program(program.instructions, program.origin)
    .expect("Failed to install program");

// Configure state machine
let sm = pio.state_machine(0);
sm.set_wrap(program.wrap_target as u8, program.wrap as u8);
sm.set_set_pins(25, 1);  // GPIO 25

// Start
sm.restart();
sm.set_enabled(true);
```

## Differences from SDK

- No C code blocks (`% c-sdk { ... }`)
- No `.define` support yet
- All compilation happens at Rust compile time
- No separate `.pio` files needed (inline in Rust code)

## Error Messages

Compilation errors are reported at Rust compile time:

```rust
// This will fail to compile:
let bad = pio_asm!(
    "jmp unknown_label"  // Error: Unknown label: unknown_label
);
```

## Converting from SDK .pio Files

To convert SDK `.pio` files to `pio_asm!`:

1. Copy the program instructions into string literals
2. Remove or ignore C code blocks (`% c-sdk { ... }`)
3. Wrap in `pio_asm!(...)`

```rust
// From: blink.pio
let blink = pio_asm!(
    ".program blink",
    "    pull block",
    "    out y, 32",
    ".wrap_target",
    "    mov x, y",
    "    set pins, 1",
    "lp1:",
    "    jmp x-- lp1",
    "    mov x, y",
    "    set pins, 0",
    "lp2:",
    "    jmp x-- lp2",
    ".wrap"
);
```
