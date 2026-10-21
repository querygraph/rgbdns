---
type: "code-fragment"
fragment_id: "rgbdns-frag-46ea9a62f255"
source_path: "RGBDNS_LETS.md"
code_note: "DNS from First Principles/Code/RGBDNS_LETS.md.source"
language: "markdown"
subsystem: "Repository and build"
symbol: "Configuration"
kind: "heading"
start_line: 82
end_line: 94
---

# Configuration

- Fragment ID: `rgbdns-frag-46ea9a62f255`
- Source file: [[DNS from First Principles/Code/RGBDNS_LETS.md.source|RGBDNS_LETS.md]]
- Lines: 82-94
- Subsystem: [[DNS from First Principles/Subsystems/Repository and build|Repository and build]]

```rgbdns-fragment
{"id": "rgbdns-frag-46ea9a62f255", "codeNote": "DNS from First Principles/Code/RGBDNS_LETS.md.source", "sourcePath": "RGBDNS_LETS.md", "startLine": 82, "endLine": 94}
```

## Excerpt

<span id="rgbdns-frag-46ea9a62f255" class="rgbdns-fragment-target"></span>
### rgbdns-frag-46ea9a62f255: heading Configuration

````markdown
## Configuration

ACME updates are disabled when `ACME_UPDATE_CONFIG` is absent or empty. A
primary enables them with:

```text
ACME_UPDATE_CONFIG=/etc/rgbdns/acme-update.conf
ACME_STATE_DIR=/var/lib/rgbdns/tinydns
```

The credential file uses one tab-separated policy per non-comment line:

```text
````
