---
type: "code-fragment"
fragment_id: "rgbdns-frag-ba98b1577a40"
source_path: "src/dnssec.rs"
code_note: "DNS from First Principles/Code/src/dnssec.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "sign_zones"
kind: "fn"
start_line: 362
end_line: 370
---

# sign_zones

- Fragment ID: `rgbdns-frag-ba98b1577a40`
- Source file: [[DNS from First Principles/Code/src/dnssec.rs.source|src/dnssec.rs]]
- Lines: 362-370
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-ba98b1577a40", "codeNote": "DNS from First Principles/Code/src/dnssec.rs.source", "sourcePath": "src/dnssec.rs", "startLine": 362, "endLine": 370}
```

## Excerpt

<span id="rgbdns-frag-ba98b1577a40" class="rgbdns-fragment-target"></span>
### rgbdns-frag-ba98b1577a40: fn sign_zones

```rust
fn sign_zones(zone: &Zone, policies: &[Policy]) -> Result<Vec<Record>> {
    let authoritative = zone.authoritative_names();
    let mut records = zone.records_outside(&authoritative);
    for policy in policies {
        records.extend(sign_zone(zone, policy)?);
    }
    Ok(records)
}

```
