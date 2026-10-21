---
type: "code-fragment"
fragment_id: "rgbdns-frag-50bcd452c709"
source_path: "docs/blog/operationalizing-rgbdns/post.md"
code_note: "DNS from First Principles/Code/docs/blog/operationalizing-rgbdns/post.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "Verify the system at its boundaries"
kind: "heading"
start_line: 148
end_line: 181
---

# Verify the system at its boundaries

- Fragment ID: `rgbdns-frag-50bcd452c709`
- Source file: [[DNS from First Principles/Code/docs/blog/operationalizing-rgbdns/post.md.source|docs/blog/operationalizing-rgbdns/post.md]]
- Lines: 148-181
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-50bcd452c709", "codeNote": "DNS from First Principles/Code/docs/blog/operationalizing-rgbdns/post.md.source", "sourcePath": "docs/blog/operationalizing-rgbdns/post.md", "startLine": 148, "endLine": 181}
```

## Excerpt

<span id="rgbdns-frag-50bcd452c709" class="rgbdns-fragment-target"></span>
### rgbdns-frag-50bcd452c709: heading Verify the system at its boundaries

````markdown
## Verify the system at its boundaries

On either authority, ordinary clients should see an authoritative address
answer, not the transfer representation:

```sh
dig @127.0.0.1 fieldnotes.es A +norecurse
dig @127.0.0.1 fieldnotes.es SOA +norecurse
```

On the secondary, the retained source proves that ANAME intent survived the
transfer:

```sh
sudo grep '^Afieldnotes\.es' \
  /var/lib/rgbdns/tinydns/secondary-zones/fieldnotes.es.data
```

The primary-to-secondary AXFR check belongs on the authorized VPC path, not on
an arbitrary workstation outside AWS:

```sh
dig @172.31.60.189 fieldnotes.es AXFR
```

Do not widen the AXFR allow-list merely to make a public test convenient. The
public checks are ordinary UDP and TCP DNS queries to `a.ns.cron.sh` and
`b.ns.cron.sh`; the transfer check runs from an authorized secondary address.

The result is a deliberately small operating model: one editable data file on
the primary, one declarative zone list on each secondary, atomic activation,
and independent authorities that can preserve and execute the same ANAME
intent.

````
