---
type: "code-fragment"
fragment_id: "rgbdns-frag-defe520b4c5c"
source_path: "src/zone.rs"
code_note: "DNS from First Principles/Code/src/zone.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "aname"
kind: "fn"
start_line: 253
end_line: 255
---

# aname

- Fragment ID: `rgbdns-frag-defe520b4c5c`
- Source file: [[DNS from First Principles/Code/src/zone.rs.source|src/zone.rs]]
- Lines: 253-255
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-defe520b4c5c", "codeNote": "DNS from First Principles/Code/src/zone.rs.source", "sourcePath": "src/zone.rs", "startLine": 253, "endLine": 255}
```

## Excerpt

<span id="rgbdns-frag-defe520b4c5c" class="rgbdns-fragment-target"></span>
### rgbdns-frag-defe520b4c5c: fn aname

```rust
    pub(crate) fn aname(&self, owner: &Name) -> Option<&Aname> {
        self.anames.get(owner)
    }
```
