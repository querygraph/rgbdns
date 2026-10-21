---
type: "code-fragment"
fragment_id: "rgbdns-frag-ecc0f67aca0e"
source_path: "src/dnssec.rs"
code_note: "DNS from First Principles/Code/src/dnssec.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "from_hickory"
kind: "fn"
start_line: 826
end_line: 838
---

# from_hickory

- Fragment ID: `rgbdns-frag-ecc0f67aca0e`
- Source file: [[DNS from First Principles/Code/src/dnssec.rs.source|src/dnssec.rs]]
- Lines: 826-838
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-ecc0f67aca0e", "codeNote": "DNS from First Principles/Code/src/dnssec.rs.source", "sourcePath": "src/dnssec.rs", "startLine": 826, "endLine": 838}
```

## Excerpt

<span id="rgbdns-frag-ecc0f67aca0e" class="rgbdns-fragment-target"></span>
### rgbdns-frag-ecc0f67aca0e: fn from_hickory

```rust
fn from_hickory(record: &hickory_server::proto::rr::Record) -> Result<Record> {
    let mut message = HickoryMessage::new(0, MessageType::Response, OpCode::Query);
    message.add_answer(record.clone());
    let bytes = message
        .to_vec()
        .map_err(|_| Error::Format("signed record encoding failed"))?;
    Message::decode(&bytes)?
        .answers
        .into_iter()
        .next()
        .ok_or(Error::Format("signed record conversion produced no record"))
}

```
