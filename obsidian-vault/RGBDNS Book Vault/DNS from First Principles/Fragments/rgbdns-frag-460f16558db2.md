---
type: "code-fragment"
fragment_id: "rgbdns-frag-460f16558db2"
source_path: "src/client.rs"
code_note: "DNS from First Principles/Code/src/client.rs.source"
language: "rust"
subsystem: "Resolution and recursion"
crate: "rgbdns"
symbol: "validate"
kind: "fn"
start_line: 127
end_line: 139
---

# validate

- Fragment ID: `rgbdns-frag-460f16558db2`
- Source file: [[DNS from First Principles/Code/src/client.rs.source|src/client.rs]]
- Lines: 127-139
- Subsystem: [[DNS from First Principles/Subsystems/Resolution and recursion|Resolution and recursion]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-460f16558db2", "codeNote": "DNS from First Principles/Code/src/client.rs.source", "heading": "rgbdns-frag-460f16558db2: fn validate", "sourcePath": "src/client.rs", "startLine": 127, "endLine": 139}
```

## Excerpt

<span id="rgbdns-frag-460f16558db2" class="rgbdns-fragment-target"></span>
### rgbdns-frag-460f16558db2: fn validate

```rust
fn validate(message: Message, id: u16, question: &Question, is_tcp: bool) -> Result<Message> {
    if message.id != id
        || message.flags & 0x8000 == 0
        || message.flags & 0x7800 != 0
        || message.questions.as_slice() != std::slice::from_ref(question)
        || is_tcp && message.flags & 0x0200 != 0
    {
        Err(Error::Format("mismatched DNS response"))
    } else {
        Ok(message)
    }
}

```
