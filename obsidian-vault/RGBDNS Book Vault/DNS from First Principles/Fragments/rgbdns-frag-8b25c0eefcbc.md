---
type: "code-fragment"
fragment_id: "rgbdns-frag-8b25c0eefcbc"
source_path: "src/bin/axfr-get.rs"
code_note: "DNS from First Principles/Code/src/bin/axfr-get.rs.source"
language: "rust"
subsystem: "Command-line programs"
crate: "axfr-get"
symbol: "main"
kind: "fn"
start_line: 4
end_line: 10
---

# main

- Fragment ID: `rgbdns-frag-8b25c0eefcbc`
- Source file: [[DNS from First Principles/Code/src/bin/axfr-get.rs.source|src/bin/axfr-get.rs]]
- Lines: 4-10
- Subsystem: [[DNS from First Principles/Subsystems/Command-line programs|Command-line programs]]
- Component: [[DNS from First Principles/Components/axfr-get|axfr-get]]

```rgbdns-fragment
{"id": "rgbdns-frag-8b25c0eefcbc", "codeNote": "DNS from First Principles/Code/src/bin/axfr-get.rs.source", "heading": "rgbdns-frag-8b25c0eefcbc: fn main", "sourcePath": "src/bin/axfr-get.rs", "startLine": 4, "endLine": 10}
```

## Excerpt

<span id="rgbdns-frag-8b25c0eefcbc" class="rgbdns-fragment-target"></span>
### rgbdns-frag-8b25c0eefcbc: fn main

```rust
fn main() {
    if let Err(error) = run() {
        eprintln!("axfr-get: fatal: {error}");
        std::process::exit(111);
    }
}

```
