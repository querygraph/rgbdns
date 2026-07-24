---
type: "code-fragment"
fragment_id: "rgbdns-frag-454fe9e80b77"
source_path: "src/axfr.rs"
code_note: "DNS from First Principles/Code/src/axfr.rs.source"
language: "rust"
subsystem: "Transport and zone transfer"
crate: "rgbdns"
symbol: "read_message"
kind: "fn"
start_line: 155
end_line: 162
---

# read_message

- Fragment ID: `rgbdns-frag-454fe9e80b77`
- Source file: [[DNS from First Principles/Code/src/axfr.rs.source|src/axfr.rs]]
- Lines: 155-162
- Subsystem: [[DNS from First Principles/Subsystems/Transport and zone transfer|Transport and zone transfer]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-454fe9e80b77", "codeNote": "DNS from First Principles/Code/src/axfr.rs.source", "heading": "rgbdns-frag-454fe9e80b77: fn read_message", "sourcePath": "src/axfr.rs", "startLine": 155, "endLine": 162}
```

## Excerpt

<span id="rgbdns-frag-454fe9e80b77" class="rgbdns-fragment-target"></span>
### rgbdns-frag-454fe9e80b77: fn read_message

```rust
fn read_message(stream: &mut TcpStream) -> Result<Message> {
    let mut length = [0; 2];
    stream.read_exact(&mut length)?;
    let mut wire = vec![0; u16::from_be_bytes(length) as usize];
    stream.read_exact(&mut wire)?;
    Message::decode(&wire)
}

```
