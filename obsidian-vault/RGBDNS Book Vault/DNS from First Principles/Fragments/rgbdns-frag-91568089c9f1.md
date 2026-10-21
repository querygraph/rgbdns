---
type: "code-fragment"
fragment_id: "rgbdns-frag-91568089c9f1"
source_path: "docs/book/rgbdns.md"
code_note: "DNS from First Principles/Code/docs/book/rgbdns.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "One line per zone"
kind: "heading"
start_line: 735
end_line: 762
---

# One line per zone

- Fragment ID: `rgbdns-frag-91568089c9f1`
- Source file: [[DNS from First Principles/Code/docs/book/rgbdns.md.source|docs/book/rgbdns.md]]
- Lines: 735-762
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-91568089c9f1", "codeNote": "DNS from First Principles/Code/docs/book/rgbdns.md.source", "sourcePath": "docs/book/rgbdns.md", "startLine": 735, "endLine": 762}
```

## Excerpt

<span id="rgbdns-frag-91568089c9f1" class="rgbdns-fragment-target"></span>
### rgbdns-frag-91568089c9f1: heading One line per zone

````markdown
## One line per zone

The policy file is intentionally close to tinydns data: comments, blank lines,
and exactly one disposition line for every authoritative zone. `K` means sign;
`U` means explicitly unsigned:

```text
Kexample.com.:/etc/rgbdns/keys/example.com.pk8:13:1209600:86400:3600
Ulegacy.example.
```

The signed line has this shape:

```text
Kzone:keyfile:algorithm:validity:refresh:inception-skew
```

Algorithm 13 is ECDSA P-256 with SHA-256. The example signs for fourteen days,
requires at least one day of remaining lifetime, and moves inception one hour
into the past to tolerate modest clock skew. The key path is absolute. Missing,
duplicate, or conflicting dispositions stop publication; omission never means
“probably unsigned.” A `U` zone is also checked for stale DNSKEY, RRSIG, NSEC,
or NSEC3 records.

This fail-closed coverage rule is what makes mixed snapshots understandable.
The policy is not merely a list of zones that happened to be signed today. It
is the security disposition of the complete authoritative source.

````
