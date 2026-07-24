---
type: "code-fragment"
fragment_id: "rgbdns-frag-6bf6af6b6066"
source_path: "src/multilog.rs"
code_note: "DNS from First Principles/Code/src/multilog.rs.source"
language: "rust"
subsystem: "Operations and supervision"
crate: "rgbdns"
symbol: "refuses_a_symlinked_current_file"
kind: "fn"
start_line: 296
end_line: 314
---

# refuses_a_symlinked_current_file

- Fragment ID: `rgbdns-frag-6bf6af6b6066`
- Source file: [[DNS from First Principles/Code/src/multilog.rs.source|src/multilog.rs]]
- Lines: 296-314
- Subsystem: [[DNS from First Principles/Subsystems/Operations and supervision|Operations and supervision]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-6bf6af6b6066", "codeNote": "DNS from First Principles/Code/src/multilog.rs.source", "heading": "rgbdns-frag-6bf6af6b6066: fn refuses_a_symlinked_current_file", "sourcePath": "src/multilog.rs", "startLine": 296, "endLine": 314}
```

## Excerpt

<span id="rgbdns-frag-6bf6af6b6066" class="rgbdns-fragment-target"></span>
### rgbdns-frag-6bf6af6b6066: fn refuses_a_symlinked_current_file

```rust
    fn refuses_a_symlinked_current_file() {
        use std::os::unix::fs::symlink;

        let path = directory("symlink");
        fs::create_dir(&path).unwrap();
        let target = path.join("target");
        fs::write(&target, b"unchanged").unwrap();
        symlink(&target, path.join("current")).unwrap();
        let config = Config {
            timestamp: false,
            max_size: 100,
            retain: 2,
            directories: vec![path.clone()],
        };
        assert!(run(&config, Cursor::new(b"attack\n")).is_err());
        assert_eq!(fs::read(target).unwrap(), b"unchanged");
        fs::remove_dir_all(path).unwrap();
    }
}
```
