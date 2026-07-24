---
type: "code-fragment"
fragment_id: "rgbdns-frag-3013c3b46f39"
source_path: "src/packet.rs"
code_note: "DNS from First Principles/Code/src/packet.rs.source"
language: "rust"
subsystem: "DNS data model and wire codec"
crate: "rgbdns"
symbol: "record"
kind: "fn"
start_line: 517
end_line: 597
---

# record

- Fragment ID: `rgbdns-frag-3013c3b46f39`
- Source file: [[DNS from First Principles/Code/src/packet.rs.source|src/packet.rs]]
- Lines: 517-597
- Subsystem: [[DNS from First Principles/Subsystems/DNS data model and wire codec|DNS data model and wire codec]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-3013c3b46f39", "codeNote": "DNS from First Principles/Code/src/packet.rs.source", "heading": "rgbdns-frag-3013c3b46f39: fn record", "sourcePath": "src/packet.rs", "startLine": 517, "endLine": 597}
```

## Excerpt

<span id="rgbdns-frag-3013c3b46f39" class="rgbdns-fragment-target"></span>
### rgbdns-frag-3013c3b46f39: fn record

```rust
    fn record(&mut self, r: &Record) -> Result<()> {
        self.name(&r.name)?;
        self.u16(r.rr_type().code());
        match &r.data {
            RData::Opt {
                udp_payload,
                extended_rcode,
                version,
                flags,
                ..
            } => {
                self.u16(*udp_payload);
                self.u32(
                    u32::from(*extended_rcode) << 24
                        | u32::from(*version) << 16
                        | u32::from(*flags),
                );
            }
            _ => {
                self.u16(1);
                self.u32(r.ttl);
            }
        }
        let at = self.0.len();
        self.u16(0);
        let start = self.0.len();
        match &r.data {
            RData::A(x) => self.0.extend(x.octets()),
            RData::Aaaa(x) => self.0.extend(x.octets()),
            RData::Name(_, n) => self.name(n)?,
            RData::Mx(p, n) => {
                self.u16(*p);
                self.name(n)?
            }
            RData::Txt(v) => {
                for s in v {
                    if s.len() > 255 {
                        return Err(Error::Format("TXT chunk too long"));
                    }
                    self.u8(s.len() as u8);
                    self.0.extend(s)
                }
            }
            RData::Soa {
                mname: n,
                admin,
                serial,
                refresh,
                retry,
                expire,
                minimum,
            } => {
                self.name(n)?;
                self.name(admin)?;
                for x in [serial, refresh, retry, expire, minimum] {
                    self.u32(*x)
                }
            }
            RData::Srv {
                priority,
                weight,
                port,
                target,
            } => {
                self.u16(*priority);
                self.u16(*weight);
                self.u16(*port);
                self.name(target)?
            }
            RData::Caa { flags, tag, value } => {
                self.u8(*flags);
                self.u8(tag
                    .len()
                    .try_into()
                    .map_err(|_| Error::Format("CAA tag too long"))?);
                self.0.extend(tag);
                self.0.extend(value)
            }
            RData::Opt { options, .. } => {
                validate_edns_options(options)?;
                self.0.extend(options)
```
