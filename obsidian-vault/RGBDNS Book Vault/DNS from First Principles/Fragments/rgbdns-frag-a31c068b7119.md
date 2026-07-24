---
type: "code-fragment"
fragment_id: "rgbdns-frag-a31c068b7119"
source_path: "src/axfr.rs"
code_note: "DNS from First Principles/Code/src/axfr.rs.source"
language: "rust"
subsystem: "Transport and zone transfer"
crate: "rgbdns"
symbol: "validate_axfr_message"
kind: "fn"
start_line: 230
end_line: 255
---

# validate_axfr_message

- Fragment ID: `rgbdns-frag-a31c068b7119`
- Source file: [[DNS from First Principles/Code/src/axfr.rs.source|src/axfr.rs]]
- Lines: 230-255
- Subsystem: [[DNS from First Principles/Subsystems/Transport and zone transfer|Transport and zone transfer]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-a31c068b7119", "codeNote": "DNS from First Principles/Code/src/axfr.rs.source", "heading": "rgbdns-frag-a31c068b7119: fn validate_axfr_message", "sourcePath": "src/axfr.rs", "startLine": 230, "endLine": 255}
```

## Excerpt

<span id="rgbdns-frag-a31c068b7119" class="rgbdns-fragment-target"></span>
### rgbdns-frag-a31c068b7119: fn validate_axfr_message

```rust
fn validate_axfr_message(
    response: &Message,
    id: u16,
    question: &Question,
    first: bool,
) -> Result<()> {
    let valid_question = if first {
        response.questions.as_slice() == std::slice::from_ref(question)
    } else {
        response.questions.is_empty()
            || response.questions.as_slice() == std::slice::from_ref(question)
    };
    if response.id != id
        || response.flags & 0x8000 == 0
        || response.flags & 0x7800 != 0
        || response.flags & 0x0200 != 0
        || response.flags & 0x000f == 0 && response.flags & 0x0400 == 0
        || !valid_question
        || !response.authorities.is_empty()
    {
        Err(Error::Format("invalid AXFR response"))
    } else {
        Ok(())
    }
}

```
