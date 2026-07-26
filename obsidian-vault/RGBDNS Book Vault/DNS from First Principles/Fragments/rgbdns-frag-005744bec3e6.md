---
type: "code-fragment"
fragment_id: "rgbdns-frag-005744bec3e6"
source_path: "docs/DEBIAN.md"
code_note: "DNS from First Principles/Code/docs/DEBIAN.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "Prepare a primary nameserver"
kind: "heading"
start_line: 78
end_line: 132
---

# Prepare a primary nameserver

- Fragment ID: `rgbdns-frag-005744bec3e6`
- Source file: [[DNS from First Principles/Code/docs/DEBIAN.md.source|docs/DEBIAN.md]]
- Lines: 78-132
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-005744bec3e6", "codeNote": "DNS from First Principles/Code/docs/DEBIAN.md.source", "heading": "rgbdns-frag-005744bec3e6: heading Prepare a primary nameserver", "sourcePath": "docs/DEBIAN.md", "startLine": 78, "endLine": 132}
```

## Excerpt

<span id="rgbdns-frag-005744bec3e6" class="rgbdns-fragment-target"></span>
### rgbdns-frag-005744bec3e6: heading Prepare a primary nameserver

```markdown
## Prepare a primary nameserver

rgbdns uses the tinydns text format. Start from the packaged example:

```sh
sudo install -m 0644 \
  /usr/share/doc/rgbdns/examples/data \
  /root/example.net.data
sudo editor /root/example.net.data
```

A minimal zone needs an SOA, authoritative NS records, and address records for
in-zone nameserver names:

```text
Zexample.net:ns1.example.net:hostmaster.example.net:2026072301:16384:2048:1048576:2560:3600
&example.net:192.0.2.53:ns1.example.net:3600
+ns1.example.net:192.0.2.53:3600
+www.example.net:192.0.2.80:300
```

Replace all documentation addresses. The SOA administrator field uses a DNS
name (`hostmaster.example.net` represents `hostmaster@example.net`). Increment
the serial whenever the zone changes.

Configure, compile, enable, and start the primary:

```sh
sudo rgbdns-setup primary \
  --data /root/example.net.data \
  --listen-ip 192.0.2.53
```

The setup command creates the account if necessary, installs the source as
`/var/lib/rgbdns/tinydns/data`, writes `/etc/rgbdns/tinydns.env`, compiles
`data.cdb` as the service user, reloads systemd, and enables
`rgbdns-tinydns.service`. Re-running the command safely replaces the managed
configuration.

To validate before starting anything, add `--no-start`, then inspect:

```sh
sudo -u rgbdns /usr/lib/rgbdns/compile-zone
sudo systemd-analyze verify \
  /lib/systemd/system/rgbdns-tinydns.service
sudo systemctl start rgbdns-tinydns
```

After editing the managed data file directly, compile and restart:

```sh
sudo -u rgbdns /usr/lib/rgbdns/compile-zone
sudo systemctl restart rgbdns-tinydns
```

```
