---
type: "chapter"
source_file: "docs/book/rgbdns.md"
---

# Authority: answering from owned facts

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

## tinydns data as a source language

djbdns uses a compact line-oriented zone source called `data`. The first
character selects a record form. Common forms include:

| Prefix | Meaning |
|---|---|
| `.` | zone authority plus NS and address data |
| `&` | delegation NS and optional glue |
| `Z` | explicit SOA |
| `=` | A plus matching reverse PTR |
| `+` | A only |
| `6` | AAAA plus reverse PTR forms |
| `3` | AAAA only |
| `@` | MX and optional exchanger address |
| `C` | CNAME |
| `A` | private ANAME (flattened A/AAAA alias) |
| `^` | PTR |
| `'` | TXT |
| `S` | SRV |
| `:` | generic record |
| `%` | client-location mapping |

Fields are colon-separated with octal escapes for bytes that would otherwise
be ambiguous. Optional fields carry TTL, timestamp, and location information.
The format is terse because it was designed for mechanical generation as well
as hand editing.

`Zone::parse` reads this language line by line. It ignores blank, comment, and
disabled lines; reports the failing line number; expands convenience forms
into ordinary typed records; validates IPv4, flat 32-digit IPv6, names, numeric
ranges, and escaped bytes; and records authoritative and delegation structure.
When an SOA serial is omitted, file loading derives a nonzero default from the
source modification time.

Timestamp fields use TAI64-style cutoffs. Depending on the marker, a record can
be visible before or after a specified instant. Location codes select records
using configured client IPv4 prefixes. rgbdns carries that metadata beside the
record and evaluates it at lookup time.

## ANAME and apex address flattening

A zone apex necessarily owns SOA and NS data. A CNAME owner, by contrast,
cannot also own ordinary records. A literal apex CNAME would therefore make
the zone internally contradictory: the alias rule says the owner has no other
data while the authority rules require other data at that same owner.

Hosted sites still need a way for `example.com` to track addresses controlled
by a platform such as `customer.blog-host.example`. DNS providers commonly
call the solution CNAME flattening, ALIAS, or ANAME. The authoritative server
resolves the configured target itself and publishes the resulting addresses
under the configured owner.

rgbdns calls this feature **ANAME** and uses the private `A` source marker:

```text
# Authority and nameserver address.
.example.com:192.0.2.53:ns1.example.com

# ANAME owner:target:maximum-ttl
Aexample.com:customer.blog-host.example:300

# Other apex data remains independent.
@example.com::mail.example.com:10:3600
'example.com:v=spf1 -all:3600
```

The form is:

```text
Aowner:target:maximum-ttl
```

The TTL field is optional and defaults to 300 seconds. It is a ceiling, not a
promise to extend the target’s lifetime. If the upstream address has 45
seconds remaining and the ANAME limit is 300, rgbdns returns 45. If the
upstream has 900 seconds remaining, rgbdns returns no more than 300.

ANAME is stored separately from ordinary `Record` values. It can coexist with
SOA, NS, MX, TXT, CAA, and other non-address data. Zone validation rejects:

- A, AAAA, or CNAME data at the same owner;
- a wildcard ANAME owner;
- an owner that targets itself;
- different ANAME targets at one owner;
- a zero TTL.

The server only applies ANAME to A and AAAA questions. SOA, NS, MX, TXT, CAA,
and all other questions continue through normal authoritative lookup. ANAME
also does not override a delegation cut: a name beneath a delegated child
still produces a referral from the parent.

For an address question, the response path is:

1. establish that ordinary authoritative lookup reaches the ANAME owner and
   does not cross a delegation;
2. query the configured recursive resolver for the target and requested
   address family;
3. validate response identity and framing;
4. follow only a connected CNAME chain beginning at the configured target;
5. collect the terminal A or AAAA RRset;
6. replace each terminal owner with the ANAME owner;
7. cap the remaining TTL and return an authoritative answer.

For example, an upstream result such as:

```text
customer.blog-host.example. 180 IN CNAME edge.host.example.
edge.host.example.          120 IN A     192.0.2.80
```

becomes:

```text
example.com.                120 IN A     192.0.2.80
```

The CNAME is deliberately absent. Consumers see a conventional authoritative
address RRset at the apex.

The resolver cache is shared by requests handled by one server process.
Positive entries expire with the upstream chain’s shortest relevant TTL.
Negative results use the authority SOA’s negative TTL when available and 60
seconds otherwise. The configured ANAME ceiling is applied when constructing
each response, so two owners may safely share a target while using different
TTL policies.

Resolution is bounded in the same spirit as the rest of rgbdns:

- CNAME chains stop after 16 links;
- visited names detect cycles;
- no more than 64 terminal addresses are accepted;
- conflicting CNAME targets are rejected;
- upstream SERVFAIL and other resolver errors become authoritative SERVFAIL,
  not false NODATA;
- A and AAAA are cached independently.

`DNSCACHEIP` selects one or more recursive endpoints, separated by commas.
Each endpoint may be an IP address using port 53 or an explicit socket address.
Without it, rgbdns reads `/etc/resolv.conf`. A local validating `dnscache` is
the preferred upstream when operators want DNSSEC validation and a cache shared
with other local DNS work:

```sh
DNSCACHEIP=127.0.0.1:5354 IP=0.0.0.0 PORT=53 tinydns
```

