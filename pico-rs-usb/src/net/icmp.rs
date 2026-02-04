//! ICMP (Internet Control Message Protocol) implementation

/// ICMP message types
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum IcmpType {
    /// Echo Reply (ping response)
    EchoReply = 0,
    /// Destination Unreachable
    DestinationUnreachable = 3,
    /// Redirect
    Redirect = 5,
    /// Echo Request (ping)
    EchoRequest = 8,
    /// Time Exceeded
    TimeExceeded = 11,
    /// Parameter Problem
    ParameterProblem = 12,
    /// Timestamp Request
    TimestampRequest = 13,
    /// Timestamp Reply
    TimestampReply = 14,
    /// Unknown type
    Unknown = 255,
}

impl From<u8> for IcmpType {
    fn from(value: u8) -> Self {
        match value {
            0 => IcmpType::EchoReply,
            3 => IcmpType::DestinationUnreachable,
            5 => IcmpType::Redirect,
            8 => IcmpType::EchoRequest,
            11 => IcmpType::TimeExceeded,
            12 => IcmpType::ParameterProblem,
            13 => IcmpType::TimestampRequest,
            14 => IcmpType::TimestampReply,
            _ => IcmpType::Unknown,
        }
    }
}

/// ICMP destination unreachable codes
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum IcmpDestUnreachableCode {
    /// Network unreachable
    NetworkUnreachable = 0,
    /// Host unreachable
    HostUnreachable = 1,
    /// Protocol unreachable
    ProtocolUnreachable = 2,
    /// Port unreachable
    PortUnreachable = 3,
    /// Fragmentation needed but DF set
    FragmentationNeeded = 4,
    /// Source route failed
    SourceRouteFailed = 5,
}

/// Parsed ICMP packet
#[derive(Clone, Copy, Debug)]
pub struct IcmpPacket<'a> {
    /// ICMP type
    pub icmp_type: IcmpType,
    /// ICMP code
    pub code: u8,
    /// Checksum
    pub checksum: u16,
    /// Rest of header (4 bytes, interpretation depends on type)
    pub rest_of_header: [u8; 4],
    /// Payload data
    pub payload: &'a [u8],
}

impl<'a> IcmpPacket<'a> {
    /// ICMP header size
    pub const HEADER_SIZE: usize = 8;

    /// Parse an ICMP packet from raw bytes
    pub fn parse(data: &'a [u8]) -> Option<Self> {
        if data.len() < Self::HEADER_SIZE {
            return None;
        }

        let icmp_type = IcmpType::from(data[0]);
        let code = data[1];
        let checksum = u16::from_be_bytes([data[2], data[3]]);
        let rest_of_header = [data[4], data[5], data[6], data[7]];
        let payload = &data[Self::HEADER_SIZE..];

        Some(Self {
            icmp_type,
            code,
            checksum,
            rest_of_header,
            payload,
        })
    }

    /// Get echo identifier (for Echo Request/Reply)
    pub fn echo_identifier(&self) -> u16 {
        u16::from_be_bytes([self.rest_of_header[0], self.rest_of_header[1]])
    }

    /// Get echo sequence number (for Echo Request/Reply)
    pub fn echo_sequence(&self) -> u16 {
        u16::from_be_bytes([self.rest_of_header[2], self.rest_of_header[3]])
    }

    /// Build an ICMP Echo Request
    pub fn build_echo_request(
        buf: &mut [u8],
        identifier: u16,
        sequence: u16,
        payload: &[u8],
    ) -> Option<usize> {
        let total_len = Self::HEADER_SIZE + payload.len();
        if buf.len() < total_len {
            return None;
        }

        buf[0] = IcmpType::EchoRequest as u8;
        buf[1] = 0; // Code
        buf[2..4].copy_from_slice(&0u16.to_be_bytes()); // Checksum placeholder
        buf[4..6].copy_from_slice(&identifier.to_be_bytes());
        buf[6..8].copy_from_slice(&sequence.to_be_bytes());
        buf[Self::HEADER_SIZE..total_len].copy_from_slice(payload);

        // Compute checksum
        let checksum = Self::compute_checksum(&buf[..total_len]);
        buf[2..4].copy_from_slice(&checksum.to_be_bytes());

        Some(total_len)
    }

