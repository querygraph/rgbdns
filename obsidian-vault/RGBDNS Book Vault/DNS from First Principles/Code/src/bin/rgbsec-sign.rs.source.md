---
type: "code-file"
source_path: "src/bin/rgbsec-sign.rs"
language: "rust"
subsystem: "Command-line programs"
crate: "rgbsec-sign"
line_count: 19
fragment_count: 1
rgbdns_commit: "5ae24772"
---

# src/bin/rgbsec-sign.rs

- Subsystem: [[DNS from First Principles/Subsystems/Command-line programs|Command-line programs]]
- Component: [[DNS from First Principles/Components/rgbsec-sign|rgbsec-sign]]
- Source path: `src/bin/rgbsec-sign.rs`
- Lines: 19
- Summary: Source file in the Command-line programs subsystem.

## Extracted Fragments

- [[DNS from First Principles/Fragments/rgbdns-frag-175ca828ace4|main]]: lines 4-19

## Full Source

```rust
use rgbdns::dnssec;
use std::{path::Path, process::ExitCode};

fn main() -> ExitCode {
    let arguments = std::env::args().skip(1).collect::<Vec<_>>();
    if arguments.len() > 2 {
        eprintln!("usage: rgbsec-sign [data [data.signed]]");
        return ExitCode::from(100);
    }
    let input = Path::new(arguments.first().map_or("data", String::as_str));
    let output = Path::new(arguments.get(1).map_or("data.signed", String::as_str));
    match dnssec::sign_file(input, Path::new("dnssec"), output) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("rgbsec-sign: {error}");
            ExitCode::from(111)
        }
    }
}
```
