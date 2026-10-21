---
type: "code-fragment"
fragment_id: "rgbdns-frag-a58c3bf7ea93"
source_path: "RGBDNS_LETS.md"
code_note: "DNS from First Principles/Code/RGBDNS_LETS.md.source"
language: "markdown"
subsystem: "Repository and build"
symbol: "key-name algorithm base64-secret zone owner-prefix ttl"
kind: "heading"
start_line: 95
end_line: 115
---

# key-name algorithm base64-secret zone owner-prefix ttl

- Fragment ID: `rgbdns-frag-a58c3bf7ea93`
- Source file: [[DNS from First Principles/Code/RGBDNS_LETS.md.source|RGBDNS_LETS.md]]
- Lines: 95-115
- Subsystem: [[DNS from First Principles/Subsystems/Repository and build|Repository and build]]

```rgbdns-fragment
{"id": "rgbdns-frag-a58c3bf7ea93", "codeNote": "DNS from First Principles/Code/RGBDNS_LETS.md.source", "sourcePath": "RGBDNS_LETS.md", "startLine": 95, "endLine": 115}
```

## Excerpt

<span id="rgbdns-frag-a58c3bf7ea93" class="rgbdns-fragment-target"></span>
### rgbdns-frag-a58c3bf7ea93: heading key-name algorithm base64-secret zone owner-prefix ttl

````markdown
# key-name  algorithm     base64-secret  zone                  owner-prefix       ttl
certbot-chiefscientist. hmac-sha256. SECRET chiefscientist.org. _acme-challenge. 60
```

Fields have these meanings:

- `key-name`: absolute TSIG owner, compared case-insensitively.
- `algorithm`: exactly `hmac-sha256.` (the common aliases are accepted on the
  wire).
- `base64-secret`: decoded key bytes, at least 16 and at most 64 bytes.
- `zone`: absolute primary zone authorized for this key.
- `owner-prefix`: exactly `_acme-challenge.` in 0.4.0. It authorizes both
  `_acme-challenge.<zone>` and `_acme-challenge.<validated-name>.<zone>`.
- `ttl`: positive TTL from 30 through 300 seconds. The server replaces client
  TTLs with this policy TTL.

Repeated key names are rejected. Configuration, state, and canonical data are
bounded. The implementation rejects more than 256 keys, 256 managed zones,
128 live values per owner, 4096 total live challenge values, 255 bytes per TXT
RDATA, or 64 KiB of generated overlay state.

````
