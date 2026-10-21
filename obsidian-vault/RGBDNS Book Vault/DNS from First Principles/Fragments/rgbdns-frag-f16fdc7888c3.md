---
type: "code-fragment"
fragment_id: "rgbdns-frag-f16fdc7888c3"
source_path: "docs/work-review-1.md"
code_note: "DNS from First Principles/Code/docs/work-review-1.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "Recommendation"
kind: "heading"
start_line: 106
end_line: 120
---

# Recommendation

- Fragment ID: `rgbdns-frag-f16fdc7888c3`
- Source file: [[DNS from First Principles/Code/docs/work-review-1.md.source|docs/work-review-1.md]]
- Lines: 106-120
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-f16fdc7888c3", "codeNote": "DNS from First Principles/Code/docs/work-review-1.md.source", "sourcePath": "docs/work-review-1.md", "startLine": 106, "endLine": 120}
```

## Excerpt

<span id="rgbdns-frag-f16fdc7888c3" class="rgbdns-fragment-target"></span>
### rgbdns-frag-f16fdc7888c3: heading Recommendation

```markdown
## Recommendation

The package machinery is strong. Before production deployment on the current
machine:

1. Combine authoritative TCP and AXFR handling on `52.10.53.234:53`.
2. Make `rgbdns-setup` restart already-running services after configuration
   changes.
3. Ensure zone reloads restart both tinydns and axfrdns.
4. Cleanly disable stale units and configuration when changing server roles.
5. Adopt Debian package revisions for every packaging release.

After those changes, the package will support the intended architecture:
rgbdns as the public primary and canonical zone source, with BuddyNS providing
secondary authoritative service through AXFR.
```
