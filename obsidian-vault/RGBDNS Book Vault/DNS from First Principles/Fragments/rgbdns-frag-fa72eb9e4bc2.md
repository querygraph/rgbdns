---
type: "code-fragment"
fragment_id: "rgbdns-frag-fa72eb9e4bc2"
source_path: "src/bin/rgbdns-acme.rs"
code_note: "DNS from First Principles/Code/src/bin/rgbdns-acme.rs.source"
language: "rust"
subsystem: "Command-line programs"
crate: "rgbdns-acme"
symbol: "json_escape"
kind: "fn"
start_line: 92
end_line: 94
---

# json_escape

- Fragment ID: `rgbdns-frag-fa72eb9e4bc2`
- Source file: [[DNS from First Principles/Code/src/bin/rgbdns-acme.rs.source|src/bin/rgbdns-acme.rs]]
- Lines: 92-94
- Subsystem: [[DNS from First Principles/Subsystems/Command-line programs|Command-line programs]]
- Component: [[DNS from First Principles/Components/rgbdns-acme|rgbdns-acme]]

```rgbdns-fragment
{"id": "rgbdns-frag-fa72eb9e4bc2", "codeNote": "DNS from First Principles/Code/src/bin/rgbdns-acme.rs.source", "sourcePath": "src/bin/rgbdns-acme.rs", "startLine": 92, "endLine": 94}
```

## Excerpt

<span id="rgbdns-frag-fa72eb9e4bc2" class="rgbdns-fragment-target"></span>
### rgbdns-frag-fa72eb9e4bc2: fn json_escape

```rust
fn json_escape(value: &str) -> String {
    value.replace('\\', "\\\\").replace('"', "\\\"")
}
```
