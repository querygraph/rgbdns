---
type: "code-fragment"
fragment_id: "rgbdns-frag-6c046f5c35d8"
source_path: "src/aname.rs"
code_note: "DNS from First Principles/Code/src/aname.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "concurrent_misses_are_coalesced"
kind: "fn"
start_line: 290
end_line: 330
---

# concurrent_misses_are_coalesced

- Fragment ID: `rgbdns-frag-6c046f5c35d8`
- Source file: [[DNS from First Principles/Code/src/aname.rs.source|src/aname.rs]]
- Lines: 290-330
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-6c046f5c35d8", "codeNote": "DNS from First Principles/Code/src/aname.rs.source", "sourcePath": "src/aname.rs", "startLine": 290, "endLine": 330}
```

## Excerpt

<span id="rgbdns-frag-6c046f5c35d8" class="rgbdns-fragment-target"></span>
### rgbdns-frag-6c046f5c35d8: fn concurrent_misses_are_coalesced

```rust
    fn concurrent_misses_are_coalesced() {
        let resolver = Arc::new(Resolver::new(Vec::new()));
        let owner: Name = "owner.example".parse().unwrap();
        let target: Name = "target.example".parse().unwrap();
        let calls = Arc::new(AtomicUsize::new(0));
        let barrier = Arc::new(Barrier::new(8));
        let workers = (0..8)
            .map(|_| {
                let resolver = resolver.clone();
                let owner = owner.clone();
                let target = target.clone();
                let calls = calls.clone();
                let barrier = barrier.clone();
                thread::spawn(move || {
                    barrier.wait();
                    resolver
                        .resolve_with(&owner, &target, RecordType::A, 300, |name, _| {
                            calls.fetch_add(1, Ordering::SeqCst);
                            thread::sleep(Duration::from_millis(50));
                            Ok(Message {
                                flags: 0x8000,
                                answers: vec![Record {
                                    name,
                                    ttl: 60,
                                    data: RData::A(Ipv4Addr::new(192, 0, 2, 8)),
                                }],
                                ..Default::default()
                            })
                        })
                        .unwrap()
                })
            })
            .collect::<Vec<_>>();
        for worker in workers {
            let records = worker.join().unwrap();
            assert_eq!(records[0].data, RData::A(Ipv4Addr::new(192, 0, 2, 8)));
        }
        assert_eq!(calls.load(Ordering::SeqCst), 1);
    }

    #[test]
```
