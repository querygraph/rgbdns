---
type: "code-fragment"
fragment_id: "rgbdns-frag-afabbed29de5"
source_path: "src/dnssec.rs"
code_note: "DNS from First Principles/Code/src/dnssec.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "materialize_file_with_policies"
kind: "fn"
start_line: 626
end_line: 696
---

# materialize_file_with_policies

- Fragment ID: `rgbdns-frag-afabbed29de5`
- Source file: [[DNS from First Principles/Code/src/dnssec.rs.source|src/dnssec.rs]]
- Lines: 626-696
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-afabbed29de5", "codeNote": "DNS from First Principles/Code/src/dnssec.rs.source", "sourcePath": "src/dnssec.rs", "startLine": 626, "endLine": 696}
```

## Excerpt

<span id="rgbdns-frag-afabbed29de5" class="rgbdns-fragment-target"></span>
### rgbdns-frag-afabbed29de5: fn materialize_file_with_policies

```rust
fn materialize_file_with_policies(
    input: &Path,
    policies: Option<&PolicySet>,
    output: &Path,
) -> Result<Option<u64>> {
    let zone = Zone::from_file(input)?;
    if let Some(policies) = policies {
        policies.validate_zone_coverage(&zone)?;
        if !policies
            .signing
            .iter()
            .any(|policy| zone.has_aname_in_zone(&policy.zone))
        {
            atomic_copy(input, output)?;
            return Ok(None);
        }
    }
    let (mut records, anames) = zone.materialization_input()?;
    let mut retained = Vec::new();
    let mut selected = Vec::new();
    for (owner, aname) in anames {
        if policies.is_none_or(|policies| policies.signs_owner(&owner)) {
            selected.push((owner, aname));
        } else {
            retained.push((owner, aname));
        }
    }
    let mut minimum_ttl = u32::MAX;
    if !selected.is_empty() {
        let resolver = crate::aname::Resolver::from_system()?;
        for (owner, aname) in selected {
            let mut addresses = Vec::new();
            for record_type in [RecordType::A, RecordType::Aaaa] {
                addresses.extend(resolver.resolve(
                    &owner,
                    &aname.target,
                    record_type,
                    aname.ttl,
                )?);
            }
            if addresses.is_empty() {
                return Err(Error::InvalidRecord(format!(
                    "ANAME target {} produced no addresses for {}",
                    aname.target, owner
                )));
            }
            minimum_ttl = minimum_ttl.min(addresses.iter().map(|record| record.ttl).min().unwrap());
            records.extend(addresses);
        }
    }
    let materialized = Zone::from_compiled_records(
        records
            .into_iter()
            .map(|record| (record, crate::zone::RecordMetadata::default()))
            .collect(),
        Vec::new(),
        retained,
    );
    let temporary = sibling_temporary(output);
    axfr::write_zone(&materialized, output, &temporary)?;
    sync_parent(output)?;
    if minimum_ttl == u32::MAX {
        return Ok(None);
    }
    let now = std::time::SystemTime::now()
        .duration_since(std::time::SystemTime::UNIX_EPOCH)
        .map_err(|_| Error::Format("system clock predates Unix epoch"))?
        .as_secs();
    Ok(Some(now.saturating_add(u64::from(minimum_ttl))))
}

```
