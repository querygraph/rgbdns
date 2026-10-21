---
type: "code-fragment"
fragment_id: "rgbdns-frag-824f09e1469c"
source_path: "docs/DNSSEC-DESIGN.md"
code_note: "DNS from First Principles/Code/docs/DNSSEC-DESIGN.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "One-line policy"
kind: "heading"
start_line: 44
end_line: 79
---

# One-line policy

- Fragment ID: `rgbdns-frag-824f09e1469c`
- Source file: [[DNS from First Principles/Code/docs/DNSSEC-DESIGN.md.source|docs/DNSSEC-DESIGN.md]]
- Lines: 44-79
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-824f09e1469c", "codeNote": "DNS from First Principles/Code/docs/DNSSEC-DESIGN.md.source", "sourcePath": "docs/DNSSEC-DESIGN.md", "startLine": 44, "endLine": 79}
```

## Excerpt

<span id="rgbdns-frag-824f09e1469c" class="rgbdns-fragment-target"></span>
### rgbdns-frag-824f09e1469c: heading One-line policy

````markdown
## One-line policy

The file named `dnssec` contains comments, blank lines, and exactly one
disposition line per authoritative zone:

```text
Kexample.com:/etc/rgbdns/keys/example.com.pk8:13:1209600:86400:3600
Ulegacy.example
```

Fields are colon-delimited and contain no whitespace:

```text
Kzone:keyfile:algorithm:validity:refresh:inception-skew
```

- `zone` is an absolute DNS name in rgbdns text form; a final dot is optional.
- `keyfile` is an absolute path to a PKCS#8 DER private key.
- `algorithm` is `13` (ECDSAP256SHA256). Unknown algorithms are rejected.
- `validity` is the RRSIG lifetime in seconds.
- `refresh` is the minimum remaining lifetime accepted by `rgbsec-check`.
- `inception-skew` moves signature inception into the past to tolerate clock
  skew.

`Uzone` explicitly retains a zone as unsigned. It has no other fields and does
not weaken coverage checking: every authoritative zone must have exactly one
`K` or `U` line, duplicate or missing dispositions fail publication, and a
zone marked `U` must contain no stale DNSKEY, RRSIG, NSEC, or NSEC3 signing
records.

The initial release uses one combined signing key per zone. Multiple active
`K` lines for a zone are reserved for an explicit rollover release; accepting
them now would promise rollover semantics that do not yet exist. Bounds are
enforced for every duration. Key material never appears in `data`, `dnssec`,
logs, stdout, CDB metadata, or AXFR-only private records.

````
