---
type: "code-fragment"
fragment_id: "rgbdns-frag-db65ba809c6a"
source_path: "docs/book/rgbdns.md"
code_note: "DNS from First Principles/Code/docs/book/rgbdns.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "Layers of evidence"
kind: "heading"
start_line: 1222
end_line: 1243
---

# Layers of evidence

- Fragment ID: `rgbdns-frag-db65ba809c6a`
- Source file: [[DNS from First Principles/Code/docs/book/rgbdns.md.source|docs/book/rgbdns.md]]
- Lines: 1222-1243
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-db65ba809c6a", "codeNote": "DNS from First Principles/Code/docs/book/rgbdns.md.source", "heading": "rgbdns-frag-db65ba809c6a: heading Layers of evidence", "sourcePath": "docs/book/rgbdns.md", "startLine": 1222, "endLine": 1243}
```

## Excerpt

<span id="rgbdns-frag-db65ba809c6a" class="rgbdns-fragment-target"></span>
### rgbdns-frag-db65ba809c6a: heading Layers of evidence

```markdown
## Layers of evidence

Unit tests establish local invariants: name limits, record parsing, lookup
outcomes, leap conversion. Property tests explore parser state spaces that
examples miss. Golden fixtures establish compatibility with an external file
format. Integration tests cross process and socket boundaries. Live
interoperability tests compare behavior with independent clients and servers.

rgbdns uses all of these. A useful local sequence is:

```sh
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
```

Network tests bind unprivileged loopback ports. CDB tests compile canonical
fixtures and compare entries. Daemontools tests exercise process replacement,
rotation, and TAI64 filter behavior. Packet properties assert that arbitrary
input does not panic and that supported structured messages survive
encode/decode round trips.

```
