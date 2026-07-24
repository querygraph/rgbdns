---
type: "code-fragment"
fragment_id: "rgbdns-frag-6c356aa252b7"
source_path: "docs/performance.md"
code_note: "DNS from First Principles/Code/docs/performance.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "Optimization checkpoint"
kind: "heading"
start_line: 34
end_line: 55
---

# Optimization checkpoint

- Fragment ID: `rgbdns-frag-6c356aa252b7`
- Source file: [[DNS from First Principles/Code/docs/performance.md.source|docs/performance.md]]
- Lines: 34-55
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-6c356aa252b7", "codeNote": "DNS from First Principles/Code/docs/performance.md.source", "heading": "rgbdns-frag-6c356aa252b7: heading Optimization checkpoint", "sourcePath": "docs/performance.md", "startLine": 34, "endLine": 55}
```

## Excerpt

<span id="rgbdns-frag-6c356aa252b7" class="rgbdns-fragment-target"></span>
### rgbdns-frag-6c356aa252b7: heading Optimization checkpoint

```markdown
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
```