ANAME metadata survives `tinydns-data` compilation through private CDB entries;
it is not encoded as a made-up public RR type. This retains the source
semantics across text and CDB operation without teaching ordinary DNS clients
about a private wire format.

Standard AXFR has no interoperable way to describe this private policy.
rgbdns therefore does not emit ANAME metadata in AXFR. An independently
configured secondary needs the same ANAME source directive, while a
conventional secondary can only serve address snapshots supplied through an
external materialization workflow. Operators should account for that
difference before treating ANAME zones as ordinary transferable zones.

## CDB: compile once, read predictably

The traditional `tinydns-data` compiles text into a constant database, CDB.
The serving process reads the compiled file instead of reparsing editable text
for every startup or query. Compilation also enables atomic replacement:
write and validate a new file, then rename it into place.

rgbdns’s `src/cdb.rs` preserves the djbdns key/value layout for ordinary
records and uses a private, NUL-prefixed key namespace for ANAME metadata.
`compile` serializes typed records and metadata; `load` reads entries and
reconstructs a `Zone`.
The loader does not trust the database merely because it is local. It bounds
file and entry sizes, validates keys, checks record layouts, decodes names and
RDATA through explicit lengths, and rejects malformed data.

This is an important general rule: compiled configuration is still input. It
may be truncated by a failed copy, generated by an older tool, or replaced by
an attacker with filesystem access. Memory safety should not depend on the
provenance story being perfect.

## Generated Code Fragment Index

These generated links open the collocated rgbdns codebase notes.

```rgbdns-fragment
{"id": "rgbdns-frag-e5fff4b8cb2b", "codeNote": "DNS from First Principles/Code/src/cdb.rs.source", "heading": "rgbdns-frag-e5fff4b8cb2b: const HEADER_LEN", "sourcePath": "src/cdb.rs", "startLine": 9, "endLine": 9}
```

```rgbdns-fragment
{"id": "rgbdns-frag-4907bb687e44", "codeNote": "DNS from First Principles/Code/src/cdb.rs.source", "heading": "rgbdns-frag-4907bb687e44: const MAX_DATABASE_SIZE", "sourcePath": "src/cdb.rs", "startLine": 10, "endLine": 11}
```

```rgbdns-fragment
{"id": "rgbdns-frag-f318af8bdeaa", "codeNote": "DNS from First Principles/Code/src/cdb.rs.source", "heading": "rgbdns-frag-f318af8bdeaa: fn compile", "sourcePath": "src/cdb.rs", "startLine": 12, "endLine": 61}
```

```rgbdns-fragment
{"id": "rgbdns-frag-f67eebb3c015", "codeNote": "DNS from First Principles/Code/src/cdb.rs.source", "heading": "rgbdns-frag-f67eebb3c015: fn load", "sourcePath": "src/cdb.rs", "startLine": 62, "endLine": 101}
```

```rgbdns-fragment
{"id": "rgbdns-frag-916ec1cbc28e", "codeNote": "DNS from First Principles/Code/src/cdb.rs.source", "heading": "rgbdns-frag-916ec1cbc28e: fn read_entries", "sourcePath": "src/cdb.rs", "startLine": 102, "endLine": 156}
```

```rgbdns-fragment
{"id": "rgbdns-frag-f9047bc1a1a2", "codeNote": "DNS from First Principles/Code/src/cdb.rs.source", "heading": "rgbdns-frag-f9047bc1a1a2: fn decode_record", "sourcePath": "src/cdb.rs", "startLine": 157, "endLine": 210}
```

```rgbdns-fragment
{"id": "rgbdns-frag-bf06479bb119", "codeNote": "DNS from First Principles/Code/src/cdb.rs.source", "heading": "rgbdns-frag-bf06479bb119: fn encode_rdata", "sourcePath": "src/cdb.rs", "startLine": 211, "endLine": 269}
```

```rgbdns-fragment
{"id": "rgbdns-frag-f2d13363a376", "codeNote": "DNS from First Principles/Code/src/cdb.rs.source", "heading": "rgbdns-frag-f2d13363a376: fn decode_name", "sourcePath": "src/cdb.rs", "startLine": 270, "endLine": 296}
```

```rgbdns-fragment
{"id": "rgbdns-frag-ca380a5004ce", "codeNote": "DNS from First Principles/Code/src/cdb.rs.source", "heading": "rgbdns-frag-ca380a5004ce: fn le_u32", "sourcePath": "src/cdb.rs", "startLine": 297, "endLine": 301}
```

```rgbdns-fragment
{"id": "rgbdns-frag-9cc58af3bb02", "codeNote": "DNS from First Principles/Code/src/cdb.rs.source", "heading": "rgbdns-frag-9cc58af3bb02: mod tests", "sourcePath": "src/cdb.rs", "startLine": 302, "endLine": 307}
```

```rgbdns-fragment
{"id": "rgbdns-frag-d600fc524f2b", "codeNote": "DNS from First Principles/Code/src/cdb.rs.source", "heading": "rgbdns-frag-d600fc524f2b: fn exact_cdb_roundtrip_preserves_lookup_semantics", "sourcePath": "src/cdb.rs", "startLine": 308, "endLine": 364}
```

```rgbdns-fragment
{"id": "rgbdns-frag-428b1ce3e4be", "codeNote": "DNS from First Principles/Code/src/cdb.rs.source", "heading": "rgbdns-frag-428b1ce3e4be: fn rejects_truncated_database", "sourcePath": "src/cdb.rs", "startLine": 365, "endLine": 371}
```
