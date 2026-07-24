---
type: "code-fragment"
fragment_id: "rgbdns-frag-db93967bb299"
source_path: "src/bin/multilog.rs"
code_note: "DNS from First Principles/Code/src/bin/multilog.rs.source"
language: "rust"
subsystem: "Command-line programs"
crate: "multilog"
symbol: "main"
kind: "fn"
start_line: 4
end_line: 10
---

# main

- Fragment ID: `rgbdns-frag-db93967bb299`
- Source file: [[DNS from First Principles/Code/src/bin/multilog.rs.source|src/bin/multilog.rs]]
- Lines: 4-10
- Subsystem: [[DNS from First Principles/Subsystems/Command-line programs|Command-line programs]]
- Component: [[DNS from First Principles/Components/multilog|multilog]]

```rgbdns-fragment
{"id": "rgbdns-frag-db93967bb299", "codeNote": "DNS from First Principles/Code/src/bin/multilog.rs.source", "heading": "rgbdns-frag-db93967bb299: fn main", "sourcePath": "src/bin/multilog.rs", "startLine": 4, "endLine": 10}
```

## Excerpt

<span id="rgbdns-frag-db93967bb299" class="rgbdns-fragment-target"></span>
### rgbdns-frag-db93967bb299: fn main

```rust
fn main() {
    if let Err(error) = run() {
        eprintln!("multilog: fatal: {error}");
        std::process::exit(111);
    }
}

```
