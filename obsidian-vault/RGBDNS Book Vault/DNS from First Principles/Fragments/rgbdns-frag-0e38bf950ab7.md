---
type: "code-fragment"
fragment_id: "rgbdns-frag-0e38bf950ab7"
source_path: "docs/book/rgbdns.md"
code_note: "DNS from First Principles/Code/docs/book/rgbdns.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "Why there are two transports"
kind: "heading"
start_line: 323
end_line: 339
---

# Why there are two transports

- Fragment ID: `rgbdns-frag-0e38bf950ab7`
- Source file: [[DNS from First Principles/Code/docs/book/rgbdns.md.source|docs/book/rgbdns.md]]
- Lines: 323-339
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-0e38bf950ab7", "codeNote": "DNS from First Principles/Code/docs/book/rgbdns.md.source", "heading": "rgbdns-frag-0e38bf950ab7: heading Why there are two transports", "sourcePath": "docs/book/rgbdns.md", "startLine": 323, "endLine": 339}
```

## Excerpt

<span id="rgbdns-frag-0e38bf950ab7" class="rgbdns-fragment-target"></span>
### rgbdns-frag-0e38bf950ab7: heading Why there are two transports

```markdown
## Why there are two transports

Classic DNS uses UDP for ordinary queries because one request and one response
need no connection setup. Traditional UDP DNS assumed a 512-byte message.
Larger answers set TC, telling the client to retry over TCP. TCP frames every
DNS message with a two-byte length.

Zone transfers use TCP. Modern responses—especially DNSSEC responses—often
need more than 512 bytes, so EDNS lets a client advertise a larger UDP receive
size through an OPT pseudo-record. Internet paths can still drop fragmented
UDP packets. A commonly conservative payload is 1232 bytes, large enough for
useful DNSSEC answers while fitting the IPv6 minimum MTU without fragmentation
under normal headers.

TCP is not merely an emergency protocol. Firewalls that assume DNS is
UDP-only break standards-compliant resolution.

```
