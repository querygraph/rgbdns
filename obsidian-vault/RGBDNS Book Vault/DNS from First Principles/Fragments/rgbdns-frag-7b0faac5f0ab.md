---
type: "code-fragment"
fragment_id: "rgbdns-frag-7b0faac5f0ab"
source_path: "src/name.rs"
code_note: "DNS from First Principles/Code/src/name.rs.source"
language: "rust"
subsystem: "DNS data model and wire codec"
crate: "rgbdns"
symbol: "escapes"
kind: "fn"
start_line: 219
end_line: 226
---

# escapes

- Fragment ID: `rgbdns-frag-7b0faac5f0ab`
- Source file: [[DNS from First Principles/Code/src/name.rs.source|src/name.rs]]
- Lines: 219-226
- Subsystem: [[DNS from First Principles/Subsystems/DNS data model and wire codec|DNS data model and wire codec]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-7b0faac5f0ab", "codeNote": "DNS from First Principles/Code/src/name.rs.source", "heading": "rgbdns-frag-7b0faac5f0ab: fn escapes", "sourcePath": "src/name.rs", "startLine": 219, "endLine": 226}
```

## Excerpt

<span id="rgbdns-frag-7b0faac5f0ab" class="rgbdns-fragment-target"></span>
### rgbdns-frag-7b0faac5f0ab: fn escapes

```rust
    fn escapes() {
        assert_eq!(
            r"a\.b.example".parse::<Name>().unwrap().to_string(),
            r"a\.b.example."
        );
        assert!(r"\999".parse::<Name>().is_err());
    }
    #[test]
```
