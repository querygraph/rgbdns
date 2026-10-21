---
type: "code-file"
source_path: "packaging/default/tinydns.env"
language: ""
subsystem: "Repository and build"
line_count: 5
fragment_count: 1
rgbdns_commit: "5ae24772"
---

# packaging/default/tinydns.env

- Subsystem: [[DNS from First Principles/Subsystems/Repository and build|Repository and build]]
- Source path: `packaging/default/tinydns.env`
- Lines: 5
- Summary: Managed defaults for rgbdns-tinydns.service.

## Extracted Fragments

- [[DNS from First Principles/Fragments/rgbdns-frag-33f633dfa15c|tinydns.env]]: lines 1-5

## Full Source

```
# Managed defaults for rgbdns-tinydns.service.
IP=0.0.0.0
PORT=53
DATA=/var/lib/rgbdns/tinydns/data.cdb
QUERY_LOG=1
```
