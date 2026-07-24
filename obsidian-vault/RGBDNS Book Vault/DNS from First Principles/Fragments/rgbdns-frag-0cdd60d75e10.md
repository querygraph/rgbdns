---
type: "code-fragment"
fragment_id: "rgbdns-frag-0cdd60d75e10"
source_path: "src/multilog.rs"
code_note: "DNS from First Principles/Code/src/multilog.rs.source"
language: "rust"
subsystem: "Operations and supervision"
crate: "rgbdns"
symbol: "rotate"
kind: "fn"
start_line: 153
end_line: 173
---

# rotate

- Fragment ID: `rgbdns-frag-0cdd60d75e10`
- Source file: [[DNS from First Principles/Code/src/multilog.rs.source|src/multilog.rs]]
- Lines: 153-173
- Subsystem: [[DNS from First Principles/Subsystems/Operations and supervision|Operations and supervision]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-0cdd60d75e10", "codeNote": "DNS from First Principles/Code/src/multilog.rs.source", "heading": "rgbdns-frag-0cdd60d75e10: fn rotate", "sourcePath": "src/multilog.rs", "startLine": 153, "endLine": 173}
```

## Excerpt

<span id="rgbdns-frag-0cdd60d75e10" class="rgbdns-fragment-target"></span>
### rgbdns-frag-0cdd60d75e10: fn rotate

```rust
    fn rotate(&mut self) -> io::Result<()> {
        self.file.flush()?;
        self.file.sync_all()?;
        let current = self.directory.join("current");
        let mut rotated;
        loop {
            let stamp = tai64n_label(SystemTime::now());
            rotated = self
                .directory
                .join(format!("{stamp}.{:08x}.s", self.sequence));
            self.sequence = self.sequence.wrapping_add(1);
            if !rotated.exists() {
                break;
            }
        }
        fs::rename(&current, rotated)?;
        self.file = secure_append(&current)?;
        self.size = 0;
        self.prune()
    }

```
