---
type: "code-fragment"
fragment_id: "rgbdns-frag-b31b353bc960"
source_path: "docs/book/rgbdns.md"
code_note: "DNS from First Principles/Code/docs/book/rgbdns.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "A practical selection rule"
kind: "heading"
start_line: 891
end_line: 905
---

# A practical selection rule

- Fragment ID: `rgbdns-frag-b31b353bc960`
- Source file: [[DNS from First Principles/Code/docs/book/rgbdns.md.source|docs/book/rgbdns.md]]
- Lines: 891-905
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-b31b353bc960", "codeNote": "DNS from First Principles/Code/docs/book/rgbdns.md.source", "heading": "rgbdns-frag-b31b353bc960: heading A practical selection rule", "sourcePath": "docs/book/rgbdns.md", "startLine": 891, "endLine": 905}
```

## Excerpt

<span id="rgbdns-frag-b31b353bc960" class="rgbdns-fragment-target"></span>
### rgbdns-frag-b31b353bc960: heading A practical selection rule

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
