---
type: "code-file"
source_path: "src/bin/rgbsec-check.rs"
language: "rust"
subsystem: "Command-line programs"
crate: "rgbsec-check"
line_count: 24
fragment_count: 1
rgbdns_commit: "5ae24772"
---

# src/bin/rgbsec-check.rs

- Subsystem: [[DNS from First Principles/Subsystems/Command-line programs|Command-line programs]]
- Component: [[DNS from First Principles/Components/rgbsec-check|rgbsec-check]]
- Source path: `src/bin/rgbsec-check.rs`
- Lines: 24
- Summary: Source file in the Command-line programs subsystem.

## Extracted Fragments

- [[DNS from First Principles/Fragments/rgbdns-frag-920b1792717f|main]]: lines 4-24

## Full Source

```rust
use rgbdns::dnssec;
use std::{path::Path, process::ExitCode};

fn main() -> ExitCode {
    let arguments = std::env::args().skip(1).collect::<Vec<_>>();
    if arguments.len() > 2 {
        eprintln!("usage: rgbsec-check [data.signed [dnssec]]");
        return ExitCode::from(100);
    }
    let input = Path::new(arguments.first().map_or("data.signed", String::as_str));
    let policy = Path::new(arguments.get(1).map_or("dnssec", String::as_str));
    match dnssec::check_file(input, policy) {
        Ok(statuses) => {
            for status in statuses {
                println!("{status}");
            }
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("rgbsec-check: {error}");
            ExitCode::from(111)
        }
    }
}
```
