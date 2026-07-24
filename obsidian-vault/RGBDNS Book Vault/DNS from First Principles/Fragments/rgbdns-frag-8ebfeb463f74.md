---
type: "code-fragment"
fragment_id: "rgbdns-frag-8ebfeb463f74"
source_path: "src/bin/rbldns-conf.rs"
code_note: "DNS from First Principles/Code/src/bin/rbldns-conf.rs.source"
language: "rust"
subsystem: "Command-line programs"
crate: "rbldns-conf"
symbol: "main"
kind: "fn"
start_line: 1
end_line: 7
---

# main

- Fragment ID: `rgbdns-frag-8ebfeb463f74`
- Source file: [[DNS from First Principles/Code/src/bin/rbldns-conf.rs.source|src/bin/rbldns-conf.rs]]
- Lines: 1-7
- Subsystem: [[DNS from First Principles/Subsystems/Command-line programs|Command-line programs]]
- Component: [[DNS from First Principles/Components/rbldns-conf|rbldns-conf]]

```rgbdns-fragment
{"id": "rgbdns-frag-8ebfeb463f74", "codeNote": "DNS from First Principles/Code/src/bin/rbldns-conf.rs.source", "heading": "rgbdns-frag-8ebfeb463f74: fn main", "sourcePath": "src/bin/rbldns-conf.rs", "startLine": 1, "endLine": 7}
```

## Excerpt

<span id="rgbdns-frag-8ebfeb463f74" class="rgbdns-fragment-target"></span>
### rgbdns-frag-8ebfeb463f74: fn main

```rust
fn main() {
    let arguments = std::env::args().skip(1).collect::<Vec<_>>();
    if let Err(error) = rgbdns::conf::configure(rgbdns::conf::Service::Rbldns, &arguments) {
        eprintln!("rbldns-conf: fatal: {error}");
        std::process::exit(111);
    }
}
```
