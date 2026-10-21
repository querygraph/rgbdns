---
type: "code-fragment"
fragment_id: "rgbdns-frag-faeef6810168"
source_path: "src/zone.rs"
code_note: "DNS from First Principles/Code/src/zone.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "has_aname_in_zone"
kind: "fn"
start_line: 259
end_line: 262
---

# has_aname_in_zone

- Fragment ID: `rgbdns-frag-faeef6810168`
- Source file: [[DNS from First Principles/Code/src/zone.rs.source|src/zone.rs]]
- Lines: 259-262
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-faeef6810168", "codeNote": "DNS from First Principles/Code/src/zone.rs.source", "sourcePath": "src/zone.rs", "startLine": 259, "endLine": 262}
```

## Excerpt

<span id="rgbdns-frag-faeef6810168" class="rgbdns-fragment-target"></span>
### rgbdns-frag-faeef6810168: fn has_aname_in_zone

```rust
    pub(crate) fn has_aname_in_zone(&self, name: &Name) -> bool {
        self.transfer_anames(name)
            .is_some_and(|anames| !anames.is_empty())
    }
```
