//! DCP (Double-precision Co-Processor) comprehensive test
//!
//! Tests all DCP operations on real hardware and logs results via defmt RTT.
//! Each test prints PASS/FAIL with actual vs expected values.

#![no_std]
#![no_main]

#[cfg(feature = "defmt")]
use defmt_rtt as _;
use rp2350_examples::log::*;

use rp2350::dcp::Dcp;
use rp2350::main; // ==========================================================================
// Test helpers
// ==========================================================================

static mut PASSED: u32 = 0;
static mut FAILED: u32 = 0;

fn inc_passed() {
    unsafe {
        PASSED += 1;
    }
}
fn inc_failed() {
    unsafe {
        FAILED += 1;
    }
}
fn passed() -> u32 {
    unsafe { PASSED }
}
fn failed() -> u32 {
    unsafe { FAILED }
}

fn check_f32(name: &str, actual: f32, expected: f32) {
    let ok = actual == expected || (actual.is_nan() && expected.is_nan());
    if ok {
        inc_passed();
        info!(
            "  PASS {}: {=f32} (expected {=f32})",
            name, actual, expected
        );
    } else {
        inc_failed();
        error!(
            "  FAIL {}: got {=f32} (0x{:08x}), expected {=f32} (0x{:08x})",
            name,
            actual,
            actual.to_bits(),
            expected,
            expected.to_bits()
        );
    }
}

fn check_f32_approx(name: &str, actual: f32, expected: f32, tolerance: f32) {
    let diff = (actual - expected).abs();
    let ok = diff <= tolerance || (actual.is_nan() && expected.is_nan());
    if ok {
        inc_passed();
        info!(
            "  PASS {}: {=f32} (expected {=f32}, diff {=f32})",
            name, actual, expected, diff
        );
    } else {
        inc_failed();
        error!(
            "  FAIL {}: got {=f32} (0x{:08x}), expected {=f32} (0x{:08x}), diff {=f32}",
            name,
            actual,
            actual.to_bits(),
            expected,
            expected.to_bits(),
            diff
        );
    }
}

fn check_f64(name: &str, actual: f64, expected: f64) {
    let ok = actual == expected || (actual.is_nan() && expected.is_nan());
    if ok {
        inc_passed();
        info!("  PASS {}: {=f64}", name, actual);
    } else {
        inc_failed();
        error!(
            "  FAIL {}: got {=f64} (0x{:016x}), expected {=f64} (0x{:016x})",
            name,
            actual,
            actual.to_bits(),
            expected,
            expected.to_bits()
        );
    }
}

fn check_f64_approx(name: &str, actual: f64, expected: f64, tolerance: f64) {
    let diff = (actual - expected).abs();
    let ok = diff <= tolerance || (actual.is_nan() && expected.is_nan());
    if ok {
        inc_passed();
        info!(
            "  PASS {}: {=f64} (expected {=f64}, diff {=f64})",
            name, actual, expected, diff
        );
    } else {
        inc_failed();
        error!(
            "  FAIL {}: got {=f64} (0x{:016x}), expected {=f64} (0x{:016x}), diff {=f64}",
            name,
            actual,
            actual.to_bits(),
            expected,
            expected.to_bits(),
            diff
        );
    }
}

fn check_bool(name: &str, actual: bool, expected: bool) {
    if actual == expected {
        inc_passed();
        info!("  PASS {}: {} (expected {})", name, actual, expected);
    } else {
        inc_failed();
        error!("  FAIL {}: got {}, expected {}", name, actual, expected);
    }
}

fn check_u32(name: &str, actual: u32, expected: u32) {
    if actual == expected {
        inc_passed();
        info!("  PASS {}: 0x{:08x}", name, actual);
    } else {
        inc_failed();
        error!(
            "  FAIL {}: got 0x{:08x}, expected 0x{:08x}",
            name, actual, expected
        );
    }
}

// ==========================================================================
// Test sections
// ==========================================================================

