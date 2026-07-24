---
type: "code-fragment"
fragment_id: "rgbdns-frag-9df199853f5b"
source_path: "src/bin/dnsname.rs"
code_note: "DNS from First Principles/Code/src/bin/dnsname.rs.source"
language: "rust"
subsystem: "Command-line programs"
crate: "dnsname"
symbol: "main"
kind: "fn"
start_line: 4
end_line: 10
---

# main

- Fragment ID: `rgbdns-frag-9df199853f5b`
- Source file: [[DNS from First Principles/Code/src/bin/dnsname.rs.source|src/bin/dnsname.rs]]
- Lines: 4-10
- Subsystem: [[DNS from First Principles/Subsystems/Command-line programs|Command-line programs]]
- Component: [[DNS from First Principles/Components/dnsname|dnsname]]

```rgbdns-fragment
{"id": "rgbdns-frag-9df199853f5b", "codeNote": "DNS from First Principles/Code/src/bin/dnsname.rs.source", "heading": "rgbdns-frag-9df199853f5b: fn main", "sourcePath": "src/bin/dnsname.rs", "startLine": 4, "endLine": 10}
```

## Excerpt

<span id="rgbdns-frag-9df199853f5b" class="rgbdns-fragment-target"></span>
### rgbdns-frag-9df199853f5b: fn main

```rust
fn main() {
    if let Err(error) = run() {
        eprintln!("dnsname: fatal: {error}");
        std::process::exit(111);
    }
}

```
