---
type: "code-fragment"
fragment_id: "rgbdns-frag-425f4904b6d4"
source_path: "docs/book/rgbdns.md"
code_note: "DNS from First Principles/Code/docs/book/rgbdns.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "ANAME and apex address flattening"
kind: "heading"
start_line: 434
end_line: 449
---

# ANAME and apex address flattening

- Fragment ID: `rgbdns-frag-425f4904b6d4`
- Source file: [[DNS from First Principles/Code/docs/book/rgbdns.md.source|docs/book/rgbdns.md]]
- Lines: 434-449
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-425f4904b6d4", "codeNote": "DNS from First Principles/Code/docs/book/rgbdns.md.source", "heading": "rgbdns-frag-425f4904b6d4: heading ANAME and apex address flattening", "sourcePath": "docs/book/rgbdns.md", "startLine": 434, "endLine": 449}
```

## Excerpt

<span id="rgbdns-frag-425f4904b6d4" class="rgbdns-fragment-target"></span>
### rgbdns-frag-425f4904b6d4: heading ANAME and apex address flattening

```markdown
## ANAME and apex address flattening

A zone apex necessarily owns SOA and NS data. A CNAME owner, by contrast,
cannot also own ordinary records. A literal apex CNAME would therefore make
the zone internally contradictory: the alias rule says the owner has no other
data while the authority rules require other data at that same owner.

Hosted sites still need a way for `example.com` to track addresses controlled
by a platform such as `customer.blog-host.example`. DNS providers commonly
call the solution CNAME flattening, ALIAS, or ANAME. The authoritative server
resolves the configured target itself and publishes the resulting addresses
under the configured owner.

rgbdns calls this feature **ANAME** and uses the private `A` source marker:

```text
```
