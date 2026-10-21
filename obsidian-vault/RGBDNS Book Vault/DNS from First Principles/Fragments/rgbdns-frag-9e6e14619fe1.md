---
type: "code-fragment"
fragment_id: "rgbdns-frag-9e6e14619fe1"
source_path: "docs/DEBIAN.md"
code_note: "DNS from First Principles/Code/docs/DEBIAN.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "Prepare a primary nameserver"
kind: "heading"
start_line: 82
end_line: 159
---

# Prepare a primary nameserver

- Fragment ID: `rgbdns-frag-9e6e14619fe1`
- Source file: [[DNS from First Principles/Code/docs/DEBIAN.md.source|docs/DEBIAN.md]]
- Lines: 82-159
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-9e6e14619fe1", "codeNote": "DNS from First Principles/Code/docs/DEBIAN.md.source", "sourcePath": "docs/DEBIAN.md", "startLine": 82, "endLine": 159}
```

## Excerpt

<span id="rgbdns-frag-9e6e14619fe1" class="rgbdns-fragment-target"></span>
### rgbdns-frag-9e6e14619fe1: heading Prepare a primary nameserver

````markdown
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

Setup also watches `rgbdns.data` in the invoking sudo user's home directory.
Override the destination with `--data-drop FILE` and
`--data-drop-owner USER`. Publish edits atomically:

```sh
scp rgbdns.data primary.example:rgbdns.data.new
ssh primary.example 'mv rgbdns.data.new rgbdns.data'
```

`rgbdns-data.path` verifies the file owner, compiles a private staged copy, and
replaces the live `data` and `data.cdb` only after compilation succeeds. It
then restarts tinydns. A malformed, partial, or symlinked upload leaves the
currently served database unchanged.

On a DNSSEC primary the import validates the upload with the plain
`tinydns-data` compiler, replaces only `data`, and leaves `data.cdb` to
`rgbdns-dnssec-publish.service`, which signs, verifies and publishes it.

A primary whose data is deployed into `/var/lib/rgbdns/tinydns` directly, for
example by a CI workflow, can turn the import off with
`--data-drop-import disabled`, or by adding `DATA_DROP_IMPORT=disabled` to
`/etc/rgbdns/data-drop.env`. Uploads are then skipped, not failed.

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

````
