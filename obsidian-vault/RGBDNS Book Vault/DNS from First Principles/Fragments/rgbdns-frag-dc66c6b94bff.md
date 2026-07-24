---
type: "code-fragment"
fragment_id: "rgbdns-frag-dc66c6b94bff"
source_path: "src/multilog.rs"
code_note: "DNS from First Principles/Code/src/multilog.rs.source"
language: "rust"
subsystem: "Operations and supervision"
crate: "rgbdns"
symbol: "prune"
kind: "fn"
start_line: 174
end_line: 194
---

# prune

- Fragment ID: `rgbdns-frag-dc66c6b94bff`
- Source file: [[DNS from First Principles/Code/src/multilog.rs.source|src/multilog.rs]]
- Lines: 174-194
- Subsystem: [[DNS from First Principles/Subsystems/Operations and supervision|Operations and supervision]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-dc66c6b94bff", "codeNote": "DNS from First Principles/Code/src/multilog.rs.source", "heading": "rgbdns-frag-dc66c6b94bff: fn prune", "sourcePath": "src/multilog.rs", "startLine": 174, "endLine": 194}
```

## Excerpt

<span id="rgbdns-frag-dc66c6b94bff" class="rgbdns-fragment-target"></span>
### rgbdns-frag-dc66c6b94bff: fn prune

```rust
    fn prune(&self) -> io::Result<()> {
        let mut rotated = fs::read_dir(&self.directory)?
            .filter_map(|entry| entry.ok())
            .filter(|entry| {
                let name = entry.file_name();
                let name = name.to_string_lossy();
                entry.file_type().is_ok_and(|kind| kind.is_file())
                    && name.starts_with('@')
                    && name.ends_with(".s")
            })
            .map(|entry| entry.path())
            .collect::<Vec<_>>();
        rotated.sort();
        let remove = rotated.len().saturating_sub(self.retain);
        for path in rotated.into_iter().take(remove) {
            fs::remove_file(path)?;
        }
        Ok(())
    }
}

```
