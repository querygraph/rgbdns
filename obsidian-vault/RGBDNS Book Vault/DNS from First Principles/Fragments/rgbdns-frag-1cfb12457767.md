---
type: "code-fragment"
fragment_id: "rgbdns-frag-1cfb12457767"
source_path: "src/cdb.rs"
code_note: "DNS from First Principles/Code/src/cdb.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "decode_name"
kind: "fn"
start_line: 238
end_line: 264
---

# decode_name

- Fragment ID: `rgbdns-frag-1cfb12457767`
- Source file: [[DNS from First Principles/Code/src/cdb.rs.source|src/cdb.rs]]
- Lines: 238-264
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-1cfb12457767", "codeNote": "DNS from First Principles/Code/src/cdb.rs.source", "heading": "rgbdns-frag-1cfb12457767: fn decode_name", "sourcePath": "src/cdb.rs", "startLine": 238, "endLine": 264}
```

## Excerpt

<span id="rgbdns-frag-1cfb12457767" class="rgbdns-fragment-target"></span>
### rgbdns-frag-1cfb12457767: fn decode_name

```rust
pub(crate) fn decode_name(wire: &[u8]) -> Result<Name> {
    let mut labels = Vec::new();
    let mut position = 0;
    loop {
        let length = *wire
            .get(position)
            .ok_or(Error::Format("truncated CDB owner"))? as usize;
        position += 1;
        if length == 0 {
            if position != wire.len() {
                return Err(Error::Format("trailing CDB owner data"));
            }
            break;
        }
        if length > 63 {
            return Err(Error::Format("invalid CDB owner label"));
        }
        let end = position
            .checked_add(length)
            .filter(|end| *end <= wire.len())
            .ok_or(Error::Format("truncated CDB owner label"))?;
        labels.push(wire[position..end].to_vec());
        position = end;
    }
    Name::from_labels(labels)
}

```
