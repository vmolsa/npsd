use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};

use super::PayloadInfo;

impl PayloadInfo for Ipv4Addr {
    const TYPE: &'static str = "Ipv4Addr";
    const SIZE: Option<usize> = Some(std::mem::size_of::<Ipv4Addr>());
}

impl PayloadInfo for Ipv6Addr {
    const TYPE: &'static str = "Ipv6Addr";
    const SIZE: Option<usize> = Some(std::mem::size_of::<Ipv6Addr>());
}

impl PayloadInfo for IpAddr {
    const TYPE: &'static str = "IpAddr";
    const SIZE: Option<usize> = Some(std::mem::size_of::<IpAddr>());
}

impl PayloadInfo for SocketAddr {
    const TYPE: &'static str = "SocketAddr";
    const SIZE: Option<usize> = <[u8; 20]>::SIZE;
}