---
type: "code-fragment"
fragment_id: "rgbdns-frag-8865b8fe1f9d"
source_path: "src/zone.rs"
code_note: "DNS from First Principles/Code/src/zone.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "aname_entries"
kind: "fn"
start_line: 231
end_line: 233
---

# aname_entries

- Fragment ID: `rgbdns-frag-8865b8fe1f9d`
- Source file: [[DNS from First Principles/Code/src/zone.rs.source|src/zone.rs]]
- Lines: 231-233
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-8865b8fe1f9d", "codeNote": "DNS from First Principles/Code/src/zone.rs.source", "sourcePath": "src/zone.rs", "startLine": 231, "endLine": 233}
```

## Excerpt

<span id="rgbdns-frag-8865b8fe1f9d" class="rgbdns-fragment-target"></span>
### rgbdns-frag-8865b8fe1f9d: fn aname_entries

```rust
    pub(crate) fn aname_entries(&self) -> impl Iterator<Item = (&Name, &Aname)> {
        self.anames.iter()
    }
```
