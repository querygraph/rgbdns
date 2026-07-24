---
type: "code-fragment"
fragment_id: "rgbdns-frag-29b0066bd3e1"
source_path: "docs/book/rgbdns.md"
code_note: "DNS from First Principles/Code/docs/book/rgbdns.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "Appendix A Configuration quick reference"
kind: "heading"
start_line: 1531
end_line: 1550
---

# Appendix A Configuration quick reference

- Fragment ID: `rgbdns-frag-29b0066bd3e1`
- Source file: [[DNS from First Principles/Code/docs/book/rgbdns.md.source|docs/book/rgbdns.md]]
- Lines: 1531-1550
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-29b0066bd3e1", "codeNote": "DNS from First Principles/Code/docs/book/rgbdns.md.source", "heading": "rgbdns-frag-29b0066bd3e1: heading Appendix A Configuration quick reference", "sourcePath": "docs/book/rgbdns.md", "startLine": 1531, "endLine": 1550}
```

## Excerpt

<span id="rgbdns-frag-29b0066bd3e1" class="rgbdns-fragment-target"></span>
### rgbdns-frag-29b0066bd3e1: heading Appendix A Configuration quick reference

```markdown
# Appendix A: Configuration quick reference

Common daemon variables include:

| Variable | Meaning |
|---|---|
| `IP` | listen address |
| `PORT` | listen port |
| `DATA` | authoritative text or CDB path where supported |
| `ALLOW_NETS` | comma-separated client CIDRs for recursion or transfer |
| `DNSCACHEIP` | recursive endpoints used by client tools |
| `CACHESIZE` | bounded recursive response-cache capacity |
| `NSCACHESIZE` | bounded nameserver-cache entries |
| `RECURSION_LIMIT` | ordinary recursion depth |
| `NS_RECURSION_LIMIT` | nameserver-resolution recursion depth |
| `ROOT` | djbdns-compatible resolver configuration root |

Use the command’s `*-conf` generator as a starting point, then adapt the
foreground `run` contract to the chosen native supervisor.

```
