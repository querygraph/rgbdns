---
type: "code-fragment"
fragment_id: "rgbdns-frag-de5b5b041b46"
source_path: "src/acme.rs"
code_note: "DNS from First Principles/Code/src/acme.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "new"
kind: "fn"
start_line: 76
end_line: 84
---

# new

- Fragment ID: `rgbdns-frag-de5b5b041b46`
- Source file: [[DNS from First Principles/Code/src/acme.rs.source|src/acme.rs]]
- Lines: 76-84
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-de5b5b041b46", "codeNote": "DNS from First Principles/Code/src/acme.rs.source", "sourcePath": "src/acme.rs", "startLine": 76, "endLine": 84}
```

## Excerpt

<span id="rgbdns-frag-de5b5b041b46" class="rgbdns-fragment-target"></span>
### rgbdns-frag-de5b5b041b46: fn new

```rust
    pub fn new(command: PathBuf, data: PathBuf) -> Result<Self> {
        if !command.is_absolute() || !data.is_absolute() {
            return Err(Error::InvalidRecord(
                "ACME publication command and data path must be absolute".into(),
            ));
        }
        Ok(Self { command, data })
    }

```
