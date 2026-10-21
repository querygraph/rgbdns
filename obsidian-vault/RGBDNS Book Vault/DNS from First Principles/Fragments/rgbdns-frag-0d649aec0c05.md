---
type: "code-fragment"
fragment_id: "rgbdns-frag-0d649aec0c05"
source_path: "src/dnssec.rs"
code_note: "DNS from First Principles/Code/src/dnssec.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "signs_owner"
kind: "fn"
start_line: 173
end_line: 183
---

# signs_owner

- Fragment ID: `rgbdns-frag-0d649aec0c05`
- Source file: [[DNS from First Principles/Code/src/dnssec.rs.source|src/dnssec.rs]]
- Lines: 173-183
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-0d649aec0c05", "codeNote": "DNS from First Principles/Code/src/dnssec.rs.source", "sourcePath": "src/dnssec.rs", "startLine": 173, "endLine": 183}
```

## Excerpt

<span id="rgbdns-frag-0d649aec0c05" class="rgbdns-fragment-target"></span>
### rgbdns-frag-0d649aec0c05: fn signs_owner

```rust
    fn signs_owner(&self, owner: &Name) -> bool {
        self.signing
            .iter()
            .map(|policy| (&policy.zone, true))
            .chain(self.unsigned.iter().map(|zone| (zone, false)))
            .filter(|(zone, _)| owner.is_subdomain_of(zone))
            .max_by_key(|(zone, _)| zone.labels().count())
            .is_some_and(|(_, signed)| signed)
    }
}

```
