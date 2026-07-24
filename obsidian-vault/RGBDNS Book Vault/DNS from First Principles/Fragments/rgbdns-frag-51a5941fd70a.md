---
type: "code-fragment"
fragment_id: "rgbdns-frag-51a5941fd70a"
source_path: "docs/book/rgbdns.md"
code_note: "DNS from First Principles/Code/docs/book/rgbdns.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "Wildcards are synthesis rules"
kind: "heading"
start_line: 150
end_line: 162
---

# Wildcards are synthesis rules

- Fragment ID: `rgbdns-frag-51a5941fd70a`
- Source file: [[DNS from First Principles/Code/docs/book/rgbdns.md.source|docs/book/rgbdns.md]]
- Lines: 150-162
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-51a5941fd70a", "codeNote": "DNS from First Principles/Code/docs/book/rgbdns.md.source", "heading": "rgbdns-frag-51a5941fd70a: heading Wildcards are synthesis rules", "sourcePath": "docs/book/rgbdns.md", "startLine": 150, "endLine": 162}
```

## Excerpt

<span id="rgbdns-frag-51a5941fd70a" class="rgbdns-fragment-target"></span>
### rgbdns-frag-51a5941fd70a: heading Wildcards are synthesis rules

```markdown
## Wildcards are synthesis rules

A wildcard such as `*.example.com.` does not mean “return this record for every
name ending in example.com.” It participates only when the queried name does
not exist, and the closest-encloser rules determine which wildcard, if any,
can synthesize an answer. Existing intermediate names can block a wildcard.

rgbdns stores wildcard records under their literal wildcard owner and its zone
lookup searches from the queried name toward the closest existing ancestor.
It synthesizes the queried owner in returned records. This is more precise
than a string suffix match and is one reason the `Zone` abstraction tracks
known nodes in addition to records.

```
