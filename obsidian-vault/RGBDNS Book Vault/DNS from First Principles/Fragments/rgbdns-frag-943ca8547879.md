---
type: "code-fragment"
fragment_id: "rgbdns-frag-943ca8547879"
source_path: "tests/packet_properties.rs"
code_note: "DNS from First Principles/Code/tests/packet_properties.rs.source"
language: "rust"
subsystem: "Tests and performance"
symbol: "accepted_packets_are_stably_reparseable"
kind: "fn"
start_line: 63
end_line: 71
---

# accepted_packets_are_stably_reparseable

- Fragment ID: `rgbdns-frag-943ca8547879`
- Source file: [[DNS from First Principles/Code/tests/packet_properties.rs.source|tests/packet_properties.rs]]
- Lines: 63-71
- Subsystem: [[DNS from First Principles/Subsystems/Tests and performance|Tests and performance]]

```rgbdns-fragment
{"id": "rgbdns-frag-943ca8547879", "codeNote": "DNS from First Principles/Code/tests/packet_properties.rs.source", "heading": "rgbdns-frag-943ca8547879: fn accepted_packets_are_stably_reparseable", "sourcePath": "tests/packet_properties.rs", "startLine": 63, "endLine": 71}
```

## Excerpt

<span id="rgbdns-frag-943ca8547879" class="rgbdns-fragment-target"></span>
### rgbdns-frag-943ca8547879: fn accepted_packets_are_stably_reparseable

```rust
    fn accepted_packets_are_stably_reparseable(bytes in prop::collection::vec(any::<u8>(), 0..2048)) {
        if let Ok(message) = Message::decode(&bytes) {
            let encoded = message.encode().expect("decoded messages must be encodable");
            let reparsed = Message::decode(&encoded).expect("encoder must produce valid wire data");
            prop_assert_eq!(reparsed, message);
        }
    }

    #[test]
```
