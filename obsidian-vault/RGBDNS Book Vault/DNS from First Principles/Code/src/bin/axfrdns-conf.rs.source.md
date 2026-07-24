---
type: "code-file"
source_path: "src/bin/axfrdns-conf.rs"
language: "rust"
subsystem: "Command-line programs"
crate: "axfrdns-conf"
line_count: 7
fragment_count: 1
rgbdns_commit: "472c2087"
---

# src/bin/axfrdns-conf.rs

- Subsystem: [[DNS from First Principles/Subsystems/Command-line programs|Command-line programs]]
- Component: [[DNS from First Principles/Components/axfrdns-conf|axfrdns-conf]]
- Source path: `src/bin/axfrdns-conf.rs`
- Lines: 7
- Summary: Source file in the Command-line programs subsystem.

## Extracted Fragments

- [[DNS from First Principles/Fragments/rgbdns-frag-5cd78cd0c01f|main]]: lines 1-7

## Full Source

```rust
fn main() {
    let arguments = std::env::args().skip(1).collect::<Vec<_>>();
    if let Err(error) = rgbdns::conf::configure(rgbdns::conf::Service::Axfrdns, &arguments) {
        eprintln!("axfrdns-conf: fatal: {error}");
        std::process::exit(111);
    }
}
```
