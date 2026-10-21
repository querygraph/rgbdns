---
type: "code-fragment"
fragment_id: "rgbdns-frag-3b7d91408492"
source_path: "src/zone.rs"
code_note: "DNS from First Principles/Code/src/zone.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "is_authoritative"
kind: "fn"
start_line: 110
end_line: 112
---

# is_authoritative

- Fragment ID: `rgbdns-frag-3b7d91408492`
- Source file: [[DNS from First Principles/Code/src/zone.rs.source|src/zone.rs]]
- Lines: 110-112
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-3b7d91408492", "codeNote": "DNS from First Principles/Code/src/zone.rs.source", "sourcePath": "src/zone.rs", "startLine": 110, "endLine": 112}
```

## Excerpt

<span id="rgbdns-frag-3b7d91408492" class="rgbdns-fragment-target"></span>
### rgbdns-frag-3b7d91408492: fn is_authoritative

```rust
    pub(crate) fn is_authoritative(&self, zone: &Name) -> bool {
        self.authoritative.contains(zone)
    }
```
