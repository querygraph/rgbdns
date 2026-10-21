---
type: "code-fragment"
fragment_id: "rgbdns-frag-0a5c23593f04"
source_path: "src/acme.rs"
code_note: "DNS from First Principles/Code/src/acme.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "request_mac"
kind: "fn"
start_line: 996
end_line: 1009
---

# request_mac

- Fragment ID: `rgbdns-frag-0a5c23593f04`
- Source file: [[DNS from First Principles/Code/src/acme.rs.source|src/acme.rs]]
- Lines: 996-1009
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-0a5c23593f04", "codeNote": "DNS from First Principles/Code/src/acme.rs.source", "sourcePath": "src/acme.rs", "startLine": 996, "endLine": 1009}
```

## Excerpt

<span id="rgbdns-frag-0a5c23593f04" class="rgbdns-fragment-target"></span>
### rgbdns-frag-0a5c23593f04: fn request_mac

```rust
fn request_mac(message: &ParsedUpdate, policy: &Policy) -> Result<Vec<u8>> {
    let mut input = message.unsigned.clone();
    append_tsig_variables(
        &mut input,
        &message.tsig.key_name,
        &message.tsig.algorithm,
        message.tsig.time_signed,
        message.tsig.fudge,
        message.tsig.error,
        &message.tsig.other,
    )?;
    hmac(&policy.secret, &input)
}

```
