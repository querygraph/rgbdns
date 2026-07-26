---
type: "code-fragment"
fragment_id: "rgbdns-frag-20532ab9b198"
source_path: "docs/book/rgbdns.md"
code_note: "DNS from First Principles/Code/docs/book/rgbdns.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "Appendix A Configuration quick reference"
kind: "heading"
start_line: 1801
end_line: 1820
---

# Appendix A Configuration quick reference

- Fragment ID: `rgbdns-frag-20532ab9b198`
- Source file: [[DNS from First Principles/Code/docs/book/rgbdns.md.source|docs/book/rgbdns.md]]
- Lines: 1801-1820
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-20532ab9b198", "codeNote": "DNS from First Principles/Code/docs/book/rgbdns.md.source", "heading": "rgbdns-frag-20532ab9b198: heading Appendix A Configuration quick reference", "sourcePath": "docs/book/rgbdns.md", "startLine": 1801, "endLine": 1820}
```

## Excerpt

<span id="rgbdns-frag-20532ab9b198" class="rgbdns-fragment-target"></span>
### rgbdns-frag-20532ab9b198: heading Appendix A Configuration quick reference

```markdown
# Appendix A: Configuration quick reference

Common daemon variables include:

| Variable | Meaning |
|---|---|
| `IP` | listen address |
| `PORT` | listen port |
| `DATA` | authoritative text or CDB path where supported |
| `ALLOW_NETS` | comma-separated client CIDRs for recursion or transfer |
| `DNSCACHEIP` | recursive endpoints used by client tools and ANAME flattening |
| `CACHESIZE` | bounded recursive response-cache capacity |
| `NSCACHESIZE` | bounded nameserver-cache entries |
| `RECURSION_LIMIT` | ordinary recursion depth |
| `NS_RECURSION_LIMIT` | nameserver-resolution recursion depth |
| `ROOT` | djbdns-compatible resolver configuration root |

Use the command’s `*-conf` generator as a starting point, then adapt the
foreground `run` contract to the chosen native supervisor.

```
