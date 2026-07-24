---
type: "code-fragment"
fragment_id: "rgbdns-frag-83ee748c0fa8"
source_path: "src/server.rs"
code_note: "DNS from First Principles/Code/src/server.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "live_udp_and_tcp_service"
kind: "fn"
start_line: 588
end_line: 639
---

# live_udp_and_tcp_service

- Fragment ID: `rgbdns-frag-83ee748c0fa8`
- Source file: [[DNS from First Principles/Code/src/server.rs.source|src/server.rs]]
- Lines: 588-639
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-83ee748c0fa8", "codeNote": "DNS from First Principles/Code/src/server.rs.source", "heading": "rgbdns-frag-83ee748c0fa8: fn live_udp_and_tcp_service", "sourcePath": "src/server.rs", "startLine": 588, "endLine": 639}
```

## Excerpt

<span id="rgbdns-frag-83ee748c0fa8" class="rgbdns-fragment-target"></span>
### rgbdns-frag-83ee748c0fa8: fn live_udp_and_tcp_service

```rust
    fn live_udp_and_tcp_service() {
        let tcp = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = tcp.local_addr().unwrap();
        let udp = UdpSocket::bind(address).unwrap();
        let mut data = ".example::ns.example\n+www.example:192.0.2.1\n".to_owned();
        for index in 0..80 {
            data.push_str(&format!("+many.example:192.0.2.{}\n", index % 250 + 1));
        }
        let zone = Zone::parse(&data).unwrap();
        thread::spawn(move || serve_sockets(zone, udp, tcp).unwrap());

        let request = query("www.example", RecordType::A, None);
        let udp_client = UdpSocket::bind("127.0.0.1:0").unwrap();
        udp_client
            .set_read_timeout(Some(Duration::from_secs(2)))
            .unwrap();
        udp_client.send_to(&request, address).unwrap();
        let mut buffer = [0; 2048];
        let size = udp_client.recv(&mut buffer).unwrap();
        assert_eq!(Message::decode(&buffer[..size]).unwrap().answers.len(), 1);

        let mut tcp_client = TcpStream::connect(address).unwrap();
        tcp_client
            .set_read_timeout(Some(Duration::from_secs(2)))
            .unwrap();
        let mut pipelined = Vec::new();
        for _ in 0..2 {
            pipelined.extend((request.len() as u16).to_be_bytes());
            pipelined.extend(&request);
        }
        tcp_client.write_all(&pipelined).unwrap();
        let mut length = [0; 2];
        for _ in 0..2 {
            tcp_client.read_exact(&mut length).unwrap();
            let mut response = vec![0; u16::from_be_bytes(length) as usize];
            tcp_client.read_exact(&mut response).unwrap();
            assert_eq!(Message::decode(&response).unwrap().answers.len(), 1);
        }

        let large_request = query("many.example", RecordType::A, None);
        tcp_client
            .write_all(&(large_request.len() as u16).to_be_bytes())
            .unwrap();
        tcp_client.write_all(&large_request).unwrap();
        tcp_client.read_exact(&mut length).unwrap();
        let mut response = vec![0; u16::from_be_bytes(length) as usize];
        tcp_client.read_exact(&mut response).unwrap();
        let response = Message::decode(&response).unwrap();
        assert_eq!(response.flags & 0x0200, 0);
        assert_eq!(response.answers.len(), 80, "{response:#?}");
    }
}
```
