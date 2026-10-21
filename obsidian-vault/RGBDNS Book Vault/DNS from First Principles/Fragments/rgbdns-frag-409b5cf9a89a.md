---
type: "code-fragment"
fragment_id: "rgbdns-frag-409b5cf9a89a"
source_path: "src/pick.rs"
code_note: "DNS from First Principles/Code/src/pick.rs.source"
language: "rust"
subsystem: "Specialized responders"
crate: "rgbdns"
symbol: "location_selection_falls_back_and_caps_answers"
kind: "fn"
start_line: 249
end_line: 280
---

# location_selection_falls_back_and_caps_answers

- Fragment ID: `rgbdns-frag-409b5cf9a89a`
- Source file: [[DNS from First Principles/Code/src/pick.rs.source|src/pick.rs]]
- Lines: 249-280
- Subsystem: [[DNS from First Principles/Subsystems/Specialized responders|Specialized responders]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-409b5cf9a89a", "codeNote": "DNS from First Principles/Code/src/pick.rs.source", "sourcePath": "src/pick.rs", "startLine": 249, "endLine": 280}
```

## Excerpt

<span id="rgbdns-frag-409b5cf9a89a" class="rgbdns-fragment-target"></span>
### rgbdns-frag-409b5cf9a89a: fn location_selection_falls_back_and_caps_answers

```rust
    fn location_selection_falls_back_and_caps_answers() {
        let database = Database::parse(
            "%aa:192.0.2\n\
             +www.example:192.0.2.1:\n\
             +www.example:192.0.2.2:aa\n\
             +www.example:192.0.2.3:aa\n\
             +www.example:192.0.2.4:aa\n\
             +www.example:192.0.2.5:aa\n",
        )
        .unwrap();
        let local = Message::decode(
            &database
                .respond(&query("www.example"), 512, "192.0.2.44".parse().unwrap())
                .unwrap(),
        )
        .unwrap();
        assert_eq!(local.answers.len(), 3);
        assert!(local.answers.iter().all(|record| record.ttl == 5));
        let fallback = Message::decode(
            &database
                .respond(&query("www.example"), 512, "198.51.100.1".parse().unwrap())
                .unwrap(),
        )
        .unwrap();
        assert_eq!(fallback.answers.len(), 1);
        assert_eq!(
            fallback.answers[0].data,
            RData::A(Ipv4Addr::new(192, 0, 2, 1))
        );
    }

    #[test]
```
