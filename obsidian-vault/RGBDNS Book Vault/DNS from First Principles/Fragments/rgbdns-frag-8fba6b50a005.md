---
type: "code-fragment"
fragment_id: "rgbdns-frag-8fba6b50a005"
source_path: "CHANGELOG.md"
code_note: "DNS from First Principles/Code/CHANGELOG.md.source"
language: "markdown"
subsystem: "Repository and build"
symbol: "0.6.2 - 2026-08-19"
kind: "heading"
start_line: 45
end_line: 55
---

# 0.6.2 - 2026-08-19

- Fragment ID: `rgbdns-frag-8fba6b50a005`
- Source file: [[DNS from First Principles/Code/CHANGELOG.md.source|CHANGELOG.md]]
- Lines: 45-55
- Subsystem: [[DNS from First Principles/Subsystems/Repository and build|Repository and build]]

```rgbdns-fragment
{"id": "rgbdns-frag-8fba6b50a005", "codeNote": "DNS from First Principles/Code/CHANGELOG.md.source", "sourcePath": "CHANGELOG.md", "startLine": 45, "endLine": 55}
```

## Excerpt

<span id="rgbdns-frag-8fba6b50a005" class="rgbdns-fragment-target"></span>
### rgbdns-frag-8fba6b50a005: heading 0.6.2 - 2026-08-19

```markdown
## [0.6.2] - 2026-08-19

### Fixed

- Keep ACME-managed `U` zones on the unprivileged in-memory overlay path when
  another zone in the same snapshot is signed. This avoids invoking the
  root-only signer during authoritative startup or an unsigned-zone update.
- Stop `rgbdns-setup` from installing a root-only signer as an unprivileged
  ACME publication hook. ACME updates to a `K` zone continue to fail closed
  unless the operator supplies an explicit privileged publication mechanism.

```
