---
type: "code-fragment"
fragment_id: "rgbdns-frag-c4eddbb40457"
source_path: "src/axfr.rs"
code_note: "DNS from First Principles/Code/src/axfr.rs.source"
language: "rust"
subsystem: "Transport and zone transfer"
crate: "rgbdns"
symbol: "response_wires"
kind: "fn"
start_line: 71
end_line: 151
---

# response_wires

- Fragment ID: `rgbdns-frag-c4eddbb40457`
- Source file: [[DNS from First Principles/Code/src/axfr.rs.source|src/axfr.rs]]
- Lines: 71-151
- Subsystem: [[DNS from First Principles/Subsystems/Transport and zone transfer|Transport and zone transfer]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-c4eddbb40457", "codeNote": "DNS from First Principles/Code/src/axfr.rs.source", "heading": "rgbdns-frag-c4eddbb40457: fn response_wires", "sourcePath": "src/axfr.rs", "startLine": 71, "endLine": 151}
```

## Excerpt

<span id="rgbdns-frag-c4eddbb40457" class="rgbdns-fragment-target"></span>
### rgbdns-frag-c4eddbb40457: fn response_wires

```rust
pub(crate) fn response_wires(zone: &Zone, query: Message) -> Result<Vec<Vec<u8>>> {
    if query.flags & 0x8000 != 0 {
        return Err(Error::Format("received an AXFR response as a query"));
    }
    if query.questions.len() != 1 {
        return Ok(vec![response_wire(query.id, None, 1, Vec::new())?]);
    }
    let question = query.questions[0].clone();
    if query.flags & 0x7800 != 0 {
        return Ok(vec![response_wire(
            query.id,
            Some(question),
            4,
            Vec::new(),
        )?]);
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
        return Ok(vec![response_wire(
            query.id,
            Some(question),
            1,
            Vec::new(),
        )?]);
    }
    if question.qclass != 1 || question.qtype != RecordType::Axfr {
        return Ok(vec![response_wire(
            query.id,
            Some(question),
            4,
            Vec::new(),
        )?]);
    }
    let Some(records) = zone.transfer(&question.name) else {
        return Ok(vec![response_wire(
            query.id,
            Some(question),
            5,
            Vec::new(),
        )?]);
    };
    let mut responses = Vec::new();
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
        responses.push(response_wire(
            query.id,
            first.then(|| question.clone()),
            0,
            std::mem::take(&mut batch),
        )?);
        first = false;
        batch.push(record);
    }
    if !batch.is_empty() {
        responses.push(response_wire(
            query.id,
            first.then_some(question),
            0,
            batch,
        )?);
```
