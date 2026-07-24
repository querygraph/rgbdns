---
type: "code-fragment"
fragment_id: "rgbdns-frag-78979cd5a287"
source_path: "src/dnscache_config.rs"
code_note: "DNS from First Principles/Code/src/dnscache_config.rs.source"
language: "rust"
subsystem: "Resolution and recursion"
crate: "rgbdns"
symbol: "loads_bounded_per_zone_forwarders"
kind: "fn"
start_line: 245
end_line: 266
---

# loads_bounded_per_zone_forwarders

- Fragment ID: `rgbdns-frag-78979cd5a287`
- Source file: [[DNS from First Principles/Code/src/dnscache_config.rs.source|src/dnscache_config.rs]]
- Lines: 245-266
- Subsystem: [[DNS from First Principles/Subsystems/Resolution and recursion|Resolution and recursion]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-78979cd5a287", "codeNote": "DNS from First Principles/Code/src/dnscache_config.rs.source", "heading": "rgbdns-frag-78979cd5a287: fn loads_bounded_per_zone_forwarders", "sourcePath": "src/dnscache_config.rs", "startLine": 245, "endLine": 266}
```

## Excerpt

<span id="rgbdns-frag-78979cd5a287" class="rgbdns-fragment-target"></span>
### rgbdns-frag-78979cd5a287: fn loads_bounded_per_zone_forwarders

```rust
    fn loads_bounded_per_zone_forwarders() {
        let directory = path("forwarders");
        fs::create_dir(&directory).unwrap();
        fs::write(directory.join("@"), "198.41.0.4\n").unwrap();
        fs::write(
            directory.join("internal.example"),
            "192.0.2.53\n2001:db8::53\n",
        )
        .unwrap();
        assert_eq!(
            load_forward_zones(&directory).unwrap(),
            vec![ForwardZone {
                name: "internal.example".into(),
                servers: vec![
                    "192.0.2.53".parse().unwrap(),
                    "2001:db8::53".parse().unwrap()
                ],
            }]
        );
        fs::remove_dir_all(directory).unwrap();
    }
}
```
