---
type: "code-fragment"
fragment_id: "rgbdns-frag-fbc8bb9e3567"
source_path: "src/rbl.rs"
code_note: "DNS from First Principles/Code/src/rbl.rs.source"
language: "rust"
subsystem: "Specialized responders"
crate: "rgbdns"
symbol: "query"
kind: "fn"
start_line: 226
end_line: 240
---

# query

- Fragment ID: `rgbdns-frag-fbc8bb9e3567`
- Source file: [[DNS from First Principles/Code/src/rbl.rs.source|src/rbl.rs]]
- Lines: 226-240
- Subsystem: [[DNS from First Principles/Subsystems/Specialized responders|Specialized responders]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-fbc8bb9e3567", "codeNote": "DNS from First Principles/Code/src/rbl.rs.source", "heading": "rgbdns-frag-fbc8bb9e3567: fn query", "sourcePath": "src/rbl.rs", "startLine": 226, "endLine": 240}
```

## Excerpt

<span id="rgbdns-frag-fbc8bb9e3567" class="rgbdns-fragment-target"></span>
### rgbdns-frag-fbc8bb9e3567: fn query

```rust
    fn query(name: &str, record_type: RecordType) -> Vec<u8> {
        Message {
            id: 7,
            questions: vec![Question {
                name: name.parse().unwrap(),
                qtype: record_type,
                qclass: 1,
            }],
            ..Default::default()
        }
        .encode()
        .unwrap()
    }

    #[test]
```
