---
type: "code-fragment"
fragment_id: "rgbdns-frag-a3f35901d5b4"
source_path: "docs/book/rgbdns.md"
code_note: "DNS from First Principles/Code/docs/book/rgbdns.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "The twelve-byte header"
kind: "heading"
start_line: 243
end_line: 272
---

# The twelve-byte header

- Fragment ID: `rgbdns-frag-a3f35901d5b4`
- Source file: [[DNS from First Principles/Code/docs/book/rgbdns.md.source|docs/book/rgbdns.md]]
- Lines: 243-272
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-a3f35901d5b4", "codeNote": "DNS from First Principles/Code/docs/book/rgbdns.md.source", "heading": "rgbdns-frag-a3f35901d5b4: heading The twelve-byte header", "sourcePath": "docs/book/rgbdns.md", "startLine": 243, "endLine": 272}
```

## Excerpt

<span id="rgbdns-frag-a3f35901d5b4" class="rgbdns-fragment-target"></span>
### rgbdns-frag-a3f35901d5b4: heading The twelve-byte header

```markdown
## The twelve-byte header

A DNS message begins with a fixed twelve-byte header:

```text
0                   1                   2                   3
0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
+-------------------------------+-------------------------------+
|              ID               |            flags              |
+-------------------------------+-------------------------------+
|          question count       |          answer count         |
+-------------------------------+-------------------------------+
|         authority count       |         additional count      |
+-------------------------------+-------------------------------+
```

The transaction ID lets a client associate a response with a query. Important
flags include QR (query versus response), opcode, AA (authoritative answer), TC
(truncated), RD (recursion desired), RA (recursion available), and the
four-bit response code.

The four following sections contain questions, answers, authority records, and
additional records. A normal question carries a name, requested type, and
class. Resource-record sections add TTL, RDATA length, and RDATA.

All multibyte integers are network byte order. Every count and length comes
from an untrusted peer. A decoder must prove that bytes exist before reading
them, cap allocations, reject invalid labels and pointers, and finish with a
coherent message rather than a partially trusted structure.

```
