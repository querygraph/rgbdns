---
type: "code-fragment"
fragment_id: "rgbdns-frag-e1ca3db94035"
source_path: "src/zone.rs"
code_note: "DNS from First Principles/Code/src/zone.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "split_fields"
kind: "fn"
start_line: 630
end_line: 642
---

# split_fields

- Fragment ID: `rgbdns-frag-e1ca3db94035`
- Source file: [[DNS from First Principles/Code/src/zone.rs.source|src/zone.rs]]
- Lines: 630-642
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-e1ca3db94035", "codeNote": "DNS from First Principles/Code/src/zone.rs.source", "heading": "rgbdns-frag-e1ca3db94035: fn split_fields", "sourcePath": "src/zone.rs", "startLine": 630, "endLine": 642}
```

## Excerpt

<span id="rgbdns-frag-e1ca3db94035" class="rgbdns-fragment-target"></span>
### rgbdns-frag-e1ca3db94035: fn split_fields

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
