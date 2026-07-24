---
type: "code-fragment"
fragment_id: "rgbdns-frag-7e2ec0ce150f"
source_path: "src/pick.rs"
code_note: "DNS from First Principles/Code/src/pick.rs.source"
language: "rust"
subsystem: "Specialized responders"
crate: "rgbdns"
symbol: "from_cdb"
kind: "fn"
start_line: 65
end_line: 93
---

# from_cdb

- Fragment ID: `rgbdns-frag-7e2ec0ce150f`
- Source file: [[DNS from First Principles/Code/src/pick.rs.source|src/pick.rs]]
- Lines: 65-93
- Subsystem: [[DNS from First Principles/Subsystems/Specialized responders|Specialized responders]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-7e2ec0ce150f", "codeNote": "DNS from First Principles/Code/src/pick.rs.source", "heading": "rgbdns-frag-7e2ec0ce150f: fn from_cdb", "sourcePath": "src/pick.rs", "startLine": 65, "endLine": 93}
```

## Excerpt

<span id="rgbdns-frag-7e2ec0ce150f" class="rgbdns-fragment-target"></span>
### rgbdns-frag-7e2ec0ce150f: fn from_cdb

```rust
    fn from_cdb(path: &Path) -> Result<Self> {
        let mut database = Self::default();
        for (key, value) in crate::cdb::read_entries(path)? {
            if key.first() == Some(&b'%') && key.len() <= 5 {
                if value.len() != 2 {
                    return Err(Error::Format("invalid pickdns location value"));
                }
                database
                    .locations
                    .push((key[1..].to_vec(), [value[0], value[1]]));
            } else if key.first() == Some(&b'+') && key.len() >= 4 {
                if value.len() % 4 != 0 {
                    return Err(Error::Format("invalid pickdns address value"));
                }
                let name = crate::cdb::decode_name(&key[3..])?;
                let addresses = value
                    .chunks_exact(4)
                    .map(|bytes| Ipv4Addr::new(bytes[0], bytes[1], bytes[2], bytes[3]))
                    .collect();
                database
                    .addresses
                    .insert(([key[1], key[2]], name), addresses);
            } else {
                return Err(Error::Format("invalid pickdns CDB key"));
            }
        }
        Ok(database)
    }

```
