---
type: "code-fragment"
fragment_id: "rgbdns-frag-a64686cbedbb"
source_path: "src/server.rs"
code_note: "DNS from First Principles/Code/src/server.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "aname_synthesizes_authoritative_addresses_without_emitting_cname"
kind: "fn"
start_line: 616
end_line: 677
---

# aname_synthesizes_authoritative_addresses_without_emitting_cname

- Fragment ID: `rgbdns-frag-a64686cbedbb`
- Source file: [[DNS from First Principles/Code/src/server.rs.source|src/server.rs]]
- Lines: 616-677
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-a64686cbedbb", "codeNote": "DNS from First Principles/Code/src/server.rs.source", "heading": "rgbdns-frag-a64686cbedbb: fn aname_synthesizes_authoritative_addresses_without_emitting_cname", "sourcePath": "src/server.rs", "startLine": 616, "endLine": 677}
```

## Excerpt

<span id="rgbdns-frag-a64686cbedbb" class="rgbdns-fragment-target"></span>
### rgbdns-frag-a64686cbedbb: fn aname_synthesizes_authoritative_addresses_without_emitting_cname

```rust
    fn aname_synthesizes_authoritative_addresses_without_emitting_cname() {
        let upstream = UdpSocket::bind("127.0.0.1:0").unwrap();
        let upstream_address = upstream.local_addr().unwrap();
        let upstream_thread = thread::spawn(move || {
            for address in [
                RData::A("192.0.2.44".parse().unwrap()),
                RData::Aaaa("2001:db8::44".parse().unwrap()),
            ] {
                let mut wire = [0; 512];
                let (length, peer) = upstream.recv_from(&mut wire).unwrap();
                let request = Message::decode(&wire[..length]).unwrap();
                let response = Message {
                    id: request.id,
                    flags: 0x8000 | 0x0100,
                    questions: request.questions.clone(),
                    answers: vec![Record {
                        name: request.questions[0].name.clone(),
                        ttl: 600,
                        data: address,
                    }],
                    ..Default::default()
                }
                .encode()
                .unwrap();
                upstream.send_to(&response, peer).unwrap();
            }
        });
        let zone = Zone::parse(
            ".example:192.0.2.53:ns.example\n\
             Aexample:blog-host.example.net:120\n",
        )
        .unwrap();
        let resolver = crate::aname::Resolver::new(vec![upstream_address]);
        for record_type in [RecordType::A, RecordType::Aaaa] {
            let response = Message::decode(
                &respond_over_transport(
                    &zone,
                    Some(&resolver),
                    &query("example", record_type, None),
                    4096,
                    true,
                    None,
                )
                .unwrap(),
            )
            .unwrap();
            assert_eq!(response.flags & 0x040f, 0x0400);
            assert_eq!(response.answers.len(), 1);
            assert_eq!(response.answers[0].name, "example".parse().unwrap());
            assert_eq!(response.answers[0].rr_type(), record_type);
            assert!(response.answers[0].ttl <= 120);
            assert!(
                response
                    .answers
                    .iter()
                    .all(|record| record.rr_type() != RecordType::Cname)
            );
        }
        upstream_thread.join().unwrap();
    }

    #[test]
```
