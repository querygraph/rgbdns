---
type: "code-fragment"
fragment_id: "rgbdns-frag-477ba6f85fdb"
source_path: "src/zone.rs"
code_note: "DNS from First Principles/Code/src/zone.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "text_escapes_use_one_to_three_octal_digits"
kind: "fn"
start_line: 1164
end_line: 1168
---

# text_escapes_use_one_to_three_octal_digits

- Fragment ID: `rgbdns-frag-477ba6f85fdb`
- Source file: [[DNS from First Principles/Code/src/zone.rs.source|src/zone.rs]]
- Lines: 1164-1168
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-477ba6f85fdb", "codeNote": "DNS from First Principles/Code/src/zone.rs.source", "sourcePath": "src/zone.rs", "startLine": 1164, "endLine": 1168}
```

## Excerpt

<span id="rgbdns-frag-477ba6f85fdb" class="rgbdns-fragment-target"></span>
### rgbdns-frag-477ba6f85fdb: fn text_escapes_use_one_to_three_octal_digits

```rust
    fn text_escapes_use_one_to_three_octal_digits() {
        assert_eq!(unescape(r"\1\12\123\8").unwrap(), [1, 10, 83, b'8']);
    }

    #[test]
```
