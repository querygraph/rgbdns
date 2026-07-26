---
type: "code-file"
source_path: "packaging/default/tinydns.env"
language: ""
subsystem: "Repository and build"
line_count: 4
fragment_count: 1
rgbdns_commit: "79502939"
---

# packaging/default/tinydns.env

- Subsystem: [[DNS from First Principles/Subsystems/Repository and build|Repository and build]]
- Source path: `packaging/default/tinydns.env`
- Lines: 4
- Summary: Managed defaults for rgbdns-tinydns.service.

## Extracted Fragments

- [[DNS from First Principles/Fragments/rgbdns-frag-211708b5ce49|tinydns.env]]: lines 1-4

## Full Source

```
# Managed defaults for rgbdns-tinydns.service.
IP=0.0.0.0
PORT=53
DATA=/var/lib/rgbdns/tinydns/data.cdb
```
