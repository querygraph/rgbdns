---
type: "code-fragment"
fragment_id: "rgbdns-frag-1dc9d0701169"
source_path: "src/zone.rs"
code_note: "DNS from First Principles/Code/src/zone.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "has_anames"
kind: "fn"
start_line: 256
end_line: 258
---

# has_anames

- Fragment ID: `rgbdns-frag-1dc9d0701169`
- Source file: [[DNS from First Principles/Code/src/zone.rs.source|src/zone.rs]]
- Lines: 256-258
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-1dc9d0701169", "codeNote": "DNS from First Principles/Code/src/zone.rs.source", "sourcePath": "src/zone.rs", "startLine": 256, "endLine": 258}
```

## Excerpt

<span id="rgbdns-frag-1dc9d0701169" class="rgbdns-fragment-target"></span>
### rgbdns-frag-1dc9d0701169: fn has_anames

```rust
    pub(crate) fn has_anames(&self) -> bool {
        !self.anames.is_empty()
    }
```
