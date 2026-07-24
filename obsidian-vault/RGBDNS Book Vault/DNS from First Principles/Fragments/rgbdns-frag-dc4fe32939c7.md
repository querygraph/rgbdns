---
type: "code-fragment"
fragment_id: "rgbdns-frag-dc4fe32939c7"
source_path: "src/bin/rbldns.rs"
code_note: "DNS from First Principles/Code/src/bin/rbldns.rs.source"
language: "rust"
subsystem: "Command-line programs"
crate: "rbldns"
symbol: "main"
kind: "fn"
start_line: 4
end_line: 10
---

# main

- Fragment ID: `rgbdns-frag-dc4fe32939c7`
- Source file: [[DNS from First Principles/Code/src/bin/rbldns.rs.source|src/bin/rbldns.rs]]
- Lines: 4-10
- Subsystem: [[DNS from First Principles/Subsystems/Command-line programs|Command-line programs]]
- Component: [[DNS from First Principles/Components/rbldns|rbldns]]

```rgbdns-fragment
{"id": "rgbdns-frag-dc4fe32939c7", "codeNote": "DNS from First Principles/Code/src/bin/rbldns.rs.source", "heading": "rgbdns-frag-dc4fe32939c7: fn main", "sourcePath": "src/bin/rbldns.rs", "startLine": 4, "endLine": 10}
```

## Excerpt

<span id="rgbdns-frag-dc4fe32939c7" class="rgbdns-fragment-target"></span>
### rgbdns-frag-dc4fe32939c7: fn main

```rust
fn main() {
    if let Err(error) = run() {
        eprintln!("rbldns: fatal: {error}");
        std::process::exit(111);
    }
}

```
