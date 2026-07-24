---
type: "code-fragment"
fragment_id: "rgbdns-frag-1c885999a2f2"
source_path: "src/dnscache_config.rs"
code_note: "DNS from First Principles/Code/src/dnscache_config.rs.source"
language: "rust"
subsystem: "Resolution and recursion"
crate: "rgbdns"
symbol: "from_environment"
kind: "fn"
start_line: 33
end_line: 45
---

# from_environment

- Fragment ID: `rgbdns-frag-1c885999a2f2`
- Source file: [[DNS from First Principles/Code/src/dnscache_config.rs.source|src/dnscache_config.rs]]
- Lines: 33-45
- Subsystem: [[DNS from First Principles/Subsystems/Resolution and recursion|Resolution and recursion]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-1c885999a2f2", "codeNote": "DNS from First Principles/Code/src/dnscache_config.rs.source", "heading": "rgbdns-frag-1c885999a2f2: fn from_environment", "sourcePath": "src/dnscache_config.rs", "startLine": 33, "endLine": 45}
```

## Excerpt

<span id="rgbdns-frag-1c885999a2f2" class="rgbdns-fragment-target"></span>
### rgbdns-frag-1c885999a2f2: fn from_environment

```rust
    pub fn from_environment() -> Result<Self> {
        let path = env::var_os("ROOTS").map_or_else(
            || {
                env::var_os("ROOT")
                    .map(PathBuf::from)
                    .map(|root| root.join("servers/@"))
                    .unwrap_or_else(|| PathBuf::from("config/root.hints"))
            },
            PathBuf::from,
        );
        Self::prepare(path)
    }

```
