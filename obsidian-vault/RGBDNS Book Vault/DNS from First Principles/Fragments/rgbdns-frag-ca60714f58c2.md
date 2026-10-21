---
type: "code-fragment"
fragment_id: "rgbdns-frag-ca60714f58c2"
source_path: "src/zone.rs"
code_note: "DNS from First Principles/Code/src/zone.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "expanded_target"
kind: "fn"
start_line: 995
end_line: 1002
---

# expanded_target

- Fragment ID: `rgbdns-frag-ca60714f58c2`
- Source file: [[DNS from First Principles/Code/src/zone.rs.source|src/zone.rs]]
- Lines: 995-1002
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-ca60714f58c2", "codeNote": "DNS from First Principles/Code/src/zone.rs.source", "sourcePath": "src/zone.rs", "startLine": 995, "endLine": 1002}
```

## Excerpt

<span id="rgbdns-frag-ca60714f58c2" class="rgbdns-fragment-target"></span>
### rgbdns-frag-ca60714f58c2: fn expanded_target

```rust
fn expanded_target(value: &str, role: &str, owner: &Name) -> Result<Name> {
    let target = if value.contains('.') {
        value.to_owned()
    } else {
        format!("{value}.{role}.{owner}")
    };
    target.parse()
}
```
