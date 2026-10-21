---
type: "chapter"
source_file: "docs/book/rgbdns.md"
---

# Client behavior and diagnostics

## A query is more than sending bytes

A DNS client creates a random transaction ID, encodes one question, sends it
to an intended server, receives a response, and validates at least:

- source endpoint where the transport permits;
- transaction ID;
- QR and response shape;
- matching question;
- declared section lengths and names;
- truncation, with TCP retry when needed.

`src/client.rs` reads `DNSCACHEIP` or `/etc/resolv.conf`, supports IPv4 and
IPv6 socket syntax, gets IDs from the operating system, applies UDP timeouts,
rejects mismatched responses, and retries truncated UDP replies over TCP. The
small command binaries format results for different use cases, while `dnsq`
allows an explicit server and `dnsqr` uses recursive configuration.

`dnstrace` is conceptually different from a recursive lookup: it exposes the
delegation path and intermediate authority so an operator can see where the
chain stops. Good diagnosis asks four separate questions:

1. What did the stub send?
2. What did the recursive resolver cache or validate?
3. What delegation did the parent publish?
4. What does the authoritative server say directly?

Testing only the final application collapses all four layers and encourages
guessing.

## Practical checks

When a name fails, inspect type, server, flags, and authority section rather
than asking only whether an address appeared.

```sh
dnsq A www.example.com 192.0.2.53
dnsq AAAA www.example.com 192.0.2.53
dnsq SOA example.com 192.0.2.53
dnstrace A www.example.com
```

Compare UDP and TCP when answers are large. Query the parent-side NS records
and the child authority separately. An NXDOMAIN with an SOA is different from
a timeout, SERVFAIL, or REFUSED, and each points to a different layer.

## Generated Code Fragment Index

Each card names an rgbdns fragment. In FirstPair Reader, Show code lists its numbered lines and Open file opens the collocated code note with those lines marked.

```rgbdns-fragment
{"id": "rgbdns-frag-d8a776f4a8ff", "codeNote": "DNS from First Principles/Code/src/client.rs.source", "sourcePath": "src/client.rs", "startLine": 11, "endLine": 12}
```

```rgbdns-fragment
{"id": "rgbdns-frag-87ff6d272c3e", "codeNote": "DNS from First Principles/Code/src/client.rs.source", "sourcePath": "src/client.rs", "startLine": 13, "endLine": 16}
```

```rgbdns-fragment
{"id": "rgbdns-frag-8adf36f03d5d", "codeNote": "DNS from First Principles/Code/src/client.rs.source", "sourcePath": "src/client.rs", "startLine": 17, "endLine": 56}
```

```rgbdns-fragment
{"id": "rgbdns-frag-690e4ae608a4", "codeNote": "DNS from First Principles/Code/src/client.rs.source", "sourcePath": "src/client.rs", "startLine": 57, "endLine": 83}
```

```rgbdns-fragment
{"id": "rgbdns-frag-c4ff7d3f54c7", "codeNote": "DNS from First Principles/Code/src/client.rs.source", "sourcePath": "src/client.rs", "startLine": 84, "endLine": 93}
```

```rgbdns-fragment
{"id": "rgbdns-frag-5406a3b7ab21", "codeNote": "DNS from First Principles/Code/src/client.rs.source", "sourcePath": "src/client.rs", "startLine": 94, "endLine": 109}
```

```rgbdns-fragment
{"id": "rgbdns-frag-4c8bd136a924", "codeNote": "DNS from First Principles/Code/src/client.rs.source", "sourcePath": "src/client.rs", "startLine": 110, "endLine": 126}
```

```rgbdns-fragment
{"id": "rgbdns-frag-460f16558db2", "codeNote": "DNS from First Principles/Code/src/client.rs.source", "sourcePath": "src/client.rs", "startLine": 127, "endLine": 139}
```

```rgbdns-fragment
{"id": "rgbdns-frag-3d3fb10dac1b", "codeNote": "DNS from First Principles/Code/src/client.rs.source", "sourcePath": "src/client.rs", "startLine": 140, "endLine": 147}
```

```rgbdns-fragment
{"id": "rgbdns-frag-99e21bd0c5e3", "codeNote": "DNS from First Principles/Code/src/client.rs.source", "sourcePath": "src/client.rs", "startLine": 148, "endLine": 153}
```

```rgbdns-fragment
{"id": "rgbdns-frag-681cb74d8929", "codeNote": "DNS from First Principles/Code/src/client.rs.source", "sourcePath": "src/client.rs", "startLine": 154, "endLine": 207}
```

```rgbdns-fragment
{"id": "rgbdns-frag-b8619bcfbdf7", "codeNote": "DNS from First Principles/Code/src/client.rs.source", "sourcePath": "src/client.rs", "startLine": 208, "endLine": 219}
```
