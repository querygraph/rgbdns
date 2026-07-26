---
type: "code-fragment"
fragment_id: "rgbdns-frag-a772e04cdd1d"
source_path: "docs/book/rgbdns.md"
code_note: "DNS from First Principles/Code/docs/book/rgbdns.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "Valid names instead of hopeful strings"
kind: "heading"
start_line: 1515
end_line: 1552
---

# Valid names instead of hopeful strings

- Fragment ID: `rgbdns-frag-a772e04cdd1d`
- Source file: [[DNS from First Principles/Code/docs/book/rgbdns.md.source|docs/book/rgbdns.md]]
- Lines: 1515-1552
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-a772e04cdd1d", "codeNote": "DNS from First Principles/Code/docs/book/rgbdns.md.source", "heading": "rgbdns-frag-a772e04cdd1d: heading Valid names instead of hopeful strings", "sourcePath": "docs/book/rgbdns.md", "startLine": 1515, "endLine": 1552}
```

## Excerpt

<span id="rgbdns-frag-a772e04cdd1d" class="rgbdns-fragment-target"></span>
### rgbdns-frag-a772e04cdd1d: heading Valid names instead of hopeful strings

```markdown
# Valid names instead of hopeful strings

[`Name`](https://github.com/querygraph/rgbdns/blob/master/src/name.rs) is the first load-bearing abstraction. It stores a
sequence of byte labels rather than a UTF-8 domain string. Its constructor is
private to the module; all construction passes through parsing or
`from_labels`, and both reach the same validation rule:

```rust
fn validate(labels: &[Vec<u8>]) -> Result<()> {
    if labels.iter().any(|l| l.is_empty() || l.len() > 63) {
        return Err(Error::InvalidName(
            "label must contain 1..=63 octets".into(),
        ));
    }
    let len = 1 + labels.iter().map(|l| l.len() + 1).sum::<usize>();
    if len > 255 {
        return Err(Error::InvalidName("wire name exceeds 255 octets".into()));
    }
    Ok(())
}
```

That small private function changes the rest of the codebase. `Zone` can use
`Name` as a `BTreeMap` key without rechecking label lengths. The packet writer
can calculate `wire_len` without wondering whether it will overflow the DNS
name limit. `parent`, `suffix`, `wildcard`, and `is_subdomain_of` operate on
labels rather than fragile dotted-string suffixes.

DNS identity is case-insensitive but responses should preserve the query’s
case. `Name` therefore retains original bytes while implementing `Eq`, `Hash`,
and `Ord` with ASCII-folded comparisons. In C this invariant must be remembered
by every hash table and comparison call site. Here it belongs to the key type.

This is a zero-surprise form of abstraction. It adds allocations when a name
is built, but it removes repeated parsing and validation later. The benchmarked
hot paths operate on already validated values, while the network boundary
absorbs the cost once.

```
