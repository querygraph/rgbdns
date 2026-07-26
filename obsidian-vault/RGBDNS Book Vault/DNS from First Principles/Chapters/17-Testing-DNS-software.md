---
type: "chapter"
source_file: "docs/book/rgbdns.md"
---

# Testing DNS software

## Layers of evidence

Unit tests establish local invariants: name limits, record parsing, lookup
outcomes, leap conversion. Property tests explore parser state spaces that
examples miss. Golden fixtures establish compatibility with an external file
format. Integration tests cross process and socket boundaries. Live
interoperability tests compare behavior with independent clients and servers.

rgbdns uses all of these. A useful local sequence is:

```sh
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
```

Network tests bind unprivileged loopback ports. CDB tests compile canonical
fixtures and compare entries. Daemontools tests exercise process replacement,
rotation, and TAI64 filter behavior. Packet properties assert that arbitrary
input does not panic and that supported structured messages survive
encode/decode round trips.

## Adversarial cases worth keeping

Every DNS implementation should retain regression cases for:

- a compression pointer to itself or a pointer cycle;
- a pointer or RDATA length just beyond the packet;
- maximum-length labels and names;
- counts that cannot be satisfied by the remaining bytes;
- duplicate or malformed OPT records;
- tiny advertised transport limits;
- CNAME loops and excessive chains;
- ANAME self-reference, upstream CNAME loops, excessive address results, and
  resolver failure;
- wildcard names blocked by existing nodes;
- delegation cuts beneath an authoritative apex;
- NODATA versus NXDOMAIN;
- AXFR without a closing SOA;
- an enormous log line;
- configuration counts at and beyond each bound.

Tests should assert protocol meaning, not only that the process remains alive.
A safe FORMERR is better than a crash, but a silent NOERROR can still be a
serious bug.

## Conformance as an executable specification

The conformance suite turns protocol prose into named, reviewable cases. Its
scope is the DNS surface rgbdns implements; it does not imply support for every
extension ever assigned by IANA. The principal coverage is:

| Standard | Behavior exercised |
|---|---|
| RFC 1035 | header identity, flags, names, compression, typed RDATA, UDP and TCP results |
| RFC 2181 | in-bailiwick glue, coherent RRset TTLs, duplicate suppression, CNAME exclusivity |
| RFC 2308 | NXDOMAIN versus NODATA, authoritative SOA, negative-cache TTL |
| RFC 3597 | unknown QTYPE behavior and lossless opaque RDATA |
| RFC 4343 | case-insensitive name identity with query-case preservation |
| RFC 4592 | closest-encloser wildcard synthesis and empty non-terminals |
| RFC 5936 | AXFR framing, identity, flags, SOA bookends, and zone boundaries |
| RFC 6891 | one root-owned OPT, payload negotiation, DO, BADVERS, and unknown options |
| RFC 7766 | TCP framing, connection reuse, pipelining, and full-size responses |
| RFC 8906 | the authoritative-server matrix for unknown types, opcodes, flags, and EDNS fields |
| RFC 9619 | exactly one question in a standard query |

This is more useful than a single “RFC compliant” label. A test name identifies
the rule, a packet fixture demonstrates it, and a failure points to a specific
semantic regression.

RFC 8906 is especially valuable because it tests how a server behaves at the
edges of what it understands. An unknown ordinary type is not a protocol
error: the answer depends on whether the owner name exists. An unknown opcode
is different. Because that opcode may define a body layout unlike QUERY, the
server must produce NOTIMP from the header without first interpreting the body
as an ordinary question. Unknown EDNS options are structurally validated and
then ignored. An unsupported EDNS version produces BADVERS while retaining an
OPT response.

The independent `drill` integration test supplies another boundary. It launches
the real `tinydns` binary and asks the ldns client to make UDP, TCP, EDNS,
mixed-case, and unknown-type queries. This catches accidental agreement
between rgbdns's own encoder and decoder: the request and response cross an
implementation boundary.

The complete focused matrix is:

```sh
cargo test --test rfc_conformance
cargo test --test wire_security
cargo test --test packet_properties
cargo test --test drill_interop
```

The generated suite exercises forty thousand cases per complete run. It feeds
arbitrary bytes to the decoder, reparses every accepted packet, generates
structured messages for semantic round trips, and changes ASCII letter case
without changing DNS name identity. A separate truncation corpus tries every
prefix of a valid structured packet. These properties do not prove the absence
of all parser defects, but they explore combinations that hand-written examples
rarely anticipate.

## Hardening found by conformance work

Conformance testing improved the implementation rather than merely describing
it.

The name decoder now records valid prior name boundaries. A compression
pointer must be backward *and* must target one of those boundaries. Merely
pointing at earlier bytes that happen to resemble a label sequence is rejected.
This closes a class of ambiguous parses without forbidding legal compression.

Stub responses are bound to the request ID, QR bit, opcode, and exact question.
TCP responses carrying TC are rejected. AXFR applies the same identity checks
and additionally requires authoritative, non-truncated messages, controlled
question repetition, an empty authority section, matching opening and closing
SOAs, and records confined to the requested zone. These rules prevent a
plausible-looking but unrelated response from being accepted as the answer to
the outstanding operation.

