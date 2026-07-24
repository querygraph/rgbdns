---
type: "code-fragment"
fragment_id: "rgbdns-frag-681cb74d8929"
source_path: "src/client.rs"
code_note: "DNS from First Principles/Code/src/client.rs.source"
language: "rust"
subsystem: "Resolution and recursion"
crate: "rgbdns"
symbol: "truncated_udp_response_falls_back_to_tcp"
kind: "fn"
start_line: 154
end_line: 207
---

# truncated_udp_response_falls_back_to_tcp

- Fragment ID: `rgbdns-frag-681cb74d8929`
- Source file: [[DNS from First Principles/Code/src/client.rs.source|src/client.rs]]
- Lines: 154-207
- Subsystem: [[DNS from First Principles/Subsystems/Resolution and recursion|Resolution and recursion]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-681cb74d8929", "codeNote": "DNS from First Principles/Code/src/client.rs.source", "heading": "rgbdns-frag-681cb74d8929: fn truncated_udp_response_falls_back_to_tcp", "sourcePath": "src/client.rs", "startLine": 154, "endLine": 207}
```

## Excerpt

<span id="rgbdns-frag-681cb74d8929" class="rgbdns-fragment-target"></span>
### rgbdns-frag-681cb74d8929: fn truncated_udp_response_falls_back_to_tcp

```rust
    fn truncated_udp_response_falls_back_to_tcp() {
        let tcp = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        let address = tcp.local_addr().unwrap();
        let udp = UdpSocket::bind(address).unwrap();
        let udp_thread = thread::spawn(move || {
            let mut wire = [0; 512];
            let (length, peer) = udp.recv_from(&mut wire).unwrap();
            let request = Message::decode(&wire[..length]).unwrap();
            let response = Message {
                id: request.id,
                flags: 0x8000 | 0x0200,
                questions: request.questions,
                ..Default::default()
            }
            .encode()
            .unwrap();
            udp.send_to(&response, peer).unwrap();
        });
        let tcp_thread = thread::spawn(move || {
            let (mut stream, _) = tcp.accept().unwrap();
            let mut length = [0; 2];
            stream.read_exact(&mut length).unwrap();
            let mut wire = vec![0; u16::from_be_bytes(length) as usize];
            stream.read_exact(&mut wire).unwrap();
            let request = Message::decode(&wire).unwrap();
            let name = request.questions[0].name.clone();
            let response = Message {
                id: request.id,
                flags: 0x8000,
                questions: request.questions,
                answers: vec![Record {
                    name,
                    ttl: 60,
                    data: RData::A(Ipv4Addr::new(192, 0, 2, 1)),
                }],
                ..Default::default()
            }
            .encode()
            .unwrap();
            stream
                .write_all(&(response.len() as u16).to_be_bytes())
                .unwrap();
            stream.write_all(&response).unwrap();
        });
        let response = query("example".parse().unwrap(), RecordType::A, true, &[address]).unwrap();
        udp_thread.join().unwrap();
        tcp_thread.join().unwrap();
        assert_eq!(
            response.answers[0].data,
            RData::A(Ipv4Addr::new(192, 0, 2, 1))
        );
    }

    #[test]
```
