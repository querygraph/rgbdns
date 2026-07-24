---
type: "code-fragment"
fragment_id: "rgbdns-frag-9cc348c66b0c"
source_path: "src/zone.rs"
code_note: "DNS from First Principles/Code/src/zone.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "parse"
kind: "fn"
start_line: 42
end_line: 44
---

# parse

- Fragment ID: `rgbdns-frag-9cc348c66b0c`
- Source file: [[DNS from First Principles/Code/src/zone.rs.source|src/zone.rs]]
- Lines: 42-44
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-9cc348c66b0c", "codeNote": "DNS from First Principles/Code/src/zone.rs.source", "heading": "rgbdns-frag-9cc348c66b0c: fn parse", "sourcePath": "src/zone.rs", "startLine": 42, "endLine": 44}
```

## Excerpt

<span id="rgbdns-frag-9cc348c66b0c" class="rgbdns-fragment-target"></span>
### rgbdns-frag-9cc348c66b0c: fn parse

```rust
    pub fn parse(text: &str) -> Result<Self> {
        Self::parse_with_serial(text, 1)
    }
```
