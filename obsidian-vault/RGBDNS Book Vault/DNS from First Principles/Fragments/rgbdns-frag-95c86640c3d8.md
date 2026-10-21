---
type: "code-fragment"
fragment_id: "rgbdns-frag-95c86640c3d8"
source_path: "RGBDNS_LETS.md"
code_note: "DNS from First Principles/Code/RGBDNS_LETS.md.source"
language: "markdown"
subsystem: "Repository and build"
symbol: "Status"
kind: "heading"
start_line: 3
end_line: 14
---

# Status

- Fragment ID: `rgbdns-frag-95c86640c3d8`
- Source file: [[DNS from First Principles/Code/RGBDNS_LETS.md.source|RGBDNS_LETS.md]]
- Lines: 3-14
- Subsystem: [[DNS from First Principles/Subsystems/Repository and build|Repository and build]]

```rgbdns-fragment
{"id": "rgbdns-frag-95c86640c3d8", "codeNote": "DNS from First Principles/Code/RGBDNS_LETS.md.source", "sourcePath": "RGBDNS_LETS.md", "startLine": 3, "endLine": 14}
```

## Excerpt

<span id="rgbdns-frag-95c86640c3d8" class="rgbdns-fragment-target"></span>
### rgbdns-frag-95c86640c3d8: heading Status

```markdown
## Status

This document specifies the rgbdns 0.4.0 ACME DNS-01 update feature. It is the
implementation contract for authenticated dynamic TXT updates, durable ACME
state, atomic publication, zone transfer, packaging, and interoperability with
RFC 2136-capable ACME clients such as Certbot.

rgbdns does not implement the ACME protocol, request certificates, store
private certificate keys, or terminate TLS. An ACME client performs those
jobs. rgbdns provides the narrowly scoped authoritative DNS mutation needed to
answer DNS-01 challenges.

```
