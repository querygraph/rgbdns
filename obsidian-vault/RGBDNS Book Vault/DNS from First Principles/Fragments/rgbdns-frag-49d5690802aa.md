---
type: "code-fragment"
fragment_id: "rgbdns-frag-49d5690802aa"
source_path: "src/bin/dnscache-conf.rs"
code_note: "DNS from First Principles/Code/src/bin/dnscache-conf.rs.source"
language: "rust"
subsystem: "Command-line programs"
crate: "dnscache-conf"
symbol: "main"
kind: "fn"
start_line: 1
end_line: 7
---

# main

- Fragment ID: `rgbdns-frag-49d5690802aa`
- Source file: [[DNS from First Principles/Code/src/bin/dnscache-conf.rs.source|src/bin/dnscache-conf.rs]]
- Lines: 1-7
- Subsystem: [[DNS from First Principles/Subsystems/Command-line programs|Command-line programs]]
- Component: [[DNS from First Principles/Components/dnscache-conf|dnscache-conf]]

```rgbdns-fragment
{"id": "rgbdns-frag-49d5690802aa", "codeNote": "DNS from First Principles/Code/src/bin/dnscache-conf.rs.source", "heading": "rgbdns-frag-49d5690802aa: fn main", "sourcePath": "src/bin/dnscache-conf.rs", "startLine": 1, "endLine": 7}
```

## Excerpt

<span id="rgbdns-frag-49d5690802aa" class="rgbdns-fragment-target"></span>
### rgbdns-frag-49d5690802aa: fn main

```rust
fn main() {
    let arguments = std::env::args().skip(1).collect::<Vec<_>>();
    if let Err(error) = rgbdns::conf::configure(rgbdns::conf::Service::Dnscache, &arguments) {
        eprintln!("dnscache-conf: fatal: {error}");
        std::process::exit(111);
    }
}
```
