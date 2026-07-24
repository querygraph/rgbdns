---
type: "code-fragment"
fragment_id: "rgbdns-frag-23b5c2816ab6"
source_path: "docs/book/rgbdns.md"
code_note: "DNS from First Principles/Code/docs/book/rgbdns.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "The common envelope"
kind: "heading"
start_line: 165
end_line: 185
---

# The common envelope

- Fragment ID: `rgbdns-frag-23b5c2816ab6`
- Source file: [[DNS from First Principles/Code/docs/book/rgbdns.md.source|docs/book/rgbdns.md]]
- Lines: 165-185
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-23b5c2816ab6", "codeNote": "DNS from First Principles/Code/docs/book/rgbdns.md.source", "heading": "rgbdns-frag-23b5c2816ab6: heading The common envelope", "sourcePath": "docs/book/rgbdns.md", "startLine": 165, "endLine": 185}
```

## Excerpt

<span id="rgbdns-frag-23b5c2816ab6" class="rgbdns-fragment-target"></span>
### rgbdns-frag-23b5c2816ab6: heading The common envelope

```markdown
## The common envelope

Every resource record has:

- an owner name;
- a numeric type;
- a class, almost always Internet class `IN`;
- a time to live, or TTL;
- type-specific data called RDATA.

The TTL is a lease offered to caches. If an authoritative server returns a TTL
of 300 seconds, a cache may reuse that answer for at most five minutes before
refreshing it. The TTL does not schedule a change and does not guarantee that
every cache holds the answer for the full interval. It establishes an upper
bound.

Changing a record and then lowering its TTL is too late for clients that
already cached the older, longer lease. Planned migrations lower the TTL at
least one old-TTL interval before the change, wait, make the change, and later
raise it.

```
