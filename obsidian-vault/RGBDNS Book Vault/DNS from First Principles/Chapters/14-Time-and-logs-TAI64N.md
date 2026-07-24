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

These generated links open the collocated rgbdns codebase notes.

```rgbdns-fragment
{"id": "rgbdns-frag-cfb0faf3b090", "codeNote": "DNS from First Principles/Code/README.md.source", "heading": "rgbdns-frag-cfb0faf3b090: heading rgbdns", "sourcePath": "README.md", "startLine": 1, "endLine": 43}
```

```rgbdns-fragment
{"id": "rgbdns-frag-31920af81303", "codeNote": "DNS from First Principles/Code/README.md.source", "heading": "rgbdns-frag-31920af81303: heading Book", "sourcePath": "README.md", "startLine": 44, "endLine": 63}
```

```rgbdns-fragment
{"id": "rgbdns-frag-855d7f20ae34", "codeNote": "DNS from First Principles/Code/README.md.source", "heading": "rgbdns-frag-855d7f20ae34: heading Conformance and performance", "sourcePath": "README.md", "startLine": 64, "endLine": 75}
```

```rgbdns-fragment
{"id": "rgbdns-frag-d26839b00cc0", "codeNote": "DNS from First Principles/Code/src/lib.rs.source", "heading": "rgbdns-frag-d26839b00cc0: mod axfr", "sourcePath": "src/lib.rs", "startLine": 3, "endLine": 3}
```

```rgbdns-fragment
{"id": "rgbdns-frag-c0387f24e3b2", "codeNote": "DNS from First Principles/Code/src/lib.rs.source", "heading": "rgbdns-frag-c0387f24e3b2: mod cdb", "sourcePath": "src/lib.rs", "startLine": 4, "endLine": 4}
```

```rgbdns-fragment
{"id": "rgbdns-frag-ea9a942c2c24", "codeNote": "DNS from First Principles/Code/src/lib.rs.source", "heading": "rgbdns-frag-ea9a942c2c24: mod client", "sourcePath": "src/lib.rs", "startLine": 5, "endLine": 5}
```

```rgbdns-fragment
{"id": "rgbdns-frag-4ca6b4a66ffe", "codeNote": "DNS from First Principles/Code/src/lib.rs.source", "heading": "rgbdns-frag-4ca6b4a66ffe: mod conf", "sourcePath": "src/lib.rs", "startLine": 6, "endLine": 6}
```

```rgbdns-fragment
{"id": "rgbdns-frag-47cdd9e7aa73", "codeNote": "DNS from First Principles/Code/src/lib.rs.source", "heading": "rgbdns-frag-47cdd9e7aa73: mod dnscache_config", "sourcePath": "src/lib.rs", "startLine": 7, "endLine": 7}
```

```rgbdns-fragment
{"id": "rgbdns-frag-64be14e515dc", "codeNote": "DNS from First Principles/Code/src/lib.rs.source", "heading": "rgbdns-frag-64be14e515dc: mod multilog", "sourcePath": "src/lib.rs", "startLine": 8, "endLine": 8}
```

```rgbdns-fragment
{"id": "rgbdns-frag-1198b492e8d3", "codeNote": "DNS from First Principles/Code/src/lib.rs.source", "heading": "rgbdns-frag-1198b492e8d3: mod name", "sourcePath": "src/lib.rs", "startLine": 9, "endLine": 9}
```

```rgbdns-fragment
{"id": "rgbdns-frag-9a69adb381ac", "codeNote": "DNS from First Principles/Code/src/lib.rs.source", "heading": "rgbdns-frag-9a69adb381ac: mod packet", "sourcePath": "src/lib.rs", "startLine": 10, "endLine": 10}
```

```rgbdns-fragment
{"id": "rgbdns-frag-cc4b83e1818c", "codeNote": "DNS from First Principles/Code/src/lib.rs.source", "heading": "rgbdns-frag-cc4b83e1818c: mod pick", "sourcePath": "src/lib.rs", "startLine": 11, "endLine": 11}
```
