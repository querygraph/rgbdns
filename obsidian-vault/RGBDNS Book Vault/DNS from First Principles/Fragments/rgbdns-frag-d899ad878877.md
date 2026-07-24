---
type: "code-fragment"
fragment_id: "rgbdns-frag-d899ad878877"
source_path: "tests/packet_properties.rs"
code_note: "DNS from First Principles/Code/tests/packet_properties.rs.source"
language: "rust"
subsystem: "Tests and performance"
symbol: "ascii_case_changes_do_not_change_name_identity"
kind: "fn"
start_line: 79
end_line: 103
---

# ascii_case_changes_do_not_change_name_identity

- Fragment ID: `rgbdns-frag-d899ad878877`
- Source file: [[DNS from First Principles/Code/tests/packet_properties.rs.source|tests/packet_properties.rs]]
- Lines: 79-103
- Subsystem: [[DNS from First Principles/Subsystems/Tests and performance|Tests and performance]]

```rgbdns-fragment
{"id": "rgbdns-frag-d899ad878877", "codeNote": "DNS from First Principles/Code/tests/packet_properties.rs.source", "heading": "rgbdns-frag-d899ad878877: fn ascii_case_changes_do_not_change_name_identity", "sourcePath": "tests/packet_properties.rs", "startLine": 79, "endLine": 103}
```

## Excerpt

<span id="rgbdns-frag-d899ad878877" class="rgbdns-fragment-target"></span>
### rgbdns-frag-d899ad878877: fn ascii_case_changes_do_not_change_name_identity

```rust
    fn ascii_case_changes_do_not_change_name_identity(
        labels in prop::collection::vec("[a-z]{1,20}", 1..=8),
        choices in prop::collection::vec(any::<bool>(), 1..=160),
    ) {
        let lower = labels.join(".");
        let mut index = 0;
        let mixed = lower
            .bytes()
            .map(|byte| {
                if byte.is_ascii_alphabetic() {
                    let upper = choices[index % choices.len()];
                    index += 1;
                    if upper { byte.to_ascii_uppercase() } else { byte }
                } else {
                    byte
                }
            })
            .collect::<Vec<_>>();
        let mixed = String::from_utf8(mixed).unwrap();
        prop_assert_eq!(
            lower.parse::<rgbdns::Name>().unwrap(),
            mixed.parse::<rgbdns::Name>().unwrap()
        );
    }
}
```
