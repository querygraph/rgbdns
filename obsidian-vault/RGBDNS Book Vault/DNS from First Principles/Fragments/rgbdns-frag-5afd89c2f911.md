---
type: "code-fragment"
fragment_id: "rgbdns-frag-5afd89c2f911"
source_path: "src/rbl.rs"
code_note: "DNS from First Principles/Code/src/rbl.rs.source"
language: "rust"
subsystem: "Specialized responders"
crate: "rgbdns"
symbol: "from_file"
kind: "fn"
start_line: 49
end_line: 57
---

# from_file

- Fragment ID: `rgbdns-frag-5afd89c2f911`
- Source file: [[DNS from First Principles/Code/src/rbl.rs.source|src/rbl.rs]]
- Lines: 49-57
- Subsystem: [[DNS from First Principles/Subsystems/Specialized responders|Specialized responders]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-5afd89c2f911", "codeNote": "DNS from First Principles/Code/src/rbl.rs.source", "heading": "rgbdns-frag-5afd89c2f911: fn from_file", "sourcePath": "src/rbl.rs", "startLine": 49, "endLine": 57}
```

## Excerpt

<span id="rgbdns-frag-5afd89c2f911" class="rgbdns-fragment-target"></span>
### rgbdns-frag-5afd89c2f911: fn from_file

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
