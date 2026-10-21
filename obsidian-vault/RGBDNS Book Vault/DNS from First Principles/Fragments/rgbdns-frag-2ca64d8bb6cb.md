---
type: "code-fragment"
fragment_id: "rgbdns-frag-2ca64d8bb6cb"
source_path: "docs/blog/rgbdns-dnssec/post.md"
code_note: "DNS from First Principles/Code/docs/blog/rgbdns-dnssec/post.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "Five programs, five jobs"
kind: "heading"
start_line: 24
end_line: 50
---

# Five programs, five jobs

- Fragment ID: `rgbdns-frag-2ca64d8bb6cb`
- Source file: [[DNS from First Principles/Code/docs/blog/rgbdns-dnssec/post.md.source|docs/blog/rgbdns-dnssec/post.md]]
- Lines: 24-50
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-2ca64d8bb6cb", "codeNote": "DNS from First Principles/Code/docs/blog/rgbdns-dnssec/post.md.source", "sourcePath": "docs/blog/rgbdns-dnssec/post.md", "startLine": 24, "endLine": 50}
```

## Excerpt

<span id="rgbdns-frag-2ca64d8bb6cb" class="rgbdns-fragment-target"></span>
### rgbdns-frag-2ca64d8bb6cb: heading Five programs, five jobs

````markdown
## Five programs, five jobs

The new commands are deliberately unambitious in isolation:

- `rgbsec-keygen` creates one ECDSA P-256 key and prints its policy line.
- `rgbsec-sign` turns tinydns text into inspectable signed tinydns text.
- `rgbsec-data` signs and compiles directly to CDB.
- `rgbsec-ds` derives the DS record that belongs at the parent.
- `rgbsec-check` verifies every signed RRset, the NSEC cycle, and the remaining
  signature lifetime.

The policy is one line per authoritative zone:

```text
Kexample.com.:/etc/rgbdns/keys/example.com.pk8:13:1209600:86400:3600
Ulegacy.example.
```

`K` means that the zone is signed. `U` means that it is deliberately unsigned.
Every zone in the source must have exactly one disposition. Missing,
duplicated, or conflicting lines stop publication. An omitted line never
silently changes a zone's security state.

That makes mixed operation possible without making it ambiguous. A stable zone
can be signed while a neighboring zone continues to use location-qualified
records, live ACME updates, or runtime ANAME behavior.

````
