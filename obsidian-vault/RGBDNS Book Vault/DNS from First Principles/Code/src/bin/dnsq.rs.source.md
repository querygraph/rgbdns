---
type: "code-file"
source_path: "src/bin/dnsq.rs"
language: "rust"
subsystem: "Command-line programs"
crate: "dnsq"
line_count: 24
fragment_count: 2
rgbdns_commit: "472c2087"
---

# src/bin/dnsq.rs

- Subsystem: [[DNS from First Principles/Subsystems/Command-line programs|Command-line programs]]
- Component: [[DNS from First Principles/Components/dnsq|dnsq]]
- Source path: `src/bin/dnsq.rs`
- Lines: 24
- Summary: Source file in the Command-line programs subsystem.

## Extracted Fragments

- [[DNS from First Principles/Fragments/rgbdns-frag-400cbebbf4c7|main]]: lines 3-9
- [[DNS from First Principles/Fragments/rgbdns-frag-20811dbcff5c|run]]: lines 10-24

## Full Source

```rust
use rgbdns::{RecordType, client};

fn main() {
    if let Err(error) = run() {
        eprintln!("dnsq: fatal: {error}");
        std::process::exit(111);
    }
}

fn run() -> rgbdns::Result<()> {
    let arguments = std::env::args().skip(1).collect::<Vec<_>>();
    if arguments.len() != 3 {
        return Err(rgbdns::Error::Format("usage: dnsq type name server"));
    }
    let server = client::server_address(&arguments[2])?;
    let response = client::query(
        arguments[1].parse()?,
        arguments[0].parse::<RecordType>()?,
        false,
        &[server],
    )?;
    println!("{response:#?}");
    Ok(())
}
```
