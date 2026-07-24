---
type: "code-fragment"
fragment_id: "rgbdns-frag-0b879085623a"
source_path: "src/rbl.rs"
code_note: "DNS from First Principles/Code/src/rbl.rs.source"
language: "rust"
subsystem: "Specialized responders"
crate: "rgbdns"
symbol: "parse"
kind: "fn"
start_line: 13
end_line: 48
---

# parse

- Fragment ID: `rgbdns-frag-0b879085623a`
- Source file: [[DNS from First Principles/Code/src/rbl.rs.source|src/rbl.rs]]
- Lines: 13-48
- Subsystem: [[DNS from First Principles/Subsystems/Specialized responders|Specialized responders]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-0b879085623a", "codeNote": "DNS from First Principles/Code/src/rbl.rs.source", "heading": "rgbdns-frag-0b879085623a: fn parse", "sourcePath": "src/rbl.rs", "startLine": 13, "endLine": 48}
```

## Excerpt

<span id="rgbdns-frag-0b879085623a" class="rgbdns-fragment-target"></span>
### rgbdns-frag-0b879085623a: fn parse

```rust
    pub fn parse(text: &str) -> Result<Self> {
        let mut database = Self {
            networks: HashSet::new(),
            responses: Vec::new(),
        };
        for (line_number, raw) in text.lines().enumerate() {
            let line = raw.trim_end_matches([' ', '\t', '\r']);
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            if let Some(setting) = line.strip_prefix(':') {
                let (address, message) = setting.split_once(':').ok_or_else(|| {
                    Error::InvalidRecord(format!(
                        "line {}: missing response colon",
                        line_number + 1
                    ))
                })?;
                let address = address.parse().map_err(|_| {
                    Error::InvalidRecord(format!(
                        "line {}: malformed response IPv4 address",
                        line_number + 1
                    ))
                })?;
                database
                    .responses
                    .push((address, message.as_bytes().to_vec()));
                continue;
            }
            let (address, prefix) = parse_network(line).map_err(|error| {
                Error::InvalidRecord(format!("line {}: {error}", line_number + 1))
            })?;
            database.networks.insert((address, prefix));
        }
        Ok(database)
    }

```
