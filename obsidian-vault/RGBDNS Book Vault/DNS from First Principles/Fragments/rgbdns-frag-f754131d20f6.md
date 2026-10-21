---
type: "code-fragment"
fragment_id: "rgbdns-frag-f754131d20f6"
source_path: "src/server.rs"
code_note: "DNS from First Principles/Code/src/server.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "starting"
kind: "fn"
start_line: 30
end_line: 35
---

# starting

- Fragment ID: `rgbdns-frag-f754131d20f6`
- Source file: [[DNS from First Principles/Code/src/server.rs.source|src/server.rs]]
- Lines: 30-35
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-f754131d20f6", "codeNote": "DNS from First Principles/Code/src/server.rs.source", "sourcePath": "src/server.rs", "startLine": 30, "endLine": 35}
```

## Excerpt

<span id="rgbdns-frag-f754131d20f6" class="rgbdns-fragment-target"></span>
### rgbdns-frag-f754131d20f6: fn starting

```rust
    fn starting(self) {
        if self.enabled {
            write_log_line("starting tinydns");
        }
    }

```
