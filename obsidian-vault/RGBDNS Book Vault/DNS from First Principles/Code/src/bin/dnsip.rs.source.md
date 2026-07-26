---
type: "code-file"
source_path: "src/bin/dnsip.rs"
language: "rust"
subsystem: "Command-line programs"
crate: "dnsip"
line_count: 26
fragment_count: 2
rgbdns_commit: "79502939"
---

# src/bin/dnsip.rs

- Subsystem: [[DNS from First Principles/Subsystems/Command-line programs|Command-line programs]]
- Component: [[DNS from First Principles/Components/dnsip|dnsip]]
- Source path: `src/bin/dnsip.rs`
- Lines: 26
- Summary: Source file in the Command-line programs subsystem.

## Extracted Fragments

- [[DNS from First Principles/Fragments/rgbdns-frag-b38d4016c315|main]]: lines 3-9
- [[DNS from First Principles/Fragments/rgbdns-frag-11cab49278dc|run]]: lines 10-26

## Full Source

```rust
use rgbdns::{RData, RecordType, client};

fn main() {
    if let Err(error) = run() {
        eprintln!("dnsip: fatal: {error}");
        std::process::exit(111);
    }
}

fn run() -> rgbdns::Result<()> {
    for argument in std::env::args().skip(1) {
        let response = client::recursive(argument.parse()?, RecordType::A)?;
        for address in response
            .answers
            .iter()
            .filter_map(|record| match record.data {
                RData::A(address) => Some(address),
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
