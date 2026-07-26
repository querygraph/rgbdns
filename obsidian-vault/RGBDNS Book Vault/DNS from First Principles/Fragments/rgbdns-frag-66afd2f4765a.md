---
type: "code-fragment"
fragment_id: "rgbdns-frag-66afd2f4765a"
source_path: "src/zone.rs"
code_note: "DNS from First Principles/Code/src/zone.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "split_fields"
kind: "fn"
start_line: 700
end_line: 712
---

# split_fields

- Fragment ID: `rgbdns-frag-66afd2f4765a`
- Source file: [[DNS from First Principles/Code/src/zone.rs.source|src/zone.rs]]
- Lines: 700-712
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-66afd2f4765a", "codeNote": "DNS from First Principles/Code/src/zone.rs.source", "heading": "rgbdns-frag-66afd2f4765a: fn split_fields", "sourcePath": "src/zone.rs", "startLine": 700, "endLine": 712}
```

## Excerpt

<span id="rgbdns-frag-66afd2f4765a" class="rgbdns-fragment-target"></span>
### rgbdns-frag-66afd2f4765a: fn split_fields

```rust
fn split_fields(s: &str) -> Vec<String> {
    let mut out = vec![String::new()];
    let mut esc = false;
    for c in s.chars() {
        if c == ':' && !esc {
            out.push(String::new())
        } else {
            out.last_mut().unwrap().push(c)
        }
        esc = c == '\\' && !esc;
    }
    out
}
```
