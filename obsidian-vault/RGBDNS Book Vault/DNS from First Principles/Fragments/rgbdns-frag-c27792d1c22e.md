---
type: "code-fragment"
fragment_id: "rgbdns-frag-c27792d1c22e"
source_path: "docs/blog/announcing-rgbdns/post.md"
code_note: "DNS from First Principles/Code/docs/blog/announcing-rgbdns/post.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "Hardening that came from the tests"
kind: "heading"
start_line: 91
end_line: 110
---

# Hardening that came from the tests

- Fragment ID: `rgbdns-frag-c27792d1c22e`
- Source file: [[DNS from First Principles/Code/docs/blog/announcing-rgbdns/post.md.source|docs/blog/announcing-rgbdns/post.md]]
- Lines: 91-110
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-c27792d1c22e", "codeNote": "DNS from First Principles/Code/docs/blog/announcing-rgbdns/post.md.source", "heading": "rgbdns-frag-c27792d1c22e: heading Hardening that came from the tests", "sourcePath": "docs/blog/announcing-rgbdns/post.md", "startLine": 91, "endLine": 110}
```

## Excerpt

<span id="rgbdns-frag-c27792d1c22e" class="rgbdns-fragment-target"></span>
### rgbdns-frag-c27792d1c22e: heading Hardening that came from the tests

```markdown
## Hardening that came from the tests

The conformance work changed the design.

The decoder now records valid prior name boundaries, so compression pointers
must target an actual earlier name occurrence. Stub replies are bound to their
request ID, response bit, opcode, and exact question. AXFR adds authoritative
and truncation checks, question rules, zone confinement, and matching opening
and closing SOAs.

Zone loading rejects a CNAME that coexists with other data or points at
multiple different targets. RRsets are normalized to their minimum TTL and
deduplicated before transmission. EDNS placement, count, version, payload, and
option framing are checked explicitly.

UDP and TCP servers share one bounded transport implementation. TCP uses a
fixed worker pool, per-connection deadlines, persistent framing, and pipelined
queries. The result is less duplicated socket code and a stronger common
service contract.

```
