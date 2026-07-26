---
type: "code-fragment"
fragment_id: "rgbdns-frag-916ec1cbc28e"
source_path: "src/cdb.rs"
code_note: "DNS from First Principles/Code/src/cdb.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "read_entries"
kind: "fn"
start_line: 102
end_line: 156
---

# read_entries

- Fragment ID: `rgbdns-frag-916ec1cbc28e`
- Source file: [[DNS from First Principles/Code/src/cdb.rs.source|src/cdb.rs]]
- Lines: 102-156
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-916ec1cbc28e", "codeNote": "DNS from First Principles/Code/src/cdb.rs.source", "heading": "rgbdns-frag-916ec1cbc28e: fn read_entries", "sourcePath": "src/cdb.rs", "startLine": 102, "endLine": 156}
```

## Excerpt

<span id="rgbdns-frag-916ec1cbc28e" class="rgbdns-fragment-target"></span>
### rgbdns-frag-916ec1cbc28e: fn read_entries

```rust
pub(crate) fn read_entries(path: impl AsRef<Path>) -> Result<Vec<(Vec<u8>, Vec<u8>)>> {
    let metadata = fs::metadata(path.as_ref())?;
    if metadata.len() > MAX_DATABASE_SIZE {
        return Err(Error::Format("CDB exceeds configured safety limit"));
    }
    let bytes = fs::read(path)?;
    if bytes.len() < HEADER_LEN {
        return Err(Error::Format("short CDB header"));
    }
    let mut data_end = bytes.len();
    for index in 0..256 {
        let offset = index * 8;
        let position = le_u32(&bytes[offset..offset + 4]) as usize;
        let slots = le_u32(&bytes[offset + 4..offset + 8]) as usize;
        if position < HEADER_LEN || position > bytes.len() {
            return Err(Error::Format("invalid CDB hash-table position"));
        }
        position
            .checked_add(
                slots
                    .checked_mul(8)
                    .ok_or(Error::Format("CDB hash-table size overflow"))?,
            )
            .filter(|end| *end <= bytes.len())
            .ok_or(Error::Format("invalid CDB hash-table size"))?;
        data_end = data_end.min(position);
    }
    let mut position = HEADER_LEN;
    let mut entries = Vec::new();
    while position < data_end {
        let header = bytes
            .get(position..position + 8)
            .ok_or(Error::Format("truncated CDB record header"))?;
        let key_len = le_u32(&header[..4]) as usize;
        let data_len = le_u32(&header[4..]) as usize;
        position += 8;
        let key_end = position
            .checked_add(key_len)
            .filter(|end| *end <= data_end)
            .ok_or(Error::Format("invalid CDB key length"))?;
        let value_end = key_end
            .checked_add(data_len)
            .filter(|end| *end <= data_end)
            .ok_or(Error::Format("invalid CDB value length"))?;
        let key = bytes[position..key_end].to_vec();
        let value = bytes[key_end..value_end].to_vec();
        position = value_end;
        entries.push((key, value));
    }
    if position != data_end {
        return Err(Error::Format("CDB data section is misaligned"));
    }
    Ok(entries)
}

```
