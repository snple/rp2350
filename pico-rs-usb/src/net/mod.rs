//! Lightweight Network Protocol Stack
//!
//! This module provides a minimal network protocol stack for embedded use,
//! supporting IPv4, ICMP, ARP, and UDP protocols.
//!
//! ## Features
//!
//! - Ethernet frame parsing and building
//! - ARP request/response handling
//! - IPv4 packet handling
//! - ICMP Echo (ping) support
//! - UDP datagram send/receive
//!
//! ## Example
//!
//! ```ignore
//! use pico_rs_usb::net::{NetInterface, Ipv4Addr, MacAddr};
//!
//! let mac = MacAddr::new([0x02, 0x00, 0x00, 0x00, 0x00, 0x01]);
//! let ip = Ipv4Addr::new(192, 168, 7, 1);
//! let mut iface = NetInterface::new(mac, ip);
//!
//! // Process incoming frame
//! if let Some(response) = iface.process_frame(&received_data) {
//!     // Send response
//!     ecm.transmit(&response)?;
//! }
//!
//! // Send UDP packet
//! let dest_ip = Ipv4Addr::new(192, 168, 7, 2);
//! iface.send_udp(dest_ip, 1234, 5678, b"Hello")?;
//! ```

#![allow(dead_code)]

mod arp;
mod ethernet;
mod icmp;
mod ipv4;
mod udp;

pub use arp::{ArpOperation, ArpPacket};
pub use ethernet::{EtherType, EthernetFrame};
pub use icmp::{IcmpPacket, IcmpType};
pub use ipv4::{IpProtocol, Ipv4Addr, Ipv4Packet};
pub use udp::UdpPacket;

use crate::error::{UsbError, UsbResult};

/// Maximum Ethernet frame size
pub const MAX_FRAME_SIZE: usize = 1514;

/// Ethernet header size
pub const ETH_HEADER_SIZE: usize = 14;

/// IPv4 header size (without options)
pub const IPV4_HEADER_SIZE: usize = 20;

/// ICMP header size
pub const ICMP_HEADER_SIZE: usize = 8;

/// UDP header size
pub const UDP_HEADER_SIZE: usize = 8;

/// ARP packet size
pub const ARP_PACKET_SIZE: usize = 28;

/// MAC address (6 bytes)
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub struct MacAddr(pub [u8; 6]);

impl MacAddr {
    /// Create a new MAC address
    pub const fn new(bytes: [u8; 6]) -> Self {
        Self(bytes)
    }

    /// Broadcast MAC address
    pub const BROADCAST: Self = Self([0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);

    /// Zero MAC address
    pub const ZERO: Self = Self([0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);

    /// Check if this is a broadcast address
    pub fn is_broadcast(&self) -> bool {
        self.0 == Self::BROADCAST.0
    }

    /// Check if this is a multicast address
    pub fn is_multicast(&self) -> bool {
        self.0[0] & 0x01 != 0
    }

    /// Get the raw bytes
    pub fn as_bytes(&self) -> &[u8; 6] {
        &self.0
    }
}

impl core::fmt::Debug for MacAddr {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}",
            self.0[0], self.0[1], self.0[2], self.0[3], self.0[4], self.0[5]
        )
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for MacAddr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}",
            self.0[0],
            self.0[1],
            self.0[2],
            self.0[3],
            self.0[4],
            self.0[5]
        )
    }
}

impl From<[u8; 6]> for MacAddr {
    fn from(bytes: [u8; 6]) -> Self {
        Self(bytes)
    }
}

/// ARP cache entry
#[derive(Clone, Copy, Debug)]
struct ArpCacheEntry {
    ip: Ipv4Addr,
    mac: MacAddr,
    /// Time-to-live (in poll cycles, decremented each poll)
    ttl: u16,
}

/// Simple ARP cache
const ARP_CACHE_SIZE: usize = 8;

/// Network interface
///
/// Manages network state and provides protocol processing.
pub struct NetInterface {
    /// Local MAC address
    mac: MacAddr,
    /// Local IPv4 address
    ip: Ipv4Addr,
    /// Subnet mask
    netmask: Ipv4Addr,
    /// Gateway address (optional)
    gateway: Option<Ipv4Addr>,
    /// ARP cache
    arp_cache: [Option<ArpCacheEntry>; ARP_CACHE_SIZE],
    /// IPv4 identification counter
    ip_id: u16,
    /// Transmit buffer
    tx_buf: [u8; MAX_FRAME_SIZE],
    /// UDP receive callback storage
    udp_ports: [Option<u16>; 4],
}

