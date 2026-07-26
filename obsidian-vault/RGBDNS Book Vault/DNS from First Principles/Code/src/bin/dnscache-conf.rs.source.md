---
type: "code-file"
source_path: "src/bin/dnscache-conf.rs"
language: "rust"
subsystem: "Command-line programs"
crate: "dnscache-conf"
line_count: 7
fragment_count: 1
rgbdns_commit: "79502939"
---

# src/bin/dnscache-conf.rs

- Subsystem: [[DNS from First Principles/Subsystems/Command-line programs|Command-line programs]]
- Component: [[DNS from First Principles/Components/dnscache-conf|dnscache-conf]]
- Source path: `src/bin/dnscache-conf.rs`
- Lines: 7
- Summary: Source file in the Command-line programs subsystem.

## Extracted Fragments

- [[DNS from First Principles/Fragments/rgbdns-frag-49d5690802aa|main]]: lines 1-7

## Full Source

```rust
fn main() {
    let arguments = std::env::args().skip(1).collect::<Vec<_>>();
    if let Err(error) = rgbdns::conf::configure(rgbdns::conf::Service::Dnscache, &arguments) {
        eprintln!("dnscache-conf: fatal: {error}");
        std::process::exit(111);
    }
}
```
