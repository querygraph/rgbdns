---
type: "code-fragment"
fragment_id: "rgbdns-frag-23e65a489524"
source_path: "src/pick.rs"
code_note: "DNS from First Principles/Code/src/pick.rs.source"
language: "rust"
subsystem: "Specialized responders"
crate: "rgbdns"
symbol: "respond"
kind: "fn"
start_line: 96
end_line: 139
---

# respond

- Fragment ID: `rgbdns-frag-23e65a489524`
- Source file: [[DNS from First Principles/Code/src/pick.rs.source|src/pick.rs]]
- Lines: 96-139
- Subsystem: [[DNS from First Principles/Subsystems/Specialized responders|Specialized responders]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-23e65a489524", "codeNote": "DNS from First Principles/Code/src/pick.rs.source", "sourcePath": "src/pick.rs", "startLine": 96, "endLine": 139}
```

## Excerpt

<span id="rgbdns-frag-23e65a489524" class="rgbdns-fragment-target"></span>
### rgbdns-frag-23e65a489524: fn respond

```rust
    pub fn respond(&self, wire: &[u8], _limit: usize, client: IpAddr) -> Result<Vec<u8>> {
        let query = Message::decode(wire)?;
        if query.flags & 0x8000 != 0 || query.questions.len() != 1 {
            return Err(Error::Format("expected one query"));
        }
        let question = query.questions[0].clone();
        let mut response = Message {
            id: query.id,
            flags: 0x8000 | 0x0400,
            questions: vec![question.clone()],
            ..Default::default()
        };
        if question.qclass != 1
            || !matches!(
                question.qtype,
                RecordType::A | RecordType::Mx | RecordType::Any
            )
        {
            response.flags = 0x8000 | 5;
            return response.encode();
        }
        let selected = self.client_location(client);
        let addresses = self
            .addresses
            .get(&(selected, question.name.clone()))
            .or_else(|| self.addresses.get(&([0, 0], question.name.clone())));
        let Some(addresses) = addresses else {
            response.flags = 0x8000 | 5;
            return response.encode();
        };
        if matches!(question.qtype, RecordType::A | RecordType::Any) {
            let mut addresses = addresses.clone();
            shuffle(&mut addresses)?;
            response
                .answers
                .extend(addresses.into_iter().take(3).map(|address| Record {
                    name: question.name.clone(),
                    ttl: 5,
                    data: RData::A(address),
                }));
        }
        response.encode()
    }

```
