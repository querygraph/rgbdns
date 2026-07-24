---
type: "code-fragment"
fragment_id: "rgbdns-frag-999162e0163d"
source_path: "tests/wire_security.rs"
code_note: "DNS from First Principles/Code/tests/wire_security.rs.source"
language: "rust"
subsystem: "Tests and performance"
symbol: "decoder_rejects_every_truncation_of_a_valid_structured_packet"
kind: "fn"
start_line: 88
end_line: 108
---

# decoder_rejects_every_truncation_of_a_valid_structured_packet

- Fragment ID: `rgbdns-frag-999162e0163d`
- Source file: [[DNS from First Principles/Code/tests/wire_security.rs.source|tests/wire_security.rs]]
- Lines: 88-108
- Subsystem: [[DNS from First Principles/Subsystems/Tests and performance|Tests and performance]]

```rgbdns-fragment
{"id": "rgbdns-frag-999162e0163d", "codeNote": "DNS from First Principles/Code/tests/wire_security.rs.source", "heading": "rgbdns-frag-999162e0163d: fn decoder_rejects_every_truncation_of_a_valid_structured_packet", "sourcePath": "tests/wire_security.rs", "startLine": 88, "endLine": 108}
```

## Excerpt

<span id="rgbdns-frag-999162e0163d" class="rgbdns-fragment-target"></span>
### rgbdns-frag-999162e0163d: fn decoder_rejects_every_truncation_of_a_valid_structured_packet

```rust
fn decoder_rejects_every_truncation_of_a_valid_structured_packet() {
    let message = Message {
        id: 0x1234,
        questions: vec![rgbdns::Question {
            name: "www.example".parse().unwrap(),
            qtype: rgbdns::RecordType::A,
            qclass: 1,
        }],
        answers: vec![rgbdns::Record {
            name: "www.example".parse().unwrap(),
            ttl: 300,
            data: rgbdns::RData::A("192.0.2.1".parse().unwrap()),
        }],
        ..Message::default()
    };
    let wire = message.encode().unwrap();
    for length in 0..wire.len() {
        assert!(Message::decode(&wire[..length]).is_err(), "length {length}");
    }
    assert_eq!(Message::decode(&wire).unwrap(), message);
}
```
