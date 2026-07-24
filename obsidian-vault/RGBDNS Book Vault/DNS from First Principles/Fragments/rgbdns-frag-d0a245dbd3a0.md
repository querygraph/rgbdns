---
type: "code-fragment"
fragment_id: "rgbdns-frag-d0a245dbd3a0"
source_path: "src/tai64.rs"
code_note: "DNS from First Principles/Code/src/tai64.rs.source"
language: "rust"
subsystem: "Operations and supervision"
crate: "rgbdns"
symbol: "parse_label"
kind: "fn"
start_line: 60
end_line: 76
---

# parse_label

- Fragment ID: `rgbdns-frag-d0a245dbd3a0`
- Source file: [[DNS from First Principles/Code/src/tai64.rs.source|src/tai64.rs]]
- Lines: 60-76
- Subsystem: [[DNS from First Principles/Subsystems/Operations and supervision|Operations and supervision]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-d0a245dbd3a0", "codeNote": "DNS from First Principles/Code/src/tai64.rs.source", "heading": "rgbdns-frag-d0a245dbd3a0: fn parse_label", "sourcePath": "src/tai64.rs", "startLine": 60, "endLine": 76}
```

## Excerpt

<span id="rgbdns-frag-d0a245dbd3a0" class="rgbdns-fragment-target"></span>
### rgbdns-frag-d0a245dbd3a0: fn parse_label

```rust
pub fn parse_label(value: &str) -> Option<Timestamp> {
    if value.len() != 25 || !value.starts_with('@') {
        return None;
    }
    let seconds = u64::from_str_radix(&value[1..17], 16).ok()?;
    let nanoseconds = u32::from_str_radix(&value[17..25], 16).ok()?;
    if seconds < TAI64_BIAS || nanoseconds >= 1_000_000_000 {
        return None;
    }
    let tai_seconds = i64::try_from(seconds - TAI64_BIAS).ok()?;
    let unix_seconds = tai_to_unix(tai_seconds);
    Some(Timestamp {
        unix_seconds,
        nanoseconds,
    })
}

```
