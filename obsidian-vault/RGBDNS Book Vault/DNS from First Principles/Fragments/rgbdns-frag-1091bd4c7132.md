---
type: "code-fragment"
fragment_id: "rgbdns-frag-1091bd4c7132"
source_path: "docs/DEBIAN.md"
code_note: "DNS from First Principles/Code/docs/DEBIAN.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "Serve AXFR to secondary nameservers"
kind: "heading"
start_line: 133
end_line: 177
---

# Serve AXFR to secondary nameservers

- Fragment ID: `rgbdns-frag-1091bd4c7132`
- Source file: [[DNS from First Principles/Code/docs/DEBIAN.md.source|docs/DEBIAN.md]]
- Lines: 133-177
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-1091bd4c7132", "codeNote": "DNS from First Principles/Code/docs/DEBIAN.md.source", "heading": "rgbdns-frag-1091bd4c7132: heading Serve AXFR to secondary nameservers", "sourcePath": "docs/DEBIAN.md", "startLine": 133, "endLine": 177}
```

## Excerpt

<span id="rgbdns-frag-1091bd4c7132" class="rgbdns-fragment-target"></span>
### rgbdns-frag-1091bd4c7132: heading Serve AXFR to secondary nameservers

```markdown
## Serve AXFR to secondary nameservers

`tinydns` handles ordinary DNS and AXFR through the same TCP listener. AXFR is
disabled unless an explicit client allow-list is configured.

AXFR has no TSIG implementation in this release. Its security boundary is the
explicit source-address CIDR allow-list, so use narrow secondary addresses and
enforce the same policy in the host and perimeter firewalls.

Allow two secondary hosts to transfer from the ordinary DNS endpoint:

```sh
sudo rgbdns-setup primary \
  --data /root/example.net.data \
  --listen-ip 192.0.2.53 \
  --allow-nets 198.51.100.10/32,2001:db8:100::10/128
```

For a local lab:

```sh
sudo rgbdns-setup primary \
  --data /root/example.net.data \
  --listen-ip 127.0.0.1 --port 5353 \
  --allow-nets 127.0.0.1/32
```

The generated `/etc/rgbdns/tinydns.env` includes `ALLOW_NETS`. Protect that
file as configuration even though the allow-list is not a secret.

For nftables, a narrowly scoped IPv4 rule resembles:

```nft
ip saddr { 198.51.100.10, 198.51.100.11 } \
  ip daddr 192.0.2.53 tcp dport 53 accept
```

Permit UDP and TCP 53 to the public authoritative endpoint. The application
allow-list restricts AXFR; adjust the example to the host's existing table and
chain rather than pasting it blindly.

Test a transfer from an allowed secondary:

```sh
dig +tcp AXFR example.net @192.0.2.53
```
