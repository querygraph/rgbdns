---
type: "code-file"
source_path: "src/bin/pickdns-data.rs"
language: "rust"
subsystem: "Command-line programs"
crate: "pickdns-data"
line_count: 10
fragment_count: 1
rgbdns_commit: "472c2087"
---

# src/bin/pickdns-data.rs

- Subsystem: [[DNS from First Principles/Subsystems/Command-line programs|Command-line programs]]
- Component: [[DNS from First Principles/Components/pickdns-data|pickdns-data]]
- Source path: `src/bin/pickdns-data.rs`
- Lines: 10
- Summary: Source file in the Command-line programs subsystem.

## Extracted Fragments

- [[DNS from First Principles/Fragments/rgbdns-frag-a0a2fb666be5|main]]: lines 3-10

## Full Source

```rust
use rgbdns::pick::{self, Database};

fn main() {
    let result =
        Database::from_file("data").and_then(|database| pick::compile(&database, "data.cdb"));
    if let Err(error) = result {
        eprintln!("pickdns-data: fatal: {error}");
        std::process::exit(111);
    }
}
```
