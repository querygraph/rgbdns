---
type: "code-fragment"
fragment_id: "rgbdns-frag-d2d519f43a8b"
source_path: "src/tinydns_edit.rs"
code_note: "DNS from First Principles/Code/src/tinydns_edit.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "host_mode_rejects_duplicate_owner_or_address"
kind: "fn"
start_line: 274
end_line: 300
---

# host_mode_rejects_duplicate_owner_or_address

- Fragment ID: `rgbdns-frag-d2d519f43a8b`
- Source file: [[DNS from First Principles/Code/src/tinydns_edit.rs.source|src/tinydns_edit.rs]]
- Lines: 274-300
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-d2d519f43a8b", "codeNote": "DNS from First Principles/Code/src/tinydns_edit.rs.source", "heading": "rgbdns-frag-d2d519f43a8b: fn host_mode_rejects_duplicate_owner_or_address", "sourcePath": "src/tinydns_edit.rs", "startLine": 274, "endLine": 300}
```

## Excerpt

<span id="rgbdns-frag-d2d519f43a8b" class="rgbdns-fragment-target"></span>
### rgbdns-frag-d2d519f43a8b: fn host_mode_rejects_duplicate_owner_or_address

```rust
    fn host_mode_rejects_duplicate_owner_or_address() {
        let (data, temporary) = paths();
        fs::write(&data, "=host.example:192.0.2.1:60\n").unwrap();
        assert!(
            add(
                &data,
                &temporary,
                Mode::Host,
                "host.example".parse().unwrap(),
                Address::V4("192.0.2.2".parse().unwrap()),
            )
            .is_err()
        );
        assert!(
            add(
                &data,
                &temporary,
                Mode::Host,
                "other.example".parse().unwrap(),
                Address::V4("192.0.2.1".parse().unwrap()),
            )
            .is_err()
        );
        fs::remove_file(data).unwrap();
    }

    #[test]
```
