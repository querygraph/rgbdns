# Preface {-}

DNS is often introduced as “the Internet’s phone book.” That metaphor is
useful for about a minute. A phone book is one database, published in editions,
mapping people to telephone numbers. DNS is a distributed protocol for finding
typed, time-limited statements in a delegated tree. It has many writers, many
readers, caches between them, multiple transports, and rules for proving both
presence and absence. Names can point to addresses, but they can also identify
mail exchangers, service endpoints, authoritative servers, cryptographic keys,
and arbitrary text.

This book develops DNS from those underlying problems. The first half builds a
mental model independent of any implementation. The second half walks through
rgbdns, a memory-safe Rust reimplementation of the djbdns suite. The aim is not
only to explain what each program does, but why its boundaries look the way
they do: immutable compiled data for authority, a separate recursive cache,
small diagnostic clients, foreground daemons, and stream-oriented logging.

The code is the final authority for rgbdns behavior. This book describes
version 0.1.0 as built on 2026-07-23.

# The problem DNS solves

## Identity is not location

A network delivers packets to addresses. Humans and applications want stable
identities. Those two things should not be fused.

Suppose a service is reached at `192.0.2.8`. If that address is embedded in
every configuration, moving the service requires changing every client. A name
such as `api.example` introduces indirection:

```text
application → api.example → 192.0.2.8 → packets
```

Indirection has a cost: another system must answer the middle question. Its
benefit is that the service owner can change the answer without changing the
application. DNS is the globally deployed mechanism for this indirection.

The mapping is not a function from one name to one address. One name may have
several addresses. The answers may differ by client location. A mail domain
may name several mail exchangers with preferences. A service may delegate a
subtree to another organization. The useful abstraction is therefore:

```text
(owner name, record type, class) → a set of resource records
```

The owner and type together select an RRset. “RRset” means all resource records
with the same owner, type, and class. Implementations should normally treat the
set as a unit because caches and DNSSEC signatures do.

## Requirements that pull in different directions

A global naming system must satisfy conflicting demands:

- It must scale without one central database receiving every query.
- Different organizations must control different parts of the namespace.
- Changes must propagate, but cached answers are essential for performance.
- Replies should usually fit in one datagram, but some answers are large.
- Old implementations must coexist with protocol extensions.
- A client needs to distinguish “no such name” from “that name has no record
  of this type.”
- Operators need a way to transfer complete zones and to diagnose individual
  exchanges.

DNS answers these demands with hierarchy, delegation, caching lifetimes,
compact binary messages, UDP plus TCP, explicit result codes, and typed
records. Many operational surprises are direct consequences of those design
choices rather than random quirks.

## Roles, not just “DNS servers”

The phrase “DNS server” hides several jobs.

An **authoritative server** publishes data for zones it controls. It answers
from configured facts and does not chase referrals on behalf of a client.

A **recursive resolver** accepts a question from a stub client, follows the
delegation chain, validates and caches what it learns, and returns a final
answer.

A **stub resolver** is the client-side library or program that sends a
recursive query to a configured resolver.

A **forwarder** sends selected questions to another resolver rather than
performing iteration itself.

Keeping these roles distinct is both conceptual hygiene and a security
boundary. An authoritative daemon does not need a large mutable Internet-fed
cache. A recursive resolver does not need the private machinery used to edit a
zone. rgbdns follows the djbdns design and runs authority and recursion as
different programs.

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

# Resource records: typed facts with lifetimes

## The common envelope

Every resource record has:

- an owner name;
- a numeric type;
- a class, almost always Internet class `IN`;
- a time to live, or TTL;
- type-specific data called RDATA.

The TTL is a lease offered to caches. If an authoritative server returns a TTL
of 300 seconds, a cache may reuse that answer for at most five minutes before
refreshing it. The TTL does not schedule a change and does not guarantee that
every cache holds the answer for the full interval. It establishes an upper
bound.

Changing a record and then lowering its TTL is too late for clients that
already cached the older, longer lease. Planned migrations lower the TTL at
least one old-TTL interval before the change, wait, make the change, and later
raise it.

## Core types

**A** maps an owner to an IPv4 address. **AAAA** maps it to an IPv6 address.
Several records at one owner form an address RRset; DNS does not promise that
clients use them in listed order.

**NS** names an authoritative server for a zone or delegation.

