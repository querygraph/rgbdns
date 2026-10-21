---
type: "code-fragment"
fragment_id: "rgbdns-frag-73e9ad084753"
source_path: "src/zone.rs"
code_note: "DNS from First Principles/Code/src/zone.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "rrsig_covered"
kind: "fn"
start_line: 945
end_line: 951
---

# rrsig_covered

- Fragment ID: `rgbdns-frag-73e9ad084753`
- Source file: [[DNS from First Principles/Code/src/zone.rs.source|src/zone.rs]]
- Lines: 945-951
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-73e9ad084753", "codeNote": "DNS from First Principles/Code/src/zone.rs.source", "sourcePath": "src/zone.rs", "startLine": 945, "endLine": 951}
```

## Excerpt

<span id="rgbdns-frag-73e9ad084753" class="rgbdns-fragment-target"></span>
### rgbdns-frag-73e9ad084753: fn rrsig_covered

```rust
fn rrsig_covered(data: &RData) -> Option<RecordType> {
    match data.dnssec().ok().flatten()? {
        crate::DnssecRData::Rrsig { type_covered, .. } => Some(type_covered),
        _ => None,
    }
}

```
