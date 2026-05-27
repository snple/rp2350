//! Binary Info support for RP2350
//!
//! Binary info is embedded machine readable information with the binary in flash.
//! This can be used by tools like `picotool` to identify programs.
//!
//! ## Features
//!
//! - Program name, version, description, URL
//! - Build date and attributes
//! - Pin assignments with functions
//! - Custom features/tags
//!
//! ## Example
//!
//! ```ignore
//! use roschi::binary_info;
//!
//! // Declare program info
//! binary_info::program_name!("My Application");
//! binary_info::program_version!("1.0.0");
//! binary_info::program_description!("An example RP2350 application");
//! binary_info::program_url!("https://github.com/example/app");
//! binary_info::program_feature!("LED blink");
//!
//! // Declare pin usage
//! binary_info::pin_with_func!(25, Sio);  // LED pin
//! binary_info::pin_with_name!(25, "LED");
//! ```

/// Binary Info types
#[repr(u16)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum BinaryInfoType {
    /// Raw data bytes
    RawData = 1,
    /// Sized data with length prefix
    SizedData = 2,
    /// Null-terminated list of binary info pointers
    ListZeroTerminated = 3,
    /// BSON data
    Bson = 4,
    /// ID and integer value
    IdAndInt = 5,
    /// ID and string value
    IdAndString = 6,
    /// Block device descriptor
    BlockDevice = 7,
    /// Pins with GPIO function (32-bit encoding)
    PinsWithFunc = 8,
    /// Pins with custom name (32-bit mask)
    PinsWithName = 9,
    /// Named group
    NamedGroup = 10,
    /// Pointer to int32 with name
    PtrInt32WithName = 11,
    /// Pointer to string with name
    PtrStringWithName = 12,
    /// Pins with GPIO function (64-bit encoding)
    Pins64WithFunc = 13,
    /// Pins with custom name (64-bit mask)
    Pins64WithName = 14,
}

/// Create a tag from two characters
#[inline]
pub const fn make_tag(c1: u8, c2: u8) -> u16 {
    ((c2 as u16) << 8) | (c1 as u16)
}

/// Raspberry Pi official tag - do not use for custom entries
pub const TAG_RASPBERRY_PI: u16 = make_tag(b'R', b'P');

/// Binary Info magic markers
pub const MARKER_START: u32 = 0x7188ebf2;
pub const MARKER_END: u32 = 0xe71aa390;

// Raspberry Pi defined IDs
/// ID for program name
pub const ID_RP_PROGRAM_NAME: u32 = 0x02031c86;
/// ID for program version string
pub const ID_RP_PROGRAM_VERSION_STRING: u32 = 0x11a9bc3a;
/// ID for program build date string
pub const ID_RP_PROGRAM_BUILD_DATE_STRING: u32 = 0x9da22254;
/// ID for binary end address
pub const ID_RP_BINARY_END: u32 = 0x68f465de;
/// ID for program URL
pub const ID_RP_PROGRAM_URL: u32 = 0x1856239a;
/// ID for program description
pub const ID_RP_PROGRAM_DESCRIPTION: u32 = 0xb6a07c19;
/// ID for program feature
pub const ID_RP_PROGRAM_FEATURE: u32 = 0xa1f4b453;
/// ID for program build attribute
pub const ID_RP_PROGRAM_BUILD_ATTRIBUTE: u32 = 0x4275f0d3;
/// ID for SDK version
pub const ID_RP_SDK_VERSION: u32 = 0x5360b3ab;
/// ID for Pico board identifier
pub const ID_RP_PICO_BOARD: u32 = 0xb63cffbb;
/// ID for boot2 name
pub const ID_RP_BOOT2_NAME: u32 = 0x7f8882e1;

/// Pin encoding: individual pins
pub const BI_PINS_ENCODING_MULTI: u32 = 2;
/// Pin encoding: pin range
pub const BI_PINS_ENCODING_RANGE: u32 = 1;

/// Core binary info structure
#[repr(C)]
#[derive(Clone, Copy)]
pub struct BinaryInfoCore {
    pub info_type: u16,
    pub tag: u16,
}

