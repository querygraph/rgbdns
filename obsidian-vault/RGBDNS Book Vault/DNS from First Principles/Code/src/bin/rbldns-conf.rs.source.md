---
type: "code-file"
source_path: "src/bin/rbldns-conf.rs"
language: "rust"
subsystem: "Command-line programs"
crate: "rbldns-conf"
line_count: 7
fragment_count: 1
rgbdns_commit: "472c2087"
---

# src/bin/rbldns-conf.rs

- Subsystem: [[DNS from First Principles/Subsystems/Command-line programs|Command-line programs]]
- Component: [[DNS from First Principles/Components/rbldns-conf|rbldns-conf]]
- Source path: `src/bin/rbldns-conf.rs`
- Lines: 7
- Summary: Source file in the Command-line programs subsystem.

## Extracted Fragments

- [[DNS from First Principles/Fragments/rgbdns-frag-8ebfeb463f74|main]]: lines 1-7

## Full Source

```rust
fn main() {
    let arguments = std::env::args().skip(1).collect::<Vec<_>>();
    if let Err(error) = rgbdns::conf::configure(rgbdns::conf::Service::Rbldns, &arguments) {
        eprintln!("rbldns-conf: fatal: {error}");
        std::process::exit(111);
    }
}
```
