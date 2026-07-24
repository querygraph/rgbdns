---
type: "code-fragment"
fragment_id: "rgbdns-frag-94b6d2b9f927"
source_path: "src/bin/dnsipq.rs"
code_note: "DNS from First Principles/Code/src/bin/dnsipq.rs.source"
language: "rust"
subsystem: "Command-line programs"
crate: "dnsipq"
symbol: "main"
kind: "fn"
start_line: 3
end_line: 9
---

# main

- Fragment ID: `rgbdns-frag-94b6d2b9f927`
- Source file: [[DNS from First Principles/Code/src/bin/dnsipq.rs.source|src/bin/dnsipq.rs]]
- Lines: 3-9
- Subsystem: [[DNS from First Principles/Subsystems/Command-line programs|Command-line programs]]
- Component: [[DNS from First Principles/Components/dnsipq|dnsipq]]

```rgbdns-fragment
{"id": "rgbdns-frag-94b6d2b9f927", "codeNote": "DNS from First Principles/Code/src/bin/dnsipq.rs.source", "heading": "rgbdns-frag-94b6d2b9f927: fn main", "sourcePath": "src/bin/dnsipq.rs", "startLine": 3, "endLine": 9}
```

## Excerpt

<span id="rgbdns-frag-94b6d2b9f927" class="rgbdns-fragment-target"></span>
### rgbdns-frag-94b6d2b9f927: fn main

```rust
fn main() {
    if let Err(error) = run() {
        eprintln!("dnsipq: fatal: {error}");
        std::process::exit(111);
    }
}

```
