//! IPv4 protocol implementation

/// IPv4 address (4 bytes)
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub struct Ipv4Addr(pub [u8; 4]);

impl Ipv4Addr {
    /// Create a new IPv4 address
    pub const fn new(a: u8, b: u8, c: u8, d: u8) -> Self {
        Self([a, b, c, d])
    }

    /// Broadcast address (255.255.255.255)
    pub const BROADCAST: Self = Self([255, 255, 255, 255]);

    /// Any address (0.0.0.0)
    pub const ANY: Self = Self([0, 0, 0, 0]);

    /// Localhost (127.0.0.1)
    pub const LOCALHOST: Self = Self([127, 0, 0, 1]);

    /// Check if this is a broadcast address
    pub fn is_broadcast(&self) -> bool {
        self.0 == Self::BROADCAST.0
    }

    /// Check if this is a multicast address (224.0.0.0 - 239.255.255.255)
    pub fn is_multicast(&self) -> bool {
        self.0[0] >= 224 && self.0[0] <= 239
    }

    /// Check if this is a link-local address (169.254.0.0/16)
    pub fn is_link_local(&self) -> bool {
        self.0[0] == 169 && self.0[1] == 254
    }

    /// Get the raw bytes
    pub fn as_bytes(&self) -> &[u8; 4] {
        &self.0
    }

    /// Convert to u32 (network byte order)
    pub fn to_u32(&self) -> u32 {
        u32::from_be_bytes(self.0)
    }

    /// Create from u32 (network byte order)
    pub fn from_u32(value: u32) -> Self {
        Self(value.to_be_bytes())
    }
}

impl core::fmt::Debug for Ipv4Addr {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}.{}.{}.{}", self.0[0], self.0[1], self.0[2], self.0[3])
    }
}

impl core::fmt::Display for Ipv4Addr {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}.{}.{}.{}", self.0[0], self.0[1], self.0[2], self.0[3])
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for Ipv4Addr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "{}.{}.{}.{}", self.0[0], self.0[1], self.0[2], self.0[3])
    }
}

impl From<[u8; 4]> for Ipv4Addr {
    fn from(bytes: [u8; 4]) -> Self {
        Self(bytes)
    }
}

impl From<u32> for Ipv4Addr {
    fn from(value: u32) -> Self {
        Self::from_u32(value)
    }
}

/// IP protocol numbers
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum IpProtocol {
    /// ICMP
    Icmp = 1,
    /// TCP
    Tcp = 6,
    /// UDP
    Udp = 17,
    /// Unknown protocol
    Unknown = 255,
}

impl From<u8> for IpProtocol {
    fn from(value: u8) -> Self {
        match value {
            1 => IpProtocol::Icmp,
            6 => IpProtocol::Tcp,
            17 => IpProtocol::Udp,
            _ => IpProtocol::Unknown,
        }
    }
}

/// Parsed IPv4 packet
#[derive(Clone, Copy, Debug)]
pub struct Ipv4Packet<'a> {
    /// IP version (should be 4)
    pub version: u8,
    /// Header length in 32-bit words
    pub ihl: u8,
    /// Differentiated Services Code Point
    pub dscp: u8,
    /// Explicit Congestion Notification
    pub ecn: u8,
    /// Total length including header
    pub total_length: u16,
    /// Identification
    pub identification: u16,
    /// Don't Fragment flag
    pub dont_fragment: bool,
    /// More Fragments flag
    pub more_fragments: bool,
    /// Fragment offset
    pub fragment_offset: u16,
    /// Time to Live
    pub ttl: u8,
    /// Protocol
    pub protocol: IpProtocol,
    /// Header checksum
    pub checksum: u16,
    /// Source address
    pub src: Ipv4Addr,
    /// Destination address
    pub dst: Ipv4Addr,
    /// Options (if any)
    pub options: &'a [u8],
    /// Payload
    pub payload: &'a [u8],
}

impl<'a> Ipv4Packet<'a> {
    /// Minimum IPv4 header size (without options)
    pub const MIN_HEADER_SIZE: usize = 20;

