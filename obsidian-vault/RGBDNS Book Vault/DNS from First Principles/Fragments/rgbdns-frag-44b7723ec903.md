---
type: "code-fragment"
fragment_id: "rgbdns-frag-44b7723ec903"
source_path: "src/acme.rs"
code_note: "DNS from First Principles/Code/src/acme.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "validate_published"
kind: "fn"
start_line: 402
end_line: 436
---

# validate_published

- Fragment ID: `rgbdns-frag-44b7723ec903`
- Source file: [[DNS from First Principles/Code/src/acme.rs.source|src/acme.rs]]
- Lines: 402-436
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-44b7723ec903", "codeNote": "DNS from First Principles/Code/src/acme.rs.source", "sourcePath": "src/acme.rs", "startLine": 402, "endLine": 436}
```

## Excerpt

<span id="rgbdns-frag-44b7723ec903" class="rgbdns-fragment-target"></span>
### rgbdns-frag-44b7723ec903: fn validate_published

```rust
fn validate_published(
    zone: &Zone,
    overlay: &Overlay,
    serials: &BTreeMap<Name, u32>,
    policies: &BTreeMap<Name, Policy>,
) -> Result<()> {
    for policy in policies.values() {
        if !zone.is_dnssec_signed(&policy.zone) {
            return Err(Error::Format("ACME publication produced an unsigned zone"));
        }
        if serials
            .get(&policy.zone)
            .is_some_and(|serial| zone.soa_serial(&policy.zone) != Some(*serial))
        {
            return Err(Error::Format("ACME publication has the wrong SOA serial"));
        }
    }
    for (owner, values) in overlay {
        let found = match zone.lookup(owner, crate::RecordType::Txt) {
            crate::zone::Lookup::Answer(records) => records
                .into_iter()
                .filter_map(|record| match record.data {
                    crate::RData::Txt(chunks) => Some(chunks.into_iter().flatten().collect()),
                    _ => None,
                })
                .collect::<BTreeSet<Vec<u8>>>(),
            _ => BTreeSet::new(),
        };
        if !values.is_subset(&found) {
            return Err(Error::Format("ACME publication omitted challenge data"));
        }
    }
    Ok(())
}

```
