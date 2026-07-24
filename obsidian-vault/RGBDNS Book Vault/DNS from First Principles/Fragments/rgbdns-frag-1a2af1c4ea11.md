---
type: "code-fragment"
fragment_id: "rgbdns-frag-1a2af1c4ea11"
source_path: "docs/book/rgbdns.md"
code_note: "DNS from First Principles/Code/docs/book/rgbdns.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "The service contract"
kind: "heading"
start_line: 757
end_line: 777
---

# The service contract

- Fragment ID: `rgbdns-frag-1a2af1c4ea11`
- Source file: [[DNS from First Principles/Code/docs/book/rgbdns.md.source|docs/book/rgbdns.md]]
- Lines: 757-777
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-1a2af1c4ea11", "codeNote": "DNS from First Principles/Code/docs/book/rgbdns.md.source", "heading": "rgbdns-frag-1a2af1c4ea11: heading The service contract", "sourcePath": "docs/book/rgbdns.md", "startLine": 757, "endLine": 777}
```

## Excerpt

<span id="rgbdns-frag-1a2af1c4ea11" class="rgbdns-fragment-target"></span>
### rgbdns-frag-1a2af1c4ea11: heading The service contract

```markdown
## The service contract

rgbdns daemons run in the foreground, emit diagnostics to standard error, take
configuration from files and environment, and terminate on fatal startup
errors. That is the portable contract a supervisor needs. The generated
djbdns-style directories additionally provide `run` and `log/run` programs,
but the daemon binaries do not require a particular supervisor.

The classic daemontools control plane is:

```text
supervise service/       keep one process running
svc -u service/          bring it up
svc -d service/          bring it down
svc -t service/          send TERM
svstat service/          inspect status
```

No modern replacement is universally best. Choose according to the host and
the migration boundary.

```
