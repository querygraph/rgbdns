---
type: "code-fragment"
fragment_id: "rgbdns-frag-365aec844a5c"
source_path: "src/tinydns_edit.rs"
code_note: "DNS from First Principles/Code/src/tinydns_edit.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "allocates_next_ns_and_mx_slots_atomically"
kind: "fn"
start_line: 244
end_line: 273
---

# allocates_next_ns_and_mx_slots_atomically

- Fragment ID: `rgbdns-frag-365aec844a5c`
- Source file: [[DNS from First Principles/Code/src/tinydns_edit.rs.source|src/tinydns_edit.rs]]
- Lines: 244-273
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-365aec844a5c", "codeNote": "DNS from First Principles/Code/src/tinydns_edit.rs.source", "heading": "rgbdns-frag-365aec844a5c: fn allocates_next_ns_and_mx_slots_atomically", "sourcePath": "src/tinydns_edit.rs", "startLine": 244, "endLine": 273}
```

## Excerpt

<span id="rgbdns-frag-365aec844a5c" class="rgbdns-fragment-target"></span>
### rgbdns-frag-365aec844a5c: fn allocates_next_ns_and_mx_slots_atomically

```rust
    fn allocates_next_ns_and_mx_slots_atomically() {
        let (data, temporary) = paths();
        fs::write(
            &data,
            ".example:192.0.2.1:a:300\n@example:192.0.2.2:a::400\n",
        )
        .unwrap();
        add(
            &data,
            &temporary,
            Mode::Ns,
            "example".parse().unwrap(),
            Address::V4("192.0.2.3".parse().unwrap()),
        )
        .unwrap();
        add(
            &data,
            &temporary,
            Mode::Mx,
            "example".parse().unwrap(),
            Address::V4("192.0.2.4".parse().unwrap()),
        )
        .unwrap();
        let result = fs::read_to_string(&data).unwrap();
        fs::remove_file(data).unwrap();
        assert!(result.contains(".example:192.0.2.3:b:300\n"));
        assert!(result.contains("@example:192.0.2.4:b::400\n"));
    }

    #[test]
```
