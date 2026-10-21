---
type: "code-fragment"
fragment_id: "rgbdns-frag-6bd723a00c8b"
source_path: "src/acme.rs"
code_note: "DNS from First Principles/Code/src/acme.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "AcmeUpdates"
kind: "struct"
start_line: 60
end_line: 69
---

# AcmeUpdates

- Fragment ID: `rgbdns-frag-6bd723a00c8b`
- Source file: [[DNS from First Principles/Code/src/acme.rs.source|src/acme.rs]]
- Lines: 60-69
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-6bd723a00c8b", "codeNote": "DNS from First Principles/Code/src/acme.rs.source", "sourcePath": "src/acme.rs", "startLine": 60, "endLine": 69}
```

## Excerpt

<span id="rgbdns-frag-6bd723a00c8b" class="rgbdns-fragment-target"></span>
### rgbdns-frag-6bd723a00c8b: struct AcmeUpdates

```rust
pub struct AcmeUpdates {
    policies: BTreeMap<Name, Policy>,
    base: Zone,
    live: LiveZone,
    state_dir: PathBuf,
    publication: Option<Publication>,
    state: Mutex<State>,
}

#[derive(Clone)]
```
