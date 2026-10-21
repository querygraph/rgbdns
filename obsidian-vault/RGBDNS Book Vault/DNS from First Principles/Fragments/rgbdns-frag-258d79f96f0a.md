---
type: "code-fragment"
fragment_id: "rgbdns-frag-258d79f96f0a"
source_path: "src/acme.rs"
code_note: "DNS from First Principles/Code/src/acme.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "parse_update"
kind: "fn"
start_line: 813
end_line: 868
---

# parse_update

- Fragment ID: `rgbdns-frag-258d79f96f0a`
- Source file: [[DNS from First Principles/Code/src/acme.rs.source|src/acme.rs]]
- Lines: 813-868
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-258d79f96f0a", "codeNote": "DNS from First Principles/Code/src/acme.rs.source", "sourcePath": "src/acme.rs", "startLine": 813, "endLine": 868}
```

## Excerpt

<span id="rgbdns-frag-258d79f96f0a" class="rgbdns-fragment-target"></span>
### rgbdns-frag-258d79f96f0a: fn parse_update

```rust
fn parse_update(wire: &[u8]) -> Result<ParsedUpdate> {
    let mut reader = Reader::new(wire)?;
    let id = reader.u16()?;
    let flags = reader.u16()?;
    let qd = reader.u16()?;
    let an = reader.u16()?;
    let ns = reader.u16()?;
    let ar = reader.u16()?;
    if qd != 1 || ar == 0 || an > 64 || ns > 64 || ar > 16 {
        return Err(Error::Format("invalid UPDATE section count"));
    }
    let zone = reader.name()?;
    if reader.u16()? != TYPE_SOA || reader.u16()? != CLASS_IN {
        return Err(Error::Format("invalid UPDATE zone section"));
    }
    let prerequisites = (0..an)
        .map(|_| reader.record())
        .collect::<Result<Vec<_>>>()?;
    let updates = (0..ns)
        .map(|_| reader.record())
        .collect::<Result<Vec<_>>>()?;
    let mut additionals = Vec::new();
    let mut tsig_start = None;
    for index in 0..ar {
        let start = reader.position;
        let record = reader.record()?;
        if record.typ == TYPE_TSIG {
            if index + 1 != ar || tsig_start.is_some() {
                return Err(Error::Format("TSIG must be last"));
            }
            tsig_start = Some(start);
        }
        additionals.push(record);
    }
    if reader.position != wire.len() {
        return Err(Error::Format("trailing UPDATE data"));
    }
    let tsig_record = additionals
        .last()
        .filter(|record| record.typ == TYPE_TSIG && record.class == CLASS_ANY && record.ttl == 0)
        .ok_or(Error::Format("missing TSIG"))?;
    let tsig = parse_tsig(tsig_record)?;
    let start = tsig_start.ok_or(Error::Format("missing TSIG"))?;
    let mut unsigned = wire[..start].to_vec();
    unsigned[10..12].copy_from_slice(&(ar - 1).to_be_bytes());
    Ok(ParsedUpdate {
        id,
        flags,
        zone,
        prerequisites,
        updates,
        tsig,
        unsigned,
    })
}

```