**SOA**, the start of authority, identifies the zone and carries operational
parameters: primary server, responsible mailbox, serial, refresh, retry,
expire, and negative-cache values. A secondary compares serial numbers to
decide whether a transfer is needed. Serial arithmetic wraps in a defined
32-bit space, so blindly treating it as an ordinary integer can be wrong near
the boundary.

**CNAME** says that its owner is an alias of another name. Except for DNSSEC
and narrowly specified metadata, an owner with CNAME should not also hold
unrelated data. A resolver follows the chain while defending against loops and
excessive depth.

**MX** names a mail exchanger and gives it a preference. Lower numbers are
preferred. The target is a name, not an address.

**PTR** provides a name-valued reverse mapping. IPv4 reverse names live below
`in-addr.arpa.` with octets reversed. IPv6 reverse names live below
`ip6.arpa.` with hexadecimal nibbles reversed.

**TXT** carries one or more length-delimited byte strings. Presentation formats
often make it look like one free-form string, but the wire format retains
segments.

**SRV** names a service endpoint with priority, weight, port, and target.

**CAA** constrains which certification authorities may issue certificates for
a domain.

**OPT** is not ordinary zone data. It is a pseudo-record used by EDNS to
negotiate UDP payload size and carry extension flags and options.

rgbdns models these forms with `RecordType`, `Record`, and the `RData` enum in
`src/packet.rs`. Known structured types receive structured variants. Unknown
types can remain opaque where the format permits, preserving extensibility
without confusing untrusted lengths with trusted objects.

## Additional data is an optimization

If an answer contains MX, NS, or SRV targets, the server may include associated
A and AAAA records in the additional section. This can save queries. It does
not change which RRset directly answers the question, and a resolver must apply
the correct credibility rules rather than trusting unrelated additional data.

The rgbdns authoritative response path collects target names from those record
types and adds locally available addresses. It de-duplicates targets before
lookup and preserves the distinction between answers and helpful additionals.

# Messages on the wire

## The twelve-byte header

A DNS message begins with a fixed twelve-byte header:

```text
0                   1                   2                   3
0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
+-------------------------------+-------------------------------+
|              ID               |            flags              |
+-------------------------------+-------------------------------+
|          question count       |          answer count         |
+-------------------------------+-------------------------------+
|         authority count       |         additional count      |
+-------------------------------+-------------------------------+
```

The transaction ID lets a client associate a response with a query. Important
flags include QR (query versus response), opcode, AA (authoritative answer), TC
(truncated), RD (recursion desired), RA (recursion available), and the
four-bit response code.

The four following sections contain questions, answers, authority records, and
additional records. A normal question carries a name, requested type, and
class. Resource-record sections add TTL, RDATA length, and RDATA.

All multibyte integers are network byte order. Every count and length comes
from an untrusted peer. A decoder must prove that bytes exist before reading
them, cap allocations, reject invalid labels and pointers, and finish with a
coherent message rather than a partially trusted structure.

## Name compression

Repeating full names would waste scarce datagram space. DNS permits a name
suffix to be replaced by a two-byte pointer whose high bits are `11` and whose
remaining bits are an offset earlier in the message.

Compression turns name decoding into graph traversal. A malicious packet can
contain a pointer loop, excessive indirection, or an offset outside the packet.
A safe decoder tracks visited offsets or imposes a strict jump bound, checks
every target, and separately enforces the expanded 255-octet name limit.

rgbdns’s `Reader` in `src/packet.rs` keeps all reads within a borrowed byte
slice. Name decoding validates pointer targets and bounds traversal. Record
decoding confines each RDATA parser to the declared RDLENGTH. EDNS option
iteration likewise checks the option header and value before advancing.

The `Writer` performs the reverse operation. Encoding is fallible: counts must
fit 16 bits, names and RDATA must fit their fields, and the result must remain
valid. This symmetry—decode into typed data, manipulate typed data, encode with
checks—is the packet layer’s central safety property.

## Errors are protocol results

Several results that sound similar are materially different:

- **NOERROR with answers**: the requested RRset exists.
- **NOERROR without answers**, often called NODATA: the name exists but the
  requested type does not.
- **NXDOMAIN**: the queried name does not exist.
- **SERVFAIL**: the server could not safely complete processing.
- **REFUSED**: policy forbids the operation.
- **FORMERR**: the message is malformed.
- **NOTIMP**: the requested opcode is unsupported.

