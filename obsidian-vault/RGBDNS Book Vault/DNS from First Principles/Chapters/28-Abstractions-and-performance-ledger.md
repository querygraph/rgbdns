---
type: "chapter"
source_file: "docs/book/rgbdns.md"
---

# Abstractions and performance ledger

The rewrite’s gains are not a single “Rust is faster” claim. Some changes buy
safety, some buy clarity, and some measurably improve a hot path.

| Design move | Rust expression | Operational effect |
|---|---|---|
| Valid names at construction | private fields, `FromStr`, `Result` | invalid labels cannot circulate |
| Complete DNS states | `RecordType`, `RData`, `Lookup` enums | unknown types survive; negative answers stay distinct |
| Bounded packet access | borrowed slices and checked indexing | malformed packets fail without memory corruption |
| Shared immutable service state | borrowing and `Arc` | explicit thread-safe ownership |
| Compatibility quarantine | checked CDB decoder into owned values | old files do not become trusted memory |
| Independent resource limits | typed bounded configuration | one limit cannot silently stand in for another |
| All-node zone index | `BTreeSet<Name>` | NXDOMAIN lookup improved about 11× |
| Compressed writer | bounded suffix reuse | repeated-owner answer became 50.7% smaller |
| Binary-search truncation | monotone fit search | 200-record truncation improved 17% |
| Thin binaries | library functions plus small adapters | easier tests and independent supervision |

The encoder example is especially important. Rust did not automatically make
it faster: compression made encoding 2.3 times slower than the uncompressed
baseline. But it halved the measured wire size, reduced fragmentation risk,
and sharply improved the complete authoritative response path. Good systems
engineering reports the trade rather than reducing it to a language slogan.

The deeper improvement over C is control. The data model says what is valid,
the compiler checks ownership, the boundaries return typed failure, the tests
state protocol properties, and benchmarks expose the remaining costs. That
combination makes rgbdns easier to change without making DNS easier to fool.

## Generated Code Fragment Index

These generated links open the collocated rgbdns codebase notes.

```rgbdns-fragment
{"id": "rgbdns-frag-b143870e07fb", "codeNote": "DNS from First Principles/Code/benches/dns_core.rs.source", "heading": "rgbdns-frag-b143870e07fb: const DEFAULT_ITERATIONS", "sourcePath": "benches/dns_core.rs", "startLine": 8, "endLine": 13}
```

```rgbdns-fragment
{"id": "rgbdns-frag-113e32eecd5c", "codeNote": "DNS from First Principles/Code/benches/dns_core.rs.source", "heading": "rgbdns-frag-113e32eecd5c: fn measure", "sourcePath": "benches/dns_core.rs", "startLine": 14, "endLine": 28}
```

```rgbdns-fragment
{"id": "rgbdns-frag-0591f6d10783", "codeNote": "DNS from First Principles/Code/benches/dns_core.rs.source", "heading": "rgbdns-frag-0591f6d10783: fn report", "sourcePath": "benches/dns_core.rs", "startLine": 29, "endLine": 32}
```

```rgbdns-fragment
{"id": "rgbdns-frag-b5f853019841", "codeNote": "DNS from First Principles/Code/benches/dns_core.rs.source", "heading": "rgbdns-frag-b5f853019841: fn main", "sourcePath": "benches/dns_core.rs", "startLine": 33, "endLine": 113}
```

```rgbdns-fragment
{"id": "rgbdns-frag-9d0d58ec4f3e", "codeNote": "DNS from First Principles/Code/docs/performance.md.source", "heading": "rgbdns-frag-9d0d58ec4f3e: heading Performance methodology", "sourcePath": "docs/performance.md", "startLine": 1, "endLine": 33}
```

```rgbdns-fragment
{"id": "rgbdns-frag-6c356aa252b7", "codeNote": "DNS from First Principles/Code/docs/performance.md.source", "heading": "rgbdns-frag-6c356aa252b7: heading Optimization checkpoint", "sourcePath": "docs/performance.md", "startLine": 34, "endLine": 55}
```
