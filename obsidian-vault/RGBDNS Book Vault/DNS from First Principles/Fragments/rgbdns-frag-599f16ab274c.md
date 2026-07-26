---
type: "code-fragment"
fragment_id: "rgbdns-frag-599f16ab274c"
source_path: "docs/book/rgbdns.md"
code_note: "DNS from First Principles/Code/docs/book/rgbdns.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "Rust as a protocol-design tool"
kind: "heading"
start_line: 1469
end_line: 1514
---

# Rust as a protocol-design tool

- Fragment ID: `rgbdns-frag-599f16ab274c`
- Source file: [[DNS from First Principles/Code/docs/book/rgbdns.md.source|docs/book/rgbdns.md]]
- Lines: 1469-1514
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-599f16ab274c", "codeNote": "DNS from First Principles/Code/docs/book/rgbdns.md.source", "heading": "rgbdns-frag-599f16ab274c: heading Rust as a protocol-design tool", "sourcePath": "docs/book/rgbdns.md", "startLine": 1469, "endLine": 1514}
```

## Excerpt

<span id="rgbdns-frag-599f16ab274c" class="rgbdns-fragment-target"></span>
### rgbdns-frag-599f16ab274c: heading Rust as a protocol-design tool

```markdown
# Rust as a protocol-design tool

The important comparison with the original C is not that Rust uses newer
syntax. It is that rgbdns can express DNS invariants at boundaries and have the
compiler preserve them across the program.

The crate begins with `#![forbid(unsafe_code)]` in
[`src/lib.rs`](https://github.com/querygraph/rgbdns/blob/master/src/lib.rs). This is stronger than merely having no
currently known unsafe block: a later contribution cannot introduce one
without first making an explicit, reviewable change to the crate policy. The
implementation still performs byte-level packet parsing, binary CDB loading,
socket I/O, process credential changes, and concurrency. Those jobs do not
require unchecked pointer arithmetic.

Rust improves the design along four axes.

**Ownership makes lifetime and aliasing rules executable.** A decoded
`Message` owns its questions and records. A server borrows a `Zone` while
constructing a response. Shared handlers use `Arc`, making cross-thread
ownership visible in the type rather than implicit in process convention.
There is no corresponding path for a response to retain a dangling pointer
into the query buffer.

**Algebraic data types preserve protocol distinctions.** `RecordType` retains
unknown numeric types as `Unknown(u16)`. `RData` distinguishes addresses,
names, MX, SOA, TXT, SRV, CAA, EDNS, and opaque future data. `Lookup`
distinguishes an answer, referral, NODATA, NXDOMAIN, and refusal. In C these
states are commonly represented by related integers, nullable pointers, and
out-parameters. In Rust, a `match` makes the cases locally exhaustive.

**Fallible work is visible.** Parsing and I/O return `Result`; optional facts
return `Option`. The `?` operator propagates a typed failure without the
unchecked sentinel values that make C error paths easy to omit. Conversion
such as `u16::try_from(response.len())` documents the narrowing boundary and
rejects overflow.

**Concurrency composes with ownership.** The bounded transport layer shares an
immutable handler through `Arc<Handler>` and gives each TCP connection an
exclusive mutable `TcpStream`. The compiler prevents a worker from retaining a
borrow of a stack packet buffer after the buffer is reused.

Rust does not prove the DNS protocol correct. It removes broad classes of
memory corruption and turns many design assumptions into compile-time or
construction-time obligations. The remaining protocol work becomes visible
enough to test directly.

```
