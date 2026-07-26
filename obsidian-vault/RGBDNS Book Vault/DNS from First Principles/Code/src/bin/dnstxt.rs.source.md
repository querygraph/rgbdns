---
type: "code-file"
source_path: "src/bin/dnstxt.rs"
language: "rust"
subsystem: "Command-line programs"
crate: "dnstxt"
line_count: 28
fragment_count: 2
rgbdns_commit: "79502939"
---

# src/bin/dnstxt.rs

- Subsystem: [[DNS from First Principles/Subsystems/Command-line programs|Command-line programs]]
- Component: [[DNS from First Principles/Components/dnstxt|dnstxt]]
- Source path: `src/bin/dnstxt.rs`
- Lines: 28
- Summary: Source file in the Command-line programs subsystem.

## Extracted Fragments

- [[DNS from First Principles/Fragments/rgbdns-frag-d0264366d4be|main]]: lines 4-10
- [[DNS from First Principles/Fragments/rgbdns-frag-e3d3acd5d11e|run]]: lines 11-28

## Full Source

```rust
use rgbdns::{RData, RecordType, client};
use std::io::Write;

fn main() {
    if let Err(error) = run() {
        eprintln!("dnstxt: fatal: {error}");
        std::process::exit(111);
    }
}

fn run() -> rgbdns::Result<()> {
    let mut stdout = std::io::stdout().lock();
    for argument in std::env::args().skip(1) {
        let response = client::recursive(argument.parse()?, RecordType::Txt)?;
        for chunk in response
            .answers
            .iter()
            .flat_map(|record| match &record.data {
                RData::Txt(chunks) => chunks.as_slice(),
                _ => &[],
            })
        {
            stdout.write_all(chunk)?;
        }
        stdout.write_all(b"\n")?;
    }
    Ok(())
}
```
