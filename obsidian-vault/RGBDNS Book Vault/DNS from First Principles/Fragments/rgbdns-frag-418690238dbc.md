---
type: "code-fragment"
fragment_id: "rgbdns-frag-418690238dbc"
source_path: "src/packet.rs"
code_note: "DNS from First Principles/Code/src/packet.rs.source"
language: "rust"
subsystem: "DNS data model and wire codec"
crate: "rgbdns"
symbol: "rejects_pointer_loop"
kind: "fn"
start_line: 627
end_line: 633
---

# rejects_pointer_loop

- Fragment ID: `rgbdns-frag-418690238dbc`
- Source file: [[DNS from First Principles/Code/src/packet.rs.source|src/packet.rs]]
- Lines: 627-633
- Subsystem: [[DNS from First Principles/Subsystems/DNS data model and wire codec|DNS data model and wire codec]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-418690238dbc", "codeNote": "DNS from First Principles/Code/src/packet.rs.source", "heading": "rgbdns-frag-418690238dbc: fn rejects_pointer_loop", "sourcePath": "src/packet.rs", "startLine": 627, "endLine": 633}
```

## Excerpt

<span id="rgbdns-frag-418690238dbc" class="rgbdns-fragment-target"></span>
### rgbdns-frag-418690238dbc: fn rejects_pointer_loop

```rust
    fn rejects_pointer_loop() {
        let mut b = vec![0; 12];
        b[5] = 1;
        b.extend([0xc0, 0x0c, 0, 1, 0, 1]);
        assert!(Message::decode(&b).is_err())
    }
    #[test]
```
