---
type: "code-fragment"
fragment_id: "rgbdns-frag-31aba88be59b"
source_path: "src/bin/dnsmx.rs"
code_note: "DNS from First Principles/Code/src/bin/dnsmx.rs.source"
language: "rust"
subsystem: "Command-line programs"
crate: "dnsmx"
symbol: "main"
kind: "fn"
start_line: 3
end_line: 9
---

# main

- Fragment ID: `rgbdns-frag-31aba88be59b`
- Source file: [[DNS from First Principles/Code/src/bin/dnsmx.rs.source|src/bin/dnsmx.rs]]
- Lines: 3-9
- Subsystem: [[DNS from First Principles/Subsystems/Command-line programs|Command-line programs]]
- Component: [[DNS from First Principles/Components/dnsmx|dnsmx]]

```rgbdns-fragment
{"id": "rgbdns-frag-31aba88be59b", "codeNote": "DNS from First Principles/Code/src/bin/dnsmx.rs.source", "heading": "rgbdns-frag-31aba88be59b: fn main", "sourcePath": "src/bin/dnsmx.rs", "startLine": 3, "endLine": 9}
```

## Excerpt

<span id="rgbdns-frag-31aba88be59b" class="rgbdns-fragment-target"></span>
### rgbdns-frag-31aba88be59b: fn main

```rust
fn main() {
    if let Err(error) = run() {
        eprintln!("dnsmx: fatal: {error}");
        std::process::exit(111);
    }
}

```
