---
type: "code-fragment"
fragment_id: "rgbdns-frag-4d4637ac8ae2"
source_path: "src/name.rs"
code_note: "DNS from First Principles/Code/src/name.rs.source"
language: "rust"
subsystem: "DNS data model and wire codec"
crate: "rgbdns"
symbol: "root_and_case"
kind: "fn"
start_line: 207
end_line: 218
---

# root_and_case

- Fragment ID: `rgbdns-frag-4d4637ac8ae2`
- Source file: [[DNS from First Principles/Code/src/name.rs.source|src/name.rs]]
- Lines: 207-218
- Subsystem: [[DNS from First Principles/Subsystems/DNS data model and wire codec|DNS data model and wire codec]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-4d4637ac8ae2", "codeNote": "DNS from First Principles/Code/src/name.rs.source", "heading": "rgbdns-frag-4d4637ac8ae2: fn root_and_case", "sourcePath": "src/name.rs", "startLine": 207, "endLine": 218}
```

## Excerpt

<span id="rgbdns-frag-4d4637ac8ae2" class="rgbdns-fragment-target"></span>
### rgbdns-frag-4d4637ac8ae2: fn root_and_case

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
    }
    #[test]
```
