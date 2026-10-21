---
type: "code-fragment"
fragment_id: "rgbdns-frag-3d80c9d6d68e"
source_path: "src/aname.rs"
code_note: "DNS from First Principles/Code/src/aname.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "tests"
kind: "mod"
start_line: 237
end_line: 248
---

# tests

- Fragment ID: `rgbdns-frag-3d80c9d6d68e`
- Source file: [[DNS from First Principles/Code/src/aname.rs.source|src/aname.rs]]
- Lines: 237-248
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-3d80c9d6d68e", "codeNote": "DNS from First Principles/Code/src/aname.rs.source", "sourcePath": "src/aname.rs", "startLine": 237, "endLine": 248}
```

## Excerpt

<span id="rgbdns-frag-3d80c9d6d68e" class="rgbdns-fragment-target"></span>
### rgbdns-frag-3d80c9d6d68e: mod tests

```rust
mod tests {
    use super::*;
    use std::{
        net::{Ipv4Addr, Ipv6Addr},
        sync::{
            Arc, Barrier,
            atomic::{AtomicUsize, Ordering},
        },
        thread,
    };

    #[test]
```
