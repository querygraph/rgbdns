---
type: "code-fragment"
fragment_id: "rgbdns-frag-8c366d542d3c"
source_path: "src/axfr.rs"
code_note: "DNS from First Principles/Code/src/axfr.rs.source"
language: "rust"
subsystem: "Transport and zone transfer"
crate: "rgbdns"
symbol: "write_response"
kind: "fn"
start_line: 123
end_line: 138
---

# write_response

- Fragment ID: `rgbdns-frag-8c366d542d3c`
- Source file: [[DNS from First Principles/Code/src/axfr.rs.source|src/axfr.rs]]
- Lines: 123-138
- Subsystem: [[DNS from First Principles/Subsystems/Transport and zone transfer|Transport and zone transfer]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-8c366d542d3c", "codeNote": "DNS from First Principles/Code/src/axfr.rs.source", "heading": "rgbdns-frag-8c366d542d3c: fn write_response", "sourcePath": "src/axfr.rs", "startLine": 123, "endLine": 138}
```

## Excerpt

<span id="rgbdns-frag-8c366d542d3c" class="rgbdns-fragment-target"></span>
### rgbdns-frag-8c366d542d3c: fn write_response

```rust
fn write_response(
    stream: &mut TcpStream,
    id: u16,
    question: Option<Question>,
    rcode: u16,
    answers: Vec<Record>,
) -> Result<()> {
    let wire = response_wire(id, question, rcode, answers)?;
    if wire.len() > MAX_TCP_MESSAGE {
        return Err(Error::Format("AXFR message exceeds DNS TCP framing"));
    }
    stream.write_all(&(wire.len() as u16).to_be_bytes())?;
    stream.write_all(&wire)?;
    Ok(())
}

```
