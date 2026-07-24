---
type: "code-fragment"
fragment_id: "rgbdns-frag-178e4f13b310"
source_path: "src/name.rs"
code_note: "DNS from First Principles/Code/src/name.rs.source"
language: "rust"
subsystem: "DNS data model and wire codec"
crate: "rgbdns"
symbol: "limits"
kind: "fn"
start_line: 227
end_line: 232
---

# limits

- Fragment ID: `rgbdns-frag-178e4f13b310`
- Source file: [[DNS from First Principles/Code/src/name.rs.source|src/name.rs]]
- Lines: 227-232
- Subsystem: [[DNS from First Principles/Subsystems/DNS data model and wire codec|DNS data model and wire codec]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-178e4f13b310", "codeNote": "DNS from First Principles/Code/src/name.rs.source", "heading": "rgbdns-frag-178e4f13b310: fn limits", "sourcePath": "src/name.rs", "startLine": 227, "endLine": 232}
```

## Excerpt

<span id="rgbdns-frag-178e4f13b310" class="rgbdns-fragment-target"></span>
### rgbdns-frag-178e4f13b310: fn limits

```rust
    fn limits() {
        assert!("x".repeat(64).parse::<Name>().is_err());
        let long = (0..4).map(|_| "x".repeat(63)).collect::<Vec<_>>().join(".");
        assert!(long.parse::<Name>().is_err());
    }
}
```
