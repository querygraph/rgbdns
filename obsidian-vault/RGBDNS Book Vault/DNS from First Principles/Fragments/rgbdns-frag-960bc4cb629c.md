---
type: "code-fragment"
fragment_id: "rgbdns-frag-960bc4cb629c"
source_path: "docs/book/rgbdns.md"
code_note: "DNS from First Principles/Code/docs/book/rgbdns.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "Finding the closest relevant authority"
kind: "heading"
start_line: 352
end_line: 387
---

# Finding the closest relevant authority

- Fragment ID: `rgbdns-frag-960bc4cb629c`
- Source file: [[DNS from First Principles/Code/docs/book/rgbdns.md.source|docs/book/rgbdns.md]]
- Lines: 352-387
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-960bc4cb629c", "codeNote": "DNS from First Principles/Code/docs/book/rgbdns.md.source", "heading": "rgbdns-frag-960bc4cb629c: heading Finding the closest relevant authority", "sourcePath": "docs/book/rgbdns.md", "startLine": 352, "endLine": 387}
```

## Excerpt

<span id="rgbdns-frag-960bc4cb629c" class="rgbdns-fragment-target"></span>
### rgbdns-frag-960bc4cb629c: heading Finding the closest relevant authority

```markdown
## Finding the closest relevant authority

For a question `(name, type)`, an authoritative server determines:

1. whether the name lies in a served zone;
2. whether a delegation cut is closer than that zone’s apex;
3. whether the exact name exists;
4. whether the requested RRset exists;
5. whether a CNAME or wildcard changes the answer;
6. which SOA proves a negative result.

A query beneath a delegated child should produce a referral, not an
authoritative negative answer from the parent. A query outside all configured
zones should normally be refused. These boundary checks matter more than a
simple map lookup.

rgbdns’s `Zone` stores records in a `BTreeMap<Name, Vec<Record>>`, authoritative
apices and delegation owners in ordered sets, and separate metadata for
location and activation. Lookup walks name ancestry, recognizes cuts, filters
visible records, applies exact-name and wildcard rules, and returns the typed
`Lookup` outcome.

The response builder then:

- copies the query ID and relevant RD bit;
- marks authoritative answers with AA;
- expands CNAME chains with a 16-hop limit and visited-name set;
- adds address records for NS, MX, and SRV targets;
- clears AA on referrals;
- adds the SOA to negative answers;
- maps malformed, unsupported, and policy cases to protocol response codes.

The finite CNAME bound and visited set are deliberate denial-of-service and
correctness controls. A cyclic zone must not turn one datagram into unbounded
work.

```
