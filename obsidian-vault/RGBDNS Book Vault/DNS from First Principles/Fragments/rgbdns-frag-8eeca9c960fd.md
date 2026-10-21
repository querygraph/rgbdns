---
type: "code-fragment"
fragment_id: "rgbdns-frag-8eeca9c960fd"
source_path: "src/acme.rs"
code_note: "DNS from First Principles/Code/src/acme.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "append_tsig_record"
kind: "fn"
start_line: 1052
end_line: 1081
---

# append_tsig_record

- Fragment ID: `rgbdns-frag-8eeca9c960fd`
- Source file: [[DNS from First Principles/Code/src/acme.rs.source|src/acme.rs]]
- Lines: 1052-1081
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-8eeca9c960fd", "codeNote": "DNS from First Principles/Code/src/acme.rs.source", "sourcePath": "src/acme.rs", "startLine": 1052, "endLine": 1081}
```

## Excerpt

<span id="rgbdns-frag-8eeca9c960fd" class="rgbdns-fragment-target"></span>
### rgbdns-frag-8eeca9c960fd: fn append_tsig_record

```rust
fn append_tsig_record(
    wire: &mut Vec<u8>,
    policy: &Policy,
    algorithm: &Name,
    time: u64,
    fudge: u16,
    mac: &[u8],
    original_id: u16,
    error: u16,
    other: &[u8],
) -> Result<()> {
    encode_name(wire, &policy.key_name)?;
    wire.extend_from_slice(&TYPE_TSIG.to_be_bytes());
    wire.extend_from_slice(&CLASS_ANY.to_be_bytes());
    wire.extend_from_slice(&0u32.to_be_bytes());
    let mut rdata = Vec::new();
    encode_name(&mut rdata, algorithm)?;
    rdata.extend_from_slice(&time.to_be_bytes()[2..]);
    rdata.extend_from_slice(&fudge.to_be_bytes());
    rdata.extend_from_slice(&(mac.len() as u16).to_be_bytes());
    rdata.extend_from_slice(mac);
    rdata.extend_from_slice(&original_id.to_be_bytes());
    rdata.extend_from_slice(&error.to_be_bytes());
    rdata.extend_from_slice(&(other.len() as u16).to_be_bytes());
    rdata.extend_from_slice(other);
    wire.extend_from_slice(&(rdata.len() as u16).to_be_bytes());
    wire.extend_from_slice(&rdata);
    Ok(())
}

```
