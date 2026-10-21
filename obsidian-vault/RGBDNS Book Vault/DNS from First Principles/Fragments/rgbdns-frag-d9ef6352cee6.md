---
type: "code-fragment"
fragment_id: "rgbdns-frag-d9ef6352cee6"
source_path: "src/name.rs"
code_note: "DNS from First Principles/Code/src/name.rs.source"
language: "rust"
subsystem: "DNS data model and wire codec"
crate: "rgbdns"
symbol: "root_and_case"
kind: "fn"
start_line: 219
end_line: 236
---

# root_and_case

- Fragment ID: `rgbdns-frag-d9ef6352cee6`
- Source file: [[DNS from First Principles/Code/src/name.rs.source|src/name.rs]]
- Lines: 219-236
- Subsystem: [[DNS from First Principles/Subsystems/DNS data model and wire codec|DNS data model and wire codec]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-d9ef6352cee6", "codeNote": "DNS from First Principles/Code/src/name.rs.source", "sourcePath": "src/name.rs", "startLine": 219, "endLine": 236}
```

## Excerpt

<span id="rgbdns-frag-d9ef6352cee6" class="rgbdns-fragment-target"></span>
### rgbdns-frag-d9ef6352cee6: fn root_and_case

```rust
    fn root_and_case() {
        assert_eq!(
            "WWW.Example".parse::<Name>().unwrap().to_string(),
            "WWW.Example."
        );
        assert_eq!(
            "WWW.Example".parse::<Name>().unwrap(),
            "www.example".parse::<Name>().unwrap()
        );
        assert_eq!(".".parse::<Name>().unwrap(), Name::root());
        assert!(
            "MiSsInG.Bitcoin.Science"
                .parse::<Name>()
                .unwrap()
                .is_subdomain_of(&"bitcoin.science".parse().unwrap())
        );
    }
    #[test]
```
