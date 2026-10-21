---
type: "code-fragment"
fragment_id: "rgbdns-frag-373e095ef9a2"
source_path: "src/zone.rs"
code_note: "DNS from First Principles/Code/src/zone.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "location_entries"
kind: "fn"
start_line: 226
end_line: 230
---

# location_entries

- Fragment ID: `rgbdns-frag-373e095ef9a2`
- Source file: [[DNS from First Principles/Code/src/zone.rs.source|src/zone.rs]]
- Lines: 226-230
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-373e095ef9a2", "codeNote": "DNS from First Principles/Code/src/zone.rs.source", "sourcePath": "src/zone.rs", "startLine": 226, "endLine": 230}
```

## Excerpt

<span id="rgbdns-frag-373e095ef9a2" class="rgbdns-fragment-target"></span>
### rgbdns-frag-373e095ef9a2: fn location_entries

```rust
    pub(crate) fn location_entries(&self) -> impl Iterator<Item = (&[u8], [u8; 2])> {
        self.locations
            .iter()
            .map(|(prefix, location)| (prefix.as_slice(), *location))
    }
```
