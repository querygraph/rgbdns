---
type: "code-fragment"
fragment_id: "rgbdns-frag-5e582526ed53"
source_path: "src/zone.rs"
code_note: "DNS from First Principles/Code/src/zone.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "soa"
kind: "fn"
start_line: 622
end_line: 626
---

# soa

- Fragment ID: `rgbdns-frag-5e582526ed53`
- Source file: [[DNS from First Principles/Code/src/zone.rs.source|src/zone.rs]]
- Lines: 622-626
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-5e582526ed53", "codeNote": "DNS from First Principles/Code/src/zone.rs.source", "heading": "rgbdns-frag-5e582526ed53: fn soa", "sourcePath": "src/zone.rs", "startLine": 622, "endLine": 626}
```

## Excerpt

<span id="rgbdns-frag-5e582526ed53" class="rgbdns-fragment-target"></span>
### rgbdns-frag-5e582526ed53: fn soa

```rust
    fn soa(&self, z: &Name, location: [u8; 2], now: u64) -> Option<Record> {
        self.visible_records(z, location, now)
            .into_iter()
            .find(|r| r.rr_type() == RecordType::Soa)
    }
```
