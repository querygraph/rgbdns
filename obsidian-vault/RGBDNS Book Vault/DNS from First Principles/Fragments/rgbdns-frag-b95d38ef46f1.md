---
type: "code-fragment"
fragment_id: "rgbdns-frag-b95d38ef46f1"
source_path: "docs/DNSSEC-DESIGN.md"
code_note: "DNS from First Principles/Code/docs/DNSSEC-DESIGN.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "Key lifecycle"
kind: "heading"
start_line: 151
end_line: 163
---

# Key lifecycle

- Fragment ID: `rgbdns-frag-b95d38ef46f1`
- Source file: [[DNS from First Principles/Code/docs/DNSSEC-DESIGN.md.source|docs/DNSSEC-DESIGN.md]]
- Lines: 151-163
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-b95d38ef46f1", "codeNote": "DNS from First Principles/Code/docs/DNSSEC-DESIGN.md.source", "sourcePath": "docs/DNSSEC-DESIGN.md", "startLine": 151, "endLine": 163}
```

## Excerpt

<span id="rgbdns-frag-b95d38ef46f1" class="rgbdns-fragment-target"></span>
### rgbdns-frag-b95d38ef46f1: heading Key lifecycle

```markdown
## Key lifecycle

Key creation, activation, parent DS publication, rollover, revocation, and
retirement are distinct operator actions represented by files and policy
lines. Version 0.6.5 delivers safe single-key operation and DS derivation. A
later feature release will add a deliberately specified multi-key rollover
state machine; operators must not simulate rollover by silently swapping a key
file under an unchanged policy line.

Private keys are primary-only, mode 0600, and readable only by the offline
publication account. `tinydns`, `axfrdns`, and secondaries do not receive key
access. Backups and destruction are operator-visible file operations.

```
