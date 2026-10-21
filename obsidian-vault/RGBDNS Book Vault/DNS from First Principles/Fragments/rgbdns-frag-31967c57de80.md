---
type: "code-fragment"
fragment_id: "rgbdns-frag-31967c57de80"
source_path: "RGBDNS_LETS.md"
code_note: "DNS from First Principles/Code/RGBDNS_LETS.md.source"
language: "markdown"
subsystem: "Repository and build"
symbol: "Security properties"
kind: "heading"
start_line: 232
end_line: 248
---

# Security properties

- Fragment ID: `rgbdns-frag-31967c57de80`
- Source file: [[DNS from First Principles/Code/RGBDNS_LETS.md.source|RGBDNS_LETS.md]]
- Lines: 232-248
- Subsystem: [[DNS from First Principles/Subsystems/Repository and build|Repository and build]]

```rgbdns-fragment
{"id": "rgbdns-frag-31967c57de80", "codeNote": "DNS from First Principles/Code/RGBDNS_LETS.md.source", "sourcePath": "RGBDNS_LETS.md", "startLine": 232, "endLine": 248}
```

## Excerpt

<span id="rgbdns-frag-31967c57de80" class="rgbdns-fragment-target"></span>
### rgbdns-frag-31967c57de80: heading Security properties

```markdown
## Security properties

- Network updates are disabled by default and fail closed on bad config.
- There is no unauthenticated source-address authorization fallback.
- A compromised certificate host receives authority only over configured ACME
  TXT owners, not the zone.
- TCP-only operation and strict size limits constrain amplification and memory
  use.
- Replay resistance comes from TSIG signed time and bounded fudge. The server
  also keeps a bounded recent-MAC cache for the acceptance window and rejects
  an identical authenticated request twice.
- File permissions, atomic replacement, durable writes, and no-follow opens
  protect local state.
- Logs do not contain TSIG secrets or ACME token values.
- Failed authentication, authorization, prerequisites, persistence, merge, or
  publication cannot partially apply a multi-operation update.

```
