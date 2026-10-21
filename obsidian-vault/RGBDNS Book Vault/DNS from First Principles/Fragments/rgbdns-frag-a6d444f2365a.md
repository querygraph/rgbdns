---
type: "code-fragment"
fragment_id: "rgbdns-frag-a6d444f2365a"
source_path: "src/dnssec.rs"
code_note: "DNS from First Principles/Code/src/dnssec.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "check_nsec_chain"
kind: "fn"
start_line: 539
end_line: 595
---

# check_nsec_chain

- Fragment ID: `rgbdns-frag-a6d444f2365a`
- Source file: [[DNS from First Principles/Code/src/dnssec.rs.source|src/dnssec.rs]]
- Lines: 539-595
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-a6d444f2365a", "codeNote": "DNS from First Principles/Code/src/dnssec.rs.source", "sourcePath": "src/dnssec.rs", "startLine": 539, "endLine": 595}
```

## Excerpt

<span id="rgbdns-frag-a6d444f2365a" class="rgbdns-fragment-target"></span>
### rgbdns-frag-a6d444f2365a: fn check_nsec_chain

```rust
fn check_nsec_chain(records: &[Record], apex: &Name) -> Result<()> {
    let mut chain = BTreeMap::new();
    let cuts = records
        .iter()
        .filter(|record| {
            record.name != *apex && matches!(record.data, RData::Name(RecordType::Ns, _))
        })
        .map(|record| record.name.clone())
        .collect::<BTreeSet<_>>();
    let authoritative_owners = records
        .iter()
        .filter(|record| {
            !cuts
                .iter()
                .any(|cut| record.name != *cut && record.name.is_subdomain_of(cut))
        })
        .map(|record| record.name.clone())
        .collect::<BTreeSet<_>>();
    for record in records {
        let Some(DnssecRData::Nsec { next, types }) = record.data.dnssec()? else {
            continue;
        };
        if !types.contains(&RecordType::Nsec) || !types.contains(&RecordType::Rrsig) {
            return Err(Error::Format("NSEC bitmap omits NSEC or RRSIG"));
        }
        if chain.insert(record.name.clone(), next).is_some() {
            return Err(Error::Format("NSEC owner appears more than once"));
        }
        let expected_types = records
            .iter()
            .filter(|candidate| candidate.name == record.name)
            .map(Record::rr_type)
            .collect::<BTreeSet<_>>();
        if types.into_iter().collect::<BTreeSet<_>>() != expected_types {
            return Err(Error::Format("NSEC bitmap does not match its owner RRsets"));
        }
    }
    if chain.is_empty() {
        return Err(Error::Format("signed zone has no NSEC chain"));
    }
    if !chain.contains_key(apex) {
        return Err(Error::Format("NSEC chain does not contain the zone apex"));
    }
    if !authoritative_owners.is_subset(&chain.keys().cloned().collect()) {
        return Err(Error::Format("NSEC chain omits an authoritative owner"));
    }
    let mut owners = chain.keys().cloned().collect::<Vec<_>>();
    owners.sort_by(canonical_name_cmp);
    for (index, owner) in owners.iter().enumerate() {
        let expected = &owners[(index + 1) % owners.len()];
        if chain.get(owner) != Some(expected) {
            return Err(Error::Format("NSEC chain is not in canonical name order"));
        }
    }
    Ok(())
}

```
