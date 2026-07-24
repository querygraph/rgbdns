---
type: "code-fragment"
fragment_id: "rgbdns-frag-4ecc3d4e7baf"
source_path: "src/tai64.rs"
code_note: "DNS from First Principles/Code/src/tai64.rs.source"
language: "rust"
subsystem: "Operations and supervision"
crate: "rgbdns"
symbol: "current_labels_include_the_post_2017_offset"
kind: "fn"
start_line: 202
end_line: 214
---

# current_labels_include_the_post_2017_offset

- Fragment ID: `rgbdns-frag-4ecc3d4e7baf`
- Source file: [[DNS from First Principles/Code/src/tai64.rs.source|src/tai64.rs]]
- Lines: 202-214
- Subsystem: [[DNS from First Principles/Subsystems/Operations and supervision|Operations and supervision]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-4ecc3d4e7baf", "codeNote": "DNS from First Principles/Code/src/tai64.rs.source", "heading": "rgbdns-frag-4ecc3d4e7baf: fn current_labels_include_the_post_2017_offset", "sourcePath": "src/tai64.rs", "startLine": 202, "endLine": 214}
```

## Excerpt

<span id="rgbdns-frag-4ecc3d4e7baf" class="rgbdns-fragment-target"></span>
### rgbdns-frag-4ecc3d4e7baf: fn current_labels_include_the_post_2017_offset

```rust
    fn current_labels_include_the_post_2017_offset() {
        let time = UNIX_EPOCH + Duration::from_secs(1_483_228_800);
        assert_eq!(&label(time)[..17], "@40000000586846a5");
        assert_eq!(
            parse_label(&label(time)).unwrap(),
            Timestamp {
                unix_seconds: 1_483_228_800,
                nanoseconds: 0,
            }
        );
    }

    #[test]
```
