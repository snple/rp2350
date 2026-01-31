//! USB types and constants

/// USB device state
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DeviceState {
    /// Device is not connected or powered
    #[default]
    Detached,
    /// Device is powered but not reset
    Powered,
    /// Device is in the Default state (address 0)
    Default,
    /// Device has been assigned an address
    Addressed,
    /// Device has been configured
    Configured,
    /// Device is suspended
    Suspended,
}

impl DeviceState {
    /// Returns true if the device is connected and not suspended
    pub fn is_active(&self) -> bool {
        matches!(
            self,
            DeviceState::Default | DeviceState::Addressed | DeviceState::Configured
        )
    }

    /// Returns true if the device is configured
    pub fn is_configured(&self) -> bool {
        matches!(self, DeviceState::Configured)
    }
}

/// USB speed
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum UsbSpeed {
    /// Low speed (1.5 Mbps)
    Low,
    /// Full speed (12 Mbps)
    #[default]
    Full,
}

/// Standard USB request types (bmRequestType)
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RequestType {
    /// Standard request
    Standard,
    /// Class-specific request
    Class,
    /// Vendor-specific request
    Vendor,
    /// Reserved
    Reserved,
}

impl From<u8> for RequestType {
    fn from(val: u8) -> Self {
        match (val >> 5) & 0x03 {
            0 => RequestType::Standard,
            1 => RequestType::Class,
            2 => RequestType::Vendor,
            _ => RequestType::Reserved,
        }
    }
}

/// USB request recipient
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Recipient {
    /// Request is for the device
    Device,
    /// Request is for an interface
    Interface,
    /// Request is for an endpoint
    Endpoint,
    /// Request is for another recipient
    Other,
}

impl From<u8> for Recipient {
    fn from(val: u8) -> Self {
        match val & 0x1f {
            0 => Recipient::Device,
            1 => Recipient::Interface,
            2 => Recipient::Endpoint,
            _ => Recipient::Other,
        }
    }
}

/// USB data transfer direction
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Direction {
    /// Host to device (OUT)
    Out,
    /// Device to host (IN)
    In,
}

impl From<u8> for Direction {
    fn from(val: u8) -> Self {
        if val & 0x80 != 0 {
            Direction::In
        } else {
            Direction::Out
        }
    }
}

/// Standard USB device requests
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum StandardRequest {
    /// GET_STATUS
    GetStatus = 0,
    /// CLEAR_FEATURE
    ClearFeature = 1,
    /// SET_FEATURE
    SetFeature = 3,
    /// SET_ADDRESS
    SetAddress = 5,
    /// GET_DESCRIPTOR
    GetDescriptor = 6,
    /// SET_DESCRIPTOR
    SetDescriptor = 7,
    /// GET_CONFIGURATION
    GetConfiguration = 8,
    /// SET_CONFIGURATION
    SetConfiguration = 9,
    /// GET_INTERFACE
    GetInterface = 10,
    /// SET_INTERFACE
    SetInterface = 11,
    /// SYNCH_FRAME
    SynchFrame = 12,
}

impl TryFrom<u8> for StandardRequest {
    type Error = u8;

    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(StandardRequest::GetStatus),
            1 => Ok(StandardRequest::ClearFeature),
            3 => Ok(StandardRequest::SetFeature),
            5 => Ok(StandardRequest::SetAddress),
            6 => Ok(StandardRequest::GetDescriptor),
            7 => Ok(StandardRequest::SetDescriptor),
            8 => Ok(StandardRequest::GetConfiguration),
            9 => Ok(StandardRequest::SetConfiguration),
            10 => Ok(StandardRequest::GetInterface),
            11 => Ok(StandardRequest::SetInterface),
            12 => Ok(StandardRequest::SynchFrame),
            _ => Err(val),
        }
    }
}

/// USB descriptor types
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum DescriptorType {
    /// Device descriptor
    Device = 1,
    /// Configuration descriptor
    Configuration = 2,
    /// String descriptor
    String = 3,
    /// Interface descriptor
    Interface = 4,
    /// Endpoint descriptor
    Endpoint = 5,
    /// Device qualifier descriptor
    DeviceQualifier = 6,
    /// Other speed configuration
    OtherSpeedConfiguration = 7,
    /// Interface power
    InterfacePower = 8,
    /// BOS descriptor
    Bos = 15,
    /// Device capability
    DeviceCapability = 16,
}

impl TryFrom<u8> for DescriptorType {
    type Error = u8;

    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            1 => Ok(DescriptorType::Device),
            2 => Ok(DescriptorType::Configuration),
            3 => Ok(DescriptorType::String),
            4 => Ok(DescriptorType::Interface),
            5 => Ok(DescriptorType::Endpoint),
            6 => Ok(DescriptorType::DeviceQualifier),
            7 => Ok(DescriptorType::OtherSpeedConfiguration),
            8 => Ok(DescriptorType::InterfacePower),
            15 => Ok(DescriptorType::Bos),
            16 => Ok(DescriptorType::DeviceCapability),
            _ => Err(val),
        }
    }
}

/// USB feature selectors
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u16)]
pub enum Feature {
    /// ENDPOINT_HALT feature
    EndpointHalt = 0,
    /// DEVICE_REMOTE_WAKEUP feature
    DeviceRemoteWakeup = 1,
    /// TEST_MODE feature
    TestMode = 2,
}

impl TryFrom<u16> for Feature {
    type Error = u16;

    fn try_from(val: u16) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Feature::EndpointHalt),
            1 => Ok(Feature::DeviceRemoteWakeup),
            2 => Ok(Feature::TestMode),
            _ => Err(val),
        }
    }
}

/// Language IDs for string descriptors
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u16)]
pub enum LangId {
    /// English (United States)
    #[default]
    EnglishUS = 0x0409,
    /// English (United Kingdom)
    EnglishUK = 0x0809,
    /// German
    German = 0x0407,
    /// French
    French = 0x040C,
    /// Spanish
    Spanish = 0x040A,
    /// Chinese (Simplified)
    ChineseSimplified = 0x0804,
    /// Japanese
    Japanese = 0x0411,
}
