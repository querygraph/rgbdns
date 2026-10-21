---
type: "code-fragment"
fragment_id: "rgbdns-frag-6d39caa65f9a"
source_path: "src/bin/rgbdns-log-report.rs"
code_note: "DNS from First Principles/Code/src/bin/rgbdns-log-report.rs.source"
language: "rust"
subsystem: "Command-line programs"
crate: "rgbdns-log-report"
symbol: "normalize_name"
kind: "fn"
start_line: 46
end_line: 55
---

# normalize_name

- Fragment ID: `rgbdns-frag-6d39caa65f9a`
- Source file: [[DNS from First Principles/Code/src/bin/rgbdns-log-report.rs.source|src/bin/rgbdns-log-report.rs]]
- Lines: 46-55
- Subsystem: [[DNS from First Principles/Subsystems/Command-line programs|Command-line programs]]
- Component: [[DNS from First Principles/Components/rgbdns-log-report|rgbdns-log-report]]

```rgbdns-fragment
{"id": "rgbdns-frag-6d39caa65f9a", "codeNote": "DNS from First Principles/Code/src/bin/rgbdns-log-report.rs.source", "sourcePath": "src/bin/rgbdns-log-report.rs", "startLine": 46, "endLine": 55}
```

## Excerpt

<span id="rgbdns-frag-6d39caa65f9a" class="rgbdns-fragment-target"></span>
### rgbdns-frag-6d39caa65f9a: fn normalize_name

```rust
fn normalize_name(value: &str) -> Option<String> {
    let value = value.trim().trim_end_matches('.').to_ascii_lowercase();
    (!value.is_empty()
        && value.len() <= 253
        && value
            .split('.')
            .all(|label| !label.is_empty() && label.len() <= 63))
    .then_some(value)
}

```