/// Binary info with ID and integer value
#[repr(C)]
#[derive(Clone, Copy)]
pub struct BinaryInfoIdAndInt {
    pub core: BinaryInfoCore,
    pub id: u32,
    pub value: i32,
}

/// Binary info with ID and string value
#[repr(C)]
#[derive(Clone, Copy)]
pub struct BinaryInfoIdAndString {
    pub core: BinaryInfoCore,
    pub id: u32,
    pub value: *const u8,
}

// SAFETY: BinaryInfoIdAndString only contains read-only pointers to static strings
unsafe impl Sync for BinaryInfoIdAndString {}
unsafe impl Send for BinaryInfoIdAndString {}

/// Binary info for pins with function (32-bit encoding)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct BinaryInfoPinsWithFunc {
    pub core: BinaryInfoCore,
    /// Pin encoding: see encoding functions below
    pub pin_encoding: u32,
}

/// Binary info for pins with name (32-bit mask)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct BinaryInfoPinsWithName {
    pub core: BinaryInfoCore,
    pub pin_mask: u32,
    pub label: *const u8,
}

// SAFETY: BinaryInfoPinsWithName only contains read-only pointers to static strings
unsafe impl Sync for BinaryInfoPinsWithName {}
unsafe impl Send for BinaryInfoPinsWithName {}

/// Binary info for pins with function (64-bit encoding for RP2350B)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct BinaryInfoPins64WithFunc {
    pub core: BinaryInfoCore,
    pub pin_encoding: u64,
}

/// Binary info for pins with name (64-bit mask for RP2350B)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct BinaryInfoPins64WithName {
    pub core: BinaryInfoCore,
    pub pin_mask: u64,
    pub label: *const u8,
}

// SAFETY: BinaryInfoPins64WithName only contains read-only pointers to static strings
unsafe impl Sync for BinaryInfoPins64WithName {}
unsafe impl Send for BinaryInfoPins64WithName {}

/// GPIO Function identifiers for pin encoding
#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum GpioFunc {
    /// SPI function
    Spi = 1,
    /// UART function
    Uart = 2,
    /// I2C function
    I2c = 3,
    /// PWM function
    Pwm = 4,
    /// SIO (software I/O) function
    Sio = 5,
    /// PIO0 function
    Pio0 = 6,
    /// PIO1 function
    Pio1 = 7,
    /// PIO2 function (RP2350 only)
    Pio2 = 8,
    /// XIP (execute in place) function
    Xip = 9,
    /// USB function
    Usb = 10,
    /// HSTX function (RP2350 only)
    Hstx = 12,
}

// ============================================================================
// Pin Encoding Functions (32-bit for RP2350A)
// ============================================================================

/// Encode a single pin with function (32-bit)
///
/// Format: p0_5 : func_4 : 010_3
#[inline]
pub const fn encode_1pin_with_func(p0: u32, func: GpioFunc) -> u32 {
    BI_PINS_ENCODING_MULTI | ((func as u32) << 3) | (p0 << 7) | (p0 << 12)
}

/// Encode two pins with function (32-bit)
#[inline]
pub const fn encode_2pins_with_func(p0: u32, p1: u32, func: GpioFunc) -> u32 {
    BI_PINS_ENCODING_MULTI | ((func as u32) << 3) | (p0 << 7) | (p1 << 12) | (p1 << 17)
}

/// Encode three pins with function (32-bit)
#[inline]
pub const fn encode_3pins_with_func(p0: u32, p1: u32, p2: u32, func: GpioFunc) -> u32 {
    BI_PINS_ENCODING_MULTI | ((func as u32) << 3) | (p0 << 7) | (p1 << 12) | (p2 << 17) | (p2 << 22)
}

/// Encode four pins with function (32-bit)
#[inline]
pub const fn encode_4pins_with_func(p0: u32, p1: u32, p2: u32, p3: u32, func: GpioFunc) -> u32 {
    BI_PINS_ENCODING_MULTI
        | ((func as u32) << 3)
        | (p0 << 7)
        | (p1 << 12)
        | (p2 << 17)
        | (p3 << 22)
        | (p3 << 27)
}

