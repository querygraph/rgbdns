---
type: "code-fragment"
fragment_id: "rgbdns-frag-7c984d59305e"
source_path: "RGBDNS_LETS.md"
code_note: "DNS from First Principles/Code/RGBDNS_LETS.md.source"
language: "markdown"
subsystem: "Repository and build"
symbol: "DNS UPDATE profile"
kind: "heading"
start_line: 116
end_line: 138
---

# DNS UPDATE profile

- Fragment ID: `rgbdns-frag-7c984d59305e`
- Source file: [[DNS from First Principles/Code/RGBDNS_LETS.md.source|RGBDNS_LETS.md]]
- Lines: 116-138
- Subsystem: [[DNS from First Principles/Subsystems/Repository and build|Repository and build]]

```rgbdns-fragment
{"id": "rgbdns-frag-7c984d59305e", "codeNote": "DNS from First Principles/Code/RGBDNS_LETS.md.source", "sourcePath": "RGBDNS_LETS.md", "startLine": 116, "endLine": 138}
```

## Excerpt

<span id="rgbdns-frag-7c984d59305e" class="rgbdns-fragment-target"></span>
### rgbdns-frag-7c984d59305e: heading DNS UPDATE profile

```markdown
## DNS UPDATE profile

The message opcode is UPDATE (5). The Zone section contains exactly one SOA
question in class IN. The Update section may contain only these operations:

- Add one TXT value: class IN, type TXT, nonzero TTL, one or more character
  strings whose concatenation is at most 255 bytes.
- Delete one TXT value: class NONE, type TXT, TTL zero, matching RDATA.
- Delete the TXT RRset: class ANY, type TXT, TTL zero, empty RDATA. This is
  accepted for client cleanup but affects only the authorized owner.

Multiple operations are atomic. Names must be inside the Zone section zone and
must match the authenticated key's policy. All other record types, classes,
zone shapes, meta-records, and delete-name operations are refused.

The first implementation accepts an empty prerequisite section. It also
accepts the RFC 2136 value-independent "RRset exists" and "RRset does not
exist" TXT prerequisites at authorized owners. Unsupported prerequisites
return NOTIMP; false prerequisites return YXRRSET or NXRRSET as appropriate.

Only TCP UPDATE is processed. UPDATE received over UDP returns REFUSED. An
UPDATE sent to a secondary returns NOTAUTH.

```
