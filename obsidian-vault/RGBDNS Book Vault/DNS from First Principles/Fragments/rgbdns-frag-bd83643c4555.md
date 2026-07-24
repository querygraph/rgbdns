---
type: "code-fragment"
fragment_id: "rgbdns-frag-bd83643c4555"
source_path: "src/multilog.rs"
code_note: "DNS from First Principles/Code/src/multilog.rs.source"
language: "rust"
subsystem: "Operations and supervision"
crate: "rgbdns"
symbol: "rotates_atomically_and_enforces_retention"
kind: "fn"
start_line: 271
end_line: 295
---

# rotates_atomically_and_enforces_retention

- Fragment ID: `rgbdns-frag-bd83643c4555`
- Source file: [[DNS from First Principles/Code/src/multilog.rs.source|src/multilog.rs]]
- Lines: 271-295
- Subsystem: [[DNS from First Principles/Subsystems/Operations and supervision|Operations and supervision]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-bd83643c4555", "codeNote": "DNS from First Principles/Code/src/multilog.rs.source", "heading": "rgbdns-frag-bd83643c4555: fn rotates_atomically_and_enforces_retention", "sourcePath": "src/multilog.rs", "startLine": 271, "endLine": 295}
```

## Excerpt

<span id="rgbdns-frag-bd83643c4555" class="rgbdns-fragment-target"></span>
### rgbdns-frag-bd83643c4555: fn rotates_atomically_and_enforces_retention

```rust
    fn rotates_atomically_and_enforces_retention() {
        let path = directory("rotate");
        let config = Config {
            timestamp: false,
            max_size: 4,
            retain: 2,
            directories: vec![path.clone()],
        };
        run(&config, Cursor::new(b"aaaa\nbbbb\ncccc\n")).unwrap();
        let rotated = fs::read_dir(&path)
            .unwrap()
            .filter_map(|entry| entry.ok())
            .filter(|entry| {
                let name = entry.file_name();
                assert!(!name.to_string_lossy().contains(' '));
                name != "current"
            })
            .count();
        assert_eq!(rotated, 2);
        assert_eq!(fs::read(path.join("current")).unwrap(), b"cccc\n");
        fs::remove_dir_all(path).unwrap();
    }

    #[cfg(unix)]
    #[test]
```
