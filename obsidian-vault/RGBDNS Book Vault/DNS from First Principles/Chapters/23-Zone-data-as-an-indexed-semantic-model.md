---
type: "chapter"
source_file: "docs/book/rgbdns.md"
---

# Zone data as an indexed semantic model

[`Zone`](https://github.com/querygraph/rgbdns/blob/master/src/zone.rs) is more than a parser for tinydns text. It is the
semantic index used by authoritative answers:

```rust
pub struct Zone {
    records: BTreeMap<Name, Vec<Record>>,
    metadata: BTreeMap<Name, Vec<RecordMetadata>>,
    authoritative: BTreeSet<Name>,
    delegations: BTreeSet<Name>,
    locations: Vec<(Vec<u8>, [u8; 2])>,
    current_metadata: RecordMetadata,
    default_serial: u32,
    nodes: BTreeSet<Name>,
    unqualified_nodes: BTreeSet<Name>,
}
```

The maps hold records and djbdns location metadata. The sets encode facts that
would otherwise require scans: zone apexes, delegation cuts, all existing
nodes, and nodes that exist independently of location-qualified records. This
all-node index is why the optimized NXDOMAIN benchmark is about eleven times
faster than the earlier scan-based implementation.

The type also prevents semantic drift. Parsing validates CNAME exclusivity
once. Lookup returns a `Lookup` enum, so absence cannot collapse into one null
result:

- `Answer` carries the matching RRset.
- `Referral` carries delegation NS records and in-bailiwick glue.
- `NoData` says the name exists but the requested type does not.
- `NxDomain` says the name itself does not exist.
- `Refused` says the server is not authoritative for the question.

Wildcard processing uses the `nodes` index to find the closest encloser.
Delegation processing uses the explicit `delegations` set. Transfer processing
walks the same model while excluding child-zone contents. One representation
therefore supplies ordinary answers, negative answers, wildcards, referrals,
and AXFR without five subtly different interpretations of a zone.

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
