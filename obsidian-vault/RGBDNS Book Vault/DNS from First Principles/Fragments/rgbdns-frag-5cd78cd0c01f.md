---
type: "code-fragment"
fragment_id: "rgbdns-frag-5cd78cd0c01f"
source_path: "src/bin/axfrdns-conf.rs"
code_note: "DNS from First Principles/Code/src/bin/axfrdns-conf.rs.source"
language: "rust"
subsystem: "Command-line programs"
crate: "axfrdns-conf"
symbol: "main"
kind: "fn"
start_line: 1
end_line: 7
---

# main

- Fragment ID: `rgbdns-frag-5cd78cd0c01f`
- Source file: [[DNS from First Principles/Code/src/bin/axfrdns-conf.rs.source|src/bin/axfrdns-conf.rs]]
- Lines: 1-7
- Subsystem: [[DNS from First Principles/Subsystems/Command-line programs|Command-line programs]]
- Component: [[DNS from First Principles/Components/axfrdns-conf|axfrdns-conf]]

```rgbdns-fragment
{"id": "rgbdns-frag-5cd78cd0c01f", "codeNote": "DNS from First Principles/Code/src/bin/axfrdns-conf.rs.source", "heading": "rgbdns-frag-5cd78cd0c01f: fn main", "sourcePath": "src/bin/axfrdns-conf.rs", "startLine": 1, "endLine": 7}
```

## Excerpt

<span id="rgbdns-frag-5cd78cd0c01f" class="rgbdns-fragment-target"></span>
### rgbdns-frag-5cd78cd0c01f: fn main

```rust
fn main() {
    let arguments = std::env::args().skip(1).collect::<Vec<_>>();
    if let Err(error) = rgbdns::conf::configure(rgbdns::conf::Service::Axfrdns, &arguments) {
        eprintln!("axfrdns-conf: fatal: {error}");
        std::process::exit(111);
    }
}
```
