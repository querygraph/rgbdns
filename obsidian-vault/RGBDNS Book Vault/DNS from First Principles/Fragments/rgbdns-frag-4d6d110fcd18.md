---
type: "code-fragment"
fragment_id: "rgbdns-frag-4d6d110fcd18"
source_path: "docs/DNSSEC.md"
code_note: "DNS from First Principles/Code/docs/DNSSEC.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "ANAME and ACME"
kind: "heading"
start_line: 102
end_line: 120
---

# ANAME and ACME

- Fragment ID: `rgbdns-frag-4d6d110fcd18`
- Source file: [[DNS from First Principles/Code/docs/DNSSEC.md.source|docs/DNSSEC.md]]
- Lines: 102-120
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-4d6d110fcd18", "codeNote": "DNS from First Principles/Code/docs/DNSSEC.md.source", "sourcePath": "docs/DNSSEC.md", "startLine": 102, "endLine": 120}
```

## Excerpt

<span id="rgbdns-frag-4d6d110fcd18" class="rgbdns-fragment-target"></span>
### rgbdns-frag-4d6d110fcd18: heading ANAME and ACME

```markdown
## ANAME and ACME

A signed answer cannot be synthesized after signing. `aname-materialize`
therefore resolves private `Aowner:target:ttl-cap` directives in `K` zones and
writes ordinary A/AAAA records with capped TTLs. It preserves the directives in
`U` zones, which continue using the original runtime resolver. The scheduled
publisher refreshes signed materializations before signatures expire. Signed
AXFR contains the materialized addresses and signatures, not a requirement for
the secondary to resolve the target.

For ACME, delegation of `_acme-challenge` to a small unsigned validation zone
is simplest. An ACME-managed `U` zone remains on the unprivileged live-overlay
path even when another zone in the snapshot is signed; periodic root
publication incorporates its durable overlay without invoking the signer from
`tinydns`. Inline updates to a `K` zone require an explicitly configured
privileged publication command. `rgbdns-setup` does not synthesize that trust
boundary. Without such a hook, a signed ACME zone refuses startup rather than
publishing unsigned TXT.

```
