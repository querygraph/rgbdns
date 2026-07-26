---
type: "chapter"
source_file: "docs/book/rgbdns.md"
---

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

## Generated Code Fragment Index

These generated links open the collocated rgbdns codebase notes.

```rgbdns-fragment
{"id": "rgbdns-frag-fd362110f0ff", "codeNote": "DNS from First Principles/Code/src/name.rs.source", "heading": "rgbdns-frag-fd362110f0ff: struct Name", "sourcePath": "src/name.rs", "startLine": 15, "endLine": 16}
```

```rgbdns-fragment
{"id": "rgbdns-frag-0c521630e572", "codeNote": "DNS from First Principles/Code/src/name.rs.source", "heading": "rgbdns-frag-0c521630e572: impl PartialEq", "sourcePath": "src/name.rs", "startLine": 17, "endLine": 17}
```

```rgbdns-fragment
{"id": "rgbdns-frag-ad7b342aa5e0", "codeNote": "DNS from First Principles/Code/src/name.rs.source", "heading": "rgbdns-frag-ad7b342aa5e0: fn eq", "sourcePath": "src/name.rs", "startLine": 18, "endLine": 29}
```

```rgbdns-fragment
{"id": "rgbdns-frag-5aebca818d0e", "codeNote": "DNS from First Principles/Code/src/name.rs.source", "heading": "rgbdns-frag-5aebca818d0e: impl Eq", "sourcePath": "src/name.rs", "startLine": 30, "endLine": 31}
```

```rgbdns-fragment
{"id": "rgbdns-frag-fe46138a7df6", "codeNote": "DNS from First Principles/Code/src/name.rs.source", "heading": "rgbdns-frag-fe46138a7df6: impl Hash", "sourcePath": "src/name.rs", "startLine": 32, "endLine": 32}
```

```rgbdns-fragment
{"id": "rgbdns-frag-d375ebc0f305", "codeNote": "DNS from First Principles/Code/src/name.rs.source", "heading": "rgbdns-frag-d375ebc0f305: fn hash", "sourcePath": "src/name.rs", "startLine": 33, "endLine": 43}
```

```rgbdns-fragment
{"id": "rgbdns-frag-1ecb9ab00152", "codeNote": "DNS from First Principles/Code/src/name.rs.source", "heading": "rgbdns-frag-1ecb9ab00152: impl PartialOrd", "sourcePath": "src/name.rs", "startLine": 44, "endLine": 44}
```

```rgbdns-fragment
{"id": "rgbdns-frag-5f4c4b9589f4", "codeNote": "DNS from First Principles/Code/src/name.rs.source", "heading": "rgbdns-frag-5f4c4b9589f4: fn partial_cmp", "sourcePath": "src/name.rs", "startLine": 45, "endLine": 49}
```

```rgbdns-fragment
{"id": "rgbdns-frag-e4982a174651", "codeNote": "DNS from First Principles/Code/src/name.rs.source", "heading": "rgbdns-frag-e4982a174651: impl Ord", "sourcePath": "src/name.rs", "startLine": 50, "endLine": 50}
```

```rgbdns-fragment
{"id": "rgbdns-frag-0dc862d179e7", "codeNote": "DNS from First Principles/Code/src/name.rs.source", "heading": "rgbdns-frag-0dc862d179e7: fn cmp", "sourcePath": "src/name.rs", "startLine": 51, "endLine": 64}
```

```rgbdns-fragment
{"id": "rgbdns-frag-5029e4e250ec", "codeNote": "DNS from First Principles/Code/src/name.rs.source", "heading": "rgbdns-frag-5029e4e250ec: impl Name", "sourcePath": "src/name.rs", "startLine": 65, "endLine": 65}
```

```rgbdns-fragment
{"id": "rgbdns-frag-695a58a3c146", "codeNote": "DNS from First Principles/Code/src/name.rs.source", "heading": "rgbdns-frag-695a58a3c146: fn root", "sourcePath": "src/name.rs", "startLine": 66, "endLine": 68}
```
