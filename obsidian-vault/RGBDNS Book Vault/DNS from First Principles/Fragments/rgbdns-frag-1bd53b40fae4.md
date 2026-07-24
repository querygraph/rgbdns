---
type: "code-fragment"
fragment_id: "rgbdns-frag-1bd53b40fae4"
source_path: "tests/daemontools.rs"
code_note: "DNS from First Principles/Code/tests/daemontools.rs.source"
language: "rust"
subsystem: "Tests and performance"
symbol: "setuidgid_replaces_itself_and_preserves_child_status"
kind: "fn"
start_line: 57
end_line: 67
---

# setuidgid_replaces_itself_and_preserves_child_status

- Fragment ID: `rgbdns-frag-1bd53b40fae4`
- Source file: [[DNS from First Principles/Code/tests/daemontools.rs.source|tests/daemontools.rs]]
- Lines: 57-67
- Subsystem: [[DNS from First Principles/Subsystems/Tests and performance|Tests and performance]]

```rgbdns-fragment
{"id": "rgbdns-frag-1bd53b40fae4", "codeNote": "DNS from First Principles/Code/tests/daemontools.rs.source", "heading": "rgbdns-frag-1bd53b40fae4: fn setuidgid_replaces_itself_and_preserves_child_status", "sourcePath": "tests/daemontools.rs", "startLine": 57, "endLine": 67}
```

## Excerpt

<span id="rgbdns-frag-1bd53b40fae4" class="rgbdns-fragment-target"></span>
### rgbdns-frag-1bd53b40fae4: fn setuidgid_replaces_itself_and_preserves_child_status

```rust
fn setuidgid_replaces_itself_and_preserves_child_status() {
    let user = User::from_uid(Uid::effective()).unwrap().unwrap();
    let status = Command::new(env!("CARGO_BIN_EXE_setuidgid"))
        .arg(user.name)
        .args(["/bin/sh", "-c", "exit 7"])
        .status()
        .unwrap();
    assert_eq!(status.code(), Some(7));
}

#[test]
```
