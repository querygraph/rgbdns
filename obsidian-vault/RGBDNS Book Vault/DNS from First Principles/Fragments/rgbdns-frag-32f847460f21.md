---
type: "code-fragment"
fragment_id: "rgbdns-frag-32f847460f21"
source_path: "src/conf.rs"
code_note: "DNS from First Principles/Code/src/conf.rs.source"
language: "rust"
subsystem: "Operations and supervision"
crate: "rgbdns"
symbol: "dnscache_tree_contains_current_hints_and_private_seed"
kind: "fn"
start_line: 286
end_line: 299
---

# dnscache_tree_contains_current_hints_and_private_seed

- Fragment ID: `rgbdns-frag-32f847460f21`
- Source file: [[DNS from First Principles/Code/src/conf.rs.source|src/conf.rs]]
- Lines: 286-299
- Subsystem: [[DNS from First Principles/Subsystems/Operations and supervision|Operations and supervision]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-32f847460f21", "codeNote": "DNS from First Principles/Code/src/conf.rs.source", "heading": "rgbdns-frag-32f847460f21: fn dnscache_tree_contains_current_hints_and_private_seed", "sourcePath": "src/conf.rs", "startLine": 286, "endLine": 299}
```

## Excerpt

<span id="rgbdns-frag-32f847460f21" class="rgbdns-fragment-target"></span>
### rgbdns-frag-32f847460f21: fn dnscache_tree_contains_current_hints_and_private_seed

```rust
    fn dnscache_tree_contains_current_hints_and_private_seed() {
        let directory = directory("dnscache");
        let arguments = vec![
            "dns".into(),
            "log".into(),
            directory.to_string_lossy().into_owned(),
        ];
        configure(Service::Dnscache, &arguments).unwrap();
        let hints = fs::read_to_string(directory.join("root/servers/@")).unwrap();
        assert!(hints.contains("A.ROOT-SERVERS.NET."));
        assert_eq!(fs::metadata(directory.join("seed")).unwrap().len(), 128);
        fs::remove_dir_all(directory).unwrap();
    }
}
```
