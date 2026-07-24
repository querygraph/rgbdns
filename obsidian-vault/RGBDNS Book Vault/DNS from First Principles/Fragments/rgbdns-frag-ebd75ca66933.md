---
type: "code-fragment"
fragment_id: "rgbdns-frag-ebd75ca66933"
source_path: "tests/support/mod.rs"
code_note: "DNS from First Principles/Code/tests/support/mod.rs.source"
language: "rust"
subsystem: "Tests and performance"
symbol: "zone"
kind: "fn"
start_line: 7
end_line: 18
---

# zone

- Fragment ID: `rgbdns-frag-ebd75ca66933`
- Source file: [[DNS from First Principles/Code/tests/support/mod.rs.source|tests/support/mod.rs]]
- Lines: 7-18
- Subsystem: [[DNS from First Principles/Subsystems/Tests and performance|Tests and performance]]

```rgbdns-fragment
{"id": "rgbdns-frag-ebd75ca66933", "codeNote": "DNS from First Principles/Code/tests/support/mod.rs.source", "heading": "rgbdns-frag-ebd75ca66933: fn zone", "sourcePath": "tests/support/mod.rs", "startLine": 7, "endLine": 18}
```

## Excerpt

<span id="rgbdns-frag-ebd75ca66933" class="rgbdns-fragment-target"></span>
### rgbdns-frag-ebd75ca66933: fn zone

```rust
pub fn zone() -> Zone {
    Zone::parse(
        ".example:192.0.2.53:ns:300\n\
         +www.example:192.0.2.1:300\n\
         3www.example:20010db8000000000000000000000001:300\n\
         Calias.example:www.example:300\n\
         'txt.example:first\\072segment:300\n\
         &child.example:192.0.2.54:ns.child.example:300\n",
    )
    .unwrap()
}

```
