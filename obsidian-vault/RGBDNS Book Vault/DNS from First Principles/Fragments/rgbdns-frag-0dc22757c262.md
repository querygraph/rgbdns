---
type: "code-fragment"
fragment_id: "rgbdns-frag-0dc22757c262"
source_path: "src/acme.rs"
code_note: "DNS from First Principles/Code/src/acme.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "flatten"
kind: "fn"
start_line: 626
end_line: 644
---

# flatten

- Fragment ID: `rgbdns-frag-0dc22757c262`
- Source file: [[DNS from First Principles/Code/src/acme.rs.source|src/acme.rs]]
- Lines: 626-644
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-0dc22757c262", "codeNote": "DNS from First Principles/Code/src/acme.rs.source", "sourcePath": "src/acme.rs", "startLine": 626, "endLine": 644}
```

## Excerpt

<span id="rgbdns-frag-0dc22757c262" class="rgbdns-fragment-target"></span>
### rgbdns-frag-0dc22757c262: fn flatten

```rust
fn flatten(
    overlay: &Overlay,
    policies: &BTreeMap<Name, Policy>,
) -> Result<Vec<(Name, Vec<u8>, u32)>> {
    let mut records = Vec::new();
    for (owner, values) in overlay {
        let ttl = policies
            .values()
            .filter(|policy| authorized(policy, owner))
            .map(|policy| policy.ttl)
            .min()
            .ok_or(Error::Format("ACME overlay owner has no policy"))?;
        for value in values {
            records.push((owner.clone(), value.clone(), ttl));
        }
    }
    Ok(records)
}

```
