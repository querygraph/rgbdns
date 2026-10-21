---
type: "code-fragment"
fragment_id: "rgbdns-frag-3eaad9f2e40f"
source_path: "src/zone.rs"
code_note: "DNS from First Principles/Code/src/zone.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "field_opt"
kind: "fn"
start_line: 986
end_line: 988
---

# field_opt

- Fragment ID: `rgbdns-frag-3eaad9f2e40f`
- Source file: [[DNS from First Principles/Code/src/zone.rs.source|src/zone.rs]]
- Lines: 986-988
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-3eaad9f2e40f", "codeNote": "DNS from First Principles/Code/src/zone.rs.source", "sourcePath": "src/zone.rs", "startLine": 986, "endLine": 988}
```

## Excerpt

<span id="rgbdns-frag-3eaad9f2e40f" class="rgbdns-fragment-target"></span>
### rgbdns-frag-3eaad9f2e40f: fn field_opt

```rust
fn field_opt(f: &[String], i: usize) -> Option<&str> {
    f.get(i).map(String::as_str)
}
```
