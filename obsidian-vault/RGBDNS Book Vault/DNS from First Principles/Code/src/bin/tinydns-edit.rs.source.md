---
type: "code-file"
source_path: "src/bin/tinydns-edit.rs"
language: "rust"
subsystem: "Command-line programs"
crate: "tinydns-edit"
line_count: 33
fragment_count: 2
rgbdns_commit: "79502939"
---

# src/bin/tinydns-edit.rs

- Subsystem: [[DNS from First Principles/Subsystems/Command-line programs|Command-line programs]]
- Component: [[DNS from First Principles/Components/tinydns-edit|tinydns-edit]]
- Source path: `src/bin/tinydns-edit.rs`
- Lines: 33
- Summary: Source file in the Command-line programs subsystem.

## Extracted Fragments

- [[DNS from First Principles/Fragments/rgbdns-frag-6688d251e48c|main]]: lines 7-13
- [[DNS from First Principles/Fragments/rgbdns-frag-ae456975540f|run]]: lines 14-33

## Full Source

```rust
use rgbdns::{Name, tinydns_edit};
use std::{
    net::{Ipv4Addr, Ipv6Addr},
    path::Path,
};

fn main() {
    if let Err(error) = run() {
        eprintln!("tinydns-edit: fatal: {error}");
        std::process::exit(111);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let arguments = std::env::args().skip(1).collect::<Vec<_>>();
    if arguments.len() != 6 || arguments[2] != "add" {
        return Err("usage: tinydns-edit data data.new add \
             [ns|childns|host|alias|mx|host6|alias6] domain address"
            .into());
    }
    tinydns_edit::add(
        Path::new(&arguments[0]),
        Path::new(&arguments[1]),
        tinydns_edit::Mode::parse(&arguments[3])?,
        arguments[4].parse::<Name>()?,
        if matches!(arguments[3].as_str(), "host6" | "alias6") {
            tinydns_edit::Address::V6(arguments[5].parse::<Ipv6Addr>()?)
        } else {
            tinydns_edit::Address::V4(arguments[5].parse::<Ipv4Addr>()?)
        },
    )?;
    Ok(())
}
```
