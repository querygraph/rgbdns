---
type: "code-fragment"
fragment_id: "rgbdns-frag-88c395e5e728"
source_path: "src/wall.rs"
code_note: "DNS from First Principles/Code/src/wall.rs.source"
language: "rust"
subsystem: "Specialized responders"
crate: "rgbdns"
symbol: "query"
kind: "fn"
start_line: 67
end_line: 82
---

# query

- Fragment ID: `rgbdns-frag-88c395e5e728`
- Source file: [[DNS from First Principles/Code/src/wall.rs.source|src/wall.rs]]
- Lines: 67-82
- Subsystem: [[DNS from First Principles/Subsystems/Specialized responders|Specialized responders]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-88c395e5e728", "codeNote": "DNS from First Principles/Code/src/wall.rs.source", "heading": "rgbdns-frag-88c395e5e728: fn query", "sourcePath": "src/wall.rs", "startLine": 67, "endLine": 82}
```

## Excerpt

<span id="rgbdns-frag-88c395e5e728" class="rgbdns-fragment-target"></span>
### rgbdns-frag-88c395e5e728: fn query

```rust
    fn query(name: &str, record_type: RecordType) -> Message {
        let wire = Message {
            id: 9,
            questions: vec![Question {
                name: name.parse().unwrap(),
                qtype: record_type,
                qclass: 1,
            }],
            ..Default::default()
        }
        .encode()
        .unwrap();
        Message::decode(&respond(&wire, 512).unwrap()).unwrap()
    }

    #[test]
```
