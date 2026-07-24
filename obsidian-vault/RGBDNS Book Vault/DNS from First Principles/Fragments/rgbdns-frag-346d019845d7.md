---
type: "code-fragment"
fragment_id: "rgbdns-frag-346d019845d7"
source_path: "src/multilog.rs"
code_note: "DNS from First Principles/Code/src/multilog.rs.source"
language: "rust"
subsystem: "Operations and supervision"
crate: "rgbdns"
symbol: "timestamps_streamed_lines_without_buffering_the_input"
kind: "fn"
start_line: 235
end_line: 253
---

# timestamps_streamed_lines_without_buffering_the_input

- Fragment ID: `rgbdns-frag-346d019845d7`
- Source file: [[DNS from First Principles/Code/src/multilog.rs.source|src/multilog.rs]]
- Lines: 235-253
- Subsystem: [[DNS from First Principles/Subsystems/Operations and supervision|Operations and supervision]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-346d019845d7", "codeNote": "DNS from First Principles/Code/src/multilog.rs.source", "heading": "rgbdns-frag-346d019845d7: fn timestamps_streamed_lines_without_buffering_the_input", "sourcePath": "src/multilog.rs", "startLine": 235, "endLine": 253}
```

## Excerpt

<span id="rgbdns-frag-346d019845d7" class="rgbdns-fragment-target"></span>
### rgbdns-frag-346d019845d7: fn timestamps_streamed_lines_without_buffering_the_input

```rust
    fn timestamps_streamed_lines_without_buffering_the_input() {
        let path = directory("timestamp");
        let config = Config {
            timestamp: true,
            max_size: 10_000,
            retain: 2,
            directories: vec![path.clone()],
        };
        run(&config, Cursor::new(b"one\ntwo\n")).unwrap();
        let contents = fs::read_to_string(path.join("current")).unwrap();
        let lines = contents.lines().collect::<Vec<_>>();
        assert_eq!(lines.len(), 2);
        assert!(lines[0].starts_with("@4"));
        assert!(lines[0].ends_with(" one"));
        assert!(lines[1].ends_with(" two"));
        fs::remove_dir_all(path).unwrap();
    }

    #[test]
```
