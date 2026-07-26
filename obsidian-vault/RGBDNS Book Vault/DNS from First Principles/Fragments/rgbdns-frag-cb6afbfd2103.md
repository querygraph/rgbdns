---
type: "code-fragment"
fragment_id: "rgbdns-frag-cb6afbfd2103"
source_path: "docs/book/rgbdns.md"
code_note: "DNS from First Principles/Code/docs/book/rgbdns.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "Zone data as an indexed semantic model"
kind: "heading"
start_line: 1591
end_line: 1631
---

# Zone data as an indexed semantic model

- Fragment ID: `rgbdns-frag-cb6afbfd2103`
- Source file: [[DNS from First Principles/Code/docs/book/rgbdns.md.source|docs/book/rgbdns.md]]
- Lines: 1591-1631
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-cb6afbfd2103", "codeNote": "DNS from First Principles/Code/docs/book/rgbdns.md.source", "heading": "rgbdns-frag-cb6afbfd2103: heading Zone data as an indexed semantic model", "sourcePath": "docs/book/rgbdns.md", "startLine": 1591, "endLine": 1631}
```

## Excerpt

<span id="rgbdns-frag-cb6afbfd2103" class="rgbdns-fragment-target"></span>
### rgbdns-frag-cb6afbfd2103: heading Zone data as an indexed semantic model

```markdown
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

```
