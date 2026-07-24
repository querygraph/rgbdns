---
type: "code-file"
source_path: "src/bin/rbldns-data.rs"
language: "rust"
subsystem: "Command-line programs"
crate: "rbldns-data"
line_count: 10
fragment_count: 1
rgbdns_commit: "472c2087"
---

# src/bin/rbldns-data.rs

- Subsystem: [[DNS from First Principles/Subsystems/Command-line programs|Command-line programs]]
- Component: [[DNS from First Principles/Components/rbldns-data|rbldns-data]]
- Source path: `src/bin/rbldns-data.rs`
- Lines: 10
- Summary: Source file in the Command-line programs subsystem.

## Extracted Fragments

- [[DNS from First Principles/Fragments/rgbdns-frag-d51679605713|main]]: lines 3-10

## Full Source

```rust
use rgbdns::rbl::{self, Database};

fn main() {
    let result =
        Database::from_file("data").and_then(|database| rbl::compile(&database, "data.cdb"));
    if let Err(error) = result {
        eprintln!("rbldns-data: fatal: {error}");
        std::process::exit(111);
    }
}
```
