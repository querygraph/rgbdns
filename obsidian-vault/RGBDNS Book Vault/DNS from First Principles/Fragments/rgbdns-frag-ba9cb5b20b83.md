---
type: "code-fragment"
fragment_id: "rgbdns-frag-ba9cb5b20b83"
source_path: "AGENTS.md"
code_note: "DNS from First Principles/Code/AGENTS.md.source"
language: "markdown"
subsystem: "Repository and build"
symbol: "ANAME and zone transfer"
kind: "heading"
start_line: 29
end_line: 55
---

# ANAME and zone transfer

- Fragment ID: `rgbdns-frag-ba9cb5b20b83`
- Source file: [[DNS from First Principles/Code/AGENTS.md.source|AGENTS.md]]
- Lines: 29-55
- Subsystem: [[DNS from First Principles/Subsystems/Repository and build|Repository and build]]

```rgbdns-fragment
{"id": "rgbdns-frag-ba9cb5b20b83", "codeNote": "DNS from First Principles/Code/AGENTS.md.source", "sourcePath": "AGENTS.md", "startLine": 29, "endLine": 55}
```

## Excerpt

<span id="rgbdns-frag-ba9cb5b20b83" class="rgbdns-fragment-target"></span>
### rgbdns-frag-ba9cb5b20b83: heading ANAME and zone transfer

```markdown
## ANAME and zone transfer

- rgbdns source uses the private tinydns directive
  `Aowner.example:target.example:ttl-cap`.
- rgbdns peers preserve that directive through an explicitly negotiated
  experimental AXFR extension:
  - EDNS option `65001`;
  - capability token `RGA1`;
  - private-use RR TYPE `65401`;
  - RDATA `RGA1` followed by the uncompressed target DNS name;
  - the private RR TTL carries the configured ANAME ceiling.
- TYPE65401 collides with PowerDNS's incompatible private ALIAS encoding.
  Never infer semantics from TYPE65401 without the `RGA1` negotiation and
  payload validation.
- Standard AXFR clients receive no rgbdns private ANAME metadata. BuddyNS and
  other ordinary secondaries cannot reproduce an ANAME-backed zone. Delegate
  such zones only to upgraded rgbdns authorities unless the zone uses standard
  A/AAAA records.
- The published Internet-Draft requests IANA-assigned ANAME and EDNS values as
  `TBD1` and `TBD2`. Do not deploy guessed values. Keep the experimental
  protocol until assignments exist and a versioned migration is implemented.
- Upgrade both primary and secondary for ANAME transfer changes. The secondary
  resolves transferred ANAME targets independently.
- ANAME target lookups are bounded, concurrent misses are coalesced, and
  failures are briefly suppressed to limit recursive retry and cross-provider
  loop amplification.

```
