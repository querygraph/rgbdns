---
type: "code-fragment"
fragment_id: "rgbdns-frag-5767978f27f4"
source_path: "tests/support/mod.rs"
code_note: "DNS from First Principles/Code/tests/support/mod.rs.source"
language: "rust"
subsystem: "Tests and performance"
symbol: "opt"
kind: "fn"
start_line: 32
end_line: 45
---

# opt

- Fragment ID: `rgbdns-frag-5767978f27f4`
- Source file: [[DNS from First Principles/Code/tests/support/mod.rs.source|tests/support/mod.rs]]
- Lines: 32-45
- Subsystem: [[DNS from First Principles/Subsystems/Tests and performance|Tests and performance]]

```rgbdns-fragment
{"id": "rgbdns-frag-5767978f27f4", "codeNote": "DNS from First Principles/Code/tests/support/mod.rs.source", "heading": "rgbdns-frag-5767978f27f4: fn opt", "sourcePath": "tests/support/mod.rs", "startLine": 32, "endLine": 45}
```

## Excerpt

<span id="rgbdns-frag-5767978f27f4" class="rgbdns-fragment-target"></span>
### rgbdns-frag-5767978f27f4: fn opt

```rust
pub fn opt(payload: u16, version: u8, flags: u16, options: Vec<u8>) -> Record {
    Record {
        name: Name::root(),
        ttl: 0,
        data: RData::Opt {
            udp_payload: payload,
            extended_rcode: 0,
            version,
            flags,
            options,
        },
    }
}

```
