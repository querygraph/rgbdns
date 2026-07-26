---
type: "code-fragment"
fragment_id: "rgbdns-frag-bf06479bb119"
source_path: "src/cdb.rs"
code_note: "DNS from First Principles/Code/src/cdb.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "encode_rdata"
kind: "fn"
start_line: 211
end_line: 269
---

# encode_rdata

- Fragment ID: `rgbdns-frag-bf06479bb119`
- Source file: [[DNS from First Principles/Code/src/cdb.rs.source|src/cdb.rs]]
- Lines: 211-269
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-bf06479bb119", "codeNote": "DNS from First Principles/Code/src/cdb.rs.source", "heading": "rgbdns-frag-bf06479bb119: fn encode_rdata", "sourcePath": "src/cdb.rs", "startLine": 211, "endLine": 269}
```

## Excerpt

<span id="rgbdns-frag-bf06479bb119" class="rgbdns-fragment-target"></span>
### rgbdns-frag-bf06479bb119: fn encode_rdata

```rust
fn encode_rdata(data: &RData, out: &mut Vec<u8>) -> Result<()> {
    match data {
        RData::A(address) => out.extend(address.octets()),
        RData::Aaaa(address) => out.extend(address.octets()),
        RData::Name(_, name) => out.extend(name.to_wire()),
        RData::Mx(preference, name) => {
            out.extend(preference.to_be_bytes());
            out.extend(name.to_wire());
        }
        RData::Soa {
            mname,
            admin,
            serial,
            refresh,
            retry,
            expire,
            minimum,
        } => {
            out.extend(mname.to_wire());
            out.extend(admin.to_wire());
            for value in [serial, refresh, retry, expire, minimum] {
                out.extend(value.to_be_bytes());
            }
        }
        RData::Txt(chunks) => {
            for chunk in chunks {
                for part in chunk.chunks(127) {
                    out.push(part.len() as u8);
                    out.extend(part);
                }
            }
        }
        RData::Srv {
            priority,
            weight,
            port,
            target,
        } => {
            out.extend(priority.to_be_bytes());
            out.extend(weight.to_be_bytes());
            out.extend(port.to_be_bytes());
            out.extend(target.to_wire());
        }
        RData::Caa { flags, tag, value } => {
            out.push(*flags);
            out.push(
                tag.len()
                    .try_into()
                    .map_err(|_| Error::Format("CAA tag is too long"))?,
            );
            out.extend(tag);
            out.extend(value);
        }
        RData::Opaque(_, bytes) => out.extend(bytes),
        RData::Opt { .. } => return Err(Error::Format("OPT cannot be stored in tinydns CDB")),
    }
    Ok(())
}

```
