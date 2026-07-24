---
type: "code-fragment"
fragment_id: "rgbdns-frag-ed917dea9ded"
source_path: "src/bin/dnstrace.rs"
code_note: "DNS from First Principles/Code/src/bin/dnstrace.rs.source"
language: "rust"
subsystem: "Command-line programs"
crate: "dnstrace"
symbol: "main"
kind: "fn"
start_line: 4
end_line: 10
---

# main

- Fragment ID: `rgbdns-frag-ed917dea9ded`
- Source file: [[DNS from First Principles/Code/src/bin/dnstrace.rs.source|src/bin/dnstrace.rs]]
- Lines: 4-10
- Subsystem: [[DNS from First Principles/Subsystems/Command-line programs|Command-line programs]]
- Component: [[DNS from First Principles/Components/dnstrace|dnstrace]]

```rgbdns-fragment
{"id": "rgbdns-frag-ed917dea9ded", "codeNote": "DNS from First Principles/Code/src/bin/dnstrace.rs.source", "heading": "rgbdns-frag-ed917dea9ded: fn main", "sourcePath": "src/bin/dnstrace.rs", "startLine": 4, "endLine": 10}
```

## Excerpt

<span id="rgbdns-frag-ed917dea9ded" class="rgbdns-fragment-target"></span>
### rgbdns-frag-ed917dea9ded: fn main

```rust
fn main() {
    if let Err(error) = run() {
        eprintln!("dnstrace: fatal: {error}");
        std::process::exit(111);
    }
}

```
