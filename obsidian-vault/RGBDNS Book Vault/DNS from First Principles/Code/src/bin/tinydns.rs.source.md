---
type: "code-file"
source_path: "src/bin/tinydns.rs"
language: "rust"
subsystem: "Command-line programs"
crate: "tinydns"
line_count: 18
fragment_count: 1
rgbdns_commit: "472c2087"
---

# src/bin/tinydns.rs

- Subsystem: [[DNS from First Principles/Subsystems/Command-line programs|Command-line programs]]
- Component: [[DNS from First Principles/Components/tinydns|tinydns]]
- Source path: `src/bin/tinydns.rs`
- Lines: 18
- Summary: Source file in the Command-line programs subsystem.

## Extracted Fragments

- [[DNS from First Principles/Fragments/rgbdns-frag-7629df224c60|main]]: lines 2-18

## Full Source

```rust
use rgbdns::{server, zone::Zone};
fn main() {
    let data = std::env::var("DATA").unwrap_or_else(|_| "data.cdb".into());
    let ip = std::env::var("IP").unwrap_or_else(|_| "0.0.0.0".into());
    let port = std::env::var("PORT").unwrap_or_else(|_| "53".into());
    let address = rgbdns::socket_address(&ip, &port).unwrap_or_else(|e| {
        eprintln!("tinydns: fatal: {e}");
        std::process::exit(111)
    });
    let z = Zone::from_file(data).unwrap_or_else(|e| {
        eprintln!("tinydns: fatal: {e}");
        std::process::exit(111)
    });
    if let Err(e) = server::serve(z, &address.to_string()) {
        eprintln!("tinydns: fatal: {e}");
        std::process::exit(111)
    }
}
```
