---
type: "code-fragment"
fragment_id: "rgbdns-frag-1d938f8ec076"
source_path: "src/rbl.rs"
code_note: "DNS from First Principles/Code/src/rbl.rs.source"
language: "rust"
subsystem: "Specialized responders"
crate: "rgbdns"
symbol: "parse_network"
kind: "fn"
start_line: 177
end_line: 195
---

# parse_network

- Fragment ID: `rgbdns-frag-1d938f8ec076`
- Source file: [[DNS from First Principles/Code/src/rbl.rs.source|src/rbl.rs]]
- Lines: 177-195
- Subsystem: [[DNS from First Principles/Subsystems/Specialized responders|Specialized responders]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-1d938f8ec076", "codeNote": "DNS from First Principles/Code/src/rbl.rs.source", "heading": "rgbdns-frag-1d938f8ec076: fn parse_network", "sourcePath": "src/rbl.rs", "startLine": 177, "endLine": 195}
```

## Excerpt

<span id="rgbdns-frag-1d938f8ec076" class="rgbdns-fragment-target"></span>
### rgbdns-frag-1d938f8ec076: fn parse_network

```rust
fn parse_network(input: &str) -> std::result::Result<(u32, u8), &'static str> {
    let (address, prefix) = input.split_once('/').unwrap_or((input, "32"));
    let prefix = prefix.parse::<u8>().unwrap_or(32).min(32);
    let fields = address.split('.').collect::<Vec<_>>();
    if fields.is_empty() || fields.len() > 4 {
        return Err("malformed IPv4 network");
    }
    let mut octets = [0; 4];
    for (index, field) in fields.iter().enumerate() {
        octets[index] = field.parse().map_err(|_| "malformed IPv4 network")?;
    }
    let mask = if prefix == 0 {
        0
    } else {
        u32::MAX << (32 - prefix)
    };
    Ok((u32::from_be_bytes(octets) & mask, prefix))
}

```
