---
type: "code-file"
source_path: "src/lib.rs"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
line_count: 81
fragment_count: 29
rgbdns_commit: "79502939"
---

# src/lib.rs

- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]
- Source path: `src/lib.rs`
- Lines: 81
- Summary: Constructs a listen address without ambiguous IPv6 string concatenation.

## Extracted Fragments

- [[DNS from First Principles/Fragments/rgbdns-frag-f52ccf723277|aname]]: lines 3-3
- [[DNS from First Principles/Fragments/rgbdns-frag-4a817a7124e1|axfr]]: lines 4-4
- [[DNS from First Principles/Fragments/rgbdns-frag-ef82c203a6e1|cdb]]: lines 5-5
- [[DNS from First Principles/Fragments/rgbdns-frag-ac93886065b4|client]]: lines 6-6
- [[DNS from First Principles/Fragments/rgbdns-frag-558fa31b05c5|conf]]: lines 7-7
- [[DNS from First Principles/Fragments/rgbdns-frag-fa79f1453710|dnscache_config]]: lines 8-8
- [[DNS from First Principles/Fragments/rgbdns-frag-060fb35dda55|multilog]]: lines 9-9
- [[DNS from First Principles/Fragments/rgbdns-frag-9b91c16392f6|name]]: lines 10-10
- [[DNS from First Principles/Fragments/rgbdns-frag-fb88c09e6337|packet]]: lines 11-11
- [[DNS from First Principles/Fragments/rgbdns-frag-09f6fa5decbe|pick]]: lines 12-12
- [[DNS from First Principles/Fragments/rgbdns-frag-6e6427a50d71|rbl]]: lines 13-13
- [[DNS from First Principles/Fragments/rgbdns-frag-e5dfdf77ddb4|server]]: lines 14-15
- [[DNS from First Principles/Fragments/rgbdns-frag-4399bf8d7bca|setuidgid]]: lines 16-16
- [[DNS from First Principles/Fragments/rgbdns-frag-a9b71133a8a7|special]]: lines 17-17
- [[DNS from First Principles/Fragments/rgbdns-frag-c381c3286bae|tai64]]: lines 18-18
- [[DNS from First Principles/Fragments/rgbdns-frag-3cb5aab2f951|tinydns_edit]]: lines 19-19
- [[DNS from First Principles/Fragments/rgbdns-frag-65d27f5c3043|transport]]: lines 20-20
- [[DNS from First Principles/Fragments/rgbdns-frag-6f2db39a6784|wall]]: lines 21-21
- [[DNS from First Principles/Fragments/rgbdns-frag-3591e119df49|zone]]: lines 22-27
- [[DNS from First Principles/Fragments/rgbdns-frag-cf6623db375a|Error]]: lines 28-34
- [[DNS from First Principles/Fragments/rgbdns-frag-81662b0f8334|std]]: lines 35-35
- [[DNS from First Principles/Fragments/rgbdns-frag-b7a10a8752c6|fmt]]: lines 36-44
- [[DNS from First Principles/Fragments/rgbdns-frag-767195231108|std]]: lines 45-45
- [[DNS from First Principles/Fragments/rgbdns-frag-02f6775511ec|From]]: lines 46-46
- [[DNS from First Principles/Fragments/rgbdns-frag-3fe85af2221d|from]]: lines 47-50
- [[DNS from First Principles/Fragments/rgbdns-frag-4373cfd6a122|Result]]: lines 51-53
- [[DNS from First Principles/Fragments/rgbdns-frag-2c97a1c68bb4|socket_address]]: lines 54-64
- [[DNS from First Principles/Fragments/rgbdns-frag-dd954b8525ae|address_tests]]: lines 65-68
- [[DNS from First Principles/Fragments/rgbdns-frag-61289dbc0218|constructs_ipv4_and_ipv6_socket_addresses]]: lines 69-81

## Full Source

```rust
#![forbid(unsafe_code)]

mod aname;
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
mod transport;
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
```
