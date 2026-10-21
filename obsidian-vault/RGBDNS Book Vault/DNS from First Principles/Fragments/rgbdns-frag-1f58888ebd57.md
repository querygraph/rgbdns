---
type: "code-fragment"
fragment_id: "rgbdns-frag-1f58888ebd57"
source_path: "RGBDNS_LETS.md"
code_note: "DNS from First Principles/Code/RGBDNS_LETS.md.source"
language: "markdown"
subsystem: "Repository and build"
symbol: "Canonical import interaction"
kind: "heading"
start_line: 184
end_line: 196
---

# Canonical import interaction

- Fragment ID: `rgbdns-frag-1f58888ebd57`
- Source file: [[DNS from First Principles/Code/RGBDNS_LETS.md.source|RGBDNS_LETS.md]]
- Lines: 184-196
- Subsystem: [[DNS from First Principles/Subsystems/Repository and build|Repository and build]]

```rgbdns-fragment
{"id": "rgbdns-frag-1f58888ebd57", "codeNote": "DNS from First Principles/Code/RGBDNS_LETS.md.source", "sourcePath": "RGBDNS_LETS.md", "startLine": 184, "endLine": 196}
```

## Excerpt

<span id="rgbdns-frag-1f58888ebd57" class="rgbdns-fragment-target"></span>
### rgbdns-frag-1f58888ebd57: heading Canonical import interaction

```markdown
## Canonical import interaction

`rgbdns-import-data` must preserve the operator's existing atomic delivery
contract. Its staging compilation validates canonical data alone and then the
canonical-plus-overlay logical snapshot. It holds the shared update lock only
for final revalidation and activation. If merge validation fails, neither
canonical files nor ACME state changes.

A canonical import may add a permanent TXT record at an ACME owner. It may not
add a CNAME or non-TXT data at an owner with live challenge state. Operators
must first allow challenge cleanup or explicitly clear the overlay with the
administrative command.

```
