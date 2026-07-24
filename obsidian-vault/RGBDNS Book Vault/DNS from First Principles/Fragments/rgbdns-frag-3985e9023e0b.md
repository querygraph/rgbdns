---
type: "code-fragment"
fragment_id: "rgbdns-frag-3985e9023e0b"
source_path: "src/axfr.rs"
code_note: "DNS from First Principles/Code/src/axfr.rs.source"
language: "rust"
subsystem: "Transport and zone transfer"
crate: "rgbdns"
symbol: "fetch"
kind: "fn"
start_line: 163
end_line: 229
---

# fetch

- Fragment ID: `rgbdns-frag-3985e9023e0b`
- Source file: [[DNS from First Principles/Code/src/axfr.rs.source|src/axfr.rs]]
- Lines: 163-229
- Subsystem: [[DNS from First Principles/Subsystems/Transport and zone transfer|Transport and zone transfer]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-3985e9023e0b", "codeNote": "DNS from First Principles/Code/src/axfr.rs.source", "heading": "rgbdns-frag-3985e9023e0b: fn fetch", "sourcePath": "src/axfr.rs", "startLine": 163, "endLine": 229}
```

## Excerpt

<span id="rgbdns-frag-3985e9023e0b" class="rgbdns-fragment-target"></span>
### rgbdns-frag-3985e9023e0b: fn fetch

```rust
pub fn fetch(server: SocketAddr, zone: Name) -> Result<Vec<Record>> {
    let mut stream = TcpStream::connect_timeout(&server, Duration::from_secs(15))?;
    stream.set_read_timeout(Some(Duration::from_secs(30)))?;
    stream.set_write_timeout(Some(Duration::from_secs(30)))?;
    let id = random_id()?;
    let question = Question {
        name: zone.clone(),
        qtype: RecordType::Axfr,
        qclass: 1,
    };
    let wire = Message {
        id,
        questions: vec![question.clone()],
        ..Default::default()
    }
    .encode()?;
    stream.write_all(&(wire.len() as u16).to_be_bytes())?;
    stream.write_all(&wire)?;

    let mut records = Vec::new();
    let mut opening_soa = None;
    let mut transfer_bytes = 0usize;
    for message_number in 0..MAX_TRANSFER_MESSAGES {
        let response = read_message(&mut stream)?;
        transfer_bytes = transfer_bytes
            .checked_add(response.encode()?.len())
            .filter(|bytes| *bytes <= MAX_TRANSFER_BYTES)
            .ok_or(Error::Format("AXFR byte limit exceeded"))?;
        validate_axfr_message(&response, id, &question, message_number == 0)?;
        if response.flags & 0x000f != 0 {
            return Err(Error::Format("AXFR server returned an error"));
        }
        if response.answers.is_empty() {
            return Err(Error::Format("empty AXFR response"));
        }
        let answer_count = response.answers.len();
        for (answer_index, record) in response.answers.into_iter().enumerate() {
            if opening_soa.is_none() {
                if record.rr_type() != RecordType::Soa || record.name != zone {
                    return Err(Error::Format("AXFR does not begin with zone SOA"));
                }
                opening_soa = Some(record.clone());
            } else if record.rr_type() == RecordType::Soa {
                if opening_soa.as_ref() != Some(&record) {
                    return Err(Error::Format("AXFR contains a mismatched SOA"));
                }
                if answer_index + 1 != answer_count {
                    return Err(Error::Format("records follow closing AXFR SOA"));
                }
                records.push(record);
                return Ok(records);
            }
            if !record.name.is_subdomain_of(&zone) {
                return Err(Error::Format("AXFR record is outside the requested zone"));
            }
            records.push(record);
        }
        if records.len() > MAX_TRANSFER_RECORDS {
            return Err(Error::Format("AXFR record limit exceeded"));
        }
        if message_number + 1 == MAX_TRANSFER_MESSAGES {
            return Err(Error::Format("AXFR message limit exceeded"));
        }
    }
    Err(Error::Format("AXFR message limit exceeded"))
}

```
