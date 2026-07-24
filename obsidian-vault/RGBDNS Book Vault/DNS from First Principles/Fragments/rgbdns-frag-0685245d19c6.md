---
type: "code-fragment"
fragment_id: "rgbdns-frag-0685245d19c6"
source_path: "src/bin/random-ip.rs"
code_note: "DNS from First Principles/Code/src/bin/random-ip.rs.source"
language: "rust"
subsystem: "Command-line programs"
crate: "random-ip"
symbol: "main"
kind: "fn"
start_line: 1
end_line: 7
---

# main

- Fragment ID: `rgbdns-frag-0685245d19c6`
- Source file: [[DNS from First Principles/Code/src/bin/random-ip.rs.source|src/bin/random-ip.rs]]
- Lines: 1-7
- Subsystem: [[DNS from First Principles/Subsystems/Command-line programs|Command-line programs]]
- Component: [[DNS from First Principles/Components/random-ip|random-ip]]

```rgbdns-fragment
{"id": "rgbdns-frag-0685245d19c6", "codeNote": "DNS from First Principles/Code/src/bin/random-ip.rs.source", "heading": "rgbdns-frag-0685245d19c6: fn main", "sourcePath": "src/bin/random-ip.rs", "startLine": 1, "endLine": 7}
```

## Excerpt

<span id="rgbdns-frag-0685245d19c6" class="rgbdns-fragment-target"></span>
### rgbdns-frag-0685245d19c6: fn main

```rust
fn main() {
    if let Err(error) = run() {
        eprintln!("random-ip: fatal: {error}");
        std::process::exit(111);
    }
}

```
