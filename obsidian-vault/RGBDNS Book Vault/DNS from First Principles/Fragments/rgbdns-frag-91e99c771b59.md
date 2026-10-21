---
type: "code-fragment"
fragment_id: "rgbdns-frag-91e99c771b59"
source_path: "src/acme.rs"
code_note: "DNS from First Principles/Code/src/acme.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "run"
kind: "fn"
start_line: 85
end_line: 101
---

# run

- Fragment ID: `rgbdns-frag-91e99c771b59`
- Source file: [[DNS from First Principles/Code/src/acme.rs.source|src/acme.rs]]
- Lines: 85-101
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-91e99c771b59", "codeNote": "DNS from First Principles/Code/src/acme.rs.source", "sourcePath": "src/acme.rs", "startLine": 85, "endLine": 101}
```

## Excerpt

<span id="rgbdns-frag-91e99c771b59" class="rgbdns-fragment-target"></span>
### rgbdns-frag-91e99c771b59: fn run

```rust
    fn run(
        &self,
        state_dir: &Path,
        overlay: &Overlay,
        serials: &BTreeMap<Name, u32>,
        policies: &BTreeMap<Name, Policy>,
    ) -> Result<Zone> {
        let status = Command::new(&self.command).arg(state_dir).status()?;
        if !status.success() {
            return Err(Error::Format("ACME publication command failed"));
        }
        let zone = Zone::from_file(&self.data)?;
        validate_published(&zone, overlay, serials, policies)?;
        Ok(zone)
    }
}

```
