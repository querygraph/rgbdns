---
type: "code-fragment"
fragment_id: "rgbdns-frag-151195f92fbd"
source_path: "RGBDNS_LETS.md"
code_note: "DNS from First Principles/Code/RGBDNS_LETS.md.source"
language: "markdown"
subsystem: "Repository and build"
symbol: "Goals"
kind: "heading"
start_line: 15
end_line: 31
---

# Goals

- Fragment ID: `rgbdns-frag-151195f92fbd`
- Source file: [[DNS from First Principles/Code/RGBDNS_LETS.md.source|RGBDNS_LETS.md]]
- Lines: 15-31
- Subsystem: [[DNS from First Principles/Subsystems/Repository and build|Repository and build]]

```rgbdns-fragment
{"id": "rgbdns-frag-151195f92fbd", "codeNote": "DNS from First Principles/Code/RGBDNS_LETS.md.source", "sourcePath": "RGBDNS_LETS.md", "startLine": 15, "endLine": 31}
```

## Excerpt

<span id="rgbdns-frag-151195f92fbd" class="rgbdns-fragment-target"></span>
### rgbdns-frag-151195f92fbd: heading Goals

```markdown
## Goals

- Accept standards-based RFC 2136 DNS UPDATE messages from existing ACME
  clients.
- Authenticate every accepted update with a TSIG HMAC-SHA256 key.
- Authorize a key only for configured `_acme-challenge` owners in configured
  primary zones.
- Add and remove individual TXT values without destroying concurrent
  challenges at the same owner.
- Make a successful update visible on the primary before returning success.
- Persist accepted state across daemon restart and canonical zone imports.
- Include dynamic challenge records and the advanced SOA serial in AXFR so
  secondaries converge normally.
- Preserve the existing atomic `rgbdns.data` import and last-known-good
  behavior.
- Bound packet, state, credential, name, value, and concurrency resource use.

```
