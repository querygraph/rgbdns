---
type: "code-fragment"
fragment_id: "rgbdns-frag-faa185040c9a"
source_path: "src/bin/dnsip6q.rs"
code_note: "DNS from First Principles/Code/src/bin/dnsip6q.rs.source"
language: "rust"
subsystem: "Command-line programs"
crate: "dnsip6q"
symbol: "main"
kind: "fn"
start_line: 3
end_line: 9
---

# main

- Fragment ID: `rgbdns-frag-faa185040c9a`
- Source file: [[DNS from First Principles/Code/src/bin/dnsip6q.rs.source|src/bin/dnsip6q.rs]]
- Lines: 3-9
- Subsystem: [[DNS from First Principles/Subsystems/Command-line programs|Command-line programs]]
- Component: [[DNS from First Principles/Components/dnsip6q|dnsip6q]]

```rgbdns-fragment
{"id": "rgbdns-frag-faa185040c9a", "codeNote": "DNS from First Principles/Code/src/bin/dnsip6q.rs.source", "heading": "rgbdns-frag-faa185040c9a: fn main", "sourcePath": "src/bin/dnsip6q.rs", "startLine": 3, "endLine": 9}
```

## Excerpt

<span id="rgbdns-frag-faa185040c9a" class="rgbdns-fragment-target"></span>
### rgbdns-frag-faa185040c9a: fn main

```rust
fn main() {
    if let Err(error) = run() {
        eprintln!("dnsip6q: fatal: {error}");
        std::process::exit(111);
    }
}

```
