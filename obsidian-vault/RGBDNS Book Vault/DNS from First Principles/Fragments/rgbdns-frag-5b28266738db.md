---
type: "code-fragment"
fragment_id: "rgbdns-frag-5b28266738db"
source_path: "src/acme.rs"
code_note: "DNS from First Principles/Code/src/acme.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "from_file"
kind: "fn"
start_line: 250
end_line: 258
---

# from_file

- Fragment ID: `rgbdns-frag-5b28266738db`
- Source file: [[DNS from First Principles/Code/src/acme.rs.source|src/acme.rs]]
- Lines: 250-258
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-5b28266738db", "codeNote": "DNS from First Principles/Code/src/acme.rs.source", "sourcePath": "src/acme.rs", "startLine": 250, "endLine": 258}
```

## Excerpt

<span id="rgbdns-frag-5b28266738db" class="rgbdns-fragment-target"></span>
### rgbdns-frag-5b28266738db: fn from_file

```rust
    pub fn from_file(
        config: impl AsRef<Path>,
        state_dir: impl AsRef<Path>,
        base: Zone,
        live: LiveZone,
    ) -> Result<Self> {
        Self::from_file_with_publication(config, state_dir, base, live, None)
    }

```
