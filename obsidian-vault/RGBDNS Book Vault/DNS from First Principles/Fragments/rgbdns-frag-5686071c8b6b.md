---
type: "code-fragment"
fragment_id: "rgbdns-frag-5686071c8b6b"
source_path: "docs/blog/rgbdns-dnssec/post.md"
code_note: "DNS from First Principles/Code/docs/blog/rgbdns-dnssec/post.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "Evidence, not a checkbox"
kind: "heading"
start_line: 192
end_line: 219
---

# Evidence, not a checkbox

- Fragment ID: `rgbdns-frag-5686071c8b6b`
- Source file: [[DNS from First Principles/Code/docs/blog/rgbdns-dnssec/post.md.source|docs/blog/rgbdns-dnssec/post.md]]
- Lines: 192-219
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-5686071c8b6b", "codeNote": "DNS from First Principles/Code/docs/blog/rgbdns-dnssec/post.md.source", "sourcePath": "docs/blog/rgbdns-dnssec/post.md", "startLine": 192, "endLine": 219}
```

## Excerpt

<span id="rgbdns-frag-5686071c8b6b" class="rgbdns-fragment-target"></span>
### rgbdns-frag-5686071c8b6b: heading Evidence, not a checkbox

````markdown
## Evidence, not a checkbox

The strongest local command is:

```sh
sudo rgbsec-check \
  /var/lib/rgbdns/tinydns/data.cdb \
  /etc/rgbdns/dnssec
```

It exits zero only when every configured zone has the promised security state,
every authoritative RRset verifies, every NSEC cycle closes, and the earliest
signature remains outside its refresh window. Its tab-separated output gives
the zone, SOA serial, key tag, expiration, seconds remaining, and `ok`.

The public proof adds parent DS checks, direct UDP and TCP queries to every
authority, signed positive, NXDOMAIN, and NODATA answers, and AD validation
through several recursive networks. The lifecycle proof is still longer: watch
the publisher refresh signatures before they expire, restart each authority,
and rehearse recovery from the protected primary-only key backup.

That is DNSSEC in the spirit of tinydns: cryptographic protection without
making the serving process magical. Small boats, visible routes, independent
checks, and no single vessel carrying every secret.

The full design and operational examples are in the updated
[*DNS from First Principles*](https://firstpair.org/books/rgbdns/) and the repository's
[authoritative DNSSEC guide](https://github.com/querygraph/rgbdns/blob/master/docs/DNSSEC.md).
````
