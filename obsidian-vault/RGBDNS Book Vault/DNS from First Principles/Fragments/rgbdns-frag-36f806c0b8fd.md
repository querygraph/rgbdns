---
type: "code-fragment"
fragment_id: "rgbdns-frag-36f806c0b8fd"
source_path: "src/bin/tai64n.rs"
code_note: "DNS from First Principles/Code/src/bin/tai64n.rs.source"
language: "rust"
subsystem: "Command-line programs"
crate: "tai64n"
symbol: "main"
kind: "fn"
start_line: 3
end_line: 13
---

# main

- Fragment ID: `rgbdns-frag-36f806c0b8fd`
- Source file: [[DNS from First Principles/Code/src/bin/tai64n.rs.source|src/bin/tai64n.rs]]
- Lines: 3-13
- Subsystem: [[DNS from First Principles/Subsystems/Command-line programs|Command-line programs]]
- Component: [[DNS from First Principles/Components/tai64n|tai64n]]

```rgbdns-fragment
{"id": "rgbdns-frag-36f806c0b8fd", "codeNote": "DNS from First Principles/Code/src/bin/tai64n.rs.source", "heading": "rgbdns-frag-36f806c0b8fd: fn main", "sourcePath": "src/bin/tai64n.rs", "startLine": 3, "endLine": 13}
```

## Excerpt

<span id="rgbdns-frag-36f806c0b8fd" class="rgbdns-fragment-target"></span>
### rgbdns-frag-36f806c0b8fd: fn main

```rust
fn main() {
    if std::env::args_os().len() != 1
        || rgbdns::tai64::stamp(
            BufReader::new(std::io::stdin().lock()),
            std::io::stdout().lock(),
        )
        .is_err()
    {
        std::process::exit(111);
    }
}
```
