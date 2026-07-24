---
type: "code-fragment"
fragment_id: "rgbdns-frag-c608c25b9171"
source_path: "src/packet.rs"
code_note: "DNS from First Principles/Code/src/packet.rs.source"
language: "rust"
subsystem: "DNS data model and wire codec"
crate: "rgbdns"
symbol: "rejects_trailing_packet_data_and_forward_compression"
kind: "fn"
start_line: 634
end_line: 653
---

# rejects_trailing_packet_data_and_forward_compression

- Fragment ID: `rgbdns-frag-c608c25b9171`
- Source file: [[DNS from First Principles/Code/src/packet.rs.source|src/packet.rs]]
- Lines: 634-653
- Subsystem: [[DNS from First Principles/Subsystems/DNS data model and wire codec|DNS data model and wire codec]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-c608c25b9171", "codeNote": "DNS from First Principles/Code/src/packet.rs.source", "heading": "rgbdns-frag-c608c25b9171: fn rejects_trailing_packet_data_and_forward_compression", "sourcePath": "src/packet.rs", "startLine": 634, "endLine": 653}
```

## Excerpt

<span id="rgbdns-frag-c608c25b9171" class="rgbdns-fragment-target"></span>
### rgbdns-frag-c608c25b9171: fn rejects_trailing_packet_data_and_forward_compression

```rust
    fn rejects_trailing_packet_data_and_forward_compression() {
        let mut packet = Message {
            questions: vec![Question {
                name: "example".parse().unwrap(),
                qtype: RecordType::A,
                qclass: 1,
            }],
            ..Default::default()
        }
        .encode()
        .unwrap();
        packet.push(0);
        assert!(Message::decode(&packet).is_err());

        let mut forward = vec![0; 12];
        forward[5] = 1;
        forward.extend([0xc0, 18, 0, 1, 0, 1, 0]);
        assert!(Message::decode(&forward).is_err());
    }
    #[test]
```
