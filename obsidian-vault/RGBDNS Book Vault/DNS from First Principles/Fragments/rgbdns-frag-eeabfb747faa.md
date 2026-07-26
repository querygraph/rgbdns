---
type: "code-fragment"
fragment_id: "rgbdns-frag-eeabfb747faa"
source_path: "src/zone.rs"
code_note: "DNS from First Principles/Code/src/zone.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "has_anames"
kind: "fn"
start_line: 153
end_line: 155
---

# has_anames

- Fragment ID: `rgbdns-frag-eeabfb747faa`
- Source file: [[DNS from First Principles/Code/src/zone.rs.source|src/zone.rs]]
- Lines: 153-155
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-eeabfb747faa", "codeNote": "DNS from First Principles/Code/src/zone.rs.source", "heading": "rgbdns-frag-eeabfb747faa: fn has_anames", "sourcePath": "src/zone.rs", "startLine": 153, "endLine": 155}
```

## Excerpt

<span id="rgbdns-frag-eeabfb747faa" class="rgbdns-fragment-target"></span>
### rgbdns-frag-eeabfb747faa: fn has_anames

```rust
    pub(crate) fn has_anames(&self) -> bool {
        !self.anames.is_empty()
    }
```
