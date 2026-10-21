---
type: "code-fragment"
fragment_id: "rgbdns-frag-11a440e2eae5"
source_path: "src/zone.rs"
code_note: "DNS from First Principles/Code/src/zone.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "dnssec_nsec_zone"
kind: "fn"
start_line: 909
end_line: 944
---

# dnssec_nsec_zone

- Fragment ID: `rgbdns-frag-11a440e2eae5`
- Source file: [[DNS from First Principles/Code/src/zone.rs.source|src/zone.rs]]
- Lines: 909-944
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-11a440e2eae5", "codeNote": "DNS from First Principles/Code/src/zone.rs.source", "sourcePath": "src/zone.rs", "startLine": 909, "endLine": 944}
```

## Excerpt

<span id="rgbdns-frag-11a440e2eae5" class="rgbdns-fragment-target"></span>
### rgbdns-frag-11a440e2eae5: fn dnssec_nsec_zone

```rust
    pub(crate) fn dnssec_nsec_zone(&self, name: &Name, client: Option<IpAddr>) -> Vec<Record> {
        let location = self.client_location(client);
        let now = 4_611_686_018_427_387_914u64.saturating_add(unix_now());
        let Some(zone) = self
            .authoritative
            .iter()
            .filter(|zone| name.is_subdomain_of(zone))
            .max_by_key(|zone| zone.labels().count())
        else {
            return Vec::new();
        };
        let mut output = Vec::new();
        for owner in self.records.keys().filter(|owner| {
            owner.is_subdomain_of(zone)
                && !self.authoritative.iter().any(|child| {
                    child != zone && child.is_subdomain_of(zone) && owner.is_subdomain_of(child)
                })
        }) {
            let nsec = self
                .visible_records(owner, location, now)
                .into_iter()
                .filter(|record| record.rr_type() == RecordType::Nsec)
                .collect::<Vec<_>>();
            if !nsec.is_empty() {
                output.extend(nsec);
                output.extend(self.dnssec_signatures(
                    owner,
                    &BTreeSet::from([RecordType::Nsec]),
                    client,
                ));
            }
        }
        output
    }
}

```
