---
type: "code-fragment"
fragment_id: "rgbdns-frag-2a770356b45a"
source_path: "src/zone.rs"
code_note: "DNS from First Principles/Code/src/zone.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "with_acme_overlay"
kind: "fn"
start_line: 48
end_line: 99
---

# with_acme_overlay

- Fragment ID: `rgbdns-frag-2a770356b45a`
- Source file: [[DNS from First Principles/Code/src/zone.rs.source|src/zone.rs]]
- Lines: 48-99
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-2a770356b45a", "codeNote": "DNS from First Principles/Code/src/zone.rs.source", "sourcePath": "src/zone.rs", "startLine": 48, "endLine": 99}
```

## Excerpt

<span id="rgbdns-frag-2a770356b45a" class="rgbdns-fragment-target"></span>
### rgbdns-frag-2a770356b45a: fn with_acme_overlay

```rust
    pub(crate) fn with_acme_overlay(
        &self,
        records: &[(Name, Vec<u8>, u32)],
        serials: &BTreeMap<Name, u32>,
    ) -> Result<Self> {
        let mut zone = self.clone();
        for (owner, value, ttl) in records {
            if zone.records.get(owner).is_some_and(|existing| {
                existing
                    .iter()
                    .any(|record| record.rr_type() != RecordType::Txt)
            }) || zone.anames.contains_key(owner)
            {
                return Err(Error::InvalidRecord(format!(
                    "ACME TXT at {owner} conflicts with alias data"
                )));
            }
            zone.current_metadata = RecordMetadata::default();
            zone.add(Record {
                name: owner.clone(),
                ttl: *ttl,
                data: RData::Txt(value.chunks(255).map(<[u8]>::to_vec).collect()),
            });
        }
        for (name, serial) in serials {
            let Some(zone_records) = zone.records.get_mut(name) else {
                return Err(Error::InvalidRecord(format!(
                    "ACME policy zone {name} is not authoritative"
                )));
            };
            let Some(soa) = zone_records
                .iter_mut()
                .find(|record| record.rr_type() == RecordType::Soa)
            else {
                return Err(Error::InvalidRecord(format!(
                    "ACME policy zone {name} has no SOA"
                )));
            };
            if let RData::Soa {
                serial: current, ..
            } = &mut soa.data
            {
                // The stored ACME serial only moves forward from updates; a newer
                // source serial (an operator's edit) must still publish, or
                // secondaries never see the change.
                *current = newer_serial(*current, *serial);
            }
        }
        zone.validate_aliases()?;
        Ok(zone)
    }

```
