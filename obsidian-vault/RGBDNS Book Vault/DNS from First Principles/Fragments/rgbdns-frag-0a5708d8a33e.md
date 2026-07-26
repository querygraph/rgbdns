---
type: "code-fragment"
fragment_id: "rgbdns-frag-0a5708d8a33e"
source_path: "src/zone.rs"
code_note: "DNS from First Principles/Code/src/zone.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "aname_entries"
kind: "fn"
start_line: 147
end_line: 149
---

# aname_entries

- Fragment ID: `rgbdns-frag-0a5708d8a33e`
- Source file: [[DNS from First Principles/Code/src/zone.rs.source|src/zone.rs]]
- Lines: 147-149
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-0a5708d8a33e", "codeNote": "DNS from First Principles/Code/src/zone.rs.source", "heading": "rgbdns-frag-0a5708d8a33e: fn aname_entries", "sourcePath": "src/zone.rs", "startLine": 147, "endLine": 149}
```

## Excerpt

<span id="rgbdns-frag-0a5708d8a33e" class="rgbdns-fragment-target"></span>
### rgbdns-frag-0a5708d8a33e: fn aname_entries

```rust
    pub(crate) fn aname_entries(&self) -> impl Iterator<Item = (&Name, &Aname)> {
        self.anames.iter()
    }
```
