---
type: "code-fragment"
fragment_id: "rgbdns-frag-c3c2a07d858c"
source_path: "docs/book/rgbdns.md"
code_note: "DNS from First Principles/Code/docs/book/rgbdns.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "Least privilege and filesystem boundaries"
kind: "heading"
start_line: 706
end_line: 722
---

# Least privilege and filesystem boundaries

- Fragment ID: `rgbdns-frag-c3c2a07d858c`
- Source file: [[DNS from First Principles/Code/docs/book/rgbdns.md.source|docs/book/rgbdns.md]]
- Lines: 706-722
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-c3c2a07d858c", "codeNote": "DNS from First Principles/Code/docs/book/rgbdns.md.source", "heading": "rgbdns-frag-c3c2a07d858c: heading Least privilege and filesystem boundaries", "sourcePath": "docs/book/rgbdns.md", "startLine": 706, "endLine": 722}
```

## Excerpt

<span id="rgbdns-frag-c3c2a07d858c" class="rgbdns-fragment-target"></span>
### rgbdns-frag-c3c2a07d858c: heading Least privilege and filesystem boundaries

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
