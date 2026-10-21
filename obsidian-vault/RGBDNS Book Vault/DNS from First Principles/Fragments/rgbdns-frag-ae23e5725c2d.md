---
type: "code-fragment"
fragment_id: "rgbdns-frag-ae23e5725c2d"
source_path: "docs/OPENSUSE.md"
code_note: "DNS from First Principles/Code/docs/OPENSUSE.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "Configure firewalld and the EC2 security group"
kind: "heading"
start_line: 133
end_line: 172
---

# Configure firewalld and the EC2 security group

- Fragment ID: `rgbdns-frag-ae23e5725c2d`
- Source file: [[DNS from First Principles/Code/docs/OPENSUSE.md.source|docs/OPENSUSE.md]]
- Lines: 133-172
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-ae23e5725c2d", "codeNote": "DNS from First Principles/Code/docs/OPENSUSE.md.source", "sourcePath": "docs/OPENSUSE.md", "startLine": 133, "endLine": 172}
```

## Excerpt

<span id="rgbdns-frag-ae23e5725c2d" class="rgbdns-fragment-target"></span>
### rgbdns-frag-ae23e5725c2d: heading Configure firewalld and the EC2 security group

````markdown
## Configure firewalld and the EC2 security group

The EC2 security group must permit:

- UDP 53 from `0.0.0.0/0`;
- TCP 53 from `0.0.0.0/0`;
- SSH TCP 22 only from trusted administration networks.

If the instance has a routed IPv6 address and the zone publishes it, add the
equivalent IPv6 DNS rules. Do not limit all TCP 53 to the secondaries: normal
clients need TCP fallback. rgbdns applies its transfer allow-list only to AXFR
questions.

If firewalld is already active, enable the predefined DNS service in the
instance's active zone:

```sh
sudo systemctl is-active firewalld
sudo firewall-cmd --get-active-zones
sudo firewall-cmd --permanent --add-service=dns
sudo firewall-cmd --reload
sudo firewall-cmd --list-services
```

If firewalld is not installed or active, preserve that policy choice and make
the equivalent change in the host firewall actually in use. Do not enable a
new remote firewall until its active zone explicitly permits the current SSH
administration path. Keep the AWS security group and host firewall consistent.

Leap 16.0 can run SELinux. Leave it enforcing:

```sh
getenforce 2>/dev/null || true
sudo journalctl -t setroubleshoot --since today 2>/dev/null || true
```

The packaged service uses standard executable, configuration, and state paths.
If a locally customized policy denies access, inspect the audit record and add
a narrow local rule; do not disable SELinux as a workaround.

````
