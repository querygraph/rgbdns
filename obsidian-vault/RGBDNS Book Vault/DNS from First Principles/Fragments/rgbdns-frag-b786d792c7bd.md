---
type: "code-fragment"
fragment_id: "rgbdns-frag-b786d792c7bd"
source_path: "src/dnscache_config.rs"
code_note: "DNS from First Principles/Code/src/dnscache_config.rs.source"
language: "rust"
subsystem: "Resolution and recursion"
crate: "rgbdns"
symbol: "forward_zones_from_environment"
kind: "fn"
start_line: 105
end_line: 111
---

# forward_zones_from_environment

- Fragment ID: `rgbdns-frag-b786d792c7bd`
- Source file: [[DNS from First Principles/Code/src/dnscache_config.rs.source|src/dnscache_config.rs]]
- Lines: 105-111
- Subsystem: [[DNS from First Principles/Subsystems/Resolution and recursion|Resolution and recursion]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-b786d792c7bd", "codeNote": "DNS from First Principles/Code/src/dnscache_config.rs.source", "heading": "rgbdns-frag-b786d792c7bd: fn forward_zones_from_environment", "sourcePath": "src/dnscache_config.rs", "startLine": 105, "endLine": 111}
```

## Excerpt

<span id="rgbdns-frag-b786d792c7bd" class="rgbdns-fragment-target"></span>
### rgbdns-frag-b786d792c7bd: fn forward_zones_from_environment

```rust
pub fn forward_zones_from_environment() -> Result<Vec<ForwardZone>> {
    let Some(root) = env::var_os("ROOT").map(PathBuf::from) else {
        return Ok(Vec::new());
    };
    load_forward_zones(&root.join("servers"))
}

```