    /// Parse an IPv4 packet from raw bytes
    pub fn parse(data: &'a [u8]) -> Option<Self> {
        if data.len() < Self::MIN_HEADER_SIZE {
            return None;
        }

        let version = data[0] >> 4;
        if version != 4 {
            return None;
        }

        let ihl = data[0] & 0x0F;
        let header_len = (ihl as usize) * 4;

        if header_len < Self::MIN_HEADER_SIZE || data.len() < header_len {
            return None;
        }

        let dscp = data[1] >> 2;
        let ecn = data[1] & 0x03;
        let total_length = u16::from_be_bytes([data[2], data[3]]);
        let identification = u16::from_be_bytes([data[4], data[5]]);
        let flags_fragment = u16::from_be_bytes([data[6], data[7]]);
        let dont_fragment = (flags_fragment & 0x4000) != 0;
        let more_fragments = (flags_fragment & 0x2000) != 0;
        let fragment_offset = flags_fragment & 0x1FFF;
        let ttl = data[8];
        let protocol = IpProtocol::from(data[9]);
        let checksum = u16::from_be_bytes([data[10], data[11]]);
        let src = Ipv4Addr::new(data[12], data[13], data[14], data[15]);
        let dst = Ipv4Addr::new(data[16], data[17], data[18], data[19]);

        let options = if header_len > Self::MIN_HEADER_SIZE {
            &data[Self::MIN_HEADER_SIZE..header_len]
        } else {
            &[]
        };

        // Payload length is total_length - header_len, but clamp to available data
        let payload_len = (total_length as usize).saturating_sub(header_len);
        let payload_end = core::cmp::min(header_len + payload_len, data.len());
        let payload = &data[header_len..payload_end];

        Some(Self {
            version,
            ihl,
            dscp,
            ecn,
            total_length,
            identification,
            dont_fragment,
            more_fragments,
            fragment_offset,
            ttl,
            protocol,
            checksum,
            src,
            dst,
            options,
            payload,
        })
    }

    /// Build an IPv4 header into a buffer (without checksum)
    ///
    /// Returns the header length. Caller should compute and fill in checksum.
    pub fn build_header(
        buf: &mut [u8],
        src: Ipv4Addr,
        dst: Ipv4Addr,
        protocol: IpProtocol,
        ttl: u8,
        identification: u16,
        payload_len: usize,
    ) -> Option<usize> {
        if buf.len() < Self::MIN_HEADER_SIZE {
            return None;
        }

        let total_length = (Self::MIN_HEADER_SIZE + payload_len) as u16;

        buf[0] = 0x45; // Version 4, IHL 5
        buf[1] = 0x00; // DSCP 0, ECN 0
        buf[2..4].copy_from_slice(&total_length.to_be_bytes());
        buf[4..6].copy_from_slice(&identification.to_be_bytes());
        buf[6..8].copy_from_slice(&0x4000u16.to_be_bytes()); // Don't Fragment
        buf[8] = ttl;
        buf[9] = protocol as u8;
        buf[10..12].copy_from_slice(&0u16.to_be_bytes()); // Checksum placeholder
        buf[12..16].copy_from_slice(&src.0);
        buf[16..20].copy_from_slice(&dst.0);

        Some(Self::MIN_HEADER_SIZE)
    }

    /// Verify the header checksum
    pub fn verify_checksum(&self, header_data: &[u8]) -> bool {
        let computed = Self::compute_checksum(header_data);
        computed == 0
    }

    /// Compute checksum for header data
    pub fn compute_checksum(data: &[u8]) -> u16 {
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ipv4_addr() {
        let addr = Ipv4Addr::new(192, 168, 1, 1);
        assert_eq!(addr.0, [192, 168, 1, 1]);
        assert!(!addr.is_broadcast());
        assert!(!addr.is_multicast());
    }

    #[test]
    fn test_parse_ipv4() {
        let data = [
            0x45, // Version 4, IHL 5
            0x00, // DSCP/ECN
            0x00, 0x28, // Total length 40
            0x00, 0x01, // Identification
            0x40, 0x00, // Flags (DF) + Fragment offset
            0x40, // TTL 64
            0x06, // Protocol TCP
            0x00, 0x00, // Checksum (unchecked here)
            192, 168, 1, 100, // Source IP
            192, 168, 1, 1, // Dest IP
            // 20 bytes of payload...
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ];

        let packet = Ipv4Packet::parse(&data).unwrap();
        assert_eq!(packet.version, 4);
        assert_eq!(packet.ihl, 5);
        assert_eq!(packet.total_length, 40);
        assert_eq!(packet.ttl, 64);
        assert_eq!(packet.protocol, IpProtocol::Tcp);
        assert_eq!(packet.src, Ipv4Addr::new(192, 168, 1, 100));
        assert_eq!(packet.dst, Ipv4Addr::new(192, 168, 1, 1));
        assert!(packet.dont_fragment);
    }
}
