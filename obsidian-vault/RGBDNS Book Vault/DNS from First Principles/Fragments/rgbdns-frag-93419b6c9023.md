---
type: "code-fragment"
fragment_id: "rgbdns-frag-93419b6c9023"
source_path: "src/bin/dnscache.rs"
code_note: "DNS from First Principles/Code/src/bin/dnscache.rs.source"
language: "rust"
subsystem: "Command-line programs"
crate: "dnscache"
symbol: "bounded_env_u8"
kind: "fn"
start_line: 152
end_line: 160
---

# bounded_env_u8

- Fragment ID: `rgbdns-frag-93419b6c9023`
- Source file: [[DNS from First Principles/Code/src/bin/dnscache.rs.source|src/bin/dnscache.rs]]
- Lines: 152-160
- Subsystem: [[DNS from First Principles/Subsystems/Command-line programs|Command-line programs]]
- Component: [[DNS from First Principles/Components/dnscache|dnscache]]

```rgbdns-fragment
{"id": "rgbdns-frag-93419b6c9023", "codeNote": "DNS from First Principles/Code/src/bin/dnscache.rs.source", "heading": "rgbdns-frag-93419b6c9023: fn bounded_env_u8", "sourcePath": "src/bin/dnscache.rs", "startLine": 152, "endLine": 160}
```

## Excerpt

<span id="rgbdns-frag-93419b6c9023" class="rgbdns-fragment-target"></span>
### rgbdns-frag-93419b6c9023: fn bounded_env_u8

```rust
fn bounded_env_u8(
    name: &str,
    default: u8,
    minimum: u8,
    maximum: u8,
) -> Result<u8, Box<dyn std::error::Error>> {
    bounded_env(name, default, minimum, maximum)
}

```
