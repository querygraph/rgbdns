---
type: "code-fragment"
fragment_id: "rgbdns-frag-f757a4996b8a"
source_path: "src/bin/rgbdns-log-report.rs"
code_note: "DNS from First Principles/Code/src/bin/rgbdns-log-report.rs.source"
language: "rust"
subsystem: "Command-line programs"
crate: "rgbdns-log-report"
symbol: "parse_query"
kind: "fn"
start_line: 88
end_line: 113
---

# parse_query

- Fragment ID: `rgbdns-frag-f757a4996b8a`
- Source file: [[DNS from First Principles/Code/src/bin/rgbdns-log-report.rs.source|src/bin/rgbdns-log-report.rs]]
- Lines: 88-113
- Subsystem: [[DNS from First Principles/Subsystems/Command-line programs|Command-line programs]]
- Component: [[DNS from First Principles/Components/rgbdns-log-report|rgbdns-log-report]]

```rgbdns-fragment
{"id": "rgbdns-frag-f757a4996b8a", "codeNote": "DNS from First Principles/Code/src/bin/rgbdns-log-report.rs.source", "sourcePath": "src/bin/rgbdns-log-report.rs", "startLine": 88, "endLine": 113}
```

## Excerpt

<span id="rgbdns-frag-f757a4996b8a" class="rgbdns-fragment-target"></span>
### rgbdns-frag-f757a4996b8a: fn parse_query

```rust
fn parse_query(line: &str) -> Option<(&str, String)> {
    let mut fields = line.split_whitespace();
    let request = fields.next()?;
    if fields.next()? != "+" {
        return None;
    }
    fields.next()?;
    let name = normalize_name(fields.next()?)?;
    if fields.next().is_some() {
        return None;
    }
    let mut request_fields = request.split(':');
    let client = request_fields.next()?;
    let port = request_fields.next()?;
    let id = request_fields.next()?;
    if request_fields.next().is_some()
        || !matches!(client.len(), 8 | 32)
        || !client.bytes().all(|byte| byte.is_ascii_hexdigit())
        || port.len() != 4
        || id.len() != 4
    {
        return None;
    }
    Some((client, name))
}

```
