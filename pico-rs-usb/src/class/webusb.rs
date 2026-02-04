//! # WebUSB Class Implementation
//!
//! This module provides WebUSB support for USB devices, allowing them to be
//! accessed from web browsers using the WebUSB API.
//!
//! ## Features
//!
//! - BOS (Binary Object Store) descriptor with WebUSB Platform Capability
//! - Microsoft OS 2.0 descriptors for Windows compatibility
//! - Landing page URL support
//! - Vendor-specific request handling
//!
//! ## Usage
//!
//! ```ignore
//! use pico_rs_usb::class::webusb::WebUsb;
//!
//! // Create WebUSB instance with landing page URL
//! let mut webusb = WebUsb::new("https://example.com/device");
//!
//! // Poll along with other USB classes
//! usb_dev.poll(&mut [&mut webusb, &mut other_class]);
//! ```
//!
//! ## Protocol Overview
//!
//! WebUSB uses:
//! 1. **BOS Descriptor**: Contains Platform Capability descriptor with WebUSB UUID
//! 2. **Vendor Request 0x01**: Get WebUSB URL descriptor (landing page)
//! 3. **Microsoft OS 2.0**: Optional descriptors for Windows compatibility
//!
//! ## Browser Support
//!
//! WebUSB is supported in:
//! - Chrome/Chromium 61+
//! - Edge 79+
//! - Opera 48+
//!
//! Note: Firefox and Safari do not support WebUSB.

use crate::control::SetupPacket;
use crate::descriptor::DescriptorWriter;
use crate::device::UsbClass;
use crate::error::{UsbError, UsbResult};
use crate::types::{Recipient, RequestType};

// ============================================================================
// WebUSB Constants
// ============================================================================

/// WebUSB vendor request code for getting URL
pub const WEBUSB_REQUEST_GET_URL: u8 = 0x01;

/// WebUSB URL descriptor type
pub const WEBUSB_URL_DESCRIPTOR_TYPE: u8 = 0x03;

/// WebUSB URL scheme: "http://"
pub const URL_SCHEME_HTTP: u8 = 0x00;
/// WebUSB URL scheme: "https://"
pub const URL_SCHEME_HTTPS: u8 = 0x01;
/// WebUSB URL scheme: "" (full URL in descriptor)
pub const URL_SCHEME_NONE: u8 = 0xFF;

/// BOS descriptor type
pub const BOS_DESCRIPTOR_TYPE: u8 = 0x0F;

/// Device capability descriptor type
pub const DEVICE_CAPABILITY_DESCRIPTOR_TYPE: u8 = 0x10;

/// Platform capability type
pub const PLATFORM_CAPABILITY_TYPE: u8 = 0x05;

/// WebUSB Platform Capability UUID: {3408b638-09a9-47a0-8bfd-a0768815b665}
pub const WEBUSB_UUID: [u8; 16] = [
    0x38, 0xB6, 0x08, 0x34, 0xA9, 0x09, 0xA0, 0x47, 0x8B, 0xFD, 0xA0, 0x76, 0x88, 0x15, 0xB6, 0x65,
];

/// Microsoft OS 2.0 Platform Capability UUID: {D8DD60DF-4589-4CC7-9CD2-659D9E648A9F}
pub const MS_OS_20_UUID: [u8; 16] = [
    0xDF, 0x60, 0xDD, 0xD8, 0x89, 0x45, 0xC7, 0x4C, 0x9C, 0xD2, 0x65, 0x9D, 0x9E, 0x64, 0x8A, 0x9F,
];

/// Microsoft vendor request code
pub const MS_VENDOR_CODE: u8 = 0x02;

// ============================================================================
// BOS Descriptor
// ============================================================================

/// BOS (Binary Object Store) descriptor header size
const BOS_HEADER_SIZE: usize = 5;

/// WebUSB Platform Capability descriptor size (without vendor code and URL index)
const WEBUSB_CAPABILITY_SIZE: usize = 24;