/// Encode five pins with function (32-bit)
#[inline]
pub const fn encode_5pins_with_func(
    p0: u32,
    p1: u32,
    p2: u32,
    p3: u32,
    p4: u32,
    func: GpioFunc,
) -> u32 {
    BI_PINS_ENCODING_MULTI
        | ((func as u32) << 3)
        | (p0 << 7)
        | (p1 << 12)
        | (p2 << 17)
        | (p3 << 22)
        | (p4 << 27)
}

/// Encode a pin range with function (32-bit)
///
/// Format: phi_5 : plo_5 : func_4 : 001_3
#[inline]
pub const fn encode_pin_range_with_func(plo: u32, phi: u32, func: GpioFunc) -> u32 {
    BI_PINS_ENCODING_RANGE | ((func as u32) << 3) | (plo << 7) | (phi << 12)
}

// ============================================================================
// Pin Encoding Functions (64-bit for RP2350B)
// ============================================================================

/// Encode a single pin with function (64-bit)
#[inline]
pub const fn encode_1pin_with_func_64(p0: u32, func: GpioFunc) -> u64 {
    (BI_PINS_ENCODING_MULTI as u64)
        | (((func as u32) << 3) as u64)
        | ((p0 as u64) << 8)
        | ((p0 as u64) << 16)
}

/// Encode two pins with function (64-bit)
#[inline]
pub const fn encode_2pins_with_func_64(p0: u32, p1: u32, func: GpioFunc) -> u64 {
    (BI_PINS_ENCODING_MULTI as u64)
        | (((func as u32) << 3) as u64)
        | ((p0 as u64) << 8)
        | ((p1 as u64) << 16)
        | ((p1 as u64) << 24)
}

/// Encode a pin range with function (64-bit)
#[inline]
pub const fn encode_pin_range_with_func_64(plo: u32, phi: u32, func: GpioFunc) -> u64 {
    (BI_PINS_ENCODING_RANGE as u64)
        | (((func as u32) << 3) as u64)
        | ((plo as u64) << 8)
        | ((phi as u64) << 16)
}

// ============================================================================
// Macros for declaring binary info
// ============================================================================

/// Declare a program name
///
/// # Example
/// ```ignore
/// binary_info::program_name!("My App");
/// ```
#[macro_export]
macro_rules! bi_program_name {
    ($name:literal) => {
        #[unsafe(link_section = ".bi_entries")]
        #[used]
        static _BI_PROGRAM_NAME: $crate::binary_info::BinaryInfoIdAndString =
            $crate::binary_info::BinaryInfoIdAndString {
                core: $crate::binary_info::BinaryInfoCore {
                    info_type: $crate::binary_info::BinaryInfoType::IdAndString as u16,
                    tag: $crate::binary_info::TAG_RASPBERRY_PI,
                },
                id: $crate::binary_info::ID_RP_PROGRAM_NAME,
                value: concat!($name, "\0").as_ptr(),
            };
    };
}

/// Declare a program version string
///
/// # Example
/// ```ignore
/// binary_info::program_version!("1.0.0");
/// ```
#[macro_export]
macro_rules! bi_program_version {
    ($version:literal) => {
        #[unsafe(link_section = ".bi_entries")]
        #[used]
        static _BI_PROGRAM_VERSION: $crate::binary_info::BinaryInfoIdAndString =
            $crate::binary_info::BinaryInfoIdAndString {
                core: $crate::binary_info::BinaryInfoCore {
                    info_type: $crate::binary_info::BinaryInfoType::IdAndString as u16,
                    tag: $crate::binary_info::TAG_RASPBERRY_PI,
                },
                id: $crate::binary_info::ID_RP_PROGRAM_VERSION_STRING,
                value: concat!($version, "\0").as_ptr(),
            };
    };
}

