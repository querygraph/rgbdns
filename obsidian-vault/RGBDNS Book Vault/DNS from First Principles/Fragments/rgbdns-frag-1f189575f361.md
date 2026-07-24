---
type: "code-fragment"
fragment_id: "rgbdns-frag-1f189575f361"
source_path: "docs/book/rgbdns.md"
code_note: "DNS from First Principles/Code/docs/book/rgbdns.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "A bounded wire codec"
kind: "heading"
start_line: 1283
end_line: 1320
---

# A bounded wire codec

- Fragment ID: `rgbdns-frag-1f189575f361`
- Source file: [[DNS from First Principles/Code/docs/book/rgbdns.md.source|docs/book/rgbdns.md]]
- Lines: 1283-1320
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-1f189575f361", "codeNote": "DNS from First Principles/Code/docs/book/rgbdns.md.source", "heading": "rgbdns-frag-1f189575f361: heading A bounded wire codec", "sourcePath": "docs/book/rgbdns.md", "startLine": 1283, "endLine": 1320}
```

## Excerpt

<span id="rgbdns-frag-1f189575f361" class="rgbdns-fragment-target"></span>
### rgbdns-frag-1f189575f361: heading A bounded wire codec

```markdown
# A bounded wire codec

DNS packets are attacker-controlled binary graphs: compression pointers can
jump backward, names can share suffixes, section counts can lie, and RDATA
lengths can disagree with actual bytes. [`packet.rs`](../../src/packet.rs)
contains the codec and deliberately keeps the reader state small:

```rust
struct Reader<'a> {
    b: &'a [u8],
    p: usize,
    name_offsets: Vec<bool>,
}
```

The lifetime on `b` prevents the reader from outliving its input. Every scalar
read uses slice bounds checks. Name decoding separately limits pointer hops,
requires pointers to move backward, and records valid prior name boundaries.
The last rule is stricter than merely checking that a pointer lands inside the
packet: an interior byte can accidentally look like a valid label.

Decoded records become an `RData` variant. Unknown types are not discarded;
they become opaque bytes paired with their numeric `RecordType`. This is the
extension-safe behavior required by modern DNS. It also means an encoder can
round-trip data it does not understand.

The writer uses compression, but optimization remains subordinate to a valid
message. A last-owner cache handles repeated owners cheaply while suffix
sharing reduces wire size across related names. The July 2026 benchmark shows
the trade: a 64-record answer fell from 2,147 to 1,059 bytes, while compression
made encoding slower than the uncompressed baseline. On DNS, fewer datagrams
and less amplification surface can be worth several microseconds of local CPU.

Truncation uses a bounded search for the largest response that fits instead of
repeatedly rebuilding one record at a time. The result preserves complete
RRsets and required EDNS state. Performance work is therefore expressed as an
algorithmic improvement behind the same `Message::encode` contract.

```
