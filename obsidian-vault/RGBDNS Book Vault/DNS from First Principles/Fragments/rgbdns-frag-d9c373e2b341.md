---
type: "code-fragment"
fragment_id: "rgbdns-frag-d9c373e2b341"
source_path: "docs/book/rgbdns.md"
code_note: "DNS from First Principles/Code/docs/book/rgbdns.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "The cache is part of correctness"
kind: "heading"
start_line: 468
end_line: 494
---

# The cache is part of correctness

- Fragment ID: `rgbdns-frag-d9c373e2b341`
- Source file: [[DNS from First Principles/Code/docs/book/rgbdns.md.source|docs/book/rgbdns.md]]
- Lines: 468-494
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-d9c373e2b341", "codeNote": "DNS from First Principles/Code/docs/book/rgbdns.md.source", "heading": "rgbdns-frag-d9c373e2b341: heading The cache is part of correctness", "sourcePath": "docs/book/rgbdns.md", "startLine": 468, "endLine": 494}
```

## Excerpt

<span id="rgbdns-frag-d9c373e2b341" class="rgbdns-fragment-target"></span>
### rgbdns-frag-d9c373e2b341: heading The cache is part of correctness

```markdown
## The cache is part of correctness

A cache key includes at least name, type, and class. A cached positive RRset
expires according to TTL. Negative results also have bounded lifetimes derived
from SOA data. Delegation and nameserver-address caches help the iterative
algorithm navigate efficiently.

Capacity is as important as time. An attacker can generate endless distinct
names. An unbounded cache converts traffic into memory exhaustion. A practical
resolver bounds response cache bytes, nameserver cache entries, recursion
depth, referral work, packet sizes, concurrent operations, and timeouts.

`src/bin/dnscache.rs` uses Hickory’s recursive zone handler inside rgbdns’s
process and policy shell. It configures:

- randomized query-name letter case;
- a bounded response cache, defaulting to 16 MiB;
- a bounded nameserver cache;
- bounded ordinary and nameserver recursion depth;
- a 1232-byte EDNS payload;
- UDP and TCP listeners;
- loopback-only clients by default, expanded through `ALLOW_NETS`.

Configuration values are parsed with explicit minimums and maximums. A typo
such as an enormous cache size fails startup rather than silently allocating an
operator’s mistake.

```
