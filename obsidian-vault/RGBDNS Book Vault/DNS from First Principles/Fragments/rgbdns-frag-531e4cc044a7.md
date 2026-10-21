---
type: "code-fragment"
fragment_id: "rgbdns-frag-531e4cc044a7"
source_path: "docs/DNSSEC-DESIGN.md"
code_note: "DNS from First Principles/Code/docs/DNSSEC-DESIGN.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "DNSSEC design small tools and visible state"
kind: "heading"
start_line: 1
end_line: 13
---

# DNSSEC design small tools and visible state

- Fragment ID: `rgbdns-frag-531e4cc044a7`
- Source file: [[DNS from First Principles/Code/docs/DNSSEC-DESIGN.md.source|docs/DNSSEC-DESIGN.md]]
- Lines: 1-13
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-531e4cc044a7", "codeNote": "DNS from First Principles/Code/docs/DNSSEC-DESIGN.md.source", "sourcePath": "docs/DNSSEC-DESIGN.md", "startLine": 1, "endLine": 13}
```

## Excerpt

<span id="rgbdns-frag-531e4cc044a7" class="rgbdns-fragment-target"></span>
### rgbdns-frag-531e4cc044a7: heading DNSSEC design small tools and visible state

```markdown
# DNSSEC design: small tools and visible state

Status: implementation contract for rgbdns 0.6.5.

rgbdns DNSSEC follows the tinydns posture: each program does one bounded job,
configuration is one record per line, persistent state is an ordinary file, and
publication is an atomic rename. There is no signing daemon, key database, RPC
control plane, or implicit key generation in the authoritative server.

The cryptographic implementation may use Hickory's reviewed DNSSEC primitives.
That is an implementation detail. Hickory's configuration and authority server
are not part of the operator-facing design.

```
