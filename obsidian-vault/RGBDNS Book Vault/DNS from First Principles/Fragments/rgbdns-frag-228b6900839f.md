---
type: "code-fragment"
fragment_id: "rgbdns-frag-228b6900839f"
source_path: "tests/support/mod.rs"
code_note: "DNS from First Principles/Code/tests/support/mod.rs.source"
language: "rust"
subsystem: "Tests and performance"
symbol: "rcode"
kind: "fn"
start_line: 51
end_line: 54
---

# rcode

- Fragment ID: `rgbdns-frag-228b6900839f`
- Source file: [[DNS from First Principles/Code/tests/support/mod.rs.source|tests/support/mod.rs]]
- Lines: 51-54
- Subsystem: [[DNS from First Principles/Subsystems/Tests and performance|Tests and performance]]

```rgbdns-fragment
{"id": "rgbdns-frag-228b6900839f", "codeNote": "DNS from First Principles/Code/tests/support/mod.rs.source", "heading": "rgbdns-frag-228b6900839f: fn rcode", "sourcePath": "tests/support/mod.rs", "startLine": 51, "endLine": 54}
```

## Excerpt

<span id="rgbdns-frag-228b6900839f" class="rgbdns-fragment-target"></span>
### rgbdns-frag-228b6900839f: fn rcode

```rust
pub fn rcode(message: &Message) -> u16 {
    message.flags & 0x000f
}

```
