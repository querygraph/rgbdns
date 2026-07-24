---
type: "code-fragment"
fragment_id: "rgbdns-frag-ace5d7b6b93d"
source_path: "src/zone.rs"
code_note: "DNS from First Principles/Code/src/zone.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "location_code"
kind: "fn"
start_line: 683
end_line: 689
---

# location_code

- Fragment ID: `rgbdns-frag-ace5d7b6b93d`
- Source file: [[DNS from First Principles/Code/src/zone.rs.source|src/zone.rs]]
- Lines: 683-689
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-ace5d7b6b93d", "codeNote": "DNS from First Principles/Code/src/zone.rs.source", "heading": "rgbdns-frag-ace5d7b6b93d: fn location_code", "sourcePath": "src/zone.rs", "startLine": 683, "endLine": 689}
```

## Excerpt

<span id="rgbdns-frag-ace5d7b6b93d" class="rgbdns-fragment-target"></span>
### rgbdns-frag-ace5d7b6b93d: fn location_code

```rust
fn location_code(value: &str) -> [u8; 2] {
    let bytes = value.as_bytes();
    [
        bytes.first().copied().unwrap_or(0),
        bytes.get(1).copied().unwrap_or(0),
    ]
}
```
