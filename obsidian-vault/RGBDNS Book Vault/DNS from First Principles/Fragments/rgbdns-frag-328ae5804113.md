---
type: "code-fragment"
fragment_id: "rgbdns-frag-328ae5804113"
source_path: "src/acme.rs"
code_note: "DNS from First Principles/Code/src/acme.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "parse_config_without_zone"
kind: "fn"
start_line: 450
end_line: 507
---

# parse_config_without_zone

- Fragment ID: `rgbdns-frag-328ae5804113`
- Source file: [[DNS from First Principles/Code/src/acme.rs.source|src/acme.rs]]
- Lines: 450-507
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-328ae5804113", "codeNote": "DNS from First Principles/Code/src/acme.rs.source", "sourcePath": "src/acme.rs", "startLine": 450, "endLine": 507}
```

## Excerpt

<span id="rgbdns-frag-328ae5804113" class="rgbdns-fragment-target"></span>
### rgbdns-frag-328ae5804113: fn parse_config_without_zone

```rust
fn parse_config_without_zone(text: &str) -> Result<BTreeMap<Name, Policy>> {
    let mut policies = BTreeMap::new();
    for (index, raw) in text.lines().enumerate() {
        let line = raw.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let fields = line.split_whitespace().collect::<Vec<_>>();
        if fields.len() != 6 {
            return Err(Error::InvalidRecord(format!(
                "ACME config line {} must have six fields",
                index + 1
            )));
        }
        let key_name = Name::from_str(fields[0])?;
        if !is_hmac_sha256(&Name::from_str(fields[1])?) || fields[4] != "_acme-challenge." {
            return Err(Error::InvalidRecord(format!(
                "unsupported ACME policy on line {}",
                index + 1
            )));
        }
        let secret = STANDARD
            .decode(fields[2])
            .map_err(|_| Error::InvalidRecord("invalid ACME TSIG secret".into()))?;
        if !(16..=64).contains(&secret.len()) {
            return Err(Error::InvalidRecord(
                "ACME TSIG secret must contain 16 through 64 bytes".into(),
            ));
        }
        let zone = Name::from_str(fields[3])?;
        let ttl = fields[5]
            .parse::<u32>()
            .map_err(|_| Error::InvalidRecord("invalid ACME TTL".into()))?;
        if !(30..=300).contains(&ttl) {
            return Err(Error::InvalidRecord(
                "ACME TTL must be 30 through 300".into(),
            ));
        }
        let policy = Policy {
            key_name: key_name.clone(),
            secret,
            zone,
            ttl,
        };
        if policies.insert(key_name, policy).is_some() || policies.len() > MAX_KEYS {
            return Err(Error::InvalidRecord(
                "duplicate or excessive ACME update keys".into(),
            ));
        }
    }
    if policies.is_empty() {
        return Err(Error::InvalidRecord(
            "ACME update config has no keys".into(),
        ));
    }
    Ok(policies)
}

```
