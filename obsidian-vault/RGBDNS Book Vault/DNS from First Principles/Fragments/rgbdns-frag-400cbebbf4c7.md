---
type: "code-fragment"
fragment_id: "rgbdns-frag-400cbebbf4c7"
source_path: "src/bin/dnsq.rs"
code_note: "DNS from First Principles/Code/src/bin/dnsq.rs.source"
language: "rust"
subsystem: "Command-line programs"
crate: "dnsq"
symbol: "main"
kind: "fn"
start_line: 3
end_line: 9
---

# main

- Fragment ID: `rgbdns-frag-400cbebbf4c7`
- Source file: [[DNS from First Principles/Code/src/bin/dnsq.rs.source|src/bin/dnsq.rs]]
- Lines: 3-9
- Subsystem: [[DNS from First Principles/Subsystems/Command-line programs|Command-line programs]]
- Component: [[DNS from First Principles/Components/dnsq|dnsq]]

```rgbdns-fragment
{"id": "rgbdns-frag-400cbebbf4c7", "codeNote": "DNS from First Principles/Code/src/bin/dnsq.rs.source", "heading": "rgbdns-frag-400cbebbf4c7: fn main", "sourcePath": "src/bin/dnsq.rs", "startLine": 3, "endLine": 9}
```

## Excerpt

<span id="rgbdns-frag-400cbebbf4c7" class="rgbdns-fragment-target"></span>
### rgbdns-frag-400cbebbf4c7: fn main

```rust
fn main() {
    if let Err(error) = run() {
        eprintln!("dnsq: fatal: {error}");
        std::process::exit(111);
    }
}

```
