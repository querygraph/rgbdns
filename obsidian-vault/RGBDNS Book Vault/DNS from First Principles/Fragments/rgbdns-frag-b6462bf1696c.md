---
type: "code-fragment"
fragment_id: "rgbdns-frag-b6462bf1696c"
source_path: "src/tai64.rs"
code_note: "DNS from First Principles/Code/src/tai64.rs.source"
language: "rust"
subsystem: "Operations and supervision"
crate: "rgbdns"
symbol: "matches_the_published_daemontools_example"
kind: "fn"
start_line: 195
end_line: 201
---

# matches_the_published_daemontools_example

- Fragment ID: `rgbdns-frag-b6462bf1696c`
- Source file: [[DNS from First Principles/Code/src/tai64.rs.source|src/tai64.rs]]
- Lines: 195-201
- Subsystem: [[DNS from First Principles/Subsystems/Operations and supervision|Operations and supervision]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-b6462bf1696c", "codeNote": "DNS from First Principles/Code/src/tai64.rs.source", "heading": "rgbdns-frag-b6462bf1696c: fn matches_the_published_daemontools_example", "sourcePath": "src/tai64.rs", "startLine": 195, "endLine": 201}
```

## Excerpt

<span id="rgbdns-frag-b6462bf1696c" class="rgbdns-fragment-target"></span>
### rgbdns-frag-b6462bf1696c: fn matches_the_published_daemontools_example

```rust
    fn matches_the_published_daemontools_example() {
        let parsed = parse_label("@4000000037c219bf2ef02e94").unwrap();
        assert_eq!(parsed.unix_seconds, 935_467_423);
        assert_eq!(parsed.nanoseconds, 787_492_500);
    }

    #[test]
```
