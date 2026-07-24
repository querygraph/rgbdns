---
type: "code-fragment"
fragment_id: "rgbdns-frag-4654b35f1e96"
source_path: "src/pick.rs"
code_note: "DNS from First Principles/Code/src/pick.rs.source"
language: "rust"
subsystem: "Specialized responders"
crate: "rgbdns"
symbol: "from_file"
kind: "fn"
start_line: 56
end_line: 64
---

# from_file

- Fragment ID: `rgbdns-frag-4654b35f1e96`
- Source file: [[DNS from First Principles/Code/src/pick.rs.source|src/pick.rs]]
- Lines: 56-64
- Subsystem: [[DNS from First Principles/Subsystems/Specialized responders|Specialized responders]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-4654b35f1e96", "codeNote": "DNS from First Principles/Code/src/pick.rs.source", "heading": "rgbdns-frag-4654b35f1e96: fn from_file", "sourcePath": "src/pick.rs", "startLine": 56, "endLine": 64}
```

## Excerpt

<span id="rgbdns-frag-4654b35f1e96" class="rgbdns-fragment-target"></span>
### rgbdns-frag-4654b35f1e96: fn from_file

```rust
    pub fn from_file(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        if path.extension().is_some_and(|extension| extension == "cdb") {
            Self::from_cdb(path)
        } else {
            Self::parse(&fs::read_to_string(path)?)
        }
    }

```
