---
type: "code-fragment"
fragment_id: "rgbdns-frag-9b2e36ed71f1"
source_path: "docs/book/rgbdns.md"
code_note: "DNS from First Principles/Code/docs/book/rgbdns.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "What ordinary DNS cannot prove"
kind: "heading"
start_line: 516
end_line: 542
---

# What ordinary DNS cannot prove

- Fragment ID: `rgbdns-frag-9b2e36ed71f1`
- Source file: [[DNS from First Principles/Code/docs/book/rgbdns.md.source|docs/book/rgbdns.md]]
- Lines: 516-542
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-9b2e36ed71f1", "codeNote": "DNS from First Principles/Code/docs/book/rgbdns.md.source", "heading": "rgbdns-frag-9b2e36ed71f1: heading What ordinary DNS cannot prove", "sourcePath": "docs/book/rgbdns.md", "startLine": 516, "endLine": 542}
```

## Excerpt

<span id="rgbdns-frag-9b2e36ed71f1" class="rgbdns-fragment-target"></span>
### rgbdns-frag-9b2e36ed71f1: heading What ordinary DNS cannot prove

```markdown
## What ordinary DNS cannot prove

Transaction IDs, source ports, and query-case randomization make blind
spoofing harder, but they do not cryptographically establish who published an
RRset. DNSSEC adds signatures and a chain of trust.

A zone signs RRsets with private keys and publishes DNSKEY records. A parent
publishes a DS digest that identifies a child key. Starting from a configured
root trust anchor, a validating resolver can authenticate the root DNSKEY,
then a top-level domain’s DS and DNSKEY, and so on to the answer.

RRSIG authenticates an RRset over a validity interval. DS links parent to
child. NSEC or NSEC3 authenticates nonexistence by proving gaps in the ordered
namespace. DNSSEC provides origin authentication and integrity; it does not
encrypt queries or hide names.

Validation outcomes matter:

- **secure**: a valid chain reaches the answer;
- **insecure**: the chain proves that the child is unsigned;
- **bogus**: signatures or proofs fail;
- **indeterminate**: validation cannot be completed safely.

A resolver must not turn bogus data into a normal answer merely to improve
availability. Clock correctness also becomes a dependency because signatures
have inception and expiration times.

```
