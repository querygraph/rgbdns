---
type: "code-file"
source_path: "src/bin/aname-materialize.rs"
language: "rust"
subsystem: "Command-line programs"
crate: "aname-materialize"
line_count: 31
fragment_count: 1
rgbdns_commit: "5ae24772"
---

# src/bin/aname-materialize.rs

- Subsystem: [[DNS from First Principles/Subsystems/Command-line programs|Command-line programs]]
- Component: [[DNS from First Principles/Components/aname-materialize|aname-materialize]]
- Source path: `src/bin/aname-materialize.rs`
- Lines: 31
- Summary: Source file in the Command-line programs subsystem.

## Extracted Fragments

- [[DNS from First Principles/Fragments/rgbdns-frag-330c4abefa00|main]]: lines 4-31

## Full Source

```rust
use rgbdns::dnssec;
use std::{path::Path, process::ExitCode};

fn main() -> ExitCode {
    let arguments = std::env::args().skip(1).collect::<Vec<_>>();
    if arguments.len() > 3 {
        eprintln!("usage: aname-materialize [data [data.materialized [dnssec]]]");
        return ExitCode::from(100);
    }
    let input = Path::new(arguments.first().map_or("data", String::as_str));
    let output = Path::new(arguments.get(1).map_or("data.materialized", String::as_str));
    let result = if let Some(policy) = arguments.get(2) {
        dnssec::materialize_policy_file(input, Path::new(policy), output)
    } else {
        dnssec::materialize_file(input, output)
    };
    match result {
        Ok(Some(expires)) => {
            println!("{expires}\tok");
            ExitCode::SUCCESS
        }
        Ok(None) => {
            println!("-\tok");
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("aname-materialize: {error}");
            ExitCode::from(111)
        }
    }
}
```
