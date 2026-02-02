//! Cortex-M33 Core Register Access
//!
//! This module provides access to the processor's special registers:
//!
//! - `primask` - Priority Mask Register (interrupt control)
//! - `basepri` - Base Priority Register (ARMv7-M/v8-M Mainline only)
//! - `faultmask` - Fault Mask Register (ARMv7-M/v8-M Mainline only)
//! - `control` - Control Register (stack pointer selection, privilege)
//! - `msp` - Main Stack Pointer
//! - `psp` - Process Stack Pointer
//! - `msplim` - Main Stack Pointer Limit (ARMv8-M only)
//! - `psplim` - Process Stack Pointer Limit (ARMv8-M only)
//! - `nsacr` - Non-Secure Access Control Register (ARMv8-M TrustZone)

pub mod basepri;
pub mod control;
pub mod faultmask;
pub mod msp;
pub mod msplim;
pub mod nsacr;
pub mod primask;
pub mod psp;
pub mod psplim;
