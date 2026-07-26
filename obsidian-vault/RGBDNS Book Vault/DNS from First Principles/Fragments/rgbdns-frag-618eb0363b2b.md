---
type: "code-fragment"
fragment_id: "rgbdns-frag-618eb0363b2b"
source_path: "docs/book/rgbdns.md"
code_note: "DNS from First Principles/Code/docs/book/rgbdns.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "Hardening found by conformance work"
kind: "heading"
start_line: 1324
end_line: 1353
---

# Hardening found by conformance work

- Fragment ID: `rgbdns-frag-618eb0363b2b`
- Source file: [[DNS from First Principles/Code/docs/book/rgbdns.md.source|docs/book/rgbdns.md]]
- Lines: 1324-1353
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-618eb0363b2b", "codeNote": "DNS from First Principles/Code/docs/book/rgbdns.md.source", "heading": "rgbdns-frag-618eb0363b2b: heading Hardening found by conformance work", "sourcePath": "docs/book/rgbdns.md", "startLine": 1324, "endLine": 1353}
```

## Excerpt

<span id="rgbdns-frag-618eb0363b2b" class="rgbdns-fragment-target"></span>
### rgbdns-frag-618eb0363b2b: heading Hardening found by conformance work

```markdown
## Hardening found by conformance work

Conformance testing improved the implementation rather than merely describing
it.

The name decoder now records valid prior name boundaries. A compression
pointer must be backward *and* must target one of those boundaries. Merely
pointing at earlier bytes that happen to resemble a label sequence is rejected.
This closes a class of ambiguous parses without forbidding legal compression.

Stub responses are bound to the request ID, QR bit, opcode, and exact question.
TCP responses carrying TC are rejected. AXFR applies the same identity checks
and additionally requires authoritative, non-truncated messages, controlled
question repetition, an empty authority section, matching opening and closing
SOAs, and records confined to the requested zone. These rules prevent a
plausible-looking but unrelated response from being accepted as the answer to
the outstanding operation.

Zone loading rejects a CNAME owner that also has other data and rejects
multiple different CNAME targets. Before transmission, RRsets are normalized
to their minimum TTL and duplicate records are removed. Negative answers cap
the SOA TTL at the SOA MINIMUM field as RFC 2308 requires. EDNS OPT records in
the wrong section and duplicate OPT records produce FORMERR.

The UDP and TCP daemons now share one bounded transport module. TCP connections
carry deadlines, use a fixed worker pool, accept multiple framed queries, and
support pipelined requests. This removes duplicated socket code while making
RFC 7766 behavior an invariant shared by the authoritative and specialized
servers.

```
