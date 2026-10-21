---
type: "code-fragment"
fragment_id: "rgbdns-frag-ef74981da6e2"
source_path: "src/dnssec.rs"
code_note: "DNS from First Principles/Code/src/dnssec.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "ds_line"
kind: "fn"
start_line: 397
end_line: 421
---

# ds_line

- Fragment ID: `rgbdns-frag-ef74981da6e2`
- Source file: [[DNS from First Principles/Code/src/dnssec.rs.source|src/dnssec.rs]]
- Lines: 397-421
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-ef74981da6e2", "codeNote": "DNS from First Principles/Code/src/dnssec.rs.source", "sourcePath": "src/dnssec.rs", "startLine": 397, "endLine": 421}
```

## Excerpt

<span id="rgbdns-frag-ef74981da6e2" class="rgbdns-fragment-target"></span>
### rgbdns-frag-ef74981da6e2: fn ds_line

```rust
pub fn ds_line(policy: &Policy) -> Result<String> {
    let key = load_key(&policy.key_file)?;
    let public = key
        .to_public_key()
        .map_err(|_| Error::Format("cannot derive DNSSEC public key"))?;
    let dnskey = DNSKEY::from_key(&public);
    let origin = HickoryName::from_str(&policy.zone.to_string())
        .map_err(|_| Error::Format("DNSSEC origin conversion failed"))?;
    let key_tag = dnskey
        .calculate_key_tag()
        .map_err(|_| Error::Format("cannot calculate DNSSEC key tag"))?;
    let digest = dnskey
        .to_digest(&origin, DigestType::SHA256)
        .map_err(|_| Error::Format("cannot calculate DNSSEC DS digest"))?;
    let hexadecimal = digest
        .as_ref()
        .iter()
        .map(|byte| format!("{byte:02X}"))
        .collect::<String>();
    Ok(format!(
        "{} IN DS {key_tag} {} 2 {hexadecimal}",
        policy.zone, ALGORITHM
    ))
}

```
