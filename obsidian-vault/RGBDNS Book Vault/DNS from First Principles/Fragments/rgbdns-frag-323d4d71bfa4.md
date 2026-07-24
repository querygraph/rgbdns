---
type: "code-fragment"
fragment_id: "rgbdns-frag-323d4d71bfa4"
source_path: "src/multilog.rs"
code_note: "DNS from First Principles/Code/src/multilog.rs.source"
language: "rust"
subsystem: "Operations and supervision"
crate: "rgbdns"
symbol: "rotation_does_not_separate_a_timestamp_from_its_line"
kind: "fn"
start_line: 254
end_line: 270
---

# rotation_does_not_separate_a_timestamp_from_its_line

- Fragment ID: `rgbdns-frag-323d4d71bfa4`
- Source file: [[DNS from First Principles/Code/src/multilog.rs.source|src/multilog.rs]]
- Lines: 254-270
- Subsystem: [[DNS from First Principles/Subsystems/Operations and supervision|Operations and supervision]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-323d4d71bfa4", "codeNote": "DNS from First Principles/Code/src/multilog.rs.source", "heading": "rgbdns-frag-323d4d71bfa4: fn rotation_does_not_separate_a_timestamp_from_its_line", "sourcePath": "src/multilog.rs", "startLine": 254, "endLine": 270}
```

## Excerpt

<span id="rgbdns-frag-323d4d71bfa4" class="rgbdns-fragment-target"></span>
### rgbdns-frag-323d4d71bfa4: fn rotation_does_not_separate_a_timestamp_from_its_line

```rust
    fn rotation_does_not_separate_a_timestamp_from_its_line() {
        let path = directory("record-boundary");
        let config = Config {
            timestamp: true,
            max_size: 40,
            retain: 2,
            directories: vec![path.clone()],
        };
        run(&config, Cursor::new(b"alpha\nbeta\n")).unwrap();
        let current = fs::read_to_string(path.join("current")).unwrap();
        assert!(current.starts_with("@4"));
        assert!(current.ends_with(" beta\n"));
        assert!(!current.starts_with("beta"));
        fs::remove_dir_all(path).unwrap();
    }

    #[test]
```
