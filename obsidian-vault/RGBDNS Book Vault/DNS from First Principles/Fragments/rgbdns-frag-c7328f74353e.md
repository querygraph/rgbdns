---
type: "code-fragment"
fragment_id: "rgbdns-frag-c7328f74353e"
source_path: "docs/OPENSUSE.md"
code_note: "DNS from First Principles/Code/docs/OPENSUSE.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "Install and inspect the package"
kind: "heading"
start_line: 93
end_line: 132
---

# Install and inspect the package

- Fragment ID: `rgbdns-frag-c7328f74353e`
- Source file: [[DNS from First Principles/Code/docs/OPENSUSE.md.source|docs/OPENSUSE.md]]
- Lines: 93-132
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-c7328f74353e", "codeNote": "DNS from First Principles/Code/docs/OPENSUSE.md.source", "sourcePath": "docs/OPENSUSE.md", "startLine": 93, "endLine": 132}
```

## Excerpt

<span id="rgbdns-frag-c7328f74353e" class="rgbdns-fragment-target"></span>
### rgbdns-frag-c7328f74353e: heading Install and inspect the package

````markdown
## Install and inspect the package

Copy a locally built package to the EC2 instance:

```sh
scp -i ~/.ssh/KEY.pem \
  dist/rpmbuild/RPMS/x86_64/rgbdns-0.6.5-1.x86_64.rpm \
  ec2-user@PUBLIC_ADDRESS:/tmp/
ssh -i ~/.ssh/KEY.pem ec2-user@PUBLIC_ADDRESS
sudo zypper --non-interactive --no-gpg-checks install \
  /tmp/rgbdns-0.6.5-1.x86_64.rpm
rpm -q rgbdns
rpm -V rgbdns
```

Installation creates:

- the non-login `rgbdns` system user and group;
- `/etc/rgbdns`, owned by `root:rgbdns`;
- `/var/lib/rgbdns/tinydns`, owned by `rgbdns:rgbdns`;
- commands under `/usr/bin` and `rgbdns-setup` under `/usr/sbin`;
- helpers under `/usr/lib/rgbdns`;
- systemd units under `/usr/lib/systemd/system`.

The units are installed disabled and stopped. They bind port 53 with
`CAP_NET_BIND_SERVICE` while the server remains the unprivileged `rgbdns` user.
Filesystem and kernel protections constrain the services to their managed
state and required address families.

Check that no other daemon owns port 53:

```sh
sudo ss -lntup '( sport = :53 )'
sudo systemctl status named unbound dnsmasq systemd-resolved 2>/dev/null || true
```

Resolve conflicts deliberately. Do not break the instance's recursive resolver
or `/etc/resolv.conf`; rgbdns is the public authoritative service, not a
replacement for the host's resolver in this setup.

````
