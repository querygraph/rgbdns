---
type: "chapter"
source_file: "docs/book/rgbdns.md"
---

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

## Two distinct DNSSEC roles

DNSSEC has a consumer side and a producer side. A validating recursive
resolver follows DS and DNSKEY records from a trust anchor and rejects bogus
answers. An authoritative operator creates keys, signs each stable RRset,
publishes proofs of nonexistence, and arranges for the parent to publish the
matching DS.

rgbdns implements both roles, but deliberately keeps them separate. `dnscache`
uses a static root trust anchor and DNSSEC validation. Validation and NSEC3 work
receive bounded caches and iteration policies. A failed validation becomes a
resolution failure rather than an unchecked answer.

Authoritative signing is an offline publication pipeline. `tinydns` never
opens a private key and never signs in response to a packet. It serves DNSKEY,
RRSIG, NSEC, and DS records already present in its CDB. This is the same
compile-then-serve division used by ordinary tinydns data, extended to signed
snapshots.

## DNSSEC in the tinydns posture

The design goal is not a control plane. It is a set of small programs and
visible files:

| Program | One bounded job |
|---|---|
| `rgbsec-keygen` | create one ECDSA P-256 key and print its policy line |
| `rgbsec-sign` | turn tinydns text into inspectable signed tinydns text |
| `rgbsec-data` | sign and compile directly to a CDB |
| `rgbsec-ds` | derive the parent DS record from one policy line |
| `rgbsec-check` | verify RRsets, signatures, NSEC cycles, and remaining lifetime |

There is no signing daemon, key database, online RPC service, or implicit key
generation in the authority. Each transformation writes beside its destination
and renames only after success. A failed materialization, signature, compile,
reload, or verification leaves the last-known-good CDB active.

The feature is optional. Without a DNSSEC policy and
`/etc/rgbdns/dnssec.env`, `tinydns-data`, `tinydns`, ACME, ANAME, AXFR, and
secondary synchronization retain the djbdns-compatible path. Enabling signing
for one zone does not force every zone in the same CDB to become signed.

## One line per zone

The policy file is intentionally close to tinydns data: comments, blank lines,
and exactly one disposition line for every authoritative zone. `K` means sign;
`U` means explicitly unsigned:

```text
Kexample.com.:/etc/rgbdns/keys/example.com.pk8:13:1209600:86400:3600
Ulegacy.example.
```

The signed line has this shape:

```text
Kzone:keyfile:algorithm:validity:refresh:inception-skew
```

Algorithm 13 is ECDSA P-256 with SHA-256. The example signs for fourteen days,
requires at least one day of remaining lifetime, and moves inception one hour
into the past to tolerate modest clock skew. The key path is absolute. Missing,
duplicate, or conflicting dispositions stop publication; omission never means
“probably unsigned.” A `U` zone is also checked for stale DNSKEY, RRSIG, NSEC,
or NSEC3 records.

This fail-closed coverage rule is what makes mixed snapshots understandable.
The policy is not merely a list of zones that happened to be signed today. It
is the security disposition of the complete authoritative source.

## Minimal signed primary

Begin with ordinary tinydns source. The signer preserves the SOA serial, so the
source producer must increment it when the signed publication first becomes
visible:

```text
Zexample.com:a.ns.example.com:hostmaster.example.com:2026082101:16384:2048:1048576:2560:3600
&example.com:192.0.2.53:a.ns.example.com:3600
+example.com:192.0.2.80:3600
+www.example.com:192.0.2.80:3600
```

Create the key only on the primary. The command refuses to overwrite an
existing file and creates the PKCS#8 key with mode 0600:

```sh
sudo install -d -o root -g root -m 0700 /etc/rgbdns/keys
sudo rgbsec-keygen example.com \
  /etc/rgbdns/keys/example.com.pk8 \
  | sudo tee /etc/rgbdns/dnssec
sudo chown root:rgbdns /etc/rgbdns/dnssec
sudo chmod 0640 /etc/rgbdns/dnssec
```

