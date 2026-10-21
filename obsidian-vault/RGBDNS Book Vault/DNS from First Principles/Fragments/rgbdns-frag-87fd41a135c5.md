---
type: "code-fragment"
fragment_id: "rgbdns-frag-87fd41a135c5"
source_path: "docs/book/rgbdns.md"
code_note: "DNS from First Principles/Code/docs/book/rgbdns.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "Practical checks"
kind: "heading"
start_line: 1143
end_line: 1158
---

# Practical checks

- Fragment ID: `rgbdns-frag-87fd41a135c5`
- Source file: [[DNS from First Principles/Code/docs/book/rgbdns.md.source|docs/book/rgbdns.md]]
- Lines: 1143-1158
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-87fd41a135c5", "codeNote": "DNS from First Principles/Code/docs/book/rgbdns.md.source", "sourcePath": "docs/book/rgbdns.md", "startLine": 1143, "endLine": 1158}
```

## Excerpt

<span id="rgbdns-frag-87fd41a135c5" class="rgbdns-fragment-target"></span>
### rgbdns-frag-87fd41a135c5: heading Practical checks

````markdown
## Practical checks

When a name fails, inspect type, server, flags, and authority section rather
than asking only whether an address appeared.

```sh
dnsq A www.example.com 192.0.2.53
dnsq AAAA www.example.com 192.0.2.53
dnsq SOA example.com 192.0.2.53
dnstrace A www.example.com
```

Compare UDP and TCP when answers are large. Query the parent-side NS records
and the child authority separately. An NXDOMAIN with an SOA is different from
a timeout, SERVFAIL, or REFUSED, and each points to a different layer.

````
