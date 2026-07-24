---
type: "code-fragment"
fragment_id: "rgbdns-frag-0eb5901d07d1"
source_path: "src/zone.rs"
code_note: "DNS from First Principles/Code/src/zone.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "escaped_colon"
kind: "fn"
start_line: 751
end_line: 754
---

# escaped_colon

- Fragment ID: `rgbdns-frag-0eb5901d07d1`
- Source file: [[DNS from First Principles/Code/src/zone.rs.source|src/zone.rs]]
- Lines: 751-754
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-0eb5901d07d1", "codeNote": "DNS from First Principles/Code/src/zone.rs.source", "heading": "rgbdns-frag-0eb5901d07d1: fn escaped_colon", "sourcePath": "src/zone.rs", "startLine": 751, "endLine": 754}
```

## Excerpt

<span id="rgbdns-frag-0eb5901d07d1" class="rgbdns-fragment-target"></span>
### rgbdns-frag-0eb5901d07d1: fn escaped_colon

```rust
    fn escaped_colon() {
        assert_eq!(unescape(r"a\072b").unwrap(), b"a:b")
    }
    #[test]
```