Zone loading rejects a CNAME owner that also has other data and rejects
multiple different CNAME targets. Before transmission, RRsets are normalized
to their minimum TTL and duplicate records are removed. Negative answers cap
the SOA TTL at the SOA MINIMUM field as RFC 2308 requires. EDNS OPT records in
the wrong section and duplicate OPT records produce FORMERR.

The UDP and TCP daemons now share one bounded transport module. TCP connections
carry deadlines, use a fixed worker pool, accept multiple framed queries, and
support pipelined requests. This removes duplicated socket code while making
RFC 7766 behavior an invariant shared by the authoritative and specialized
servers.

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

## Generated Code Fragment Index

These generated links open the collocated rgbdns codebase notes.

```rgbdns-fragment
{"id": "rgbdns-frag-d523e32e6d92", "codeNote": "DNS from First Principles/Code/tests/packet_properties.rs.source", "heading": "rgbdns-frag-d523e32e6d92: fn dns_name", "sourcePath": "tests/packet_properties.rs", "startLine": 5, "endLine": 9}
```

```rgbdns-fragment
{"id": "rgbdns-frag-ebda46b4d11b", "codeNote": "DNS from First Principles/Code/tests/packet_properties.rs.source", "heading": "rgbdns-frag-ebda46b4d11b: fn record", "sourcePath": "tests/packet_properties.rs", "startLine": 10, "endLine": 25}
```

```rgbdns-fragment
{"id": "rgbdns-frag-d74635b61765", "codeNote": "DNS from First Principles/Code/tests/packet_properties.rs.source", "heading": "rgbdns-frag-d74635b61765: fn structured_message", "sourcePath": "tests/packet_properties.rs", "startLine": 26, "endLine": 57}
```

```rgbdns-fragment
{"id": "rgbdns-frag-3533f4b0fcb7", "codeNote": "DNS from First Principles/Code/tests/packet_properties.rs.source", "heading": "rgbdns-frag-3533f4b0fcb7: fn arbitrary_packets_never_panic", "sourcePath": "tests/packet_properties.rs", "startLine": 58, "endLine": 62}
```

```rgbdns-fragment
{"id": "rgbdns-frag-943ca8547879", "codeNote": "DNS from First Principles/Code/tests/packet_properties.rs.source", "heading": "rgbdns-frag-943ca8547879: fn accepted_packets_are_stably_reparseable", "sourcePath": "tests/packet_properties.rs", "startLine": 63, "endLine": 71}
```

```rgbdns-fragment
{"id": "rgbdns-frag-50dd1f4e2ab3", "codeNote": "DNS from First Principles/Code/tests/packet_properties.rs.source", "heading": "rgbdns-frag-50dd1f4e2ab3: fn structured_messages_roundtrip_without_semantic_loss", "sourcePath": "tests/packet_properties.rs", "startLine": 72, "endLine": 78}
```

```rgbdns-fragment
{"id": "rgbdns-frag-d899ad878877", "codeNote": "DNS from First Principles/Code/tests/packet_properties.rs.source", "heading": "rgbdns-frag-d899ad878877: fn ascii_case_changes_do_not_change_name_identity", "sourcePath": "tests/packet_properties.rs", "startLine": 79, "endLine": 103}
```

```rgbdns-fragment
{"id": "rgbdns-frag-69e831b1a28c", "codeNote": "DNS from First Principles/Code/tests/rfc_conformance.rs.source", "heading": "rgbdns-frag-69e831b1a28c: mod support", "sourcePath": "tests/rfc_conformance.rs", "startLine": 1, "endLine": 6}
```

```rgbdns-fragment
{"id": "rgbdns-frag-5292cc2a0403", "codeNote": "DNS from First Principles/Code/tests/rfc_conformance.rs.source", "heading": "rgbdns-frag-5292cc2a0403: fn rfc1035_response_identity_flags_and_question_are_coherent", "sourcePath": "tests/rfc_conformance.rs", "startLine": 7, "endLine": 22}
```

```rgbdns-fragment
{"id": "rgbdns-frag-114e43800a7f", "codeNote": "DNS from First Principles/Code/tests/rfc_conformance.rs.source", "heading": "rgbdns-frag-114e43800a7f: fn rfc9619_standard_queries_require_exactly_one_question", "sourcePath": "tests/rfc_conformance.rs", "startLine": 23, "endLine": 37}
```

```rgbdns-fragment
{"id": "rgbdns-frag-e4b1690ff7a3", "codeNote": "DNS from First Principles/Code/tests/rfc_conformance.rs.source", "heading": "rgbdns-frag-e4b1690ff7a3: fn rfc8906_unknown_opcode_is_notimp_even_when_its_body_is_unknown", "sourcePath": "tests/rfc_conformance.rs", "startLine": 38, "endLine": 51}
```

```rgbdns-fragment
{"id": "rgbdns-frag-6a8de18b800c", "codeNote": "DNS from First Principles/Code/tests/rfc_conformance.rs.source", "heading": "rgbdns-frag-6a8de18b800c: fn rfc8906_all_unimplemented_opcodes_get_notimp_without_flag_reflection", "sourcePath": "tests/rfc_conformance.rs", "startLine": 52, "endLine": 65}
```