Negative answers normally include the zone’s SOA so resolvers can cache the
negative result. Confusing NODATA with NXDOMAIN can suppress other valid types
at the same name.

rgbdns expresses authoritative lookup outcomes as `Lookup::Answer`,
`Referral`, `NoData`, `NxDomain`, and `Refused`. That internal sum type forces
the response builder to handle each protocol meaning explicitly.

# UDP, TCP, EDNS, and truncation

## Why there are two transports

Classic DNS uses UDP for ordinary queries because one request and one response
need no connection setup. Traditional UDP DNS assumed a 512-byte message.
Larger answers set TC, telling the client to retry over TCP. TCP frames every
DNS message with a two-byte length.

Zone transfers use TCP. Modern responses—especially DNSSEC responses—often
need more than 512 bytes, so EDNS lets a client advertise a larger UDP receive
size through an OPT pseudo-record. Internet paths can still drop fragmented
UDP packets. A commonly conservative payload is 1232 bytes, large enough for
useful DNSSEC answers while fitting the IPv6 minimum MTU without fragmentation
under normal headers.

TCP is not merely an emergency protocol. Firewalls that assume DNS is
UDP-only break standards-compliant resolution.

## Truncation must preserve a valid message

Cutting the last bytes off an encoded message creates a malformed packet.
Correct truncation removes complete records, sets TC, updates section counts,
and re-encodes. A useful removal order discards nonessential additional data
before authority and answer data. OPT sometimes needs special treatment
because it carries the EDNS response.

`src/server.rs` calculates a response limit from the caller’s transport limit
and the client’s EDNS advertisement. It caps advertised UDP size, rejects
multiple OPT records, responds to unsupported EDNS versions, and constructs a
full typed response. If encoding exceeds the limit, `truncate` sets TC and
removes complete records in a defined order until the packet fits. The same
core response logic serves UDP and TCP without treating TCP as a giant UDP
datagram.

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

## CDB: compile once, read predictably

The traditional `tinydns-data` compiles text into a constant database, CDB.
The serving process reads the compiled file instead of reparsing editable text
for every startup or query. Compilation also enables atomic replacement:
write and validate a new file, then rename it into place.

rgbdns’s `src/cdb.rs` preserves the djbdns key/value layout. `compile` serializes
typed records and metadata; `load` reads entries and reconstructs a `Zone`.
The loader does not trust the database merely because it is local. It bounds
file and entry sizes, validates keys, checks record layouts, decodes names and
RDATA through explicit lengths, and rejects malformed data.

This is an important general rule: compiled configuration is still input. It
may be truncated by a failed copy, generated by an older tool, or replaced by
an attacker with filesystem access. Memory safety should not depend on the
provenance story being perfect.

# Recursion: discovering an answer

## Iteration from the root

A recursive resolver turns one client request into a bounded sequence of
queries. For `www.example.com. A`, a cold lookup is approximately:

```text
stub → recursive resolver
          ├─ root:       who serves com?
          ├─ com server: who serves example.com?
          └─ example:    what is www.example.com A?
     ← final answer
```

The resolver follows referrals, resolves nameserver addresses when glue is
insufficient, handles aliases, retries servers and transports, and detects
loops. It caches useful RRsets so later clients may skip most of this path.

Root hints are not answers to every name. They are bootstrap addresses for
reaching the root authority. They need periodic maintenance because the set
can change, though names and anycast make changes infrequent.

## The cache is part of correctness

A cache key includes at least name, type, and class. A cached positive RRset
expires according to TTL. Negative results also have bounded lifetimes derived
from SOA data. Delegation and nameserver-address caches help the iterative
algorithm navigate efficiently.

Capacity is as important as time. An attacker can generate endless distinct
names. An unbounded cache converts traffic into memory exhaustion. A practical
resolver bounds response cache bytes, nameserver cache entries, recursion
depth, referral work, packet sizes, concurrent operations, and timeouts.

`src/bin/dnscache.rs` uses Hickory’s recursive zone handler inside rgbdns’s
process and policy shell. It configures:

- randomized query-name letter case;
- a bounded response cache, defaulting to 16 MiB;
- a bounded nameserver cache;
- bounded ordinary and nameserver recursion depth;
- a 1232-byte EDNS payload;
- UDP and TCP listeners;
- loopback-only clients by default, expanded through `ALLOW_NETS`.

