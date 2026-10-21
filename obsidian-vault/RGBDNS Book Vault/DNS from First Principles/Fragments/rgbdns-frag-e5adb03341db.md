---
type: "code-fragment"
fragment_id: "rgbdns-frag-e5adb03341db"
source_path: "AGENTS.md"
code_note: "DNS from First Principles/Code/AGENTS.md.source"
language: "markdown"
subsystem: "Repository and build"
symbol: "Release versioning"
kind: "heading"
start_line: 3
end_line: 20
---

# Release versioning

- Fragment ID: `rgbdns-frag-e5adb03341db`
- Source file: [[DNS from First Principles/Code/AGENTS.md.source|AGENTS.md]]
- Lines: 3-20
- Subsystem: [[DNS from First Principles/Subsystems/Repository and build|Repository and build]]

```rgbdns-fragment
{"id": "rgbdns-frag-e5adb03341db", "codeNote": "DNS from First Principles/Code/AGENTS.md.source", "sourcePath": "AGENTS.md", "startLine": 3, "endLine": 20}
```

## Excerpt

<span id="rgbdns-frag-e5adb03341db" class="rgbdns-fragment-target"></span>
### rgbdns-frag-e5adb03341db: heading Release versioning

```markdown
## Release versioning

- Follow semantic versioning consistently across every package and document.
- Increment the **minor** component for a new feature. Examples include a new
  DNS behavior, command, transfer capability, service role, or package
  workflow. ANAME preservation over AXFR is the feature release `0.3.0`.
- Increment the **patch** component only for compatible bug fixes, security
  fixes, documentation corrections, and packaging repairs that add no new
  behavior.
- Reserve the **major** component for incompatible public behavior or format
  changes once the public surface is stable.
- Keep `Cargo.toml`, the rgbdns entry in `Cargo.lock`, `debian/changelog`,
  `packaging/rpm/rgbdns.spec`, installed manual version strings, examples,
  deployment guides, and release documentation synchronized.
- Debian package versions use the software version directly. RPM feature and
  bug releases start at release `1`; increment the RPM release only for a
  packaging-only rebuild of the same software version.

```
