---
type: "code-fragment"
fragment_id: "rgbdns-frag-ddfc81828cbd"
source_path: "src/bin/rgbdns-acme.rs"
code_note: "DNS from First Principles/Code/src/bin/rgbdns-acme.rs.source"
language: "rust"
subsystem: "Command-line programs"
crate: "rgbdns-acme"
symbol: "usage"
kind: "fn"
start_line: 7
end_line: 13
---

# usage

- Fragment ID: `rgbdns-frag-ddfc81828cbd`
- Source file: [[DNS from First Principles/Code/src/bin/rgbdns-acme.rs.source|src/bin/rgbdns-acme.rs]]
- Lines: 7-13
- Subsystem: [[DNS from First Principles/Subsystems/Command-line programs|Command-line programs]]
- Component: [[DNS from First Principles/Components/rgbdns-acme|rgbdns-acme]]

```rgbdns-fragment
{"id": "rgbdns-frag-ddfc81828cbd", "codeNote": "DNS from First Principles/Code/src/bin/rgbdns-acme.rs.source", "sourcePath": "src/bin/rgbdns-acme.rs", "startLine": 7, "endLine": 13}
```

## Excerpt

<span id="rgbdns-frag-ddfc81828cbd" class="rgbdns-fragment-target"></span>
### rgbdns-frag-ddfc81828cbd: fn usage

```rust
fn usage() -> ! {
    eprintln!(
        "usage:\n  rgbdns-acme present --zone Z --name N --value V [options]\n  rgbdns-acme cleanup --zone Z --name N --value V [options]\n  rgbdns-acme clear --zone Z --name N [options]\n  rgbdns-acme list [--zone Z] [options]\n\noptions:\n  --config FILE     default /etc/rgbdns/acme-update.conf\n  --server ADDRESS  default 127.0.0.1:53\n  --state-dir DIR   default /var/lib/rgbdns/tinydns\n  --json"
    );
    std::process::exit(2)
}

```
