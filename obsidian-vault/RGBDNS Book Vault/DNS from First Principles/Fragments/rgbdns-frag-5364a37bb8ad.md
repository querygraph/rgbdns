---
type: "code-fragment"
fragment_id: "rgbdns-frag-5364a37bb8ad"
source_path: "src/dnssec.rs"
code_note: "DNS from First Principles/Code/src/dnssec.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "check_file"
kind: "fn"
start_line: 422
end_line: 447
---

# check_file

- Fragment ID: `rgbdns-frag-5364a37bb8ad`
- Source file: [[DNS from First Principles/Code/src/dnssec.rs.source|src/dnssec.rs]]
- Lines: 422-447
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-5364a37bb8ad", "codeNote": "DNS from First Principles/Code/src/dnssec.rs.source", "sourcePath": "src/dnssec.rs", "startLine": 422, "endLine": 447}
```

## Excerpt

<span id="rgbdns-frag-5364a37bb8ad" class="rgbdns-fragment-target"></span>
### rgbdns-frag-5364a37bb8ad: fn check_file

```rust
pub fn check_file(input: &Path, policy_file: &Path) -> Result<Vec<CheckStatus>> {
    let policies = read_policies(policy_file)?;
    let zone = Zone::from_file(input)?;
    policies.validate_zone_coverage(&zone)?;
    for unsigned in &policies.unsigned {
        let records = zone
            .transfer(unsigned)
            .ok_or_else(|| Error::InvalidRecord(format!("{unsigned} is not authoritative")))?;
        if records.iter().any(|record| {
            matches!(
                record.rr_type(),
                RecordType::Dnskey | RecordType::Rrsig | RecordType::Nsec | RecordType::Unknown(50)
            )
        }) {
            return Err(Error::InvalidRecord(format!(
                "explicitly unsigned zone {unsigned} contains DNSSEC signing records"
            )));
        }
    }
    policies
        .signing
        .iter()
        .map(|policy| check_zone(&zone, policy))
        .collect()
}

```
