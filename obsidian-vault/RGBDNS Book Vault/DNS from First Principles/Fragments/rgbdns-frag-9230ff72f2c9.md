---
type: "code-fragment"
fragment_id: "rgbdns-frag-9230ff72f2c9"
source_path: "src/multilog.rs"
code_note: "DNS from First Principles/Code/src/multilog.rs.source"
language: "rust"
subsystem: "Operations and supervision"
crate: "rgbdns"
symbol: "tai64n_label"
kind: "fn"
start_line: 210
end_line: 214
---

# tai64n_label

- Fragment ID: `rgbdns-frag-9230ff72f2c9`
- Source file: [[DNS from First Principles/Code/src/multilog.rs.source|src/multilog.rs]]
- Lines: 210-214
- Subsystem: [[DNS from First Principles/Subsystems/Operations and supervision|Operations and supervision]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-9230ff72f2c9", "codeNote": "DNS from First Principles/Code/src/multilog.rs.source", "heading": "rgbdns-frag-9230ff72f2c9: fn tai64n_label", "sourcePath": "src/multilog.rs", "startLine": 210, "endLine": 214}
```

## Excerpt

<span id="rgbdns-frag-9230ff72f2c9" class="rgbdns-fragment-target"></span>
### rgbdns-frag-9230ff72f2c9: fn tai64n_label

```rust
fn tai64n_label(time: SystemTime) -> String {
    crate::tai64::label(time)
}

#[cfg(test)]
```
