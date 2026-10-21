---
type: "code-fragment"
fragment_id: "rgbdns-frag-82b6684016b2"
source_path: "src/bin/rgbdns-log-report.rs"
code_note: "DNS from First Principles/Code/src/bin/rgbdns-log-report.rs.source"
language: "rust"
subsystem: "Command-line programs"
crate: "rgbdns-log-report"
symbol: "containing_zone"
kind: "fn"
start_line: 76
end_line: 87
---

# containing_zone

- Fragment ID: `rgbdns-frag-82b6684016b2`
- Source file: [[DNS from First Principles/Code/src/bin/rgbdns-log-report.rs.source|src/bin/rgbdns-log-report.rs]]
- Lines: 76-87
- Subsystem: [[DNS from First Principles/Subsystems/Command-line programs|Command-line programs]]
- Component: [[DNS from First Principles/Components/rgbdns-log-report|rgbdns-log-report]]

```rgbdns-fragment
{"id": "rgbdns-frag-82b6684016b2", "codeNote": "DNS from First Principles/Code/src/bin/rgbdns-log-report.rs.source", "sourcePath": "src/bin/rgbdns-log-report.rs", "startLine": 76, "endLine": 87}
```

## Excerpt

<span id="rgbdns-frag-82b6684016b2" class="rgbdns-fragment-target"></span>
### rgbdns-frag-82b6684016b2: fn containing_zone

```rust
fn containing_zone<'a>(name: &str, zones: &'a [String]) -> Option<&'a str> {
    zones
        .iter()
        .find(|zone| {
            name == zone.as_str()
                || name
                    .strip_suffix(zone.as_str())
                    .is_some_and(|prefix| prefix.ends_with('.'))
        })
        .map(String::as_str)
}

```
