---
type: "code-fragment"
fragment_id: "rgbdns-frag-a0a2fb666be5"
source_path: "src/bin/pickdns-data.rs"
code_note: "DNS from First Principles/Code/src/bin/pickdns-data.rs.source"
language: "rust"
subsystem: "Command-line programs"
crate: "pickdns-data"
symbol: "main"
kind: "fn"
start_line: 3
end_line: 10
---

# main

- Fragment ID: `rgbdns-frag-a0a2fb666be5`
- Source file: [[DNS from First Principles/Code/src/bin/pickdns-data.rs.source|src/bin/pickdns-data.rs]]
- Lines: 3-10
- Subsystem: [[DNS from First Principles/Subsystems/Command-line programs|Command-line programs]]
- Component: [[DNS from First Principles/Components/pickdns-data|pickdns-data]]

```rgbdns-fragment
{"id": "rgbdns-frag-a0a2fb666be5", "codeNote": "DNS from First Principles/Code/src/bin/pickdns-data.rs.source", "heading": "rgbdns-frag-a0a2fb666be5: fn main", "sourcePath": "src/bin/pickdns-data.rs", "startLine": 3, "endLine": 10}
```

## Excerpt

<span id="rgbdns-frag-a0a2fb666be5" class="rgbdns-fragment-target"></span>
### rgbdns-frag-a0a2fb666be5: fn main

```rust
fn main() {
    let result =
        Database::from_file("data").and_then(|database| pick::compile(&database, "data.cdb"));
    if let Err(error) = result {
        eprintln!("pickdns-data: fatal: {error}");
        std::process::exit(111);
    }
}
```
