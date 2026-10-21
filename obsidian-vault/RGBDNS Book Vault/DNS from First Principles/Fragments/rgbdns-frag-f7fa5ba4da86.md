---
type: "code-fragment"
fragment_id: "rgbdns-frag-f7fa5ba4da86"
source_path: "docs/book/rgbdns.md"
code_note: "DNS from First Principles/Code/docs/book/rgbdns.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "a + b"
kind: "heading"
start_line: 1801
end_line: 1881
---

# a + b

- Fragment ID: `rgbdns-frag-f7fa5ba4da86`
- Source file: [[DNS from First Principles/Code/docs/book/rgbdns.md.source|docs/book/rgbdns.md]]
- Lines: 1801-1881
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-f7fa5ba4da86", "codeNote": "DNS from First Principles/Code/docs/book/rgbdns.md.source", "sourcePath": "docs/book/rgbdns.md", "startLine": 1801, "endLine": 1881}
```

## Excerpt

<span id="rgbdns-frag-f7fa5ba4da86" class="rgbdns-fragment-target"></span>
### rgbdns-frag-f7fa5ba4da86: heading a + b

````markdown
# a + b
PRIMARY_ALLOW_NETS="$SECONDARY_PRIVATE_IP/32"
```

Run only the assignment for the selected topology. Never allow the entire VPC
when one stable secondary address or security-group path is sufficient.

### Configure and cut over the primary

First stage configuration without claiming port 53:

```sh
sudo rgbdns-setup primary \
  --data /root/rgbdns.data \
  --listen-ip 0.0.0.0 \
  --port 53 \
  --allow-nets "$PRIMARY_ALLOW_NETS" \
  --no-start
sudo -u rgbdns /usr/lib/rgbdns/compile-zone
sudo systemd-analyze verify \
  /lib/systemd/system/rgbdns-tinydns.service 2>/dev/null ||
sudo systemd-analyze verify \
  /usr/lib/systemd/system/rgbdns-tinydns.service
sudo ls -lah /var/lib/rgbdns/tinydns
sudo cat /etc/rgbdns/tinydns.env
```

Setup watches `rgbdns.data` in the invoking sudo user's home directory. After
editing the complete source locally, publish it atomically:

```sh
scp rgbdns.data a.ns.example.net:rgbdns.data.new
ssh a.ns.example.net 'mv rgbdns.data.new rgbdns.data'
```

`rgbdns-data.path` verifies ownership, compiles a private staged copy, and
replaces the live source and CDB only after successful compilation. Invalid,
partial, or symlinked uploads leave the currently served database unchanged.
Use `--data-drop FILE` and `--data-drop-owner USER` when another destination is
required.

The two unit paths cover Debian-family and openSUSE layouts. An unrelated
legacy-unit warning from `systemd-analyze` does not invalidate a successful
rgbdns unit check.

On a migration host, identify the existing owner of port 53:

```sh
sudo ss -lntup '( sport = :53 )'
```

Stop only the old authoritative services. Do not stop an entire `runsvdir`
tree on a host where it also owns unrelated applications. For a classic
djbdns layout:

```sh
sudo sv down /etc/axfrdns /etc/axfrdns/log
sudo sv down /etc/tinydns /etc/tinydns/log
sudo ss -lntup '( sport = :53 )'
```

Then enable rgbdns:

```sh
sudo systemctl enable --now rgbdns-tinydns.service
sudo systemctl status rgbdns-tinydns.service --no-pager --full
sudo ss -lntup '( sport = :53 )'
```

The authoritative daemon serves normal UDP, normal TCP, and allowed AXFR on
the same port. Do not start the separately packaged `axfrdns` compatibility
command on port 53.

Verify the primary locally and publicly before configuring delegation:

```sh
dig @127.0.0.1 example.net SOA +norecurse
dig @127.0.0.1 example.net NS +norecurse
dig @127.0.0.1 example.net A +norecurse
dig +tcp @127.0.0.1 example.net SOA +norecurse

````
