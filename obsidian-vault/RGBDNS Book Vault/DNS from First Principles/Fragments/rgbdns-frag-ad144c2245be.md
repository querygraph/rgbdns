---
type: "code-fragment"
fragment_id: "rgbdns-frag-ad144c2245be"
source_path: "docs/book/rgbdns.md"
code_note: "DNS from First Principles/Code/docs/book/rgbdns.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "Adversarial cases worth keeping"
kind: "heading"
start_line: 976
end_line: 997
---

# Adversarial cases worth keeping

- Fragment ID: `rgbdns-frag-ad144c2245be`
- Source file: [[DNS from First Principles/Code/docs/book/rgbdns.md.source|docs/book/rgbdns.md]]
- Lines: 976-997
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-ad144c2245be", "codeNote": "DNS from First Principles/Code/docs/book/rgbdns.md.source", "heading": "rgbdns-frag-ad144c2245be: heading Adversarial cases worth keeping", "sourcePath": "docs/book/rgbdns.md", "startLine": 976, "endLine": 997}
```

## Excerpt

<span id="rgbdns-frag-ad144c2245be" class="rgbdns-fragment-target"></span>
### rgbdns-frag-ad144c2245be: heading Adversarial cases worth keeping

```markdown
## Adversarial cases worth keeping

Every DNS implementation should retain regression cases for:

- a compression pointer to itself or a pointer cycle;
- a pointer or RDATA length just beyond the packet;
- maximum-length labels and names;
- counts that cannot be satisfied by the remaining bytes;
- duplicate or malformed OPT records;
- tiny advertised transport limits;
- CNAME loops and excessive chains;
- wildcard names blocked by existing nodes;
- delegation cuts beneath an authoritative apex;
- NODATA versus NXDOMAIN;
- AXFR without a closing SOA;
- an enormous log line;
- configuration counts at and beyond each bound.

Tests should assert protocol meaning, not only that the process remains alive.
A safe FORMERR is better than a crash, but a silent NOERROR can still be a
serious bug.

```
