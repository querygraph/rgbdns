---
type: "code-fragment"
fragment_id: "rgbdns-frag-fd0d6ceb6138"
source_path: "docs/blog/rgbdns-dnssec/post.md"
code_note: "DNS from First Principles/Code/docs/blog/rgbdns-dnssec/post.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "The bug that only Google showed us"
kind: "heading"
start_line: 154
end_line: 171
---

# The bug that only Google showed us

- Fragment ID: `rgbdns-frag-fd0d6ceb6138`
- Source file: [[DNS from First Principles/Code/docs/blog/rgbdns-dnssec/post.md.source|docs/blog/rgbdns-dnssec/post.md]]
- Lines: 154-171
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-fd0d6ceb6138", "codeNote": "DNS from First Principles/Code/docs/blog/rgbdns-dnssec/post.md.source", "sourcePath": "docs/blog/rgbdns-dnssec/post.md", "startLine": 154, "endLine": 171}
```

## Excerpt

<span id="rgbdns-frag-fd0d6ceb6138" class="rgbdns-fragment-target"></span>
### rgbdns-frag-fd0d6ceb6138: heading The bug that only Google showed us

```markdown
## The bug that only Google showed us

The first `bitcoin.science` activation exposed a good example of why independent
validators matter. Cloudflare and Quad9 accepted signed positive and negative
answers. Google accepted positive answers but returned SERVFAIL for fresh
negative names.

Google's recursive path randomized the letter case of the query name. rgbdns's
name equality and ordering were already case-insensitive, but the helper that
decided whether a name was beneath an authoritative zone compared raw label
slices. A mixed-case nonexistent name was therefore refused before the server
could return its signed NSEC proof.

rgbdns 0.6.3 fixes authoritative suffix matching and retains the exact case as a
regression test. The production deployment gate now asks for
`MiSsInG.<zone>`, not only lowercase `missing.<zone>`. A property discovered on
the public Internet became a permanent local invariant.

```
