---
type: "code-fragment"
fragment_id: "rgbdns-frag-8f6c800e8ac3"
source_path: "README.md"
code_note: "DNS from First Principles/Code/README.md.source"
language: "markdown"
subsystem: "Repository and build"
symbol: "ACME DNS-01 updates"
kind: "heading"
start_line: 128
end_line: 187
---

# ACME DNS-01 updates

- Fragment ID: `rgbdns-frag-8f6c800e8ac3`
- Source file: [[DNS from First Principles/Code/README.md.source|README.md]]
- Lines: 128-187
- Subsystem: [[DNS from First Principles/Subsystems/Repository and build|Repository and build]]

```rgbdns-fragment
{"id": "rgbdns-frag-8f6c800e8ac3", "codeNote": "DNS from First Principles/Code/README.md.source", "sourcePath": "README.md", "startLine": 128, "endLine": 187}
```

## Excerpt

<span id="rgbdns-frag-8f6c800e8ac3" class="rgbdns-fragment-target"></span>
### rgbdns-frag-8f6c800e8ac3: heading ACME DNS-01 updates

````markdown
## ACME DNS-01 updates

rgbdns 0.4.0 accepts TCP RFC 2136 UPDATE messages authenticated with
HMAC-SHA256 TSIG for narrowly scoped ACME DNS-01 TXT records. Updates are
disabled by default. A key can modify only `_acme-challenge` owners in its
configured primary zone; accepted values are durable, immediately visible to
queries, assigned a monotonic SOA serial, and included in standard AXFR.

Create a 32-byte secret and configure a policy:

```sh
openssl rand -base64 32
sudoedit /etc/rgbdns/acme-update.conf
```

```text
certbot-chiefscientist. hmac-sha256. BASE64_SECRET chiefscientist.org. _acme-challenge. 60
```

Enable it while configuring the primary:

```sh
sudo rgbdns-setup primary --data rgbdns.data \
  --acme-update-config /etc/rgbdns/acme-update.conf
```

Certbot's RFC 2136 credentials use the same server, key name, secret, and
HMAC-SHA256 algorithm. `rgbdns-acme present` and `cleanup` provide a local
manual-hook interface; cleanup removes only its specified value so overlapping
wildcard and ordinary validations remain safe. See [`RGBDNS_LETS.md`](RGBDNS_LETS.md)
for the protocol, state, policy, security, and test design.

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
sudo apt install ../rgbdns_0.6.5_$(dpkg --print-architecture).deb
```

On openSUSE Leap 16.0, build the RPM with:

```sh
sudo zypper --non-interactive install \
  git cargo rust python3 rpm-build systemd-rpm-macros
packaging/build-rpm.sh
sudo zypper --non-interactive --no-gpg-checks install \
  dist/rpmbuild/RPMS/x86_64/rgbdns-0.6.5-1.x86_64.rpm
```

````
