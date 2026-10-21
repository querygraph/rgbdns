---
type: "code-file"
source_path: "src/bin/rgbsec-ds.rs"
language: "rust"
subsystem: "Command-line programs"
crate: "rgbsec-ds"
line_count: 20
fragment_count: 1
rgbdns_commit: "5ae24772"
---

# src/bin/rgbsec-ds.rs

- Subsystem: [[DNS from First Principles/Subsystems/Command-line programs|Command-line programs]]
- Component: [[DNS from First Principles/Components/rgbsec-ds|rgbsec-ds]]
- Source path: `src/bin/rgbsec-ds.rs`
- Lines: 20
- Summary: Source file in the Command-line programs subsystem.

## Extracted Fragments

- [[DNS from First Principles/Fragments/rgbdns-frag-9d663b8530e3|main]]: lines 4-20

## Full Source

```rust
use rgbdns::dnssec;
use std::process::ExitCode;

fn main() -> ExitCode {
    let arguments = std::env::args().skip(1).collect::<Vec<_>>();
    if arguments.len() != 1 {
        eprintln!("usage: rgbsec-ds dnssec-line");
        return ExitCode::from(100);
    }
    match dnssec::Policy::parse(&arguments[0]).and_then(|policy| dnssec::ds_line(&policy)) {
        Ok(line) => {
            println!("{line}");
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("rgbsec-ds: {error}");
            ExitCode::from(111)
        }
    }
}
```
