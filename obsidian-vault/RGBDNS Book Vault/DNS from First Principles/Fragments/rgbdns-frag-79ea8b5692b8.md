---
type: "code-fragment"
fragment_id: "rgbdns-frag-79ea8b5692b8"
source_path: "src/zone.rs"
code_note: "DNS from First Principles/Code/src/zone.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "newer_serial"
kind: "fn"
start_line: 37
end_line: 44
---

# newer_serial

- Fragment ID: `rgbdns-frag-79ea8b5692b8`
- Source file: [[DNS from First Principles/Code/src/zone.rs.source|src/zone.rs]]
- Lines: 37-44
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-79ea8b5692b8", "codeNote": "DNS from First Principles/Code/src/zone.rs.source", "sourcePath": "src/zone.rs", "startLine": 37, "endLine": 44}
```

## Excerpt

<span id="rgbdns-frag-79ea8b5692b8" class="rgbdns-fragment-target"></span>
### rgbdns-frag-79ea8b5692b8: fn newer_serial

```rust
pub(crate) fn newer_serial(a: u32, b: u32) -> u32 {
    if b != a && b.wrapping_sub(a) < (1 << 31) {
        b
    } else {
        a
    }
}

```
