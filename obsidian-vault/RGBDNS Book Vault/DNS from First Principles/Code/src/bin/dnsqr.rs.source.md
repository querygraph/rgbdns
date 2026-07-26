---
type: "code-file"
source_path: "src/bin/dnsqr.rs"
language: "rust"
subsystem: "Command-line programs"
crate: "dnsqr"
line_count: 20
fragment_count: 2
rgbdns_commit: "79502939"
---

# src/bin/dnsqr.rs

- Subsystem: [[DNS from First Principles/Subsystems/Command-line programs|Command-line programs]]
- Component: [[DNS from First Principles/Components/dnsqr|dnsqr]]
- Source path: `src/bin/dnsqr.rs`
- Lines: 20
- Summary: Source file in the Command-line programs subsystem.

## Extracted Fragments

- [[DNS from First Principles/Fragments/rgbdns-frag-046ae295dd0a|main]]: lines 3-9
- [[DNS from First Principles/Fragments/rgbdns-frag-4b69648afa22|run]]: lines 10-20

## Full Source

```rust
use rgbdns::{RecordType, client};

fn main() {
    if let Err(error) = run() {
        eprintln!("dnsqr: fatal: {error}");
        std::process::exit(111);
    }
}

fn run() -> rgbdns::Result<()> {
    let arguments = std::env::args().skip(1).collect::<Vec<_>>();
    if arguments.len() != 2 {
        return Err(rgbdns::Error::Format("usage: dnsqr type name"));
    }
    let record_type = arguments[0].parse::<RecordType>()?;
    let name = arguments[1].parse()?;
    println!("{} {}:", record_type.code(), name);
    println!("{:#?}", client::recursive(name, record_type)?);
    Ok(())
}
```
