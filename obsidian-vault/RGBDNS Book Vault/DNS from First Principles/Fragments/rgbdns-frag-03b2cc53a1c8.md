---
type: "code-fragment"
fragment_id: "rgbdns-frag-03b2cc53a1c8"
source_path: "docs/book/rgbdns.md"
code_note: "DNS from First Principles/Code/docs/book/rgbdns.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "Specialized responders"
kind: "heading"
start_line: 741
end_line: 762
---

# Specialized responders

- Fragment ID: `rgbdns-frag-03b2cc53a1c8`
- Source file: [[DNS from First Principles/Code/docs/book/rgbdns.md.source|docs/book/rgbdns.md]]
- Lines: 741-762
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-03b2cc53a1c8", "codeNote": "DNS from First Principles/Code/docs/book/rgbdns.md.source", "heading": "rgbdns-frag-03b2cc53a1c8: heading Specialized responders", "sourcePath": "docs/book/rgbdns.md", "startLine": 741, "endLine": 762}
```

## Excerpt

<span id="rgbdns-frag-03b2cc53a1c8" class="rgbdns-fragment-target"></span>
### rgbdns-frag-03b2cc53a1c8: heading Specialized responders

```markdown
## Specialized responders

`rbldns` treats the labels before a configured suffix as a numeric address,
finds the most-specific matching IPv4 prefix in a compiled database, and
returns configured A/TXT data. Parsing caps the number of numeric labels and
validates networks before compilation.

`pickdns` maps client prefixes to two-byte locations and selects address sets
for that location. It shuffles eligible addresses with operating-system
randomness. Location-aware answers are a policy feature; clients behind shared
resolvers may appear at the resolver’s address, a limitation operators must
understand.

`walldns` synthesizes narrowly defined forward and reverse answers without a
zone database. These specialized services run through `src/special.rs`, which
provides shared bounded UDP/TCP serving and passes the peer address to the
handler.

The lesson is architectural: once parsing, transport, names, and record models
are sound, unusual DNS policies can be small pure response functions rather
than new monolithic servers.

```
