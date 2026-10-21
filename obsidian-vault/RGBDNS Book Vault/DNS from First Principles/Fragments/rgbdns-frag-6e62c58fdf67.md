---
type: "code-fragment"
fragment_id: "rgbdns-frag-6e62c58fdf67"
source_path: "src/dnssec.rs"
code_note: "DNS from First Principles/Code/src/dnssec.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "sign_file"
kind: "fn"
start_line: 329
end_line: 337
---

# sign_file

- Fragment ID: `rgbdns-frag-6e62c58fdf67`
- Source file: [[DNS from First Principles/Code/src/dnssec.rs.source|src/dnssec.rs]]
- Lines: 329-337
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-6e62c58fdf67", "codeNote": "DNS from First Principles/Code/src/dnssec.rs.source", "sourcePath": "src/dnssec.rs", "startLine": 329, "endLine": 337}
```

## Excerpt

<span id="rgbdns-frag-6e62c58fdf67" class="rgbdns-fragment-target"></span>
### rgbdns-frag-6e62c58fdf67: fn sign_file

```rust
pub fn sign_file(input: &Path, policy_file: &Path, output: &Path) -> Result<()> {
    let policies = read_policies(policy_file)?;
    let zone = Zone::from_file(input)?;
    let signed = build_snapshot(&zone, &policies)?;
    let temporary = sibling_temporary(output);
    axfr::write_zone(&signed, output, &temporary)?;
    sync_parent(output)
}

```
