---
type: "code-fragment"
fragment_id: "rgbdns-frag-63b1cf7ef29d"
source_path: "src/server.rs"
code_note: "DNS from First Principles/Code/src/server.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "parse_query_log"
kind: "fn"
start_line: 53
end_line: 60
---

# parse_query_log

- Fragment ID: `rgbdns-frag-63b1cf7ef29d`
- Source file: [[DNS from First Principles/Code/src/server.rs.source|src/server.rs]]
- Lines: 53-60
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-63b1cf7ef29d", "codeNote": "DNS from First Principles/Code/src/server.rs.source", "sourcePath": "src/server.rs", "startLine": 53, "endLine": 60}
```

## Excerpt

<span id="rgbdns-frag-63b1cf7ef29d" class="rgbdns-fragment-target"></span>
### rgbdns-frag-63b1cf7ef29d: fn parse_query_log

```rust
fn parse_query_log(value: Option<&str>) -> Result<bool> {
    match value.map(str::trim).map(str::to_ascii_lowercase).as_deref() {
        None | Some("1" | "true" | "yes" | "on") => Ok(true),
        Some("0" | "false" | "no" | "off") => Ok(false),
        Some(_) => Err(Error::Format("invalid QUERY_LOG")),
    }
}

```
