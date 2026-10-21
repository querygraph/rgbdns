---
type: "code-fragment"
fragment_id: "rgbdns-frag-dfdc979d66ff"
source_path: "src/server.rs"
code_note: "DNS from First Principles/Code/src/server.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "authoritative_stream_handler"
kind: "fn"
start_line: 679
end_line: 717
---

# authoritative_stream_handler

- Fragment ID: `rgbdns-frag-dfdc979d66ff`
- Source file: [[DNS from First Principles/Code/src/server.rs.source|src/server.rs]]
- Lines: 679-717
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-dfdc979d66ff", "codeNote": "DNS from First Principles/Code/src/server.rs.source", "sourcePath": "src/server.rs", "startLine": 679, "endLine": 717}
```

## Excerpt

<span id="rgbdns-frag-dfdc979d66ff" class="rgbdns-fragment-target"></span>
### rgbdns-frag-dfdc979d66ff: fn authoritative_stream_handler

```rust
fn authoritative_stream_handler(
    zone: crate::acme::LiveZone,
    allowed: Option<Arc<Vec<ipnet::IpNet>>>,
    acme: Option<Arc<crate::acme::AcmeUpdates>>,
    query_logger: QueryLogger,
) -> Arc<crate::transport::StreamHandler> {
    Arc::new(move |wire: &[u8], client: SocketAddr| {
        if crate::acme::AcmeUpdates::is_update(wire) {
            let response = if let Some(acme) = &acme {
                acme.handle(wire)
            } else {
                error_response(wire, 5)
            }?;
            query_logger.request(client, wire, Some(&response));
            return Ok(Some(vec![response]));
        }
        let Ok(query) = Message::decode(wire) else {
            return Ok(None);
        };
        let is_axfr =
            query.questions.len() == 1 && query.questions[0].qtype == crate::RecordType::Axfr;
        if !is_axfr {
            return Ok(None);
        }
        let Some(allowed) = &allowed else {
            return Ok(None);
        };
        if !allowed.iter().any(|network| network.contains(&client.ip())) {
            query_logger.axfr(client, wire, false);
            return Err(Error::Format("AXFR client is not allowed"));
        }
        let snapshot = zone.snapshot();
        let response = crate::axfr::response_wires(&snapshot, query).map(Some);
        query_logger.axfr(client, wire, response.is_ok());
        response
    })
}

#[cfg(test)]
```