fn test_f32_arithmetic(dcp: &Dcp) {
    info!("--- f32 Arithmetic ---");
    check_f32("fadd 1+2", dcp.fadd(1.0, 2.0), 3.0);
    check_f32("fadd -1+1", dcp.fadd(-1.0, 1.0), 0.0);
    check_f32("fadd 0.1+0.2", dcp.fadd(0.1, 0.2), 0.3);
    check_f32("fsub 3-1", dcp.fsub(3.0, 1.0), 2.0);
    check_f32("fsub 1-2", dcp.fsub(1.0, 2.0), -1.0);
    check_f32("fmul 2*3", dcp.fmul(2.0, 3.0), 6.0);
    check_f32("fmul -2*3", dcp.fmul(-2.0, 3.0), -6.0);
    check_f32("fdiv 6/2", dcp.fdiv(6.0, 2.0), 3.0);
    check_f32("fdiv 1/3", dcp.fdiv(1.0, 3.0), 1.0 / 3.0);
    check_f32_approx("fdiv_fast 6/2", dcp.fdiv_fast(6.0, 2.0), 3.0, 0.01);
    check_f32_approx("fdiv_fast 1/3", dcp.fdiv_fast(1.0, 3.0), 1.0 / 3.0, 0.01);
}

fn test_f32_sqrt(dcp: &Dcp) {
    info!("--- f32 Sqrt ---");
    check_f32("fsqrt 4", dcp.fsqrt(4.0), 2.0);
    check_f32("fsqrt 2", dcp.fsqrt(2.0), core::f32::consts::SQRT_2);
    check_f32("fsqrt 0", dcp.fsqrt(0.0), 0.0);
    check_f32_approx("fsqrt_fast 4", dcp.fsqrt_fast(4.0), 2.0, 0.01);
    check_f32_approx(
        "fsqrt_fast 2",
        dcp.fsqrt_fast(2.0),
        core::f32::consts::SQRT_2,
        0.01,
    );
}

fn test_f32_compare(dcp: &Dcp) {
    info!("--- f32 Compare ---");
    check_bool("fcmp_eq 1==1", dcp.fcmp_eq(1.0, 1.0), true);
    check_bool("fcmp_eq 1==2", dcp.fcmp_eq(1.0, 2.0), false);
    check_bool("fcmp_lt 1<2", dcp.fcmp_lt(1.0, 2.0), true);
    check_bool("fcmp_lt 2<1", dcp.fcmp_lt(2.0, 1.0), false);
    check_bool("fcmp_le 1<=2", dcp.fcmp_le(1.0, 2.0), true);
    check_bool("fcmp_le 1<=1", dcp.fcmp_le(1.0, 1.0), true);
    check_bool("fcmp_le 2<=1", dcp.fcmp_le(2.0, 1.0), false);
    check_bool("fcmp_gt 2>1", dcp.fcmp_gt(2.0, 1.0), true);
    check_bool("fcmp_gt 1>2", dcp.fcmp_gt(1.0, 2.0), false);
    check_bool("fcmp_ge 2>=1", dcp.fcmp_ge(2.0, 1.0), true);
    check_bool("fcmp_ge 1>=1", dcp.fcmp_ge(1.0, 1.0), true);
    check_bool("fcmp_unord NaN", dcp.fcmp_unordered(f32::NAN, 1.0), true);
    check_bool("fcmp_eq NaN==1", dcp.fcmp_eq(f32::NAN, 1.0), false);
    check_bool("fcmp_eq 0==-0", dcp.fcmp_eq(0.0, -0.0), true);

    info!("--- f32 Raw Compare Bits ---");
    info!(
        "  fcmp(1,1)   = 0x{:08x} (expect Z=1, bit30)",
        dcp.fcmp(1.0, 1.0)
    );
    info!(
        "  fcmp(1,2)   = 0x{:08x} (expect C=0,Z=0,V=0)",
        dcp.fcmp(1.0, 2.0)
    );
    info!(
        "  fcmp(2,1)   = 0x{:08x} (expect C=1,Z=0,V=0)",
        dcp.fcmp(2.0, 1.0)
    );
    info!(
        "  fcmp(NaN,1) = 0x{:08x} (expect V=1, bit28)",
        dcp.fcmp(f32::NAN, 1.0)
    );
}

