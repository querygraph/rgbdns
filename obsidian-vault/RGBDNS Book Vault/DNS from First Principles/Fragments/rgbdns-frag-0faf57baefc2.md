---
type: "code-fragment"
fragment_id: "rgbdns-frag-0faf57baefc2"
source_path: "CHANGELOG.md"
code_note: "DNS from First Principles/Code/CHANGELOG.md.source"
language: "markdown"
subsystem: "Repository and build"
symbol: "0.6.3 - 2026-08-20"
kind: "heading"
start_line: 37
end_line: 44
---

# 0.6.3 - 2026-08-20

- Fragment ID: `rgbdns-frag-0faf57baefc2`
- Source file: [[DNS from First Principles/Code/CHANGELOG.md.source|CHANGELOG.md]]
- Lines: 37-44
- Subsystem: [[DNS from First Principles/Subsystems/Repository and build|Repository and build]]

```rgbdns-fragment
{"id": "rgbdns-frag-0faf57baefc2", "codeNote": "DNS from First Principles/Code/CHANGELOG.md.source", "sourcePath": "CHANGELOG.md", "startLine": 37, "endLine": 44}
```

## Excerpt

<span id="rgbdns-frag-0faf57baefc2" class="rgbdns-fragment-target"></span>
### rgbdns-frag-0faf57baefc2: heading 0.6.3 - 2026-08-20

```markdown
## [0.6.3] - 2026-08-20

### Fixed

- Treat DNS suffix checks as ASCII case-insensitive so 0x20-randomized queries
  for nonexistent names receive signed NXDOMAIN/NSEC proofs instead of
  REFUSED responses.

```
