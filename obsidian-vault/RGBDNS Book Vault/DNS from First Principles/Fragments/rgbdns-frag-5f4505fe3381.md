---
type: "code-fragment"
fragment_id: "rgbdns-frag-5f4505fe3381"
source_path: "src/packet.rs"
code_note: "DNS from First Principles/Code/src/packet.rs.source"
language: "rust"
subsystem: "DNS data model and wire codec"
crate: "rgbdns"
symbol: "from_str"
kind: "fn"
start_line: 76
end_line: 117
---

# from_str

- Fragment ID: `rgbdns-frag-5f4505fe3381`
- Source file: [[DNS from First Principles/Code/src/packet.rs.source|src/packet.rs]]
- Lines: 76-117
- Subsystem: [[DNS from First Principles/Subsystems/DNS data model and wire codec|DNS data model and wire codec]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-5f4505fe3381", "codeNote": "DNS from First Principles/Code/src/packet.rs.source", "heading": "rgbdns-frag-5f4505fe3381: fn from_str", "sourcePath": "src/packet.rs", "startLine": 76, "endLine": 117}
```

## Excerpt

<span id="rgbdns-frag-5f4505fe3381" class="rgbdns-fragment-target"></span>
### rgbdns-frag-5f4505fe3381: fn from_str

```rust
    fn from_str(s: &str) -> Result<Self> {
        Ok(match s.to_ascii_uppercase().as_str() {
            "A" => Self::A,
            "NS" => Self::Ns,
            "CNAME" => Self::Cname,
            "SOA" => Self::Soa,
            "PTR" => Self::Ptr,
            "MX" => Self::Mx,
            "TXT" => Self::Txt,
            "AAAA" => Self::Aaaa,
            "SRV" => Self::Srv,
            "OPT" => Self::Opt,
            "CAA" => Self::Caa,
            "DS" => Self::Ds,
            "RRSIG" => Self::Rrsig,
            "NSEC" => Self::Nsec,
            "DNSKEY" => Self::Dnskey,
            "AXFR" => Self::Axfr,
            "ANY" => Self::Any,
            "HINFO" => Self::Unknown(13),
            "RP" => Self::Unknown(17),
            "SIG" => Self::Unknown(24),
            "KEY" => Self::Unknown(25),
            "NAPTR" => Self::Unknown(35),
            "NSEC3" => Self::Unknown(50),
            "NSEC3PARAM" => Self::Unknown(51),
            "TLSA" => Self::Unknown(52),
            "SVCB" => Self::Unknown(64),
            "HTTPS" => Self::Unknown(65),
            x => {
                let number = x.strip_prefix("TYPE").unwrap_or(x);
                Self::Unknown(
                    number
                        .parse()
                        .map_err(|_| Error::Format("unknown record type"))?,
                )
            }
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
```
