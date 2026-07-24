---
type: "chapter"
source_file: "docs/book/rgbdns.md"
---

# CDB compatibility without trusting the file

Compatibility is most valuable at the data boundary. rgbdns reads and writes
the original tinydns `data.cdb` layout, so operators can preserve compilation
and rollout habits. [`cdb.rs`](../../src/cdb.rs) does not, however, inherit the
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

These generated links open the collocated rgbdns codebase notes.

```rgbdns-fragment
{"id": "rgbdns-frag-e5fff4b8cb2b", "codeNote": "DNS from First Principles/Code/src/cdb.rs.source", "heading": "rgbdns-frag-e5fff4b8cb2b: const HEADER_LEN", "sourcePath": "src/cdb.rs", "startLine": 9, "endLine": 9}
```

```rgbdns-fragment
{"id": "rgbdns-frag-4907bb687e44", "codeNote": "DNS from First Principles/Code/src/cdb.rs.source", "heading": "rgbdns-frag-4907bb687e44: const MAX_DATABASE_SIZE", "sourcePath": "src/cdb.rs", "startLine": 10, "endLine": 11}
```

```rgbdns-fragment
{"id": "rgbdns-frag-4a9794b88127", "codeNote": "DNS from First Principles/Code/src/cdb.rs.source", "heading": "rgbdns-frag-4a9794b88127: fn compile", "sourcePath": "src/cdb.rs", "startLine": 12, "endLine": 52}
```

```rgbdns-fragment
{"id": "rgbdns-frag-83b463908a3c", "codeNote": "DNS from First Principles/Code/src/cdb.rs.source", "heading": "rgbdns-frag-83b463908a3c: fn load", "sourcePath": "src/cdb.rs", "startLine": 53, "endLine": 69}
```

```rgbdns-fragment
{"id": "rgbdns-frag-9e8e0d51389c", "codeNote": "DNS from First Principles/Code/src/cdb.rs.source", "heading": "rgbdns-frag-9e8e0d51389c: fn read_entries", "sourcePath": "src/cdb.rs", "startLine": 70, "endLine": 124}
```

```rgbdns-fragment
{"id": "rgbdns-frag-4a71bdeba2ec", "codeNote": "DNS from First Principles/Code/src/cdb.rs.source", "heading": "rgbdns-frag-4a71bdeba2ec: fn decode_record", "sourcePath": "src/cdb.rs", "startLine": 125, "endLine": 178}
```

```rgbdns-fragment
{"id": "rgbdns-frag-a98232a8cdb0", "codeNote": "DNS from First Principles/Code/src/cdb.rs.source", "heading": "rgbdns-frag-a98232a8cdb0: fn encode_rdata", "sourcePath": "src/cdb.rs", "startLine": 179, "endLine": 237}
```

```rgbdns-fragment
{"id": "rgbdns-frag-1cfb12457767", "codeNote": "DNS from First Principles/Code/src/cdb.rs.source", "heading": "rgbdns-frag-1cfb12457767: fn decode_name", "sourcePath": "src/cdb.rs", "startLine": 238, "endLine": 264}
```

```rgbdns-fragment
{"id": "rgbdns-frag-908320134aee", "codeNote": "DNS from First Principles/Code/src/cdb.rs.source", "heading": "rgbdns-frag-908320134aee: fn le_u32", "sourcePath": "src/cdb.rs", "startLine": 265, "endLine": 269}
```

```rgbdns-fragment
{"id": "rgbdns-frag-489b39a43ae6", "codeNote": "DNS from First Principles/Code/src/cdb.rs.source", "heading": "rgbdns-frag-489b39a43ae6: mod tests", "sourcePath": "src/cdb.rs", "startLine": 270, "endLine": 275}
```

```rgbdns-fragment
{"id": "rgbdns-frag-563d0d5def13", "codeNote": "DNS from First Principles/Code/src/cdb.rs.source", "heading": "rgbdns-frag-563d0d5def13: fn exact_cdb_roundtrip_preserves_lookup_semantics", "sourcePath": "src/cdb.rs", "startLine": 276, "endLine": 324}
```

```rgbdns-fragment
{"id": "rgbdns-frag-af1b07391a27", "codeNote": "DNS from First Principles/Code/src/cdb.rs.source", "heading": "rgbdns-frag-af1b07391a27: fn rejects_truncated_database", "sourcePath": "src/cdb.rs", "startLine": 325, "endLine": 331}
```