Configuration values are parsed with explicit minimums and maximums. A typo
such as an enormous cache size fails startup rather than silently allocating an
operator’s mistake.

## Forward zones and djbdns roots

Private namespaces and split DNS often need selected suffixes sent to specific
servers. rgbdns reads forward-zone configuration from the environment and the
djbdns-style `ROOT/servers` directory. The filename identifies a suffix and
the file lists bounded server addresses.

The special `servers/@` file represents root servers. Hickory consumes a root
hints file in master-file syntax, so `PreparedRoots` translates djbdns’s plain
address list into a private temporary file. Creation uses restrictive
permissions and cleanup occurs when the prepared object is dropped. This
adapter preserves the external configuration contract without weakening the
library boundary.

Forwarded private zones disable strict case-randomization response matching
because legacy authorities may canonicalize owner case. They retain TCP retry
and a bounded cache. This is a scoped compatibility decision, not a global
removal of query hardening.

# DNSSEC: authenticating the chain

## What ordinary DNS cannot prove

Transaction IDs, source ports, and query-case randomization make blind
spoofing harder, but they do not cryptographically establish who published an
RRset. DNSSEC adds signatures and a chain of trust.

A zone signs RRsets with private keys and publishes DNSKEY records. A parent
publishes a DS digest that identifies a child key. Starting from a configured
root trust anchor, a validating resolver can authenticate the root DNSKEY,
then a top-level domain’s DS and DNSKEY, and so on to the answer.

RRSIG authenticates an RRset over a validity interval. DS links parent to
child. NSEC or NSEC3 authenticates nonexistence by proving gaps in the ordered
namespace. DNSSEC provides origin authentication and integrity; it does not
encrypt queries or hide names.

Validation outcomes matter:

- **secure**: a valid chain reaches the answer;
- **insecure**: the chain proves that the child is unsigned;
- **bogus**: signatures or proofs fail;
- **indeterminate**: validation cannot be completed safely.

A resolver must not turn bogus data into a normal answer merely to improve
availability. Clock correctness also becomes a dependency because signatures
have inception and expiration times.

## rgbdns validation policy

rgbdns configures the recursive handler with a static root trust anchor and
DNSSEC validation enabled. Validation and NSEC3 work receive bounded caches and
iteration policies. A failed validation surfaces as resolution failure rather
than an unchecked answer.

The authoritative rgbdns data path focuses on the djbdns record surface; the
recursive path is where DNSSEC validation is currently integrated. This is an
example of honest component boundaries: “the suite supports validating
recursion” does not imply that every authoritative signing workflow has been
recreated.

# Zone transfer and secondary service

## AXFR is a stream, not a giant datagram

AXFR transfers a complete zone over TCP. A successful stream begins with the
zone’s SOA, contains the zone records, and ends with the SOA again. The records
may span many DNS messages. A client must continue until it sees the closing
SOA under the transfer rules; reading one response is insufficient.

Transfers reveal the zone contents and can consume resources, so authorities
normally restrict clients. TSIG is a common authentication mechanism in the
wider ecosystem, while IP allowlists are a simpler policy with weaker identity
properties.

`src/axfr.rs` provides both sides. `axfrdns` accepts TCP only and checks client
networks, loopback by default. It requires one AXFR question, obtains a
boundary-aware transfer from `Zone`, and frames bounded messages. `Zone::transfer`
excludes records beneath delegated child zones and wraps the result in the
apex SOA.

`axfr-get` generates a random transaction ID, validates response identity and
shape, collects records until the closing SOA, renders them in tinydns source
form, writes a temporary output, and atomically installs the completed file.
The temporary/final path pair prevents a failed transfer from replacing usable
data with a partial zone.

# The rgbdns program family

## One suite, small purposes

rgbdns deliberately exposes separate commands:

| Command family | Purpose |
|---|---|
| `tinydns`, `tinydns-data`, `tinydns-get`, `tinydns-edit` | authoritative service and data maintenance |
| `dnscache` | validating recursive resolver and cache |
| `axfrdns`, `axfr-get` | zone transfer server and client |
| `rbldns`, `rbldns-data` | address-prefix blocklist DNS |
| `pickdns`, `pickdns-data` | location-aware address selection |
| `walldns` | synthetic address/reverse answers |
| `dnsq`, `dnsqr`, `dnsip*`, `dnsname`, `dnsmx`, `dnstxt` | queries and diagnostics |
| `dnsfilter`, `dnstrace`, `random-ip` | stream lookup, delegation tracing, testing |
| `*-conf` | service-directory generation |
| `setuidgid`, `multilog`, `tai64n`, `tai64nlocal` | process and logging support |

