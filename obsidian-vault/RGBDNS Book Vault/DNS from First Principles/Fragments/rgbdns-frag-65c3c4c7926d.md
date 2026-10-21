---
type: "code-fragment"
fragment_id: "rgbdns-frag-65c3c4c7926d"
source_path: "docs/book/rgbdns.md"
code_note: "DNS from First Principles/Code/docs/book/rgbdns.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "Least privilege and filesystem boundaries"
kind: "heading"
start_line: 1189
end_line: 1205
---

# Least privilege and filesystem boundaries

- Fragment ID: `rgbdns-frag-65c3c4c7926d`
- Source file: [[DNS from First Principles/Code/docs/book/rgbdns.md.source|docs/book/rgbdns.md]]
- Lines: 1189-1205
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-65c3c4c7926d", "codeNote": "DNS from First Principles/Code/docs/book/rgbdns.md.source", "sourcePath": "docs/book/rgbdns.md", "startLine": 1189, "endLine": 1205}
```

## Excerpt

<span id="rgbdns-frag-65c3c4c7926d" class="rgbdns-fragment-target"></span>
### rgbdns-frag-65c3c4c7926d: heading Least privilege and filesystem boundaries

```markdown
## Least privilege and filesystem boundaries

The `*-conf` commands generate service directories whose run scripts execute
the daemon under a selected account. rgbdns’s `setuidgid` resolves the user and
group, initializes supplementary groups, drops GID and UID, verifies the
result, and directly replaces itself with the target program. Direct
replacement preserves signals and exit status and avoids an extra shell-owned
process.

Generated paths are shell-quoted and support binaries by absolute path.
Configuration writers reject unsafe existing file types and apply intentional
modes. CDB and AXFR update workflows install only complete outputs.

Privilege dropping is not a substitute for a restricted service account,
read-only data, network policy, or supervisor hardening. It is one layer in a
deployment.

```