/// Microsoft OS 2.0 Platform Capability descriptor size
const MS_OS_20_CAPABILITY_SIZE: usize = 28;

// ============================================================================
// WebUSB Class
// ============================================================================

/// WebUSB device class
///
/// Provides WebUSB support including BOS descriptors and vendor request handling.
pub struct WebUsb<'a> {
    /// Landing page URL (without scheme prefix if using URL_SCHEME_HTTP/HTTPS)
    landing_page_url: &'a str,
    /// URL scheme (http, https, or none)
    url_scheme: u8,
    /// Vendor code for WebUSB requests
    vendor_code: u8,
    /// Enable Microsoft OS 2.0 descriptors
    enable_ms_os_20: bool,
    /// Response buffer for control transfers
    response_buf: [u8; 256],
    /// Response length
    response_len: usize,
    /// Currently handling a control transfer
    pending_response: bool,
}

impl<'a> WebUsb<'a> {
    /// Create a new WebUSB instance with HTTPS URL
    ///
    /// The URL should not include the "https://" prefix.
    ///
    /// # Arguments
    /// * `url` - Landing page URL without scheme (e.g., "example.com/device")
    pub fn new(url: &'a str) -> Self {
        Self {
            landing_page_url: url,
            url_scheme: URL_SCHEME_HTTPS,
            vendor_code: WEBUSB_REQUEST_GET_URL,
            enable_ms_os_20: true,
            response_buf: [0u8; 256],
            response_len: 0,
            pending_response: false,
        }
    }

    /// Create WebUSB with full URL (including scheme)
    ///
    /// Use this when the URL scheme is not http or https.
    pub fn with_full_url(url: &'a str) -> Self {
        Self {
            landing_page_url: url,
            url_scheme: URL_SCHEME_NONE,
            vendor_code: WEBUSB_REQUEST_GET_URL,
            enable_ms_os_20: true,
            response_buf: [0u8; 256],
            response_len: 0,
            pending_response: false,
        }
    }

    /// Set the URL scheme
    pub fn url_scheme(mut self, scheme: u8) -> Self {
        self.url_scheme = scheme;
        self
    }

    /// Set the vendor code for WebUSB requests
    pub fn vendor_code(mut self, code: u8) -> Self {
        self.vendor_code = code;
        self
    }

    /// Enable or disable Microsoft OS 2.0 descriptors
    pub fn with_ms_os_20(mut self, enable: bool) -> Self {
        self.enable_ms_os_20 = enable;
        self
    }

    /// Get the total BOS descriptor length
    pub fn bos_descriptor_len(&self) -> usize {
        let mut len = BOS_HEADER_SIZE + WEBUSB_CAPABILITY_SIZE;
        if self.enable_ms_os_20 {
            len += MS_OS_20_CAPABILITY_SIZE;
        }
        len
    }

