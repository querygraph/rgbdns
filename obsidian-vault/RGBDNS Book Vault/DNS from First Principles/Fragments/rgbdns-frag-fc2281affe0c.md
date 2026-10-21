---
type: "code-fragment"
fragment_id: "rgbdns-frag-fc2281affe0c"
source_path: "src/zone.rs"
code_note: "DNS from First Principles/Code/src/zone.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "soa_serial"
kind: "fn"
start_line: 100
end_line: 109
---

# soa_serial

- Fragment ID: `rgbdns-frag-fc2281affe0c`
- Source file: [[DNS from First Principles/Code/src/zone.rs.source|src/zone.rs]]
- Lines: 100-109
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-fc2281affe0c", "codeNote": "DNS from First Principles/Code/src/zone.rs.source", "sourcePath": "src/zone.rs", "startLine": 100, "endLine": 109}
```

## Excerpt

<span id="rgbdns-frag-fc2281affe0c" class="rgbdns-fragment-target"></span>
### rgbdns-frag-fc2281affe0c: fn soa_serial

```rust
    pub(crate) fn soa_serial(&self, zone: &Name) -> Option<u32> {
        self.records.get(zone)?.iter().find_map(|record| {
            if let RData::Soa { serial, .. } = record.data {
                Some(serial)
            } else {
                None
            }
        })
    }

```
