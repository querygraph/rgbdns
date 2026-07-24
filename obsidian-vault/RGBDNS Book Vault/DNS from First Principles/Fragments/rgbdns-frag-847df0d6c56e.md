---
type: "code-fragment"
fragment_id: "rgbdns-frag-847df0d6c56e"
source_path: "src/multilog.rs"
code_note: "DNS from First Principles/Code/src/multilog.rs.source"
language: "rust"
subsystem: "Operations and supervision"
crate: "rgbdns"
symbol: "open"
kind: "fn"
start_line: 119
end_line: 133
---

# open

- Fragment ID: `rgbdns-frag-847df0d6c56e`
- Source file: [[DNS from First Principles/Code/src/multilog.rs.source|src/multilog.rs]]
- Lines: 119-133
- Subsystem: [[DNS from First Principles/Subsystems/Operations and supervision|Operations and supervision]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-847df0d6c56e", "codeNote": "DNS from First Principles/Code/src/multilog.rs.source", "heading": "rgbdns-frag-847df0d6c56e: fn open", "sourcePath": "src/multilog.rs", "startLine": 119, "endLine": 133}
```

## Excerpt

<span id="rgbdns-frag-847df0d6c56e" class="rgbdns-fragment-target"></span>
### rgbdns-frag-847df0d6c56e: fn open

```rust
    fn open(directory: &Path, maximum: u64, retain: usize) -> io::Result<Self> {
        fs::create_dir_all(directory)?;
        let path = directory.join("current");
        let file = secure_append(&path)?;
        let size = file.metadata()?.len();
        Ok(Self {
            directory: directory.to_owned(),
            file,
            size,
            maximum,
            retain,
            sequence: 0,
        })
    }

```
