---
type: "code-fragment"
fragment_id: "rgbdns-frag-ad2f12a33827"
source_path: "src/zone.rs"
code_note: "DNS from First Principles/Code/src/zone.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "location_entries"
kind: "fn"
start_line: 111
end_line: 115
---

# location_entries

- Fragment ID: `rgbdns-frag-ad2f12a33827`
- Source file: [[DNS from First Principles/Code/src/zone.rs.source|src/zone.rs]]
- Lines: 111-115
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-ad2f12a33827", "codeNote": "DNS from First Principles/Code/src/zone.rs.source", "heading": "rgbdns-frag-ad2f12a33827: fn location_entries", "sourcePath": "src/zone.rs", "startLine": 111, "endLine": 115}
```

## Excerpt

<span id="rgbdns-frag-ad2f12a33827" class="rgbdns-fragment-target"></span>
### rgbdns-frag-ad2f12a33827: fn location_entries

```rust
    pub(crate) fn location_entries(&self) -> impl Iterator<Item = (&[u8], [u8; 2])> {
        self.locations
            .iter()
            .map(|(prefix, location)| (prefix.as_slice(), *location))
    }
```