/// Declare a program description
///
/// # Example
/// ```ignore
/// binary_info::program_description!("An example application");
/// ```
#[macro_export]
macro_rules! bi_program_description {
    ($desc:literal) => {
        #[unsafe(link_section = ".bi_entries")]
        #[used]
        static _BI_PROGRAM_DESCRIPTION: $crate::binary_info::BinaryInfoIdAndString =
            $crate::binary_info::BinaryInfoIdAndString {
                core: $crate::binary_info::BinaryInfoCore {
                    info_type: $crate::binary_info::BinaryInfoType::IdAndString as u16,
                    tag: $crate::binary_info::TAG_RASPBERRY_PI,
                },
                id: $crate::binary_info::ID_RP_PROGRAM_DESCRIPTION,
                value: concat!($desc, "\0").as_ptr(),
            };
    };
}

/// Declare a program URL
///
/// # Example
/// ```ignore
/// binary_info::program_url!("https://github.com/example/app");
/// ```
#[macro_export]
macro_rules! bi_program_url {
    ($url:literal) => {
        #[unsafe(link_section = ".bi_entries")]
        #[used]
        static _BI_PROGRAM_URL: $crate::binary_info::BinaryInfoIdAndString =
            $crate::binary_info::BinaryInfoIdAndString {
                core: $crate::binary_info::BinaryInfoCore {
                    info_type: $crate::binary_info::BinaryInfoType::IdAndString as u16,
                    tag: $crate::binary_info::TAG_RASPBERRY_PI,
                },
                id: $crate::binary_info::ID_RP_PROGRAM_URL,
                value: concat!($url, "\0").as_ptr(),
            };
    };
}

/// Declare a program build date
///
/// # Example
/// ```ignore
/// binary_info::program_build_date!("2024-01-15");
/// ```
#[macro_export]
macro_rules! bi_program_build_date {
    ($date:literal) => {
        #[unsafe(link_section = ".bi_entries")]
        #[used]
        static _BI_PROGRAM_BUILD_DATE: $crate::binary_info::BinaryInfoIdAndString =
            $crate::binary_info::BinaryInfoIdAndString {
                core: $crate::binary_info::BinaryInfoCore {
                    info_type: $crate::binary_info::BinaryInfoType::IdAndString as u16,
                    tag: $crate::binary_info::TAG_RASPBERRY_PI,
                },
                id: $crate::binary_info::ID_RP_PROGRAM_BUILD_DATE_STRING,
                value: concat!($date, "\0").as_ptr(),
            };
    };
}

/// Declare a program feature
///
/// Multiple features can be declared using multiple invocations.
///
/// # Example
/// ```ignore
/// binary_info::program_feature!("LED blink");
/// binary_info::program_feature!("USB serial");
/// ```
#[macro_export]
macro_rules! bi_program_feature {
    ($feature:literal) => {
        $crate::paste::paste! {
            #[unsafe(link_section = ".bi_entries")]
            #[used]
            static [<_BI_PROGRAM_FEATURE_ $feature:snake:upper>]: $crate::binary_info::BinaryInfoIdAndString =
                $crate::binary_info::BinaryInfoIdAndString {
                    core: $crate::binary_info::BinaryInfoCore {
                        info_type: $crate::binary_info::BinaryInfoType::IdAndString as u16,
                        tag: $crate::binary_info::TAG_RASPBERRY_PI,
                    },
                    id: $crate::binary_info::ID_RP_PROGRAM_FEATURE,
                    value: concat!($feature, "\0").as_ptr(),
                };
        }
    };
}

/// Declare a program build attribute
///
/// # Example
/// ```ignore
/// binary_info::program_build_attribute!("debug");
/// ```
#[macro_export]
macro_rules! bi_program_build_attribute {
    ($attr:literal) => {
        $crate::paste::paste! {
            #[unsafe(link_section = ".bi_entries")]
            #[used]
            static [<_BI_BUILD_ATTR_ $attr:snake:upper>]: $crate::binary_info::BinaryInfoIdAndString =
                $crate::binary_info::BinaryInfoIdAndString {
                    core: $crate::binary_info::BinaryInfoCore {
                        info_type: $crate::binary_info::BinaryInfoType::IdAndString as u16,
                        tag: $crate::binary_info::TAG_RASPBERRY_PI,
                    },
                    id: $crate::binary_info::ID_RP_PROGRAM_BUILD_ATTRIBUTE,
                    value: concat!($attr, "\0").as_ptr(),
                };
        }
    };
}

