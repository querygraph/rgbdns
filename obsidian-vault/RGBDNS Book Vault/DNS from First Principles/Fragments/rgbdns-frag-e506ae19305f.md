---
type: "code-fragment"
fragment_id: "rgbdns-frag-e506ae19305f"
source_path: "docs/book/rgbdns.md"
code_note: "DNS from First Principles/Code/docs/book/rgbdns.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "Iteration from the root"
kind: "heading"
start_line: 447
end_line: 467
---

# Iteration from the root

- Fragment ID: `rgbdns-frag-e506ae19305f`
- Source file: [[DNS from First Principles/Code/docs/book/rgbdns.md.source|docs/book/rgbdns.md]]
- Lines: 447-467
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-e506ae19305f", "codeNote": "DNS from First Principles/Code/docs/book/rgbdns.md.source", "heading": "rgbdns-frag-e506ae19305f: heading Iteration from the root", "sourcePath": "docs/book/rgbdns.md", "startLine": 447, "endLine": 467}
```

## Excerpt

<span id="rgbdns-frag-e506ae19305f" class="rgbdns-fragment-target"></span>
### rgbdns-frag-e506ae19305f: heading Iteration from the root

```markdown
## Iteration from the root

A recursive resolver turns one client request into a bounded sequence of
queries. For `www.example.com. A`, a cold lookup is approximately:

```text
stub → recursive resolver
          ├─ root:       who serves com?
          ├─ com server: who serves example.com?
          └─ example:    what is www.example.com A?
     ← final answer
```

The resolver follows referrals, resolves nameserver addresses when glue is
insufficient, handles aliases, retries servers and transports, and detects
loops. It caches useful RRsets so later clients may skip most of this path.

Root hints are not answers to every name. They are bootstrap addresses for
reaching the root authority. They need periodic maintenance because the set
can change, though names and anycast make changes infrequent.

```
