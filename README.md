# rgbdns

`rgbdns` is a memory-safe Rust reimplementation of djbdns. The current runnable
slice provides djbdns-compatible tinydns text and CDB data, authoritative UDP
and TCP DNS, a DNSSEC-validating iterative cache, `tinydns-get`,
`tinydns-data`, and `dnsq`, with strict bounded packet parsing, IPv4/IPv6,
wildcards, negative answers, and safe OS-generated query IDs.

```sh
cargo test
IP=127.0.0.1 PORT=5353 cargo run --release --bin tinydns
IP=127.0.0.1 PORT=5354 cargo run --release --bin dnscache
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
