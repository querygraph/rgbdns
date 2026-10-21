---
type: "code-fragment"
fragment_id: "rgbdns-frag-5b73196a8388"
source_path: "src/dnssec.rs"
code_note: "DNS from First Principles/Code/src/dnssec.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "connect_tcp"
kind: "fn"
start_line: 97
end_line: 105
---

# connect_tcp

- Fragment ID: `rgbdns-frag-5b73196a8388`
- Source file: [[DNS from First Principles/Code/src/dnssec.rs.source|src/dnssec.rs]]
- Lines: 97-105
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-5b73196a8388", "codeNote": "DNS from First Principles/Code/src/dnssec.rs.source", "sourcePath": "src/dnssec.rs", "startLine": 97, "endLine": 105}
```

## Excerpt

<span id="rgbdns-frag-5b73196a8388" class="rgbdns-fragment-target"></span>
### rgbdns-frag-5b73196a8388: fn connect_tcp

```rust
    fn connect_tcp(
        &self,
        server_addr: SocketAddr,
        bind_addr: Option<SocketAddr>,
        timeout: Option<Duration>,
    ) -> Pin<Box<dyn Send + Future<Output = std::result::Result<Self::Tcp, io::Error>>>> {
        self.0.connect_tcp(server_addr, bind_addr, timeout)
    }

```
