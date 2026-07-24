---
type: "code-fragment"
fragment_id: "rgbdns-frag-2c2cadfbad0f"
source_path: "src/tai64.rs"
code_note: "DNS from First Principles/Code/src/tai64.rs.source"
language: "rust"
subsystem: "Operations and supervision"
crate: "rgbdns"
symbol: "stream_filters_preserve_lines_and_invalid_prefixes"
kind: "fn"
start_line: 215
end_line: 233
---

# stream_filters_preserve_lines_and_invalid_prefixes

- Fragment ID: `rgbdns-frag-2c2cadfbad0f`
- Source file: [[DNS from First Principles/Code/src/tai64.rs.source|src/tai64.rs]]
- Lines: 215-233
- Subsystem: [[DNS from First Principles/Subsystems/Operations and supervision|Operations and supervision]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-2c2cadfbad0f", "codeNote": "DNS from First Principles/Code/src/tai64.rs.source", "heading": "rgbdns-frag-2c2cadfbad0f: fn stream_filters_preserve_lines_and_invalid_prefixes", "sourcePath": "src/tai64.rs", "startLine": 215, "endLine": 233}
```

## Excerpt

<span id="rgbdns-frag-2c2cadfbad0f" class="rgbdns-fragment-target"></span>
### rgbdns-frag-2c2cadfbad0f: fn stream_filters_preserve_lines_and_invalid_prefixes

```rust
    fn stream_filters_preserve_lines_and_invalid_prefixes() {
        let mut stamped = Vec::new();
        stamp(Cursor::new(b"one\ntwo"), &mut stamped).unwrap();
        let text = String::from_utf8(stamped).unwrap();
        assert_eq!(text.lines().count(), 2);
        assert!(text.lines().all(|line| line.starts_with("@4")));

        let mut localized = Vec::new();
        localize(
            Cursor::new(b"not-a-stamp\n@4000000037c219bf2ef02e94 mark\n"),
            &mut localized,
        )
        .unwrap();
        let localized = String::from_utf8(localized).unwrap();
        assert!(localized.starts_with("not-a-stamp\n"));
        assert!(localized.ends_with(".787492500 mark\n"));
    }

    #[test]
```
