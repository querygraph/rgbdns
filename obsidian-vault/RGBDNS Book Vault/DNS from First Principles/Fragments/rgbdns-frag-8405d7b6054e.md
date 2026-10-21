---
type: "code-fragment"
fragment_id: "rgbdns-frag-8405d7b6054e"
source_path: "src/acme.rs"
code_note: "DNS from First Principles/Code/src/acme.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "parse_tsig"
kind: "fn"
start_line: 869
end_line: 896
---

# parse_tsig

- Fragment ID: `rgbdns-frag-8405d7b6054e`
- Source file: [[DNS from First Principles/Code/src/acme.rs.source|src/acme.rs]]
- Lines: 869-896
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-8405d7b6054e", "codeNote": "DNS from First Principles/Code/src/acme.rs.source", "sourcePath": "src/acme.rs", "startLine": 869, "endLine": 896}
```

## Excerpt

<span id="rgbdns-frag-8405d7b6054e" class="rgbdns-fragment-target"></span>
### rgbdns-frag-8405d7b6054e: fn parse_tsig

```rust
fn parse_tsig(record: &WireRecord) -> Result<ParsedTsig> {
    let mut reader = Reader::new(&record.rdata)?;
    let algorithm = reader.name()?;
    let high = u64::from(reader.u16()?);
    let low = u64::from(reader.u32()?);
    let time_signed = (high << 32) | low;
    let fudge = reader.u16()?;
    let mac_length = usize::from(reader.u16()?);
    let mac = reader.bytes(mac_length)?.to_vec();
    let original_id = reader.u16()?;
    let error = reader.u16()?;
    let other_length = usize::from(reader.u16()?);
    let other = reader.bytes(other_length)?.to_vec();
    if reader.position != record.rdata.len() {
        return Err(Error::Format("trailing TSIG data"));
    }
    Ok(ParsedTsig {
        key_name: record.name.clone(),
        algorithm,
        time_signed,
        fudge,
        mac,
        original_id,
        error,
        other,
    })
}

```
