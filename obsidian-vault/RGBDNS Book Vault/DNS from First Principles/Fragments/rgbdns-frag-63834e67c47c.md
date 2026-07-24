---
type: "code-fragment"
fragment_id: "rgbdns-frag-63834e67c47c"
source_path: "docs/book/rgbdns.md"
code_note: "DNS from First Principles/Code/docs/book/rgbdns.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "tinydns data as a source language"
kind: "heading"
start_line: 388
end_line: 426
---

# tinydns data as a source language

- Fragment ID: `rgbdns-frag-63834e67c47c`
- Source file: [[DNS from First Principles/Code/docs/book/rgbdns.md.source|docs/book/rgbdns.md]]
- Lines: 388-426
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-63834e67c47c", "codeNote": "DNS from First Principles/Code/docs/book/rgbdns.md.source", "heading": "rgbdns-frag-63834e67c47c: heading tinydns data as a source language", "sourcePath": "docs/book/rgbdns.md", "startLine": 388, "endLine": 426}
```

## Excerpt

<span id="rgbdns-frag-63834e67c47c" class="rgbdns-fragment-target"></span>
### rgbdns-frag-63834e67c47c: heading tinydns data as a source language

```markdown
## tinydns data as a source language

djbdns uses a compact line-oriented zone source called `data`. The first
character selects a record form. Common forms include:

| Prefix | Meaning |
|---|---|
| `.` | zone authority plus NS and address data |
| `&` | delegation NS and optional glue |
| `Z` | explicit SOA |
| `=` | A plus matching reverse PTR |
| `+` | A only |
| `6` | AAAA plus reverse PTR forms |
| `3` | AAAA only |
| `@` | MX and optional exchanger address |
| `C` | CNAME |
| `^` | PTR |
| `'` | TXT |
| `S` | SRV |
| `:` | generic record |
| `%` | client-location mapping |

Fields are colon-separated with octal escapes for bytes that would otherwise
be ambiguous. Optional fields carry TTL, timestamp, and location information.
The format is terse because it was designed for mechanical generation as well
as hand editing.

`Zone::parse` reads this language line by line. It ignores blank, comment, and
disabled lines; reports the failing line number; expands convenience forms
into ordinary typed records; validates IPv4, flat 32-digit IPv6, names, numeric
ranges, and escaped bytes; and records authoritative and delegation structure.
When an SOA serial is omitted, file loading derives a nonzero default from the
source modification time.

Timestamp fields use TAI64-style cutoffs. Depending on the marker, a record can
be visible before or after a specified instant. Location codes select records
using configured client IPv4 prefixes. rgbdns carries that metadata beside the
record and evaluates it at lookup time.

```
