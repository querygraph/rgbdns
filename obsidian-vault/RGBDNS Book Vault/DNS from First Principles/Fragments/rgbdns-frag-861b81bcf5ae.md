---
type: "code-fragment"
fragment_id: "rgbdns-frag-861b81bcf5ae"
source_path: "docs/book/rgbdns.md"
code_note: "DNS from First Principles/Code/docs/book/rgbdns.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "Requirements that pull in different directions"
kind: "heading"
start_line: 54
end_line: 72
---

# Requirements that pull in different directions

- Fragment ID: `rgbdns-frag-861b81bcf5ae`
- Source file: [[DNS from First Principles/Code/docs/book/rgbdns.md.source|docs/book/rgbdns.md]]
- Lines: 54-72
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-861b81bcf5ae", "codeNote": "DNS from First Principles/Code/docs/book/rgbdns.md.source", "heading": "rgbdns-frag-861b81bcf5ae: heading Requirements that pull in different directions", "sourcePath": "docs/book/rgbdns.md", "startLine": 54, "endLine": 72}
```

## Excerpt

<span id="rgbdns-frag-861b81bcf5ae" class="rgbdns-fragment-target"></span>
### rgbdns-frag-861b81bcf5ae: heading Requirements that pull in different directions

```markdown
## Requirements that pull in different directions

A global naming system must satisfy conflicting demands:

- It must scale without one central database receiving every query.
- Different organizations must control different parts of the namespace.
- Changes must propagate, but cached answers are essential for performance.
- Replies should usually fit in one datagram, but some answers are large.
- Old implementations must coexist with protocol extensions.
- A client needs to distinguish “no such name” from “that name has no record
  of this type.”
- Operators need a way to transfer complete zones and to diagnose individual
  exchanges.

DNS answers these demands with hierarchy, delegation, caching lifetimes,
compact binary messages, UDP plus TCP, explicit result codes, and typed
records. Many operational surprises are direct consequences of those design
choices rather than random quirks.

```
