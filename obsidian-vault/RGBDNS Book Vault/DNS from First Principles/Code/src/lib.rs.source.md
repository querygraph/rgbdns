---
type: "code-file"
source_path: "src/lib.rs"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
line_count: 83
fragment_count: 31
rgbdns_commit: "5ae24772"
---

# src/lib.rs

- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]
- Source path: `src/lib.rs`
- Lines: 83
- Summary: Constructs a listen address without ambiguous IPv6 string concatenation.

## Extracted Fragments

- [[DNS from First Principles/Fragments/rgbdns-frag-0764202cbb5b|acme]]: lines 3-3
- [[DNS from First Principles/Fragments/rgbdns-frag-d814f58c3808|aname]]: lines 4-4
- [[DNS from First Principles/Fragments/rgbdns-frag-137fad6744bd|axfr]]: lines 5-5
- [[DNS from First Principles/Fragments/rgbdns-frag-56c62ce446e3|cdb]]: lines 6-6
- [[DNS from First Principles/Fragments/rgbdns-frag-bc885b4aa7d2|client]]: lines 7-7
- [[DNS from First Principles/Fragments/rgbdns-frag-56f7dee056ca|conf]]: lines 8-8
- [[DNS from First Principles/Fragments/rgbdns-frag-da2d3b034c7a|dnscache_config]]: lines 9-9
- [[DNS from First Principles/Fragments/rgbdns-frag-a6cdb39853c9|dnssec]]: lines 10-10
- [[DNS from First Principles/Fragments/rgbdns-frag-b36bcc23faf7|multilog]]: lines 11-11
- [[DNS from First Principles/Fragments/rgbdns-frag-983676a6753c|name]]: lines 12-12
- [[DNS from First Principles/Fragments/rgbdns-frag-e3c86250c709|packet]]: lines 13-13
- [[DNS from First Principles/Fragments/rgbdns-frag-1558698bf167|pick]]: lines 14-14
- [[DNS from First Principles/Fragments/rgbdns-frag-255cc902289d|rbl]]: lines 15-15
- [[DNS from First Principles/Fragments/rgbdns-frag-2c8edce20554|server]]: lines 16-17
- [[DNS from First Principles/Fragments/rgbdns-frag-d26c0683c011|setuidgid]]: lines 18-18
- [[DNS from First Principles/Fragments/rgbdns-frag-d5bf794126be|special]]: lines 19-19
- [[DNS from First Principles/Fragments/rgbdns-frag-a0aa9852935c|tai64]]: lines 20-20
- [[DNS from First Principles/Fragments/rgbdns-frag-28bde1925187|tinydns_edit]]: lines 21-21
- [[DNS from First Principles/Fragments/rgbdns-frag-14457cfcfbac|transport]]: lines 22-22
- [[DNS from First Principles/Fragments/rgbdns-frag-8c5add6775f5|wall]]: lines 23-23
- [[DNS from First Principles/Fragments/rgbdns-frag-a42b86cbda5b|zone]]: lines 24-29
- [[DNS from First Principles/Fragments/rgbdns-frag-32d77847bf6c|Error]]: lines 30-36
- [[DNS from First Principles/Fragments/rgbdns-frag-3dbe316885d2|std]]: lines 37-37
- [[DNS from First Principles/Fragments/rgbdns-frag-6ed18c32b5da|fmt]]: lines 38-46
- [[DNS from First Principles/Fragments/rgbdns-frag-a52891f21993|std]]: lines 47-47
- [[DNS from First Principles/Fragments/rgbdns-frag-2f1017ffb9a6|From]]: lines 48-48
- [[DNS from First Principles/Fragments/rgbdns-frag-ac93eef1f009|from]]: lines 49-52
- [[DNS from First Principles/Fragments/rgbdns-frag-e7b7438516db|Result]]: lines 53-55
- [[DNS from First Principles/Fragments/rgbdns-frag-e9d3572dc4d9|socket_address]]: lines 56-66
- [[DNS from First Principles/Fragments/rgbdns-frag-5fd00984d01a|address_tests]]: lines 67-70
- [[DNS from First Principles/Fragments/rgbdns-frag-c2b6e405dc68|constructs_ipv4_and_ipv6_socket_addresses]]: lines 71-83

## Full Source

```rust
#![forbid(unsafe_code)]

pub mod acme;
mod aname;
pub mod axfr;
pub mod cdb;
pub mod client;
pub mod conf;
pub mod dnscache_config;
pub mod dnssec;
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
pub use packet::{DnssecRData, Message, Question, RData, Record, RecordType};

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