Keep an encrypted, recoverable copy of the key outside the server. The public
policy may be readable by the checker, but the key directory remains
root-only. The authority process and the secondary never need access.

Configure the packaged primary with the complete existing role options plus
the policy:

```sh
sudo rgbdns-setup primary \
  --data /srv/dns/rgbdns.data \
  --data-drop /srv/dns/rgbdns.data \
  --data-drop-owner dns-publisher \
  --listen-ip 0.0.0.0 \
  --allow-nets 10.0.2.10/32 \
  --dnssec-policy /etc/rgbdns/dnssec
```

Setup performs the first privileged publication and enables two supervised
jobs. `rgbdns-dnssec-publish.timer` refreshes materialized data and signatures
every twelve hours. `rgbdns-dnssec-check.timer` checks the active CDB hourly.
The root publisher can read the key; the checker runs as `rgbdns` and verifies
only public state.

The same stages can be inspected manually in a scratch directory:

```sh
ln -s /etc/rgbdns/dnssec dnssec
rgbsec-sign data data.signed
rgbsec-data data data.cdb
rgbsec-check data.cdb /etc/rgbdns/dnssec
rgbsec-ds 'Kexample.com.:/etc/rgbdns/keys/example.com.pk8:13:1209600:86400:3600'
```

`rgbsec-check` emits one tab-separated line per `K` zone:

```text
example.com.  2026082101  53856  20260904023527  1204490  ok
```

The exact times and key tag vary. The important operational contract is the
exit status and final `ok`: every authoritative RRset verifies, the NSEC cycle
is complete, and the earliest signature remains outside the refresh window.

## Mixed zones and a keyless secondary

One CDB can serve a stable signed zone beside a zone that must remain unsigned:

```text
Kexample.com.:/etc/rgbdns/keys/example.com.pk8:13:1209600:86400:3600
Uacme-live.example.
```

The source contains both SOAs and both NS sets. `rgbsec-data` signs only
`example.com`; it preserves ordinary records, qualified records, cutoffs, and
ANAME directives in the `U` zone. This behavior is explicit rather than
inferred from the absence of a key.

A secondary needs neither the policy nor the private key. Configure it in the
ordinary way:

```sh
sudo rgbdns-setup secondary \
  --zone example.com \
  --zone acme-live.example \
  --primary 10.0.1.10 \
  --listen-ip 0.0.0.0
```

AXFR carries DNSKEY, RRSIG, NSEC, and DS as standard resource records. The
secondary validates the transfer framing, compiles the received snapshot, and
serves the same finished signatures. A transfer failure retains the last good
zone. This is a useful security boundary: compromise of a serving secondary
does not disclose the signing key.

Before touching the parent, query every delegated authority over UDP and TCP:

```sh
for server in a.ns.example.com b.ns.example.com; do
  dig +dnssec +norecurse @"$server" example.com DNSKEY
  dig +dnssec +norecurse @"$server" example.com A
  dig +dnssec +norecurse @"$server" \
    "MiXeD-$(date +%s).example.com" A
  dig +tcp +dnssec +norecurse @"$server" \
    "tcp-$(date +%s).example.com" A
done
```

Require matching SOA serials and DNSKEYs. Positive answers need the data RRset
and its RRSIG. A nonexistent mixed-case name must return NXDOMAIN with NSEC and
RRSIG. The mixed case is deliberate: validating resolvers use DNS 0x20 case
randomization, and authoritative suffix matching must remain ASCII
case-insensitive. A query for an absent type at an existing name tests signed
NODATA separately from NXDOMAIN.

## ANAME and ACME boundaries

A signed answer cannot be synthesized after signing. The publication graph is
therefore visible:

```text
source -> ACME overlay -> selected ANAME materialization
       -> rgbsec-data -> rgbsec-check -> atomic activation
```

