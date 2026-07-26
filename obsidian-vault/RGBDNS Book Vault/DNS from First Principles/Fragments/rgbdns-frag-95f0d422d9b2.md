---
type: "code-fragment"
fragment_id: "rgbdns-frag-95f0d422d9b2"
source_path: "src/server.rs"
code_note: "DNS from First Principles/Code/src/server.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "integrated_tcp_listener_serves_axfr"
kind: "fn"
start_line: 812
end_line: 833
---

# integrated_tcp_listener_serves_axfr

- Fragment ID: `rgbdns-frag-95f0d422d9b2`
- Source file: [[DNS from First Principles/Code/src/server.rs.source|src/server.rs]]
- Lines: 812-833
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-95f0d422d9b2", "codeNote": "DNS from First Principles/Code/src/server.rs.source", "heading": "rgbdns-frag-95f0d422d9b2: fn integrated_tcp_listener_serves_axfr", "sourcePath": "src/server.rs", "startLine": 812, "endLine": 833}
```

## Excerpt

<span id="rgbdns-frag-95f0d422d9b2" class="rgbdns-fragment-target"></span>
### rgbdns-frag-95f0d422d9b2: fn integrated_tcp_listener_serves_axfr

```rust
    fn integrated_tcp_listener_serves_axfr() {
        let tcp = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = tcp.local_addr().unwrap();
        let udp = UdpSocket::bind(address).unwrap();
        let zone = Zone::parse(
            "Zexample:ns.example:hostmaster.example:7:8:9:10:11:12\n\
             &example:192.0.2.53:ns.example:300\n\
             +www.example:192.0.2.1:60\n",
        )
        .unwrap();
        thread::spawn(move || {
            serve_sockets(zone, udp, tcp, Some(vec!["127.0.0.0/8".parse().unwrap()])).unwrap()
        });

        let records = crate::axfr::fetch(address, "example".parse().unwrap()).unwrap();
        assert_eq!(records.first(), records.last());
        assert!(records.iter().any(|record| {
            record.name == "www.example".parse().unwrap()
                && record.data == RData::A("192.0.2.1".parse().unwrap())
        }));
    }
}
```
