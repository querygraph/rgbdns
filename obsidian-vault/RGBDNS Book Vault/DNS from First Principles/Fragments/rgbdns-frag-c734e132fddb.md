---
type: "code-fragment"
fragment_id: "rgbdns-frag-c734e132fddb"
source_path: "src/tai64.rs"
code_note: "DNS from First Principles/Code/src/tai64.rs.source"
language: "rust"
subsystem: "Operations and supervision"
crate: "rgbdns"
symbol: "local"
kind: "fn"
start_line: 77
end_line: 90
---

# local

- Fragment ID: `rgbdns-frag-c734e132fddb`
- Source file: [[DNS from First Principles/Code/src/tai64.rs.source|src/tai64.rs]]
- Lines: 77-90
- Subsystem: [[DNS from First Principles/Subsystems/Operations and supervision|Operations and supervision]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-c734e132fddb", "codeNote": "DNS from First Principles/Code/src/tai64.rs.source", "heading": "rgbdns-frag-c734e132fddb: fn local", "sourcePath": "src/tai64.rs", "startLine": 77, "endLine": 90}
```

## Excerpt

<span id="rgbdns-frag-c734e132fddb" class="rgbdns-fragment-target"></span>
### rgbdns-frag-c734e132fddb: fn local

```rust
pub fn local(value: Timestamp) -> Option<String> {
    let date = Local
        .timestamp_opt(value.unix_seconds, value.nanoseconds)
        .single()?;
    if !(0..=9999).contains(&date.year()) {
        return None;
    }
    Some(format!(
        "{}.{:09}",
        date.format("%Y-%m-%d %H:%M:%S"),
        value.nanoseconds
    ))
}

```
