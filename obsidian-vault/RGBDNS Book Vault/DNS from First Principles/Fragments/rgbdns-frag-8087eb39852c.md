---
type: "code-fragment"
fragment_id: "rgbdns-frag-8087eb39852c"
source_path: "tests/daemontools.rs"
code_note: "DNS from First Principles/Code/tests/daemontools.rs.source"
language: "rust"
subsystem: "Tests and performance"
symbol: "directory"
kind: "fn"
start_line: 12
end_line: 23
---

# directory

- Fragment ID: `rgbdns-frag-8087eb39852c`
- Source file: [[DNS from First Principles/Code/tests/daemontools.rs.source|tests/daemontools.rs]]
- Lines: 12-23
- Subsystem: [[DNS from First Principles/Subsystems/Tests and performance|Tests and performance]]

```rgbdns-fragment
{"id": "rgbdns-frag-8087eb39852c", "codeNote": "DNS from First Principles/Code/tests/daemontools.rs.source", "heading": "rgbdns-frag-8087eb39852c: fn directory", "sourcePath": "tests/daemontools.rs", "startLine": 12, "endLine": 23}
```

## Excerpt

<span id="rgbdns-frag-8087eb39852c" class="rgbdns-fragment-target"></span>
### rgbdns-frag-8087eb39852c: fn directory

```rust
fn directory() -> PathBuf {
    std::env::temp_dir().join(format!(
        "rgbdns-daemontools-test-{}-{}",
        std::process::id(),
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ))
}

#[test]
```
