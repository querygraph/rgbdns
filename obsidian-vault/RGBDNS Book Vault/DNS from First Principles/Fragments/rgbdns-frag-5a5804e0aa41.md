---
type: "code-fragment"
fragment_id: "rgbdns-frag-5a5804e0aa41"
source_path: "src/bin/axfrdns.rs"
code_note: "DNS from First Principles/Code/src/bin/axfrdns.rs.source"
language: "rust"
subsystem: "Command-line programs"
crate: "axfrdns"
symbol: "main"
kind: "fn"
start_line: 4
end_line: 10
---

# main

- Fragment ID: `rgbdns-frag-5a5804e0aa41`
- Source file: [[DNS from First Principles/Code/src/bin/axfrdns.rs.source|src/bin/axfrdns.rs]]
- Lines: 4-10
- Subsystem: [[DNS from First Principles/Subsystems/Command-line programs|Command-line programs]]
- Component: [[DNS from First Principles/Components/axfrdns|axfrdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-5a5804e0aa41", "codeNote": "DNS from First Principles/Code/src/bin/axfrdns.rs.source", "heading": "rgbdns-frag-5a5804e0aa41: fn main", "sourcePath": "src/bin/axfrdns.rs", "startLine": 4, "endLine": 10}
```

## Excerpt

<span id="rgbdns-frag-5a5804e0aa41" class="rgbdns-fragment-target"></span>
### rgbdns-frag-5a5804e0aa41: fn main

```rust
fn main() {
    if let Err(error) = run() {
        eprintln!("axfrdns: fatal: {error}");
        std::process::exit(111);
    }
}

```
