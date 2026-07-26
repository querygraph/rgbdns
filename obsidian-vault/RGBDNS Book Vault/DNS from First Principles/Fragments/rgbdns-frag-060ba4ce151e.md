---
type: "code-fragment"
fragment_id: "rgbdns-frag-060ba4ce151e"
source_path: "docs/book/rgbdns.md"
code_note: "DNS from First Principles/Code/docs/book/rgbdns.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "Name compression"
kind: "heading"
start_line: 279
end_line: 299
---

# Name compression

- Fragment ID: `rgbdns-frag-060ba4ce151e`
- Source file: [[DNS from First Principles/Code/docs/book/rgbdns.md.source|docs/book/rgbdns.md]]
- Lines: 279-299
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-060ba4ce151e", "codeNote": "DNS from First Principles/Code/docs/book/rgbdns.md.source", "heading": "rgbdns-frag-060ba4ce151e: heading Name compression", "sourcePath": "docs/book/rgbdns.md", "startLine": 279, "endLine": 299}
```

## Excerpt

<span id="rgbdns-frag-060ba4ce151e" class="rgbdns-fragment-target"></span>
### rgbdns-frag-060ba4ce151e: heading Name compression

```markdown
## Name compression

Repeating full names would waste scarce datagram space. DNS permits a name
suffix to be replaced by a two-byte pointer whose high bits are `11` and whose
remaining bits are an offset earlier in the message.

Compression turns name decoding into graph traversal. A malicious packet can
contain a pointer loop, excessive indirection, or an offset outside the packet.
A safe decoder tracks visited offsets or imposes a strict jump bound, checks
every target, and separately enforces the expanded 255-octet name limit.

rgbdns’s `Reader` in `src/packet.rs` keeps all reads within a borrowed byte
slice. Name decoding validates pointer targets and bounds traversal. Record
decoding confines each RDATA parser to the declared RDLENGTH. EDNS option
iteration likewise checks the option header and value before advancing.

The `Writer` performs the reverse operation. Encoding is fallible: counts must
fit 16 bits, names and RDATA must fit their fields, and the result must remain
valid. This symmetry—decode into typed data, manipulate typed data, encode with
checks—is the packet layer’s central safety property.

```
