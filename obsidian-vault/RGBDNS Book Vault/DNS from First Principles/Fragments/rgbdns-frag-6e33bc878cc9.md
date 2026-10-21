---
type: "code-fragment"
fragment_id: "rgbdns-frag-6e33bc878cc9"
source_path: "src/axfr.rs"
code_note: "DNS from First Principles/Code/src/axfr.rs.source"
language: "rust"
subsystem: "Transport and zone transfer"
crate: "rgbdns"
symbol: "requests_aname_extension"
kind: "fn"
start_line: 177
end_line: 198
---

# requests_aname_extension

- Fragment ID: `rgbdns-frag-6e33bc878cc9`
- Source file: [[DNS from First Principles/Code/src/axfr.rs.source|src/axfr.rs]]
- Lines: 177-198
- Subsystem: [[DNS from First Principles/Subsystems/Transport and zone transfer|Transport and zone transfer]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-6e33bc878cc9", "codeNote": "DNS from First Principles/Code/src/axfr.rs.source", "sourcePath": "src/axfr.rs", "startLine": 177, "endLine": 198}
```

## Excerpt

<span id="rgbdns-frag-6e33bc878cc9" class="rgbdns-fragment-target"></span>
### rgbdns-frag-6e33bc878cc9: fn requests_aname_extension

```rust
fn requests_aname_extension(query: &Message) -> bool {
    query.additionals.iter().any(|record| {
        let RData::Opt { options, .. } = &record.data else {
            return false;
        };
        let mut remaining = options.as_slice();
        while remaining.len() >= 4 {
            let code = u16::from_be_bytes([remaining[0], remaining[1]]);
            let length = u16::from_be_bytes([remaining[2], remaining[3]]) as usize;
            remaining = &remaining[4..];
            if remaining.len() < length {
                return false;
            }
            if code == RGBDNS_ANAME_OPTION && remaining[..length] == *RGBDNS_ANAME_MAGIC {
                return true;
            }
            remaining = &remaining[length..];
        }
        false
    })
}

```
