---
type: "code-fragment"
fragment_id: "rgbdns-frag-8dbfb904e1a6"
source_path: "docs/book/rgbdns.md"
code_note: "DNS from First Principles/Code/docs/book/rgbdns.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "Roles, not just \u201cDNS servers\u201d"
kind: "heading"
start_line: 73
end_line: 95
---

# Roles, not just “DNS servers”

- Fragment ID: `rgbdns-frag-8dbfb904e1a6`
- Source file: [[DNS from First Principles/Code/docs/book/rgbdns.md.source|docs/book/rgbdns.md]]
- Lines: 73-95
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-8dbfb904e1a6", "codeNote": "DNS from First Principles/Code/docs/book/rgbdns.md.source", "heading": "rgbdns-frag-8dbfb904e1a6: heading Roles, not just \u201cDNS servers\u201d", "sourcePath": "docs/book/rgbdns.md", "startLine": 73, "endLine": 95}
```

## Excerpt

<span id="rgbdns-frag-8dbfb904e1a6" class="rgbdns-fragment-target"></span>
### rgbdns-frag-8dbfb904e1a6: heading Roles, not just “DNS servers”

```markdown
## Roles, not just “DNS servers”

The phrase “DNS server” hides several jobs.

An **authoritative server** publishes data for zones it controls. It answers
from configured facts and does not chase referrals on behalf of a client.

A **recursive resolver** accepts a question from a stub client, follows the
delegation chain, validates and caches what it learns, and returns a final
answer.

A **stub resolver** is the client-side library or program that sends a
recursive query to a configured resolver.

A **forwarder** sends selected questions to another resolver rather than
performing iteration itself.

Keeping these roles distinct is both conceptual hygiene and a security
boundary. An authoritative daemon does not need a large mutable Internet-fed
cache. A recursive resolver does not need the private machinery used to edit a
zone. rgbdns follows the djbdns design and runs authority and recursion as
different programs.

```
