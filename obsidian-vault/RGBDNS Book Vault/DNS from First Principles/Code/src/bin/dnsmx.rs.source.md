---
type: "code-file"
source_path: "src/bin/dnsmx.rs"
language: "rust"
subsystem: "Command-line programs"
crate: "dnsmx"
line_count: 32
fragment_count: 2
rgbdns_commit: "79502939"
---

# src/bin/dnsmx.rs

- Subsystem: [[DNS from First Principles/Subsystems/Command-line programs|Command-line programs]]
- Component: [[DNS from First Principles/Components/dnsmx|dnsmx]]
- Source path: `src/bin/dnsmx.rs`
- Lines: 32
- Summary: Source file in the Command-line programs subsystem.

## Extracted Fragments

- [[DNS from First Principles/Fragments/rgbdns-frag-31aba88be59b|main]]: lines 3-9
- [[DNS from First Principles/Fragments/rgbdns-frag-68fc31fecca6|run]]: lines 10-32

## Full Source

```rust
use rgbdns::{RData, RecordType, client};

fn main() {
    if let Err(error) = run() {
        eprintln!("dnsmx: fatal: {error}");
        std::process::exit(111);
    }
}

fn run() -> rgbdns::Result<()> {
    for argument in std::env::args().skip(1) {
        let name = argument.parse()?;
        let response = client::recursive(name, RecordType::Mx)?;
        let mut found = false;
        for (preference, target) in
            response
                .answers
                .iter()
                .filter_map(|record| match &record.data {
                    RData::Mx(preference, target) => Some((preference, target)),
                    _ => None,
                })
        {
            println!("{preference} {target}");
            found = true;
        }
        if !found {
            println!("0 {argument}.");
        }
    }
    Ok(())
}
```
