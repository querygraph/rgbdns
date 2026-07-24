---
type: "code-fragment"
fragment_id: "rgbdns-frag-33e30225780d"
source_path: "src/conf.rs"
code_note: "DNS from First Principles/Code/src/conf.rs.source"
language: "rust"
subsystem: "Operations and supervision"
crate: "rgbdns"
symbol: "directory"
kind: "fn"
start_line: 252
end_line: 263
---

# directory

- Fragment ID: `rgbdns-frag-33e30225780d`
- Source file: [[DNS from First Principles/Code/src/conf.rs.source|src/conf.rs]]
- Lines: 252-263
- Subsystem: [[DNS from First Principles/Subsystems/Operations and supervision|Operations and supervision]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-33e30225780d", "codeNote": "DNS from First Principles/Code/src/conf.rs.source", "heading": "rgbdns-frag-33e30225780d: fn directory", "sourcePath": "src/conf.rs", "startLine": 252, "endLine": 263}
```

## Excerpt

<span id="rgbdns-frag-33e30225780d" class="rgbdns-fragment-target"></span>
### rgbdns-frag-33e30225780d: fn directory

```rust
    fn directory(label: &str) -> PathBuf {
        std::env::temp_dir().join(format!(
            "rgbdns-conf-{label}-{}-{}",
            std::process::id(),
            SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ))
    }

    #[test]
```
