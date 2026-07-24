---
type: "code-fragment"
fragment_id: "rgbdns-frag-8e52eff80870"
source_path: "tests/wire_security.rs"
code_note: "DNS from First Principles/Code/tests/wire_security.rs.source"
language: "rust"
subsystem: "Tests and performance"
symbol: "compression_pointer_must_target_a_previous_name_boundary"
kind: "fn"
start_line: 71
end_line: 87
---

# compression_pointer_must_target_a_previous_name_boundary

- Fragment ID: `rgbdns-frag-8e52eff80870`
- Source file: [[DNS from First Principles/Code/tests/wire_security.rs.source|tests/wire_security.rs]]
- Lines: 71-87
- Subsystem: [[DNS from First Principles/Subsystems/Tests and performance|Tests and performance]]

```rgbdns-fragment
{"id": "rgbdns-frag-8e52eff80870", "codeNote": "DNS from First Principles/Code/tests/wire_security.rs.source", "heading": "rgbdns-frag-8e52eff80870: fn compression_pointer_must_target_a_previous_name_boundary", "sourcePath": "tests/wire_security.rs", "startLine": 71, "endLine": 87}
```

## Excerpt

<span id="rgbdns-frag-8e52eff80870" class="rgbdns-fragment-target"></span>
### rgbdns-frag-8e52eff80870: fn compression_pointer_must_target_a_previous_name_boundary

```rust
fn compression_pointer_must_target_a_previous_name_boundary() {
    // The question starts with a one-byte label containing NUL. Offset 13
    // (inside that label) happens to decode as a syntactically valid name,
    // but RFC 1035 pointers may only refer to a prior name occurrence.
    let mut wire = standard_header(1, 1, 0, 0);
    wire.extend([1, 0, 0, 0, 1, 0, 1]);
    wire.extend([
        0xc0, 0x0d, // illegal pointer into the question label
        0, 1, // A
        0, 1, // IN
        0, 0, 0, 1, // TTL
        0, 4, 192, 0, 2, 1,
    ]);
    assert!(Message::decode(&wire).is_err());
}

#[test]
```
