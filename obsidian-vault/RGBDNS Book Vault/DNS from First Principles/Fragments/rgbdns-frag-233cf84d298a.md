---
type: "code-fragment"
fragment_id: "rgbdns-frag-233cf84d298a"
source_path: "src/dnssec.rs"
code_note: "DNS from First Principles/Code/src/dnssec.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "bind_udp"
kind: "fn"
start_line: 106
end_line: 114
---

# bind_udp

- Fragment ID: `rgbdns-frag-233cf84d298a`
- Source file: [[DNS from First Principles/Code/src/dnssec.rs.source|src/dnssec.rs]]
- Lines: 106-114
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-233cf84d298a", "codeNote": "DNS from First Principles/Code/src/dnssec.rs.source", "sourcePath": "src/dnssec.rs", "startLine": 106, "endLine": 114}
```

## Excerpt

<span id="rgbdns-frag-233cf84d298a" class="rgbdns-fragment-target"></span>
### rgbdns-frag-233cf84d298a: fn bind_udp

```rust
    fn bind_udp(
        &self,
        local_addr: SocketAddr,
        server_addr: SocketAddr,
    ) -> Pin<Box<dyn Send + Future<Output = std::result::Result<Self::Udp, io::Error>>>> {
        self.0.bind_udp(local_addr, server_addr)
    }
}

```
