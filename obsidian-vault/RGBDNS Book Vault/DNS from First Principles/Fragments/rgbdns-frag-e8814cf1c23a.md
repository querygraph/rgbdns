---
type: "code-fragment"
fragment_id: "rgbdns-frag-e8814cf1c23a"
source_path: "src/packet.rs"
code_note: "DNS from First Principles/Code/src/packet.rs.source"
language: "rust"
subsystem: "DNS data model and wire codec"
crate: "rgbdns"
symbol: "encode"
kind: "fn"
start_line: 596
end_line: 620
---

# encode

- Fragment ID: `rgbdns-frag-e8814cf1c23a`
- Source file: [[DNS from First Principles/Code/src/packet.rs.source|src/packet.rs]]
- Lines: 596-620
- Subsystem: [[DNS from First Principles/Subsystems/DNS data model and wire codec|DNS data model and wire codec]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-e8814cf1c23a", "codeNote": "DNS from First Principles/Code/src/packet.rs.source", "sourcePath": "src/packet.rs", "startLine": 596, "endLine": 620}
```

## Excerpt

<span id="rgbdns-frag-e8814cf1c23a" class="rgbdns-fragment-target"></span>
### rgbdns-frag-e8814cf1c23a: fn encode

```rust
    pub fn encode(&self) -> Result<Vec<u8>> {
        let mut w = Writer(Vec::with_capacity(512), HashMap::new(), None);
        w.u16(self.id);
        w.u16(self.flags);
        for n in [
            self.questions.len(),
            self.answers.len(),
            self.authorities.len(),
            self.additionals.len(),
        ] {
            w.u16(u16::try_from(n).map_err(|_| Error::Format("section too large"))?)
        }
        for q in &self.questions {
            w.name(&q.name)?;
            w.u16(q.qtype.code());
            w.u16(q.qclass)
        }
        for section in [&self.answers, &self.authorities, &self.additionals] {
            for r in section {
                w.record(r)?
            }
        }
        Ok(w.0)
    }
}
```
