# Optional authoritative DNSSEC

rgbdns 0.6.2 adds authoritative DNSSEC as an explicit offline publication
pipeline. It is disabled by default. If `/etc/rgbdns/dnssec.env` and the
working-directory `dnssec` policy are absent, `tinydns-data`, `tinydns`, ACME,
ANAME, setup, and secondary synchronization follow the existing
djbdns-compatible path.

The authoritative server never opens a private key. It only serves DNSKEY,
RRSIG, NSEC, and DS records already present in its CDB. Secondaries receive the
same finished records over ordinary AXFR and also need no key.

## Create a key and policy

Create the key as root on the primary. `rgbsec-keygen` refuses to overwrite a
file and creates it mode 0600:

```sh
sudo install -d -o root -g root -m 0700 /etc/rgbdns/keys
sudo rgbsec-keygen example.com /etc/rgbdns/keys/example.com.pk8 \
  | sudo tee /etc/rgbdns/dnssec
sudo chown root:rgbdns /etc/rgbdns/dnssec
sudo chmod 0640 /etc/rgbdns/dnssec
```

The output is one colon-delimited line:

```text
Kexample.com.:/etc/rgbdns/keys/example.com.pk8:13:1209600:86400:3600
```

The fields are zone, absolute key path, algorithm, signature validity, refresh
window, and inception skew. Algorithm 13 is ECDSA P-256/SHA-256. Put exactly
one disposition line in the policy for every authoritative zone in the source
file. `K` signs a zone; `Uzone` explicitly leaves one unsigned:

```text
Ulegacy.example.
```

The signer fails closed if a zone is missing, duplicated, or both `K` and `U`.
This permits a deliberate mixed signed/unsigned CDB without making omission
mean “unsigned.”

Keep a recoverable encrypted backup of the key. Key replacement is not an
implicit file operation: version 0.6.2 deliberately supports one active
combined signing key per zone and does not claim an automated multi-key
rollover state machine.

## Inspect the stages manually

The individual transformations can be run without systemd:

```sh
acme-materialize data /etc/rgbdns/acme-update.conf \
  /var/lib/rgbdns/tinydns data.acme
aname-materialize data.acme data.materialized dnssec
ln -s /etc/rgbdns/dnssec dnssec
rgbsec-sign data.materialized data.signed
rgbsec-data data.materialized data.cdb
rgbsec-check data.cdb /etc/rgbdns/dnssec
rgbsec-ds 'Kexample.com.:/etc/rgbdns/keys/example.com.pk8:13:1209600:86400:3600'
```

With the third argument, `aname-materialize` resolves ANAME only in `K` zones
and preserves ANAME directives in `U` zones. The first three commands expose
each text stage for inspection. `rgbsec-data` is the shorter sign-and-compile
transform when the intermediate signed text is not needed. Both signing
commands read the one-line policy from `dnssec` in their working directory.
Every output is built beside its destination and renamed only after a successful
write. `rgbsec-check` cryptographically verifies every authoritative RRset,
checks the NSEC chain and validity interval, and emits:

```text
example.com.    SERIAL    KEYTAG    EARLIEST_EXPIRATION    SECONDS_LEFT    ok
```

The fields are tab-separated and suitable for cron or another supervisor.

## Enable packaged publication

Pass the policy while configuring a primary:

```sh
sudo rgbdns-setup primary \
  --data /path/to/rgbdns.data \
  --dnssec-policy /etc/rgbdns/dnssec
```

This creates `/etc/rgbdns/dnssec.env`, performs the first publication as root,
and enables two timers. `rgbdns-dnssec-publish.timer` refreshes materialized
ANAME data and signatures every 12 hours. `rgbdns-dnssec-check.timer` verifies
the active CDB hourly as the unprivileged `rgbdns` account. A failed resolver,
signer, compiler, or checker leaves the prior CDB active.

The root-only publisher reads the private key. The `rgbdns` service account
can read the public policy and active CDB but cannot traverse
`/etc/rgbdns/keys`. Removing `/etc/rgbdns/dnssec.env` returns `compile-zone` to
the original `tinydns-data` path; use `rgbdns-setup` to make role changes so
timer state remains consistent.

## ANAME and ACME

A signed answer cannot be synthesized after signing. `aname-materialize`
therefore resolves private `Aowner:target:ttl-cap` directives in `K` zones and
writes ordinary A/AAAA records with capped TTLs. It preserves the directives in
`U` zones, which continue using the original runtime resolver. The scheduled
publisher refreshes signed materializations before signatures expire. Signed
AXFR contains the materialized addresses and signatures, not a requirement for
the secondary to resolve the target.

For ACME, delegation of `_acme-challenge` to a small unsigned validation zone
is simplest. An ACME-managed `U` zone remains on the unprivileged live-overlay
path even when another zone in the snapshot is signed; periodic root
publication incorporates its durable overlay without invoking the signer from
`tinydns`. Inline updates to a `K` zone require an explicitly configured
privileged publication command. `rgbdns-setup` does not synthesize that trust
boundary. Without such a hook, a signed ACME zone refuses startup rather than
publishing unsigned TXT.

## Parent DS and activation

Print the DS line with `rgbsec-ds`, then install the exact key tag, algorithm
13, digest type 2, and digest at the parent. Do not publish the DS until every
delegated authority answers DNSKEY and signed positive and negative queries.

Useful checks are:

```sh
dig @127.0.0.1 example.com SOA +dnssec
dig @127.0.0.1 example.com DNSKEY +dnssec
dig @127.0.0.1 does-not-exist.example.com A +dnssec
dig @127.0.0.1 child.example.com DS +dnssec
delv example.com
```

After parent publication, monitor from more than one validating network. To
disable DNSSEC safely, remove the parent DS first, wait through its TTL and
negative caches, and only then remove local policy and republish unsigned data.

## Constraints

- Location-dependent (`%`) or TAI64 activation/expiration data cannot be
  signed because one owner/type must identify one stable RRset. `rgbsec-data`
  retains such data in `U` zones. The text-only `rgbsec-sign` export cannot.
- If a `K` zone contains ANAME, the current text materialization stage requires
  all zones in that source to be free of location and time qualifiers. Without
  ANAME in a `K` zone, mixed-zone materialization is a byte-for-byte pass-through.
- ANAME must be materialized before signing.
- Pre-existing DNSKEY, RRSIG, NSEC, or NSEC3 records in source are rejected.
- NSEC is intentionally used instead of NSEC3: it is smaller, simpler, and
  avoids iteration and opt-out complexity. Zone contents are enumerable.
- Direct public-network DNSSEC validation remains an opt-in test.

The exact command and file contract is specified in
[`DNSSEC-DESIGN.md`](DNSSEC-DESIGN.md).
