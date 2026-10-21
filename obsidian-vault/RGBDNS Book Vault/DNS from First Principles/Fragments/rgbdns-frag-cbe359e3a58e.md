---
type: "code-fragment"
fragment_id: "rgbdns-frag-cbe359e3a58e"
source_path: "docs/book/rgbdns.md"
code_note: "DNS from First Principles/Code/docs/book/rgbdns.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "Layers of evidence"
kind: "heading"
start_line: 2154
end_line: 2175
---

# Layers of evidence

- Fragment ID: `rgbdns-frag-cbe359e3a58e`
- Source file: [[DNS from First Principles/Code/docs/book/rgbdns.md.source|docs/book/rgbdns.md]]
- Lines: 2154-2175
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-cbe359e3a58e", "codeNote": "DNS from First Principles/Code/docs/book/rgbdns.md.source", "sourcePath": "docs/book/rgbdns.md", "startLine": 2154, "endLine": 2175}
```

## Excerpt

<span id="rgbdns-frag-cbe359e3a58e" class="rgbdns-fragment-target"></span>
### rgbdns-frag-cbe359e3a58e: heading Layers of evidence

````markdown
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

````