    /// Build BOS descriptor
    ///
    /// Returns the number of bytes written.
    pub fn build_bos_descriptor(&self, buf: &mut [u8]) -> usize {
        let total_len = self.bos_descriptor_len();
        if buf.len() < total_len {
            return 0;
        }

        let mut pos = 0;
        let num_capabilities = if self.enable_ms_os_20 { 2 } else { 1 };

        // BOS Descriptor Header (5 bytes)
        buf[pos] = 5; // bLength
        buf[pos + 1] = BOS_DESCRIPTOR_TYPE; // bDescriptorType
        buf[pos + 2] = total_len as u8; // wTotalLength (low)
        buf[pos + 3] = (total_len >> 8) as u8; // wTotalLength (high)
        buf[pos + 4] = num_capabilities; // bNumDeviceCaps
        pos += 5;

        // WebUSB Platform Capability Descriptor (24 bytes)
        buf[pos] = WEBUSB_CAPABILITY_SIZE as u8; // bLength
        buf[pos + 1] = DEVICE_CAPABILITY_DESCRIPTOR_TYPE; // bDescriptorType
        buf[pos + 2] = PLATFORM_CAPABILITY_TYPE; // bDevCapabilityType
        buf[pos + 3] = 0; // bReserved
        buf[pos + 4..pos + 20].copy_from_slice(&WEBUSB_UUID); // PlatformCapabilityUUID
        buf[pos + 20] = 0x00; // bcdVersion (low) - WebUSB version 1.0
        buf[pos + 21] = 0x01; // bcdVersion (high)
        buf[pos + 22] = self.vendor_code; // bVendorCode
        buf[pos + 23] = 1; // iLandingPage (URL descriptor index)
        pos += WEBUSB_CAPABILITY_SIZE;

        // Microsoft OS 2.0 Platform Capability Descriptor (28 bytes)
        if self.enable_ms_os_20 {
            buf[pos] = MS_OS_20_CAPABILITY_SIZE as u8; // bLength
            buf[pos + 1] = DEVICE_CAPABILITY_DESCRIPTOR_TYPE; // bDescriptorType
            buf[pos + 2] = PLATFORM_CAPABILITY_TYPE; // bDevCapabilityType
            buf[pos + 3] = 0; // bReserved
            buf[pos + 4..pos + 20].copy_from_slice(&MS_OS_20_UUID); // PlatformCapabilityUUID
            // MS OS 2.0 Descriptor Information
            buf[pos + 20] = 0x00; // dwWindowsVersion (Windows 8.1)
            buf[pos + 21] = 0x00;
            buf[pos + 22] = 0x03;
            buf[pos + 23] = 0x06;
            // Total length of MS OS 2.0 descriptor set
            let ms_desc_len = self.ms_os_20_descriptor_len();
            buf[pos + 24] = ms_desc_len as u8; // wMSOSDescriptorSetTotalLength (low)
            buf[pos + 25] = (ms_desc_len >> 8) as u8; // wMSOSDescriptorSetTotalLength (high)
            buf[pos + 26] = MS_VENDOR_CODE; // bMS_VendorCode
            buf[pos + 27] = 0; // bAltEnumCode
            pos += MS_OS_20_CAPABILITY_SIZE;
        }

        pos
    }

    /// Build WebUSB URL descriptor
    ///
    /// Returns the number of bytes written.
    pub fn build_url_descriptor(&self, buf: &mut [u8]) -> usize {
        let url_bytes = self.landing_page_url.as_bytes();
        let len = 3 + url_bytes.len(); // header (3) + URL

        if buf.len() < len {
            return 0;
        }

        buf[0] = len as u8; // bLength
        buf[1] = WEBUSB_URL_DESCRIPTOR_TYPE; // bDescriptorType
        buf[2] = self.url_scheme; // bScheme
        buf[3..3 + url_bytes.len()].copy_from_slice(url_bytes);

        len
    }

    /// Get MS OS 2.0 descriptor set length
    fn ms_os_20_descriptor_len(&self) -> usize {
        // Descriptor Set Header (10) + Configuration Subset Header (8) +
        // Function Subset Header (8) + Compatible ID Descriptor (20)
        10 + 8 + 8 + 20
    }

