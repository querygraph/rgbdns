---
type: "code-fragment"
fragment_id: "rgbdns-frag-7f3ab8886533"
source_path: "src/bin/pickdns-conf.rs"
code_note: "DNS from First Principles/Code/src/bin/pickdns-conf.rs.source"
language: "rust"
subsystem: "Command-line programs"
crate: "pickdns-conf"
symbol: "main"
kind: "fn"
start_line: 1
end_line: 7
---

# main

- Fragment ID: `rgbdns-frag-7f3ab8886533`
- Source file: [[DNS from First Principles/Code/src/bin/pickdns-conf.rs.source|src/bin/pickdns-conf.rs]]
- Lines: 1-7
- Subsystem: [[DNS from First Principles/Subsystems/Command-line programs|Command-line programs]]
- Component: [[DNS from First Principles/Components/pickdns-conf|pickdns-conf]]

```rgbdns-fragment
{"id": "rgbdns-frag-7f3ab8886533", "codeNote": "DNS from First Principles/Code/src/bin/pickdns-conf.rs.source", "heading": "rgbdns-frag-7f3ab8886533: fn main", "sourcePath": "src/bin/pickdns-conf.rs", "startLine": 1, "endLine": 7}
```

## Excerpt

<span id="rgbdns-frag-7f3ab8886533" class="rgbdns-fragment-target"></span>
### rgbdns-frag-7f3ab8886533: fn main

```rust
fn main() {
    let arguments = std::env::args().skip(1).collect::<Vec<_>>();
    if let Err(error) = rgbdns::conf::configure(rgbdns::conf::Service::Pickdns, &arguments) {
        eprintln!("pickdns-conf: fatal: {error}");
        std::process::exit(111);
    }
}
```
