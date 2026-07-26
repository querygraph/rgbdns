---
type: "code-fragment"
fragment_id: "rgbdns-frag-a7aa463839ef"
source_path: "src/zone.rs"
code_note: "DNS from First Principles/Code/src/zone.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "parse"
kind: "fn"
start_line: 49
end_line: 51
---

# parse

- Fragment ID: `rgbdns-frag-a7aa463839ef`
- Source file: [[DNS from First Principles/Code/src/zone.rs.source|src/zone.rs]]
- Lines: 49-51
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-a7aa463839ef", "codeNote": "DNS from First Principles/Code/src/zone.rs.source", "heading": "rgbdns-frag-a7aa463839ef: fn parse", "sourcePath": "src/zone.rs", "startLine": 49, "endLine": 51}
```

## Excerpt

<span id="rgbdns-frag-a7aa463839ef" class="rgbdns-fragment-target"></span>
### rgbdns-frag-a7aa463839ef: fn parse

```rust
    pub fn parse(text: &str) -> Result<Self> {
        Self::parse_with_serial(text, 1)
    }
```
