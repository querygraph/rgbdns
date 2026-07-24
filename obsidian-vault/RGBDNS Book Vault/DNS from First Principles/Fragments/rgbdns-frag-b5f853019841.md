---
type: "code-fragment"
fragment_id: "rgbdns-frag-b5f853019841"
source_path: "benches/dns_core.rs"
code_note: "DNS from First Principles/Code/benches/dns_core.rs.source"
language: "rust"
subsystem: "Tests and performance"
symbol: "main"
kind: "fn"
start_line: 33
end_line: 113
---

# main

- Fragment ID: `rgbdns-frag-b5f853019841`
- Source file: [[DNS from First Principles/Code/benches/dns_core.rs.source|benches/dns_core.rs]]
- Lines: 33-113
- Subsystem: [[DNS from First Principles/Subsystems/Tests and performance|Tests and performance]]

```rgbdns-fragment
{"id": "rgbdns-frag-b5f853019841", "codeNote": "DNS from First Principles/Code/benches/dns_core.rs.source", "heading": "rgbdns-frag-b5f853019841: fn main", "sourcePath": "benches/dns_core.rs", "startLine": 33, "endLine": 113}
```

## Excerpt

<span id="rgbdns-frag-b5f853019841" class="rgbdns-fragment-target"></span>
### rgbdns-frag-b5f853019841: fn main

```rust
fn main() {
    let query = Message {
        id: 0x1234,
        flags: 0x0100,
        questions: vec![Question {
            name: "www.example".parse().unwrap(),
            qtype: RecordType::A,
            qclass: 1,
        }],
        ..Message::default()
    };
    let query_wire = query.encode().unwrap();

    let answers = (1..=64)
        .map(|last| Record {
            name: "many.deep.example".parse().unwrap(),
            ttl: 300,
            data: RData::A(Ipv4Addr::new(192, 0, 2, last)),
        })
        .collect::<Vec<_>>();
    let response = Message {
        id: 0x1234,
        flags: 0x8400,
        questions: vec![Question {
            name: "many.deep.example".parse().unwrap(),
            qtype: RecordType::A,
            qclass: 1,
        }],
        answers,
        ..Message::default()
    };
    let response_wire = response.encode().unwrap();

    let mut data = ".example::ns.example\n".to_owned();
    for index in 0..1_000 {
        data.push_str(&format!("+host{index}.example:192.0.2.1:300\n"));
    }
    for last in 1..=200 {
        data.push_str(&format!("+many.example:192.0.2.{last}:300\n"));
    }
    let zone = Zone::parse(&data).unwrap();
    let lookup_name = "host500.example".parse().unwrap();
    let missing_name = "missing.example".parse().unwrap();
    let small_request = query_wire.clone();
    let large_request = Message {
        questions: vec![Question {
            name: "many.example".parse().unwrap(),
            qtype: RecordType::A,
            qclass: 1,
        }],
        ..Message::default()
    }
    .encode()
    .unwrap();

    println!("rgbdns core microbenchmarks");
    println!("encoded 64-record response: {} bytes", response_wire.len());
    report("decode small query", || {
        black_box(Message::decode(black_box(&query_wire)).unwrap());
    });
    report("decode 64-record response", || {
        black_box(Message::decode(black_box(&response_wire)).unwrap());
    });
    report("encode 64-record response", || {
        black_box(black_box(&response).encode().unwrap());
    });
    report("zone exact lookup (1k names)", || {
        black_box(black_box(&zone).lookup(black_box(&lookup_name), black_box(RecordType::A)));
    });
    report("zone NXDOMAIN (1k names)", || {
        black_box(black_box(&zone).lookup(black_box(&missing_name), black_box(RecordType::A)));
    });
    report("authoritative small response", || {
        black_box(
            server::respond(black_box(&zone), black_box(&small_request), black_box(4096)).unwrap(),
        );
    });
    report("authoritative truncation (200 RR)", || {
        black_box(
            server::respond(black_box(&zone), black_box(&large_request), black_box(4096)).unwrap(),
        );
```
