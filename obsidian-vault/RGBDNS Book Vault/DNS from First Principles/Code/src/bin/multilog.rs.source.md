---
type: "code-file"
source_path: "src/bin/multilog.rs"
language: "rust"
subsystem: "Command-line programs"
crate: "multilog"
line_count: 16
fragment_count: 2
rgbdns_commit: "472c2087"
---

# src/bin/multilog.rs

- Subsystem: [[DNS from First Principles/Subsystems/Command-line programs|Command-line programs]]
- Component: [[DNS from First Principles/Components/multilog|multilog]]
- Source path: `src/bin/multilog.rs`
- Lines: 16
- Summary: Source file in the Command-line programs subsystem.

## Extracted Fragments

- [[DNS from First Principles/Fragments/rgbdns-frag-db93967bb299|main]]: lines 4-10
- [[DNS from First Principles/Fragments/rgbdns-frag-631de8a93e74|run]]: lines 11-16

## Full Source

```rust
use rgbdns::multilog::{self, Config};
use std::io::BufReader;

fn main() {
    if let Err(error) = run() {
        eprintln!("multilog: fatal: {error}");
        std::process::exit(111);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let arguments = std::env::args().skip(1).collect::<Vec<_>>();
    let config = Config::parse(&arguments)?;
    multilog::run(&config, BufReader::new(std::io::stdin()))?;
    Ok(())
}
```