fn test_f32_classify(dcp: &Dcp) {
    info!("--- f32 Classify ---");
    info!("  classify(0.0)     = 0x{:08x}", dcp.fclassify(0.0));
    info!("  classify(-0.0)    = 0x{:08x}", dcp.fclassify(-0.0));
    info!("  classify(1.0)     = 0x{:08x}", dcp.fclassify(1.0));
    info!("  classify(-1.0)    = 0x{:08x}", dcp.fclassify(-1.0));
    info!(
        "  classify(inf)     = 0x{:08x}",
        dcp.fclassify(f32::INFINITY)
    );
    info!(
        "  classify(-inf)    = 0x{:08x}",
        dcp.fclassify(f32::NEG_INFINITY)
    );
    info!("  classify(NaN)     = 0x{:08x}", dcp.fclassify(f32::NAN));
    info!(
        "  classify(subnorm) = 0x{:08x}",
        dcp.fclassify(f32::from_bits(0x00000001))
    );
}

fn test_f32_conversions(dcp: &Dcp) {
    info!("--- f32 Conversions ---");
    check_f32("int2float 42", dcp.int2float(42), 42.0);
    check_f32("int2float -7", dcp.int2float(-7), -7.0);
    check_f32("uint2float 42", dcp.uint2float(42), 42.0);
    check_f32(
        "uint2float 2^31",
        dcp.uint2float(0x8000_0000),
        2_147_483_648.0,
    );
    check_u32("float2int 3.7", dcp.float2int(3.7) as u32, 3);
    check_u32("float2int -3.7", dcp.float2int(-3.7) as u32, (-3i32) as u32);
    check_u32("float2int_r 3.7", dcp.float2int_r(3.7) as u32, 4);
    check_u32("float2int_r 2.5", dcp.float2int_r(2.5) as u32, 2);
    check_u32("float2uint 3.7", dcp.float2uint(3.7), 3);
    check_u32("float2uint_r 3.7", dcp.float2uint_r(3.7), 4);
}

fn test_f64_arithmetic(dcp: &Dcp) {
    info!("--- f64 Arithmetic ---");
    check_f64("dadd 1+2", dcp.dadd(1.0, 2.0), 3.0);
    check_f64("dadd -1+1", dcp.dadd(-1.0, 1.0), 0.0);
    check_f64("dsub 3-1", dcp.dsub(3.0, 1.0), 2.0);
    check_f64("dsub 1-2", dcp.dsub(1.0, 2.0), -1.0);
    check_f64("dmul 2*3", dcp.dmul(2.0, 3.0), 6.0);
    check_f64("dmul -2*3", dcp.dmul(-2.0, 3.0), -6.0);
    check_f64("ddiv 6/2", dcp.ddiv(6.0, 2.0), 3.0);
    check_f64("ddiv 1/3", dcp.ddiv(1.0, 3.0), 1.0 / 3.0);
    check_f64_approx("ddiv_fast 6/2", dcp.ddiv_fast(6.0, 2.0), 3.0, 0.01);
    check_f64_approx("ddiv_fast 1/3", dcp.ddiv_fast(1.0, 3.0), 1.0 / 3.0, 0.01);
}

fn test_f64_sqrt(dcp: &Dcp) {
    info!("--- f64 Sqrt ---");
    check_f64("dsqrt 4", dcp.dsqrt(4.0), 2.0);
    check_f64("dsqrt 2", dcp.dsqrt(2.0), core::f64::consts::SQRT_2);
    check_f64("dsqrt 0", dcp.dsqrt(0.0), 0.0);
    check_f64_approx("dsqrt_fast 4", dcp.dsqrt_fast(4.0), 2.0, 0.01);
    check_f64_approx(
        "dsqrt_fast 2",
        dcp.dsqrt_fast(2.0),
        core::f64::consts::SQRT_2,
        0.01,
    );
}

