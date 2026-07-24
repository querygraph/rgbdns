---
type: "code-fragment"
fragment_id: "rgbdns-frag-e49f18aa0a17"
source_path: "src/zone.rs"
code_note: "DNS from First Principles/Code/src/zone.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "text_escapes_use_one_to_three_octal_digits"
kind: "fn"
start_line: 826
end_line: 830
---

# text_escapes_use_one_to_three_octal_digits

- Fragment ID: `rgbdns-frag-e49f18aa0a17`
- Source file: [[DNS from First Principles/Code/src/zone.rs.source|src/zone.rs]]
- Lines: 826-830
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-e49f18aa0a17", "codeNote": "DNS from First Principles/Code/src/zone.rs.source", "heading": "rgbdns-frag-e49f18aa0a17: fn text_escapes_use_one_to_three_octal_digits", "sourcePath": "src/zone.rs", "startLine": 826, "endLine": 830}
```

## Excerpt

<span id="rgbdns-frag-e49f18aa0a17" class="rgbdns-fragment-target"></span>
### rgbdns-frag-e49f18aa0a17: fn text_escapes_use_one_to_three_octal_digits

```rust
    fn text_escapes_use_one_to_three_octal_digits() {
        assert_eq!(unescape(r"\1\12\123\8").unwrap(), [1, 10, 83, b'8']);
    }

    #[test]
```
