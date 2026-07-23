#![forbid(unsafe_code)]

pub mod axfr;
pub mod cdb;
pub mod client;
pub mod conf;
pub mod dnscache_config;
pub mod multilog;
pub mod name;
pub mod packet;
pub mod pick;
pub mod rbl;
pub mod server;
#[cfg(unix)]
pub mod setuidgid;
pub mod special;
pub mod tai64;
pub mod tinydns_edit;
pub mod wall;
pub mod zone;

pub use name::Name;
pub use packet::{Message, Question, RData, Record, RecordType};

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    Format(&'static str),
    InvalidName(String),
    InvalidRecord(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(e) => e.fmt(f),
            Self::Format(s) => write!(f, "DNS format error: {s}"),
            Self::InvalidName(s) => write!(f, "invalid DNS name: {s}"),
            Self::InvalidRecord(s) => write!(f, "invalid tinydns record: {s}"),
        }
    }
}
impl std::error::Error for Error {}
impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}
pub type Result<T> = std::result::Result<T, Error>;

/// Constructs a listen address without ambiguous IPv6 string concatenation.
pub fn socket_address(ip: &str, port: &str) -> Result<std::net::SocketAddr> {
    let ip = ip
        .parse::<std::net::IpAddr>()
        .map_err(|_| Error::Format("invalid IP address"))?;
    let port = port
        .parse::<u16>()
        .map_err(|_| Error::Format("invalid port"))?;
    Ok(std::net::SocketAddr::new(ip, port))
}

#[cfg(test)]
mod address_tests {
    use super::*;

    #[test]
    fn constructs_ipv4_and_ipv6_socket_addresses() {
        assert_eq!(
            socket_address("192.0.2.1", "5353").unwrap().to_string(),
            "192.0.2.1:5353"
        );
        assert_eq!(
            socket_address("2001:db8::1", "53").unwrap().to_string(),
            "[2001:db8::1]:53"
        );
        assert!(socket_address("bad", "53").is_err());
        assert!(socket_address("127.0.0.1", "65536").is_err());
    }
}
