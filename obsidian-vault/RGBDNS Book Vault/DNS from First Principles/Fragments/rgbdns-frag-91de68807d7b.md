---
type: "code-fragment"
fragment_id: "rgbdns-frag-91de68807d7b"
source_path: "docs/book/rgbdns.md"
code_note: "DNS from First Principles/Code/docs/book/rgbdns.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "Why a DNS suite contains time tools"
kind: "heading"
start_line: 725
end_line: 754
---

# Why a DNS suite contains time tools

- Fragment ID: `rgbdns-frag-91de68807d7b`
- Source file: [[DNS from First Principles/Code/docs/book/rgbdns.md.source|docs/book/rgbdns.md]]
- Lines: 725-754
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-91de68807d7b", "codeNote": "DNS from First Principles/Code/docs/book/rgbdns.md.source", "heading": "rgbdns-frag-91de68807d7b: heading Why a DNS suite contains time tools", "sourcePath": "docs/book/rgbdns.md", "startLine": 725, "endLine": 754}
```

## Excerpt

<span id="rgbdns-frag-91de68807d7b" class="rgbdns-fragment-target"></span>
### rgbdns-frag-91de68807d7b: heading Why a DNS suite contains time tools

```markdown
## Why a DNS suite contains time tools

Long-running daemons need logs, and djb’s tools use TAI64N labels. A label has
an `@`, sixteen hexadecimal digits of biased TAI seconds, and eight hexadecimal
digits of nanoseconds:

```text
@4000000037c219bf2ef02e94
```

TAI is a continuous atomic timescale. UTC inserts leap seconds, so converting
between a POSIX/UTC timestamp and TAI requires the applicable TAI−UTC offset.
The offset was 10 seconds at the Unix epoch convention used here and reached
37 seconds after the 2016 leap second.

`tai64n` timestamps each input line at the moment its first bytes are read.
`tai64nlocal` recognizes a valid leading label and replaces it with local civil
time at nanosecond precision. Invalid prefixes pass through unchanged.
`multilog t` uses the same label generator, ensuring standalone filters and log
files agree.

`src/tai64.rs` contains the complete 1972–2017 positive-leap transition table.
It validates the fixed-width hexadecimal form and nanosecond range. Both stream
filters use bounded memory: the localizer buffers only the 25-byte candidate
prefix and streams the rest of even an extremely long line. At I/O failure the
command exits 111, following the daemontools convention.

TAI64N provides sortable, unambiguous event labels. Converting to local time is
a presentation step, not the archival representation.

```
