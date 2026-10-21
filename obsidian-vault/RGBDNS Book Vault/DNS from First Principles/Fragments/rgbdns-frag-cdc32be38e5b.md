---
type: "code-fragment"
fragment_id: "rgbdns-frag-cdc32be38e5b"
source_path: "docs/blog/operationalizing-rgbdns/post.md"
code_note: "DNS from First Principles/Code/docs/blog/operationalizing-rgbdns/post.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "From a package to an operating nameserver"
kind: "heading"
start_line: 26
end_line: 105
---

# From a package to an operating nameserver

- Fragment ID: `rgbdns-frag-cdc32be38e5b`
- Source file: [[DNS from First Principles/Code/docs/blog/operationalizing-rgbdns/post.md.source|docs/blog/operationalizing-rgbdns/post.md]]
- Lines: 26-105
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-cdc32be38e5b", "codeNote": "DNS from First Principles/Code/docs/blog/operationalizing-rgbdns/post.md.source", "sourcePath": "docs/blog/operationalizing-rgbdns/post.md", "startLine": 26, "endLine": 105}
```

## Excerpt

<span id="rgbdns-frag-cdc32be38e5b" class="rgbdns-fragment-target"></span>
### rgbdns-frag-cdc32be38e5b: heading From a package to an operating nameserver

````markdown
## From a package to an operating nameserver

The Debian package is intended for a Debian or Ubuntu primary. Download the
current `.deb` artifact, then install the local file with an explicit path:

```sh
sudo apt install ./rgbdns_0.3.3_amd64.deb
```

The openSUSE package provides the same binaries, service account, configuration
layout, and role tooling:

```sh
sudo zypper install ./rgbdns-0.3.3-1.x86_64.rpm
```

Both packages deliberately install an inert service. Installation creates the
`rgbdns` account and systemd units, but it does not guess whether the machine
is a primary, a secondary, or neither. The role is selected once with
`rgbdns-setup`.

For a primary at `a.ns.cron.sh`, the editable source can arrive as
`rgbdns.data` in the administrator's home directory:

```sh
sudo rgbdns-setup primary \
  --data-drop /home/bitnami/rgbdns.data \
  --data-drop-owner bitnami \
  --listen-ip 0.0.0.0 \
  --allow-nets 172.31.0.125/32
```

The path unit watches the drop location. A complete file is validated,
compiled into CDB, installed atomically, and activated without exposing a
partially written database. Publishing another zone is consequently a source
control and file-transfer operation, not an interactive edit on the server:

```sh
scp rgbdns.data.new primary.example:rgbdns.data.new
ssh primary.example 'mv rgbdns.data.new rgbdns.data'
```

For the openSUSE secondary at `b.ns.cron.sh`, the corresponding one-time setup
names the primary's private VPC address and a drop location for the desired
zone list:

```sh
sudo rgbdns-setup secondary \
  --zones-drop-owner ec2-user \
  --primary 172.31.60.189 \
  --listen-ip 0.0.0.0
```

The operator publishes one zone name per line:

```text
fieldnotes.es
aname-axfr.test
```

As on the primary, upload to a temporary name and rename only after the copy
finishes:

```sh
scp rgbdns.zones.new \
  secondary.example:/var/lib/rgbdns/incoming/rgbdns.zones.new
ssh secondary.example \
  'mv /var/lib/rgbdns/incoming/rgbdns.zones.new /var/lib/rgbdns/incoming/rgbdns.zones'
```

The secondary imports the list, transfers each zone, validates the result,
rebuilds its combined serving database, and keeps last-known-good data for a
zone whose refresh fails. One broken zone therefore does not make healthy
zones disappear.

Package upgrades preserve operator-owned configuration. Debian uses conffile
semantics; RPM uses `%config(noreplace)`. Since 0.3.3, an upgrade also restores
the configured authority and role-specific path or timer units, so installing
a newer package does not leave a previously running nameserver stopped.

````
