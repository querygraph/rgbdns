---
type: "code-fragment"
fragment_id: "rgbdns-frag-1a679f85ce82"
source_path: "src/zone.rs"
code_note: "DNS from First Principles/Code/src/zone.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "location_code"
kind: "fn"
start_line: 1021
end_line: 1027
---

# location_code

- Fragment ID: `rgbdns-frag-1a679f85ce82`
- Source file: [[DNS from First Principles/Code/src/zone.rs.source|src/zone.rs]]
- Lines: 1021-1027
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-1a679f85ce82", "codeNote": "DNS from First Principles/Code/src/zone.rs.source", "sourcePath": "src/zone.rs", "startLine": 1021, "endLine": 1027}
```

## Excerpt

<span id="rgbdns-frag-1a679f85ce82" class="rgbdns-fragment-target"></span>
### rgbdns-frag-1a679f85ce82: fn location_code

```rust
fn location_code(value: &str) -> [u8; 2] {
    let bytes = value.as_bytes();
    [
        bytes.first().copied().unwrap_or(0),
        bytes.get(1).copied().unwrap_or(0),
    ]
}
```
