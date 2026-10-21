---
type: "code-fragment"
fragment_id: "rgbdns-frag-d2822b688108"
source_path: "src/dnssec.rs"
code_note: "DNS from First Principles/Code/src/dnssec.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "parse"
kind: "fn"
start_line: 185
end_line: 234
---

# parse

- Fragment ID: `rgbdns-frag-d2822b688108`
- Source file: [[DNS from First Principles/Code/src/dnssec.rs.source|src/dnssec.rs]]
- Lines: 185-234
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-d2822b688108", "codeNote": "DNS from First Principles/Code/src/dnssec.rs.source", "sourcePath": "src/dnssec.rs", "startLine": 185, "endLine": 234}
```

## Excerpt

<span id="rgbdns-frag-d2822b688108" class="rgbdns-fragment-target"></span>
### rgbdns-frag-d2822b688108: fn parse

```rust
    pub fn parse(line: &str) -> Result<Self> {
        let fields = line
            .strip_prefix('K')
            .ok_or_else(|| Error::InvalidRecord("DNSSEC policy must start with K".into()))?
            .split(':')
            .collect::<Vec<_>>();
        if fields.len() != 6 || fields.iter().any(|field| field.is_empty()) {
            return Err(Error::InvalidRecord(
                "DNSSEC policy is Kzone:keyfile:algorithm:validity:refresh:inception-skew".into(),
            ));
        }
        if fields[2] != ALGORITHM.to_string() {
            return Err(Error::InvalidRecord(
                "only DNSSEC algorithm 13 is supported".into(),
            ));
        }
        let duration = |field: &str, label: &str| -> Result<u64> {
            let value = field
                .parse::<u64>()
                .map_err(|_| Error::InvalidRecord(format!("invalid DNSSEC {label}")))?;
            if !(300..=31_536_000).contains(&value) {
                return Err(Error::InvalidRecord(format!(
                    "DNSSEC {label} must be between 300 and 31536000 seconds"
                )));
            }
            Ok(value)
        };
        let validity = duration(fields[3], "validity")?;
        let refresh = duration(fields[4], "refresh")?;
        let inception_skew = duration(fields[5], "inception skew")?;
        if refresh >= validity || inception_skew >= validity {
            return Err(Error::InvalidRecord(
                "DNSSEC refresh and inception skew must be shorter than validity".into(),
            ));
        }
        let key_file = PathBuf::from(fields[1]);
        if !key_file.is_absolute() {
            return Err(Error::InvalidRecord(
                "DNSSEC key path must be absolute".into(),
            ));
        }
        Ok(Self {
            zone: fields[0].parse()?,
            key_file,
            validity,
            refresh,
            inception_skew,
        })
    }

```
