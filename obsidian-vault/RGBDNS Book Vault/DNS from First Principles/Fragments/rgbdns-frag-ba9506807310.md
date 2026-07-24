---
type: "code-fragment"
fragment_id: "rgbdns-frag-ba9506807310"
source_path: "src/conf.rs"
code_note: "DNS from First Principles/Code/src/conf.rs.source"
language: "rust"
subsystem: "Operations and supervision"
crate: "rgbdns"
symbol: "creates_tinydns_service_tree_without_overwriting"
kind: "fn"
start_line: 264
end_line: 285
---

# creates_tinydns_service_tree_without_overwriting

- Fragment ID: `rgbdns-frag-ba9506807310`
- Source file: [[DNS from First Principles/Code/src/conf.rs.source|src/conf.rs]]
- Lines: 264-285
- Subsystem: [[DNS from First Principles/Subsystems/Operations and supervision|Operations and supervision]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-ba9506807310", "codeNote": "DNS from First Principles/Code/src/conf.rs.source", "heading": "rgbdns-frag-ba9506807310: fn creates_tinydns_service_tree_without_overwriting", "sourcePath": "src/conf.rs", "startLine": 264, "endLine": 285}
```

## Excerpt

<span id="rgbdns-frag-ba9506807310" class="rgbdns-fragment-target"></span>
### rgbdns-frag-ba9506807310: fn creates_tinydns_service_tree_without_overwriting

```rust
    fn creates_tinydns_service_tree_without_overwriting() {
        let directory = directory("tinydns");
        let arguments = vec![
            "dns".into(),
            "log".into(),
            directory.to_string_lossy().into_owned(),
            "127.0.0.1".into(),
        ];
        configure(Service::Tinydns, &arguments).unwrap();
        assert!(directory.join("run").is_file());
        let run = fs::read_to_string(directory.join("run")).unwrap();
        let log_run = fs::read_to_string(directory.join("log/run")).unwrap();
        assert!(run.contains("/setuidgid"));
        assert!(log_run.contains("/setuidgid"));
        assert!(log_run.contains("/multilog"));
        assert!(directory.join("root/data").is_file());
        assert!(directory.join("root/add-host").is_file());
        assert!(configure(Service::Tinydns, &arguments).is_err());
        fs::remove_dir_all(directory).unwrap();
    }

    #[test]
```
