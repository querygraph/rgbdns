---
type: "code-fragment"
fragment_id: "rgbdns-frag-046ae295dd0a"
source_path: "src/bin/dnsqr.rs"
code_note: "DNS from First Principles/Code/src/bin/dnsqr.rs.source"
language: "rust"
subsystem: "Command-line programs"
crate: "dnsqr"
symbol: "main"
kind: "fn"
start_line: 3
end_line: 9
---

# main

- Fragment ID: `rgbdns-frag-046ae295dd0a`
- Source file: [[DNS from First Principles/Code/src/bin/dnsqr.rs.source|src/bin/dnsqr.rs]]
- Lines: 3-9
- Subsystem: [[DNS from First Principles/Subsystems/Command-line programs|Command-line programs]]
- Component: [[DNS from First Principles/Components/dnsqr|dnsqr]]

```rgbdns-fragment
{"id": "rgbdns-frag-046ae295dd0a", "codeNote": "DNS from First Principles/Code/src/bin/dnsqr.rs.source", "heading": "rgbdns-frag-046ae295dd0a: fn main", "sourcePath": "src/bin/dnsqr.rs", "startLine": 3, "endLine": 9}
```

## Excerpt

<span id="rgbdns-frag-046ae295dd0a" class="rgbdns-fragment-target"></span>
### rgbdns-frag-046ae295dd0a: fn main

```rust
fn main() {
    if let Err(error) = run() {
        eprintln!("dnsqr: fatal: {error}");
        std::process::exit(111);
    }
}

```
