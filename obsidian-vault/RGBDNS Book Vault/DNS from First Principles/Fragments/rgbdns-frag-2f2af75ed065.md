---
type: "code-fragment"
fragment_id: "rgbdns-frag-2f2af75ed065"
source_path: "src/server.rs"
code_note: "DNS from First Principles/Code/src/server.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "respond_over_transport"
kind: "fn"
start_line: 25
end_line: 105
---

# respond_over_transport

- Fragment ID: `rgbdns-frag-2f2af75ed065`
- Source file: [[DNS from First Principles/Code/src/server.rs.source|src/server.rs]]
- Lines: 25-105
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-2f2af75ed065", "codeNote": "DNS from First Principles/Code/src/server.rs.source", "heading": "rgbdns-frag-2f2af75ed065: fn respond_over_transport", "sourcePath": "src/server.rs", "startLine": 25, "endLine": 105}
```

## Excerpt

<span id="rgbdns-frag-2f2af75ed065" class="rgbdns-fragment-target"></span>
### rgbdns-frag-2f2af75ed065: fn respond_over_transport

```rust
fn respond_over_transport(
    zone: &Zone,
    wire: &[u8],
    transport_limit: usize,
    is_udp: bool,
    client: Option<IpAddr>,
) -> Result<Vec<u8>> {
    // Unknown opcodes can define a body layout different from QUERY. RFC 8906
    // therefore requires NOTIMP based on the header alone, without attempting
    // to parse the body as a standard question.
    if wire.len() >= 4 {
        let flags = u16::from_be_bytes([wire[2], wire[3]]);
        if flags & 0x8000 == 0 && flags & 0x7800 != 0 {
            return error_response(wire, 4);
        }
    }
    let q = match Message::decode(wire) {
        Ok(query) => query,
        Err(_) if wire.len() >= 12 && wire[2] & 0x80 == 0 => {
            return error_response(wire, 1);
        }
        Err(error) => return Err(error),
    };
    if q.flags & 0x8000 != 0 {
        return Err(Error::Format("received a DNS response"));
    }
    if q.answers
        .iter()
        .chain(&q.authorities)
        .any(|record| record.rr_type() == crate::RecordType::Opt)
    {
        return error_response(wire, 1);
    }
    if q.questions.len() != 1 {
        return error_response(wire, 1);
    }
    let question = q.questions[0].clone();
    if q.flags & 0x7800 != 0 {
        return error_response(wire, 4);
    }
    let options = q
        .additionals
        .iter()
        .filter_map(|record| match &record.data {
            crate::RData::Opt {
                udp_payload,
                version,
                flags,
                ..
            } => Some((*udp_payload, *version, *flags)),
            _ => None,
        })
        .collect::<Vec<_>>();
    if options.len() > 1 {
        return error_response(wire, 1);
    }
    let opt = options.first().copied();
    let response_limit = if is_udp {
        opt.map_or(512, |(size, _, _)| usize::from(size).max(512))
            .min(transport_limit)
    } else {
        transport_limit
    };
    let mut r = Message {
        id: q.id,
        flags: 0x8000 | 0x0400 | (q.flags & 0x0100),
        questions: vec![question.clone()],
        ..Default::default()
    };
    if let Some((payload, version, flags)) = opt {
        let bad_version = version != 0;
        r.additionals.push(crate::Record {
            name: crate::Name::root(),
            ttl: 0,
            data: crate::RData::Opt {
                udp_payload: payload.min(4096),
                extended_rcode: u8::from(bad_version),
                version: 0,
                flags: flags & 0x8000,
                options: Vec::new(),
            },
```
