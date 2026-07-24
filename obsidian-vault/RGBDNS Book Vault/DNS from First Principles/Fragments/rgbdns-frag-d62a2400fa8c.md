---
type: "code-fragment"
fragment_id: "rgbdns-frag-d62a2400fa8c"
source_path: "src/multilog.rs"
code_note: "DNS from First Principles/Code/src/multilog.rs.source"
language: "rust"
subsystem: "Operations and supervision"
crate: "rgbdns"
symbol: "tai64n"
kind: "fn"
start_line: 206
end_line: 209
---

# tai64n

- Fragment ID: `rgbdns-frag-d62a2400fa8c`
- Source file: [[DNS from First Principles/Code/src/multilog.rs.source|src/multilog.rs]]
- Lines: 206-209
- Subsystem: [[DNS from First Principles/Subsystems/Operations and supervision|Operations and supervision]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-d62a2400fa8c", "codeNote": "DNS from First Principles/Code/src/multilog.rs.source", "heading": "rgbdns-frag-d62a2400fa8c: fn tai64n", "sourcePath": "src/multilog.rs", "startLine": 206, "endLine": 209}
```

## Excerpt

<span id="rgbdns-frag-d62a2400fa8c" class="rgbdns-fragment-target"></span>
### rgbdns-frag-d62a2400fa8c: fn tai64n

```rust
pub fn tai64n(time: SystemTime) -> String {
    format!("{} ", crate::tai64::label(time))
}

```
