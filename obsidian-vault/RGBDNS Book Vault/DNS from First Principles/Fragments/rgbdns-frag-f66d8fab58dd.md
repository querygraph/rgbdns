---
type: "code-fragment"
fragment_id: "rgbdns-frag-f66d8fab58dd"
source_path: "src/zone.rs"
code_note: "DNS from First Principles/Code/src/zone.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "field"
kind: "fn"
start_line: 981
end_line: 985
---

# field

- Fragment ID: `rgbdns-frag-f66d8fab58dd`
- Source file: [[DNS from First Principles/Code/src/zone.rs.source|src/zone.rs]]
- Lines: 981-985
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-f66d8fab58dd", "codeNote": "DNS from First Principles/Code/src/zone.rs.source", "sourcePath": "src/zone.rs", "startLine": 981, "endLine": 985}
```

## Excerpt

<span id="rgbdns-frag-f66d8fab58dd" class="rgbdns-fragment-target"></span>
### rgbdns-frag-f66d8fab58dd: fn field

```rust
fn field(f: &[String], i: usize) -> Result<&str> {
    f.get(i)
        .map(String::as_str)
        .ok_or_else(|| Error::InvalidRecord(format!("missing field {}", i + 1)))
}
```
