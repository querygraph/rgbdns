---
type: "code-fragment"
fragment_id: "rgbdns-frag-2f6290042d8d"
source_path: "RGBDNS_LETS.md"
code_note: "DNS from First Principles/Code/RGBDNS_LETS.md.source"
language: "markdown"
subsystem: "Repository and build"
symbol: "Administrative command"
kind: "heading"
start_line: 197
end_line: 214
---

# Administrative command

- Fragment ID: `rgbdns-frag-2f6290042d8d`
- Source file: [[DNS from First Principles/Code/RGBDNS_LETS.md.source|RGBDNS_LETS.md]]
- Lines: 197-214
- Subsystem: [[DNS from First Principles/Subsystems/Repository and build|Repository and build]]

```rgbdns-fragment
{"id": "rgbdns-frag-2f6290042d8d", "codeNote": "DNS from First Principles/Code/RGBDNS_LETS.md.source", "sourcePath": "RGBDNS_LETS.md", "startLine": 197, "endLine": 214}
```

## Excerpt

<span id="rgbdns-frag-2f6290042d8d" class="rgbdns-fragment-target"></span>
### rgbdns-frag-2f6290042d8d: heading Administrative command

````markdown
## Administrative command

`rgbdns-acme` provides local, root/operator-controlled state management and
Certbot manual-hook compatibility:

```text
rgbdns-acme present --zone ZONE --name OWNER --value VALUE
rgbdns-acme cleanup --zone ZONE --name OWNER --value VALUE
rgbdns-acme clear --zone ZONE [--name OWNER]
rgbdns-acme list [--zone ZONE]
```

`present` and `cleanup` use the same policy validation, lock, transaction, and
publication mechanism as network UPDATE. They do not require or expose a TSIG
secret. Mutating commands require root or the configured service account.
Machine-readable output is available with `--json`. The command returns only
after the local primary serves the requested state.

````
