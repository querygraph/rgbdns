---
type: "code-fragment"
fragment_id: "rgbdns-frag-dc8850828d5c"
source_path: "docs/book/rgbdns.md"
code_note: "DNS from First Principles/Code/docs/book/rgbdns.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "Where DNS ends"
kind: "heading"
start_line: 1779
end_line: 1800
---

# Where DNS ends

- Fragment ID: `rgbdns-frag-dc8850828d5c`
- Source file: [[DNS from First Principles/Code/docs/book/rgbdns.md.source|docs/book/rgbdns.md]]
- Lines: 1779-1800
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-dc8850828d5c", "codeNote": "DNS from First Principles/Code/docs/book/rgbdns.md.source", "heading": "rgbdns-frag-dc8850828d5c: heading Where DNS ends", "sourcePath": "docs/book/rgbdns.md", "startLine": 1779, "endLine": 1800}
```

## Excerpt

<span id="rgbdns-frag-dc8850828d5c" class="rgbdns-fragment-target"></span>
### rgbdns-frag-dc8850828d5c: heading Where DNS ends

```markdown
# Where DNS ends

DNS establishes named, cacheable facts and, with DNSSEC, their authenticated
origin. It does not prove that the address belongs to the application a user
intended, encrypt the subsequent connection, guarantee freshness inside the
TTL window, or choose a healthy endpoint. TLS identity, application discovery,
load balancing, routing, and monitoring build on DNS but remain separate
systems.

That boundary is the best final replacement for the phone-book metaphor. DNS
is a delegated publication and discovery protocol. Its tree assigns authority;
its records carry typed statements; its TTLs make caching explicit; its packet
format makes efficient exchange possible; recursion joins many authorities
into one answer; DNSSEC authenticates the chain; and supervision keeps the
implementing processes available without becoming part of the protocol.

rgbdns expresses those ideas as small programs over shared, validated Rust
types. Understanding the protocol makes the program family unsurprising.
Reading the program family, in turn, shows how the abstract DNS model becomes
bounded packets, immutable databases, iterative queries, atomic files, and
foreground processes.

```
