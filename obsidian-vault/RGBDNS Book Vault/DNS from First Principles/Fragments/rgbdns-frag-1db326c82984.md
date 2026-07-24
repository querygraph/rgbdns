---
type: "code-fragment"
fragment_id: "rgbdns-frag-1db326c82984"
source_path: "src/rbl.rs"
code_note: "DNS from First Principles/Code/src/rbl.rs.source"
language: "rust"
subsystem: "Specialized responders"
crate: "rgbdns"
symbol: "respond"
kind: "fn"
start_line: 101
end_line: 151
---

# respond

- Fragment ID: `rgbdns-frag-1db326c82984`
- Source file: [[DNS from First Principles/Code/src/rbl.rs.source|src/rbl.rs]]
- Lines: 101-151
- Subsystem: [[DNS from First Principles/Subsystems/Specialized responders|Specialized responders]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-1db326c82984", "codeNote": "DNS from First Principles/Code/src/rbl.rs.source", "heading": "rgbdns-frag-1db326c82984: fn respond", "sourcePath": "src/rbl.rs", "startLine": 101, "endLine": 151}
```

## Excerpt

<span id="rgbdns-frag-1db326c82984" class="rgbdns-fragment-target"></span>
### rgbdns-frag-1db326c82984: fn respond

```rust
    pub fn respond(&self, base: &Name, wire: &[u8], _limit: usize) -> Result<Vec<u8>> {
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
        let wants_a = matches!(question.qtype, RecordType::A | RecordType::Any);
        let wants_txt = matches!(question.qtype, RecordType::Txt | RecordType::Any);
        let Some(labels) = numeric_prefix(&question.name, base, 4) else {
            response.flags = 0x8000 | 5;
            return response.encode();
        };
        if question.qclass != 1 || (!wants_a && !wants_txt) || labels.len() != 4 {
            response.flags = 0x8000 | 5;
            return response.encode();
        }
        let address = Ipv4Addr::new(labels[3], labels[2], labels[1], labels[0]);
        if !self.listed(address) {
            response.flags |= 3;
            return response.encode();
        }
        let (answer, configured_text) = self.response();
        if wants_a {
            response.answers.push(Record {
                name: question.name.clone(),
                ttl: 2048,
                data: RData::A(answer),
            });
        }
        if wants_txt {
            let mut text = configured_text[..configured_text.len().min(96)].to_vec();
            if text.last() == Some(&b'$') {
                text.pop();
                text.extend(address.to_string().bytes());
            }
            response.answers.push(Record {
                name: question.name,
                ttl: 2048,
                data: RData::Txt(vec![text]),
            });
        }
        response.encode()
    }
}

```
