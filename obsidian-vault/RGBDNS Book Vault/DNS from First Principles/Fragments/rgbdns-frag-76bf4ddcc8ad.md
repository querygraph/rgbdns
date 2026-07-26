---
type: "code-fragment"
fragment_id: "rgbdns-frag-76bf4ddcc8ad"
source_path: "src/server.rs"
code_note: "DNS from First Principles/Code/src/server.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "axfr_stream_handler"
kind: "fn"
start_line: 424
end_line: 444
---

# axfr_stream_handler

- Fragment ID: `rgbdns-frag-76bf4ddcc8ad`
- Source file: [[DNS from First Principles/Code/src/server.rs.source|src/server.rs]]
- Lines: 424-444
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-76bf4ddcc8ad", "codeNote": "DNS from First Principles/Code/src/server.rs.source", "heading": "rgbdns-frag-76bf4ddcc8ad: fn axfr_stream_handler", "sourcePath": "src/server.rs", "startLine": 424, "endLine": 444}
```

## Excerpt

<span id="rgbdns-frag-76bf4ddcc8ad" class="rgbdns-fragment-target"></span>
### rgbdns-frag-76bf4ddcc8ad: fn axfr_stream_handler

```rust
fn axfr_stream_handler(
    zone: Arc<Zone>,
    allowed: Arc<Vec<ipnet::IpNet>>,
) -> Arc<crate::transport::StreamHandler> {
    Arc::new(move |wire: &[u8], client: IpAddr| {
        let Ok(query) = Message::decode(wire) else {
            return Ok(None);
        };
        let is_axfr =
            query.questions.len() == 1 && query.questions[0].qtype == crate::RecordType::Axfr;
        if !is_axfr {
            return Ok(None);
        }
        if !allowed.iter().any(|network| network.contains(&client)) {
            return Err(Error::Format("AXFR client is not allowed"));
        }
        crate::axfr::response_wires(&zone, query).map(Some)
    })
}

#[cfg(test)]
```
