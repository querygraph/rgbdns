---
type: "code-fragment"
fragment_id: "rgbdns-frag-f73fe576d5fc"
source_path: "docs/blog/announcing-rgbdns/post.md"
code_note: "DNS from First Principles/Code/docs/blog/announcing-rgbdns/post.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "Compatibility without inherited memory hazards"
kind: "heading"
start_line: 43
end_line: 60
---

# Compatibility without inherited memory hazards

- Fragment ID: `rgbdns-frag-f73fe576d5fc`
- Source file: [[DNS from First Principles/Code/docs/blog/announcing-rgbdns/post.md.source|docs/blog/announcing-rgbdns/post.md]]
- Lines: 43-60
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-f73fe576d5fc", "codeNote": "DNS from First Principles/Code/docs/blog/announcing-rgbdns/post.md.source", "heading": "rgbdns-frag-f73fe576d5fc: heading Compatibility without inherited memory hazards", "sourcePath": "docs/blog/announcing-rgbdns/post.md", "startLine": 43, "endLine": 60}
```

## Excerpt

<span id="rgbdns-frag-f73fe576d5fc" class="rgbdns-fragment-target"></span>
### rgbdns-frag-f73fe576d5fc: heading Compatibility without inherited memory hazards

```markdown
## Compatibility without inherited memory hazards

rgbdns reads the tinydns source language and the original `data.cdb` key/value
layout. It supports the familiar record markers, location-aware answers,
query-time TAI64 activation and expiration, patched IPv6 and SRV forms, and
the original suite's service-directory conventions.

Compatibility is checked rather than assumed. Golden fixtures compare compiled
CDB entries with the patched C implementation. Tests retain the awkward parts:
field positions, default target expansion, escaped colons, octal text, SOA TTL
rules, wildcard behavior, client-location selection, IPv6 reverse trees, and
SRV ordering.

The compatibility boundary is deliberately narrow. Historical files are
accepted as hostile input, parsed into validated Rust types, and served through
bounded code. The implementation preserves the useful contract without
preserving the old trust assumptions.

```