impl NetInterface {
    /// Create a new network interface
    pub fn new(mac: MacAddr, ip: Ipv4Addr) -> Self {
        Self {
            mac,
            ip,
            netmask: Ipv4Addr::new(255, 255, 255, 0),
            gateway: None,
            arp_cache: [None; ARP_CACHE_SIZE],
            ip_id: 0,
            tx_buf: [0u8; MAX_FRAME_SIZE],
            udp_ports: [None; 4],
        }
    }

    /// Create with custom netmask
    pub fn with_netmask(mac: MacAddr, ip: Ipv4Addr, netmask: Ipv4Addr) -> Self {
        Self {
            mac,
            ip,
            netmask,
            gateway: None,
            arp_cache: [None; ARP_CACHE_SIZE],
            ip_id: 0,
            tx_buf: [0u8; MAX_FRAME_SIZE],
            udp_ports: [None; 4],
        }
    }

    /// Set gateway address
    pub fn set_gateway(&mut self, gateway: Ipv4Addr) {
        self.gateway = Some(gateway);
    }

    /// Get local MAC address
    pub fn mac(&self) -> MacAddr {
        self.mac
    }

    /// Get local IP address
    pub fn ip(&self) -> Ipv4Addr {
        self.ip
    }

    /// Register a UDP port for receiving
    pub fn register_udp_port(&mut self, port: u16) -> bool {
        for slot in &mut self.udp_ports {
            if slot.is_none() {
                *slot = Some(port);
                return true;
            }
        }
        false
    }

    /// Unregister a UDP port
    pub fn unregister_udp_port(&mut self, port: u16) {
        for slot in &mut self.udp_ports {
            if *slot == Some(port) {
                *slot = None;
                return;
            }
        }
    }

    /// Check if a UDP port is registered
    fn is_udp_port_registered(&self, port: u16) -> bool {
        self.udp_ports.contains(&Some(port))
    }

