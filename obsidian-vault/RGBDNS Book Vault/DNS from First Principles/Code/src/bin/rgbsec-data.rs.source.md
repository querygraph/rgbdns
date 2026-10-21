---
type: "code-file"
source_path: "src/bin/rgbsec-data.rs"
language: "rust"
subsystem: "Command-line programs"
crate: "rgbsec-data"
line_count: 19
fragment_count: 1
rgbdns_commit: "5ae24772"
---

# src/bin/rgbsec-data.rs

- Subsystem: [[DNS from First Principles/Subsystems/Command-line programs|Command-line programs]]
- Component: [[DNS from First Principles/Components/rgbsec-data|rgbsec-data]]
- Source path: `src/bin/rgbsec-data.rs`
- Lines: 19
- Summary: Source file in the Command-line programs subsystem.

## Extracted Fragments

- [[DNS from First Principles/Fragments/rgbdns-frag-5b3e2e25c63a|main]]: lines 4-19

## Full Source

```rust
use rgbdns::dnssec;
use std::{path::Path, process::ExitCode};

fn main() -> ExitCode {
    let arguments = std::env::args().skip(1).collect::<Vec<_>>();
    if arguments.len() > 2 {
        eprintln!("usage: rgbsec-data [data [data.cdb]]");
        return ExitCode::from(100);
    }
    let input = Path::new(arguments.first().map_or("data", String::as_str));
    let output = Path::new(arguments.get(1).map_or("data.cdb", String::as_str));
    match dnssec::compile_file(input, Path::new("dnssec"), output) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("rgbsec-data: {error}");
            ExitCode::from(111)
        }
    }
}
```
