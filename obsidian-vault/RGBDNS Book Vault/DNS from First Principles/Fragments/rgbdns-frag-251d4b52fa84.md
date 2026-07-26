---
type: "code-fragment"
fragment_id: "rgbdns-frag-251d4b52fa84"
source_path: "docs/book/rgbdns.md"
code_note: "DNS from First Principles/Code/docs/book/rgbdns.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "Tests as executable protocol commentary"
kind: "heading"
start_line: 1714
end_line: 1749
---

# Tests as executable protocol commentary

- Fragment ID: `rgbdns-frag-251d4b52fa84`
- Source file: [[DNS from First Principles/Code/docs/book/rgbdns.md.source|docs/book/rgbdns.md]]
- Lines: 1714-1749
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-251d4b52fa84", "codeNote": "DNS from First Principles/Code/docs/book/rgbdns.md.source", "heading": "rgbdns-frag-251d4b52fa84: heading Tests as executable protocol commentary", "sourcePath": "docs/book/rgbdns.md", "startLine": 1714, "endLine": 1749}
```

## Excerpt

<span id="rgbdns-frag-251d4b52fa84" class="rgbdns-fragment-target"></span>
### rgbdns-frag-251d4b52fa84: heading Tests as executable protocol commentary

```markdown
# Tests as executable protocol commentary

The strongest claims in this book have executable counterparts.
[`tests/rfc_conformance.rs`](https://github.com/querygraph/rgbdns/blob/master/tests/rfc_conformance.rs) names normative
requirements and constructs exact packets. [`tests/wire_security.rs`](https://github.com/querygraph/rgbdns/blob/master/tests/wire_security.rs)
contains a hostile corpus and checks every truncation of a structured packet.
[`tests/packet_properties.rs`](https://github.com/querygraph/rgbdns/blob/master/tests/packet_properties.rs) generates
arbitrary bytes and structured messages:

```rust
#[test]
fn arbitrary_packets_never_panic(
    bytes in prop::collection::vec(any::<u8>(), 0..4096)
) {
    let _ = Message::decode(&bytes);
}
```

The property suite does not prove that every accepted DNS packet has the
desired meaning. It establishes three valuable invariants over a large input
space: decoding never panics, accepted packets can be re-encoded and reparsed,
and generated structured messages round-trip without semantic loss.

Golden CDB fixtures protect historical compatibility. Live UDP/TCP tests
exercise connection reuse and framing. `drill` provides an independent
encoder and decoder. The stable-Rust benchmark in
[`benches/dns_core.rs`](https://github.com/querygraph/rgbdns/blob/master/benches/dns_core.rs) measures the functions that
rgbdns itself owns, and [`docs/performance.md`](https://github.com/querygraph/rgbdns/blob/master/docs/performance.md) records both
timings and wire size.

This is where Rust most clearly changes the economics of a C rewrite. Memory
safety removes many failure modes before testing. Property tests can then
spend their budget on protocol structure rather than rediscovering use-after-
free and buffer-overrun variants. The remaining failures are more likely to be
interesting DNS mistakes.

```
