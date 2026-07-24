---
type: "code-fragment"
fragment_id: "rgbdns-frag-264ff394342c"
source_path: "src/packet.rs"
code_note: "DNS from First Principles/Code/src/packet.rs.source"
language: "rust"
subsystem: "DNS data model and wire codec"
crate: "rgbdns"
symbol: "decode"
kind: "fn"
start_line: 395
end_line: 439
---

# decode

- Fragment ID: `rgbdns-frag-264ff394342c`
- Source file: [[DNS from First Principles/Code/src/packet.rs.source|src/packet.rs]]
- Lines: 395-439
- Subsystem: [[DNS from First Principles/Subsystems/DNS data model and wire codec|DNS data model and wire codec]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-264ff394342c", "codeNote": "DNS from First Principles/Code/src/packet.rs.source", "heading": "rgbdns-frag-264ff394342c: fn decode", "sourcePath": "src/packet.rs", "startLine": 395, "endLine": 439}
```

## Excerpt

<span id="rgbdns-frag-264ff394342c" class="rgbdns-fragment-target"></span>
### rgbdns-frag-264ff394342c: fn decode

```rust
    pub fn decode(b: &[u8]) -> Result<Self> {
        if b.len() < 12 {
            return Err(Error::Format("short header"));
        }
        let mut r = Reader {
            b,
            p: 0,
            name_offsets: vec![false; b.len()],
        };
        let id = r.u16()?;
        let flags = r.u16()?;
        let qd = r.u16()?;
        let an = r.u16()?;
        let ns = r.u16()?;
        let ar = r.u16()?;
        if qd > 64 || an > 4096 || ns > 4096 || ar > 4096 {
            return Err(Error::Format("excessive section count"));
        }
        let mut m = Self {
            id,
            flags,
            ..Self::default()
        };
        for _ in 0..qd {
            let name = r.name()?;
            m.questions.push(Question {
                name,
                qtype: RecordType::from_code(r.u16()?),
                qclass: r.u16()?,
            })
        }
        for _ in 0..an {
            m.answers.push(r.record()?)
        }
        for _ in 0..ns {
            m.authorities.push(r.record()?)
        }
        for _ in 0..ar {
            m.additionals.push(r.record()?)
        }
        if r.p != b.len() {
            return Err(Error::Format("trailing DNS packet data"));
        }
        Ok(m)
    }
```
