---
type: "code-fragment"
fragment_id: "rgbdns-frag-b384fa6572b4"
source_path: "src/name.rs"
code_note: "DNS from First Principles/Code/src/name.rs.source"
language: "rust"
subsystem: "DNS data model and wire codec"
crate: "rgbdns"
symbol: "limits"
kind: "fn"
start_line: 245
end_line: 250
---

# limits

- Fragment ID: `rgbdns-frag-b384fa6572b4`
- Source file: [[DNS from First Principles/Code/src/name.rs.source|src/name.rs]]
- Lines: 245-250
- Subsystem: [[DNS from First Principles/Subsystems/DNS data model and wire codec|DNS data model and wire codec]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-b384fa6572b4", "codeNote": "DNS from First Principles/Code/src/name.rs.source", "sourcePath": "src/name.rs", "startLine": 245, "endLine": 250}
```

## Excerpt

<span id="rgbdns-frag-b384fa6572b4" class="rgbdns-fragment-target"></span>
### rgbdns-frag-b384fa6572b4: fn limits

```rust
    fn limits() {
        assert!("x".repeat(64).parse::<Name>().is_err());
        let long = (0..4).map(|_| "x".repeat(63)).collect::<Vec<_>>().join(".");
        assert!(long.parse::<Name>().is_err());
    }
}
```
