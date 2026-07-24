---
type: "code-fragment"
fragment_id: "rgbdns-frag-3533f4b0fcb7"
source_path: "tests/packet_properties.rs"
code_note: "DNS from First Principles/Code/tests/packet_properties.rs.source"
language: "rust"
subsystem: "Tests and performance"
symbol: "arbitrary_packets_never_panic"
kind: "fn"
start_line: 58
end_line: 62
---

# arbitrary_packets_never_panic

- Fragment ID: `rgbdns-frag-3533f4b0fcb7`
- Source file: [[DNS from First Principles/Code/tests/packet_properties.rs.source|tests/packet_properties.rs]]
- Lines: 58-62
- Subsystem: [[DNS from First Principles/Subsystems/Tests and performance|Tests and performance]]

```rgbdns-fragment
{"id": "rgbdns-frag-3533f4b0fcb7", "codeNote": "DNS from First Principles/Code/tests/packet_properties.rs.source", "heading": "rgbdns-frag-3533f4b0fcb7: fn arbitrary_packets_never_panic", "sourcePath": "tests/packet_properties.rs", "startLine": 58, "endLine": 62}
```

## Excerpt

<span id="rgbdns-frag-3533f4b0fcb7" class="rgbdns-fragment-target"></span>
### rgbdns-frag-3533f4b0fcb7: fn arbitrary_packets_never_panic

```rust
    fn arbitrary_packets_never_panic(bytes in prop::collection::vec(any::<u8>(), 0..4096)) {
        let _ = Message::decode(&bytes);
    }

    #[test]
```
