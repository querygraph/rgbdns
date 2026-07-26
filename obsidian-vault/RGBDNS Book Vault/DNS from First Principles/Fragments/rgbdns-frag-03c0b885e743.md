---
type: "code-fragment"
fragment_id: "rgbdns-frag-03c0b885e743"
source_path: "src/zone.rs"
code_note: "DNS from First Principles/Code/src/zone.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "aname"
kind: "fn"
start_line: 150
end_line: 152
---

# aname

- Fragment ID: `rgbdns-frag-03c0b885e743`
- Source file: [[DNS from First Principles/Code/src/zone.rs.source|src/zone.rs]]
- Lines: 150-152
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-03c0b885e743", "codeNote": "DNS from First Principles/Code/src/zone.rs.source", "heading": "rgbdns-frag-03c0b885e743: fn aname", "sourcePath": "src/zone.rs", "startLine": 150, "endLine": 152}
```

## Excerpt

<span id="rgbdns-frag-03c0b885e743" class="rgbdns-fragment-target"></span>
### rgbdns-frag-03c0b885e743: fn aname

```rust
    pub(crate) fn aname(&self, owner: &Name) -> Option<&Aname> {
        self.anames.get(owner)
    }
```
