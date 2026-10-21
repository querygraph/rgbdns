---
type: "chapter"
source_file: "docs/book/rgbdns.md"
---

# CDB compatibility without trusting the file

Compatibility is most valuable at the data boundary. rgbdns reads and writes
the original tinydns `data.cdb` layout, so operators can preserve compilation
and rollout habits. [`cdb.rs`](https://github.com/querygraph/rgbdns/blob/master/src/cdb.rs) does not, however, inherit the
old assumption that the compiled file is trustworthy.

The loader applies independent limits and checked arithmetic:

- the complete database is capped at one GiB;
- the 2,048-byte CDB header must exist;
- every hash-table position and slot count must fit inside the file;
- key and value lengths use `checked_add`;
- markers, locations, names, TTLs, cutoffs, and type-specific RDATA are
  validated before a `Record` enters a `Zone`.

This is a useful modernization pattern: preserve a durable external format,
replace its implicit in-memory trust model. Operators gain compatibility; the
serving process receives validated Rust values rather than pointers into a
memory-mapped byte region.

Compilation likewise crosses an explicit boundary. A parsed `Zone` is written
to a temporary CDB and then installed through the command workflow. Serving
data is immutable between deployments. Rust’s ownership does not itself make
the rollout atomic, but it makes the stages—source text, validated model,
compiled bytes, installed file—unambiguous.

## Generated Code Fragment Index

Each card names an rgbdns fragment. In FirstPair Reader, Show code lists its numbered lines and Open file opens the collocated code note with those lines marked.

```rgbdns-fragment
{"id": "rgbdns-frag-e5fff4b8cb2b", "codeNote": "DNS from First Principles/Code/src/cdb.rs.source", "sourcePath": "src/cdb.rs", "startLine": 9, "endLine": 9}
```

```rgbdns-fragment
{"id": "rgbdns-frag-4907bb687e44", "codeNote": "DNS from First Principles/Code/src/cdb.rs.source", "sourcePath": "src/cdb.rs", "startLine": 10, "endLine": 11}
```

```rgbdns-fragment
{"id": "rgbdns-frag-f318af8bdeaa", "codeNote": "DNS from First Principles/Code/src/cdb.rs.source", "sourcePath": "src/cdb.rs", "startLine": 12, "endLine": 61}
```

```rgbdns-fragment
{"id": "rgbdns-frag-f67eebb3c015", "codeNote": "DNS from First Principles/Code/src/cdb.rs.source", "sourcePath": "src/cdb.rs", "startLine": 62, "endLine": 101}
```

```rgbdns-fragment
{"id": "rgbdns-frag-916ec1cbc28e", "codeNote": "DNS from First Principles/Code/src/cdb.rs.source", "sourcePath": "src/cdb.rs", "startLine": 102, "endLine": 156}
```

```rgbdns-fragment
{"id": "rgbdns-frag-f9047bc1a1a2", "codeNote": "DNS from First Principles/Code/src/cdb.rs.source", "sourcePath": "src/cdb.rs", "startLine": 157, "endLine": 210}
```

```rgbdns-fragment
{"id": "rgbdns-frag-bf06479bb119", "codeNote": "DNS from First Principles/Code/src/cdb.rs.source", "sourcePath": "src/cdb.rs", "startLine": 211, "endLine": 269}
```

```rgbdns-fragment
{"id": "rgbdns-frag-f2d13363a376", "codeNote": "DNS from First Principles/Code/src/cdb.rs.source", "sourcePath": "src/cdb.rs", "startLine": 270, "endLine": 296}
```

```rgbdns-fragment
{"id": "rgbdns-frag-ca380a5004ce", "codeNote": "DNS from First Principles/Code/src/cdb.rs.source", "sourcePath": "src/cdb.rs", "startLine": 297, "endLine": 301}
```

```rgbdns-fragment
{"id": "rgbdns-frag-9cc58af3bb02", "codeNote": "DNS from First Principles/Code/src/cdb.rs.source", "sourcePath": "src/cdb.rs", "startLine": 302, "endLine": 307}
```

```rgbdns-fragment
{"id": "rgbdns-frag-d600fc524f2b", "codeNote": "DNS from First Principles/Code/src/cdb.rs.source", "sourcePath": "src/cdb.rs", "startLine": 308, "endLine": 364}
```

```rgbdns-fragment
{"id": "rgbdns-frag-428b1ce3e4be", "codeNote": "DNS from First Principles/Code/src/cdb.rs.source", "sourcePath": "src/cdb.rs", "startLine": 365, "endLine": 371}
```
