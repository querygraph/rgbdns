---
type: "code-fragment"
fragment_id: "rgbdns-frag-2a00ac03ab08"
source_path: "docs/blog/rgbdns-dnssec/post.md"
code_note: "DNS from First Principles/Code/docs/blog/rgbdns-dnssec/post.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "Secondaries carry signatures, not keys"
kind: "heading"
start_line: 94
end_line: 116
---

# Secondaries carry signatures, not keys

- Fragment ID: `rgbdns-frag-2a00ac03ab08`
- Source file: [[DNS from First Principles/Code/docs/blog/rgbdns-dnssec/post.md.source|docs/blog/rgbdns-dnssec/post.md]]
- Lines: 94-116
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-2a00ac03ab08", "codeNote": "DNS from First Principles/Code/docs/blog/rgbdns-dnssec/post.md.source", "sourcePath": "docs/blog/rgbdns-dnssec/post.md", "startLine": 94, "endLine": 116}
```

## Excerpt

<span id="rgbdns-frag-2a00ac03ab08" class="rgbdns-fragment-target"></span>
### rgbdns-frag-2a00ac03ab08: heading Secondaries carry signatures, not keys

````markdown
## Secondaries carry signatures, not keys

DNSKEY, RRSIG, NSEC, and DS are ordinary DNS records on the wire. AXFR carries
them to a secondary exactly as it carries A, MX, or TXT. The secondary validates
the transfer, compiles the finished zone, and serves it without signing
anything.

```sh
sudo rgbdns-setup secondary \
  --zone example.com \
  --primary 10.0.1.10 \
  --listen-ip 0.0.0.0
```

This is a useful compromise boundary. A serving secondary may be geographically
and administratively independent, but compromise of that server does not reveal
the key that can create new signed data.

It also means the pre-DS gate is simple to state: every nameserver in the public
delegation must serve the same SOA serial, DNSKEY, positive signatures, and
authenticated negative answers. A stale ordinary secondary is not harmless
after the parent starts promising DNSSEC.

````
