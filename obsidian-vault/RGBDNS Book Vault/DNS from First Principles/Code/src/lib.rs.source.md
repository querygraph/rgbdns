---
type: "code-file"
source_path: "src/lib.rs"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
line_count: 80
fragment_count: 28
rgbdns_commit: "472c2087"
---

# src/lib.rs

- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]
- Source path: `src/lib.rs`
- Lines: 80
- Summary: Constructs a listen address without ambiguous IPv6 string concatenation.

## Extracted Fragments

- [[DNS from First Principles/Fragments/rgbdns-frag-d26839b00cc0|axfr]]: lines 3-3
- [[DNS from First Principles/Fragments/rgbdns-frag-c0387f24e3b2|cdb]]: lines 4-4
- [[DNS from First Principles/Fragments/rgbdns-frag-ea9a942c2c24|client]]: lines 5-5
- [[DNS from First Principles/Fragments/rgbdns-frag-4ca6b4a66ffe|conf]]: lines 6-6
- [[DNS from First Principles/Fragments/rgbdns-frag-47cdd9e7aa73|dnscache_config]]: lines 7-7
- [[DNS from First Principles/Fragments/rgbdns-frag-64be14e515dc|multilog]]: lines 8-8
- [[DNS from First Principles/Fragments/rgbdns-frag-1198b492e8d3|name]]: lines 9-9
- [[DNS from First Principles/Fragments/rgbdns-frag-9a69adb381ac|packet]]: lines 10-10
- [[DNS from First Principles/Fragments/rgbdns-frag-cc4b83e1818c|pick]]: lines 11-11
- [[DNS from First Principles/Fragments/rgbdns-frag-b32ba925e7c1|rbl]]: lines 12-12
- [[DNS from First Principles/Fragments/rgbdns-frag-fc0223cab124|server]]: lines 13-14
- [[DNS from First Principles/Fragments/rgbdns-frag-17439937a795|setuidgid]]: lines 15-15
- [[DNS from First Principles/Fragments/rgbdns-frag-53eaa3274eb7|special]]: lines 16-16
- [[DNS from First Principles/Fragments/rgbdns-frag-bd03ec8b2f1a|tai64]]: lines 17-17
- [[DNS from First Principles/Fragments/rgbdns-frag-3a23dfdf3153|tinydns_edit]]: lines 18-18
- [[DNS from First Principles/Fragments/rgbdns-frag-65d3faf4b4df|transport]]: lines 19-19
- [[DNS from First Principles/Fragments/rgbdns-frag-45a926c96a17|wall]]: lines 20-20
- [[DNS from First Principles/Fragments/rgbdns-frag-460d033a70bc|zone]]: lines 21-26
- [[DNS from First Principles/Fragments/rgbdns-frag-a504095bfade|Error]]: lines 27-33
- [[DNS from First Principles/Fragments/rgbdns-frag-1f2f52292c17|std]]: lines 34-34
- [[DNS from First Principles/Fragments/rgbdns-frag-762fbc6aec4b|fmt]]: lines 35-43
- [[DNS from First Principles/Fragments/rgbdns-frag-aafca40edd3c|std]]: lines 44-44
- [[DNS from First Principles/Fragments/rgbdns-frag-096bc6ecb319|From]]: lines 45-45
- [[DNS from First Principles/Fragments/rgbdns-frag-c25798b08316|from]]: lines 46-49
- [[DNS from First Principles/Fragments/rgbdns-frag-961bde153386|Result]]: lines 50-52
- [[DNS from First Principles/Fragments/rgbdns-frag-54ed88397082|socket_address]]: lines 53-63
- [[DNS from First Principles/Fragments/rgbdns-frag-d546da74e4e8|address_tests]]: lines 64-67
- [[DNS from First Principles/Fragments/rgbdns-frag-9f5243a9e8af|constructs_ipv4_and_ipv6_socket_addresses]]: lines 68-80

## Full Source

```rust
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
