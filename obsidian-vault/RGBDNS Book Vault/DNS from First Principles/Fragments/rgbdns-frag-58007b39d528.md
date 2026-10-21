---
type: "code-fragment"
fragment_id: "rgbdns-frag-58007b39d528"
source_path: "src/zone.rs"
code_note: "DNS from First Principles/Code/src/zone.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "lookup_for"
kind: "fn"
start_line: 688
end_line: 768
---

# lookup_for

- Fragment ID: `rgbdns-frag-58007b39d528`
- Source file: [[DNS from First Principles/Code/src/zone.rs.source|src/zone.rs]]
- Lines: 688-768
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-58007b39d528", "codeNote": "DNS from First Principles/Code/src/zone.rs.source", "sourcePath": "src/zone.rs", "startLine": 688, "endLine": 768}
```

## Excerpt

<span id="rgbdns-frag-58007b39d528" class="rgbdns-fragment-target"></span>
### rgbdns-frag-58007b39d528: fn lookup_for

```rust
    fn lookup_for(
        &self,
        name: &Name,
        typ: RecordType,
        client: Option<IpAddr>,
        unix_seconds: u64,
    ) -> Lookup {
        let location = self.client_location(client);
        let now = 4_611_686_018_427_387_914u64.saturating_add(unix_seconds);
        if let Some(delegation) = self
            .delegations
            .iter()
            .filter(|owner| name.is_subdomain_of(owner))
            .max_by_key(|owner| owner.labels().count())
        {
            if typ == RecordType::Ds && name == delegation {
                let parent = self
                    .authoritative
                    .iter()
                    .filter(|zone| delegation.is_subdomain_of(zone))
                    .max_by_key(|zone| zone.labels().count());
                if parent.is_some_and(|zone| self.is_dnssec_signed(zone)) {
                    let answer = self
                        .visible_records(delegation, location, now)
                        .into_iter()
                        .filter(|record| record.rr_type() == RecordType::Ds)
                        .collect::<Vec<_>>();
                    if !answer.is_empty() {
                        return Lookup::Answer(answer);
                    }
                    return Lookup::NoData(parent.and_then(|zone| self.soa(zone, location, now)));
                }
            }
            let authorities = self
                .visible_records(delegation, location, now)
                .into_iter()
                .filter(|record| record.rr_type() == RecordType::Ns)
                .collect::<Vec<_>>();
            let mut additionals = Vec::new();
            for authority in &authorities {
                let RData::Name(RecordType::Ns, target) = &authority.data else {
                    continue;
                };
                if !target.is_subdomain_of(delegation) {
                    continue;
                }
                additionals.extend(
                    self.visible_records(target, location, now)
                        .into_iter()
                        .filter(|record| {
                            matches!(record.rr_type(), RecordType::A | RecordType::Aaaa)
                        }),
                );
            }
            return Lookup::Referral {
                authorities,
                additionals,
            };
        }
        let mut rows = self.visible_records(name, location, now);
        if rows.is_empty() {
            if self.name_exists(name, location, now) {
                let zone = self
                    .authoritative
                    .iter()
                    .filter(|z| name.is_subdomain_of(z))
                    .max_by_key(|z| z.labels().count());
                return Lookup::NoData(zone.and_then(|z| self.soa(z, location, now)));
            }
            let mut p = name.parent();
            while let Some(n) = p {
                if self.name_exists(&n, location, now) {
                    rows = self.visible_records(&n.wildcard(), location, now);
                    break;
                }
                p = n.parent()
            }
        }
        let zone = self
            .authoritative
            .iter()
```
