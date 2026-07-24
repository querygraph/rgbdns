---
type: "code-fragment"
fragment_id: "rgbdns-frag-a3453c75cf0d"
source_path: "src/tai64.rs"
code_note: "DNS from First Principles/Code/src/tai64.rs.source"
language: "rust"
subsystem: "Operations and supervision"
crate: "rgbdns"
symbol: "label"
kind: "fn"
start_line: 49
end_line: 59
---

# label

- Fragment ID: `rgbdns-frag-a3453c75cf0d`
- Source file: [[DNS from First Principles/Code/src/tai64.rs.source|src/tai64.rs]]
- Lines: 49-59
- Subsystem: [[DNS from First Principles/Subsystems/Operations and supervision|Operations and supervision]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-a3453c75cf0d", "codeNote": "DNS from First Principles/Code/src/tai64.rs.source", "heading": "rgbdns-frag-a3453c75cf0d: fn label", "sourcePath": "src/tai64.rs", "startLine": 49, "endLine": 59}
```

## Excerpt

<span id="rgbdns-frag-a3453c75cf0d" class="rgbdns-fragment-target"></span>
### rgbdns-frag-a3453c75cf0d: fn label

```rust
pub fn label(time: SystemTime) -> String {
    let duration = time.duration_since(UNIX_EPOCH).unwrap_or_default();
    let unix_seconds = i64::try_from(duration.as_secs()).unwrap_or(i64::MAX);
    let tai_seconds = unix_seconds.saturating_add(offset_at_utc(unix_seconds));
    format!(
        "@{:016x}{:08x}",
        TAI64_BIAS.saturating_add(tai_seconds.max(0) as u64),
        duration.subsec_nanos()
    )
}

```
