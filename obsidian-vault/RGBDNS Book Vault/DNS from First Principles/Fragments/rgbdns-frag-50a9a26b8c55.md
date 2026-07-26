---
type: "code-fragment"
fragment_id: "rgbdns-frag-50a9a26b8c55"
source_path: "docs/DEBIAN.md"
code_note: "DNS from First Principles/Code/docs/DEBIAN.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "Cloud package build and verification"
kind: "heading"
start_line: 31
end_line: 77
---

# Cloud package build and verification

- Fragment ID: `rgbdns-frag-50a9a26b8c55`
- Source file: [[DNS from First Principles/Code/docs/DEBIAN.md.source|docs/DEBIAN.md]]
- Lines: 31-77
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-50a9a26b8c55", "codeNote": "DNS from First Principles/Code/docs/DEBIAN.md.source", "heading": "rgbdns-frag-50a9a26b8c55: heading Cloud package build and verification", "sourcePath": "docs/DEBIAN.md", "startLine": 31, "endLine": 77}
```

## Excerpt

<span id="rgbdns-frag-50a9a26b8c55" class="rgbdns-fragment-target"></span>
### rgbdns-frag-50a9a26b8c55: heading Cloud package build and verification

```markdown
## Cloud package build and verification

The `Build Debian package` GitHub Actions workflow runs for relevant changes on
pull requests and `master`, and can also be started manually:

```sh
gh workflow run build-deb.yml --ref master
gh run watch
```

The workflow builds the native debhelper package on Ubuntu 24.04 with the
current stable Rust toolchain. It then:

1. inspects the package control metadata and file table with `dpkg-deb`;
2. rejects `lintian` errors;
3. installs the package in a clean Ubuntu 24.04 container;
4. verifies dpkg's installed state, every Cargo binary, service unit, and
   packaged helper; and
5. uploads the `.deb` as the `rgbdns-debian-amd64` workflow artifact.

Download a completed build and inspect it locally:

```sh
gh run download RUN_ID -n rgbdns-debian-amd64 -D dist/cloud-deb
dpkg-deb --info dist/cloud-deb/rgbdns_*_amd64.deb
dpkg-deb --contents dist/cloud-deb/rgbdns_*_amd64.deb
```

The workflow passes `-d` to `dpkg-buildpackage` because Rust comes from the
pinned Actions toolchain instead of Ubuntu's `cargo` and `rustc` packages.
Debhelper and all other packaging tools still come from Ubuntu packages.

The package creates:

- system user and group `rgbdns`;
- configuration directory `/etc/rgbdns`, owned by `root:rgbdns`;
- state directory `/var/lib/rgbdns/tinydns`, owned by `rgbdns:rgbdns`;
- commands in `/usr/bin` and the setup command in `/usr/sbin`;
- systemd units for authoritative DNS with integrated AXFR and secondary refresh.

The account has no login shell. Services bind privileged port 53 with only
`CAP_NET_BIND_SERVICE`; they do not run as root. The units make the rest of the
filesystem read-only, hide home directories and most process information,
remove privilege escalation, lock the process personality, deny writable
executable memory, limit address families, and grant write access only to the
managed zone state.

```
