---
type: "code-fragment"
fragment_id: "rgbdns-frag-4860ba91b2b0"
source_path: "CHANGELOG.md"
code_note: "DNS from First Principles/Code/CHANGELOG.md.source"
language: "markdown"
subsystem: "Repository and build"
symbol: "0.6.4 - 2026-09-12"
kind: "heading"
start_line: 28
end_line: 36
---

# 0.6.4 - 2026-09-12

- Fragment ID: `rgbdns-frag-4860ba91b2b0`
- Source file: [[DNS from First Principles/Code/CHANGELOG.md.source|CHANGELOG.md]]
- Lines: 28-36
- Subsystem: [[DNS from First Principles/Subsystems/Repository and build|Repository and build]]

```rgbdns-fragment
{"id": "rgbdns-frag-4860ba91b2b0", "codeNote": "DNS from First Principles/Code/CHANGELOG.md.source", "sourcePath": "CHANGELOG.md", "startLine": 28, "endLine": 36}
```

## Excerpt

<span id="rgbdns-frag-4860ba91b2b0" class="rgbdns-fragment-target"></span>
### rgbdns-frag-4860ba91b2b0: heading 0.6.4 - 2026-09-12

```markdown
## [0.6.4] - 2026-09-12

### Fixed

- Publish the newer (RFC 1982 order) of the source data's SOA serial and the
  stored ACME serial for ACME-managed zones. The stored serial used to replace
  the source serial unconditionally, so an operator's edit with a higher serial
  kept publishing the older ACME serial and secondaries never transferred it.

```
