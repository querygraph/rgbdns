---
type: "code-fragment"
fragment_id: "rgbdns-frag-636ad5773610"
source_path: "src/zone.rs"
code_note: "DNS from First Principles/Code/src/zone.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "is_dnssec_signed"
kind: "fn"
start_line: 113
end_line: 119
---

# is_dnssec_signed

- Fragment ID: `rgbdns-frag-636ad5773610`
- Source file: [[DNS from First Principles/Code/src/zone.rs.source|src/zone.rs]]
- Lines: 113-119
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-636ad5773610", "codeNote": "DNS from First Principles/Code/src/zone.rs.source", "sourcePath": "src/zone.rs", "startLine": 113, "endLine": 119}
```

## Excerpt

<span id="rgbdns-frag-636ad5773610" class="rgbdns-fragment-target"></span>
### rgbdns-frag-636ad5773610: fn is_dnssec_signed

```rust
    pub(crate) fn is_dnssec_signed(&self, zone: &Name) -> bool {
        self.records.get(zone).is_some_and(|records| {
            [RecordType::Dnskey, RecordType::Nsec, RecordType::Rrsig]
                .iter()
                .all(|typ| records.iter().any(|record| record.rr_type() == *typ))
        })
    }
```
