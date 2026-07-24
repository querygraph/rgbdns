---
type: "code-fragment"
fragment_id: "rgbdns-frag-e67d14654aa5"
source_path: "src/axfr.rs"
code_note: "DNS from First Principles/Code/src/axfr.rs.source"
language: "rust"
subsystem: "Transport and zone transfer"
crate: "rgbdns"
symbol: "temp_path"
kind: "fn"
start_line: 358
end_line: 369
---

# temp_path

- Fragment ID: `rgbdns-frag-e67d14654aa5`
- Source file: [[DNS from First Principles/Code/src/axfr.rs.source|src/axfr.rs]]
- Lines: 358-369
- Subsystem: [[DNS from First Principles/Subsystems/Transport and zone transfer|Transport and zone transfer]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-e67d14654aa5", "codeNote": "DNS from First Principles/Code/src/axfr.rs.source", "heading": "rgbdns-frag-e67d14654aa5: fn temp_path", "sourcePath": "src/axfr.rs", "startLine": 358, "endLine": 369}
```

## Excerpt

<span id="rgbdns-frag-e67d14654aa5" class="rgbdns-fragment-target"></span>
### rgbdns-frag-e67d14654aa5: fn temp_path

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
