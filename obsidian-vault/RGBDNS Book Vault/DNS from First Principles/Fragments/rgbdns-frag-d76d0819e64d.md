---
type: "code-fragment"
fragment_id: "rgbdns-frag-d76d0819e64d"
source_path: "src/axfr.rs"
code_note: "DNS from First Principles/Code/src/axfr.rs.source"
language: "rust"
subsystem: "Transport and zone transfer"
crate: "rgbdns"
symbol: "response_wires"
kind: "fn"
start_line: 74
end_line: 154
---

# response_wires

- Fragment ID: `rgbdns-frag-d76d0819e64d`
- Source file: [[DNS from First Principles/Code/src/axfr.rs.source|src/axfr.rs]]
- Lines: 74-154
- Subsystem: [[DNS from First Principles/Subsystems/Transport and zone transfer|Transport and zone transfer]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-d76d0819e64d", "codeNote": "DNS from First Principles/Code/src/axfr.rs.source", "sourcePath": "src/axfr.rs", "startLine": 74, "endLine": 154}
```

## Excerpt

<span id="rgbdns-frag-d76d0819e64d" class="rgbdns-fragment-target"></span>
### rgbdns-frag-d76d0819e64d: fn response_wires

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
    let Some(mut records) = zone.transfer(&question.name) else {
        return Ok(vec![response_wire(
            query.id,
            Some(question),
            5,
            Vec::new(),
        )?]);
    };
    if requests_aname_extension(&query) {
        let closing_soa = records
            .pop()
            .ok_or(Error::Format("AXFR transfer has no closing SOA"))?;
        for (owner, aname) in zone
            .transfer_anames(&question.name)
            .ok_or(Error::Format("AXFR zone has no ANAME scope"))?
        {
            let mut payload = RGBDNS_ANAME_MAGIC.to_vec();
            payload.extend(aname.target.to_wire());
            records.push(Record {
                name: owner,
                ttl: aname.ttl,
                data: RData::Opaque(RecordType::Unknown(RGBDNS_ANAME_RECORD), payload),
            });
        }
        records.push(closing_soa);
    }
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
```