This composition makes privilege and failure boundaries visible. A compiler
can run with write access to data while the server runs read-only. A recursive
cache can be restarted without touching authority. Diagnostic clients reuse
the packet and client libraries rather than embedding daemon behavior.

## Specialized responders

`rbldns` treats the labels before a configured suffix as a numeric address,
finds the most-specific matching IPv4 prefix in a compiled database, and
returns configured A/TXT data. Parsing caps the number of numeric labels and
validates networks before compilation.

`pickdns` maps client prefixes to two-byte locations and selects address sets
for that location. It shuffles eligible addresses with operating-system
randomness. Location-aware answers are a policy feature; clients behind shared
resolvers may appear at the resolver’s address, a limitation operators must
understand.

`walldns` synthesizes narrowly defined forward and reverse answers without a
zone database. These specialized services run through `src/special.rs`, which
provides shared bounded UDP/TCP serving and passes the peer address to the
handler.

The lesson is architectural: once parsing, transport, names, and record models
are sound, unusual DNS policies can be small pure response functions rather
than new monolithic servers.

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

# Security engineering in rgbdns

## The packet is hostile

DNS combines nearly every parser hazard: nested lengths, compression pointers,
variable counts, binary strings, recursive relationships, and network-facing
availability requirements. “Written in Rust” removes broad classes of memory
corruption, but it does not automatically prevent allocation bombs, infinite
loops, CPU amplification, path races, policy errors, or accepting incoherent
messages.

rgbdns therefore uses several layers:

- `#![forbid(unsafe_code)]` for the library;
- explicit bounds before every wire read;
- validated `Name`, `Message`, and `RData` objects;
- limits on compression traversal, aliases, records, files, configuration
  lists, recursion, transfers, and cache sizes;
- cryptographic operating-system randomness for query IDs and selection;
- complete-record truncation;
- loopback-only defaults for recursion and transfer;
- atomic replacement for compiled databases and fetched zones;
- no shell interpolation when replacing a process.

Property tests in `tests/packet_properties.rs` feed arbitrary bytes to the
decoder and exercise encode/decode invariants. Golden CDB fixtures compare
compiled output with the expected djbdns layout. Network tests cross real UDP
and TCP boundaries. Compatibility tests are valuable here because a parser can
be safe yet subtly wrong, or compatible yet unsafe.

## Least privilege and filesystem boundaries

The `*-conf` commands generate service directories whose run scripts execute
the daemon under a selected account. rgbdns’s `setuidgid` resolves the user and
group, initializes supplementary groups, drops GID and UID, verifies the
result, and directly replaces itself with the target program. Direct
replacement preserves signals and exit status and avoids an extra shell-owned
process.

Generated paths are shell-quoted and support binaries by absolute path.
Configuration writers reject unsafe existing file types and apply intentional
modes. CDB and AXFR update workflows install only complete outputs.

Privilege dropping is not a substitute for a restricted service account,
read-only data, network policy, or supervisor hardening. It is one layer in a
deployment.

# Time and logs: TAI64N

## Why a DNS suite contains time tools

Long-running daemons need logs, and djb’s tools use TAI64N labels. A label has
an `@`, sixteen hexadecimal digits of biased TAI seconds, and eight hexadecimal
digits of nanoseconds:

```text
@4000000037c219bf2ef02e94
```

TAI is a continuous atomic timescale. UTC inserts leap seconds, so converting
between a POSIX/UTC timestamp and TAI requires the applicable TAI−UTC offset.
The offset was 10 seconds at the Unix epoch convention used here and reached
37 seconds after the 2016 leap second.

`tai64n` timestamps each input line at the moment its first bytes are read.
`tai64nlocal` recognizes a valid leading label and replaces it with local civil
time at nanosecond precision. Invalid prefixes pass through unchanged.
`multilog t` uses the same label generator, ensuring standalone filters and log
files agree.

`src/tai64.rs` contains the complete 1972–2017 positive-leap transition table.
It validates the fixed-width hexadecimal form and nanosecond range. Both stream
filters use bounded memory: the localizer buffers only the 25-byte candidate
prefix and streams the rest of even an extremely long line. At I/O failure the
command exits 111, following the daemontools convention.

