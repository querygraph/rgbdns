---
type: "code-fragment"
fragment_id: "rgbdns-frag-7af30d62d5e0"
source_path: "docs/DNSSEC.md"
code_note: "DNS from First Principles/Code/docs/DNSSEC.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "Optional authoritative DNSSEC"
kind: "heading"
start_line: 1
end_line: 12
---

# Optional authoritative DNSSEC

- Fragment ID: `rgbdns-frag-7af30d62d5e0`
- Source file: [[DNS from First Principles/Code/docs/DNSSEC.md.source|docs/DNSSEC.md]]
- Lines: 1-12
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-7af30d62d5e0", "codeNote": "DNS from First Principles/Code/docs/DNSSEC.md.source", "sourcePath": "docs/DNSSEC.md", "startLine": 1, "endLine": 12}
```

## Excerpt

<span id="rgbdns-frag-7af30d62d5e0" class="rgbdns-fragment-target"></span>
### rgbdns-frag-7af30d62d5e0: heading Optional authoritative DNSSEC

```markdown
# Optional authoritative DNSSEC

rgbdns 0.6.5 adds authoritative DNSSEC as an explicit offline publication
pipeline. It is disabled by default. If `/etc/rgbdns/dnssec.env` and the
working-directory `dnssec` policy are absent, `tinydns-data`, `tinydns`, ACME,
ANAME, setup, and secondary synchronization follow the existing
djbdns-compatible path.

The authoritative server never opens a private key. It only serves DNSKEY,
RRSIG, NSEC, and DS records already present in its CDB. Secondaries receive the
same finished records over ordinary AXFR and also need no key.

```
