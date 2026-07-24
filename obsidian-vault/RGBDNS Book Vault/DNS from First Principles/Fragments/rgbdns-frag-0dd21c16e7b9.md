---
type: "code-fragment"
fragment_id: "rgbdns-frag-0dd21c16e7b9"
source_path: "src/zone.rs"
code_note: "DNS from First Principles/Code/src/zone.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "client_location"
kind: "fn"
start_line: 583
end_line: 600
---

# client_location

- Fragment ID: `rgbdns-frag-0dd21c16e7b9`
- Source file: [[DNS from First Principles/Code/src/zone.rs.source|src/zone.rs]]
- Lines: 583-600
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-0dd21c16e7b9", "codeNote": "DNS from First Principles/Code/src/zone.rs.source", "heading": "rgbdns-frag-0dd21c16e7b9: fn client_location", "sourcePath": "src/zone.rs", "startLine": 583, "endLine": 600}
```

## Excerpt

<span id="rgbdns-frag-0dd21c16e7b9" class="rgbdns-fragment-target"></span>
### rgbdns-frag-0dd21c16e7b9: fn client_location

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
