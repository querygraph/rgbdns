---
type: "code-fragment"
fragment_id: "rgbdns-frag-ca8414b505a4"
source_path: "src/axfr.rs"
code_note: "DNS from First Principles/Code/src/axfr.rs.source"
language: "rust"
subsystem: "Transport and zone transfer"
crate: "rgbdns"
symbol: "response_wire"
kind: "fn"
start_line: 156
end_line: 171
---

# response_wire

- Fragment ID: `rgbdns-frag-ca8414b505a4`
- Source file: [[DNS from First Principles/Code/src/axfr.rs.source|src/axfr.rs]]
- Lines: 156-171
- Subsystem: [[DNS from First Principles/Subsystems/Transport and zone transfer|Transport and zone transfer]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-ca8414b505a4", "codeNote": "DNS from First Principles/Code/src/axfr.rs.source", "heading": "rgbdns-frag-ca8414b505a4: fn response_wire", "sourcePath": "src/axfr.rs", "startLine": 156, "endLine": 171}
```

## Excerpt

<span id="rgbdns-frag-ca8414b505a4" class="rgbdns-fragment-target"></span>
### rgbdns-frag-ca8414b505a4: fn response_wire

```rust
fn response_wire(
    id: u16,
    question: Option<Question>,
    rcode: u16,
    answers: Vec<Record>,
) -> Result<Vec<u8>> {
    Message {
        id,
        flags: 0x8000 | 0x0400 | rcode,
        questions: question.into_iter().collect(),
        answers,
        ..Default::default()
    }
    .encode()
}

```
