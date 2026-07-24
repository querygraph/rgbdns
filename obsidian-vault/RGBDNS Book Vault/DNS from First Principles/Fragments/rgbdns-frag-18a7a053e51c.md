---
type: "code-fragment"
fragment_id: "rgbdns-frag-18a7a053e51c"
source_path: "src/client.rs"
code_note: "DNS from First Principles/Code/src/client.rs.source"
language: "rust"
subsystem: "Resolution and recursion"
crate: "rgbdns"
symbol: "rejects_responses_with_a_mismatched_question_opcode_or_tcp_truncation"
kind: "fn"
start_line: 220
end_line: 246
---

# rejects_responses_with_a_mismatched_question_opcode_or_tcp_truncation

- Fragment ID: `rgbdns-frag-18a7a053e51c`
- Source file: [[DNS from First Principles/Code/src/client.rs.source|src/client.rs]]
- Lines: 220-246
- Subsystem: [[DNS from First Principles/Subsystems/Resolution and recursion|Resolution and recursion]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-18a7a053e51c", "codeNote": "DNS from First Principles/Code/src/client.rs.source", "heading": "rgbdns-frag-18a7a053e51c: fn rejects_responses_with_a_mismatched_question_opcode_or_tcp_truncation", "sourcePath": "src/client.rs", "startLine": 220, "endLine": 246}
```

## Excerpt

<span id="rgbdns-frag-18a7a053e51c" class="rgbdns-fragment-target"></span>
### rgbdns-frag-18a7a053e51c: fn rejects_responses_with_a_mismatched_question_opcode_or_tcp_truncation

```rust
    fn rejects_responses_with_a_mismatched_question_opcode_or_tcp_truncation() {
        let question = Question {
            name: "example".parse().unwrap(),
            qtype: RecordType::A,
            qclass: 1,
        };
        let valid = Message {
            id: 7,
            flags: 0x8000,
            questions: vec![question.clone()],
            ..Default::default()
        };
        assert!(validate(valid.clone(), 7, &question, false).is_ok());

        let mut mismatched = valid.clone();
        mismatched.questions[0].name = "attacker.example".parse().unwrap();
        assert!(validate(mismatched, 7, &question, false).is_err());

        let mut opcode = valid.clone();
        opcode.flags |= 1 << 11;
        assert!(validate(opcode, 7, &question, false).is_err());

        let mut truncated = valid;
        truncated.flags |= 0x0200;
        assert!(validate(truncated, 7, &question, true).is_err());
    }
}
```
