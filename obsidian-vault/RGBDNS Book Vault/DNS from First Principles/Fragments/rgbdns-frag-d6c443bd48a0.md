---
type: "code-fragment"
fragment_id: "rgbdns-frag-d6c443bd48a0"
source_path: "src/packet.rs"
code_note: "DNS from First Principles/Code/src/packet.rs.source"
language: "rust"
subsystem: "DNS data model and wire codec"
crate: "rgbdns"
symbol: "name"
kind: "fn"
start_line: 476
end_line: 516
---

# name

- Fragment ID: `rgbdns-frag-d6c443bd48a0`
- Source file: [[DNS from First Principles/Code/src/packet.rs.source|src/packet.rs]]
- Lines: 476-516
- Subsystem: [[DNS from First Principles/Subsystems/DNS data model and wire codec|DNS data model and wire codec]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-d6c443bd48a0", "codeNote": "DNS from First Principles/Code/src/packet.rs.source", "heading": "rgbdns-frag-d6c443bd48a0: fn name", "sourcePath": "src/packet.rs", "startLine": 476, "endLine": 516}
```

## Excerpt

<span id="rgbdns-frag-d6c443bd48a0" class="rgbdns-fragment-target"></span>
### rgbdns-frag-d6c443bd48a0: fn name

```rust
    fn name(&mut self, n: &Name) -> Result<()> {
        if n.wire_len() > 255 {
            return Err(Error::Format("name too long"));
        }
        if let Some((last, offset)) = &self.2
            && last == n
        {
            self.u16(0xc000 | *offset);
            return Ok(());
        }
        // Owner names commonly repeat across an RRset. Avoid rebuilding every
        // possible suffix when the complete name already has a pointer.
        if let Some(offset) = self.1.get(n).copied() {
            self.2 = Some((n.clone(), offset));
            self.u16(0xc000 | offset);
            return Ok(());
        }
        let start = u16::try_from(self.0.len())
            .ok()
            .filter(|offset| *offset < 0x4000);
        let labels = n.labels().collect::<Vec<_>>();
        for (index, label) in labels.iter().enumerate() {
            let suffix = n.suffix(index);
            if let Some(offset) = self.1.get(&suffix).copied() {
                self.u16(0xc000 | offset);
                return Ok(());
            }
            if let Ok(offset) = u16::try_from(self.0.len())
                && offset < 0x4000
            {
                self.1.entry(suffix).or_insert(offset);
            }
            self.u8(label.len() as u8);
            self.0.extend(*label)
        }
        self.u8(0);
        if let Some(offset) = start {
            self.2 = Some((n.clone(), offset));
        }
        Ok(())
    }
```
