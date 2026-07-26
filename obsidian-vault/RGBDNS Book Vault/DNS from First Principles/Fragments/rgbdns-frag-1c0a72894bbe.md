---
type: "code-fragment"
fragment_id: "rgbdns-frag-1c0a72894bbe"
source_path: "src/zone.rs"
code_note: "DNS from First Principles/Code/src/zone.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "lookup_for"
kind: "fn"
start_line: 535
end_line: 615
---

# lookup_for

- Fragment ID: `rgbdns-frag-1c0a72894bbe`
- Source file: [[DNS from First Principles/Code/src/zone.rs.source|src/zone.rs]]
- Lines: 535-615
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-1c0a72894bbe", "codeNote": "DNS from First Principles/Code/src/zone.rs.source", "heading": "rgbdns-frag-1c0a72894bbe: fn lookup_for", "sourcePath": "src/zone.rs", "startLine": 535, "endLine": 615}
```

## Excerpt

<span id="rgbdns-frag-1c0a72894bbe" class="rgbdns-fragment-target"></span>
### rgbdns-frag-1c0a72894bbe: fn lookup_for

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
            .filter(|z| name.is_subdomain_of(z))
            .max_by_key(|z| z.labels().count());
        if rows.is_empty() {
            return if let Some(zone) = zone {
                Lookup::NxDomain(self.soa(zone, location, now))
            } else {
                Lookup::Refused
            };
        }
        let mut answer: Vec<Record> = rows
            .into_iter()
            .filter(|r| {
                typ == RecordType::Any || r.rr_type() == typ || r.rr_type() == RecordType::Cname
            })
            .collect();
        for r in &mut answer {
            r.name = name.clone()
        }
```