TAI64N provides sortable, unambiguous event labels. Converting to local time is
a presentation step, not the archival representation.

# Running rgbdns under supervision

## The service contract

rgbdns daemons run in the foreground, emit diagnostics to standard error, take
configuration from files and environment, and terminate on fatal startup
errors. That is the portable contract a supervisor needs. The generated
djbdns-style directories additionally provide `run` and `log/run` programs,
but the daemon binaries do not require a particular supervisor.

The classic daemontools control plane is:

```text
supervise service/       keep one process running
svc -u service/          bring it up
svc -d service/          bring it down
svc -t service/          send TERM
svstat service/          inspect status
```

No modern replacement is universally best. Choose according to the host and
the migration boundary.

## Recommendations

### Existing Linux host: systemd

Use systemd when the machine already boots and manages services with systemd.
It supplies dependency ordering, restart policy, socket and readiness models,
resource controls, credential and filesystem sandboxing, a unified journal,
and distribution-native administration. Avoid wrapping an rgbdns daemon in a
second nested supervisor.

A minimal authoritative unit is:

```ini
[Unit]
Description=rgbdns authoritative DNS
After=network-online.target
Wants=network-online.target

[Service]
Type=simple
User=dns
Group=dns
Environment=IP=192.0.2.53
Environment=PORT=53
Environment=DATA=/etc/rgbdns/data.cdb
ExecStart=/usr/local/bin/tinydns
Restart=on-failure
RestartSec=1s
NoNewPrivileges=yes
ProtectSystem=strict
ProtectHome=yes
PrivateTmp=yes
ReadOnlyPaths=/etc/rgbdns
AmbientCapabilities=CAP_NET_BIND_SERVICE
CapabilityBoundingSet=CAP_NET_BIND_SERVICE

[Install]
WantedBy=multi-user.target
```

Prefer a socket above 1024 or a narrowly bounded bind capability over running
the daemon as root. Test hardening settings on the target distribution because
name-service libraries and trust-anchor paths may require additional read-only
access.

Command mapping:

| daemontools | systemd |
|---|---|
| `svc -u service` | `systemctl start service` |
| `svc -d service` | `systemctl stop service` |
| `svc -t service` | `systemctl kill --signal=TERM service` |
| `svc -h service` | `systemctl kill --signal=HUP service` |
| `svstat service` | `systemctl status service` |
| `multilog` output | journal, or explicit file logging policy |

### Closest service-directory migration: runit

Use runit when you want the smallest conceptual migration from daemontools.
It uses a service directory with a `run` program, keeps the supervised process
in the foreground, has a companion log service, and exposes the compact `sv`
control command. Existing rgbdns generated `run` scripts are close to the
required shape; adjust the directory layout and enablement symlink for the
distribution.

| daemontools | runit |
|---|---|
| `svc -u service` | `sv up service` |
| `svc -d service` | `sv down service` |
| `svc -t service` | `sv term service` |
| `svstat service` | `sv status service` |

Choose runit for minimal hosts, appliances, or migrations where preserving the
service-directory model matters more than rich dependency and sandbox policy.

### Strong supervision composition: s6 and s6-rc

Use s6 when precise process supervision, reliable readiness, and composable
small tools are primary requirements. Its `s6-supervise` and `s6-svc` are close
in spirit to `supervise` and `svc`; `s6-rc` adds declared dependencies and
transactional service-state changes. The ecosystem is particularly effective
in carefully constructed containers and small systems, but its compilation
and directory conventions make migration more involved than runit.

| daemontools | s6 |
|---|---|
| `supervise service` | `s6-supervise service` |
| `svc -u service` | `s6-svc -u service` |
| `svc -d service` | `s6-svc -d service` |
| `svc -t service` | `s6-svc -t service` |
| `svstat service` | `s6-svstat service` |

Choose s6/s6-rc when the team is willing to own its service database and wants
more rigorous dependency transitions than ad hoc shell orchestration.

### OpenRC and container orchestrators

On an OpenRC-based distribution, use the native init integration unless there
is a deliberate reason to introduce another supervision tree. OpenRC service
scripts can use its supervisor support while retaining distribution-standard
boot ordering and administration.

