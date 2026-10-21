---
type: "code-fragment"
fragment_id: "rgbdns-frag-d25ea4312732"
source_path: "src/acme.rs"
code_note: "DNS from First Principles/Code/src/acme.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "authorized"
kind: "fn"
start_line: 508
end_line: 517
---

# authorized

- Fragment ID: `rgbdns-frag-d25ea4312732`
- Source file: [[DNS from First Principles/Code/src/acme.rs.source|src/acme.rs]]
- Lines: 508-517
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-d25ea4312732", "codeNote": "DNS from First Principles/Code/src/acme.rs.source", "sourcePath": "src/acme.rs", "startLine": 508, "endLine": 517}
```

## Excerpt

<span id="rgbdns-frag-d25ea4312732" class="rgbdns-fragment-target"></span>
### rgbdns-frag-d25ea4312732: fn authorized

```rust
fn authorized(policy: &Policy, owner: &Name) -> bool {
    if !owner.is_subdomain_of(&policy.zone) {
        return false;
    }
    let text = owner.to_string().to_ascii_lowercase();
    let suffix = policy.zone.to_string().to_ascii_lowercase();
    let relative = text.strip_suffix(&suffix).unwrap_or("");
    relative == "_acme-challenge." || relative.starts_with("_acme-challenge.")
}

```
