---
type: "code-fragment"
fragment_id: "rgbdns-frag-5af93fc96e2d"
source_path: "src/axfr.rs"
code_note: "DNS from First Principles/Code/src/axfr.rs.source"
language: "rust"
subsystem: "Transport and zone transfer"
crate: "rgbdns"
symbol: "temp_path"
kind: "fn"
start_line: 511
end_line: 522
---

# temp_path

- Fragment ID: `rgbdns-frag-5af93fc96e2d`
- Source file: [[DNS from First Principles/Code/src/axfr.rs.source|src/axfr.rs]]
- Lines: 511-522
- Subsystem: [[DNS from First Principles/Subsystems/Transport and zone transfer|Transport and zone transfer]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-5af93fc96e2d", "codeNote": "DNS from First Principles/Code/src/axfr.rs.source", "sourcePath": "src/axfr.rs", "startLine": 511, "endLine": 522}
```

## Excerpt

<span id="rgbdns-frag-5af93fc96e2d" class="rgbdns-fragment-target"></span>
### rgbdns-frag-5af93fc96e2d: fn temp_path

```rust
    fn temp_path(label: &str) -> std::path::PathBuf {
        std::env::temp_dir().join(format!(
            "rgbdns-{label}-{}-{}",
            std::process::id(),
            SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ))
    }

    #[test]
```
