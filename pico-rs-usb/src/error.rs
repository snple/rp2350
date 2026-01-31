//! USB error types

use core::fmt;

/// USB operation result type
pub type UsbResult<T> = Result<T, UsbError>;

/// USB error types
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum UsbError {
    /// Buffer too small for the requested operation
    BufferOverflow,
    /// Invalid endpoint address
    InvalidEndpoint,
    /// Endpoint already allocated
    EndpointAllocated,
    /// No free endpoints available
    NoEndpointAvailable,
    /// Transfer would block (for non-blocking operations)
    WouldBlock,
    /// Endpoint is stalled
    Stalled,
    /// Invalid state for the requested operation
    InvalidState,
    /// USB bus not ready
    NotReady,
    /// Invalid descriptor
    InvalidDescriptor,
    /// Endpoint buffer memory exhausted
    BufferMemoryExhausted,
    /// Invalid configuration
    InvalidConfiguration,
    /// Unsupported operation
    Unsupported,
    /// Parse error in control request
    ParseError,
    /// General I/O error
    IoError,
}

impl fmt::Display for UsbError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UsbError::BufferOverflow => write!(f, "Buffer overflow"),
            UsbError::InvalidEndpoint => write!(f, "Invalid endpoint"),
            UsbError::EndpointAllocated => write!(f, "Endpoint already allocated"),
            UsbError::NoEndpointAvailable => write!(f, "No endpoint available"),
            UsbError::WouldBlock => write!(f, "Would block"),
            UsbError::Stalled => write!(f, "Endpoint stalled"),
            UsbError::InvalidState => write!(f, "Invalid state"),
            UsbError::NotReady => write!(f, "USB bus not ready"),
            UsbError::InvalidDescriptor => write!(f, "Invalid descriptor"),
            UsbError::BufferMemoryExhausted => write!(f, "Buffer memory exhausted"),
            UsbError::InvalidConfiguration => write!(f, "Invalid configuration"),
            UsbError::Unsupported => write!(f, "Unsupported operation"),
            UsbError::ParseError => write!(f, "Parse error"),
            UsbError::IoError => write!(f, "I/O error"),
        }
    }
}
