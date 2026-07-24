---
type: "code-fragment"
fragment_id: "rgbdns-frag-2554d1b3665a"
source_path: "src/axfr.rs"
code_note: "DNS from First Principles/Code/src/axfr.rs.source"
language: "rust"
subsystem: "Transport and zone transfer"
crate: "rgbdns"
symbol: "serve_connection"
kind: "fn"
start_line: 62
end_line: 122
---

# serve_connection

- Fragment ID: `rgbdns-frag-2554d1b3665a`
- Source file: [[DNS from First Principles/Code/src/axfr.rs.source|src/axfr.rs]]
- Lines: 62-122
- Subsystem: [[DNS from First Principles/Subsystems/Transport and zone transfer|Transport and zone transfer]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-2554d1b3665a", "codeNote": "DNS from First Principles/Code/src/axfr.rs.source", "heading": "rgbdns-frag-2554d1b3665a: fn serve_connection", "sourcePath": "src/axfr.rs", "startLine": 62, "endLine": 122}
```

## Excerpt

<span id="rgbdns-frag-2554d1b3665a" class="rgbdns-fragment-target"></span>
### rgbdns-frag-2554d1b3665a: fn serve_connection

```rust
fn serve_connection(zone: &Zone, stream: &mut TcpStream) -> Result<()> {
    let query = read_message(stream)?;
    if query.flags & 0x8000 != 0 {
        return Err(Error::Format("received an AXFR response as a query"));
    }
    if query.questions.len() != 1 {
        return write_response(stream, query.id, None, 1, Vec::new());
    }
    let question = query.questions[0].clone();
    if query.flags & 0x7800 != 0 {
        return write_response(stream, query.id, Some(question), 4, Vec::new());
    }
    if !query.answers.is_empty()
        || !query.authorities.is_empty()
        || query
            .additionals
            .iter()
            .filter(|record| record.rr_type() == RecordType::Opt)
            .count()
            > 1
    {
        return write_response(stream, query.id, Some(question), 1, Vec::new());
    }
    if question.qclass != 1 || question.qtype != RecordType::Axfr {
        return write_response(stream, query.id, Some(question), 4, Vec::new());
    }
    let Some(records) = zone.transfer(&question.name) else {
        return write_response(stream, query.id, Some(question), 5, Vec::new());
    };
    let mut first = true;
    let mut batch = Vec::new();
    for record in records {
        batch.push(record);
        let candidate = response_wire(query.id, first.then(|| question.clone()), 0, batch.clone());
        if batch.len() <= 4096
            && candidate
                .as_ref()
                .is_ok_and(|wire| wire.len() <= MAX_TCP_MESSAGE)
        {
            continue;
        }
        let record = batch.pop().unwrap();
        if batch.is_empty() {
            return Err(Error::Format("AXFR record exceeds DNS TCP framing"));
        }
        write_response(
            stream,
            query.id,
            first.then(|| question.clone()),
            0,
            std::mem::take(&mut batch),
        )?;
        first = false;
        batch.push(record);
    }
    if !batch.is_empty() {
        write_response(stream, query.id, first.then_some(question), 0, batch)?;
    }
    Ok(())
}

```
