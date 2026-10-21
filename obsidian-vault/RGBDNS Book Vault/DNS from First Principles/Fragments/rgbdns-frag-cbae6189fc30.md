---
type: "code-fragment"
fragment_id: "rgbdns-frag-cbae6189fc30"
source_path: "src/dnssec.rs"
code_note: "DNS from First Principles/Code/src/dnssec.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "validate_zone_coverage"
kind: "fn"
start_line: 144
end_line: 172
---

# validate_zone_coverage

- Fragment ID: `rgbdns-frag-cbae6189fc30`
- Source file: [[DNS from First Principles/Code/src/dnssec.rs.source|src/dnssec.rs]]
- Lines: 144-172
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-cbae6189fc30", "codeNote": "DNS from First Principles/Code/src/dnssec.rs.source", "sourcePath": "src/dnssec.rs", "startLine": 144, "endLine": 172}
```

## Excerpt

<span id="rgbdns-frag-cbae6189fc30" class="rgbdns-fragment-target"></span>
### rgbdns-frag-cbae6189fc30: fn validate_zone_coverage

```rust
    fn validate_zone_coverage(&self, zone: &Zone) -> Result<()> {
        let configured = self.covered();
        let authoritative = zone.authoritative_names();
        if configured == authoritative {
            return Ok(());
        }
        let missing = authoritative
            .difference(&configured)
            .map(ToString::to_string)
            .collect::<Vec<_>>();
        let unknown = configured
            .difference(&authoritative)
            .map(ToString::to_string)
            .collect::<Vec<_>>();
        Err(Error::InvalidRecord(format!(
            "DNSSEC policy and authoritative zones differ (missing: {}; unknown: {})",
            if missing.is_empty() {
                "none".into()
            } else {
                missing.join(",")
            },
            if unknown.is_empty() {
                "none".into()
            } else {
                unknown.join(",")
            }
        )))
    }

```
