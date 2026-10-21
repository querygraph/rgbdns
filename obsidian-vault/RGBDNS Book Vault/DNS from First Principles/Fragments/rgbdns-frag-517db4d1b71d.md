---
type: "code-fragment"
fragment_id: "rgbdns-frag-517db4d1b71d"
source_path: "src/acme.rs"
code_note: "DNS from First Principles/Code/src/acme.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "is_hmac_sha256"
kind: "fn"
start_line: 1148
end_line: 1154
---

# is_hmac_sha256

- Fragment ID: `rgbdns-frag-517db4d1b71d`
- Source file: [[DNS from First Principles/Code/src/acme.rs.source|src/acme.rs]]
- Lines: 1148-1154
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-517db4d1b71d", "codeNote": "DNS from First Principles/Code/src/acme.rs.source", "sourcePath": "src/acme.rs", "startLine": 1148, "endLine": 1154}
```

## Excerpt

<span id="rgbdns-frag-517db4d1b71d" class="rgbdns-fragment-target"></span>
### rgbdns-frag-517db4d1b71d: fn is_hmac_sha256

```rust
fn is_hmac_sha256(name: &Name) -> bool {
    matches!(
        name.to_string().to_ascii_lowercase().as_str(),
        "hmac-sha256." | "hmac-sha256.sig-alg.reg.int."
    )
}

```
