# rgbdns

`rgbdns` is a memory-safe Rust reimplementation of djbdns. The current runnable
slice provides djbdns-compatible tinydns text and CDB data, authoritative UDP
and TCP DNS, a DNSSEC-validating iterative cache, `tinydns-get`,
`tinydns-data`, `axfrdns`, `axfr-get`, `rbldns`, `walldns`, and `dnsq`, with
strict bounded packet parsing, IPv4/IPv6, wildcards, negative answers, and safe
OS-generated query IDs.

```sh
cargo test
IP=127.0.0.1 PORT=5353 cargo run --release --bin tinydns
IP=127.0.0.1 PORT=5354 cargo run --release --bin dnscache
IP=127.0.0.1 PORT=5355 cargo run --release --bin axfrdns
cargo run --release --bin axfr-get -- example 127.0.0.1:5355 data.new data.tmp
```

`tinydns-data` atomically compiles `data` to the original djbdns `data.cdb`
layout, and `tinydns` reads `data.cdb` by default. The loader bounds the database
and validates every key, value, name, and RDATA field rather than relying on
unchecked native-memory parsing. Set `DATA=data` to serve the text form
directly. See [`docs/compatibility.md`](docs/compatibility.md) for scope and
research.

`dnscache` performs iteration from `config/root.hints`, validates DNSSEC using
the bundled root trust anchor, randomizes UDP query IDs, ports, and letter case,
and only serves loopback clients by default. Set `ALLOW_NETS` to a comma-
separated CIDR list to authorize additional clients.

`axfrdns` is TCP-only and likewise permits loopback clients by default. Its
`ALLOW_NETS` setting accepts comma-separated IPv4 or IPv6 CIDRs.

The recursive client commands read `DNSCACHEIP` (a comma-separated list of IP
or `IP:port` endpoints) when set, otherwise they use `/etc/resolv.conf`.

rgbdns supports private ANAME directives for CNAME-like apex hosting without
placing an invalid CNAME on the wire:

```text
.example.com:192.0.2.53:ns1.example.com
Aexample.com:customer.blog-host.example:300
```

The `A` line resolves the target and synthesizes authoritative A and AAAA
answers owned by `example.com`; 300 seconds is the TTL cap. It may coexist
with the apex SOA, NS, MX, and TXT records, but not with A, AAAA, or CNAME at
the same owner. ANAME resolution uses `DNSCACHEIP` or `/etc/resolv.conf`.

The `*-conf` commands generate djbdns-style service directories. They reference
rgbdns's own `setuidgid` and `multilog` binaries by absolute path, so
daemontools is not a runtime dependency. `multilog t ./main` writes TAI64N
timestamps to `main/current`; optional `s<size>` and `n<count>` arguments set
the rotation threshold and retained-file count. Daemons continue to write
diagnostics to stderr, allowing the same binaries to work under daemontools,
systemd, containers, or another supervisor.

`tinydns` also writes one original-compatible request record to stderr by
default. The raw record deliberately has no timestamp, so systemd-journald or
`multilog t` can timestamp and rotate the same stream. Set `QUERY_LOG=0`, or
pass `rgbdns-setup --query-log 0`, only when per-request logging is
intentionally disabled.

## Linux packages and systemd

The repository includes native Debian and openSUSE RPM packaging, hardened systemd services, and
an idempotent `rgbdns-setup` command for primary and secondary authoritative
servers. See [`docs/DEBIAN.md`](docs/DEBIAN.md) for package builds, account and
directory layout, tinydns data-file setup, firewalls, AXFR allow-lists, timed
secondary refresh, verification, upgrades, and troubleshooting. It includes a
complete `cron.sh` deployment with `52.10.53.234` as the primary address and
BuddyNS as the secondary network.

Primaries watch `rgbdns.data`, and secondaries watch a canonical
one-zone-per-line `rgbdns.zones`, in the deployment user's home directory.
Publish either file through a `.new` name followed by a remote rename. rgbdns
validates and compiles primary data before activation, and validates secondary
lists before starting AXFR refresh.

For the complete production-shaped `fieldnotes.es` example using
`a.ns.cron.sh`, `b.ns.cron.sh`, BuddyNS, primary data pickup, and secondary
zone-list pickup, follow
[`docs/RGBDNS_SETUP.md`](docs/RGBDNS_SETUP.md).

For the current openSUSE Leap 16.0 x86_64 AWS Marketplace AMI, see
[`docs/OPENSUSE.md`](docs/OPENSUSE.md). It covers launch, RPM build and install,
firewalld, the complete `cron.sh` primary, integrated AXFR, BuddyNS delegation,
systemd persistence, upgrades, removal, and troubleshooting.

On Debian or Ubuntu, build the package with:

```sh
sudo apt install build-essential cargo debhelper rustc
packaging/build-deb.sh
sudo apt install ../rgbdns_0.2.2_$(dpkg --print-architecture).deb
```

On openSUSE Leap 16.0, build the RPM with:

```sh
sudo zypper --non-interactive install \
  git cargo rust python3 rpm-build systemd-rpm-macros
packaging/build-rpm.sh
sudo zypper --non-interactive --no-gpg-checks install \
  dist/rpmbuild/RPMS/x86_64/rgbdns-0.2.2-1.x86_64.rpm
```

## Book

[*DNS from First Principles*](docs/book/rgbdns.md) develops the protocol from
names and packets through authority, recursion, DNSSEC, transfers, operations,
and security, then maps each concept to rgbdns. It also compares systemd,
runit, s6/s6-rc, OpenRC, and container-native replacements for
`svc`/`supervise`.

The committed [Obsidian reader vault](obsidian-vault/RGBDNS%20Book%20Vault)
adds a codebase-exploration part, collocates the full text/code surface, and
bundles a reader plugin for chapter navigation and prose-to-code fragment
jumps. See [the vault guide](docs/OBSIDIAN-VAULT.md) to rebuild and validate it.

Build the FirstPair package with Pandoc and Typst:

```sh
docs/book/build.sh
docs/book/validate.sh
```

## Conformance and performance

[`docs/conformance.md`](docs/conformance.md) maps implemented DNS requirements
to RFC-numbered, adversarial, property, live-network, and independent ldns
tests. [`docs/performance.md`](docs/performance.md) documents the stable-Rust
core benchmark:

```sh
cargo test --test rfc_conformance
cargo test --test wire_security
cargo bench --bench dns_core
```
