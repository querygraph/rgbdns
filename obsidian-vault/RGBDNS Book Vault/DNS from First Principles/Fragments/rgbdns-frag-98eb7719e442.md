---
type: "code-fragment"
fragment_id: "rgbdns-frag-98eb7719e442"
source_path: "src/dnssec.rs"
code_note: "DNS from First Principles/Code/src/dnssec.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "generate_key"
kind: "fn"
start_line: 295
end_line: 322
---

# generate_key

- Fragment ID: `rgbdns-frag-98eb7719e442`
- Source file: [[DNS from First Principles/Code/src/dnssec.rs.source|src/dnssec.rs]]
- Lines: 295-322
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-98eb7719e442", "codeNote": "DNS from First Principles/Code/src/dnssec.rs.source", "sourcePath": "src/dnssec.rs", "startLine": 295, "endLine": 322}
```

## Excerpt

<span id="rgbdns-frag-98eb7719e442" class="rgbdns-fragment-target"></span>
### rgbdns-frag-98eb7719e442: fn generate_key

```rust
pub fn generate_key(zone: &Name, path: &Path) -> Result<Policy> {
    use std::os::unix::fs::OpenOptionsExt;

    let key = EcdsaSigningKey::generate_pkcs8(Algorithm::ECDSAP256SHA256)
        .map_err(|_| Error::Format("DNSSEC key generation failed"))?;
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(0o600)
        .open(path)?;
    if let Err(error) = file
        .write_all(key.secret_pkcs8_der())
        .and_then(|()| file.sync_all())
    {
        drop(file);
        let _ = fs::remove_file(path);
        return Err(error.into());
    }
    Ok(Policy {
        zone: zone.clone(),
        key_file: path.to_path_buf(),
        validity: DEFAULT_VALIDITY,
        refresh: DEFAULT_REFRESH,
        inception_skew: DEFAULT_INCEPTION_SKEW,
    })
}

#[cfg(not(unix))]
```
