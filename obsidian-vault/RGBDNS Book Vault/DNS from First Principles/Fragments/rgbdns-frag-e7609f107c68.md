---
type: "code-fragment"
fragment_id: "rgbdns-frag-e7609f107c68"
source_path: "src/bin/rgbdns-acme.rs"
code_note: "DNS from First Principles/Code/src/bin/rgbdns-acme.rs.source"
language: "rust"
subsystem: "Command-line programs"
crate: "rgbdns-acme"
symbol: "main"
kind: "fn"
start_line: 14
end_line: 20
---

# main

- Fragment ID: `rgbdns-frag-e7609f107c68`
- Source file: [[DNS from First Principles/Code/src/bin/rgbdns-acme.rs.source|src/bin/rgbdns-acme.rs]]
- Lines: 14-20
- Subsystem: [[DNS from First Principles/Subsystems/Command-line programs|Command-line programs]]
- Component: [[DNS from First Principles/Components/rgbdns-acme|rgbdns-acme]]

```rgbdns-fragment
{"id": "rgbdns-frag-e7609f107c68", "codeNote": "DNS from First Principles/Code/src/bin/rgbdns-acme.rs.source", "sourcePath": "src/bin/rgbdns-acme.rs", "startLine": 14, "endLine": 20}
```

## Excerpt

<span id="rgbdns-frag-e7609f107c68" class="rgbdns-fragment-target"></span>
### rgbdns-frag-e7609f107c68: fn main

```rust
fn main() {
    if let Err(error) = run() {
        eprintln!("rgbdns-acme: fatal: {error}");
        std::process::exit(111);
    }
}

```
