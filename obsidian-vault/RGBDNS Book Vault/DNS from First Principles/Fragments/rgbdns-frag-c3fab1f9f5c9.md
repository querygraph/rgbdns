---
type: "code-fragment"
fragment_id: "rgbdns-frag-c3fab1f9f5c9"
source_path: "src/zone.rs"
code_note: "DNS from First Principles/Code/src/zone.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "parse"
kind: "fn"
start_line: 133
end_line: 135
---

# parse

- Fragment ID: `rgbdns-frag-c3fab1f9f5c9`
- Source file: [[DNS from First Principles/Code/src/zone.rs.source|src/zone.rs]]
- Lines: 133-135
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-c3fab1f9f5c9", "codeNote": "DNS from First Principles/Code/src/zone.rs.source", "sourcePath": "src/zone.rs", "startLine": 133, "endLine": 135}
```

## Excerpt

<span id="rgbdns-frag-c3fab1f9f5c9" class="rgbdns-fragment-target"></span>
### rgbdns-frag-c3fab1f9f5c9: fn parse

```rust
    pub fn parse(text: &str) -> Result<Self> {
        Self::parse_with_serial(text, 1)
    }
```