/// Declare a Pico board identifier
///
/// # Example
/// ```ignore
/// binary_info::pico_board!("pico2");
/// ```
#[macro_export]
macro_rules! bi_pico_board {
    ($board:literal) => {
        #[unsafe(link_section = ".bi_entries")]
        #[used]
        static _BI_PICO_BOARD: $crate::binary_info::BinaryInfoIdAndString =
            $crate::binary_info::BinaryInfoIdAndString {
                core: $crate::binary_info::BinaryInfoCore {
                    info_type: $crate::binary_info::BinaryInfoType::IdAndString as u16,
                    tag: $crate::binary_info::TAG_RASPBERRY_PI,
                },
                id: $crate::binary_info::ID_RP_PICO_BOARD,
                value: concat!($board, "\0").as_ptr(),
            };
    };
}

/// Declare a pin with a GPIO function (32-bit, for RP2350A with <=32 GPIOs)
///
/// # Example
/// ```ignore
/// binary_info::pin_with_func!(25, Sio);  // GPIO 25 as SIO
/// binary_info::pin_with_func!(0, 1, Uart);  // GPIO 0,1 as UART
/// ```
#[macro_export]
macro_rules! bi_pin_with_func {
    ($pin:literal, $func:ident) => {
        $crate::paste::paste! {
            #[unsafe(link_section = ".bi_entries")]
            #[used]
            static [<_BI_PIN_ $pin _ $func:upper>]: $crate::binary_info::BinaryInfoPinsWithFunc =
                $crate::binary_info::BinaryInfoPinsWithFunc {
                    core: $crate::binary_info::BinaryInfoCore {
                        info_type: $crate::binary_info::BinaryInfoType::PinsWithFunc as u16,
                        tag: $crate::binary_info::TAG_RASPBERRY_PI,
                    },
                    pin_encoding: $crate::binary_info::encode_1pin_with_func(
                        $pin,
                        $crate::binary_info::GpioFunc::$func,
                    ),
                };
        }
    };
    ($p0:literal, $p1:literal, $func:ident) => {
        $crate::paste::paste! {
            #[unsafe(link_section = ".bi_entries")]
            #[used]
            static [<_BI_PINS_ $p0 _ $p1 _ $func:upper>]: $crate::binary_info::BinaryInfoPinsWithFunc =
                $crate::binary_info::BinaryInfoPinsWithFunc {
                    core: $crate::binary_info::BinaryInfoCore {
                        info_type: $crate::binary_info::BinaryInfoType::PinsWithFunc as u16,
                        tag: $crate::binary_info::TAG_RASPBERRY_PI,
                    },
                    pin_encoding: $crate::binary_info::encode_2pins_with_func(
                        $p0,
                        $p1,
                        $crate::binary_info::GpioFunc::$func,
                    ),
                };
        }
    };
}

/// Declare a pin range with a GPIO function
///
/// # Example
/// ```ignore
/// binary_info::pin_range_with_func!(0, 3, Spi);  // GPIO 0-3 as SPI
/// ```
#[macro_export]
macro_rules! bi_pin_range_with_func {
    ($plo:literal, $phi:literal, $func:ident) => {
        $crate::paste::paste! {
            #[unsafe(link_section = ".bi_entries")]
            #[used]
            static [<_BI_PINS_ $plo _TO_ $phi _ $func>]: $crate::binary_info::BinaryInfoPinsWithFunc =
                $crate::binary_info::BinaryInfoPinsWithFunc {
                    core: $crate::binary_info::BinaryInfoCore {
                        info_type: $crate::binary_info::BinaryInfoType::PinsWithFunc as u16,
                        tag: $crate::binary_info::TAG_RASPBERRY_PI,
                    },
                    pin_encoding: $crate::binary_info::encode_pin_range_with_func(
                        $plo,
                        $phi,
                        $crate::binary_info::GpioFunc::$func,
                    ),
                };
        }
    };
}

