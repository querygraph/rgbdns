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

These generated links open the collocated rgbdns codebase notes.

```rgbdns-fragment
{"id": "rgbdns-frag-5a01ae2d3ddb", "codeNote": "DNS from First Principles/Code/README.md.source", "heading": "rgbdns-frag-5a01ae2d3ddb: heading rgbdns", "sourcePath": "README.md", "startLine": 1, "endLine": 56}
```

```rgbdns-fragment
{"id": "rgbdns-frag-c24b9da16705", "codeNote": "DNS from First Principles/Code/README.md.source", "heading": "rgbdns-frag-c24b9da16705: heading Debian and systemd", "sourcePath": "README.md", "startLine": 57, "endLine": 74}
```

```rgbdns-fragment
{"id": "rgbdns-frag-003170c20cd5", "codeNote": "DNS from First Principles/Code/README.md.source", "heading": "rgbdns-frag-003170c20cd5: heading Book", "sourcePath": "README.md", "startLine": 75, "endLine": 94}
```

```rgbdns-fragment
{"id": "rgbdns-frag-851308e1cfeb", "codeNote": "DNS from First Principles/Code/README.md.source", "heading": "rgbdns-frag-851308e1cfeb: heading Conformance and performance", "sourcePath": "README.md", "startLine": 95, "endLine": 106}
```

```rgbdns-fragment
{"id": "rgbdns-frag-f52ccf723277", "codeNote": "DNS from First Principles/Code/src/lib.rs.source", "heading": "rgbdns-frag-f52ccf723277: mod aname", "sourcePath": "src/lib.rs", "startLine": 3, "endLine": 3}
```

```rgbdns-fragment
{"id": "rgbdns-frag-4a817a7124e1", "codeNote": "DNS from First Principles/Code/src/lib.rs.source", "heading": "rgbdns-frag-4a817a7124e1: mod axfr", "sourcePath": "src/lib.rs", "startLine": 4, "endLine": 4}
```

```rgbdns-fragment
{"id": "rgbdns-frag-ef82c203a6e1", "codeNote": "DNS from First Principles/Code/src/lib.rs.source", "heading": "rgbdns-frag-ef82c203a6e1: mod cdb", "sourcePath": "src/lib.rs", "startLine": 5, "endLine": 5}
```

```rgbdns-fragment
{"id": "rgbdns-frag-ac93886065b4", "codeNote": "DNS from First Principles/Code/src/lib.rs.source", "heading": "rgbdns-frag-ac93886065b4: mod client", "sourcePath": "src/lib.rs", "startLine": 6, "endLine": 6}
```

```rgbdns-fragment
{"id": "rgbdns-frag-558fa31b05c5", "codeNote": "DNS from First Principles/Code/src/lib.rs.source", "heading": "rgbdns-frag-558fa31b05c5: mod conf", "sourcePath": "src/lib.rs", "startLine": 7, "endLine": 7}
```

```rgbdns-fragment
{"id": "rgbdns-frag-fa79f1453710", "codeNote": "DNS from First Principles/Code/src/lib.rs.source", "heading": "rgbdns-frag-fa79f1453710: mod dnscache_config", "sourcePath": "src/lib.rs", "startLine": 8, "endLine": 8}
```

```rgbdns-fragment
{"id": "rgbdns-frag-060fb35dda55", "codeNote": "DNS from First Principles/Code/src/lib.rs.source", "heading": "rgbdns-frag-060fb35dda55: mod multilog", "sourcePath": "src/lib.rs", "startLine": 9, "endLine": 9}
```

```rgbdns-fragment
{"id": "rgbdns-frag-9b91c16392f6", "codeNote": "DNS from First Principles/Code/src/lib.rs.source", "heading": "rgbdns-frag-9b91c16392f6: mod name", "sourcePath": "src/lib.rs", "startLine": 10, "endLine": 10}
```
