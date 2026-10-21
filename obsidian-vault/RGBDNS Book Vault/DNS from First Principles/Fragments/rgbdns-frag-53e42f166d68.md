---
type: "code-fragment"
fragment_id: "rgbdns-frag-53e42f166d68"
source_path: "src/acme.rs"
code_note: "DNS from First Principles/Code/src/acme.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "record"
kind: "fn"
start_line: 979
end_line: 995
---

# record

- Fragment ID: `rgbdns-frag-53e42f166d68`
- Source file: [[DNS from First Principles/Code/src/acme.rs.source|src/acme.rs]]
- Lines: 979-995
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-53e42f166d68", "codeNote": "DNS from First Principles/Code/src/acme.rs.source", "sourcePath": "src/acme.rs", "startLine": 979, "endLine": 995}
```

## Excerpt

<span id="rgbdns-frag-53e42f166d68" class="rgbdns-fragment-target"></span>
### rgbdns-frag-53e42f166d68: fn record

```rust
    fn record(&mut self) -> Result<WireRecord> {
        let name = self.name()?;
        let typ = self.u16()?;
        let class = self.u16()?;
        let ttl = self.u32()?;
        let length = usize::from(self.u16()?);
        let rdata = self.bytes(length)?.to_vec();
        Ok(WireRecord {
            name,
            typ,
            class,
            ttl,
            rdata,
        })
    }
}

```
