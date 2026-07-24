---
type: "code-fragment"
fragment_id: "rgbdns-frag-3e7a8fc52fc3"
source_path: "src/multilog.rs"
code_note: "DNS from First Principles/Code/src/multilog.rs.source"
language: "rust"
subsystem: "Operations and supervision"
crate: "rgbdns"
symbol: "Config"
kind: "struct"
start_line: 17
end_line: 23
---

# Config

- Fragment ID: `rgbdns-frag-3e7a8fc52fc3`
- Source file: [[DNS from First Principles/Code/src/multilog.rs.source|src/multilog.rs]]
- Lines: 17-23
- Subsystem: [[DNS from First Principles/Subsystems/Operations and supervision|Operations and supervision]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-3e7a8fc52fc3", "codeNote": "DNS from First Principles/Code/src/multilog.rs.source", "heading": "rgbdns-frag-3e7a8fc52fc3: struct Config", "sourcePath": "src/multilog.rs", "startLine": 17, "endLine": 23}
```

## Excerpt

<span id="rgbdns-frag-3e7a8fc52fc3" class="rgbdns-fragment-target"></span>
### rgbdns-frag-3e7a8fc52fc3: struct Config

```rust
pub struct Config {
    pub timestamp: bool,
    pub max_size: u64,
    pub retain: usize,
    pub directories: Vec<PathBuf>,
}

```
