---
type: "code-fragment"
fragment_id: "rgbdns-frag-6e877f2c2a4e"
source_path: "src/zone.rs"
code_note: "DNS from First Principles/Code/src/zone.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "dnssec_nsec_at"
kind: "fn"
start_line: 891
end_line: 908
---

# dnssec_nsec_at

- Fragment ID: `rgbdns-frag-6e877f2c2a4e`
- Source file: [[DNS from First Principles/Code/src/zone.rs.source|src/zone.rs]]
- Lines: 891-908
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-6e877f2c2a4e", "codeNote": "DNS from First Principles/Code/src/zone.rs.source", "sourcePath": "src/zone.rs", "startLine": 891, "endLine": 908}
```

## Excerpt

<span id="rgbdns-frag-6e877f2c2a4e" class="rgbdns-fragment-target"></span>
### rgbdns-frag-6e877f2c2a4e: fn dnssec_nsec_at

```rust
    pub(crate) fn dnssec_nsec_at(&self, name: &Name, client: Option<IpAddr>) -> Vec<Record> {
        let location = self.client_location(client);
        let now = 4_611_686_018_427_387_914u64.saturating_add(unix_now());
        let mut records = self
            .visible_records(name, location, now)
            .into_iter()
            .filter(|record| record.rr_type() == RecordType::Nsec)
            .collect::<Vec<_>>();
        if !records.is_empty() {
            records.extend(self.dnssec_signatures(
                name,
                &BTreeSet::from([RecordType::Nsec]),
                client,
            ));
        }
        records
    }

```
