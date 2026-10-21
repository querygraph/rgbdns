---
type: "code-fragment"
fragment_id: "rgbdns-frag-3430245aee37"
source_path: "docs/book/rgbdns.md"
code_note: "DNS from First Principles/Code/docs/book/rgbdns.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "Two distinct DNSSEC roles"
kind: "heading"
start_line: 693
end_line: 711
---

# Two distinct DNSSEC roles

- Fragment ID: `rgbdns-frag-3430245aee37`
- Source file: [[DNS from First Principles/Code/docs/book/rgbdns.md.source|docs/book/rgbdns.md]]
- Lines: 693-711
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-3430245aee37", "codeNote": "DNS from First Principles/Code/docs/book/rgbdns.md.source", "sourcePath": "docs/book/rgbdns.md", "startLine": 693, "endLine": 711}
```

## Excerpt

<span id="rgbdns-frag-3430245aee37" class="rgbdns-fragment-target"></span>
### rgbdns-frag-3430245aee37: heading Two distinct DNSSEC roles

```markdown
## Two distinct DNSSEC roles

DNSSEC has a consumer side and a producer side. A validating recursive
resolver follows DS and DNSKEY records from a trust anchor and rejects bogus
answers. An authoritative operator creates keys, signs each stable RRset,
publishes proofs of nonexistence, and arranges for the parent to publish the
matching DS.

rgbdns implements both roles, but deliberately keeps them separate. `dnscache`
uses a static root trust anchor and DNSSEC validation. Validation and NSEC3 work
receive bounded caches and iteration policies. A failed validation becomes a
resolution failure rather than an unchecked answer.

Authoritative signing is an offline publication pipeline. `tinydns` never
opens a private key and never signs in response to a packet. It serves DNSKEY,
RRSIG, NSEC, and DS records already present in its CDB. This is the same
compile-then-serve division used by ordinary tinydns data, extended to signed
snapshots.

```