    /// Build Microsoft OS 2.0 Descriptor Set
    pub fn build_ms_os_20_descriptor(&self, buf: &mut [u8]) -> usize {
        let total_len = self.ms_os_20_descriptor_len();
        if buf.len() < total_len {
            return 0;
        }

        let mut pos = 0;

        // MS OS 2.0 Descriptor Set Header (10 bytes)
        buf[pos] = 10; // wLength (low)
        buf[pos + 1] = 0; // wLength (high)
        buf[pos + 2] = 0x00; // wDescriptorType (MS_OS_20_SET_HEADER_DESCRIPTOR)
        buf[pos + 3] = 0x00;
        buf[pos + 4] = 0x00; // dwWindowsVersion (Windows 8.1 = 0x06030000)
        buf[pos + 5] = 0x00;
        buf[pos + 6] = 0x03;
        buf[pos + 7] = 0x06;
        buf[pos + 8] = total_len as u8; // wTotalLength (low)
        buf[pos + 9] = (total_len >> 8) as u8; // wTotalLength (high)
        pos += 10;

        // Configuration Subset Header (8 bytes)
        buf[pos] = 8; // wLength (low)
        buf[pos + 1] = 0; // wLength (high)
        buf[pos + 2] = 0x01; // wDescriptorType (MS_OS_20_SUBSET_HEADER_CONFIGURATION)
        buf[pos + 3] = 0x00;
        buf[pos + 4] = 0; // bConfigurationValue
        buf[pos + 5] = 0; // bReserved
        let subset_len = 8 + 20; // Function subset + Compatible ID
        buf[pos + 6] = subset_len as u8; // wTotalLength (low)
        buf[pos + 7] = (subset_len >> 8) as u8; // wTotalLength (high)
        pos += 8;

        // Function Subset Header (8 bytes)
        buf[pos] = 8; // wLength (low)
        buf[pos + 1] = 0; // wLength (high)
        buf[pos + 2] = 0x02; // wDescriptorType (MS_OS_20_SUBSET_HEADER_FUNCTION)
        buf[pos + 3] = 0x00;
        buf[pos + 4] = 0; // bFirstInterface
        buf[pos + 5] = 0; // bReserved
        buf[pos + 6] = 20; // wSubsetLength (low) - Compatible ID descriptor
        buf[pos + 7] = 0; // wSubsetLength (high)
        pos += 8;

        // MS OS 2.0 Compatible ID Descriptor (20 bytes)
        buf[pos] = 20; // wLength (low)
        buf[pos + 1] = 0; // wLength (high)
        buf[pos + 2] = 0x03; // wDescriptorType (MS_OS_20_FEATURE_COMPATIBLE_ID)
        buf[pos + 3] = 0x00;
        // Compatible ID: "WINUSB\0\0" (8 bytes)
        buf[pos + 4..pos + 12].copy_from_slice(b"WINUSB\0\0");
        // Sub-Compatible ID (8 bytes, all zeros)
        buf[pos + 12..pos + 20].fill(0);
        pos += 20;

        pos
    }

    /// Handle WebUSB vendor request
    fn handle_vendor_request(&mut self, setup: &SetupPacket) -> bool {
        let request = setup.b_request;
        let index = setup.w_index;

        #[cfg(feature = "defmt")]
        defmt::debug!(
            "WebUSB: Vendor request={:#04x} index={:#06x}",
            request,
            index
        );

        if request == self.vendor_code {
            // WebUSB URL request
            if index == 1 {
                // Landing page URL - build descriptor inline
                let url_bytes = self.landing_page_url.as_bytes();
                let len = 3 + url_bytes.len();
                if len <= self.response_buf.len() {
                    self.response_buf[0] = len as u8;
                    self.response_buf[1] = WEBUSB_URL_DESCRIPTOR_TYPE;
                    self.response_buf[2] = self.url_scheme;
                    self.response_buf[3..3 + url_bytes.len()].copy_from_slice(url_bytes);
                    self.response_len = len;
                    self.pending_response = true;
                    #[cfg(feature = "defmt")]
                    defmt::info!("WebUSB: URL descriptor requested, len={}", len);
                    return true;
                }
            }
        } else if request == MS_VENDOR_CODE && self.enable_ms_os_20 {
            // Microsoft OS 2.0 descriptor request
            if index == 0x07 {
                // MS_OS_20_DESCRIPTOR_INDEX
                self.response_len = Self::build_ms_os_20_descriptor_static(&mut self.response_buf);
                self.pending_response = true;
                #[cfg(feature = "defmt")]
                defmt::info!(
                    "WebUSB: MS OS 2.0 descriptor requested, len={}",
                    self.response_len
                );
                return true;
            }
        }

        false
    }

