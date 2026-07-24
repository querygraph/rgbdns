---
type: "code-file"
source_path: "src/bin/dnsname.rs"
language: "rust"
subsystem: "Command-line programs"
crate: "dnsname"
line_count: 36
fragment_count: 2
rgbdns_commit: "472c2087"
---

# src/bin/dnsname.rs

- Subsystem: [[DNS from First Principles/Subsystems/Command-line programs|Command-line programs]]
- Component: [[DNS from First Principles/Components/dnsname|dnsname]]
- Source path: `src/bin/dnsname.rs`
- Lines: 36
- Summary: Source file in the Command-line programs subsystem.

## Extracted Fragments

- [[DNS from First Principles/Fragments/rgbdns-frag-9df199853f5b|main]]: lines 4-10
- [[DNS from First Principles/Fragments/rgbdns-frag-d84decb0cd46|run]]: lines 11-36

## Full Source

```rust
use rgbdns::{RData, RecordType, client};
use std::net::Ipv4Addr;

fn main() {
    if let Err(error) = run() {
        eprintln!("dnsname: fatal: {error}");
        std::process::exit(111);
    }
}

fn run() -> rgbdns::Result<()> {
    for argument in std::env::args().skip(1) {
        let address: Ipv4Addr = argument
            .parse()
            .map_err(|_| rgbdns::Error::Format("invalid IPv4 address"))?;
        let octets = address.octets();
        let name = format!(
            "{}.{}.{}.{}.in-addr.arpa",
            octets[3], octets[2], octets[1], octets[0]
        )
        .parse()?;
        let response = client::recursive(name, RecordType::Ptr)?;
        for target in response
            .answers
            .iter()
            .filter_map(|record| match &record.data {
                RData::Name(RecordType::Ptr, target) => Some(target),
                _ => None,
            })
        {
            print!("{target}");
        }
        println!();
    }
    Ok(())
}
```
