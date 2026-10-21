---
type: "code-fragment"
fragment_id: "rgbdns-frag-d977e98143ac"
source_path: "RGBDNS_LETS.md"
code_note: "DNS from First Principles/Code/RGBDNS_LETS.md.source"
language: "markdown"
subsystem: "Repository and build"
symbol: "Non-goals"
kind: "heading"
start_line: 32
end_line: 44
---

# Non-goals

- Fragment ID: `rgbdns-frag-d977e98143ac`
- Source file: [[DNS from First Principles/Code/RGBDNS_LETS.md.source|RGBDNS_LETS.md]]
- Lines: 32-44
- Subsystem: [[DNS from First Principles/Subsystems/Repository and build|Repository and build]]

```rgbdns-fragment
{"id": "rgbdns-frag-d977e98143ac", "codeNote": "DNS from First Principles/Code/RGBDNS_LETS.md.source", "sourcePath": "RGBDNS_LETS.md", "startLine": 32, "endLine": 44}
```

## Excerpt

<span id="rgbdns-frag-d977e98143ac" class="rgbdns-fragment-target"></span>
### rgbdns-frag-d977e98143ac: heading Non-goals

```markdown
## Non-goals

- General-purpose dynamic DNS.
- UPDATE of A, AAAA, ANAME, CNAME, NS, SOA, MX, CAA, DNSSEC, or arbitrary
  record types.
- UPDATE forwarding by a secondary.
- Certificate issuance, renewal scheduling, deployment, or TLS service.
- TSIG algorithms other than HMAC-SHA256 in 0.4.0.
- DNS UPDATE over UDP in 0.4.0. TCP avoids truncation and amplification and is
  supported by the intended RFC 2136 clients.
- Arbitrary RFC 2136 prerequisites. The limited prerequisite forms required by
  common clients are validated; unsupported forms receive NOTIMP.

```
