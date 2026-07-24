---
type: "code-fragment"
fragment_id: "rgbdns-frag-43c02c0a823e"
source_path: "docs/book/rgbdns.md"
code_note: "DNS from First Principles/Code/docs/book/rgbdns.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "Additional data is an optimization"
kind: "heading"
start_line: 230
end_line: 240
---

# Additional data is an optimization

- Fragment ID: `rgbdns-frag-43c02c0a823e`
- Source file: [[DNS from First Principles/Code/docs/book/rgbdns.md.source|docs/book/rgbdns.md]]
- Lines: 230-240
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-43c02c0a823e", "codeNote": "DNS from First Principles/Code/docs/book/rgbdns.md.source", "heading": "rgbdns-frag-43c02c0a823e: heading Additional data is an optimization", "sourcePath": "docs/book/rgbdns.md", "startLine": 230, "endLine": 240}
```

## Excerpt

<span id="rgbdns-frag-43c02c0a823e" class="rgbdns-fragment-target"></span>
### rgbdns-frag-43c02c0a823e: heading Additional data is an optimization

```markdown
## Additional data is an optimization

If an answer contains MX, NS, or SRV targets, the server may include associated
A and AAAA records in the additional section. This can save queries. It does
not change which RRset directly answers the question, and a resolver must apply
the correct credibility rules rather than trusting unrelated additional data.

The rgbdns authoritative response path collects target names from those record
types and adds locally available addresses. It de-duplicates targets before
lookup and preserves the distinction between answers and helpful additionals.

```
