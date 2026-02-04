//! UDP (User Datagram Protocol) implementation

use super::ipv4::{IpProtocol, Ipv4Addr};

/// Parsed UDP packet
#[derive(Clone, Copy, Debug)]
pub struct UdpPacket<'a> {
    /// Source port
    pub src_port: u16,
    /// Destination port
    pub dst_port: u16,
    /// Total length (header + payload)
    pub length: u16,
    /// Checksum
    pub checksum: u16,
    /// Payload data
    pub payload: &'a [u8],
}

impl<'a> UdpPacket<'a> {
    /// UDP header size
    pub const HEADER_SIZE: usize = 8;

    /// Parse a UDP packet from raw bytes
    pub fn parse(data: &'a [u8]) -> Option<Self> {
        if data.len() < Self::HEADER_SIZE {
            return None;
        }

        let src_port = u16::from_be_bytes([data[0], data[1]]);
        let dst_port = u16::from_be_bytes([data[2], data[3]]);
        let length = u16::from_be_bytes([data[4], data[5]]);
        let checksum = u16::from_be_bytes([data[6], data[7]]);

        // Validate length
        if (length as usize) < Self::HEADER_SIZE || data.len() < length as usize {
            // Use available data if length field is invalid
            let payload = &data[Self::HEADER_SIZE..];
            return Some(Self {
                src_port,
                dst_port,
                length,
                checksum,
                payload,
            });
        }

        let payload = &data[Self::HEADER_SIZE..length as usize];

        Some(Self {
            src_port,
            dst_port,
            length,
            checksum,
            payload,
        })
    }

    /// Build a UDP packet into a buffer
    ///
    /// Returns the total packet length.
    /// Note: Checksum is set to 0 (optional in IPv4). Use `build_with_checksum` for checksummed packets.
    pub fn build(buf: &mut [u8], src_port: u16, dst_port: u16, payload: &[u8]) -> Option<usize> {
        let total_len = Self::HEADER_SIZE + payload.len();
        if buf.len() < total_len || total_len > u16::MAX as usize {
            return None;
        }

        buf[0..2].copy_from_slice(&src_port.to_be_bytes());
        buf[2..4].copy_from_slice(&dst_port.to_be_bytes());
        buf[4..6].copy_from_slice(&(total_len as u16).to_be_bytes());
        buf[6..8].copy_from_slice(&0u16.to_be_bytes()); // Checksum = 0 (optional)
        buf[Self::HEADER_SIZE..total_len].copy_from_slice(payload);

        Some(total_len)
    }

    /// Build a UDP packet with checksum
    ///
    /// Requires source and destination IP addresses for pseudo-header checksum.
    pub fn build_with_checksum(
        buf: &mut [u8],
        src_ip: &Ipv4Addr,
        dst_ip: &Ipv4Addr,
        src_port: u16,
        dst_port: u16,
        payload: &[u8],
    ) -> Option<usize> {
        let total_len = Self::HEADER_SIZE + payload.len();
        if buf.len() < total_len || total_len > u16::MAX as usize {
            return None;
        }

        buf[0..2].copy_from_slice(&src_port.to_be_bytes());
        buf[2..4].copy_from_slice(&dst_port.to_be_bytes());
        buf[4..6].copy_from_slice(&(total_len as u16).to_be_bytes());
        buf[6..8].copy_from_slice(&0u16.to_be_bytes()); // Checksum placeholder
        buf[Self::HEADER_SIZE..total_len].copy_from_slice(payload);

        // Compute checksum
        let checksum = Self::compute_checksum(src_ip, dst_ip, &buf[..total_len]);
        buf[6..8].copy_from_slice(&checksum.to_be_bytes());

        Some(total_len)
    }

    /// Compute UDP checksum with pseudo-header
    pub fn compute_checksum(src_ip: &Ipv4Addr, dst_ip: &Ipv4Addr, udp_segment: &[u8]) -> u16 {
        let mut sum: u32 = 0;

        // Pseudo-header
        sum += u16::from_be_bytes([src_ip.0[0], src_ip.0[1]]) as u32;
        sum += u16::from_be_bytes([src_ip.0[2], src_ip.0[3]]) as u32;
        sum += u16::from_be_bytes([dst_ip.0[0], dst_ip.0[1]]) as u32;
        sum += u16::from_be_bytes([dst_ip.0[2], dst_ip.0[3]]) as u32;
        sum += IpProtocol::Udp as u32;
        sum += udp_segment.len() as u32;

        // UDP segment (skip checksum field at offset 6-7)
        let mut i = 0;
        while i < udp_segment.len() - 1 {
            if i == 6 {
                // Skip checksum field
                i += 2;
                continue;
            }
            sum += u16::from_be_bytes([udp_segment[i], udp_segment[i + 1]]) as u32;
            i += 2;
        }

        // Handle odd byte
        if udp_segment.len() % 2 == 1 {
            sum += (udp_segment[udp_segment.len() - 1] as u32) << 8;
        }

        // Fold to 16 bits
        while sum > 0xFFFF {
            sum = (sum & 0xFFFF) + (sum >> 16);
        }

        let result = !sum as u16;
        // UDP checksum of 0 is transmitted as 0xFFFF
        if result == 0 { 0xFFFF } else { result }
    }

