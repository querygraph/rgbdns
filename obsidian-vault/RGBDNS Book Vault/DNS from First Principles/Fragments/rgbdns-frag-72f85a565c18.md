---
type: "code-fragment"
fragment_id: "rgbdns-frag-72f85a565c18"
source_path: "src/zone.rs"
code_note: "DNS from First Principles/Code/src/zone.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "location_code"
kind: "fn"
start_line: 753
end_line: 759
---

# location_code

- Fragment ID: `rgbdns-frag-72f85a565c18`
- Source file: [[DNS from First Principles/Code/src/zone.rs.source|src/zone.rs]]
- Lines: 753-759
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-72f85a565c18", "codeNote": "DNS from First Principles/Code/src/zone.rs.source", "heading": "rgbdns-frag-72f85a565c18: fn location_code", "sourcePath": "src/zone.rs", "startLine": 753, "endLine": 759}
```

## Excerpt

<span id="rgbdns-frag-72f85a565c18" class="rgbdns-fragment-target"></span>
### rgbdns-frag-72f85a565c18: fn location_code

```rust
fn location_code(value: &str) -> [u8; 2] {
    let bytes = value.as_bytes();
    [
        bytes.first().copied().unwrap_or(0),
        bytes.get(1).copied().unwrap_or(0),
    ]
}
```
