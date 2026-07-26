---
type: "code-fragment"
fragment_id: "rgbdns-frag-6271b6cb7110"
source_path: "docs/book/rgbdns.md"
code_note: "DNS from First Principles/Code/docs/book/rgbdns.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "Benchmarks and evidence-driven optimization"
kind: "heading"
start_line: 1354
end_line: 1409
---

# Benchmarks and evidence-driven optimization

- Fragment ID: `rgbdns-frag-6271b6cb7110`
- Source file: [[DNS from First Principles/Code/docs/book/rgbdns.md.source|docs/book/rgbdns.md]]
- Lines: 1354-1409
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-6271b6cb7110", "codeNote": "DNS from First Principles/Code/docs/book/rgbdns.md.source", "heading": "rgbdns-frag-6271b6cb7110: heading Benchmarks and evidence-driven optimization", "sourcePath": "docs/book/rgbdns.md", "startLine": 1354, "endLine": 1409}
```

## Excerpt

<span id="rgbdns-frag-6271b6cb7110" class="rgbdns-fragment-target"></span>
### rgbdns-frag-6271b6cb7110: heading Benchmarks and evidence-driven optimization

```markdown
## Benchmarks and evidence-driven optimization

Correctness gates run before performance conclusions. The benchmark is a
dependency-free stable-Rust harness in `benches/dns_core.rs`; the same harness
is available as `examples/dns_core_bench.rs` for quick release-mode runs:

```sh
cargo bench --bench dns_core
RGBDNS_BENCH_ITERATIONS=10000 \
  cargo run --release --example dns_core_bench
```

It warms every operation, passes values through `std::hint::black_box`, and
reports nanoseconds per operation. Measurements are comparable only on the same
host, toolchain, power state, and iteration count. Wire size is reported beside
CPU time because DNS compression exchanges encoder work for fewer network
bytes.

The July 2026 checkpoint used release mode on one aarch64 Android host:

| Operation | Baseline | Optimized | Result |
|---|---:|---:|---:|
| Encoded 64-record response | 2,147 bytes | 1,059 bytes | 50.7% smaller |
| Decode small query | 542 ns | 458 ns | 15.5% faster |
| Decode 64-record response | 52,661 ns | 29,540 ns | 43.9% faster |
| Encode 64-record response | 2,318 ns | 5,309 ns | 2.3 times slower |
| Exact lookup, 1,000 names | 1,262 ns | 1,244 ns | 1.4% faster |
| NXDOMAIN, 1,000 names | 29,889 ns | 2,726 ns | 11.0 times faster |
| Small authoritative response | 17,007 ns | 7,714 ns | 54.6% faster |
| Truncate 200-record response | 3,098,232 ns | 2,570,077 ns | 17.0% faster |

Three structural changes explain most of the gains.

First, `Zone` maintains an index of every node, including empty non-terminals.
A clearly absent name can return NXDOMAIN without scanning the records of a
thousand-name zone. Conditional records still take the visibility path, so the
index does not erase time or location semantics.

Second, response truncation searches the number of tail records to remove
instead of encoding once for every removed record. It preserves the question
and OPT record as long as possible and validates the final candidate against
the transport limit.

Third, the packet writer records complete names and suffixes for RFC 1035
compression. RRsets tend to repeat the immediately preceding owner, so a
last-owner cache avoids rebuilding and hashing suffix keys on the dominant
path. The first compression design encoded the 64-record case in 34,075 ns;
the cache reduced that to 5,309 ns.

The remaining encoder regression is intentional and visible. Compression makes
the example packet roughly half as large while taking more local CPU than the
old uncompressed writer. That is a defensible trade for an authoritative
server because it reduces datagram pressure, TCP bytes, and downstream decode
work. Recording the regression matters: optimization should reveal tradeoffs,
not hide them behind one favorable number.

```
