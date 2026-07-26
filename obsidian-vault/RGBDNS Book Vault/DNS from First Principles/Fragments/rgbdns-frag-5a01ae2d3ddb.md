---
type: "code-fragment"
fragment_id: "rgbdns-frag-5a01ae2d3ddb"
source_path: "README.md"
code_note: "DNS from First Principles/Code/README.md.source"
language: "markdown"
subsystem: "Repository and build"
symbol: "rgbdns"
kind: "heading"
start_line: 1
end_line: 56
---

# rgbdns

- Fragment ID: `rgbdns-frag-5a01ae2d3ddb`
- Source file: [[DNS from First Principles/Code/README.md.source|README.md]]
- Lines: 1-56
- Subsystem: [[DNS from First Principles/Subsystems/Repository and build|Repository and build]]

```rgbdns-fragment
{"id": "rgbdns-frag-5a01ae2d3ddb", "codeNote": "DNS from First Principles/Code/README.md.source", "heading": "rgbdns-frag-5a01ae2d3ddb: heading rgbdns", "sourcePath": "README.md", "startLine": 1, "endLine": 56}
```

## Excerpt

<span id="rgbdns-frag-5a01ae2d3ddb" class="rgbdns-fragment-target"></span>
### rgbdns-frag-5a01ae2d3ddb: heading rgbdns

```markdown
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

```