    /// Build an ICMP Echo Reply
    pub fn build_echo_reply(
        buf: &mut [u8],
        identifier: u16,
        sequence: u16,
        payload: &[u8],
    ) -> Option<usize> {
        let total_len = Self::HEADER_SIZE + payload.len();
        if buf.len() < total_len {
            return None;
        }

        buf[0] = IcmpType::EchoReply as u8;
        buf[1] = 0; // Code
        buf[2..4].copy_from_slice(&0u16.to_be_bytes()); // Checksum placeholder
        buf[4..6].copy_from_slice(&identifier.to_be_bytes());
        buf[6..8].copy_from_slice(&sequence.to_be_bytes());
        buf[Self::HEADER_SIZE..total_len].copy_from_slice(payload);

        // Compute checksum
        let checksum = Self::compute_checksum(&buf[..total_len]);
        buf[2..4].copy_from_slice(&checksum.to_be_bytes());

        Some(total_len)
    }

    /// Build an ICMP Destination Unreachable message
    pub fn build_destination_unreachable(
        buf: &mut [u8],
        code: IcmpDestUnreachableCode,
        original_ip_header_and_data: &[u8],
    ) -> Option<usize> {
        // Include original IP header + first 8 bytes of payload
        let data_len = core::cmp::min(original_ip_header_and_data.len(), 28);
        let total_len = Self::HEADER_SIZE + data_len;

        if buf.len() < total_len {
            return None;
        }

        buf[0] = IcmpType::DestinationUnreachable as u8;
        buf[1] = code as u8;
        buf[2..4].copy_from_slice(&0u16.to_be_bytes()); // Checksum placeholder
        buf[4..8].copy_from_slice(&[0, 0, 0, 0]); // Unused
        buf[Self::HEADER_SIZE..Self::HEADER_SIZE + data_len]
            .copy_from_slice(&original_ip_header_and_data[..data_len]);

        // Compute checksum
        let checksum = Self::compute_checksum(&buf[..total_len]);
        buf[2..4].copy_from_slice(&checksum.to_be_bytes());

        Some(total_len)
    }

    /// Compute ICMP checksum
    fn compute_checksum(data: &[u8]) -> u16 {
        let mut sum: u32 = 0;
        let mut i = 0;

        while i < data.len() - 1 {
            sum += u16::from_be_bytes([data[i], data[i + 1]]) as u32;
            i += 2;
        }

        if data.len() % 2 == 1 {
            sum += (data[data.len() - 1] as u32) << 8;
        }

        while sum > 0xFFFF {
            sum = (sum & 0xFFFF) + (sum >> 16);
        }

        !sum as u16
    }

    /// Verify the checksum
    pub fn verify_checksum(&self, full_packet: &[u8]) -> bool {
        Self::compute_checksum(full_packet) == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_echo_request() {
        let data = [
            0x08, // Type: Echo Request
            0x00, // Code
            0x00, 0x00, // Checksum (placeholder)
            0x00, 0x01, // Identifier
            0x00, 0x02, // Sequence
            0x48, 0x65, 0x6C, 0x6C, 0x6F, // Payload: "Hello"
        ];

        let packet = IcmpPacket::parse(&data).unwrap();
        assert_eq!(packet.icmp_type, IcmpType::EchoRequest);
        assert_eq!(packet.echo_identifier(), 1);
        assert_eq!(packet.echo_sequence(), 2);
        assert_eq!(packet.payload, b"Hello");
    }

    #[test]
    fn test_build_echo_reply() {
        let mut buf = [0u8; 64];
        let payload = b"Hello";

        let len = IcmpPacket::build_echo_reply(&mut buf, 1, 2, payload).unwrap();

        assert_eq!(len, 8 + 5);
        assert_eq!(buf[0], IcmpType::EchoReply as u8);

        // Parse the built packet
        let packet = IcmpPacket::parse(&buf[..len]).unwrap();
        assert_eq!(packet.icmp_type, IcmpType::EchoReply);
        assert_eq!(packet.echo_identifier(), 1);
        assert_eq!(packet.echo_sequence(), 2);

        // Verify checksum
        assert!(packet.verify_checksum(&buf[..len]));
    }
}