In Kubernetes or a similar orchestrator, run one foreground rgbdns daemon per
container and let the platform own restart, health, resource limits, log
collection, and rollout. Use a Deployment for `tinydns` or `dnscache`, a
Service for stable network reachability, readiness/liveness probes that test
the intended DNS role, ConfigMaps or mounted immutable CDBs for public data,
and Secrets for sensitive material. Do not put systemd, daemontools, and the
orchestrator around the same single process.

An s6-based container is reasonable only when one image intentionally contains
several cooperating long-lived processes and that tradeoff is explicit.

## A practical selection rule

Use this order:

1. Follow the host’s native manager: systemd on systemd hosts, OpenRC on
   OpenRC hosts.
2. For a direct service-directory replacement, choose runit.
3. For a designed supervision graph or multi-process container, choose
   s6/s6-rc.
4. In an orchestrated single-process container, use the orchestrator.

The least risky migration preserves one owner for restart policy and logs.
Running two supervisors creates ambiguous signal paths, duplicate restarts,
and status commands that disagree.

# Operating an authoritative service

## Build, stage, verify, replace

A safe publication cycle separates source editing from serving:

```sh
cd /etc/rgbdns
tinydns-data
tinydns-get example.com A www.example.com
```

In production, compile in a staging directory, run representative exact,
wildcard, delegation, negative, IPv4, IPv6, and large-response queries, then
atomically replace `data.cdb`. Retain the previous known-good database for
rollback. Query the bound service over both UDP and TCP after deployment.

Do not expose the recursive service to arbitrary networks by accident. The
default `ALLOW_NETS` is loopback only because an open resolver can be abused
for amplification and can consume local capacity. Likewise, expand AXFR
allowlists only for intended secondaries.

## Observe the right signals

Useful signals include:

- query and error rate by transport;
- truncated UDP responses and TCP retries;
- SERVFAIL, REFUSED, NXDOMAIN, and validation-failure rates;
- resolver cache capacity and latency percentiles;
- process restarts and file-descriptor use;
- root-hint and trust-anchor freshness;
- time synchronization;
- CDB build identity and deployment time.

High NXDOMAIN volume is not automatically an incident; browsers, typo traffic,
and discovery protocols generate it. A change from baseline paired with
latency or SERVFAIL is more meaningful.

TAI64N log labels make events stable for storage. Convert them for human
display at the edge:

```sh
tail -f main/current | tai64nlocal
```

# Testing DNS software

## Layers of evidence

Unit tests establish local invariants: name limits, record parsing, lookup
outcomes, leap conversion. Property tests explore parser state spaces that
examples miss. Golden fixtures establish compatibility with an external file
format. Integration tests cross process and socket boundaries. Live
interoperability tests compare behavior with independent clients and servers.

rgbdns uses all of these. A useful local sequence is:

```sh
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
```

Network tests bind unprivileged loopback ports. CDB tests compile canonical
fixtures and compare entries. Daemontools tests exercise process replacement,
rotation, and TAI64 filter behavior. Packet properties assert that arbitrary
input does not panic and that supported structured messages survive
encode/decode round trips.

## Adversarial cases worth keeping

Every DNS implementation should retain regression cases for:

- a compression pointer to itself or a pointer cycle;
- a pointer or RDATA length just beyond the packet;
- maximum-length labels and names;
- counts that cannot be satisfied by the remaining bytes;
- duplicate or malformed OPT records;
- tiny advertised transport limits;
- CNAME loops and excessive chains;
- wildcard names blocked by existing nodes;
- delegation cuts beneath an authoritative apex;
- NODATA versus NXDOMAIN;
- AXFR without a closing SOA;
- an enormous log line;
- configuration counts at and beyond each bound.

Tests should assert protocol meaning, not only that the process remains alive.
A safe FORMERR is better than a crash, but a silent NOERROR can still be a
serious bug.

# Reading the rgbdns source

## A path through the code

Read the project in dependency order:

1. `src/name.rs` — the foundational name invariant.
2. `src/packet.rs` — types and bounded wire codec.
3. `src/zone.rs` — tinydns source and authoritative lookup semantics.
4. `src/cdb.rs` — compiled compatibility format.
5. `src/server.rs` — query validation, answer construction, transport limits.
6. `src/client.rs` — stub behavior and TCP fallback.
7. `src/axfr.rs` — streaming zone movement and atomic installation.
8. `src/dnscache_config.rs` and `src/bin/dnscache.rs` — iterative recursion,
   DNSSEC, forwarding, access, and resource policy.
