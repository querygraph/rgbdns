---
type: "code-fragment"
fragment_id: "rgbdns-frag-2ddb13686975"
source_path: "src/pick.rs"
code_note: "DNS from First Principles/Code/src/pick.rs.source"
language: "rust"
subsystem: "Specialized responders"
crate: "rgbdns"
symbol: "ip_prefix"
kind: "fn"
start_line: 193
end_line: 205
---

# ip_prefix

- Fragment ID: `rgbdns-frag-2ddb13686975`
- Source file: [[DNS from First Principles/Code/src/pick.rs.source|src/pick.rs]]
- Lines: 193-205
- Subsystem: [[DNS from First Principles/Subsystems/Specialized responders|Specialized responders]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-2ddb13686975", "codeNote": "DNS from First Principles/Code/src/pick.rs.source", "sourcePath": "src/pick.rs", "startLine": 193, "endLine": 205}
```

## Excerpt

<span id="rgbdns-frag-2ddb13686975" class="rgbdns-fragment-target"></span>
### rgbdns-frag-2ddb13686975: fn ip_prefix

```rust
fn ip_prefix(value: &str) -> std::result::Result<Vec<u8>, &'static str> {
    let prefix = value
        .split('.')
        .filter(|part| !part.is_empty())
        .map(|part| part.parse().map_err(|_| "malformed IPv4 prefix"))
        .collect::<std::result::Result<Vec<u8>, _>>()?;
    if prefix.len() > 4 {
        Err("IPv4 prefix has more than four octets")
    } else {
        Ok(prefix)
    }
}

```
