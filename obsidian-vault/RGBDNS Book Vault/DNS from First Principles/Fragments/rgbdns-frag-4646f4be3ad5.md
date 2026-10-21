---
type: "code-fragment"
fragment_id: "rgbdns-frag-4646f4be3ad5"
source_path: "src/acme.rs"
code_note: "DNS from First Principles/Code/src/acme.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "parse_config"
kind: "fn"
start_line: 437
end_line: 449
---

# parse_config

- Fragment ID: `rgbdns-frag-4646f4be3ad5`
- Source file: [[DNS from First Principles/Code/src/acme.rs.source|src/acme.rs]]
- Lines: 437-449
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-4646f4be3ad5", "codeNote": "DNS from First Principles/Code/src/acme.rs.source", "sourcePath": "src/acme.rs", "startLine": 437, "endLine": 449}
```

## Excerpt

<span id="rgbdns-frag-4646f4be3ad5" class="rgbdns-fragment-target"></span>
### rgbdns-frag-4646f4be3ad5: fn parse_config

```rust
fn parse_config(text: &str, base: &Zone) -> Result<BTreeMap<Name, Policy>> {
    let policies = parse_config_without_zone(text)?;
    for policy in policies.values() {
        if !base.is_authoritative(&policy.zone) {
            return Err(Error::InvalidRecord(format!(
                "ACME zone {} is not authoritative",
                policy.zone
            )));
        }
    }
    Ok(policies)
}

```
