---
type: "code-fragment"
fragment_id: "rgbdns-frag-7b48fc998842"
source_path: "src/zone.rs"
code_note: "DNS from First Principles/Code/src/zone.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "escaped_colon"
kind: "fn"
start_line: 1089
end_line: 1092
---

# escaped_colon

- Fragment ID: `rgbdns-frag-7b48fc998842`
- Source file: [[DNS from First Principles/Code/src/zone.rs.source|src/zone.rs]]
- Lines: 1089-1092
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-7b48fc998842", "codeNote": "DNS from First Principles/Code/src/zone.rs.source", "sourcePath": "src/zone.rs", "startLine": 1089, "endLine": 1092}
```

## Excerpt

<span id="rgbdns-frag-7b48fc998842" class="rgbdns-fragment-target"></span>
### rgbdns-frag-7b48fc998842: fn escaped_colon

```rust
    fn escaped_colon() {
        assert_eq!(unescape(r"a\072b").unwrap(), b"a:b")
    }
    #[test]
```
