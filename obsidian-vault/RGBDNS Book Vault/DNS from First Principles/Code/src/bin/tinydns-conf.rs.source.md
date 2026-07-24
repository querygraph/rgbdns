---
type: "code-file"
source_path: "src/bin/tinydns-conf.rs"
language: "rust"
subsystem: "Command-line programs"
crate: "tinydns-conf"
line_count: 7
fragment_count: 1
rgbdns_commit: "472c2087"
---

# src/bin/tinydns-conf.rs

- Subsystem: [[DNS from First Principles/Subsystems/Command-line programs|Command-line programs]]
- Component: [[DNS from First Principles/Components/tinydns-conf|tinydns-conf]]
- Source path: `src/bin/tinydns-conf.rs`
- Lines: 7
- Summary: Source file in the Command-line programs subsystem.

## Extracted Fragments

- [[DNS from First Principles/Fragments/rgbdns-frag-dd026b16b5bd|main]]: lines 1-7

## Full Source

```rust
fn main() {
    let arguments = std::env::args().skip(1).collect::<Vec<_>>();
    if let Err(error) = rgbdns::conf::configure(rgbdns::conf::Service::Tinydns, &arguments) {
        eprintln!("tinydns-conf: fatal: {error}");
        std::process::exit(111);
    }
}
```
