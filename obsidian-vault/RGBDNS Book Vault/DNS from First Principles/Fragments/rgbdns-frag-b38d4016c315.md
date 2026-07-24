---
type: "code-fragment"
fragment_id: "rgbdns-frag-b38d4016c315"
source_path: "src/bin/dnsip.rs"
code_note: "DNS from First Principles/Code/src/bin/dnsip.rs.source"
language: "rust"
subsystem: "Command-line programs"
crate: "dnsip"
symbol: "main"
kind: "fn"
start_line: 3
end_line: 9
---

# main

- Fragment ID: `rgbdns-frag-b38d4016c315`
- Source file: [[DNS from First Principles/Code/src/bin/dnsip.rs.source|src/bin/dnsip.rs]]
- Lines: 3-9
- Subsystem: [[DNS from First Principles/Subsystems/Command-line programs|Command-line programs]]
- Component: [[DNS from First Principles/Components/dnsip|dnsip]]

```rgbdns-fragment
{"id": "rgbdns-frag-b38d4016c315", "codeNote": "DNS from First Principles/Code/src/bin/dnsip.rs.source", "heading": "rgbdns-frag-b38d4016c315: fn main", "sourcePath": "src/bin/dnsip.rs", "startLine": 3, "endLine": 9}
```

## Excerpt

<span id="rgbdns-frag-b38d4016c315" class="rgbdns-fragment-target"></span>
### rgbdns-frag-b38d4016c315: fn main

```rust
fn main() {
    if let Err(error) = run() {
        eprintln!("dnsip: fatal: {error}");
        std::process::exit(111);
    }
}

```
