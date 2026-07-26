---
type: "code-file"
source_path: "src/bin/rbldns.rs"
language: "rust"
subsystem: "Command-line programs"
crate: "rbldns"
line_count: 24
fragment_count: 2
rgbdns_commit: "79502939"
---

# src/bin/rbldns.rs

- Subsystem: [[DNS from First Principles/Subsystems/Command-line programs|Command-line programs]]
- Component: [[DNS from First Principles/Components/rbldns|rbldns]]
- Source path: `src/bin/rbldns.rs`
- Lines: 24
- Summary: Source file in the Command-line programs subsystem.

## Extracted Fragments

- [[DNS from First Principles/Fragments/rgbdns-frag-dc4fe32939c7|main]]: lines 4-10
- [[DNS from First Principles/Fragments/rgbdns-frag-0151d5c4fe7b|run]]: lines 11-24

## Full Source

```rust
use rgbdns::{Name, rbl::Database, special};
use std::sync::Arc;

fn main() {
    if let Err(error) = run() {
        eprintln!("rbldns: fatal: {error}");
        std::process::exit(111);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let data = std::env::var("DATA").unwrap_or_else(|_| "data.cdb".into());
    let base: Name = std::env::var("BASE")
        .map_err(|_| "BASE is required")?
        .parse()?;
    let database = Arc::new(Database::from_file(data)?);
    let handler =
        Arc::new(move |wire: &[u8], limit: usize, _| database.respond(&base, wire, limit));
    let ip = std::env::var("IP").unwrap_or_else(|_| "0.0.0.0".into());
    let port = std::env::var("PORT").unwrap_or_else(|_| "53".into());
    let address = rgbdns::socket_address(&ip, &port)?;
    special::serve(&address.to_string(), handler)?;
    Ok(())
}
```
