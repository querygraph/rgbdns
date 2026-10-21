---
type: "code-fragment"
fragment_id: "rgbdns-frag-b66581e6e555"
source_path: "src/zone.rs"
code_note: "DNS from First Principles/Code/src/zone.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "client_location"
kind: "fn"
start_line: 824
end_line: 841
---

# client_location

- Fragment ID: `rgbdns-frag-b66581e6e555`
- Source file: [[DNS from First Principles/Code/src/zone.rs.source|src/zone.rs]]
- Lines: 824-841
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-b66581e6e555", "codeNote": "DNS from First Principles/Code/src/zone.rs.source", "sourcePath": "src/zone.rs", "startLine": 824, "endLine": 841}
```

## Excerpt

<span id="rgbdns-frag-b66581e6e555" class="rgbdns-fragment-target"></span>
### rgbdns-frag-b66581e6e555: fn client_location

```rust
    fn client_location(&self, client: Option<IpAddr>) -> [u8; 2] {
        let Some(IpAddr::V4(address)) = client else {
            return [0, 0];
        };
        let octets = address.octets();
        let mut selected = [0, 0];
        let mut selected_length = None;
        for (prefix, location) in &self.locations {
            if prefix.len() <= octets.len()
                && octets[..prefix.len()] == prefix[..]
                && selected_length.is_none_or(|length| prefix.len() > length)
            {
                selected = *location;
                selected_length = Some(prefix.len());
            }
        }
        selected
    }
```
