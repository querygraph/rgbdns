---
type: "code-fragment"
fragment_id: "rgbdns-frag-e1b65c8ad257"
source_path: "docs/DNSSEC-DESIGN.md"
code_note: "DNS from First Principles/Code/docs/DNSSEC-DESIGN.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "Snapshot contract"
kind: "heading"
start_line: 80
end_line: 105
---

# Snapshot contract

- Fragment ID: `rgbdns-frag-e1b65c8ad257`
- Source file: [[DNS from First Principles/Code/docs/DNSSEC-DESIGN.md.source|docs/DNSSEC-DESIGN.md]]
- Lines: 80-105
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-e1b65c8ad257", "codeNote": "DNS from First Principles/Code/docs/DNSSEC-DESIGN.md.source", "sourcePath": "docs/DNSSEC-DESIGN.md", "startLine": 80, "endLine": 105}
```

## Excerpt

<span id="rgbdns-frag-e1b65c8ad257" class="rgbdns-fragment-target"></span>
### rgbdns-frag-e1b65c8ad257: heading Snapshot contract

```markdown
## Snapshot contract

The unsigned `data` file is source. The signed text and CDB are derived,
replaceable artifacts. For each configured zone, the signer:

1. requires exactly one SOA and a conventional authoritative zone;
2. rejects location-dependent records, time cutoffs, and pre-existing DNSSEC
   records in that zone;
3. requires every ANAME to have been materialized to ordinary A and AAAA data;
4. adds the apex DNSKEY;
5. builds an NSEC chain over authoritative names, excluding data beneath zone
   cuts;
6. signs every authoritative RRset, including NSEC and DNSKEY; and
7. emits canonical generic tinydns records for DNSKEY, NSEC, and RRSIG.

The SOA serial belongs to the input producer. Signing does not invent a second
serial sequence. A byte-identical input, policy, key, and signing instant
produce the same RRset content (ECDSA signature bytes need not be reproducible).

Every authoritative zone in one source snapshot must have exactly one `K` or
`U` line. This fail-closed rule prevents an accidentally omitted policy line
from silently changing security state. Unsigned zones, their ANAME directives,
qualified records, and unscoped glue pass through the CDB compiler unchanged.
DNSSEC must not be enabled for a zone whose answer varies by client location or
time: one signed owner/type must always identify one RRset.

```
