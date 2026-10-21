---
type: "code-fragment"
fragment_id: "rgbdns-frag-4157bad85e31"
source_path: "AGENTS.md"
code_note: "DNS from First Principles/Code/AGENTS.md.source"
language: "markdown"
subsystem: "Repository and build"
symbol: "Toolchain"
kind: "heading"
start_line: 21
end_line: 28
---

# Toolchain

- Fragment ID: `rgbdns-frag-4157bad85e31`
- Source file: [[DNS from First Principles/Code/AGENTS.md.source|AGENTS.md]]
- Lines: 21-28
- Subsystem: [[DNS from First Principles/Subsystems/Repository and build|Repository and build]]

```rgbdns-fragment
{"id": "rgbdns-frag-4157bad85e31", "codeNote": "DNS from First Principles/Code/AGENTS.md.source", "sourcePath": "AGENTS.md", "startLine": 21, "endLine": 28}
```

## Excerpt

<span id="rgbdns-frag-4157bad85e31" class="rgbdns-fragment-target"></span>
### rgbdns-frag-4157bad85e31: heading Toolchain

```markdown
## Toolchain

- Use the repository `.tool-versions` through asdf.
- Use uv/uvx for Python tools; do not create ad-hoc pip virtual environments.
- The RFC build is `make -C ietf check`.
- The FirstPair book builder enforces its own locked publishing toolchain. Do
  not silently regenerate committed book artifacts with mismatched tools.

```
