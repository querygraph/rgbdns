---
type: "code-fragment"
fragment_id: "rgbdns-frag-d74635b61765"
source_path: "tests/packet_properties.rs"
code_note: "DNS from First Principles/Code/tests/packet_properties.rs.source"
language: "rust"
subsystem: "Tests and performance"
symbol: "structured_message"
kind: "fn"
start_line: 26
end_line: 57
---

# structured_message

- Fragment ID: `rgbdns-frag-d74635b61765`
- Source file: [[DNS from First Principles/Code/tests/packet_properties.rs.source|tests/packet_properties.rs]]
- Lines: 26-57
- Subsystem: [[DNS from First Principles/Subsystems/Tests and performance|Tests and performance]]

```rgbdns-fragment
{"id": "rgbdns-frag-d74635b61765", "codeNote": "DNS from First Principles/Code/tests/packet_properties.rs.source", "heading": "rgbdns-frag-d74635b61765: fn structured_message", "sourcePath": "tests/packet_properties.rs", "startLine": 26, "endLine": 57}
```

## Excerpt

<span id="rgbdns-frag-d74635b61765" class="rgbdns-fragment-target"></span>
### rgbdns-frag-d74635b61765: fn structured_message

```rust
fn structured_message() -> impl Strategy<Value = Message> {
    (
        any::<u16>(),
        any::<u16>(),
        prop::collection::vec((dns_name(), 0_u16..=u16::MAX), 0..=4),
        prop::collection::vec(record(), 0..=16),
        prop::collection::vec(record(), 0..=8),
        prop::collection::vec(record(), 0..=8),
    )
        .prop_map(
            |(id, flags, questions, answers, authorities, additionals)| Message {
                id,
                flags,
                questions: questions
                    .into_iter()
                    .map(|(name, code)| Question {
                        name,
                        qtype: RecordType::from_code(code),
                        qclass: 1,
                    })
                    .collect(),
                answers,
                authorities,
                additionals,
            },
        )
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(10_000))]

    #[test]
```
