---
type: "code-file"
source_path: "src/bin/axfr-get.rs"
language: "rust"
subsystem: "Command-line programs"
crate: "axfr-get"
line_count: 37
fragment_count: 2
rgbdns_commit: "472c2087"
---

# src/bin/axfr-get.rs

- Subsystem: [[DNS from First Principles/Subsystems/Command-line programs|Command-line programs]]
- Component: [[DNS from First Principles/Components/axfr-get|axfr-get]]
- Source path: `src/bin/axfr-get.rs`
- Lines: 37
- Summary: Source file in the Command-line programs subsystem.

## Extracted Fragments

- [[DNS from First Principles/Fragments/rgbdns-frag-8b25c0eefcbc|main]]: lines 4-10
- [[DNS from First Principles/Fragments/rgbdns-frag-6df4a30c4d1f|run]]: lines 11-37

## Full Source

```rust
use rgbdns::{Name, axfr};
use std::{net::SocketAddr, path::Path};

fn main() {
    if let Err(error) = run() {
        eprintln!("axfr-get: fatal: {error}");
        std::process::exit(111);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut arguments = std::env::args().skip(1);
    let zone: Name = arguments
        .next()
        .ok_or("usage: axfr-get zone server[:port] output temporary")?
        .parse()?;
    let server_text = arguments
        .next()
        .ok_or("usage: axfr-get zone server[:port] output temporary")?;
    let server: SocketAddr = if server_text.contains(':') {
        server_text.parse()?
    } else {
        format!("{server_text}:53").parse()?
    };
    let output = arguments
        .next()
        .ok_or("usage: axfr-get zone server[:port] output temporary")?;
    let temporary = arguments
        .next()
        .ok_or("usage: axfr-get zone server[:port] output temporary")?;
    if arguments.next().is_some() {
        return Err("usage: axfr-get zone server[:port] output temporary".into());
    }
    let records = axfr::fetch(server, zone)?;
    axfr::write_tinydns(&records, Path::new(&output), Path::new(&temporary))?;
    Ok(())
}
```
