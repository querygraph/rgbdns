---
type: "code-fragment"
fragment_id: "rgbdns-frag-513fe5bd319b"
source_path: "docs/book/rgbdns.md"
code_note: "DNS from First Principles/Code/docs/book/rgbdns.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "Abstractions and performance ledger"
kind: "heading"
start_line: 1480
end_line: 1508
---

# Abstractions and performance ledger

- Fragment ID: `rgbdns-frag-513fe5bd319b`
- Source file: [[DNS from First Principles/Code/docs/book/rgbdns.md.source|docs/book/rgbdns.md]]
- Lines: 1480-1508
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-513fe5bd319b", "codeNote": "DNS from First Principles/Code/docs/book/rgbdns.md.source", "heading": "rgbdns-frag-513fe5bd319b: heading Abstractions and performance ledger", "sourcePath": "docs/book/rgbdns.md", "startLine": 1480, "endLine": 1508}
```

## Excerpt

<span id="rgbdns-frag-513fe5bd319b" class="rgbdns-fragment-target"></span>
### rgbdns-frag-513fe5bd319b: heading Abstractions and performance ledger

```markdown
# Abstractions and performance ledger

The rewrite’s gains are not a single “Rust is faster” claim. Some changes buy
safety, some buy clarity, and some measurably improve a hot path.

| Design move | Rust expression | Operational effect |
|---|---|---|
| Valid names at construction | private fields, `FromStr`, `Result` | invalid labels cannot circulate |
| Complete DNS states | `RecordType`, `RData`, `Lookup` enums | unknown types survive; negative answers stay distinct |
| Bounded packet access | borrowed slices and checked indexing | malformed packets fail without memory corruption |
| Shared immutable service state | borrowing and `Arc` | explicit thread-safe ownership |
| Compatibility quarantine | checked CDB decoder into owned values | old files do not become trusted memory |
| Independent resource limits | typed bounded configuration | one limit cannot silently stand in for another |
| All-node zone index | `BTreeSet<Name>` | NXDOMAIN lookup improved about 11× |
| Compressed writer | bounded suffix reuse | repeated-owner answer became 50.7% smaller |
| Binary-search truncation | monotone fit search | 200-record truncation improved 17% |
| Thin binaries | library functions plus small adapters | easier tests and independent supervision |

The encoder example is especially important. Rust did not automatically make
it faster: compression made encoding 2.3 times slower than the uncompressed
baseline. But it halved the measured wire size, reduced fragmentation risk,
and sharply improved the complete authoritative response path. Good systems
engineering reports the trade rather than reducing it to a language slogan.

The deeper improvement over C is control. The data model says what is valid,
the compiler checks ownership, the boundaries return typed failure, the tests
state protocol properties, and benchmarks expose the remaining costs. That
combination makes rgbdns easier to change without making DNS easier to fool.

```
