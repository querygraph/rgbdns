---
type: "code-fragment"
fragment_id: "rgbdns-frag-b1201af3a743"
source_path: "src/name.rs"
code_note: "DNS from First Principles/Code/src/name.rs.source"
language: "rust"
subsystem: "DNS data model and wire codec"
crate: "rgbdns"
symbol: "escapes"
kind: "fn"
start_line: 237
end_line: 244
---

# escapes

- Fragment ID: `rgbdns-frag-b1201af3a743`
- Source file: [[DNS from First Principles/Code/src/name.rs.source|src/name.rs]]
- Lines: 237-244
- Subsystem: [[DNS from First Principles/Subsystems/DNS data model and wire codec|DNS data model and wire codec]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-b1201af3a743", "codeNote": "DNS from First Principles/Code/src/name.rs.source", "sourcePath": "src/name.rs", "startLine": 237, "endLine": 244}
```

## Excerpt

<span id="rgbdns-frag-b1201af3a743" class="rgbdns-fragment-target"></span>
### rgbdns-frag-b1201af3a743: fn escapes

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