For an ANAME in a `K` zone, `aname-materialize` resolves the target before
signing and writes ordinary A and AAAA RRsets under the configured TTL ceiling.
The secondary receives those addresses and signatures; it does not resolve the
target independently for that signed snapshot. ANAME directives in `U` zones
retain the ordinary runtime behavior.

ACME is a privilege-boundary question, not only a record-format question. An
ACME-managed `U` zone can keep using the unprivileged live overlay even while
another zone in the same CDB is signed. Inline updates to a `K` zone would need
an explicitly designed privileged publisher that acknowledges an update only
after a new signed CDB is durable. `rgbdns-setup` does not invent that
escalation. Without such a hook, a signed ACME policy fails closed.

The simplest arrangement is often a small unsigned validation child:

```text
_acme-challenge.example.com.  NS  ns1.validation.example.
```

The production lesson is conservative: choose the first DNSSEC pilot from a
stable zone with ordinary A or AAAA data, no live ACME overlay, and no ANAME.
Move dynamic zones only after their signed publication boundary is explicit.

## Activating the parent DS

Signing the child does not complete the chain. The parent must publish a DS
that matches the active child DNSKEY. Derive it from the installed policy
rather than transcribing public-key material by hand:

```sh
sudo rgbsec-ds "$(
  sudo grep '^Kexample[.]com[.]:' /etc/rgbdns/dnssec
)"
```

The output is a presentation-format record:

```text
example.com. IN DS <key-tag> 13 2 <64-hex-sha256-digest>
```

That line is illustrative; use the output for the actual key. Registrar forms
usually ask for four fields: key tag, algorithm 13, digest type 2, and the
SHA-256 digest. Publish it only after every nameserver in the parent delegation
serves the matching signed snapshot. A stale third-party secondary is enough
to make validation intermittent.

Check the parent directly, then use several independent validating resolvers:

```sh
dig +short +nosplit example.com DS

for resolver in 1.1.1.1 9.9.9.9 8.8.8.8; do
  dig +dnssec @"$resolver" example.com A
  dig +dnssec @"$resolver" \
    "MiXeD-$(date +%s).example.com" A
done
```

A secure positive response is `NOERROR` with the AD flag. A secure negative
response is NXDOMAIN with AD. Querying a fresh name avoids mistaking an old
negative cache entry for current behavior. The parent DS, child DNSKEY, and
signatures can be correct while a resolver still exposes a transport,
delegation, case-handling, or denial-proof defect; end-to-end testing is what
turns a collection of records into evidence.

## Lifecycle and safe rollback

DNSSEC adds time to the authority contract. Monitor the publisher and checker,
the earliest RRSIG expiration, key tag, parent DS, primary/secondary serials,
and both positive and negative validation. Restart each authority during the
pilot and prove that the last signed CDB remains available. Before the first
signature window closes, observe an automatic refresh and verify the new
inception and expiration on both authorities.

Version 0.6.3 deliberately supports one active combined signing key per zone.
It does not pretend that replacing a key file is a rollover protocol. A future
multi-key release must represent publication, DS overlap, retirement, and
recovery as explicit states.

Rollback order is asymmetric. Before DS activation, the operator may return to
the unsigned path because no validator expects signatures. After DS activation:

1. remove the DS at the parent;
2. wait through the parent DS TTL, negative caches, and relevant resolver
   caches;
3. confirm that independent resolvers no longer observe the DS; and only then
4. disable signing and publish unsigned data.

Removing signatures while the DS remains visible converts the zone from secure
to bogus. Keep the retired private key backed up until the DS removal and cache
window are conclusively complete.

## Constraints are part of the design

