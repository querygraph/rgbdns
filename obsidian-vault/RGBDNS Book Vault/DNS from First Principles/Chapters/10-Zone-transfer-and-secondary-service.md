---
type: "chapter"
source_file: "docs/book/rgbdns.md"
---

# Zone transfer and secondary service

## AXFR is a stream, not a giant datagram

AXFR transfers a complete zone over TCP. A successful stream begins with the
zone’s SOA, contains the zone records, and ends with the SOA again. The records
may span many DNS messages. A client must continue until it sees the closing
SOA under the transfer rules; reading one response is insufficient.

Transfers reveal the zone contents and can consume resources, so authorities
normally restrict clients. TSIG is a common authentication mechanism in the
wider ecosystem, while IP allowlists are a simpler policy with weaker identity
properties.

`src/axfr.rs` provides both sides. `axfrdns` accepts TCP only and checks client
networks, loopback by default. It requires one AXFR question, obtains a
boundary-aware transfer from `Zone`, and frames bounded messages. `Zone::transfer`
excludes records beneath delegated child zones and wraps the result in the
apex SOA.

`axfr-get` generates a random transaction ID, validates response identity and
shape, collects records until the closing SOA, renders them in tinydns source
form, writes a temporary output, and atomically installs the completed file.
The temporary/final path pair prevents a failed transfer from replacing usable
data with a partial zone.

## Generated Code Fragment Index

These generated links open the collocated rgbdns codebase notes.

```rgbdns-fragment
{"id": "rgbdns-frag-ad1ed1e4cb84", "codeNote": "DNS from First Principles/Code/src/axfr.rs.source", "heading": "rgbdns-frag-ad1ed1e4cb84: const MAX_TCP_MESSAGE", "sourcePath": "src/axfr.rs", "startLine": 15, "endLine": 15}
```

```rgbdns-fragment
{"id": "rgbdns-frag-169fc0dbbb4a", "codeNote": "DNS from First Principles/Code/src/axfr.rs.source", "heading": "rgbdns-frag-169fc0dbbb4a: const MAX_TRANSFER_RECORDS", "sourcePath": "src/axfr.rs", "startLine": 16, "endLine": 16}
```

```rgbdns-fragment
{"id": "rgbdns-frag-704dd44da912", "codeNote": "DNS from First Principles/Code/src/axfr.rs.source", "heading": "rgbdns-frag-704dd44da912: const MAX_TRANSFER_MESSAGES", "sourcePath": "src/axfr.rs", "startLine": 17, "endLine": 17}
```

```rgbdns-fragment
{"id": "rgbdns-frag-aee4934c3db3", "codeNote": "DNS from First Principles/Code/src/axfr.rs.source", "heading": "rgbdns-frag-aee4934c3db3: const MAX_TRANSFER_BYTES", "sourcePath": "src/axfr.rs", "startLine": 18, "endLine": 19}
```

```rgbdns-fragment
{"id": "rgbdns-frag-884813b3737c", "codeNote": "DNS from First Principles/Code/src/axfr.rs.source", "heading": "rgbdns-frag-884813b3737c: fn serve", "sourcePath": "src/axfr.rs", "startLine": 20, "endLine": 27}
```

```rgbdns-fragment
{"id": "rgbdns-frag-9a2021bd9440", "codeNote": "DNS from First Principles/Code/src/axfr.rs.source", "heading": "rgbdns-frag-9a2021bd9440: fn serve_listener", "sourcePath": "src/axfr.rs", "startLine": 28, "endLine": 61}
```

```rgbdns-fragment
{"id": "rgbdns-frag-2554d1b3665a", "codeNote": "DNS from First Principles/Code/src/axfr.rs.source", "heading": "rgbdns-frag-2554d1b3665a: fn serve_connection", "sourcePath": "src/axfr.rs", "startLine": 62, "endLine": 122}
```

```rgbdns-fragment
{"id": "rgbdns-frag-8c366d542d3c", "codeNote": "DNS from First Principles/Code/src/axfr.rs.source", "heading": "rgbdns-frag-8c366d542d3c: fn write_response", "sourcePath": "src/axfr.rs", "startLine": 123, "endLine": 138}
```

```rgbdns-fragment
{"id": "rgbdns-frag-4268f778fa47", "codeNote": "DNS from First Principles/Code/src/axfr.rs.source", "heading": "rgbdns-frag-4268f778fa47: fn response_wire", "sourcePath": "src/axfr.rs", "startLine": 139, "endLine": 154}
```

```rgbdns-fragment
{"id": "rgbdns-frag-454fe9e80b77", "codeNote": "DNS from First Principles/Code/src/axfr.rs.source", "heading": "rgbdns-frag-454fe9e80b77: fn read_message", "sourcePath": "src/axfr.rs", "startLine": 155, "endLine": 162}
```

```rgbdns-fragment
{"id": "rgbdns-frag-3985e9023e0b", "codeNote": "DNS from First Principles/Code/src/axfr.rs.source", "heading": "rgbdns-frag-3985e9023e0b: fn fetch", "sourcePath": "src/axfr.rs", "startLine": 163, "endLine": 229}
```

```rgbdns-fragment
{"id": "rgbdns-frag-a31c068b7119", "codeNote": "DNS from First Principles/Code/src/axfr.rs.source", "heading": "rgbdns-frag-a31c068b7119: fn validate_axfr_message", "sourcePath": "src/axfr.rs", "startLine": 230, "endLine": 255}
```
