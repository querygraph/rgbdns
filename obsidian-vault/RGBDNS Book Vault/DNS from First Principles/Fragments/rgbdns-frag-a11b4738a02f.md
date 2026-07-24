---
type: "code-fragment"
fragment_id: "rgbdns-frag-a11b4738a02f"
source_path: "src/tai64.rs"
code_note: "DNS from First Principles/Code/src/tai64.rs.source"
language: "rust"
subsystem: "Operations and supervision"
crate: "rgbdns"
symbol: "offset_at_utc"
kind: "fn"
start_line: 172
end_line: 176
---

# offset_at_utc

- Fragment ID: `rgbdns-frag-a11b4738a02f`
- Source file: [[DNS from First Principles/Code/src/tai64.rs.source|src/tai64.rs]]
- Lines: 172-176
- Subsystem: [[DNS from First Principles/Subsystems/Operations and supervision|Operations and supervision]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-a11b4738a02f", "codeNote": "DNS from First Principles/Code/src/tai64.rs.source", "heading": "rgbdns-frag-a11b4738a02f: fn offset_at_utc", "sourcePath": "src/tai64.rs", "startLine": 172, "endLine": 176}
```

## Excerpt

<span id="rgbdns-frag-a11b4738a02f" class="rgbdns-fragment-target"></span>
### rgbdns-frag-a11b4738a02f: fn offset_at_utc

```rust
fn offset_at_utc(unix_seconds: i64) -> i64 {
    INITIAL_TAI_UTC_OFFSET
        + LEAP_TRANSITIONS.partition_point(|transition| *transition <= unix_seconds) as i64
}

```
