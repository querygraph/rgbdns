---
type: "code-fragment"
fragment_id: "rgbdns-frag-6a8de18b800c"
source_path: "tests/rfc_conformance.rs"
code_note: "DNS from First Principles/Code/tests/rfc_conformance.rs.source"
language: "rust"
subsystem: "Tests and performance"
symbol: "rfc8906_all_unimplemented_opcodes_get_notimp_without_flag_reflection"
kind: "fn"
start_line: 52
end_line: 65
---

# rfc8906_all_unimplemented_opcodes_get_notimp_without_flag_reflection

- Fragment ID: `rgbdns-frag-6a8de18b800c`
- Source file: [[DNS from First Principles/Code/tests/rfc_conformance.rs.source|tests/rfc_conformance.rs]]
- Lines: 52-65
- Subsystem: [[DNS from First Principles/Subsystems/Tests and performance|Tests and performance]]

```rgbdns-fragment
{"id": "rgbdns-frag-6a8de18b800c", "codeNote": "DNS from First Principles/Code/tests/rfc_conformance.rs.source", "heading": "rgbdns-frag-6a8de18b800c: fn rfc8906_all_unimplemented_opcodes_get_notimp_without_flag_reflection", "sourcePath": "tests/rfc_conformance.rs", "startLine": 52, "endLine": 65}
```

## Excerpt

<span id="rgbdns-frag-6a8de18b800c" class="rgbdns-fragment-target"></span>
### rgbdns-frag-6a8de18b800c: fn rfc8906_all_unimplemented_opcodes_get_notimp_without_flag_reflection

```rust
fn rfc8906_all_unimplemented_opcodes_get_notimp_without_flag_reflection() {
    for opcode in 1_u16..=15 {
        let mut wire = [0_u8; 12];
        wire[..2].copy_from_slice(&ID.to_be_bytes());
        wire[2..4].copy_from_slice(&(opcode << 11 | 0x0170).to_be_bytes());
        let answer =
            Message::decode(&rgbdns::server::respond(&zone(), &wire, 4096).unwrap()).unwrap();
        assert_eq!(rcode(&answer), 4, "opcode {opcode}");
        assert_eq!(answer.flags & 0x7800, opcode << 11, "opcode {opcode}");
        assert_eq!(answer.flags & 0x0070, 0, "opcode {opcode}");
    }
}

#[test]
```
