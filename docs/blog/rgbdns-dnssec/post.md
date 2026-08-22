# DNSSEC in the spirit of tinydns

*August 2026 — rgbdns 0.6.3*

![Polynesian islands protected by double-hulled canoes and Kon-Tiki-style rafts, linked by electric pulses carried through the sea by eels and through the air by flying fish.](rgbdns-dnssec-islands.png)

DNSSEC can look like an invitation to build a control plane: a key database, a
signing service, an RPC protocol, a stateful rollover controller, and an online
authority that knows too much. That would be one way to add cryptographic
signatures to DNS. It would not be the tinydns way.

rgbdns 0.6.3 now supports authoritative DNSSEC as a composition of small
utilities, one-line policy, immutable snapshots, and atomic publication. The
ordinary server never opens a private key. A secondary never receives one. A
zone without DNSSEC continues through the original djbdns-compatible path.

The image above is the mental model I ended up with. Each island is a zone. The
boats around it are independent authorities carrying the same signed facts.
The electric pulses are signatures and authenticated transfers. Eels keep the
underwater path alive; flying fish bridge gaps in the air. The protection does
not come from one enormous ship. It comes from a visible network whose members
can be checked independently.

## Five programs, five jobs

The new commands are deliberately unambitious in isolation:

- `rgbsec-keygen` creates one ECDSA P-256 key and prints its policy line.
- `rgbsec-sign` turns tinydns text into inspectable signed tinydns text.
- `rgbsec-data` signs and compiles directly to CDB.
- `rgbsec-ds` derives the DS record that belongs at the parent.
- `rgbsec-check` verifies every signed RRset, the NSEC cycle, and the remaining
  signature lifetime.

The policy is one line per authoritative zone:

```text
Kexample.com.:/etc/rgbdns/keys/example.com.pk8:13:1209600:86400:3600
Ulegacy.example.
```

`K` means that the zone is signed. `U` means that it is deliberately unsigned.
Every zone in the source must have exactly one disposition. Missing,
duplicated, or conflicting lines stop publication. An omitted line never
silently changes a zone's security state.

That makes mixed operation possible without making it ambiguous. A stable zone
can be signed while a neighboring zone continues to use location-qualified
records, live ACME updates, or runtime ANAME behavior.

## The private key stops at the primary

Key creation is an explicit root operation:

```sh
sudo install -d -o root -g root -m 0700 /etc/rgbdns/keys
sudo rgbsec-keygen example.com \
  /etc/rgbdns/keys/example.com.pk8 \
  | sudo tee /etc/rgbdns/dnssec
sudo chown root:rgbdns /etc/rgbdns/dnssec
sudo chmod 0640 /etc/rgbdns/dnssec
```

The key is PKCS#8 DER, mode 0600, and backed up separately. The line printed on
stdout is public policy, not secret material. It records the zone, absolute key
path, algorithm, signature lifetime, refresh window, and inception skew.

The publication graph stays inspectable:

```text
source -> ACME overlay -> selected ANAME materialization
       -> rgbsec-data -> rgbsec-check -> atomic activation
```

Every stage writes a sibling temporary file and renames only after success. A
failed lookup, signature, compile, reload, or verification leaves the previous
CDB active. `tinydns` only serves records from that finished CDB.

The packaged setup enables a privileged publisher every twelve hours and an
unprivileged checker every hour:

```sh
sudo rgbdns-setup primary \
  --data /srv/dns/rgbdns.data \
  --listen-ip 0.0.0.0 \
  --allow-nets 10.0.2.10/32 \
  --dnssec-policy /etc/rgbdns/dnssec
```

The root-only publisher reads the key. The `rgbdns` checker reads the active
public snapshot. That separation matters more than whether the commands happen
to share a Rust crate.

## Secondaries carry signatures, not keys

DNSKEY, RRSIG, NSEC, and DS are ordinary DNS records on the wire. AXFR carries
them to a secondary exactly as it carries A, MX, or TXT. The secondary validates
the transfer, compiles the finished zone, and serves it without signing
anything.

```sh
sudo rgbdns-setup secondary \
  --zone example.com \
  --primary 10.0.1.10 \
  --listen-ip 0.0.0.0
```

This is a useful compromise boundary. A serving secondary may be geographically
and administratively independent, but compromise of that server does not reveal
the key that can create new signed data.

It also means the pre-DS gate is simple to state: every nameserver in the public
delegation must serve the same SOA serial, DNSKEY, positive signatures, and
authenticated negative answers. A stale ordinary secondary is not harmless
after the parent starts promising DNSSEC.

