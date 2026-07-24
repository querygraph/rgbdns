---
type: "code-fragment"
fragment_id: "rgbdns-frag-4372b73cba03"
source_path: "src/dnscache_config.rs"
code_note: "DNS from First Principles/Code/src/dnscache_config.rs.source"
language: "rust"
subsystem: "Resolution and recursion"
crate: "rgbdns"
symbol: "preserves_master_files_and_rejects_mixed_input"
kind: "fn"
start_line: 225
end_line: 244
---

# preserves_master_files_and_rejects_mixed_input

- Fragment ID: `rgbdns-frag-4372b73cba03`
- Source file: [[DNS from First Principles/Code/src/dnscache_config.rs.source|src/dnscache_config.rs]]
- Lines: 225-244
- Subsystem: [[DNS from First Principles/Subsystems/Resolution and recursion|Resolution and recursion]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-4372b73cba03", "codeNote": "DNS from First Principles/Code/src/dnscache_config.rs.source", "heading": "rgbdns-frag-4372b73cba03: fn preserves_master_files_and_rejects_mixed_input", "sourcePath": "src/dnscache_config.rs", "startLine": 225, "endLine": 244}
```

## Excerpt

<span id="rgbdns-frag-4372b73cba03" class="rgbdns-fragment-target"></span>
### rgbdns-frag-4372b73cba03: fn preserves_master_files_and_rejects_mixed_input

```rust
    fn preserves_master_files_and_rejects_mixed_input() {
        let master = path("master");
        fs::write(
            &master,
            ". 3600000 NS a.root.\na.root. 3600000 A 192.0.2.1\n",
        )
        .unwrap();
        let prepared = PreparedRoots::prepare(master.clone()).unwrap();
        assert_eq!(prepared.path(), master);
        drop(prepared);
        assert!(master.exists());
        fs::remove_file(master).unwrap();

        let mixed = path("mixed");
        fs::write(&mixed, "192.0.2.1\nnot-an-address\n").unwrap();
        assert!(PreparedRoots::prepare(mixed.clone()).is_err());
        fs::remove_file(mixed).unwrap();
    }

    #[test]
```
