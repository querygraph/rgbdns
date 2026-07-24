---
type: "code-fragment"
fragment_id: "rgbdns-frag-c79ac41f0fa0"
source_path: "tests/drill_interop.rs"
code_note: "DNS from First Principles/Code/tests/drill_interop.rs.source"
language: "rust"
subsystem: "Tests and performance"
symbol: "drill_available"
kind: "fn"
start_line: 23
end_line: 31
---

# drill_available

- Fragment ID: `rgbdns-frag-c79ac41f0fa0`
- Source file: [[DNS from First Principles/Code/tests/drill_interop.rs.source|tests/drill_interop.rs]]
- Lines: 23-31
- Subsystem: [[DNS from First Principles/Subsystems/Tests and performance|Tests and performance]]

```rgbdns-fragment
{"id": "rgbdns-frag-c79ac41f0fa0", "codeNote": "DNS from First Principles/Code/tests/drill_interop.rs.source", "heading": "rgbdns-frag-c79ac41f0fa0: fn drill_available", "sourcePath": "tests/drill_interop.rs", "startLine": 23, "endLine": 31}
```

## Excerpt

<span id="rgbdns-frag-c79ac41f0fa0" class="rgbdns-fragment-target"></span>
### rgbdns-frag-c79ac41f0fa0: fn drill_available

```rust
fn drill_available() -> bool {
    Command::new("drill")
        .arg("-v")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .is_ok_and(|status| status.success())
}

```