9. `src/rbl.rs`, `src/pick.rs`, `src/wall.rs`, and `src/special.rs` —
   specialized responders.
10. `src/conf.rs`, `src/setuidgid.rs`, `src/multilog.rs`, and `src/tai64.rs` —
    deployment and operations.

The binaries in `src/bin` should then look thin. That is intentional. They
parse the command contract, load configuration, call a library boundary, print
diagnostics, and map fatal errors to the suite’s exit convention.

## Design patterns to carry elsewhere

Several rgbdns choices generalize beyond DNS.

**Parse into valid types.** If an invalid name can circulate as an ordinary
string, every consumer must rediscover validation.

**Bound dimensions independently.** A packet byte limit does not replace a
compression-depth limit; a cache byte limit does not replace a recursion-depth
limit.

**Separate policy from mechanism.** `special.rs` owns transport while small
handlers own synthesized-answer policy.

**Compile mutable source into immutable serving data.** This gives validation,
atomic rollout, simple readers, and easy rollback.

**Preserve protocol distinctions internally.** A `Lookup` enum prevents
NXDOMAIN, NODATA, referral, and refusal from collapsing into “no records.”

**Run in the foreground.** It composes with old and new supervisors and keeps
signals understandable.

**Treat compatibility files as hostile.** Historical layout fidelity need not
mean historical trust assumptions.

# Where DNS ends

DNS establishes named, cacheable facts and, with DNSSEC, their authenticated
origin. It does not prove that the address belongs to the application a user
intended, encrypt the subsequent connection, guarantee freshness inside the
TTL window, or choose a healthy endpoint. TLS identity, application discovery,
load balancing, routing, and monitoring build on DNS but remain separate
systems.

That boundary is the best final replacement for the phone-book metaphor. DNS
is a delegated publication and discovery protocol. Its tree assigns authority;
its records carry typed statements; its TTLs make caching explicit; its packet
format makes efficient exchange possible; recursion joins many authorities
into one answer; DNSSEC authenticates the chain; and supervision keeps the
implementing processes available without becoming part of the protocol.

rgbdns expresses those ideas as small programs over shared, validated Rust
types. Understanding the protocol makes the program family unsurprising.
Reading the program family, in turn, shows how the abstract DNS model becomes
bounded packets, immutable databases, iterative queries, atomic files, and
foreground processes.

# Appendix A: Configuration quick reference

Common daemon variables include:

| Variable | Meaning |
|---|---|
| `IP` | listen address |
| `PORT` | listen port |
| `DATA` | authoritative text or CDB path where supported |
| `ALLOW_NETS` | comma-separated client CIDRs for recursion or transfer |
| `DNSCACHEIP` | recursive endpoints used by client tools |
| `CACHESIZE` | bounded recursive response-cache capacity |
| `NSCACHESIZE` | bounded nameserver-cache entries |
| `RECURSION_LIMIT` | ordinary recursion depth |
| `NS_RECURSION_LIMIT` | nameserver-resolution recursion depth |
| `ROOT` | djbdns-compatible resolver configuration root |

Use the command’s `*-conf` generator as a starting point, then adapt the
foreground `run` contract to the chosen native supervisor.

# Appendix B: Further reading

The protocol is defined across many RFCs. A productive sequence is:

- RFC 1034, *Domain Names—Concepts and Facilities*.
- RFC 1035, *Domain Names—Implementation and Specification*.
- RFC 2181, clarifications including RRset and credibility rules.
- RFC 2308, negative caching.
- RFC 6891, Extension Mechanisms for DNS (EDNS(0)).
- RFC 7766, DNS over TCP requirements.
- RFC 4033, RFC 4034, and RFC 4035, DNSSEC.
- RFC 5155, NSEC3.
- RFC 5936, AXFR.
- RFC 1982, serial number arithmetic.

Implementation and operational references:

- The djbdns documentation: <https://cr.yp.to/djbdns.html>
- TAI64N format and tools: <https://cr.yp.to/daemontools/tai64n.html>
- s6 overview: <https://skarnet.org/software/s6/overview.html>
- s6-rc overview: <https://skarnet.org/software/s6-rc/overview.html>
- runit benefits: <https://smarden.org/runit/benefits.html>
- systemd project documentation: <https://systemd.io/>
- Hickory DNS: <https://hickory-dns.org/>

