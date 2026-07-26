---
type: "code-fragment"
fragment_id: "rgbdns-frag-f67eebb3c015"
source_path: "src/cdb.rs"
code_note: "DNS from First Principles/Code/src/cdb.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "load"
kind: "fn"
start_line: 62
end_line: 101
---

# load

- Fragment ID: `rgbdns-frag-f67eebb3c015`
- Source file: [[DNS from First Principles/Code/src/cdb.rs.source|src/cdb.rs]]
- Lines: 62-101
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-f67eebb3c015", "codeNote": "DNS from First Principles/Code/src/cdb.rs.source", "heading": "rgbdns-frag-f67eebb3c015: fn load", "sourcePath": "src/cdb.rs", "startLine": 62, "endLine": 101}
```

## Excerpt

<span id="rgbdns-frag-f67eebb3c015" class="rgbdns-fragment-target"></span>
### rgbdns-frag-f67eebb3c015: fn load

```rust
pub fn load(path: impl AsRef<Path>) -> Result<Zone> {
    let entries = read_entries(path)?;
    let mut records = Vec::new();
    let mut locations = Vec::new();
    let mut anames = Vec::new();
    for (key, value) in entries {
        if key.starts_with(b"\0%") {
            if value.len() != 2 || key.len() > 6 {
                return Err(Error::Format("invalid location mapping"));
            }
            locations.push((key[2..].to_vec(), [value[0], value[1]]));
            continue;
        }
        if key.starts_with(b"\0A") {
            if value.len() < 5 {
                return Err(Error::Format("short ANAME CDB value"));
            }
            let owner = decode_name(&key[2..])?;
            let ttl = u32::from_be_bytes(
                value[..4]
                    .try_into()
                    .map_err(|_| Error::Format("short ANAME TTL"))?,
            );
            if ttl == 0 {
                return Err(Error::Format("ANAME TTL must be positive"));
            }
            anames.push((
                owner,
                Aname {
                    target: decode_name(&value[4..])?,
                    ttl,
                },
            ));
            continue;
        }
        records.push(decode_record(&key, &value)?);
    }
    Ok(Zone::from_compiled_records(records, locations, anames))
}

```
