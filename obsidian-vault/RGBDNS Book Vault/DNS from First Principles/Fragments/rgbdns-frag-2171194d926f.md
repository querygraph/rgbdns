---
type: "code-fragment"
fragment_id: "rgbdns-frag-2171194d926f"
source_path: "src/dnssec.rs"
code_note: "DNS from First Principles/Code/src/dnssec.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "compile_file"
kind: "fn"
start_line: 338
end_line: 352
---

# compile_file

- Fragment ID: `rgbdns-frag-2171194d926f`
- Source file: [[DNS from First Principles/Code/src/dnssec.rs.source|src/dnssec.rs]]
- Lines: 338-352
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-2171194d926f", "codeNote": "DNS from First Principles/Code/src/dnssec.rs.source", "sourcePath": "src/dnssec.rs", "startLine": 338, "endLine": 352}
```

## Excerpt

<span id="rgbdns-frag-2171194d926f" class="rgbdns-fragment-target"></span>
### rgbdns-frag-2171194d926f: fn compile_file

```rust
pub fn compile_file(input: &Path, policy_file: &Path, output: &Path) -> Result<()> {
    let policies = read_policies(policy_file)?;
    let source = Zone::from_file(input)?;
    let signed = build_snapshot(&source, &policies)?;
    let temporary = sibling_temporary(output);
    if let Err(error) = crate::cdb::compile(&signed, &temporary) {
        let _ = fs::remove_file(&temporary);
        return Err(error);
    }
    File::open(&temporary)?.sync_all()?;
    crate::cdb::load(&temporary)?;
    fs::rename(&temporary, output)?;
    sync_parent(output)
}

```
