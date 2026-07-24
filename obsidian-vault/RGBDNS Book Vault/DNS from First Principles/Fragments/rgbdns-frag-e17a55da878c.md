---
type: "code-fragment"
fragment_id: "rgbdns-frag-e17a55da878c"
source_path: "docs/blog/announcing-rgbdns/post.md"
code_note: "DNS from First Principles/Code/docs/blog/announcing-rgbdns/post.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "Measure before calling it faster"
kind: "heading"
start_line: 111
end_line: 136
---

# Measure before calling it faster

- Fragment ID: `rgbdns-frag-e17a55da878c`
- Source file: [[DNS from First Principles/Code/docs/blog/announcing-rgbdns/post.md.source|docs/blog/announcing-rgbdns/post.md]]
- Lines: 111-136
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-e17a55da878c", "codeNote": "DNS from First Principles/Code/docs/blog/announcing-rgbdns/post.md.source", "heading": "rgbdns-frag-e17a55da878c: heading Measure before calling it faster", "sourcePath": "docs/blog/announcing-rgbdns/post.md", "startLine": 111, "endLine": 136}
```

## Excerpt

<span id="rgbdns-frag-e17a55da878c" class="rgbdns-fragment-target"></span>
### rgbdns-frag-e17a55da878c: heading Measure before calling it faster

```markdown
## Measure before calling it faster

rgbdns includes a dependency-free stable-Rust benchmark for packet decoding,
packet encoding, exact and negative zone lookup, authoritative response
construction, and large-response truncation.

On the July 2026 aarch64 Android checkpoint:

- a 64-record response shrank from 2,147 bytes to 1,059 bytes;
- decoding that response improved from 52,661 ns to 29,540 ns;
- an absent-name lookup in a 1,000-name zone improved from 29,889 ns to
  2,726 ns;
- a small authoritative response improved from 17,007 ns to 7,714 ns;
- truncating a 200-record response improved from 3,098,232 ns to 2,570,077 ns.

The numbers also preserve an unfavorable result. Encoding the compressed
64-record response takes 5,309 ns instead of the uncompressed writer's
2,318 ns. That extra CPU buys roughly half the wire bytes and faster downstream
decoding. It is a reasonable DNS tradeoff, but only if it remains visible.

The largest speedup comes from an index of every zone node, including empty
non-terminals. A clearly absent name no longer scans the records of a
thousand-name zone. Truncation searches how many tail records must be removed
instead of encoding after every single removal. Name compression records
suffixes, while a last-owner cache makes repeated RRset owners cheap.

```
