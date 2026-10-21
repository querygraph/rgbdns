---
type: "code-fragment"
fragment_id: "rgbdns-frag-7e4b7d60965d"
source_path: "src/dnssec.rs"
code_note: "DNS from First Principles/Code/src/dnssec.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "sign_zone"
kind: "fn"
start_line: 711
end_line: 791
---

# sign_zone

- Fragment ID: `rgbdns-frag-7e4b7d60965d`
- Source file: [[DNS from First Principles/Code/src/dnssec.rs.source|src/dnssec.rs]]
- Lines: 711-791
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-7e4b7d60965d", "codeNote": "DNS from First Principles/Code/src/dnssec.rs.source", "sourcePath": "src/dnssec.rs", "startLine": 711, "endLine": 791}
```

## Excerpt

<span id="rgbdns-frag-7e4b7d60965d" class="rgbdns-fragment-target"></span>
### rgbdns-frag-7e4b7d60965d: fn sign_zone

```rust
pub fn sign_zone(zone: &Zone, policy: &Policy) -> Result<Vec<Record>> {
    if zone.has_aname_in_zone(&policy.zone) {
        return Err(Error::InvalidRecord(format!(
            "signed zone {} contains an unmaterialized ANAME",
            policy.zone
        )));
    }
    if zone.has_qualified_data_in_zone(&policy.zone) {
        return Err(Error::InvalidRecord(format!(
            "signed zone {} contains location-dependent or expiring data",
            policy.zone
        )));
    }
    let mut source = zone
        .transfer(&policy.zone)
        .ok_or_else(|| Error::InvalidRecord(format!("{} is not authoritative", policy.zone)))?;
    source.pop();
    if source.iter().any(|record| {
        matches!(
            record.rr_type(),
            RecordType::Dnskey | RecordType::Rrsig | RecordType::Nsec | RecordType::Unknown(50)
        )
    }) {
        return Err(Error::InvalidRecord(format!(
            "signed zone {} already contains DNSSEC records",
            policy.zone
        )));
    }

    // Hickory's signer increments the SOA before signing. Feed it the prior
    // serial so that the published serial remains owned by the input producer.
    if let Some(Record {
        data: RData::Soa { serial, .. },
        ..
    }) = source
        .iter_mut()
        .find(|record| record.name == policy.zone && record.rr_type() == RecordType::Soa)
    {
        *serial = serial.wrapping_sub(1);
    }

    let origin = HickoryName::from_str(&policy.zone.to_string())
        .map_err(|_| Error::Format("DNSSEC origin conversion failed"))?;
    let mut signed: InMemoryZoneHandler<SigningRuntime> = InMemoryZoneHandler::empty(
        origin.clone(),
        ZoneType::Primary,
        AxfrPolicy::Deny,
        Some(NxProofKind::Nsec),
    );
    for record in source {
        let record = to_hickory(&record)?;
        if !signed.upsert_mut(record, 0) {
            return Err(Error::Format("DNSSEC signer rejected a zone record"));
        }
    }

    let key = load_key(&policy.key_file)?;
    let dnskey = DNSKEY::from_key(
        &key.to_public_key()
            .map_err(|_| Error::Format("cannot derive DNSSEC public key"))?,
    );
    let signer = DnssecSigner::new(
        dnskey,
        Box::new(key),
        origin,
        Duration::from_secs(policy.validity),
    );
    signed
        .add_zone_signing_key_mut(signer)
        .map_err(|_| Error::Format("cannot install DNSSEC signing key"))?;
    SIGNING_SKEW.with(|skew| skew.set(policy.inception_skew));
    let result = signed.secure_zone_mut();
    SIGNING_SKEW.with(|skew| skew.set(0));
    result.map_err(|_| Error::Format("DNSSEC signing failed"))?;

    let mut output = Vec::new();
    for set in signed.records_get_mut().values() {
        for record in set.records(true) {
            output.push(from_hickory(record)?);
        }
    }
```
