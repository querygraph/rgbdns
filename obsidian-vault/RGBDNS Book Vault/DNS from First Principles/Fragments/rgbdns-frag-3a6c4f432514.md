---
type: "code-fragment"
fragment_id: "rgbdns-frag-3a6c4f432514"
source_path: "src/acme.rs"
code_note: "DNS from First Principles/Code/src/acme.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "sync_directory"
kind: "fn"
start_line: 695
end_line: 700
---

# sync_directory

- Fragment ID: `rgbdns-frag-3a6c4f432514`
- Source file: [[DNS from First Principles/Code/src/acme.rs.source|src/acme.rs]]
- Lines: 695-700
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-3a6c4f432514", "codeNote": "DNS from First Principles/Code/src/acme.rs.source", "sourcePath": "src/acme.rs", "startLine": 695, "endLine": 700}
```

## Excerpt

<span id="rgbdns-frag-3a6c4f432514" class="rgbdns-fragment-target"></span>
### rgbdns-frag-3a6c4f432514: fn sync_directory

```rust
    fn sync_directory(path: &Path) -> Result<()> {
        fs::File::open(path)?.sync_all()?;
        Ok(())
    }
}

```
