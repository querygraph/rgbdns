---
type: "code-fragment"
fragment_id: "rgbdns-frag-9990017fae6d"
source_path: "src/axfr.rs"
code_note: "DNS from First Principles/Code/src/axfr.rs.source"
language: "rust"
subsystem: "Transport and zone transfer"
crate: "rgbdns"
symbol: "axfr_client_rejects_spoofable_or_structurally_invalid_messages"
kind: "fn"
start_line: 423
end_line: 457
---

# axfr_client_rejects_spoofable_or_structurally_invalid_messages

- Fragment ID: `rgbdns-frag-9990017fae6d`
- Source file: [[DNS from First Principles/Code/src/axfr.rs.source|src/axfr.rs]]
- Lines: 423-457
- Subsystem: [[DNS from First Principles/Subsystems/Transport and zone transfer|Transport and zone transfer]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-9990017fae6d", "codeNote": "DNS from First Principles/Code/src/axfr.rs.source", "heading": "rgbdns-frag-9990017fae6d: fn axfr_client_rejects_spoofable_or_structurally_invalid_messages", "sourcePath": "src/axfr.rs", "startLine": 423, "endLine": 457}
```

## Excerpt

<span id="rgbdns-frag-9990017fae6d" class="rgbdns-fragment-target"></span>
### rgbdns-frag-9990017fae6d: fn axfr_client_rejects_spoofable_or_structurally_invalid_messages

```rust
    fn axfr_client_rejects_spoofable_or_structurally_invalid_messages() {
        let question = Question {
            name: "example".parse().unwrap(),
            qtype: RecordType::Axfr,
            qclass: 1,
        };
        let valid = Message {
            id: 7,
            flags: 0x8400,
            questions: vec![question.clone()],
            ..Default::default()
        };
        assert!(validate_axfr_message(&valid, 7, &question, true).is_ok());

        let mut wrong_question = valid.clone();
        wrong_question.questions[0].name = "attacker.example".parse().unwrap();
        assert!(validate_axfr_message(&wrong_question, 7, &question, true).is_err());

        let mut truncated = valid.clone();
        truncated.flags |= 0x0200;
        assert!(validate_axfr_message(&truncated, 7, &question, true).is_err());

        let mut non_authoritative = valid.clone();
        non_authoritative.flags &= !0x0400;
        assert!(validate_axfr_message(&non_authoritative, 7, &question, true).is_err());

        let mut authority_data = valid;
        authority_data.authorities.push(Record {
            name: "example".parse().unwrap(),
            ttl: 60,
            data: RData::A(Ipv4Addr::new(192, 0, 2, 1)),
        });
        assert!(validate_axfr_message(&authority_data, 7, &question, true).is_err());
    }
}
```
