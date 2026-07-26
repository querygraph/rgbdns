---
type: "code-file"
source_path: "src/bin/dnsipq.rs"
language: "rust"
subsystem: "Command-line programs"
crate: "dnsipq"
line_count: 28
fragment_count: 2
rgbdns_commit: "79502939"
---

# src/bin/dnsipq.rs

- Subsystem: [[DNS from First Principles/Subsystems/Command-line programs|Command-line programs]]
- Component: [[DNS from First Principles/Components/dnsipq|dnsipq]]
- Source path: `src/bin/dnsipq.rs`
- Lines: 28
- Summary: Source file in the Command-line programs subsystem.

## Extracted Fragments

- [[DNS from First Principles/Fragments/rgbdns-frag-94b6d2b9f927|main]]: lines 3-9
- [[DNS from First Principles/Fragments/rgbdns-frag-39f8c406f2bb|run]]: lines 10-28

## Full Source

```rust
use rgbdns::{Name, RData, RecordType, client};

fn main() {
    if let Err(error) = run() {
        eprintln!("dnsipq: fatal: {error}");
        std::process::exit(111);
    }
}

fn run() -> rgbdns::Result<()> {
    for argument in std::env::args().skip(1) {
        let name: Name = argument.parse()?;
        let response = client::recursive(name.clone(), RecordType::A)?;
        print!("{name} ");
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
