---
type: "code-fragment"
fragment_id: "rgbdns-frag-5d5f0374b6f5"
source_path: "src/bin/tinydns-get.rs"
code_note: "DNS from First Principles/Code/src/bin/tinydns-get.rs.source"
language: "rust"
subsystem: "Command-line programs"
crate: "tinydns-get"
symbol: "main"
kind: "fn"
start_line: 7
end_line: 13
---

# main

- Fragment ID: `rgbdns-frag-5d5f0374b6f5`
- Source file: [[DNS from First Principles/Code/src/bin/tinydns-get.rs.source|src/bin/tinydns-get.rs]]
- Lines: 7-13
- Subsystem: [[DNS from First Principles/Subsystems/Command-line programs|Command-line programs]]
- Component: [[DNS from First Principles/Components/tinydns-get|tinydns-get]]

```rgbdns-fragment
{"id": "rgbdns-frag-5d5f0374b6f5", "codeNote": "DNS from First Principles/Code/src/bin/tinydns-get.rs.source", "heading": "rgbdns-frag-5d5f0374b6f5: fn main", "sourcePath": "src/bin/tinydns-get.rs", "startLine": 7, "endLine": 13}
```

## Excerpt

<span id="rgbdns-frag-5d5f0374b6f5" class="rgbdns-fragment-target"></span>
### rgbdns-frag-5d5f0374b6f5: fn main

```rust
fn main() {
    if let Err(error) = run() {
        eprintln!("tinydns-get: fatal: {error}");
        std::process::exit(111);
    }
}

```
