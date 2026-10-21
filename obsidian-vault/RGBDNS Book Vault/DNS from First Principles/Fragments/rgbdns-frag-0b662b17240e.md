---
type: "code-fragment"
fragment_id: "rgbdns-frag-0b662b17240e"
source_path: "src/axfr.rs"
code_note: "DNS from First Principles/Code/src/axfr.rs.source"
language: "rust"
subsystem: "Transport and zone transfer"
crate: "rgbdns"
symbol: "serve_connection"
kind: "fn"
start_line: 65
end_line: 73
---

# serve_connection

- Fragment ID: `rgbdns-frag-0b662b17240e`
- Source file: [[DNS from First Principles/Code/src/axfr.rs.source|src/axfr.rs]]
- Lines: 65-73
- Subsystem: [[DNS from First Principles/Subsystems/Transport and zone transfer|Transport and zone transfer]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-0b662b17240e", "codeNote": "DNS from First Principles/Code/src/axfr.rs.source", "sourcePath": "src/axfr.rs", "startLine": 65, "endLine": 73}
```

## Excerpt

<span id="rgbdns-frag-0b662b17240e" class="rgbdns-fragment-target"></span>
### rgbdns-frag-0b662b17240e: fn serve_connection

```rust
fn serve_connection(zone: &Zone, stream: &mut TcpStream) -> Result<()> {
    let query = read_message(stream)?;
    for wire in response_wires(zone, query)? {
        stream.write_all(&(wire.len() as u16).to_be_bytes())?;
        stream.write_all(&wire)?;
    }
    Ok(())
}

```
