---
type: "code-fragment"
fragment_id: "rgbdns-frag-e1966e8f1b30"
source_path: "src/tinydns_edit.rs"
code_note: "DNS from First Principles/Code/src/tinydns_edit.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "paths"
kind: "fn"
start_line: 228
end_line: 243
---

# paths

- Fragment ID: `rgbdns-frag-e1966e8f1b30`
- Source file: [[DNS from First Principles/Code/src/tinydns_edit.rs.source|src/tinydns_edit.rs]]
- Lines: 228-243
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-e1966e8f1b30", "codeNote": "DNS from First Principles/Code/src/tinydns_edit.rs.source", "heading": "rgbdns-frag-e1966e8f1b30: fn paths", "sourcePath": "src/tinydns_edit.rs", "startLine": 228, "endLine": 243}
```

## Excerpt

<span id="rgbdns-frag-e1966e8f1b30" class="rgbdns-fragment-target"></span>
### rgbdns-frag-e1966e8f1b30: fn paths

```rust
    fn paths() -> (std::path::PathBuf, std::path::PathBuf) {
        let stem = format!(
            "rgbdns-edit-{}-{}",
            std::process::id(),
            SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        );
        (
            std::env::temp_dir().join(&stem),
            std::env::temp_dir().join(format!("{stem}.new")),
        )
    }

    #[test]
```
