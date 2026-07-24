---
type: "code-fragment"
fragment_id: "rgbdns-frag-554c269fd7f5"
source_path: "src/rbl.rs"
code_note: "DNS from First Principles/Code/src/rbl.rs.source"
language: "rust"
subsystem: "Specialized responders"
crate: "rgbdns"
symbol: "response"
kind: "fn"
start_line: 94
end_line: 100
---

# response

- Fragment ID: `rgbdns-frag-554c269fd7f5`
- Source file: [[DNS from First Principles/Code/src/rbl.rs.source|src/rbl.rs]]
- Lines: 94-100
- Subsystem: [[DNS from First Principles/Subsystems/Specialized responders|Specialized responders]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-554c269fd7f5", "codeNote": "DNS from First Principles/Code/src/rbl.rs.source", "heading": "rgbdns-frag-554c269fd7f5: fn response", "sourcePath": "src/rbl.rs", "startLine": 94, "endLine": 100}
```

## Excerpt

<span id="rgbdns-frag-554c269fd7f5" class="rgbdns-fragment-target"></span>
### rgbdns-frag-554c269fd7f5: fn response

```rust
    fn response(&self) -> (Ipv4Addr, &[u8]) {
        self.responses
            .first()
            .map(|(address, text)| (*address, text.as_slice()))
            .unwrap_or((Ipv4Addr::new(127, 0, 0, 2), b"Listed $"))
    }

```
