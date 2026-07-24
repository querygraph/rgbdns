---
type: "chapter"
source_file: "docs/book/rgbdns.md"
---

# Zone data as an indexed semantic model

[`Zone`](../../src/zone.rs) is more than a parser for tinydns text. It is the
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
