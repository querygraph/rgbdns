---
type: "code-fragment"
fragment_id: "rgbdns-frag-1b04b3628374"
source_path: "src/server.rs"
code_note: "DNS from First Principles/Code/src/server.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "tests"
kind: "mod"
start_line: 753
end_line: 762
---

# tests

- Fragment ID: `rgbdns-frag-1b04b3628374`
- Source file: [[DNS from First Principles/Code/src/server.rs.source|src/server.rs]]
- Lines: 753-762
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-1b04b3628374", "codeNote": "DNS from First Principles/Code/src/server.rs.source", "sourcePath": "src/server.rs", "startLine": 753, "endLine": 762}
```

## Excerpt

<span id="rgbdns-frag-1b04b3628374" class="rgbdns-fragment-target"></span>
### rgbdns-frag-1b04b3628374: mod tests

```rust
mod tests {
    use super::*;
    use crate::{Name, Question, RData, Record, RecordType, zone::RecordMetadata};
    use std::{
        io::{Read, Write},
        net::{TcpListener, TcpStream, UdpSocket},
        thread,
        time::Duration,
    };

```
