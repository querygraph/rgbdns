---
type: "code-fragment"
fragment_id: "rgbdns-frag-f618d3cd6667"
source_path: "src/pick.rs"
code_note: "DNS from First Principles/Code/src/pick.rs.source"
language: "rust"
subsystem: "Specialized responders"
crate: "rgbdns"
symbol: "query"
kind: "fn"
start_line: 234
end_line: 248
---

# query

- Fragment ID: `rgbdns-frag-f618d3cd6667`
- Source file: [[DNS from First Principles/Code/src/pick.rs.source|src/pick.rs]]
- Lines: 234-248
- Subsystem: [[DNS from First Principles/Subsystems/Specialized responders|Specialized responders]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-f618d3cd6667", "codeNote": "DNS from First Principles/Code/src/pick.rs.source", "sourcePath": "src/pick.rs", "startLine": 234, "endLine": 248}
```

## Excerpt

<span id="rgbdns-frag-f618d3cd6667" class="rgbdns-fragment-target"></span>
### rgbdns-frag-f618d3cd6667: fn query

```rust
    fn query(name: &str) -> Vec<u8> {
        Message {
            id: 22,
            questions: vec![Question {
                name: name.parse().unwrap(),
                qtype: RecordType::A,
                qclass: 1,
            }],
            ..Default::default()
        }
        .encode()
        .unwrap()
    }

    #[test]
```
