---
type: "code-fragment"
fragment_id: "rgbdns-frag-08ba9525d1a2"
source_path: "src/bin/walldns-conf.rs"
code_note: "DNS from First Principles/Code/src/bin/walldns-conf.rs.source"
language: "rust"
subsystem: "Command-line programs"
crate: "walldns-conf"
symbol: "main"
kind: "fn"
start_line: 1
end_line: 7
---

# main

- Fragment ID: `rgbdns-frag-08ba9525d1a2`
- Source file: [[DNS from First Principles/Code/src/bin/walldns-conf.rs.source|src/bin/walldns-conf.rs]]
- Lines: 1-7
- Subsystem: [[DNS from First Principles/Subsystems/Command-line programs|Command-line programs]]
- Component: [[DNS from First Principles/Components/walldns-conf|walldns-conf]]

```rgbdns-fragment
{"id": "rgbdns-frag-08ba9525d1a2", "codeNote": "DNS from First Principles/Code/src/bin/walldns-conf.rs.source", "heading": "rgbdns-frag-08ba9525d1a2: fn main", "sourcePath": "src/bin/walldns-conf.rs", "startLine": 1, "endLine": 7}
```

## Excerpt

<span id="rgbdns-frag-08ba9525d1a2" class="rgbdns-fragment-target"></span>
### rgbdns-frag-08ba9525d1a2: fn main

```rust
fn main() {
    let arguments = std::env::args().skip(1).collect::<Vec<_>>();
    if let Err(error) = rgbdns::conf::configure(rgbdns::conf::Service::Walldns, &arguments) {
        eprintln!("walldns-conf: fatal: {error}");
        std::process::exit(111);
    }
}
```