    /// Build MS OS 2.0 descriptor (static version to avoid borrow issues)
    fn build_ms_os_20_descriptor_static(buf: &mut [u8]) -> usize {
        let total_len = 10 + 8 + 8 + 20; // Header + Config subset + Function subset + Compatible ID
        if buf.len() < total_len {
            return 0;
        }

        let mut pos = 0;

        // MS OS 2.0 Descriptor Set Header (10 bytes)
        buf[pos] = 10;
        buf[pos + 1] = 0;
        buf[pos + 2] = 0x00;
        buf[pos + 3] = 0x00;
        buf[pos + 4] = 0x00;
        buf[pos + 5] = 0x00;
        buf[pos + 6] = 0x03;
        buf[pos + 7] = 0x06;
        buf[pos + 8] = total_len as u8;
        buf[pos + 9] = (total_len >> 8) as u8;
        pos += 10;

        // Configuration Subset Header (8 bytes)
        buf[pos] = 8;
        buf[pos + 1] = 0;
        buf[pos + 2] = 0x01;
        buf[pos + 3] = 0x00;
        buf[pos + 4] = 0;
        buf[pos + 5] = 0;
        let subset_len = 8 + 20;
        buf[pos + 6] = subset_len as u8;
        buf[pos + 7] = (subset_len >> 8) as u8;
        pos += 8;

        // Function Subset Header (8 bytes)
        buf[pos] = 8;
        buf[pos + 1] = 0;
        buf[pos + 2] = 0x02;
        buf[pos + 3] = 0x00;
        buf[pos + 4] = 0;
        buf[pos + 5] = 0;
        buf[pos + 6] = 20;
        buf[pos + 7] = 0;
        pos += 8;

        // MS OS 2.0 Compatible ID Descriptor (20 bytes)
        buf[pos] = 20;
        buf[pos + 1] = 0;
        buf[pos + 2] = 0x03;
        buf[pos + 3] = 0x00;
        buf[pos + 4..pos + 12].copy_from_slice(b"WINUSB\0\0");
        buf[pos + 12..pos + 20].fill(0);
        pos += 20;

        pos
    }

    /// Build BOS descriptor (takes configuration by value to avoid borrow issues)
    fn build_bos_descriptor_with_config(
        buf: &mut [u8],
        vendor_code: u8,
        enable_ms_os_20: bool,
    ) -> usize {
        let bos_len = BOS_HEADER_SIZE + WEBUSB_CAPABILITY_SIZE;
        let total_len = if enable_ms_os_20 {
            bos_len + MS_OS_20_CAPABILITY_SIZE
        } else {
            bos_len
        };

        if buf.len() < total_len {
            return 0;
        }

        let mut pos = 0;
        let num_capabilities = if enable_ms_os_20 { 2u8 } else { 1u8 };

        // BOS Descriptor Header (5 bytes)
        buf[pos] = 5;
        buf[pos + 1] = BOS_DESCRIPTOR_TYPE;
        buf[pos + 2] = total_len as u8;
        buf[pos + 3] = (total_len >> 8) as u8;
        buf[pos + 4] = num_capabilities;
        pos += 5;

        // WebUSB Platform Capability Descriptor (24 bytes)
        buf[pos] = WEBUSB_CAPABILITY_SIZE as u8;
        buf[pos + 1] = DEVICE_CAPABILITY_DESCRIPTOR_TYPE;
        buf[pos + 2] = PLATFORM_CAPABILITY_TYPE;
        buf[pos + 3] = 0;
        buf[pos + 4..pos + 20].copy_from_slice(&WEBUSB_UUID);
        buf[pos + 20] = 0x00;
        buf[pos + 21] = 0x01;
        buf[pos + 22] = vendor_code;
        buf[pos + 23] = 1;
        pos += WEBUSB_CAPABILITY_SIZE;

        // Microsoft OS 2.0 Platform Capability Descriptor (28 bytes)
        if enable_ms_os_20 {
            let ms_desc_len = 10 + 8 + 8 + 20; // Same as build_ms_os_20_descriptor_static
            buf[pos] = MS_OS_20_CAPABILITY_SIZE as u8;
            buf[pos + 1] = DEVICE_CAPABILITY_DESCRIPTOR_TYPE;
            buf[pos + 2] = PLATFORM_CAPABILITY_TYPE;
            buf[pos + 3] = 0;
            buf[pos + 4..pos + 20].copy_from_slice(&MS_OS_20_UUID);
            buf[pos + 20] = 0x00;
            buf[pos + 21] = 0x00;
            buf[pos + 22] = 0x03;
            buf[pos + 23] = 0x06;
            buf[pos + 24] = ms_desc_len as u8;
            buf[pos + 25] = (ms_desc_len >> 8) as u8;
            buf[pos + 26] = MS_VENDOR_CODE;
            buf[pos + 27] = 0;
            pos += MS_OS_20_CAPABILITY_SIZE;
        }

        pos
    }
}

