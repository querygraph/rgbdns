---
type: "code-fragment"
fragment_id: "rgbdns-frag-41648bb6489a"
source_path: "src/bin/setuidgid.rs"
code_note: "DNS from First Principles/Code/src/bin/setuidgid.rs.source"
language: "rust"
subsystem: "Command-line programs"
crate: "setuidgid"
symbol: "main"
kind: "fn"
start_line: 22
end_line: 25
---

# main

- Fragment ID: `rgbdns-frag-41648bb6489a`
- Source file: [[DNS from First Principles/Code/src/bin/setuidgid.rs.source|src/bin/setuidgid.rs]]
- Lines: 22-25
- Subsystem: [[DNS from First Principles/Subsystems/Command-line programs|Command-line programs]]
- Component: [[DNS from First Principles/Components/setuidgid|setuidgid]]

```rgbdns-fragment
{"id": "rgbdns-frag-41648bb6489a", "codeNote": "DNS from First Principles/Code/src/bin/setuidgid.rs.source", "heading": "rgbdns-frag-41648bb6489a: fn main", "sourcePath": "src/bin/setuidgid.rs", "startLine": 22, "endLine": 25}
```

## Excerpt

<span id="rgbdns-frag-41648bb6489a" class="rgbdns-fragment-target"></span>
### rgbdns-frag-41648bb6489a: fn main

```rust
fn main() {
    eprintln!("setuidgid: fatal: this platform does not support Unix identities");
    std::process::exit(111);
}
```
