---
type: "code-file"
source_path: "src/bin/axfrdns.rs"
language: "rust"
subsystem: "Command-line programs"
crate: "axfrdns"
line_count: 25
fragment_count: 2
rgbdns_commit: "472c2087"
---

# src/bin/axfrdns.rs

- Subsystem: [[DNS from First Principles/Subsystems/Command-line programs|Command-line programs]]
- Component: [[DNS from First Principles/Components/axfrdns|axfrdns]]
- Source path: `src/bin/axfrdns.rs`
- Lines: 25
- Summary: Source file in the Command-line programs subsystem.

## Extracted Fragments

- [[DNS from First Principles/Fragments/rgbdns-frag-5a5804e0aa41|main]]: lines 4-10
- [[DNS from First Principles/Fragments/rgbdns-frag-24f0c6bf421c|run]]: lines 11-25

## Full Source

```rust
use ipnet::IpNet;
use rgbdns::{axfr, zone::Zone};

fn main() {
    if let Err(error) = run() {
        eprintln!("axfrdns: fatal: {error}");
        std::process::exit(111);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let data = std::env::var("DATA").unwrap_or_else(|_| "data.cdb".into());
    let ip = std::env::var("IP").unwrap_or_else(|_| "127.0.0.1".into());
    let port = std::env::var("PORT").unwrap_or_else(|_| "53".into());
    let address = rgbdns::socket_address(&ip, &port)?;
    let allowed = std::env::var("ALLOW_NETS")
        .unwrap_or_else(|_| "127.0.0.0/8,::1/128".into())
        .split(',')
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::parse::<IpNet>)
        .collect::<Result<Vec<_>, _>>()?;
    axfr::serve(Zone::from_file(data)?, &address.to_string(), allowed)?;
    Ok(())
}
```
