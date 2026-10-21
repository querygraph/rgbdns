---
type: "code-fragment"
fragment_id: "rgbdns-frag-72d8b138de21"
source_path: "src/acme.rs"
code_note: "DNS from First Principles/Code/src/acme.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "admin_wire"
kind: "fn"
start_line: 140
end_line: 182
---

# admin_wire

- Fragment ID: `rgbdns-frag-72d8b138de21`
- Source file: [[DNS from First Principles/Code/src/acme.rs.source|src/acme.rs]]
- Lines: 140-182
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-72d8b138de21", "codeNote": "DNS from First Principles/Code/src/acme.rs.source", "sourcePath": "src/acme.rs", "startLine": 140, "endLine": 182}
```

## Excerpt

<span id="rgbdns-frag-72d8b138de21" class="rgbdns-fragment-target"></span>
### rgbdns-frag-72d8b138de21: fn admin_wire

```rust
fn admin_wire(
    policy: &Policy,
    zone: &Name,
    owner: &Name,
    action: AdminAction<'_>,
) -> Result<Vec<u8>> {
    let id = random_id()?;
    let now = unix_now();
    let mut wire = Vec::new();
    wire.extend_from_slice(&id.to_be_bytes());
    wire.extend_from_slice(&OPCODE_UPDATE.to_be_bytes());
    wire.extend_from_slice(&[0, 1, 0, 0, 0, 1, 0, 0]);
    encode_name(&mut wire, zone)?;
    wire.extend_from_slice(&TYPE_SOA.to_be_bytes());
    wire.extend_from_slice(&CLASS_IN.to_be_bytes());
    encode_name(&mut wire, owner)?;
    wire.extend_from_slice(&TYPE_TXT.to_be_bytes());
    let (class, ttl, rdata) = match action {
        AdminAction::Present(value) => (CLASS_IN, policy.ttl, encode_txt(value)?),
        AdminAction::Cleanup(value) => (CLASS_NONE, 0, encode_txt(value)?),
        AdminAction::Clear => (CLASS_ANY, 0, Vec::new()),
    };
    wire.extend_from_slice(&class.to_be_bytes());
    wire.extend_from_slice(&ttl.to_be_bytes());
    wire.extend_from_slice(&(rdata.len() as u16).to_be_bytes());
    wire.extend_from_slice(&rdata);
    let algorithm = Name::from_str("hmac-sha256.")?;
    let mut mac_input = wire.clone();
    append_tsig_variables(
        &mut mac_input,
        &policy.key_name,
        &algorithm,
        now,
        300,
        0,
        &[],
    )?;
    let mac = hmac(&policy.secret, &mac_input)?;
    wire[10..12].copy_from_slice(&1u16.to_be_bytes());
    append_tsig_record(&mut wire, policy, &algorithm, now, 300, &mac, id, 0, &[])?;
    Ok(wire)
}

```
