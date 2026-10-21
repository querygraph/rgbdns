---
type: "code-fragment"
fragment_id: "rgbdns-frag-de5bbb54683c"
source_path: "src/dnssec.rs"
code_note: "DNS from First Principles/Code/src/dnssec.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "mixed_policy_requires_one_explicit_disposition_per_zone"
kind: "fn"
start_line: 874
end_line: 904
---

# mixed_policy_requires_one_explicit_disposition_per_zone

- Fragment ID: `rgbdns-frag-de5bbb54683c`
- Source file: [[DNS from First Principles/Code/src/dnssec.rs.source|src/dnssec.rs]]
- Lines: 874-904
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-de5bbb54683c", "codeNote": "DNS from First Principles/Code/src/dnssec.rs.source", "sourcePath": "src/dnssec.rs", "startLine": 874, "endLine": 904}
```

## Excerpt

<span id="rgbdns-frag-de5bbb54683c" class="rgbdns-fragment-target"></span>
### rgbdns-frag-de5bbb54683c: fn mixed_policy_requires_one_explicit_disposition_per_zone

```rust
    fn mixed_policy_requires_one_explicit_disposition_per_zone() {
        let directory = std::env::temp_dir().join(format!(
            "rgbdns-dnssec-policy-{}-{}",
            std::process::id(),
            std::thread::current().name().unwrap_or("test")
        ));
        fs::create_dir_all(&directory).unwrap();
        let policy_file = directory.join("dnssec");
        fs::write(
            &policy_file,
            "Ksigned.example.:/key:13:1209600:86400:3600\nUlegacy.example.\n",
        )
        .unwrap();
        let policies = read_policies(&policy_file).unwrap();
        assert_eq!(policies.signing.len(), 1);
        assert!(
            policies
                .unsigned
                .contains(&"legacy.example".parse().unwrap())
        );

        fs::write(
            &policy_file,
            "Ksigned.example.:/key:13:1209600:86400:3600\nUsigned.example.\n",
        )
        .unwrap();
        assert!(read_policies(&policy_file).is_err());
        fs::remove_dir_all(directory).unwrap();
    }

    #[test]
```
