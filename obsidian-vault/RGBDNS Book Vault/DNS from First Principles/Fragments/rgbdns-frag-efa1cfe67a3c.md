---
type: "code-fragment"
fragment_id: "rgbdns-frag-efa1cfe67a3c"
source_path: "docs/blog/rgbdns-dnssec/post.md"
code_note: "DNS from First Principles/Code/docs/blog/rgbdns-dnssec/post.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "ANAME and ACME make the boundary visible"
kind: "heading"
start_line: 172
end_line: 191
---

# ANAME and ACME make the boundary visible

- Fragment ID: `rgbdns-frag-efa1cfe67a3c`
- Source file: [[DNS from First Principles/Code/docs/blog/rgbdns-dnssec/post.md.source|docs/blog/rgbdns-dnssec/post.md]]
- Lines: 172-191
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-efa1cfe67a3c", "codeNote": "DNS from First Principles/Code/docs/blog/rgbdns-dnssec/post.md.source", "sourcePath": "docs/blog/rgbdns-dnssec/post.md", "startLine": 172, "endLine": 191}
```

## Excerpt

<span id="rgbdns-frag-efa1cfe67a3c" class="rgbdns-fragment-target"></span>
### rgbdns-frag-efa1cfe67a3c: heading ANAME and ACME make the boundary visible

```markdown
## ANAME and ACME make the boundary visible

An offline signature can authenticate only a stable RRset. A signed ANAME zone
must therefore resolve its target before signing. `aname-materialize` converts
the directive into ordinary A and AAAA records under the configured TTL ceiling;
the publisher refreshes those records and their signatures before expiry.

An unsigned zone in the same snapshot can retain runtime ANAME behavior. The
policy says which model applies instead of asking the server to guess.

ACME is stricter. An unprivileged live TXT overlay can continue in a `U` zone.
An inline update to a `K` zone would cross into the root-only signing path and
needs an explicitly designed privileged publisher. rgbdns does not synthesize
that escalation. The simplest answer is often to delegate
`_acme-challenge` to a small unsigned validation child.

This is why the first pilots were intentionally boring: ordinary addresses, no
ANAME, no live ACME updates. Cryptography is easier to reason about when the
data model is not moving underneath it.

```
