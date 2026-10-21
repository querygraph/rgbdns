---
type: "code-fragment"
fragment_id: "rgbdns-frag-810e23c0f3de"
source_path: "docs/DNSSEC-DESIGN.md"
code_note: "DNS from First Principles/Code/docs/DNSSEC-DESIGN.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "Failure and observability"
kind: "heading"
start_line: 164
end_line: 173
---

# Failure and observability

- Fragment ID: `rgbdns-frag-810e23c0f3de`
- Source file: [[DNS from First Principles/Code/docs/DNSSEC-DESIGN.md.source|docs/DNSSEC-DESIGN.md]]
- Lines: 164-173
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-810e23c0f3de", "codeNote": "DNS from First Principles/Code/docs/DNSSEC-DESIGN.md.source", "sourcePath": "docs/DNSSEC-DESIGN.md", "startLine": 164, "endLine": 173}
```

## Excerpt

<span id="rgbdns-frag-810e23c0f3de" class="rgbdns-fragment-target"></span>
### rgbdns-frag-810e23c0f3de: heading Failure and observability

```markdown
## Failure and observability

All programs log one line per failure to stderr and use stable nonzero exit
status. They do not retry indefinitely. `rgbsec-check` reports the SOA serial,
key tag, earliest expiration, remaining lifetime, and status. Service timers
run the check frequently enough to alert before the configured refresh window.

The last-known-good signed CDB remains authoritative after resolution, signing,
validation, transfer, or disk failures. Expired signatures are treated as an
urgent operational fault, never repaired by online signing inside `tinydns`.
```
