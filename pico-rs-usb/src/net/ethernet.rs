//! Ethernet frame parsing and building

use super::MacAddr;

/// Ethernet type (EtherType)
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u16)]
pub enum EtherType {
    /// IPv4
    Ipv4 = 0x0800,
    /// ARP
    Arp = 0x0806,
    /// IPv6
    Ipv6 = 0x86DD,
    /// VLAN tagged frame
    Vlan = 0x8100,
    /// Unknown type
    Unknown = 0xFFFF,
}

impl From<u16> for EtherType {
    fn from(value: u16) -> Self {
        match value {
            0x0800 => EtherType::Ipv4,
            0x0806 => EtherType::Arp,
            0x86DD => EtherType::Ipv6,
            0x8100 => EtherType::Vlan,
            _ => EtherType::Unknown,
        }
    }
}

/// Parsed Ethernet frame
#[derive(Clone, Copy, Debug)]
pub struct EthernetFrame<'a> {
    /// Destination MAC address
    pub dst: MacAddr,
    /// Source MAC address
    pub src: MacAddr,
    /// EtherType
    pub ethertype: EtherType,
    /// Payload data
    pub payload: &'a [u8],
}

impl<'a> EthernetFrame<'a> {
    /// Minimum Ethernet frame size (header only)
    pub const HEADER_SIZE: usize = 14;

    /// Parse an Ethernet frame from raw bytes
    pub fn parse(data: &'a [u8]) -> Option<Self> {
        if data.len() < Self::HEADER_SIZE {
            return None;
        }

        let dst = MacAddr::new([data[0], data[1], data[2], data[3], data[4], data[5]]);
        let src = MacAddr::new([data[6], data[7], data[8], data[9], data[10], data[11]]);
        let ethertype = EtherType::from(u16::from_be_bytes([data[12], data[13]]));
        let payload = &data[Self::HEADER_SIZE..];

        Some(Self {
            dst,
            src,
            ethertype,
            payload,
        })
    }

    /// Build an Ethernet frame into a buffer
    ///
    /// Returns the total frame length.
    pub fn build(
        buf: &mut [u8],
        dst: MacAddr,
        src: MacAddr,
        ethertype: EtherType,
        payload: &[u8],
    ) -> Option<usize> {
        let total_len = Self::HEADER_SIZE + payload.len();
        if buf.len() < total_len {
            return None;
        }

        buf[0..6].copy_from_slice(&dst.0);
        buf[6..12].copy_from_slice(&src.0);
        buf[12..14].copy_from_slice(&(ethertype as u16).to_be_bytes());
        buf[Self::HEADER_SIZE..total_len].copy_from_slice(payload);

        Some(total_len)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_ethernet_frame() {
        let data = [
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, // Destination (broadcast)
            0x02, 0x00, 0x00, 0x00, 0x00, 0x01, // Source
            0x08, 0x00, // EtherType (IPv4)
            0x45, 0x00, // Payload start
        ];

        let frame = EthernetFrame::parse(&data).unwrap();
        assert!(frame.dst.is_broadcast());
        assert_eq!(frame.ethertype, EtherType::Ipv4);
        assert_eq!(frame.payload, &[0x45, 0x00]);
    }
}