## The parent adopts the child's key by digest

The child publishes DNSKEY. The parent publishes DS. `rgbsec-ds` connects the
two without asking an operator to reconstruct wire data by hand:

```sh
sudo rgbsec-ds "$(
  sudo grep '^Kexample[.]com[.]:' /etc/rgbdns/dnssec
)"
```

Registrar forms usually split the result into key tag, algorithm 13, digest
type 2, and a 64-hex-digit SHA-256 digest. The DS must not appear until every
delegated authority has the signed child data. Once it does appear, removing
signatures first makes the domain bogus. Safe rollback removes the parent DS,
waits through caches, confirms that resolvers no longer see it, and only then
returns the child to unsigned publication.

Two small public zones carried the rollout. `bitcoin.science` established the
first complete chain. Its parent publishes key tag 53856, algorithm 13, digest
type 2, and public resolvers validate both positive and negative answers.
`foto.gs` followed with key tag 38557. Its DS is now present at the `.gs` parent
and validates through independent recursive services.

The useful test is not merely “does DNSKEY exist?” It is an end-to-end fresh
negative query:

```sh
for resolver in 1.1.1.1 9.9.9.9 8.8.8.8; do
  dig +dnssec @"$resolver" \
    "MiXeD-$(date +%s).bitcoin.science" A
done
```

Each answer should be NXDOMAIN with the AD flag. The fresh owner avoids an old
negative cache entry. The mixed case exercises DNS 0x20 randomization.

## The bug that only Google showed us

The first `bitcoin.science` activation exposed a good example of why independent
validators matter. Cloudflare and Quad9 accepted signed positive and negative
answers. Google accepted positive answers but returned SERVFAIL for fresh
negative names.

Google's recursive path randomized the letter case of the query name. rgbdns's
name equality and ordering were already case-insensitive, but the helper that
decided whether a name was beneath an authoritative zone compared raw label
slices. A mixed-case nonexistent name was therefore refused before the server
could return its signed NSEC proof.

rgbdns 0.6.3 fixes authoritative suffix matching and retains the exact case as a
regression test. The production deployment gate now asks for
`MiSsInG.<zone>`, not only lowercase `missing.<zone>`. A property discovered on
the public Internet became a permanent local invariant.

## ANAME and ACME make the boundary visible

An offline signature can authenticate only a stable RRset. A signed ANAME zone
must therefore resolve its target before signing. `aname-materialize` converts
the directive into ordinary A and AAAA records under the configured TTL ceiling;
the publisher refreshes those records and their signatures before expiry.

An unsigned zone in the same snapshot can retain runtime ANAME behavior. The
policy says which model applies instead of asking the server to guess.

ACME is stricter. An unprivileged live TXT overlay can continue in a `U` zone.
An inline update to a `K` zone would cross into the root-only signing path and
needs an explicitly designed privileged publisher. rgbdns does not synthesize
that escalation. The simplest answer is often to delegate
`_acme-challenge` to a small unsigned validation child.

This is why the first pilots were intentionally boring: ordinary addresses, no
ANAME, no live ACME updates. Cryptography is easier to reason about when the
data model is not moving underneath it.

## Evidence, not a checkbox

The strongest local command is:

```sh
sudo rgbsec-check \
  /var/lib/rgbdns/tinydns/data.cdb \
  /etc/rgbdns/dnssec
```

It exits zero only when every configured zone has the promised security state,
every authoritative RRset verifies, every NSEC cycle closes, and the earliest
signature remains outside its refresh window. Its tab-separated output gives
the zone, SOA serial, key tag, expiration, seconds remaining, and `ok`.

The public proof adds parent DS checks, direct UDP and TCP queries to every
authority, signed positive, NXDOMAIN, and NODATA answers, and AD validation
through several recursive networks. The lifecycle proof is still longer: watch
the publisher refresh signatures before they expire, restart each authority,
and rehearse recovery from the protected primary-only key backup.

That is DNSSEC in the spirit of tinydns: cryptographic protection without
making the serving process magical. Small boats, visible routes, independent
checks, and no single vessel carrying every secret.

The full design and operational examples are in the updated
[*DNS from First Principles*](https://firstpair.org/books/rgbdns/) and the repository's
[authoritative DNSSEC guide](https://github.com/querygraph/rgbdns/blob/master/docs/DNSSEC.md).
