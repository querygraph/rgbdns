---
type: "code-fragment"
fragment_id: "rgbdns-frag-d3e351ff4ce5"
source_path: "tests/wire_security.rs"
code_note: "DNS from First Principles/Code/tests/wire_security.rs.source"
language: "rust"
subsystem: "Tests and performance"
symbol: "standard_header"
kind: "fn"
start_line: 3
end_line: 11
---

# standard_header

- Fragment ID: `rgbdns-frag-d3e351ff4ce5`
- Source file: [[DNS from First Principles/Code/tests/wire_security.rs.source|tests/wire_security.rs]]
- Lines: 3-11
- Subsystem: [[DNS from First Principles/Subsystems/Tests and performance|Tests and performance]]

```rgbdns-fragment
{"id": "rgbdns-frag-d3e351ff4ce5", "codeNote": "DNS from First Principles/Code/tests/wire_security.rs.source", "heading": "rgbdns-frag-d3e351ff4ce5: fn standard_header", "sourcePath": "tests/wire_security.rs", "startLine": 3, "endLine": 11}
```

## Excerpt

<span id="rgbdns-frag-d3e351ff4ce5" class="rgbdns-fragment-target"></span>
### rgbdns-frag-d3e351ff4ce5: fn standard_header

```rust
fn standard_header(qd: u16, an: u16, ns: u16, ar: u16) -> Vec<u8> {
    let mut wire = Vec::with_capacity(12);
    for value in [0x1234, 0, qd, an, ns, ar] {
        wire.extend(value.to_be_bytes());
    }
    wire
}

#[test]
```