    /// Verify the checksum
    pub fn verify_checksum(
        &self,
        src_ip: &Ipv4Addr,
        dst_ip: &Ipv4Addr,
        full_packet: &[u8],
    ) -> bool {
        if self.checksum == 0 {
            // Checksum was not computed (valid in IPv4)
            return true;
        }

        // Recompute and compare
        let mut sum: u32 = 0;

        // Pseudo-header
        sum += u16::from_be_bytes([src_ip.0[0], src_ip.0[1]]) as u32;
        sum += u16::from_be_bytes([src_ip.0[2], src_ip.0[3]]) as u32;
        sum += u16::from_be_bytes([dst_ip.0[0], dst_ip.0[1]]) as u32;
        sum += u16::from_be_bytes([dst_ip.0[2], dst_ip.0[3]]) as u32;
        sum += IpProtocol::Udp as u32;
        sum += full_packet.len() as u32;

        // Full UDP segment including checksum
        let mut i = 0;
        while i < full_packet.len() - 1 {
            sum += u16::from_be_bytes([full_packet[i], full_packet[i + 1]]) as u32;
            i += 2;
        }

        if full_packet.len() % 2 == 1 {
            sum += (full_packet[full_packet.len() - 1] as u32) << 8;
        }

        while sum > 0xFFFF {
            sum = (sum & 0xFFFF) + (sum >> 16);
        }

        // Result should be 0xFFFF if checksum is valid
        sum as u16 == 0xFFFF
    }
}

/// UDP socket abstraction
#[derive(Clone, Copy, Debug)]
pub struct UdpSocket {
    /// Local port
    pub local_port: u16,
    /// Remote IP (None for any)
    pub remote_ip: Option<Ipv4Addr>,
    /// Remote port (None for any)
    pub remote_port: Option<u16>,
}

impl UdpSocket {
    /// Create a new UDP socket bound to a local port
    pub const fn new(local_port: u16) -> Self {
        Self {
            local_port,
            remote_ip: None,
            remote_port: None,
        }
    }

    /// Connect to a remote endpoint
    pub fn connect(&mut self, remote_ip: Ipv4Addr, remote_port: u16) {
        self.remote_ip = Some(remote_ip);
        self.remote_port = Some(remote_port);
    }

    /// Check if a received packet matches this socket
    pub fn matches(&self, dst_port: u16, src_ip: Option<Ipv4Addr>, src_port: Option<u16>) -> bool {
        if dst_port != self.local_port {
            return false;
        }

        if let Some(remote_ip) = self.remote_ip
            && (src_ip.is_none() || src_ip != Some(remote_ip))
        {
            return false;
        }

        if let Some(remote_port) = self.remote_port
            && (src_port.is_none() || src_port != Some(remote_port))
        {
            return false;
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_udp() {
        let data = [
            0x04, 0xD2, // Source port: 1234
            0x16, 0x2E, // Dest port: 5678
            0x00, 0x0D, // Length: 13
            0x00, 0x00, // Checksum: 0
            0x48, 0x65, 0x6C, 0x6C, 0x6F, // Payload: "Hello"
        ];

        let packet = UdpPacket::parse(&data).unwrap();
        assert_eq!(packet.src_port, 1234);
        assert_eq!(packet.dst_port, 5678);
        assert_eq!(packet.length, 13);
        assert_eq!(packet.payload, b"Hello");
    }

    #[test]
    fn test_build_udp() {
        let mut buf = [0u8; 64];
        let payload = b"Hello";

        let len = UdpPacket::build(&mut buf, 1234, 5678, payload).unwrap();

        assert_eq!(len, 13);

        // Parse the built packet
        let packet = UdpPacket::parse(&buf[..len]).unwrap();
        assert_eq!(packet.src_port, 1234);
        assert_eq!(packet.dst_port, 5678);
        assert_eq!(packet.payload, b"Hello");
    }

    #[test]
    fn test_udp_checksum() {
        let src_ip = Ipv4Addr::new(192, 168, 1, 100);
        let dst_ip = Ipv4Addr::new(192, 168, 1, 1);
        let mut buf = [0u8; 64];
        let payload = b"Hello";

        let len = UdpPacket::build_with_checksum(&mut buf, &src_ip, &dst_ip, 1234, 5678, payload)
            .unwrap();

        let packet = UdpPacket::parse(&buf[..len]).unwrap();
        assert_ne!(packet.checksum, 0);
        assert!(packet.verify_checksum(&src_ip, &dst_ip, &buf[..len]));
    }
}
