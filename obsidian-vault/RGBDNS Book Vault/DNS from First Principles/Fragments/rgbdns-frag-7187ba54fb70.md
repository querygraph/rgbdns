---
type: "code-fragment"
fragment_id: "rgbdns-frag-7187ba54fb70"
source_path: "src/setuidgid.rs"
code_note: "DNS from First Principles/Code/src/setuidgid.rs.source"
language: "rust"
subsystem: "Operations and supervision"
crate: "rgbdns"
symbol: "resolves_the_current_account_and_optional_group"
kind: "fn"
start_line: 91
end_line: 99
---

# resolves_the_current_account_and_optional_group

- Fragment ID: `rgbdns-frag-7187ba54fb70`
- Source file: [[DNS from First Principles/Code/src/setuidgid.rs.source|src/setuidgid.rs]]
- Lines: 91-99
- Subsystem: [[DNS from First Principles/Subsystems/Operations and supervision|Operations and supervision]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-7187ba54fb70", "codeNote": "DNS from First Principles/Code/src/setuidgid.rs.source", "heading": "rgbdns-frag-7187ba54fb70: fn resolves_the_current_account_and_optional_group", "sourcePath": "src/setuidgid.rs", "startLine": 91, "endLine": 99}
```

## Excerpt

<span id="rgbdns-frag-7187ba54fb70" class="rgbdns-fragment-target"></span>
### rgbdns-frag-7187ba54fb70: fn resolves_the_current_account_and_optional_group

```rust
    fn resolves_the_current_account_and_optional_group() {
        let current = User::from_uid(Uid::effective()).unwrap().unwrap();
        let identity = resolve(&current.name).unwrap();
        assert_eq!(identity.uid, Uid::effective());
        assert_eq!(identity.gid, current.gid);
        assert!(resolve("rgbdns-user-that-cannot-exist").is_err());
    }

    #[test]
```
