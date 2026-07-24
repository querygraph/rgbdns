---
type: "code-fragment"
fragment_id: "rgbdns-frag-c4c75c363342"
source_path: "src/bin/dnscache.rs"
code_note: "DNS from First Principles/Code/src/bin/dnscache.rs.source"
language: "rust"
subsystem: "Command-line programs"
crate: "dnscache"
symbol: "bounded_env_usize"
kind: "fn"
start_line: 143
end_line: 151
---

# bounded_env_usize

- Fragment ID: `rgbdns-frag-c4c75c363342`
- Source file: [[DNS from First Principles/Code/src/bin/dnscache.rs.source|src/bin/dnscache.rs]]
- Lines: 143-151
- Subsystem: [[DNS from First Principles/Subsystems/Command-line programs|Command-line programs]]
- Component: [[DNS from First Principles/Components/dnscache|dnscache]]

```rgbdns-fragment
{"id": "rgbdns-frag-c4c75c363342", "codeNote": "DNS from First Principles/Code/src/bin/dnscache.rs.source", "heading": "rgbdns-frag-c4c75c363342: fn bounded_env_usize", "sourcePath": "src/bin/dnscache.rs", "startLine": 143, "endLine": 151}
```

## Excerpt

<span id="rgbdns-frag-c4c75c363342" class="rgbdns-fragment-target"></span>
### rgbdns-frag-c4c75c363342: fn bounded_env_usize

```rust
fn bounded_env_usize(
    name: &str,
    default: usize,
    minimum: usize,
    maximum: usize,
) -> Result<usize, Box<dyn std::error::Error>> {
    bounded_env(name, default, minimum, maximum)
}

```
