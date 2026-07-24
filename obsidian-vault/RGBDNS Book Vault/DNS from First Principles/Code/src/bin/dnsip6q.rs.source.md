---
type: "code-file"
source_path: "src/bin/dnsip6q.rs"
language: "rust"
subsystem: "Command-line programs"
crate: "dnsip6q"
line_count: 28
fragment_count: 2
rgbdns_commit: "472c2087"
---

# src/bin/dnsip6q.rs

- Subsystem: [[DNS from First Principles/Subsystems/Command-line programs|Command-line programs]]
- Component: [[DNS from First Principles/Components/dnsip6q|dnsip6q]]
- Source path: `src/bin/dnsip6q.rs`
- Lines: 28
- Summary: Source file in the Command-line programs subsystem.

## Extracted Fragments

- [[DNS from First Principles/Fragments/rgbdns-frag-faa185040c9a|main]]: lines 3-9
- [[DNS from First Principles/Fragments/rgbdns-frag-7a0c14c0646b|run]]: lines 10-28

## Full Source

```rust
use rgbdns::{Name, RData, RecordType, client};

fn main() {
    if let Err(error) = run() {
        eprintln!("dnsip6q: fatal: {error}");
        std::process::exit(111);
    }
}

fn run() -> rgbdns::Result<()> {
    for argument in std::env::args().skip(1) {
        let name: Name = argument.parse()?;
        let response = client::recursive(name.clone(), RecordType::Aaaa)?;
        print!("{name} ");
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
