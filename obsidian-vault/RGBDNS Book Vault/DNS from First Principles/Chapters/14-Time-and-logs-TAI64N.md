---
type: "chapter"
source_file: "docs/book/rgbdns.md"
---

# Time and logs: TAI64N

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

## Generated Code Fragment Index

Each card names an rgbdns fragment. In FirstPair Reader, Show code lists its numbered lines and Open file opens the collocated code note with those lines marked.

```rgbdns-fragment
{"id": "rgbdns-frag-df36f646ddea", "codeNote": "DNS from First Principles/Code/README.md.source", "sourcePath": "README.md", "startLine": 1, "endLine": 32}
```

```rgbdns-fragment
{"id": "rgbdns-frag-530a102cf907", "codeNote": "DNS from First Principles/Code/README.md.source", "sourcePath": "README.md", "startLine": 33, "endLine": 48}
```

```rgbdns-fragment
{"id": "rgbdns-frag-087f5fd9bf6b", "codeNote": "DNS from First Principles/Code/README.md.source", "sourcePath": "README.md", "startLine": 49, "endLine": 111}
```

```rgbdns-fragment
{"id": "rgbdns-frag-c50fc5b27770", "codeNote": "DNS from First Principles/Code/README.md.source", "sourcePath": "README.md", "startLine": 112, "endLine": 127}
```

```rgbdns-fragment
{"id": "rgbdns-frag-8f6c800e8ac3", "codeNote": "DNS from First Principles/Code/README.md.source", "sourcePath": "README.md", "startLine": 128, "endLine": 187}
```

```rgbdns-fragment
{"id": "rgbdns-frag-44ef9d3327cb", "codeNote": "DNS from First Principles/Code/README.md.source", "sourcePath": "README.md", "startLine": 188, "endLine": 207}
```

```rgbdns-fragment
{"id": "rgbdns-frag-d1fb368f56c0", "codeNote": "DNS from First Principles/Code/README.md.source", "sourcePath": "README.md", "startLine": 208, "endLine": 219}
```

```rgbdns-fragment
{"id": "rgbdns-frag-0764202cbb5b", "codeNote": "DNS from First Principles/Code/src/lib.rs.source", "sourcePath": "src/lib.rs", "startLine": 3, "endLine": 3}
```

```rgbdns-fragment
{"id": "rgbdns-frag-d814f58c3808", "codeNote": "DNS from First Principles/Code/src/lib.rs.source", "sourcePath": "src/lib.rs", "startLine": 4, "endLine": 4}
```

```rgbdns-fragment
{"id": "rgbdns-frag-137fad6744bd", "codeNote": "DNS from First Principles/Code/src/lib.rs.source", "sourcePath": "src/lib.rs", "startLine": 5, "endLine": 5}
```

```rgbdns-fragment
{"id": "rgbdns-frag-56c62ce446e3", "codeNote": "DNS from First Principles/Code/src/lib.rs.source", "sourcePath": "src/lib.rs", "startLine": 6, "endLine": 6}
```

```rgbdns-fragment
{"id": "rgbdns-frag-bc885b4aa7d2", "codeNote": "DNS from First Principles/Code/src/lib.rs.source", "sourcePath": "src/lib.rs", "startLine": 7, "endLine": 7}
```
