---
type: "code-fragment"
fragment_id: "rgbdns-frag-226221718a5a"
source_path: "src/pick.rs"
code_note: "DNS from First Principles/Code/src/pick.rs.source"
language: "rust"
subsystem: "Specialized responders"
crate: "rgbdns"
symbol: "query"
kind: "fn"
start_line: 232
end_line: 246
---

# query

- Fragment ID: `rgbdns-frag-226221718a5a`
- Source file: [[DNS from First Principles/Code/src/pick.rs.source|src/pick.rs]]
- Lines: 232-246
- Subsystem: [[DNS from First Principles/Subsystems/Specialized responders|Specialized responders]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-226221718a5a", "codeNote": "DNS from First Principles/Code/src/pick.rs.source", "heading": "rgbdns-frag-226221718a5a: fn query", "sourcePath": "src/pick.rs", "startLine": 232, "endLine": 246}
```

## Excerpt

<span id="rgbdns-frag-226221718a5a" class="rgbdns-fragment-target"></span>
### rgbdns-frag-226221718a5a: fn query

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
