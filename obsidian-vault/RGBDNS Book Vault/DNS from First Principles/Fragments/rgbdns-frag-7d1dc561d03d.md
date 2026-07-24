---
type: "code-fragment"
fragment_id: "rgbdns-frag-7d1dc561d03d"
source_path: "tests/drill_interop.rs"
code_note: "DNS from First Principles/Code/tests/drill_interop.rs.source"
language: "rust"
subsystem: "Tests and performance"
symbol: "drill"
kind: "fn"
start_line: 63
end_line: 84
---

# drill

- Fragment ID: `rgbdns-frag-7d1dc561d03d`
- Source file: [[DNS from First Principles/Code/tests/drill_interop.rs.source|tests/drill_interop.rs]]
- Lines: 63-84
- Subsystem: [[DNS from First Principles/Subsystems/Tests and performance|Tests and performance]]

```rgbdns-fragment
{"id": "rgbdns-frag-7d1dc561d03d", "codeNote": "DNS from First Principles/Code/tests/drill_interop.rs.source", "heading": "rgbdns-frag-7d1dc561d03d: fn drill", "sourcePath": "tests/drill_interop.rs", "startLine": 63, "endLine": 84}
```

## Excerpt

<span id="rgbdns-frag-7d1dc561d03d" class="rgbdns-fragment-target"></span>
### rgbdns-frag-7d1dc561d03d: fn drill

```rust
fn drill(port: u16, extra: &[&str], name: &str, record_type: &str) -> String {
    let output = Command::new("drill")
        .args(extra)
        .args([
            "-p",
            &port.to_string(),
            name,
            "@127.0.0.1",
            record_type,
            "IN",
        ])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "drill failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout).unwrap()
}

#[test]
```
