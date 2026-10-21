---
type: "code-fragment"
fragment_id: "rgbdns-frag-ed4699815d6e"
source_path: "CHANGELOG.md"
code_note: "DNS from First Principles/Code/CHANGELOG.md.source"
language: "markdown"
subsystem: "Repository and build"
symbol: "0.6.5 - 2026-09-12"
kind: "heading"
start_line: 8
end_line: 27
---

# 0.6.5 - 2026-09-12

- Fragment ID: `rgbdns-frag-ed4699815d6e`
- Source file: [[DNS from First Principles/Code/CHANGELOG.md.source|CHANGELOG.md]]
- Lines: 8-27
- Subsystem: [[DNS from First Principles/Subsystems/Repository and build|Repository and build]]

```rgbdns-fragment
{"id": "rgbdns-frag-ed4699815d6e", "codeNote": "DNS from First Principles/Code/CHANGELOG.md.source", "sourcePath": "CHANGELOG.md", "startLine": 8, "endLine": 27}
```

## Excerpt

<span id="rgbdns-frag-ed4699815d6e" class="rgbdns-fragment-target"></span>
### rgbdns-frag-ed4699815d6e: heading 0.6.5 - 2026-09-12

```markdown
## [0.6.5] - 2026-09-12

### Fixed

- `rgbdns-data-import` now works on a DNSSEC primary. In DNSSEC mode
  `compile-zone` only verifies the already published signed CDB, which the
  import's private stage does not contain, so every upload failed silently
  with status 1. The upload is now validated with the plain `tinydns-data`
  compiler, only the `data` source is replaced, and
  `rgbdns-dnssec-publish.service` (already started after the import) signs,
  verifies and publishes `data.cdb`. An unsigned database is never served.

### Added

- `DATA_DROP_IMPORT=disabled` in `/etc/rgbdns/data-drop.env`, written by
  `rgbdns-setup primary --data-drop-import disabled`, skips the data import
  for primaries whose data is deployed into the state directory by other
  means. `rgbdns-data-import.service` checks it with `ExecCondition=`, so a
  disabled run is skipped rather than failed.

```