rgbdns uses NSEC rather than NSEC3 for authoritative signing. NSEC is smaller,
simpler, and avoids iteration and opt-out machinery; it also makes zone names
enumerable. Location-dependent and time-qualified data cannot be signed because
one owner and type must identify one stable RRset. Pre-existing DNSKEY, RRSIG,
NSEC, and NSEC3 input is rejected. ANAME must be materialized before signing.

These are not footnotes hidden behind a “DNSSEC enabled” switch. They are the
conditions under which an offline immutable snapshot can honestly claim to be
the zone. The small-tool approach makes each condition visible, testable, and
replaceable without putting private keys in the packet-serving process.

## Generated Code Fragment Index

Each card names an rgbdns fragment. In FirstPair Reader, Show code lists its numbered lines and Open file opens the collocated code note with those lines marked.

```rgbdns-fragment
{"id": "rgbdns-frag-0c2a1a2a636a", "codeNote": "DNS from First Principles/Code/src/bin/dnscache.rs.source", "sourcePath": "src/bin/dnscache.rs", "startLine": 21, "endLine": 35}
```

```rgbdns-fragment
{"id": "rgbdns-frag-f9c2e38b98db", "codeNote": "DNS from First Principles/Code/src/bin/dnscache.rs.source", "sourcePath": "src/bin/dnscache.rs", "startLine": 36, "endLine": 116}
```

```rgbdns-fragment
{"id": "rgbdns-frag-c4c75c363342", "codeNote": "DNS from First Principles/Code/src/bin/dnscache.rs.source", "sourcePath": "src/bin/dnscache.rs", "startLine": 143, "endLine": 151}
```

```rgbdns-fragment
{"id": "rgbdns-frag-93419b6c9023", "codeNote": "DNS from First Principles/Code/src/bin/dnscache.rs.source", "sourcePath": "src/bin/dnscache.rs", "startLine": 152, "endLine": 160}
```

```rgbdns-fragment
{"id": "rgbdns-frag-985f21d17a56", "codeNote": "DNS from First Principles/Code/src/bin/dnscache.rs.source", "sourcePath": "src/bin/dnscache.rs", "startLine": 161, "endLine": 180}
```

```rgbdns-fragment
{"id": "rgbdns-frag-3790fed38817", "codeNote": "DNS from First Principles/Code/src/dnscache_config.rs.source", "sourcePath": "src/dnscache_config.rs", "startLine": 12, "endLine": 12}
```

```rgbdns-fragment
{"id": "rgbdns-frag-f59cd49b7a3e", "codeNote": "DNS from First Principles/Code/src/dnscache_config.rs.source", "sourcePath": "src/dnscache_config.rs", "startLine": 13, "endLine": 13}
```

```rgbdns-fragment
{"id": "rgbdns-frag-e2e70010249a", "codeNote": "DNS from First Principles/Code/src/dnscache_config.rs.source", "sourcePath": "src/dnscache_config.rs", "startLine": 14, "endLine": 16}
```

```rgbdns-fragment
{"id": "rgbdns-frag-128d48d8a73e", "codeNote": "DNS from First Principles/Code/src/dnscache_config.rs.source", "sourcePath": "src/dnscache_config.rs", "startLine": 17, "endLine": 26}
```

```rgbdns-fragment
{"id": "rgbdns-frag-c527bbc0d8ea", "codeNote": "DNS from First Principles/Code/src/dnscache_config.rs.source", "sourcePath": "src/dnscache_config.rs", "startLine": 27, "endLine": 31}
```

```rgbdns-fragment
{"id": "rgbdns-frag-2ed2d81c2acf", "codeNote": "DNS from First Principles/Code/src/dnscache_config.rs.source", "sourcePath": "src/dnscache_config.rs", "startLine": 32, "endLine": 32}
```

```rgbdns-fragment
{"id": "rgbdns-frag-1c885999a2f2", "codeNote": "DNS from First Principles/Code/src/dnscache_config.rs.source", "sourcePath": "src/dnscache_config.rs", "startLine": 33, "endLine": 45}
```