/// Declare a pin with a custom name
///
/// # Example
/// ```ignore
/// binary_info::pin_with_name!(25, "LED");
/// binary_info::pin_with_name!([0, 1, 2, 3], "SPI0");
/// ```
#[macro_export]
macro_rules! bi_pin_with_name {
    ($pin:literal, $name:literal) => {
        $crate::paste::paste! {
            #[unsafe(link_section = ".bi_entries")]
            #[used]
            static [<_BI_PIN_ $pin _NAME>]: $crate::binary_info::BinaryInfoPinsWithName =
                $crate::binary_info::BinaryInfoPinsWithName {
                    core: $crate::binary_info::BinaryInfoCore {
                        info_type: $crate::binary_info::BinaryInfoType::PinsWithName as u16,
                        tag: $crate::binary_info::TAG_RASPBERRY_PI,
                    },
                    pin_mask: 1 << $pin,
                    label: concat!($name, "\0").as_ptr(),
                };
        }
    };
    ([$($pin:literal),+ $(,)?], $name:literal) => {
        #[unsafe(link_section = ".bi_entries")]
        #[used]
        static _BI_PINS_NAME: $crate::binary_info::BinaryInfoPinsWithName =
            $crate::binary_info::BinaryInfoPinsWithName {
                core: $crate::binary_info::BinaryInfoCore {
                    info_type: $crate::binary_info::BinaryInfoType::PinsWithName as u16,
                    tag: $crate::binary_info::TAG_RASPBERRY_PI,
                },
                pin_mask: $((1u32 << $pin))|+,
                label: concat!($name, "\0").as_ptr(),
            };
    };
}

/// Declare an integer value with a tag and ID
///
/// # Example
/// ```ignore
/// binary_info::int_value!(TAG_RASPBERRY_PI, ID_RP_BINARY_END, 0x10010000);
/// ```
#[macro_export]
macro_rules! bi_int_value {
    ($tag:expr, $id:expr, $value:expr) => {
        #[unsafe(link_section = ".bi_entries")]
        #[used]
        static _BI_INT: $crate::binary_info::BinaryInfoIdAndInt =
            $crate::binary_info::BinaryInfoIdAndInt {
                core: $crate::binary_info::BinaryInfoCore {
                    info_type: $crate::binary_info::BinaryInfoType::IdAndInt as u16,
                    tag: $tag,
                },
                id: $id,
                value: $value as i32,
            };
    };
}

/// Declare a string value with a tag and ID
///
/// # Example
/// ```ignore
/// binary_info::string_value!(TAG_RASPBERRY_PI, ID_RP_SDK_VERSION, "roschi 0.1.0");
/// ```
#[macro_export]
macro_rules! bi_string_value {
    ($tag:expr, $id:expr, $value:literal) => {
        #[unsafe(link_section = ".bi_entries")]
        #[used]
        static _BI_STRING: $crate::binary_info::BinaryInfoIdAndString =
            $crate::binary_info::BinaryInfoIdAndString {
                core: $crate::binary_info::BinaryInfoCore {
                    info_type: $crate::binary_info::BinaryInfoType::IdAndString as u16,
                    tag: $tag,
                },
                id: $id,
                value: concat!($value, "\0").as_ptr(),
            };
    };
}

// Re-export macros at module level
pub use bi_int_value as int_value;
pub use bi_pico_board as pico_board;
pub use bi_pin_range_with_func as pin_range_with_func;
pub use bi_pin_with_func as pin_with_func;
pub use bi_pin_with_name as pin_with_name;
pub use bi_program_build_attribute as program_build_attribute;
pub use bi_program_build_date as program_build_date;
pub use bi_program_description as program_description;
pub use bi_program_feature as program_feature;
pub use bi_program_name as program_name;
pub use bi_program_url as program_url;
pub use bi_program_version as program_version;
pub use bi_string_value as string_value;
