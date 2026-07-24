---
type: "code-fragment"
fragment_id: "rgbdns-frag-bed5d669e0e8"
source_path: "src/multilog.rs"
code_note: "DNS from First Principles/Code/src/multilog.rs.source"
language: "rust"
subsystem: "Operations and supervision"
crate: "rgbdns"
symbol: "secure_append"
kind: "fn"
start_line: 195
end_line: 205
---

# secure_append

- Fragment ID: `rgbdns-frag-bed5d669e0e8`
- Source file: [[DNS from First Principles/Code/src/multilog.rs.source|src/multilog.rs]]
- Lines: 195-205
- Subsystem: [[DNS from First Principles/Subsystems/Operations and supervision|Operations and supervision]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-bed5d669e0e8", "codeNote": "DNS from First Principles/Code/src/multilog.rs.source", "heading": "rgbdns-frag-bed5d669e0e8: fn secure_append", "sourcePath": "src/multilog.rs", "startLine": 195, "endLine": 205}
```

## Excerpt

<span id="rgbdns-frag-bed5d669e0e8" class="rgbdns-fragment-target"></span>
### rgbdns-frag-bed5d669e0e8: fn secure_append

```rust
fn secure_append(path: &Path) -> io::Result<File> {
    let mut options = OpenOptions::new();
    options.create(true).append(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(0o644).custom_flags(nix::libc::O_NOFOLLOW);
    }
    options.open(path)
}

```
