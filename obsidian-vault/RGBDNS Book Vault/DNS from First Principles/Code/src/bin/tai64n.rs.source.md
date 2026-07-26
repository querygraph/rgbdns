---
type: "code-file"
source_path: "src/bin/tai64n.rs"
language: "rust"
subsystem: "Command-line programs"
crate: "tai64n"
line_count: 13
fragment_count: 1
rgbdns_commit: "79502939"
---

# src/bin/tai64n.rs

- Subsystem: [[DNS from First Principles/Subsystems/Command-line programs|Command-line programs]]
- Component: [[DNS from First Principles/Components/tai64n|tai64n]]
- Source path: `src/bin/tai64n.rs`
- Lines: 13
- Summary: Source file in the Command-line programs subsystem.

## Extracted Fragments

- [[DNS from First Principles/Fragments/rgbdns-frag-36f806c0b8fd|main]]: lines 3-13

## Full Source

```rust
use std::io::BufReader;

fn main() {
    if std::env::args_os().len() != 1
        || rgbdns::tai64::stamp(
            BufReader::new(std::io::stdin().lock()),
            std::io::stdout().lock(),
        )
        .is_err()
    {
        std::process::exit(111);
    }
}
```
