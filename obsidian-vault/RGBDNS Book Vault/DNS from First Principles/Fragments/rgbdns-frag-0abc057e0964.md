---
type: "code-fragment"
fragment_id: "rgbdns-frag-0abc057e0964"
source_path: "src/packet.rs"
code_note: "DNS from First Principles/Code/src/packet.rs.source"
language: "rust"
subsystem: "DNS data model and wire codec"
crate: "rgbdns"
symbol: "record"
kind: "fn"
start_line: 276
end_line: 356
---

# record

- Fragment ID: `rgbdns-frag-0abc057e0964`
- Source file: [[DNS from First Principles/Code/src/packet.rs.source|src/packet.rs]]
- Lines: 276-356
- Subsystem: [[DNS from First Principles/Subsystems/DNS data model and wire codec|DNS data model and wire codec]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-0abc057e0964", "codeNote": "DNS from First Principles/Code/src/packet.rs.source", "heading": "rgbdns-frag-0abc057e0964: fn record", "sourcePath": "src/packet.rs", "startLine": 276, "endLine": 356}
```

## Excerpt

<span id="rgbdns-frag-0abc057e0964" class="rgbdns-fragment-target"></span>
### rgbdns-frag-0abc057e0964: fn record

```rust
    fn record(&mut self) -> Result<Record> {
        let name = self.name()?;
        let typ = RecordType::from_code(self.u16()?);
        let class = self.u16()?;
        if typ != RecordType::Opt && class != 1 {
            return Err(Error::Format("non-IN record"));
        }
        let ttl = self.u32()?;
        let len = self.u16()? as usize;
        let end = self
            .p
            .checked_add(len)
            .filter(|e| *e <= self.b.len())
            .ok_or(Error::Format("truncated rdata"))?;
        let data = match typ {
            RecordType::A if len == 4 => RData::A(Ipv4Addr::new(
                self.u8()?,
                self.u8()?,
                self.u8()?,
                self.u8()?,
            )),
            RecordType::Aaaa if len == 16 => {
                let mut x = [0; 16];
                x.copy_from_slice(&self.b[self.p..end]);
                self.p = end;
                RData::Aaaa(x.into())
            }
            RecordType::Ns | RecordType::Cname | RecordType::Ptr => RData::Name(typ, self.name()?),
            RecordType::Mx => {
                let p = self.u16()?;
                RData::Mx(p, self.name()?)
            }
            RecordType::Srv => RData::Srv {
                priority: self.u16()?,
                weight: self.u16()?,
                port: self.u16()?,
                target: self.name()?,
            },
            RecordType::Soa => RData::Soa {
                mname: self.name()?,
                admin: self.name()?,
                serial: self.u32()?,
                refresh: self.u32()?,
                retry: self.u32()?,
                expire: self.u32()?,
                minimum: self.u32()?,
            },
            RecordType::Caa if len >= 2 => {
                let flags = self.u8()?;
                let tag_len = self.u8()? as usize;
                if self.p + tag_len > end {
                    return Err(Error::Format("bad CAA tag length"));
                }
                let tag = self.b[self.p..self.p + tag_len].to_vec();
                self.p += tag_len;
                let value = self.b[self.p..end].to_vec();
                self.p = end;
                RData::Caa { flags, tag, value }
            }
            RecordType::Opt => {
                if !name.is_root() {
                    return Err(Error::Format("OPT owner is not root"));
                }
                let options = self.b[self.p..end].to_vec();
                validate_edns_options(&options)?;
                self.p = end;
                RData::Opt {
                    udp_payload: class,
                    extended_rcode: (ttl >> 24) as u8,
                    version: (ttl >> 16) as u8,
                    flags: ttl as u16,
                    options,
                }
            }
            RecordType::Txt => {
                let mut v = Vec::new();
                while self.p < end {
                    let n = self.u8()? as usize;
                    if self.p + n > end {
                        return Err(Error::Format("bad TXT"));
                    }
```
