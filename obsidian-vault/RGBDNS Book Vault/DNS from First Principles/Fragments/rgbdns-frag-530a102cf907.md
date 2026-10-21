---
type: "code-fragment"
fragment_id: "rgbdns-frag-530a102cf907"
source_path: "README.md"
code_note: "DNS from First Principles/Code/README.md.source"
language: "markdown"
subsystem: "Repository and build"
symbol: "Optional authoritative DNSSEC"
kind: "heading"
start_line: 33
end_line: 48
---

# Optional authoritative DNSSEC

- Fragment ID: `rgbdns-frag-530a102cf907`
- Source file: [[DNS from First Principles/Code/README.md.source|README.md]]
- Lines: 33-48
- Subsystem: [[DNS from First Principles/Subsystems/Repository and build|Repository and build]]

```rgbdns-fragment
{"id": "rgbdns-frag-530a102cf907", "codeNote": "DNS from First Principles/Code/README.md.source", "sourcePath": "README.md", "startLine": 33, "endLine": 48}
```

## Excerpt

<span id="rgbdns-frag-530a102cf907" class="rgbdns-fragment-target"></span>
### rgbdns-frag-530a102cf907: heading Optional authoritative DNSSEC

```markdown
## Optional authoritative DNSSEC

Authoritative DNSSEC is entirely opt-in. With no `dnssec` policy file, rgbdns
uses the original djbdns-compatible `data` → `tinydns-data` → `data.cdb` path
and preserves its existing query, referral, ANAME, ACME, and service behavior.

When enabled, small utilities compose an offline signed snapshot: `rgbsec-keygen`
creates a protected ECDSA P-256 key, `aname-materialize` converts ANAME answers
to ordinary A/AAAA records, `rgbsec-sign` adds DNSKEY/NSEC/RRSIG records,
`rgbsec-data` compiles atomically, `rgbsec-ds` prints the parent DS, and
`rgbsec-check` verifies every signature and its remaining lifetime. `tinydns`
and all secondaries remain keyless; standard AXFR carries the finished DNSSEC
records. A `K` policy line signs a zone and a `U` line explicitly preserves an
unsigned zone, so one CDB can safely serve both. See
[`docs/DNSSEC.md`](docs/DNSSEC.md).

```
