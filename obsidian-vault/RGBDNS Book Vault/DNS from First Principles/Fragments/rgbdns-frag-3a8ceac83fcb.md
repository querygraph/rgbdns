---
type: "code-fragment"
fragment_id: "rgbdns-frag-3a8ceac83fcb"
source_path: "src/tai64.rs"
code_note: "DNS from First Principles/Code/src/tai64.rs.source"
language: "rust"
subsystem: "Operations and supervision"
crate: "rgbdns"
symbol: "rejects_malformed_and_out_of_range_labels"
kind: "fn"
start_line: 234
end_line: 239
---

# rejects_malformed_and_out_of_range_labels

- Fragment ID: `rgbdns-frag-3a8ceac83fcb`
- Source file: [[DNS from First Principles/Code/src/tai64.rs.source|src/tai64.rs]]
- Lines: 234-239
- Subsystem: [[DNS from First Principles/Subsystems/Operations and supervision|Operations and supervision]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-3a8ceac83fcb", "codeNote": "DNS from First Principles/Code/src/tai64.rs.source", "heading": "rgbdns-frag-3a8ceac83fcb: fn rejects_malformed_and_out_of_range_labels", "sourcePath": "src/tai64.rs", "startLine": 234, "endLine": 239}
```

## Excerpt

<span id="rgbdns-frag-3a8ceac83fcb" class="rgbdns-fragment-target"></span>
### rgbdns-frag-3a8ceac83fcb: fn rejects_malformed_and_out_of_range_labels

```rust
    fn rejects_malformed_and_out_of_range_labels() {
        assert!(parse_label("4000000037c219bf2ef02e94").is_none());
        assert!(parse_label("@4000000037c219bfzzzzzzzz").is_none());
        assert!(parse_label("@4000000037c219bf3b9aca00").is_none());
    }
}
```
