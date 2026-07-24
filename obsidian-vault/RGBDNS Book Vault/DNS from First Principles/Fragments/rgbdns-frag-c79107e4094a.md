---
type: "code-fragment"
fragment_id: "rgbdns-frag-c79107e4094a"
source_path: "src/bin/walldns.rs"
code_note: "DNS from First Principles/Code/src/bin/walldns.rs.source"
language: "rust"
subsystem: "Command-line programs"
crate: "walldns"
symbol: "main"
kind: "fn"
start_line: 4
end_line: 10
---

# main

- Fragment ID: `rgbdns-frag-c79107e4094a`
- Source file: [[DNS from First Principles/Code/src/bin/walldns.rs.source|src/bin/walldns.rs]]
- Lines: 4-10
- Subsystem: [[DNS from First Principles/Subsystems/Command-line programs|Command-line programs]]
- Component: [[DNS from First Principles/Components/walldns|walldns]]

```rgbdns-fragment
{"id": "rgbdns-frag-c79107e4094a", "codeNote": "DNS from First Principles/Code/src/bin/walldns.rs.source", "heading": "rgbdns-frag-c79107e4094a: fn main", "sourcePath": "src/bin/walldns.rs", "startLine": 4, "endLine": 10}
```

## Excerpt

<span id="rgbdns-frag-c79107e4094a" class="rgbdns-fragment-target"></span>
### rgbdns-frag-c79107e4094a: fn main

```rust
fn main() {
    if let Err(error) = run() {
        eprintln!("walldns: fatal: {error}");
        std::process::exit(111);
    }
}

```
