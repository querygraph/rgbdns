---
type: "code-fragment"
fragment_id: "rgbdns-frag-773b5d6dafa9"
source_path: "src/acme.rs"
code_note: "DNS from First Principles/Code/src/acme.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "hmac"
kind: "fn"
start_line: 1131
end_line: 1137
---

# hmac

- Fragment ID: `rgbdns-frag-773b5d6dafa9`
- Source file: [[DNS from First Principles/Code/src/acme.rs.source|src/acme.rs]]
- Lines: 1131-1137
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-773b5d6dafa9", "codeNote": "DNS from First Principles/Code/src/acme.rs.source", "sourcePath": "src/acme.rs", "startLine": 1131, "endLine": 1137}
```

## Excerpt

<span id="rgbdns-frag-773b5d6dafa9" class="rgbdns-fragment-target"></span>
### rgbdns-frag-773b5d6dafa9: fn hmac

```rust
fn hmac(secret: &[u8], input: &[u8]) -> Result<Vec<u8>> {
    let mut mac =
        HmacSha256::new_from_slice(secret).map_err(|_| Error::Format("invalid TSIG key length"))?;
    mac.update(input);
    Ok(mac.finalize().into_bytes().to_vec())
}

```
