---
type: "code-fragment"
fragment_id: "rgbdns-frag-28cd1b8dc5d1"
source_path: "src/multilog.rs"
code_note: "DNS from First Principles/Code/src/multilog.rs.source"
language: "rust"
subsystem: "Operations and supervision"
crate: "rgbdns"
symbol: "directory"
kind: "fn"
start_line: 219
end_line: 223
---

# directory

- Fragment ID: `rgbdns-frag-28cd1b8dc5d1`
- Source file: [[DNS from First Principles/Code/src/multilog.rs.source|src/multilog.rs]]
- Lines: 219-223
- Subsystem: [[DNS from First Principles/Subsystems/Operations and supervision|Operations and supervision]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-28cd1b8dc5d1", "codeNote": "DNS from First Principles/Code/src/multilog.rs.source", "heading": "rgbdns-frag-28cd1b8dc5d1: fn directory", "sourcePath": "src/multilog.rs", "startLine": 219, "endLine": 223}
```

## Excerpt

<span id="rgbdns-frag-28cd1b8dc5d1" class="rgbdns-fragment-target"></span>
### rgbdns-frag-28cd1b8dc5d1: fn directory

```rust
    fn directory(label: &str) -> PathBuf {
        std::env::temp_dir().join(format!("rgbdns-multilog-{label}-{}", std::process::id()))
    }

    #[test]
```
