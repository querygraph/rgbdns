---
type: "code-fragment"
fragment_id: "rgbdns-frag-d5f7800921ae"
source_path: "src/bin/dnsfilter.rs"
code_note: "DNS from First Principles/Code/src/bin/dnsfilter.rs.source"
language: "rust"
subsystem: "Command-line programs"
crate: "dnsfilter"
symbol: "main"
kind: "fn"
start_line: 13
end_line: 19
---

# main

- Fragment ID: `rgbdns-frag-d5f7800921ae`
- Source file: [[DNS from First Principles/Code/src/bin/dnsfilter.rs.source|src/bin/dnsfilter.rs]]
- Lines: 13-19
- Subsystem: [[DNS from First Principles/Subsystems/Command-line programs|Command-line programs]]
- Component: [[DNS from First Principles/Components/dnsfilter|dnsfilter]]

```rgbdns-fragment
{"id": "rgbdns-frag-d5f7800921ae", "codeNote": "DNS from First Principles/Code/src/bin/dnsfilter.rs.source", "heading": "rgbdns-frag-d5f7800921ae: fn main", "sourcePath": "src/bin/dnsfilter.rs", "startLine": 13, "endLine": 19}
```

## Excerpt

<span id="rgbdns-frag-d5f7800921ae" class="rgbdns-fragment-target"></span>
### rgbdns-frag-d5f7800921ae: fn main

```rust
fn main() {
    if let Err(error) = run() {
        eprintln!("dnsfilter: fatal: {error}");
        std::process::exit(111);
    }
}

```
