---
type: "code-fragment"
fragment_id: "rgbdns-frag-17d606c73a93"
source_path: "src/packet.rs"
code_note: "DNS from First Principles/Code/src/packet.rs.source"
language: "rust"
subsystem: "DNS data model and wire codec"
crate: "rgbdns"
symbol: "dnssec"
kind: "fn"
start_line: 219
end_line: 286
---

# dnssec

- Fragment ID: `rgbdns-frag-17d606c73a93`
- Source file: [[DNS from First Principles/Code/src/packet.rs.source|src/packet.rs]]
- Lines: 219-286
- Subsystem: [[DNS from First Principles/Subsystems/DNS data model and wire codec|DNS data model and wire codec]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-17d606c73a93", "codeNote": "DNS from First Principles/Code/src/packet.rs.source", "sourcePath": "src/packet.rs", "startLine": 219, "endLine": 286}
```

## Excerpt

<span id="rgbdns-frag-17d606c73a93" class="rgbdns-fragment-target"></span>
### rgbdns-frag-17d606c73a93: fn dnssec

```rust
    pub fn dnssec(&self) -> Result<Option<DnssecRData>> {
        let Self::Opaque(record_type, bytes) = self else {
            return Ok(None);
        };
        Ok(Some(match record_type {
            RecordType::Ds => {
                if bytes.len() < 5 {
                    return Err(Error::Format("short DS RDATA"));
                }
                DnssecRData::Ds {
                    key_tag: u16::from_be_bytes([bytes[0], bytes[1]]),
                    algorithm: bytes[2],
                    digest_type: bytes[3],
                    digest: bytes[4..].to_vec(),
                }
            }
            RecordType::Dnskey => {
                if bytes.len() < 5 {
                    return Err(Error::Format("short DNSKEY RDATA"));
                }
                if bytes[2] != 3 {
                    return Err(Error::Format("DNSKEY protocol is not 3"));
                }
                DnssecRData::Dnskey {
                    flags: u16::from_be_bytes([bytes[0], bytes[1]]),
                    protocol: bytes[2],
                    algorithm: bytes[3],
                    public_key: bytes[4..].to_vec(),
                }
            }
            RecordType::Rrsig => {
                if bytes.len() < 19 {
                    return Err(Error::Format("short RRSIG RDATA"));
                }
                let (signer, consumed) = uncompressed_name(&bytes[18..])?;
                let signature = bytes
                    .get(18 + consumed..)
                    .filter(|signature| !signature.is_empty())
                    .ok_or(Error::Format("RRSIG has no signature"))?
                    .to_vec();
                DnssecRData::Rrsig {
                    type_covered: RecordType::from_code(u16::from_be_bytes([bytes[0], bytes[1]])),
                    algorithm: bytes[2],
                    labels: bytes[3],
                    original_ttl: u32::from_be_bytes(bytes[4..8].try_into().unwrap()),
                    expiration: u32::from_be_bytes(bytes[8..12].try_into().unwrap()),
                    inception: u32::from_be_bytes(bytes[12..16].try_into().unwrap()),
                    key_tag: u16::from_be_bytes([bytes[16], bytes[17]]),
                    signer,
                    signature,
                }
            }
            RecordType::Nsec => {
                let (next, consumed) = uncompressed_name(bytes)?;
                let bitmap = bytes
                    .get(consumed..)
                    .filter(|bitmap| !bitmap.is_empty())
                    .ok_or(Error::Format("NSEC has no type bitmap"))?;
                DnssecRData::Nsec {
                    next,
                    types: type_bitmap(bitmap)?,
                }
            }
            _ => return Ok(None),
        }))
    }
}

```
