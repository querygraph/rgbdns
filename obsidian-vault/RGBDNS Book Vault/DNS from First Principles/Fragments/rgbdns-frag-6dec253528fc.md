---
type: "code-fragment"
fragment_id: "rgbdns-frag-6dec253528fc"
source_path: "src/aname.rs"
code_note: "DNS from First Principles/Code/src/aname.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "resolve"
kind: "fn"
start_line: 49
end_line: 64
---

# resolve

- Fragment ID: `rgbdns-frag-6dec253528fc`
- Source file: [[DNS from First Principles/Code/src/aname.rs.source|src/aname.rs]]
- Lines: 49-64
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-6dec253528fc", "codeNote": "DNS from First Principles/Code/src/aname.rs.source", "sourcePath": "src/aname.rs", "startLine": 49, "endLine": 64}
```

## Excerpt

<span id="rgbdns-frag-6dec253528fc" class="rgbdns-fragment-target"></span>
### rgbdns-frag-6dec253528fc: fn resolve

```rust
    pub(crate) fn resolve(
        &self,
        owner: &Name,
        target: &Name,
        record_type: RecordType,
        ttl_limit: u32,
    ) -> Result<Vec<Record>> {
        self.resolve_with(
            owner,
            target,
            record_type,
            ttl_limit,
            |target, record_type| client::query(target, record_type, true, &self.servers),
        )
    }

```
