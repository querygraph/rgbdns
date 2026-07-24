---
type: "code-fragment"
fragment_id: "rgbdns-frag-f39b8ab849b2"
source_path: "src/wall.rs"
code_note: "DNS from First Principles/Code/src/wall.rs.source"
language: "rust"
subsystem: "Specialized responders"
crate: "rgbdns"
symbol: "direct_and_reverse_wall_mappings"
kind: "fn"
start_line: 83
end_line: 95
---

# direct_and_reverse_wall_mappings

- Fragment ID: `rgbdns-frag-f39b8ab849b2`
- Source file: [[DNS from First Principles/Code/src/wall.rs.source|src/wall.rs]]
- Lines: 83-95
- Subsystem: [[DNS from First Principles/Subsystems/Specialized responders|Specialized responders]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-f39b8ab849b2", "codeNote": "DNS from First Principles/Code/src/wall.rs.source", "heading": "rgbdns-frag-f39b8ab849b2: fn direct_and_reverse_wall_mappings", "sourcePath": "src/wall.rs", "startLine": 83, "endLine": 95}
```

## Excerpt

<span id="rgbdns-frag-f39b8ab849b2" class="rgbdns-fragment-target"></span>
### rgbdns-frag-f39b8ab849b2: fn direct_and_reverse_wall_mappings

```rust
    fn direct_and_reverse_wall_mappings() {
        let direct = query("1.2.3.4", RecordType::A);
        assert_eq!(direct.answers[0].data, RData::A(Ipv4Addr::new(1, 2, 3, 4)));
        let reverse = query("4.3.2.1.in-addr.arpa", RecordType::Any);
        assert_eq!(reverse.answers[0].data, RData::A(Ipv4Addr::new(1, 2, 3, 4)));
        assert!(matches!(
            &reverse.answers[1].data,
            RData::Name(RecordType::Ptr, name)
                if name.to_string() == "4.3.2.1.in-addr.arpa."
        ));
    }

    #[test]
```
