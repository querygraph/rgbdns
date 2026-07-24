---
type: "code-fragment"
fragment_id: "rgbdns-frag-50dd1f4e2ab3"
source_path: "tests/packet_properties.rs"
code_note: "DNS from First Principles/Code/tests/packet_properties.rs.source"
language: "rust"
subsystem: "Tests and performance"
symbol: "structured_messages_roundtrip_without_semantic_loss"
kind: "fn"
start_line: 72
end_line: 78
---

# structured_messages_roundtrip_without_semantic_loss

- Fragment ID: `rgbdns-frag-50dd1f4e2ab3`
- Source file: [[DNS from First Principles/Code/tests/packet_properties.rs.source|tests/packet_properties.rs]]
- Lines: 72-78
- Subsystem: [[DNS from First Principles/Subsystems/Tests and performance|Tests and performance]]

```rgbdns-fragment
{"id": "rgbdns-frag-50dd1f4e2ab3", "codeNote": "DNS from First Principles/Code/tests/packet_properties.rs.source", "heading": "rgbdns-frag-50dd1f4e2ab3: fn structured_messages_roundtrip_without_semantic_loss", "sourcePath": "tests/packet_properties.rs", "startLine": 72, "endLine": 78}
```

## Excerpt

<span id="rgbdns-frag-50dd1f4e2ab3" class="rgbdns-fragment-target"></span>
### rgbdns-frag-50dd1f4e2ab3: fn structured_messages_roundtrip_without_semantic_loss

```rust
    fn structured_messages_roundtrip_without_semantic_loss(message in structured_message()) {
        let encoded = message.encode().expect("bounded generated message must encode");
        let decoded = Message::decode(&encoded).expect("generated encoding must decode");
        prop_assert_eq!(decoded, message);
    }

    #[test]
```
