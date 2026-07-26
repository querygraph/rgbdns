---
type: "code-fragment"
fragment_id: "rgbdns-frag-fce321309f41"
source_path: "src/zone.rs"
code_note: "DNS from First Principles/Code/src/zone.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "expanded_target"
kind: "fn"
start_line: 727
end_line: 734
---

# expanded_target

- Fragment ID: `rgbdns-frag-fce321309f41`
- Source file: [[DNS from First Principles/Code/src/zone.rs.source|src/zone.rs]]
- Lines: 727-734
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-fce321309f41", "codeNote": "DNS from First Principles/Code/src/zone.rs.source", "heading": "rgbdns-frag-fce321309f41: fn expanded_target", "sourcePath": "src/zone.rs", "startLine": 727, "endLine": 734}
```

## Excerpt

<span id="rgbdns-frag-fce321309f41" class="rgbdns-fragment-target"></span>
### rgbdns-frag-fce321309f41: fn expanded_target

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
