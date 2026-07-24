---
type: "code-fragment"
fragment_id: "rgbdns-frag-2daa3b3665ed"
source_path: "src/wall.rs"
code_note: "DNS from First Principles/Code/src/wall.rs.source"
language: "rust"
subsystem: "Specialized responders"
crate: "rgbdns"
symbol: "partial_reverse_names_have_self_ptr"
kind: "fn"
start_line: 96
end_line: 104
---

# partial_reverse_names_have_self_ptr

- Fragment ID: `rgbdns-frag-2daa3b3665ed`
- Source file: [[DNS from First Principles/Code/src/wall.rs.source|src/wall.rs]]
- Lines: 96-104
- Subsystem: [[DNS from First Principles/Subsystems/Specialized responders|Specialized responders]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-2daa3b3665ed", "codeNote": "DNS from First Principles/Code/src/wall.rs.source", "heading": "rgbdns-frag-2daa3b3665ed: fn partial_reverse_names_have_self_ptr", "sourcePath": "src/wall.rs", "startLine": 96, "endLine": 104}
```

## Excerpt

<span id="rgbdns-frag-2daa3b3665ed" class="rgbdns-fragment-target"></span>
### rgbdns-frag-2daa3b3665ed: fn partial_reverse_names_have_self_ptr

```rust
    fn partial_reverse_names_have_self_ptr() {
        let response = query("3.2.1.in-addr.arpa", RecordType::Ptr);
        assert_eq!(response.answers.len(), 1);
        assert_eq!(
            response.answers[0].name,
            "3.2.1.in-addr.arpa".parse().unwrap()
        );
    }
}
```
