---
type: "code-fragment"
fragment_id: "rgbdns-frag-31d9d1d37c9e"
source_path: "docs/RGBDNS_SETUP.md"
code_note: "DNS from First Principles/Code/docs/RGBDNS_SETUP.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "1. Understand the two zones"
kind: "heading"
start_line: 27
end_line: 45
---

# 1. Understand the two zones

- Fragment ID: `rgbdns-frag-31d9d1d37c9e`
- Source file: [[DNS from First Principles/Code/docs/RGBDNS_SETUP.md.source|docs/RGBDNS_SETUP.md]]
- Lines: 27-45
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-31d9d1d37c9e", "codeNote": "DNS from First Principles/Code/docs/RGBDNS_SETUP.md.source", "sourcePath": "docs/RGBDNS_SETUP.md", "startLine": 27, "endLine": 45}
```

## Excerpt

<span id="rgbdns-frag-31d9d1d37c9e" class="rgbdns-fragment-target"></span>
### rgbdns-frag-31d9d1d37c9e: heading 1. Understand the two zones

````markdown
## 1. Understand the two zones

`fieldnotes.es` delegates authority to names beneath `cron.sh`:

```text
a.ns.cron.sh
b.ns.cron.sh
```

Consequently, the primary must serve:

- `cron.sh`, which contains the address records for `a.ns.cron.sh` and
  `b.ns.cron.sh`; and
- `fieldnotes.es`, the application zone.

This walkthrough makes `a`, `b`, and BuddyNS authoritative for both zones. If
`b` is not published as an authority for `cron.sh`, it may transfer only
`fieldnotes.es`; keep `cron.sh` out of its list in that case.

````