    /// Process an incoming Ethernet frame
    ///
    /// Returns an optional response frame to send.
    pub fn process_frame<'a>(&'a mut self, data: &[u8]) -> Option<&'a [u8]> {
        let frame = EthernetFrame::parse(data)?;

        // Check if frame is for us
        if !frame.dst.is_broadcast() && frame.dst != self.mac {
            return None;
        }

        match frame.ethertype {
            EtherType::Arp => self.process_arp(frame.payload),
            EtherType::Ipv4 => self.process_ipv4(frame.payload, &frame.src),
            _ => None,
        }
    }

    /// Process incoming frame and return parsed UDP data if available
    ///
    /// Returns (source_ip, source_port, dest_port, payload) if UDP packet received.
    pub fn process_frame_udp<'a>(
        &mut self,
        data: &'a [u8],
    ) -> Option<(Ipv4Addr, u16, u16, &'a [u8])> {
        let frame = EthernetFrame::parse(data)?;

        // Check if frame is for us
        if !frame.dst.is_broadcast() && frame.dst != self.mac {
            return None;
        }

        if frame.ethertype != EtherType::Ipv4 {
            return None;
        }

        let ip_packet = Ipv4Packet::parse(frame.payload)?;

        // Check destination IP
        if ip_packet.dst != self.ip && !ip_packet.dst.is_broadcast() {
            return None;
        }

        if ip_packet.protocol != IpProtocol::Udp {
            return None;
        }

        let udp = UdpPacket::parse(ip_packet.payload)?;

        // Check if port is registered
        if !self.is_udp_port_registered(udp.dst_port) {
            return None;
        }

        // Update ARP cache
        self.arp_cache_update(ip_packet.src, frame.src);

        Some((ip_packet.src, udp.src_port, udp.dst_port, udp.payload))
    }

    /// Process ARP packet
    fn process_arp<'a>(&'a mut self, data: &[u8]) -> Option<&'a [u8]> {
        let arp = ArpPacket::parse(data)?;

        // Only handle Ethernet/IPv4 ARP
        if arp.hw_type != 1 || arp.proto_type != EtherType::Ipv4 as u16 {
            return None;
        }

        // Update ARP cache with sender info
        self.arp_cache_update(arp.sender_ip, arp.sender_mac);

        match arp.operation {
            ArpOperation::Request => {
                // Check if request is for our IP
                if arp.target_ip != self.ip {
                    return None;
                }

                // Build ARP reply
                let len = self.build_arp_reply(&arp);
                Some(&self.tx_buf[..len])
            }
            ArpOperation::Reply => {
                // Already updated cache above
                None
            }
        }
    }

    /// Process IPv4 packet
    fn process_ipv4<'a>(&'a mut self, data: &[u8], src_mac: &MacAddr) -> Option<&'a [u8]> {
        let ip_packet = Ipv4Packet::parse(data)?;

        // Check destination IP
        if ip_packet.dst != self.ip && !ip_packet.dst.is_broadcast() {
            return None;
        }

        // Update ARP cache
        self.arp_cache_update(ip_packet.src, *src_mac);

        match ip_packet.protocol {
            IpProtocol::Icmp => self.process_icmp(&ip_packet, src_mac),
            IpProtocol::Udp => {
                // UDP packets are handled via process_frame_udp
                None
            }
            _ => None,
        }
    }

    /// Process ICMP packet
    fn process_icmp<'a>(
        &'a mut self,
        ip_packet: &Ipv4Packet,
        src_mac: &MacAddr,
    ) -> Option<&'a [u8]> {
        let icmp = IcmpPacket::parse(ip_packet.payload)?;

        match icmp.icmp_type {
            IcmpType::EchoRequest => {
                // Build echo reply
                let len = self.build_icmp_reply(ip_packet, &icmp, src_mac);
                Some(&self.tx_buf[..len])
            }
            _ => None,
        }
    }

    /// Build ARP reply
    fn build_arp_reply(&mut self, request: &ArpPacket) -> usize {
        let mut offset = 0;

        // Ethernet header
        self.tx_buf[offset..offset + 6].copy_from_slice(&request.sender_mac.0);
        offset += 6;
        self.tx_buf[offset..offset + 6].copy_from_slice(&self.mac.0);
        offset += 6;
        self.tx_buf[offset..offset + 2].copy_from_slice(&(EtherType::Arp as u16).to_be_bytes());
        offset += 2;

        // ARP payload
        self.tx_buf[offset..offset + 2].copy_from_slice(&1u16.to_be_bytes()); // Hardware type
        offset += 2;
        self.tx_buf[offset..offset + 2].copy_from_slice(&(EtherType::Ipv4 as u16).to_be_bytes()); // Protocol type
        offset += 2;
        self.tx_buf[offset] = 6; // Hardware size
        offset += 1;
        self.tx_buf[offset] = 4; // Protocol size
        offset += 1;
        self.tx_buf[offset..offset + 2]
            .copy_from_slice(&(ArpOperation::Reply as u16).to_be_bytes());
        offset += 2;

        // Sender (us)
        self.tx_buf[offset..offset + 6].copy_from_slice(&self.mac.0);
        offset += 6;
        self.tx_buf[offset..offset + 4].copy_from_slice(&self.ip.0);
        offset += 4;

        // Target (requester)
        self.tx_buf[offset..offset + 6].copy_from_slice(&request.sender_mac.0);
        offset += 6;
        self.tx_buf[offset..offset + 4].copy_from_slice(&request.sender_ip.0);
        offset += 4;

        offset
    }

    /// Build ICMP echo reply
    fn build_icmp_reply(
        &mut self,
        request_ip: &Ipv4Packet,
        request_icmp: &IcmpPacket,
        dst_mac: &MacAddr,
    ) -> usize {
        let icmp_len = ICMP_HEADER_SIZE + request_icmp.payload.len();
        let ip_len = IPV4_HEADER_SIZE + icmp_len;
        let total_len = ETH_HEADER_SIZE + ip_len;

        let mut offset = 0;

        // Ethernet header
        self.tx_buf[offset..offset + 6].copy_from_slice(&dst_mac.0);
        offset += 6;
        self.tx_buf[offset..offset + 6].copy_from_slice(&self.mac.0);
        offset += 6;
        self.tx_buf[offset..offset + 2].copy_from_slice(&(EtherType::Ipv4 as u16).to_be_bytes());
        offset += 2;

        // IPv4 header
        self.tx_buf[offset] = 0x45; // Version + IHL
        offset += 1;
        self.tx_buf[offset] = 0; // DSCP + ECN
        offset += 1;
        self.tx_buf[offset..offset + 2].copy_from_slice(&(ip_len as u16).to_be_bytes());
        offset += 2;
        self.ip_id = self.ip_id.wrapping_add(1);
        self.tx_buf[offset..offset + 2].copy_from_slice(&self.ip_id.to_be_bytes());
        offset += 2;
        self.tx_buf[offset..offset + 2].copy_from_slice(&0u16.to_be_bytes()); // Flags + Fragment
        offset += 2;
        self.tx_buf[offset] = 64; // TTL
        offset += 1;
        self.tx_buf[offset] = IpProtocol::Icmp as u8;
        offset += 1;
        self.tx_buf[offset..offset + 2].copy_from_slice(&0u16.to_be_bytes()); // Checksum placeholder
        offset += 2;
        self.tx_buf[offset..offset + 4].copy_from_slice(&self.ip.0);
        offset += 4;
        self.tx_buf[offset..offset + 4].copy_from_slice(&request_ip.src.0);
        offset += 4;

        // Calculate IP checksum
        let ip_checksum =
            Self::checksum(&self.tx_buf[ETH_HEADER_SIZE..ETH_HEADER_SIZE + IPV4_HEADER_SIZE]);
        self.tx_buf[ETH_HEADER_SIZE + 10..ETH_HEADER_SIZE + 12]
            .copy_from_slice(&ip_checksum.to_be_bytes());

        // ICMP header
        let icmp_start = offset;
        self.tx_buf[offset] = IcmpType::EchoReply as u8;
        offset += 1;
        self.tx_buf[offset] = 0; // Code
        offset += 1;
        self.tx_buf[offset..offset + 2].copy_from_slice(&0u16.to_be_bytes()); // Checksum placeholder
        offset += 2;
        // Copy identifier and sequence from request
        self.tx_buf[offset..offset + 4].copy_from_slice(&request_icmp.rest_of_header);
        offset += 4;

        // Copy payload
        self.tx_buf[offset..offset + request_icmp.payload.len()]
            .copy_from_slice(request_icmp.payload);
        offset += request_icmp.payload.len();

        // Calculate ICMP checksum
        let icmp_checksum = Self::checksum(&self.tx_buf[icmp_start..offset]);
        self.tx_buf[icmp_start + 2..icmp_start + 4].copy_from_slice(&icmp_checksum.to_be_bytes());

        total_len
    }

    /// Build and return a UDP packet
    ///
    /// Returns the length of the packet in the internal buffer.
    pub fn build_udp_packet(
        &mut self,
        dst_mac: MacAddr,
        dst_ip: Ipv4Addr,
        src_port: u16,
        dst_port: u16,
        payload: &[u8],
    ) -> UsbResult<&[u8]> {
        let udp_len = UDP_HEADER_SIZE + payload.len();
        let ip_len = IPV4_HEADER_SIZE + udp_len;
        let total_len = ETH_HEADER_SIZE + ip_len;

        if total_len > MAX_FRAME_SIZE {
            return Err(UsbError::BufferOverflow);
        }

        let mut offset = 0;

        // Ethernet header
        self.tx_buf[offset..offset + 6].copy_from_slice(&dst_mac.0);
        offset += 6;
        self.tx_buf[offset..offset + 6].copy_from_slice(&self.mac.0);
        offset += 6;
        self.tx_buf[offset..offset + 2].copy_from_slice(&(EtherType::Ipv4 as u16).to_be_bytes());
        offset += 2;

        // IPv4 header
        self.tx_buf[offset] = 0x45; // Version + IHL
        offset += 1;
        self.tx_buf[offset] = 0; // DSCP + ECN
        offset += 1;
        self.tx_buf[offset..offset + 2].copy_from_slice(&(ip_len as u16).to_be_bytes());
        offset += 2;
        self.ip_id = self.ip_id.wrapping_add(1);
        self.tx_buf[offset..offset + 2].copy_from_slice(&self.ip_id.to_be_bytes());
        offset += 2;
        self.tx_buf[offset..offset + 2].copy_from_slice(&0u16.to_be_bytes()); // Flags + Fragment
        offset += 2;
        self.tx_buf[offset] = 64; // TTL
        offset += 1;
        self.tx_buf[offset] = IpProtocol::Udp as u8;
        offset += 1;
        self.tx_buf[offset..offset + 2].copy_from_slice(&0u16.to_be_bytes()); // Checksum placeholder
        offset += 2;
        self.tx_buf[offset..offset + 4].copy_from_slice(&self.ip.0);
        offset += 4;
        self.tx_buf[offset..offset + 4].copy_from_slice(&dst_ip.0);
        offset += 4;

        // Calculate IP checksum
        let ip_checksum =
            Self::checksum(&self.tx_buf[ETH_HEADER_SIZE..ETH_HEADER_SIZE + IPV4_HEADER_SIZE]);
        self.tx_buf[ETH_HEADER_SIZE + 10..ETH_HEADER_SIZE + 12]
            .copy_from_slice(&ip_checksum.to_be_bytes());

        // UDP header
        let udp_start = offset;
        self.tx_buf[offset..offset + 2].copy_from_slice(&src_port.to_be_bytes());
        offset += 2;
        self.tx_buf[offset..offset + 2].copy_from_slice(&dst_port.to_be_bytes());
        offset += 2;
        self.tx_buf[offset..offset + 2].copy_from_slice(&(udp_len as u16).to_be_bytes());
        offset += 2;
        self.tx_buf[offset..offset + 2].copy_from_slice(&0u16.to_be_bytes()); // Checksum (optional for IPv4)
        offset += 2;

        // UDP payload
        self.tx_buf[offset..offset + payload.len()].copy_from_slice(payload);
        offset += payload.len();

        // Calculate UDP checksum (optional but recommended)
        let udp_checksum = self.udp_checksum(&self.ip, &dst_ip, &self.tx_buf[udp_start..offset]);
        self.tx_buf[udp_start + 6..udp_start + 8].copy_from_slice(&udp_checksum.to_be_bytes());

        Ok(&self.tx_buf[..total_len])
    }

    /// Send UDP packet (returns frame to transmit)
    ///
    /// Looks up MAC address in ARP cache. Returns None if MAC unknown.
    pub fn send_udp(
        &mut self,
        dst_ip: Ipv4Addr,
        src_port: u16,
        dst_port: u16,
        payload: &[u8],
    ) -> UsbResult<Option<&[u8]>> {
        // Check if destination is on local network
        let target_ip = if self.is_local(&dst_ip) {
            dst_ip
        } else {
            // Use gateway
            self.gateway.ok_or(UsbError::InvalidState)?
        };

        // Lookup MAC in ARP cache
        let dst_mac = match self.arp_cache_lookup(target_ip) {
            Some(mac) => mac,
            None => return Ok(None), // Need to send ARP request first
        };

        let frame = self.build_udp_packet(dst_mac, dst_ip, src_port, dst_port, payload)?;
        Ok(Some(frame))
    }

    /// Build an ARP request for the given IP
    pub fn build_arp_request(&mut self, target_ip: Ipv4Addr) -> &[u8] {
        let mut offset = 0;

        // Ethernet header
        self.tx_buf[offset..offset + 6].copy_from_slice(&MacAddr::BROADCAST.0);
        offset += 6;
        self.tx_buf[offset..offset + 6].copy_from_slice(&self.mac.0);
        offset += 6;
        self.tx_buf[offset..offset + 2].copy_from_slice(&(EtherType::Arp as u16).to_be_bytes());
        offset += 2;

        // ARP payload
        self.tx_buf[offset..offset + 2].copy_from_slice(&1u16.to_be_bytes()); // Hardware type
        offset += 2;
        self.tx_buf[offset..offset + 2].copy_from_slice(&(EtherType::Ipv4 as u16).to_be_bytes()); // Protocol type
        offset += 2;
        self.tx_buf[offset] = 6; // Hardware size
        offset += 1;
        self.tx_buf[offset] = 4; // Protocol size
        offset += 1;
        self.tx_buf[offset..offset + 2]
            .copy_from_slice(&(ArpOperation::Request as u16).to_be_bytes());
        offset += 2;

        // Sender (us)
        self.tx_buf[offset..offset + 6].copy_from_slice(&self.mac.0);
        offset += 6;
        self.tx_buf[offset..offset + 4].copy_from_slice(&self.ip.0);
        offset += 4;

        // Target
        self.tx_buf[offset..offset + 6].copy_from_slice(&MacAddr::ZERO.0);
        offset += 6;
        self.tx_buf[offset..offset + 4].copy_from_slice(&target_ip.0);
        offset += 4;

        &self.tx_buf[..offset]
    }

    /// Check if IP is on local network
    fn is_local(&self, ip: &Ipv4Addr) -> bool {
        for i in 0..4 {
            if (ip.0[i] & self.netmask.0[i]) != (self.ip.0[i] & self.netmask.0[i]) {
                return false;
            }
        }
        true
    }

    /// Update ARP cache
    fn arp_cache_update(&mut self, ip: Ipv4Addr, mac: MacAddr) {
        // Check if already in cache
        for entry in self.arp_cache.iter_mut().flatten() {
            if entry.ip == ip {
                entry.mac = mac;
                entry.ttl = 300; // Reset TTL
                return;
            }
        }

        // Find empty slot or oldest entry
        let mut oldest_idx = 0;
        let mut oldest_ttl = u16::MAX;

        for (i, entry) in self.arp_cache.iter().enumerate() {
            match entry {
                None => {
                    oldest_idx = i;
                    break;
                }
                Some(e) if e.ttl < oldest_ttl => {
                    oldest_idx = i;
                    oldest_ttl = e.ttl;
                }
                _ => {}
            }
        }

        self.arp_cache[oldest_idx] = Some(ArpCacheEntry { ip, mac, ttl: 300 });
    }

    /// Lookup MAC address in ARP cache
    fn arp_cache_lookup(&self, ip: Ipv4Addr) -> Option<MacAddr> {
        for entry in self.arp_cache.iter().flatten() {
            if entry.ip == ip {
                return Some(entry.mac);
            }
        }
        None
    }

    /// Decrement ARP cache TTL (call periodically)
    pub fn arp_cache_tick(&mut self) {
        for entry in &mut self.arp_cache {
            if let Some(e) = entry {
                if e.ttl > 0 {
                    e.ttl -= 1;
                }
                if e.ttl == 0 {
                    *entry = None;
                }
            }
        }
    }

    /// Calculate Internet checksum
    fn checksum(data: &[u8]) -> u16 {
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

    /// Calculate UDP checksum with pseudo-header
    fn udp_checksum(&self, src_ip: &Ipv4Addr, dst_ip: &Ipv4Addr, udp_segment: &[u8]) -> u16 {
        let mut sum: u32 = 0;

        // Pseudo-header
        sum += u16::from_be_bytes([src_ip.0[0], src_ip.0[1]]) as u32;
        sum += u16::from_be_bytes([src_ip.0[2], src_ip.0[3]]) as u32;
        sum += u16::from_be_bytes([dst_ip.0[0], dst_ip.0[1]]) as u32;
        sum += u16::from_be_bytes([dst_ip.0[2], dst_ip.0[3]]) as u32;
        sum += IpProtocol::Udp as u32;
        sum += udp_segment.len() as u32;

        // UDP segment
        let mut i = 0;
        while i < udp_segment.len() - 1 {
            // Skip checksum field
            if i == 6 {
                i += 2;
                continue;
            }
            sum += u16::from_be_bytes([udp_segment[i], udp_segment[i + 1]]) as u32;
            i += 2;
        }

        if udp_segment.len() % 2 == 1 {
            sum += (udp_segment[udp_segment.len() - 1] as u32) << 8;
        }

        while sum > 0xFFFF {
            sum = (sum & 0xFFFF) + (sum >> 16);
        }

        let result = !sum as u16;
        if result == 0 {
            0xFFFF // UDP checksum of 0 is transmitted as 0xFFFF
        } else {
            result
        }
    }
}
