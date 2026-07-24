---
type: "code-file"
source_path: "src/bin/tinydns-data.rs"
language: "rust"
subsystem: "Command-line programs"
crate: "tinydns-data"
line_count: 8
fragment_count: 1
rgbdns_commit: "472c2087"
---

# src/bin/tinydns-data.rs

- Subsystem: [[DNS from First Principles/Subsystems/Command-line programs|Command-line programs]]
- Component: [[DNS from First Principles/Components/tinydns-data|tinydns-data]]
- Source path: `src/bin/tinydns-data.rs`
- Lines: 8
- Summary: Source file in the Command-line programs subsystem.

## Extracted Fragments

- [[DNS from First Principles/Fragments/rgbdns-frag-f5d401fa9b1e|main]]: lines 2-8

## Full Source

```rust
use rgbdns::{cdb, zone::Zone};
fn main() {
    let result = Zone::from_file("data").and_then(|zone| cdb::compile(&zone, "data.cdb"));
    if let Err(e) = result {
        eprintln!("tinydns-data: fatal: {e}");
        std::process::exit(111)
    }
}
```
