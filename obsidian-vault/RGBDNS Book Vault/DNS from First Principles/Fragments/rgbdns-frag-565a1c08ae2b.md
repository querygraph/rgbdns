---
type: "code-fragment"
fragment_id: "rgbdns-frag-565a1c08ae2b"
source_path: "src/bin/tai64nlocal.rs"
code_note: "DNS from First Principles/Code/src/bin/tai64nlocal.rs.source"
language: "rust"
subsystem: "Command-line programs"
crate: "tai64nlocal"
symbol: "main"
kind: "fn"
start_line: 3
end_line: 13
---

# main

- Fragment ID: `rgbdns-frag-565a1c08ae2b`
- Source file: [[DNS from First Principles/Code/src/bin/tai64nlocal.rs.source|src/bin/tai64nlocal.rs]]
- Lines: 3-13
- Subsystem: [[DNS from First Principles/Subsystems/Command-line programs|Command-line programs]]
- Component: [[DNS from First Principles/Components/tai64nlocal|tai64nlocal]]

```rgbdns-fragment
{"id": "rgbdns-frag-565a1c08ae2b", "codeNote": "DNS from First Principles/Code/src/bin/tai64nlocal.rs.source", "heading": "rgbdns-frag-565a1c08ae2b: fn main", "sourcePath": "src/bin/tai64nlocal.rs", "startLine": 3, "endLine": 13}
```

## Excerpt

<span id="rgbdns-frag-565a1c08ae2b" class="rgbdns-fragment-target"></span>
### rgbdns-frag-565a1c08ae2b: fn main

```rust
fn main() {
    if std::env::args_os().len() != 1
        || rgbdns::tai64::localize(
            BufReader::new(std::io::stdin().lock()),
            std::io::stdout().lock(),
        )
        .is_err()
    {
        std::process::exit(111);
    }
}
```
