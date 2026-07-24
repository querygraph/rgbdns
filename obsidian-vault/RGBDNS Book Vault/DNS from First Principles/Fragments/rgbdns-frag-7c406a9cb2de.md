---
type: "code-fragment"
fragment_id: "rgbdns-frag-7c406a9cb2de"
source_path: "docs/blog/announcing-rgbdns/post.md"
code_note: "DNS from First Principles/Code/docs/blog/announcing-rgbdns/post.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "Conformance is a test matrix"
kind: "heading"
start_line: 61
end_line: 90
---

# Conformance is a test matrix

- Fragment ID: `rgbdns-frag-7c406a9cb2de`
- Source file: [[DNS from First Principles/Code/docs/blog/announcing-rgbdns/post.md.source|docs/blog/announcing-rgbdns/post.md]]
- Lines: 61-90
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-7c406a9cb2de", "codeNote": "DNS from First Principles/Code/docs/blog/announcing-rgbdns/post.md.source", "heading": "rgbdns-frag-7c406a9cb2de: heading Conformance is a test matrix", "sourcePath": "docs/blog/announcing-rgbdns/post.md", "startLine": 61, "endLine": 90}
```

## Excerpt

<span id="rgbdns-frag-7c406a9cb2de" class="rgbdns-fragment-target"></span>
### rgbdns-frag-7c406a9cb2de: heading Conformance is a test matrix

```markdown
## Conformance is a test matrix

“RFC compliant” is not a useful finish line unless each claim has an oracle.
rgbdns now carries a traceable conformance matrix covering the implemented
surface of RFC 1035, 2181, 2308, 3597, 4343, 4592, 5936, 6891, 7766, 8482,
8906, and 9619.

The tests distinguish details that disappear in broad success/failure checks:

- NXDOMAIN from NODATA, with the correct negative SOA lifetime;
- an unknown record type from an unknown opcode;
- legal unknown EDNS options from malformed option framing;
- a backward compression pointer from a pointer into arbitrary earlier bytes;
- a wildcard synthesis point from an existing empty non-terminal;
- a referral from an authoritative answer, with only in-bailiwick glue;
- an AXFR stream with matching SOA bookends from a plausible but unrelated
  response;
- a truncated UDP message from a complete persistent TCP response.

The RFC cases are joined by hostile-wire corpora and forty thousand generated
property cases per full run. Arbitrary packets must never panic. Every accepted
packet must reparse stably. Structured messages must survive semantic
round-trips. Changing ASCII letter case must not change DNS name identity.
Every truncated prefix of a valid structured packet must be rejected.

An independent ldns `drill` test launches the real `tinydns` binary and makes
UDP, TCP, EDNS, mixed-case, and unknown-type requests. That boundary matters:
rgbdns cannot accidentally pass by making the same mistake in its own encoder
and decoder.

```
