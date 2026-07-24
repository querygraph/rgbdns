---
type: "code-fragment"
fragment_id: "rgbdns-frag-4c4b17d589da"
source_path: "docs/book/rgbdns.md"
code_note: "DNS from First Principles/Code/docs/book/rgbdns.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "Build, stage, verify, replace"
kind: "heading"
start_line: 908
end_line: 927
---

# Build, stage, verify, replace

- Fragment ID: `rgbdns-frag-4c4b17d589da`
- Source file: [[DNS from First Principles/Code/docs/book/rgbdns.md.source|docs/book/rgbdns.md]]
- Lines: 908-927
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-4c4b17d589da", "codeNote": "DNS from First Principles/Code/docs/book/rgbdns.md.source", "heading": "rgbdns-frag-4c4b17d589da: heading Build, stage, verify, replace", "sourcePath": "docs/book/rgbdns.md", "startLine": 908, "endLine": 927}
```

## Excerpt

<span id="rgbdns-frag-4c4b17d589da" class="rgbdns-fragment-target"></span>
### rgbdns-frag-4c4b17d589da: heading Build, stage, verify, replace

```markdown
## Build, stage, verify, replace

A safe publication cycle separates source editing from serving:

```sh
cd /etc/rgbdns
tinydns-data
tinydns-get example.com A www.example.com
```

In production, compile in a staging directory, run representative exact,
wildcard, delegation, negative, IPv4, IPv6, and large-response queries, then
atomically replace `data.cdb`. Retain the previous known-good database for
rollback. Query the bound service over both UDP and TCP after deployment.

Do not expose the recursive service to arbitrary networks by accident. The
default `ALLOW_NETS` is loopback only because an open resolver can be abused
for amplification and can consume local capacity. Likewise, expand AXFR
allowlists only for intended secondaries.

```
