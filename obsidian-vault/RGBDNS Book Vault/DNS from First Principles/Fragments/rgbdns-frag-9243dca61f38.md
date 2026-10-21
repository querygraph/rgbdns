---
type: "code-fragment"
fragment_id: "rgbdns-frag-9243dca61f38"
source_path: "src/bin/rgbdns-log-report.rs"
code_note: "DNS from First Principles/Code/src/bin/rgbdns-log-report.rs.source"
language: "rust"
subsystem: "Command-line programs"
crate: "rgbdns-log-report"
symbol: "main"
kind: "fn"
start_line: 173
end_line: 187
---

# main

- Fragment ID: `rgbdns-frag-9243dca61f38`
- Source file: [[DNS from First Principles/Code/src/bin/rgbdns-log-report.rs.source|src/bin/rgbdns-log-report.rs]]
- Lines: 173-187
- Subsystem: [[DNS from First Principles/Subsystems/Command-line programs|Command-line programs]]
- Component: [[DNS from First Principles/Components/rgbdns-log-report|rgbdns-log-report]]

```rgbdns-fragment
{"id": "rgbdns-frag-9243dca61f38", "codeNote": "DNS from First Principles/Code/src/bin/rgbdns-log-report.rs.source", "sourcePath": "src/bin/rgbdns-log-report.rs", "startLine": 173, "endLine": 187}
```

## Excerpt

<span id="rgbdns-frag-9243dca61f38" class="rgbdns-fragment-target"></span>
### rgbdns-frag-9243dca61f38: fn main

```rust
fn main() {
    if env::args()
        .nth(1)
        .is_some_and(|argument| matches!(argument.as_str(), "-h" | "--help"))
    {
        println!("{}", usage());
        return;
    }
    if let Err(error) = run() {
        eprintln!("rgbdns-log-report: {error}");
        std::process::exit(100);
    }
}

#[cfg(test)]
```
