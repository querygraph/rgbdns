---
type: "code-fragment"
fragment_id: "rgbdns-frag-bcdc357e6f0e"
source_path: "docs/book/rgbdns.md"
code_note: "DNS from First Principles/Code/docs/book/rgbdns.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "Specialized responders"
kind: "heading"
start_line: 606
end_line: 627
---

# Specialized responders

- Fragment ID: `rgbdns-frag-bcdc357e6f0e`
- Source file: [[DNS from First Principles/Code/docs/book/rgbdns.md.source|docs/book/rgbdns.md]]
- Lines: 606-627
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-bcdc357e6f0e", "codeNote": "DNS from First Principles/Code/docs/book/rgbdns.md.source", "heading": "rgbdns-frag-bcdc357e6f0e: heading Specialized responders", "sourcePath": "docs/book/rgbdns.md", "startLine": 606, "endLine": 627}
```

## Excerpt

<span id="rgbdns-frag-bcdc357e6f0e" class="rgbdns-fragment-target"></span>
### rgbdns-frag-bcdc357e6f0e: heading Specialized responders

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
