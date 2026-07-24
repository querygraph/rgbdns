---
type: "chapter"
source_file: "docs/book/rgbdns.md"
---

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

## Generated Code Fragment Index

These generated links open the collocated rgbdns codebase notes.

```rgbdns-fragment
{"id": "rgbdns-frag-0c2a1a2a636a", "codeNote": "DNS from First Principles/Code/src/bin/dnscache.rs.source", "heading": "rgbdns-frag-0c2a1a2a636a: fn main", "sourcePath": "src/bin/dnscache.rs", "startLine": 21, "endLine": 35}
```

```rgbdns-fragment
{"id": "rgbdns-frag-f9c2e38b98db", "codeNote": "DNS from First Principles/Code/src/bin/dnscache.rs.source", "heading": "rgbdns-frag-f9c2e38b98db: fn run", "sourcePath": "src/bin/dnscache.rs", "startLine": 36, "endLine": 116}
```

```rgbdns-fragment
{"id": "rgbdns-frag-c4c75c363342", "codeNote": "DNS from First Principles/Code/src/bin/dnscache.rs.source", "heading": "rgbdns-frag-c4c75c363342: fn bounded_env_usize", "sourcePath": "src/bin/dnscache.rs", "startLine": 143, "endLine": 151}
```

```rgbdns-fragment
{"id": "rgbdns-frag-93419b6c9023", "codeNote": "DNS from First Principles/Code/src/bin/dnscache.rs.source", "heading": "rgbdns-frag-93419b6c9023: fn bounded_env_u8", "sourcePath": "src/bin/dnscache.rs", "startLine": 152, "endLine": 160}
```

```rgbdns-fragment
{"id": "rgbdns-frag-985f21d17a56", "codeNote": "DNS from First Principles/Code/src/bin/dnscache.rs.source", "heading": "rgbdns-frag-985f21d17a56: fn bounded_env", "sourcePath": "src/bin/dnscache.rs", "startLine": 161, "endLine": 180}
```

```rgbdns-fragment
{"id": "rgbdns-frag-3790fed38817", "codeNote": "DNS from First Principles/Code/src/dnscache_config.rs.source", "heading": "rgbdns-frag-3790fed38817: const MAX_ROOTS_FILE", "sourcePath": "src/dnscache_config.rs", "startLine": 12, "endLine": 12}
```

```rgbdns-fragment
{"id": "rgbdns-frag-f59cd49b7a3e", "codeNote": "DNS from First Principles/Code/src/dnscache_config.rs.source", "heading": "rgbdns-frag-f59cd49b7a3e: const MAX_ROOT_ADDRESSES", "sourcePath": "src/dnscache_config.rs", "startLine": 13, "endLine": 13}
```

```rgbdns-fragment
{"id": "rgbdns-frag-e2e70010249a", "codeNote": "DNS from First Principles/Code/src/dnscache_config.rs.source", "heading": "rgbdns-frag-e2e70010249a: const MAX_FORWARD_ZONES", "sourcePath": "src/dnscache_config.rs", "startLine": 14, "endLine": 16}
```

```rgbdns-fragment
{"id": "rgbdns-frag-128d48d8a73e", "codeNote": "DNS from First Principles/Code/src/dnscache_config.rs.source", "heading": "rgbdns-frag-128d48d8a73e: struct ForwardZone", "sourcePath": "src/dnscache_config.rs", "startLine": 17, "endLine": 26}
```

```rgbdns-fragment
{"id": "rgbdns-frag-c527bbc0d8ea", "codeNote": "DNS from First Principles/Code/src/dnscache_config.rs.source", "heading": "rgbdns-frag-c527bbc0d8ea: struct PreparedRoots", "sourcePath": "src/dnscache_config.rs", "startLine": 27, "endLine": 31}
```

```rgbdns-fragment
{"id": "rgbdns-frag-2ed2d81c2acf", "codeNote": "DNS from First Principles/Code/src/dnscache_config.rs.source", "heading": "rgbdns-frag-2ed2d81c2acf: impl PreparedRoots", "sourcePath": "src/dnscache_config.rs", "startLine": 32, "endLine": 32}
```

```rgbdns-fragment
{"id": "rgbdns-frag-1c885999a2f2", "codeNote": "DNS from First Principles/Code/src/dnscache_config.rs.source", "heading": "rgbdns-frag-1c885999a2f2: fn from_environment", "sourcePath": "src/dnscache_config.rs", "startLine": 33, "endLine": 45}
```
