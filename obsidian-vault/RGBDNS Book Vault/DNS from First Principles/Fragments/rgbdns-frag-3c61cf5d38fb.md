---
type: "code-fragment"
fragment_id: "rgbdns-frag-3c61cf5d38fb"
source_path: "src/pick.rs"
code_note: "DNS from First Principles/Code/src/pick.rs.source"
language: "rust"
subsystem: "Specialized responders"
crate: "rgbdns"
symbol: "client_location"
kind: "fn"
start_line: 138
end_line: 154
---

# client_location

- Fragment ID: `rgbdns-frag-3c61cf5d38fb`
- Source file: [[DNS from First Principles/Code/src/pick.rs.source|src/pick.rs]]
- Lines: 138-154
- Subsystem: [[DNS from First Principles/Subsystems/Specialized responders|Specialized responders]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-3c61cf5d38fb", "codeNote": "DNS from First Principles/Code/src/pick.rs.source", "heading": "rgbdns-frag-3c61cf5d38fb: fn client_location", "sourcePath": "src/pick.rs", "startLine": 138, "endLine": 154}
```

## Excerpt

<span id="rgbdns-frag-3c61cf5d38fb" class="rgbdns-fragment-target"></span>
### rgbdns-frag-3c61cf5d38fb: fn client_location

```rust
    fn client_location(&self, client: IpAddr) -> [u8; 2] {
        let IpAddr::V4(client) = client else {
            return [0, 0];
        };
        let octets = client.octets();
        let mut selected = [0, 0];
        let mut length = None;
        for (prefix, location) in &self.locations {
            if octets.starts_with(prefix) && length.is_none_or(|current| prefix.len() > current) {
                selected = *location;
                length = Some(prefix.len());
            }
        }
        selected
    }
}

```
