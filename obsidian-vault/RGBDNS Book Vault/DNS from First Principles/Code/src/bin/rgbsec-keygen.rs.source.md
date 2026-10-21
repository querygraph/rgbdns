---
type: "code-file"
source_path: "src/bin/rgbsec-keygen.rs"
language: "rust"
subsystem: "Command-line programs"
crate: "rgbsec-keygen"
line_count: 26
fragment_count: 1
rgbdns_commit: "5ae24772"
---

# src/bin/rgbsec-keygen.rs

- Subsystem: [[DNS from First Principles/Subsystems/Command-line programs|Command-line programs]]
- Component: [[DNS from First Principles/Components/rgbsec-keygen|rgbsec-keygen]]
- Source path: `src/bin/rgbsec-keygen.rs`
- Lines: 26
- Summary: Source file in the Command-line programs subsystem.

## Extracted Fragments

- [[DNS from First Principles/Fragments/rgbdns-frag-71796072aaca|main]]: lines 4-26

## Full Source

```rust
use rgbdns::{Name, dnssec};
use std::{path::Path, process::ExitCode};

fn main() -> ExitCode {
    let arguments = std::env::args().skip(1).collect::<Vec<_>>();
    if arguments.len() != 2 {
        eprintln!("usage: rgbsec-keygen zone keyfile");
        return ExitCode::from(100);
    }
    let result = arguments[0]
        .parse::<Name>()
        .map_err(|error| error.to_string())
        .and_then(|zone| {
            dnssec::generate_key(&zone, Path::new(&arguments[1])).map_err(|error| error.to_string())
        });
    match result {
        Ok(policy) => {
            println!("{}", policy.line());
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("rgbsec-keygen: {error}");
            ExitCode::from(111)
        }
    }
}
```
