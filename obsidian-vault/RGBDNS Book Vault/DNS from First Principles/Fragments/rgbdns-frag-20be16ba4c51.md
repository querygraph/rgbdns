---
type: "code-fragment"
fragment_id: "rgbdns-frag-20be16ba4c51"
source_path: "src/server.rs"
code_note: "DNS from First Principles/Code/src/server.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "tests"
kind: "mod"
start_line: 477
end_line: 486
---

# tests

- Fragment ID: `rgbdns-frag-20be16ba4c51`
- Source file: [[DNS from First Principles/Code/src/server.rs.source|src/server.rs]]
- Lines: 477-486
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-20be16ba4c51", "codeNote": "DNS from First Principles/Code/src/server.rs.source", "heading": "rgbdns-frag-20be16ba4c51: mod tests", "sourcePath": "src/server.rs", "startLine": 477, "endLine": 486}
```

## Excerpt

<span id="rgbdns-frag-20be16ba4c51" class="rgbdns-fragment-target"></span>
### rgbdns-frag-20be16ba4c51: mod tests

```rust
mod tests {
    use super::*;
    use crate::{Name, Question, RData, Record, RecordType};
    use std::{
        io::{Read, Write},
        net::{TcpListener, TcpStream, UdpSocket},
        thread,
        time::Duration,
    };

```
