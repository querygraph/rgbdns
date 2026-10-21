---
type: "code-fragment"
fragment_id: "rgbdns-frag-4792f972416c"
source_path: "src/packet.rs"
code_note: "DNS from First Principles/Code/src/packet.rs.source"
language: "rust"
subsystem: "DNS data model and wire codec"
crate: "rgbdns"
symbol: "uncompressed_name"
kind: "fn"
start_line: 287
end_line: 310
---

# uncompressed_name

- Fragment ID: `rgbdns-frag-4792f972416c`
- Source file: [[DNS from First Principles/Code/src/packet.rs.source|src/packet.rs]]
- Lines: 287-310
- Subsystem: [[DNS from First Principles/Subsystems/DNS data model and wire codec|DNS data model and wire codec]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-4792f972416c", "codeNote": "DNS from First Principles/Code/src/packet.rs.source", "sourcePath": "src/packet.rs", "startLine": 287, "endLine": 310}
```

## Excerpt

<span id="rgbdns-frag-4792f972416c" class="rgbdns-fragment-target"></span>
### rgbdns-frag-4792f972416c: fn uncompressed_name

```rust
fn uncompressed_name(bytes: &[u8]) -> Result<(Name, usize)> {
    let mut labels = Vec::new();
    let mut position = 0;
    loop {
        let length = usize::from(
            *bytes
                .get(position)
                .ok_or(Error::Format("truncated DNSSEC name"))?,
        );
        position += 1;
        if length == 0 {
            return Ok((Name::from_labels(labels)?, position));
        }
        if length > 63 || length & 0xc0 != 0 {
            return Err(Error::Format("compressed or invalid DNSSEC name"));
        }
        let label = bytes
            .get(position..position + length)
            .ok_or(Error::Format("truncated DNSSEC label"))?;
        labels.push(label.to_vec());
        position += length;
    }
}

```
