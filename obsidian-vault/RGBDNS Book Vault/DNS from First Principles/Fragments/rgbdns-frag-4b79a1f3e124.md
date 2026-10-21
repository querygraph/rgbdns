---
type: "code-fragment"
fragment_id: "rgbdns-frag-4b79a1f3e124"
source_path: "src/aname.rs"
code_note: "DNS from First Principles/Code/src/aname.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "failures_are_suppressed_briefly"
kind: "fn"
start_line: 331
end_line: 348
---

# failures_are_suppressed_briefly

- Fragment ID: `rgbdns-frag-4b79a1f3e124`
- Source file: [[DNS from First Principles/Code/src/aname.rs.source|src/aname.rs]]
- Lines: 331-348
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-4b79a1f3e124", "codeNote": "DNS from First Principles/Code/src/aname.rs.source", "sourcePath": "src/aname.rs", "startLine": 331, "endLine": 348}
```

## Excerpt

<span id="rgbdns-frag-4b79a1f3e124" class="rgbdns-fragment-target"></span>
### rgbdns-frag-4b79a1f3e124: fn failures_are_suppressed_briefly

```rust
    fn failures_are_suppressed_briefly() {
        let resolver = Resolver::new(Vec::new());
        let owner: Name = "owner.example".parse().unwrap();
        let target: Name = "target.example".parse().unwrap();
        let calls = AtomicUsize::new(0);
        let first = resolver.resolve_with(&owner, &target, RecordType::A, 300, |_, _| {
            calls.fetch_add(1, Ordering::SeqCst);
            Err(Error::Format("simulated upstream failure"))
        });
        assert!(first.is_err());
        let second = resolver.resolve_with(&owner, &target, RecordType::A, 300, |_, _| {
            calls.fetch_add(1, Ordering::SeqCst);
            Ok(Message::default())
        });
        assert!(second.is_err());
        assert_eq!(calls.load(Ordering::SeqCst), 1);
    }
}
```
