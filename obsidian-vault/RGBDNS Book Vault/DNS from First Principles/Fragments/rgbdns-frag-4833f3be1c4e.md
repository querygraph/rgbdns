---
type: "code-fragment"
fragment_id: "rgbdns-frag-4833f3be1c4e"
source_path: "src/axfr.rs"
code_note: "DNS from First Principles/Code/src/axfr.rs.source"
language: "rust"
subsystem: "Transport and zone transfer"
crate: "rgbdns"
symbol: "decode_aname_target"
kind: "fn"
start_line: 452
end_line: 472
---

# decode_aname_target

- Fragment ID: `rgbdns-frag-4833f3be1c4e`
- Source file: [[DNS from First Principles/Code/src/axfr.rs.source|src/axfr.rs]]
- Lines: 452-472
- Subsystem: [[DNS from First Principles/Subsystems/Transport and zone transfer|Transport and zone transfer]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-4833f3be1c4e", "codeNote": "DNS from First Principles/Code/src/axfr.rs.source", "sourcePath": "src/axfr.rs", "startLine": 452, "endLine": 472}
```

## Excerpt

<span id="rgbdns-frag-4833f3be1c4e" class="rgbdns-fragment-target"></span>
### rgbdns-frag-4833f3be1c4e: fn decode_aname_target

```rust
fn decode_aname_target(mut wire: &[u8]) -> Result<Name> {
    let mut labels = Vec::new();
    loop {
        let Some((&length, rest)) = wire.split_first() else {
            return Err(Error::Format("truncated rgbdns ANAME target"));
        };
        wire = rest;
        if length == 0 {
            if !wire.is_empty() {
                return Err(Error::Format("trailing rgbdns ANAME target data"));
            }
            return Name::from_labels(labels);
        }
        if length > 63 || wire.len() < length as usize {
            return Err(Error::Format("invalid rgbdns ANAME target"));
        }
        labels.push(wire[..length as usize].to_vec());
        wire = &wire[length as usize..];
    }
}

```
