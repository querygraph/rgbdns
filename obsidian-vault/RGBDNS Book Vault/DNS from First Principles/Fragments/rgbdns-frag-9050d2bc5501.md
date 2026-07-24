---
type: "code-fragment"
fragment_id: "rgbdns-frag-9050d2bc5501"
source_path: "src/rbl.rs"
code_note: "DNS from First Principles/Code/src/rbl.rs.source"
language: "rust"
subsystem: "Specialized responders"
crate: "rgbdns"
symbol: "from_cdb"
kind: "fn"
start_line: 58
end_line: 81
---

# from_cdb

- Fragment ID: `rgbdns-frag-9050d2bc5501`
- Source file: [[DNS from First Principles/Code/src/rbl.rs.source|src/rbl.rs]]
- Lines: 58-81
- Subsystem: [[DNS from First Principles/Subsystems/Specialized responders|Specialized responders]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-9050d2bc5501", "codeNote": "DNS from First Principles/Code/src/rbl.rs.source", "heading": "rgbdns-frag-9050d2bc5501: fn from_cdb", "sourcePath": "src/rbl.rs", "startLine": 58, "endLine": 81}
```

## Excerpt

<span id="rgbdns-frag-9050d2bc5501" class="rgbdns-fragment-target"></span>
### rgbdns-frag-9050d2bc5501: fn from_cdb

```rust
    fn from_cdb(path: &Path) -> Result<Self> {
        let mut database = Self {
            networks: HashSet::new(),
            responses: Vec::new(),
        };
        for (key, value) in crate::cdb::read_entries(path)? {
            match key.len() {
                0 if value.len() >= 4 => {
                    database.responses.push((
                        Ipv4Addr::new(value[0], value[1], value[2], value[3]),
                        value[4..value.len().min(100)].to_vec(),
                    ));
                }
                5 => {
                    let address = u32::from_be_bytes([key[0], key[1], key[2], key[3]]);
                    database.networks.insert((address, key[4].min(32)));
                }
                0 => {}
                _ => return Err(Error::Format("invalid rbldns CDB key")),
            }
        }
        Ok(database)
    }

```
