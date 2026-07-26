---
type: "code-fragment"
fragment_id: "rgbdns-frag-c624c9699d4f"
source_path: "src/zone.rs"
code_note: "DNS from First Principles/Code/src/zone.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "escaped_colon"
kind: "fn"
start_line: 821
end_line: 824
---

# escaped_colon

- Fragment ID: `rgbdns-frag-c624c9699d4f`
- Source file: [[DNS from First Principles/Code/src/zone.rs.source|src/zone.rs]]
- Lines: 821-824
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-c624c9699d4f", "codeNote": "DNS from First Principles/Code/src/zone.rs.source", "heading": "rgbdns-frag-c624c9699d4f: fn escaped_colon", "sourcePath": "src/zone.rs", "startLine": 821, "endLine": 824}
```

## Excerpt

<span id="rgbdns-frag-c624c9699d4f" class="rgbdns-fragment-target"></span>
### rgbdns-frag-c624c9699d4f: fn escaped_colon

```rust
    fn escaped_colon() {
        assert_eq!(unescape(r"a\072b").unwrap(), b"a:b")
    }
    #[test]
```
