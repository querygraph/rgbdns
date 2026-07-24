---
type: "code-fragment"
fragment_id: "rgbdns-frag-b05f56fdf049"
source_path: "tests/wire_security.rs"
code_note: "DNS from First Principles/Code/tests/wire_security.rs.source"
language: "rust"
subsystem: "Tests and performance"
symbol: "malformed_wire_corpus_is_rejected_without_partial_acceptance"
kind: "fn"
start_line: 34
end_line: 70
---

# malformed_wire_corpus_is_rejected_without_partial_acceptance

- Fragment ID: `rgbdns-frag-b05f56fdf049`
- Source file: [[DNS from First Principles/Code/tests/wire_security.rs.source|tests/wire_security.rs]]
- Lines: 34-70
- Subsystem: [[DNS from First Principles/Subsystems/Tests and performance|Tests and performance]]

```rgbdns-fragment
{"id": "rgbdns-frag-b05f56fdf049", "codeNote": "DNS from First Principles/Code/tests/wire_security.rs.source", "heading": "rgbdns-frag-b05f56fdf049: fn malformed_wire_corpus_is_rejected_without_partial_acceptance", "sourcePath": "tests/wire_security.rs", "startLine": 34, "endLine": 70}
```

## Excerpt

<span id="rgbdns-frag-b05f56fdf049" class="rgbdns-fragment-target"></span>
### rgbdns-frag-b05f56fdf049: fn malformed_wire_corpus_is_rejected_without_partial_acceptance

```rust
fn malformed_wire_corpus_is_rejected_without_partial_acceptance() {
    let mut count_bomb = standard_header(65, 0, 0, 0);
    let mut reserved_label = standard_header(1, 0, 0, 0);
    reserved_label.extend([0x40, 0, 1, 0, 1]);
    let mut truncated_pointer = standard_header(1, 0, 0, 0);
    truncated_pointer.push(0xc0);
    let mut forward_pointer = standard_header(1, 0, 0, 0);
    forward_pointer.extend([0xc0, 14, 0, 1, 0, 1]);
    let mut invalid_a_length = standard_header(0, 1, 0, 0);
    invalid_a_length.extend([0, 0, 1, 0, 1, 0, 0, 0, 1, 0, 3, 1, 2, 3]);
    let mut truncated_txt = standard_header(0, 1, 0, 0);
    truncated_txt.extend([0, 0, 16, 0, 1, 0, 0, 0, 1, 0, 2, 4, b'x']);
    let mut bad_opt_owner = standard_header(0, 0, 0, 1);
    bad_opt_owner.extend([1, b'x', 0, 0, 41, 4, 208, 0, 0, 0, 0, 0, 0]);
    let mut truncated_option = standard_header(0, 0, 0, 1);
    truncated_option.extend([0, 0, 41, 4, 208, 0, 0, 0, 0, 0, 5, 0, 1, 0, 2, 0]);
    let mut trailing = standard_header(0, 0, 0, 0);
    trailing.push(0);
    count_bomb.extend([0; 5]);

    for (name, wire) in [
        ("short header", vec![0; 11]),
        ("section count bomb", count_bomb),
        ("reserved label", reserved_label),
        ("truncated pointer", truncated_pointer),
        ("forward pointer", forward_pointer),
        ("invalid A length", invalid_a_length),
        ("truncated TXT chunk", truncated_txt),
        ("non-root OPT owner", bad_opt_owner),
        ("truncated EDNS option", truncated_option),
        ("trailing bytes", trailing),
    ] {
        assert!(Message::decode(&wire).is_err(), "{name}");
    }
}

#[test]
```
