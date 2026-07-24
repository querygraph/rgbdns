---
type: "code-fragment"
fragment_id: "rgbdns-frag-f2bbdc5532d8"
source_path: "src/packet.rs"
code_note: "DNS from First Principles/Code/src/packet.rs.source"
language: "rust"
subsystem: "DNS data model and wire codec"
crate: "rgbdns"
symbol: "validate_edns_options"
kind: "fn"
start_line: 382
end_line: 393
---

# validate_edns_options

- Fragment ID: `rgbdns-frag-f2bbdc5532d8`
- Source file: [[DNS from First Principles/Code/src/packet.rs.source|src/packet.rs]]
- Lines: 382-393
- Subsystem: [[DNS from First Principles/Subsystems/DNS data model and wire codec|DNS data model and wire codec]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-f2bbdc5532d8", "codeNote": "DNS from First Principles/Code/src/packet.rs.source", "heading": "rgbdns-frag-f2bbdc5532d8: fn validate_edns_options", "sourcePath": "src/packet.rs", "startLine": 382, "endLine": 393}
```

## Excerpt

<span id="rgbdns-frag-f2bbdc5532d8" class="rgbdns-fragment-target"></span>
### rgbdns-frag-f2bbdc5532d8: fn validate_edns_options

```rust
fn validate_edns_options(mut options: &[u8]) -> Result<()> {
    while !options.is_empty() {
        if options.len() < 4 {
            return Err(Error::Format("truncated EDNS option"));
        }
        let len = u16::from_be_bytes([options[2], options[3]]) as usize;
        options = options
            .get(4 + len..)
            .ok_or(Error::Format("truncated EDNS option data"))?;
    }
    Ok(())
}
```
