//! Shared color / timing helpers for the LED examples.
//!
//! Previously these helpers were copy-pasted into every LED example
//! (`ws2812.rs`, `sk9822_spi.rs`, `sk9822_pio.rs`). They now live in one
//! place so the three examples stay consistent.

/// Simple HSV to RGB conversion (all values 0-255).
///
/// `h` is the hue (0-255 wraps around the color wheel), `s` the
/// saturation and `v` the value/brightness.
#[inline]
pub fn hsv_to_rgb(h: u8, s: u8, v: u8) -> (u8, u8, u8) {
    if s == 0 {
        return (v, v, v);
    }

    let region = h / 43;
    let remainder = (h as u16 - region as u16 * 43) * 6;

    let p = ((v as u16 * (255 - s as u16)) >> 8) as u8;
    let q = ((v as u16 * (255 - ((s as u16 * remainder) >> 8))) >> 8) as u8;
    let t = ((v as u16 * (255 - ((s as u16 * (255 - remainder)) >> 8))) >> 8) as u8;

    match region {
        0 => (v, t, p),
        1 => (q, v, p),
        2 => (p, v, t),
        3 => (p, q, v),
        4 => (t, p, v),
        _ => (v, p, q),
    }
}

/// Pack RGB into the GRB 32-bit word expected by the WS2812 PIO program.
///
/// The PIO program shifts out MSB first. We put GRB in the upper 24 bits
/// so that autopull with 24-bit threshold works correctly after a left shift.
#[inline]
pub const fn urgb_u32(r: u8, g: u8, b: u8) -> u32 {
    ((g as u32) << 24) | ((r as u32) << 16) | ((b as u32) << 8)
}

/// Compute the PIO clock divider for the desired WS2812 bit frequency.
///
/// The PIO program uses 10 cycles per bit (T1 + T2 + T3 = 2 + 5 + 3), so
/// `clk_div = sys_clk / (ws2812_freq * 10)`.
pub fn ws2812_clkdiv(sys_clk: u32, ws2812_freq: u32) -> (u16, u8) {
    // cycles_per_bit = 10
    let target = sys_clk / (ws2812_freq * 10);
    let frac_256 = ((sys_clk as u64 * 256) / (ws2812_freq as u64 * 10)) as u32;
    let int_div = target as u16;
    let frac_div = (frac_256 & 0xFF) as u8;
    (int_div, frac_div)
}
