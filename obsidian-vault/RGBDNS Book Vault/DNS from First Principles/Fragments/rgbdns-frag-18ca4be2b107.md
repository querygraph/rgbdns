---
type: "code-fragment"
fragment_id: "rgbdns-frag-18ca4be2b107"
source_path: "src/zone.rs"
code_note: "DNS from First Principles/Code/src/zone.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "from_compiled_records"
kind: "fn"
start_line: 180
end_line: 218
---

# from_compiled_records

- Fragment ID: `rgbdns-frag-18ca4be2b107`
- Source file: [[DNS from First Principles/Code/src/zone.rs.source|src/zone.rs]]
- Lines: 180-218
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-18ca4be2b107", "codeNote": "DNS from First Principles/Code/src/zone.rs.source", "heading": "rgbdns-frag-18ca4be2b107: fn from_compiled_records", "sourcePath": "src/zone.rs", "startLine": 180, "endLine": 218}
```

## Excerpt

<span id="rgbdns-frag-18ca4be2b107" class="rgbdns-fragment-target"></span>
### rgbdns-frag-18ca4be2b107: fn from_compiled_records

```rust
    pub(crate) fn from_compiled_records(
        records: Vec<(Record, RecordMetadata)>,
        locations: Vec<(Vec<u8>, [u8; 2])>,
        anames: Vec<(Name, Aname)>,
    ) -> Self {
        let mut zone = Self {
            locations,
            anames: anames.into_iter().collect(),
            ..Self::default()
        };
        for (record, metadata) in records {
            zone.current_metadata = metadata;
            if record.rr_type() == RecordType::Soa {
                zone.authoritative.insert(record.name.clone());
            }
            zone.add(record);
        }
        zone.current_metadata = RecordMetadata::default();
        let ns_owners = zone
            .records
            .iter()
            .filter(|(_, records)| records.iter().any(|r| r.rr_type() == RecordType::Ns))
            .map(|(name, _)| name.clone())
            .collect::<Vec<_>>();
        for owner in ns_owners {
            if !zone.authoritative.contains(&owner) {
                zone.delegations.insert(owner);
            }
        }
        for owner in zone.anames.keys() {
            let mut node = Some(owner.clone());
            while let Some(name) = node {
                zone.nodes.insert(name.clone());
                zone.unqualified_nodes.insert(name.clone());
                node = name.parent();
            }
        }
        zone
    }
```
