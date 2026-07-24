---
type: "code-fragment"
fragment_id: "rgbdns-frag-f26143011b10"
source_path: "src/packet.rs"
code_note: "DNS from First Principles/Code/src/packet.rs.source"
language: "rust"
subsystem: "DNS data model and wire codec"
crate: "rgbdns"
symbol: "name"
kind: "fn"
start_line: 223
end_line: 275
---

# name

- Fragment ID: `rgbdns-frag-f26143011b10`
- Source file: [[DNS from First Principles/Code/src/packet.rs.source|src/packet.rs]]
- Lines: 223-275
- Subsystem: [[DNS from First Principles/Subsystems/DNS data model and wire codec|DNS data model and wire codec]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-f26143011b10", "codeNote": "DNS from First Principles/Code/src/packet.rs.source", "heading": "rgbdns-frag-f26143011b10: fn name", "sourcePath": "src/packet.rs", "startLine": 223, "endLine": 275}
```

## Excerpt

<span id="rgbdns-frag-f26143011b10" class="rgbdns-fragment-target"></span>
### rgbdns-frag-f26143011b10: fn name

```rust
    fn name(&mut self) -> Result<Name> {
        let mut labels = Vec::new();
        let mut pos = self.p;
        let mut jumped = false;
        let mut hops = 0;
        loop {
            if hops > 128 {
                return Err(Error::Format("compression pointer loop"));
            }
            hops += 1;
            let n = *self.b.get(pos).ok_or(Error::Format("truncated name"))?;
            if n & 0xc0 == 0xc0 {
                let b = *self
                    .b
                    .get(pos + 1)
                    .ok_or(Error::Format("truncated pointer"))?;
                let q = (((n & 0x3f) as usize) << 8) | b as usize;
                if q >= pos {
                    return Err(Error::Format("compression pointer is not backward"));
                }
                if !self.name_offsets.get(q).is_some_and(|valid| *valid) {
                    return Err(Error::Format(
                        "compression pointer does not target a prior name",
                    ));
                }
                self.name_offsets[pos] = true;
                if !jumped {
                    self.p = pos + 2;
                    jumped = true;
                }
                pos = q;
                continue;
            }
            if n & 0xc0 != 0 {
                return Err(Error::Format("reserved label type"));
            }
            self.name_offsets[pos] = true;
            pos += 1;
            if n == 0 {
                if !jumped {
                    self.p = pos
                }
                break;
            }
            let end = pos + n as usize;
            if end > self.b.len() {
                return Err(Error::Format("truncated label"));
            }
            labels.push(self.b[pos..end].to_vec());
            pos = end;
        }
        Name::from_labels(labels)
    }
```
