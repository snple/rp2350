//! ARP (Address Resolution Protocol) implementation

use super::{EtherType, MacAddr};
use crate::net::ipv4::Ipv4Addr;

/// ARP operation type
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u16)]
pub enum ArpOperation {
    /// ARP Request
    Request = 1,
    /// ARP Reply
    Reply = 2,
}

impl TryFrom<u16> for ArpOperation {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(ArpOperation::Request),
            2 => Ok(ArpOperation::Reply),
            v => Err(v),
        }
    }
}

/// Parsed ARP packet
#[derive(Clone, Copy, Debug)]
pub struct ArpPacket {
    /// Hardware type (1 for Ethernet)
    pub hw_type: u16,
    /// Protocol type (0x0800 for IPv4)
    pub proto_type: u16,
    /// Hardware address length (6 for Ethernet)
    pub hw_len: u8,
    /// Protocol address length (4 for IPv4)
    pub proto_len: u8,
    /// ARP operation
    pub operation: ArpOperation,
    /// Sender hardware address (MAC)
    pub sender_mac: MacAddr,
    /// Sender protocol address (IP)
    pub sender_ip: Ipv4Addr,
    /// Target hardware address (MAC)
    pub target_mac: MacAddr,
    /// Target protocol address (IP)
    pub target_ip: Ipv4Addr,
}

impl ArpPacket {
    /// ARP packet size for Ethernet/IPv4
    pub const SIZE: usize = 28;

    /// Parse an ARP packet from raw bytes
    pub fn parse(data: &[u8]) -> Option<Self> {
        if data.len() < Self::SIZE {
            return None;
        }

        let hw_type = u16::from_be_bytes([data[0], data[1]]);
        let proto_type = u16::from_be_bytes([data[2], data[3]]);
        let hw_len = data[4];
        let proto_len = data[5];
        let operation = ArpOperation::try_from(u16::from_be_bytes([data[6], data[7]])).ok()?;

        // Only support Ethernet/IPv4
        if hw_type != 1 || proto_type != EtherType::Ipv4 as u16 || hw_len != 6 || proto_len != 4 {
            return None;
        }

        let sender_mac = MacAddr::new([data[8], data[9], data[10], data[11], data[12], data[13]]);
        let sender_ip = Ipv4Addr::new(data[14], data[15], data[16], data[17]);
        let target_mac = MacAddr::new([data[18], data[19], data[20], data[21], data[22], data[23]]);
        let target_ip = Ipv4Addr::new(data[24], data[25], data[26], data[27]);

        Some(Self {
            hw_type,
            proto_type,
            hw_len,
            proto_len,
            operation,
            sender_mac,
            sender_ip,
            target_mac,
            target_ip,
        })
    }

    /// Build an ARP packet into a buffer
    ///
    /// Returns the packet length (always 28 bytes).
    pub fn build(
        buf: &mut [u8],
        operation: ArpOperation,
        sender_mac: MacAddr,
        sender_ip: Ipv4Addr,
        target_mac: MacAddr,
        target_ip: Ipv4Addr,
    ) -> Option<usize> {
        if buf.len() < Self::SIZE {
            return None;
        }

        buf[0..2].copy_from_slice(&1u16.to_be_bytes()); // Hardware type: Ethernet
        buf[2..4].copy_from_slice(&(EtherType::Ipv4 as u16).to_be_bytes()); // Protocol type: IPv4
        buf[4] = 6; // Hardware address length
        buf[5] = 4; // Protocol address length
        buf[6..8].copy_from_slice(&(operation as u16).to_be_bytes());
        buf[8..14].copy_from_slice(&sender_mac.0);
        buf[14..18].copy_from_slice(&sender_ip.0);
        buf[18..24].copy_from_slice(&target_mac.0);
        buf[24..28].copy_from_slice(&target_ip.0);

        Some(Self::SIZE)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_arp_request() {
        let data = [
            0x00, 0x01, // Hardware type: Ethernet
            0x08, 0x00, // Protocol type: IPv4
            0x06, // Hardware length
            0x04, // Protocol length
            0x00, 0x01, // Operation: Request
            0x02, 0x00, 0x00, 0x00, 0x00, 0x01, // Sender MAC
            192, 168, 1, 100, // Sender IP
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // Target MAC
            192, 168, 1, 1, // Target IP
        ];

        let arp = ArpPacket::parse(&data).unwrap();
        assert_eq!(arp.operation, ArpOperation::Request);
        assert_eq!(arp.sender_ip, Ipv4Addr::new(192, 168, 1, 100));
        assert_eq!(arp.target_ip, Ipv4Addr::new(192, 168, 1, 1));
    }
}
