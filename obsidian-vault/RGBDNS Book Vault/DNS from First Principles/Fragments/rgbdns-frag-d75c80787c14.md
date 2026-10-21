---
type: "code-fragment"
fragment_id: "rgbdns-frag-d75c80787c14"
source_path: "docs/book/rgbdns.md"
code_note: "DNS from First Principles/Code/docs/book/rgbdns.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "Adversarial cases worth keeping"
kind: "heading"
start_line: 2176
end_line: 2199
---

# Adversarial cases worth keeping

- Fragment ID: `rgbdns-frag-d75c80787c14`
- Source file: [[DNS from First Principles/Code/docs/book/rgbdns.md.source|docs/book/rgbdns.md]]
- Lines: 2176-2199
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-d75c80787c14", "codeNote": "DNS from First Principles/Code/docs/book/rgbdns.md.source", "sourcePath": "docs/book/rgbdns.md", "startLine": 2176, "endLine": 2199}
```

## Excerpt

<span id="rgbdns-frag-d75c80787c14" class="rgbdns-fragment-target"></span>
### rgbdns-frag-d75c80787c14: heading Adversarial cases worth keeping

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
- ANAME self-reference, upstream CNAME loops, excessive address results, and
  resolver failure;
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
