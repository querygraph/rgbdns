---
type: "code-fragment"
fragment_id: "rgbdns-frag-7fc0ca2de20a"
source_path: "src/dnssec.rs"
code_note: "DNS from First Principles/Code/src/dnssec.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "read_policies"
kind: "fn"
start_line: 248
end_line: 294
---

# read_policies

- Fragment ID: `rgbdns-frag-7fc0ca2de20a`
- Source file: [[DNS from First Principles/Code/src/dnssec.rs.source|src/dnssec.rs]]
- Lines: 248-294
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-7fc0ca2de20a", "codeNote": "DNS from First Principles/Code/src/dnssec.rs.source", "sourcePath": "src/dnssec.rs", "startLine": 248, "endLine": 294}
```

## Excerpt

<span id="rgbdns-frag-7fc0ca2de20a" class="rgbdns-fragment-target"></span>
### rgbdns-frag-7fc0ca2de20a: fn read_policies

```rust
pub fn read_policies(path: impl AsRef<Path>) -> Result<PolicySet> {
    let text = fs::read_to_string(path)?;
    let mut signing = Vec::new();
    let mut unsigned = BTreeSet::new();
    let mut covered = BTreeSet::new();
    for (index, raw) in text.lines().enumerate() {
        let line = raw.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let (zone, policy) = if let Some(name) = line.strip_prefix('U') {
            if name.is_empty() || name.contains(':') {
                return Err(Error::InvalidRecord(format!(
                    "DNSSEC policy line {}: unsigned policy is Uzone",
                    index + 1
                )));
            }
            let zone = name.parse::<Name>().map_err(|error| {
                Error::InvalidRecord(format!("DNSSEC policy line {}: {error}", index + 1))
            })?;
            (zone, None)
        } else {
            let policy = Policy::parse(line).map_err(|error| {
                Error::InvalidRecord(format!("DNSSEC policy line {}: {error}", index + 1))
            })?;
            (policy.zone.clone(), Some(policy))
        };
        if !covered.insert(zone.clone()) {
            return Err(Error::InvalidRecord(format!(
                "multiple DNSSEC dispositions for {zone} are not supported"
            )));
        }
        if let Some(policy) = policy {
            signing.push(policy);
        } else {
            unsigned.insert(zone);
        }
    }
    if signing.is_empty() {
        return Err(Error::InvalidRecord(
            "DNSSEC policy has no signed zones".into(),
        ));
    }
    Ok(PolicySet { signing, unsigned })
}

#[cfg(unix)]
```
