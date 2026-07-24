---
type: "code-fragment"
fragment_id: "rgbdns-frag-a8de23dd524c"
source_path: "src/rbl.rs"
code_note: "DNS from First Principles/Code/src/rbl.rs.source"
language: "rust"
subsystem: "Specialized responders"
crate: "rgbdns"
symbol: "most_specific_network_and_substitution"
kind: "fn"
start_line: 241
end_line: 268
---

# most_specific_network_and_substitution

- Fragment ID: `rgbdns-frag-a8de23dd524c`
- Source file: [[DNS from First Principles/Code/src/rbl.rs.source|src/rbl.rs]]
- Lines: 241-268
- Subsystem: [[DNS from First Principles/Subsystems/Specialized responders|Specialized responders]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-a8de23dd524c", "codeNote": "DNS from First Principles/Code/src/rbl.rs.source", "heading": "rgbdns-frag-a8de23dd524c: fn most_specific_network_and_substitution", "sourcePath": "src/rbl.rs", "startLine": 241, "endLine": 268}
```

## Excerpt

<span id="rgbdns-frag-a8de23dd524c" class="rgbdns-fragment-target"></span>
### rgbdns-frag-a8de23dd524c: fn most_specific_network_and_substitution

```rust
    fn most_specific_network_and_substitution() {
        let database = Database::parse(":127.0.0.9:Blocked $\n1.2.3/24\n10/8\n").unwrap();
        let base = "rbl.example".parse().unwrap();
        let response = Message::decode(
            &database
                .respond(&base, &query("4.3.2.1.rbl.example", RecordType::Any), 512)
                .unwrap(),
        )
        .unwrap();
        assert_eq!(response.answers.len(), 2);
        assert_eq!(
            response.answers[0].data,
            RData::A(Ipv4Addr::new(127, 0, 0, 9))
        );
        assert_eq!(
            response.answers[1].data,
            RData::Txt(vec![b"Blocked 1.2.3.4".to_vec()])
        );
        let missing = Message::decode(
            &database
                .respond(&base, &query("4.3.9.1.rbl.example", RecordType::A), 512)
                .unwrap(),
        )
        .unwrap();
        assert_eq!(missing.flags & 15, 3);
    }

    #[test]
```
