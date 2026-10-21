---
type: "code-fragment"
fragment_id: "rgbdns-frag-6163f8fafac5"
source_path: "src/dnssec.rs"
code_note: "DNS from First Principles/Code/src/dnssec.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "sign_snapshot"
kind: "fn"
start_line: 353
end_line: 361
---

# sign_snapshot

- Fragment ID: `rgbdns-frag-6163f8fafac5`
- Source file: [[DNS from First Principles/Code/src/dnssec.rs.source|src/dnssec.rs]]
- Lines: 353-361
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-6163f8fafac5", "codeNote": "DNS from First Principles/Code/src/dnssec.rs.source", "sourcePath": "src/dnssec.rs", "startLine": 353, "endLine": 361}
```

## Excerpt

<span id="rgbdns-frag-6163f8fafac5" class="rgbdns-fragment-target"></span>
### rgbdns-frag-6163f8fafac5: fn sign_snapshot

```rust
pub fn sign_snapshot(zone: &Zone, policies: &[Policy]) -> Result<Vec<Record>> {
    let policy_set = PolicySet {
        signing: policies.to_vec(),
        unsigned: BTreeSet::new(),
    };
    policy_set.validate_zone_coverage(zone)?;
    sign_zones(zone, policies)
}

```
