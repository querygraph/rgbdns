---
type: "code-file"
source_path: "src/bin/acme-materialize.rs"
language: "rust"
subsystem: "Command-line programs"
crate: "acme-materialize"
line_count: 22
fragment_count: 1
rgbdns_commit: "5ae24772"
---

# src/bin/acme-materialize.rs

- Subsystem: [[DNS from First Principles/Subsystems/Command-line programs|Command-line programs]]
- Component: [[DNS from First Principles/Components/acme-materialize|acme-materialize]]
- Source path: `src/bin/acme-materialize.rs`
- Lines: 22
- Summary: Source file in the Command-line programs subsystem.

## Extracted Fragments

- [[DNS from First Principles/Fragments/rgbdns-frag-3ec05933a0fe|main]]: lines 4-22

## Full Source

```rust
use rgbdns::acme;
use std::{path::Path, process::ExitCode};

fn main() -> ExitCode {
    let arguments = std::env::args().skip(1).collect::<Vec<_>>();
    if arguments.len() != 4 {
        eprintln!("usage: acme-materialize data acme-config state-directory output");
        return ExitCode::from(100);
    }
    match acme::materialize_state(
        Path::new(&arguments[0]),
        Path::new(&arguments[1]),
        Path::new(&arguments[2]),
        Path::new(&arguments[3]),
    ) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("acme-materialize: {error}");
            ExitCode::from(111)
        }
    }
}
```
