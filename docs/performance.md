# Performance methodology

`benches/dns_core.rs` is a dependency-free, stable-Rust microbenchmark for the
hot paths owned by rgbdns:

- decoding a small query and a 64-record response;
- encoding that response;
- exact lookup in a 1,000-name zone;
- producing a small authoritative response;
- producing and truncating a 200-record response.

Run it with:

```sh
cargo bench --bench dns_core
```

For a faster local build that reuses the normal release profile artifacts, the
same harness is also exposed as an example:

```sh
cargo run --release --example dns_core_bench
```

Set `RGBDNS_BENCH_ITERATIONS` to shorten an exploratory run or lengthen a
release measurement. Compare results only on the same host, toolchain, power
state, and iteration count. The harness warms every operation, uses
`std::hint::black_box`, and reports integer nanoseconds per operation.

Optimization changes must preserve the RFC, property, interoperability, and
security suites. Wire size is reported beside timings because DNS compression
can improve network efficiency even when it adds local CPU work.

## Optimization checkpoint

The July 2026 checkpoint below used release mode on the same aarch64 Android
host. The baseline predates the all-node zone index, binary-search truncation,
strict pointer-boundary decoder, and compressed writer. Values are
microbenchmark results rather than cross-machine promises.

| Operation | Baseline | Optimized | Change |
|---|---:|---:|---:|
| Encoded 64-record response | 2,147 bytes | 1,059 bytes | 50.7% smaller |
| Decode small query | 542 ns | 458 ns | 15.5% faster |
| Decode 64-record response | 52,661 ns | 29,540 ns | 43.9% faster |
| Encode 64-record response | 2,318 ns | 5,309 ns | 2.3× slower |
| Exact lookup, 1,000 names | 1,262 ns | 1,244 ns | 1.4% faster |
| NXDOMAIN, 1,000 names | 29,889 ns | 2,726 ns | 11.0× faster |
| Small authoritative response | 17,007 ns | 7,714 ns | 54.6% faster |
| Truncate 200-record response | 3,098,232 ns | 2,570,077 ns | 17.0% faster |

Compression deliberately trades additional encoder CPU for roughly half the
wire bytes in the repeated-owner workload. A last-owner cache keeps that cost
well below the initial suffix-map implementation (34,075 ns), while preserving
compression across different names that share suffixes.
