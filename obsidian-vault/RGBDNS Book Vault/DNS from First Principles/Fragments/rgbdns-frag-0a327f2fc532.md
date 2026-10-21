---
type: "code-fragment"
fragment_id: "rgbdns-frag-0a327f2fc532"
source_path: "docs/book/rgbdns.md"
code_note: "DNS from First Principles/Code/docs/book/rgbdns.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "Mixed zones and a keyless secondary"
kind: "heading"
start_line: 831
end_line: 881
---

# Mixed zones and a keyless secondary

- Fragment ID: `rgbdns-frag-0a327f2fc532`
- Source file: [[DNS from First Principles/Code/docs/book/rgbdns.md.source|docs/book/rgbdns.md]]
- Lines: 831-881
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-0a327f2fc532", "codeNote": "DNS from First Principles/Code/docs/book/rgbdns.md.source", "sourcePath": "docs/book/rgbdns.md", "startLine": 831, "endLine": 881}
```

## Excerpt

<span id="rgbdns-frag-0a327f2fc532" class="rgbdns-fragment-target"></span>
### rgbdns-frag-0a327f2fc532: heading Mixed zones and a keyless secondary

````markdown
## Mixed zones and a keyless secondary

One CDB can serve a stable signed zone beside a zone that must remain unsigned:

```text
Kexample.com.:/etc/rgbdns/keys/example.com.pk8:13:1209600:86400:3600
Uacme-live.example.
```

The source contains both SOAs and both NS sets. `rgbsec-data` signs only
`example.com`; it preserves ordinary records, qualified records, cutoffs, and
ANAME directives in the `U` zone. This behavior is explicit rather than
inferred from the absence of a key.

A secondary needs neither the policy nor the private key. Configure it in the
ordinary way:

```sh
sudo rgbdns-setup secondary \
  --zone example.com \
  --zone acme-live.example \
  --primary 10.0.1.10 \
  --listen-ip 0.0.0.0
```

AXFR carries DNSKEY, RRSIG, NSEC, and DS as standard resource records. The
secondary validates the transfer framing, compiles the received snapshot, and
serves the same finished signatures. A transfer failure retains the last good
zone. This is a useful security boundary: compromise of a serving secondary
does not disclose the signing key.

Before touching the parent, query every delegated authority over UDP and TCP:

```sh
for server in a.ns.example.com b.ns.example.com; do
  dig +dnssec +norecurse @"$server" example.com DNSKEY
  dig +dnssec +norecurse @"$server" example.com A
  dig +dnssec +norecurse @"$server" \
    "MiXeD-$(date +%s).example.com" A
  dig +tcp +dnssec +norecurse @"$server" \
    "tcp-$(date +%s).example.com" A
done
```

Require matching SOA serials and DNSKEYs. Positive answers need the data RRset
and its RRSIG. A nonexistent mixed-case name must return NXDOMAIN with NSEC and
RRSIG. The mixed case is deliberate: validating resolvers use DNS 0x20 case
randomization, and authoritative suffix matching must remain ASCII
case-insensitive. A query for an absent type at an existing name tests signed
NODATA separately from NXDOMAIN.

````
