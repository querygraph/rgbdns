---
type: "code-file"
source_path: "src/bin/dnsip6.rs"
language: "rust"
subsystem: "Command-line programs"
crate: "dnsip6"
line_count: 26
fragment_count: 2
rgbdns_commit: "472c2087"
---

# src/bin/dnsip6.rs

- Subsystem: [[DNS from First Principles/Subsystems/Command-line programs|Command-line programs]]
- Component: [[DNS from First Principles/Components/dnsip6|dnsip6]]
- Source path: `src/bin/dnsip6.rs`
- Lines: 26
- Summary: Source file in the Command-line programs subsystem.

## Extracted Fragments

- [[DNS from First Principles/Fragments/rgbdns-frag-91e0d84e9e85|main]]: lines 3-9
- [[DNS from First Principles/Fragments/rgbdns-frag-9934e8742654|run]]: lines 10-26

## Full Source

```rust
use rgbdns::{RData, RecordType, client};

fn main() {
    if let Err(error) = run() {
        eprintln!("dnsip6: fatal: {error}");
        std::process::exit(111);
    }
}

fn run() -> rgbdns::Result<()> {
    for argument in std::env::args().skip(1) {
        let response = client::recursive(argument.parse()?, RecordType::Aaaa)?;
        for address in response
            .answers
            .iter()
            .filter_map(|record| match record.data {
                RData::Aaaa(address) => Some(address),
                _ => None,
            })
        {
            print!("{address} ");
        }
        println!();
    }
    Ok(())
}
```
