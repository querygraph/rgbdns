---
type: "code-fragment"
fragment_id: "rgbdns-frag-62336da4de19"
source_path: "tests/daemontools.rs"
code_note: "DNS from First Principles/Code/tests/daemontools.rs.source"
language: "rust"
subsystem: "Tests and performance"
symbol: "tai64_filters_roundtrip_a_published_timestamp"
kind: "fn"
start_line: 68
end_line: 99
---

# tai64_filters_roundtrip_a_published_timestamp

- Fragment ID: `rgbdns-frag-62336da4de19`
- Source file: [[DNS from First Principles/Code/tests/daemontools.rs.source|tests/daemontools.rs]]
- Lines: 68-99
- Subsystem: [[DNS from First Principles/Subsystems/Tests and performance|Tests and performance]]

```rgbdns-fragment
{"id": "rgbdns-frag-62336da4de19", "codeNote": "DNS from First Principles/Code/tests/daemontools.rs.source", "heading": "rgbdns-frag-62336da4de19: fn tai64_filters_roundtrip_a_published_timestamp", "sourcePath": "tests/daemontools.rs", "startLine": 68, "endLine": 99}
```

## Excerpt

<span id="rgbdns-frag-62336da4de19" class="rgbdns-fragment-target"></span>
### rgbdns-frag-62336da4de19: fn tai64_filters_roundtrip_a_published_timestamp

```rust
fn tai64_filters_roundtrip_a_published_timestamp() {
    let output = Command::new(env!("CARGO_BIN_EXE_tai64nlocal"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .and_then(|mut child| {
            child
                .stdin
                .take()
                .unwrap()
                .write_all(b"@4000000037c219bf2ef02e94 mark\n")?;
            child.wait_with_output()
        })
        .unwrap();
    assert!(output.status.success());
    let text = String::from_utf8(output.stdout).unwrap();
    assert!(text.ends_with(".787492500 mark\n"));

    let output = Command::new(env!("CARGO_BIN_EXE_tai64n"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .and_then(|mut child| {
            child.stdin.take().unwrap().write_all(b"line\n")?;
            child.wait_with_output()
        })
        .unwrap();
    assert!(output.status.success());
    let text = String::from_utf8(output.stdout).unwrap();
    assert!(text.starts_with("@4"));
    assert!(text.ends_with(" line\n"));
}
```
