---
type: "code-fragment"
fragment_id: "rgbdns-frag-a4e8a21459c7"
source_path: "src/zone.rs"
code_note: "DNS from First Principles/Code/src/zone.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "soa"
kind: "fn"
start_line: 793
end_line: 797
---

# soa

- Fragment ID: `rgbdns-frag-a4e8a21459c7`
- Source file: [[DNS from First Principles/Code/src/zone.rs.source|src/zone.rs]]
- Lines: 793-797
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-a4e8a21459c7", "codeNote": "DNS from First Principles/Code/src/zone.rs.source", "sourcePath": "src/zone.rs", "startLine": 793, "endLine": 797}
```

## Excerpt

<span id="rgbdns-frag-a4e8a21459c7" class="rgbdns-fragment-target"></span>
### rgbdns-frag-a4e8a21459c7: fn soa

```rust
    fn soa(&self, z: &Name, location: [u8; 2], now: u64) -> Option<Record> {
        self.visible_records(z, location, now)
            .into_iter()
            .find(|r| r.rr_type() == RecordType::Soa)
    }
```