fn test_f64_compare(dcp: &Dcp) {
    info!("--- f64 Compare ---");
    check_bool("dcmp_eq 1==1", dcp.dcmp_eq(1.0, 1.0), true);
    check_bool("dcmp_eq 1==2", dcp.dcmp_eq(1.0, 2.0), false);
    check_bool("dcmp_lt 1<2", dcp.dcmp_lt(1.0, 2.0), true);
    check_bool("dcmp_lt 2<1", dcp.dcmp_lt(2.0, 1.0), false);
    check_bool("dcmp_le 1<=2", dcp.dcmp_le(1.0, 2.0), true);
    check_bool("dcmp_le 1<=1", dcp.dcmp_le(1.0, 1.0), true);
    check_bool("dcmp_gt 2>1", dcp.dcmp_gt(2.0, 1.0), true);
    check_bool("dcmp_ge 2>=1", dcp.dcmp_ge(2.0, 1.0), true);
    check_bool("dcmp_unord NaN", dcp.dcmp_unordered(f64::NAN, 1.0), true);
    check_bool("dcmp_eq NaN==1", dcp.dcmp_eq(f64::NAN, 1.0), false);

    info!("--- f64 Raw Compare Bits ---");
    info!(
        "  dcmp(1,1)   = 0x{:08x} (expect Z=1, bit30)",
        dcp.dcmp(1.0, 1.0)
    );
    info!(
        "  dcmp(1,2)   = 0x{:08x} (expect C=0,Z=0,V=0)",
        dcp.dcmp(1.0, 2.0)
    );
    info!(
        "  dcmp(NaN,1) = 0x{:08x} (expect V=1, bit28)",
        dcp.dcmp(f64::NAN, 1.0)
    );
}

fn test_f64_classify(dcp: &Dcp) {
    info!("--- f64 Classify ---");
    info!("  dclassify(0.0)  = 0x{:08x}", dcp.dclassify(0.0));
    info!("  dclassify(1.0)  = 0x{:08x}", dcp.dclassify(1.0));
    info!("  dclassify(-1.0) = 0x{:08x}", dcp.dclassify(-1.0));
    info!("  dclassify(inf)  = 0x{:08x}", dcp.dclassify(f64::INFINITY));
    info!("  dclassify(NaN)  = 0x{:08x}", dcp.dclassify(f64::NAN));
}

fn test_conversions_between(dcp: &Dcp) {
    info!("--- Conversions Between Types ---");
    check_f32("f2d2f 3.15", dcp.double2float(dcp.float2double(3.15)), 3.15);
    check_f32("f2d2f -0.5", dcp.double2float(dcp.float2double(-0.5)), -0.5);
    let v: i32 = 123_456_789;
    check_u32("i2d2i", dcp.double2int(dcp.int2double(v)) as u32, v as u32);
    let uv: u32 = 3_000_000_000;
    check_u32("u2d2u", dcp.double2uint(dcp.uint2double(uv)), uv);
    let pi = core::f64::consts::PI;
    let pi_f32 = dcp.double2float(pi);
    let pi_back = dcp.float2double(pi_f32);
    info!(
        "  d2f2d pi: {=f64} -> {=f32} -> {=f64} (loss={=f64})",
        pi,
        pi_f32,
        pi_back,
        (pi - pi_back).abs()
    );
}

fn test_fxprod(dcp: &Dcp) {
    info!("--- Extended Product ---");
    let (lo, hi) = dcp.fxprod(1.0, 1.0);
    info!("  fxprod(1,1)  = lo=0x{:08x} hi=0x{:08x}", lo, hi);
    let (lo, hi) = dcp.fxprod(2.0, 0.5);
    info!("  fxprod(2,0.5)= lo=0x{:08x} hi=0x{:08x}", lo, hi);
    let (lo, hi) = dcp.fxprod(3.0, 7.0);
    info!("  fxprod(3,7)  = lo=0x{:08x} hi=0x{:08x}", lo, hi);
}

// ==========================================================================
// Entry point
// ==========================================================================

#[main]
fn main() -> ! {
    info!("=== DCP Comprehensive Test ===");

    let dcp = unsafe { Dcp::steal() };

    test_f32_arithmetic(&dcp);
    test_f32_sqrt(&dcp);
    test_f32_compare(&dcp);
    test_f32_classify(&dcp);
    test_f32_conversions(&dcp);
    test_fxprod(&dcp);

    test_f64_arithmetic(&dcp);
    test_f64_sqrt(&dcp);
    test_f64_compare(&dcp);
    test_f64_classify(&dcp);
    test_conversions_between(&dcp);

    info!("==================================");
    info!("Results: {} passed, {} failed", passed(), failed());
    if failed() == 0 {
        info!("ALL TESTS PASSED!");
    } else {
        error!("{} TESTS FAILED!", failed());
    }
    info!("==================================");

    loop {
        core::hint::spin_loop();
    }
}
