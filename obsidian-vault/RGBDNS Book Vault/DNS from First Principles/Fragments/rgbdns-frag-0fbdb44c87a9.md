---
type: "code-fragment"
fragment_id: "rgbdns-frag-0fbdb44c87a9"
source_path: "src/dnssec.rs"
code_note: "DNS from First Principles/Code/src/dnssec.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "load_key"
kind: "fn"
start_line: 795
end_line: 810
---

# load_key

- Fragment ID: `rgbdns-frag-0fbdb44c87a9`
- Source file: [[DNS from First Principles/Code/src/dnssec.rs.source|src/dnssec.rs]]
- Lines: 795-810
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-0fbdb44c87a9", "codeNote": "DNS from First Principles/Code/src/dnssec.rs.source", "sourcePath": "src/dnssec.rs", "startLine": 795, "endLine": 810}
```

## Excerpt

<span id="rgbdns-frag-0fbdb44c87a9" class="rgbdns-fragment-target"></span>
### rgbdns-frag-0fbdb44c87a9: fn load_key

```rust
fn load_key(path: &Path) -> Result<EcdsaSigningKey> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        if fs::metadata(path)?.permissions().mode() & 0o077 != 0 {
            return Err(Error::InvalidRecord(format!(
                "DNSSEC key {} must not be accessible by group or other",
                path.display()
            )));
        }
    }
    let key_der = PrivatePkcs8KeyDer::from(fs::read(path)?);
    EcdsaSigningKey::from_pkcs8(&key_der, Algorithm::ECDSAP256SHA256)
        .map_err(|_| Error::Format("invalid DNSSEC PKCS#8 key"))
}

```
