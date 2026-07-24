---
type: "code-fragment"
fragment_id: "rgbdns-frag-4c047cc0095a"
source_path: "tests/support/mod.rs"
code_note: "DNS from First Principles/Code/tests/support/mod.rs.source"
language: "rust"
subsystem: "Tests and performance"
symbol: "extended_rcode"
kind: "fn"
start_line: 55
end_line: 65
---

# extended_rcode

- Fragment ID: `rgbdns-frag-4c047cc0095a`
- Source file: [[DNS from First Principles/Code/tests/support/mod.rs.source|tests/support/mod.rs]]
- Lines: 55-65
- Subsystem: [[DNS from First Principles/Subsystems/Tests and performance|Tests and performance]]

```rgbdns-fragment
{"id": "rgbdns-frag-4c047cc0095a", "codeNote": "DNS from First Principles/Code/tests/support/mod.rs.source", "heading": "rgbdns-frag-4c047cc0095a: fn extended_rcode", "sourcePath": "tests/support/mod.rs", "startLine": 55, "endLine": 65}
```

## Excerpt

<span id="rgbdns-frag-4c047cc0095a" class="rgbdns-fragment-target"></span>
### rgbdns-frag-4c047cc0095a: fn extended_rcode

```rust
pub fn extended_rcode(message: &Message) -> u16 {
    let extension = message
        .additionals
        .iter()
        .find_map(|record| match record.data {
            RData::Opt { extended_rcode, .. } => Some(u16::from(extended_rcode)),
            _ => None,
        })
        .unwrap_or(0);
    extension << 4 | rcode(message)
}
```
