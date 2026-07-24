---
type: "code-fragment"
fragment_id: "rgbdns-frag-141adf020e41"
source_path: "tests/daemontools.rs"
code_note: "DNS from First Principles/Code/tests/daemontools.rs.source"
language: "rust"
subsystem: "Tests and performance"
symbol: "multilog_binary_timestamps_and_rotates"
kind: "fn"
start_line: 24
end_line: 56
---

# multilog_binary_timestamps_and_rotates

- Fragment ID: `rgbdns-frag-141adf020e41`
- Source file: [[DNS from First Principles/Code/tests/daemontools.rs.source|tests/daemontools.rs]]
- Lines: 24-56
- Subsystem: [[DNS from First Principles/Subsystems/Tests and performance|Tests and performance]]

```rgbdns-fragment
{"id": "rgbdns-frag-141adf020e41", "codeNote": "DNS from First Principles/Code/tests/daemontools.rs.source", "heading": "rgbdns-frag-141adf020e41: fn multilog_binary_timestamps_and_rotates", "sourcePath": "tests/daemontools.rs", "startLine": 24, "endLine": 56}
```

## Excerpt

<span id="rgbdns-frag-141adf020e41" class="rgbdns-fragment-target"></span>
### rgbdns-frag-141adf020e41: fn multilog_binary_timestamps_and_rotates

```rust
fn multilog_binary_timestamps_and_rotates() {
    let directory = directory();
    let mut child = Command::new(env!("CARGO_BIN_EXE_multilog"))
        .args(["t", "s40", "n2"])
        .arg(&directory)
        .stdin(Stdio::piped())
        .spawn()
        .unwrap();
    child
        .stdin
        .take()
        .unwrap()
        .write_all(b"alpha\nbeta\ngamma\n")
        .unwrap();
    assert!(child.wait().unwrap().success());

    let entries = fs::read_dir(&directory)
        .unwrap()
        .map(|entry| entry.unwrap().file_name().to_string_lossy().into_owned())
        .collect::<Vec<_>>();
    assert!(entries.contains(&"current".to_owned()));
    assert_eq!(
        entries
            .iter()
            .filter(|name| name.starts_with('@') && name.ends_with(".s"))
            .count(),
        2
    );
    assert!(entries.iter().all(|name| !name.contains(' ')));
    fs::remove_dir_all(directory).unwrap();
}

#[test]
```
