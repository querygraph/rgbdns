---
type: "code-file"
source_path: "src/bin/walldns.rs"
language: "rust"
subsystem: "Command-line programs"
crate: "walldns"
line_count: 20
fragment_count: 2
rgbdns_commit: "79502939"
---

# src/bin/walldns.rs

- Subsystem: [[DNS from First Principles/Subsystems/Command-line programs|Command-line programs]]
- Component: [[DNS from First Principles/Components/walldns|walldns]]
- Source path: `src/bin/walldns.rs`
- Lines: 20
- Summary: Source file in the Command-line programs subsystem.

## Extracted Fragments

- [[DNS from First Principles/Fragments/rgbdns-frag-c79107e4094a|main]]: lines 4-10
- [[DNS from First Principles/Fragments/rgbdns-frag-813ff75d332f|run]]: lines 11-20

## Full Source

```rust
use rgbdns::{special, wall};
use std::sync::Arc;

fn main() {
    if let Err(error) = run() {
        eprintln!("walldns: fatal: {error}");
        std::process::exit(111);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let ip = std::env::var("IP").unwrap_or_else(|_| "0.0.0.0".into());
    let port = std::env::var("PORT").unwrap_or_else(|_| "53".into());
    let address = rgbdns::socket_address(&ip, &port)?;
    special::serve(
        &address.to_string(),
        Arc::new(|wire: &[u8], limit: usize, _| wall::respond(wire, limit)),
    )?;
    Ok(())
}
```
