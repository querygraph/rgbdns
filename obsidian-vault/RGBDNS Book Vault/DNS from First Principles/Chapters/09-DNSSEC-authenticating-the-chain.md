---
type: "chapter"
source_file: "docs/book/rgbdns.md"
---

# DNSSEC: authenticating the chain

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

## rgbdns validation policy

rgbdns configures the recursive handler with a static root trust anchor and
DNSSEC validation enabled. Validation and NSEC3 work receive bounded caches and
iteration policies. A failed validation surfaces as resolution failure rather
than an unchecked answer.

The authoritative rgbdns data path focuses on the djbdns record surface; the
recursive path is where DNSSEC validation is currently integrated. This is an
example of honest component boundaries: “the suite supports validating
recursion” does not imply that every authoritative signing workflow has been
recreated.

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
