---
type: "code-fragment"
fragment_id: "rgbdns-frag-dd026b16b5bd"
source_path: "src/bin/tinydns-conf.rs"
code_note: "DNS from First Principles/Code/src/bin/tinydns-conf.rs.source"
language: "rust"
subsystem: "Command-line programs"
crate: "tinydns-conf"
symbol: "main"
kind: "fn"
start_line: 1
end_line: 7
---

# main

- Fragment ID: `rgbdns-frag-dd026b16b5bd`
- Source file: [[DNS from First Principles/Code/src/bin/tinydns-conf.rs.source|src/bin/tinydns-conf.rs]]
- Lines: 1-7
- Subsystem: [[DNS from First Principles/Subsystems/Command-line programs|Command-line programs]]
- Component: [[DNS from First Principles/Components/tinydns-conf|tinydns-conf]]

```rgbdns-fragment
{"id": "rgbdns-frag-dd026b16b5bd", "codeNote": "DNS from First Principles/Code/src/bin/tinydns-conf.rs.source", "heading": "rgbdns-frag-dd026b16b5bd: fn main", "sourcePath": "src/bin/tinydns-conf.rs", "startLine": 1, "endLine": 7}
```

## Excerpt

<span id="rgbdns-frag-dd026b16b5bd" class="rgbdns-fragment-target"></span>
### rgbdns-frag-dd026b16b5bd: fn main

```rust
fn main() {
    let arguments = std::env::args().skip(1).collect::<Vec<_>>();
    if let Err(error) = rgbdns::conf::configure(rgbdns::conf::Service::Tinydns, &arguments) {
        eprintln!("tinydns-conf: fatal: {error}");
        std::process::exit(111);
    }
}
```
