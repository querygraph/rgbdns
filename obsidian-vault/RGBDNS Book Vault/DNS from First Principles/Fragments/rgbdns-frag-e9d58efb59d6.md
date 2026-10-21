---
type: "code-fragment"
fragment_id: "rgbdns-frag-e9d58efb59d6"
source_path: "src/dnssec.rs"
code_note: "DNS from First Principles/Code/src/dnssec.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "build_snapshot"
kind: "fn"
start_line: 371
end_line: 396
---

# build_snapshot

- Fragment ID: `rgbdns-frag-e9d58efb59d6`
- Source file: [[DNS from First Principles/Code/src/dnssec.rs.source|src/dnssec.rs]]
- Lines: 371-396
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-e9d58efb59d6", "codeNote": "DNS from First Principles/Code/src/dnssec.rs.source", "sourcePath": "src/dnssec.rs", "startLine": 371, "endLine": 396}
```

## Excerpt

<span id="rgbdns-frag-e9d58efb59d6" class="rgbdns-fragment-target"></span>
### rgbdns-frag-e9d58efb59d6: fn build_snapshot

```rust
fn build_snapshot(zone: &Zone, policies: &PolicySet) -> Result<Zone> {
    policies.validate_zone_coverage(zone)?;
    let mut records = zone
        .record_entries()
        .filter(|(record, _)| !policies.signs_owner(&record.name))
        .map(|(record, metadata)| (record.clone(), metadata))
        .collect::<Vec<_>>();
    for policy in &policies.signing {
        records.extend(
            sign_zone(zone, policy)?
                .into_iter()
                .map(|record| (record, crate::zone::RecordMetadata::default())),
        );
    }
    Ok(Zone::from_compiled_records(
        records,
        zone.location_entries()
            .map(|(prefix, location)| (prefix.to_vec(), location))
            .collect(),
        zone.aname_entries()
            .filter(|(owner, _)| !policies.signs_owner(owner))
            .map(|(owner, aname)| (owner.clone(), aname.clone()))
            .collect(),
    ))
}

```
