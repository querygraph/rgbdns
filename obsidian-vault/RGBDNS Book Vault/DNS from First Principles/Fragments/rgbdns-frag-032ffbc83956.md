---
type: "code-fragment"
fragment_id: "rgbdns-frag-032ffbc83956"
source_path: "src/acme.rs"
code_note: "DNS from First Principles/Code/src/acme.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "signed_response"
kind: "fn"
start_line: 1010
end_line: 1051
---

# signed_response

- Fragment ID: `rgbdns-frag-032ffbc83956`
- Source file: [[DNS from First Principles/Code/src/acme.rs.source|src/acme.rs]]
- Lines: 1010-1051
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-032ffbc83956", "codeNote": "DNS from First Principles/Code/src/acme.rs.source", "sourcePath": "src/acme.rs", "startLine": 1010, "endLine": 1051}
```

## Excerpt

<span id="rgbdns-frag-032ffbc83956" class="rgbdns-fragment-target"></span>
### rgbdns-frag-032ffbc83956: fn signed_response

```rust
fn signed_response(
    request: &ParsedUpdate,
    policy: &Policy,
    rcode: u16,
    tsig_error: u16,
    other: &[u8],
) -> Result<Vec<u8>> {
    let now = unix_now();
    let mut wire = Vec::new();
    wire.extend_from_slice(&request.id.to_be_bytes());
    wire.extend_from_slice(&(0x8000 | OPCODE_UPDATE | (rcode & 0x0f)).to_be_bytes());
    wire.extend_from_slice(&[0, 0, 0, 0, 0, 0, 0, 1]);
    let mut mac_input = Vec::new();
    mac_input.extend_from_slice(&(request.tsig.mac.len() as u16).to_be_bytes());
    mac_input.extend_from_slice(&request.tsig.mac);
    mac_input.extend_from_slice(&wire[..10]);
    mac_input.extend_from_slice(&[0, 0]);
    append_tsig_variables(
        &mut mac_input,
        &policy.key_name,
        &request.tsig.algorithm,
        now,
        request.tsig.fudge.min(300),
        tsig_error,
        other,
    )?;
    let mac = hmac(&policy.secret, &mac_input)?;
    append_tsig_record(
        &mut wire,
        policy,
        &request.tsig.algorithm,
        now,
        request.tsig.fudge.min(300),
        &mac,
        request.id,
        tsig_error,
        other,
    )?;
    Ok(wire)
}

#[allow(clippy::too_many_arguments)]
```
