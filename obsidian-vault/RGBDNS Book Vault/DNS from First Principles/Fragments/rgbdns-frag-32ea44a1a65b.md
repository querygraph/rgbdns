---
type: "code-fragment"
fragment_id: "rgbdns-frag-32ea44a1a65b"
source_path: "RGBDNS_LETS.md"
code_note: "DNS from First Principles/Code/RGBDNS_LETS.md.source"
language: "markdown"
subsystem: "Repository and build"
symbol: "Zone data and serials"
kind: "heading"
start_line: 159
end_line: 183
---

# Zone data and serials

- Fragment ID: `rgbdns-frag-32ea44a1a65b`
- Source file: [[DNS from First Principles/Code/RGBDNS_LETS.md.source|RGBDNS_LETS.md]]
- Lines: 159-183
- Subsystem: [[DNS from First Principles/Subsystems/Repository and build|Repository and build]]

```rgbdns-fragment
{"id": "rgbdns-frag-32ea44a1a65b", "codeNote": "DNS from First Principles/Code/RGBDNS_LETS.md.source", "sourcePath": "RGBDNS_LETS.md", "startLine": 159, "endLine": 183}
```

## Excerpt

<span id="rgbdns-frag-32ea44a1a65b" class="rgbdns-fragment-target"></span>
### rgbdns-frag-32ea44a1a65b: heading Zone data and serials

````markdown
## Zone data and serials

The durable overlay is generated tinydns text containing escaped TXT lines:

```text
'_acme-challenge.chiefscientist.org:token:60
```

Generated values use tinydns octal escapes where required, are sorted by owner
and value, and are parsed by the normal zone parser before activation. Dynamic
data may coexist only with TXT records at the owner; a canonical CNAME or other
conflicting data causes publication to fail safely.

For every affected primary zone the update serial becomes:

```text
max(previous_dynamic_serial + 1, canonical_serial + 1, current_unix_time)
```

with RFC 1982 wrapping rules. The logical zone replaces that zone's SOA serial
in authoritative answers and AXFR without rewriting the operator's canonical
SOA line. Overlay TXT records appear in ordinary lookup and AXFR. Standard and
rgbdns secondaries therefore receive the same public DNS-01 state; no private
transfer extension is involved.

````
