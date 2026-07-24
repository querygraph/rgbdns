---
type: "code-fragment"
fragment_id: "rgbdns-frag-e6804860354f"
source_path: "tests/support/mod.rs"
code_note: "DNS from First Principles/Code/tests/support/mod.rs.source"
language: "rust"
subsystem: "Tests and performance"
symbol: "query"
kind: "fn"
start_line: 19
end_line: 31
---

# query

- Fragment ID: `rgbdns-frag-e6804860354f`
- Source file: [[DNS from First Principles/Code/tests/support/mod.rs.source|tests/support/mod.rs]]
- Lines: 19-31
- Subsystem: [[DNS from First Principles/Subsystems/Tests and performance|Tests and performance]]

```rgbdns-fragment
{"id": "rgbdns-frag-e6804860354f", "codeNote": "DNS from First Principles/Code/tests/support/mod.rs.source", "heading": "rgbdns-frag-e6804860354f: fn query", "sourcePath": "tests/support/mod.rs", "startLine": 19, "endLine": 31}
```

## Excerpt

<span id="rgbdns-frag-e6804860354f" class="rgbdns-fragment-target"></span>
### rgbdns-frag-e6804860354f: fn query

```rust
pub fn query(name: &str, qtype: RecordType) -> Message {
    Message {
        id: ID,
        flags: 0x0100,
        questions: vec![Question {
            name: name.parse().unwrap(),
            qtype,
            qclass: 1,
        }],
        ..Message::default()
    }
}

```
