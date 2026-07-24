---
type: "code-fragment"
fragment_id: "rgbdns-frag-0c2a1a2a636a"
source_path: "src/bin/dnscache.rs"
code_note: "DNS from First Principles/Code/src/bin/dnscache.rs.source"
language: "rust"
subsystem: "Command-line programs"
crate: "dnscache"
symbol: "main"
kind: "fn"
start_line: 21
end_line: 35
---

# main

- Fragment ID: `rgbdns-frag-0c2a1a2a636a`
- Source file: [[DNS from First Principles/Code/src/bin/dnscache.rs.source|src/bin/dnscache.rs]]
- Lines: 21-35
- Subsystem: [[DNS from First Principles/Subsystems/Command-line programs|Command-line programs]]
- Component: [[DNS from First Principles/Components/dnscache|dnscache]]

```rgbdns-fragment
{"id": "rgbdns-frag-0c2a1a2a636a", "codeNote": "DNS from First Principles/Code/src/bin/dnscache.rs.source", "heading": "rgbdns-frag-0c2a1a2a636a: fn main", "sourcePath": "src/bin/dnscache.rs", "startLine": 21, "endLine": 35}
```

## Excerpt

<span id="rgbdns-frag-0c2a1a2a636a" class="rgbdns-fragment-target"></span>
### rgbdns-frag-0c2a1a2a636a: fn main

```rust
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("warn")),
        )
        .with_target(false)
        .compact()
        .init();
    if let Err(error) = run().await {
        eprintln!("dnscache: fatal: {error}");
        std::process::exit(111);
    }
}

```
