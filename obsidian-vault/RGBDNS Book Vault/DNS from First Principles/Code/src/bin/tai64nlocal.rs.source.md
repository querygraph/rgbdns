---
type: "code-file"
source_path: "src/bin/tai64nlocal.rs"
language: "rust"
subsystem: "Command-line programs"
crate: "tai64nlocal"
line_count: 13
fragment_count: 1
rgbdns_commit: "472c2087"
---

# src/bin/tai64nlocal.rs

- Subsystem: [[DNS from First Principles/Subsystems/Command-line programs|Command-line programs]]
- Component: [[DNS from First Principles/Components/tai64nlocal|tai64nlocal]]
- Source path: `src/bin/tai64nlocal.rs`
- Lines: 13
- Summary: Source file in the Command-line programs subsystem.

## Extracted Fragments

- [[DNS from First Principles/Fragments/rgbdns-frag-565a1c08ae2b|main]]: lines 3-13

## Full Source

```rust
use std::io::BufReader;

fn main() {
    if std::env::args_os().len() != 1
        || rgbdns::tai64::localize(
            BufReader::new(std::io::stdin().lock()),
            std::io::stdout().lock(),
        )
        .is_err()
    {
        std::process::exit(111);
    }
}
```
