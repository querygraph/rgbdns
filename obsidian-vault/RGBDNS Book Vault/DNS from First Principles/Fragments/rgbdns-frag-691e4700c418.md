---
type: "code-fragment"
fragment_id: "rgbdns-frag-691e4700c418"
source_path: "RGBDNS_LETS.md"
code_note: "DNS from First Principles/Code/RGBDNS_LETS.md.source"
language: "markdown"
subsystem: "Repository and build"
symbol: "Certbot interoperability"
kind: "heading"
start_line: 215
end_line: 231
---

# Certbot interoperability

- Fragment ID: `rgbdns-frag-691e4700c418`
- Source file: [[DNS from First Principles/Code/RGBDNS_LETS.md.source|RGBDNS_LETS.md]]
- Lines: 215-231
- Subsystem: [[DNS from First Principles/Subsystems/Repository and build|Repository and build]]

```rgbdns-fragment
{"id": "rgbdns-frag-691e4700c418", "codeNote": "DNS from First Principles/Code/RGBDNS_LETS.md.source", "sourcePath": "RGBDNS_LETS.md", "startLine": 215, "endLine": 231}
```

## Excerpt

<span id="rgbdns-frag-691e4700c418" class="rgbdns-fragment-target"></span>
### rgbdns-frag-691e4700c418: heading Certbot interoperability

```markdown
## Certbot interoperability

An operator creates a separate key per certificate host or administrative
boundary and configures Certbot's RFC 2136 plugin with the primary address,
port 53, key name, HMAC-SHA256 secret, and algorithm. The credentials file must
not be world-readable. Renewal uses DNS-01 and an explicit propagation delay
long enough for every authoritative secondary.

Operators may instead delegate `_acme-challenge` with CNAME or NS records to a
small rgbdns validation zone. This limits update authority and follows the
DNS-01 delegation behavior documented by Let's Encrypt.

The integration test uses an RFC 2136 client to add a staging-shaped token,
queries UDP and TCP authority, transfers the zone, restarts the daemon, imports
new canonical data, removes only that token, and confirms unrelated/concurrent
TXT values remain.

```
