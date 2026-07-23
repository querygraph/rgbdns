# rgbdns

`rgbdns` is a memory-safe Rust reimplementation of djbdns. The first runnable
slice provides djbdns-compatible tinydns data parsing, authoritative UDP and TCP
DNS, `tinydns-get`, `tinydns-data`, and `dnsq`, with strict bounded packet
parsing, IPv4/IPv6, wildcards, negative answers, and safe OS-generated query IDs.

```sh
cargo test
IP=127.0.0.1 PORT=5353 cargo run --release --bin tinydns
```

The service reads `data` directly and atomically on startup; it deliberately
does not reproduce cdb's unchecked native-memory parsing. See
[`docs/compatibility.md`](docs/compatibility.md) for scope and research.
