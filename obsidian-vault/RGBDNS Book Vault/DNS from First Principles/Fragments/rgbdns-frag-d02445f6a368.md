---
type: "code-fragment"
fragment_id: "rgbdns-frag-d02445f6a368"
source_path: "src/dnssec.rs"
code_note: "DNS from First Principles/Code/src/dnssec.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "check_zone"
kind: "fn"
start_line: 448
end_line: 528
---

# check_zone

- Fragment ID: `rgbdns-frag-d02445f6a368`
- Source file: [[DNS from First Principles/Code/src/dnssec.rs.source|src/dnssec.rs]]
- Lines: 448-528
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-d02445f6a368", "codeNote": "DNS from First Principles/Code/src/dnssec.rs.source", "sourcePath": "src/dnssec.rs", "startLine": 448, "endLine": 528}
```

## Excerpt

<span id="rgbdns-frag-d02445f6a368" class="rgbdns-fragment-target"></span>
### rgbdns-frag-d02445f6a368: fn check_zone

```rust
pub fn check_zone(zone: &Zone, policy: &Policy) -> Result<CheckStatus> {
    let mut records = zone
        .transfer(&policy.zone)
        .ok_or_else(|| Error::InvalidRecord(format!("{} is not authoritative", policy.zone)))?;
    records.pop();
    let serial = records
        .iter()
        .find_map(|record| match record.data {
            RData::Soa { serial, .. } if record.name == policy.zone => Some(serial),
            _ => None,
        })
        .ok_or(Error::Format("signed zone has no apex SOA"))?;
    check_nsec_chain(&records, &policy.zone)?;
    let hickory = records.iter().map(to_hickory).collect::<Result<Vec<_>>>()?;
    let keys = hickory
        .iter()
        .filter_map(|record| match &record.data {
            HickoryRData::DNSSEC(DNSSECRData::DNSKEY(key)) => Some(key),
            _ => None,
        })
        .collect::<Vec<_>>();
    if keys.is_empty() {
        return Err(Error::Format("signed zone has no DNSKEY"));
    }

    let now = std::time::SystemTime::now()
        .duration_since(std::time::SystemTime::UNIX_EPOCH)
        .map_err(|_| Error::Format("system clock predates Unix epoch"))?
        .as_secs();
    let mut signed_sets = BTreeSet::new();
    let mut earliest = None::<u32>;
    let mut reported_tag = None;
    for (custom, signature_record) in records.iter().zip(&hickory) {
        let HickoryRData::DNSSEC(DNSSECRData::RRSIG(signature)) = &signature_record.data else {
            continue;
        };
        let input = signature.input();
        let key = keys
            .iter()
            .find(|key| {
                key.algorithm() == input.algorithm
                    && key.calculate_key_tag().ok() == Some(input.key_tag)
            })
            .ok_or(Error::Format("RRSIG has no matching DNSKEY"))?;
        let rrset = hickory.iter().filter(|record| {
            record.name == signature_record.name && record.record_type() == input.type_covered
        });
        key.verify_rrsig(&signature_record.name, DNSClass::IN, signature, rrset)
            .map_err(|_| Error::Format("RRSIG cryptographic verification failed"))?;
        signed_sets.insert((
            custom.name.clone(),
            RecordType::from_code(input.type_covered.into()),
        ));
        let Some(DnssecRData::Rrsig {
            expiration,
            inception,
            ..
        }) = custom.data.dnssec()?
        else {
            return Err(Error::Format("RRSIG conversion mismatch"));
        };
        if u64::from(inception) > now || u64::from(expiration) <= now {
            return Err(Error::Format("RRSIG is outside its validity interval"));
        }
        earliest = Some(earliest.map_or(expiration, |value| value.min(expiration)));
        reported_tag = Some(input.key_tag);
    }
    for record in records
        .iter()
        .filter(|record| record.rr_type() != RecordType::Rrsig)
    {
        if !signed_sets.contains(&(record.name.clone(), record.rr_type())) {
            return Err(Error::Format("authoritative RRset lacks an RRSIG"));
        }
    }
    let earliest_expiration = earliest.ok_or(Error::Format("signed zone has no RRSIG"))?;
    let remaining = u64::from(earliest_expiration).saturating_sub(now);
    if remaining < policy.refresh {
        return Err(Error::Format(
            "DNSSEC signatures are inside the refresh window",
        ));
```
