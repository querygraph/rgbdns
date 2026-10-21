---
type: "code-fragment"
fragment_id: "rgbdns-frag-a2043173b4ff"
source_path: "src/packet.rs"
code_note: "DNS from First Principles/Code/src/packet.rs.source"
language: "rust"
subsystem: "DNS data model and wire codec"
crate: "rgbdns"
symbol: "type_bitmap"
kind: "fn"
start_line: 311
end_line: 342
---

# type_bitmap

- Fragment ID: `rgbdns-frag-a2043173b4ff`
- Source file: [[DNS from First Principles/Code/src/packet.rs.source|src/packet.rs]]
- Lines: 311-342
- Subsystem: [[DNS from First Principles/Subsystems/DNS data model and wire codec|DNS data model and wire codec]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-a2043173b4ff", "codeNote": "DNS from First Principles/Code/src/packet.rs.source", "sourcePath": "src/packet.rs", "startLine": 311, "endLine": 342}
```

## Excerpt

<span id="rgbdns-frag-a2043173b4ff" class="rgbdns-fragment-target"></span>
### rgbdns-frag-a2043173b4ff: fn type_bitmap

```rust
fn type_bitmap(mut bytes: &[u8]) -> Result<Vec<RecordType>> {
    let mut types = Vec::new();
    let mut previous = None;
    while !bytes.is_empty() {
        if bytes.len() < 3 {
            return Err(Error::Format("truncated NSEC type bitmap"));
        }
        let window = bytes[0];
        let length = usize::from(bytes[1]);
        if !(1..=32).contains(&length)
            || bytes.len() < 2 + length
            || previous.is_some_and(|value| window <= value)
            || bytes[1 + length] == 0
        {
            return Err(Error::Format("invalid NSEC type bitmap"));
        }
        for (octet, bits) in bytes[2..2 + length].iter().enumerate() {
            for bit in 0..8 {
                if bits & (0x80 >> bit) != 0 {
                    types.push(RecordType::from_code(
                        u16::from(window) * 256 + (octet * 8 + bit) as u16,
                    ));
                }
            }
        }
        previous = Some(window);
        bytes = &bytes[2 + length..];
    }
    Ok(types)
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
```
