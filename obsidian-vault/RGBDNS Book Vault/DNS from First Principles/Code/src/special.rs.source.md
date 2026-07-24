---
type: "code-file"
source_path: "src/special.rs"
language: "rust"
subsystem: "Specialized responders"
crate: "rgbdns"
line_count: 10
fragment_count: 2
rgbdns_commit: "472c2087"
---

# src/special.rs

- Subsystem: [[DNS from First Principles/Subsystems/Specialized responders|Specialized responders]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]
- Source path: `src/special.rs`
- Lines: 10
- Summary: Shared UDP/TCP transport for specialized authoritative responders.

## Extracted Fragments

- [[DNS from First Principles/Fragments/rgbdns-frag-89699876ef8a|Handler]]: lines 6-7
- [[DNS from First Principles/Fragments/rgbdns-frag-c270157bbc26|serve]]: lines 8-10

## Full Source

```rust
//! Shared UDP/TCP transport for specialized authoritative responders.

use crate::Result;
use std::sync::Arc;

pub type Handler = crate::transport::Handler;

pub fn serve(address: &str, handler: Arc<Handler>) -> Result<()> {
    crate::transport::serve(address, handler)
}
```
