---
type: "code-fragment"
fragment_id: "rgbdns-frag-8679aacc97e7"
source_path: "docs/book/rgbdns.md"
code_note: "DNS from First Principles/Code/docs/book/rgbdns.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "Zones are administrative cuts"
kind: "heading"
start_line: 128
end_line: 149
---

# Zones are administrative cuts

- Fragment ID: `rgbdns-frag-8679aacc97e7`
- Source file: [[DNS from First Principles/Code/docs/book/rgbdns.md.source|docs/book/rgbdns.md]]
- Lines: 128-149
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-8679aacc97e7", "codeNote": "DNS from First Principles/Code/docs/book/rgbdns.md.source", "heading": "rgbdns-frag-8679aacc97e7: heading Zones are administrative cuts", "sourcePath": "docs/book/rgbdns.md", "startLine": 128, "endLine": 149}
```

## Excerpt

<span id="rgbdns-frag-8679aacc97e7" class="rgbdns-fragment-target"></span>
### rgbdns-frag-8679aacc97e7: heading Zones are administrative cuts

```markdown
## Zones are administrative cuts

The namespace is one tree; a zone is an administratively served portion of
that tree. The two are not identical.

The zone `example.com.` might contain records for `www.example.com.` and
`mail.example.com.`, then delegate `research.example.com.` to other servers.
The child remains below `example.com.` in the namespace but is outside the
parent zone’s authoritative contents.

A delegation is expressed by NS records at the cut. If a named server lies
inside the delegated child, a resolver cannot first resolve that server’s name
through the child—it needs its address in order to reach the child. The parent
therefore supplies an address record called **glue**. Glue is navigation data,
not an assertion that the parent is authoritative for every fact about the
host.

The root zone delegates top-level domains. A cold recursive resolver starts
with a small configured set of root server addresses, asks the root where to
find a top-level domain, asks that domain where to find the next child, and
continues.

```
