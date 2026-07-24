---
type: "code-fragment"
fragment_id: "rgbdns-frag-e0c3c03feb8d"
source_path: "src/multilog.rs"
code_note: "DNS from First Principles/Code/src/multilog.rs.source"
language: "rust"
subsystem: "Operations and supervision"
crate: "rgbdns"
symbol: "parses_supported_daemontools_options"
kind: "fn"
start_line: 224
end_line: 234
---

# parses_supported_daemontools_options

- Fragment ID: `rgbdns-frag-e0c3c03feb8d`
- Source file: [[DNS from First Principles/Code/src/multilog.rs.source|src/multilog.rs]]
- Lines: 224-234
- Subsystem: [[DNS from First Principles/Subsystems/Operations and supervision|Operations and supervision]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-e0c3c03feb8d", "codeNote": "DNS from First Principles/Code/src/multilog.rs.source", "heading": "rgbdns-frag-e0c3c03feb8d: fn parses_supported_daemontools_options", "sourcePath": "src/multilog.rs", "startLine": 224, "endLine": 234}
```

## Excerpt

<span id="rgbdns-frag-e0c3c03feb8d" class="rgbdns-fragment-target"></span>
### rgbdns-frag-e0c3c03feb8d: fn parses_supported_daemontools_options

```rust
    fn parses_supported_daemontools_options() {
        let config = Config::parse(&["t", "s1024", "n5", "./main"].map(str::to_owned)).unwrap();
        assert!(config.timestamp);
        assert_eq!(config.max_size, 1024);
        assert_eq!(config.retain, 5);
        assert_eq!(config.directories, [PathBuf::from("./main")]);
        assert!(Config::parse(&["s0".into(), "main".into()]).is_err());
        assert!(Config::parse(&["t".into()]).is_err());
    }

    #[test]
```
