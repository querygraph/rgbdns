---
type: "code-fragment"
fragment_id: "rgbdns-frag-d0264366d4be"
source_path: "src/bin/dnstxt.rs"
code_note: "DNS from First Principles/Code/src/bin/dnstxt.rs.source"
language: "rust"
subsystem: "Command-line programs"
crate: "dnstxt"
symbol: "main"
kind: "fn"
start_line: 4
end_line: 10
---

# main

- Fragment ID: `rgbdns-frag-d0264366d4be`
- Source file: [[DNS from First Principles/Code/src/bin/dnstxt.rs.source|src/bin/dnstxt.rs]]
- Lines: 4-10
- Subsystem: [[DNS from First Principles/Subsystems/Command-line programs|Command-line programs]]
- Component: [[DNS from First Principles/Components/dnstxt|dnstxt]]

```rgbdns-fragment
{"id": "rgbdns-frag-d0264366d4be", "codeNote": "DNS from First Principles/Code/src/bin/dnstxt.rs.source", "heading": "rgbdns-frag-d0264366d4be: fn main", "sourcePath": "src/bin/dnstxt.rs", "startLine": 4, "endLine": 10}
```

## Excerpt

<span id="rgbdns-frag-d0264366d4be" class="rgbdns-fragment-target"></span>
### rgbdns-frag-d0264366d4be: fn main

```rust
fn main() {
    if let Err(error) = run() {
        eprintln!("dnstxt: fatal: {error}");
        std::process::exit(111);
    }
}

```
