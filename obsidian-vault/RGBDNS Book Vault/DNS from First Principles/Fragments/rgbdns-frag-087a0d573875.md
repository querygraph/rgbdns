---
type: "code-fragment"
fragment_id: "rgbdns-frag-087a0d573875"
source_path: "src/tinydns_edit.rs"
code_note: "DNS from First Principles/Code/src/tinydns_edit.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "ipv6_modes_emit_flat_unambiguous_addresses"
kind: "fn"
start_line: 301
end_line: 319
---

# ipv6_modes_emit_flat_unambiguous_addresses

- Fragment ID: `rgbdns-frag-087a0d573875`
- Source file: [[DNS from First Principles/Code/src/tinydns_edit.rs.source|src/tinydns_edit.rs]]
- Lines: 301-319
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-087a0d573875", "codeNote": "DNS from First Principles/Code/src/tinydns_edit.rs.source", "heading": "rgbdns-frag-087a0d573875: fn ipv6_modes_emit_flat_unambiguous_addresses", "sourcePath": "src/tinydns_edit.rs", "startLine": 301, "endLine": 319}
```

## Excerpt

<span id="rgbdns-frag-087a0d573875" class="rgbdns-fragment-target"></span>
### rgbdns-frag-087a0d573875: fn ipv6_modes_emit_flat_unambiguous_addresses

```rust
    fn ipv6_modes_emit_flat_unambiguous_addresses() {
        let (data, temporary) = paths();
        fs::write(&data, "").unwrap();
        add(
            &data,
            &temporary,
            Mode::Host6,
            "v6.example".parse().unwrap(),
            Address::V6("2001:db8::1".parse().unwrap()),
        )
        .unwrap();
        let contents = fs::read_to_string(&data).unwrap();
        fs::remove_file(data).unwrap();
        assert_eq!(
            contents,
            "6v6.example:20010db8000000000000000000000001:86400\n"
        );
    }
}
```
