---
type: "code-fragment"
fragment_id: "rgbdns-frag-6688d251e48c"
source_path: "src/bin/tinydns-edit.rs"
code_note: "DNS from First Principles/Code/src/bin/tinydns-edit.rs.source"
language: "rust"
subsystem: "Command-line programs"
crate: "tinydns-edit"
symbol: "main"
kind: "fn"
start_line: 7
end_line: 13
---

# main

- Fragment ID: `rgbdns-frag-6688d251e48c`
- Source file: [[DNS from First Principles/Code/src/bin/tinydns-edit.rs.source|src/bin/tinydns-edit.rs]]
- Lines: 7-13
- Subsystem: [[DNS from First Principles/Subsystems/Command-line programs|Command-line programs]]
- Component: [[DNS from First Principles/Components/tinydns-edit|tinydns-edit]]

```rgbdns-fragment
{"id": "rgbdns-frag-6688d251e48c", "codeNote": "DNS from First Principles/Code/src/bin/tinydns-edit.rs.source", "heading": "rgbdns-frag-6688d251e48c: fn main", "sourcePath": "src/bin/tinydns-edit.rs", "startLine": 7, "endLine": 13}
```

## Excerpt

<span id="rgbdns-frag-6688d251e48c" class="rgbdns-fragment-target"></span>
### rgbdns-frag-6688d251e48c: fn main

```rust
fn main() {
    if let Err(error) = run() {
        eprintln!("tinydns-edit: fatal: {error}");
        std::process::exit(111);
    }
}

```
