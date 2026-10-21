---
type: "code-fragment"
fragment_id: "rgbdns-frag-8020d371d034"
source_path: "docs/book/rgbdns.md"
code_note: "DNS from First Principles/Code/docs/book/rgbdns.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "A practical selection rule"
kind: "heading"
start_line: 1374
end_line: 1388
---

# A practical selection rule

- Fragment ID: `rgbdns-frag-8020d371d034`
- Source file: [[DNS from First Principles/Code/docs/book/rgbdns.md.source|docs/book/rgbdns.md]]
- Lines: 1374-1388
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-8020d371d034", "codeNote": "DNS from First Principles/Code/docs/book/rgbdns.md.source", "sourcePath": "docs/book/rgbdns.md", "startLine": 1374, "endLine": 1388}
```

## Excerpt

<span id="rgbdns-frag-8020d371d034" class="rgbdns-fragment-target"></span>
### rgbdns-frag-8020d371d034: heading A practical selection rule

```markdown
## A practical selection rule

Use this order:

1. Follow the host’s native manager: systemd on systemd hosts, OpenRC on
   OpenRC hosts.
2. For a direct service-directory replacement, choose runit.
3. For a designed supervision graph or multi-process container, choose
   s6/s6-rc.
4. In an orchestrated single-process container, use the orchestrator.

The least risky migration preserves one owner for restart policy and logs.
Running two supervisors creates ambiguous signal paths, duplicate restarts,
and status commands that disagree.

```
