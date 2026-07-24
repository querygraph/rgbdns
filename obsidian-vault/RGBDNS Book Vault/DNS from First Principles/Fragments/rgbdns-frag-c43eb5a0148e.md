---
type: "code-fragment"
fragment_id: "rgbdns-frag-c43eb5a0148e"
source_path: "docs/book/rgbdns.md"
code_note: "DNS from First Principles/Code/docs/book/rgbdns.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "rgbdns validation policy"
kind: "heading"
start_line: 543
end_line: 555
---

# rgbdns validation policy

- Fragment ID: `rgbdns-frag-c43eb5a0148e`
- Source file: [[DNS from First Principles/Code/docs/book/rgbdns.md.source|docs/book/rgbdns.md]]
- Lines: 543-555
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-c43eb5a0148e", "codeNote": "DNS from First Principles/Code/docs/book/rgbdns.md.source", "heading": "rgbdns-frag-c43eb5a0148e: heading rgbdns validation policy", "sourcePath": "docs/book/rgbdns.md", "startLine": 543, "endLine": 555}
```

## Excerpt

<span id="rgbdns-frag-c43eb5a0148e" class="rgbdns-fragment-target"></span>
### rgbdns-frag-c43eb5a0148e: heading rgbdns validation policy

```markdown
## rgbdns validation policy

rgbdns configures the recursive handler with a static root trust anchor and
DNSSEC validation enabled. Validation and NSEC3 work receive bounded caches and
iteration policies. A failed validation surfaces as resolution failure rather
than an unchecked answer.

The authoritative rgbdns data path focuses on the djbdns record surface; the
recursive path is where DNSSEC validation is currently integrated. This is an
example of honest component boundaries: “the suite supports validating
recursion” does not imply that every authoritative signing workflow has been
recreated.

```
