---
type: "code-fragment"
fragment_id: "rgbdns-frag-fe2c63956fd1"
source_path: "src/pick.rs"
code_note: "DNS from First Principles/Code/src/pick.rs.source"
language: "rust"
subsystem: "Specialized responders"
crate: "rgbdns"
symbol: "location"
kind: "fn"
start_line: 183
end_line: 190
---

# location

- Fragment ID: `rgbdns-frag-fe2c63956fd1`
- Source file: [[DNS from First Principles/Code/src/pick.rs.source|src/pick.rs]]
- Lines: 183-190
- Subsystem: [[DNS from First Principles/Subsystems/Specialized responders|Specialized responders]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-fe2c63956fd1", "codeNote": "DNS from First Principles/Code/src/pick.rs.source", "heading": "rgbdns-frag-fe2c63956fd1: fn location", "sourcePath": "src/pick.rs", "startLine": 183, "endLine": 190}
```

## Excerpt

<span id="rgbdns-frag-fe2c63956fd1" class="rgbdns-fragment-target"></span>
### rgbdns-frag-fe2c63956fd1: fn location

```rust
fn location(value: &str) -> [u8; 2] {
    let bytes = value.as_bytes();
    [
        bytes.first().copied().unwrap_or(0),
        bytes.get(1).copied().unwrap_or(0),
    ]
}

```