impl<'a> UsbClass for WebUsb<'a> {
    fn control_setup(&mut self, setup: &SetupPacket) -> bool {
        let request_type = RequestType::from(setup.bm_request_type);
        let recipient = Recipient::from(setup.bm_request_type);

        // Handle BOS descriptor request
        if request_type == RequestType::Standard && recipient == Recipient::Device {
            let desc_type = (setup.w_value >> 8) as u8;
            if setup.b_request == 0x06 && desc_type == BOS_DESCRIPTOR_TYPE {
                // GET_DESCRIPTOR for BOS
                let vendor_code = self.vendor_code;
                let enable_ms = self.enable_ms_os_20;
                self.response_len = Self::build_bos_descriptor_with_config(
                    &mut self.response_buf,
                    vendor_code,
                    enable_ms,
                );
                let len = core::cmp::min(self.response_len, setup.w_length as usize);
                self.response_len = len;
                self.pending_response = true;
                #[cfg(feature = "defmt")]
                defmt::info!("WebUSB: BOS descriptor requested, len={}", len);
                return true;
            }
        }

        // Handle vendor requests
        if request_type == RequestType::Vendor && recipient == Recipient::Device {
            return self.handle_vendor_request(setup);
        }

        false
    }

    fn control_in(&mut self, buf: &mut [u8]) -> UsbResult<usize> {
        if self.pending_response && self.response_len > 0 {
            let len = core::cmp::min(self.response_len, buf.len());
            buf[..len].copy_from_slice(&self.response_buf[..len]);
            self.pending_response = false;
            Ok(len)
        } else {
            Err(UsbError::Unsupported)
        }
    }

    fn control_out(&mut self, _data: &[u8]) -> UsbResult<()> {
        Ok(())
    }

    fn get_descriptors(&self, _writer: &mut DescriptorWriter) -> bool {
        // WebUSB doesn't add interface descriptors
        // BOS descriptor is handled via control_setup
        true
    }

    fn reset(&mut self) {
        self.pending_response = false;
        self.response_len = 0;
    }
}

// ============================================================================
// Helper Types
// ============================================================================

/// URL scheme for WebUSB descriptors
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum UrlScheme {
    /// HTTP scheme (http://)
    Http,
    /// HTTPS scheme (https://)
    Https,
    /// No scheme prefix (full URL in descriptor)
    None,
}

impl UrlScheme {
    /// Get the byte value for the URL scheme
    pub const fn as_byte(&self) -> u8 {
        match self {
            UrlScheme::Http => URL_SCHEME_HTTP,
            UrlScheme::Https => URL_SCHEME_HTTPS,
            UrlScheme::None => URL_SCHEME_NONE,
        }
    }
}

impl From<UrlScheme> for u8 {
    fn from(scheme: UrlScheme) -> u8 {
        scheme.as_byte()
    }
}
