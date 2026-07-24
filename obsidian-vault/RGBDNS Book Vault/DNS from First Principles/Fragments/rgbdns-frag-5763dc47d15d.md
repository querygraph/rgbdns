---
type: "code-fragment"
fragment_id: "rgbdns-frag-5763dc47d15d"
source_path: "docs/book/rgbdns.md"
code_note: "DNS from First Principles/Code/docs/book/rgbdns.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "Recursion by composition"
kind: "heading"
start_line: 1418
end_line: 1443
---

# Recursion by composition

- Fragment ID: `rgbdns-frag-5763dc47d15d`
- Source file: [[DNS from First Principles/Code/docs/book/rgbdns.md.source|docs/book/rgbdns.md]]
- Lines: 1418-1443
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-5763dc47d15d", "codeNote": "DNS from First Principles/Code/docs/book/rgbdns.md.source", "heading": "rgbdns-frag-5763dc47d15d: heading Recursion by composition", "sourcePath": "docs/book/rgbdns.md", "startLine": 1418, "endLine": 1443}
```

## Excerpt

<span id="rgbdns-frag-5763dc47d15d" class="rgbdns-fragment-target"></span>
### rgbdns-frag-5763dc47d15d: heading Recursion by composition

```markdown
# Recursion by composition

Authoritative DNS is implemented in rgbdns’s own small model. Recursive DNS,
DNSSEC validation, caching, and upstream transport are composed from Hickory
in [`src/bin/dnscache.rs`](../../src/bin/dnscache.rs). This is not a retreat
from the rewrite; it is a deliberate abstraction boundary.

rgbdns owns policy that must remain djbdns-compatible or operator-visible:
root hints, forwarding zones, allowed networks, cache budgets, recursion
limits, EDNS payload, DNSSEC policy, listener addresses, and shutdown.
Hickory supplies the complex iterative resolver machinery behind typed
configuration and handler interfaces.

Every operator-controlled dimension is bounded. Cache sizes, recursion depth,
name-server recursion depth, network lists, timeouts, and TCP message sizes
have explicit limits. The `bounded_env` generic converts an environment value
and verifies its range before server construction. A C implementation can do
the same checks, but Rust makes the parsed type and the allowed range part of
one reusable function.

Composition also improves performance engineering. The custom authoritative
path stays small and directly benchmarkable. The resolver can use Tokio and a
mature async DNS implementation without imposing that runtime on tinydns,
rbldns, or walldns. Different concurrency models remain behind process and
library boundaries rather than forcing one architecture across the suite.

```
