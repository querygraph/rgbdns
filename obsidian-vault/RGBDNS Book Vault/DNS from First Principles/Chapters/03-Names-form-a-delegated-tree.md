---
type: "chapter"
source_file: "docs/book/rgbdns.md"
---

# Names form a delegated tree

## Labels and the root

A DNS name is a sequence of labels. In the presentation form
`www.example.com.`, the dots separate the labels `www`, `example`, and `com`.
The final dot represents the root’s empty label. Reading from right to left
walks from general to specific:

```text
.                     root
└── com.               top-level domain
    └── example.com.   delegated domain
        └── www.example.com.
```

The absolute name has a wire limit of 255 octets, including length bytes and
the terminating root label. Each ordinary label is at most 63 octets. DNS
names are not inherently UTF-8 strings. Internationalized names are normally
converted by applications into ASCII-compatible labels before DNS sees them.

DNS comparison is case-insensitive for ordinary ASCII letters, although the
original spelling can be preserved. A robust implementation therefore needs a
canonical comparison rule without losing the bytes required for encoding.

In rgbdns, `src/name.rs` represents a name as a vector of byte-vector labels.
Construction validates label and total lengths. Parsing accepts the familiar
dotted form and backslash escapes. The type provides parent and subdomain
operations, case-insensitive ordering, display formatting, and wire encoding.
Making invalid names difficult to construct removes repeated checks from the
rest of the system.

## Zones are administrative cuts

The namespace is one tree; a zone is an administratively served portion of
that tree. The two are not identical.

The zone `example.com.` might contain records for `www.example.com.` and
`mail.example.com.`, then delegate `research.example.com.` to other servers.
The child remains below `example.com.` in the namespace but is outside the
parent zone’s authoritative contents.

A delegation is expressed by NS records at the cut. If a named server lies
inside the delegated child, a resolver cannot first resolve that server’s name
through the child—it needs its address in order to reach the child. The parent
therefore supplies an address record called **glue**. Glue is navigation data,
not an assertion that the parent is authoritative for every fact about the
host.

The root zone delegates top-level domains. A cold recursive resolver starts
with a small configured set of root server addresses, asks the root where to
find a top-level domain, asks that domain where to find the next child, and
continues.

## Wildcards are synthesis rules

A wildcard such as `*.example.com.` does not mean “return this record for every
name ending in example.com.” It participates only when the queried name does
not exist, and the closest-encloser rules determine which wildcard, if any,
can synthesize an answer. Existing intermediate names can block a wildcard.

rgbdns stores wildcard records under their literal wildcard owner and its zone
lookup searches from the queried name toward the closest existing ancestor.
It synthesizes the queried owner in returned records. This is more precise
than a string suffix match and is one reason the `Zone` abstraction tracks
known nodes in addition to records.

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
