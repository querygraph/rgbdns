---
type: "code-fragment"
fragment_id: "rgbdns-frag-46cfc7f2c472"
source_path: "docs/work-review-1.md"
code_note: "DNS from First Principles/Code/docs/work-review-1.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "Additional issues"
kind: "heading"
start_line: 77
end_line: 91
---

# Additional issues

- Fragment ID: `rgbdns-frag-46cfc7f2c472`
- Source file: [[DNS from First Principles/Code/docs/work-review-1.md.source|docs/work-review-1.md]]
- Lines: 77-91
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-46cfc7f2c472", "codeNote": "DNS from First Principles/Code/docs/work-review-1.md.source", "sourcePath": "docs/work-review-1.md", "startLine": 77, "endLine": 91}
```

## Excerpt

<span id="rgbdns-frag-46cfc7f2c472" class="rgbdns-fragment-target"></span>
### rgbdns-frag-46cfc7f2c472: heading Additional issues

```markdown
## Additional issues

- Switching from primary to secondary does not disable a previously enabled
  AXFR service.
- Switching from secondary to primary does not disable the secondary
  synchronization timer.
- Omitting AXFR options during a later primary setup leaves the old
  `/etc/rgbdns/axfrdns.env` and potentially its running service behind.
- The package version remains `0.1.0`. Rebuilding changed packages under the
  same version makes upgrades ambiguous. Debian revisions such as `0.1.0-1`
  and `0.1.0-2` should be used.
- AXFR authentication uses source-address allow-listing and does not implement
  TSIG. This is acceptable for BuddyNS when both AWS and rgbdns restrict
  transfers to the exact BuddyNS transfer addresses.

```
