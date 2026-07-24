---
type: "code-fragment"
fragment_id: "rgbdns-frag-f5d401fa9b1e"
source_path: "src/bin/tinydns-data.rs"
code_note: "DNS from First Principles/Code/src/bin/tinydns-data.rs.source"
language: "rust"
subsystem: "Command-line programs"
crate: "tinydns-data"
symbol: "main"
kind: "fn"
start_line: 2
end_line: 8
---

# main

- Fragment ID: `rgbdns-frag-f5d401fa9b1e`
- Source file: [[DNS from First Principles/Code/src/bin/tinydns-data.rs.source|src/bin/tinydns-data.rs]]
- Lines: 2-8
- Subsystem: [[DNS from First Principles/Subsystems/Command-line programs|Command-line programs]]
- Component: [[DNS from First Principles/Components/tinydns-data|tinydns-data]]

```rgbdns-fragment
{"id": "rgbdns-frag-f5d401fa9b1e", "codeNote": "DNS from First Principles/Code/src/bin/tinydns-data.rs.source", "heading": "rgbdns-frag-f5d401fa9b1e: fn main", "sourcePath": "src/bin/tinydns-data.rs", "startLine": 2, "endLine": 8}
```

## Excerpt

<span id="rgbdns-frag-f5d401fa9b1e" class="rgbdns-fragment-target"></span>
### rgbdns-frag-f5d401fa9b1e: fn main

```rust
fn main() {
    let result = Zone::from_file("data").and_then(|zone| cdb::compile(&zone, "data.cdb"));
    if let Err(e) = result {
        eprintln!("tinydns-data: fatal: {e}");
        std::process::exit(111)
    }
}
```
