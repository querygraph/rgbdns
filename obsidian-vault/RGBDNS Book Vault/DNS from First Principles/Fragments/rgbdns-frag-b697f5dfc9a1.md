---
type: "code-fragment"
fragment_id: "rgbdns-frag-b697f5dfc9a1"
source_path: "src/packet.rs"
code_note: "DNS from First Principles/Code/src/packet.rs.source"
language: "rust"
subsystem: "DNS data model and wire codec"
crate: "rgbdns"
symbol: "rejects_malformed_edns_options_and_address_length"
kind: "fn"
start_line: 929
end_line: 939
---

# rejects_malformed_edns_options_and_address_length

- Fragment ID: `rgbdns-frag-b697f5dfc9a1`
- Source file: [[DNS from First Principles/Code/src/packet.rs.source|src/packet.rs]]
- Lines: 929-939
- Subsystem: [[DNS from First Principles/Subsystems/DNS data model and wire codec|DNS data model and wire codec]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-b697f5dfc9a1", "codeNote": "DNS from First Principles/Code/src/packet.rs.source", "sourcePath": "src/packet.rs", "startLine": 929, "endLine": 939}
```

## Excerpt

<span id="rgbdns-frag-b697f5dfc9a1" class="rgbdns-fragment-target"></span>
### rgbdns-frag-b697f5dfc9a1: fn rejects_malformed_edns_options_and_address_length

```rust
    fn rejects_malformed_edns_options_and_address_length() {
        let malformed_opt = [
            0, 0, 0x01, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 41, 4, 0, 0, 0, 0, 0, 0, 3, 0, 1, 0,
        ];
        assert!(Message::decode(&malformed_opt).is_err());
        let bad_a = [
            0, 0, 0x80, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 1, 0, 3, 1, 2, 3,
        ];
        assert!(Message::decode(&bad_a).is_err());
    }
    #[test]
```
