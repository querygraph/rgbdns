---
type: "code-fragment"
fragment_id: "rgbdns-frag-896528afc440"
source_path: "src/dnscache_config.rs"
code_note: "DNS from First Principles/Code/src/dnscache_config.rs.source"
language: "rust"
subsystem: "Resolution and recursion"
crate: "rgbdns"
symbol: "converts_legacy_ipv4_and_ipv6_server_lines"
kind: "fn"
start_line: 210
end_line: 224
---

# converts_legacy_ipv4_and_ipv6_server_lines

- Fragment ID: `rgbdns-frag-896528afc440`
- Source file: [[DNS from First Principles/Code/src/dnscache_config.rs.source|src/dnscache_config.rs]]
- Lines: 210-224
- Subsystem: [[DNS from First Principles/Subsystems/Resolution and recursion|Resolution and recursion]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-896528afc440", "codeNote": "DNS from First Principles/Code/src/dnscache_config.rs.source", "heading": "rgbdns-frag-896528afc440: fn converts_legacy_ipv4_and_ipv6_server_lines", "sourcePath": "src/dnscache_config.rs", "startLine": 210, "endLine": 224}
```

## Excerpt

<span id="rgbdns-frag-896528afc440" class="rgbdns-fragment-target"></span>
### rgbdns-frag-896528afc440: fn converts_legacy_ipv4_and_ipv6_server_lines

```rust
    fn converts_legacy_ipv4_and_ipv6_server_lines() {
        let source = path("legacy");
        fs::write(&source, "# roots\n198.41.0.4\n2001:503:ba3e::2:30\n").unwrap();
        let prepared = PreparedRoots::prepare(source.clone()).unwrap();
        assert_ne!(prepared.path(), source);
        let master = fs::read_to_string(prepared.path()).unwrap();
        assert!(master.contains(" A 198.41.0.4"));
        assert!(master.contains(" AAAA 2001:503:ba3e::2:30"));
        let temporary = prepared.path().to_owned();
        drop(prepared);
        assert!(!temporary.exists());
        fs::remove_file(source).unwrap();
    }

    #[test]
```
