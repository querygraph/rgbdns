---
type: "code-fragment"
fragment_id: "rgbdns-frag-43f5784a63d0"
source_path: "src/bin/pickdns.rs"
code_note: "DNS from First Principles/Code/src/bin/pickdns.rs.source"
language: "rust"
subsystem: "Command-line programs"
crate: "pickdns"
symbol: "main"
kind: "fn"
start_line: 4
end_line: 10
---

# main

- Fragment ID: `rgbdns-frag-43f5784a63d0`
- Source file: [[DNS from First Principles/Code/src/bin/pickdns.rs.source|src/bin/pickdns.rs]]
- Lines: 4-10
- Subsystem: [[DNS from First Principles/Subsystems/Command-line programs|Command-line programs]]
- Component: [[DNS from First Principles/Components/pickdns|pickdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-43f5784a63d0", "codeNote": "DNS from First Principles/Code/src/bin/pickdns.rs.source", "heading": "rgbdns-frag-43f5784a63d0: fn main", "sourcePath": "src/bin/pickdns.rs", "startLine": 4, "endLine": 10}
```

## Excerpt

<span id="rgbdns-frag-43f5784a63d0" class="rgbdns-fragment-target"></span>
### rgbdns-frag-43f5784a63d0: fn main

```rust
fn main() {
    if let Err(error) = run() {
        eprintln!("pickdns: fatal: {error}");
        std::process::exit(111);
    }
}

```
