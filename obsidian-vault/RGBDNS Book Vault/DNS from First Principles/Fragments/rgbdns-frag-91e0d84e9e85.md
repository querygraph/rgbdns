---
type: "code-fragment"
fragment_id: "rgbdns-frag-91e0d84e9e85"
source_path: "src/bin/dnsip6.rs"
code_note: "DNS from First Principles/Code/src/bin/dnsip6.rs.source"
language: "rust"
subsystem: "Command-line programs"
crate: "dnsip6"
symbol: "main"
kind: "fn"
start_line: 3
end_line: 9
---

# main

- Fragment ID: `rgbdns-frag-91e0d84e9e85`
- Source file: [[DNS from First Principles/Code/src/bin/dnsip6.rs.source|src/bin/dnsip6.rs]]
- Lines: 3-9
- Subsystem: [[DNS from First Principles/Subsystems/Command-line programs|Command-line programs]]
- Component: [[DNS from First Principles/Components/dnsip6|dnsip6]]

```rgbdns-fragment
{"id": "rgbdns-frag-91e0d84e9e85", "codeNote": "DNS from First Principles/Code/src/bin/dnsip6.rs.source", "heading": "rgbdns-frag-91e0d84e9e85: fn main", "sourcePath": "src/bin/dnsip6.rs", "startLine": 3, "endLine": 9}
```

## Excerpt

<span id="rgbdns-frag-91e0d84e9e85" class="rgbdns-fragment-target"></span>
### rgbdns-frag-91e0d84e9e85: fn main

```rust
fn main() {
    if let Err(error) = run() {
        eprintln!("dnsip6: fatal: {error}");
        std::process::exit(111);
    }
}

```
